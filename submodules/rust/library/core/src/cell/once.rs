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
mkuse!{use crate :: { fmt , mem } ;}
mkitem!{mkstruct!{#[doc = " A cell which can nominally be written to only once."] #[doc = ""] #[doc = " This allows obtaining a shared `&T` reference to its inner value without copying or replacing"] #[doc = " it (unlike [`Cell`]), and without runtime borrow checks (unlike [`RefCell`]). However,"] #[doc = " only immutable references can be obtained unless one has a mutable reference to the cell"] #[doc = " itself. In the same vein, the cell can only be re-initialized with such a mutable reference."] #[doc = ""] #[doc = " A `OnceCell` can be thought of as a safe abstraction over uninitialized data that becomes"] #[doc = " initialized once written."] #[doc = ""] #[doc = " For a thread-safe version of this struct, see [`std::sync::OnceLock`]."] #[doc = ""] #[doc = " [`RefCell`]: crate::cell::RefCell"] #[doc = " [`Cell`]: crate::cell::Cell"] #[doc = " [`std::sync::OnceLock`]: ../../std/sync/struct.OnceLock.html"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::cell::OnceCell;"] #[doc = ""] #[doc = " let cell = OnceCell::new();"] #[doc = " assert!(cell.get().is_none());"] #[doc = ""] #[doc = " let value: &String = cell.get_or_init(|| {"] #[doc = "     \"Hello, World!\".to_string()"] #[doc = " });"] #[doc = " assert_eq!(value, \"Hello, World!\");"] #[doc = " assert!(cell.get().is_some());"] #[doc = " ```"] #[stable (feature = "once_cell" , since = "1.70.0")] pub struct OnceCell < T > { inner : UnsafeCell < Option < T > > , }}}
mkitem!{mkimpl!{impl < T > OnceCell < T > { #[doc = " Creates a new uninitialized cell."] #[inline] #[must_use] #[stable (feature = "once_cell" , since = "1.70.0")] #[rustc_const_stable (feature = "once_cell" , since = "1.70.0")] pub const fn new () -> OnceCell < T > { OnceCell { inner : UnsafeCell :: new (None) } } #[doc = " Gets the reference to the underlying value."] #[doc = ""] #[doc = " Returns `None` if the cell is uninitialized."] #[inline] #[stable (feature = "once_cell" , since = "1.70.0")] pub fn get (& self) -> Option < & T > { unsafe { & * self . inner . get () } . as_ref () } #[doc = " Gets the mutable reference to the underlying value."] #[doc = ""] #[doc = " Returns `None` if the cell is uninitialized."] #[inline] #[stable (feature = "once_cell" , since = "1.70.0")] pub fn get_mut (& mut self) -> Option < & mut T > { self . inner . get_mut () . as_mut () } #[doc = " Initializes the contents of the cell to `value`."] #[doc = ""] #[doc = " # Errors"] #[doc = ""] #[doc = " This method returns `Ok(())` if the cell was uninitialized"] #[doc = " and `Err(value)` if it was already initialized."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::cell::OnceCell;"] #[doc = ""] #[doc = " let cell = OnceCell::new();"] #[doc = " assert!(cell.get().is_none());"] #[doc = ""] #[doc = " assert_eq!(cell.set(92), Ok(()));"] #[doc = " assert_eq!(cell.set(62), Err(62));"] #[doc = ""] #[doc = " assert!(cell.get().is_some());"] #[doc = " ```"] #[inline] #[stable (feature = "once_cell" , since = "1.70.0")] pub fn set (& self , value : T) -> Result < () , T > { match self . try_insert (value) { Ok (_) => Ok (()) , Err ((_ , value)) => Err (value) , } } #[doc = " Initializes the contents of the cell to `value` if the cell was"] #[doc = " uninitialized, then returns a reference to it."] #[doc = ""] #[doc = " # Errors"] #[doc = ""] #[doc = " This method returns `Ok(&value)` if the cell was uninitialized"] #[doc = " and `Err((&current_value, value))` if it was already initialized."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(once_cell_try_insert)]"] #[doc = ""] #[doc = " use std::cell::OnceCell;"] #[doc = ""] #[doc = " let cell = OnceCell::new();"] #[doc = " assert!(cell.get().is_none());"] #[doc = ""] #[doc = " assert_eq!(cell.try_insert(92), Ok(&92));"] #[doc = " assert_eq!(cell.try_insert(62), Err((&92, 62)));"] #[doc = ""] #[doc = " assert!(cell.get().is_some());"] #[doc = " ```"] #[inline] #[unstable (feature = "once_cell_try_insert" , issue = "116693")] pub fn try_insert (& self , value : T) -> Result < & T , (& T , T) > { if let Some (old) = self . get () { return Err ((old , value)) ; } let slot = unsafe { & mut * self . inner . get () } ; Ok (slot . insert (value)) } #[doc = " Gets the contents of the cell, initializing it to `f()`"] #[doc = " if the cell was uninitialized."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc = " If `f()` panics, the panic is propagated to the caller, and the cell"] #[doc = " remains uninitialized."] #[doc = ""] #[doc = " It is an error to reentrantly initialize the cell from `f`. Doing"] #[doc = " so results in a panic."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::cell::OnceCell;"] #[doc = ""] #[doc = " let cell = OnceCell::new();"] #[doc = " let value = cell.get_or_init(|| 92);"] #[doc = " assert_eq!(value, &92);"] #[doc = " let value = cell.get_or_init(|| unreachable!());"] #[doc = " assert_eq!(value, &92);"] #[doc = " ```"] #[inline] #[stable (feature = "once_cell" , since = "1.70.0")] pub fn get_or_init < F > (& self , f : F) -> & T where F : FnOnce () -> T , { match self . get_or_try_init (| | Ok :: < T , ! > (f ())) { Ok (val) => val , } } #[doc = " Gets the mutable reference of the contents of the cell,"] #[doc = " initializing it to `f()` if the cell was uninitialized."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc = " If `f()` panics, the panic is propagated to the caller, and the cell"] #[doc = " remains uninitialized."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(once_cell_get_mut)]"] #[doc = ""] #[doc = " use std::cell::OnceCell;"] #[doc = ""] #[doc = " let mut cell = OnceCell::new();"] #[doc = " let value = cell.get_mut_or_init(|| 92);"] #[doc = " assert_eq!(*value, 92);"] #[doc = ""] #[doc = " *value += 2;"] #[doc = " assert_eq!(*value, 94);"] #[doc = ""] #[doc = " let value = cell.get_mut_or_init(|| unreachable!());"] #[doc = " assert_eq!(*value, 94);"] #[doc = " ```"] #[inline] #[unstable (feature = "once_cell_get_mut" , issue = "121641")] pub fn get_mut_or_init < F > (& mut self , f : F) -> & mut T where F : FnOnce () -> T , { match self . get_mut_or_try_init (| | Ok :: < T , ! > (f ())) { Ok (val) => val , } } #[doc = " Gets the contents of the cell, initializing it to `f()` if"] #[doc = " the cell was uninitialized. If the cell was uninitialized"] #[doc = " and `f()` failed, an error is returned."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc = " If `f()` panics, the panic is propagated to the caller, and the cell"] #[doc = " remains uninitialized."] #[doc = ""] #[doc = " It is an error to reentrantly initialize the cell from `f`. Doing"] #[doc = " so results in a panic."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(once_cell_try)]"] #[doc = ""] #[doc = " use std::cell::OnceCell;"] #[doc = ""] #[doc = " let cell = OnceCell::new();"] #[doc = " assert_eq!(cell.get_or_try_init(|| Err(())), Err(()));"] #[doc = " assert!(cell.get().is_none());"] #[doc = " let value = cell.get_or_try_init(|| -> Result<i32, ()> {"] #[doc = "     Ok(92)"] #[doc = " });"] #[doc = " assert_eq!(value, Ok(&92));"] #[doc = " assert_eq!(cell.get(), Some(&92))"] #[doc = " ```"] #[unstable (feature = "once_cell_try" , issue = "109737")] pub fn get_or_try_init < F , E > (& self , f : F) -> Result < & T , E > where F : FnOnce () -> Result < T , E > , { if let Some (val) = self . get () { return Ok (val) ; } self . try_init (f) } #[doc = " Gets the mutable reference of the contents of the cell, initializing"] #[doc = " it to `f()` if the cell was uninitialized. If the cell was uninitialized"] #[doc = " and `f()` failed, an error is returned."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc = " If `f()` panics, the panic is propagated to the caller, and the cell"] #[doc = " remains uninitialized."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(once_cell_get_mut)]"] #[doc = ""] #[doc = " use std::cell::OnceCell;"] #[doc = ""] #[doc = " let mut cell: OnceCell<u32> = OnceCell::new();"] #[doc = ""] #[doc = " // Failed attempts to initialize the cell do not change its contents"] #[doc = " assert!(cell.get_mut_or_try_init(|| \"not a number!\".parse()).is_err());"] #[doc = " assert!(cell.get().is_none());"] #[doc = ""] #[doc = " let value = cell.get_mut_or_try_init(|| \"1234\".parse());"] #[doc = " assert_eq!(value, Ok(&mut 1234));"] #[doc = ""] #[doc = " let Ok(value) = value else { return; };"] #[doc = " *value += 2;"] #[doc = " assert_eq!(cell.get(), Some(&1236))"] #[doc = " ```"] #[unstable (feature = "once_cell_get_mut" , issue = "121641")] pub fn get_mut_or_try_init < F , E > (& mut self , f : F) -> Result < & mut T , E > where F : FnOnce () -> Result < T , E > , { if self . get () . is_none () { self . try_init (f) ? ; } Ok (self . get_mut () . unwrap ()) } #[cold] fn try_init < F , E > (& self , f : F) -> Result < & T , E > where F : FnOnce () -> Result < T , E > , { let val = f () ? ; if let Ok (val) = self . try_insert (val) { Ok (val) } else { panic ! ("reentrant init") } } #[doc = " Consumes the cell, returning the wrapped value."] #[doc = ""] #[doc = " Returns `None` if the cell was uninitialized."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::cell::OnceCell;"] #[doc = ""] #[doc = " let cell: OnceCell<String> = OnceCell::new();"] #[doc = " assert_eq!(cell.into_inner(), None);"] #[doc = ""] #[doc = " let cell = OnceCell::new();"] #[doc = " let _ = cell.set(\"hello\".to_owned());"] #[doc = " assert_eq!(cell.into_inner(), Some(\"hello\".to_owned()));"] #[doc = " ```"] #[inline] #[stable (feature = "once_cell" , since = "1.70.0")] #[rustc_const_stable (feature = "const_cell_into_inner" , since = "1.83.0")] #[rustc_allow_const_fn_unstable (const_precise_live_drops)] pub const fn into_inner (self) -> Option < T > { self . inner . into_inner () } #[doc = " Takes the value out of this `OnceCell`, moving it back to an uninitialized state."] #[doc = ""] #[doc = " Has no effect and returns `None` if the `OnceCell` is uninitialized."] #[doc = ""] #[doc = " Safety is guaranteed by requiring a mutable reference."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::cell::OnceCell;"] #[doc = ""] #[doc = " let mut cell: OnceCell<String> = OnceCell::new();"] #[doc = " assert_eq!(cell.take(), None);"] #[doc = ""] #[doc = " let mut cell = OnceCell::new();"] #[doc = " let _ = cell.set(\"hello\".to_owned());"] #[doc = " assert_eq!(cell.take(), Some(\"hello\".to_owned()));"] #[doc = " assert_eq!(cell.get(), None);"] #[doc = " ```"] #[inline] #[stable (feature = "once_cell" , since = "1.70.0")] pub fn take (& mut self) -> Option < T > { mem :: take (self) . into_inner () } }}}
mkitem!{mkimpl!{#[stable (feature = "once_cell" , since = "1.70.0")] impl < T > Default for OnceCell < T > { #[inline] fn default () -> Self { Self :: new () } }}}
mkitem!{mkimpl!{#[stable (feature = "once_cell" , since = "1.70.0")] impl < T : fmt :: Debug > fmt :: Debug for OnceCell < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_tuple ("OnceCell") ; match self . get () { Some (v) => d . field (v) , None => d . field (& format_args ! ("<uninit>")) , } ; d . finish () } }}}
mkitem!{mkimpl!{#[stable (feature = "once_cell" , since = "1.70.0")] impl < T : Clone > Clone for OnceCell < T > { #[inline] fn clone (& self) -> OnceCell < T > { let res = OnceCell :: new () ; if let Some (value) = self . get () { match res . set (value . clone ()) { Ok (()) => () , Err (_) => unreachable ! () , } } res } }}}
mkitem!{mkimpl!{#[stable (feature = "once_cell" , since = "1.70.0")] impl < T : PartialEq > PartialEq for OnceCell < T > { #[inline] fn eq (& self , other : & Self) -> bool { self . get () == other . get () } }}}
mkitem!{mkimpl!{#[stable (feature = "once_cell" , since = "1.70.0")] impl < T : Eq > Eq for OnceCell < T > { }}}
mkitem!{#[stable (feature = "once_cell" , since = "1.70.0")] #[rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl < T > const From < T > for OnceCell < T > { #[doc = " Creates a new `OnceCell<T>` which already contains the given `value`."] #[inline] fn from (value : T) -> Self { OnceCell { inner : UnsafeCell :: new (Some (value)) } } }}
mkitem!{mkimpl!{#[stable (feature = "once_cell" , since = "1.70.0")] impl < T > ! Sync for OnceCell < T > { }}}