// Generated macro for impl_399 (impl)
macro_rules! Depcrate_rowimpl_399 {
() => {
// Module: crate::row
// Provides: {"impl_399"}
// Dependencies: {}
impl RowIndex for usize { # [inline] fn idx (& self , stmt : & Statement < '_ >) -> Result < usize > { if * self >= stmt . column_count () { Err (Error :: InvalidColumnIndex (* self)) } else { Ok (* self) } } }
};
}
