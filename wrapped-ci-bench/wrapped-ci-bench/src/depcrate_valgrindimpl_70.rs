// Generated macro for impl_70 (impl)
macro_rules! Depcrate_valgrindimpl_70 {
() => {
// Module: crate::valgrind
// Provides: {"impl_70"}
// Dependencies: {}
impl Sub for InstructionCounts { type Output = Self ; fn sub (self , rhs : Self) -> Self :: Output { Self { client : self . client - rhs . client , server : self . server - rhs . server , } } }
};
}
