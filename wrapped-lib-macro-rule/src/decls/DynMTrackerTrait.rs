macro_rules! deps {
    () => {
        Failure!();
        ParseResultBase!();
        LibMacroRuleTracker!();
    };
}

macro_rules! DynMTrackerTrait {
    () => {
        deps!();
        # [macro_export] macro_rules ! DynMTrackerTrait { () => { dyn $ crate :: tracker :: LibMacroRuleTracker < Failure = Box < dyn $ crate :: parse_result :: ParseResultBase < () >>> } ; }
    };
}

DynMTrackerTrait!()