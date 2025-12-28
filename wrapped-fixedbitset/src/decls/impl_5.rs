macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl BitAndAssign for Block { # [inline] fn bitand_assign (& mut self , other : Self) { self . 0 . bitand_assign (other . 0) ; } }
    };
}

impl_5!()