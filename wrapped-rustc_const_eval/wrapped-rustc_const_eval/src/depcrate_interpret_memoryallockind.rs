// Generated macro for AllocKind (enum)
macro_rules! Depcrate_interpret_memoryAllocKind {
() => {
// Module: crate::interpret::memory
// Provides: {"AllocKind"}
// Dependencies: {}
# [doc = " The return value of `get_alloc_info` indicates the \"kind\" of the allocation."] # [derive (Copy , Clone , PartialEq , Debug)] pub enum AllocKind { # [doc = " A regular live data allocation."] LiveData , # [doc = " A function allocation (that fn ptrs point to)."] Function , # [doc = " A vtable allocation."] VTable , # [doc = " A TypeId allocation."] TypeId , # [doc = " A dead allocation."] Dead , }
};
}
