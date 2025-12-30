// Generated macro for impl_181 (impl)
macro_rules! Depcrate_schemaimpl_181 {
() => {
// Module: crate::schema
// Provides: {"impl_181"}
// Dependencies: {}
impl < T > BorshSchema for BTreeSet < T > where T : BorshSchema , { fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { let definition = Definition :: Sequence { length_width : Definition :: DEFAULT_LENGTH_WIDTH , length_range : Definition :: DEFAULT_LENGTH_RANGE , elements : < T > :: declaration () , } ; add_definition (Self :: declaration () , definition , definitions) ; < T > :: add_definitions_recursively (definitions) ; } fn declaration () -> Declaration { format ! (r#"BTreeSet<{}>"# , T :: declaration ()) } }
};
}
