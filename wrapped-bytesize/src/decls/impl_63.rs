macro_rules! deps {
    () => {
        ByteSize!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl ops :: AddAssign < ByteSize > for ByteSize { # [inline (always)] fn add_assign (& mut self , rhs : ByteSize) { self . 0 += rhs . 0 } }
    };
}

impl_63!()