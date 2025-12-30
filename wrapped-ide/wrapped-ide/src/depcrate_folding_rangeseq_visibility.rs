// Generated macro for eq_visibility (function)
macro_rules! Depcrate_folding_rangeseq_visibility {
() => {
// Module: crate::folding_ranges
// Provides: {"eq_visibility"}
// Dependencies: {}
fn eq_visibility (vis0 : Option < ast :: Visibility > , vis1 : Option < ast :: Visibility >) -> bool { match (vis0 , vis1) { (None , None) => true , (Some (vis0) , Some (vis1)) => vis_eq (& vis0 , & vis1) , _ => false , } }
};
}
