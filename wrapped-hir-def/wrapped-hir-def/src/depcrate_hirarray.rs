// Generated macro for Array (enum)
macro_rules! Depcrate_hirArray {
() => {
// Module: crate::hir
// Provides: {"Array"}
// Dependencies: {}
# [derive (Debug , Clone , Eq , PartialEq)] pub enum Array { ElementList { elements : Box < [ExprId] > } , Repeat { initializer : ExprId , repeat : ExprId } , }
};
}
