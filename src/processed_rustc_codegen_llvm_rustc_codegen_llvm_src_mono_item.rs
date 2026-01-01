/* FP:mono_item.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_mono_item_USE_0001
/* FP:mono_item.rs-0002 */ use crate :: rustc_codegen_ssa :: traits :: * ;
/* FP:mono_item.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_mono_item_USE_0002
/* FP:mono_item.rs-0004 */ use crate :: rustc_complete :: attrs :: Linkage ;
/* FP:mono_item.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_mono_item_USE_0003
/* FP:mono_item.rs-0006 */ use crate :: rustc_complete :: def :: DefKind ;
/* FP:mono_item.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_mono_item_USE_0004
/* FP:mono_item.rs-0008 */ use crate :: rustc_complete :: def_id :: { DefId , LOCAL_CRATE } ;
/* FP:mono_item.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_mono_item_USE_0005
/* FP:mono_item.rs-0010 */ use crate :: rustc_complete :: bug ;
/* FP:mono_item.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_mono_item_USE_0006
/* FP:mono_item.rs-0012 */ use crate :: rustc_complete :: mir :: mono :: Visibility ;
/* FP:mono_item.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_mono_item_USE_0007
/* FP:mono_item.rs-0014 */ use crate :: rustc_complete :: ty :: layout :: { FnAbiOf , HasTypingEnv , LayoutOf } ;
/* FP:mono_item.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_mono_item_USE_0008
/* FP:mono_item.rs-0016 */ use crate :: rustc_complete :: ty :: { self , Instance , TypeVisitableExt } ;
/* FP:mono_item.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_mono_item_USE_0009
/* FP:mono_item.rs-0018 */ use crate :: rustc_complete :: config :: CrateType ;
/* FP:mono_item.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_mono_item_USE_0010
/* FP:mono_item.rs-0020 */ use crate :: rustc_target :: spec :: RelocModel ;
/* FP:mono_item.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_mono_item_USE_0011
/* FP:mono_item.rs-0022 */ use tracing :: debug ;
/* FP:mono_item.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_mono_item_USE_0012
/* FP:mono_item.rs-0024 */ use crate :: context :: CodegenCx ;
/* FP:mono_item.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_mono_item_USE_0013
/* FP:mono_item.rs-0026 */ use crate :: errors :: SymbolAlreadyDefined ;
/* FP:mono_item.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_mono_item_USE_0014
/* FP:mono_item.rs-0028 */ use crate :: type_of :: LayoutLlvmExt ;
/* FP:mono_item.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_mono_item_USE_0015
/* FP:mono_item.rs-0030 */ use crate :: { base , llvm } ;
/* FP:mono_item.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_mono_item_IMPL_0016
/* FP:mono_item.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_mono_item_IMPL_0017
/* FP:mono_item.rs-0034 */ impl CodegenCx < '_ , '_ > { # [doc = " Whether a definition or declaration can be assumed to be local to a group of"] # [doc = " libraries that form a single DSO or executable."] # [doc = " Marks the local as DSO if so."] pub (crate) fn assume_dso_local (& self , llval : & llvm :: Value , is_declaration : bool) -> bool { let assume = self . should_assume_dso_local (llval , is_declaration) ; if assume { llvm :: set_dso_local (llval) ; } assume } fn should_assume_dso_local (& self , llval : & llvm :: Value , is_declaration : bool) -> bool { let linkage = llvm :: get_linkage (llval) ; let visibility = llvm :: get_visibility (llval) ; if matches ! (linkage , llvm :: Linkage :: InternalLinkage | llvm :: Linkage :: PrivateLinkage) { return true ; } if visibility != llvm :: Visibility :: Default && linkage != llvm :: Linkage :: ExternalWeakLinkage { return true ; } let all_exe = self . tcx . crate_types () . iter () . all (| ty | * ty == CrateType :: Executable) ; let is_declaration_for_linker = is_declaration || linkage == llvm :: Linkage :: AvailableExternallyLinkage ; if all_exe && ! is_declaration_for_linker { return true ; } if matches ! (&* self . tcx . sess . target . arch , "powerpc64" | "powerpc64le") { return false ; } if self . tcx . sess . target . is_like_darwin { return false ; } if self . tcx . sess . relocation_model () == RelocModel :: Pie && ! is_declaration { return true ; } let is_thread_local_var = llvm :: LLVMIsAGlobalVariable (llval) . is_some_and (| v | llvm :: LLVMIsThreadLocal (v) . is_true ()) ; if is_thread_local_var { return false ; } if let Some (direct) = self . tcx . sess . direct_access_external_data () { return direct ; } self . tcx . sess . relocation_model () == RelocModel :: Static } }