// Generated macro for CombineSubstructureFunc (type)
macro_rules! Depcrate_deriving_genericCombineSubstructureFunc {
() => {
// Module: crate::deriving::generic
// Provides: {"CombineSubstructureFunc"}
// Dependencies: {}
# [doc = " Combine the values of all the fields together. The last argument is"] # [doc = " all the fields of all the structures."] pub (crate) type CombineSubstructureFunc < 'a > = Box < dyn FnMut (& ExtCtxt < '_ > , Span , & Substructure < '_ >) -> BlockOrExpr + 'a > ;
};
}
