macro_rules! deps {
    () => {
        Diverges!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl std :: ops :: BitOrAssign for Diverges { fn bitor_assign (& mut self , other : Self) { * self = * self | other ; } }
    };
}

impl_90!()