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
mkuse!{use crate :: fmt ;}
mkuse!{use crate :: sync :: WaitTimeoutResult ;}
mkuse!{use crate :: sync :: nonpoison :: { MutexGuard , mutex } ;}
mkuse!{use crate :: sys :: sync as sys ;}
mkuse!{use crate :: time :: { Duration , Instant } ;}
mkitem!{mkstruct!{# [doc = " A Condition Variable"] # [doc = ""] # [doc = " For more information about condition variables, check out the documentation for the poisoning"] # [doc = " variant of this type at [`poison::Condvar`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Note that this `Condvar` does **not** propagate information about threads that panic while"] # [doc = " holding a lock. If you need this functionality, see [`poison::Mutex`] and [`poison::Condvar`]."] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_mutex)]"] # [doc = " #![feature(nonpoison_condvar)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::{Mutex, Condvar};"] # [doc = " use std::sync::Arc;"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let pair = Arc::new((Mutex::new(false), Condvar::new()));"] # [doc = " let pair2 = Arc::clone(&pair);"] # [doc = ""] # [doc = " // Inside of our lock, spawn a new thread, and then wait for it to start."] # [doc = " thread::spawn(move || {"] # [doc = "     let (lock, cvar) = &*pair2;"] # [doc = "     let mut started = lock.lock();"] # [doc = "     *started = true;"] # [doc = "     // We notify the condvar that the value has changed."] # [doc = "     cvar.notify_one();"] # [doc = " });"] # [doc = ""] # [doc = " // Wait for the thread to start up."] # [doc = " let (lock, cvar) = &*pair;"] # [doc = " let mut started = lock.lock();"] # [doc = " while !*started {"] # [doc = "     started = cvar.wait(started);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [`poison::Mutex`]: crate::sync::poison::Mutex"] # [doc = " [`poison::Condvar`]: crate::sync::poison::Condvar"] # [unstable (feature = "nonpoison_condvar" , issue = "134645")] pub struct Condvar { inner : sys :: Condvar , }}}
mkitem!{mkimpl!{impl Condvar { # [doc = " Creates a new condition variable which is ready to be waited on and"] # [doc = " notified."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::Condvar;"] # [doc = ""] # [doc = " let condvar = Condvar::new();"] # [doc = " ```"] # [unstable (feature = "nonpoison_condvar" , issue = "134645")] # [must_use] # [inline] pub const fn new () -> Condvar { Condvar { inner : sys :: Condvar :: new () } } # [doc = " Blocks the current thread until this condition variable receives a"] # [doc = " notification."] # [doc = ""] # [doc = " This function will atomically unlock the mutex specified (represented by"] # [doc = " `guard`) and block the current thread. This means that any calls"] # [doc = " to [`notify_one`] or [`notify_all`] which happen logically after the"] # [doc = " mutex is unlocked are candidates to wake this thread up. When this"] # [doc = " function call returns, the lock specified will have been re-acquired."] # [doc = ""] # [doc = " Note that this function is susceptible to spurious wakeups. Condition"] # [doc = " variables normally have a boolean predicate associated with them, and"] # [doc = " the predicate must always be checked each time this function returns to"] # [doc = " protect against spurious wakeups."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function may [`panic!`] if it is used with more than one mutex"] # [doc = " over time."] # [doc = ""] # [doc = " [`notify_one`]: Self::notify_one"] # [doc = " [`notify_all`]: Self::notify_all"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_mutex)]"] # [doc = " #![feature(nonpoison_condvar)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::{Mutex, Condvar};"] # [doc = " use std::sync::Arc;"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let pair = Arc::new((Mutex::new(false), Condvar::new()));"] # [doc = " let pair2 = Arc::clone(&pair);"] # [doc = ""] # [doc = " thread::spawn(move || {"] # [doc = "     let (lock, cvar) = &*pair2;"] # [doc = "     let mut started = lock.lock();"] # [doc = "     *started = true;"] # [doc = "     // We notify the condvar that the value has changed."] # [doc = "     cvar.notify_one();"] # [doc = " });"] # [doc = ""] # [doc = " // Wait for the thread to start up."] # [doc = " let (lock, cvar) = &*pair;"] # [doc = " let mut started = lock.lock();"] # [doc = " // As long as the value inside the `Mutex<bool>` is `false`, we wait."] # [doc = " while !*started {"] # [doc = "     started = cvar.wait(started);"] # [doc = " }"] # [doc = " ```"] # [unstable (feature = "nonpoison_condvar" , issue = "134645")] pub fn wait < 'a , T > (& self , guard : MutexGuard < 'a , T >) -> MutexGuard < 'a , T > { unsafe { let lock = mutex :: guard_lock (& guard) ; self . inner . wait (lock) ; } guard } # [doc = " Blocks the current thread until the provided condition becomes false."] # [doc = ""] # [doc = " `condition` is checked immediately; if not met (returns `true`), this"] # [doc = " will [`wait`] for the next notification then check again. This repeats"] # [doc = " until `condition` returns `false`, in which case this function returns."] # [doc = ""] # [doc = " This function will atomically unlock the mutex specified (represented by"] # [doc = " `guard`) and block the current thread. This means that any calls"] # [doc = " to [`notify_one`] or [`notify_all`] which happen logically after the"] # [doc = " mutex is unlocked are candidates to wake this thread up. When this"] # [doc = " function call returns, the lock specified will have been re-acquired."] # [doc = ""] # [doc = " [`wait`]: Self::wait"] # [doc = " [`notify_one`]: Self::notify_one"] # [doc = " [`notify_all`]: Self::notify_all"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_mutex)]"] # [doc = " #![feature(nonpoison_condvar)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::{Mutex, Condvar};"] # [doc = " use std::sync::Arc;"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let pair = Arc::new((Mutex::new(true), Condvar::new()));"] # [doc = " let pair2 = Arc::clone(&pair);"] # [doc = ""] # [doc = " thread::spawn(move || {"] # [doc = "     let (lock, cvar) = &*pair2;"] # [doc = "     let mut pending = lock.lock();"] # [doc = "     *pending = false;"] # [doc = "     // We notify the condvar that the value has changed."] # [doc = "     cvar.notify_one();"] # [doc = " });"] # [doc = ""] # [doc = " // Wait for the thread to start up."] # [doc = " let (lock, cvar) = &*pair;"] # [doc = " // As long as the value inside the `Mutex<bool>` is `true`, we wait."] # [doc = " let _guard = cvar.wait_while(lock.lock(), |pending| { *pending });"] # [doc = " ```"] # [unstable (feature = "nonpoison_condvar" , issue = "134645")] pub fn wait_while < 'a , T , F > (& self , mut guard : MutexGuard < 'a , T > , mut condition : F ,) -> MutexGuard < 'a , T > where F : FnMut (& mut T) -> bool , { while condition (& mut * guard) { guard = self . wait (guard) ; } guard } # [doc = " Waits on this condition variable for a notification, timing out after a"] # [doc = " specified duration."] # [doc = ""] # [doc = " The semantics of this function are equivalent to [`wait`] except that"] # [doc = " the thread will be blocked for roughly no longer than `dur`. This"] # [doc = " method should not be used for precise timing due to anomalies such as"] # [doc = " preemption or platform differences that might not cause the maximum"] # [doc = " amount of time waited to be precisely `dur`."] # [doc = ""] # [doc = " Note that the best effort is made to ensure that the time waited is"] # [doc = " measured with a monotonic clock, and not affected by the changes made to"] # [doc = " the system time. This function is susceptible to spurious wakeups."] # [doc = " Condition variables normally have a boolean predicate associated with"] # [doc = " them, and the predicate must always be checked each time this function"] # [doc = " returns to protect against spurious wakeups.  Furthermore, since the timeout"] # [doc = " is given relative to the moment this function is called, it needs to be adjusted"] # [doc = " when this function is called in a loop. The [`wait_timeout_while`] method"] # [doc = " lets you wait with a timeout while a predicate is true, taking care of all these concerns."] # [doc = ""] # [doc = " The returned [`WaitTimeoutResult`] value indicates if the timeout is"] # [doc = " known to have elapsed."] # [doc = ""] # [doc = " Like [`wait`], the lock specified will be re-acquired when this function"] # [doc = " returns, regardless of whether the timeout elapsed or not."] # [doc = ""] # [doc = " [`wait`]: Self::wait"] # [doc = " [`wait_timeout_while`]: Self::wait_timeout_while"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_mutex)]"] # [doc = " #![feature(nonpoison_condvar)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::{Mutex, Condvar};"] # [doc = " use std::sync::Arc;"] # [doc = " use std::thread;"] # [doc = " use std::time::Duration;"] # [doc = ""] # [doc = " let pair = Arc::new((Mutex::new(false), Condvar::new()));"] # [doc = " let pair2 = Arc::clone(&pair);"] # [doc = ""] # [doc = " thread::spawn(move || {"] # [doc = "     let (lock, cvar) = &*pair2;"] # [doc = "     let mut started = lock.lock();"] # [doc = "     *started = true;"] # [doc = "     // We notify the condvar that the value has changed."] # [doc = "     cvar.notify_one();"] # [doc = " });"] # [doc = ""] # [doc = " // wait for the thread to start up"] # [doc = " let (lock, cvar) = &*pair;"] # [doc = " let mut started = lock.lock();"] # [doc = " // as long as the value inside the `Mutex<bool>` is `false`, we wait"] # [doc = " loop {"] # [doc = "     let result = cvar.wait_timeout(started, Duration::from_millis(10));"] # [doc = "     // 10 milliseconds have passed, or maybe the value changed!"] # [doc = "     started = result.0;"] # [doc = "     if *started == true {"] # [doc = "         // We received the notification and the value has been updated, we can leave."] # [doc = "         break"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [unstable (feature = "nonpoison_condvar" , issue = "134645")] pub fn wait_timeout < 'a , T > (& self , guard : MutexGuard < 'a , T > , dur : Duration ,) -> (MutexGuard < 'a , T > , WaitTimeoutResult) { let success = unsafe { let lock = mutex :: guard_lock (& guard) ; self . inner . wait_timeout (lock , dur) } ; (guard , WaitTimeoutResult (! success)) } # [doc = " Waits on this condition variable for a notification, timing out after a"] # [doc = " specified duration."] # [doc = ""] # [doc = " The semantics of this function are equivalent to [`wait_while`] except"] # [doc = " that the thread will be blocked for roughly no longer than `dur`. This"] # [doc = " method should not be used for precise timing due to anomalies such as"] # [doc = " preemption or platform differences that might not cause the maximum"] # [doc = " amount of time waited to be precisely `dur`."] # [doc = ""] # [doc = " Note that the best effort is made to ensure that the time waited is"] # [doc = " measured with a monotonic clock, and not affected by the changes made to"] # [doc = " the system time."] # [doc = ""] # [doc = " The returned [`WaitTimeoutResult`] value indicates if the timeout is"] # [doc = " known to have elapsed without the condition being met."] # [doc = ""] # [doc = " Like [`wait_while`], the lock specified will be re-acquired when this"] # [doc = " function returns, regardless of whether the timeout elapsed or not."] # [doc = ""] # [doc = " [`wait_while`]: Self::wait_while"] # [doc = " [`wait_timeout`]: Self::wait_timeout"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_mutex)]"] # [doc = " #![feature(nonpoison_condvar)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::{Mutex, Condvar};"] # [doc = " use std::sync::Arc;"] # [doc = " use std::thread;"] # [doc = " use std::time::Duration;"] # [doc = ""] # [doc = " let pair = Arc::new((Mutex::new(true), Condvar::new()));"] # [doc = " let pair2 = Arc::clone(&pair);"] # [doc = ""] # [doc = " thread::spawn(move || {"] # [doc = "     let (lock, cvar) = &*pair2;"] # [doc = "     let mut pending = lock.lock();"] # [doc = "     *pending = false;"] # [doc = "     // We notify the condvar that the value has changed."] # [doc = "     cvar.notify_one();"] # [doc = " });"] # [doc = ""] # [doc = " // wait for the thread to start up"] # [doc = " let (lock, cvar) = &*pair;"] # [doc = " let result = cvar.wait_timeout_while("] # [doc = "     lock.lock(),"] # [doc = "     Duration::from_millis(100),"] # [doc = "     |&mut pending| pending,"] # [doc = " );"] # [doc = " if result.1.timed_out() {"] # [doc = "     // timed-out without the condition ever evaluating to false."] # [doc = " }"] # [doc = " // access the locked mutex via result.0"] # [doc = " ```"] # [unstable (feature = "nonpoison_condvar" , issue = "134645")] pub fn wait_timeout_while < 'a , T , F > (& self , mut guard : MutexGuard < 'a , T > , dur : Duration , mut condition : F ,) -> (MutexGuard < 'a , T > , WaitTimeoutResult) where F : FnMut (& mut T) -> bool , { let start = Instant :: now () ; loop { if ! condition (& mut * guard) { return (guard , WaitTimeoutResult (false)) ; } let timeout = match dur . checked_sub (start . elapsed ()) { Some (timeout) => timeout , None => return (guard , WaitTimeoutResult (true)) , } ; guard = self . wait_timeout (guard , timeout) . 0 ; } } # [doc = " Wakes up one blocked thread on this condvar."] # [doc = ""] # [doc = " If there is a blocked thread on this condition variable, then it will"] # [doc = " be woken up from its call to [`wait`] or [`wait_timeout`]. Calls to"] # [doc = " `notify_one` are not buffered in any way."] # [doc = ""] # [doc = " To wake up all threads, see [`notify_all`]."] # [doc = ""] # [doc = " [`wait`]: Self::wait"] # [doc = " [`wait_timeout`]: Self::wait_timeout"] # [doc = " [`notify_all`]: Self::notify_all"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_mutex)]"] # [doc = " #![feature(nonpoison_condvar)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::{Mutex, Condvar};"] # [doc = " use std::sync::Arc;"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let pair = Arc::new((Mutex::new(false), Condvar::new()));"] # [doc = " let pair2 = Arc::clone(&pair);"] # [doc = ""] # [doc = " thread::spawn(move || {"] # [doc = "     let (lock, cvar) = &*pair2;"] # [doc = "     let mut started = lock.lock();"] # [doc = "     *started = true;"] # [doc = "     // We notify the condvar that the value has changed."] # [doc = "     cvar.notify_one();"] # [doc = " });"] # [doc = ""] # [doc = " // Wait for the thread to start up."] # [doc = " let (lock, cvar) = &*pair;"] # [doc = " let mut started = lock.lock();"] # [doc = " // As long as the value inside the `Mutex<bool>` is `false`, we wait."] # [doc = " while !*started {"] # [doc = "     started = cvar.wait(started);"] # [doc = " }"] # [doc = " ```"] # [unstable (feature = "nonpoison_condvar" , issue = "134645")] pub fn notify_one (& self) { self . inner . notify_one () } # [doc = " Wakes up all blocked threads on this condvar."] # [doc = ""] # [doc = " This method will ensure that any current waiters on the condition"] # [doc = " variable are awoken. Calls to `notify_all()` are not buffered in any"] # [doc = " way."] # [doc = ""] # [doc = " To wake up only one thread, see [`notify_one`]."] # [doc = ""] # [doc = " [`notify_one`]: Self::notify_one"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_mutex)]"] # [doc = " #![feature(nonpoison_condvar)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::{Mutex, Condvar};"] # [doc = " use std::sync::Arc;"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let pair = Arc::new((Mutex::new(false), Condvar::new()));"] # [doc = " let pair2 = Arc::clone(&pair);"] # [doc = ""] # [doc = " thread::spawn(move || {"] # [doc = "     let (lock, cvar) = &*pair2;"] # [doc = "     let mut started = lock.lock();"] # [doc = "     *started = true;"] # [doc = "     // We notify the condvar that the value has changed."] # [doc = "     cvar.notify_all();"] # [doc = " });"] # [doc = ""] # [doc = " // Wait for the thread to start up."] # [doc = " let (lock, cvar) = &*pair;"] # [doc = " let mut started = lock.lock();"] # [doc = " // As long as the value inside the `Mutex<bool>` is `false`, we wait."] # [doc = " while !*started {"] # [doc = "     started = cvar.wait(started);"] # [doc = " }"] # [doc = " ```"] # [unstable (feature = "nonpoison_condvar" , issue = "134645")] pub fn notify_all (& self) { self . inner . notify_all () } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_condvar" , issue = "134645")] impl fmt :: Debug for Condvar { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Condvar") . finish_non_exhaustive () } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_condvar" , issue = "134645")] impl Default for Condvar { # [doc = " Creates a `Condvar` which is ready to be waited on and notified."] fn default () -> Condvar { Condvar :: new () } }}}