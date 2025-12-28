macro_rules! deps {
    () => {
        NavigationTarget!();
        TryToNav!();
        UpmappingResult!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < T : TryToNav , U : TryToNav > TryToNav for Either < T , U > { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { match self { Either :: Left (it) => it . try_to_nav (sema) , Either :: Right (it) => it . try_to_nav (sema) , } } }
    };
}

impl_19!();