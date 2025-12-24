use introspector_decl2_macros::decl_module;
decl_module!(
    wrapped_test_crate_decls_hello,
    wrapped_test_crate_decls_TestStruct,
    wrapped_test_crate_decls_TestEnum
);
