macro_rules! deps {
    () => {
        U32X4!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl AddAssign < Self > for U32X4 { # [inline] fn add_assign (& mut self , other : Self) { self . 0 [0] += other . 0 [0] ; self . 0 [1] += other . 0 [1] ; self . 0 [2] += other . 0 [2] ; self . 0 [3] += other . 0 [3] ; } }
    };
}

impl_4!()