/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_profiling_tests_USE_0001
/* FP:tests.rs-0002 */ use super :: JsonTimePassesEntry ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_profiling_tests_FN_0002
/* FP:tests.rs-0004 */ # [test] fn with_rss () { let entry = JsonTimePassesEntry { pass : "typeck" , time : 56.1 , start_rss : Some (10) , end_rss : Some (20) } ; assert_eq ! (entry . to_string () , r#"{"pass":"typeck","time":56.1,"rss_start":10,"rss_end":20}"#) }
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_profiling_tests_FN_0003
/* FP:tests.rs-0006 */ # [test] fn no_rss () { let entry = JsonTimePassesEntry { pass : "typeck" , time : 56.1 , start_rss : None , end_rss : None } ; assert_eq ! (entry . to_string () , r#"{"pass":"typeck","time":56.1,"rss_start":null,"rss_end":null}"#) }