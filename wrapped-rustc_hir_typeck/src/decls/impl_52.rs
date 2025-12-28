macro_rules! deps {
    () => {
        Diverges!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl ops :: BitAndAssign for Diverges { fn bitand_assign (& mut self , other : Self) { * self = * self & other ; } }
    };
}

impl_52!()