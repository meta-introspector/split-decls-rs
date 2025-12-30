// Generated macro for impl_693 (impl)
macro_rules! Depcrate_view_memory_layoutimpl_693 {
() => {
// Module: crate::view_memory_layout
// Provides: {"impl_693"}
// Dependencies: {}
impl FieldOrTupleIdx { fn name (& self , db : & RootDatabase) -> String { match * self { FieldOrTupleIdx :: Field (f) => f . name (db) . as_str () . to_owned () , FieldOrTupleIdx :: TupleIdx (i) => format ! (".{i}") , } } }
};
}
