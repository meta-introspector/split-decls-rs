// Generated macro for size_asserts (module)
macro_rules! Depcrate_tokensize_asserts {
() => {
// Module: crate::token
// Provides: {"size_asserts"}
// Dependencies: {}
# [cfg (target_pointer_width = "64")] mod size_asserts { use rustc_data_structures :: static_assert_size ; use super :: * ; static_assert_size ! (Lit , 12) ; static_assert_size ! (LitKind , 2) ; static_assert_size ! (Token , 24) ; static_assert_size ! (TokenKind , 16) ; }
};
}
