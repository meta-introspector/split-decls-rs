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
mkuse!{use core :: any :: Any ;}
mkuse!{use core :: error :: Error ;}
mkuse!{use core :: mem ;}
mkuse!{use core :: pin :: Pin ;}
mkuse!{#[cfg (not (no_global_oom_handling))] use core :: { fmt , ptr } ;}
mkuse!{use crate :: alloc :: Allocator ;}
mkuse!{#[cfg (not (no_global_oom_handling))] use crate :: borrow :: Cow ;}
mkuse!{use crate :: boxed :: Box ;}
mkuse!{#[cfg (not (no_global_oom_handling))] use crate :: raw_vec :: RawVec ;}
mkuse!{#[cfg (not (no_global_oom_handling))] use crate :: str :: from_boxed_utf8_unchecked ;}
mkuse!{#[cfg (not (no_global_oom_handling))] use crate :: string :: String ;}
mkuse!{#[cfg (not (no_global_oom_handling))] use crate :: vec :: Vec ;}
mkitem!{mkimpl!{#[cfg (not (no_global_oom_handling))] #[stable (feature = "from_for_ptrs" , since = "1.6.0")] impl < T > From < T > for Box < T > { #[doc = " Converts a `T` into a `Box<T>`"] #[doc = ""] #[doc = " The conversion allocates on the heap and moves `t`"] #[doc = " from the stack into it."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```rust"] #[doc = " let x = 5;"] #[doc = " let boxed = Box::new(5);"] #[doc = ""] #[doc = " assert_eq!(Box::from(x), boxed);"] #[doc = " ```"] fn from (t : T) -> Self { Box :: new (t) } }}}
mkitem!{mkimpl!{#[stable (feature = "pin" , since = "1.33.0")] impl < T : ? Sized , A : Allocator > From < Box < T , A > > for Pin < Box < T , A > > where A : 'static , { #[doc = " Converts a `Box<T>` into a `Pin<Box<T>>`. If `T` does not implement [`Unpin`], then"] #[doc = " `*boxed` will be pinned in memory and unable to be moved."] #[doc = ""] #[doc = " This conversion does not allocate on the heap and happens in place."] #[doc = ""] #[doc = " This is also available via [`Box::into_pin`]."] #[doc = ""] #[doc = " Constructing and pinning a `Box` with <code><Pin<Box\\<T>>>::from([Box::new]\\(x))</code>"] #[doc = " can also be written more concisely using <code>[Box::pin]\\(x)</code>."] #[doc = " This `From` implementation is useful if you already have a `Box<T>`, or you are"] #[doc = " constructing a (pinned) `Box` in a different way than with [`Box::new`]."] fn from (boxed : Box < T , A >) -> Self { Box :: into_pin (boxed) } }}}
mkitem!{mktrait!{#[doc = " Specialization trait used for `From<&[T]>`."] #[cfg (not (no_global_oom_handling))] trait BoxFromSlice < T > { fn from_slice (slice : & [T]) -> Self ; }}}
mkitem!{mkimpl!{#[cfg (not (no_global_oom_handling))] impl < T : Clone > BoxFromSlice < T > for Box < [T] > { #[inline] default fn from_slice (slice : & [T]) -> Self { slice . to_vec () . into_boxed_slice () } }}}
mkitem!{mkimpl!{#[cfg (not (no_global_oom_handling))] impl < T : Copy > BoxFromSlice < T > for Box < [T] > { #[inline] fn from_slice (slice : & [T]) -> Self { let len = slice . len () ; let buf = RawVec :: with_capacity (len) ; unsafe { ptr :: copy_nonoverlapping (slice . as_ptr () , buf . ptr () , len) ; buf . into_box (slice . len ()) . assume_init () } } }}}
mkitem!{mkimpl!{#[cfg (not (no_global_oom_handling))] #[stable (feature = "box_from_slice" , since = "1.17.0")] impl < T : Clone > From < & [T] > for Box < [T] > { #[doc = " Converts a `&[T]` into a `Box<[T]>`"] #[doc = ""] #[doc = " This conversion allocates on the heap"] #[doc = " and performs a copy of `slice` and its contents."] #[doc = ""] #[doc = " # Examples"] #[doc = " ```rust"] #[doc = " // create a &[u8] which will be used to create a Box<[u8]>"] #[doc = " let slice: &[u8] = &[104, 101, 108, 108, 111];"] #[doc = " let boxed_slice: Box<[u8]> = Box::from(slice);"] #[doc = ""] #[doc = " println!(\"{boxed_slice:?}\");"] #[doc = " ```"] #[inline] fn from (slice : & [T]) -> Box < [T] > { < Self as BoxFromSlice < T > > :: from_slice (slice) } }}}
mkitem!{mkimpl!{#[cfg (not (no_global_oom_handling))] #[stable (feature = "box_from_mut_slice" , since = "1.84.0")] impl < T : Clone > From < & mut [T] > for Box < [T] > { #[doc = " Converts a `&mut [T]` into a `Box<[T]>`"] #[doc = ""] #[doc = " This conversion allocates on the heap"] #[doc = " and performs a copy of `slice` and its contents."] #[doc = ""] #[doc = " # Examples"] #[doc = " ```rust"] #[doc = " // create a &mut [u8] which will be used to create a Box<[u8]>"] #[doc = " let mut array = [104, 101, 108, 108, 111];"] #[doc = " let slice: &mut [u8] = &mut array;"] #[doc = " let boxed_slice: Box<[u8]> = Box::from(slice);"] #[doc = ""] #[doc = " println!(\"{boxed_slice:?}\");"] #[doc = " ```"] #[inline] fn from (slice : & mut [T]) -> Box < [T] > { Self :: from (& * slice) } }}}
mkitem!{mkimpl!{#[cfg (not (no_global_oom_handling))] #[stable (feature = "box_from_cow" , since = "1.45.0")] impl < T : Clone > From < Cow < '_ , [T] > > for Box < [T] > { #[doc = " Converts a `Cow<'_, [T]>` into a `Box<[T]>`"] #[doc = ""] #[doc = " When `cow` is the `Cow::Borrowed` variant, this"] #[doc = " conversion allocates on the heap and copies the"] #[doc = " underlying slice. Otherwise, it will try to reuse the owned"] #[doc = " `Vec`'s allocation."] #[inline] fn from (cow : Cow < '_ , [T] >) -> Box < [T] > { match cow { Cow :: Borrowed (slice) => Box :: from (slice) , Cow :: Owned (slice) => Box :: from (slice) , } } }}}
mkitem!{mkimpl!{#[cfg (not (no_global_oom_handling))] #[stable (feature = "box_from_slice" , since = "1.17.0")] impl From < & str > for Box < str > { #[doc = " Converts a `&str` into a `Box<str>`"] #[doc = ""] #[doc = " This conversion allocates on the heap"] #[doc = " and performs a copy of `s`."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```rust"] #[doc = " let boxed: Box<str> = Box::from(\"hello\");"] #[doc = " println!(\"{boxed}\");"] #[doc = " ```"] #[inline] fn from (s : & str) -> Box < str > { unsafe { from_boxed_utf8_unchecked (Box :: from (s . as_bytes ())) } } }}}
mkitem!{mkimpl!{#[cfg (not (no_global_oom_handling))] #[stable (feature = "box_from_mut_slice" , since = "1.84.0")] impl From < & mut str > for Box < str > { #[doc = " Converts a `&mut str` into a `Box<str>`"] #[doc = ""] #[doc = " This conversion allocates on the heap"] #[doc = " and performs a copy of `s`."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```rust"] #[doc = " let mut original = String::from(\"hello\");"] #[doc = " let original: &mut str = &mut original;"] #[doc = " let boxed: Box<str> = Box::from(original);"] #[doc = " println!(\"{boxed}\");"] #[doc = " ```"] #[inline] fn from (s : & mut str) -> Box < str > { Self :: from (& * s) } }}}
mkitem!{mkimpl!{#[cfg (not (no_global_oom_handling))] #[stable (feature = "box_from_cow" , since = "1.45.0")] impl From < Cow < '_ , str > > for Box < str > { #[doc = " Converts a `Cow<'_, str>` into a `Box<str>`"] #[doc = ""] #[doc = " When `cow` is the `Cow::Borrowed` variant, this"] #[doc = " conversion allocates on the heap and copies the"] #[doc = " underlying `str`. Otherwise, it will try to reuse the owned"] #[doc = " `String`'s allocation."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```rust"] #[doc = " use std::borrow::Cow;"] #[doc = ""] #[doc = " let unboxed = Cow::Borrowed(\"hello\");"] #[doc = " let boxed: Box<str> = Box::from(unboxed);"] #[doc = " println!(\"{boxed}\");"] #[doc = " ```"] #[doc = ""] #[doc = " ```rust"] #[doc = " # use std::borrow::Cow;"] #[doc = " let unboxed = Cow::Owned(\"hello\".to_string());"] #[doc = " let boxed: Box<str> = Box::from(unboxed);"] #[doc = " println!(\"{boxed}\");"] #[doc = " ```"] #[inline] fn from (cow : Cow < '_ , str >) -> Box < str > { match cow { Cow :: Borrowed (s) => Box :: from (s) , Cow :: Owned (s) => Box :: from (s) , } } }}}
mkitem!{mkimpl!{#[stable (feature = "boxed_str_conv" , since = "1.19.0")] impl < A : Allocator > From < Box < str , A > > for Box < [u8] , A > { #[doc = " Converts a `Box<str>` into a `Box<[u8]>`"] #[doc = ""] #[doc = " This conversion does not allocate on the heap and happens in place."] #[doc = ""] #[doc = " # Examples"] #[doc = " ```rust"] #[doc = " // create a Box<str> which will be used to create a Box<[u8]>"] #[doc = " let boxed: Box<str> = Box::from(\"hello\");"] #[doc = " let boxed_str: Box<[u8]> = Box::from(boxed);"] #[doc = ""] #[doc = " // create a &[u8] which will be used to create a Box<[u8]>"] #[doc = " let slice: &[u8] = &[104, 101, 108, 108, 111];"] #[doc = " let boxed_slice = Box::from(slice);"] #[doc = ""] #[doc = " assert_eq!(boxed_slice, boxed_str);"] #[doc = " ```"] #[inline] fn from (s : Box < str , A >) -> Self { let (raw , alloc) = Box :: into_raw_with_allocator (s) ; unsafe { Box :: from_raw_in (raw as * mut [u8] , alloc) } } }}}
mkitem!{mkimpl!{#[cfg (not (no_global_oom_handling))] #[stable (feature = "box_from_array" , since = "1.45.0")] impl < T , const N : usize > From < [T ; N] > for Box < [T] > { #[doc = " Converts a `[T; N]` into a `Box<[T]>`"] #[doc = ""] #[doc = " This conversion moves the array to newly heap-allocated memory."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```rust"] #[doc = " let boxed: Box<[u8]> = Box::from([4, 2]);"] #[doc = " println!(\"{boxed:?}\");"] #[doc = " ```"] fn from (array : [T ; N]) -> Box < [T] > { Box :: new (array) } }}}

macro_rules! boxed_slice_as_array_unchecked_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function boxed_slice_as_array_unchecked in module {}", module_path!());
    };
}

mkfn!{
    boxed_slice_as_array_unchecked_introspect!();
    #[doc = " Casts a boxed slice to a boxed array."] #[doc = ""] #[doc = " # Safety"] #[doc = ""] #[doc = " `boxed_slice.len()` must be exactly `N`."] unsafe fn boxed_slice_as_array_unchecked < T , A : Allocator , const N : usize > (boxed_slice : Box < [T] , A > ,) -> Box < [T ; N] , A > { debug_assert_eq ! (boxed_slice . len () , N) ; let (ptr , alloc) = Box :: into_raw_with_allocator (boxed_slice) ; unsafe { Box :: from_raw_in (ptr as * mut [T ; N] , alloc) } }
}
mkitem!{mkimpl!{#[stable (feature = "boxed_slice_try_from" , since = "1.43.0")] impl < T , const N : usize > TryFrom < Box < [T] > > for Box < [T ; N] > { type Error = Box < [T] > ; #[doc = " Attempts to convert a `Box<[T]>` into a `Box<[T; N]>`."] #[doc = ""] #[doc = " The conversion occurs in-place and does not require a"] #[doc = " new memory allocation."] #[doc = ""] #[doc = " # Errors"] #[doc = ""] #[doc = " Returns the old `Box<[T]>` in the `Err` variant if"] #[doc = " `boxed_slice.len()` does not equal `N`."] fn try_from (boxed_slice : Box < [T] >) -> Result < Self , Self :: Error > { if boxed_slice . len () == N { Ok (unsafe { boxed_slice_as_array_unchecked (boxed_slice) }) } else { Err (boxed_slice) } } }}}
mkitem!{mkimpl!{#[cfg (not (no_global_oom_handling))] #[stable (feature = "boxed_array_try_from_vec" , since = "1.66.0")] impl < T , const N : usize > TryFrom < Vec < T > > for Box < [T ; N] > { type Error = Vec < T > ; #[doc = " Attempts to convert a `Vec<T>` into a `Box<[T; N]>`."] #[doc = ""] #[doc = " Like [`Vec::into_boxed_slice`], this is in-place if `vec.capacity() == N`,"] #[doc = " but will require a reallocation otherwise."] #[doc = ""] #[doc = " # Errors"] #[doc = ""] #[doc = " Returns the original `Vec<T>` in the `Err` variant if"] #[doc = " `boxed_slice.len()` does not equal `N`."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " This can be used with [`vec!`] to create an array on the heap:"] #[doc = ""] #[doc = " ```"] #[doc = " let state: Box<[f32; 100]> = vec![1.0; 100].try_into().unwrap();"] #[doc = " assert_eq!(state.len(), 100);"] #[doc = " ```"] fn try_from (vec : Vec < T >) -> Result < Self , Self :: Error > { if vec . len () == N { let boxed_slice = vec . into_boxed_slice () ; Ok (unsafe { boxed_slice_as_array_unchecked (boxed_slice) }) } else { Err (vec) } } }}}
mkitem!{mkimpl!{impl < A : Allocator > Box < dyn Any , A > { #[doc = " Attempts to downcast the box to a concrete type."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::any::Any;"] #[doc = ""] #[doc = " fn print_if_string(value: Box<dyn Any>) {"] #[doc = "     if let Ok(string) = value.downcast::<String>() {"] #[doc = "         println!(\"String ({}): {}\", string.len(), string);"] #[doc = "     }"] #[doc = " }"] #[doc = ""] #[doc = " let my_string = \"Hello World\".to_string();"] #[doc = " print_if_string(Box::new(my_string));"] #[doc = " print_if_string(Box::new(0i8));"] #[doc = " ```"] #[inline] #[stable (feature = "rust1" , since = "1.0.0")] pub fn downcast < T : Any > (self) -> Result < Box < T , A > , Self > { if self . is :: < T > () { unsafe { Ok (self . downcast_unchecked :: < T > ()) } } else { Err (self) } } #[doc = " Downcasts the box to a concrete type."] #[doc = ""] #[doc = " For a safe alternative see [`downcast`]."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(downcast_unchecked)]"] #[doc = ""] #[doc = " use std::any::Any;"] #[doc = ""] #[doc = " let x: Box<dyn Any> = Box::new(1_usize);"] #[doc = ""] #[doc = " unsafe {"] #[doc = "     assert_eq!(*x.downcast_unchecked::<usize>(), 1);"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " # Safety"] #[doc = ""] #[doc = " The contained value must be of type `T`. Calling this method"] #[doc = " with the incorrect type is *undefined behavior*."] #[doc = ""] #[doc = " [`downcast`]: Self::downcast"] #[inline] #[unstable (feature = "downcast_unchecked" , issue = "90850")] pub unsafe fn downcast_unchecked < T : Any > (self) -> Box < T , A > { debug_assert ! (self . is ::< T > ()) ; unsafe { let (raw , alloc) : (* mut dyn Any , _) = Box :: into_raw_with_allocator (self) ; Box :: from_raw_in (raw as * mut T , alloc) } } }}}
mkitem!{mkimpl!{impl < A : Allocator > Box < dyn Any + Send , A > { #[doc = " Attempts to downcast the box to a concrete type."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::any::Any;"] #[doc = ""] #[doc = " fn print_if_string(value: Box<dyn Any + Send>) {"] #[doc = "     if let Ok(string) = value.downcast::<String>() {"] #[doc = "         println!(\"String ({}): {}\", string.len(), string);"] #[doc = "     }"] #[doc = " }"] #[doc = ""] #[doc = " let my_string = \"Hello World\".to_string();"] #[doc = " print_if_string(Box::new(my_string));"] #[doc = " print_if_string(Box::new(0i8));"] #[doc = " ```"] #[inline] #[stable (feature = "rust1" , since = "1.0.0")] pub fn downcast < T : Any > (self) -> Result < Box < T , A > , Self > { if self . is :: < T > () { unsafe { Ok (self . downcast_unchecked :: < T > ()) } } else { Err (self) } } #[doc = " Downcasts the box to a concrete type."] #[doc = ""] #[doc = " For a safe alternative see [`downcast`]."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(downcast_unchecked)]"] #[doc = ""] #[doc = " use std::any::Any;"] #[doc = ""] #[doc = " let x: Box<dyn Any + Send> = Box::new(1_usize);"] #[doc = ""] #[doc = " unsafe {"] #[doc = "     assert_eq!(*x.downcast_unchecked::<usize>(), 1);"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " # Safety"] #[doc = ""] #[doc = " The contained value must be of type `T`. Calling this method"] #[doc = " with the incorrect type is *undefined behavior*."] #[doc = ""] #[doc = " [`downcast`]: Self::downcast"] #[inline] #[unstable (feature = "downcast_unchecked" , issue = "90850")] pub unsafe fn downcast_unchecked < T : Any > (self) -> Box < T , A > { debug_assert ! (self . is ::< T > ()) ; unsafe { let (raw , alloc) : (* mut (dyn Any + Send) , _) = Box :: into_raw_with_allocator (self) ; Box :: from_raw_in (raw as * mut T , alloc) } } }}}
mkitem!{mkimpl!{impl < A : Allocator > Box < dyn Any + Send + Sync , A > { #[doc = " Attempts to downcast the box to a concrete type."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::any::Any;"] #[doc = ""] #[doc = " fn print_if_string(value: Box<dyn Any + Send + Sync>) {"] #[doc = "     if let Ok(string) = value.downcast::<String>() {"] #[doc = "         println!(\"String ({}): {}\", string.len(), string);"] #[doc = "     }"] #[doc = " }"] #[doc = ""] #[doc = " let my_string = \"Hello World\".to_string();"] #[doc = " print_if_string(Box::new(my_string));"] #[doc = " print_if_string(Box::new(0i8));"] #[doc = " ```"] #[inline] #[stable (feature = "box_send_sync_any_downcast" , since = "1.51.0")] pub fn downcast < T : Any > (self) -> Result < Box < T , A > , Self > { if self . is :: < T > () { unsafe { Ok (self . downcast_unchecked :: < T > ()) } } else { Err (self) } } #[doc = " Downcasts the box to a concrete type."] #[doc = ""] #[doc = " For a safe alternative see [`downcast`]."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(downcast_unchecked)]"] #[doc = ""] #[doc = " use std::any::Any;"] #[doc = ""] #[doc = " let x: Box<dyn Any + Send + Sync> = Box::new(1_usize);"] #[doc = ""] #[doc = " unsafe {"] #[doc = "     assert_eq!(*x.downcast_unchecked::<usize>(), 1);"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " # Safety"] #[doc = ""] #[doc = " The contained value must be of type `T`. Calling this method"] #[doc = " with the incorrect type is *undefined behavior*."] #[doc = ""] #[doc = " [`downcast`]: Self::downcast"] #[inline] #[unstable (feature = "downcast_unchecked" , issue = "90850")] pub unsafe fn downcast_unchecked < T : Any > (self) -> Box < T , A > { debug_assert ! (self . is ::< T > ()) ; unsafe { let (raw , alloc) : (* mut (dyn Any + Send + Sync) , _) = Box :: into_raw_with_allocator (self) ; Box :: from_raw_in (raw as * mut T , alloc) } } }}}
mkitem!{mkimpl!{#[cfg (not (no_global_oom_handling))] #[stable (feature = "rust1" , since = "1.0.0")] impl < 'a , E : Error + 'a > From < E > for Box < dyn Error + 'a > { #[doc = " Converts a type of [`Error`] into a box of dyn [`Error`]."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::error::Error;"] #[doc = " use std::fmt;"] #[doc = ""] #[doc = " #[derive(Debug)]"] #[doc = " struct AnError;"] #[doc = ""] #[doc = " impl fmt::Display for AnError {"] #[doc = "     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {"] #[doc = "         write!(f, \"An error\")"] #[doc = "     }"] #[doc = " }"] #[doc = ""] #[doc = " impl Error for AnError {}"] #[doc = ""] #[doc = " let an_error = AnError;"] #[doc = " assert!(0 == size_of_val(&an_error));"] #[doc = " let a_boxed_error = Box::<dyn Error>::from(an_error);"] #[doc = " assert!(size_of::<Box<dyn Error>>() == size_of_val(&a_boxed_error))"] #[doc = " ```"] fn from (err : E) -> Box < dyn Error + 'a > { Box :: new (err) } }}}
mkitem!{mkimpl!{#[cfg (not (no_global_oom_handling))] #[stable (feature = "rust1" , since = "1.0.0")] impl < 'a , E : Error + Send + Sync + 'a > From < E > for Box < dyn Error + Send + Sync + 'a > { #[doc = " Converts a type of [`Error`] + [`Send`] + [`Sync`] into a box of"] #[doc = " dyn [`Error`] + [`Send`] + [`Sync`]."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::error::Error;"] #[doc = " use std::fmt;"] #[doc = ""] #[doc = " #[derive(Debug)]"] #[doc = " struct AnError;"] #[doc = ""] #[doc = " impl fmt::Display for AnError {"] #[doc = "     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {"] #[doc = "         write!(f, \"An error\")"] #[doc = "     }"] #[doc = " }"] #[doc = ""] #[doc = " impl Error for AnError {}"] #[doc = ""] #[doc = " unsafe impl Send for AnError {}"] #[doc = ""] #[doc = " unsafe impl Sync for AnError {}"] #[doc = ""] #[doc = " let an_error = AnError;"] #[doc = " assert!(0 == size_of_val(&an_error));"] #[doc = " let a_boxed_error = Box::<dyn Error + Send + Sync>::from(an_error);"] #[doc = " assert!("] #[doc = "     size_of::<Box<dyn Error + Send + Sync>>() == size_of_val(&a_boxed_error))"] #[doc = " ```"] fn from (err : E) -> Box < dyn Error + Send + Sync + 'a > { Box :: new (err) } }}}
mkitem!{mkimpl!{#[cfg (not (no_global_oom_handling))] #[stable (feature = "rust1" , since = "1.0.0")] impl < 'a > From < String > for Box < dyn Error + Send + Sync + 'a > { #[doc = " Converts a [`String`] into a box of dyn [`Error`] + [`Send`] + [`Sync`]."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::error::Error;"] #[doc = ""] #[doc = " let a_string_error = \"a string error\".to_string();"] #[doc = " let a_boxed_error = Box::<dyn Error + Send + Sync>::from(a_string_error);"] #[doc = " assert!("] #[doc = "     size_of::<Box<dyn Error + Send + Sync>>() == size_of_val(&a_boxed_error))"] #[doc = " ```"] #[inline] fn from (err : String) -> Box < dyn Error + Send + Sync + 'a > { struct StringError (String) ; impl Error for StringError { } impl fmt :: Display for StringError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } } impl fmt :: Debug for StringError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . 0 , f) } } Box :: new (StringError (err)) } }}}
mkitem!{mkimpl!{#[cfg (not (no_global_oom_handling))] #[stable (feature = "string_box_error" , since = "1.6.0")] impl < 'a > From < String > for Box < dyn Error + 'a > { #[doc = " Converts a [`String`] into a box of dyn [`Error`]."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::error::Error;"] #[doc = ""] #[doc = " let a_string_error = \"a string error\".to_string();"] #[doc = " let a_boxed_error = Box::<dyn Error>::from(a_string_error);"] #[doc = " assert!(size_of::<Box<dyn Error>>() == size_of_val(&a_boxed_error))"] #[doc = " ```"] fn from (str_err : String) -> Box < dyn Error + 'a > { let err1 : Box < dyn Error + Send + Sync > = From :: from (str_err) ; let err2 : Box < dyn Error > = err1 ; err2 } }}}
mkitem!{mkimpl!{#[cfg (not (no_global_oom_handling))] #[stable (feature = "rust1" , since = "1.0.0")] impl < 'a > From < & str > for Box < dyn Error + Send + Sync + 'a > { #[doc = " Converts a [`str`] into a box of dyn [`Error`] + [`Send`] + [`Sync`]."] #[doc = ""] #[doc = " [`str`]: prim@str"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::error::Error;"] #[doc = ""] #[doc = " let a_str_error = \"a str error\";"] #[doc = " let a_boxed_error = Box::<dyn Error + Send + Sync>::from(a_str_error);"] #[doc = " assert!("] #[doc = "     size_of::<Box<dyn Error + Send + Sync>>() == size_of_val(&a_boxed_error))"] #[doc = " ```"] #[inline] fn from (err : & str) -> Box < dyn Error + Send + Sync + 'a > { From :: from (String :: from (err)) } }}}
mkitem!{mkimpl!{#[cfg (not (no_global_oom_handling))] #[stable (feature = "string_box_error" , since = "1.6.0")] impl < 'a > From < & str > for Box < dyn Error + 'a > { #[doc = " Converts a [`str`] into a box of dyn [`Error`]."] #[doc = ""] #[doc = " [`str`]: prim@str"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::error::Error;"] #[doc = ""] #[doc = " let a_str_error = \"a str error\";"] #[doc = " let a_boxed_error = Box::<dyn Error>::from(a_str_error);"] #[doc = " assert!(size_of::<Box<dyn Error>>() == size_of_val(&a_boxed_error))"] #[doc = " ```"] fn from (err : & str) -> Box < dyn Error + 'a > { From :: from (String :: from (err)) } }}}
mkitem!{mkimpl!{#[cfg (not (no_global_oom_handling))] #[stable (feature = "cow_box_error" , since = "1.22.0")] impl < 'a , 'b > From < Cow < 'b , str > > for Box < dyn Error + Send + Sync + 'a > { #[doc = " Converts a [`Cow`] into a box of dyn [`Error`] + [`Send`] + [`Sync`]."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::error::Error;"] #[doc = " use std::borrow::Cow;"] #[doc = ""] #[doc = " let a_cow_str_error = Cow::from(\"a str error\");"] #[doc = " let a_boxed_error = Box::<dyn Error + Send + Sync>::from(a_cow_str_error);"] #[doc = " assert!("] #[doc = "     size_of::<Box<dyn Error + Send + Sync>>() == size_of_val(&a_boxed_error))"] #[doc = " ```"] fn from (err : Cow < 'b , str >) -> Box < dyn Error + Send + Sync + 'a > { From :: from (String :: from (err)) } }}}
mkitem!{mkimpl!{#[cfg (not (no_global_oom_handling))] #[stable (feature = "cow_box_error" , since = "1.22.0")] impl < 'a , 'b > From < Cow < 'b , str > > for Box < dyn Error + 'a > { #[doc = " Converts a [`Cow`] into a box of dyn [`Error`]."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::error::Error;"] #[doc = " use std::borrow::Cow;"] #[doc = ""] #[doc = " let a_cow_str_error = Cow::from(\"a str error\");"] #[doc = " let a_boxed_error = Box::<dyn Error>::from(a_cow_str_error);"] #[doc = " assert!(size_of::<Box<dyn Error>>() == size_of_val(&a_boxed_error))"] #[doc = " ```"] fn from (err : Cow < 'b , str >) -> Box < dyn Error + 'a > { From :: from (String :: from (err)) } }}}
mkitem!{mkimpl!{impl dyn Error { #[doc = " Attempts to downcast the box to a concrete type."] #[inline] #[stable (feature = "error_downcast" , since = "1.3.0")] #[rustc_allow_incoherent_impl] pub fn downcast < T : Error + 'static > (self : Box < Self >) -> Result < Box < T > , Box < dyn Error > > { if self . is :: < T > () { unsafe { let raw : * mut dyn Error = Box :: into_raw (self) ; Ok (Box :: from_raw (raw as * mut T)) } } else { Err (self) } } }}}
mkitem!{mkimpl!{impl dyn Error + Send { #[doc = " Attempts to downcast the box to a concrete type."] #[inline] #[stable (feature = "error_downcast" , since = "1.3.0")] #[rustc_allow_incoherent_impl] pub fn downcast < T : Error + 'static > (self : Box < Self >) -> Result < Box < T > , Box < dyn Error + Send > > { let err : Box < dyn Error > = self ; < dyn Error > :: downcast (err) . map_err (| s | unsafe { mem :: transmute :: < Box < dyn Error > , Box < dyn Error + Send > > (s) }) } }}}
mkitem!{mkimpl!{impl dyn Error + Send + Sync { #[doc = " Attempts to downcast the box to a concrete type."] #[inline] #[stable (feature = "error_downcast" , since = "1.3.0")] #[rustc_allow_incoherent_impl] pub fn downcast < T : Error + 'static > (self : Box < Self >) -> Result < Box < T > , Box < Self > > { let err : Box < dyn Error > = self ; < dyn Error > :: downcast (err) . map_err (| s | unsafe { mem :: transmute :: < Box < dyn Error > , Box < dyn Error + Send + Sync > > (s) }) } }}}