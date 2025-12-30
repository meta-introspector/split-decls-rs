// Generated macro for impl_42 (impl)
macro_rules! Depcrate_walk_classifyimpl_42 {
() => {
// Module: crate::walk::classify
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'a > EntryRef < 'a > { pub (super) fn from_outcome (rela_path : Cow < 'a , BStr > , info : crate :: walk :: classify :: Outcome) -> Self { EntryRef { rela_path , property : info . property , status : info . status , disk_kind : info . disk_kind , index_kind : info . index_kind , pathspec_match : info . pathspec_match , } } }
};
}
