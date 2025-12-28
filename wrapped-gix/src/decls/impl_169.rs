macro_rules! deps {
    () => {
        ObjectDetached!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl AsRef < [u8] > for ObjectDetached { fn as_ref (& self) -> & [u8] { & self . data } }
    };
}

impl_169!()