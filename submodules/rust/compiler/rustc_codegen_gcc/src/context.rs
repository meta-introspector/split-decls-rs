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
mkuse!{use std :: cell :: { Cell , RefCell } ;}
mkuse!{use std :: collections :: HashMap ;}
mkuse!{use gccjit :: { Block , CType , Context , Function , FunctionPtrType , FunctionType , LValue , Location , RValue , Type , } ;}
mkuse!{use rustc_abi :: { Align , HasDataLayout , PointeeInfo , Size , TargetDataLayout , VariantIdx } ;}
mkuse!{use rustc_codegen_ssa :: base :: wants_msvc_seh ;}
mkuse!{use rustc_codegen_ssa :: errors as ssa_errors ;}
mkuse!{use rustc_codegen_ssa :: traits :: { BackendTypes , BaseTypeCodegenMethods , MiscCodegenMethods } ;}
mkuse!{use rustc_data_structures :: base_n :: { ALPHANUMERIC_ONLY , ToBaseN } ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashMap , FxHashSet } ;}
mkuse!{use rustc_middle :: mir :: interpret :: Allocation ;}
mkuse!{use rustc_middle :: mir :: mono :: CodegenUnit ;}
mkuse!{use rustc_middle :: span_bug ;}
mkuse!{use rustc_middle :: ty :: layout :: { FnAbiError , FnAbiOf , FnAbiOfHelpers , FnAbiRequest , HasTyCtxt , HasTypingEnv , LayoutError , LayoutOfHelpers , } ;}
mkuse!{use rustc_middle :: ty :: { self , ExistentialTraitRef , Instance , Ty , TyCtxt } ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_span :: source_map :: respan ;}
mkuse!{use rustc_span :: { DUMMY_SP , Span } ;}
mkuse!{use rustc_target :: spec :: { HasTargetSpec , HasX86AbiOpt , Target , TlsModel , X86Abi } ;}
mkuse!{#[cfg (feature = "master")] use crate :: abi :: conv_to_fn_attribute ;}
mkuse!{use crate :: callee :: get_fn ;}
mkuse!{use crate :: common :: SignType ;}
mkitem!{mkstruct!{#[cfg_attr (not (feature = "master") , allow (dead_code))] pub struct CodegenCx < 'gcc , 'tcx > { #[doc = " A cache of converted ConstAllocs"] pub const_cache : RefCell < HashMap < Allocation , RValue < 'gcc > > > , pub codegen_unit : & 'tcx CodegenUnit < 'tcx > , pub context : & 'gcc Context < 'gcc > , pub current_func : RefCell < Option < Function < 'gcc > > > , pub normal_function_addresses : RefCell < FxHashSet < RValue < 'gcc > > > , pub function_address_names : RefCell < FxHashMap < RValue < 'gcc > , String > > , pub functions : RefCell < FxHashMap < String , Function < 'gcc > > > , pub intrinsics : RefCell < FxHashMap < String , Function < 'gcc > > > , pub tls_model : gccjit :: TlsModel , pub bool_type : Type < 'gcc > , pub i8_type : Type < 'gcc > , pub i16_type : Type < 'gcc > , pub i32_type : Type < 'gcc > , pub i64_type : Type < 'gcc > , pub i128_type : Type < 'gcc > , pub isize_type : Type < 'gcc > , pub u8_type : Type < 'gcc > , pub u16_type : Type < 'gcc > , pub u32_type : Type < 'gcc > , pub u64_type : Type < 'gcc > , pub u128_type : Type < 'gcc > , pub usize_type : Type < 'gcc > , pub char_type : Type < 'gcc > , pub uchar_type : Type < 'gcc > , pub short_type : Type < 'gcc > , pub ushort_type : Type < 'gcc > , pub int_type : Type < 'gcc > , pub uint_type : Type < 'gcc > , pub long_type : Type < 'gcc > , pub ulong_type : Type < 'gcc > , pub longlong_type : Type < 'gcc > , pub ulonglong_type : Type < 'gcc > , pub sizet_type : Type < 'gcc > , pub supports_128bit_integers : bool , pub supports_f16_type : bool , pub supports_f32_type : bool , pub supports_f64_type : bool , pub supports_f128_type : bool , pub float_type : Type < 'gcc > , pub double_type : Type < 'gcc > , pub linkage : Cell < FunctionType > , pub scalar_types : RefCell < FxHashMap < Ty < 'tcx > , Type < 'gcc > > > , pub types : RefCell < FxHashMap < (Ty < 'tcx > , Option < VariantIdx >) , Type < 'gcc > > > , pub tcx : TyCtxt < 'tcx > , pub struct_types : RefCell < FxHashMap < Vec < Type < 'gcc > > , Type < 'gcc > > > , #[doc = " Cache instances of monomorphic and polymorphic items"] pub instances : RefCell < FxHashMap < Instance < 'tcx > , LValue < 'gcc > > > , #[doc = " Cache function instances of monomorphic and polymorphic items"] pub function_instances : RefCell < FxHashMap < Instance < 'tcx > , Function < 'gcc > > > , #[doc = " Cache generated vtables"] pub vtables : RefCell < FxHashMap < (Ty < 'tcx > , Option < ty :: ExistentialTraitRef < 'tcx > >) , RValue < 'gcc > > > , #[doc = " Mapping from function pointer type to indexes of on stack parameters."] pub on_stack_params : RefCell < FxHashMap < FunctionPtrType < 'gcc > , FxHashSet < usize > > > , #[doc = " Mapping from function to indexes of on stack parameters."] pub on_stack_function_params : RefCell < FxHashMap < Function < 'gcc > , FxHashSet < usize > > > , #[doc = " Cache of emitted const globals (value -> global)"] pub const_globals : RefCell < FxHashMap < RValue < 'gcc > , RValue < 'gcc > > > , #[doc = " Map from the address of a global variable (rvalue) to the global variable itself (lvalue)."] #[doc = " TODO(antoyo): remove when the rustc API is fixed."] pub global_lvalues : RefCell < FxHashMap < RValue < 'gcc > , LValue < 'gcc > > > , #[doc = " Cache of constant strings,"] pub const_str_cache : RefCell < FxHashMap < String , LValue < 'gcc > > > , #[doc = " Cache of globals."] pub globals : RefCell < FxHashMap < String , RValue < 'gcc > > > , #[doc = " A counter that is used for generating local symbol names"] local_gen_sym_counter : Cell < usize > , eh_personality : Cell < Option < Function < 'gcc > > > , #[cfg (feature = "master")] pub rust_try_fn : Cell < Option < (Type < 'gcc > , Function < 'gcc >) > > , pub pointee_infos : RefCell < FxHashMap < (Ty < 'tcx > , Size) , Option < PointeeInfo > > > , #[cfg (feature = "master")] pub cleanup_blocks : RefCell < FxHashSet < Block < 'gcc > > > , #[doc = " The alignment of a u128/i128 type."] pub int128_align : Align , }}}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > CodegenCx < 'gcc , 'tcx > { #[allow (clippy :: too_many_arguments)] pub fn new (context : & 'gcc Context < 'gcc > , codegen_unit : & 'tcx CodegenUnit < 'tcx > , tcx : TyCtxt < 'tcx > , supports_128bit_integers : bool , supports_f16_type : bool , supports_f32_type : bool , supports_f64_type : bool , supports_f128_type : bool ,) -> Self { let create_type = | ctype , rust_type | { let layout = tcx . layout_of (ty :: TypingEnv :: fully_monomorphized () . as_query_input (rust_type)) . unwrap () ; let align = layout . align . abi . bytes () ; if layout . size . bytes () == 1 { return context . new_c_type (ctype) ; } #[cfg (feature = "master")] { context . new_c_type (ctype) . get_aligned (align) } #[cfg (not (feature = "master"))] { if layout . ty . int_size_and_signed (tcx) . 0 . bytes () == 16 { context . new_c_type (ctype) . get_aligned (align) } else { context . new_c_type (ctype) } } } ; let i8_type = create_type (CType :: Int8t , tcx . types . i8) ; let i16_type = create_type (CType :: Int16t , tcx . types . i16) ; let i32_type = create_type (CType :: Int32t , tcx . types . i32) ; let i64_type = create_type (CType :: Int64t , tcx . types . i64) ; let u8_type = create_type (CType :: UInt8t , tcx . types . u8) ; let u16_type = create_type (CType :: UInt16t , tcx . types . u16) ; let u32_type = create_type (CType :: UInt32t , tcx . types . u32) ; let u64_type = create_type (CType :: UInt64t , tcx . types . u64) ; let (i128_type , u128_type) = if supports_128bit_integers { let i128_type = create_type (CType :: Int128t , tcx . types . i128) ; let u128_type = create_type (CType :: UInt128t , tcx . types . u128) ; (i128_type , u128_type) } else { let i128_type = context . new_array_type (None , i64_type , 2) ; let u128_type = context . new_array_type (None , u64_type , 2) ; (i128_type , u128_type) } ; let tls_model = to_gcc_tls_mode (tcx . sess . tls_model ()) ; let float_type = context . new_type :: < f32 > () ; let double_type = context . new_type :: < f64 > () ; let char_type = context . new_c_type (CType :: Char) ; let uchar_type = context . new_c_type (CType :: UChar) ; let short_type = context . new_c_type (CType :: Short) ; let ushort_type = context . new_c_type (CType :: UShort) ; let int_type = context . new_c_type (CType :: Int) ; let uint_type = context . new_c_type (CType :: UInt) ; let long_type = context . new_c_type (CType :: Long) ; let ulong_type = context . new_c_type (CType :: ULong) ; let longlong_type = context . new_c_type (CType :: LongLong) ; let ulonglong_type = context . new_c_type (CType :: ULongLong) ; let sizet_type = context . new_c_type (CType :: SizeT) ; let usize_type = sizet_type ; let isize_type = usize_type ; let bool_type = context . new_type :: < bool > () ; let mut functions = FxHashMap :: default () ; let builtins = ["abort"] ; for builtin in builtins . iter () { functions . insert (builtin . to_string () , context . get_builtin_function (builtin)) ; } let mut cx = Self { int128_align : tcx . layout_of (ty :: TypingEnv :: fully_monomorphized () . as_query_input (tcx . types . i128)) . expect ("Can't get the layout of `i128`") . align . abi , const_cache : Default :: default () , codegen_unit , context , current_func : RefCell :: new (None) , normal_function_addresses : Default :: default () , function_address_names : Default :: default () , functions : RefCell :: new (functions) , intrinsics : RefCell :: new (FxHashMap :: default ()) , tls_model , bool_type , i8_type , i16_type , i32_type , i64_type , i128_type , isize_type , usize_type , u8_type , u16_type , u32_type , u64_type , u128_type , char_type , uchar_type , short_type , ushort_type , int_type , uint_type , long_type , ulong_type , longlong_type , ulonglong_type , sizet_type , supports_128bit_integers , supports_f16_type , supports_f32_type , supports_f64_type , supports_f128_type , float_type , double_type , linkage : Cell :: new (FunctionType :: Internal) , instances : Default :: default () , function_instances : Default :: default () , on_stack_params : Default :: default () , on_stack_function_params : Default :: default () , vtables : Default :: default () , const_globals : Default :: default () , global_lvalues : Default :: default () , const_str_cache : Default :: default () , globals : Default :: default () , scalar_types : Default :: default () , types : Default :: default () , tcx , struct_types : Default :: default () , local_gen_sym_counter : Cell :: new (0) , eh_personality : Cell :: new (None) , #[cfg (feature = "master")] rust_try_fn : Cell :: new (None) , pointee_infos : Default :: default () , #[cfg (feature = "master")] cleanup_blocks : Default :: default () , } ; cx . isize_type = usize_type . to_signed (& cx) ; cx } pub fn rvalue_as_function (& self , value : RValue < 'gcc >) -> Function < 'gcc > { let function : Function < 'gcc > = unsafe { std :: mem :: transmute (value) } ; debug_assert ! (self . functions . borrow () . values () . any (| value | * value == function) , "{:?} ({:?}) is not a function" , value , value . get_type ()) ; function } pub fn is_native_int_type (& self , typ : Type < 'gcc >) -> bool { let types = [self . u8_type , self . u16_type , self . u32_type , self . u64_type , self . i8_type , self . i16_type , self . i32_type , self . i64_type ,] ; for native_type in types { if native_type . is_compatible_with (typ) { return true ; } } self . supports_128bit_integers && (self . u128_type . is_compatible_with (typ) || self . i128_type . is_compatible_with (typ)) } pub fn is_non_native_int_type (& self , typ : Type < 'gcc >) -> bool { ! self . supports_128bit_integers && (self . u128_type . is_compatible_with (typ) || self . i128_type . is_compatible_with (typ)) } pub fn is_native_int_type_or_bool (& self , typ : Type < 'gcc >) -> bool { self . is_native_int_type (typ) || typ . is_compatible_with (self . bool_type) } pub fn is_int_type_or_bool (& self , typ : Type < 'gcc >) -> bool { self . is_native_int_type (typ) || self . is_non_native_int_type (typ) || typ . is_compatible_with (self . bool_type) } pub fn sess (& self) -> & 'tcx Session { self . tcx . sess } pub fn bitcast_if_needed (& self , value : RValue < 'gcc > , expected_type : Type < 'gcc > ,) -> RValue < 'gcc > { if value . get_type () != expected_type { self . context . new_bitcast (None , value , expected_type) } else { value } } }}}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > BackendTypes for CodegenCx < 'gcc , 'tcx > { type Value = RValue < 'gcc > ; type Metadata = RValue < 'gcc > ; type Function = Function < 'gcc > ; type BasicBlock = Block < 'gcc > ; type Type = Type < 'gcc > ; type Funclet = () ; type DIScope = () ; type DILocation = Location < 'gcc > ; type DIVariable = () ; }}}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > MiscCodegenMethods < 'tcx > for CodegenCx < 'gcc , 'tcx > { fn vtables (& self ,) -> & RefCell < FxHashMap < (Ty < 'tcx > , Option < ExistentialTraitRef < 'tcx > >) , RValue < 'gcc > > > { & self . vtables } fn get_fn (& self , instance : Instance < 'tcx >) -> Function < 'gcc > { let func = get_fn (self , instance) ; * self . current_func . borrow_mut () = Some (func) ; func } fn get_fn_addr (& self , instance : Instance < 'tcx >) -> RValue < 'gcc > { let func_name = self . tcx . symbol_name (instance) . name ; let func = if self . intrinsics . borrow () . contains_key (func_name) { self . intrinsics . borrow () [func_name] } else if let Some (variable) = self . get_declared_value (func_name) { return variable ; } else { get_fn (self , instance) } ; let ptr = func . get_address (None) ; self . normal_function_addresses . borrow_mut () . insert (ptr) ; self . function_address_names . borrow_mut () . insert (ptr , func_name . to_string ()) ; ptr } fn eh_personality (& self) -> Function < 'gcc > { if let Some (personality_func) = self . eh_personality . get () { return personality_func ; } let tcx = self . tcx ; let func = match tcx . lang_items () . eh_personality () { Some (def_id) if ! wants_msvc_seh (self . sess ()) => { let instance = ty :: Instance :: expect_resolve (tcx , self . typing_env () , def_id , ty :: List :: empty () , DUMMY_SP ,) ; let symbol_name = tcx . symbol_name (instance) . name ; let fn_abi = self . fn_abi_of_instance (instance , ty :: List :: empty ()) ; self . linkage . set (FunctionType :: Extern) ; self . declare_fn (symbol_name , fn_abi) } _ => { let name = if wants_msvc_seh (self . sess ()) { "__CxxFrameHandler3" } else { "rust_eh_personality" } ; self . declare_func (name , self . type_i32 () , & [] , true) } } ; self . eh_personality . set (Some (func)) ; func } fn sess (& self) -> & Session { self . tcx . sess } fn set_frame_pointer_type (& self , _llfn : Function < 'gcc >) { } fn apply_target_cpu_attr (& self , _llfn : Function < 'gcc >) { } fn declare_c_main (& self , fn_type : Self :: Type) -> Option < Self :: Function > { let entry_name = self . sess () . target . entry_name . as_ref () ; if ! self . functions . borrow () . contains_key (entry_name) { #[cfg (feature = "master")] let conv = conv_to_fn_attribute (self . sess () . target . entry_abi , & self . sess () . target . arch) ; #[cfg (not (feature = "master"))] let conv = None ; Some (self . declare_entry_fn (entry_name , fn_type , conv)) } else { None } } }}}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > HasTyCtxt < 'tcx > for CodegenCx < 'gcc , 'tcx > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } }}}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > HasDataLayout for CodegenCx < 'gcc , 'tcx > { fn data_layout (& self) -> & TargetDataLayout { & self . tcx . data_layout } }}}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > HasTargetSpec for CodegenCx < 'gcc , 'tcx > { fn target_spec (& self) -> & Target { & self . tcx . sess . target } }}}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > HasX86AbiOpt for CodegenCx < 'gcc , 'tcx > { fn x86_abi_opt (& self) -> X86Abi { X86Abi { regparm : self . tcx . sess . opts . unstable_opts . regparm , reg_struct_return : self . tcx . sess . opts . unstable_opts . reg_struct_return , } } }}}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > LayoutOfHelpers < 'tcx > for CodegenCx < 'gcc , 'tcx > { #[inline] fn handle_layout_err (& self , err : LayoutError < 'tcx > , span : Span , ty : Ty < 'tcx >) -> ! { if let LayoutError :: SizeOverflow (_) | LayoutError :: ReferencesError (_) = err { self . tcx . dcx () . emit_fatal (respan (span , err . into_diagnostic ())) } else { self . tcx . dcx () . emit_fatal (ssa_errors :: FailedToGetLayout { span , ty , err }) } } }}}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > FnAbiOfHelpers < 'tcx > for CodegenCx < 'gcc , 'tcx > { #[inline] fn handle_fn_abi_err (& self , err : FnAbiError < 'tcx > , span : Span , fn_abi_request : FnAbiRequest < 'tcx > ,) -> ! { if let FnAbiError :: Layout (LayoutError :: SizeOverflow (_)) = err { self . tcx . dcx () . emit_fatal (respan (span , err)) } else { match fn_abi_request { FnAbiRequest :: OfFnPtr { sig , extra_args } => { span_bug ! (span , "`fn_abi_of_fn_ptr({sig}, {extra_args:?})` failed: {err:?}") ; } FnAbiRequest :: OfInstance { instance , extra_args } => { span_bug ! (span , "`fn_abi_of_instance({instance}, {extra_args:?})` failed: {err:?}") ; } } } } }}}
mkitem!{mkimpl!{impl < 'tcx , 'gcc > HasTypingEnv < 'tcx > for CodegenCx < 'gcc , 'tcx > { fn typing_env (& self) -> ty :: TypingEnv < 'tcx > { ty :: TypingEnv :: fully_monomorphized () } }}}
mkitem!{mkimpl!{impl < 'b , 'tcx > CodegenCx < 'b , 'tcx > { #[doc = " Generates a new symbol name with the given prefix. This symbol name must"] #[doc = " only be used for definitions with `internal` or `private` linkage."] pub fn generate_local_symbol_name (& self , prefix : & str) -> String { let idx = self . local_gen_sym_counter . get () ; self . local_gen_sym_counter . set (idx + 1) ; let mut name = String :: with_capacity (prefix . len () + 6) ; name . push_str (prefix) ; name . push ('.') ; name . push_str (& (idx as u64 + ALPHANUMERIC_ONLY as u64) . to_base (ALPHANUMERIC_ONLY)) ; name } }}}

macro_rules! to_gcc_tls_mode_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_gcc_tls_mode in module {}", module_path!());
    };
}

mkfn!{
    to_gcc_tls_mode_introspect!();
    fn to_gcc_tls_mode (tls_model : TlsModel) -> gccjit :: TlsModel { match tls_model { TlsModel :: GeneralDynamic => gccjit :: TlsModel :: GlobalDynamic , TlsModel :: LocalDynamic => gccjit :: TlsModel :: LocalDynamic , TlsModel :: InitialExec => gccjit :: TlsModel :: InitialExec , TlsModel :: LocalExec => gccjit :: TlsModel :: LocalExec , TlsModel :: Emulated => gccjit :: TlsModel :: GlobalDynamic , } }
}