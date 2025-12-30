// Generated macro for SccAnnotations (struct)
macro_rules! Depcrate_handle_placeholdersSccAnnotations {
() => {
// Module: crate::handle_placeholders
// Provides: {"SccAnnotations"}
// Dependencies: {}
# [doc = " A Visitor for SCC annotation construction."] pub (crate) struct SccAnnotations < 'd , 'tcx , A : scc :: Annotation > { pub (crate) scc_to_annotation : IndexVec < ConstraintSccIndex , A > , definitions : & 'd IndexVec < RegionVid , RegionDefinition < 'tcx > > , }
};
}
