macro_rules! deps {
    () => {
        LibMacroRuleTracker!();
    };
}

macro_rules! ImplMTrackerTrait {
    () => {
        deps!();
        # [macro_export] macro_rules ! ImplMTrackerTrait { ($ for_type : ty { $ ($ body : tt) * }) => { impl $ crate :: tracker :: LibMacroRuleTracker for $ for_type { $ ($ body) * } } ; }
    };
}

ImplMTrackerTrait!();