macro_rules! deps {
    () => {
        FnCtxt!();
    };
}

macro_rules! Coerce {
    () => {
        deps!();
        struct Coerce < 'a , 'tcx > { fcx : & 'a FnCtxt < 'a , 'tcx > , cause : ObligationCause < 'tcx > , use_lub : bool , # [doc = " Determines whether or not allow_two_phase_borrow is set on any"] # [doc = " autoref adjustments we create while coercing. We don't want to"] # [doc = " allow deref coercions to create two-phase borrows, at least initially,"] # [doc = " but we do need two-phase borrows for function argument reborrows."] # [doc = " See #47489 and #48598"] # [doc = " See docs on the \"AllowTwoPhase\" type for a more detailed discussion"] allow_two_phase : AllowTwoPhase , # [doc = " Whether we allow `NeverToAny` coercions. This is unsound if we're"] # [doc = " coercing a place expression without it counting as a read in the MIR."] # [doc = " This is a side-effect of HIR not really having a great distinction"] # [doc = " between places and values."] coerce_never : bool , }
    };
}

Coerce!()