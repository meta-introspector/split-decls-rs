macro_rules! deps {
    () => {
        Borrows!();
    };
}

macro_rules! Borrowck {
    () => {
        deps!();
        pub (crate) struct Borrowck < 'a , 'tcx > { pub (crate) borrows : Borrows < 'a , 'tcx > , pub (crate) uninits : MaybeUninitializedPlaces < 'a , 'tcx > , pub (crate) ever_inits : EverInitializedPlaces < 'a , 'tcx > , }
    };
}

Borrowck!();