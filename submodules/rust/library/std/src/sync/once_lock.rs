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
mkuse!{use crate :: mem :: MaybeUninit ;}
mkuse!{use crate :: panic :: { RefUnwindSafe , UnwindSafe } ;}
mkuse!{use crate :: sync :: Once ;}
mkitem!{mkstruct!{# [doc = " A synchronization primitive which can nominally be written to only once."] # [doc = ""] # [doc = " This type is a thread-safe [`OnceCell`], and can be used in statics."] # [doc = " In many simple cases, you can use [`LazyLock<T, F>`] instead to get the benefits of this type"] # [doc = " with less effort: `LazyLock<T, F>` \"looks like\" `&T` because it initializes with `F` on deref!"] # [doc = " Where OnceLock shines is when LazyLock is too simple to support a given case, as LazyLock"] # [doc = " doesn't allow additional inputs to its function after you call [`LazyLock::new(|| ...)`]."] # [doc = ""] # [doc = " A `OnceLock` can be thought of as a safe abstraction over uninitialized data that becomes"] # [doc = " initialized once written."] # [doc = ""] # [doc = " Unlike [`Mutex`](crate::sync::Mutex), `OnceLock` is never poisoned on panic."] # [doc = ""] # [doc = " [`OnceCell`]: crate::cell::OnceCell"] # [doc = " [`LazyLock<T, F>`]: crate::sync::LazyLock"] # [doc = " [`LazyLock::new(|| ...)`]: crate::sync::LazyLock::new"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Writing to a `OnceLock` from a separate thread:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::OnceLock;"] # [doc = ""] # [doc = " static CELL: OnceLock<usize> = OnceLock::new();"] # [doc = ""] # [doc = " // `OnceLock` has not been written to yet."] # [doc = " assert!(CELL.get().is_none());"] # [doc = ""] # [doc = " // Spawn a thread and write to `OnceLock`."] # [doc = " std::thread::spawn(|| {"] # [doc = "     let value = CELL.get_or_init(|| 12345);"] # [doc = "     assert_eq!(value, &12345);"] # [doc = " })"] # [doc = " .join()"] # [doc = " .unwrap();"] # [doc = ""] # [doc = " // `OnceLock` now contains the value."] # [doc = " assert_eq!("] # [doc = "     CELL.get(),"] # [doc = "     Some(&12345),"] # [doc = " );"] # [doc = " ```"] # [doc = ""] # [doc = " You can use `OnceLock` to implement a type that requires \"append-only\" logic:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::{OnceLock, atomic::{AtomicU32, Ordering}};"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " struct OnceList<T> {"] # [doc = "     data: OnceLock<T>,"] # [doc = "     next: OnceLock<Box<OnceList<T>>>,"] # [doc = " }"] # [doc = " impl<T> OnceList<T> {"] # [doc = "     const fn new() -> OnceList<T> {"] # [doc = "         OnceList { data: OnceLock::new(), next: OnceLock::new() }"] # [doc = "     }"] # [doc = "     fn push(&self, value: T) {"] # [doc = "         // FIXME: this impl is concise, but is also slow for long lists or many threads."] # [doc = "         // as an exercise, consider how you might improve on it while preserving the behavior"] # [doc = "         if let Err(value) = self.data.set(value) {"] # [doc = "             let next = self.next.get_or_init(|| Box::new(OnceList::new()));"] # [doc = "             next.push(value)"] # [doc = "         };"] # [doc = "     }"] # [doc = "     fn contains(&self, example: &T) -> bool"] # [doc = "     where"] # [doc = "         T: PartialEq,"] # [doc = "     {"] # [doc = "         self.data.get().map(|item| item == example).filter(|v| *v).unwrap_or_else(|| {"] # [doc = "             self.next.get().map(|next| next.contains(example)).unwrap_or(false)"] # [doc = "         })"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " // Let's exercise this new Sync append-only list by doing a little counting"] # [doc = " static LIST: OnceList<u32> = OnceList::new();"] # [doc = " static COUNTER: AtomicU32 = AtomicU32::new(0);"] # [doc = ""] # [doc = " # const LEN: u32 = if cfg!(miri) { 50 } else { 1000 };"] # [doc = " # /*"] # [doc = " const LEN: u32 = 1000;"] # [doc = " # */"] # [doc = " thread::scope(|s| {"] # [doc = "     for _ in 0..thread::available_parallelism().unwrap().get() {"] # [doc = "         s.spawn(|| {"] # [doc = "             while let i @ 0..LEN = COUNTER.fetch_add(1, Ordering::Relaxed) {"] # [doc = "                 LIST.push(i);"] # [doc = "             }"] # [doc = "         });"] # [doc = "     }"] # [doc = " });"] # [doc = ""] # [doc = " for i in 0..LEN {"] # [doc = "     assert!(LIST.contains(&i));"] # [doc = " }"] # [doc = ""] # [doc = " ```"] # [stable (feature = "once_cell" , since = "1.70.0")] pub struct OnceLock < T > { once : Once , value : UnsafeCell < MaybeUninit < T > > , # [doc = " `PhantomData` to make sure dropck understands we're dropping T in our Drop impl."] # [doc = ""] # [doc = " ```compile_fail,E0597"] # [doc = " use std::sync::OnceLock;"] # [doc = ""] # [doc = " struct A<'a>(&'a str);"] # [doc = ""] # [doc = " impl<'a> Drop for A<'a> {"] # [doc = "     fn drop(&mut self) {}"] # [doc = " }"] # [doc = ""] # [doc = " let cell = OnceLock::new();"] # [doc = " {"] # [doc = "     let s = String::new();"] # [doc = "     let _ = cell.set(A(&s));"] # [doc = " }"] # [doc = " ```"] _marker : PhantomData < T > , }}}
mkitem!{mkimpl!{impl < T > OnceLock < T > { # [doc = " Creates a new uninitialized cell."] # [inline] # [must_use] # [stable (feature = "once_cell" , since = "1.70.0")] # [rustc_const_stable (feature = "once_cell" , since = "1.70.0")] pub const fn new () -> OnceLock < T > { OnceLock { once : Once :: new () , value : UnsafeCell :: new (MaybeUninit :: uninit ()) , _marker : PhantomData , } } # [doc = " Gets the reference to the underlying value."] # [doc = ""] # [doc = " Returns `None` if the cell is uninitialized, or being initialized."] # [doc = " This method never blocks."] # [inline] # [stable (feature = "once_cell" , since = "1.70.0")] pub fn get (& self) -> Option < & T > { if self . is_initialized () { Some (unsafe { self . get_unchecked () }) } else { None } } # [doc = " Gets the mutable reference to the underlying value."] # [doc = ""] # [doc = " Returns `None` if the cell is uninitialized."] # [doc = ""] # [doc = " This method never blocks. Since it borrows the `OnceLock` mutably,"] # [doc = " it is statically guaranteed that no active borrows to the `OnceLock`"] # [doc = " exist, including from other threads."] # [inline] # [stable (feature = "once_cell" , since = "1.70.0")] pub fn get_mut (& mut self) -> Option < & mut T > { if self . is_initialized () { Some (unsafe { self . get_unchecked_mut () }) } else { None } } # [doc = " Blocks the current thread until the cell is initialized."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Waiting for a computation on another thread to finish:"] # [doc = " ```rust"] # [doc = " use std::thread;"] # [doc = " use std::sync::OnceLock;"] # [doc = ""] # [doc = " let value = OnceLock::new();"] # [doc = ""] # [doc = " thread::scope(|s| {"] # [doc = "     s.spawn(|| value.set(1 + 1));"] # [doc = ""] # [doc = "     let result = value.wait();"] # [doc = "     assert_eq!(result, &2);"] # [doc = " })"] # [doc = " ```"] # [inline] # [stable (feature = "once_wait" , since = "1.86.0")] pub fn wait (& self) -> & T { self . once . wait_force () ; unsafe { self . get_unchecked () } } # [doc = " Initializes the contents of the cell to `value`."] # [doc = ""] # [doc = " May block if another thread is currently attempting to initialize the cell. The cell is"] # [doc = " guaranteed to contain a value when `set` returns, though not necessarily the one provided."] # [doc = ""] # [doc = " Returns `Ok(())` if the cell was uninitialized and"] # [doc = " `Err(value)` if the cell was already initialized."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::OnceLock;"] # [doc = ""] # [doc = " static CELL: OnceLock<i32> = OnceLock::new();"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     assert!(CELL.get().is_none());"] # [doc = ""] # [doc = "     std::thread::spawn(|| {"] # [doc = "         assert_eq!(CELL.set(92), Ok(()));"] # [doc = "     }).join().unwrap();"] # [doc = ""] # [doc = "     assert_eq!(CELL.set(62), Err(62));"] # [doc = "     assert_eq!(CELL.get(), Some(&92));"] # [doc = " }"] # [doc = " ```"] # [inline] # [stable (feature = "once_cell" , since = "1.70.0")] pub fn set (& self , value : T) -> Result < () , T > { match self . try_insert (value) { Ok (_) => Ok (()) , Err ((_ , value)) => Err (value) , } } # [doc = " Initializes the contents of the cell to `value` if the cell was uninitialized,"] # [doc = " then returns a reference to it."] # [doc = ""] # [doc = " May block if another thread is currently attempting to initialize the cell. The cell is"] # [doc = " guaranteed to contain a value when `try_insert` returns, though not necessarily the"] # [doc = " one provided."] # [doc = ""] # [doc = " Returns `Ok(&value)` if the cell was uninitialized and"] # [doc = " `Err((&current_value, value))` if it was already initialized."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(once_cell_try_insert)]"] # [doc = ""] # [doc = " use std::sync::OnceLock;"] # [doc = ""] # [doc = " static CELL: OnceLock<i32> = OnceLock::new();"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     assert!(CELL.get().is_none());"] # [doc = ""] # [doc = "     std::thread::spawn(|| {"] # [doc = "         assert_eq!(CELL.try_insert(92), Ok(&92));"] # [doc = "     }).join().unwrap();"] # [doc = ""] # [doc = "     assert_eq!(CELL.try_insert(62), Err((&92, 62)));"] # [doc = "     assert_eq!(CELL.get(), Some(&92));"] # [doc = " }"] # [doc = " ```"] # [inline] # [unstable (feature = "once_cell_try_insert" , issue = "116693")] pub fn try_insert (& self , value : T) -> Result < & T , (& T , T) > { let mut value = Some (value) ; let res = self . get_or_init (| | value . take () . unwrap ()) ; match value { None => Ok (res) , Some (value) => Err ((res , value)) , } } # [doc = " Gets the contents of the cell, initializing it to `f()` if the cell"] # [doc = " was uninitialized."] # [doc = ""] # [doc = " Many threads may call `get_or_init` concurrently with different"] # [doc = " initializing functions, but it is guaranteed that only one function"] # [doc = " will be executed if the function doesn't panic."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If `f()` panics, the panic is propagated to the caller, and the cell"] # [doc = " remains uninitialized."] # [doc = ""] # [doc = " It is an error to reentrantly initialize the cell from `f`. The"] # [doc = " exact outcome is unspecified. Current implementation deadlocks, but"] # [doc = " this may be changed to a panic in the future."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::OnceLock;"] # [doc = ""] # [doc = " let cell = OnceLock::new();"] # [doc = " let value = cell.get_or_init(|| 92);"] # [doc = " assert_eq!(value, &92);"] # [doc = " let value = cell.get_or_init(|| unreachable!());"] # [doc = " assert_eq!(value, &92);"] # [doc = " ```"] # [inline] # [stable (feature = "once_cell" , since = "1.70.0")] pub fn get_or_init < F > (& self , f : F) -> & T where F : FnOnce () -> T , { match self . get_or_try_init (| | Ok :: < T , ! > (f ())) { Ok (val) => val , } } # [doc = " Gets the mutable reference of the contents of the cell, initializing"] # [doc = " it to `f()` if the cell was uninitialized."] # [doc = ""] # [doc = " This method never blocks. Since it borrows the `OnceLock` mutably,"] # [doc = " it is statically guaranteed that no active borrows to the `OnceLock`"] # [doc = " exist, including from other threads."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If `f()` panics, the panic is propagated to the caller, and the cell"] # [doc = " remains uninitialized."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(once_cell_get_mut)]"] # [doc = ""] # [doc = " use std::sync::OnceLock;"] # [doc = ""] # [doc = " let mut cell = OnceLock::new();"] # [doc = " let value = cell.get_mut_or_init(|| 92);"] # [doc = " assert_eq!(*value, 92);"] # [doc = ""] # [doc = " *value += 2;"] # [doc = " assert_eq!(*value, 94);"] # [doc = ""] # [doc = " let value = cell.get_mut_or_init(|| unreachable!());"] # [doc = " assert_eq!(*value, 94);"] # [doc = " ```"] # [inline] # [unstable (feature = "once_cell_get_mut" , issue = "121641")] pub fn get_mut_or_init < F > (& mut self , f : F) -> & mut T where F : FnOnce () -> T , { match self . get_mut_or_try_init (| | Ok :: < T , ! > (f ())) { Ok (val) => val , } } # [doc = " Gets the contents of the cell, initializing it to `f()` if"] # [doc = " the cell was uninitialized. If the cell was uninitialized"] # [doc = " and `f()` failed, an error is returned."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If `f()` panics, the panic is propagated to the caller, and"] # [doc = " the cell remains uninitialized."] # [doc = ""] # [doc = " It is an error to reentrantly initialize the cell from `f`."] # [doc = " The exact outcome is unspecified. Current implementation"] # [doc = " deadlocks, but this may be changed to a panic in the future."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(once_cell_try)]"] # [doc = ""] # [doc = " use std::sync::OnceLock;"] # [doc = ""] # [doc = " let cell = OnceLock::new();"] # [doc = " assert_eq!(cell.get_or_try_init(|| Err(())), Err(()));"] # [doc = " assert!(cell.get().is_none());"] # [doc = " let value = cell.get_or_try_init(|| -> Result<i32, ()> {"] # [doc = "     Ok(92)"] # [doc = " });"] # [doc = " assert_eq!(value, Ok(&92));"] # [doc = " assert_eq!(cell.get(), Some(&92))"] # [doc = " ```"] # [inline] # [unstable (feature = "once_cell_try" , issue = "109737")] pub fn get_or_try_init < F , E > (& self , f : F) -> Result < & T , E > where F : FnOnce () -> Result < T , E > , { if let Some (value) = self . get () { return Ok (value) ; } self . initialize (f) ? ; debug_assert ! (self . is_initialized ()) ; Ok (unsafe { self . get_unchecked () }) } # [doc = " Gets the mutable reference of the contents of the cell, initializing"] # [doc = " it to `f()` if the cell was uninitialized. If the cell was uninitialized"] # [doc = " and `f()` failed, an error is returned."] # [doc = ""] # [doc = " This method never blocks. Since it borrows the `OnceLock` mutably,"] # [doc = " it is statically guaranteed that no active borrows to the `OnceLock`"] # [doc = " exist, including from other threads."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If `f()` panics, the panic is propagated to the caller, and"] # [doc = " the cell remains uninitialized."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(once_cell_get_mut)]"] # [doc = ""] # [doc = " use std::sync::OnceLock;"] # [doc = ""] # [doc = " let mut cell: OnceLock<u32> = OnceLock::new();"] # [doc = ""] # [doc = " // Failed attempts to initialize the cell do not change its contents"] # [doc = " assert!(cell.get_mut_or_try_init(|| \"not a number!\".parse()).is_err());"] # [doc = " assert!(cell.get().is_none());"] # [doc = ""] # [doc = " let value = cell.get_mut_or_try_init(|| \"1234\".parse());"] # [doc = " assert_eq!(value, Ok(&mut 1234));"] # [doc = " *value.unwrap() += 2;"] # [doc = " assert_eq!(cell.get(), Some(&1236))"] # [doc = " ```"] # [inline] # [unstable (feature = "once_cell_get_mut" , issue = "121641")] pub fn get_mut_or_try_init < F , E > (& mut self , f : F) -> Result < & mut T , E > where F : FnOnce () -> Result < T , E > , { if self . get () . is_none () { self . initialize (f) ? ; } debug_assert ! (self . is_initialized ()) ; Ok (unsafe { self . get_unchecked_mut () }) } # [doc = " Consumes the `OnceLock`, returning the wrapped value. Returns"] # [doc = " `None` if the cell was uninitialized."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::OnceLock;"] # [doc = ""] # [doc = " let cell: OnceLock<String> = OnceLock::new();"] # [doc = " assert_eq!(cell.into_inner(), None);"] # [doc = ""] # [doc = " let cell = OnceLock::new();"] # [doc = " cell.set(\"hello\".to_string()).unwrap();"] # [doc = " assert_eq!(cell.into_inner(), Some(\"hello\".to_string()));"] # [doc = " ```"] # [inline] # [stable (feature = "once_cell" , since = "1.70.0")] pub fn into_inner (mut self) -> Option < T > { self . take () } # [doc = " Takes the value out of this `OnceLock`, moving it back to an uninitialized state."] # [doc = ""] # [doc = " Has no effect and returns `None` if the `OnceLock` was uninitialized."] # [doc = ""] # [doc = " Since this method borrows the `OnceLock` mutably, it is statically guaranteed that"] # [doc = " no active borrows to the `OnceLock` exist, including from other threads."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::OnceLock;"] # [doc = ""] # [doc = " let mut cell: OnceLock<String> = OnceLock::new();"] # [doc = " assert_eq!(cell.take(), None);"] # [doc = ""] # [doc = " let mut cell = OnceLock::new();"] # [doc = " cell.set(\"hello\".to_string()).unwrap();"] # [doc = " assert_eq!(cell.take(), Some(\"hello\".to_string()));"] # [doc = " assert_eq!(cell.get(), None);"] # [doc = " ```"] # [inline] # [stable (feature = "once_cell" , since = "1.70.0")] pub fn take (& mut self) -> Option < T > { if self . is_initialized () { self . once = Once :: new () ; unsafe { Some ((& mut * self . value . get ()) . assume_init_read ()) } } else { None } } # [inline] fn is_initialized (& self) -> bool { self . once . is_completed () } # [cold] # [optimize (size)] fn initialize < F , E > (& self , f : F) -> Result < () , E > where F : FnOnce () -> Result < T , E > , { let mut res : Result < () , E > = Ok (()) ; let slot = & self . value ; self . once . call_once_force (| p | { match f () { Ok (value) => { unsafe { (& mut * slot . get ()) . write (value) } ; } Err (e) => { res = Err (e) ; p . poison () ; } } }) ; res } # [doc = " # Safety"] # [doc = ""] # [doc = " The cell must be initialized"] # [inline] unsafe fn get_unchecked (& self) -> & T { debug_assert ! (self . is_initialized ()) ; unsafe { (& * self . value . get ()) . assume_init_ref () } } # [doc = " # Safety"] # [doc = ""] # [doc = " The cell must be initialized"] # [inline] unsafe fn get_unchecked_mut (& mut self) -> & mut T { debug_assert ! (self . is_initialized ()) ; unsafe { (& mut * self . value . get ()) . assume_init_mut () } } }}}
mkitem!{mkimpl!{# [stable (feature = "once_cell" , since = "1.70.0")] unsafe impl < T : Sync + Send > Sync for OnceLock < T > { }}}
mkitem!{mkimpl!{# [stable (feature = "once_cell" , since = "1.70.0")] unsafe impl < T : Send > Send for OnceLock < T > { }}}
mkitem!{mkimpl!{# [stable (feature = "once_cell" , since = "1.70.0")] impl < T : RefUnwindSafe + UnwindSafe > RefUnwindSafe for OnceLock < T > { }}}
mkitem!{mkimpl!{# [stable (feature = "once_cell" , since = "1.70.0")] impl < T : UnwindSafe > UnwindSafe for OnceLock < T > { }}}
mkitem!{mkimpl!{# [stable (feature = "once_cell" , since = "1.70.0")] impl < T > Default for OnceLock < T > { # [doc = " Creates a new uninitialized cell."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::OnceLock;"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     assert_eq!(OnceLock::<()>::new(), OnceLock::default());"] # [doc = " }"] # [doc = " ```"] # [inline] fn default () -> OnceLock < T > { OnceLock :: new () } }}}
mkitem!{mkimpl!{# [stable (feature = "once_cell" , since = "1.70.0")] impl < T : fmt :: Debug > fmt :: Debug for OnceLock < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_tuple ("OnceLock") ; match self . get () { Some (v) => d . field (v) , None => d . field (& format_args ! ("<uninit>")) , } ; d . finish () } }}}
mkitem!{mkimpl!{# [stable (feature = "once_cell" , since = "1.70.0")] impl < T : Clone > Clone for OnceLock < T > { # [inline] fn clone (& self) -> OnceLock < T > { let cell = Self :: new () ; if let Some (value) = self . get () { match cell . set (value . clone ()) { Ok (()) => () , Err (_) => unreachable ! () , } } cell } }}}
mkitem!{mkimpl!{# [stable (feature = "once_cell" , since = "1.70.0")] impl < T > From < T > for OnceLock < T > { # [doc = " Creates a new cell with its contents set to `value`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::OnceLock;"] # [doc = ""] # [doc = " # fn main() -> Result<(), i32> {"] # [doc = " let a = OnceLock::from(3);"] # [doc = " let b = OnceLock::new();"] # [doc = " b.set(3)?;"] # [doc = " assert_eq!(a, b);"] # [doc = " Ok(())"] # [doc = " # }"] # [doc = " ```"] # [inline] fn from (value : T) -> Self { let cell = Self :: new () ; match cell . set (value) { Ok (()) => cell , Err (_) => unreachable ! () , } } }}}
mkitem!{mkimpl!{# [stable (feature = "once_cell" , since = "1.70.0")] impl < T : PartialEq > PartialEq for OnceLock < T > { # [doc = " Equality for two `OnceLock`s."] # [doc = ""] # [doc = " Two `OnceLock`s are equal if they either both contain values and their"] # [doc = " values are equal, or if neither contains a value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::OnceLock;"] # [doc = ""] # [doc = " let five = OnceLock::new();"] # [doc = " five.set(5).unwrap();"] # [doc = ""] # [doc = " let also_five = OnceLock::new();"] # [doc = " also_five.set(5).unwrap();"] # [doc = ""] # [doc = " assert!(five == also_five);"] # [doc = ""] # [doc = " assert!(OnceLock::<u32>::new() == OnceLock::<u32>::new());"] # [doc = " ```"] # [inline] fn eq (& self , other : & OnceLock < T >) -> bool { self . get () == other . get () } }}}
mkitem!{mkimpl!{# [stable (feature = "once_cell" , since = "1.70.0")] impl < T : Eq > Eq for OnceLock < T > { }}}
mkitem!{mkimpl!{# [stable (feature = "once_cell" , since = "1.70.0")] unsafe impl < # [may_dangle] T > Drop for OnceLock < T > { # [inline] fn drop (& mut self) { if self . is_initialized () { unsafe { (& mut * self . value . get ()) . assume_init_drop () } ; } } }}}