macro_rules! deps {
    () => {
        ByteSize!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl ops :: SubAssign < ByteSize > for ByteSize { # [inline (always)] fn sub_assign (& mut self , rhs : ByteSize) { self . 0 -= rhs . 0 } }
    };
}

impl_67!()