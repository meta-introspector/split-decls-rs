// Generated macro for impl_138 (impl)
macro_rules! Depcrate_schemaimpl_138 {
() => {
// Module: crate::schema
// Provides: {"impl_138"}
// Dependencies: {}
impl < T > BorshSchema for Box < T > where T : BorshSchema + ? Sized , { fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { T :: add_definitions_recursively (definitions) ; } fn declaration () -> Declaration { T :: declaration () } }
};
}
