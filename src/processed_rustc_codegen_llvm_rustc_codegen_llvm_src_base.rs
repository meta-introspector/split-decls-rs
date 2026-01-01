/* FP:base.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_USE_0001
/* FP:base.rs-0002 */ use std :: time :: Instant ;
/* FP:base.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_USE_0002
/* FP:base.rs-0004 */ use crate :: rustc_codegen_ssa :: ModuleCodegen ;
/* FP:base.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_USE_0003
/* FP:base.rs-0006 */ use crate :: rustc_codegen_ssa :: base :: maybe_create_entry_wrapper ;
/* FP:base.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_USE_0004
/* FP:base.rs-0008 */ use crate :: rustc_codegen_ssa :: mono_item :: MonoItemExt ;
/* FP:base.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_USE_0005
/* FP:base.rs-0010 */ use crate :: rustc_codegen_ssa :: traits :: * ;
/* FP:base.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_USE_0006
/* FP:base.rs-0012 */ use crate :: rustc_data_structures :: small_c_str :: SmallCStr ;
/* FP:base.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_USE_0007
/* FP:base.rs-0014 */ use crate :: rustc_complete :: attrs :: Linkage ;
/* FP:base.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_USE_0008
/* FP:base.rs-0016 */ use crate :: rustc_complete :: dep_graph ;
/* FP:base.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_USE_0009
/* FP:base.rs-0018 */ use crate :: rustc_complete :: middle :: codegen_fn_attrs :: CodegenFnAttrs ;
/* FP:base.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_USE_0010
/* FP:base.rs-0020 */ use crate :: rustc_complete :: mir :: mono :: Visibility ;
/* FP:base.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_USE_0011
/* FP:base.rs-0022 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:base.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_USE_0012
/* FP:base.rs-0024 */ use crate :: rustc_complete :: config :: DebugInfo ;
/* FP:base.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_USE_0013
/* FP:base.rs-0026 */ use crate :: rustc_complete :: Symbol ;
/* FP:base.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_USE_0014
/* FP:base.rs-0028 */ use crate :: rustc_target :: spec :: SanitizerSet ;
/* FP:base.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_USE_0015
/* FP:base.rs-0030 */ use super :: ModuleLlvm ;
/* FP:base.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_USE_0016
/* FP:base.rs-0032 */ use crate :: builder :: Builder ;
/* FP:base.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_USE_0017
/* FP:base.rs-0034 */ use crate :: context :: CodegenCx ;
/* FP:base.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_USE_0018
/* FP:base.rs-0036 */ use crate :: value :: Value ;
/* FP:base.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_USE_0019
/* FP:base.rs-0038 */ use crate :: { attributes , llvm } ;
/* FP:base.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_STRUCT_0020
/* FP:base.rs-0040 */ pub (crate) struct ValueIter < 'll > { cur : Option < & 'll Value > , step : unsafe extern "C" fn (& 'll Value) -> Option < & 'll Value > , }
/* FP:base.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_IMPL_0021
/* FP:base.rs-0042 */ impl < 'll > Iterator for ValueIter < 'll > { type Item = & 'll Value ; fn next (& mut self) -> Option < & 'll Value > { let old = self . cur ; if let Some (old) = old { self . cur = unsafe { (self . step) (old) } ; } old } }
/* FP:base.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_FN_0022
/* FP:base.rs-0044 */ pub (crate) fn iter_globals (llmod : & llvm :: Module) -> ValueIter < '_ > { unsafe { ValueIter { cur : llvm :: LLVMGetFirstGlobal (llmod) , step : llvm :: LLVMGetNextGlobal } } }
/* FP:base.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_FN_0023
/* FP:base.rs-0046 */ pub (crate) fn compile_codegen_unit (tcx : TyCtxt < '_ > , cgu_name : Symbol ,) -> (ModuleCodegen < ModuleLlvm > , u64) { let start_time = Instant :: now () ; let dep_node = tcx . codegen_unit (cgu_name) . codegen_dep_node (tcx) ; let (module , _) = tcx . dep_graph . with_task (dep_node , tcx , cgu_name , module_codegen , Some (dep_graph :: hash_result) ,) ; let time_to_codegen = start_time . elapsed () ; let cost = time_to_codegen . as_nanos () as u64 ; fn module_codegen (tcx : TyCtxt < '_ > , cgu_name : Symbol) -> ModuleCodegen < ModuleLlvm > { let cgu = tcx . codegen_unit (cgu_name) ; let _prof_timer = tcx . prof . generic_activity_with_arg_recorder ("codegen_module" , | recorder | { recorder . record_arg (cgu_name . to_string ()) ; recorder . record_arg (cgu . size_estimate () . to_string ()) ; }) ; let llvm_module = ModuleLlvm :: new (tcx , cgu_name . as_str ()) ; { let mut cx = CodegenCx :: new (tcx , cgu , & llvm_module) ; let mono_items = cx . codegen_unit . items_in_deterministic_order (cx . tcx) ; for & (mono_item , data) in & mono_items { mono_item . predefine :: < Builder < '_ , '_ , '_ > > (& mut cx , cgu_name . as_str () , data . linkage , data . visibility ,) ; } for & (mono_item , item_data) in & mono_items { mono_item . define :: < Builder < '_ , '_ , '_ > > (& mut cx , cgu_name . as_str () , item_data) ; } if let Some (entry) = maybe_create_entry_wrapper :: < Builder < '_ , '_ , '_ > > (& cx , cx . codegen_unit) { let attrs = attributes :: sanitize_attrs (& cx , SanitizerSet :: empty ()) ; attributes :: apply_to_llfn (entry , llvm :: AttributePlace :: Function , & attrs) ; } if cx . sess () . instrument_coverage () { cx . coverageinfo_finalize () ; } if ! cx . used_statics . is_empty () { cx . create_used_variable_impl (c"llvm.used" , & cx . used_statics) ; } if ! cx . compiler_used_statics . is_empty () { cx . create_used_variable_impl (c"llvm.compiler.used" , & cx . compiler_used_statics) ; } for & (old_g , new_g) in cx . statics_to_rauw () . borrow () . iter () { unsafe { llvm :: LLVMReplaceAllUsesWith (old_g , new_g) ; llvm :: LLVMDeleteGlobal (old_g) ; } } if cx . sess () . opts . debuginfo != DebugInfo :: None { cx . debuginfo_finalize () ; } } ModuleCodegen :: new_regular (cgu_name . to_string () , llvm_module) } (module , cost) }
/* FP:base.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_FN_0024
/* FP:base.rs-0048 */ pub (crate) fn set_link_section (llval : & Value , attrs : & CodegenFnAttrs) { let Some (sect) = attrs . link_section else { return } ; let buf = SmallCStr :: new (sect . as_str ()) ; llvm :: set_section (llval , & buf) ; }
/* FP:base.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_FN_0025
/* FP:base.rs-0050 */ pub (crate) fn linkage_to_llvm (linkage : Linkage) -> llvm :: Linkage { match linkage { Linkage :: External => llvm :: Linkage :: ExternalLinkage , Linkage :: AvailableExternally => llvm :: Linkage :: AvailableExternallyLinkage , Linkage :: LinkOnceAny => llvm :: Linkage :: LinkOnceAnyLinkage , Linkage :: LinkOnceODR => llvm :: Linkage :: LinkOnceODRLinkage , Linkage :: WeakAny => llvm :: Linkage :: WeakAnyLinkage , Linkage :: WeakODR => llvm :: Linkage :: WeakODRLinkage , Linkage :: Internal => llvm :: Linkage :: InternalLinkage , Linkage :: ExternalWeak => llvm :: Linkage :: ExternalWeakLinkage , Linkage :: Common => llvm :: Linkage :: CommonLinkage , } }
/* FP:base.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_FN_0026
/* FP:base.rs-0052 */ pub (crate) fn visibility_to_llvm (linkage : Visibility) -> llvm :: Visibility { match linkage { Visibility :: Default => llvm :: Visibility :: Default , Visibility :: Hidden => llvm :: Visibility :: Hidden , Visibility :: Protected => llvm :: Visibility :: Protected , } }
/* FP:base.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_base_FN_0027
/* FP:base.rs-0054 */ pub (crate) fn set_variable_sanitizer_attrs (llval : & Value , attrs : & CodegenFnAttrs) { if attrs . no_sanitize . contains (SanitizerSet :: ADDRESS) { unsafe { llvm :: LLVMRustSetNoSanitizeAddress (llval) } ; } if attrs . no_sanitize . contains (SanitizerSet :: HWADDRESS) { unsafe { llvm :: LLVMRustSetNoSanitizeHWAddress (llval) } ; } }