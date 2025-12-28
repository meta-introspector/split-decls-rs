macro_rules! macro_103 {
    () => {
        bitflags ! { # [derive (Debug , Clone , Copy , Eq , PartialEq , Default)] pub struct FnFlags : u16 { const HAS_BODY = 1 << 1 ; const DEFAULT = 1 << 2 ; const CONST = 1 << 3 ; const ASYNC = 1 << 4 ; const UNSAFE = 1 << 5 ; const HAS_VARARGS = 1 << 6 ; const RUSTC_ALLOW_INCOHERENT_IMPL = 1 << 7 ; const HAS_SELF_PARAM = 1 << 8 ; # [doc = " The `#[target_feature]` attribute is necessary to check safety (with RFC 2396),"] # [doc = " but keeping it for all functions will consume a lot of memory when there are"] # [doc = " only very few functions with it. So we only encode its existence here, and lookup"] # [doc = " it if needed."] const HAS_TARGET_FEATURE = 1 << 9 ; const DEPRECATED_SAFE_2024 = 1 << 10 ; const EXPLICIT_SAFE = 1 << 11 ; const RUSTC_INTRINSIC = 1 << 12 ; } }
    };
}

macro_103!()