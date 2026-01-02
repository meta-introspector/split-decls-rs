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
mkuse!{use crate :: time :: Duration ;}
mkitem!{mkstruct!{# [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Debug , Hash)] pub struct Instant (Duration) ;}}
mkitem!{mkstruct!{# [doc = " When a Timezone is specified, the stored Duration is in UTC. If timezone is unspecified, then"] # [doc = " the timezone is assumed to be in UTC."] # [doc = ""] # [doc = " UEFI SystemTime is stored as Duration from 1900-01-01-00:00:00 with timezone -1440 as anchor"] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Debug , Hash)] pub struct SystemTime (Duration) ;}}
mkitem!{pub const UNIX_EPOCH : SystemTime = SystemTime :: from_uefi (r_efi :: efi :: Time { year : 1970 , month : 1 , day : 1 , hour : 0 , minute : 0 , second : 0 , nanosecond : 0 , timezone : 0 , daylight : 0 , pad1 : 0 , pad2 : 0 , }) ;}
mkitem!{const MAX_UEFI_TIME : SystemTime = SystemTime :: from_uefi (r_efi :: efi :: Time { year : 9999 , month : 12 , day : 31 , hour : 23 , minute : 59 , second : 59 , nanosecond : 999_999_999 , timezone : 1440 , daylight : 0 , pad1 : 0 , pad2 : 0 , }) ;}
mkitem!{mkimpl!{impl Instant { pub fn now () -> Instant { if let Some (x) = instant_internal :: timestamp_protocol () { return x ; } if let Some (x) = instant_internal :: platform_specific () { return x ; } panic ! ("time not implemented on this platform") } pub fn checked_sub_instant (& self , other : & Instant) -> Option < Duration > { self . 0 . checked_sub (other . 0) } pub fn checked_add_duration (& self , other : & Duration) -> Option < Instant > { Some (Instant (self . 0 . checked_add (* other) ?)) } pub fn checked_sub_duration (& self , other : & Duration) -> Option < Instant > { Some (Instant (self . 0 . checked_sub (* other) ?)) } }}}
mkitem!{mkimpl!{impl SystemTime { pub (crate) const fn from_uefi (t : r_efi :: efi :: Time) -> Self { Self (system_time_internal :: from_uefi (& t)) } # [expect (dead_code)] pub (crate) const fn to_uefi (self , timezone : i16 , daylight : u8) -> Option < r_efi :: efi :: Time > { system_time_internal :: to_uefi (& self . 0 , timezone , daylight) } pub fn now () -> SystemTime { system_time_internal :: now () . unwrap_or_else (| | panic ! ("time not implemented on this platform")) } pub fn sub_time (& self , other : & SystemTime) -> Result < Duration , Duration > { self . 0 . checked_sub (other . 0) . ok_or_else (| | other . 0 - self . 0) } pub fn checked_add_duration (& self , other : & Duration) -> Option < SystemTime > { let temp = Self (self . 0 . checked_add (* other) ?) ; if temp <= MAX_UEFI_TIME { Some (temp) } else { None } } pub fn checked_sub_duration (& self , other : & Duration) -> Option < SystemTime > { self . 0 . checked_sub (* other) . map (Self) } }}}
mkmod!{system_time_internal, { 
                getname!(system_time_internal);
                getsrc!(system_time_internal);
                getpath!(system_time_internal);
                get_deps!(system_time_internal);
                get_crates!(system_time_internal);
                mkinclude!(system_time_internal);
                mkuse!{use r_efi :: efi :: { RuntimeServices , Time } ;}
mkuse!{use super :: super :: helpers ;}
mkuse!{use super :: * ;}
mkuse!{use crate :: mem :: MaybeUninit ;}
mkuse!{use crate :: ptr :: NonNull ;}
mkitem!{const SECS_IN_MINUTE : u64 = 60 ;}
mkitem!{const SECS_IN_HOUR : u64 = SECS_IN_MINUTE * 60 ;}
mkitem!{const SECS_IN_DAY : u64 = SECS_IN_HOUR * 24 ;}
mkitem!{const TIMEZONE_DELTA : u64 = 1440 * SECS_IN_MINUTE ;}

macro_rules! now_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function now in module {}", module_path!());
    };
}

mkfn!{
    now_introspect!();
    pub fn now () -> Option < SystemTime > { let runtime_services : NonNull < RuntimeServices > = helpers :: runtime_services () ? ; let mut t : MaybeUninit < Time > = MaybeUninit :: uninit () ; let r = unsafe { ((* runtime_services . as_ptr ()) . get_time) (t . as_mut_ptr () , crate :: ptr :: null_mut ()) } ; if r . is_error () { return None ; } let t = unsafe { t . assume_init () } ; Some (SystemTime :: from_uefi (t)) }
}

macro_rules! from_uefi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function from_uefi in module {}", module_path!());
    };
}

mkfn!{
    from_uefi_introspect!();
    # [doc = " This algorithm is a modified form of the one described in the post"] # [doc = " https://blog.reverberate.org/2020/05/12/optimizing-date-algorithms.html"] # [doc = ""] # [doc = " The changes are to use 1900-01-01-00:00:00 with timezone -1440 as anchor instead of UNIX"] # [doc = " epoch used in the original algorithm."] pub (crate) const fn from_uefi (t : & Time) -> Duration { assert ! (t . month <= 12 && t . month != 0) ; assert ! (t . year >= 1900 && t . year <= 9999) ; assert ! (t . day <= 31 && t . day != 0) ; assert ! (t . second < 60) ; assert ! (t . minute < 60) ; assert ! (t . hour < 24) ; assert ! (t . nanosecond < 1_000_000_000) ; assert ! ((t . timezone <= 1440 && t . timezone >= - 1440) || t . timezone == r_efi :: efi :: UNSPECIFIED_TIMEZONE) ; const YEAR_BASE : u32 = 4800 ; let (m_adj , overflow) : (u32 , bool) = (t . month as u32) . overflowing_sub (3) ; let (carry , adjust) : (u32 , u32) = if overflow { (1 , 12) } else { (0 , 0) } ; let y_adj : u32 = (t . year as u32) + YEAR_BASE - carry ; let month_days : u32 = (m_adj . wrapping_add (adjust) * 62719 + 769) / 2048 ; let leap_days : u32 = y_adj / 4 - y_adj / 100 + y_adj / 400 ; let days : u32 = y_adj * 365 + leap_days + month_days + (t . day as u32 - 1) - 2447065 ; let localtime_epoch : u64 = (days as u64) * SECS_IN_DAY + (t . second as u64) + (t . minute as u64) * SECS_IN_MINUTE + (t . hour as u64) * SECS_IN_HOUR ; let adjusted_localtime_epoc : u64 = localtime_epoch + TIMEZONE_DELTA ; let epoch : u64 = if t . timezone == r_efi :: efi :: UNSPECIFIED_TIMEZONE { adjusted_localtime_epoc } else { adjusted_localtime_epoc . checked_add_signed ((t . timezone as i64) * SECS_IN_MINUTE as i64) . unwrap () } ; Duration :: new (epoch , t . nanosecond) }
}

macro_rules! to_uefi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_uefi in module {}", module_path!());
    };
}

mkfn!{
    to_uefi_introspect!();
    # [doc = " This algorithm is a modified version of the one described in the post:"] # [doc = " https://howardhinnant.github.io/date_algorithms.html#clive_from_days"] # [doc = ""] # [doc = " The changes are to use 1900-01-01-00:00:00 with timezone -1440 as anchor instead of UNIX"] # [doc = " epoch used in the original algorithm."] pub (crate) const fn to_uefi (dur : & Duration , timezone : i16 , daylight : u8) -> Option < Time > { assert ! (timezone <= 1440 && timezone >= - 1440) ; let secs = dur . as_secs () . checked_add_signed ((- timezone as i64) * SECS_IN_MINUTE as i64) . unwrap () ; let Some (secs) = secs . checked_sub (TIMEZONE_DELTA) else { return None } ; let days = secs / SECS_IN_DAY ; let remaining_secs = secs % SECS_IN_DAY ; let z = days + 693901 ; let era = z / 146097 ; let doe = z - (era * 146097) ; let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365 ; let mut y = yoe + era * 400 ; let doy = doe - (365 * yoe + yoe / 4 - yoe / 100) ; let mp = (5 * doy + 2) / 153 ; let d = doy - (153 * mp + 2) / 5 + 1 ; let m = if mp < 10 { mp + 3 } else { mp - 9 } ; if m <= 2 { y += 1 ; } let hour = (remaining_secs / SECS_IN_HOUR) as u8 ; let minute = ((remaining_secs % SECS_IN_HOUR) / SECS_IN_MINUTE) as u8 ; let second = (remaining_secs % SECS_IN_MINUTE) as u8 ; if y >= 1900 && y <= 9999 { Some (Time { year : y as u16 , month : m as u8 , day : d as u8 , hour , minute , second , nanosecond : dur . subsec_nanos () , timezone , daylight , pad1 : 0 , pad2 : 0 , }) } else { None } }
} 
            }}
mkmod!{instant_internal, { 
                getname!(instant_internal);
                getsrc!(instant_internal);
                getpath!(instant_internal);
                get_deps!(instant_internal);
                get_crates!(instant_internal);
                mkinclude!(instant_internal);
                mkuse!{use r_efi :: protocols :: timestamp ;}
mkuse!{use super :: super :: helpers ;}
mkuse!{use super :: * ;}
mkuse!{use crate :: mem :: MaybeUninit ;}
mkuse!{use crate :: ptr :: NonNull ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicPtr , Ordering } ;}
mkuse!{use crate :: sys_common :: mul_div_u64 ;}
mkitem!{const NS_PER_SEC : u64 = 1_000_000_000 ;}

macro_rules! timestamp_protocol_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function timestamp_protocol in module {}", module_path!());
    };
}

mkfn!{
    timestamp_protocol_introspect!();
    pub fn timestamp_protocol () -> Option < Instant > { fn try_handle (handle : NonNull < crate :: ffi :: c_void >) -> Option < u64 > { let protocol : NonNull < timestamp :: Protocol > = helpers :: open_protocol (handle , timestamp :: PROTOCOL_GUID) . ok () ? ; let mut properties : MaybeUninit < timestamp :: Properties > = MaybeUninit :: uninit () ; let r = unsafe { ((* protocol . as_ptr ()) . get_properties) (properties . as_mut_ptr ()) } ; if r . is_error () { return None ; } let freq = unsafe { properties . assume_init () . frequency } ; let ts = unsafe { ((* protocol . as_ptr ()) . get_timestamp) () } ; Some (mul_div_u64 (ts , NS_PER_SEC , freq)) } static LAST_VALID_HANDLE : Atomic < * mut crate :: ffi :: c_void > = AtomicPtr :: new (crate :: ptr :: null_mut ()) ; if let Some (handle) = NonNull :: new (LAST_VALID_HANDLE . load (Ordering :: Acquire)) { if let Some (ns) = try_handle (handle) { return Some (Instant (Duration :: from_nanos (ns))) ; } } if let Ok (handles) = helpers :: locate_handles (timestamp :: PROTOCOL_GUID) { for handle in handles { if let Some (ns) = try_handle (handle) { LAST_VALID_HANDLE . store (handle . as_ptr () , Ordering :: Release) ; return Some (Instant (Duration :: from_nanos (ns))) ; } } } None }
}

macro_rules! platform_specific_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function platform_specific in module {}", module_path!());
    };
}

mkfn!{
    platform_specific_introspect!();
    pub fn platform_specific () -> Option < Instant > { cfg_select ! { any (target_arch = "x86_64" , target_arch = "x86") => timestamp_rdtsc () . map (Instant) , _ => None , } }
}

macro_rules! timestamp_rdtsc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function timestamp_rdtsc in module {}", module_path!());
    };
}

mkfn!{
    timestamp_rdtsc_introspect!();
    # [cfg (target_arch = "x86_64")] fn timestamp_rdtsc () -> Option < Duration > { static FREQUENCY : crate :: sync :: OnceLock < u64 > = crate :: sync :: OnceLock :: new () ; let freq = FREQUENCY . get_or_try_init (| | { let cpuid = unsafe { crate :: arch :: x86_64 :: __cpuid (0x15) } ; if cpuid . eax == 0 || cpuid . ebx == 0 || cpuid . ecx == 0 { return Err (()) ; } Ok (mul_div_u64 (cpuid . ecx as u64 , cpuid . ebx as u64 , cpuid . eax as u64)) }) . ok () ? ; let ts = unsafe { crate :: arch :: x86_64 :: _rdtsc () } ; let ns = mul_div_u64 (ts , 1000 , * freq) ; Some (Duration :: from_nanos (ns)) }
}

macro_rules! timestamp_rdtsc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function timestamp_rdtsc in module {}", module_path!());
    };
}

mkfn!{
    timestamp_rdtsc_introspect!();
    # [cfg (target_arch = "x86")] fn timestamp_rdtsc () -> Option < Duration > { static FREQUENCY : crate :: sync :: OnceLock < u64 > = crate :: sync :: OnceLock :: new () ; let freq = FREQUENCY . get_or_try_init (| | { let cpuid = unsafe { crate :: arch :: x86 :: __cpuid (0x15) } ; if cpuid . eax == 0 || cpuid . ebx == 0 || cpuid . ecx == 0 { return Err (()) ; } Ok (mul_div_u64 (cpuid . ecx as u64 , cpuid . ebx as u64 , cpuid . eax as u64)) }) . ok () ? ; let ts = unsafe { crate :: arch :: x86 :: _rdtsc () } ; let ns = mul_div_u64 (ts , 1000 , * freq) ; Some (Duration :: from_nanos (ns)) }
} 
            }}