// Generated macro for sample_array (function)
macro_rules! Depcrate_tests_arraysample_array {
() => {
// Module: crate::tests::array
// Provides: {"sample_array"}
// Dependencies: {}
fn sample_array (len : usize) -> Retained < NSArray < NSObject > > { let mut vec = Vec :: with_capacity (len) ; for _ in 0 .. len { vec . push (NSObject :: new ()) ; } NSArray :: from_retained_slice (& vec) }
};
}
