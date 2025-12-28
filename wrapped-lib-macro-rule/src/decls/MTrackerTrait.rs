macro_rules! deps {
    () => {
        LibMacroRuleTracker!();
    };
}

macro_rules! MTrackerTrait {
    () => {
        deps!();
        # [macro_export] macro_rules ! MTrackerTrait { () => { $ crate :: tracker :: LibMacroRuleTracker } ; }
    };
}

MTrackerTrait!();