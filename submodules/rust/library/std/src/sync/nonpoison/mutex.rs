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
mkuse!{use crate :: marker :: PhantomData ;}
mkuse!{use crate :: mem :: { self , ManuallyDrop } ;}
mkuse!{use crate :: ops :: { Deref , DerefMut } ;}
mkuse!{use crate :: ptr :: NonNull ;}
mkuse!{use crate :: sync :: nonpoison :: { TryLockResult , WouldBlock } ;}
mkuse!{use crate :: sys :: sync as sys ;}
mkitem!{mkstruct!{# [doc = " A mutual exclusion primitive useful for protecting shared data that does not keep track of"] # [doc = " lock poisoning."] # [doc = ""] # [doc = " For more information about mutexes, check out the documentation for the poisoning variant of"] # [doc = " this lock at [`poison::Mutex`]."] # [doc = ""] # [doc = " [`poison::Mutex`]: crate::sync::poison::Mutex"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Note that this `Mutex` does **not** propagate threads that panic while holding the lock via"] # [doc = " poisoning. If you need this functionality, see [`poison::Mutex`]."] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_mutex)]"] # [doc = ""] # [doc = " use std::thread;"] # [doc = " use std::sync::{Arc, nonpoison::Mutex};"] # [doc = ""] # [doc = " let mutex = Arc::new(Mutex::new(0u32));"] # [doc = " let mut handles = Vec::new();"] # [doc = ""] # [doc = " for n in 0..10 {"] # [doc = "     let m = Arc::clone(&mutex);"] # [doc = "     let handle = thread::spawn(move || {"] # [doc = "         let mut guard = m.lock();"] # [doc = "         *guard += 1;"] # [doc = "         panic!(\"panic from thread {n} {guard}\")"] # [doc = "     });"] # [doc = "     handles.push(handle);"] # [doc = " }"] # [doc = ""] # [doc = " for h in handles {"] # [doc = "     let _ = h.join();"] # [doc = " }"] # [doc = ""] # [doc = " println!(\"Finished, locked {} times\", mutex.lock());"] # [doc = " ```"] # [unstable (feature = "nonpoison_mutex" , issue = "134645")] # [cfg_attr (not (test) , rustc_diagnostic_item = "NonPoisonMutex")] pub struct Mutex < T : ? Sized > { inner : sys :: Mutex , data : UnsafeCell < T > , }}}
mkitem!{mkimpl!{# [doc = " `T` must be `Send` for a [`Mutex`] to be `Send` because it is possible to acquire"] # [doc = " the owned `T` from the `Mutex` via [`into_inner`]."] # [doc = ""] # [doc = " [`into_inner`]: Mutex::into_inner"] # [unstable (feature = "nonpoison_mutex" , issue = "134645")] unsafe impl < T : ? Sized + Send > Send for Mutex < T > { }}}
mkitem!{mkimpl!{# [doc = " `T` must be `Send` for [`Mutex`] to be `Sync`."] # [doc = " This ensures that the protected data can be accessed safely from multiple threads"] # [doc = " without causing data races or other unsafe behavior."] # [doc = ""] # [doc = " [`Mutex<T>`] provides mutable access to `T` to one thread at a time. However, it's essential"] # [doc = " for `T` to be `Send` because it's not safe for non-`Send` structures to be accessed in"] # [doc = " this manner. For instance, consider [`Rc`], a non-atomic reference counted smart pointer,"] # [doc = " which is not `Send`. With `Rc`, we can have multiple copies pointing to the same heap"] # [doc = " allocation with a non-atomic reference count. If we were to use `Mutex<Rc<_>>`, it would"] # [doc = " only protect one instance of `Rc` from shared access, leaving other copies vulnerable"] # [doc = " to potential data races."] # [doc = ""] # [doc = " Also note that it is not necessary for `T` to be `Sync` as `&T` is only made available"] # [doc = " to one thread at a time if `T` is not `Sync`."] # [doc = ""] # [doc = " [`Rc`]: crate::rc::Rc"] # [unstable (feature = "nonpoison_mutex" , issue = "134645")] unsafe impl < T : ? Sized + Send > Sync for Mutex < T > { }}}
mkitem!{mkstruct!{# [doc = " An RAII implementation of a \"scoped lock\" of a mutex. When this structure is"] # [doc = " dropped (falls out of scope), the lock will be unlocked."] # [doc = ""] # [doc = " The data protected by the mutex can be accessed through this guard via its"] # [doc = " [`Deref`] and [`DerefMut`] implementations."] # [doc = ""] # [doc = " This structure is created by the [`lock`] and [`try_lock`] methods on"] # [doc = " [`Mutex`]."] # [doc = ""] # [doc = " [`lock`]: Mutex::lock"] # [doc = " [`try_lock`]: Mutex::try_lock"] # [must_use = "if unused the Mutex will immediately unlock"] # [must_not_suspend = "holding a MutexGuard across suspend \
                      points can cause deadlocks, delays, \
                      and cause Futures to not implement `Send`"] # [unstable (feature = "nonpoison_mutex" , issue = "134645")] # [clippy :: has_significant_drop] # [cfg_attr (not (test) , rustc_diagnostic_item = "NonPoisonMutexGuard")] pub struct MutexGuard < 'a , T : ? Sized + 'a > { lock : & 'a Mutex < T > , }}}
mkitem!{mkimpl!{# [doc = " A [`MutexGuard`] is not `Send` to maximize platform portability."] # [doc = ""] # [doc = " On platforms that use POSIX threads (commonly referred to as pthreads) there is a requirement to"] # [doc = " release mutex locks on the same thread they were acquired."] # [doc = " For this reason, [`MutexGuard`] must not implement `Send` to prevent it being dropped from"] # [doc = " another thread."] # [unstable (feature = "nonpoison_mutex" , issue = "134645")] impl < T : ? Sized > ! Send for MutexGuard < '_ , T > { }}}
mkitem!{mkimpl!{# [doc = " `T` must be `Sync` for a [`MutexGuard<T>`] to be `Sync`"] # [doc = " because it is possible to get a `&T` from `&MutexGuard` (via `Deref`)."] # [unstable (feature = "nonpoison_mutex" , issue = "134645")] unsafe impl < T : ? Sized + Sync > Sync for MutexGuard < '_ , T > { }}}
mkitem!{mkstruct!{# [doc = " An RAII mutex guard returned by `MutexGuard::map`, which can point to a"] # [doc = " subfield of the protected data. When this structure is dropped (falls out"] # [doc = " of scope), the lock will be unlocked."] # [doc = ""] # [doc = " The main difference between `MappedMutexGuard` and [`MutexGuard`] is that the"] # [doc = " former cannot be used with [`Condvar`], since that could introduce soundness issues if the"] # [doc = " locked object is modified by another thread while the `Mutex` is unlocked."] # [doc = ""] # [doc = " The data protected by the mutex can be accessed through this guard via its"] # [doc = " [`Deref`] and [`DerefMut`] implementations."] # [doc = ""] # [doc = " This structure is created by the [`map`] and [`filter_map`] methods on"] # [doc = " [`MutexGuard`]."] # [doc = ""] # [doc = " [`map`]: MutexGuard::map"] # [doc = " [`filter_map`]: MutexGuard::filter_map"] # [doc = " [`Condvar`]: crate::sync::nonpoison::Condvar"] # [must_use = "if unused the Mutex will immediately unlock"] # [must_not_suspend = "holding a MappedMutexGuard across suspend \
                      points can cause deadlocks, delays, \
                      and cause Futures to not implement `Send`"] # [unstable (feature = "mapped_lock_guards" , issue = "117108")] # [clippy :: has_significant_drop] pub struct MappedMutexGuard < 'a , T : ? Sized + 'a > { data : NonNull < T > , inner : & 'a sys :: Mutex , _variance : PhantomData < & 'a mut T > , }}}
mkitem!{mkimpl!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized > ! Send for MappedMutexGuard < '_ , T > { }}}
mkitem!{mkimpl!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] unsafe impl < T : ? Sized + Sync > Sync for MappedMutexGuard < '_ , T > { }}}
mkitem!{mkimpl!{impl < T > Mutex < T > { # [doc = " Creates a new mutex in an unlocked state ready for use."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_mutex)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::Mutex;"] # [doc = ""] # [doc = " let mutex = Mutex::new(0);"] # [doc = " ```"] # [unstable (feature = "nonpoison_mutex" , issue = "134645")] # [inline] pub const fn new (t : T) -> Mutex < T > { Mutex { inner : sys :: Mutex :: new () , data : UnsafeCell :: new (t) } } # [doc = " Returns the contained value by cloning it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_mutex)]"] # [doc = " #![feature(lock_value_accessors)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::Mutex;"] # [doc = ""] # [doc = " let mut mutex = Mutex::new(7);"] # [doc = ""] # [doc = " assert_eq!(mutex.get_cloned(), 7);"] # [doc = " ```"] # [unstable (feature = "lock_value_accessors" , issue = "133407")] pub fn get_cloned (& self) -> T where T : Clone , { self . lock () . clone () } # [doc = " Sets the contained value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_mutex)]"] # [doc = " #![feature(lock_value_accessors)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::Mutex;"] # [doc = ""] # [doc = " let mut mutex = Mutex::new(7);"] # [doc = ""] # [doc = " assert_eq!(mutex.get_cloned(), 7);"] # [doc = " mutex.set(11);"] # [doc = " assert_eq!(mutex.get_cloned(), 11);"] # [doc = " ```"] # [unstable (feature = "lock_value_accessors" , issue = "133407")] pub fn set (& self , value : T) { if mem :: needs_drop :: < T > () { drop (self . replace (value)) } else { * self . lock () = value ; } } # [doc = " Replaces the contained value with `value`, and returns the old contained value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_mutex)]"] # [doc = " #![feature(lock_value_accessors)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::Mutex;"] # [doc = ""] # [doc = " let mut mutex = Mutex::new(7);"] # [doc = ""] # [doc = " assert_eq!(mutex.replace(11), 7);"] # [doc = " assert_eq!(mutex.get_cloned(), 11);"] # [doc = " ```"] # [unstable (feature = "lock_value_accessors" , issue = "133407")] pub fn replace (& self , value : T) -> T { let mut guard = self . lock () ; mem :: replace (& mut * guard , value) } }}}
mkitem!{mkimpl!{impl < T : ? Sized > Mutex < T > { # [doc = " Acquires a mutex, blocking the current thread until it is able to do so."] # [doc = ""] # [doc = " This function will block the local thread until it is available to acquire"] # [doc = " the mutex. Upon returning, the thread is the only thread with the lock"] # [doc = " held. An RAII guard is returned to allow scoped unlock of the lock. When"] # [doc = " the guard goes out of scope, the mutex will be unlocked."] # [doc = ""] # [doc = " The exact behavior on locking a mutex in the thread which already holds"] # [doc = " the lock is left unspecified. However, this function will not return on"] # [doc = " the second call (it might panic or deadlock, for example)."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function might panic when called if the lock is already held by"] # [doc = " the current thread."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_mutex)]"] # [doc = ""] # [doc = " use std::sync::{Arc, nonpoison::Mutex};"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let mutex = Arc::new(Mutex::new(0));"] # [doc = " let c_mutex = Arc::clone(&mutex);"] # [doc = ""] # [doc = " thread::spawn(move || {"] # [doc = "     *c_mutex.lock() = 10;"] # [doc = " }).join().expect(\"thread::spawn failed\");"] # [doc = " assert_eq!(*mutex.lock(), 10);"] # [doc = " ```"] # [unstable (feature = "nonpoison_mutex" , issue = "134645")] pub fn lock (& self) -> MutexGuard < '_ , T > { unsafe { self . inner . lock () ; MutexGuard :: new (self) } } # [doc = " Attempts to acquire this lock."] # [doc = ""] # [doc = " This function does not block. If the lock could not be acquired at this time, then"] # [doc = " [`WouldBlock`] is returned. Otherwise, an RAII guard is returned."] # [doc = ""] # [doc = " The lock will be unlocked when the guard is dropped."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " If the mutex could not be acquired because it is already locked, then this call will return"] # [doc = " the [`WouldBlock`] error."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::{Arc, Mutex};"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let mutex = Arc::new(Mutex::new(0));"] # [doc = " let c_mutex = Arc::clone(&mutex);"] # [doc = ""] # [doc = " thread::spawn(move || {"] # [doc = "     let mut lock = c_mutex.try_lock();"] # [doc = "     if let Ok(ref mut mutex) = lock {"] # [doc = "         **mutex = 10;"] # [doc = "     } else {"] # [doc = "         println!(\"try_lock failed\");"] # [doc = "     }"] # [doc = " }).join().expect(\"thread::spawn failed\");"] # [doc = " assert_eq!(*mutex.lock().unwrap(), 10);"] # [doc = " ```"] # [unstable (feature = "nonpoison_mutex" , issue = "134645")] pub fn try_lock (& self) -> TryLockResult < MutexGuard < '_ , T > > { unsafe { if self . inner . try_lock () { Ok (MutexGuard :: new (self)) } else { Err (WouldBlock) } } } # [doc = " Consumes this mutex, returning the underlying data."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_mutex)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::Mutex;"] # [doc = ""] # [doc = " let mutex = Mutex::new(0);"] # [doc = " assert_eq!(mutex.into_inner(), 0);"] # [doc = " ```"] # [unstable (feature = "nonpoison_mutex" , issue = "134645")] pub fn into_inner (self) -> T where T : Sized , { self . data . into_inner () } # [doc = " Returns a mutable reference to the underlying data."] # [doc = ""] # [doc = " Since this call borrows the `Mutex` mutably, no actual locking needs to"] # [doc = " take place -- the mutable borrow statically guarantees no locks exist."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_mutex)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::Mutex;"] # [doc = ""] # [doc = " let mut mutex = Mutex::new(0);"] # [doc = " *mutex.get_mut() = 10;"] # [doc = " assert_eq!(*mutex.lock(), 10);"] # [doc = " ```"] # [unstable (feature = "nonpoison_mutex" , issue = "134645")] pub fn get_mut (& mut self) -> & mut T { self . data . get_mut () } # [doc = " Returns a raw pointer to the underlying data."] # [doc = ""] # [doc = " The returned pointer is always non-null and properly aligned, but it is"] # [doc = " the user's responsibility to ensure that any reads and writes through it"] # [doc = " are properly synchronized to avoid data races, and that it is not read"] # [doc = " or written through after the mutex is dropped."] # [unstable (feature = "mutex_data_ptr" , issue = "140368")] pub fn data_ptr (& self) -> * mut T { self . data . get () } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_mutex" , issue = "134645")] impl < T > From < T > for Mutex < T > { # [doc = " Creates a new mutex in an unlocked state ready for use."] # [doc = " This is equivalent to [`Mutex::new`]."] fn from (t : T) -> Self { Mutex :: new (t) } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_mutex" , issue = "134645")] impl < T : ? Sized + Default > Default for Mutex < T > { # [doc = " Creates a `Mutex<T>`, with the `Default` value for T."] fn default () -> Mutex < T > { Mutex :: new (Default :: default ()) } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_mutex" , issue = "134645")] impl < T : ? Sized + fmt :: Debug > fmt :: Debug for Mutex < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_struct ("Mutex") ; match self . try_lock () { Ok (guard) => { d . field ("data" , & & * guard) ; } Err (WouldBlock) => { d . field ("data" , & "<locked>") ; } } d . finish_non_exhaustive () } }}}
mkitem!{mkimpl!{impl < 'mutex , T : ? Sized > MutexGuard < 'mutex , T > { unsafe fn new (lock : & 'mutex Mutex < T >) -> MutexGuard < 'mutex , T > { return MutexGuard { lock } ; } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_mutex" , issue = "134645")] impl < T : ? Sized > Deref for MutexGuard < '_ , T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * self . lock . data . get () } } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_mutex" , issue = "134645")] impl < T : ? Sized > DerefMut for MutexGuard < '_ , T > { fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . lock . data . get () } } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_mutex" , issue = "134645")] impl < T : ? Sized > Drop for MutexGuard < '_ , T > { # [inline] fn drop (& mut self) { unsafe { self . lock . inner . unlock () ; } } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_mutex" , issue = "134645")] impl < T : ? Sized + fmt :: Debug > fmt :: Debug for MutexGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_mutex" , issue = "134645")] impl < T : ? Sized + fmt :: Display > fmt :: Display for MutexGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }}}

macro_rules! guard_lock_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function guard_lock in module {}", module_path!());
    };
}

mkfn!{
    guard_lock_introspect!();
    # [doc = " For use in [`nonpoison::condvar`](super::condvar)."] pub (super) fn guard_lock < 'a , T : ? Sized > (guard : & MutexGuard < 'a , T >) -> & 'a sys :: Mutex { & guard . lock . inner }
}
mkitem!{mkimpl!{impl < 'a , T : ? Sized > MutexGuard < 'a , T > { # [doc = " Makes a [`MappedMutexGuard`] for a component of the borrowed data, e.g."] # [doc = " an enum variant."] # [doc = ""] # [doc = " The `Mutex` is already locked, so this cannot fail."] # [doc = ""] # [doc = " This is an associated function that needs to be used as"] # [doc = " `MutexGuard::map(...)`. A method would interfere with methods of the"] # [doc = " same name on the contents of the `MutexGuard` used through `Deref`."] # [unstable (feature = "mapped_lock_guards" , issue = "117108")] pub fn map < U , F > (orig : Self , f : F) -> MappedMutexGuard < 'a , U > where F : FnOnce (& mut T) -> & mut U , U : ? Sized , { let data = NonNull :: from (f (unsafe { & mut * orig . lock . data . get () })) ; let orig = ManuallyDrop :: new (orig) ; MappedMutexGuard { data , inner : & orig . lock . inner , _variance : PhantomData } } # [doc = " Makes a [`MappedMutexGuard`] for a component of the borrowed data. The"] # [doc = " original guard is returned as an `Err(...)` if the closure returns"] # [doc = " `None`."] # [doc = ""] # [doc = " The `Mutex` is already locked, so this cannot fail."] # [doc = ""] # [doc = " This is an associated function that needs to be used as"] # [doc = " `MutexGuard::filter_map(...)`. A method would interfere with methods of the"] # [doc = " same name on the contents of the `MutexGuard` used through `Deref`."] # [unstable (feature = "mapped_lock_guards" , issue = "117108")] pub fn filter_map < U , F > (orig : Self , f : F) -> Result < MappedMutexGuard < 'a , U > , Self > where F : FnOnce (& mut T) -> Option < & mut U > , U : ? Sized , { match f (unsafe { & mut * orig . lock . data . get () }) { Some (data) => { let data = NonNull :: from (data) ; let orig = ManuallyDrop :: new (orig) ; Ok (MappedMutexGuard { data , inner : & orig . lock . inner , _variance : PhantomData }) } None => Err (orig) , } } }}}
mkitem!{mkimpl!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized > Deref for MappedMutexGuard < '_ , T > { type Target = T ; fn deref (& self) -> & T { unsafe { self . data . as_ref () } } }}}
mkitem!{mkimpl!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized > DerefMut for MappedMutexGuard < '_ , T > { fn deref_mut (& mut self) -> & mut T { unsafe { self . data . as_mut () } } }}}
mkitem!{mkimpl!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized > Drop for MappedMutexGuard < '_ , T > { # [inline] fn drop (& mut self) { unsafe { self . inner . unlock () ; } } }}}
mkitem!{mkimpl!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized + fmt :: Debug > fmt :: Debug for MappedMutexGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }}}
mkitem!{mkimpl!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized + fmt :: Display > fmt :: Display for MappedMutexGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }}}
mkitem!{mkimpl!{impl < 'a , T : ? Sized > MappedMutexGuard < 'a , T > { # [doc = " Makes a [`MappedMutexGuard`] for a component of the borrowed data, e.g."] # [doc = " an enum variant."] # [doc = ""] # [doc = " The `Mutex` is already locked, so this cannot fail."] # [doc = ""] # [doc = " This is an associated function that needs to be used as"] # [doc = " `MappedMutexGuard::map(...)`. A method would interfere with methods of the"] # [doc = " same name on the contents of the `MutexGuard` used through `Deref`."] # [unstable (feature = "mapped_lock_guards" , issue = "117108")] pub fn map < U , F > (mut orig : Self , f : F) -> MappedMutexGuard < 'a , U > where F : FnOnce (& mut T) -> & mut U , U : ? Sized , { let data = NonNull :: from (f (unsafe { orig . data . as_mut () })) ; let orig = ManuallyDrop :: new (orig) ; MappedMutexGuard { data , inner : orig . inner , _variance : PhantomData } } # [doc = " Makes a [`MappedMutexGuard`] for a component of the borrowed data. The"] # [doc = " original guard is returned as an `Err(...)` if the closure returns"] # [doc = " `None`."] # [doc = ""] # [doc = " The `Mutex` is already locked, so this cannot fail."] # [doc = ""] # [doc = " This is an associated function that needs to be used as"] # [doc = " `MappedMutexGuard::filter_map(...)`. A method would interfere with methods of the"] # [doc = " same name on the contents of the `MutexGuard` used through `Deref`."] # [unstable (feature = "mapped_lock_guards" , issue = "117108")] pub fn filter_map < U , F > (mut orig : Self , f : F) -> Result < MappedMutexGuard < 'a , U > , Self > where F : FnOnce (& mut T) -> Option < & mut U > , U : ? Sized , { match f (unsafe { orig . data . as_mut () }) { Some (data) => { let data = NonNull :: from (data) ; let orig = ManuallyDrop :: new (orig) ; Ok (MappedMutexGuard { data , inner : orig . inner , _variance : PhantomData }) } None => Err (orig) , } } }}}