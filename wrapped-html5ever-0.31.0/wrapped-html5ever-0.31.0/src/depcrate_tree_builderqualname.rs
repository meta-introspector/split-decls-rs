// Generated macro for qualname (macro)
macro_rules! Depcrate_tree_builderqualname {
() => {
// Module: crate::tree_builder
// Provides: {"qualname"}
// Dependencies: {}
macro_rules ! qualname { ("" , $ local : tt) => { QualName { prefix : None , ns : ns ! () , local : local_name ! ($ local) , } } ; ($ prefix : tt $ ns : tt $ local : tt) => { QualName { prefix : Some (namespace_prefix ! ($ prefix)) , ns : ns ! ($ ns) , local : local_name ! ($ local) , } } ; }
};
}
