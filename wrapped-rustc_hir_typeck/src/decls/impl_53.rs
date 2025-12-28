macro_rules! deps {
    () => {
        Diverges!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl ops :: BitOrAssign for Diverges { fn bitor_assign (& mut self , other : Self) { * self = * self | other ; } }
    };
}

impl_53!();