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

macro_rules! check_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check in module {}", module_path!());
    };
}

mkfn!{
    check_introspect!();
    fn check (x : u16 , singletonuppers : & [(u8 , u8)] , singletonlowers : & [u8] , normal : & [u8]) -> bool { let xupper = (x >> 8) as u8 ; let mut lowerstart = 0 ; for & (upper , lowercount) in singletonuppers { let lowerend = lowerstart + lowercount as usize ; if xupper == upper { for & lower in & singletonlowers [lowerstart .. lowerend] { if lower == x as u8 { return false ; } } } else if xupper < upper { break ; } lowerstart = lowerend ; } let mut x = x as i32 ; let mut normal = normal . iter () . cloned () ; let mut current = true ; while let Some (v) = normal . next () { let len = if v & 0x80 != 0 { ((v & 0x7f) as i32) << 8 | normal . next () . unwrap () as i32 } else { v as i32 } ; x -= len ; if x < 0 { break ; } current = ! current ; } current }
}

macro_rules! is_printable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_printable in module {}", module_path!());
    };
}

mkfn!{
    is_printable_introspect!();
    pub (crate) fn is_printable (x : char) -> bool { let x = x as u32 ; let lower = x as u16 ; if x < 32 { false } else if x < 127 { true } else if x < 0x10000 { check (lower , SINGLETONS0U , SINGLETONS0L , NORMAL0) } else if x < 0x20000 { check (lower , SINGLETONS1U , SINGLETONS1L , NORMAL1) } else { if 0x2a6e0 <= x && x < 0x2a700 { return false ; } if 0x2b73a <= x && x < 0x2b740 { return false ; } if 0x2b81e <= x && x < 0x2b820 { return false ; } if 0x2cea2 <= x && x < 0x2ceb0 { return false ; } if 0x2ebe1 <= x && x < 0x2ebf0 { return false ; } if 0x2ee5e <= x && x < 0x2f800 { return false ; } if 0x2fa1e <= x && x < 0x30000 { return false ; } if 0x3134b <= x && x < 0x31350 { return false ; } if 0x323b0 <= x && x < 0xe0100 { return false ; } if 0xe01f0 <= x && x < 0x110000 { return false ; } true } }
}
mkitem!{# [rustfmt :: skip] const SINGLETONS0U : & [(u8 , u8)] = & [(0x00 , 1) , (0x03 , 5) , (0x05 , 6) , (0x06 , 2) , (0x07 , 6) , (0x08 , 7) , (0x09 , 17) , (0x0a , 28) , (0x0b , 25) , (0x0c , 26) , (0x0d , 16) , (0x0e , 12) , (0x0f , 4) , (0x10 , 3) , (0x12 , 18) , (0x13 , 9) , (0x16 , 1) , (0x17 , 4) , (0x18 , 1) , (0x19 , 3) , (0x1a , 7) , (0x1b , 1) , (0x1c , 2) , (0x1f , 22) , (0x20 , 3) , (0x2b , 3) , (0x2d , 11) , (0x2e , 1) , (0x30 , 4) , (0x31 , 2) , (0x32 , 1) , (0xa7 , 4) , (0xa9 , 2) , (0xaa , 4) , (0xab , 8) , (0xfa , 2) , (0xfb , 5) , (0xfd , 2) , (0xfe , 3) , (0xff , 9) ,] ;}
mkitem!{# [rustfmt :: skip] const SINGLETONS0L : & [u8] = & [0xad , 0x78 , 0x79 , 0x8b , 0x8d , 0xa2 , 0x30 , 0x57 , 0x58 , 0x8b , 0x8c , 0x90 , 0x1c , 0xdd , 0x0e , 0x0f , 0x4b , 0x4c , 0xfb , 0xfc , 0x2e , 0x2f , 0x3f , 0x5c , 0x5d , 0x5f , 0xe2 , 0x84 , 0x8d , 0x8e , 0x91 , 0x92 , 0xa9 , 0xb1 , 0xba , 0xbb , 0xc5 , 0xc6 , 0xc9 , 0xca , 0xde , 0xe4 , 0xe5 , 0xff , 0x00 , 0x04 , 0x11 , 0x12 , 0x29 , 0x31 , 0x34 , 0x37 , 0x3a , 0x3b , 0x3d , 0x49 , 0x4a , 0x5d , 0x84 , 0x8e , 0x92 , 0xa9 , 0xb1 , 0xb4 , 0xba , 0xbb , 0xc6 , 0xca , 0xce , 0xcf , 0xe4 , 0xe5 , 0x00 , 0x04 , 0x0d , 0x0e , 0x11 , 0x12 , 0x29 , 0x31 , 0x34 , 0x3a , 0x3b , 0x45 , 0x46 , 0x49 , 0x4a , 0x5e , 0x64 , 0x65 , 0x84 , 0x91 , 0x9b , 0x9d , 0xc9 , 0xce , 0xcf , 0x0d , 0x11 , 0x29 , 0x3a , 0x3b , 0x45 , 0x49 , 0x57 , 0x5b , 0x5c , 0x5e , 0x5f , 0x64 , 0x65 , 0x8d , 0x91 , 0xa9 , 0xb4 , 0xba , 0xbb , 0xc5 , 0xc9 , 0xdf , 0xe4 , 0xe5 , 0xf0 , 0x0d , 0x11 , 0x45 , 0x49 , 0x64 , 0x65 , 0x80 , 0x84 , 0xb2 , 0xbc , 0xbe , 0xbf , 0xd5 , 0xd7 , 0xf0 , 0xf1 , 0x83 , 0x85 , 0x8b , 0xa4 , 0xa6 , 0xbe , 0xbf , 0xc5 , 0xc7 , 0xcf , 0xda , 0xdb , 0x48 , 0x98 , 0xbd , 0xcd , 0xc6 , 0xce , 0xcf , 0x49 , 0x4e , 0x4f , 0x57 , 0x59 , 0x5e , 0x5f , 0x89 , 0x8e , 0x8f , 0xb1 , 0xb6 , 0xb7 , 0xbf , 0xc1 , 0xc6 , 0xc7 , 0xd7 , 0x11 , 0x16 , 0x17 , 0x5b , 0x5c , 0xf6 , 0xf7 , 0xfe , 0xff , 0x80 , 0x6d , 0x71 , 0xde , 0xdf , 0x0e , 0x1f , 0x6e , 0x6f , 0x1c , 0x1d , 0x5f , 0x7d , 0x7e , 0xae , 0xaf , 0x4d , 0xbb , 0xbc , 0x16 , 0x17 , 0x1e , 0x1f , 0x46 , 0x47 , 0x4e , 0x4f , 0x58 , 0x5a , 0x5c , 0x5e , 0x7e , 0x7f , 0xb5 , 0xc5 , 0xd4 , 0xd5 , 0xdc , 0xf0 , 0xf1 , 0xf5 , 0x72 , 0x73 , 0x8f , 0x74 , 0x75 , 0x96 , 0x26 , 0x2e , 0x2f , 0xa7 , 0xaf , 0xb7 , 0xbf , 0xc7 , 0xcf , 0xd7 , 0xdf , 0x9a , 0x00 , 0x40 , 0x97 , 0x98 , 0x30 , 0x8f , 0x1f , 0xce , 0xcf , 0xd2 , 0xd4 , 0xce , 0xff , 0x4e , 0x4f , 0x5a , 0x5b , 0x07 , 0x08 , 0x0f , 0x10 , 0x27 , 0x2f , 0xee , 0xef , 0x6e , 0x6f , 0x37 , 0x3d , 0x3f , 0x42 , 0x45 , 0x90 , 0x91 , 0x53 , 0x67 , 0x75 , 0xc8 , 0xc9 , 0xd0 , 0xd1 , 0xd8 , 0xd9 , 0xe7 , 0xfe , 0xff ,] ;}
mkitem!{# [rustfmt :: skip] const SINGLETONS1U : & [(u8 , u8)] = & [(0x00 , 6) , (0x01 , 1) , (0x03 , 1) , (0x04 , 2) , (0x05 , 7) , (0x07 , 2) , (0x08 , 8) , (0x09 , 2) , (0x0a , 5) , (0x0b , 2) , (0x0e , 4) , (0x10 , 1) , (0x11 , 2) , (0x12 , 5) , (0x13 , 28) , (0x14 , 1) , (0x15 , 2) , (0x17 , 2) , (0x19 , 13) , (0x1c , 5) , (0x1d , 8) , (0x1f , 1) , (0x24 , 1) , (0x6a , 4) , (0x6b , 2) , (0xaf , 3) , (0xb1 , 2) , (0xbc , 2) , (0xcf , 2) , (0xd1 , 2) , (0xd4 , 12) , (0xd5 , 9) , (0xd6 , 2) , (0xd7 , 2) , (0xda , 1) , (0xe0 , 5) , (0xe1 , 2) , (0xe7 , 4) , (0xe8 , 2) , (0xee , 32) , (0xf0 , 4) , (0xf8 , 2) , (0xfa , 4) , (0xfb , 1) ,] ;}
mkitem!{# [rustfmt :: skip] const SINGLETONS1L : & [u8] = & [0x0c , 0x27 , 0x3b , 0x3e , 0x4e , 0x4f , 0x8f , 0x9e , 0x9e , 0x9f , 0x7b , 0x8b , 0x93 , 0x96 , 0xa2 , 0xb2 , 0xba , 0x86 , 0xb1 , 0x06 , 0x07 , 0x09 , 0x36 , 0x3d , 0x3e , 0x56 , 0xf3 , 0xd0 , 0xd1 , 0x04 , 0x14 , 0x18 , 0x36 , 0x37 , 0x56 , 0x57 , 0x7f , 0xaa , 0xae , 0xaf , 0xbd , 0x35 , 0xe0 , 0x12 , 0x87 , 0x89 , 0x8e , 0x9e , 0x04 , 0x0d , 0x0e , 0x11 , 0x12 , 0x29 , 0x31 , 0x34 , 0x3a , 0x45 , 0x46 , 0x49 , 0x4a , 0x4e , 0x4f , 0x64 , 0x65 , 0x8a , 0x8c , 0x8d , 0x8f , 0xb6 , 0xc1 , 0xc3 , 0xc4 , 0xc6 , 0xcb , 0xd6 , 0x5c , 0xb6 , 0xb7 , 0x1b , 0x1c , 0x07 , 0x08 , 0x0a , 0x0b , 0x14 , 0x17 , 0x36 , 0x39 , 0x3a , 0xa8 , 0xa9 , 0xd8 , 0xd9 , 0x09 , 0x37 , 0x90 , 0x91 , 0xa8 , 0x07 , 0x0a , 0x3b , 0x3e , 0x66 , 0x69 , 0x8f , 0x92 , 0x11 , 0x6f , 0x5f , 0xbf , 0xee , 0xef , 0x5a , 0x62 , 0xf4 , 0xfc , 0xff , 0x53 , 0x54 , 0x9a , 0x9b , 0x2e , 0x2f , 0x27 , 0x28 , 0x55 , 0x9d , 0xa0 , 0xa1 , 0xa3 , 0xa4 , 0xa7 , 0xa8 , 0xad , 0xba , 0xbc , 0xc4 , 0x06 , 0x0b , 0x0c , 0x15 , 0x1d , 0x3a , 0x3f , 0x45 , 0x51 , 0xa6 , 0xa7 , 0xcc , 0xcd , 0xa0 , 0x07 , 0x19 , 0x1a , 0x22 , 0x25 , 0x3e , 0x3f , 0xe7 , 0xec , 0xef , 0xff , 0xc5 , 0xc6 , 0x04 , 0x20 , 0x23 , 0x25 , 0x26 , 0x28 , 0x33 , 0x38 , 0x3a , 0x48 , 0x4a , 0x4c , 0x50 , 0x53 , 0x55 , 0x56 , 0x58 , 0x5a , 0x5c , 0x5e , 0x60 , 0x63 , 0x65 , 0x66 , 0x6b , 0x73 , 0x78 , 0x7d , 0x7f , 0x8a , 0xa4 , 0xaa , 0xaf , 0xb0 , 0xc0 , 0xd0 , 0xae , 0xaf , 0x6e , 0x6f , 0xdd , 0xde , 0x93 ,] ;}
mkitem!{# [rustfmt :: skip] const NORMAL0 : & [u8] = & [0x00 , 0x20 , 0x5f , 0x22 , 0x82 , 0xdf , 0x04 , 0x82 , 0x44 , 0x08 , 0x1b , 0x04 , 0x06 , 0x11 , 0x81 , 0xac , 0x0e , 0x80 , 0xab , 0x05 , 0x1f , 0x08 , 0x81 , 0x1c , 0x03 , 0x19 , 0x08 , 0x01 , 0x04 , 0x2f , 0x04 , 0x34 , 0x04 , 0x07 , 0x03 , 0x01 , 0x07 , 0x06 , 0x07 , 0x11 , 0x0a , 0x50 , 0x0f , 0x12 , 0x07 , 0x55 , 0x07 , 0x03 , 0x04 , 0x1c , 0x0a , 0x09 , 0x03 , 0x08 , 0x03 , 0x07 , 0x03 , 0x02 , 0x03 , 0x03 , 0x03 , 0x0c , 0x04 , 0x05 , 0x03 , 0x0b , 0x06 , 0x01 , 0x0e , 0x15 , 0x05 , 0x4e , 0x07 , 0x1b , 0x07 , 0x57 , 0x07 , 0x02 , 0x06 , 0x17 , 0x0c , 0x50 , 0x04 , 0x43 , 0x03 , 0x2d , 0x03 , 0x01 , 0x04 , 0x11 , 0x06 , 0x0f , 0x0c , 0x3a , 0x04 , 0x1d , 0x25 , 0x5f , 0x20 , 0x6d , 0x04 , 0x6a , 0x25 , 0x80 , 0xc8 , 0x05 , 0x82 , 0xb0 , 0x03 , 0x1a , 0x06 , 0x82 , 0xfd , 0x03 , 0x59 , 0x07 , 0x16 , 0x09 , 0x18 , 0x09 , 0x14 , 0x0c , 0x14 , 0x0c , 0x6a , 0x06 , 0x0a , 0x06 , 0x1a , 0x06 , 0x59 , 0x07 , 0x2b , 0x05 , 0x46 , 0x0a , 0x2c , 0x04 , 0x0c , 0x04 , 0x01 , 0x03 , 0x31 , 0x0b , 0x2c , 0x04 , 0x1a , 0x06 , 0x0b , 0x03 , 0x80 , 0xac , 0x06 , 0x0a , 0x06 , 0x2f , 0x31 , 0x80 , 0xf4 , 0x08 , 0x3c , 0x03 , 0x0f , 0x03 , 0x3e , 0x05 , 0x38 , 0x08 , 0x2b , 0x05 , 0x82 , 0xff , 0x11 , 0x18 , 0x08 , 0x2f , 0x11 , 0x2d , 0x03 , 0x21 , 0x0f , 0x21 , 0x0f , 0x80 , 0x8c , 0x04 , 0x82 , 0x9a , 0x16 , 0x0b , 0x15 , 0x88 , 0x94 , 0x05 , 0x2f , 0x05 , 0x3b , 0x07 , 0x02 , 0x0e , 0x18 , 0x09 , 0x80 , 0xbe , 0x22 , 0x74 , 0x0c , 0x80 , 0xd6 , 0x1a , 0x81 , 0x10 , 0x05 , 0x80 , 0xe1 , 0x09 , 0xf2 , 0x9e , 0x03 , 0x37 , 0x09 , 0x81 , 0x5c , 0x14 , 0x80 , 0xb8 , 0x08 , 0x80 , 0xdd , 0x15 , 0x3b , 0x03 , 0x0a , 0x06 , 0x38 , 0x08 , 0x46 , 0x08 , 0x0c , 0x06 , 0x74 , 0x0b , 0x1e , 0x03 , 0x5a , 0x04 , 0x59 , 0x09 , 0x80 , 0x83 , 0x18 , 0x1c , 0x0a , 0x16 , 0x09 , 0x4c , 0x04 , 0x80 , 0x8a , 0x06 , 0xab , 0xa4 , 0x0c , 0x17 , 0x04 , 0x31 , 0xa1 , 0x04 , 0x81 , 0xda , 0x26 , 0x07 , 0x0c , 0x05 , 0x05 , 0x80 , 0xa6 , 0x10 , 0x81 , 0xf5 , 0x07 , 0x01 , 0x20 , 0x2a , 0x06 , 0x4c , 0x04 , 0x80 , 0x8d , 0x04 , 0x80 , 0xbe , 0x03 , 0x1b , 0x03 , 0x0f , 0x0d ,] ;}
mkitem!{# [rustfmt :: skip] const NORMAL1 : & [u8] = & [0x5e , 0x22 , 0x7b , 0x05 , 0x03 , 0x04 , 0x2d , 0x03 , 0x66 , 0x03 , 0x01 , 0x2f , 0x2e , 0x80 , 0x82 , 0x1d , 0x03 , 0x31 , 0x0f , 0x1c , 0x04 , 0x24 , 0x09 , 0x1e , 0x05 , 0x2b , 0x05 , 0x44 , 0x04 , 0x0e , 0x2a , 0x80 , 0xaa , 0x06 , 0x24 , 0x04 , 0x24 , 0x04 , 0x28 , 0x08 , 0x34 , 0x0b , 0x4e , 0x03 , 0x34 , 0x0c , 0x81 , 0x37 , 0x09 , 0x16 , 0x0a , 0x08 , 0x18 , 0x3b , 0x45 , 0x39 , 0x03 , 0x63 , 0x08 , 0x09 , 0x30 , 0x16 , 0x05 , 0x21 , 0x03 , 0x1b , 0x05 , 0x01 , 0x40 , 0x38 , 0x04 , 0x4b , 0x05 , 0x2f , 0x04 , 0x0a , 0x07 , 0x09 , 0x07 , 0x40 , 0x20 , 0x27 , 0x04 , 0x0c , 0x09 , 0x36 , 0x03 , 0x3a , 0x05 , 0x1a , 0x07 , 0x04 , 0x0c , 0x07 , 0x50 , 0x49 , 0x37 , 0x33 , 0x0d , 0x33 , 0x07 , 0x2e , 0x08 , 0x0a , 0x06 , 0x26 , 0x03 , 0x1d , 0x08 , 0x02 , 0x80 , 0xd0 , 0x52 , 0x10 , 0x03 , 0x37 , 0x2c , 0x08 , 0x2a , 0x16 , 0x1a , 0x26 , 0x1c , 0x14 , 0x17 , 0x09 , 0x4e , 0x04 , 0x24 , 0x09 , 0x44 , 0x0d , 0x19 , 0x07 , 0x0a , 0x06 , 0x48 , 0x08 , 0x27 , 0x09 , 0x75 , 0x0b , 0x42 , 0x3e , 0x2a , 0x06 , 0x3b , 0x05 , 0x0a , 0x06 , 0x51 , 0x06 , 0x01 , 0x05 , 0x10 , 0x03 , 0x05 , 0x0b , 0x59 , 0x08 , 0x02 , 0x1d , 0x62 , 0x1e , 0x48 , 0x08 , 0x0a , 0x80 , 0xa6 , 0x5e , 0x22 , 0x45 , 0x0b , 0x0a , 0x06 , 0x0d , 0x13 , 0x3a , 0x06 , 0x0a , 0x06 , 0x14 , 0x1c , 0x2c , 0x04 , 0x17 , 0x80 , 0xb9 , 0x3c , 0x64 , 0x53 , 0x0c , 0x48 , 0x09 , 0x0a , 0x46 , 0x45 , 0x1b , 0x48 , 0x08 , 0x53 , 0x0d , 0x49 , 0x07 , 0x0a , 0x80 , 0xb6 , 0x22 , 0x0e , 0x0a , 0x06 , 0x46 , 0x0a , 0x1d , 0x03 , 0x47 , 0x49 , 0x37 , 0x03 , 0x0e , 0x08 , 0x0a , 0x06 , 0x39 , 0x07 , 0x0a , 0x81 , 0x36 , 0x19 , 0x07 , 0x3b , 0x03 , 0x1d , 0x55 , 0x01 , 0x0f , 0x32 , 0x0d , 0x83 , 0x9b , 0x66 , 0x75 , 0x0b , 0x80 , 0xc4 , 0x8a , 0x4c , 0x63 , 0x0d , 0x84 , 0x30 , 0x10 , 0x16 , 0x0a , 0x8f , 0x9b , 0x05 , 0x82 , 0x47 , 0x9a , 0xb9 , 0x3a , 0x86 , 0xc6 , 0x82 , 0x39 , 0x07 , 0x2a , 0x04 , 0x5c , 0x06 , 0x26 , 0x0a , 0x46 , 0x0a , 0x28 , 0x05 , 0x13 , 0x81 , 0xb0 , 0x3a , 0x80 , 0xc6 , 0x5b , 0x65 , 0x4b , 0x04 , 0x39 , 0x07 , 0x11 , 0x40 , 0x05 , 0x0b , 0x02 , 0x0e , 0x97 , 0xf8 , 0x08 , 0x84 , 0xd6 , 0x29 , 0x0a , 0xa2 , 0xe7 , 0x81 , 0x33 , 0x0f , 0x01 , 0x1d , 0x06 , 0x0e , 0x04 , 0x08 , 0x81 , 0x8c , 0x89 , 0x04 , 0x6b , 0x05 , 0x0d , 0x03 , 0x09 , 0x07 , 0x10 , 0x8f , 0x60 , 0x80 , 0xfa , 0x06 , 0x81 , 0xb4 , 0x4c , 0x47 , 0x09 , 0x74 , 0x3c , 0x80 , 0xf6 , 0x0a , 0x73 , 0x08 , 0x70 , 0x15 , 0x46 , 0x7a , 0x14 , 0x0c , 0x14 , 0x0c , 0x57 , 0x09 , 0x19 , 0x80 , 0x87 , 0x81 , 0x47 , 0x03 , 0x85 , 0x42 , 0x0f , 0x15 , 0x84 , 0x50 , 0x1f , 0x06 , 0x06 , 0x80 , 0xd5 , 0x2b , 0x05 , 0x3e , 0x21 , 0x01 , 0x70 , 0x2d , 0x03 , 0x1a , 0x04 , 0x02 , 0x81 , 0x40 , 0x1f , 0x11 , 0x3a , 0x05 , 0x01 , 0x81 , 0xd0 , 0x2a , 0x80 , 0xd6 , 0x2b , 0x04 , 0x01 , 0x81 , 0xe0 , 0x80 , 0xf7 , 0x29 , 0x4c , 0x04 , 0x0a , 0x04 , 0x02 , 0x83 , 0x11 , 0x44 , 0x4c , 0x3d , 0x80 , 0xc2 , 0x3c , 0x06 , 0x01 , 0x04 , 0x55 , 0x05 , 0x1b , 0x34 , 0x02 , 0x81 , 0x0e , 0x2c , 0x04 , 0x64 , 0x0c , 0x56 , 0x0a , 0x80 , 0xae , 0x38 , 0x1d , 0x0d , 0x2c , 0x04 , 0x09 , 0x07 , 0x02 , 0x0e , 0x06 , 0x80 , 0x9a , 0x83 , 0xd8 , 0x04 , 0x11 , 0x03 , 0x0d , 0x03 , 0x77 , 0x04 , 0x5f , 0x06 , 0x0c , 0x04 , 0x01 , 0x0f , 0x0c , 0x04 , 0x38 , 0x08 , 0x0a , 0x06 , 0x28 , 0x08 , 0x2c , 0x04 , 0x02 , 0x3e , 0x81 , 0x54 , 0x0c , 0x1d , 0x03 , 0x0a , 0x05 , 0x38 , 0x07 , 0x1c , 0x06 , 0x09 , 0x07 , 0x80 , 0xfa , 0x84 , 0x06 ,] ;}