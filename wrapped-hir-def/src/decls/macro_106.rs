macro_rules! macro_106 {
    () => {
        bitflags ! { # [derive (Debug , Clone , Copy , Eq , PartialEq , Default)] pub struct TypeAliasFlags : u8 { const RUSTC_HAS_INCOHERENT_INHERENT_IMPL = 1 << 1 ; const IS_EXTERN = 1 << 6 ; const RUSTC_ALLOW_INCOHERENT_IMPL = 1 << 7 ; } }
    };
}

macro_106!();