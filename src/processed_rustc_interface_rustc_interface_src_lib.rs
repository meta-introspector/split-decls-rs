/* FP:lib.rs-0001 */ // tidy-alphabetical-start
/* FP:lib.rs-0002 */ #[feature(decl_macro)]
/* FP:lib.rs-0003 */ #[feature(file_buffered)]
/* FP:lib.rs-0004 */ #[feature(iter_intersperse)]
/* FP:lib.rs-0005 */ #[feature(try_blocks)]
/* FP:lib.rs-0006 */ // tidy-alphabetical-end
/* FP:lib.rs-0007 */ 
/* FP:lib.rs-0016 */ 
/* FP:lib.rs-0017 */ pub use callbacks::setup_callbacks;
/* FP:lib.rs-0018 */ pub use interface::{Config, run_compiler};
/* FP:lib.rs-0019 */ pub use passes::{DEFAULT_QUERY_PROVIDERS, create_and_enter_global_ctxt, parse};
/* FP:lib.rs-0020 */ pub use queries::Linker;
/* FP:lib.rs-0021 */ 
/* FP:lib.rs-0022 */ #[cfg(test)]
/* FP:lib.rs-0024 */ 
/* FP:lib.rs-0025 */ rustc_fluent_macro::fluent_messages! { "../messages.ftl" }