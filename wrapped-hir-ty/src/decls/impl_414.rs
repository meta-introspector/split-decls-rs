macro_rules! deps {
    () => {
        NextTraitSolveResult!();
    };
}

macro_rules! impl_414 {
    () => {
        deps!();
        impl NextTraitSolveResult { pub fn no_solution (& self) -> bool { matches ! (self , NextTraitSolveResult :: NoSolution) } pub fn certain (& self) -> bool { matches ! (self , NextTraitSolveResult :: Certain) } pub fn uncertain (& self) -> bool { matches ! (self , NextTraitSolveResult :: Uncertain) } }
    };
}

impl_414!();