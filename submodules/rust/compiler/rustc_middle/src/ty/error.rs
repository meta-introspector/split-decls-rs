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
mkuse!{use std :: borrow :: Cow ;}
mkuse!{use std :: fs :: File ;}
mkuse!{use std :: hash :: { DefaultHasher , Hash , Hasher } ;}
mkuse!{use std :: io :: { Read , Write } ;}
mkuse!{use std :: path :: PathBuf ;}
mkuse!{use rustc_errors :: pluralize ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def :: { CtorOf , DefKind } ;}
mkuse!{use rustc_hir :: limit :: Limit ;}
mkuse!{use rustc_macros :: extension ;}
mkuse!{pub use rustc_type_ir :: error :: ExpectedFound ;}
mkuse!{use crate :: ty :: print :: { FmtPrinter , Print , with_forced_trimmed_paths } ;}
mkuse!{use crate :: ty :: { self , Lift , Ty , TyCtxt } ;}
mkitem!{pub type TypeError < 'tcx > = rustc_type_ir :: error :: TypeError < TyCtxt < 'tcx > > ;}
mkitem!{mkimpl!{# [doc = " Explains the source of a type err in a short, human readable way."] # [doc = " This is meant to be placed in parentheses after some larger message."] # [doc = " You should also invoke `note_and_explain_type_err()` afterwards"] # [doc = " to present additional details, particularly when it comes to lifetime-"] # [doc = " related errors."] # [extension (pub trait TypeErrorToStringExt <'tcx >)] impl < 'tcx > TypeError < 'tcx > { fn to_string (self , tcx : TyCtxt < 'tcx >) -> Cow < 'static , str > { fn report_maybe_different (expected : & str , found : & str) -> String { if expected == found { format ! ("expected {expected}, found a different {found}") } else { format ! ("expected {expected}, found {found}") } } match self { TypeError :: CyclicTy (_) => "cyclic type of infinite size" . into () , TypeError :: CyclicConst (_) => "encountered a self-referencing constant" . into () , TypeError :: Mismatch => "types differ" . into () , TypeError :: PolarityMismatch (values) => { format ! ("expected {} polarity, found {} polarity" , values . expected , values . found) . into () } TypeError :: SafetyMismatch (values) => { format ! ("expected {} fn, found {} fn" , values . expected , values . found) . into () } TypeError :: AbiMismatch (values) => { format ! ("expected {} fn, found {} fn" , values . expected , values . found) . into () } TypeError :: ArgumentMutability (_) | TypeError :: Mutability => { "types differ in mutability" . into () } TypeError :: TupleSize (values) => format ! ("expected a tuple with {} element{}, found one with {} element{}" , values . expected , pluralize ! (values . expected) , values . found , pluralize ! (values . found)) . into () , TypeError :: ArraySize (values) => format ! ("expected an array with a size of {}, found one with a size of {}" , values . expected , values . found ,) . into () , TypeError :: ArgCount => "incorrect number of function parameters" . into () , TypeError :: RegionsDoesNotOutlive (..) => "lifetime mismatch" . into () , TypeError :: RegionsInsufficientlyPolymorphic (..) => { "one type is more general than the other" . into () } TypeError :: RegionsPlaceholderMismatch => { "one type is more general than the other" . into () } TypeError :: ArgumentSorts (values , _) | TypeError :: Sorts (values) => { let expected = values . expected . sort_string (tcx) ; let found = values . found . sort_string (tcx) ; report_maybe_different (& expected , & found) . into () } TypeError :: Traits (values) => { let (mut expected , mut found) = with_forced_trimmed_paths ! ((tcx . def_path_str (values . expected) , tcx . def_path_str (values . found) ,)) ; if expected == found { expected = tcx . def_path_str (values . expected) ; found = tcx . def_path_str (values . found) ; } report_maybe_different (& format ! ("trait `{expected}`") , & format ! ("trait `{found}`")) . into () } TypeError :: VariadicMismatch (ref values) => format ! ("expected {} fn, found {} function" , if values . expected { "variadic" } else { "non-variadic" } , if values . found { "variadic" } else { "non-variadic" }) . into () , TypeError :: ProjectionMismatched (ref values) => format ! ("expected `{}`, found `{}`" , tcx . def_path_str (values . expected) , tcx . def_path_str (values . found)) . into () , TypeError :: ExistentialMismatch (ref values) => report_maybe_different (& format ! ("trait `{}`" , values . expected) , & format ! ("trait `{}`" , values . found) ,) . into () , TypeError :: ConstMismatch (ref values) => { format ! ("expected `{}`, found `{}`" , values . expected , values . found) . into () } TypeError :: ForceInlineCast => { "cannot coerce functions which must be inlined to function pointers" . into () } TypeError :: IntrinsicCast => "cannot coerce intrinsics to function pointers" . into () , TypeError :: TargetFeatureCast (_) => { "cannot coerce functions with `#[target_feature]` to safe function pointers" . into () } } } }}}
mkitem!{mkimpl!{impl < 'tcx > Ty < 'tcx > { pub fn sort_string (self , tcx : TyCtxt < 'tcx >) -> Cow < 'static , str > { match * self . kind () { ty :: Foreign (def_id) => format ! ("extern type `{}`" , tcx . def_path_str (def_id)) . into () , ty :: FnDef (def_id , ..) => match tcx . def_kind (def_id) { DefKind :: Ctor (CtorOf :: Struct , _) => "struct constructor" . into () , DefKind :: Ctor (CtorOf :: Variant , _) => "enum constructor" . into () , _ => "fn item" . into () , } , ty :: FnPtr (..) => "fn pointer" . into () , ty :: Dynamic (inner , ..) if let Some (principal) = inner . principal () => { format ! ("`dyn {}`" , tcx . def_path_str (principal . def_id ())) . into () } ty :: Dynamic (..) => "trait object" . into () , ty :: Closure (..) => "closure" . into () , ty :: Coroutine (def_id , ..) => { format ! ("{:#}" , tcx . coroutine_kind (def_id) . unwrap ()) . into () } ty :: CoroutineWitness (..) => "coroutine witness" . into () , ty :: Infer (ty :: TyVar (_)) => "inferred type" . into () , ty :: Infer (ty :: IntVar (_)) => "integer" . into () , ty :: Infer (ty :: FloatVar (_)) => "floating-point number" . into () , ty :: Placeholder (..) => "placeholder type" . into () , ty :: Bound (..) => "bound type" . into () , ty :: Infer (ty :: FreshTy (_)) => "fresh type" . into () , ty :: Infer (ty :: FreshIntTy (_)) => "fresh integral type" . into () , ty :: Infer (ty :: FreshFloatTy (_)) => "fresh floating-point type" . into () , ty :: Alias (ty :: Projection | ty :: Inherent , _) => "associated type" . into () , ty :: Param (p) => format ! ("type parameter `{p}`") . into () , ty :: Alias (ty :: Opaque , ..) => { if tcx . ty_is_opaque_future (self) { "future" . into () } else { "opaque type" . into () } } ty :: Error (_) => "type error" . into () , _ => { let width = tcx . sess . diagnostic_width () ; let length_limit = std :: cmp :: max (width / 4 , 40) ; format ! ("`{}`" , tcx . string_with_limit (self , length_limit , hir :: def :: Namespace :: TypeNS)) . into () } } } pub fn prefix_string (self , tcx : TyCtxt < '_ >) -> Cow < 'static , str > { match * self . kind () { ty :: Infer (_) | ty :: Error (_) | ty :: Bool | ty :: Char | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Str | ty :: Never => "type" . into () , ty :: Tuple (tys) if tys . is_empty () => "unit type" . into () , ty :: Adt (def , _) => def . descr () . into () , ty :: Foreign (_) => "extern type" . into () , ty :: Array (..) => "array" . into () , ty :: Pat (..) => "pattern type" . into () , ty :: Slice (_) => "slice" . into () , ty :: RawPtr (_ , _) => "raw pointer" . into () , ty :: Ref (.. , mutbl) => match mutbl { hir :: Mutability :: Mut => "mutable reference" , _ => "reference" , } . into () , ty :: FnDef (def_id , ..) => match tcx . def_kind (def_id) { DefKind :: Ctor (CtorOf :: Struct , _) => "struct constructor" . into () , DefKind :: Ctor (CtorOf :: Variant , _) => "enum constructor" . into () , _ => "fn item" . into () , } , ty :: FnPtr (..) => "fn pointer" . into () , ty :: UnsafeBinder (_) => "unsafe binder" . into () , ty :: Dynamic (..) => "trait object" . into () , ty :: Closure (..) | ty :: CoroutineClosure (..) => "closure" . into () , ty :: Coroutine (def_id , ..) => { format ! ("{:#}" , tcx . coroutine_kind (def_id) . unwrap ()) . into () } ty :: CoroutineWitness (..) => "coroutine witness" . into () , ty :: Tuple (..) => "tuple" . into () , ty :: Placeholder (..) => "higher-ranked type" . into () , ty :: Bound (..) => "bound type variable" . into () , ty :: Alias (ty :: Projection | ty :: Inherent , _) => "associated type" . into () , ty :: Alias (ty :: Free , _) => "type alias" . into () , ty :: Param (_) => "type parameter" . into () , ty :: Alias (ty :: Opaque , ..) => "opaque type" . into () , } } }}}
mkitem!{mkimpl!{impl < 'tcx > TyCtxt < 'tcx > { pub fn string_with_limit < T > (self , t : T , length_limit : usize , ns : hir :: def :: Namespace) -> String where T : Copy + for < 'a , 'b > Lift < TyCtxt < 'b > , Lifted : Print < 'b , FmtPrinter < 'a , 'b > > > , { let mut type_limit = 50 ; let regular = FmtPrinter :: print_string (self , ns , | p | { self . lift (t) . expect ("could not lift for printing") . print (p) }) . expect ("could not write to `String`") ; if regular . len () <= length_limit { return regular ; } let mut short ; loop { short = with_forced_trimmed_paths ! ({ let mut p = FmtPrinter :: new_with_limit (self , ns , Limit (type_limit)) ; self . lift (t) . expect ("could not lift for printing") . print (& mut p) . expect ("could not print type") ; p . into_buffer () }) ; if short . len () <= length_limit || type_limit == 0 { break ; } type_limit -= 1 ; } short } # [doc = " When calling this after a `Diag` is constructed, the preferred way of doing so is"] # [doc = " `tcx.short_string(ty, diag.long_ty_path())`. The diagnostic itself is the one that keeps"] # [doc = " the existence of a \"long type\" anywhere in the diagnostic, so the note telling the user"] # [doc = " where we wrote the file to is only printed once. The path will use the type namespace."] pub fn short_string < T > (self , t : T , path : & mut Option < PathBuf >) -> String where T : Copy + Hash + for < 'a , 'b > Lift < TyCtxt < 'b > , Lifted : Print < 'b , FmtPrinter < 'a , 'b > > > , { self . short_string_namespace (t , path , hir :: def :: Namespace :: TypeNS) } # [doc = " When calling this after a `Diag` is constructed, the preferred way of doing so is"] # [doc = " `tcx.short_string(ty, diag.long_ty_path())`. The diagnostic itself is the one that keeps"] # [doc = " the existence of a \"long type\" anywhere in the diagnostic, so the note telling the user"] # [doc = " where we wrote the file to is only printed once."] pub fn short_string_namespace < T > (self , t : T , path : & mut Option < PathBuf > , namespace : hir :: def :: Namespace ,) -> String where T : Copy + Hash + for < 'a , 'b > Lift < TyCtxt < 'b > , Lifted : Print < 'b , FmtPrinter < 'a , 'b > > > , { let regular = FmtPrinter :: print_string (self , namespace , | p | { self . lift (t) . expect ("could not lift for printing") . print (p) }) . expect ("could not write to `String`") ; if ! self . sess . opts . unstable_opts . write_long_types_to_disk || self . sess . opts . verbose { return regular ; } let width = self . sess . diagnostic_width () ; let length_limit = width / 2 ; if regular . len () <= width * 2 / 3 { return regular ; } let short = self . string_with_limit (t , length_limit , namespace) ; if regular == short { return regular ; } let mut s = DefaultHasher :: new () ; t . hash (& mut s) ; let hash = s . finish () ; * path = Some (path . take () . unwrap_or_else (| | { self . output_filenames (()) . temp_path_for_diagnostic (& format ! ("long-type-{hash}.txt")) })) ; let Ok (mut file) = File :: options () . create (true) . read (true) . append (true) . open (& path . as_ref () . unwrap ()) else { return regular ; } ; let mut contents = String :: new () ; let _ = file . read_to_string (& mut contents) ; if let Some (_) = contents . lines () . find (| line | line == & regular) { return short ; } match write ! (file , "{regular}\n") { Ok (_) => short , Err (_) => regular , } } }}}