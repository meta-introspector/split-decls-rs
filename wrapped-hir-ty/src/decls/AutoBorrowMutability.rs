macro_rules! deps {
    () => {
        AllowTwoPhase!();
    };
}

macro_rules! AutoBorrowMutability {
    () => {
        deps!();
        # [derive (Copy , Clone , PartialEq , Eq , Hash , Debug)] pub enum AutoBorrowMutability { Mut { allow_two_phase_borrow : AllowTwoPhase } , Not , }
    };
}

AutoBorrowMutability!()