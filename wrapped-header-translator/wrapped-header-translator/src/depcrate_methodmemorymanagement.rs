// Generated macro for MemoryManagement (enum)
macro_rules! Depcrate_methodMemoryManagement {
() => {
// Module: crate::method
// Provides: {"MemoryManagement"}
// Dependencies: {}
# [doc = " The retain semantics calling convention for a method."] # [doc = ""] # [doc = " This also encodes the \"method family\" that a method belongs to."] # [derive (Debug , PartialEq , Eq , Hash , Clone , Copy)] pub enum MemoryManagement { RetainedAlloc , RetainedCopy { returns_not_retained : bool } , RetainedMutableCopy { returns_not_retained : bool } , RetainedNew { returns_not_retained : bool } , RetainedInit , RetainedNone { returns_retained : bool } , InnerPointer , Normal , }
};
}
