macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl BitOrAssign for Block { # [inline] fn bitor_assign (& mut self , other : Self) { self . 0 . bitor_assign (other . 0) } }
    };
}

impl_7!()