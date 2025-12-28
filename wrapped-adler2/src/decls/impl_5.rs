macro_rules! deps {
    () => {
        U32X4!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl RemAssign < u32 > for U32X4 { # [inline] fn rem_assign (& mut self , quotient : u32) { self . 0 [0] %= quotient ; self . 0 [1] %= quotient ; self . 0 [2] %= quotient ; self . 0 [3] %= quotient ; } }
    };
}

impl_5!()