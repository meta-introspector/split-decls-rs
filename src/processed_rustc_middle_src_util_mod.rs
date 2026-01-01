/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_util_mod_MOD_0001
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_util_mod_STRUCT_0002
/* FP:mod.rs-0004 */ # [derive (Default , Copy , Clone)] pub struct Providers { pub queries : crate :: query :: Providers , pub extern_queries : crate :: query :: ExternProviders , pub hooks : crate :: hooks :: Providers , }
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_util_mod_IMPL_0003
/* FP:mod.rs-0006 */ # [doc = " Backwards compatibility hack to keep the diff small. This"] # [doc = " gives direct access to the `queries` field's fields, which"] # [doc = " are what almost everything wants access to."] impl std :: ops :: DerefMut for Providers { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . queries } }
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_util_mod_IMPL_0004
/* FP:mod.rs-0008 */ impl std :: ops :: Deref for Providers { type Target = crate :: query :: Providers ; fn deref (& self) -> & Self :: Target { & self . queries } }