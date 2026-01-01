/* FP:mono_item.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_mono_item_USE_0001
/* FP:mono_item.rs-0002 */ # [cfg (feature = "master")] use gccjit :: { FnAttribute , VarAttribute } ;
/* FP:mono_item.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_mono_item_USE_0002
/* FP:mono_item.rs-0004 */ use crate :: rustc_codegen_ssa :: traits :: PreDefineCodegenMethods ;
/* FP:mono_item.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_mono_item_USE_0003
/* FP:mono_item.rs-0006 */ use crate :: rustc_complete :: attrs :: Linkage ;
/* FP:mono_item.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_mono_item_USE_0004
/* FP:mono_item.rs-0008 */ use crate :: rustc_complete :: def :: DefKind ;
/* FP:mono_item.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_mono_item_USE_0005
/* FP:mono_item.rs-0010 */ use crate :: rustc_complete :: def_id :: { DefId , LOCAL_CRATE } ;
/* FP:mono_item.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_mono_item_USE_0006
/* FP:mono_item.rs-0012 */ use crate :: rustc_complete :: bug ;
/* FP:mono_item.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_mono_item_USE_0007
/* FP:mono_item.rs-0014 */ use crate :: rustc_complete :: middle :: codegen_fn_attrs :: CodegenFnAttrFlags ;
/* FP:mono_item.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_mono_item_USE_0008
/* FP:mono_item.rs-0016 */ use crate :: rustc_complete :: mir :: mono :: Visibility ;
/* FP:mono_item.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_mono_item_USE_0009
/* FP:mono_item.rs-0018 */ use crate :: rustc_complete :: ty :: layout :: { FnAbiOf , HasTypingEnv , LayoutOf } ;
/* FP:mono_item.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_mono_item_USE_0010
/* FP:mono_item.rs-0020 */ use crate :: rustc_complete :: ty :: { self , Instance , TypeVisitableExt } ;
/* FP:mono_item.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_mono_item_USE_0011
/* FP:mono_item.rs-0022 */ use crate :: context :: CodegenCx ;
/* FP:mono_item.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_mono_item_USE_0012
/* FP:mono_item.rs-0024 */ use crate :: type_of :: LayoutGccExt ;
/* FP:mono_item.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_mono_item_USE_0013
/* FP:mono_item.rs-0026 */ use crate :: { attributes , base } ;
/* FP:mono_item.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_mono_item_IMPL_0014
/* FP:mono_item.rs-0028 */ impl < 'gcc , 'tcx > PreDefineCodegenMethods < 'tcx > for CodegenCx < 'gcc , 'tcx > { # [cfg_attr (not (feature = "master") , allow (unused_variables))] fn predefine_static (& mut self , def_id : DefId , _linkage : Linkage , visibility : Visibility , symbol_name : & str ,) { let attrs = self . tcx . codegen_fn_attrs (def_id) ; let instance = Instance :: mono (self . tcx , def_id) ; let DefKind :: Static { nested , .. } = self . tcx . def_kind (def_id) else { bug ! () } ; let ty = if nested { self . tcx . types . unit } else { instance . ty (self . tcx , self . typing_env ()) } ; let gcc_type = self . layout_of (ty) . gcc_type (self) ; let is_tls = attrs . flags . contains (CodegenFnAttrFlags :: THREAD_LOCAL) ; let global = self . define_global (symbol_name , gcc_type , is_tls , attrs . link_section) ; # [cfg (feature = "master")] global . add_attribute (VarAttribute :: Visibility (base :: visibility_to_gcc (visibility))) ; self . instances . borrow_mut () . insert (instance , global) ; } # [cfg_attr (not (feature = "master") , allow (unused_variables))] fn predefine_fn (& mut self , instance : Instance < 'tcx > , linkage : Linkage , visibility : Visibility , symbol_name : & str ,) { assert ! (! instance . args . has_infer ()) ; let fn_abi = self . fn_abi_of_instance (instance , ty :: List :: empty ()) ; self . linkage . set (base :: linkage_to_gcc (linkage)) ; let decl = self . declare_fn (symbol_name , fn_abi) ; attributes :: from_fn_attrs (self , decl , instance) ; if linkage != Linkage :: Internal && self . tcx . is_compiler_builtins (LOCAL_CRATE) { # [cfg (feature = "master")] decl . add_attribute (FnAttribute :: Visibility (gccjit :: Visibility :: Hidden)) ; } else if visibility != Visibility :: Default { # [cfg (feature = "master")] decl . add_attribute (FnAttribute :: Visibility (base :: visibility_to_gcc (visibility))) ; } self . functions . borrow_mut () . insert (symbol_name . to_string () , decl) ; self . function_instances . borrow_mut () . insert (instance , decl) ; } }