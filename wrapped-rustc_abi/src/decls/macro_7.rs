macro_rules! deps {
    () => {
        ReprFlags!();
    };
}

macro_rules! macro_7 {
    () => {
        deps!();
        bitflags ! { impl ReprFlags : u8 { const IS_C = 1 << 0 ; const IS_SIMD = 1 << 1 ; const IS_TRANSPARENT = 1 << 2 ; const IS_LINEAR = 1 << 3 ; const RANDOMIZE_LAYOUT = 1 << 4 ; const FIELD_ORDER_UNOPTIMIZABLE = ReprFlags :: IS_C . bits () | ReprFlags :: IS_SIMD . bits () | ReprFlags :: IS_LINEAR . bits () ; const ABI_UNOPTIMIZABLE = ReprFlags :: IS_C . bits () | ReprFlags :: IS_SIMD . bits () ; } }
    };
}

macro_7!()