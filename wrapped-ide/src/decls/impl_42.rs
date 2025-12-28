macro_rules! deps {
    () => {
        TryToNav!();
        UpmappingResult!();
        NavigationTarget!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl TryToNav for hir :: Adt { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { match self { hir :: Adt :: Struct (it) => it . try_to_nav (sema) , hir :: Adt :: Union (it) => it . try_to_nav (sema) , hir :: Adt :: Enum (it) => it . try_to_nav (sema) , } } }
    };
}

impl_42!()