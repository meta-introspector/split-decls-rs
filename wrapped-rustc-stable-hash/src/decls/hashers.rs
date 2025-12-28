macro_rules! deps {
    () => {
        StableHasher!();
        SipHasher128Hash!();
        SipHasher128!();
    };
}

macro_rules! hashers {
    () => {
        deps!();
        # [doc = " Hashers collection"] pub mod hashers { # [doc (inline)] pub use super :: sip128 :: { SipHasher128 , SipHasher128Hash } ; # [doc = " Stable 128-bits Sip Hasher"] # [doc = ""] # [doc = " [`StableHasher`] version of [`SipHasher128`]."] # [doc = ""] # [doc = " [`StableHasher`]: super::StableHasher"] pub type StableSipHasher128 = super :: StableHasher < SipHasher128 > ; }
    };
}

hashers!()