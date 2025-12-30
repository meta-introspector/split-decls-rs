// Generated macro for PointerKind (enum)
macro_rules! DepcratePointerKind {
() => {
// Module: crate
// Provides: {"PointerKind"}
// Dependencies: {}
# [derive (Copy , Clone , PartialEq , Eq , Debug)] pub enum PointerKind { # [doc = " Shared reference. `frozen` indicates the absence of any `UnsafeCell`."] SharedRef { frozen : bool } , # [doc = " Mutable reference. `unpin` indicates the absence of any pinned data."] MutableRef { unpin : bool } , # [doc = " Box. `unpin` indicates the absence of any pinned data. `global` indicates whether this box"] # [doc = " uses the global allocator or a custom one."] Box { unpin : bool , global : bool } , }
};
}
