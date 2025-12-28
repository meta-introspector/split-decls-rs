macro_rules! deps {
    () => {
        Diverges!();
    };
}

macro_rules! impl_275 {
    () => {
        deps!();
        impl std :: ops :: BitAndAssign for Diverges { fn bitand_assign (& mut self , other : Self) { * self = * self & other ; } }
    };
}

impl_275!()