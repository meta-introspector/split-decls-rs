macro_rules! deps {
    () => {
        BorrowckDomain!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl JoinSemiLattice for BorrowckDomain { fn join (& mut self , _other : & Self) -> bool { unreachable ! () ; } }
    };
}

impl_47!();