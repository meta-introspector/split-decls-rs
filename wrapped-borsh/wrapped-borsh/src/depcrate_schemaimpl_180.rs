// Generated macro for impl_180 (impl)
macro_rules! Depcrate_schemaimpl_180 {
() => {
// Module: crate::schema
// Provides: {"impl_180"}
// Dependencies: {}
impl < K , V > BorshSchema for BTreeMap < K , V > where K : BorshSchema , V : BorshSchema , { fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { let definition = Definition :: Sequence { length_width : Definition :: DEFAULT_LENGTH_WIDTH , length_range : Definition :: DEFAULT_LENGTH_RANGE , elements : < (K , V) > :: declaration () , } ; add_definition (Self :: declaration () , definition , definitions) ; < (K , V) > :: add_definitions_recursively (definitions) ; } fn declaration () -> Declaration { format ! (r#"BTreeMap<{}, {}>"# , K :: declaration () , V :: declaration ()) } }
};
}
