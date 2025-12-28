macro_rules! deps {
    () => {
        RawIdx!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl From < u32 > for RawIdx { # [inline] fn from (idx : u32) -> RawIdx { RawIdx (idx) } }
    };
}

impl_23!();