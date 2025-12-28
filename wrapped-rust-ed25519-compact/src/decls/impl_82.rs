macro_rules! deps {
    () => {
        Seed!();
        Hash!();
        Error!();
        KeyPair!();
        PublicKey!();
        SecretKey!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl KeyPair { # [doc = " Number of bytes in a key pair."] pub const BYTES : usize = SecretKey :: BYTES ; # [doc = " Generates a new key pair."] # [cfg (feature = "random")] pub fn generate () -> KeyPair { KeyPair :: from_seed (Seed :: default ()) } # [doc = " Generates a new key pair using a secret seed."] pub fn from_seed (seed : Seed) -> KeyPair { if seed . iter () . fold (0 , | acc , x | acc | x) == 0 { panic ! ("All-zero seed") ; } let (scalar , _) = { let hash_output = sha512 :: Hash :: hash (& seed [..]) ; KeyPair :: split (& hash_output , false , true) } ; let pk = ge_scalarmult_base (& scalar) . to_bytes () ; let mut sk = [0u8 ; 64] ; sk [0 .. 32] . copy_from_slice (& * seed) ; sk [32 .. 64] . copy_from_slice (& pk) ; KeyPair { pk : PublicKey (pk) , sk : SecretKey (sk) , } } # [doc = " Creates a key pair from a slice."] pub fn from_slice (bytes : & [u8]) -> Result < Self , Error > { let sk = SecretKey :: from_slice (bytes) ? ; let pk = sk . public_key () ; Ok (KeyPair { pk , sk }) } # [doc = " Clamp a scalar."] pub fn clamp (scalar : & mut [u8]) { scalar [0] &= 248 ; scalar [31] &= 63 ; scalar [31] |= 64 ; } # [doc = " Split a serialized representation of a key pair into a secret scalar and"] # [doc = " a prefix."] pub fn split (bytes : & [u8 ; 64] , reduce : bool , clamp : bool) -> ([u8 ; 32] , [u8 ; 32]) { let mut scalar = [0u8 ; 32] ; scalar . copy_from_slice (& bytes [0 .. 32]) ; if clamp { Self :: clamp (& mut scalar) ; } if reduce { sc_reduce32 (& mut scalar) ; } let mut prefix = [0u8 ; 32] ; prefix . copy_from_slice (& bytes [32 .. 64]) ; (scalar , prefix) } # [doc = " Check that the public key is valid for the secret key."] pub fn validate (& self) -> Result < () , Error > { self . sk . validate_public_key (& self . pk) } }
    };
}

impl_82!()