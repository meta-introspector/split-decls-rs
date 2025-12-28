macro_rules! deps {
    () => {
        PredicateObligations!();
    };
}

macro_rules! ScrubbedTraitError {
    () => {
        deps!();
        # [doc = " A trait error with most of its information removed. This is the error"] # [doc = " returned by an `ObligationCtxt` by default, and suitable if you just"] # [doc = " want to see if a predicate holds, and don't particularly care about the"] # [doc = " error itself (except for if it's an ambiguity or true error)."] # [doc = ""] # [doc = " use `ObligationCtxt::new_with_diagnostics` to get a `FulfillmentError`."] # [derive (Clone , Debug)] pub enum ScrubbedTraitError < 'tcx > { # [doc = " A real error. This goal definitely does not hold."] TrueError , # [doc = " An ambiguity. This goal may hold if further inference is done."] Ambiguity , # [doc = " An old-solver-style cycle error, which will fatal. This is not"] # [doc = " returned by the new solver."] Cycle (PredicateObligations < 'tcx >) , }
    };
}

ScrubbedTraitError!()