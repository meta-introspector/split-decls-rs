// Generated macro for impl_172 (impl)
macro_rules! Depcrate_schemaimpl_172 {
() => {
// Module: crate::schema
// Provides: {"impl_172"}
// Dependencies: {}
impl < T > BorshSchema for Option < T > where T : BorshSchema , { fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { let definition = Definition :: Enum { tag_width : 1 , variants : vec ! [(0u8 as i64 , "None" . to_string () , < () >:: declaration ()) , (1u8 as i64 , "Some" . to_string () , T :: declaration ()) ,] , } ; add_definition (Self :: declaration () , definition , definitions) ; T :: add_definitions_recursively (definitions) ; < () > :: add_definitions_recursively (definitions) ; } fn declaration () -> Declaration { format ! (r#"Option<{}>"# , T :: declaration ()) } }
};
}
