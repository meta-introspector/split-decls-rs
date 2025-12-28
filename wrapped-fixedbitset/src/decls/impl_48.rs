macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl BitAndAssign for Block { # [inline] fn bitand_assign (& mut self , other : Self) { self . 0 = v128_and (self . 0 , other . 0) ; } }
    };
}

impl_48!()