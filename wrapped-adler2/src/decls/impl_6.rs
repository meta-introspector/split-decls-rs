macro_rules! deps {
    () => {
        U32X4!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl MulAssign < u32 > for U32X4 { # [inline] fn mul_assign (& mut self , rhs : u32) { self . 0 [0] *= rhs ; self . 0 [1] *= rhs ; self . 0 [2] *= rhs ; self . 0 [3] *= rhs ; } }
    };
}

impl_6!()