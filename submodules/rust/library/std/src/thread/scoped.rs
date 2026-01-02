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
mkuse!{use super :: { Builder , JoinInner , Result , Thread , current_or_unnamed } ;}
mkuse!{use crate :: marker :: PhantomData ;}
mkuse!{use crate :: panic :: { AssertUnwindSafe , catch_unwind , resume_unwind } ;}
mkuse!{use crate :: sync :: Arc ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicBool , AtomicUsize , Ordering } ;}
mkuse!{use crate :: { fmt , io } ;}
mkitem!{mkstruct!{# [doc = " A scope to spawn scoped threads in."] # [doc = ""] # [doc = " See [`scope`] for details."] # [stable (feature = "scoped_threads" , since = "1.63.0")] pub struct Scope < 'scope , 'env : 'scope > { data : Arc < ScopeData > , # [doc = " Invariance over 'scope, to make sure 'scope cannot shrink,"] # [doc = " which is necessary for soundness."] # [doc = ""] # [doc = " Without invariance, this would compile fine but be unsound:"] # [doc = ""] # [doc = " ```compile_fail,E0373"] # [doc = " std::thread::scope(|s| {"] # [doc = "     s.spawn(|| {"] # [doc = "         let a = String::from(\"abcd\");"] # [doc = "         s.spawn(|| println!(\"{a:?}\")); // might run after `a` is dropped"] # [doc = "     });"] # [doc = " });"] # [doc = " ```"] scope : PhantomData < & 'scope mut & 'scope () > , env : PhantomData < & 'env mut & 'env () > , }}}
mkitem!{mkstruct!{# [doc = " An owned permission to join on a scoped thread (block on its termination)."] # [doc = ""] # [doc = " See [`Scope::spawn`] for details."] # [stable (feature = "scoped_threads" , since = "1.63.0")] pub struct ScopedJoinHandle < 'scope , T > (JoinInner < 'scope , T >) ;}}
mkitem!{mkstruct!{pub (super) struct ScopeData { num_running_threads : Atomic < usize > , a_thread_panicked : Atomic < bool > , main_thread : Thread , }}}
mkitem!{mkimpl!{impl ScopeData { pub (super) fn increment_num_running_threads (& self) { if self . num_running_threads . fetch_add (1 , Ordering :: Relaxed) > usize :: MAX / 2 { self . overflow () ; } } # [cold] fn overflow (& self) { self . decrement_num_running_threads (false) ; panic ! ("too many running threads in thread scope") ; } pub (super) fn decrement_num_running_threads (& self , panic : bool) { if panic { self . a_thread_panicked . store (true , Ordering :: Relaxed) ; } if self . num_running_threads . fetch_sub (1 , Ordering :: Release) == 1 { self . main_thread . unpark () ; } } }}}

macro_rules! scope_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function scope in module {}", module_path!());
    };
}

mkfn!{
    scope_introspect!();
    # [doc = " Creates a scope for spawning scoped threads."] # [doc = ""] # [doc = " The function passed to `scope` will be provided a [`Scope`] object,"] # [doc = " through which scoped threads can be [spawned][`Scope::spawn`]."] # [doc = ""] # [doc = " Unlike non-scoped threads, scoped threads can borrow non-`'static` data,"] # [doc = " as the scope guarantees all threads will be joined at the end of the scope."] # [doc = ""] # [doc = " All threads spawned within the scope that haven't been manually joined"] # [doc = " will be automatically joined before this function returns."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If any of the automatically joined threads panicked, this function will panic."] # [doc = ""] # [doc = " If you want to handle panics from spawned threads,"] # [doc = " [`join`][ScopedJoinHandle::join] them before the end of the scope."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let mut a = vec![1, 2, 3];"] # [doc = " let mut x = 0;"] # [doc = ""] # [doc = " thread::scope(|s| {"] # [doc = "     s.spawn(|| {"] # [doc = "         println!(\"hello from the first scoped thread\");"] # [doc = "         // We can borrow `a` here."] # [doc = "         dbg!(&a);"] # [doc = "     });"] # [doc = "     s.spawn(|| {"] # [doc = "         println!(\"hello from the second scoped thread\");"] # [doc = "         // We can even mutably borrow `x` here,"] # [doc = "         // because no other threads are using it."] # [doc = "         x += a[0] + a[2];"] # [doc = "     });"] # [doc = "     println!(\"hello from the main thread\");"] # [doc = " });"] # [doc = ""] # [doc = " // After the scope, we can modify and access our variables again:"] # [doc = " a.push(4);"] # [doc = " assert_eq!(x, a.len());"] # [doc = " ```"] # [doc = ""] # [doc = " # Lifetimes"] # [doc = ""] # [doc = " Scoped threads involve two lifetimes: `'scope` and `'env`."] # [doc = ""] # [doc = " The `'scope` lifetime represents the lifetime of the scope itself."] # [doc = " That is: the time during which new scoped threads may be spawned,"] # [doc = " and also the time during which they might still be running."] # [doc = " Once this lifetime ends, all scoped threads are joined."] # [doc = " This lifetime starts within the `scope` function, before `f` (the argument to `scope`) starts."] # [doc = " It ends after `f` returns and all scoped threads have been joined, but before `scope` returns."] # [doc = ""] # [doc = " The `'env` lifetime represents the lifetime of whatever is borrowed by the scoped threads."] # [doc = " This lifetime must outlast the call to `scope`, and thus cannot be smaller than `'scope`."] # [doc = " It can be as small as the call to `scope`, meaning that anything that outlives this call,"] # [doc = " such as local variables defined right before the scope, can be borrowed by the scoped threads."] # [doc = ""] # [doc = " The `'env: 'scope` bound is part of the definition of the `Scope` type."] # [track_caller] # [stable (feature = "scoped_threads" , since = "1.63.0")] pub fn scope < 'env , F , T > (f : F) -> T where F : for < 'scope > FnOnce (& 'scope Scope < 'scope , 'env >) -> T , { let scope = Scope { data : Arc :: new (ScopeData { num_running_threads : AtomicUsize :: new (0) , main_thread : current_or_unnamed () , a_thread_panicked : AtomicBool :: new (false) , }) , env : PhantomData , scope : PhantomData , } ; let result = catch_unwind (AssertUnwindSafe (| | f (& scope))) ; while scope . data . num_running_threads . load (Ordering :: Acquire) != 0 { unsafe { scope . data . main_thread . park () } ; } match result { Err (e) => resume_unwind (e) , Ok (_) if scope . data . a_thread_panicked . load (Ordering :: Relaxed) => { panic ! ("a scoped thread panicked") } Ok (result) => result , } }
}
mkitem!{mkimpl!{impl < 'scope , 'env > Scope < 'scope , 'env > { # [doc = " Spawns a new thread within a scope, returning a [`ScopedJoinHandle`] for it."] # [doc = ""] # [doc = " Unlike non-scoped threads, threads spawned with this function may"] # [doc = " borrow non-`'static` data from the outside the scope. See [`scope`] for"] # [doc = " details."] # [doc = ""] # [doc = " The join handle provides a [`join`] method that can be used to join the spawned"] # [doc = " thread. If the spawned thread panics, [`join`] will return an [`Err`] containing"] # [doc = " the panic payload."] # [doc = ""] # [doc = " If the join handle is dropped, the spawned thread will be implicitly joined at the"] # [doc = " end of the scope. In that case, if the spawned thread panics, [`scope`] will"] # [doc = " panic after all threads are joined."] # [doc = ""] # [doc = " This call will create a thread using default parameters of [`Builder`]."] # [doc = " If you want to specify the stack size or the name of the thread, use"] # [doc = " [`Builder::spawn_scoped`] instead."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the OS fails to create a thread; use [`Builder::spawn_scoped`]"] # [doc = " to recover from such errors."] # [doc = ""] # [doc = " [`join`]: ScopedJoinHandle::join"] # [stable (feature = "scoped_threads" , since = "1.63.0")] pub fn spawn < F , T > (& 'scope self , f : F) -> ScopedJoinHandle < 'scope , T > where F : FnOnce () -> T + Send + 'scope , T : Send + 'scope , { Builder :: new () . spawn_scoped (self , f) . expect ("failed to spawn thread") } }}}
mkitem!{mkimpl!{impl Builder { # [doc = " Spawns a new scoped thread using the settings set through this `Builder`."] # [doc = ""] # [doc = " Unlike [`Scope::spawn`], this method yields an [`io::Result`] to"] # [doc = " capture any failure to create the thread at the OS level."] # [doc = ""] # [doc = " [`io::Result`]: crate::io::Result"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if a thread name was set and it contained null bytes."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let mut a = vec![1, 2, 3];"] # [doc = " let mut x = 0;"] # [doc = ""] # [doc = " thread::scope(|s| {"] # [doc = "     thread::Builder::new()"] # [doc = "         .name(\"first\".to_string())"] # [doc = "         .spawn_scoped(s, ||"] # [doc = "     {"] # [doc = "         println!(\"hello from the {:?} scoped thread\", thread::current().name());"] # [doc = "         // We can borrow `a` here."] # [doc = "         dbg!(&a);"] # [doc = "     })"] # [doc = "     .unwrap();"] # [doc = "     thread::Builder::new()"] # [doc = "         .name(\"second\".to_string())"] # [doc = "         .spawn_scoped(s, ||"] # [doc = "     {"] # [doc = "         println!(\"hello from the {:?} scoped thread\", thread::current().name());"] # [doc = "         // We can even mutably borrow `x` here,"] # [doc = "         // because no other threads are using it."] # [doc = "         x += a[0] + a[2];"] # [doc = "     })"] # [doc = "     .unwrap();"] # [doc = "     println!(\"hello from the main thread\");"] # [doc = " });"] # [doc = ""] # [doc = " // After the scope, we can modify and access our variables again:"] # [doc = " a.push(4);"] # [doc = " assert_eq!(x, a.len());"] # [doc = " ```"] # [stable (feature = "scoped_threads" , since = "1.63.0")] pub fn spawn_scoped < 'scope , 'env , F , T > (self , scope : & 'scope Scope < 'scope , 'env > , f : F ,) -> io :: Result < ScopedJoinHandle < 'scope , T > > where F : FnOnce () -> T + Send + 'scope , T : Send + 'scope , { Ok (ScopedJoinHandle (unsafe { self . spawn_unchecked_ (f , Some (scope . data . clone ())) } ?)) } }}}
mkitem!{mkimpl!{impl < 'scope , T > ScopedJoinHandle < 'scope , T > { # [doc = " Extracts a handle to the underlying thread."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " thread::scope(|s| {"] # [doc = "     let t = s.spawn(|| {"] # [doc = "         println!(\"hello\");"] # [doc = "     });"] # [doc = "     println!(\"thread id: {:?}\", t.thread().id());"] # [doc = " });"] # [doc = " ```"] # [must_use] # [stable (feature = "scoped_threads" , since = "1.63.0")] pub fn thread (& self) -> & Thread { & self . 0 . thread } # [doc = " Waits for the associated thread to finish."] # [doc = ""] # [doc = " This function will return immediately if the associated thread has already finished."] # [doc = ""] # [doc = " In terms of [atomic memory orderings], the completion of the associated"] # [doc = " thread synchronizes with this function returning."] # [doc = " In other words, all operations performed by that thread"] # [doc = " [happen before](https://doc.rust-lang.org/nomicon/atomics.html#data-accesses)"] # [doc = " all operations that happen after `join` returns."] # [doc = ""] # [doc = " If the associated thread panics, [`Err`] is returned with the panic payload."] # [doc = ""] # [doc = " [atomic memory orderings]: crate::sync::atomic"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " thread::scope(|s| {"] # [doc = "     let t = s.spawn(|| {"] # [doc = "         panic!(\"oh no\");"] # [doc = "     });"] # [doc = "     assert!(t.join().is_err());"] # [doc = " });"] # [doc = " ```"] # [stable (feature = "scoped_threads" , since = "1.63.0")] pub fn join (self) -> Result < T > { self . 0 . join () } # [doc = " Checks if the associated thread has finished running its main function."] # [doc = ""] # [doc = " `is_finished` supports implementing a non-blocking join operation, by checking"] # [doc = " `is_finished`, and calling `join` if it returns `true`. This function does not block. To"] # [doc = " block while waiting on the thread to finish, use [`join`][Self::join]."] # [doc = ""] # [doc = " This might return `true` for a brief moment after the thread's main"] # [doc = " function has returned, but before the thread itself has stopped running."] # [doc = " However, once this returns `true`, [`join`][Self::join] can be expected"] # [doc = " to return quickly, without blocking for any significant amount of time."] # [stable (feature = "scoped_threads" , since = "1.63.0")] pub fn is_finished (& self) -> bool { Arc :: strong_count (& self . 0 . packet) == 1 } }}}
mkitem!{mkimpl!{# [stable (feature = "scoped_threads" , since = "1.63.0")] impl fmt :: Debug for Scope < '_ , '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Scope") . field ("num_running_threads" , & self . data . num_running_threads . load (Ordering :: Relaxed)) . field ("a_thread_panicked" , & self . data . a_thread_panicked . load (Ordering :: Relaxed)) . field ("main_thread" , & self . data . main_thread) . finish_non_exhaustive () } }}}
mkitem!{mkimpl!{# [stable (feature = "scoped_threads" , since = "1.63.0")] impl < 'scope , T > fmt :: Debug for ScopedJoinHandle < 'scope , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ScopedJoinHandle") . finish_non_exhaustive () } }}}