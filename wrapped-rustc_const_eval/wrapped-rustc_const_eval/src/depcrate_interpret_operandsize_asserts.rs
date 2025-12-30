// Generated macro for size_asserts (module)
macro_rules! Depcrate_interpret_operandsize_asserts {
() => {
// Module: crate::interpret::operand
// Provides: {"size_asserts"}
// Dependencies: {}
# [cfg (target_pointer_width = "64")] mod size_asserts { use rustc_data_structures :: static_assert_size ; use super :: * ; static_assert_size ! (ImmTy <'_ >, 64) ; static_assert_size ! (Immediate , 48) ; static_assert_size ! (OpTy <'_ >, 72) ; static_assert_size ! (Operand , 56) ; }
};
}
