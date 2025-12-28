macro_rules! deps {
    () => {
        RawIdx!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl From < RawIdx > for u32 { # [inline] fn from (raw : RawIdx) -> u32 { raw . 0 } }
    };
}

impl_22!()