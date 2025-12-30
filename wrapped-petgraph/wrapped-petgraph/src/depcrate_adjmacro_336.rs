// Generated macro for macro_336 (macro)
macro_rules! Depcrate_adjmacro_336 {
() => {
// Module: crate::adj
// Provides: {"macro_336"}
// Dependencies: {}
iterator_wrap ! { impl (Iterator DoubleEndedIterator ExactSizeIterator) for # [doc = " An iterator over the indices of the neighbors of a node."] # [derive (Debug , Clone)] struct Neighbors <'a , E , Ix > where { Ix : IndexType } item : NodeIndex < Ix >, iter : core :: iter :: Map < RowIter <'a , E , Ix >, fn (& WSuc < E , Ix >) -> NodeIndex < Ix >>, }
};
}
