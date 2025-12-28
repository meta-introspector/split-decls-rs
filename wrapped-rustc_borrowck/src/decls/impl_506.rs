macro_rules! deps {
    () => {
        GatherUsedMutsVisitor!();
    };
}

macro_rules! impl_506 {
    () => {
        deps!();
        impl GatherUsedMutsVisitor < '_ , '_ , '_ , '_ > { fn remove_never_initialized_mut_locals (& mut self , into : Place < '_ >) { self . never_initialized_mut_locals . swap_remove (& into . local) ; } }
    };
}

impl_506!();