macro_rules! deps {
    () => {
        EphemeralServerSession!();
        UnknownCryptoError!();
        PublicKey!();
        SessionKeys!();
        PrivateKey!();
    };
}

macro_rules! impl_626 {
    () => {
        deps!();
        impl EphemeralServerSession { # [doc = " Generate a new random key pair."] pub fn new () -> Result < Self , UnknownCryptoError > { let privkey = PrivateKey :: generate () ; let pubkey : PublicKey = PublicKey :: try_from (& privkey) ? ; Ok (Self { private_key : privkey , public_key : pubkey , }) } # [doc = " Get a reference to the [`PublicKey`]."] pub fn public_key (& self) -> & PublicKey { & self . public_key } # [doc = " Get a reference to the [`PrivateKey`]."] pub fn private_key (& self) -> & PrivateKey { & self . private_key } # [doc = " Establish session keys with a client. This moves `self` to ensure that the keys"] # [doc = " generated with [`Self::new()`] are only used for this key exchange, thus remaining ephemeral."] pub fn establish_with_client (self , client_public_key : & PublicKey ,) -> Result < SessionKeys , UnknownCryptoError > { let q = x25519 :: key_agreement (& self . private_key , client_public_key) ? ; let keys = establish_session_keys (& q , client_public_key , & self . public_key) ? ; Ok (SessionKeys { rx : SecretKey :: from_slice (& keys . as_ref () [32 ..]) ? , tx : SecretKey :: from_slice (& keys . as_ref () [.. 32]) ? , }) } }
    };
}

impl_626!();