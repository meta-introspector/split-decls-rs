// Generated macro for impl_112 (impl)
macro_rules! Depcrate_typesimpl_112 {
() => {
// Module: crate::types
// Provides: {"impl_112"}
// Dependencies: {}
impl SourceItemOrderingWithinModuleItemGroupings { pub fn ordered_within (& self , grouping_name : & String) -> bool { match self { SourceItemOrderingWithinModuleItemGroupings :: All => true , SourceItemOrderingWithinModuleItemGroupings :: None => false , SourceItemOrderingWithinModuleItemGroupings :: Custom (groups) => groups . contains (grouping_name) , } } }
};
}
