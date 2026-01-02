mkuse!{use std :: ffi :: { CStr , CString } ;}
mkuse!{use std :: num :: NonZero ;}
mkuse!{use std :: ptr ;}
mkuse!{use std :: string :: FromUtf8Error ;}
mkuse!{use libc :: c_uint ;}
mkuse!{use rustc_abi :: { Align , Size , WrappingRange } ;}
mkuse!{use rustc_llvm :: RustString ;}
mkuse!{pub (crate) use self :: CallConv :: * ;}
mkuse!{pub (crate) use self :: CodeGenOptSize :: * ;}
mkuse!{pub (crate) use self :: MetadataType :: * ;}
mkuse!{pub (crate) use self :: ffi :: * ;}
mkuse!{use crate :: common :: AsCCharPtr ;}
mkmod!{diagnostic, { 
                getname!(diagnostic);
                getsrc!(diagnostic);
                getpath!(diagnostic);
                get_deps!(diagnostic);
                get_crates!(diagnostic);
                mkinclude!(diagnostic);
                 
            }}
mkmod!{enzyme_ffi, { 
                getname!(enzyme_ffi);
                getsrc!(enzyme_ffi);
                getpath!(enzyme_ffi);
                get_deps!(enzyme_ffi);
                get_crates!(enzyme_ffi);
                mkinclude!(enzyme_ffi);
                 
            }}
mkmod!{ffi, { 
                getname!(ffi);
                getsrc!(ffi);
                getpath!(ffi);
                get_deps!(ffi);
                get_crates!(ffi);
                mkinclude!(ffi);
                 
            }}
mkuse!{pub (crate) use self :: enzyme_ffi :: * ;}
mkitem!{mkimpl!{impl LLVMRustResult { pub (crate) fn into_result (self) -> Result < () , () > { match self { LLVMRustResult :: Success => Ok (()) , LLVMRustResult :: Failure => Err (()) , } } }}}

macro_rules! AddFunctionAttributes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function AddFunctionAttributes in module {}", module_path!());
    };
}

mkfn!{
    AddFunctionAttributes_introspect!();
    pub (crate) fn AddFunctionAttributes < 'll > (llfn : & 'll Value , idx : AttributePlace , attrs : & [& 'll Attribute] ,) { unsafe { LLVMRustAddFunctionAttributes (llfn , idx . as_uint () , attrs . as_ptr () , attrs . len ()) ; } }
}

macro_rules! AddCallSiteAttributes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function AddCallSiteAttributes in module {}", module_path!());
    };
}

mkfn!{
    AddCallSiteAttributes_introspect!();
    pub (crate) fn AddCallSiteAttributes < 'll > (callsite : & 'll Value , idx : AttributePlace , attrs : & [& 'll Attribute] ,) { unsafe { LLVMRustAddCallSiteAttributes (callsite , idx . as_uint () , attrs . as_ptr () , attrs . len ()) ; } }
}

macro_rules! CreateAttrStringValue_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function CreateAttrStringValue in module {}", module_path!());
    };
}

mkfn!{
    CreateAttrStringValue_introspect!();
    pub (crate) fn CreateAttrStringValue < 'll > (llcx : & 'll Context , attr : & str , value : & str ,) -> & 'll Attribute { unsafe { LLVMCreateStringAttribute (llcx , attr . as_c_char_ptr () , attr . len () . try_into () . unwrap () , value . as_c_char_ptr () , value . len () . try_into () . unwrap () ,) } }
}

macro_rules! CreateAttrString_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function CreateAttrString in module {}", module_path!());
    };
}

mkfn!{
    CreateAttrString_introspect!();
    pub (crate) fn CreateAttrString < 'll > (llcx : & 'll Context , attr : & str) -> & 'll Attribute { unsafe { LLVMCreateStringAttribute (llcx , attr . as_c_char_ptr () , attr . len () . try_into () . unwrap () , std :: ptr :: null () , 0 ,) } }
}

macro_rules! CreateAlignmentAttr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function CreateAlignmentAttr in module {}", module_path!());
    };
}

mkfn!{
    CreateAlignmentAttr_introspect!();
    pub (crate) fn CreateAlignmentAttr (llcx : & Context , bytes : u64) -> & Attribute { unsafe { LLVMRustCreateAlignmentAttr (llcx , bytes) } }
}

macro_rules! CreateDereferenceableAttr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function CreateDereferenceableAttr in module {}", module_path!());
    };
}

mkfn!{
    CreateDereferenceableAttr_introspect!();
    pub (crate) fn CreateDereferenceableAttr (llcx : & Context , bytes : u64) -> & Attribute { unsafe { LLVMRustCreateDereferenceableAttr (llcx , bytes) } }
}

macro_rules! CreateDereferenceableOrNullAttr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function CreateDereferenceableOrNullAttr in module {}", module_path!());
    };
}

mkfn!{
    CreateDereferenceableOrNullAttr_introspect!();
    pub (crate) fn CreateDereferenceableOrNullAttr (llcx : & Context , bytes : u64) -> & Attribute { unsafe { LLVMRustCreateDereferenceableOrNullAttr (llcx , bytes) } }
}

macro_rules! CreateByValAttr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function CreateByValAttr in module {}", module_path!());
    };
}

mkfn!{
    CreateByValAttr_introspect!();
    pub (crate) fn CreateByValAttr < 'll > (llcx : & 'll Context , ty : & 'll Type) -> & 'll Attribute { unsafe { LLVMRustCreateByValAttr (llcx , ty) } }
}

macro_rules! CreateStructRetAttr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function CreateStructRetAttr in module {}", module_path!());
    };
}

mkfn!{
    CreateStructRetAttr_introspect!();
    pub (crate) fn CreateStructRetAttr < 'll > (llcx : & 'll Context , ty : & 'll Type) -> & 'll Attribute { unsafe { LLVMRustCreateStructRetAttr (llcx , ty) } }
}

macro_rules! CreateUWTableAttr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function CreateUWTableAttr in module {}", module_path!());
    };
}

mkfn!{
    CreateUWTableAttr_introspect!();
    pub (crate) fn CreateUWTableAttr (llcx : & Context , async_ : bool) -> & Attribute { unsafe { LLVMRustCreateUWTableAttr (llcx , async_) } }
}

macro_rules! CreateAllocSizeAttr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function CreateAllocSizeAttr in module {}", module_path!());
    };
}

mkfn!{
    CreateAllocSizeAttr_introspect!();
    pub (crate) fn CreateAllocSizeAttr (llcx : & Context , size_arg : u32) -> & Attribute { unsafe { LLVMRustCreateAllocSizeAttr (llcx , size_arg) } }
}

macro_rules! CreateAllocKindAttr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function CreateAllocKindAttr in module {}", module_path!());
    };
}

mkfn!{
    CreateAllocKindAttr_introspect!();
    pub (crate) fn CreateAllocKindAttr (llcx : & Context , kind_arg : AllocKindFlags) -> & Attribute { unsafe { LLVMRustCreateAllocKindAttr (llcx , kind_arg . bits ()) } }
}

macro_rules! CreateRangeAttr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function CreateRangeAttr in module {}", module_path!());
    };
}

mkfn!{
    CreateRangeAttr_introspect!();
    pub (crate) fn CreateRangeAttr (llcx : & Context , size : Size , range : WrappingRange) -> & Attribute { let lower = range . start ; let upper = range . end . wrapping_add (1) ; let as_u64_array = | x : u128 | [x as u64 , (x >> 64) as u64] ; let lower_words : [u64 ; 2] = as_u64_array (lower) ; let upper_words : [u64 ; 2] = as_u64_array (upper) ; let size_bits = size . bits () ; assert ! (size_bits <= 128) ; assert ! (size_bits . div_ceil (64) <= u64 :: try_from (lower_words . len ()) . unwrap ()) ; assert ! (size_bits . div_ceil (64) <= u64 :: try_from (upper_words . len ()) . unwrap ()) ; let size_bits = c_uint :: try_from (size_bits) . unwrap () ; unsafe { LLVMRustCreateRangeAttribute (llcx , size_bits , lower_words . as_ptr () , upper_words . as_ptr ()) } }
}
mkitem!{mkenum!{# [derive (Copy , Clone)] pub (crate) enum AttributePlace { ReturnValue , Argument (u32) , Function , }}}
mkitem!{mkimpl!{impl AttributePlace { pub (crate) fn as_uint (self) -> c_uint { match self { AttributePlace :: ReturnValue => 0 , AttributePlace :: Argument (i) => 1 + i , AttributePlace :: Function => ! 0 , } } }}}
mkitem!{mkenum!{# [derive (Copy , Clone , PartialEq)] # [repr (C)] pub (crate) enum CodeGenOptSize { CodeGenOptSizeNone = 0 , CodeGenOptSizeDefault = 1 , CodeGenOptSizeAggressive = 2 , }}}

macro_rules! SetInstructionCallConv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function SetInstructionCallConv in module {}", module_path!());
    };
}

mkfn!{
    SetInstructionCallConv_introspect!();
    pub (crate) fn SetInstructionCallConv (instr : & Value , cc : CallConv) { unsafe { LLVMSetInstructionCallConv (instr , cc as c_uint) ; } }
}

macro_rules! SetFunctionCallConv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function SetFunctionCallConv in module {}", module_path!());
    };
}

mkfn!{
    SetFunctionCallConv_introspect!();
    pub (crate) fn SetFunctionCallConv (fn_ : & Value , cc : CallConv) { unsafe { LLVMSetFunctionCallConv (fn_ , cc as c_uint) ; } }
}

macro_rules! SetUniqueComdat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function SetUniqueComdat in module {}", module_path!());
    };
}

mkfn!{
    SetUniqueComdat_introspect!();
    pub (crate) fn SetUniqueComdat (llmod : & Module , val : & Value) { let name_buf = get_value_name (val) ; let name = CString :: from_vec_with_nul (name_buf) . or_else (| buf | CString :: new (buf . into_bytes ())) . unwrap () ; set_comdat (llmod , val , & name) ; }
}

macro_rules! set_unnamed_address_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_unnamed_address in module {}", module_path!());
    };
}

mkfn!{
    set_unnamed_address_introspect!();
    pub (crate) fn set_unnamed_address (global : & Value , unnamed : UnnamedAddr) { LLVMSetUnnamedAddress (global , unnamed) ; }
}

macro_rules! set_thread_local_mode_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_thread_local_mode in module {}", module_path!());
    };
}

mkfn!{
    set_thread_local_mode_introspect!();
    pub (crate) fn set_thread_local_mode (global : & Value , mode : ThreadLocalMode) { unsafe { LLVMSetThreadLocalMode (global , mode) ; } }
}
mkitem!{mkimpl!{impl AttributeKind { # [doc = " Create an LLVM Attribute with no associated value."] pub (crate) fn create_attr (self , llcx : & Context) -> & Attribute { unsafe { LLVMRustCreateAttrNoValue (llcx , self) } } }}}
mkitem!{mkimpl!{impl MemoryEffects { # [doc = " Create an LLVM Attribute with these memory effects."] pub (crate) fn create_attr (self , llcx : & Context) -> & Attribute { unsafe { LLVMRustCreateMemoryEffectsAttr (llcx , self) } } }}}

macro_rules! set_section_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_section in module {}", module_path!());
    };
}

mkfn!{
    set_section_introspect!();
    pub (crate) fn set_section (llglobal : & Value , section_name : & CStr) { unsafe { LLVMSetSection (llglobal , section_name . as_ptr ()) ; } }
}

macro_rules! add_global_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_global in module {}", module_path!());
    };
}

mkfn!{
    add_global_introspect!();
    pub (crate) fn add_global < 'a > (llmod : & 'a Module , ty : & 'a Type , name_cstr : & CStr) -> & 'a Value { unsafe { LLVMAddGlobal (llmod , ty , name_cstr . as_ptr ()) } }
}

macro_rules! set_initializer_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_initializer in module {}", module_path!());
    };
}

mkfn!{
    set_initializer_introspect!();
    pub (crate) fn set_initializer (llglobal : & Value , constant_val : & Value) { unsafe { LLVMSetInitializer (llglobal , constant_val) ; } }
}

macro_rules! set_global_constant_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_global_constant in module {}", module_path!());
    };
}

mkfn!{
    set_global_constant_introspect!();
    pub (crate) fn set_global_constant (llglobal : & Value , is_constant : bool) { LLVMSetGlobalConstant (llglobal , is_constant . to_llvm_bool ()) ; }
}

macro_rules! get_linkage_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_linkage in module {}", module_path!());
    };
}

mkfn!{
    get_linkage_introspect!();
    pub (crate) fn get_linkage (llglobal : & Value) -> Linkage { unsafe { LLVMGetLinkage (llglobal) } . to_rust () }
}

macro_rules! set_linkage_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_linkage in module {}", module_path!());
    };
}

mkfn!{
    set_linkage_introspect!();
    pub (crate) fn set_linkage (llglobal : & Value , linkage : Linkage) { unsafe { LLVMSetLinkage (llglobal , linkage) ; } }
}

macro_rules! is_declaration_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_declaration in module {}", module_path!());
    };
}

mkfn!{
    is_declaration_introspect!();
    pub (crate) fn is_declaration (llglobal : & Value) -> bool { unsafe { LLVMIsDeclaration (llglobal) } . is_true () }
}

macro_rules! get_visibility_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_visibility in module {}", module_path!());
    };
}

mkfn!{
    get_visibility_introspect!();
    pub (crate) fn get_visibility (llglobal : & Value) -> Visibility { unsafe { LLVMGetVisibility (llglobal) } . to_rust () }
}

macro_rules! set_visibility_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_visibility in module {}", module_path!());
    };
}

mkfn!{
    set_visibility_introspect!();
    pub (crate) fn set_visibility (llglobal : & Value , visibility : Visibility) { unsafe { LLVMSetVisibility (llglobal , visibility) ; } }
}

macro_rules! set_alignment_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_alignment in module {}", module_path!());
    };
}

mkfn!{
    set_alignment_introspect!();
    pub (crate) fn set_alignment (llglobal : & Value , align : Align) { unsafe { ffi :: LLVMSetAlignment (llglobal , align . bytes () as c_uint) ; } }
}

macro_rules! set_comdat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_comdat in module {}", module_path!());
    };
}

mkfn!{
    set_comdat_introspect!();
    # [doc = " Get the `name`d comdat from `llmod` and assign it to `llglobal`."] # [doc = ""] # [doc = " Inserts the comdat into `llmod` if it does not exist."] # [doc = " It is an error to call this if the target does not support comdat."] pub (crate) fn set_comdat (llmod : & Module , llglobal : & Value , name : & CStr) { unsafe { let comdat = LLVMGetOrInsertComdat (llmod , name . as_ptr ()) ; LLVMSetComdat (llglobal , comdat) ; } }
}

macro_rules! get_param_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_param in module {}", module_path!());
    };
}

mkfn!{
    get_param_introspect!();
    # [doc = " Safe wrapper around `LLVMGetParam`, because segfaults are no fun."] pub (crate) fn get_param (llfn : & Value , index : c_uint) -> & Value { unsafe { assert ! (index < LLVMCountParams (llfn) , "out of bounds argument access: {} out of {} arguments" , index , LLVMCountParams (llfn)) ; LLVMGetParam (llfn , index) } }
}

macro_rules! get_value_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_value_name in module {}", module_path!());
    };
}

mkfn!{
    get_value_name_introspect!();
    # [doc = " Safe wrapper for `LLVMGetValueName2`"] # [doc = " Needs to allocate the value, because `set_value_name` will invalidate"] # [doc = " the pointer."] pub (crate) fn get_value_name (value : & Value) -> Vec < u8 > { unsafe { let mut len = 0 ; let data = LLVMGetValueName2 (value , & mut len) ; std :: slice :: from_raw_parts (data . cast () , len) . to_vec () } }
}
mkitem!{mkstruct!{# [derive (Debug , Copy , Clone)] pub (crate) struct Intrinsic { id : NonZero < c_uint > , }}}
mkitem!{mkimpl!{impl Intrinsic { pub (crate) fn lookup (name : & [u8]) -> Option < Self > { let id = unsafe { LLVMLookupIntrinsicID (name . as_c_char_ptr () , name . len ()) } ; NonZero :: new (id) . map (| id | Self { id }) } pub (crate) fn get_declaration < 'll > (self , llmod : & 'll Module , type_params : & [& 'll Type] ,) -> & 'll Value { unsafe { LLVMGetIntrinsicDeclaration (llmod , self . id , type_params . as_ptr () , type_params . len ()) } } }}}

macro_rules! set_value_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_value_name in module {}", module_path!());
    };
}

mkfn!{
    set_value_name_introspect!();
    # [doc = " Safe wrapper for `LLVMSetValueName2` from a byte slice"] pub (crate) fn set_value_name (value : & Value , name : & [u8]) { unsafe { let data = name . as_c_char_ptr () ; LLVMSetValueName2 (value , data , name . len ()) ; } }
}

macro_rules! build_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_string in module {}", module_path!());
    };
}

mkfn!{
    build_string_introspect!();
    pub (crate) fn build_string (f : impl FnOnce (& RustString)) -> Result < String , FromUtf8Error > { String :: from_utf8 (RustString :: build_byte_buffer (f)) }
}

macro_rules! build_byte_buffer_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_byte_buffer in module {}", module_path!());
    };
}

mkfn!{
    build_byte_buffer_introspect!();
    pub (crate) fn build_byte_buffer (f : impl FnOnce (& RustString)) -> Vec < u8 > { RustString :: build_byte_buffer (f) }
}

macro_rules! twine_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function twine_to_string in module {}", module_path!());
    };
}

mkfn!{
    twine_to_string_introspect!();
    pub (crate) fn twine_to_string (tr : & Twine) -> String { unsafe { build_string (| s | LLVMRustWriteTwineToString (tr , s)) . expect ("got a non-UTF8 Twine from LLVM") } }
}

macro_rules! last_error_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function last_error in module {}", module_path!());
    };
}

mkfn!{
    last_error_introspect!();
    pub (crate) fn last_error () -> Option < String > { unsafe { let cstr = LLVMRustGetLastError () ; if cstr . is_null () { None } else { let err = CStr :: from_ptr (cstr) . to_bytes () ; let err = String :: from_utf8_lossy (err) . to_string () ; libc :: free (cstr as * mut _) ; Some (err) } } }
}
mkitem!{mkstruct!{# [doc = " Owning pointer to an [`OperandBundle`] that will dispose of the bundle"] # [doc = " when dropped."] pub (crate) struct OperandBundleBox < 'a > { raw : ptr :: NonNull < OperandBundle < 'a > > , }}}
mkitem!{mkimpl!{impl < 'a > OperandBundleBox < 'a > { pub (crate) fn new (name : & str , vals : & [& 'a Value]) -> Self { let raw = unsafe { LLVMCreateOperandBundle (name . as_c_char_ptr () , name . len () , vals . as_ptr () , vals . len () as c_uint ,) } ; Self { raw : ptr :: NonNull :: new (raw) . unwrap () } } # [doc = " Dereferences to the underlying `&OperandBundle`."] # [doc = ""] # [doc = " This can't be a `Deref` implementation because `OperandBundle` transitively"] # [doc = " contains an extern type, which is incompatible with `Deref::Target: ?Sized`."] pub (crate) fn as_ref (& self) -> & OperandBundle < 'a > { unsafe { self . raw . as_ref () } } }}}
mkitem!{mkimpl!{impl Drop for OperandBundleBox < '_ > { fn drop (& mut self) { unsafe { LLVMDisposeOperandBundle (self . raw) ; } } }}}

macro_rules! add_module_flag_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_module_flag_u32 in module {}", module_path!());
    };
}

mkfn!{
    add_module_flag_u32_introspect!();
    pub (crate) fn add_module_flag_u32 (module : & Module , merge_behavior : ModuleFlagMergeBehavior , key : & str , value : u32 ,) { unsafe { LLVMRustAddModuleFlagU32 (module , merge_behavior , key . as_c_char_ptr () , key . len () , value) ; } }
}

macro_rules! add_module_flag_str_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_module_flag_str in module {}", module_path!());
    };
}

mkfn!{
    add_module_flag_str_introspect!();
    pub (crate) fn add_module_flag_str (module : & Module , merge_behavior : ModuleFlagMergeBehavior , key : & str , value : & str ,) { unsafe { LLVMRustAddModuleFlagString (module , merge_behavior , key . as_c_char_ptr () , key . len () , value . as_c_char_ptr () , value . len () ,) ; } }
}

macro_rules! set_dllimport_storage_class_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_dllimport_storage_class in module {}", module_path!());
    };
}

mkfn!{
    set_dllimport_storage_class_introspect!();
    pub (crate) fn set_dllimport_storage_class < 'll > (v : & 'll Value) { unsafe { LLVMSetDLLStorageClass (v , DLLStorageClass :: DllImport) ; } }
}

macro_rules! set_dso_local_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_dso_local in module {}", module_path!());
    };
}

mkfn!{
    set_dso_local_introspect!();
    pub (crate) fn set_dso_local < 'll > (v : & 'll Value) { unsafe { LLVMRustSetDSOLocal (v , true) ; } }
}

macro_rules! append_module_inline_asm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function append_module_inline_asm in module {}", module_path!());
    };
}

mkfn!{
    append_module_inline_asm_introspect!();
    # [doc = " Safe wrapper for `LLVMAppendModuleInlineAsm`, which delegates to"] # [doc = " `Module::appendModuleInlineAsm`."] pub (crate) fn append_module_inline_asm < 'll > (llmod : & 'll Module , asm : & [u8]) { unsafe { LLVMAppendModuleInlineAsm (llmod , asm . as_ptr () , asm . len ()) ; } }
}