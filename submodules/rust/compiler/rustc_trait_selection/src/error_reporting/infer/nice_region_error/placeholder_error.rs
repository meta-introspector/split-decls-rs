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
mkuse!{use std :: fmt ;}
mkuse!{use rustc_data_structures :: intern :: Interned ;}
mkuse!{use rustc_errors :: { Diag , IntoDiagArg } ;}
mkuse!{use rustc_hir :: def :: Namespace ;}
mkuse!{use rustc_hir :: def_id :: { CRATE_DEF_ID , DefId } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: error :: ExpectedFound ;}
mkuse!{use rustc_middle :: ty :: print :: { FmtPrinter , Print , PrintTraitRefExt as _ , RegionHighlightMode } ;}
mkuse!{use rustc_middle :: ty :: { self , GenericArgsRef , RePlaceholder , Region , TyCtxt } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use crate :: error_reporting :: infer :: nice_region_error :: NiceRegionError ;}
mkuse!{use crate :: errors :: { ActualImplExpectedKind , ActualImplExpectedLifetimeKind , ActualImplExplNotes , TraitPlaceholderMismatch , TyOrSig , } ;}
mkuse!{use crate :: infer :: { RegionResolutionError , SubregionOrigin , TypeTrace , ValuePairs } ;}
mkuse!{use crate :: traits :: { ObligationCause , ObligationCauseCode } ;}
mkitem!{mkstruct!{#[derive (Copy , Clone)] pub struct Highlighted < 'tcx , T > { pub tcx : TyCtxt < 'tcx > , pub highlight : RegionHighlightMode < 'tcx > , pub value : T , pub ns : Namespace , }}}
mkitem!{mkimpl!{impl < 'tcx , T > IntoDiagArg for Highlighted < 'tcx , T > where T : for < 'a > Print < 'tcx , FmtPrinter < 'a , 'tcx > > , { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> rustc_errors :: DiagArgValue { rustc_errors :: DiagArgValue :: Str (self . to_string () . into ()) } }}}
mkitem!{mkimpl!{impl < 'tcx , T > Highlighted < 'tcx , T > { fn map < U > (self , f : impl FnOnce (T) -> U) -> Highlighted < 'tcx , U > { Highlighted { tcx : self . tcx , highlight : self . highlight , value : f (self . value) , ns : self . ns } } }}}
mkitem!{mkimpl!{impl < 'tcx , T > fmt :: Display for Highlighted < 'tcx , T > where T : for < 'a > Print < 'tcx , FmtPrinter < 'a , 'tcx > > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut p = ty :: print :: FmtPrinter :: new (self . tcx , self . ns) ; p . region_highlight_mode = self . highlight ; self . value . print (& mut p) ? ; f . write_str (& p . into_buffer ()) } }}}
mkitem!{mkimpl!{impl < 'tcx > NiceRegionError < '_ , 'tcx > { #[doc = " When given a `ConcreteFailure` for a function with arguments containing a named region and"] #[doc = " an anonymous region, emit a descriptive diagnostic error."] pub (super) fn try_report_placeholder_conflict (& self) -> Option < Diag < 'tcx > > { match & self . error { Some (RegionResolutionError :: SubSupConflict (vid , _ , SubregionOrigin :: Subtype (box TypeTrace { cause , values }) , sub_placeholder @ Region (Interned (RePlaceholder (_) , _)) , _ , sup_placeholder @ Region (Interned (RePlaceholder (_) , _)) , _ ,)) => self . try_report_trait_placeholder_mismatch (Some (ty :: Region :: new_var (self . tcx () , * vid)) , cause , Some (* sub_placeholder) , Some (* sup_placeholder) , values ,) , Some (RegionResolutionError :: SubSupConflict (vid , _ , SubregionOrigin :: Subtype (box TypeTrace { cause , values }) , sub_placeholder @ Region (Interned (RePlaceholder (_) , _)) , _ , _ , _ ,)) => self . try_report_trait_placeholder_mismatch (Some (ty :: Region :: new_var (self . tcx () , * vid)) , cause , Some (* sub_placeholder) , None , values ,) , Some (RegionResolutionError :: SubSupConflict (vid , _ , SubregionOrigin :: Subtype (box TypeTrace { cause , values }) , _ , _ , sup_placeholder @ Region (Interned (RePlaceholder (_) , _)) , _ ,)) => self . try_report_trait_placeholder_mismatch (Some (ty :: Region :: new_var (self . tcx () , * vid)) , cause , None , Some (* sup_placeholder) , values ,) , Some (RegionResolutionError :: SubSupConflict (vid , _ , _ , _ , SubregionOrigin :: Subtype (box TypeTrace { cause , values }) , sup_placeholder @ Region (Interned (RePlaceholder (_) , _)) , _ ,)) => self . try_report_trait_placeholder_mismatch (Some (ty :: Region :: new_var (self . tcx () , * vid)) , cause , None , Some (* sup_placeholder) , values ,) , Some (RegionResolutionError :: UpperBoundUniverseConflict (vid , _ , _ , SubregionOrigin :: Subtype (box TypeTrace { cause , values }) , sup_placeholder @ Region (Interned (RePlaceholder (_) , _)) ,)) => self . try_report_trait_placeholder_mismatch (Some (ty :: Region :: new_var (self . tcx () , * vid)) , cause , None , Some (* sup_placeholder) , values ,) , Some (RegionResolutionError :: ConcreteFailure (SubregionOrigin :: Subtype (box TypeTrace { cause , values }) , sub_region @ Region (Interned (RePlaceholder (_) , _)) , sup_region @ Region (Interned (RePlaceholder (_) , _)) ,)) => self . try_report_trait_placeholder_mismatch (None , cause , Some (* sub_region) , Some (* sup_region) , values ,) , Some (RegionResolutionError :: ConcreteFailure (SubregionOrigin :: Subtype (box TypeTrace { cause , values }) , sub_region @ Region (Interned (RePlaceholder (_) , _)) , sup_region ,)) => self . try_report_trait_placeholder_mismatch ((! sup_region . is_named (self . tcx ())) . then_some (* sup_region) , cause , Some (* sub_region) , None , values ,) , Some (RegionResolutionError :: ConcreteFailure (SubregionOrigin :: Subtype (box TypeTrace { cause , values }) , sub_region , sup_region @ Region (Interned (RePlaceholder (_) , _)) ,)) => self . try_report_trait_placeholder_mismatch ((! sub_region . is_named (self . tcx ())) . then_some (* sub_region) , cause , None , Some (* sup_region) , values ,) , _ => None , } } fn try_report_trait_placeholder_mismatch (& self , vid : Option < Region < 'tcx > > , cause : & ObligationCause < 'tcx > , sub_placeholder : Option < Region < 'tcx > > , sup_placeholder : Option < Region < 'tcx > > , value_pairs : & ValuePairs < 'tcx > ,) -> Option < Diag < 'tcx > > { let (expected_args , found_args , trait_def_id) = match value_pairs { ValuePairs :: TraitRefs (ExpectedFound { expected , found }) if expected . def_id == found . def_id => { (expected . args , found . args , expected . def_id) } _ => return None , } ; Some (self . report_trait_placeholder_mismatch (vid , cause , sub_placeholder , sup_placeholder , trait_def_id , expected_args , found_args ,)) } #[instrument (level = "debug" , skip (self))] fn report_trait_placeholder_mismatch (& self , vid : Option < Region < 'tcx > > , cause : & ObligationCause < 'tcx > , sub_placeholder : Option < Region < 'tcx > > , sup_placeholder : Option < Region < 'tcx > > , trait_def_id : DefId , expected_args : GenericArgsRef < 'tcx > , actual_args : GenericArgsRef < 'tcx > ,) -> Diag < 'tcx > { let span = cause . span ; let (leading_ellipsis , satisfy_span , where_span , dup_span , def_id) = if let ObligationCauseCode :: WhereClause (def_id , span) | ObligationCauseCode :: WhereClauseInExpr (def_id , span , ..) = * cause . code () && def_id != CRATE_DEF_ID . to_def_id () { (true , Some (span) , Some (self . tcx () . def_span (def_id)) , None , self . tcx () . def_path_str (def_id) ,) } else { (false , None , None , Some (span) , String :: new ()) } ; let expected_trait_ref = self . cx . resolve_vars_if_possible (ty :: TraitRef :: new_from_args (self . cx . tcx , trait_def_id , expected_args ,)) ; let actual_trait_ref = self . cx . resolve_vars_if_possible (ty :: TraitRef :: new_from_args (self . cx . tcx , trait_def_id , actual_args ,)) ; let mut counter = 0 ; let mut has_sub = None ; let mut has_sup = None ; let mut actual_has_vid = None ; let mut expected_has_vid = None ; self . tcx () . for_each_free_region (& expected_trait_ref , | r | { if Some (r) == sub_placeholder && has_sub . is_none () { has_sub = Some (counter) ; counter += 1 ; } else if Some (r) == sup_placeholder && has_sup . is_none () { has_sup = Some (counter) ; counter += 1 ; } if Some (r) == vid && expected_has_vid . is_none () { expected_has_vid = Some (counter) ; counter += 1 ; } }) ; self . tcx () . for_each_free_region (& actual_trait_ref , | r | { if Some (r) == vid && actual_has_vid . is_none () { actual_has_vid = Some (counter) ; counter += 1 ; } }) ; let actual_self_ty_has_vid = self . tcx () . any_free_region_meets (& actual_trait_ref . self_ty () , | r | Some (r) == vid) ; let expected_self_ty_has_vid = self . tcx () . any_free_region_meets (& expected_trait_ref . self_ty () , | r | Some (r) == vid) ; let any_self_ty_has_vid = actual_self_ty_has_vid || expected_self_ty_has_vid ; debug ! (? actual_has_vid , ? expected_has_vid , ? has_sub , ? has_sup , ? actual_self_ty_has_vid , ? expected_self_ty_has_vid ,) ; let actual_impl_expl_notes = self . explain_actual_impl_that_was_found (sub_placeholder , sup_placeholder , has_sub , has_sup , expected_trait_ref , actual_trait_ref , vid , expected_has_vid , actual_has_vid , any_self_ty_has_vid , leading_ellipsis ,) ; self . tcx () . dcx () . create_err (TraitPlaceholderMismatch { span , satisfy_span , where_span , dup_span , def_id , trait_def_id : self . tcx () . def_path_str (trait_def_id) , actual_impl_expl_notes , }) } #[doc = " Add notes with details about the expected and actual trait refs, with attention to cases"] #[doc = " when placeholder regions are involved: either the trait or the self type containing"] #[doc = " them needs to be mentioned the closest to the placeholders."] #[doc = " This makes the error messages read better, however at the cost of some complexity"] #[doc = " due to the number of combinations we have to deal with."] fn explain_actual_impl_that_was_found (& self , sub_placeholder : Option < Region < 'tcx > > , sup_placeholder : Option < Region < 'tcx > > , has_sub : Option < usize > , has_sup : Option < usize > , expected_trait_ref : ty :: TraitRef < 'tcx > , actual_trait_ref : ty :: TraitRef < 'tcx > , vid : Option < Region < 'tcx > > , expected_has_vid : Option < usize > , actual_has_vid : Option < usize > , any_self_ty_has_vid : bool , leading_ellipsis : bool ,) -> Vec < ActualImplExplNotes < 'tcx > > { let highlight_trait_ref = | trait_ref | Highlighted { tcx : self . tcx () , highlight : RegionHighlightMode :: default () , value : trait_ref , ns : Namespace :: TypeNS , } ; let same_self_type = actual_trait_ref . self_ty () == expected_trait_ref . self_ty () ; let mut expected_trait_ref = highlight_trait_ref (expected_trait_ref) ; expected_trait_ref . highlight . maybe_highlighting_region (sub_placeholder , has_sub) ; expected_trait_ref . highlight . maybe_highlighting_region (sup_placeholder , has_sup) ; let passive_voice = match (has_sub , has_sup) { (Some (_) , _) | (_ , Some (_)) => any_self_ty_has_vid , (None , None) => { expected_trait_ref . highlight . maybe_highlighting_region (vid , expected_has_vid) ; match expected_has_vid { Some (_) => true , None => any_self_ty_has_vid , } } } ; let (kind , ty_or_sig , trait_path) = if same_self_type { let mut self_ty = expected_trait_ref . map (| tr | tr . self_ty ()) ; self_ty . highlight . maybe_highlighting_region (vid , actual_has_vid) ; if self_ty . value . is_closure () && self . tcx () . is_fn_trait (expected_trait_ref . value . def_id) { let closure_sig = self_ty . map (| closure | { if let ty :: Closure (_ , args) = closure . kind () { self . tcx () . signature_unclosure (args . as_closure () . sig () , rustc_hir :: Safety :: Safe) } else { bug ! ("type is not longer closure") ; } }) ; (ActualImplExpectedKind :: Signature , TyOrSig :: ClosureSig (closure_sig) , expected_trait_ref . map (| tr | tr . print_only_trait_path ()) ,) } else { (ActualImplExpectedKind :: Other , TyOrSig :: Ty (self_ty) , expected_trait_ref . map (| tr | tr . print_only_trait_path ()) ,) } } else if passive_voice { (ActualImplExpectedKind :: Passive , TyOrSig :: Ty (expected_trait_ref . map (| tr | tr . self_ty ())) , expected_trait_ref . map (| tr | tr . print_only_trait_path ()) ,) } else { (ActualImplExpectedKind :: Other , TyOrSig :: Ty (expected_trait_ref . map (| tr | tr . self_ty ())) , expected_trait_ref . map (| tr | tr . print_only_trait_path ()) ,) } ; let (lt_kind , lifetime_1 , lifetime_2) = match (has_sub , has_sup) { (Some (n1) , Some (n2)) => { (ActualImplExpectedLifetimeKind :: Two , std :: cmp :: min (n1 , n2) , std :: cmp :: max (n1 , n2)) } (Some (n) , _) | (_ , Some (n)) => (ActualImplExpectedLifetimeKind :: Any , n , 0) , (None , None) => { if let Some (n) = expected_has_vid { (ActualImplExpectedLifetimeKind :: Some , n , 0) } else { (ActualImplExpectedLifetimeKind :: Nothing , 0 , 0) } } } ; let note_1 = ActualImplExplNotes :: new_expected (kind , lt_kind , leading_ellipsis , ty_or_sig , trait_path , lifetime_1 , lifetime_2 ,) ; let mut actual_trait_ref = highlight_trait_ref (actual_trait_ref) ; actual_trait_ref . highlight . maybe_highlighting_region (vid , actual_has_vid) ; let passive_voice = match actual_has_vid { Some (_) => any_self_ty_has_vid , None => true , } ; let trait_path = actual_trait_ref . map (| tr | tr . print_only_trait_path ()) ; let ty = actual_trait_ref . map (| tr | tr . self_ty ()) . to_string () ; let has_lifetime = actual_has_vid . is_some () ; let lifetime = actual_has_vid . unwrap_or_default () ; let note_2 = if same_self_type { ActualImplExplNotes :: ButActuallyImplementsTrait { trait_path , has_lifetime , lifetime } } else if passive_voice { ActualImplExplNotes :: ButActuallyImplementedForTy { trait_path , ty , has_lifetime , lifetime , } } else { ActualImplExplNotes :: ButActuallyTyImplements { trait_path , ty , has_lifetime , lifetime } } ; vec ! [note_1 , note_2] } }}}