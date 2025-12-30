// Generated macro for macro_332 (macro)
macro_rules! Depcrate_adjmacro_332 {
() => {
// Module: crate::adj
// Provides: {"macro_332"}
// Dependencies: {}
iterator_wrap ! { impl (Iterator) for # [doc = " An Iterator over the indices of the outgoing edges from a node."] # [doc = ""] # [doc = " It does not borrow the graph during iteration."] # [derive (Debug , Clone)] struct OutgoingEdgeIndices < Ix > where { Ix : IndexType } item : EdgeIndex < Ix >, iter : core :: iter :: Map < core :: iter :: Zip < Range < usize >, core :: iter :: Repeat < NodeIndex < Ix >>>, fn ((usize , NodeIndex < Ix >)) -> EdgeIndex < Ix >>, }
};
}
