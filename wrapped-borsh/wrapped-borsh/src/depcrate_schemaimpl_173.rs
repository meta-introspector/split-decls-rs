// Generated macro for impl_173 (impl)
macro_rules! Depcrate_schemaimpl_173 {
() => {
// Module: crate::schema
// Provides: {"impl_173"}
// Dependencies: {}
impl < T , E > BorshSchema for core :: result :: Result < T , E > where T : BorshSchema , E : BorshSchema , { fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { let definition = Definition :: Enum { tag_width : 1 , variants : vec ! [(1u8 as i64 , "Ok" . to_string () , T :: declaration ()) , (0u8 as i64 , "Err" . to_string () , E :: declaration ()) ,] , } ; add_definition (Self :: declaration () , definition , definitions) ; T :: add_definitions_recursively (definitions) ; E :: add_definitions_recursively (definitions) ; } fn declaration () -> Declaration { format ! (r#"Result<{}, {}>"# , T :: declaration () , E :: declaration ()) } }
};
}
