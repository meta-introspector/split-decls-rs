/* FP:context.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0001
/* FP:context.rs-0002 */ use std :: borrow :: { Borrow , Cow } ;
/* FP:context.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0002
/* FP:context.rs-0004 */ use std :: cell :: { Cell , RefCell } ;
/* FP:context.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0003
/* FP:context.rs-0006 */ use std :: ffi :: { CStr , c_char , c_uint } ;
/* FP:context.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0004
/* FP:context.rs-0008 */ use std :: marker :: PhantomData ;
/* FP:context.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0005
/* FP:context.rs-0010 */ use std :: ops :: { Deref , DerefMut } ;
/* FP:context.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0006
/* FP:context.rs-0012 */ use std :: str ;
/* FP:context.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0007
/* FP:context.rs-0014 */ use crate :: rustc_abi :: { HasDataLayout , Size , TargetDataLayout , VariantIdx } ;
/* FP:context.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0008
/* FP:context.rs-0016 */ use crate :: rustc_codegen_ssa :: back :: versioned_llvm_target ;
/* FP:context.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0009
/* FP:context.rs-0018 */ use crate :: rustc_codegen_ssa :: base :: { wants_msvc_seh , wants_wasm_eh } ;
/* FP:context.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0010
/* FP:context.rs-0020 */ use crate :: rustc_codegen_ssa :: errors as ssa_errors ;
/* FP:context.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0011
/* FP:context.rs-0022 */ use crate :: rustc_codegen_ssa :: traits :: * ;
/* FP:context.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0012
/* FP:context.rs-0024 */ use crate :: rustc_data_structures :: base_n :: { ALPHANUMERIC_ONLY , ToBaseN } ;
/* FP:context.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0013
/* FP:context.rs-0026 */ use crate :: rustc_data_structures :: fx :: FxHashMap ;
/* FP:context.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0014
/* FP:context.rs-0028 */ use crate :: rustc_data_structures :: small_c_str :: SmallCStr ;
/* FP:context.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0015
/* FP:context.rs-0030 */ use crate :: rustc_complete :: def_id :: DefId ;
/* FP:context.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0016
/* FP:context.rs-0032 */ use crate :: rustc_complete :: middle :: codegen_fn_attrs :: PatchableFunctionEntry ;
/* FP:context.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0017
/* FP:context.rs-0034 */ use crate :: rustc_complete :: mir :: mono :: CodegenUnit ;
/* FP:context.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0018
/* FP:context.rs-0036 */ use crate :: rustc_complete :: ty :: layout :: { FnAbiError , FnAbiOfHelpers , FnAbiRequest , HasTypingEnv , LayoutError , LayoutOfHelpers , } ;
/* FP:context.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0019
/* FP:context.rs-0038 */ use crate :: rustc_complete :: ty :: { self , Instance , Ty , TyCtxt } ;
/* FP:context.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0020
/* FP:context.rs-0040 */ use crate :: rustc_complete :: { bug , span_bug } ;
/* FP:context.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0021
/* FP:context.rs-0042 */ use crate :: rustc_complete :: Session ;
/* FP:context.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0022
/* FP:context.rs-0044 */ use crate :: rustc_complete :: config :: { BranchProtection , CFGuard , CFProtection , CrateType , DebugInfo , FunctionReturn , PAuthKey , PacRet , } ;
/* FP:context.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0023
/* FP:context.rs-0046 */ use crate :: rustc_complete :: source_map :: Spanned ;
/* FP:context.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0024
/* FP:context.rs-0048 */ use crate :: rustc_complete :: { DUMMY_SP , Span } ;
/* FP:context.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0025
/* FP:context.rs-0050 */ use rustc_symbol_mangling :: mangle_internal_symbol ;
/* FP:context.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0026
/* FP:context.rs-0052 */ use crate :: rustc_target :: spec :: { HasTargetSpec , RelocModel , SmallDataThresholdSupport , Target , TlsModel } ;
/* FP:context.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0027
/* FP:context.rs-0054 */ use smallvec :: SmallVec ;
/* FP:context.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0028
/* FP:context.rs-0056 */ use crate :: back :: write :: to_llvm_code_model ;
/* FP:context.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0029
/* FP:context.rs-0058 */ use crate :: callee :: get_fn ;
/* FP:context.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0030
/* FP:context.rs-0060 */ use crate :: debuginfo :: metadata :: apply_vcall_visibility_metadata ;
/* FP:context.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0031
/* FP:context.rs-0062 */ use crate :: llvm :: Metadata ;
/* FP:context.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0032
/* FP:context.rs-0064 */ use crate :: type_ :: Type ;
/* FP:context.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0033
/* FP:context.rs-0066 */ use crate :: value :: Value ;
/* FP:context.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_USE_0034
/* FP:context.rs-0068 */ use crate :: { attributes , common , coverageinfo , debuginfo , llvm , llvm_util } ;
/* FP:context.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_STRUCT_0035
/* FP:context.rs-0070 */ # [doc = " `TyCtxt` (and related cache datastructures) can't be move between threads."] # [doc = " However, there are various cx related functions which we want to be available to the builder and"] # [doc = " other compiler pieces. Here we define a small subset which has enough information and can be"] # [doc = " moved around more freely."] pub (crate) struct SCx < 'll > { pub llmod : & 'll llvm :: Module , pub llcx : & 'll llvm :: Context , pub isize_ty : & 'll Type , }
/* FP:context.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_IMPL_0036
/* FP:context.rs-0072 */ impl < 'll > Borrow < SCx < 'll > > for FullCx < 'll , '_ > { fn borrow (& self) -> & SCx < 'll > { & self . scx } }
/* FP:context.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_IMPL_0037
/* FP:context.rs-0074 */ impl < 'll , 'tcx > Deref for FullCx < 'll , 'tcx > { type Target = SimpleCx < 'll > ; # [inline] fn deref (& self) -> & Self :: Target { & self . scx } }
/* FP:context.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_STRUCT_0038
/* FP:context.rs-0076 */ pub (crate) struct GenericCx < 'll , T : Borrow < SCx < 'll > > > (T , PhantomData < SCx < 'll > >) ;
/* FP:context.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_IMPL_0039
/* FP:context.rs-0078 */ impl < 'll , T : Borrow < SCx < 'll > > > Deref for GenericCx < 'll , T > { type Target = T ; # [inline] fn deref (& self) -> & Self :: Target { & self . 0 } }
/* FP:context.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_IMPL_0040
/* FP:context.rs-0080 */ impl < 'll , T : Borrow < SCx < 'll > > > DerefMut for GenericCx < 'll , T > { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }
/* FP:context.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_TYPE_0041
/* FP:context.rs-0082 */ pub (crate) type SimpleCx < 'll > = GenericCx < 'll , SCx < 'll > > ;
/* FP:context.rs-0083 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_TYPE_0042
/* FP:context.rs-0084 */ # [doc = " There is one `CodegenCx` per codegen unit. Each one has its own LLVM"] # [doc = " `llvm::Context` so that several codegen units may be processed in parallel."] # [doc = " All other LLVM data structures in the `CodegenCx` are tied to that `llvm::Context`."] pub (crate) type CodegenCx < 'll , 'tcx > = GenericCx < 'll , FullCx < 'll , 'tcx > > ;
/* FP:context.rs-0085 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_STRUCT_0043
/* FP:context.rs-0086 */ pub (crate) struct FullCx < 'll , 'tcx > { pub tcx : TyCtxt < 'tcx > , pub scx : SimpleCx < 'll > , pub use_dll_storage_attrs : bool , pub tls_model : llvm :: ThreadLocalMode , pub codegen_unit : & 'tcx CodegenUnit < 'tcx > , # [doc = " Cache instances of monomorphic and polymorphic items"] pub instances : RefCell < FxHashMap < Instance < 'tcx > , & 'll Value > > , # [doc = " Cache generated vtables"] pub vtables : RefCell < FxHashMap < (Ty < 'tcx > , Option < ty :: ExistentialTraitRef < 'tcx > >) , & 'll Value > > , # [doc = " Cache of constant strings,"] pub const_str_cache : RefCell < FxHashMap < String , & 'll Value > > , # [doc = " Cache of emitted const globals (value -> global)"] pub const_globals : RefCell < FxHashMap < & 'll Value , & 'll Value > > , # [doc = " List of globals for static variables which need to be passed to the"] # [doc = " LLVM function ReplaceAllUsesWith (RAUW) when codegen is complete."] # [doc = " (We have to make sure we don't invalidate any Values referring"] # [doc = " to constants.)"] pub statics_to_rauw : RefCell < Vec < (& 'll Value , & 'll Value) > > , # [doc = " Statics that will be placed in the llvm.used variable"] # [doc = " See <https://llvm.org/docs/LangRef.html#the-llvm-used-global-variable> for details"] pub used_statics : Vec < & 'll Value > , # [doc = " Statics that will be placed in the llvm.compiler.used variable"] # [doc = " See <https://llvm.org/docs/LangRef.html#the-llvm-compiler-used-global-variable> for details"] pub compiler_used_statics : Vec < & 'll Value > , # [doc = " Mapping of non-scalar types to llvm types."] pub type_lowering : RefCell < FxHashMap < (Ty < 'tcx > , Option < VariantIdx >) , & 'll Type > > , # [doc = " Mapping of scalar types to llvm types."] pub scalar_lltypes : RefCell < FxHashMap < Ty < 'tcx > , & 'll Type > > , # [doc = " Extra per-CGU codegen state needed when coverage instrumentation is enabled."] pub coverage_cx : Option < coverageinfo :: CguCoverageContext < 'll , 'tcx > > , pub dbg_cx : Option < debuginfo :: CodegenUnitDebugContext < 'll , 'tcx > > , eh_personality : Cell < Option < & 'll Value > > , eh_catch_typeinfo : Cell < Option < & 'll Value > > , pub rust_try_fn : Cell < Option < (& 'll Type , & 'll Value) > > , intrinsics : RefCell < FxHashMap < (Cow < 'static , str > , SmallVec < [& 'll Type ; 2] >) , (& 'll Type , & 'll Value) > > , # [doc = " A counter that is used for generating local symbol names"] local_gen_sym_counter : Cell < usize > , # [doc = " `codegen_static` will sometimes create a second global variable with a"] # [doc = " different type and clear the symbol name of the original global."] # [doc = " `global_asm!` needs to be able to find this new global so that it can"] # [doc = " compute the correct mangled symbol name to insert into the asm."] pub renamed_statics : RefCell < FxHashMap < DefId , & 'll Value > > , }
/* FP:context.rs-0087 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_FN_0044
/* FP:context.rs-0088 */ fn to_llvm_tls_model (tls_model : TlsModel) -> llvm :: ThreadLocalMode { match tls_model { TlsModel :: GeneralDynamic => llvm :: ThreadLocalMode :: GeneralDynamic , TlsModel :: LocalDynamic => llvm :: ThreadLocalMode :: LocalDynamic , TlsModel :: InitialExec => llvm :: ThreadLocalMode :: InitialExec , TlsModel :: LocalExec => llvm :: ThreadLocalMode :: LocalExec , TlsModel :: Emulated => llvm :: ThreadLocalMode :: GeneralDynamic , } }
/* FP:context.rs-0089 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_FN_0045
/* FP:context.rs-0092 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_IMPL_0046
/* FP:context.rs-0094 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_IMPL_0047
/* FP:context.rs-0095 */ impl < 'll > SimpleCx < 'll > { pub (crate) fn get_type_of_global (& self , val : & 'll Value) -> & 'll Type { unsafe { llvm :: LLVMGlobalGetValueType (val) } } pub (crate) fn val_ty (& self , v : & 'll Value) -> & 'll Type { common :: val_ty (v) } }
/* FP:context.rs-0096 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_IMPL_0048
/* FP:context.rs-0098 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_IMPL_0049
/* FP:context.rs-0100 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_IMPL_0050
/* FP:context.rs-0101 */ impl < 'll , 'tcx > MiscCodegenMethods < 'tcx > for CodegenCx < 'll , 'tcx > { fn vtables (& self ,) -> & RefCell < FxHashMap < (Ty < 'tcx > , Option < ty :: ExistentialTraitRef < 'tcx > >) , & 'll Value > > { & self . vtables } fn apply_vcall_visibility_metadata (& self , ty : Ty < 'tcx > , poly_trait_ref : Option < ty :: ExistentialTraitRef < 'tcx > > , vtable : & 'll Value ,) { apply_vcall_visibility_metadata (self , ty , poly_trait_ref , vtable) ; } fn get_fn (& self , instance : Instance < 'tcx >) -> & 'll Value { get_fn (self , instance) } fn get_fn_addr (& self , instance : Instance < 'tcx >) -> & 'll Value { get_fn (self , instance) } fn eh_personality (& self) -> & 'll Value { if let Some (llpersonality) = self . eh_personality . get () { return llpersonality ; } let name = if wants_msvc_seh (self . sess ()) { Some ("__CxxFrameHandler3") } else if wants_wasm_eh (self . sess ()) { Some ("__gxx_wasm_personality_v0") } else { None } ; let tcx = self . tcx ; let llfn = match tcx . lang_items () . eh_personality () { Some (def_id) if name . is_none () => self . get_fn_addr (ty :: Instance :: expect_resolve (tcx , self . typing_env () , def_id , ty :: List :: empty () , DUMMY_SP ,)) , _ => { let name = name . unwrap_or ("rust_eh_personality") ; if let Some (llfn) = self . get_declared_value (name) { llfn } else { let fty = self . type_variadic_func (& [] , self . type_i32 ()) ; let llfn = self . declare_cfn (name , llvm :: UnnamedAddr :: Global , fty) ; let target_cpu = attributes :: target_cpu_attr (self) ; attributes :: apply_to_llfn (llfn , llvm :: AttributePlace :: Function , & [target_cpu]) ; llfn } } } ; self . eh_personality . set (Some (llfn)) ; llfn } fn sess (& self) -> & Session { self . tcx . sess } fn set_frame_pointer_type (& self , llfn : & 'll Value) { if let Some (attr) = attributes :: frame_pointer_type_attr (self) { attributes :: apply_to_llfn (llfn , llvm :: AttributePlace :: Function , & [attr]) ; } } fn apply_target_cpu_attr (& self , llfn : & 'll Value) { let mut attrs = SmallVec :: < [_ ; 2] > :: new () ; attrs . push (attributes :: target_cpu_attr (self)) ; attrs . extend (attributes :: tune_cpu_attr (self)) ; attributes :: apply_to_llfn (llfn , llvm :: AttributePlace :: Function , & attrs) ; } fn declare_c_main (& self , fn_type : Self :: Type) -> Option < Self :: Function > { let entry_name = self . sess () . target . entry_name . as_ref () ; if self . get_declared_value (entry_name) . is_none () { let llfn = self . declare_entry_fn (entry_name , llvm :: CallConv :: from_conv (self . sess () . target . entry_abi , self . sess () . target . arch . borrow () ,) , llvm :: UnnamedAddr :: Global , fn_type ,) ; attributes :: apply_to_llfn (llfn , llvm :: AttributePlace :: Function , attributes :: target_features_attr (self , vec ! []) . as_slice () ,) ; Some (llfn) } else { None } } }
/* FP:context.rs-0102 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_IMPL_0051
/* FP:context.rs-0104 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_IMPL_0052
/* FP:context.rs-0105 */ impl CodegenCx < '_ , '_ > { # [doc = " Generates a new symbol name with the given prefix. This symbol name must"] # [doc = " only be used for definitions with `internal` or `private` linkage."] pub (crate) fn generate_local_symbol_name (& self , prefix : & str) -> String { let idx = self . local_gen_sym_counter . get () ; self . local_gen_sym_counter . set (idx + 1) ; let mut name = String :: with_capacity (prefix . len () + 6) ; name . push_str (prefix) ; name . push ('.') ; name . push_str (& (idx as u64) . to_base (ALPHANUMERIC_ONLY)) ; name } }
/* FP:context.rs-0106 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_IMPL_0053
/* FP:context.rs-0107 */ impl < 'll , CX : Borrow < SCx < 'll > > > GenericCx < 'll , CX > { # [doc = " A wrapper for [`llvm::LLVMSetMetadata`], but it takes `Metadata` as a parameter instead of `Value`."] pub (crate) fn set_metadata < 'a > (& self , val : & 'a Value , kind_id : impl Into < llvm :: MetadataKindId > , md : & 'll Metadata ,) { let node = self . get_metadata_value (md) ; llvm :: LLVMSetMetadata (val , kind_id . into () , node) ; } }
/* FP:context.rs-0108 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_IMPL_0054
/* FP:context.rs-0109 */ impl HasDataLayout for CodegenCx < '_ , '_ > { # [inline] fn data_layout (& self) -> & TargetDataLayout { & self . tcx . data_layout } }
/* FP:context.rs-0110 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_IMPL_0055
/* FP:context.rs-0111 */ impl HasTargetSpec for CodegenCx < '_ , '_ > { # [inline] fn target_spec (& self) -> & Target { & self . tcx . sess . target } }
/* FP:context.rs-0112 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_IMPL_0056
/* FP:context.rs-0113 */ impl < 'tcx > ty :: layout :: HasTyCtxt < 'tcx > for CodegenCx < '_ , 'tcx > { # [inline] fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } }
/* FP:context.rs-0114 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_IMPL_0057
/* FP:context.rs-0115 */ impl < 'tcx , 'll > HasTypingEnv < 'tcx > for CodegenCx < 'll , 'tcx > { fn typing_env (& self) -> ty :: TypingEnv < 'tcx > { ty :: TypingEnv :: fully_monomorphized () } }
/* FP:context.rs-0116 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_IMPL_0058
/* FP:context.rs-0117 */ impl < 'tcx > LayoutOfHelpers < 'tcx > for CodegenCx < '_ , 'tcx > { # [inline] fn handle_layout_err (& self , err : LayoutError < 'tcx > , span : Span , ty : Ty < 'tcx >) -> ! { if let LayoutError :: SizeOverflow (_) | LayoutError :: ReferencesError (_) = err { self . tcx . dcx () . emit_fatal (Spanned { span , node : err . into_diagnostic () }) } else { self . tcx . dcx () . emit_fatal (ssa_errors :: FailedToGetLayout { span , ty , err }) } } }
/* FP:context.rs-0118 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_context_IMPL_0059
/* FP:context.rs-0119 */ impl < 'tcx > FnAbiOfHelpers < 'tcx > for CodegenCx < '_ , 'tcx > { # [inline] fn handle_fn_abi_err (& self , err : FnAbiError < 'tcx > , span : Span , fn_abi_request : FnAbiRequest < 'tcx > ,) -> ! { match err { FnAbiError :: Layout (LayoutError :: SizeOverflow (_) | LayoutError :: Cycle (_)) => { self . tcx . dcx () . emit_fatal (Spanned { span , node : err }) ; } _ => match fn_abi_request { FnAbiRequest :: OfFnPtr { sig , extra_args } => { span_bug ! (span , "`fn_abi_of_fn_ptr({sig}, {extra_args:?})` failed: {err:?}" ,) ; } FnAbiRequest :: OfInstance { instance , extra_args } => { span_bug ! (span , "`fn_abi_of_instance({instance}, {extra_args:?})` failed: {err:?}" ,) ; } } , } } }