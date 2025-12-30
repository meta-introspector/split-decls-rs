// Generated macro for IterUncheckedWithBackingEnum (struct)
macro_rules! Depcrate_iterIterUncheckedWithBackingEnum {
() => {
// Module: crate::iter
// Provides: {"IterUncheckedWithBackingEnum"}
// Dependencies: {}
# [derive (Debug , PartialEq)] pub (crate) struct IterUncheckedWithBackingEnum < 'a , C : ? Sized + 'a , E : ? Sized + 'a > { helper : FastEnumeratorHelper , # [doc = " 'a and C are covariant."] collection : & 'a C , # [doc = " E is covariant."] enumerator : Retained < E > , # [cfg (debug_assertions)] mutations_state : MutationState , }
};
}
