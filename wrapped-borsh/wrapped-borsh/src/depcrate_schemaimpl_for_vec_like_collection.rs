// Generated macro for impl_for_vec_like_collection (macro)
macro_rules! Depcrate_schemaimpl_for_vec_like_collection {
() => {
// Module: crate::schema
// Provides: {"impl_for_vec_like_collection"}
// Dependencies: {}
macro_rules ! impl_for_vec_like_collection { ($ type : ident) => { impl < T > BorshSchema for $ type < T > where T : BorshSchema , { fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { let definition = Definition :: Sequence { length_width : Definition :: DEFAULT_LENGTH_WIDTH , length_range : Definition :: DEFAULT_LENGTH_RANGE , elements : T :: declaration () , } ; add_definition (Self :: declaration () , definition , definitions) ; T :: add_definitions_recursively (definitions) ; } fn declaration () -> Declaration { format ! (r#"{}<{}>"# , stringify ! ($ type) , T :: declaration ()) } } } ; }
};
}
