// Generated macro for in_place_collectible (function)
macro_rules! Depcrate_vec_in_place_collectin_place_collectible {
() => {
// Module: crate::vec::in_place_collect
// Provides: {"in_place_collectible"}
// Dependencies: {}
const fn in_place_collectible < DEST , SRC > (step_merge : Option < NonZero < usize > > , step_expand : Option < NonZero < usize > > ,) -> bool { if const { SRC :: IS_ZST || DEST :: IS_ZST || align_of :: < SRC > () != align_of :: < DEST > () } { return false ; } match (step_merge , step_expand) { (Some (step_merge) , Some (step_expand)) => { size_of :: < SRC > () * step_merge . get () >= size_of :: < DEST > () * step_expand . get () } _ => false , } }
};
}
