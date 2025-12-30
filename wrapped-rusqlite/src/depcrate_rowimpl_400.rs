// Generated macro for impl_400 (impl)
macro_rules! Depcrate_rowimpl_400 {
() => {
// Module: crate::row
// Provides: {"impl_400"}
// Dependencies: {}
impl RowIndex for & '_ str { # [inline] fn idx (& self , stmt : & Statement < '_ >) -> Result < usize > { stmt . column_index (self) } }
};
}
