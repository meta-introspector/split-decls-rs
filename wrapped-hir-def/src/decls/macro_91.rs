macro_rules! macro_91 {
    () => {
        bitflags :: bitflags ! { # [derive (Debug , Clone , Copy , Eq , PartialEq , Default)] pub struct ConstFlags : u8 { const HAS_BODY = 1 << 1 ; const RUSTC_ALLOW_INCOHERENT_IMPL = 1 << 7 ; } }
    };
}

macro_91!();