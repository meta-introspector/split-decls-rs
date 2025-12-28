macro_rules! deps {
    () => {
        AllowTwoPhase!();
        AutoBorrowMutability!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl AutoBorrowMutability { # [doc = " Creates an `AutoBorrowMutability` from a mutability and allowance of two phase borrows."] # [doc = ""] # [doc = " Note that when `mutbl.is_not()`, `allow_two_phase_borrow` is ignored"] pub fn new (mutbl : Mutability , allow_two_phase_borrow : AllowTwoPhase) -> Self { match mutbl { Mutability :: Not => Self :: Not , Mutability :: Mut => Self :: Mut { allow_two_phase_borrow } , } } }
    };
}

impl_64!();