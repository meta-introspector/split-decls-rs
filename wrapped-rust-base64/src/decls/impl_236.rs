macro_rules! deps {
    () => {
        DecodeError!();
        DecodeSliceError!();
    };
}

macro_rules! impl_236 {
    () => {
        deps!();
        impl From < DecodeError > for DecodeSliceError { fn from (e : DecodeError) -> Self { DecodeSliceError :: DecodeError (e) } }
    };
}

impl_236!()