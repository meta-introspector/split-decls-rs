macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < 'a > Bytes < 'a > { # [doc = " Views the remaining underlying data as a subslice of the original data."] # [doc = " This has the same lifetime as the original slice,"] # [doc = " and so the iterator can continue to be used while this exists."] # [inline] pub fn as_bytes (& self) -> & 'a [u8] { self . it . as_slice () } }
    };
}

impl_77!()