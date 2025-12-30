// Generated macro for Iter (struct)
macro_rules! Depcrate_iterIter {
() => {
// Module: crate::iter
// Provides: {"Iter"}
// Dependencies: {}
# [derive (Debug , PartialEq)] pub (crate) struct Iter < 'a , C : ? Sized + 'a > { helper : FastEnumeratorHelper , # [doc = " 'a and C are covariant."] collection : & 'a C , mutations_state : MutationState , }
};
}
