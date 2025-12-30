// Generated macro for Array (struct)
macro_rules! Depcrate_tests_helperArray {
() => {
// Module: crate::tests::helper
// Provides: {"Array"}
// Dependencies: {}
pub (crate) struct Array < T : Primitive > { arr : Box < Align16 < [AtomicMaybeUninit < T > ; 10] > > , base : T , idx : usize , }
};
}
