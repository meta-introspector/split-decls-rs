// Generated macro for impl_for_primitives (macro)
macro_rules! Depcrate_schemaimpl_for_primitives {
() => {
// Module: crate::schema
// Provides: {"impl_for_primitives"}
// Dependencies: {}
macro_rules ! impl_for_primitives { ($ ($ ty : ident => $ size : expr) ;+) => { impl_for_renamed_primitives ! { $ ($ ty : $ ty => $ size) ;+ } } ; }
};
}
