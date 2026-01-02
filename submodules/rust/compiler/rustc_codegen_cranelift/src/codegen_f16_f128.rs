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
mkuse!{use crate :: prelude :: * ;}

macro_rules! f16_to_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f16_to_f32 in module {}", module_path!());
    };
}

mkfn!{
    f16_to_f32_introspect!();
    pub (crate) fn f16_to_f32 (fx : & mut FunctionCx < '_ , '_ , '_ > , value : Value) -> Value { let (value , arg_ty) = if fx . tcx . sess . target . vendor == "apple" && fx . tcx . sess . target . arch == "x86_64" { (fx . bcx . ins () . bitcast (types :: I16 , MemFlags :: new () , value) , lib_call_arg_param (fx . tcx , types :: I16 , false) ,) } else { (value , AbiParam :: new (types :: F16)) } ; fx . lib_call ("__extendhfsf2" , vec ! [arg_ty] , vec ! [AbiParam :: new (types :: F32)] , & [value]) [0] }
}

macro_rules! f16_to_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f16_to_f64 in module {}", module_path!());
    };
}

mkfn!{
    f16_to_f64_introspect!();
    fn f16_to_f64 (fx : & mut FunctionCx < '_ , '_ , '_ > , value : Value) -> Value { let ret = f16_to_f32 (fx , value) ; fx . bcx . ins () . fpromote (types :: F64 , ret) }
}

macro_rules! f32_to_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f32_to_f16 in module {}", module_path!());
    };
}

mkfn!{
    f32_to_f16_introspect!();
    pub (crate) fn f32_to_f16 (fx : & mut FunctionCx < '_ , '_ , '_ > , value : Value) -> Value { let ret_ty = if fx . tcx . sess . target . vendor == "apple" && fx . tcx . sess . target . arch == "x86_64" { types :: I16 } else { types :: F16 } ; let ret = fx . lib_call ("__truncsfhf2" , vec ! [AbiParam :: new (types :: F32)] , vec ! [AbiParam :: new (ret_ty)] , & [value] ,) [0] ; if ret_ty == types :: I16 { fx . bcx . ins () . bitcast (types :: F16 , MemFlags :: new () , ret) } else { ret } }
}

macro_rules! f64_to_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f64_to_f16 in module {}", module_path!());
    };
}

mkfn!{
    f64_to_f16_introspect!();
    fn f64_to_f16 (fx : & mut FunctionCx < '_ , '_ , '_ > , value : Value) -> Value { let ret_ty = if fx . tcx . sess . target . vendor == "apple" && fx . tcx . sess . target . arch == "x86_64" { types :: I16 } else { types :: F16 } ; let ret = fx . lib_call ("__truncdfhf2" , vec ! [AbiParam :: new (types :: F64)] , vec ! [AbiParam :: new (ret_ty)] , & [value] ,) [0] ; if ret_ty == types :: I16 { fx . bcx . ins () . bitcast (types :: F16 , MemFlags :: new () , ret) } else { ret } }
}

macro_rules! fcmp_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fcmp in module {}", module_path!());
    };
}

mkfn!{
    fcmp_introspect!();
    pub (crate) fn fcmp (fx : & mut FunctionCx < '_ , '_ , '_ > , cc : FloatCC , lhs : Value , rhs : Value) -> Value { let ty = fx . bcx . func . dfg . value_type (lhs) ; match ty { types :: F32 | types :: F64 => fx . bcx . ins () . fcmp (cc , lhs , rhs) , types :: F16 => { let lhs = f16_to_f32 (fx , lhs) ; let rhs = f16_to_f32 (fx , rhs) ; fx . bcx . ins () . fcmp (cc , lhs , rhs) } types :: F128 => { let (name , int_cc) = match cc { FloatCC :: Equal => ("__eqtf2" , IntCC :: Equal) , FloatCC :: NotEqual => ("__netf2" , IntCC :: NotEqual) , FloatCC :: LessThan => ("__lttf2" , IntCC :: SignedLessThan) , FloatCC :: LessThanOrEqual => ("__letf2" , IntCC :: SignedLessThanOrEqual) , FloatCC :: GreaterThan => ("__gttf2" , IntCC :: SignedGreaterThan) , FloatCC :: GreaterThanOrEqual => ("__getf2" , IntCC :: SignedGreaterThanOrEqual) , _ => unreachable ! ("not currently used in rustc_codegen_cranelift: {cc:?}") , } ; let res = fx . lib_call (name , vec ! [AbiParam :: new (types :: F128) , AbiParam :: new (types :: F128)] , vec ! [AbiParam :: new (types :: I32)] , & [lhs , rhs] ,) [0] ; let zero = fx . bcx . ins () . iconst (types :: I32 , 0) ; let res = fx . bcx . ins () . icmp (int_cc , res , zero) ; res } _ => unreachable ! ("{ty:?}") , } }
}

macro_rules! codegen_f128_binop_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function codegen_f128_binop in module {}", module_path!());
    };
}

mkfn!{
    codegen_f128_binop_introspect!();
    pub (crate) fn codegen_f128_binop (fx : & mut FunctionCx < '_ , '_ , '_ > , bin_op : BinOp , lhs : Value , rhs : Value ,) -> Value { let name = match bin_op { BinOp :: Add => "__addtf3" , BinOp :: Sub => "__subtf3" , BinOp :: Mul => "__multf3" , BinOp :: Div => "__divtf3" , _ => unreachable ! ("handled in `codegen_float_binop`") , } ; fx . lib_call (name , vec ! [AbiParam :: new (types :: F128) , AbiParam :: new (types :: F128)] , vec ! [AbiParam :: new (types :: F128)] , & [lhs , rhs] ,) [0] }
}

macro_rules! neg_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function neg_f16 in module {}", module_path!());
    };
}

mkfn!{
    neg_f16_introspect!();
    pub (crate) fn neg_f16 (fx : & mut FunctionCx < '_ , '_ , '_ > , value : Value) -> Value { let bits = fx . bcx . ins () . bitcast (types :: I16 , MemFlags :: new () , value) ; let bits = fx . bcx . ins () . bxor_imm (bits , 0x8000) ; fx . bcx . ins () . bitcast (types :: F16 , MemFlags :: new () , bits) }
}

macro_rules! neg_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function neg_f128 in module {}", module_path!());
    };
}

mkfn!{
    neg_f128_introspect!();
    pub (crate) fn neg_f128 (fx : & mut FunctionCx < '_ , '_ , '_ > , value : Value) -> Value { let bits = fx . bcx . ins () . bitcast (types :: I128 , MemFlags :: new () , value) ; let (low , high) = fx . bcx . ins () . isplit (bits) ; let high = fx . bcx . ins () . bxor_imm (high , 0x8000_0000_0000_0000_u64 as i64) ; let bits = fx . bcx . ins () . iconcat (low , high) ; fx . bcx . ins () . bitcast (types :: F128 , MemFlags :: new () , bits) }
}

macro_rules! abs_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function abs_f16 in module {}", module_path!());
    };
}

mkfn!{
    abs_f16_introspect!();
    pub (crate) fn abs_f16 (fx : & mut FunctionCx < '_ , '_ , '_ > , value : Value) -> Value { let bits = fx . bcx . ins () . bitcast (types :: I16 , MemFlags :: new () , value) ; let bits = fx . bcx . ins () . band_imm (bits , 0x7fff) ; fx . bcx . ins () . bitcast (types :: F16 , MemFlags :: new () , bits) }
}

macro_rules! abs_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function abs_f128 in module {}", module_path!());
    };
}

mkfn!{
    abs_f128_introspect!();
    pub (crate) fn abs_f128 (fx : & mut FunctionCx < '_ , '_ , '_ > , value : Value) -> Value { let bits = fx . bcx . ins () . bitcast (types :: I128 , MemFlags :: new () , value) ; let (low , high) = fx . bcx . ins () . isplit (bits) ; let high = fx . bcx . ins () . band_imm (high , 0x7fff_ffff_ffff_ffff_u64 as i64) ; let bits = fx . bcx . ins () . iconcat (low , high) ; fx . bcx . ins () . bitcast (types :: F128 , MemFlags :: new () , bits) }
}

macro_rules! copysign_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copysign_f16 in module {}", module_path!());
    };
}

mkfn!{
    copysign_f16_introspect!();
    pub (crate) fn copysign_f16 (fx : & mut FunctionCx < '_ , '_ , '_ > , lhs : Value , rhs : Value) -> Value { let lhs = fx . bcx . ins () . bitcast (types :: I16 , MemFlags :: new () , lhs) ; let rhs = fx . bcx . ins () . bitcast (types :: I16 , MemFlags :: new () , rhs) ; let res = fx . bcx . ins () . band_imm (lhs , 0x7fff) ; let sign = fx . bcx . ins () . band_imm (rhs , 0x8000) ; let res = fx . bcx . ins () . bor (res , sign) ; fx . bcx . ins () . bitcast (types :: F16 , MemFlags :: new () , res) }
}

macro_rules! copysign_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copysign_f128 in module {}", module_path!());
    };
}

mkfn!{
    copysign_f128_introspect!();
    pub (crate) fn copysign_f128 (fx : & mut FunctionCx < '_ , '_ , '_ > , lhs : Value , rhs : Value) -> Value { let lhs = fx . bcx . ins () . bitcast (types :: I128 , MemFlags :: new () , lhs) ; let rhs = fx . bcx . ins () . bitcast (types :: I128 , MemFlags :: new () , rhs) ; let (low , lhs_high) = fx . bcx . ins () . isplit (lhs) ; let (_ , rhs_high) = fx . bcx . ins () . isplit (rhs) ; let high = fx . bcx . ins () . band_imm (lhs_high , 0x7fff_ffff_ffff_ffff_u64 as i64) ; let sign = fx . bcx . ins () . band_imm (rhs_high , 0x8000_0000_0000_0000_u64 as i64) ; let high = fx . bcx . ins () . bor (high , sign) ; let res = fx . bcx . ins () . iconcat (low , high) ; fx . bcx . ins () . bitcast (types :: F128 , MemFlags :: new () , res) }
}

macro_rules! codegen_cast_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function codegen_cast in module {}", module_path!());
    };
}

mkfn!{
    codegen_cast_introspect!();
    pub (crate) fn codegen_cast (fx : & mut FunctionCx < '_ , '_ , '_ > , from : Value , from_signed : bool , to_ty : Type , to_signed : bool ,) -> Value { let from_ty = fx . bcx . func . dfg . value_type (from) ; if from_ty . is_float () && to_ty . is_float () { let name = match (from_ty , to_ty) { (types :: F16 , types :: F32) => return f16_to_f32 (fx , from) , (types :: F16 , types :: F64) => return f16_to_f64 (fx , from) , (types :: F16 , types :: F128) => "__extendhftf2" , (types :: F32 , types :: F128) => "__extendsftf2" , (types :: F64 , types :: F128) => "__extenddftf2" , (types :: F128 , types :: F64) => "__trunctfdf2" , (types :: F128 , types :: F32) => "__trunctfsf2" , (types :: F128 , types :: F16) => "__trunctfhf2" , (types :: F64 , types :: F16) => return f64_to_f16 (fx , from) , (types :: F32 , types :: F16) => return f32_to_f16 (fx , from) , _ => unreachable ! ("{from_ty:?} -> {to_ty:?}") , } ; fx . lib_call (name , vec ! [AbiParam :: new (from_ty)] , vec ! [AbiParam :: new (to_ty)] , & [from]) [0] } else if from_ty . is_int () && to_ty == types :: F16 { let res = clif_int_or_float_cast (fx , from , from_signed , types :: F32 , false) ; f32_to_f16 (fx , res) } else if from_ty == types :: F16 && to_ty . is_int () { let from = f16_to_f32 (fx , from) ; clif_int_or_float_cast (fx , from , false , to_ty , to_signed) } else if from_ty . is_int () && to_ty == types :: F128 { let (from , from_ty) = if from_ty . bits () < 32 { (clif_int_or_float_cast (fx , from , from_signed , types :: I32 , from_signed) , types :: I32) } else { (from , from_ty) } ; let name = format ! ("__float{sign}{size}itf" , sign = if from_signed { "" } else { "un" } , size = match from_ty { types :: I32 => 's' , types :: I64 => 'd' , types :: I128 => 't' , _ => unreachable ! ("{from_ty:?}") , } ,) ; fx . lib_call (& name , vec ! [lib_call_arg_param (fx . tcx , from_ty , from_signed)] , vec ! [AbiParam :: new (to_ty)] , & [from] ,) [0] } else if from_ty == types :: F128 && to_ty . is_int () { let ret_ty = if to_ty . bits () < 32 { types :: I32 } else { to_ty } ; let name = format ! ("__fix{sign}tf{size}i" , sign = if from_signed { "" } else { "un" } , size = match ret_ty { types :: I32 => 's' , types :: I64 => 'd' , types :: I128 => 't' , _ => unreachable ! ("{from_ty:?}") , } ,) ; let ret = fx . lib_call (& name , vec ! [AbiParam :: new (from_ty)] , vec ! [AbiParam :: new (to_ty)] , & [from]) [0] ; let val = if ret_ty == to_ty { ret } else { let (min , max) = match (to_ty , to_signed) { (types :: I8 , false) => (0 , i64 :: from (u8 :: MAX)) , (types :: I16 , false) => (0 , i64 :: from (u16 :: MAX)) , (types :: I8 , true) => (i64 :: from (i8 :: MIN as u32) , i64 :: from (i8 :: MAX as u32)) , (types :: I16 , true) => (i64 :: from (i16 :: MIN as u32) , i64 :: from (i16 :: MAX as u32)) , _ => unreachable ! ("{to_ty:?}") , } ; let min_val = fx . bcx . ins () . iconst (types :: I32 , min) ; let max_val = fx . bcx . ins () . iconst (types :: I32 , max) ; let val = if to_signed { let has_underflow = fx . bcx . ins () . icmp_imm (IntCC :: SignedLessThan , ret , min) ; let has_overflow = fx . bcx . ins () . icmp_imm (IntCC :: SignedGreaterThan , ret , max) ; let bottom_capped = fx . bcx . ins () . select (has_underflow , min_val , ret) ; fx . bcx . ins () . select (has_overflow , max_val , bottom_capped) } else { let has_overflow = fx . bcx . ins () . icmp_imm (IntCC :: UnsignedGreaterThan , ret , max) ; fx . bcx . ins () . select (has_overflow , max_val , ret) } ; fx . bcx . ins () . ireduce (to_ty , val) } ; if let Some (false) = fx . tcx . sess . opts . unstable_opts . saturating_float_casts { return val ; } let is_not_nan = fcmp (fx , FloatCC :: Equal , from , from) ; let zero = type_zero_value (& mut fx . bcx , to_ty) ; fx . bcx . ins () . select (is_not_nan , val , zero) } else { unreachable ! ("{from_ty:?} -> {to_ty:?}") ; } }
}

macro_rules! fma_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fma_f16 in module {}", module_path!());
    };
}

mkfn!{
    fma_f16_introspect!();
    pub (crate) fn fma_f16 (fx : & mut FunctionCx < '_ , '_ , '_ > , x : Value , y : Value , z : Value) -> Value { let x = f16_to_f64 (fx , x) ; let y = f16_to_f64 (fx , y) ; let z = f16_to_f64 (fx , z) ; let res = fx . bcx . ins () . fma (x , y , z) ; f64_to_f16 (fx , res) }
}

macro_rules! fmin_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmin_f128 in module {}", module_path!());
    };
}

mkfn!{
    fmin_f128_introspect!();
    pub (crate) fn fmin_f128 (fx : & mut FunctionCx < '_ , '_ , '_ > , a : Value , b : Value) -> Value { fx . lib_call ("fminimumf128" , vec ! [AbiParam :: new (types :: F128) , AbiParam :: new (types :: F128)] , vec ! [AbiParam :: new (types :: F128)] , & [a , b] ,) [0] }
}

macro_rules! fmax_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmax_f128 in module {}", module_path!());
    };
}

mkfn!{
    fmax_f128_introspect!();
    pub (crate) fn fmax_f128 (fx : & mut FunctionCx < '_ , '_ , '_ > , a : Value , b : Value) -> Value { fx . lib_call ("fmaximumf128" , vec ! [AbiParam :: new (types :: F128) , AbiParam :: new (types :: F128)] , vec ! [AbiParam :: new (types :: F128)] , & [a , b] ,) [0] }
}