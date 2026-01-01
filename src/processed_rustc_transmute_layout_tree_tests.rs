/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_layout_tree_tests_USE_0001
/* FP:tests.rs-0002 */ use super :: Tree ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_layout_tree_tests_ENUM_0002
/* FP:tests.rs-0004 */ # [derive (Debug , Hash , Eq , PartialEq , Clone , Copy)] enum Def { NoSafetyInvariants , HasSafetyInvariants , }
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_layout_tree_tests_IMPL_0003
/* FP:tests.rs-0006 */ impl super :: Def for Def { fn has_safety_invariants (& self) -> bool { self == & Self :: HasSafetyInvariants } }
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_layout_tree_tests_MOD_0004