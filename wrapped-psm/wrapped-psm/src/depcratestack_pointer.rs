// Generated macro for stack_pointer (function)
macro_rules! Depcratestack_pointer {
() => {
// Module: crate
// Provides: {"stack_pointer"}
// Dependencies: {}
# [doc = " Returns current stack pointer."] # [doc = ""] # [doc = " Note, that the stack pointer returned is from the perspective of the caller. From the"] # [doc = " perspective of `stack_pointer` function the pointer returned is the frame pointer."] # [doc = ""] # [doc = " While it is a goal to minimize the amount of stack used by this function, implementations for"] # [doc = " some targets may be unable to avoid allocating a stack frame. This makes this function"] # [doc = " suitable for stack exhaustion detection only in conjunction with sufficient padding."] # [doc = ""] # [doc = " Using `stack_pointer` to check for stack exhaustion is tricky to get right. It is impossible to"] # [doc = " know the callee’s frame size, therefore such value must be derived some other way. A common"] # [doc = " approach is to use stack padding (reserve enough stack space for any function to be called) and"] # [doc = " check against the padded threshold. If padding is chosen incorrectly, a situation similar to"] # [doc = " one described below may occur:"] # [doc = ""] # [doc = " 1. For stack exhaustion check, remaining stack is checked against `stack_pointer` with the"] # [doc = "    padding applied;"] # [doc = " 2. Callee allocates more stack than was accounted for with padding, and accesses pages outside"] # [doc = "    the stack, invalidating the execution (by e.g. crashing)."] # [cfg (asm)] pub fn stack_pointer () -> * mut u8 { unsafe { rust_psm_stack_pointer () } }
};
}
