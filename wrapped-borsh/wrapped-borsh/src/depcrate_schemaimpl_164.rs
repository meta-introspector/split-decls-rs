// Generated macro for impl_164 (impl)
macro_rules! Depcrate_schemaimpl_164 {
() => {
// Module: crate::schema
// Provides: {"impl_164"}
// Dependencies: {}
impl BorshSchema for core :: ops :: RangeFull { # [inline] fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { let fields = Fields :: Empty ; let def = Definition :: Struct { fields } ; add_definition (Self :: declaration () , def , definitions) ; } # [inline] fn declaration () -> Declaration { "RangeFull" . into () } }
};
}
