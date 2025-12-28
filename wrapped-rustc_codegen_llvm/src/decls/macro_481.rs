macro_rules! macro_481 {
    () => {
        bitflags ! { # [repr (transparent)] # [derive (Default)] pub struct GEPNoWrapFlags : c_uint { const InBounds = 1 << 0 ; const NUSW = 1 << 1 ; const NUW = 1 << 2 ; } }
    };
}

macro_481!();