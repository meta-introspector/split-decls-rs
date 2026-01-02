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
mkuse!{use crate :: mem :: { self , ManuallyDrop , forget } ;}
mkuse!{use crate :: ops :: { Deref , DerefMut } ;}
mkuse!{use crate :: ptr :: NonNull ;}
mkuse!{use crate :: sync :: nonpoison :: { TryLockResult , WouldBlock } ;}
mkuse!{use crate :: sys :: sync as sys ;}
mkitem!{mkstruct!{# [doc = " A reader-writer lock that does not keep track of lock poisoning."] # [doc = ""] # [doc = " For more information about reader-writer locks, check out the documentation for the poisoning"] # [doc = " variant of this lock (which can be found at [`poison::RwLock`])."] # [doc = ""] # [doc = " [`poison::RwLock`]: crate::sync::poison::RwLock"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_rwlock)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::RwLock;"] # [doc = ""] # [doc = " let lock = RwLock::new(5);"] # [doc = ""] # [doc = " // many reader locks can be held at once"] # [doc = " {"] # [doc = "     let r1 = lock.read();"] # [doc = "     let r2 = lock.read();"] # [doc = "     assert_eq!(*r1, 5);"] # [doc = "     assert_eq!(*r2, 5);"] # [doc = " } // read locks are dropped at this point"] # [doc = ""] # [doc = " // only one write lock may be held, however"] # [doc = " {"] # [doc = "     let mut w = lock.write();"] # [doc = "     *w += 1;"] # [doc = "     assert_eq!(*w, 6);"] # [doc = " } // write lock is dropped here"] # [doc = " ```"] # [unstable (feature = "nonpoison_rwlock" , issue = "134645")] # [cfg_attr (not (test) , rustc_diagnostic_item = "NonPoisonRwLock")] pub struct RwLock < T : ? Sized > { # [doc = " The inner [`sys::RwLock`] that synchronizes thread access to the protected data."] inner : sys :: RwLock , # [doc = " The lock-protected data."] data : UnsafeCell < T > , }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_rwlock" , issue = "134645")] unsafe impl < T : ? Sized + Send > Send for RwLock < T > { }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_rwlock" , issue = "134645")] unsafe impl < T : ? Sized + Send + Sync > Sync for RwLock < T > { }}}
mkitem!{mkstruct!{# [doc = " RAII structure used to release the shared read access of a lock when"] # [doc = " dropped."] # [doc = ""] # [doc = " This structure is created by the [`read`] and [`try_read`] methods on"] # [doc = " [`RwLock`]."] # [doc = ""] # [doc = " [`read`]: RwLock::read"] # [doc = " [`try_read`]: RwLock::try_read"] # [must_use = "if unused the RwLock will immediately unlock"] # [must_not_suspend = "holding a RwLockReadGuard across suspend \
                      points can cause deadlocks, delays, \
                      and cause Futures to not implement `Send`"] # [unstable (feature = "nonpoison_rwlock" , issue = "134645")] # [clippy :: has_significant_drop] # [cfg_attr (not (test) , rustc_diagnostic_item = "NonPoisonRwLockReadGuard")] pub struct RwLockReadGuard < 'rwlock , T : ? Sized + 'rwlock > { # [doc = " A pointer to the data protected by the `RwLock`. Note that we use a pointer here instead of"] # [doc = " `&'rwlock T` to avoid `noalias` violations, because a `RwLockReadGuard` instance only holds"] # [doc = " immutability until it drops, not for its whole scope."] # [doc = " `NonNull` is preferable over `*const T` to allow for niche optimizations. `NonNull` is also"] # [doc = " covariant over `T`, just like we would have with `&T`."] data : NonNull < T > , # [doc = " A reference to the internal [`sys::RwLock`] that we have read-locked."] inner_lock : & 'rwlock sys :: RwLock , }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_rwlock" , issue = "134645")] impl < T : ? Sized > ! Send for RwLockReadGuard < '_ , T > { }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_rwlock" , issue = "134645")] unsafe impl < T : ? Sized + Sync > Sync for RwLockReadGuard < '_ , T > { }}}
mkitem!{mkstruct!{# [doc = " RAII structure used to release the exclusive write access of a lock when"] # [doc = " dropped."] # [doc = ""] # [doc = " This structure is created by the [`write`] and [`try_write`] methods"] # [doc = " on [`RwLock`]."] # [doc = ""] # [doc = " [`write`]: RwLock::write"] # [doc = " [`try_write`]: RwLock::try_write"] # [must_use = "if unused the RwLock will immediately unlock"] # [must_not_suspend = "holding a RwLockWriteGuard across suspend \
                      points can cause deadlocks, delays, \
                      and cause Future's to not implement `Send`"] # [unstable (feature = "nonpoison_rwlock" , issue = "134645")] # [clippy :: has_significant_drop] # [cfg_attr (not (test) , rustc_diagnostic_item = "NonPoisonRwLockWriteGuard")] pub struct RwLockWriteGuard < 'rwlock , T : ? Sized + 'rwlock > { # [doc = " A reference to the [`RwLock`] that we have write-locked."] lock : & 'rwlock RwLock < T > , }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_rwlock" , issue = "134645")] impl < T : ? Sized > ! Send for RwLockWriteGuard < '_ , T > { }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_rwlock" , issue = "134645")] unsafe impl < T : ? Sized + Sync > Sync for RwLockWriteGuard < '_ , T > { }}}
mkitem!{mkstruct!{# [doc = " RAII structure used to release the shared read access of a lock when"] # [doc = " dropped, which can point to a subfield of the protected data."] # [doc = ""] # [doc = " This structure is created by the [`map`] and [`filter_map`] methods"] # [doc = " on [`RwLockReadGuard`]."] # [doc = ""] # [doc = " [`map`]: RwLockReadGuard::map"] # [doc = " [`filter_map`]: RwLockReadGuard::filter_map"] # [must_use = "if unused the RwLock will immediately unlock"] # [must_not_suspend = "holding a MappedRwLockReadGuard across suspend \
                      points can cause deadlocks, delays, \
                      and cause Futures to not implement `Send`"] # [unstable (feature = "mapped_lock_guards" , issue = "117108")] # [clippy :: has_significant_drop] pub struct MappedRwLockReadGuard < 'rwlock , T : ? Sized + 'rwlock > { # [doc = " A pointer to the data protected by the `RwLock`. Note that we use a pointer here instead of"] # [doc = " `&'rwlock T` to avoid `noalias` violations, because a `MappedRwLockReadGuard` instance only"] # [doc = " holds immutability until it drops, not for its whole scope."] # [doc = " `NonNull` is preferable over `*const T` to allow for niche optimizations. `NonNull` is also"] # [doc = " covariant over `T`, just like we would have with `&T`."] data : NonNull < T > , # [doc = " A reference to the internal [`sys::RwLock`] that we have read-locked."] inner_lock : & 'rwlock sys :: RwLock , }}}
mkitem!{mkimpl!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized > ! Send for MappedRwLockReadGuard < '_ , T > { }}}
mkitem!{mkimpl!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] unsafe impl < T : ? Sized + Sync > Sync for MappedRwLockReadGuard < '_ , T > { }}}
mkitem!{mkstruct!{# [doc = " RAII structure used to release the exclusive write access of a lock when"] # [doc = " dropped, which can point to a subfield of the protected data."] # [doc = ""] # [doc = " This structure is created by the [`map`] and [`filter_map`] methods"] # [doc = " on [`RwLockWriteGuard`]."] # [doc = ""] # [doc = " [`map`]: RwLockWriteGuard::map"] # [doc = " [`filter_map`]: RwLockWriteGuard::filter_map"] # [must_use = "if unused the RwLock will immediately unlock"] # [must_not_suspend = "holding a MappedRwLockWriteGuard across suspend \
                      points can cause deadlocks, delays, \
                      and cause Future's to not implement `Send`"] # [unstable (feature = "mapped_lock_guards" , issue = "117108")] # [clippy :: has_significant_drop] pub struct MappedRwLockWriteGuard < 'rwlock , T : ? Sized + 'rwlock > { # [doc = " A pointer to the data protected by the `RwLock`. Note that we use a pointer here instead of"] # [doc = " `&'rwlock T` to avoid `noalias` violations, because a `MappedRwLockWriteGuard` instance only"] # [doc = " holds uniquneness until it drops, not for its whole scope."] # [doc = " `NonNull` is preferable over `*const T` to allow for niche optimizations."] data : NonNull < T > , # [doc = " `NonNull` is covariant over `T`, so we add a `PhantomData<&'rwlock mut T>` field here to"] # [doc = " enforce the correct invariance over `T`."] _variance : PhantomData < & 'rwlock mut T > , # [doc = " A reference to the internal [`sys::RwLock`] that we have write-locked."] inner_lock : & 'rwlock sys :: RwLock , }}}
mkitem!{mkimpl!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized > ! Send for MappedRwLockWriteGuard < '_ , T > { }}}
mkitem!{mkimpl!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] unsafe impl < T : ? Sized + Sync > Sync for MappedRwLockWriteGuard < '_ , T > { }}}
mkitem!{mkimpl!{impl < T > RwLock < T > { # [doc = " Creates a new instance of an `RwLock<T>` which is unlocked."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_rwlock)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::RwLock;"] # [doc = ""] # [doc = " let lock = RwLock::new(5);"] # [doc = " ```"] # [unstable (feature = "nonpoison_rwlock" , issue = "134645")] # [inline] pub const fn new (t : T) -> RwLock < T > { RwLock { inner : sys :: RwLock :: new () , data : UnsafeCell :: new (t) } } # [doc = " Returns the contained value by cloning it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_rwlock)]"] # [doc = " #![feature(lock_value_accessors)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::RwLock;"] # [doc = ""] # [doc = " let mut lock = RwLock::new(7);"] # [doc = ""] # [doc = " assert_eq!(lock.get_cloned(), 7);"] # [doc = " ```"] # [unstable (feature = "lock_value_accessors" , issue = "133407")] pub fn get_cloned (& self) -> T where T : Clone , { self . read () . clone () } # [doc = " Sets the contained value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_rwlock)]"] # [doc = " #![feature(lock_value_accessors)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::RwLock;"] # [doc = ""] # [doc = " let mut lock = RwLock::new(7);"] # [doc = ""] # [doc = " assert_eq!(lock.get_cloned(), 7);"] # [doc = " lock.set(11);"] # [doc = " assert_eq!(lock.get_cloned(), 11);"] # [doc = " ```"] # [unstable (feature = "lock_value_accessors" , issue = "133407")] pub fn set (& self , value : T) { if mem :: needs_drop :: < T > () { drop (self . replace (value)) } else { * self . write () = value ; } } # [doc = " Replaces the contained value with `value`, and returns the old contained value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_rwlock)]"] # [doc = " #![feature(lock_value_accessors)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::RwLock;"] # [doc = ""] # [doc = " let mut lock = RwLock::new(7);"] # [doc = ""] # [doc = " assert_eq!(lock.replace(11), 7);"] # [doc = " assert_eq!(lock.get_cloned(), 11);"] # [doc = " ```"] # [unstable (feature = "lock_value_accessors" , issue = "133407")] pub fn replace (& self , value : T) -> T { let mut guard = self . write () ; mem :: replace (& mut * guard , value) } }}}
mkitem!{mkimpl!{impl < T : ? Sized > RwLock < T > { # [doc = " Locks this `RwLock` with shared read access, blocking the current thread"] # [doc = " until it can be acquired."] # [doc = ""] # [doc = " The calling thread will be blocked until there are no more writers which"] # [doc = " hold the lock. There may be other readers currently inside the lock when"] # [doc = " this method returns. This method does not provide any guarantees with"] # [doc = " respect to the ordering of whether contentious readers or writers will"] # [doc = " acquire the lock first."] # [doc = ""] # [doc = " Returns an RAII guard which will release this thread's shared access"] # [doc = " once it is dropped."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function might panic when called if the lock is already held by the current thread."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_rwlock)]"] # [doc = ""] # [doc = " use std::sync::Arc;"] # [doc = " use std::sync::nonpoison::RwLock;"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let lock = Arc::new(RwLock::new(1));"] # [doc = " let c_lock = Arc::clone(&lock);"] # [doc = ""] # [doc = " let n = lock.read();"] # [doc = " assert_eq!(*n, 1);"] # [doc = ""] # [doc = " thread::spawn(move || {"] # [doc = "     let r = c_lock.read();"] # [doc = " }).join().unwrap();"] # [doc = " ```"] # [inline] # [unstable (feature = "nonpoison_rwlock" , issue = "134645")] pub fn read (& self) -> RwLockReadGuard < '_ , T > { unsafe { self . inner . read () ; RwLockReadGuard :: new (self) } } # [doc = " Attempts to acquire this `RwLock` with shared read access."] # [doc = ""] # [doc = " If the access could not be granted at this time, then `Err` is returned."] # [doc = " Otherwise, an RAII guard is returned which will release the shared access"] # [doc = " when it is dropped."] # [doc = ""] # [doc = " This function does not block."] # [doc = ""] # [doc = " This function does not provide any guarantees with respect to the ordering"] # [doc = " of whether contentious readers or writers will acquire the lock first."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return the [`WouldBlock`] error if the `RwLock` could"] # [doc = " not be acquired because it was already locked exclusively."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_rwlock)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::RwLock;"] # [doc = ""] # [doc = " let lock = RwLock::new(1);"] # [doc = ""] # [doc = " match lock.try_read() {"] # [doc = "     Ok(n) => assert_eq!(*n, 1),"] # [doc = "     Err(_) => unreachable!(),"] # [doc = " };"] # [doc = " ```"] # [inline] # [unstable (feature = "nonpoison_rwlock" , issue = "134645")] pub fn try_read (& self) -> TryLockResult < RwLockReadGuard < '_ , T > > { unsafe { if self . inner . try_read () { Ok (RwLockReadGuard :: new (self)) } else { Err (WouldBlock) } } } # [doc = " Locks this `RwLock` with exclusive write access, blocking the current"] # [doc = " thread until it can be acquired."] # [doc = ""] # [doc = " This function will not return while other writers or other readers"] # [doc = " currently have access to the lock."] # [doc = ""] # [doc = " Returns an RAII guard which will drop the write access of this `RwLock`"] # [doc = " when dropped."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function might panic when called if the lock is already held by the current thread."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_rwlock)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::RwLock;"] # [doc = ""] # [doc = " let lock = RwLock::new(1);"] # [doc = ""] # [doc = " let mut n = lock.write();"] # [doc = " *n = 2;"] # [doc = ""] # [doc = " assert!(lock.try_read().is_err());"] # [doc = " ```"] # [inline] # [unstable (feature = "nonpoison_rwlock" , issue = "134645")] pub fn write (& self) -> RwLockWriteGuard < '_ , T > { unsafe { self . inner . write () ; RwLockWriteGuard :: new (self) } } # [doc = " Attempts to lock this `RwLock` with exclusive write access."] # [doc = ""] # [doc = " If the lock could not be acquired at this time, then `Err` is returned."] # [doc = " Otherwise, an RAII guard is returned which will release the lock when"] # [doc = " it is dropped."] # [doc = ""] # [doc = " This function does not block."] # [doc = ""] # [doc = " This function does not provide any guarantees with respect to the ordering"] # [doc = " of whether contentious readers or writers will acquire the lock first."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return the [`WouldBlock`] error if the `RwLock` could"] # [doc = " not be acquired because it was already locked."] # [doc = ""] # [doc = " [`WouldBlock`]: WouldBlock"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_rwlock)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::RwLock;"] # [doc = ""] # [doc = " let lock = RwLock::new(1);"] # [doc = ""] # [doc = " let n = lock.read();"] # [doc = " assert_eq!(*n, 1);"] # [doc = ""] # [doc = " assert!(lock.try_write().is_err());"] # [doc = " ```"] # [inline] # [unstable (feature = "nonpoison_rwlock" , issue = "134645")] pub fn try_write (& self) -> TryLockResult < RwLockWriteGuard < '_ , T > > { unsafe { if self . inner . try_write () { Ok (RwLockWriteGuard :: new (self)) } else { Err (WouldBlock) } } } # [doc = " Consumes this `RwLock`, returning the underlying data."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_rwlock)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::RwLock;"] # [doc = ""] # [doc = " let lock = RwLock::new(String::new());"] # [doc = " {"] # [doc = "     let mut s = lock.write();"] # [doc = "     *s = \"modified\".to_owned();"] # [doc = " }"] # [doc = " assert_eq!(lock.into_inner(), \"modified\");"] # [doc = " ```"] # [unstable (feature = "nonpoison_rwlock" , issue = "134645")] pub fn into_inner (self) -> T where T : Sized , { self . data . into_inner () } # [doc = " Returns a mutable reference to the underlying data."] # [doc = ""] # [doc = " Since this call borrows the `RwLock` mutably, no actual locking needs to"] # [doc = " take place -- the mutable borrow statically guarantees no new locks can be acquired"] # [doc = " while this reference exists. Note that this method does not clear any previously abandoned"] # [doc = " locks (e.g., via [`forget()`] on a [`RwLockReadGuard`] or [`RwLockWriteGuard`])."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_rwlock)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::RwLock;"] # [doc = ""] # [doc = " let mut lock = RwLock::new(0);"] # [doc = " *lock.get_mut() = 10;"] # [doc = " assert_eq!(*lock.read(), 10);"] # [doc = " ```"] # [unstable (feature = "nonpoison_rwlock" , issue = "134645")] pub fn get_mut (& mut self) -> & mut T { self . data . get_mut () } # [doc = " Returns a raw pointer to the underlying data."] # [doc = ""] # [doc = " The returned pointer is always non-null and properly aligned, but it is"] # [doc = " the user's responsibility to ensure that any reads and writes through it"] # [doc = " are properly synchronized to avoid data races, and that it is not read"] # [doc = " or written through after the lock is dropped."] # [unstable (feature = "rwlock_data_ptr" , issue = "140368")] pub fn data_ptr (& self) -> * mut T { self . data . get () } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_rwlock" , issue = "134645")] impl < T : ? Sized + fmt :: Debug > fmt :: Debug for RwLock < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_struct ("RwLock") ; match self . try_read () { Ok (guard) => { d . field ("data" , & & * guard) ; } Err (WouldBlock) => { d . field ("data" , & format_args ! ("<locked>")) ; } } d . finish_non_exhaustive () } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_rwlock" , issue = "134645")] impl < T : Default > Default for RwLock < T > { # [doc = " Creates a new `RwLock<T>`, with the `Default` value for T."] fn default () -> RwLock < T > { RwLock :: new (Default :: default ()) } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_rwlock" , issue = "134645")] impl < T > From < T > for RwLock < T > { # [doc = " Creates a new instance of an `RwLock<T>` which is unlocked."] # [doc = " This is equivalent to [`RwLock::new`]."] fn from (t : T) -> Self { RwLock :: new (t) } }}}
mkitem!{mkimpl!{impl < 'rwlock , T : ? Sized > RwLockReadGuard < 'rwlock , T > { # [doc = " Creates a new instance of `RwLockReadGuard<T>` from a `RwLock<T>`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is safe if and only if the same thread has successfully and safely called"] # [doc = " `lock.inner.read()`, `lock.inner.try_read()`, or `lock.inner.downgrade()` before"] # [doc = " instantiating this object."] unsafe fn new (lock : & 'rwlock RwLock < T >) -> RwLockReadGuard < 'rwlock , T > { RwLockReadGuard { data : unsafe { NonNull :: new_unchecked (lock . data . get ()) } , inner_lock : & lock . inner , } } # [doc = " Makes a [`MappedRwLockReadGuard`] for a component of the borrowed data, e.g."] # [doc = " an enum variant."] # [doc = ""] # [doc = " The `RwLock` is already locked for reading, so this cannot fail."] # [doc = ""] # [doc = " This is an associated function that needs to be used as"] # [doc = " `RwLockReadGuard::map(...)`. A method would interfere with methods of"] # [doc = " the same name on the contents of the `RwLockReadGuard` used through"] # [doc = " `Deref`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If the closure panics, the guard will be dropped (unlocked)."] # [unstable (feature = "mapped_lock_guards" , issue = "117108")] pub fn map < U , F > (orig : Self , f : F) -> MappedRwLockReadGuard < 'rwlock , U > where F : FnOnce (& T) -> & U , U : ? Sized , { let data = NonNull :: from (f (unsafe { orig . data . as_ref () })) ; let orig = ManuallyDrop :: new (orig) ; MappedRwLockReadGuard { data , inner_lock : & orig . inner_lock } } # [doc = " Makes a [`MappedRwLockReadGuard`] for a component of the borrowed data. The"] # [doc = " original guard is returned as an `Err(...)` if the closure returns"] # [doc = " `None`."] # [doc = ""] # [doc = " The `RwLock` is already locked for reading, so this cannot fail."] # [doc = ""] # [doc = " This is an associated function that needs to be used as"] # [doc = " `RwLockReadGuard::filter_map(...)`. A method would interfere with methods"] # [doc = " of the same name on the contents of the `RwLockReadGuard` used through"] # [doc = " `Deref`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If the closure panics, the guard will be dropped (unlocked)."] # [unstable (feature = "mapped_lock_guards" , issue = "117108")] pub fn filter_map < U , F > (orig : Self , f : F) -> Result < MappedRwLockReadGuard < 'rwlock , U > , Self > where F : FnOnce (& T) -> Option < & U > , U : ? Sized , { match f (unsafe { orig . data . as_ref () }) { Some (data) => { let data = NonNull :: from (data) ; let orig = ManuallyDrop :: new (orig) ; Ok (MappedRwLockReadGuard { data , inner_lock : & orig . inner_lock }) } None => Err (orig) , } } }}}
mkitem!{mkimpl!{impl < 'rwlock , T : ? Sized > RwLockWriteGuard < 'rwlock , T > { # [doc = " Creates a new instance of `RwLockWriteGuard<T>` from a `RwLock<T>`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is safe if and only if the same thread has successfully and safely called"] # [doc = " `lock.inner.write()`, `lock.inner.try_write()`, or `lock.inner.try_upgrade` before"] # [doc = " instantiating this object."] unsafe fn new (lock : & 'rwlock RwLock < T >) -> RwLockWriteGuard < 'rwlock , T > { RwLockWriteGuard { lock } } # [doc = " Downgrades a write-locked `RwLockWriteGuard` into a read-locked [`RwLockReadGuard`]."] # [doc = ""] # [doc = " Since we have the `RwLockWriteGuard`, the [`RwLock`] must already be locked for writing, so"] # [doc = " this method cannot fail."] # [doc = ""] # [doc = " After downgrading, other readers will be allowed to read the protected data."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " `downgrade` takes ownership of the `RwLockWriteGuard` and returns a [`RwLockReadGuard`]."] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_rwlock)]"] # [doc = " #![feature(rwlock_downgrade)]"] # [doc = ""] # [doc = " use std::sync::nonpoison::{RwLock, RwLockWriteGuard};"] # [doc = ""] # [doc = " let rw = RwLock::new(0);"] # [doc = ""] # [doc = " let mut write_guard = rw.write();"] # [doc = " *write_guard = 42;"] # [doc = ""] # [doc = " let read_guard = RwLockWriteGuard::downgrade(write_guard);"] # [doc = " assert_eq!(42, *read_guard);"] # [doc = " ```"] # [doc = ""] # [doc = " `downgrade` will _atomically_ change the state of the [`RwLock`] from exclusive mode into"] # [doc = " shared mode. This means that it is impossible for another writing thread to get in between a"] # [doc = " thread calling `downgrade` and any reads it performs after downgrading."] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(nonpoison_rwlock)]"] # [doc = " #![feature(rwlock_downgrade)]"] # [doc = ""] # [doc = " use std::sync::Arc;"] # [doc = " use std::sync::nonpoison::{RwLock, RwLockWriteGuard};"] # [doc = ""] # [doc = " let rw = Arc::new(RwLock::new(1));"] # [doc = ""] # [doc = " // Put the lock in write mode."] # [doc = " let mut main_write_guard = rw.write();"] # [doc = ""] # [doc = " let rw_clone = rw.clone();"] # [doc = " let evil_handle = std::thread::spawn(move || {"] # [doc = "     // This will not return until the main thread drops the `main_read_guard`."] # [doc = "     let mut evil_guard = rw_clone.write();"] # [doc = ""] # [doc = "     assert_eq!(*evil_guard, 2);"] # [doc = "     *evil_guard = 3;"] # [doc = " });"] # [doc = ""] # [doc = " *main_write_guard = 2;"] # [doc = ""] # [doc = " // Atomically downgrade the write guard into a read guard."] # [doc = " let main_read_guard = RwLockWriteGuard::downgrade(main_write_guard);"] # [doc = ""] # [doc = " // Since `downgrade` is atomic, the writer thread cannot have changed the protected data."] # [doc = " assert_eq!(*main_read_guard, 2, \"`downgrade` was not atomic\");"] # [doc = " #"] # [doc = " # drop(main_read_guard);"] # [doc = " # evil_handle.join().unwrap();"] # [doc = " #"] # [doc = " # let final_check = rw.read();"] # [doc = " # assert_eq!(*final_check, 3);"] # [doc = " ```"] # [unstable (feature = "rwlock_downgrade" , issue = "128203")] pub fn downgrade (s : Self) -> RwLockReadGuard < 'rwlock , T > { let lock = s . lock ; forget (s) ; unsafe { lock . inner . downgrade () } ; unsafe { RwLockReadGuard :: new (lock) } } # [doc = " Makes a [`MappedRwLockWriteGuard`] for a component of the borrowed data, e.g."] # [doc = " an enum variant."] # [doc = ""] # [doc = " The `RwLock` is already locked for writing, so this cannot fail."] # [doc = ""] # [doc = " This is an associated function that needs to be used as"] # [doc = " `RwLockWriteGuard::map(...)`. A method would interfere with methods of"] # [doc = " the same name on the contents of the `RwLockWriteGuard` used through"] # [doc = " `Deref`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If the closure panics, the guard will be dropped (unlocked)."] # [unstable (feature = "mapped_lock_guards" , issue = "117108")] pub fn map < U , F > (orig : Self , f : F) -> MappedRwLockWriteGuard < 'rwlock , U > where F : FnOnce (& mut T) -> & mut U , U : ? Sized , { let data = NonNull :: from (f (unsafe { & mut * orig . lock . data . get () })) ; let orig = ManuallyDrop :: new (orig) ; MappedRwLockWriteGuard { data , inner_lock : & orig . lock . inner , _variance : PhantomData } } # [doc = " Makes a [`MappedRwLockWriteGuard`] for a component of the borrowed data. The"] # [doc = " original guard is returned as an `Err(...)` if the closure returns"] # [doc = " `None`."] # [doc = ""] # [doc = " The `RwLock` is already locked for writing, so this cannot fail."] # [doc = ""] # [doc = " This is an associated function that needs to be used as"] # [doc = " `RwLockWriteGuard::filter_map(...)`. A method would interfere with methods"] # [doc = " of the same name on the contents of the `RwLockWriteGuard` used through"] # [doc = " `Deref`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If the closure panics, the guard will be dropped (unlocked)."] # [unstable (feature = "mapped_lock_guards" , issue = "117108")] pub fn filter_map < U , F > (orig : Self , f : F) -> Result < MappedRwLockWriteGuard < 'rwlock , U > , Self > where F : FnOnce (& mut T) -> Option < & mut U > , U : ? Sized , { match f (unsafe { & mut * orig . lock . data . get () }) { Some (data) => { let data = NonNull :: from (data) ; let orig = ManuallyDrop :: new (orig) ; Ok (MappedRwLockWriteGuard { data , inner_lock : & orig . lock . inner , _variance : PhantomData , }) } None => Err (orig) , } } }}}
mkitem!{mkimpl!{impl < 'rwlock , T : ? Sized > MappedRwLockReadGuard < 'rwlock , T > { # [doc = " Makes a [`MappedRwLockReadGuard`] for a component of the borrowed data,"] # [doc = " e.g. an enum variant."] # [doc = ""] # [doc = " The `RwLock` is already locked for reading, so this cannot fail."] # [doc = ""] # [doc = " This is an associated function that needs to be used as"] # [doc = " `MappedRwLockReadGuard::map(...)`. A method would interfere with"] # [doc = " methods of the same name on the contents of the `MappedRwLockReadGuard`"] # [doc = " used through `Deref`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If the closure panics, the guard will be dropped (unlocked)."] # [unstable (feature = "mapped_lock_guards" , issue = "117108")] pub fn map < U , F > (orig : Self , f : F) -> MappedRwLockReadGuard < 'rwlock , U > where F : FnOnce (& T) -> & U , U : ? Sized , { let data = NonNull :: from (f (unsafe { orig . data . as_ref () })) ; let orig = ManuallyDrop :: new (orig) ; MappedRwLockReadGuard { data , inner_lock : & orig . inner_lock } } # [doc = " Makes a [`MappedRwLockReadGuard`] for a component of the borrowed data."] # [doc = " The original guard is returned as an `Err(...)` if the closure returns"] # [doc = " `None`."] # [doc = ""] # [doc = " The `RwLock` is already locked for reading, so this cannot fail."] # [doc = ""] # [doc = " This is an associated function that needs to be used as"] # [doc = " `MappedRwLockReadGuard::filter_map(...)`. A method would interfere with"] # [doc = " methods of the same name on the contents of the `MappedRwLockReadGuard`"] # [doc = " used through `Deref`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If the closure panics, the guard will be dropped (unlocked)."] # [unstable (feature = "mapped_lock_guards" , issue = "117108")] pub fn filter_map < U , F > (orig : Self , f : F) -> Result < MappedRwLockReadGuard < 'rwlock , U > , Self > where F : FnOnce (& T) -> Option < & U > , U : ? Sized , { match f (unsafe { orig . data . as_ref () }) { Some (data) => { let data = NonNull :: from (data) ; let orig = ManuallyDrop :: new (orig) ; Ok (MappedRwLockReadGuard { data , inner_lock : & orig . inner_lock }) } None => Err (orig) , } } }}}
mkitem!{mkimpl!{impl < 'rwlock , T : ? Sized > MappedRwLockWriteGuard < 'rwlock , T > { # [doc = " Makes a [`MappedRwLockWriteGuard`] for a component of the borrowed data,"] # [doc = " e.g. an enum variant."] # [doc = ""] # [doc = " The `RwLock` is already locked for writing, so this cannot fail."] # [doc = ""] # [doc = " This is an associated function that needs to be used as"] # [doc = " `MappedRwLockWriteGuard::map(...)`. A method would interfere with"] # [doc = " methods of the same name on the contents of the `MappedRwLockWriteGuard`"] # [doc = " used through `Deref`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If the closure panics, the guard will be dropped (unlocked)."] # [unstable (feature = "mapped_lock_guards" , issue = "117108")] pub fn map < U , F > (mut orig : Self , f : F) -> MappedRwLockWriteGuard < 'rwlock , U > where F : FnOnce (& mut T) -> & mut U , U : ? Sized , { let data = NonNull :: from (f (unsafe { orig . data . as_mut () })) ; let orig = ManuallyDrop :: new (orig) ; MappedRwLockWriteGuard { data , inner_lock : orig . inner_lock , _variance : PhantomData } } # [doc = " Makes a [`MappedRwLockWriteGuard`] for a component of the borrowed data."] # [doc = " The original guard is returned as an `Err(...)` if the closure returns"] # [doc = " `None`."] # [doc = ""] # [doc = " The `RwLock` is already locked for writing, so this cannot fail."] # [doc = ""] # [doc = " This is an associated function that needs to be used as"] # [doc = " `MappedRwLockWriteGuard::filter_map(...)`. A method would interfere with"] # [doc = " methods of the same name on the contents of the `MappedRwLockWriteGuard`"] # [doc = " used through `Deref`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If the closure panics, the guard will be dropped (unlocked)."] # [unstable (feature = "mapped_lock_guards" , issue = "117108")] pub fn filter_map < U , F > (mut orig : Self , f : F ,) -> Result < MappedRwLockWriteGuard < 'rwlock , U > , Self > where F : FnOnce (& mut T) -> Option < & mut U > , U : ? Sized , { match f (unsafe { orig . data . as_mut () }) { Some (data) => { let data = NonNull :: from (data) ; let orig = ManuallyDrop :: new (orig) ; Ok (MappedRwLockWriteGuard { data , inner_lock : orig . inner_lock , _variance : PhantomData , }) } None => Err (orig) , } } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_rwlock" , issue = "134645")] impl < T : ? Sized > Drop for RwLockReadGuard < '_ , T > { fn drop (& mut self) { unsafe { self . inner_lock . read_unlock () ; } } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_rwlock" , issue = "134645")] impl < T : ? Sized > Drop for RwLockWriteGuard < '_ , T > { fn drop (& mut self) { unsafe { self . lock . inner . write_unlock () ; } } }}}
mkitem!{mkimpl!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized > Drop for MappedRwLockReadGuard < '_ , T > { fn drop (& mut self) { unsafe { self . inner_lock . read_unlock () ; } } }}}
mkitem!{mkimpl!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized > Drop for MappedRwLockWriteGuard < '_ , T > { fn drop (& mut self) { unsafe { self . inner_lock . write_unlock () ; } } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_rwlock" , issue = "134645")] impl < T : ? Sized > Deref for RwLockReadGuard < '_ , T > { type Target = T ; fn deref (& self) -> & T { unsafe { self . data . as_ref () } } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_rwlock" , issue = "134645")] impl < T : ? Sized > Deref for RwLockWriteGuard < '_ , T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * self . lock . data . get () } } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_rwlock" , issue = "134645")] impl < T : ? Sized > DerefMut for RwLockWriteGuard < '_ , T > { fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . lock . data . get () } } }}}
mkitem!{mkimpl!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized > Deref for MappedRwLockReadGuard < '_ , T > { type Target = T ; fn deref (& self) -> & T { unsafe { self . data . as_ref () } } }}}
mkitem!{mkimpl!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized > Deref for MappedRwLockWriteGuard < '_ , T > { type Target = T ; fn deref (& self) -> & T { unsafe { self . data . as_ref () } } }}}
mkitem!{mkimpl!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized > DerefMut for MappedRwLockWriteGuard < '_ , T > { fn deref_mut (& mut self) -> & mut T { unsafe { self . data . as_mut () } } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_rwlock" , issue = "134645")] impl < T : ? Sized + fmt :: Debug > fmt :: Debug for RwLockReadGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_rwlock" , issue = "134645")] impl < T : ? Sized + fmt :: Display > fmt :: Display for RwLockReadGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_rwlock" , issue = "134645")] impl < T : ? Sized + fmt :: Debug > fmt :: Debug for RwLockWriteGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }}}
mkitem!{mkimpl!{# [unstable (feature = "nonpoison_rwlock" , issue = "134645")] impl < T : ? Sized + fmt :: Display > fmt :: Display for RwLockWriteGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }}}
mkitem!{mkimpl!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized + fmt :: Debug > fmt :: Debug for MappedRwLockReadGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }}}
mkitem!{mkimpl!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized + fmt :: Display > fmt :: Display for MappedRwLockReadGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }}}
mkitem!{mkimpl!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized + fmt :: Debug > fmt :: Debug for MappedRwLockWriteGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }}}
mkitem!{mkimpl!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized + fmt :: Display > fmt :: Display for MappedRwLockWriteGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }}}