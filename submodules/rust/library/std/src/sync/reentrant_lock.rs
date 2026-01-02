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
mkuse!{use crate :: cell :: UnsafeCell ;}
mkuse!{use crate :: fmt ;}
mkuse!{use crate :: ops :: Deref ;}
mkuse!{use crate :: panic :: { RefUnwindSafe , UnwindSafe } ;}
mkuse!{use crate :: sys :: sync as sys ;}
mkuse!{use crate :: thread :: { ThreadId , current_id } ;}
mkitem!{mkstruct!{# [doc = " A re-entrant mutual exclusion lock"] # [doc = ""] # [doc = " This lock will block *other* threads waiting for the lock to become"] # [doc = " available. The thread which has already locked the mutex can lock it"] # [doc = " multiple times without blocking, preventing a common source of deadlocks."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Allow recursively calling a function needing synchronization from within"] # [doc = " a callback (this is how [`StdoutLock`](crate::io::StdoutLock) is currently"] # [doc = " implemented):"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(reentrant_lock)]"] # [doc = ""] # [doc = " use std::cell::RefCell;"] # [doc = " use std::sync::ReentrantLock;"] # [doc = ""] # [doc = " pub struct Log {"] # [doc = "     data: RefCell<String>,"] # [doc = " }"] # [doc = ""] # [doc = " impl Log {"] # [doc = "     pub fn append(&self, msg: &str) {"] # [doc = "         self.data.borrow_mut().push_str(msg);"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " static LOG: ReentrantLock<Log> = ReentrantLock::new(Log { data: RefCell::new(String::new()) });"] # [doc = ""] # [doc = " pub fn with_log<R>(f: impl FnOnce(&Log) -> R) -> R {"] # [doc = "     let log = LOG.lock();"] # [doc = "     f(&*log)"] # [doc = " }"] # [doc = ""] # [doc = " with_log(|log| {"] # [doc = "     log.append(\"Hello\");"] # [doc = "     with_log(|log| log.append(\" there!\"));"] # [doc = " });"] # [doc = " ```"] # [doc = ""] # [unstable (feature = "reentrant_lock" , issue = "121440")] pub struct ReentrantLock < T : ? Sized > { mutex : sys :: Mutex , owner : Tid , lock_count : UnsafeCell < u32 > , data : T , }}}
mkitem!{cfg_select ! (target_has_atomic = "64" => { use crate :: sync :: atomic :: { Atomic , AtomicU64 , Ordering :: Relaxed } ; struct Tid (Atomic < u64 >) ; impl Tid { const fn new () -> Self { Self (AtomicU64 :: new (0)) } # [inline] fn contains (& self , owner : ThreadId) -> bool { owner . as_u64 () . get () == self . 0 . load (Relaxed) } # [inline] unsafe fn set (& self , tid : Option < ThreadId >) { let value = tid . map_or (0 , | tid | tid . as_u64 () . get ()) ; self . 0 . store (value , Relaxed) ; } } } _ => { # [doc = " Returns the address of a TLS variable. This is guaranteed to"] # [doc = " be unique across all currently alive threads."] fn tls_addr () -> usize { thread_local ! { static X : u8 = const { 0u8 } } ; X . with (| p | <* const u8 >:: addr (p)) } use crate :: sync :: atomic :: { Atomic , AtomicUsize , Ordering , } ; struct Tid { tls_addr : Atomic < usize >, tid : UnsafeCell < u64 >, } unsafe impl Send for Tid { } unsafe impl Sync for Tid { } impl Tid { const fn new () -> Self { Self { tls_addr : AtomicUsize :: new (0) , tid : UnsafeCell :: new (0) } } # [inline] fn contains (& self , owner : ThreadId) -> bool { let tls_addr = tls_addr () ; self . tls_addr . load (Ordering :: Relaxed) == tls_addr && unsafe { * self . tid . get () } == owner . as_u64 () . get () } # [inline] unsafe fn set (& self , tid : Option < ThreadId >) { let tls_addr = if tid . is_some () { tls_addr () } else { 0 } ; let value = tid . map_or (0 , | tid | tid . as_u64 () . get ()) ; self . tls_addr . store (tls_addr , Ordering :: Relaxed) ; unsafe { * self . tid . get () = value } ; } } }) ;}
mkitem!{mkimpl!{# [unstable (feature = "reentrant_lock" , issue = "121440")] unsafe impl < T : Send + ? Sized > Send for ReentrantLock < T > { }}}
mkitem!{mkimpl!{# [unstable (feature = "reentrant_lock" , issue = "121440")] unsafe impl < T : Send + ? Sized > Sync for ReentrantLock < T > { }}}
mkitem!{mkimpl!{# [unstable (feature = "reentrant_lock" , issue = "121440")] impl < T : UnwindSafe + ? Sized > UnwindSafe for ReentrantLock < T > { }}}
mkitem!{mkimpl!{# [unstable (feature = "reentrant_lock" , issue = "121440")] impl < T : RefUnwindSafe + ? Sized > RefUnwindSafe for ReentrantLock < T > { }}}
mkitem!{mkstruct!{# [doc = " An RAII implementation of a \"scoped lock\" of a re-entrant lock. When this"] # [doc = " structure is dropped (falls out of scope), the lock will be unlocked."] # [doc = ""] # [doc = " The data protected by the mutex can be accessed through this guard via its"] # [doc = " [`Deref`] implementation."] # [doc = ""] # [doc = " This structure is created by the [`lock`](ReentrantLock::lock) method on"] # [doc = " [`ReentrantLock`]."] # [doc = ""] # [doc = " # Mutability"] # [doc = ""] # [doc = " Unlike [`MutexGuard`](super::MutexGuard), `ReentrantLockGuard` does not"] # [doc = " implement [`DerefMut`](crate::ops::DerefMut), because implementation of"] # [doc = " the trait would violate Rust’s reference aliasing rules. Use interior"] # [doc = " mutability (usually [`RefCell`](crate::cell::RefCell)) in order to mutate"] # [doc = " the guarded data."] # [must_use = "if unused the ReentrantLock will immediately unlock"] # [unstable (feature = "reentrant_lock" , issue = "121440")] pub struct ReentrantLockGuard < 'a , T : ? Sized + 'a > { lock : & 'a ReentrantLock < T > , }}}
mkitem!{mkimpl!{# [unstable (feature = "reentrant_lock" , issue = "121440")] impl < T : ? Sized > ! Send for ReentrantLockGuard < '_ , T > { }}}
mkitem!{mkimpl!{# [unstable (feature = "reentrant_lock" , issue = "121440")] unsafe impl < T : ? Sized + Sync > Sync for ReentrantLockGuard < '_ , T > { }}}
mkitem!{mkimpl!{# [unstable (feature = "reentrant_lock" , issue = "121440")] impl < T > ReentrantLock < T > { # [doc = " Creates a new re-entrant lock in an unlocked state ready for use."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(reentrant_lock)]"] # [doc = " use std::sync::ReentrantLock;"] # [doc = ""] # [doc = " let lock = ReentrantLock::new(0);"] # [doc = " ```"] pub const fn new (t : T) -> ReentrantLock < T > { ReentrantLock { mutex : sys :: Mutex :: new () , owner : Tid :: new () , lock_count : UnsafeCell :: new (0) , data : t , } } # [doc = " Consumes this lock, returning the underlying data."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(reentrant_lock)]"] # [doc = ""] # [doc = " use std::sync::ReentrantLock;"] # [doc = ""] # [doc = " let lock = ReentrantLock::new(0);"] # [doc = " assert_eq!(lock.into_inner(), 0);"] # [doc = " ```"] pub fn into_inner (self) -> T { self . data } }}}
mkitem!{mkimpl!{# [unstable (feature = "reentrant_lock" , issue = "121440")] impl < T : ? Sized > ReentrantLock < T > { # [doc = " Acquires the lock, blocking the current thread until it is able to do"] # [doc = " so."] # [doc = ""] # [doc = " This function will block the caller until it is available to acquire"] # [doc = " the lock. Upon returning, the thread is the only thread with the lock"] # [doc = " held. When the thread calling this method already holds the lock, the"] # [doc = " call succeeds without blocking."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(reentrant_lock)]"] # [doc = " use std::cell::Cell;"] # [doc = " use std::sync::{Arc, ReentrantLock};"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let lock = Arc::new(ReentrantLock::new(Cell::new(0)));"] # [doc = " let c_lock = Arc::clone(&lock);"] # [doc = ""] # [doc = " thread::spawn(move || {"] # [doc = "     c_lock.lock().set(10);"] # [doc = " }).join().expect(\"thread::spawn failed\");"] # [doc = " assert_eq!(lock.lock().get(), 10);"] # [doc = " ```"] pub fn lock (& self) -> ReentrantLockGuard < '_ , T > { let this_thread = current_id () ; unsafe { if self . owner . contains (this_thread) { self . increment_lock_count () . expect ("lock count overflow in reentrant mutex") ; } else { self . mutex . lock () ; self . owner . set (Some (this_thread)) ; debug_assert_eq ! (* self . lock_count . get () , 0) ; * self . lock_count . get () = 1 ; } } ReentrantLockGuard { lock : self } } # [doc = " Returns a mutable reference to the underlying data."] # [doc = ""] # [doc = " Since this call borrows the `ReentrantLock` mutably, no actual locking"] # [doc = " needs to take place -- the mutable borrow statically guarantees no locks"] # [doc = " exist."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(reentrant_lock)]"] # [doc = " use std::sync::ReentrantLock;"] # [doc = ""] # [doc = " let mut lock = ReentrantLock::new(0);"] # [doc = " *lock.get_mut() = 10;"] # [doc = " assert_eq!(*lock.lock(), 10);"] # [doc = " ```"] pub fn get_mut (& mut self) -> & mut T { & mut self . data } # [doc = " Attempts to acquire this lock."] # [doc = ""] # [doc = " If the lock could not be acquired at this time, then `None` is returned."] # [doc = " Otherwise, an RAII guard is returned."] # [doc = ""] # [doc = " This function does not block."] # [unstable (issue = "none" , feature = "std_internals")] # [doc (hidden)] pub fn try_lock (& self) -> Option < ReentrantLockGuard < '_ , T > > { let this_thread = current_id () ; unsafe { if self . owner . contains (this_thread) { self . increment_lock_count () ? ; Some (ReentrantLockGuard { lock : self }) } else if self . mutex . try_lock () { self . owner . set (Some (this_thread)) ; debug_assert_eq ! (* self . lock_count . get () , 0) ; * self . lock_count . get () = 1 ; Some (ReentrantLockGuard { lock : self }) } else { None } } } # [doc = " Returns a raw pointer to the underlying data."] # [doc = ""] # [doc = " The returned pointer is always non-null and properly aligned, but it is"] # [doc = " the user's responsibility to ensure that any reads through it are"] # [doc = " properly synchronized to avoid data races, and that it is not read"] # [doc = " through after the lock is dropped."] # [unstable (feature = "reentrant_lock_data_ptr" , issue = "140368")] pub fn data_ptr (& self) -> * const T { & raw const self . data } unsafe fn increment_lock_count (& self) -> Option < () > { unsafe { * self . lock_count . get () = (* self . lock_count . get ()) . checked_add (1) ? ; } Some (()) } }}}
mkitem!{mkimpl!{# [unstable (feature = "reentrant_lock" , issue = "121440")] impl < T : fmt :: Debug + ? Sized > fmt :: Debug for ReentrantLock < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_struct ("ReentrantLock") ; match self . try_lock () { Some (v) => d . field ("data" , & & * v) , None => d . field ("data" , & format_args ! ("<locked>")) , } ; d . finish_non_exhaustive () } }}}
mkitem!{mkimpl!{# [unstable (feature = "reentrant_lock" , issue = "121440")] impl < T : Default > Default for ReentrantLock < T > { fn default () -> Self { Self :: new (T :: default ()) } }}}
mkitem!{mkimpl!{# [unstable (feature = "reentrant_lock" , issue = "121440")] impl < T > From < T > for ReentrantLock < T > { fn from (t : T) -> Self { Self :: new (t) } }}}
mkitem!{mkimpl!{# [unstable (feature = "reentrant_lock" , issue = "121440")] impl < T : ? Sized > Deref for ReentrantLockGuard < '_ , T > { type Target = T ; fn deref (& self) -> & T { & self . lock . data } }}}
mkitem!{mkimpl!{# [unstable (feature = "reentrant_lock" , issue = "121440")] impl < T : fmt :: Debug + ? Sized > fmt :: Debug for ReentrantLockGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }}}
mkitem!{mkimpl!{# [unstable (feature = "reentrant_lock" , issue = "121440")] impl < T : fmt :: Display + ? Sized > fmt :: Display for ReentrantLockGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }}}
mkitem!{mkimpl!{# [unstable (feature = "reentrant_lock" , issue = "121440")] impl < T : ? Sized > Drop for ReentrantLockGuard < '_ , T > { # [inline] fn drop (& mut self) { unsafe { * self . lock . lock_count . get () -= 1 ; if * self . lock . lock_count . get () == 0 { self . lock . owner . set (None) ; self . lock . mutex . unlock () ; } } } }}}