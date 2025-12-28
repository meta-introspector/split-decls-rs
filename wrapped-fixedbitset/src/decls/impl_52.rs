macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl BitXorAssign for Block { # [inline] fn bitxor_assign (& mut self , other : Self) { self . 0 = v128_xor (self . 0 , other . 0) } }
    };
}

impl_52!()