// Generated macro for ProtectorKind (enum)
macro_rules! Depcrate_borrow_trackerProtectorKind {
() => {
// Module: crate::borrow_tracker
// Provides: {"ProtectorKind"}
// Dependencies: {}
# [doc = " The flavor of the protector."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum ProtectorKind { # [doc = " Protected against aliasing violations from other pointers."] # [doc = ""] # [doc = " Items protected like this cause UB when they are invalidated, *but* the pointer itself may"] # [doc = " still be used to issue a deallocation."] # [doc = ""] # [doc = " This is required for LLVM IR pointers that are `noalias` but *not* `dereferenceable`."] WeakProtector , # [doc = " Protected against any kind of invalidation."] # [doc = ""] # [doc = " Items protected like this cause UB when they are invalidated or the memory is deallocated."] # [doc = " This is strictly stronger protection than `WeakProtector`."] # [doc = ""] # [doc = " This is required for LLVM IR pointers that are `dereferenceable` (and also allows `noalias`)."] StrongProtector , }
};
}
