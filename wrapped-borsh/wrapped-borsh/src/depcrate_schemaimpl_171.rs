// Generated macro for impl_171 (impl)
macro_rules! Depcrate_schemaimpl_171 {
() => {
// Module: crate::schema
// Provides: {"impl_171"}
// Dependencies: {}
impl < T , const N : usize > BorshSchema for [T ; N] where T : BorshSchema , { fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { use core :: convert :: TryFrom ; let length = u64 :: try_from (N) . unwrap () ; let definition = Definition :: Sequence { length_width : Definition :: ARRAY_LENGTH_WIDTH , length_range : length ..= length , elements : T :: declaration () , } ; add_definition (Self :: declaration () , definition , definitions) ; T :: add_definitions_recursively (definitions) ; } fn declaration () -> Declaration { format ! (r#"[{}; {}]"# , T :: declaration () , N) } }
};
}
