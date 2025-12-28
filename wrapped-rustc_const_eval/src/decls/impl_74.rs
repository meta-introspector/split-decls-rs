macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl JoinSemiLattice for State { fn join (& mut self , other : & Self) -> bool { self . qualif . join (& other . qualif) || self . borrow . join (& other . borrow) } }
    };
}

impl_74!();