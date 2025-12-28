macro_rules! deps {
    () => {
        Hasher!();
        Blake2b!();
        UnknownCryptoError!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl Hasher { # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Return a digest selected by the given Blake2b variant."] pub fn digest (& self , data : & [u8]) -> Result < Digest , UnknownCryptoError > { let size : usize = match * self { Hasher :: Blake2b256 => 32 , Hasher :: Blake2b384 => 48 , Hasher :: Blake2b512 => 64 , } ; let mut state = Blake2b :: new (size) ? ; state . update (data) ? ; state . finalize () } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Return a `Blake2b` state selected by the given Blake2b variant."] pub fn init (& self) -> Result < Blake2b , UnknownCryptoError > { match * self { Hasher :: Blake2b256 => Blake2b :: new (32) , Hasher :: Blake2b384 => Blake2b :: new (48) , Hasher :: Blake2b512 => Blake2b :: new (64) , } } }
    };
}

impl_234!();