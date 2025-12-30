// Generated macro for sample_number_array (function)
macro_rules! Depcrate_tests_arraysample_number_array {
() => {
// Module: crate::tests::array
// Provides: {"sample_number_array"}
// Dependencies: {}
fn sample_number_array (len : u8) -> Retained < NSArray < NSNumber > > { let mut vec = Vec :: with_capacity (len as usize) ; for i in 0 .. len { vec . push (NSNumber :: new_u8 (i)) ; } NSArray :: from_retained_slice (& vec) }
};
}
