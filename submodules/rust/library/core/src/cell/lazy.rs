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
mkuse!{use super :: UnsafeCell ;}
mkuse!{use crate :: hint :: unreachable_unchecked ;}
mkuse!{use crate :: ops :: { Deref , DerefMut } ;}
mkuse!{use crate :: { fmt , mem } ;}
mkitem!{mkenum!{enum State < T , F > { Uninit (F) , Init (T) , Poisoned , }}}
mkitem!{mkstruct!{#[doc = " A value which is initialized on the first access."] #[doc = ""] #[doc = " For a thread-safe version of this struct, see [`std::sync::LazyLock`]."] #[doc = ""] #[doc = " [`std::sync::LazyLock`]: ../../std/sync/struct.LazyLock.html"] #[doc = ""] #[doc = " # Poisoning"] #[doc = ""] #[doc = " If the initialization closure passed to [`LazyCell::new`] panics, the cell will be poisoned."] #[doc = " Once the cell is poisoned, any threads that attempt to access this cell (via a dereference"] #[doc = " or via an explicit call to [`force()`]) will panic."] #[doc = ""] #[doc = " This concept is similar to that of poisoning in the [`std::sync::poison`] module. A key"] #[doc = " difference, however, is that poisoning in `LazyCell` is _unrecoverable_. All future accesses of"] #[doc = " the cell from other threads will panic, whereas a type in [`std::sync::poison`] like"] #[doc = " [`std::sync::poison::Mutex`] allows recovery via [`PoisonError::into_inner()`]."] #[doc = ""] #[doc = " [`force()`]: LazyCell::force"] #[doc = " [`std::sync::poison`]: ../../std/sync/poison/index.html"] #[doc = " [`std::sync::poison::Mutex`]: ../../std/sync/poison/struct.Mutex.html"] #[doc = " [`PoisonError::into_inner()`]: ../../std/sync/poison/struct.PoisonError.html#method.into_inner"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::cell::LazyCell;"] #[doc = ""] #[doc = " let lazy: LazyCell<i32> = LazyCell::new(|| {"] #[doc = "     println!(\"initializing\");"] #[doc = "     92"] #[doc = " });"] #[doc = " println!(\"ready\");"] #[doc = " println!(\"{}\", *lazy);"] #[doc = " println!(\"{}\", *lazy);"] #[doc = ""] #[doc = " // Prints:"] #[doc = " //   ready"] #[doc = " //   initializing"] #[doc = " //   92"] #[doc = " //   92"] #[doc = " ```"] #[stable (feature = "lazy_cell" , since = "1.80.0")] pub struct LazyCell < T , F = fn () -> T > { state : UnsafeCell < State < T , F > > , }}}
mkitem!{mkimpl!{impl < T , F : FnOnce () -> T > LazyCell < T , F > { #[doc = " Creates a new lazy value with the given initializing function."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::cell::LazyCell;"] #[doc = ""] #[doc = " let hello = \"Hello, World!\".to_string();"] #[doc = ""] #[doc = " let lazy = LazyCell::new(|| hello.to_uppercase());"] #[doc = ""] #[doc = " assert_eq!(&*lazy, \"HELLO, WORLD!\");"] #[doc = " ```"] #[inline] #[stable (feature = "lazy_cell" , since = "1.80.0")] #[rustc_const_stable (feature = "lazy_cell" , since = "1.80.0")] pub const fn new (f : F) -> LazyCell < T , F > { LazyCell { state : UnsafeCell :: new (State :: Uninit (f)) } } #[doc = " Consumes this `LazyCell` returning the stored value."] #[doc = ""] #[doc = " Returns `Ok(value)` if `Lazy` is initialized and `Err(f)` otherwise."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc = " Panics if the cell is poisoned."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(lazy_cell_into_inner)]"] #[doc = ""] #[doc = " use std::cell::LazyCell;"] #[doc = ""] #[doc = " let hello = \"Hello, World!\".to_string();"] #[doc = ""] #[doc = " let lazy = LazyCell::new(|| hello.to_uppercase());"] #[doc = ""] #[doc = " assert_eq!(&*lazy, \"HELLO, WORLD!\");"] #[doc = " assert_eq!(LazyCell::into_inner(lazy).ok(), Some(\"HELLO, WORLD!\".to_string()));"] #[doc = " ```"] #[unstable (feature = "lazy_cell_into_inner" , issue = "125623")] #[rustc_const_unstable (feature = "lazy_cell_into_inner" , issue = "125623")] pub const fn into_inner (this : Self) -> Result < T , F > { match this . state . into_inner () { State :: Init (data) => Ok (data) , State :: Uninit (f) => Err (f) , State :: Poisoned => panic_poisoned () , } } #[doc = " Forces the evaluation of this lazy value and returns a reference to"] #[doc = " the result."] #[doc = ""] #[doc = " This is equivalent to the `Deref` impl, but is explicit."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc = " If the initialization closure panics (the one that is passed to the [`new()`] method), the"] #[doc = " panic is propagated to the caller, and the cell becomes poisoned. This will cause all future"] #[doc = " accesses of the cell (via [`force()`] or a dereference) to panic."] #[doc = ""] #[doc = " [`new()`]: LazyCell::new"] #[doc = " [`force()`]: LazyCell::force"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::cell::LazyCell;"] #[doc = ""] #[doc = " let lazy = LazyCell::new(|| 92);"] #[doc = ""] #[doc = " assert_eq!(LazyCell::force(&lazy), &92);"] #[doc = " assert_eq!(&*lazy, &92);"] #[doc = " ```"] #[inline] #[stable (feature = "lazy_cell" , since = "1.80.0")] pub fn force (this : & LazyCell < T , F >) -> & T { let state = unsafe { & * this . state . get () } ; match state { State :: Init (data) => data , State :: Uninit (_) => unsafe { LazyCell :: really_init (this) } , State :: Poisoned => panic_poisoned () , } } #[doc = " Forces the evaluation of this lazy value and returns a mutable reference to"] #[doc = " the result."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc = " If the initialization closure panics (the one that is passed to the [`new()`] method), the"] #[doc = " panic is propagated to the caller, and the cell becomes poisoned. This will cause all future"] #[doc = " accesses of the cell (via [`force()`] or a dereference) to panic."] #[doc = ""] #[doc = " [`new()`]: LazyCell::new"] #[doc = " [`force()`]: LazyCell::force"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(lazy_get)]"] #[doc = " use std::cell::LazyCell;"] #[doc = ""] #[doc = " let mut lazy = LazyCell::new(|| 92);"] #[doc = ""] #[doc = " let p = LazyCell::force_mut(&mut lazy);"] #[doc = " assert_eq!(*p, 92);"] #[doc = " *p = 44;"] #[doc = " assert_eq!(*lazy, 44);"] #[doc = " ```"] #[inline] #[unstable (feature = "lazy_get" , issue = "129333")] pub fn force_mut (this : & mut LazyCell < T , F >) -> & mut T { #[cold] #[doc = " # Safety"] #[doc = " May only be called when the state is `Uninit`."] unsafe fn really_init_mut < T , F : FnOnce () -> T > (state : & mut State < T , F >) -> & mut T { struct PoisonOnPanic < T , F > (* mut State < T , F >) ; impl < T , F > Drop for PoisonOnPanic < T , F > { #[inline] fn drop (& mut self) { unsafe { self . 0 . write (State :: Poisoned) ; } } } let State :: Uninit (f) = state else { unsafe { unreachable_unchecked () } ; } ; let f = unsafe { core :: ptr :: read (f) } ; let guard = PoisonOnPanic (state) ; let data = f () ; unsafe { core :: ptr :: write (guard . 0 , State :: Init (data)) ; } core :: mem :: forget (guard) ; let State :: Init (data) = state else { unreachable ! () } ; data } let state = this . state . get_mut () ; match state { State :: Init (data) => data , State :: Uninit (_) => unsafe { really_init_mut (state) } , State :: Poisoned => panic_poisoned () , } } #[doc = " # Safety"] #[doc = " May only be called when the state is `Uninit`."] #[cold] unsafe fn really_init (this : & LazyCell < T , F >) -> & T { let state = unsafe { & mut * this . state . get () } ; let State :: Uninit (f) = mem :: replace (state , State :: Poisoned) else { unreachable ! () } ; let data = f () ; unsafe { this . state . get () . write (State :: Init (data)) } ; let state = unsafe { & * this . state . get () } ; let State :: Init (data) = state else { unreachable ! () } ; data } }}}
mkitem!{mkimpl!{impl < T , F > LazyCell < T , F > { #[doc = " Returns a mutable reference to the value if initialized. Otherwise (if uninitialized or"] #[doc = " poisoned), returns `None`."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(lazy_get)]"] #[doc = ""] #[doc = " use std::cell::LazyCell;"] #[doc = ""] #[doc = " let mut lazy = LazyCell::new(|| 92);"] #[doc = ""] #[doc = " assert_eq!(LazyCell::get_mut(&mut lazy), None);"] #[doc = " let _ = LazyCell::force(&lazy);"] #[doc = " *LazyCell::get_mut(&mut lazy).unwrap() = 44;"] #[doc = " assert_eq!(*lazy, 44);"] #[doc = " ```"] #[inline] #[unstable (feature = "lazy_get" , issue = "129333")] pub fn get_mut (this : & mut LazyCell < T , F >) -> Option < & mut T > { let state = this . state . get_mut () ; match state { State :: Init (data) => Some (data) , _ => None , } } #[doc = " Returns a reference to the value if initialized. Otherwise (if uninitialized or poisoned),"] #[doc = " returns `None`."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(lazy_get)]"] #[doc = ""] #[doc = " use std::cell::LazyCell;"] #[doc = ""] #[doc = " let lazy = LazyCell::new(|| 92);"] #[doc = ""] #[doc = " assert_eq!(LazyCell::get(&lazy), None);"] #[doc = " let _ = LazyCell::force(&lazy);"] #[doc = " assert_eq!(LazyCell::get(&lazy), Some(&92));"] #[doc = " ```"] #[inline] #[unstable (feature = "lazy_get" , issue = "129333")] pub fn get (this : & LazyCell < T , F >) -> Option < & T > { let state = unsafe { & * this . state . get () } ; match state { State :: Init (data) => Some (data) , _ => None , } } }}}
mkitem!{mkimpl!{#[stable (feature = "lazy_cell" , since = "1.80.0")] impl < T , F : FnOnce () -> T > Deref for LazyCell < T , F > { type Target = T ; #[doc = " # Panics"] #[doc = ""] #[doc = " If the initialization closure panics (the one that is passed to the [`new()`] method), the"] #[doc = " panic is propagated to the caller, and the cell becomes poisoned. This will cause all future"] #[doc = " accesses of the cell (via [`force()`] or a dereference) to panic."] #[doc = ""] #[doc = " [`new()`]: LazyCell::new"] #[doc = " [`force()`]: LazyCell::force"] #[inline] fn deref (& self) -> & T { LazyCell :: force (self) } }}}
mkitem!{mkimpl!{#[stable (feature = "lazy_deref_mut" , since = "1.89.0")] impl < T , F : FnOnce () -> T > DerefMut for LazyCell < T , F > { #[doc = " # Panics"] #[doc = ""] #[doc = " If the initialization closure panics (the one that is passed to the [`new()`] method), the"] #[doc = " panic is propagated to the caller, and the cell becomes poisoned. This will cause all future"] #[doc = " accesses of the cell (via [`force()`] or a dereference) to panic."] #[doc = ""] #[doc = " [`new()`]: LazyCell::new"] #[doc = " [`force()`]: LazyCell::force"] #[inline] fn deref_mut (& mut self) -> & mut T { LazyCell :: force_mut (self) } }}}
mkitem!{mkimpl!{#[stable (feature = "lazy_cell" , since = "1.80.0")] impl < T : Default > Default for LazyCell < T > { #[doc = " Creates a new lazy value using `Default` as the initializing function."] #[inline] fn default () -> LazyCell < T > { LazyCell :: new (T :: default) } }}}
mkitem!{mkimpl!{#[stable (feature = "lazy_cell" , since = "1.80.0")] impl < T : fmt :: Debug , F > fmt :: Debug for LazyCell < T , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_tuple ("LazyCell") ; match LazyCell :: get (self) { Some (data) => d . field (data) , None => d . field (& format_args ! ("<uninit>")) , } ; d . finish () } }}}

macro_rules! panic_poisoned_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_poisoned in module {}", module_path!());
    };
}

mkfn!{
    panic_poisoned_introspect!();
    #[cold] #[inline (never)] const fn panic_poisoned () -> ! { panic ! ("LazyCell instance has previously been poisoned") }
}