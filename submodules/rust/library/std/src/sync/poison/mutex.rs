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
mkuse!{use crate :: cell :: UnsafeCell ;}
mkuse!{use crate :: fmt ;}
mkuse!{use crate :: marker :: PhantomData ;}
mkuse!{use crate :: mem :: { self , ManuallyDrop } ;}
mkuse!{use crate :: ops :: { Deref , DerefMut } ;}
mkuse!{use crate :: ptr :: NonNull ;}
mkuse!{use crate :: sync :: { LockResult , PoisonError , TryLockError , TryLockResult , poison } ;}
mkuse!{use crate :: sys :: sync as sys ;}
mkitem!{mkstruct!{#[doc = " A mutual exclusion primitive useful for protecting shared data"] #[doc = ""] #[doc = " This mutex will block threads waiting for the lock to become available. The"] #[doc = " mutex can be created via a [`new`] constructor. Each mutex has a type parameter"] #[doc = " which represents the data that it is protecting. The data can only be accessed"] #[doc = " through the RAII guards returned from [`lock`] and [`try_lock`], which"] #[doc = " guarantees that the data is only ever accessed when the mutex is locked."] #[doc = ""] #[doc = " # Poisoning"] #[doc = ""] #[doc = " The mutexes in this module implement a strategy called \"poisoning\" where a"] #[doc = " mutex becomes poisoned if it recognizes that the thread holding it has"] #[doc = " panicked."] #[doc = ""] #[doc = " Once a mutex is poisoned, all other threads are unable to access the data by"] #[doc = " default as it is likely tainted (some invariant is not being upheld). For a"] #[doc = " mutex, this means that the [`lock`] and [`try_lock`] methods return a"] #[doc = " [`Result`] which indicates whether a mutex has been poisoned or not. Most"] #[doc = " usage of a mutex will simply [`unwrap()`] these results, propagating panics"] #[doc = " among threads to ensure that a possibly invalid invariant is not witnessed."] #[doc = ""] #[doc = " Poisoning is only advisory: the [`PoisonError`] type has an [`into_inner`]"] #[doc = " method which will return the guard that would have otherwise been returned"] #[doc = " on a successful lock. This allows access to the data, despite the lock being"] #[doc = " poisoned."] #[doc = ""] #[doc = " In addition, the panic detection is not ideal, so even unpoisoned mutexes"] #[doc = " need to be handled with care, since certain panics may have been skipped."] #[doc = " Here is a non-exhaustive list of situations where this might occur:"] #[doc = ""] #[doc = " - If a mutex is locked while a panic is underway, e.g. within a [`Drop`]"] #[doc = "   implementation or a [panic hook], panicking for the second time while the"] #[doc = "   lock is held will leave the mutex unpoisoned. Note that while double panic"] #[doc = "   usually aborts the program, [`catch_unwind`] can prevent this."] #[doc = ""] #[doc = " - Locking and unlocking the mutex across different panic contexts, e.g. by"] #[doc = "   storing the guard to a [`Cell`] within [`Drop::drop`] and accessing it"] #[doc = "   outside, or vice versa, can affect poisoning status in an unexpected way."] #[doc = ""] #[doc = " - Foreign exceptions do not currently trigger poisoning even in absence of"] #[doc = "   other panics."] #[doc = ""] #[doc = " While this rarely happens in realistic code, `unsafe` code cannot rely on"] #[doc = " poisoning for soundness, since the behavior of poisoning can depend on"] #[doc = " outside context. Here's an example of **incorrect** use of poisoning:"] #[doc = ""] #[doc = " ```rust"] #[doc = " use std::sync::Mutex;"] #[doc = ""] #[doc = " struct MutexBox<T> {"] #[doc = "     data: Mutex<*mut T>,"] #[doc = " }"] #[doc = ""] #[doc = " impl<T> MutexBox<T> {"] #[doc = "     pub fn new(value: T) -> Self {"] #[doc = "         Self {"] #[doc = "             data: Mutex::new(Box::into_raw(Box::new(value))),"] #[doc = "         }"] #[doc = "     }"] #[doc = ""] #[doc = "     pub fn replace_with(&self, f: impl FnOnce(T) -> T) {"] #[doc = "         let ptr = self.data.lock().expect(\"poisoned\");"] #[doc = "         // While `f` is running, the data is moved out of `*ptr`. If `f`"] #[doc = "         // panics, `*ptr` keeps pointing at a dropped value. The intention"] #[doc = "         // is that this will poison the mutex, so the following calls to"] #[doc = "         // `replace_with` will panic without reading `*ptr`. But since"] #[doc = "         // poisoning is not guaranteed to occur if this is run from a panic"] #[doc = "         // hook, this can lead to use-after-free."] #[doc = "         unsafe {"] #[doc = "             (*ptr).write(f((*ptr).read()));"] #[doc = "         }"] #[doc = "     }"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " [`new`]: Self::new"] #[doc = " [`lock`]: Self::lock"] #[doc = " [`try_lock`]: Self::try_lock"] #[doc = " [`unwrap()`]: Result::unwrap"] #[doc = " [`PoisonError`]: super::PoisonError"] #[doc = " [`into_inner`]: super::PoisonError::into_inner"] #[doc = " [panic hook]: crate::panic::set_hook"] #[doc = " [`catch_unwind`]: crate::panic::catch_unwind"] #[doc = " [`Cell`]: crate::cell::Cell"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::sync::{Arc, Mutex};"] #[doc = " use std::thread;"] #[doc = " use std::sync::mpsc::channel;"] #[doc = ""] #[doc = " const N: usize = 10;"] #[doc = ""] #[doc = " // Spawn a few threads to increment a shared variable (non-atomically), and"] #[doc = " // let the main thread know once all increments are done."] #[doc = " //"] #[doc = " // Here we're using an Arc to share memory among threads, and the data inside"] #[doc = " // the Arc is protected with a mutex."] #[doc = " let data = Arc::new(Mutex::new(0));"] #[doc = ""] #[doc = " let (tx, rx) = channel();"] #[doc = " for _ in 0..N {"] #[doc = "     let (data, tx) = (Arc::clone(&data), tx.clone());"] #[doc = "     thread::spawn(move || {"] #[doc = "         // The shared state can only be accessed once the lock is held."] #[doc = "         // Our non-atomic increment is safe because we're the only thread"] #[doc = "         // which can access the shared state when the lock is held."] #[doc = "         //"] #[doc = "         // We unwrap() the return value to assert that we are not expecting"] #[doc = "         // threads to ever fail while holding the lock."] #[doc = "         let mut data = data.lock().unwrap();"] #[doc = "         *data += 1;"] #[doc = "         if *data == N {"] #[doc = "             tx.send(()).unwrap();"] #[doc = "         }"] #[doc = "         // the lock is unlocked here when `data` goes out of scope."] #[doc = "     });"] #[doc = " }"] #[doc = ""] #[doc = " rx.recv().unwrap();"] #[doc = " ```"] #[doc = ""] #[doc = " To recover from a poisoned mutex:"] #[doc = ""] #[doc = " ```"] #[doc = " use std::sync::{Arc, Mutex};"] #[doc = " use std::thread;"] #[doc = ""] #[doc = " let lock = Arc::new(Mutex::new(0_u32));"] #[doc = " let lock2 = Arc::clone(&lock);"] #[doc = ""] #[doc = " let _ = thread::spawn(move || -> () {"] #[doc = "     // This thread will acquire the mutex first, unwrapping the result of"] #[doc = "     // `lock` because the lock has not been poisoned."] #[doc = "     let _guard = lock2.lock().unwrap();"] #[doc = ""] #[doc = "     // This panic while holding the lock (`_guard` is in scope) will poison"] #[doc = "     // the mutex."] #[doc = "     panic!();"] #[doc = " }).join();"] #[doc = ""] #[doc = " // The lock is poisoned by this point, but the returned result can be"] #[doc = " // pattern matched on to return the underlying guard on both branches."] #[doc = " let mut guard = match lock.lock() {"] #[doc = "     Ok(guard) => guard,"] #[doc = "     Err(poisoned) => poisoned.into_inner(),"] #[doc = " };"] #[doc = ""] #[doc = " *guard += 1;"] #[doc = " ```"] #[doc = ""] #[doc = " To unlock a mutex guard sooner than the end of the enclosing scope,"] #[doc = " either create an inner scope or drop the guard manually."] #[doc = ""] #[doc = " ```"] #[doc = " use std::sync::{Arc, Mutex};"] #[doc = " use std::thread;"] #[doc = ""] #[doc = " const N: usize = 3;"] #[doc = ""] #[doc = " let data_mutex = Arc::new(Mutex::new(vec![1, 2, 3, 4]));"] #[doc = " let res_mutex = Arc::new(Mutex::new(0));"] #[doc = ""] #[doc = " let mut threads = Vec::with_capacity(N);"] #[doc = " (0..N).for_each(|_| {"] #[doc = "     let data_mutex_clone = Arc::clone(&data_mutex);"] #[doc = "     let res_mutex_clone = Arc::clone(&res_mutex);"] #[doc = ""] #[doc = "     threads.push(thread::spawn(move || {"] #[doc = "         // Here we use a block to limit the lifetime of the lock guard."] #[doc = "         let result = {"] #[doc = "             let mut data = data_mutex_clone.lock().unwrap();"] #[doc = "             // This is the result of some important and long-ish work."] #[doc = "             let result = data.iter().fold(0, |acc, x| acc + x * 2);"] #[doc = "             data.push(result);"] #[doc = "             result"] #[doc = "             // The mutex guard gets dropped here, together with any other values"] #[doc = "             // created in the critical section."] #[doc = "         };"] #[doc = "         // The guard created here is a temporary dropped at the end of the statement, i.e."] #[doc = "         // the lock would not remain being held even if the thread did some additional work."] #[doc = "         *res_mutex_clone.lock().unwrap() += result;"] #[doc = "     }));"] #[doc = " });"] #[doc = ""] #[doc = " let mut data = data_mutex.lock().unwrap();"] #[doc = " // This is the result of some important and long-ish work."] #[doc = " let result = data.iter().fold(0, |acc, x| acc + x * 2);"] #[doc = " data.push(result);"] #[doc = " // We drop the `data` explicitly because it's not necessary anymore and the"] #[doc = " // thread still has work to do. This allows other threads to start working on"] #[doc = " // the data immediately, without waiting for the rest of the unrelated work"] #[doc = " // to be done here."] #[doc = " //"] #[doc = " // It's even more important here than in the threads because we `.join` the"] #[doc = " // threads after that. If we had not dropped the mutex guard, a thread could"] #[doc = " // be waiting forever for it, causing a deadlock."] #[doc = " // As in the threads, a block could have been used instead of calling the"] #[doc = " // `drop` function."] #[doc = " drop(data);"] #[doc = " // Here the mutex guard is not assigned to a variable and so, even if the"] #[doc = " // scope does not end after this line, the mutex is still released: there is"] #[doc = " // no deadlock."] #[doc = " *res_mutex.lock().unwrap() += result;"] #[doc = ""] #[doc = " threads.into_iter().for_each(|thread| {"] #[doc = "     thread"] #[doc = "         .join()"] #[doc = "         .expect(\"The thread creating or execution failed !\")"] #[doc = " });"] #[doc = ""] #[doc = " assert_eq!(*res_mutex.lock().unwrap(), 800);"] #[doc = " ```"] #[doc = ""] #[stable (feature = "rust1" , since = "1.0.0")] #[cfg_attr (not (test) , rustc_diagnostic_item = "Mutex")] pub struct Mutex < T : ? Sized > { inner : sys :: Mutex , poison : poison :: Flag , data : UnsafeCell < T > , }}}
mkitem!{mkimpl!{#[doc = " `T` must be `Send` for a [`Mutex`] to be `Send` because it is possible to acquire"] #[doc = " the owned `T` from the `Mutex` via [`into_inner`]."] #[doc = ""] #[doc = " [`into_inner`]: Mutex::into_inner"] #[stable (feature = "rust1" , since = "1.0.0")] unsafe impl < T : ? Sized + Send > Send for Mutex < T > { }}}
mkitem!{mkimpl!{#[doc = " `T` must be `Send` for [`Mutex`] to be `Sync`."] #[doc = " This ensures that the protected data can be accessed safely from multiple threads"] #[doc = " without causing data races or other unsafe behavior."] #[doc = ""] #[doc = " [`Mutex<T>`] provides mutable access to `T` to one thread at a time. However, it's essential"] #[doc = " for `T` to be `Send` because it's not safe for non-`Send` structures to be accessed in"] #[doc = " this manner. For instance, consider [`Rc`], a non-atomic reference counted smart pointer,"] #[doc = " which is not `Send`. With `Rc`, we can have multiple copies pointing to the same heap"] #[doc = " allocation with a non-atomic reference count. If we were to use `Mutex<Rc<_>>`, it would"] #[doc = " only protect one instance of `Rc` from shared access, leaving other copies vulnerable"] #[doc = " to potential data races."] #[doc = ""] #[doc = " Also note that it is not necessary for `T` to be `Sync` as `&T` is only made available"] #[doc = " to one thread at a time if `T` is not `Sync`."] #[doc = ""] #[doc = " [`Rc`]: crate::rc::Rc"] #[stable (feature = "rust1" , since = "1.0.0")] unsafe impl < T : ? Sized + Send > Sync for Mutex < T > { }}}
mkitem!{mkstruct!{#[doc = " An RAII implementation of a \"scoped lock\" of a mutex. When this structure is"] #[doc = " dropped (falls out of scope), the lock will be unlocked."] #[doc = ""] #[doc = " The data protected by the mutex can be accessed through this guard via its"] #[doc = " [`Deref`] and [`DerefMut`] implementations."] #[doc = ""] #[doc = " This structure is created by the [`lock`] and [`try_lock`] methods on"] #[doc = " [`Mutex`]."] #[doc = ""] #[doc = " [`lock`]: Mutex::lock"] #[doc = " [`try_lock`]: Mutex::try_lock"] #[must_use = "if unused the Mutex will immediately unlock"] #[must_not_suspend = "holding a MutexGuard across suspend \
                      points can cause deadlocks, delays, \
                      and cause Futures to not implement `Send`"] #[stable (feature = "rust1" , since = "1.0.0")] #[clippy :: has_significant_drop] #[cfg_attr (not (test) , rustc_diagnostic_item = "MutexGuard")] pub struct MutexGuard < 'a , T : ? Sized + 'a > { lock : & 'a Mutex < T > , poison : poison :: Guard , }}}
mkitem!{mkimpl!{#[doc = " A [`MutexGuard`] is not `Send` to maximize platform portability."] #[doc = ""] #[doc = " On platforms that use POSIX threads (commonly referred to as pthreads) there is a requirement to"] #[doc = " release mutex locks on the same thread they were acquired."] #[doc = " For this reason, [`MutexGuard`] must not implement `Send` to prevent it being dropped from"] #[doc = " another thread."] #[stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized > ! Send for MutexGuard < '_ , T > { }}}
mkitem!{mkimpl!{#[doc = " `T` must be `Sync` for a [`MutexGuard<T>`] to be `Sync`"] #[doc = " because it is possible to get a `&T` from `&MutexGuard` (via `Deref`)."] #[stable (feature = "mutexguard" , since = "1.19.0")] unsafe impl < T : ? Sized + Sync > Sync for MutexGuard < '_ , T > { }}}
mkitem!{mkstruct!{#[doc = " An RAII mutex guard returned by `MutexGuard::map`, which can point to a"] #[doc = " subfield of the protected data. When this structure is dropped (falls out"] #[doc = " of scope), the lock will be unlocked."] #[doc = ""] #[doc = " The main difference between `MappedMutexGuard` and [`MutexGuard`] is that the"] #[doc = " former cannot be used with [`Condvar`], since that"] #[doc = " could introduce soundness issues if the locked object is modified by another"] #[doc = " thread while the `Mutex` is unlocked."] #[doc = ""] #[doc = " The data protected by the mutex can be accessed through this guard via its"] #[doc = " [`Deref`] and [`DerefMut`] implementations."] #[doc = ""] #[doc = " This structure is created by the [`map`] and [`filter_map`] methods on"] #[doc = " [`MutexGuard`]."] #[doc = ""] #[doc = " [`map`]: MutexGuard::map"] #[doc = " [`filter_map`]: MutexGuard::filter_map"] #[doc = " [`Condvar`]: crate::sync::Condvar"] #[must_use = "if unused the Mutex will immediately unlock"] #[must_not_suspend = "holding a MappedMutexGuard across suspend \
                      points can cause deadlocks, delays, \
                      and cause Futures to not implement `Send`"] #[unstable (feature = "mapped_lock_guards" , issue = "117108")] #[clippy :: has_significant_drop] pub struct MappedMutexGuard < 'a , T : ? Sized + 'a > { data : NonNull < T > , inner : & 'a sys :: Mutex , poison_flag : & 'a poison :: Flag , poison : poison :: Guard , _variance : PhantomData < & 'a mut T > , }}}
mkitem!{mkimpl!{#[unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized > ! Send for MappedMutexGuard < '_ , T > { }}}
mkitem!{mkimpl!{#[unstable (feature = "mapped_lock_guards" , issue = "117108")] unsafe impl < T : ? Sized + Sync > Sync for MappedMutexGuard < '_ , T > { }}}
mkitem!{mkimpl!{impl < T > Mutex < T > { #[doc = " Creates a new mutex in an unlocked state ready for use."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::sync::Mutex;"] #[doc = ""] #[doc = " let mutex = Mutex::new(0);"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] #[rustc_const_stable (feature = "const_locks" , since = "1.63.0")] #[inline] pub const fn new (t : T) -> Mutex < T > { Mutex { inner : sys :: Mutex :: new () , poison : poison :: Flag :: new () , data : UnsafeCell :: new (t) } } #[doc = " Returns the contained value by cloning it."] #[doc = ""] #[doc = " # Errors"] #[doc = ""] #[doc = " If another user of this mutex panicked while holding the mutex, then"] #[doc = " this call will return an error instead."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(lock_value_accessors)]"] #[doc = ""] #[doc = " use std::sync::Mutex;"] #[doc = ""] #[doc = " let mut mutex = Mutex::new(7);"] #[doc = ""] #[doc = " assert_eq!(mutex.get_cloned().unwrap(), 7);"] #[doc = " ```"] #[unstable (feature = "lock_value_accessors" , issue = "133407")] pub fn get_cloned (& self) -> Result < T , PoisonError < () > > where T : Clone , { match self . lock () { Ok (guard) => Ok ((* guard) . clone ()) , Err (_) => Err (PoisonError :: new (())) , } } #[doc = " Sets the contained value."] #[doc = ""] #[doc = " # Errors"] #[doc = ""] #[doc = " If another user of this mutex panicked while holding the mutex, then"] #[doc = " this call will return an error containing the provided `value` instead."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(lock_value_accessors)]"] #[doc = ""] #[doc = " use std::sync::Mutex;"] #[doc = ""] #[doc = " let mut mutex = Mutex::new(7);"] #[doc = ""] #[doc = " assert_eq!(mutex.get_cloned().unwrap(), 7);"] #[doc = " mutex.set(11).unwrap();"] #[doc = " assert_eq!(mutex.get_cloned().unwrap(), 11);"] #[doc = " ```"] #[unstable (feature = "lock_value_accessors" , issue = "133407")] pub fn set (& self , value : T) -> Result < () , PoisonError < T > > { if mem :: needs_drop :: < T > () { self . replace (value) . map (drop) } else { match self . lock () { Ok (mut guard) => { * guard = value ; Ok (()) } Err (_) => Err (PoisonError :: new (value)) , } } } #[doc = " Replaces the contained value with `value`, and returns the old contained value."] #[doc = ""] #[doc = " # Errors"] #[doc = ""] #[doc = " If another user of this mutex panicked while holding the mutex, then"] #[doc = " this call will return an error containing the provided `value` instead."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(lock_value_accessors)]"] #[doc = ""] #[doc = " use std::sync::Mutex;"] #[doc = ""] #[doc = " let mut mutex = Mutex::new(7);"] #[doc = ""] #[doc = " assert_eq!(mutex.replace(11).unwrap(), 7);"] #[doc = " assert_eq!(mutex.get_cloned().unwrap(), 11);"] #[doc = " ```"] #[unstable (feature = "lock_value_accessors" , issue = "133407")] pub fn replace (& self , value : T) -> LockResult < T > { match self . lock () { Ok (mut guard) => Ok (mem :: replace (& mut * guard , value)) , Err (_) => Err (PoisonError :: new (value)) , } } }}}
mkitem!{mkimpl!{impl < T : ? Sized > Mutex < T > { #[doc = " Acquires a mutex, blocking the current thread until it is able to do so."] #[doc = ""] #[doc = " This function will block the local thread until it is available to acquire"] #[doc = " the mutex. Upon returning, the thread is the only thread with the lock"] #[doc = " held. An RAII guard is returned to allow scoped unlock of the lock. When"] #[doc = " the guard goes out of scope, the mutex will be unlocked."] #[doc = ""] #[doc = " The exact behavior on locking a mutex in the thread which already holds"] #[doc = " the lock is left unspecified. However, this function will not return on"] #[doc = " the second call (it might panic or deadlock, for example)."] #[doc = ""] #[doc = " # Errors"] #[doc = ""] #[doc = " If another user of this mutex panicked while holding the mutex, then"] #[doc = " this call will return an error once the mutex is acquired. The acquired"] #[doc = " mutex guard will be contained in the returned error."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc = " This function might panic when called if the lock is already held by"] #[doc = " the current thread."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::sync::{Arc, Mutex};"] #[doc = " use std::thread;"] #[doc = ""] #[doc = " let mutex = Arc::new(Mutex::new(0));"] #[doc = " let c_mutex = Arc::clone(&mutex);"] #[doc = ""] #[doc = " thread::spawn(move || {"] #[doc = "     *c_mutex.lock().unwrap() = 10;"] #[doc = " }).join().expect(\"thread::spawn failed\");"] #[doc = " assert_eq!(*mutex.lock().unwrap(), 10);"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] pub fn lock (& self) -> LockResult < MutexGuard < '_ , T > > { unsafe { self . inner . lock () ; MutexGuard :: new (self) } } #[doc = " Attempts to acquire this lock."] #[doc = ""] #[doc = " If the lock could not be acquired at this time, then [`Err`] is returned."] #[doc = " Otherwise, an RAII guard is returned. The lock will be unlocked when the"] #[doc = " guard is dropped."] #[doc = ""] #[doc = " This function does not block."] #[doc = ""] #[doc = " # Errors"] #[doc = ""] #[doc = " If another user of this mutex panicked while holding the mutex, then"] #[doc = " this call will return the [`Poisoned`] error if the mutex would"] #[doc = " otherwise be acquired. An acquired lock guard will be contained"] #[doc = " in the returned error."] #[doc = ""] #[doc = " If the mutex could not be acquired because it is already locked, then"] #[doc = " this call will return the [`WouldBlock`] error."] #[doc = ""] #[doc = " [`Poisoned`]: TryLockError::Poisoned"] #[doc = " [`WouldBlock`]: TryLockError::WouldBlock"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::sync::{Arc, Mutex};"] #[doc = " use std::thread;"] #[doc = ""] #[doc = " let mutex = Arc::new(Mutex::new(0));"] #[doc = " let c_mutex = Arc::clone(&mutex);"] #[doc = ""] #[doc = " thread::spawn(move || {"] #[doc = "     let mut lock = c_mutex.try_lock();"] #[doc = "     if let Ok(ref mut mutex) = lock {"] #[doc = "         **mutex = 10;"] #[doc = "     } else {"] #[doc = "         println!(\"try_lock failed\");"] #[doc = "     }"] #[doc = " }).join().expect(\"thread::spawn failed\");"] #[doc = " assert_eq!(*mutex.lock().unwrap(), 10);"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] pub fn try_lock (& self) -> TryLockResult < MutexGuard < '_ , T > > { unsafe { if self . inner . try_lock () { Ok (MutexGuard :: new (self) ?) } else { Err (TryLockError :: WouldBlock) } } } #[doc = " Determines whether the mutex is poisoned."] #[doc = ""] #[doc = " If another thread is active, the mutex can still become poisoned at any"] #[doc = " time. You should not trust a `false` value for program correctness"] #[doc = " without additional synchronization."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::sync::{Arc, Mutex};"] #[doc = " use std::thread;"] #[doc = ""] #[doc = " let mutex = Arc::new(Mutex::new(0));"] #[doc = " let c_mutex = Arc::clone(&mutex);"] #[doc = ""] #[doc = " let _ = thread::spawn(move || {"] #[doc = "     let _lock = c_mutex.lock().unwrap();"] #[doc = "     panic!(); // the mutex gets poisoned"] #[doc = " }).join();"] #[doc = " assert_eq!(mutex.is_poisoned(), true);"] #[doc = " ```"] #[inline] #[stable (feature = "sync_poison" , since = "1.2.0")] pub fn is_poisoned (& self) -> bool { self . poison . get () } #[doc = " Clear the poisoned state from a mutex."] #[doc = ""] #[doc = " If the mutex is poisoned, it will remain poisoned until this function is called. This"] #[doc = " allows recovering from a poisoned state and marking that it has recovered. For example, if"] #[doc = " the value is overwritten by a known-good value, then the mutex can be marked as"] #[doc = " un-poisoned. Or possibly, the value could be inspected to determine if it is in a"] #[doc = " consistent state, and if so the poison is removed."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::sync::{Arc, Mutex};"] #[doc = " use std::thread;"] #[doc = ""] #[doc = " let mutex = Arc::new(Mutex::new(0));"] #[doc = " let c_mutex = Arc::clone(&mutex);"] #[doc = ""] #[doc = " let _ = thread::spawn(move || {"] #[doc = "     let _lock = c_mutex.lock().unwrap();"] #[doc = "     panic!(); // the mutex gets poisoned"] #[doc = " }).join();"] #[doc = ""] #[doc = " assert_eq!(mutex.is_poisoned(), true);"] #[doc = " let x = mutex.lock().unwrap_or_else(|mut e| {"] #[doc = "     **e.get_mut() = 1;"] #[doc = "     mutex.clear_poison();"] #[doc = "     e.into_inner()"] #[doc = " });"] #[doc = " assert_eq!(mutex.is_poisoned(), false);"] #[doc = " assert_eq!(*x, 1);"] #[doc = " ```"] #[inline] #[stable (feature = "mutex_unpoison" , since = "1.77.0")] pub fn clear_poison (& self) { self . poison . clear () ; } #[doc = " Consumes this mutex, returning the underlying data."] #[doc = ""] #[doc = " # Errors"] #[doc = ""] #[doc = " If another user of this mutex panicked while holding the mutex, then"] #[doc = " this call will return an error containing the underlying data"] #[doc = " instead."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::sync::Mutex;"] #[doc = ""] #[doc = " let mutex = Mutex::new(0);"] #[doc = " assert_eq!(mutex.into_inner().unwrap(), 0);"] #[doc = " ```"] #[stable (feature = "mutex_into_inner" , since = "1.6.0")] pub fn into_inner (self) -> LockResult < T > where T : Sized , { let data = self . data . into_inner () ; poison :: map_result (self . poison . borrow () , | () | data) } #[doc = " Returns a mutable reference to the underlying data."] #[doc = ""] #[doc = " Since this call borrows the `Mutex` mutably, no actual locking needs to"] #[doc = " take place -- the mutable borrow statically guarantees no new locks can be acquired"] #[doc = " while this reference exists. Note that this method does not clear any previous abandoned locks"] #[doc = " (e.g., via [`forget()`] on a [`MutexGuard`])."] #[doc = ""] #[doc = " # Errors"] #[doc = ""] #[doc = " If another user of this mutex panicked while holding the mutex, then"] #[doc = " this call will return an error containing a mutable reference to the"] #[doc = " underlying data instead."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::sync::Mutex;"] #[doc = ""] #[doc = " let mut mutex = Mutex::new(0);"] #[doc = " *mutex.get_mut().unwrap() = 10;"] #[doc = " assert_eq!(*mutex.lock().unwrap(), 10);"] #[doc = " ```"] #[doc = ""] #[doc = " [`forget()`]: mem::forget"] #[stable (feature = "mutex_get_mut" , since = "1.6.0")] pub fn get_mut (& mut self) -> LockResult < & mut T > { let data = self . data . get_mut () ; poison :: map_result (self . poison . borrow () , | () | data) } #[doc = " Returns a raw pointer to the underlying data."] #[doc = ""] #[doc = " The returned pointer is always non-null and properly aligned, but it is"] #[doc = " the user's responsibility to ensure that any reads and writes through it"] #[doc = " are properly synchronized to avoid data races, and that it is not read"] #[doc = " or written through after the mutex is dropped."] #[unstable (feature = "mutex_data_ptr" , issue = "140368")] pub fn data_ptr (& self) -> * mut T { self . data . get () } }}}
mkitem!{mkimpl!{#[stable (feature = "mutex_from" , since = "1.24.0")] impl < T > From < T > for Mutex < T > { #[doc = " Creates a new mutex in an unlocked state ready for use."] #[doc = " This is equivalent to [`Mutex::new`]."] fn from (t : T) -> Self { Mutex :: new (t) } }}}
mkitem!{mkimpl!{#[stable (feature = "mutex_default" , since = "1.10.0")] impl < T : ? Sized + Default > Default for Mutex < T > { #[doc = " Creates a `Mutex<T>`, with the `Default` value for T."] fn default () -> Mutex < T > { Mutex :: new (Default :: default ()) } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized + fmt :: Debug > fmt :: Debug for Mutex < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_struct ("Mutex") ; match self . try_lock () { Ok (guard) => { d . field ("data" , & & * guard) ; } Err (TryLockError :: Poisoned (err)) => { d . field ("data" , & & * * err . get_ref ()) ; } Err (TryLockError :: WouldBlock) => { d . field ("data" , & "<locked>") ; } } d . field ("poisoned" , & self . poison . get ()) ; d . finish_non_exhaustive () } }}}
mkitem!{mkimpl!{impl < 'mutex , T : ? Sized > MutexGuard < 'mutex , T > { unsafe fn new (lock : & 'mutex Mutex < T >) -> LockResult < MutexGuard < 'mutex , T > > { poison :: map_result (lock . poison . guard () , | guard | MutexGuard { lock , poison : guard }) } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized > Deref for MutexGuard < '_ , T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * self . lock . data . get () } } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized > DerefMut for MutexGuard < '_ , T > { fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . lock . data . get () } } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized > Drop for MutexGuard < '_ , T > { #[inline] fn drop (& mut self) { unsafe { self . lock . poison . done (& self . poison) ; self . lock . inner . unlock () ; } } }}}
mkitem!{mkimpl!{#[stable (feature = "std_debug" , since = "1.16.0")] impl < T : ? Sized + fmt :: Debug > fmt :: Debug for MutexGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }}}
mkitem!{mkimpl!{#[stable (feature = "std_guard_impls" , since = "1.20.0")] impl < T : ? Sized + fmt :: Display > fmt :: Display for MutexGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }}}

macro_rules! guard_lock_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function guard_lock in module {}", module_path!());
    };
}

mkfn!{
    guard_lock_introspect!();
    #[doc = " For use in [`nonpoison::condvar`](super::condvar)."] pub (super) fn guard_lock < 'a , T : ? Sized > (guard : & MutexGuard < 'a , T >) -> & 'a sys :: Mutex { & guard . lock . inner }
}

macro_rules! guard_poison_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function guard_poison in module {}", module_path!());
    };
}

mkfn!{
    guard_poison_introspect!();
    #[doc = " For use in [`nonpoison::condvar`](super::condvar)."] pub (super) fn guard_poison < 'a , T : ? Sized > (guard : & MutexGuard < 'a , T >) -> & 'a poison :: Flag { & guard . lock . poison }
}
mkitem!{mkimpl!{impl < 'a , T : ? Sized > MutexGuard < 'a , T > { #[doc = " Makes a [`MappedMutexGuard`] for a component of the borrowed data, e.g."] #[doc = " an enum variant."] #[doc = ""] #[doc = " The `Mutex` is already locked, so this cannot fail."] #[doc = ""] #[doc = " This is an associated function that needs to be used as"] #[doc = " `MutexGuard::map(...)`. A method would interfere with methods of the"] #[doc = " same name on the contents of the `MutexGuard` used through `Deref`."] #[unstable (feature = "mapped_lock_guards" , issue = "117108")] pub fn map < U , F > (orig : Self , f : F) -> MappedMutexGuard < 'a , U > where F : FnOnce (& mut T) -> & mut U , U : ? Sized , { let data = NonNull :: from (f (unsafe { & mut * orig . lock . data . get () })) ; let orig = ManuallyDrop :: new (orig) ; MappedMutexGuard { data , inner : & orig . lock . inner , poison_flag : & orig . lock . poison , poison : orig . poison . clone () , _variance : PhantomData , } } #[doc = " Makes a [`MappedMutexGuard`] for a component of the borrowed data. The"] #[doc = " original guard is returned as an `Err(...)` if the closure returns"] #[doc = " `None`."] #[doc = ""] #[doc = " The `Mutex` is already locked, so this cannot fail."] #[doc = ""] #[doc = " This is an associated function that needs to be used as"] #[doc = " `MutexGuard::filter_map(...)`. A method would interfere with methods of the"] #[doc = " same name on the contents of the `MutexGuard` used through `Deref`."] #[unstable (feature = "mapped_lock_guards" , issue = "117108")] pub fn filter_map < U , F > (orig : Self , f : F) -> Result < MappedMutexGuard < 'a , U > , Self > where F : FnOnce (& mut T) -> Option < & mut U > , U : ? Sized , { match f (unsafe { & mut * orig . lock . data . get () }) { Some (data) => { let data = NonNull :: from (data) ; let orig = ManuallyDrop :: new (orig) ; Ok (MappedMutexGuard { data , inner : & orig . lock . inner , poison_flag : & orig . lock . poison , poison : orig . poison . clone () , _variance : PhantomData , }) } None => Err (orig) , } } }}}
mkitem!{mkimpl!{#[unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized > Deref for MappedMutexGuard < '_ , T > { type Target = T ; fn deref (& self) -> & T { unsafe { self . data . as_ref () } } }}}
mkitem!{mkimpl!{#[unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized > DerefMut for MappedMutexGuard < '_ , T > { fn deref_mut (& mut self) -> & mut T { unsafe { self . data . as_mut () } } }}}
mkitem!{mkimpl!{#[unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized > Drop for MappedMutexGuard < '_ , T > { #[inline] fn drop (& mut self) { unsafe { self . poison_flag . done (& self . poison) ; self . inner . unlock () ; } } }}}
mkitem!{mkimpl!{#[unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized + fmt :: Debug > fmt :: Debug for MappedMutexGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }}}
mkitem!{mkimpl!{#[unstable (feature = "mapped_lock_guards" , issue = "117108")] impl < T : ? Sized + fmt :: Display > fmt :: Display for MappedMutexGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }}}
mkitem!{mkimpl!{impl < 'a , T : ? Sized > MappedMutexGuard < 'a , T > { #[doc = " Makes a [`MappedMutexGuard`] for a component of the borrowed data, e.g."] #[doc = " an enum variant."] #[doc = ""] #[doc = " The `Mutex` is already locked, so this cannot fail."] #[doc = ""] #[doc = " This is an associated function that needs to be used as"] #[doc = " `MappedMutexGuard::map(...)`. A method would interfere with methods of the"] #[doc = " same name on the contents of the `MutexGuard` used through `Deref`."] #[unstable (feature = "mapped_lock_guards" , issue = "117108")] pub fn map < U , F > (mut orig : Self , f : F) -> MappedMutexGuard < 'a , U > where F : FnOnce (& mut T) -> & mut U , U : ? Sized , { let data = NonNull :: from (f (unsafe { orig . data . as_mut () })) ; let orig = ManuallyDrop :: new (orig) ; MappedMutexGuard { data , inner : orig . inner , poison_flag : orig . poison_flag , poison : orig . poison . clone () , _variance : PhantomData , } } #[doc = " Makes a [`MappedMutexGuard`] for a component of the borrowed data. The"] #[doc = " original guard is returned as an `Err(...)` if the closure returns"] #[doc = " `None`."] #[doc = ""] #[doc = " The `Mutex` is already locked, so this cannot fail."] #[doc = ""] #[doc = " This is an associated function that needs to be used as"] #[doc = " `MappedMutexGuard::filter_map(...)`. A method would interfere with methods of the"] #[doc = " same name on the contents of the `MutexGuard` used through `Deref`."] #[unstable (feature = "mapped_lock_guards" , issue = "117108")] pub fn filter_map < U , F > (mut orig : Self , f : F) -> Result < MappedMutexGuard < 'a , U > , Self > where F : FnOnce (& mut T) -> Option < & mut U > , U : ? Sized , { match f (unsafe { orig . data . as_mut () }) { Some (data) => { let data = NonNull :: from (data) ; let orig = ManuallyDrop :: new (orig) ; Ok (MappedMutexGuard { data , inner : orig . inner , poison_flag : orig . poison_flag , poison : orig . poison . clone () , _variance : PhantomData , }) } None => Err (orig) , } } }}}