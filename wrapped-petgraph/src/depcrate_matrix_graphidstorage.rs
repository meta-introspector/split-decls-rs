// Generated macro for IdStorage (struct)
macro_rules! Depcrate_matrix_graphIdStorage {
() => {
// Module: crate::matrix_graph
// Provides: {"IdStorage"}
// Dependencies: {}
# [derive (Debug , Clone)] struct IdStorage < T , # [cfg (not (feature = "std"))] S , # [cfg (feature = "std")] S = RandomState > { elements : Vec < Option < T > > , upper_bound : usize , removed_ids : IndexSet < usize , S > , }
};
}
