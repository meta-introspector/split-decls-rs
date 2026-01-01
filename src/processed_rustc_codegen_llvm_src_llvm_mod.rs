/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_USE_0001
/* FP:mod.rs-0002 */ # [allow (non_snake_case)] use std :: ffi :: { CStr , CString } ;
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_USE_0002
/* FP:mod.rs-0004 */ use std :: num :: NonZero ;
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_USE_0003
/* FP:mod.rs-0006 */ use std :: ptr ;
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_USE_0004
/* FP:mod.rs-0008 */ use std :: string :: FromUtf8Error ;
/* FP:mod.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_USE_0005
/* FP:mod.rs-0010 */ use libc :: c_uint ;
/* FP:mod.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_USE_0006
/* FP:mod.rs-0012 */ use crate :: rustc_abi :: { Align , Size , WrappingRange } ;
/* FP:mod.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_USE_0007
/* FP:mod.rs-0014 */ use rustc_llvm :: RustString ;
/* FP:mod.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_USE_0008
/* FP:mod.rs-0016 */ pub (crate) use self :: CallConv :: * ;
/* FP:mod.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_USE_0009
/* FP:mod.rs-0018 */ pub (crate) use self :: CodeGenOptSize :: * ;
/* FP:mod.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_USE_0010
/* FP:mod.rs-0020 */ pub (crate) use self :: MetadataType :: * ;
/* FP:mod.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_USE_0011
/* FP:mod.rs-0022 */ pub (crate) use self :: ffi :: * ;
/* FP:mod.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_USE_0012
/* FP:mod.rs-0024 */ use crate :: common :: AsCCharPtr ;
/* FP:mod.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_MOD_0013
/* FP:mod.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_MOD_0014
/* FP:mod.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_MOD_0015
/* FP:mod.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_USE_0016
/* FP:mod.rs-0032 */ pub (crate) use self :: enzyme_ffi :: * ;
/* FP:mod.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_IMPL_0017
/* FP:mod.rs-0034 */ impl LLVMRustResult { pub (crate) fn into_result (self) -> Result < () , () > { match self { LLVMRustResult :: Success => Ok (()) , LLVMRustResult :: Failure => Err (()) , } } }
/* FP:mod.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0018
/* FP:mod.rs-0036 */ pub (crate) fn AddFunctionAttributes < 'll > (llfn : & 'll Value , idx : AttributePlace , attrs : & [& 'll Attribute] ,) { unsafe { LLVMRustAddFunctionAttributes (llfn , idx . as_uint () , attrs . as_ptr () , attrs . len ()) ; } }
/* FP:mod.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0019
/* FP:mod.rs-0038 */ pub (crate) fn AddCallSiteAttributes < 'll > (callsite : & 'll Value , idx : AttributePlace , attrs : & [& 'll Attribute] ,) { unsafe { LLVMRustAddCallSiteAttributes (callsite , idx . as_uint () , attrs . as_ptr () , attrs . len ()) ; } }
/* FP:mod.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0020
/* FP:mod.rs-0040 */ pub (crate) fn CreateAttrStringValue < 'll > (llcx : & 'll Context , attr : & str , value : & str ,) -> & 'll Attribute { unsafe { LLVMCreateStringAttribute (llcx , attr . as_c_char_ptr () , attr . len () . try_into () . unwrap () , value . as_c_char_ptr () , value . len () . try_into () . unwrap () ,) } }
/* FP:mod.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0021
/* FP:mod.rs-0042 */ pub (crate) fn CreateAttrString < 'll > (llcx : & 'll Context , attr : & str) -> & 'll Attribute { unsafe { LLVMCreateStringAttribute (llcx , attr . as_c_char_ptr () , attr . len () . try_into () . unwrap () , std :: ptr :: null () , 0 ,) } }
/* FP:mod.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0022
/* FP:mod.rs-0044 */ pub (crate) fn CreateAlignmentAttr (llcx : & Context , bytes : u64) -> & Attribute { unsafe { LLVMRustCreateAlignmentAttr (llcx , bytes) } }
/* FP:mod.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0023
/* FP:mod.rs-0046 */ pub (crate) fn CreateDereferenceableAttr (llcx : & Context , bytes : u64) -> & Attribute { unsafe { LLVMRustCreateDereferenceableAttr (llcx , bytes) } }
/* FP:mod.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0024
/* FP:mod.rs-0048 */ pub (crate) fn CreateDereferenceableOrNullAttr (llcx : & Context , bytes : u64) -> & Attribute { unsafe { LLVMRustCreateDereferenceableOrNullAttr (llcx , bytes) } }
/* FP:mod.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0025
/* FP:mod.rs-0050 */ pub (crate) fn CreateByValAttr < 'll > (llcx : & 'll Context , ty : & 'll Type) -> & 'll Attribute { unsafe { LLVMRustCreateByValAttr (llcx , ty) } }
/* FP:mod.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0026
/* FP:mod.rs-0052 */ pub (crate) fn CreateStructRetAttr < 'll > (llcx : & 'll Context , ty : & 'll Type) -> & 'll Attribute { unsafe { LLVMRustCreateStructRetAttr (llcx , ty) } }
/* FP:mod.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0027
/* FP:mod.rs-0054 */ pub (crate) fn CreateUWTableAttr (llcx : & Context , async_ : bool) -> & Attribute { unsafe { LLVMRustCreateUWTableAttr (llcx , async_) } }
/* FP:mod.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0028
/* FP:mod.rs-0056 */ pub (crate) fn CreateAllocSizeAttr (llcx : & Context , size_arg : u32) -> & Attribute { unsafe { LLVMRustCreateAllocSizeAttr (llcx , size_arg) } }
/* FP:mod.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0029
/* FP:mod.rs-0058 */ pub (crate) fn CreateAllocKindAttr (llcx : & Context , kind_arg : AllocKindFlags) -> & Attribute { unsafe { LLVMRustCreateAllocKindAttr (llcx , kind_arg . bits ()) } }
/* FP:mod.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0030
/* FP:mod.rs-0060 */ pub (crate) fn CreateRangeAttr (llcx : & Context , size : Size , range : WrappingRange) -> & Attribute { let lower = range . start ; let upper = range . end . wrapping_add (1) ; let as_u64_array = | x : u128 | [x as u64 , (x >> 64) as u64] ; let lower_words : [u64 ; 2] = as_u64_array (lower) ; let upper_words : [u64 ; 2] = as_u64_array (upper) ; let size_bits = size . bits () ; assert ! (size_bits <= 128) ; assert ! (size_bits . div_ceil (64) <= u64 :: try_from (lower_words . len ()) . unwrap ()) ; assert ! (size_bits . div_ceil (64) <= u64 :: try_from (upper_words . len ()) . unwrap ()) ; let size_bits = c_uint :: try_from (size_bits) . unwrap () ; unsafe { LLVMRustCreateRangeAttribute (llcx , size_bits , lower_words . as_ptr () , upper_words . as_ptr ()) } }
/* FP:mod.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_ENUM_0031
/* FP:mod.rs-0062 */ # [derive (Copy , Clone)] pub (crate) enum AttributePlace { ReturnValue , Argument (u32) , Function , }
/* FP:mod.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_IMPL_0032
/* FP:mod.rs-0064 */ impl AttributePlace { pub (crate) fn as_uint (self) -> c_uint { match self { AttributePlace :: ReturnValue => 0 , AttributePlace :: Argument (i) => 1 + i , AttributePlace :: Function => ! 0 , } } }
/* FP:mod.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_ENUM_0033
/* FP:mod.rs-0066 */ # [derive (Copy , Clone , PartialEq)] # [repr (C)] pub (crate) enum CodeGenOptSize { CodeGenOptSizeNone = 0 , CodeGenOptSizeDefault = 1 , CodeGenOptSizeAggressive = 2 , }
/* FP:mod.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0034
/* FP:mod.rs-0068 */ pub (crate) fn SetInstructionCallConv (instr : & Value , cc : CallConv) { unsafe { LLVMSetInstructionCallConv (instr , cc as c_uint) ; } }
/* FP:mod.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0035
/* FP:mod.rs-0070 */ pub (crate) fn SetFunctionCallConv (fn_ : & Value , cc : CallConv) { unsafe { LLVMSetFunctionCallConv (fn_ , cc as c_uint) ; } }
/* FP:mod.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0036
/* FP:mod.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0037
/* FP:mod.rs-0074 */ pub (crate) fn set_unnamed_address (global : & Value , unnamed : UnnamedAddr) { LLVMSetUnnamedAddress (global , unnamed) ; }
/* FP:mod.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0038
/* FP:mod.rs-0076 */ pub (crate) fn set_thread_local_mode (global : & Value , mode : ThreadLocalMode) { unsafe { LLVMSetThreadLocalMode (global , mode) ; } }
/* FP:mod.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_IMPL_0039
/* FP:mod.rs-0078 */ impl AttributeKind { # [doc = " Create an LLVM Attribute with no associated value."] pub (crate) fn create_attr (self , llcx : & Context) -> & Attribute { unsafe { LLVMRustCreateAttrNoValue (llcx , self) } } }
/* FP:mod.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_IMPL_0040
/* FP:mod.rs-0080 */ impl MemoryEffects { # [doc = " Create an LLVM Attribute with these memory effects."] pub (crate) fn create_attr (self , llcx : & Context) -> & Attribute { unsafe { LLVMRustCreateMemoryEffectsAttr (llcx , self) } } }
/* FP:mod.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0041
/* FP:mod.rs-0082 */ pub (crate) fn set_section (llglobal : & Value , section_name : & CStr) { unsafe { LLVMSetSection (llglobal , section_name . as_ptr ()) ; } }
/* FP:mod.rs-0083 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0042
/* FP:mod.rs-0084 */ pub (crate) fn add_global < 'a > (llmod : & 'a Module , ty : & 'a Type , name_cstr : & CStr) -> & 'a Value { unsafe { LLVMAddGlobal (llmod , ty , name_cstr . as_ptr ()) } }
/* FP:mod.rs-0085 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0043
/* FP:mod.rs-0086 */ pub (crate) fn set_initializer (llglobal : & Value , constant_val : & Value) { unsafe { LLVMSetInitializer (llglobal , constant_val) ; } }
/* FP:mod.rs-0087 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0044
/* FP:mod.rs-0088 */ pub (crate) fn set_global_constant (llglobal : & Value , is_constant : bool) { LLVMSetGlobalConstant (llglobal , is_constant . to_llvm_bool ()) ; }
/* FP:mod.rs-0089 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0045
/* FP:mod.rs-0090 */ pub (crate) fn get_linkage (llglobal : & Value) -> Linkage { unsafe { LLVMGetLinkage (llglobal) } . to_rust () }
/* FP:mod.rs-0091 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0046
/* FP:mod.rs-0092 */ pub (crate) fn set_linkage (llglobal : & Value , linkage : Linkage) { unsafe { LLVMSetLinkage (llglobal , linkage) ; } }
/* FP:mod.rs-0093 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0047
/* FP:mod.rs-0094 */ pub (crate) fn is_declaration (llglobal : & Value) -> bool { unsafe { LLVMIsDeclaration (llglobal) } . is_true () }
/* FP:mod.rs-0095 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0048
/* FP:mod.rs-0096 */ pub (crate) fn get_visibility (llglobal : & Value) -> Visibility { unsafe { LLVMGetVisibility (llglobal) } . to_rust () }
/* FP:mod.rs-0097 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0049
/* FP:mod.rs-0098 */ pub (crate) fn set_visibility (llglobal : & Value , visibility : Visibility) { unsafe { LLVMSetVisibility (llglobal , visibility) ; } }
/* FP:mod.rs-0099 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0050
/* FP:mod.rs-0100 */ pub (crate) fn set_alignment (llglobal : & Value , align : Align) { unsafe { ffi :: LLVMSetAlignment (llglobal , align . bytes () as c_uint) ; } }
/* FP:mod.rs-0101 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0051
/* FP:mod.rs-0103 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0052
/* FP:mod.rs-0104 */ # [doc = " Safe wrapper around `LLVMGetParam`, because segfaults are no fun."] pub (crate) fn get_param (llfn : & Value , index : c_uint) -> & Value { unsafe { assert ! (index < LLVMCountParams (llfn) , "out of bounds argument access: {} out of {} arguments" , index , LLVMCountParams (llfn)) ; LLVMGetParam (llfn , index) } }
/* FP:mod.rs-0105 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0053
/* FP:mod.rs-0106 */ # [doc = " Safe wrapper for `LLVMGetValueName2`"] # [doc = " Needs to allocate the value, because `set_value_name` will invalidate"] # [doc = " the pointer."] pub (crate) fn get_value_name (value : & Value) -> Vec < u8 > { unsafe { let mut len = 0 ; let data = LLVMGetValueName2 (value , & mut len) ; std :: slice :: from_raw_parts (data . cast () , len) . to_vec () } }
/* FP:mod.rs-0107 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_STRUCT_0054
/* FP:mod.rs-0108 */ # [derive (Debug , Copy , Clone)] pub (crate) struct Intrinsic { id : NonZero < c_uint > , }
/* FP:mod.rs-0109 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_IMPL_0055
/* FP:mod.rs-0111 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0056
/* FP:mod.rs-0112 */ # [doc = " Safe wrapper for `LLVMSetValueName2` from a byte slice"] pub (crate) fn set_value_name (value : & Value , name : & [u8]) { unsafe { let data = name . as_c_char_ptr () ; LLVMSetValueName2 (value , data , name . len ()) ; } }
/* FP:mod.rs-0113 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0057
/* FP:mod.rs-0114 */ pub (crate) fn build_string (f : impl FnOnce (& RustString)) -> Result < String , FromUtf8Error > { String :: from_utf8 (RustString :: build_byte_buffer (f)) }
/* FP:mod.rs-0115 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0058
/* FP:mod.rs-0116 */ pub (crate) fn build_byte_buffer (f : impl FnOnce (& RustString)) -> Vec < u8 > { RustString :: build_byte_buffer (f) }
/* FP:mod.rs-0117 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0059
/* FP:mod.rs-0118 */ pub (crate) fn twine_to_string (tr : & Twine) -> String { unsafe { build_string (| s | LLVMRustWriteTwineToString (tr , s)) . expect ("got a non-UTF8 Twine from LLVM") } }
/* FP:mod.rs-0119 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0060
/* FP:mod.rs-0120 */ pub (crate) fn last_error () -> Option < String > { unsafe { let cstr = LLVMRustGetLastError () ; if cstr . is_null () { None } else { let err = CStr :: from_ptr (cstr) . to_bytes () ; let err = String :: from_utf8_lossy (err) . to_string () ; libc :: free (cstr as * mut _) ; Some (err) } } }
/* FP:mod.rs-0121 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_STRUCT_0061
/* FP:mod.rs-0122 */ # [doc = " Owning pointer to an [`OperandBundle`] that will dispose of the bundle"] # [doc = " when dropped."] pub (crate) struct OperandBundleBox < 'a > { raw : ptr :: NonNull < OperandBundle < 'a > > , }
/* FP:mod.rs-0123 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_IMPL_0062
/* FP:mod.rs-0124 */ impl < 'a > OperandBundleBox < 'a > { pub (crate) fn new (name : & str , vals : & [& 'a Value]) -> Self { let raw = unsafe { LLVMCreateOperandBundle (name . as_c_char_ptr () , name . len () , vals . as_ptr () , vals . len () as c_uint ,) } ; Self { raw : ptr :: NonNull :: new (raw) . unwrap () } } # [doc = " Dereferences to the underlying `&OperandBundle`."] # [doc = ""] # [doc = " This can't be a `Deref` implementation because `OperandBundle` transitively"] # [doc = " contains an extern type, which is incompatible with `Deref::Target: ?Sized`."] pub (crate) fn as_ref (& self) -> & OperandBundle < 'a > { unsafe { self . raw . as_ref () } } }
/* FP:mod.rs-0125 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_IMPL_0063
/* FP:mod.rs-0126 */ impl Drop for OperandBundleBox < '_ > { fn drop (& mut self) { unsafe { LLVMDisposeOperandBundle (self . raw) ; } } }
/* FP:mod.rs-0127 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0064
/* FP:mod.rs-0128 */ pub (crate) fn add_module_flag_u32 (module : & Module , merge_behavior : ModuleFlagMergeBehavior , key : & str , value : u32 ,) { unsafe { LLVMRustAddModuleFlagU32 (module , merge_behavior , key . as_c_char_ptr () , key . len () , value) ; } }
/* FP:mod.rs-0129 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0065
/* FP:mod.rs-0130 */ pub (crate) fn add_module_flag_str (module : & Module , merge_behavior : ModuleFlagMergeBehavior , key : & str , value : & str ,) { unsafe { LLVMRustAddModuleFlagString (module , merge_behavior , key . as_c_char_ptr () , key . len () , value . as_c_char_ptr () , value . len () ,) ; } }
/* FP:mod.rs-0131 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0066
/* FP:mod.rs-0132 */ pub (crate) fn set_dllimport_storage_class < 'll > (v : & 'll Value) { unsafe { LLVMSetDLLStorageClass (v , DLLStorageClass :: DllImport) ; } }
/* FP:mod.rs-0133 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0067
/* FP:mod.rs-0134 */ pub (crate) fn set_dso_local < 'll > (v : & 'll Value) { unsafe { LLVMRustSetDSOLocal (v , true) ; } }
/* FP:mod.rs-0135 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_llvm_mod_FN_0068