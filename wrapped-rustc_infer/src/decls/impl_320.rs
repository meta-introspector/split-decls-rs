macro_rules! deps {
    () => {
        PredicateObligation!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl < 'tcx > PredicateObligation < 'tcx > { # [doc = " Flips the polarity of the inner predicate."] # [doc = ""] # [doc = " Given `T: Trait` predicate it returns `T: !Trait` and given `T: !Trait` returns `T: Trait`."] pub fn flip_polarity (& self , tcx : TyCtxt < 'tcx >) -> Option < PredicateObligation < 'tcx > > { Some (PredicateObligation { cause : self . cause . clone () , param_env : self . param_env , predicate : self . predicate . flip_polarity (tcx) ? , recursion_depth : self . recursion_depth , }) } }
    };
}

impl_320!();