// Generated macro for impl_140 (impl)
macro_rules! Depcrate_schemaimpl_140 {
() => {
// Module: crate::schema
// Provides: {"impl_140"}
// Dependencies: {}
impl < T > BorshSchema for core :: cell :: RefCell < T > where T : BorshSchema + Sized , { fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { T :: add_definitions_recursively (definitions) ; } fn declaration () -> Declaration { T :: declaration () } }
};
}
