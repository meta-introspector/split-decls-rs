/* FP:lib.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_lib_MOD_0001
/* FP:lib.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_lib_MOD_0002
/* FP:lib.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_lib_MOD_0003
/* FP:lib.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_lib_MOD_0004
/* FP:lib.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_lib_MOD_0005
/* FP:lib.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_lib_USE_0006
/* FP:lib.rs-0012 */ pub use idx :: { Idx , IntoSliceIdx } ;
/* FP:lib.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_lib_USE_0007
/* FP:lib.rs-0014 */ pub use crate :: rustc_index_macros :: newtype_index ;
/* FP:lib.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_lib_USE_0008
/* FP:lib.rs-0016 */ pub use slice :: IndexSlice ;
/* FP:lib.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_lib_USE_0009
/* FP:lib.rs-0018 */ # [doc (no_inline)] pub use vec :: IndexVec ;
/* FP:lib.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_lib_MACRO_0010
/* FP:lib.rs-0020 */ # [doc = " Type size assertion. The first argument is a type and the second argument is its expected size."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " Emitting hard errors from size assertions like this is generally not"] # [doc = " recommended, especially in libraries, because they can cause build failures if the layout"] # [doc = " algorithm or dependencies change. Here in rustc we control the toolchain and layout algorithm,"] # [doc = " so the former is not a problem. For the latter we have a lockfile as rustc is an application and"] # [doc = " precompiled library."] # [doc = ""] # [doc = " Short version: Don't copy this macro into your own code. Use a `#[test]` instead."] # [doc = ""] # [doc = " </div>"] # [macro_export] # [cfg (not (feature = "rustc_randomized_layouts"))] macro_rules ! static_assert_size { ($ ty : ty , $ size : expr) => { const _ : [() ; $ size] = [() ; :: std :: mem :: size_of ::<$ ty > ()] ; } ; }
/* FP:lib.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_lib_MACRO_0011
/* FP:lib.rs-0022 */ # [macro_export] # [cfg (feature = "rustc_randomized_layouts")] macro_rules ! static_assert_size { ($ ty : ty , $ size : expr) => { const _ : (usize , usize) = ($ size , :: std :: mem :: size_of ::<$ ty > ()) ; } ; }