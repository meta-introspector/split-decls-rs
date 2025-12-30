// Generated macro for impl_162 (impl)
macro_rules! Depcrate_schemaimpl_162 {
() => {
// Module: crate::schema
// Provides: {"impl_162"}
// Dependencies: {}
impl BorshSchema for str { # [inline] fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { let definition = Definition :: Sequence { length_width : Definition :: DEFAULT_LENGTH_WIDTH , length_range : Definition :: DEFAULT_LENGTH_RANGE , elements : u8 :: declaration () , } ; add_definition (Self :: declaration () , definition , definitions) ; u8 :: add_definitions_recursively (definitions) ; } # [inline] fn declaration () -> Declaration { "String" . into () } }
};
}
