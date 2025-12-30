// Generated macro for impl_3041 (impl)
macro_rules! Depcrate_proptestimpl_3041 {
() => {
// Module: crate::proptest
// Provides: {"impl_3041"}
// Dependencies: {}
impl < T , R , C > ValueTree for MatrixValueTree < T , R , C > where T : Scalar , R : Dim , C : Dim , DefaultAllocator : Allocator < R , C > , { type Value = OMatrix < T , R , C > ; fn current (& self) -> Self :: Value { self . value_tree . current () } fn simplify (& mut self) -> bool { self . value_tree . simplify () } fn complicate (& mut self) -> bool { self . value_tree . complicate () } }
};
}
