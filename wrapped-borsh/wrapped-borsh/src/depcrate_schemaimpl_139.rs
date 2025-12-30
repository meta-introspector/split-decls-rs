// Generated macro for impl_139 (impl)
macro_rules! Depcrate_schemaimpl_139 {
() => {
// Module: crate::schema
// Provides: {"impl_139"}
// Dependencies: {}
impl < T > BorshSchema for core :: cell :: Cell < T > where T : BorshSchema + Copy , { fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { T :: add_definitions_recursively (definitions) ; } fn declaration () -> Declaration { T :: declaration () } }
};
}
