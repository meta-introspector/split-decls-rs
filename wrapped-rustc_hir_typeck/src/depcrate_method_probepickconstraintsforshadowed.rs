// Generated macro for PickConstraintsForShadowed (struct)
macro_rules! Depcrate_method_probePickConstraintsForShadowed {
() => {
// Module: crate::method::probe
// Provides: {"PickConstraintsForShadowed"}
// Dependencies: {}
# [doc = " Criteria to apply when searching for a given Pick. This is used during"] # [doc = " the search for potentially shadowed methods to ensure we don't search"] # [doc = " more candidates than strictly necessary."] # [derive (Debug)] struct PickConstraintsForShadowed { autoderefs : usize , receiver_steps : Option < usize > , def_id : DefId , }
};
}
