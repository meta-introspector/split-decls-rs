macro_rules! deps {
    () => {
        Interest!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl ops :: BitOrAssign for Interest { # [inline] fn bitor_assign (& mut self , other : Self) { self . 0 = (* self | other) . 0 ; } }
    };
}

impl_20!()