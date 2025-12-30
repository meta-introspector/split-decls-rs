// Generated macro for walk_flat_map_pat_field (function)
macro_rules! Depcrate_mut_visitwalk_flat_map_pat_field {
() => {
// Module: crate::mut_visit
// Provides: {"walk_flat_map_pat_field"}
// Dependencies: {}
pub fn walk_flat_map_pat_field < T : MutVisitor > (vis : & mut T , mut fp : PatField ,) -> SmallVec < [PatField ; 1] > { vis . visit_pat_field (& mut fp) ; smallvec ! [fp] }
};
}
