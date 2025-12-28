macro_rules! deps {
    () => {
        SecretKey!();
        Error!();
        PublicKey!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl SecretKey { # [doc = " Number of bytes in a secret key."] pub const BYTES : usize = 32 ; # [doc = " Creates a secret key from raw bytes."] pub fn new (sk : [u8 ; SecretKey :: BYTES]) -> Self { SecretKey (sk) } # [doc = " Creates a secret key from a slice."] pub fn from_slice (sk : & [u8]) -> Result < Self , Error > { let mut sk_ = [0u8 ; SecretKey :: BYTES] ; if sk . len () != sk_ . len () { return Err (Error :: InvalidSecretKey) ; } sk_ . copy_from_slice (sk) ; Ok (SecretKey :: new (sk_)) } # [doc = " Perform the X25519 clamping magic"] pub fn clamped (& self) -> SecretKey { let mut clamped = self . clone () ; clamped [0] &= 248 ; clamped [31] &= 63 ; clamped [31] |= 64 ; clamped } # [doc = " Recover the public key"] pub fn recover_public_key (& self) -> Result < PublicKey , Error > { let sk = self . clamped () ; Ok (PublicKey (PublicKey :: base_point () . ladder (& sk . 0 , 255) ?)) } # [doc = " Returns `Ok(())` if the given public key is the public counterpart of"] # [doc = " this secret key."] # [doc = " Returns `Err(Error::InvalidPublicKey)` otherwise."] pub fn validate_public_key (& self , pk : & PublicKey) -> Result < () , Error > { let recovered_pk = self . recover_public_key () ? ; if recovered_pk != * pk { return Err (Error :: InvalidPublicKey) ; } Ok (()) } }
    };
}

impl_142!();