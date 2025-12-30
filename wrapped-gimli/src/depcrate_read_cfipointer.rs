// Generated macro for Pointer (enum)
macro_rules! Depcrate_read_cfiPointer {
() => {
// Module: crate::read::cfi
// Provides: {"Pointer"}
// Dependencies: {}
# [doc = " A decoded pointer."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum Pointer { # [doc = " This value is the decoded pointer value."] Direct (u64) , # [doc = " This value is *not* the pointer value, but points to the address of"] # [doc = " where the real pointer value lives. In other words, deref this pointer"] # [doc = " to get the real pointer value."] # [doc = ""] # [doc = " Chase this pointer at your own risk: do you trust the DWARF data it came"] # [doc = " from?"] Indirect (u64) , }
};
}
