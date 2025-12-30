// Generated macro for IterUnchecked (struct)
macro_rules! Depcrate_iterIterUnchecked {
() => {
// Module: crate::iter
// Provides: {"IterUnchecked"}
// Dependencies: {}
# [derive (Debug , PartialEq)] pub (crate) struct IterUnchecked < 'a , C : ? Sized + 'a > { helper : FastEnumeratorHelper , # [doc = " 'a and C are covariant."] collection : & 'a C , # [cfg (debug_assertions)] mutations_state : MutationState , }
};
}
