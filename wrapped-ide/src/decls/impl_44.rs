macro_rules! deps {
    () => {
        NavigationTarget!();
        TryToNav!();
        UpmappingResult!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl TryToNav for hir :: GenericParam { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { match self { hir :: GenericParam :: TypeParam (it) => it . try_to_nav (sema) , hir :: GenericParam :: ConstParam (it) => it . try_to_nav (sema) , hir :: GenericParam :: LifetimeParam (it) => it . try_to_nav (sema) , } } }
    };
}

impl_44!();