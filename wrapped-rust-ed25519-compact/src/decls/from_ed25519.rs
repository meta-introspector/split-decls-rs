macro_rules! deps {
    () => {
        PublicKey!();
        SecretKey!();
        Error!();
        KeyPair!();
        Hash!();
    };
}

macro_rules! from_ed25519 {
    () => {
        deps!();
        # [cfg (not (feature = "disable-signatures"))] mod from_ed25519 { use super :: super :: { edwards25519 , sha512 , KeyPair as EdKeyPair , PublicKey as EdPublicKey , SecretKey as EdSecretKey , } ; use super :: * ; impl SecretKey { # [doc = " Convert an Ed25519 secret key to a X25519 secret key."] pub fn from_ed25519 (edsk : & EdSecretKey) -> Result < SecretKey , Error > { let seed = edsk . seed () ; let az : [u8 ; 64] = { let mut hash_output = sha512 :: Hash :: hash (* seed) ; hash_output [0] &= 248 ; hash_output [31] &= 63 ; hash_output [31] |= 64 ; hash_output } ; SecretKey :: from_slice (& az [.. 32]) } } impl PublicKey { # [doc = " Convert an Ed25519 public key to a X25519 public key."] pub fn from_ed25519 (edpk : & EdPublicKey) -> Result < PublicKey , Error > { let pk = PublicKey :: from_slice (& edwards25519 :: ge_to_x25519_vartime (edpk) . ok_or (Error :: InvalidPublicKey) ? ,) ? ; pk . clear_cofactor () ? ; Ok (pk) } } impl KeyPair { # [doc = " Convert an Ed25519 key pair to a X25519 key pair."] pub fn from_ed25519 (edkp : & EdKeyPair) -> Result < KeyPair , Error > { let pk = PublicKey :: from_ed25519 (& edkp . pk) ? ; let sk = SecretKey :: from_ed25519 (& edkp . sk) ? ; Ok (KeyPair { pk , sk }) } } }
    };
}

from_ed25519!()