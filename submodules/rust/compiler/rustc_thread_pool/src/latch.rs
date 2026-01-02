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
mkuse!{use std :: marker :: PhantomData ;}
mkuse!{use std :: ops :: Deref ;}
mkuse!{use std :: sync :: atomic :: { AtomicUsize , Ordering } ;}
mkuse!{use std :: sync :: { Arc , Condvar , Mutex } ;}
mkuse!{use crate :: job :: JobRef ;}
mkuse!{use crate :: registry :: { Registry , WorkerThread } ;}
mkitem!{mktrait!{#[doc = " We define various kinds of latches, which are all a primitive signaling"] #[doc = " mechanism. A latch starts as false. Eventually someone calls `set()` and"] #[doc = " it becomes true. You can test if it has been set by calling `probe()`."] #[doc = ""] #[doc = " Some kinds of latches, but not all, support a `wait()` operation"] #[doc = " that will wait until the latch is set, blocking efficiently. That"] #[doc = " is not part of the trait since it is not possibly to do with all"] #[doc = " latches."] #[doc = ""] #[doc = " The intention is that `set()` is called once, but `probe()` may be"] #[doc = " called any number of times. Once `probe()` returns true, the memory"] #[doc = " effects that occurred before `set()` become visible."] #[doc = ""] #[doc = " It'd probably be better to refactor the API into two paired types,"] #[doc = " but that's a bit of work, and this is not a public API."] #[doc = ""] #[doc = " ## Memory ordering"] #[doc = ""] #[doc = " Latches need to guarantee two things:"] #[doc = ""] #[doc = " - Once `probe()` returns true, all memory effects from the `set()`"] #[doc = "   are visible (in other words, the set should synchronize-with"] #[doc = "   the probe)."] #[doc = " - Once `set()` occurs, the next `probe()` *will* observe it. This"] #[doc = "   typically requires a seq-cst ordering. See [the \"tickle-then-get-sleepy\" scenario in the sleep"] #[doc = "   README](/src/sleep/README.md#tickle-then-get-sleepy) for details."] pub (super) trait Latch { #[doc = " Set the latch, signalling others."] #[doc = ""] #[doc = " # WARNING"] #[doc = ""] #[doc = " Setting a latch triggers other threads to wake up and (in some"] #[doc = " cases) complete. This may, in turn, cause memory to be"] #[doc = " deallocated and so forth. One must be very careful about this,"] #[doc = " and it's typically better to read all the fields you will need"] #[doc = " to access *before* a latch is set!"] #[doc = ""] #[doc = " This function operates on `*const Self` instead of `&self` to allow it"] #[doc = " to become dangling during this call. The caller must ensure that the"] #[doc = " pointer is valid upon entry, and not invalidated during the call by any"] #[doc = " actions other than `set` itself."] unsafe fn set (this : * const Self) ; }}}
mkitem!{mktrait!{pub (super) trait AsCoreLatch { fn as_core_latch (& self) -> & CoreLatch ; }}}
mkitem!{#[doc = " Latch is not set, owning thread is awake"] const UNSET : usize = 0 ;}
mkitem!{#[doc = " Latch is not set, owning thread is going to sleep on this latch"] #[doc = " (but has not yet fallen asleep)."] const SLEEPY : usize = 1 ;}
mkitem!{#[doc = " Latch is not set, owning thread is asleep on this latch and"] #[doc = " must be awoken."] const SLEEPING : usize = 2 ;}
mkitem!{#[doc = " Latch is set."] const SET : usize = 3 ;}
mkitem!{mkstruct!{#[doc = " Spin latches are the simplest, most efficient kind, but they do"] #[doc = " not support a `wait()` operation. They just have a boolean flag"] #[doc = " that becomes true when `set()` is called."] #[derive (Debug)] pub (super) struct CoreLatch { state : AtomicUsize , }}}
mkitem!{mkimpl!{impl CoreLatch { #[inline] fn new () -> Self { Self { state : AtomicUsize :: new (0) } } #[doc = " Invoked by owning thread as it prepares to sleep. Returns true"] #[doc = " if the owning thread may proceed to fall asleep, false if the"] #[doc = " latch was set in the meantime."] #[inline] pub (super) fn get_sleepy (& self) -> bool { self . state . compare_exchange (UNSET , SLEEPY , Ordering :: SeqCst , Ordering :: Relaxed) . is_ok () } #[doc = " Invoked by owning thread as it falls asleep sleep. Returns"] #[doc = " true if the owning thread should block, or false if the latch"] #[doc = " was set in the meantime."] #[inline] pub (super) fn fall_asleep (& self) -> bool { self . state . compare_exchange (SLEEPY , SLEEPING , Ordering :: SeqCst , Ordering :: Relaxed) . is_ok () } #[doc = " Invoked by owning thread as it falls asleep sleep. Returns"] #[doc = " true if the owning thread should block, or false if the latch"] #[doc = " was set in the meantime."] #[inline] pub (super) fn wake_up (& self) { if ! self . probe () { let _ = self . state . compare_exchange (SLEEPING , UNSET , Ordering :: SeqCst , Ordering :: Relaxed) ; } } #[doc = " Set the latch. If this returns true, the owning thread was sleeping"] #[doc = " and must be awoken."] #[doc = ""] #[doc = " This is private because, typically, setting a latch involves"] #[doc = " doing some wakeups; those are encapsulated in the surrounding"] #[doc = " latch code."] #[inline] unsafe fn set (this : * const Self) -> bool { let old_state = unsafe { (* this) . state . swap (SET , Ordering :: AcqRel) } ; old_state == SLEEPING } #[doc = " Test if this latch has been set."] #[inline] pub (super) fn probe (& self) -> bool { self . state . load (Ordering :: Acquire) == SET } }}}
mkitem!{mkimpl!{impl AsCoreLatch for CoreLatch { #[inline] fn as_core_latch (& self) -> & CoreLatch { self } }}}
mkitem!{mkstruct!{#[doc = " Spin latches are the simplest, most efficient kind, but they do"] #[doc = " not support a `wait()` operation. They just have a boolean flag"] #[doc = " that becomes true when `set()` is called."] pub (super) struct SpinLatch < 'r > { core_latch : CoreLatch , registry : & 'r Arc < Registry > , target_worker_index : usize , cross : bool , }}}
mkitem!{mkimpl!{impl < 'r > SpinLatch < 'r > { #[doc = " Creates a new spin latch that is owned by `thread`. This means"] #[doc = " that `thread` is the only thread that should be blocking on"] #[doc = " this latch -- it also means that when the latch is set, we"] #[doc = " will wake `thread` if it is sleeping."] #[inline] pub (super) fn new (thread : & 'r WorkerThread) -> SpinLatch < 'r > { SpinLatch { core_latch : CoreLatch :: new () , registry : thread . registry () , target_worker_index : thread . index () , cross : false , } } #[doc = " Creates a new spin latch for cross-threadpool blocking. Notably, we"] #[doc = " need to make sure the registry is kept alive after setting, so we can"] #[doc = " safely call the notification."] #[inline] pub (super) fn cross (thread : & 'r WorkerThread) -> SpinLatch < 'r > { SpinLatch { cross : true , .. SpinLatch :: new (thread) } } }}}
mkitem!{mkimpl!{impl < 'r > AsCoreLatch for SpinLatch < 'r > { #[inline] fn as_core_latch (& self) -> & CoreLatch { & self . core_latch } }}}
mkitem!{mkimpl!{impl < 'r > Latch for SpinLatch < 'r > { #[inline] unsafe fn set (this : * const Self) { let cross_registry ; let registry : & Registry = if unsafe { (* this) . cross } { cross_registry = Arc :: clone (unsafe { (* this) . registry }) ; & cross_registry } else { unsafe { (* this) . registry } } ; let target_worker_index = unsafe { (* this) . target_worker_index } ; if unsafe { CoreLatch :: set (& (* this) . core_latch) } { registry . notify_worker_latch_is_set (target_worker_index) ; } } }}}
mkitem!{mkstruct!{#[doc = " A Latch starts as false and eventually becomes true. You can block"] #[doc = " until it becomes true."] #[derive (Debug)] pub (super) struct LockLatch { m : Mutex < bool > , v : Condvar , }}}
mkitem!{mkimpl!{impl LockLatch { #[inline] pub (super) fn new () -> LockLatch { LockLatch { m : Mutex :: new (false) , v : Condvar :: new () } } #[doc = " Block until latch is set, then resets this lock latch so it can be reused again."] pub (super) fn wait_and_reset (& self) { let mut guard = self . m . lock () . unwrap () ; while ! * guard { guard = self . v . wait (guard) . unwrap () ; } * guard = false ; } #[doc = " Block until latch is set."] pub (super) fn wait (& self) { let mut guard = self . m . lock () . unwrap () ; while ! * guard { guard = self . v . wait (guard) . unwrap () ; } } }}}
mkitem!{mkimpl!{impl Latch for LockLatch { #[inline] unsafe fn set (this : * const Self) { let mut guard = unsafe { (* this) . m . lock () . unwrap () } ; * guard = true ; unsafe { (* this) . v . notify_all () } ; } }}}
mkitem!{mkstruct!{#[doc = " Once latches are used to implement one-time blocking, primarily"] #[doc = " for the termination flag of the threads in the pool."] #[doc = ""] #[doc = " Note: like a `SpinLatch`, once-latches are always associated with"] #[doc = " some registry that is probing them, which must be tickled when"] #[doc = " they are set. *Unlike* a `SpinLatch`, they don't themselves hold a"] #[doc = " reference to that registry. This is because in some cases the"] #[doc = " registry owns the once-latch, and that would create a cycle. So a"] #[doc = " `OnceLatch` must be given a reference to its owning registry when"] #[doc = " it is set. For this reason, it does not implement the `Latch`"] #[doc = " trait (but it doesn't have to, as it is not used in those generic"] #[doc = " contexts)."] #[derive (Debug)] pub (super) struct OnceLatch { core_latch : CoreLatch , }}}
mkitem!{mkimpl!{impl OnceLatch { #[inline] pub (super) fn new () -> OnceLatch { Self { core_latch : CoreLatch :: new () } } #[doc = " Set the latch, then tickle the specific worker thread,"] #[doc = " which should be the one that owns this latch."] #[inline] pub (super) unsafe fn set_and_tickle_one (this : * const Self , registry : & Registry , target_worker_index : usize ,) { if unsafe { CoreLatch :: set (& (* this) . core_latch) } { registry . notify_worker_latch_is_set (target_worker_index) ; } } }}}
mkitem!{mkimpl!{impl AsCoreLatch for OnceLatch { #[inline] fn as_core_latch (& self) -> & CoreLatch { & self . core_latch } }}}
mkitem!{mkstruct!{#[doc = " Counting latches are used to implement scopes. They track a"] #[doc = " counter. Unlike other latches, calling `set()` does not"] #[doc = " necessarily make the latch be considered `set()`; instead, it just"] #[doc = " decrements the counter. The latch is only \"set\" (in the sense that"] #[doc = " `probe()` returns true) once the counter reaches zero."] #[derive (Debug)] pub (super) struct CountLatch { counter : AtomicUsize , kind : CountLatchKind , }}}
mkitem!{mkenum!{enum CountLatchKind { #[doc = " A latch for scopes created on a rayon thread which will participate in work-"] #[doc = " stealing while it waits for completion. This thread is not necessarily part"] #[doc = " of the same registry as the scope itself!"] Stealing { latch : CoreLatch , #[doc = " If a worker thread in registry A calls `in_place_scope` on a ThreadPool"] #[doc = " with registry B, when a job completes in a thread of registry B, we may"] #[doc = " need to call `notify_worker_latch_is_set()` to wake the thread in registry A."] #[doc = " That means we need a reference to registry A (since at that point we will"] #[doc = " only have a reference to registry B), so we stash it here."] registry : Arc < Registry > , #[doc = " The index of the worker to wake in `registry`"] worker_index : usize , } , #[doc = " A latch for scopes created on a non-rayon thread which will block to wait."] Blocking { latch : LockLatch } , }}}
mkitem!{mkimpl!{impl std :: fmt :: Debug for CountLatchKind { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { CountLatchKind :: Stealing { latch , .. } => { f . debug_tuple ("Stealing") . field (latch) . finish () } CountLatchKind :: Blocking { latch , .. } => { f . debug_tuple ("Blocking") . field (latch) . finish () } } } }}}
mkitem!{mkimpl!{impl CountLatch { pub (super) fn new (owner : Option < & WorkerThread >) -> Self { Self :: with_count (1 , owner) } pub (super) fn with_count (count : usize , owner : Option < & WorkerThread >) -> Self { Self { counter : AtomicUsize :: new (count) , kind : match owner { Some (owner) => CountLatchKind :: Stealing { latch : CoreLatch :: new () , registry : Arc :: clone (owner . registry ()) , worker_index : owner . index () , } , None => CountLatchKind :: Blocking { latch : LockLatch :: new () } , } , } } #[inline] pub (super) fn increment (& self) { let old_counter = self . counter . fetch_add (1 , Ordering :: Relaxed) ; debug_assert ! (old_counter != 0) ; } pub (super) fn wait (& self , owner : Option < & WorkerThread > , all_jobs_started : impl FnMut () -> bool , is_job : impl FnMut (& JobRef) -> bool ,) { match & self . kind { CountLatchKind :: Stealing { latch , registry , worker_index } => unsafe { let owner = owner . expect ("owner thread") ; debug_assert_eq ! (registry . id () , owner . registry () . id ()) ; debug_assert_eq ! (* worker_index , owner . index ()) ; owner . wait_for_jobs :: < _ , true > (latch , all_jobs_started , is_job , | job | { owner . execute (job) ; }) ; } , CountLatchKind :: Blocking { latch } => latch . wait () , } } }}}
mkitem!{mkimpl!{impl Latch for CountLatch { #[inline] unsafe fn set (this : * const Self) { if unsafe { (* this) . counter . fetch_sub (1 , Ordering :: SeqCst) == 1 } { match unsafe { & (* this) . kind } { CountLatchKind :: Stealing { latch , registry , worker_index } => { let registry = Arc :: clone (registry) ; if unsafe { CoreLatch :: set (latch) } { registry . notify_worker_latch_is_set (* worker_index) ; } } CountLatchKind :: Blocking { latch } => unsafe { LockLatch :: set (latch) } , } } } }}}
mkitem!{mkstruct!{#[doc = " `&L` without any implication of `dereferenceable` for `Latch::set`"] pub (super) struct LatchRef < 'a , L > { inner : * const L , marker : PhantomData < & 'a L > , }}}
mkitem!{mkimpl!{impl < L > LatchRef < '_ , L > { pub (super) fn new (inner : & L) -> LatchRef < '_ , L > { LatchRef { inner , marker : PhantomData } } }}}
mkitem!{mkimpl!{unsafe impl < L : Sync > Sync for LatchRef < '_ , L > { }}}
mkitem!{mkimpl!{impl < L > Deref for LatchRef < '_ , L > { type Target = L ; fn deref (& self) -> & L { unsafe { & * self . inner } } }}}
mkitem!{mkimpl!{impl < L : Latch > Latch for LatchRef < '_ , L > { #[inline] unsafe fn set (this : * const Self) { unsafe { L :: set ((* this) . inner) } ; } }}}