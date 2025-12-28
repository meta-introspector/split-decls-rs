macro_rules! deps {
    () => {
        MirBorrowckCtxt!();
        Prefixes!();
        PrefixSet!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        impl < 'tcx > MirBorrowckCtxt < '_ , '_ , 'tcx > { # [doc = " Returns an iterator over the prefixes of `place`"] # [doc = " (inclusive) from longest to smallest, potentially"] # [doc = " terminating the iteration early based on `kind`."] pub (super) fn prefixes (& self , place_ref : PlaceRef < 'tcx > , kind : PrefixSet) -> Prefixes < 'tcx > { Prefixes { next : Some (place_ref) , kind } } }
    };
}

impl_310!();