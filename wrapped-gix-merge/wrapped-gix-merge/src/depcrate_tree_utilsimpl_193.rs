// Generated macro for impl_193 (impl)
macro_rules! Depcrate_tree_utilsimpl_193 {
() => {
// Module: crate::tree::utils
// Provides: {"impl_193"}
// Dependencies: {}
impl PossibleConflict { pub (super) fn change_idx (& self) -> Option < usize > { match self { PossibleConflict :: TreeToNonTree { change_idx , .. } | PossibleConflict :: NonTreeToTree { change_idx , .. } => { * change_idx } PossibleConflict :: Match { change_idx , .. } | PossibleConflict :: PassedRewrittenDirectory { change_idx , .. } => Some (* change_idx) , } } }
};
}
