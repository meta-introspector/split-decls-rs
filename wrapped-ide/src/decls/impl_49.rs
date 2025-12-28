macro_rules! deps {
    () => {
        UpmappingResult!();
        TryToNav!();
        NavigationTarget!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl TryToNav for hir :: TypeOrConstParam { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { self . split (sema . db) . try_to_nav (sema) } }
    };
}

impl_49!();