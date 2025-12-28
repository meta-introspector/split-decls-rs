macro_rules! deps {
    () => {
        UpmappingResult!();
        TryToNav!();
        NavigationTarget!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl TryToNav for hir :: AssocItem { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { match self { AssocItem :: Function (it) => it . try_to_nav (sema) , AssocItem :: Const (it) => it . try_to_nav (sema) , AssocItem :: TypeAlias (it) => it . try_to_nav (sema) , } } }
    };
}

impl_43!()