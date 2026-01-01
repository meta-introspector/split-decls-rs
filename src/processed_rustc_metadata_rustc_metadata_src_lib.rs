/* FP:lib.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_lib_USE_0001
/* FP:lib.rs-0002 */ # [allow (internal_features)] # [doc (html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/")] # [doc (rust_logo)] # [feature (decl_macro)] # [feature (error_iter)] # [feature (file_buffered)] # [feature (gen_blocks)] # [feature (if_let_guard)] # [feature (macro_metavar_expr)] # [feature (min_specialization)] # [feature (never_type)] # [feature (proc_macro_internals)] # [feature (rustdoc_internals)] # [feature (trusted_len)] pub use rmeta :: provide ;
/* FP:lib.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_lib_MOD_0002
/* FP:lib.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_lib_MOD_0003
/* FP:lib.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_lib_MOD_0004
/* FP:lib.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_lib_MOD_0005
/* FP:lib.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_lib_MOD_0006
/* FP:lib.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_lib_MOD_0007
/* FP:lib.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_lib_MOD_0008
/* FP:lib.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_lib_MOD_0009
/* FP:lib.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_lib_USE_0010
/* FP:lib.rs-0020 */ pub use creader :: { DylibError , load_symbol_from_dylib } ;
/* FP:lib.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_lib_USE_0011
/* FP:lib.rs-0022 */ pub use fs :: { METADATA_FILENAME , emit_wrapper_file } ;
/* FP:lib.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_lib_USE_0012
/* FP:lib.rs-0024 */ pub use native_libs :: { NativeLibSearchFallback , find_native_static_library , try_find_native_dynamic_library , try_find_native_static_library , walk_native_lib_search_dirs , } ;
/* FP:lib.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_lib_USE_0013
/* FP:lib.rs-0026 */ pub use rmeta :: { EncodedMetadata , METADATA_HEADER , encode_metadata , rendered_const } ;
/* FP:lib.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_lib_MACRO_0014
/* FP:lib.rs-0028 */ rustc_fluent_macro :: fluent_messages ! { "../messages.ftl" }