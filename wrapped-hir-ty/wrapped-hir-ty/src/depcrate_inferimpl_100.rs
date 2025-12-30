// Generated macro for impl_100 (impl)
macro_rules! Depcrate_inferimpl_100 {
() => {
// Module: crate::infer
// Provides: {"impl_100"}
// Dependencies: {}
impl From < AutoBorrowMutability > for Mutability { fn from (m : AutoBorrowMutability) -> Self { match m { AutoBorrowMutability :: Mut { .. } => Mutability :: Mut , AutoBorrowMutability :: Not => Mutability :: Not , } } }
};
}
