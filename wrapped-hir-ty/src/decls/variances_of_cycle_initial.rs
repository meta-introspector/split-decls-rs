macro_rules! deps {
    () => {
        HirDatabase!();
    };
}

macro_rules! variances_of_cycle_initial {
    () => {
        deps!();
        pub (crate) fn variances_of_cycle_initial (db : & dyn HirDatabase , def : GenericDefId ,) -> VariancesOf < '_ > { let interner = DbInterner :: new_with (db , None , None) ; let generics = generics (db , def) ; let count = generics . len () ; VariancesOf :: new_from_iter (interner , std :: iter :: repeat_n (Variance :: Invariant , count)) }
    };
}

variances_of_cycle_initial!();