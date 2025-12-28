macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! LenType {
    () => {
        deps!();
        # [doc = " A sealed trait representing a valid type to use as a length for a container."] # [doc = ""] # [doc = " This cannot be implemented in user code, and is restricted to `u8`, `u16`, `u32`, and `usize`."] # [cfg (not (feature = "zeroize"))] pub trait LenType : Sealed { }
    };
}

LenType!();