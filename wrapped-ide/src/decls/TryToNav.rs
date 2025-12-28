macro_rules! deps {
    () => {
        UpmappingResult!();
        NavigationTarget!();
    };
}

macro_rules! TryToNav {
    () => {
        deps!();
        pub trait TryToNav { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > ; }
    };
}

TryToNav!()