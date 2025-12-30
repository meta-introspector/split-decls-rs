// Generated macro for impl_142 (impl)
macro_rules! Depcrate_schemaimpl_142 {
() => {
// Module: crate::schema
// Provides: {"impl_142"}
// Dependencies: {}
impl < T > BorshSchema for borrow :: Cow < '_ , T > where T : borrow :: ToOwned + ? Sized , T :: Owned : BorshSchema , { fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { < T :: Owned as BorshSchema > :: add_definitions_recursively (definitions) ; } fn declaration () -> Declaration { < T :: Owned as BorshSchema > :: declaration () } }
};
}
