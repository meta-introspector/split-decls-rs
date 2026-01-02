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
mkuse!{use crate :: cmp :: Ordering ;}
mkuse!{use crate :: ffi :: CStr ;}
mkuse!{use crate :: fmt ;}
mkuse!{use crate :: hash :: { Hash , Hasher } ;}
mkuse!{use crate :: marker :: PhantomData ;}
mkuse!{use crate :: ptr :: NonNull ;}
mkitem!{mkstruct!{# [doc = " A struct containing information about the location of a panic."] # [doc = ""] # [doc = " This structure is created by [`PanicHookInfo::location()`] and [`PanicInfo::location()`]."] # [doc = ""] # [doc = " [`PanicInfo::location()`]: crate::panic::PanicInfo::location"] # [doc = " [`PanicHookInfo::location()`]: ../../std/panic/struct.PanicHookInfo.html#method.location"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use std::panic;"] # [doc = ""] # [doc = " panic::set_hook(Box::new(|panic_info| {"] # [doc = "     if let Some(location) = panic_info.location() {"] # [doc = "         println!(\"panic occurred in file '{}' at line {}\", location.file(), location.line());"] # [doc = "     } else {"] # [doc = "         println!(\"panic occurred but can't get location information...\");"] # [doc = "     }"] # [doc = " }));"] # [doc = ""] # [doc = " panic!(\"Normal panic\");"] # [doc = " ```"] # [doc = ""] # [doc = " # Comparisons"] # [doc = ""] # [doc = " Comparisons for equality and ordering are made in file, line, then column priority."] # [doc = " Files are compared as strings, not `Path`, which could be unexpected."] # [doc = " See [`Location::file`]'s documentation for more discussion."] # [lang = "panic_location"] # [derive (Copy , Clone)] # [stable (feature = "panic_hooks" , since = "1.10.0")] pub struct Location < 'a > { filename : NonNull < str > , line : u32 , col : u32 , _filename : PhantomData < & 'a str > , }}}
mkitem!{mkimpl!{# [stable (feature = "panic_hooks" , since = "1.10.0")] impl PartialEq for Location < '_ > { fn eq (& self , other : & Self) -> bool { self . col == other . col && self . line == other . line && self . file () == other . file () } }}}
mkitem!{mkimpl!{# [stable (feature = "panic_hooks" , since = "1.10.0")] impl Eq for Location < '_ > { }}}
mkitem!{mkimpl!{# [stable (feature = "panic_hooks" , since = "1.10.0")] impl Ord for Location < '_ > { fn cmp (& self , other : & Self) -> Ordering { self . file () . cmp (other . file ()) . then_with (| | self . line . cmp (& other . line)) . then_with (| | self . col . cmp (& other . col)) } }}}
mkitem!{mkimpl!{# [stable (feature = "panic_hooks" , since = "1.10.0")] impl PartialOrd for Location < '_ > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }}}
mkitem!{mkimpl!{# [stable (feature = "panic_hooks" , since = "1.10.0")] impl Hash for Location < '_ > { fn hash < H : Hasher > (& self , state : & mut H) { self . file () . hash (state) ; self . line . hash (state) ; self . col . hash (state) ; } }}}
mkitem!{mkimpl!{# [stable (feature = "panic_hooks" , since = "1.10.0")] impl fmt :: Debug for Location < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Location") . field ("file" , & self . file ()) . field ("line" , & self . line) . field ("column" , & self . col) . finish () } }}}
mkitem!{mkimpl!{impl < 'a > Location < 'a > { # [doc = " Returns the source location of the caller of this function. If that function's caller is"] # [doc = " annotated then its call location will be returned, and so on up the stack to the first call"] # [doc = " within a non-tracked function body."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```standalone_crate"] # [doc = " use std::panic::Location;"] # [doc = ""] # [doc = " /// Returns the [`Location`] at which it is called."] # [doc = " #[track_caller]"] # [doc = " fn get_caller_location() -> &'static Location<'static> {"] # [doc = "     Location::caller()"] # [doc = " }"] # [doc = ""] # [doc = " /// Returns a [`Location`] from within this function's definition."] # [doc = " fn get_just_one_location() -> &'static Location<'static> {"] # [doc = "     get_caller_location()"] # [doc = " }"] # [doc = ""] # [doc = " let fixed_location = get_just_one_location();"] # [doc = " assert_eq!(fixed_location.file(), file!());"] # [doc = " assert_eq!(fixed_location.line(), 14);"] # [doc = " assert_eq!(fixed_location.column(), 5);"] # [doc = ""] # [doc = " // running the same untracked function in a different location gives us the same result"] # [doc = " let second_fixed_location = get_just_one_location();"] # [doc = " assert_eq!(fixed_location.file(), second_fixed_location.file());"] # [doc = " assert_eq!(fixed_location.line(), second_fixed_location.line());"] # [doc = " assert_eq!(fixed_location.column(), second_fixed_location.column());"] # [doc = ""] # [doc = " let this_location = get_caller_location();"] # [doc = " assert_eq!(this_location.file(), file!());"] # [doc = " assert_eq!(this_location.line(), 28);"] # [doc = " assert_eq!(this_location.column(), 21);"] # [doc = ""] # [doc = " // running the tracked function in a different location produces a different value"] # [doc = " let another_location = get_caller_location();"] # [doc = " assert_eq!(this_location.file(), another_location.file());"] # [doc = " assert_ne!(this_location.line(), another_location.line());"] # [doc = " assert_ne!(this_location.column(), another_location.column());"] # [doc = " ```"] # [must_use] # [stable (feature = "track_caller" , since = "1.46.0")] # [rustc_const_stable (feature = "const_caller_location" , since = "1.79.0")] # [track_caller] # [inline] pub const fn caller () -> & 'static Location < 'static > { crate :: intrinsics :: caller_location () } # [doc = " Returns the name of the source file from which the panic originated."] # [doc = ""] # [doc = " # `&str`, not `&Path`"] # [doc = ""] # [doc = " The returned name refers to a source path on the compiling system, but it isn't valid to"] # [doc = " represent this directly as a `&Path`. The compiled code may run on a different system with"] # [doc = " a different `Path` implementation than the system providing the contents and this library"] # [doc = " does not currently have a different \"host path\" type."] # [doc = ""] # [doc = " The most surprising behavior occurs when \"the same\" file is reachable via multiple paths in"] # [doc = " the module system (usually using the `#[path = \"...\"]` attribute or similar), which can"] # [doc = " cause what appears to be identical code to return differing values from this function."] # [doc = ""] # [doc = " # Cross-compilation"] # [doc = ""] # [doc = " This value is not suitable for passing to `Path::new` or similar constructors when the host"] # [doc = " platform and target platform differ."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use std::panic;"] # [doc = ""] # [doc = " panic::set_hook(Box::new(|panic_info| {"] # [doc = "     if let Some(location) = panic_info.location() {"] # [doc = "         println!(\"panic occurred in file '{}'\", location.file());"] # [doc = "     } else {"] # [doc = "         println!(\"panic occurred but can't get location information...\");"] # [doc = "     }"] # [doc = " }));"] # [doc = ""] # [doc = " panic!(\"Normal panic\");"] # [doc = " ```"] # [must_use] # [stable (feature = "panic_hooks" , since = "1.10.0")] # [rustc_const_stable (feature = "const_location_fields" , since = "1.79.0")] pub const fn file (& self) -> & 'a str { unsafe { self . filename . as_ref () } } # [doc = " Returns the name of the source file as a nul-terminated `CStr`."] # [doc = ""] # [doc = " This is useful for interop with APIs that expect C/C++ `__FILE__` or"] # [doc = " `std::source_location::file_name`, both of which return a nul-terminated `const char*`."] # [must_use] # [unstable (feature = "file_with_nul" , issue = "141727")] # [inline] pub const fn file_as_c_str (& self) -> & 'a CStr { let filename = self . filename . as_ptr () ; let cstr_len = unsafe { crate :: mem :: size_of_val_raw (filename) . unchecked_add (1) } ; let slice = unsafe { crate :: slice :: from_raw_parts (filename . cast () , cstr_len) } ; unsafe { CStr :: from_bytes_with_nul_unchecked (slice) } } # [doc = " Returns the line number from which the panic originated."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use std::panic;"] # [doc = ""] # [doc = " panic::set_hook(Box::new(|panic_info| {"] # [doc = "     if let Some(location) = panic_info.location() {"] # [doc = "         println!(\"panic occurred at line {}\", location.line());"] # [doc = "     } else {"] # [doc = "         println!(\"panic occurred but can't get location information...\");"] # [doc = "     }"] # [doc = " }));"] # [doc = ""] # [doc = " panic!(\"Normal panic\");"] # [doc = " ```"] # [must_use] # [stable (feature = "panic_hooks" , since = "1.10.0")] # [rustc_const_stable (feature = "const_location_fields" , since = "1.79.0")] # [inline] pub const fn line (& self) -> u32 { self . line } # [doc = " Returns the column from which the panic originated."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use std::panic;"] # [doc = ""] # [doc = " panic::set_hook(Box::new(|panic_info| {"] # [doc = "     if let Some(location) = panic_info.location() {"] # [doc = "         println!(\"panic occurred at column {}\", location.column());"] # [doc = "     } else {"] # [doc = "         println!(\"panic occurred but can't get location information...\");"] # [doc = "     }"] # [doc = " }));"] # [doc = ""] # [doc = " panic!(\"Normal panic\");"] # [doc = " ```"] # [must_use] # [stable (feature = "panic_col" , since = "1.25.0")] # [rustc_const_stable (feature = "const_location_fields" , since = "1.79.0")] # [inline] pub const fn column (& self) -> u32 { self . col } }}}
mkitem!{mkimpl!{# [stable (feature = "panic_hook_display" , since = "1.26.0")] impl fmt :: Display for Location < '_ > { # [inline] fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (formatter , "{}:{}:{}" , self . file () , self . line , self . col) } }}}
mkitem!{mkimpl!{# [stable (feature = "panic_hooks" , since = "1.10.0")] unsafe impl Send for Location < '_ > { }}}
mkitem!{mkimpl!{# [stable (feature = "panic_hooks" , since = "1.10.0")] unsafe impl Sync for Location < '_ > { }}}