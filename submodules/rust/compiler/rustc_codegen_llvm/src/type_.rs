mkuse!{use std :: sync :: Mutex ;}
mkuse!{use std :: collections :: HashMap ;}
mkuse!{use std :: sync :: LazyLock ;}
mkitem!{static USE_MATRIX : LazyLock < Mutex < HashMap < String , Vec < String > > > > = LazyLock :: new (| | Mutex :: new (HashMap :: new ())) ;}

macro_rules! get_use_matrix_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_use_matrix in module {}", module_path!());
    };
}

mkfn!{
    get_use_matrix_introspect!();
    pub fn get_use_matrix () -> HashMap < String , Vec < String > > { USE_MATRIX . lock () . unwrap () . clone () }
}
mkitem!{macro_rules ! emit_message { ($ ($ arg : tt) *) => { { use std :: fs :: OpenOptions ; use std :: io :: Write ; let message = format ! ($ ($ arg) *) ; if let Ok (mut file) = OpenOptions :: new () . create (true) . append (true) . open ("macro_report.txt") { let _ = writeln ! (file , "{}" , message) ; } } } ; }}
mkitem!{macro_rules ! mkfn { ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: catch_fatal_errors_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: catch_fatal_errors_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn parse_crate_attrs < $ lifetime : lifetime > ($ ($ param : tt) *) -> PResult < $ lifetime2 : lifetime , ast :: AttrVec > $ body : block) => { $ (#[$ attr]) * fn parse_crate_attrs < $ lifetime > ($ ($ param) *) -> PResult < $ lifetime2 , ast :: AttrVec > { $ introspect ; emit_message ! ("🚀 MARKER: parse_crate_attrs_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: parse_crate_attrs_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn init_logger_with_additional_layer < F , T > ($ ($ param : tt) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , $ body : block) => { $ (#[$ attr]) * fn init_logger_with_additional_layer < F , T > ($ ($ param) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , { $ introspect ; emit_message ! ("🚀 MARKER: init_logger_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: init_logger_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < F , T > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where F : FnOnce ($ ($ fnonce_args : tt) *) $ ($ where_rest : tt) * $ body : block) => { $ (#[$ attr]) * fn $ name < F , T > ($ ($ param) *) $ (-> $ ret) ? where F : FnOnce ($ ($ fnonce_args) *) $ ($ where_rest) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where $ ($ where_clause : tt) * $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? where $ ($ where_clause) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub ($ vis : ident) fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub ($ vis) fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_vis - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_vis - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; fn $ name : ident () $ body : block) => { fn $ name () { $ introspect ; emit_message ! ("🚀 MARKER: simple - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: simple - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ ($ anything : tt) *) => { $ ($ anything) * } ; }}
mkitem!{macro_rules ! safe_println { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{macro_rules ! safe_print { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{#[macro_export] macro_rules ! include_rust_compiler { ($ crate_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ crate_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_" , $ file , ".rs")) ; } ; ($ crate_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_lib.rs")) ; } ; }}
mkitem!{#[macro_export] macro_rules ! include_rust_library { ($ lib_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ lib_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_" , $ file , ".rs")) ; } ; ($ lib_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_lib.rs")) ; } ; }}
mkitem!{#[macro_export] macro_rules ! include_processed { ($ path : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_" , $ path , ".rs")) ; } ; }}
mkitem!{macro_rules ! mkinclude { ($ path : ident) => { } ; ($ path : literal) => { include ! ($ path) } ; }}
mkitem!{macro_rules ! mkitem { (include ! ($ path : ident) ;) => { } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ string_lit : literal }) => { $ macro_name :: $ macro_sub ! { $ string_lit } } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ ($ args : tt) * }) => { $ macro_name :: $ macro_sub ! { $ ($ args) * } } ; ($ macro_name : ident ! { $ ($ args : tt) * }) => { $ macro_name ! { $ ($ args) * } } ; ($ item : item) => { $ item } ; }}
mkitem!{#[macro_export] macro_rules ! mkmod { ($ name : ident , { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (pub mod $ name : ident { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; pub mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (mod $ name : ident { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; }}
mkitem!{#[macro_export] macro_rules ! mkuse { ($ use_stmt : item) => { emit_message ! ("USE|{}|{}" , module_path ! () , stringify ! ($ use_stmt)) ; $ use_stmt } ; }}
mkitem!{macro_rules ! mkstruct { ($ struct_def : item) => { $ struct_def } ; }}
mkitem!{macro_rules ! mkenum { ($ enum_def : item) => { $ enum_def } ; }}
mkitem!{macro_rules ! mktrait { ($ trait_def : item) => { $ trait_def } ; }}
mkitem!{macro_rules ! mkimpl { ($ impl_def : item) => { $ impl_def } ; }}
mkitem!{macro_rules ! getname { ($ name : ident) => { stringify ! ($ name) } ; }}
mkitem!{macro_rules ! getsrc { ($ name : ident) => { "processed file" } ; }}
mkitem!{macro_rules ! getpath { ($ name : ident) => { "processed_path" } ; }}
mkitem!{macro_rules ! get_deps { ($ name : ident) => { vec ! [] } ; }}
mkmod!{rustc_complete, { 
                getname!(rustc_complete);
                getsrc!(rustc_complete);
                getpath!(rustc_complete);
                get_deps!(rustc_complete);
                get_crates!(rustc_complete);
                mkinclude!(rustc_complete);
                mkmod!{emitter, { 
                getname!(emitter);
                getsrc!(emitter);
                getpath!(emitter);
                get_deps!(emitter);
                get_crates!(emitter);
                mkinclude!(emitter);
                
macro_rules! stderr_destination_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stderr_destination in module {}", module_path!());
    };
}

mkfn!{
    stderr_destination_introspect!();
    pub fn stderr_destination () { }
} 
            }}
mkmod!{registry, { 
                getname!(registry);
                getsrc!(registry);
                getpath!(registry);
                get_deps!(registry);
                get_crates!(registry);
                mkinclude!(registry);
                mkitem!{mkstruct!{pub struct Registry ;}} 
            }}
mkmod!{translation, { 
                getname!(translation);
                getsrc!(translation);
                getpath!(translation);
                get_deps!(translation);
                get_crates!(translation);
                mkinclude!(translation);
                mkitem!{mkstruct!{pub struct Translator ;}} 
            }}
mkitem!{mkstruct!{pub struct ColorConfig ;}}
mkitem!{mkstruct!{pub struct DiagCtxt ;}}
mkitem!{mkstruct!{pub struct ErrCode ;}}
mkitem!{mkstruct!{pub struct FatalError ;}}
mkitem!{mkstruct!{pub struct PResult < T > (pub T) ;}}
mkmod!{markdown, { 
                getname!(markdown);
                getsrc!(markdown);
                getpath!(markdown);
                get_deps!(markdown);
                get_crates!(markdown);
                mkinclude!(markdown);
                 
            }}
mkmod!{config, { 
                getname!(config);
                getsrc!(config);
                getpath!(config);
                get_deps!(config);
                get_crates!(config);
                mkinclude!(config);
                mkitem!{mkstruct!{pub struct CG_OPTIONS ;}}
mkitem!{mkstruct!{pub struct CrateType ;}}
mkitem!{mkstruct!{pub struct ErrorOutputType ;}}
mkitem!{mkstruct!{pub struct Input ;}}
mkitem!{mkstruct!{pub struct OptionDesc ;}}
mkitem!{mkstruct!{pub struct OutFileName ;}}
mkitem!{mkstruct!{pub struct OutputType ;}}
mkitem!{mkstruct!{pub struct Sysroot ;}}
mkitem!{mkstruct!{pub struct UnstableOptions ;}}
mkitem!{mkstruct!{pub struct Z_OPTIONS ;}}

macro_rules! nightly_options_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function nightly_options in module {}", module_path!());
    };
}

mkfn!{
    nightly_options_introspect!();
    pub fn nightly_options () { }
}

macro_rules! parse_target_triple_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_target_triple in module {}", module_path!());
    };
}

mkfn!{
    parse_target_triple_introspect!();
    pub fn parse_target_triple () { }
} 
            }}
mkmod!{getopts, { 
                getname!(getopts);
                getsrc!(getopts);
                getpath!(getopts);
                get_deps!(getopts);
                get_crates!(getopts);
                mkinclude!(getopts);
                mkitem!{mkstruct!{pub struct Matches ;}} 
            }}
mkmod!{lint, { 
                getname!(lint);
                getsrc!(lint);
                getpath!(lint);
                get_deps!(lint);
                get_crates!(lint);
                mkinclude!(lint);
                mkitem!{mkstruct!{pub struct Lint ;}}
mkitem!{mkstruct!{pub struct LintId ;}} 
            }}
mkmod!{output, { 
                getname!(output);
                getsrc!(output);
                getpath!(output);
                get_deps!(output);
                get_crates!(output);
                mkinclude!(output);
                mkitem!{mkstruct!{pub struct CRATE_TYPES ;}}

macro_rules! collect_crate_types_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_crate_types in module {}", module_path!());
    };
}

mkfn!{
    collect_crate_types_introspect!();
    pub fn collect_crate_types () { }
}

macro_rules! invalid_output_for_target_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function invalid_output_for_target in module {}", module_path!());
    };
}

mkfn!{
    invalid_output_for_target_introspect!();
    pub fn invalid_output_for_target () { }
} 
            }}
mkitem!{mkstruct!{pub struct EarlyDiagCtxt ;}}
mkitem!{mkstruct!{pub struct Session ;}}
mkitem!{mkstruct!{pub struct FileName ;}}
mkmod!{def_id, { 
                getname!(def_id);
                getsrc!(def_id);
                getpath!(def_id);
                get_deps!(def_id);
                get_crates!(def_id);
                mkinclude!(def_id);
                mkitem!{mkstruct!{pub struct LOCAL_CRATE ;}} 
            }}
mkmod!{ty, { 
                getname!(ty);
                getsrc!(ty);
                getpath!(ty);
                get_deps!(ty);
                get_crates!(ty);
                mkinclude!(ty);
                mkitem!{mkstruct!{pub struct TyCtxt < T > (pub T) ;}} 
            }} 
            }}
mkmod!{session_diagnostics, { 
                getname!(session_diagnostics);
                getsrc!(session_diagnostics);
                getpath!(session_diagnostics);
                get_deps!(session_diagnostics);
                get_crates!(session_diagnostics);
                mkinclude!(session_diagnostics);
                mkitem!{mkstruct!{pub struct CantEmitMIR ;}}
mkitem!{mkstruct!{pub struct RLinkEmptyVersionNumber ;}}
mkitem!{mkstruct!{pub struct RLinkEncodingVersionMismatch ;}}
mkitem!{mkstruct!{pub struct RLinkRustcVersionMismatch ;}}
mkitem!{mkstruct!{pub struct RLinkWrongFileType ;}}
mkitem!{mkstruct!{pub struct RlinkCorruptFile ;}}
mkitem!{mkstruct!{pub struct RlinkNotAFile ;}}
mkitem!{mkstruct!{pub struct RlinkUnableToRead ;}}
mkitem!{mkstruct!{pub struct UnstableFeatureUsage ;}} 
            }}
mkitem!{macro_rules ! do_not_use_print { ($ ($ t : tt) *) => { compile_error ! ("Don't use print") } ; }}
mkitem!{macro_rules ! do_not_use_safe_print { ($ ($ t : tt) *) => { compile_error ! ("Don't use safe_print") } ; }}
mkitem!{macro_rules ! mktrait { ($ trait_def : item) => { $ trait_def } ; }}
mkitem!{macro_rules ! mkimpl { ($ impl_def : item) => { $ impl_def } ; }}
mkitem!{macro_rules ! getname { ($ name : ident) => { pub fn get_module_name () -> &'static str { stringify ! ($ name) } } ; }}
mkitem!{macro_rules ! getsrc { ($ name : ident) => { pub fn get_source_info () -> &'static str { concat ! ("Module: " , stringify ! ($ name)) } } ; }}
mkitem!{macro_rules ! getpath { ($ name : ident) => { pub fn get_module_path () -> &'static str { module_path ! () } } ; }}
mkitem!{macro_rules ! get_deps { ($ name : ident) => { pub fn get_dependencies () -> &'static [&'static str] { & [] } } ; }}
mkitem!{macro_rules ! get_crates { ($ name : ident) => { pub fn get_required_crates () -> &'static [&'static str] { & [] } } ; }}
mkitem!{macro_rules ! forall_crates { ($ ($ crate_name : ident) ,*) => { $ (extern crate $ crate_name ;) * } ; }}
mkitem!{macro_rules ! emit_extern { ($ crate_name : ident) => { extern crate $ crate_name ; } ; }}
mkitem!{macro_rules ! get_externs { ($ crate_name : ident) => { stringify ! ($ crate_name) } ; }}
mkuse!{use std :: borrow :: Borrow ;}
mkuse!{use std :: hash :: { Hash , Hasher } ;}
mkuse!{use std :: { fmt , ptr } ;}
mkuse!{use libc :: c_uint ;}
mkuse!{use rustc_abi :: { AddressSpace , Align , Integer , Reg , Size } ;}
mkuse!{use rustc_codegen_ssa :: common :: TypeKind ;}
mkuse!{use rustc_codegen_ssa :: traits :: * ;}
mkuse!{use rustc_data_structures :: small_c_str :: SmallCStr ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: layout :: TyAndLayout ;}
mkuse!{use rustc_middle :: ty :: { self , Ty } ;}
mkuse!{use rustc_target :: callconv :: { CastTarget , FnAbi } ;}
mkuse!{use crate :: abi :: { FnAbiLlvmExt , LlvmType } ;}
mkuse!{use crate :: context :: { CodegenCx , GenericCx , SCx } ;}
mkuse!{pub (crate) use crate :: llvm :: Type ;}
mkuse!{use crate :: llvm :: { FALSE , Metadata , TRUE , ToLlvmBool } ;}
mkuse!{use crate :: type_of :: LayoutLlvmExt ;}
mkuse!{use crate :: value :: Value ;}
mkuse!{use crate :: { common , llvm } ;}
mkitem!{mkimpl!{impl PartialEq for Type { fn eq (& self , other : & Self) -> bool { ptr :: eq (self , other) } }}}
mkitem!{mkimpl!{impl Eq for Type { }}}
mkitem!{mkimpl!{impl Hash for Type { fn hash < H : Hasher > (& self , state : & mut H) { ptr :: hash (self , state) ; } }}}
mkitem!{mkimpl!{impl fmt :: Debug for Type { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (& llvm :: build_string (| s | unsafe { llvm :: LLVMRustWriteTypeToString (self , s) ; }) . expect ("non-UTF8 type description from LLVM") ,) } }}}
mkitem!{mkimpl!{impl < 'll > CodegenCx < 'll , '_ > { }}}
mkitem!{mkimpl!{impl < 'll , CX : Borrow < SCx < 'll > > > GenericCx < 'll , CX > { pub (crate) fn type_named_struct (& self , name : & str) -> & 'll Type { let name = SmallCStr :: new (name) ; unsafe { llvm :: LLVMStructCreateNamed (self . llcx () , name . as_ptr ()) } } pub (crate) fn set_struct_body (& self , ty : & 'll Type , els : & [& 'll Type] , packed : bool) { unsafe { llvm :: LLVMStructSetBody (ty , els . as_ptr () , els . len () as c_uint , packed . to_llvm_bool ()) } } pub (crate) fn type_void (& self) -> & 'll Type { unsafe { llvm :: LLVMVoidTypeInContext (self . llcx ()) } } #[doc = "x Creates an integer type with the given number of bits, e.g., i24"] pub (crate) fn type_ix (& self , num_bits : u64) -> & 'll Type { unsafe { llvm :: LLVMIntTypeInContext (self . llcx () , num_bits as c_uint) } } pub (crate) fn type_vector (& self , ty : & 'll Type , len : u64) -> & 'll Type { unsafe { llvm :: LLVMVectorType (ty , len as c_uint) } } pub (crate) fn func_params_types (& self , ty : & 'll Type) -> Vec < & 'll Type > { unsafe { let n_args = llvm :: LLVMCountParamTypes (ty) as usize ; let mut args = Vec :: with_capacity (n_args) ; llvm :: LLVMGetParamTypes (ty , args . as_mut_ptr ()) ; args . set_len (n_args) ; args } } }}}
mkitem!{mkimpl!{impl < 'll , 'tcx > CodegenCx < 'll , 'tcx > { pub (crate) fn type_bool (& self) -> & 'll Type { self . type_i8 () } pub (crate) fn type_int_from_ty (& self , t : ty :: IntTy) -> & 'll Type { match t { ty :: IntTy :: Isize => self . type_isize () , ty :: IntTy :: I8 => self . type_i8 () , ty :: IntTy :: I16 => self . type_i16 () , ty :: IntTy :: I32 => self . type_i32 () , ty :: IntTy :: I64 => self . type_i64 () , ty :: IntTy :: I128 => self . type_i128 () , } } pub (crate) fn type_uint_from_ty (& self , t : ty :: UintTy) -> & 'll Type { match t { ty :: UintTy :: Usize => self . type_isize () , ty :: UintTy :: U8 => self . type_i8 () , ty :: UintTy :: U16 => self . type_i16 () , ty :: UintTy :: U32 => self . type_i32 () , ty :: UintTy :: U64 => self . type_i64 () , ty :: UintTy :: U128 => self . type_i128 () , } } pub (crate) fn type_float_from_ty (& self , t : ty :: FloatTy) -> & 'll Type { match t { ty :: FloatTy :: F16 => self . type_f16 () , ty :: FloatTy :: F32 => self . type_f32 () , ty :: FloatTy :: F64 => self . type_f64 () , ty :: FloatTy :: F128 => self . type_f128 () , } } #[doc = " Return an LLVM type that has at most the required alignment,"] #[doc = " and exactly the required size, as a best-effort padding array."] pub (crate) fn type_padding_filler (& self , size : Size , align : Align) -> & 'll Type { let unit = Integer :: approximate_align (self , align) ; let size = size . bytes () ; let unit_size = unit . size () . bytes () ; assert_eq ! (size % unit_size , 0) ; self . type_array (self . type_from_integer (unit) , size / unit_size) } }}}
mkitem!{mkimpl!{impl < 'll , CX : Borrow < SCx < 'll > > > GenericCx < 'll , CX > { pub (crate) fn llcx (& self) -> & 'll llvm :: Context { (* * self) . borrow () . llcx } pub (crate) fn llmod (& self) -> & 'll llvm :: Module { (* * self) . borrow () . llmod } pub (crate) fn isize_ty (& self) -> & 'll Type { (* * self) . borrow () . isize_ty } pub (crate) fn type_variadic_func (& self , args : & [& 'll Type] , ret : & 'll Type) -> & 'll Type { unsafe { llvm :: LLVMFunctionType (ret , args . as_ptr () , args . len () as c_uint , TRUE) } } pub (crate) fn type_i1 (& self) -> & 'll Type { unsafe { llvm :: LLVMInt1TypeInContext (self . llcx ()) } } pub (crate) fn type_struct (& self , els : & [& 'll Type] , packed : bool) -> & 'll Type { unsafe { llvm :: LLVMStructTypeInContext (self . llcx () , els . as_ptr () , els . len () as c_uint , packed . to_llvm_bool () ,) } } }}}
mkitem!{mkimpl!{impl < 'll , CX : Borrow < SCx < 'll > > > BaseTypeCodegenMethods for GenericCx < 'll , CX > { fn type_i8 (& self) -> & 'll Type { unsafe { llvm :: LLVMInt8TypeInContext (self . llcx ()) } } fn type_i16 (& self) -> & 'll Type { unsafe { llvm :: LLVMInt16TypeInContext (self . llcx ()) } } fn type_i32 (& self) -> & 'll Type { unsafe { llvm :: LLVMInt32TypeInContext (self . llcx ()) } } fn type_i64 (& self) -> & 'll Type { unsafe { llvm :: LLVMInt64TypeInContext (self . llcx ()) } } fn type_i128 (& self) -> & 'll Type { unsafe { llvm :: LLVMIntTypeInContext (self . llcx () , 128) } } fn type_isize (& self) -> & 'll Type { self . isize_ty () } fn type_f16 (& self) -> & 'll Type { unsafe { llvm :: LLVMHalfTypeInContext (self . llcx ()) } } fn type_f32 (& self) -> & 'll Type { unsafe { llvm :: LLVMFloatTypeInContext (self . llcx ()) } } fn type_f64 (& self) -> & 'll Type { unsafe { llvm :: LLVMDoubleTypeInContext (self . llcx ()) } } fn type_f128 (& self) -> & 'll Type { unsafe { llvm :: LLVMFP128TypeInContext (self . llcx ()) } } fn type_func (& self , args : & [& 'll Type] , ret : & 'll Type) -> & 'll Type { unsafe { llvm :: LLVMFunctionType (ret , args . as_ptr () , args . len () as c_uint , FALSE) } } fn type_kind (& self , ty : & 'll Type) -> TypeKind { llvm :: LLVMGetTypeKind (ty) . to_rust () . to_generic () } fn type_ptr (& self) -> & 'll Type { self . type_ptr_ext (AddressSpace :: ZERO) } fn type_ptr_ext (& self , address_space : AddressSpace) -> & 'll Type { unsafe { llvm :: LLVMPointerTypeInContext (self . llcx () , address_space . 0) } } fn element_type (& self , ty : & 'll Type) -> & 'll Type { match self . type_kind (ty) { TypeKind :: Array | TypeKind :: Vector => unsafe { llvm :: LLVMGetElementType (ty) } , TypeKind :: Pointer => bug ! ("element_type is not supported for opaque pointers") , other => bug ! ("element_type called on unsupported type {other:?}") , } } fn vector_length (& self , ty : & 'll Type) -> usize { unsafe { llvm :: LLVMGetVectorSize (ty) as usize } } fn float_width (& self , ty : & 'll Type) -> usize { match self . type_kind (ty) { TypeKind :: Half => 16 , TypeKind :: Float => 32 , TypeKind :: Double => 64 , TypeKind :: X86_FP80 => 80 , TypeKind :: FP128 | TypeKind :: PPC_FP128 => 128 , other => bug ! ("llvm_float_width called on a non-float type {other:?}") , } } fn int_width (& self , ty : & 'll Type) -> u64 { unsafe { llvm :: LLVMGetIntTypeWidth (ty) as u64 } } fn val_ty (& self , v : & 'll Value) -> & 'll Type { common :: val_ty (v) } fn type_array (& self , ty : & 'll Type , len : u64) -> & 'll Type { unsafe { llvm :: LLVMArrayType2 (ty , len) } } }}}
mkitem!{mkimpl!{impl Type { #[doc = " Creates an integer type with the given number of bits, e.g., i24"] pub (crate) fn ix_llcx (llcx : & llvm :: Context , num_bits : u64) -> & Type { unsafe { llvm :: LLVMIntTypeInContext (llcx , num_bits as c_uint) } } pub (crate) fn ptr_llcx (llcx : & llvm :: Context) -> & Type { unsafe { llvm :: LLVMPointerTypeInContext (llcx , AddressSpace :: ZERO . 0) } } }}}
mkitem!{mkimpl!{impl < 'll , 'tcx > LayoutTypeCodegenMethods < 'tcx > for CodegenCx < 'll , 'tcx > { fn backend_type (& self , layout : TyAndLayout < 'tcx >) -> & 'll Type { layout . llvm_type (self) } fn immediate_backend_type (& self , layout : TyAndLayout < 'tcx >) -> & 'll Type { layout . immediate_llvm_type (self) } fn is_backend_immediate (& self , layout : TyAndLayout < 'tcx >) -> bool { layout . is_llvm_immediate () } fn is_backend_scalar_pair (& self , layout : TyAndLayout < 'tcx >) -> bool { layout . is_llvm_scalar_pair () } fn scalar_pair_element_backend_type (& self , layout : TyAndLayout < 'tcx > , index : usize , immediate : bool ,) -> & 'll Type { layout . scalar_pair_element_llvm_type (self , index , immediate) } fn cast_backend_type (& self , ty : & CastTarget) -> & 'll Type { ty . llvm_type (self) } fn fn_decl_backend_type (& self , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > >) -> & 'll Type { fn_abi . llvm_type (self) } fn fn_ptr_backend_type (& self , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > >) -> & 'll Type { fn_abi . ptr_to_llvm_type (self) } fn reg_backend_type (& self , ty : & Reg) -> & 'll Type { ty . llvm_type (self) } }}}
mkitem!{mkimpl!{impl < 'll , 'tcx > TypeMembershipCodegenMethods < 'tcx > for CodegenCx < 'll , 'tcx > { fn add_type_metadata (& self , function : & 'll Value , typeid : & [u8]) { let typeid_metadata = self . create_metadata (typeid) ; unsafe { let v = [llvm :: LLVMValueAsMetadata (self . const_usize (0)) , typeid_metadata] ; llvm :: LLVMRustGlobalAddMetadata (function , llvm :: MD_type as c_uint , llvm :: LLVMMDNodeInContext2 (self . llcx , v . as_ptr () , v . len ()) ,) } } fn set_type_metadata (& self , function : & 'll Value , typeid : & [u8]) { let typeid_metadata = self . create_metadata (typeid) ; unsafe { let v = [llvm :: LLVMValueAsMetadata (self . const_usize (0)) , typeid_metadata] ; llvm :: LLVMGlobalSetMetadata (function , llvm :: MD_type as c_uint , llvm :: LLVMMDNodeInContext2 (self . llcx , v . as_ptr () , v . len ()) ,) } } fn typeid_metadata (& self , typeid : & [u8]) -> Option < & 'll Metadata > { Some (self . create_metadata (typeid)) } fn add_kcfi_type_metadata (& self , function : & 'll Value , kcfi_typeid : u32) { let kcfi_type_metadata = self . const_u32 (kcfi_typeid) ; unsafe { llvm :: LLVMRustGlobalAddMetadata (function , llvm :: MD_kcfi_type as c_uint , llvm :: LLVMMDNodeInContext2 (self . llcx , & llvm :: LLVMValueAsMetadata (kcfi_type_metadata) , 1 ,) ,) } } fn set_kcfi_type_metadata (& self , function : & 'll Value , kcfi_typeid : u32) { let kcfi_type_metadata = self . const_u32 (kcfi_typeid) ; unsafe { llvm :: LLVMGlobalSetMetadata (function , llvm :: MD_kcfi_type as c_uint , llvm :: LLVMMDNodeInContext2 (self . llcx , & llvm :: LLVMValueAsMetadata (kcfi_type_metadata) , 1 ,) ,) } } }}}