// Generated macro for bound_to_trait_def_id (function)
macro_rules! Depcrate_extra_unused_type_parametersbound_to_trait_def_id {
() => {
// Module: crate::extra_unused_type_parameters
// Provides: {"bound_to_trait_def_id"}
// Dependencies: {}
# [doc = " Given a generic bound, if the bound is for a trait that's not a `LangItem`, return the"] # [doc = " `LocalDefId` for that trait."] fn bound_to_trait_def_id (bound : & GenericBound < '_ >) -> Option < LocalDefId > { bound . trait_ref () ? . trait_def_id () ? . as_local () }
};
}
