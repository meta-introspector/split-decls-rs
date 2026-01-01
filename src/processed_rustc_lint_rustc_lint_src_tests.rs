/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_tests_USE_0001
/* FP:tests.rs-0002 */ # [allow (rustc :: symbol_intern_string_literal)] use crate :: rustc_complete :: { Symbol , create_default_session_globals_then } ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_tests_USE_0002
/* FP:tests.rs-0004 */ use crate :: levels :: parse_lint_and_tool_name ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_tests_FN_0003
/* FP:tests.rs-0006 */ # [test] fn parse_lint_no_tool () { create_default_session_globals_then (| | { assert_eq ! (parse_lint_and_tool_name ("foo") , (None , "foo")) }) ; }
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_tests_FN_0004
/* FP:tests.rs-0008 */ # [test] fn parse_lint_with_tool () { create_default_session_globals_then (| | { assert_eq ! (parse_lint_and_tool_name ("clippy::foo") , (Some (Symbol :: intern ("clippy")) , "foo")) }) ; }
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_tests_FN_0005
/* FP:tests.rs-0010 */ # [test] fn parse_lint_multiple_path () { create_default_session_globals_then (| | { assert_eq ! (parse_lint_and_tool_name ("clippy::foo::bar") , (Some (Symbol :: intern ("clippy")) , "foo::bar")) }) ; }