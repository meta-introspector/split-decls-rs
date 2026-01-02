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
mkitem!{macro_rules ! emit_message { ($ ($ arg : tt) *) => { } ; }}
mkitem!{macro_rules ! mkfn { ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: catch_fatal_errors_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: catch_fatal_errors_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn parse_crate_attrs < $ lifetime : lifetime > ($ ($ param : tt) *) -> PResult < $ lifetime2 : lifetime , ast :: AttrVec > $ body : block) => { $ (# [$ attr]) * fn parse_crate_attrs < $ lifetime > ($ ($ param) *) -> PResult < $ lifetime2 , ast :: AttrVec > { $ introspect ; emit_message ! ("🚀 MARKER: parse_crate_attrs_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: parse_crate_attrs_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn init_logger_with_additional_layer < F , T > ($ ($ param : tt) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , $ body : block) => { $ (# [$ attr]) * fn init_logger_with_additional_layer < F , T > ($ ($ param) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , { $ introspect ; emit_message ! ("🚀 MARKER: init_logger_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: init_logger_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < F , T > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where F : FnOnce ($ ($ fnonce_args : tt) *) $ ($ where_rest : tt) * $ body : block) => { $ (# [$ attr]) * fn $ name < F , T > ($ ($ param) *) $ (-> $ ret) ? where F : FnOnce ($ ($ fnonce_args) *) $ ($ where_rest) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where $ ($ where_clause : tt) * $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? where $ ($ where_clause) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub ($ vis : ident) fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub ($ vis) fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_vis - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_vis - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; fn $ name : ident () $ body : block) => { fn $ name () { $ introspect ; emit_message ! ("🚀 MARKER: simple - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: simple - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ ($ anything : tt) *) => { $ ($ anything) * } ; }}
mkitem!{macro_rules ! safe_println { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{macro_rules ! safe_print { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{# [macro_export] macro_rules ! include_rust_compiler { ($ crate_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ crate_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_" , $ file , ".rs")) ; } ; ($ crate_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_lib.rs")) ; } ; }}
mkitem!{# [macro_export] macro_rules ! include_rust_library { ($ lib_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ lib_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_" , $ file , ".rs")) ; } ; ($ lib_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_lib.rs")) ; } ; }}
mkitem!{# [macro_export] macro_rules ! include_processed { ($ path : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_" , $ path , ".rs")) ; } ; }}
mkitem!{macro_rules ! mkinclude { ($ path : ident) => { } ; ($ path : literal) => { include ! ($ path) } ; }}
mkitem!{macro_rules ! mkitem { (include ! ($ path : ident) ;) => { } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ string_lit : literal }) => { $ macro_name :: $ macro_sub ! { $ string_lit } } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ ($ args : tt) * }) => { $ macro_name :: $ macro_sub ! { $ ($ args) * } } ; ($ macro_name : ident ! { $ ($ args : tt) * }) => { $ macro_name ! { $ ($ args) * } } ; ($ item : item) => { $ item } ; }}
mkitem!{# [macro_export] macro_rules ! mkmod { ($ name : ident , { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (pub mod $ name : ident { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; pub mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (mod $ name : ident { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; }}
mkitem!{# [macro_export] macro_rules ! mkuse { ($ use_stmt : item) => { compile_error ! (concat ! ("USE|" , module_path ! () , "|" , stringify ! ($ use_stmt))) ; } ; }}
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
mkuse!{use std :: collections :: VecDeque ;}
mkuse!{use std :: ffi :: { CStr , CString } ;}
mkuse!{use std :: fmt :: Write ;}
mkuse!{use std :: path :: Path ;}
mkuse!{use std :: sync :: Once ;}
mkuse!{use std :: { ptr , slice , str } ;}
mkuse!{use libc :: c_int ;}
mkuse!{use rustc_codegen_ssa :: base :: wants_wasm_eh ;}
mkuse!{use rustc_codegen_ssa :: target_features :: cfg_target_feature ;}
mkuse!{use rustc_codegen_ssa :: { TargetConfig , target_features } ;}
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_data_structures :: small_c_str :: SmallCStr ;}
mkuse!{use rustc_fs_util :: path_to_c_string ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_session :: config :: { PrintKind , PrintRequest } ;}
mkuse!{use rustc_target :: spec :: { MergeFunctions , PanicStrategy , SmallDataThresholdSupport } ;}
mkuse!{use smallvec :: { SmallVec , smallvec } ;}
mkuse!{use crate :: back :: write :: create_informational_target_machine ;}
mkuse!{use crate :: { errors , llvm } ;}
mkitem!{static INIT : Once = Once :: new () ;}

macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    pub (crate) fn init (sess : & Session) { unsafe { if ! llvm :: LLVMIsMultithreaded () . is_true () { bug ! ("LLVM compiled without support for threads") ; } INIT . call_once (| | { configure_llvm (sess) ; }) ; } }
}

macro_rules! require_inited_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function require_inited in module {}", module_path!());
    };
}

mkfn!{
    require_inited_introspect!();
    fn require_inited () { if ! INIT . is_completed () { bug ! ("LLVM is not initialized") ; } }
}

macro_rules! configure_llvm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function configure_llvm in module {}", module_path!());
    };
}

mkfn!{
    configure_llvm_introspect!();
    unsafe fn configure_llvm (sess : & Session) { let n_args = sess . opts . cg . llvm_args . len () + sess . target . llvm_args . len () ; let mut llvm_c_strs = Vec :: with_capacity (n_args + 1) ; let mut llvm_args = Vec :: with_capacity (n_args + 1) ; unsafe { llvm :: LLVMRustInstallErrorHandlers () ; } if std :: env :: var_os ("CI") . is_some () { unsafe { llvm :: LLVMRustDisableSystemDialogsOnCrash () ; } } fn llvm_arg_to_arg_name (full_arg : & str) -> & str { full_arg . trim () . split (| c : char | c == '=' || c . is_whitespace ()) . next () . unwrap_or ("") } let cg_opts = sess . opts . cg . llvm_args . iter () . map (AsRef :: as_ref) ; let tg_opts = sess . target . llvm_args . iter () . map (AsRef :: as_ref) ; let sess_args = cg_opts . chain (tg_opts) ; let user_specified_args : FxHashSet < _ > = sess_args . clone () . map (| s | llvm_arg_to_arg_name (s)) . filter (| s | ! s . is_empty ()) . collect () ; { let mut add = | arg : & str , force : bool | { if force || ! user_specified_args . contains (llvm_arg_to_arg_name (arg)) { let s = CString :: new (arg) . unwrap () ; llvm_args . push (s . as_ptr ()) ; llvm_c_strs . push (s) ; } } ; add ("rustc -Cllvm-args=\"...\" with" , true) ; if sess . opts . unstable_opts . time_llvm_passes { add ("-time-passes" , false) ; } if sess . opts . unstable_opts . print_llvm_passes { add ("-debug-pass=Structure" , false) ; } if sess . target . generate_arange_section && ! sess . opts . unstable_opts . no_generate_arange_section { add ("-generate-arange-section" , false) ; } match sess . opts . unstable_opts . merge_functions . unwrap_or (sess . target . merge_functions) { MergeFunctions :: Disabled | MergeFunctions :: Trampolines => { } MergeFunctions :: Aliases => { add ("-mergefunc-use-aliases" , false) ; } } if wants_wasm_eh (sess) { add ("-wasm-enable-eh" , false) ; } if sess . target . os == "emscripten" && ! sess . opts . unstable_opts . emscripten_wasm_eh && sess . panic_strategy () == PanicStrategy :: Unwind { add ("-enable-emscripten-cxx-exceptions" , false) ; } add ("-preserve-alignment-assumptions-during-inlining=false" , false) ; add ("-import-cold-multiplier=0.1" , false) ; if sess . print_llvm_stats () { add ("-stats" , false) ; } for arg in sess_args { add (& (* arg) , true) ; } match (sess . opts . unstable_opts . small_data_threshold , sess . target . small_data_threshold_support () ,) { (Some (threshold) , SmallDataThresholdSupport :: LlvmArg (arg)) => { add (& format ! ("--{arg}={threshold}") , false) } _ => () , } ; } if sess . opts . unstable_opts . llvm_time_trace { unsafe { llvm :: LLVMRustTimeTraceProfilerInitialize () } ; } rustc_llvm :: initialize_available_targets () ; unsafe { llvm :: LLVMRustSetLLVMOptions (llvm_args . len () as c_int , llvm_args . as_ptr ()) } ; }
}

macro_rules! time_trace_profiler_finish_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function time_trace_profiler_finish in module {}", module_path!());
    };
}

mkfn!{
    time_trace_profiler_finish_introspect!();
    pub (crate) fn time_trace_profiler_finish (file_name : & Path) { unsafe { let file_name = path_to_c_string (file_name) ; llvm :: LLVMRustTimeTraceProfilerFinish (file_name . as_ptr ()) ; } }
}
mkitem!{mkenum!{enum TargetFeatureFoldStrength < 'a > { EnableOnly (& 'a str) , Both (& 'a str) , }}}
mkitem!{mkimpl!{impl < 'a > TargetFeatureFoldStrength < 'a > { fn as_str (& self) -> & 'a str { match self { TargetFeatureFoldStrength :: EnableOnly (feat) => feat , TargetFeatureFoldStrength :: Both (feat) => feat , } } }}}
mkitem!{mkstruct!{pub (crate) struct LLVMFeature < 'a > { llvm_feature_name : & 'a str , dependencies : SmallVec < [TargetFeatureFoldStrength < 'a > ; 1] > , }}}
mkitem!{mkimpl!{impl < 'a > LLVMFeature < 'a > { fn new (llvm_feature_name : & 'a str) -> Self { Self { llvm_feature_name , dependencies : SmallVec :: new () } } fn with_dependencies (llvm_feature_name : & 'a str , dependencies : SmallVec < [TargetFeatureFoldStrength < 'a > ; 1] > ,) -> Self { Self { llvm_feature_name , dependencies } } }}}
mkitem!{mkimpl!{impl < 'a > IntoIterator for LLVMFeature < 'a > { type Item = & 'a str ; type IntoIter = impl Iterator < Item = & 'a str > ; fn into_iter (self) -> Self :: IntoIter { let dependencies = self . dependencies . into_iter () . map (| feat | feat . as_str ()) ; std :: iter :: once (self . llvm_feature_name) . chain (dependencies) } }}}

macro_rules! to_llvm_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_llvm_features in module {}", module_path!());
    };
}

mkfn!{
    to_llvm_features_introspect!();
    # [doc = " Convert a Rust feature name to an LLVM feature name. Returning `None` means the"] # [doc = " feature should be skipped, usually because it is not supported by the current"] # [doc = " LLVM version."] # [doc = ""] # [doc = " WARNING: the features after applying `to_llvm_features` must be known"] # [doc = " to LLVM or the feature detection code will walk past the end of the feature"] # [doc = " array, leading to crashes."] # [doc = ""] # [doc = " To find a list of LLVM's names, see llvm-project/llvm/lib/Target/{ARCH}/*.td"] # [doc = " where `{ARCH}` is the architecture name. Look for instances of `SubtargetFeature`."] # [doc = ""] # [doc = " Check the current rustc fork of LLVM in the repo at"] # [doc = " <https://github.com/rust-lang/llvm-project/>. The commit in use can be found via the"] # [doc = " `llvm-project` submodule in <https://github.com/rust-lang/rust/tree/master/src> Though note that"] # [doc = " Rust can also be build with an external precompiled version of LLVM which might lead to failures"] # [doc = " if the oldest tested / supported LLVM version doesn't yet support the relevant intrinsics."] pub (crate) fn to_llvm_features < 'a > (sess : & Session , s : & 'a str) -> Option < LLVMFeature < 'a > > { let arch = if sess . target . arch == "x86_64" { "x86" } else if sess . target . arch == "arm64ec" { "aarch64" } else if sess . target . arch == "sparc64" { "sparc" } else if sess . target . arch == "powerpc64" { "powerpc" } else { & * sess . target . arch } ; match (arch , s) { ("x86" , "sse4.2") => Some (LLVMFeature :: with_dependencies ("sse4.2" , smallvec ! [TargetFeatureFoldStrength :: EnableOnly ("crc32")] ,)) , ("x86" , "pclmulqdq") => Some (LLVMFeature :: new ("pclmul")) , ("x86" , "rdrand") => Some (LLVMFeature :: new ("rdrnd")) , ("x86" , "bmi1") => Some (LLVMFeature :: new ("bmi")) , ("x86" , "cmpxchg16b") => Some (LLVMFeature :: new ("cx16")) , ("x86" , "lahfsahf") => Some (LLVMFeature :: new ("sahf")) , ("aarch64" , "rcpc2") => Some (LLVMFeature :: new ("rcpc-immo")) , ("aarch64" , "dpb") => Some (LLVMFeature :: new ("ccpp")) , ("aarch64" , "dpb2") => Some (LLVMFeature :: new ("ccdp")) , ("aarch64" , "frintts") => Some (LLVMFeature :: new ("fptoint")) , ("aarch64" , "fcma") => Some (LLVMFeature :: new ("complxnum")) , ("aarch64" , "pmuv3") => Some (LLVMFeature :: new ("perfmon")) , ("aarch64" , "paca") => Some (LLVMFeature :: new ("pauth")) , ("aarch64" , "pacg") => Some (LLVMFeature :: new ("pauth")) , ("aarch64" , "sve-b16b16") if get_version () . 0 < 20 => Some (LLVMFeature :: new ("b16b16")) , ("aarch64" , "sme-b16b16") if get_version () . 0 < 20 => Some (LLVMFeature :: new ("b16b16")) , ("aarch64" , "flagm2") => Some (LLVMFeature :: new ("altnzcv")) , ("aarch64" , "neon") => Some (LLVMFeature :: with_dependencies ("neon" , smallvec ! [TargetFeatureFoldStrength :: Both ("fp-armv8")] ,)) , ("aarch64" , "fhm") => Some (LLVMFeature :: new ("fp16fml")) , ("aarch64" , "fp16") => Some (LLVMFeature :: new ("fullfp16")) , ("aarch64" , "fpmr") => None , ("arm" , "fp16") => Some (LLVMFeature :: new ("fullfp16")) , ("nvptx64" , "sm_100") if get_version () . 0 < 20 => None , ("nvptx64" , "sm_100a") if get_version () . 0 < 20 => None , ("nvptx64" , "sm_101") if get_version () . 0 < 20 => None , ("nvptx64" , "sm_101a") if get_version () . 0 < 20 => None , ("nvptx64" , "sm_120") if get_version () . 0 < 20 => None , ("nvptx64" , "sm_120a") if get_version () . 0 < 20 => None , ("nvptx64" , "ptx86") if get_version () . 0 < 20 => None , ("nvptx64" , "ptx87") if get_version () . 0 < 20 => None , ("loongarch64" , "div32" | "lam-bh" | "lamcas" | "ld-seq-sa" | "scq") if get_version () . 0 < 20 => { None } ("loongarch32" | "loongarch64" , "32s") if get_version () . 0 < 21 => None , ("riscv32" | "riscv64" , "zacas" | "rva23u64" | "supm") if get_version () . 0 < 20 => None , ("s390x" , "message-security-assist-extension12" | "concurrent-functions" | "miscellaneous-extensions-4" | "vector-enhancements-3" | "vector-packed-decimal-enhancement-3" ,) if get_version () . 0 < 20 => None , ("x86" , s) if s . starts_with ("avx512") => Some (LLVMFeature :: with_dependencies (s , smallvec ! [TargetFeatureFoldStrength :: EnableOnly ("evex512")] ,)) , ("wasm32" | "wasm64" , "wide-arithmetic") if get_version () < (20 , 0 , 0) => None , ("sparc" , "leoncasa") => Some (LLVMFeature :: new ("hasleoncasa")) , ("sparc" , "v8plus") if get_version () . 0 == 19 => Some (LLVMFeature :: new ("v9")) , ("powerpc" , "power8-crypto") => Some (LLVMFeature :: new ("crypto")) , ("x86" , "amx-avx512" | "amx-fp8" | "amx-movrs" | "amx-tf32" | "amx-transpose") if get_version () . 0 < 20 => { None } ("x86" , "movrs") if get_version () . 0 < 20 => None , ("x86" , "avx10.1") => Some (LLVMFeature :: new ("avx10.1-512")) , ("x86" , "avx10.2") if get_version () . 0 < 20 => None , ("x86" , "avx10.2") if get_version () . 0 >= 20 => Some (LLVMFeature :: new ("avx10.2-512")) , ("x86" , "apxf") => Some (LLVMFeature :: with_dependencies ("egpr" , smallvec ! [TargetFeatureFoldStrength :: Both ("push2pop2") , TargetFeatureFoldStrength :: Both ("ppx") , TargetFeatureFoldStrength :: Both ("ndd") , TargetFeatureFoldStrength :: Both ("ccmp") , TargetFeatureFoldStrength :: Both ("cf") , TargetFeatureFoldStrength :: Both ("nf") , TargetFeatureFoldStrength :: Both ("zu") ,] ,)) , (_ , s) => Some (LLVMFeature :: new (s)) , } }
}

macro_rules! target_config_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function target_config in module {}", module_path!());
    };
}

mkfn!{
    target_config_introspect!();
    # [doc = " Used to generate cfg variables and apply features."] # [doc = " Must express features in the way Rust understands them."] # [doc = ""] # [doc = " We do not have to worry about RUSTC_SPECIFIC_FEATURES here, those are handled outside codegen."] pub (crate) fn target_config (sess : & Session) -> TargetConfig { let target_machine = create_informational_target_machine (sess , true) ; let (unstable_target_features , target_features) = cfg_target_feature (sess , | feature | { if let Some (feat) = to_llvm_features (sess , feature) { for llvm_feature in feat { let cstr = SmallCStr :: new (llvm_feature) ; if ! unsafe { llvm :: LLVMRustHasFeature (target_machine . raw () , cstr . as_ptr ()) } { return false ; } } true } else { false } }) ; let mut cfg = TargetConfig { target_features , unstable_target_features , has_reliable_f16 : true , has_reliable_f16_math : true , has_reliable_f128 : true , has_reliable_f128_math : true , } ; update_target_reliable_float_cfg (sess , & mut cfg) ; cfg }
}

macro_rules! update_target_reliable_float_cfg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function update_target_reliable_float_cfg in module {}", module_path!());
    };
}

mkfn!{
    update_target_reliable_float_cfg_introspect!();
    # [doc = " Determine whether or not experimental float types are reliable based on known bugs."] fn update_target_reliable_float_cfg (sess : & Session , cfg : & mut TargetConfig) { let target_arch = sess . target . arch . as_ref () ; let target_os = sess . target . options . os . as_ref () ; let target_env = sess . target . options . env . as_ref () ; let target_abi = sess . target . options . abi . as_ref () ; let target_pointer_width = sess . target . pointer_width ; let version = get_version () ; let lt_20_1_1 = version < (20 , 1 , 1) ; let lt_21_0_0 = version < (21 , 0 , 0) ; cfg . has_reliable_f16 = match (target_arch , target_os) { ("aarch64" , _) if ! cfg . target_features . iter () . any (| f | f . as_str () == "neon") && lt_20_1_1 => { false } ("arm64ec" , _) => false , ("s390x" , _) if lt_21_0_0 => false , ("x86_64" , "windows") if target_env == "gnu" && target_abi != "llvm" => false , ("csky" , _) => false , ("hexagon" , _) if lt_21_0_0 => false , ("powerpc" | "powerpc64" , _) => false , ("sparc" | "sparc64" , _) => false , ("wasm32" | "wasm64" , _) => false , _ => true , } ; cfg . has_reliable_f128 = match (target_arch , target_os) { ("arm64ec" , _) => false , ("mips64" | "mips64r6" , _) if lt_20_1_1 => false , ("nvptx64" , _) => false , ("amdgpu" , _) => false , ("powerpc" | "powerpc64" , _) => false , ("sparc" , _) => false , ("x86" , _) if lt_21_0_0 => false , ("x86_64" , "windows") if target_env == "gnu" && target_abi != "llvm" => false , _ => true , } ; cfg . has_reliable_f16_math = cfg . has_reliable_f16 ; cfg . has_reliable_f128_math = match (target_arch , target_os) { _ if target_env == "musl" => false , ("x86_64" , _) => false , (_ , "linux") if target_pointer_width == 64 => true , _ => false , } && cfg . has_reliable_f128 ; }
}

macro_rules! print_version_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function print_version in module {}", module_path!());
    };
}

mkfn!{
    print_version_introspect!();
    pub (crate) fn print_version () { let (major , minor , patch) = get_version () ; println ! ("LLVM version: {major}.{minor}.{patch}") ; }
}

macro_rules! get_version_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_version in module {}", module_path!());
    };
}

mkfn!{
    get_version_introspect!();
    pub (crate) fn get_version () -> (u32 , u32 , u32) { unsafe { (llvm :: LLVMRustVersionMajor () , llvm :: LLVMRustVersionMinor () , llvm :: LLVMRustVersionPatch ()) } }
}

macro_rules! print_passes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function print_passes in module {}", module_path!());
    };
}

mkfn!{
    print_passes_introspect!();
    pub (crate) fn print_passes () { unsafe { llvm :: LLVMRustPrintPasses () ; } }
}

macro_rules! llvm_target_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function llvm_target_features in module {}", module_path!());
    };
}

mkfn!{
    llvm_target_features_introspect!();
    fn llvm_target_features (tm : & llvm :: TargetMachine) -> Vec < (& str , & str) > { let len = unsafe { llvm :: LLVMRustGetTargetFeaturesCount (tm) } ; let mut ret = Vec :: with_capacity (len) ; for i in 0 .. len { unsafe { let mut feature = ptr :: null () ; let mut desc = ptr :: null () ; llvm :: LLVMRustGetTargetFeature (tm , i , & mut feature , & mut desc) ; if feature . is_null () || desc . is_null () { bug ! ("LLVM returned a `null` target feature string") ; } let feature = CStr :: from_ptr (feature) . to_str () . unwrap_or_else (| e | { bug ! ("LLVM returned a non-utf8 feature string: {}" , e) ; }) ; let desc = CStr :: from_ptr (desc) . to_str () . unwrap_or_else (| e | { bug ! ("LLVM returned a non-utf8 feature string: {}" , e) ; }) ; ret . push ((feature , desc)) ; } } ret }
}

macro_rules! print_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function print in module {}", module_path!());
    };
}

mkfn!{
    print_introspect!();
    pub (crate) fn print (req : & PrintRequest , out : & mut String , sess : & Session) { require_inited () ; let tm = create_informational_target_machine (sess , false) ; match req . kind { PrintKind :: TargetCPUs => print_target_cpus (sess , tm . raw () , out) , PrintKind :: TargetFeatures => print_target_features (sess , tm . raw () , out) , _ => bug ! ("rustc_codegen_llvm can't handle print request: {:?}" , req) , } }
}

macro_rules! print_target_cpus_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function print_target_cpus in module {}", module_path!());
    };
}

mkfn!{
    print_target_cpus_introspect!();
    fn print_target_cpus (sess : & Session , tm : & llvm :: TargetMachine , out : & mut String) { let cpu_names = llvm :: build_string (| s | unsafe { llvm :: LLVMRustPrintTargetCPUs (& tm , s) ; }) . unwrap () ; struct Cpu < 'a > { cpu_name : & 'a str , remark : String , } let target_cpu = handle_native (& sess . target . cpu) ; let make_remark = | cpu_name | { if cpu_name == target_cpu { let target = & sess . target . llvm_target ; format ! (" - This is the default target CPU for the current build target (currently {target}).") } else { "" . to_owned () } } ; let mut cpus = cpu_names . lines () . map (| cpu_name | Cpu { cpu_name , remark : make_remark (cpu_name) }) . collect :: < VecDeque < _ > > () ; if sess . host . arch == sess . target . arch { let host = get_host_cpu_name () ; cpus . push_front (Cpu { cpu_name : "native" , remark : format ! (" - Select the CPU of the current host (currently {host}).") , }) ; } let max_name_width = cpus . iter () . map (| cpu | cpu . cpu_name . len ()) . max () . unwrap_or (0) ; writeln ! (out , "Available CPUs for this target:") . unwrap () ; for Cpu { cpu_name , remark } in cpus { let width = if remark . is_empty () { 0 } else { max_name_width } ; writeln ! (out , "    {cpu_name:<width$}{remark}") . unwrap () ; } }
}

macro_rules! print_target_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function print_target_features in module {}", module_path!());
    };
}

mkfn!{
    print_target_features_introspect!();
    fn print_target_features (sess : & Session , tm : & llvm :: TargetMachine , out : & mut String) { let mut llvm_target_features = llvm_target_features (tm) ; let mut known_llvm_target_features = FxHashSet :: < & 'static str > :: default () ; let mut rustc_target_features = sess . target . rust_target_features () . iter () . filter_map (| (feature , gate , _implied) | { if ! gate . in_cfg () { return None ; } let llvm_feature = to_llvm_features (sess , * feature) ? . llvm_feature_name ; let desc = match llvm_target_features . binary_search_by_key (& llvm_feature , | (f , _d) | f) . ok () { Some (index) => { known_llvm_target_features . insert (llvm_feature) ; llvm_target_features [index] . 1 } None => "" , } ; Some ((* feature , desc)) }) . collect :: < Vec < _ > > () ; rustc_target_features . extend_from_slice (& [("crt-static" , "Enables C Run-time Libraries to be statically linked" ,)]) ; rustc_target_features . sort () ; llvm_target_features . retain (| (f , _d) | ! known_llvm_target_features . contains (f)) ; let max_feature_len = llvm_target_features . iter () . chain (rustc_target_features . iter ()) . map (| (feature , _desc) | feature . len ()) . max () . unwrap_or (0) ; writeln ! (out , "Features supported by rustc for this target:") . unwrap () ; for (feature , desc) in & rustc_target_features { writeln ! (out , "    {feature:max_feature_len$} - {desc}.") . unwrap () ; } writeln ! (out , "\nCode-generation features supported by LLVM for this target:") . unwrap () ; for (feature , desc) in & llvm_target_features { writeln ! (out , "    {feature:max_feature_len$} - {desc}.") . unwrap () ; } if llvm_target_features . is_empty () { writeln ! (out , "    Target features listing is not supported by this LLVM version.") . unwrap () ; } writeln ! (out , "\nUse +feature to enable a feature, or -feature to disable it.") . unwrap () ; writeln ! (out , "For example, rustc -C target-cpu=mycpu -C target-feature=+feature1,-feature2\n") . unwrap () ; writeln ! (out , "Code-generation features cannot be used in cfg or #[target_feature],") . unwrap () ; writeln ! (out , "and may be renamed or removed in a future version of LLVM or rustc.\n") . unwrap () ; }
}

macro_rules! get_host_cpu_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_host_cpu_name in module {}", module_path!());
    };
}

mkfn!{
    get_host_cpu_name_introspect!();
    # [doc = " Returns the host CPU name, according to LLVM."] fn get_host_cpu_name () -> & 'static str { let mut len = 0 ; let slice : & 'static [u8] = unsafe { let ptr = llvm :: LLVMRustGetHostCPUName (& mut len) ; assert ! (! ptr . is_null ()) ; slice :: from_raw_parts (ptr , len) } ; str :: from_utf8 (slice) . expect ("host CPU name should be UTF-8") }
}

macro_rules! handle_native_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function handle_native in module {}", module_path!());
    };
}

mkfn!{
    handle_native_introspect!();
    # [doc = " If the given string is `\"native\"`, returns the host CPU name according to"] # [doc = " LLVM. Otherwise, the string is returned as-is."] fn handle_native (cpu_name : & str) -> & str { match cpu_name { "native" => get_host_cpu_name () , _ => cpu_name , } }
}

macro_rules! target_cpu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function target_cpu in module {}", module_path!());
    };
}

mkfn!{
    target_cpu_introspect!();
    pub (crate) fn target_cpu (sess : & Session) -> & str { let cpu_name = sess . opts . cg . target_cpu . as_deref () . unwrap_or_else (| | & sess . target . cpu) ; handle_native (cpu_name) }
}

macro_rules! llvm_features_by_flags_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function llvm_features_by_flags in module {}", module_path!());
    };
}

mkfn!{
    llvm_features_by_flags_introspect!();
    # [doc = " The target features for compiler flags other than `-Ctarget-features`."] fn llvm_features_by_flags (sess : & Session , features : & mut Vec < String >) { target_features :: retpoline_features_by_flags (sess , features) ; if sess . opts . unstable_opts . fixed_x18 { if sess . target . arch != "aarch64" { sess . dcx () . emit_fatal (errors :: FixedX18InvalidArch { arch : & sess . target . arch }) ; } else { features . push ("+reserve-x18" . into ()) ; } } }
}

macro_rules! global_llvm_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function global_llvm_features in module {}", module_path!());
    };
}

mkfn!{
    global_llvm_features_introspect!();
    # [doc = " The list of LLVM features computed from CLI flags (`-Ctarget-cpu`, `-Ctarget-feature`,"] # [doc = " `--target` and similar)."] pub (crate) fn global_llvm_features (sess : & Session , diagnostics : bool , only_base_features : bool ,) -> Vec < String > { let mut features = vec ! [] ; match sess . opts . cg . target_cpu { Some (ref s) if s == "native" => { let features_string = unsafe { let ptr = llvm :: LLVMGetHostCPUFeatures () ; let features_string = if ! ptr . is_null () { CStr :: from_ptr (ptr) . to_str () . unwrap_or_else (| e | { bug ! ("LLVM returned a non-utf8 features string: {}" , e) ; }) . to_owned () } else { bug ! ("could not allocate host CPU features, LLVM returned a `null` string") ; } ; llvm :: LLVMDisposeMessage (ptr) ; features_string } ; features . extend (features_string . split (',') . map (String :: from)) ; } Some (_) | None => { } } ; features . extend (sess . target . features . split (',') . filter (| v | ! v . is_empty ()) . filter (| v | * v != "+v8plus" || get_version () >= (20 , 0 , 0)) . map (String :: from) ,) ; if wants_wasm_eh (sess) && sess . panic_strategy () == PanicStrategy :: Unwind { features . push ("+exception-handling" . into ()) ; } if ! only_base_features { target_features :: flag_to_backend_features (sess , diagnostics , | feature | { to_llvm_features (sess , feature) . map (| f | SmallVec :: < [& str ; 2] > :: from_iter (f . into_iter ())) . unwrap_or_default () } , | feature , enable | { let enable_disable = if enable { '+' } else { '-' } ; let Some (llvm_feature) = to_llvm_features (sess , feature) else { return } ; features . extend (std :: iter :: once (format ! ("{}{}" , enable_disable , llvm_feature . llvm_feature_name)) . chain (llvm_feature . dependencies . into_iter () . filter_map (move | feat | match (enable , feat) { (_ , TargetFeatureFoldStrength :: Both (f)) | (true , TargetFeatureFoldStrength :: EnableOnly (f)) => { Some (format ! ("{enable_disable}{f}")) } _ => None , } ,)) ,) } ,) ; } llvm_features_by_flags (sess , & mut features) ; features }
}

macro_rules! tune_cpu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function tune_cpu in module {}", module_path!());
    };
}

mkfn!{
    tune_cpu_introspect!();
    pub (crate) fn tune_cpu (sess : & Session) -> Option < & str > { let name = sess . opts . unstable_opts . tune_cpu . as_ref () ? ; Some (handle_native (name)) }
}