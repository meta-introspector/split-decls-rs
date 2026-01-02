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
mkuse!{use std :: ffi :: OsStr ;}
mkuse!{use std :: intrinsics :: transmute_unchecked ;}
mkuse!{use std :: mem :: MaybeUninit ;}
mkuse!{use rustc_span :: ErrorGuaranteed ;}
mkuse!{use crate :: mir :: interpret :: EvalToValTreeResult ;}
mkuse!{use crate :: query :: CyclePlaceholder ;}
mkuse!{use crate :: traits :: solve ;}
mkuse!{use crate :: ty :: adjustment :: CoerceUnsizedInfo ;}
mkuse!{use crate :: ty :: { self , Ty , TyCtxt } ;}
mkuse!{use crate :: { mir , traits } ;}
mkitem!{mkstruct!{# [derive (Copy , Clone)] pub struct Erased < T : Copy > { data : MaybeUninit < T > , }}}
mkitem!{mktrait!{pub trait EraseType : Copy { type Result : Copy ; }}}
mkitem!{# [allow (type_alias_bounds)] pub type Erase < T : EraseType > = Erased < impl Copy > ;}

macro_rules! erase_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function erase in module {}", module_path!());
    };
}

mkfn!{
    erase_introspect!();
    # [inline (always)] # [define_opaque (Erase)] pub fn erase < T : EraseType > (src : T) -> Erase < T > { const { if size_of :: < T > () != size_of :: < T :: Result > () { panic ! ("size of T must match erased type T::Result") } } ; Erased :: < < T as EraseType > :: Result > { data : unsafe { transmute_unchecked :: < T , MaybeUninit < T :: Result > > (src) } , } }
}

macro_rules! restore_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function restore in module {}", module_path!());
    };
}

mkfn!{
    restore_introspect!();
    # [doc = " Restores an erased value."] # [inline (always)] # [define_opaque (Erase)] pub fn restore < T : EraseType > (value : Erase < T >) -> T { let value : Erased < < T as EraseType > :: Result > = value ; unsafe { transmute_unchecked :: < MaybeUninit < T :: Result > , T > (value . data) } }
}
mkitem!{mkimpl!{impl < T > EraseType for & '_ T { type Result = [u8 ; size_of :: < & 'static () > ()] ; }}}
mkitem!{mkimpl!{impl < T > EraseType for & '_ [T] { type Result = [u8 ; size_of :: < & 'static [()] > ()] ; }}}
mkitem!{mkimpl!{impl EraseType for & '_ OsStr { type Result = [u8 ; size_of :: < & 'static OsStr > ()] ; }}}
mkitem!{mkimpl!{impl < T > EraseType for & '_ ty :: List < T > { type Result = [u8 ; size_of :: < & 'static ty :: List < () > > ()] ; }}}
mkitem!{mkimpl!{impl < T > EraseType for & '_ ty :: ListWithCachedTypeInfo < T > { type Result = [u8 ; size_of :: < & 'static ty :: ListWithCachedTypeInfo < () > > ()] ; }}}
mkitem!{mkimpl!{impl < I : rustc_index :: Idx , T > EraseType for & '_ rustc_index :: IndexSlice < I , T > { type Result = [u8 ; size_of :: < & 'static rustc_index :: IndexSlice < u32 , () > > ()] ; }}}
mkitem!{mkimpl!{impl < T > EraseType for Result < & '_ T , traits :: query :: NoSolution > { type Result = [u8 ; size_of :: < Result < & 'static () , traits :: query :: NoSolution > > ()] ; }}}
mkitem!{mkimpl!{impl < T > EraseType for Result < & '_ [T] , traits :: query :: NoSolution > { type Result = [u8 ; size_of :: < Result < & 'static [()] , traits :: query :: NoSolution > > ()] ; }}}
mkitem!{mkimpl!{impl < T > EraseType for Result < & '_ T , rustc_errors :: ErrorGuaranteed > { type Result = [u8 ; size_of :: < Result < & 'static () , rustc_errors :: ErrorGuaranteed > > ()] ; }}}
mkitem!{mkimpl!{impl < T > EraseType for Result < & '_ [T] , rustc_errors :: ErrorGuaranteed > { type Result = [u8 ; size_of :: < Result < & 'static [()] , rustc_errors :: ErrorGuaranteed > > ()] ; }}}
mkitem!{mkimpl!{impl < T > EraseType for Result < & '_ T , traits :: CodegenObligationError > { type Result = [u8 ; size_of :: < Result < & 'static () , traits :: CodegenObligationError > > ()] ; }}}
mkitem!{mkimpl!{impl < T > EraseType for Result < & '_ T , & '_ ty :: layout :: FnAbiError < '_ > > { type Result = [u8 ; size_of :: < Result < & 'static () , & 'static ty :: layout :: FnAbiError < 'static > > > ()] ; }}}
mkitem!{mkimpl!{impl < T > EraseType for Result < (& '_ T , crate :: thir :: ExprId) , rustc_errors :: ErrorGuaranteed > { type Result = [u8 ; size_of :: < Result < (& 'static () , crate :: thir :: ExprId) , rustc_errors :: ErrorGuaranteed > , > ()] ; }}}
mkitem!{mkimpl!{impl EraseType for Result < Option < ty :: Instance < '_ > > , rustc_errors :: ErrorGuaranteed > { type Result = [u8 ; size_of :: < Result < Option < ty :: Instance < 'static > > , rustc_errors :: ErrorGuaranteed > > ()] ; }}}
mkitem!{mkimpl!{impl EraseType for Result < CoerceUnsizedInfo , rustc_errors :: ErrorGuaranteed > { type Result = [u8 ; size_of :: < Result < CoerceUnsizedInfo , rustc_errors :: ErrorGuaranteed > > ()] ; }}}
mkitem!{mkimpl!{impl EraseType for Result < Option < ty :: EarlyBinder < '_ , ty :: Const < '_ > > > , rustc_errors :: ErrorGuaranteed > { type Result = [u8 ; size_of :: < Result < Option < ty :: EarlyBinder < 'static , ty :: Const < 'static > > > , rustc_errors :: ErrorGuaranteed > , > ()] ; }}}
mkitem!{mkimpl!{impl EraseType for Result < ty :: GenericArg < '_ > , traits :: query :: NoSolution > { type Result = [u8 ; size_of :: < Result < ty :: GenericArg < 'static > , traits :: query :: NoSolution > > ()] ; }}}
mkitem!{mkimpl!{impl EraseType for Result < bool , & ty :: layout :: LayoutError < '_ > > { type Result = [u8 ; size_of :: < Result < bool , & 'static ty :: layout :: LayoutError < 'static > > > ()] ; }}}
mkitem!{mkimpl!{impl EraseType for Result < rustc_abi :: TyAndLayout < '_ , Ty < '_ > > , & ty :: layout :: LayoutError < '_ > > { type Result = [u8 ; size_of :: < Result < rustc_abi :: TyAndLayout < 'static , Ty < 'static > > , & 'static ty :: layout :: LayoutError < 'static > , > , > ()] ; }}}
mkitem!{mkimpl!{impl EraseType for Result < mir :: ConstAlloc < '_ > , mir :: interpret :: ErrorHandled > { type Result = [u8 ; size_of :: < Result < mir :: ConstAlloc < 'static > , mir :: interpret :: ErrorHandled > > ()] ; }}}
mkitem!{mkimpl!{impl EraseType for Result < mir :: ConstValue , mir :: interpret :: ErrorHandled > { type Result = [u8 ; size_of :: < Result < mir :: ConstValue , mir :: interpret :: ErrorHandled > > ()] ; }}}
mkitem!{mkimpl!{impl EraseType for EvalToValTreeResult < '_ > { type Result = [u8 ; size_of :: < EvalToValTreeResult < 'static > > ()] ; }}}
mkitem!{mkimpl!{impl EraseType for Result < & '_ ty :: List < Ty < '_ > > , ty :: util :: AlwaysRequiresDrop > { type Result = [u8 ; size_of :: < Result < & 'static ty :: List < Ty < 'static > > , ty :: util :: AlwaysRequiresDrop > > ()] ; }}}
mkitem!{mkimpl!{impl EraseType for Result < ty :: EarlyBinder < '_ , Ty < '_ > > , CyclePlaceholder > { type Result = [u8 ; size_of :: < Result < ty :: EarlyBinder < 'static , Ty < '_ > > , CyclePlaceholder > > ()] ; }}}
mkitem!{mkimpl!{impl < T > EraseType for Option < & '_ T > { type Result = [u8 ; size_of :: < Option < & 'static () > > ()] ; }}}
mkitem!{mkimpl!{impl < T > EraseType for Option < & '_ [T] > { type Result = [u8 ; size_of :: < Option < & 'static [()] > > ()] ; }}}
mkitem!{mkimpl!{impl EraseType for Option < & '_ OsStr > { type Result = [u8 ; size_of :: < Option < & 'static OsStr > > ()] ; }}}
mkitem!{mkimpl!{impl EraseType for Option < mir :: DestructuredConstant < '_ > > { type Result = [u8 ; size_of :: < Option < mir :: DestructuredConstant < 'static > > > ()] ; }}}
mkitem!{mkimpl!{impl EraseType for Option < ty :: ImplTraitHeader < '_ > > { type Result = [u8 ; size_of :: < Option < ty :: ImplTraitHeader < 'static > > > ()] ; }}}
mkitem!{mkimpl!{impl EraseType for Option < ty :: EarlyBinder < '_ , Ty < '_ > > > { type Result = [u8 ; size_of :: < Option < ty :: EarlyBinder < 'static , Ty < 'static > > > > ()] ; }}}
mkitem!{mkimpl!{impl EraseType for rustc_hir :: MaybeOwner < '_ > { type Result = [u8 ; size_of :: < rustc_hir :: MaybeOwner < 'static > > ()] ; }}}
mkitem!{mkimpl!{impl < T : EraseType > EraseType for ty :: EarlyBinder < '_ , T > { type Result = T :: Result ; }}}
mkitem!{mkimpl!{impl EraseType for ty :: Binder < '_ , ty :: FnSig < '_ > > { type Result = [u8 ; size_of :: < ty :: Binder < 'static , ty :: FnSig < 'static > > > ()] ; }}}
mkitem!{mkimpl!{impl EraseType for ty :: Binder < '_ , ty :: CoroutineWitnessTypes < TyCtxt < '_ > > > { type Result = [u8 ; size_of :: < ty :: Binder < 'static , ty :: CoroutineWitnessTypes < TyCtxt < 'static > > > > ()] ; }}}
mkitem!{mkimpl!{impl EraseType for ty :: Binder < '_ , & '_ ty :: List < Ty < '_ > > > { type Result = [u8 ; size_of :: < ty :: Binder < 'static , & 'static ty :: List < Ty < 'static > > > > ()] ; }}}
mkitem!{mkimpl!{impl < T0 , T1 > EraseType for (& '_ T0 , & '_ T1) { type Result = [u8 ; size_of :: < (& 'static () , & 'static ()) > ()] ; }}}
mkitem!{mkimpl!{impl < T0 > EraseType for (solve :: QueryResult < '_ > , & '_ T0) { type Result = [u8 ; size_of :: < (solve :: QueryResult < 'static > , & 'static ()) > ()] ; }}}
mkitem!{mkimpl!{impl < T0 , T1 > EraseType for (& '_ T0 , & '_ [T1]) { type Result = [u8 ; size_of :: < (& 'static () , & 'static [()]) > ()] ; }}}
mkitem!{mkimpl!{impl < T0 , T1 > EraseType for (& '_ [T0] , & '_ [T1]) { type Result = [u8 ; size_of :: < (& 'static [()] , & 'static [()]) > ()] ; }}}
mkitem!{mkimpl!{impl < T0 > EraseType for (& '_ T0 , Result < () , ErrorGuaranteed >) { type Result = [u8 ; size_of :: < (& 'static () , Result < () , ErrorGuaranteed >) > ()] ; }}}
mkitem!{macro_rules ! trivial { ($ ($ ty : ty) ,+ $ (,) ?) => { $ (impl EraseType for $ ty { type Result = [u8 ; size_of ::<$ ty > ()] ; }) * } }}
mkitem!{trivial ! { () , bool , Option < (rustc_span :: def_id :: DefId , rustc_session :: config :: EntryFnType) >, Option < rustc_ast :: expand :: allocator :: AllocatorKind >, Option < rustc_hir :: ConstStability >, Option < rustc_hir :: DefaultBodyStability >, Option < rustc_hir :: Stability >, Option < rustc_data_structures :: svh :: Svh >, Option < rustc_hir :: def :: DefKind >, Option < rustc_hir :: CoroutineKind >, Option < rustc_hir :: HirId >, Option < rustc_middle :: middle :: stability :: DeprecationEntry >, Option < rustc_middle :: ty :: AsyncDestructor >, Option < rustc_middle :: ty :: Destructor >, Option < rustc_middle :: ty :: ImplTraitInTraitData >, Option < rustc_middle :: ty :: ScalarInt >, Option < rustc_span :: def_id :: CrateNum >, Option < rustc_span :: def_id :: DefId >, Option < rustc_span :: def_id :: LocalDefId >, Option < rustc_span :: Span >, Option < rustc_abi :: FieldIdx >, Option < rustc_target :: spec :: PanicStrategy >, Option < usize >, Option < rustc_middle :: ty :: IntrinsicDef >, Option < rustc_abi :: Align >, Result < () , rustc_errors :: ErrorGuaranteed >, Result < () , rustc_middle :: traits :: query :: NoSolution >, Result < rustc_middle :: traits :: EvaluationResult , rustc_middle :: traits :: OverflowError >, rustc_abi :: ReprOptions , rustc_ast :: expand :: allocator :: AllocatorKind , rustc_hir :: DefaultBodyStability , rustc_hir :: attrs :: Deprecation , rustc_data_structures :: svh :: Svh , rustc_errors :: ErrorGuaranteed , rustc_hir :: Constness , rustc_hir :: ConstStability , rustc_hir :: def_id :: DefId , rustc_hir :: def_id :: DefIndex , rustc_hir :: def_id :: LocalDefId , rustc_hir :: def_id :: LocalModDefId , rustc_hir :: def :: DefKind , rustc_hir :: Defaultness , rustc_hir :: definitions :: DefKey , rustc_hir :: CoroutineKind , rustc_hir :: HirId , rustc_hir :: IsAsync , rustc_hir :: ItemLocalId , rustc_hir :: LangItem , rustc_hir :: OpaqueTyOrigin < rustc_hir :: def_id :: DefId >, rustc_hir :: OwnerId , rustc_hir :: Stability , rustc_hir :: Upvar , rustc_index :: bit_set :: FiniteBitSet < u32 >, rustc_middle :: middle :: dependency_format :: Linkage , rustc_middle :: middle :: exported_symbols :: SymbolExportInfo , rustc_middle :: middle :: resolve_bound_vars :: ObjectLifetimeDefault , rustc_middle :: middle :: resolve_bound_vars :: ResolvedArg , rustc_middle :: middle :: stability :: DeprecationEntry , rustc_middle :: mir :: ConstQualifs , rustc_middle :: mir :: ConstValue , rustc_middle :: mir :: interpret :: AllocId , rustc_middle :: mir :: interpret :: CtfeProvenance , rustc_middle :: mir :: interpret :: ErrorHandled , rustc_middle :: thir :: ExprId , rustc_middle :: traits :: CodegenObligationError , rustc_middle :: traits :: EvaluationResult , rustc_middle :: traits :: OverflowError , rustc_middle :: traits :: query :: NoSolution , rustc_middle :: traits :: WellFormedLoc , rustc_middle :: ty :: adjustment :: CoerceUnsizedInfo , rustc_middle :: ty :: AssocItem , rustc_middle :: ty :: AssocContainer , rustc_middle :: ty :: Asyncness , rustc_middle :: ty :: AsyncDestructor , rustc_middle :: ty :: BoundVariableKind , rustc_middle :: ty :: AnonConstKind , rustc_middle :: ty :: DeducedParamAttrs , rustc_middle :: ty :: Destructor , rustc_middle :: ty :: fast_reject :: SimplifiedType , rustc_middle :: ty :: ImplPolarity , rustc_middle :: ty :: Representability , rustc_middle :: ty :: UnusedGenericParams , rustc_middle :: ty :: util :: AlwaysRequiresDrop , rustc_middle :: ty :: Visibility < rustc_span :: def_id :: DefId >, rustc_session :: config :: CrateType , rustc_session :: config :: EntryFnType , rustc_session :: config :: OptLevel , rustc_session :: config :: SymbolManglingVersion , rustc_session :: cstore :: CrateDepKind , rustc_session :: cstore :: ExternCrate , rustc_session :: cstore :: LinkagePreference , rustc_session :: Limits , rustc_session :: lint :: LintExpectationId , rustc_span :: def_id :: CrateNum , rustc_span :: def_id :: DefPathHash , rustc_span :: ExpnHash , rustc_span :: ExpnId , rustc_span :: Span , rustc_span :: Symbol , rustc_span :: Ident , rustc_target :: spec :: PanicStrategy , rustc_target :: spec :: SanitizerSet , rustc_type_ir :: Variance , u32 , usize , }}
mkitem!{macro_rules ! tcx_lifetime { ($ ($ ($ fake_path : ident) ::+) ,+ $ (,) ?) => { $ (impl <'tcx > EraseType for $ ($ fake_path) ::+<'tcx > { type Result = [u8 ; size_of ::<$ ($ fake_path) ::+<'static >> ()] ; }) * } }}
mkitem!{tcx_lifetime ! { rustc_middle :: middle :: exported_symbols :: ExportedSymbol , rustc_middle :: mir :: Const , rustc_middle :: mir :: DestructuredConstant , rustc_middle :: mir :: ConstAlloc , rustc_middle :: mir :: interpret :: GlobalId , rustc_middle :: mir :: interpret :: LitToConstInput , rustc_middle :: mir :: interpret :: EvalStaticInitializerRawResult , rustc_middle :: mir :: mono :: MonoItemPartitions , rustc_middle :: traits :: query :: MethodAutoderefStepsResult , rustc_middle :: traits :: query :: type_op :: AscribeUserType , rustc_middle :: traits :: query :: type_op :: Eq , rustc_middle :: traits :: query :: type_op :: ProvePredicate , rustc_middle :: traits :: query :: type_op :: Subtype , rustc_middle :: ty :: AdtDef , rustc_middle :: ty :: AliasTy , rustc_middle :: ty :: ClauseKind , rustc_middle :: ty :: ClosureTypeInfo , rustc_middle :: ty :: Const , rustc_middle :: ty :: DestructuredConst , rustc_middle :: ty :: ExistentialTraitRef , rustc_middle :: ty :: FnSig , rustc_middle :: ty :: GenericArg , rustc_middle :: ty :: GenericPredicates , rustc_middle :: ty :: ConstConditions , rustc_middle :: ty :: inhabitedness :: InhabitedPredicate , rustc_middle :: ty :: Instance , rustc_middle :: ty :: InstanceKind , rustc_middle :: ty :: layout :: FnAbiError , rustc_middle :: ty :: layout :: LayoutError , rustc_middle :: ty :: ParamEnv , rustc_middle :: ty :: TypingEnv , rustc_middle :: ty :: Predicate , rustc_middle :: ty :: SymbolName , rustc_middle :: ty :: TraitRef , rustc_middle :: ty :: Ty , rustc_middle :: ty :: UnevaluatedConst , rustc_middle :: ty :: ValTree , rustc_middle :: ty :: VtblEntry , }}