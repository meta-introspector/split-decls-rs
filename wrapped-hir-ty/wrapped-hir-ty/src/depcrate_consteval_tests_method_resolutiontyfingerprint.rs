// Generated macro for TyFingerprint (enum)
macro_rules! Depcrate_consteval_tests_method_resolutionTyFingerprint {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"TyFingerprint"}
// Dependencies: {}
# [doc = " This is used as a key for indexing impls."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] pub enum TyFingerprint { Str , Slice , Array , Never , RawPtr (Mutability) , Scalar (Scalar) , Adt (hir_def :: AdtId) , Dyn (TraitId) , ForeignType (ForeignDefId) , Unit , Unnameable , Function (u32) , }
};
}
