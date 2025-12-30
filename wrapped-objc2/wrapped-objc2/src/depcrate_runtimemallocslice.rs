// Generated macro for MallocSlice (macro)
macro_rules! Depcrate_runtimeMallocSlice {
() => {
// Module: crate::runtime
// Provides: {"MallocSlice"}
// Dependencies: {}
# [doc = " We do not want to expose `MallocSlice` to end users, because in the"] # [doc = " future, we want to be able to change it to `Box<[T], MallocAllocator>`."] # [doc = ""] # [doc = " So instead we use an unnameable type."] macro_rules ! MallocSlice { ($ t : ty) => { impl std :: ops :: Deref < Target = [$ t] > + AsRef < [$ t] > + std :: fmt :: Debug } ; }
};
}
