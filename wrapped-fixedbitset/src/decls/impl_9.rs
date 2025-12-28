macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl BitXorAssign for Block { # [inline] fn bitxor_assign (& mut self , other : Self) { self . 0 . bitxor_assign (other . 0) } }
    };
}

impl_9!();