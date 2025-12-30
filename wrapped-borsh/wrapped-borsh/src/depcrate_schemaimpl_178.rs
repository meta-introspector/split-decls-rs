// Generated macro for impl_178 (impl)
macro_rules! Depcrate_schemaimpl_178 {
() => {
// Module: crate::schema
// Provides: {"impl_178"}
// Dependencies: {}
impl < T > BorshSchema for [T] where T : BorshSchema , { fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { let definition = Definition :: Sequence { length_width : Definition :: DEFAULT_LENGTH_WIDTH , length_range : Definition :: DEFAULT_LENGTH_RANGE , elements : T :: declaration () , } ; add_definition (Self :: declaration () , definition , definitions) ; T :: add_definitions_recursively (definitions) ; } fn declaration () -> Declaration { format ! (r#"Vec<{}>"# , T :: declaration ()) } }
};
}
