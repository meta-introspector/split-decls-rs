macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl BitOrAssign for Block { # [inline] fn bitor_assign (& mut self , other : Self) { self . 0 = v128_or (self . 0 , other . 0) ; } }
    };
}

impl_50!();