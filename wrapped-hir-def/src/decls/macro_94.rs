macro_rules! macro_94 {
    () => {
        bitflags :: bitflags ! { # [derive (Debug , Clone , Copy , Eq , PartialEq , Default)] pub struct StaticFlags : u8 { const HAS_BODY = 1 << 1 ; const MUTABLE = 1 << 3 ; const UNSAFE = 1 << 4 ; const EXPLICIT_SAFE = 1 << 5 ; const EXTERN = 1 << 6 ; const RUSTC_ALLOW_INCOHERENT_IMPL = 1 << 7 ; } }
    };
}

macro_94!()