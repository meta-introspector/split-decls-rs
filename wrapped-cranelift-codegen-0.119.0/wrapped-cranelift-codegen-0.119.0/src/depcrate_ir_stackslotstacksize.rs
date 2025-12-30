// Generated macro for StackSize (type)
macro_rules! Depcrate_ir_stackslotStackSize {
() => {
// Module: crate::ir::stackslot
// Provides: {"StackSize"}
// Dependencies: {}
# [doc = " The size of an object on the stack, or the size of a stack frame."] # [doc = ""] # [doc = " We don't use `usize` to represent object sizes on the target platform because Cranelift supports"] # [doc = " cross-compilation, and `usize` is a type that depends on the host platform, not the target"] # [doc = " platform."] pub type StackSize = u32 ;
};
}
