// Generated macro for impl_for_renamed_primitives (macro)
macro_rules! Depcrate_schemaimpl_for_renamed_primitives {
() => {
// Module: crate::schema
// Provides: {"impl_for_renamed_primitives"}
// Dependencies: {}
macro_rules ! impl_for_renamed_primitives { ($ ($ ty : ty : $ name : ident => $ size : expr) ;+) => { $ (impl BorshSchema for $ ty { # [inline] fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { let definition = Definition :: Primitive ($ size) ; add_definition (Self :: declaration () , definition , definitions) ; } # [inline] fn declaration () -> Declaration { stringify ! ($ name) . into () } }) + } ; ($ ($ ty : ty : $ name : expr , $ size : expr) ;+) => { $ (impl BorshSchema for $ ty { # [inline] fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { let definition = Definition :: Primitive ($ size) ; add_definition (Self :: declaration () , definition , definitions) ; } # [inline] fn declaration () -> Declaration { $ name . into () } }) + } ; }
};
}
