// Generated macro for IterWithBackingEnum (struct)
macro_rules! Depcrate_iterIterWithBackingEnum {
() => {
// Module: crate::iter
// Provides: {"IterWithBackingEnum"}
// Dependencies: {}
# [derive (Debug , PartialEq)] pub (crate) struct IterWithBackingEnum < 'a , C : ? Sized + 'a , E : ? Sized + 'a > { helper : FastEnumeratorHelper , # [doc = " 'a and C are covariant."] collection : & 'a C , # [doc = " E is covariant."] enumerator : Retained < E > , mutations_state : MutationState , }
};
}
