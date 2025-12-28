macro_rules! deps {
    () => {
        NavigationTarget!();
        UpmappingResult!();
    };
}

macro_rules! TryToNav {
    () => {
        deps!();
        pub trait TryToNav { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > ; }
    };
}

TryToNav!();