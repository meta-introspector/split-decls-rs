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
mkuse!{use rustc_data_structures :: intern :: Interned ;}
mkuse!{use rustc_errors :: MultiSpan ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_macros :: { HashStable , TyDecodable , TyEncodable } ;}
mkuse!{use rustc_span :: { DUMMY_SP , ErrorGuaranteed , Symbol , kw , sym } ;}
mkuse!{use rustc_type_ir :: RegionKind as IrRegionKind ;}
mkuse!{pub use rustc_type_ir :: RegionVid ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: ty :: { self , BoundVar , TyCtxt , TypeFlags } ;}
mkitem!{pub type RegionKind < 'tcx > = IrRegionKind < TyCtxt < 'tcx > > ;}
mkitem!{mkstruct!{#[doc = " Use this rather than `RegionKind`, whenever possible."] #[derive (Copy , Clone , PartialEq , Eq , Hash , HashStable)] #[rustc_pass_by_value] pub struct Region < 'tcx > (pub Interned < 'tcx , RegionKind < 'tcx > >) ;}}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: inherent :: IntoKind for Region < 'tcx > { type Kind = RegionKind < 'tcx > ; fn kind (self) -> RegionKind < 'tcx > { * self . 0 . 0 } }}}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: Flags for Region < 'tcx > { fn flags (& self) -> TypeFlags { self . type_flags () } fn outer_exclusive_binder (& self) -> ty :: DebruijnIndex { match self . kind () { ty :: ReBound (debruijn , _) => debruijn . shifted_in (1) , _ => ty :: INNERMOST , } } }}}
mkitem!{mkimpl!{impl < 'tcx > Region < 'tcx > { #[inline] pub fn new_early_param (tcx : TyCtxt < 'tcx > , early_bound_region : ty :: EarlyParamRegion ,) -> Region < 'tcx > { tcx . intern_region (ty :: ReEarlyParam (early_bound_region)) } #[inline] pub fn new_bound (tcx : TyCtxt < 'tcx > , debruijn : ty :: DebruijnIndex , bound_region : ty :: BoundRegion ,) -> Region < 'tcx > { if let ty :: BoundRegion { var , kind : ty :: BoundRegionKind :: Anon } = bound_region && let Some (inner) = tcx . lifetimes . anon_re_bounds . get (debruijn . as_usize ()) && let Some (re) = inner . get (var . as_usize ()) . copied () { re } else { tcx . intern_region (ty :: ReBound (debruijn , bound_region)) } } #[inline] pub fn new_late_param (tcx : TyCtxt < 'tcx > , scope : DefId , kind : LateParamRegionKind ,) -> Region < 'tcx > { let data = LateParamRegion { scope , kind } ; tcx . intern_region (ty :: ReLateParam (data)) } #[inline] pub fn new_var (tcx : TyCtxt < 'tcx > , v : ty :: RegionVid) -> Region < 'tcx > { tcx . lifetimes . re_vars . get (v . as_usize ()) . copied () . unwrap_or_else (| | tcx . intern_region (ty :: ReVar (v))) } #[inline] pub fn new_placeholder (tcx : TyCtxt < 'tcx > , placeholder : ty :: PlaceholderRegion) -> Region < 'tcx > { tcx . intern_region (ty :: RePlaceholder (placeholder)) } #[doc = " Constructs a `RegionKind::ReError` region."] #[track_caller] pub fn new_error (tcx : TyCtxt < 'tcx > , guar : ErrorGuaranteed) -> Region < 'tcx > { tcx . intern_region (ty :: ReError (guar)) } #[doc = " Constructs a `RegionKind::ReError` region and registers a delayed bug to ensure it gets"] #[doc = " used."] #[track_caller] pub fn new_error_misc (tcx : TyCtxt < 'tcx >) -> Region < 'tcx > { Region :: new_error_with_message (tcx , DUMMY_SP , "RegionKind::ReError constructed but no error reported" ,) } #[doc = " Constructs a `RegionKind::ReError` region and registers a delayed bug with the given `msg`"] #[doc = " to ensure it gets used."] #[track_caller] pub fn new_error_with_message < S : Into < MultiSpan > > (tcx : TyCtxt < 'tcx > , span : S , msg : & 'static str ,) -> Region < 'tcx > { let reported = tcx . dcx () . span_delayed_bug (span , msg) ; Region :: new_error (tcx , reported) } #[doc = " Avoid this in favour of more specific `new_*` methods, where possible,"] #[doc = " to avoid the cost of the `match`."] pub fn new_from_kind (tcx : TyCtxt < 'tcx > , kind : RegionKind < 'tcx >) -> Region < 'tcx > { match kind { ty :: ReEarlyParam (region) => Region :: new_early_param (tcx , region) , ty :: ReBound (debruijn , region) => Region :: new_bound (tcx , debruijn , region) , ty :: ReLateParam (ty :: LateParamRegion { scope , kind }) => { Region :: new_late_param (tcx , scope , kind) } ty :: ReStatic => tcx . lifetimes . re_static , ty :: ReVar (vid) => Region :: new_var (tcx , vid) , ty :: RePlaceholder (region) => Region :: new_placeholder (tcx , region) , ty :: ReErased => tcx . lifetimes . re_erased , ty :: ReError (reported) => Region :: new_error (tcx , reported) , } } }}}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: inherent :: Region < TyCtxt < 'tcx > > for Region < 'tcx > { fn new_bound (interner : TyCtxt < 'tcx > , debruijn : ty :: DebruijnIndex , var : ty :: BoundRegion ,) -> Self { Region :: new_bound (interner , debruijn , var) } fn new_anon_bound (tcx : TyCtxt < 'tcx > , debruijn : ty :: DebruijnIndex , var : ty :: BoundVar) -> Self { Region :: new_bound (tcx , debruijn , ty :: BoundRegion { var , kind : ty :: BoundRegionKind :: Anon }) } fn new_placeholder (tcx : TyCtxt < 'tcx > , placeholder : ty :: PlaceholderRegion) -> Self { Region :: new_placeholder (tcx , placeholder) } fn new_static (tcx : TyCtxt < 'tcx >) -> Self { tcx . lifetimes . re_static } }}}
mkitem!{mkimpl!{#[doc = " Region utilities"] impl < 'tcx > Region < 'tcx > { pub fn kind (self) -> RegionKind < 'tcx > { * self . 0 . 0 } pub fn get_name (self , tcx : TyCtxt < 'tcx >) -> Option < Symbol > { match self . kind () { ty :: ReEarlyParam (ebr) => ebr . is_named () . then_some (ebr . name) , ty :: ReBound (_ , br) => br . kind . get_name (tcx) , ty :: ReLateParam (fr) => fr . kind . get_name (tcx) , ty :: ReStatic => Some (kw :: StaticLifetime) , ty :: RePlaceholder (placeholder) => placeholder . bound . kind . get_name (tcx) , _ => None , } } pub fn get_name_or_anon (self , tcx : TyCtxt < 'tcx >) -> Symbol { match self . get_name (tcx) { Some (name) => name , None => sym :: anon , } } #[doc = " Is this region named by the user?"] pub fn is_named (self , tcx : TyCtxt < 'tcx >) -> bool { match self . kind () { ty :: ReEarlyParam (ebr) => ebr . is_named () , ty :: ReBound (_ , br) => br . kind . is_named (tcx) , ty :: ReLateParam (fr) => fr . kind . is_named (tcx) , ty :: ReStatic => true , ty :: ReVar (..) => false , ty :: RePlaceholder (placeholder) => placeholder . bound . kind . is_named (tcx) , ty :: ReErased => false , ty :: ReError (_) => false , } } #[inline] pub fn is_error (self) -> bool { matches ! (self . kind () , ty :: ReError (_)) } #[inline] pub fn is_static (self) -> bool { matches ! (self . kind () , ty :: ReStatic) } #[inline] pub fn is_erased (self) -> bool { matches ! (self . kind () , ty :: ReErased) } #[inline] pub fn is_bound (self) -> bool { matches ! (self . kind () , ty :: ReBound (..)) } #[inline] pub fn is_placeholder (self) -> bool { matches ! (self . kind () , ty :: RePlaceholder (..)) } #[inline] pub fn bound_at_or_above_binder (self , index : ty :: DebruijnIndex) -> bool { match self . kind () { ty :: ReBound (debruijn , _) => debruijn >= index , _ => false , } } pub fn type_flags (self) -> TypeFlags { let mut flags = TypeFlags :: empty () ; match self . kind () { ty :: ReVar (..) => { flags = flags | TypeFlags :: HAS_FREE_REGIONS ; flags = flags | TypeFlags :: HAS_FREE_LOCAL_REGIONS ; flags = flags | TypeFlags :: HAS_RE_INFER ; } ty :: RePlaceholder (..) => { flags = flags | TypeFlags :: HAS_FREE_REGIONS ; flags = flags | TypeFlags :: HAS_FREE_LOCAL_REGIONS ; flags = flags | TypeFlags :: HAS_RE_PLACEHOLDER ; } ty :: ReEarlyParam (..) => { flags = flags | TypeFlags :: HAS_FREE_REGIONS ; flags = flags | TypeFlags :: HAS_FREE_LOCAL_REGIONS ; flags = flags | TypeFlags :: HAS_RE_PARAM ; } ty :: ReLateParam { .. } => { flags = flags | TypeFlags :: HAS_FREE_REGIONS ; flags = flags | TypeFlags :: HAS_FREE_LOCAL_REGIONS ; } ty :: ReStatic => { flags = flags | TypeFlags :: HAS_FREE_REGIONS ; } ty :: ReBound (..) => { flags = flags | TypeFlags :: HAS_RE_BOUND ; } ty :: ReErased => { flags = flags | TypeFlags :: HAS_RE_ERASED ; } ty :: ReError (_) => { flags = flags | TypeFlags :: HAS_FREE_REGIONS ; flags = flags | TypeFlags :: HAS_ERROR ; } } debug ! ("type_flags({:?}) = {:?}" , self , flags) ; flags } #[doc = " True for free regions other than `'static`."] pub fn is_param (self) -> bool { matches ! (self . kind () , ty :: ReEarlyParam (_) | ty :: ReLateParam (_)) } #[doc = " True for free region in the current context."] #[doc = ""] #[doc = " This is the case for `'static` and param regions."] pub fn is_free (self) -> bool { match self . kind () { ty :: ReStatic | ty :: ReEarlyParam (..) | ty :: ReLateParam (..) => true , ty :: ReVar (..) | ty :: RePlaceholder (..) | ty :: ReBound (..) | ty :: ReErased | ty :: ReError (..) => false , } } pub fn is_var (self) -> bool { matches ! (self . kind () , ty :: ReVar (_)) } pub fn as_var (self) -> RegionVid { match self . kind () { ty :: ReVar (vid) => vid , _ => bug ! ("expected region {:?} to be of kind ReVar" , self) , } } #[doc = " Given some item `binding_item`, check if this region is a generic parameter introduced by it"] #[doc = " or one of the parent generics. Returns the `DefId` of the parameter definition if so."] pub fn opt_param_def_id (self , tcx : TyCtxt < 'tcx > , binding_item : DefId) -> Option < DefId > { match self . kind () { ty :: ReEarlyParam (ebr) => { Some (tcx . generics_of (binding_item) . region_param (ebr , tcx) . def_id) } ty :: ReLateParam (ty :: LateParamRegion { kind : ty :: LateParamRegionKind :: Named (def_id) , .. }) => Some (def_id) , _ => None , } } }}}
mkitem!{mkstruct!{#[derive (Copy , Clone , PartialEq , Eq , Hash , TyEncodable , TyDecodable)] #[derive (HashStable)] pub struct EarlyParamRegion { pub index : u32 , pub name : Symbol , }}}
mkitem!{mkimpl!{impl EarlyParamRegion { #[doc = " Does this early bound region have a name? Early bound regions normally"] #[doc = " always have names except when using anonymous lifetimes (`'_`)."] pub fn is_named (& self) -> bool { self . name != kw :: UnderscoreLifetime } }}}
mkitem!{mkimpl!{impl rustc_type_ir :: inherent :: ParamLike for EarlyParamRegion { fn index (self) -> u32 { self . index } }}}
mkitem!{mkimpl!{impl std :: fmt :: Debug for EarlyParamRegion { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}/#{}" , self . name , self . index) } }}}
mkitem!{mkstruct!{#[derive (Clone , PartialEq , Eq , Hash , TyEncodable , TyDecodable , Copy)] #[derive (HashStable)] #[doc = " The parameter representation of late-bound function parameters, \"some region"] #[doc = " at least as big as the scope `fr.scope`\"."] #[doc = ""] #[doc = " Similar to a placeholder region as we create `LateParam` regions when entering a binder"] #[doc = " except they are always in the root universe and instead of using a boundvar to distinguish"] #[doc = " between others we use the `DefId` of the parameter. For this reason the `bound_region` field"] #[doc = " should basically always be `BoundRegionKind::Named` as otherwise there is no way of telling"] #[doc = " different parameters apart."] pub struct LateParamRegion { pub scope : DefId , pub kind : LateParamRegionKind , }}}
mkitem!{mkenum!{#[doc = " When liberating bound regions, we map their [`BoundRegionKind`]"] #[doc = " to this as we need to track the index of anonymous regions. We"] #[doc = " otherwise end up liberating multiple bound regions to the same"] #[doc = " late-bound region."] #[derive (Clone , PartialEq , Eq , Hash , TyEncodable , TyDecodable , Copy)] #[derive (HashStable)] pub enum LateParamRegionKind { #[doc = " An anonymous region parameter for a given fn (&T)"] #[doc = ""] #[doc = " Unlike [`BoundRegionKind::Anon`], this tracks the index of the"] #[doc = " liberated bound region."] #[doc = ""] #[doc = " We should ideally never liberate anonymous regions, but do so for the"] #[doc = " sake of diagnostics in `FnCtxt::sig_of_closure_with_expectation`."] Anon (u32) , #[doc = " An anonymous region parameter with a `Symbol` name."] #[doc = ""] #[doc = " Used to give late-bound regions names for things like pretty printing."] NamedAnon (u32 , Symbol) , #[doc = " Late-bound regions that appear in the AST."] Named (DefId) , #[doc = " Anonymous region for the implicit env pointer parameter"] #[doc = " to a closure"] ClosureEnv , }}}
mkitem!{mkimpl!{impl LateParamRegionKind { pub fn from_bound (var : BoundVar , br : BoundRegionKind) -> LateParamRegionKind { match br { BoundRegionKind :: Anon => LateParamRegionKind :: Anon (var . as_u32 ()) , BoundRegionKind :: Named (def_id) => LateParamRegionKind :: Named (def_id) , BoundRegionKind :: ClosureEnv => LateParamRegionKind :: ClosureEnv , BoundRegionKind :: NamedAnon (name) => LateParamRegionKind :: NamedAnon (var . as_u32 () , name) , } } pub fn is_named (& self , tcx : TyCtxt < '_ >) -> bool { self . get_name (tcx) . is_some () } pub fn get_name (& self , tcx : TyCtxt < '_ >) -> Option < Symbol > { match * self { LateParamRegionKind :: Named (def_id) => { let name = tcx . item_name (def_id) ; if name != kw :: UnderscoreLifetime { Some (name) } else { None } } LateParamRegionKind :: NamedAnon (_ , name) => Some (name) , _ => None , } } pub fn get_id (& self) -> Option < DefId > { match * self { LateParamRegionKind :: Named (id) => Some (id) , _ => None , } } }}}
mkitem!{mkenum!{#[derive (Clone , PartialEq , Eq , Hash , TyEncodable , TyDecodable , Copy)] #[derive (HashStable)] pub enum BoundRegionKind { #[doc = " An anonymous region parameter for a given fn (&T)"] Anon , #[doc = " An anonymous region parameter with a `Symbol` name."] #[doc = ""] #[doc = " Used to give late-bound regions names for things like pretty printing."] NamedAnon (Symbol) , #[doc = " Late-bound regions that appear in the AST."] Named (DefId) , #[doc = " Anonymous region for the implicit env pointer parameter"] #[doc = " to a closure"] ClosureEnv , }}}
mkitem!{mkstruct!{#[derive (Copy , Clone , PartialEq , Eq , Hash , TyEncodable , TyDecodable)] #[derive (HashStable)] pub struct BoundRegion { pub var : BoundVar , pub kind : BoundRegionKind , }}}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: inherent :: BoundVarLike < TyCtxt < 'tcx > > for BoundRegion { fn var (self) -> BoundVar { self . var } fn assert_eq (self , var : ty :: BoundVariableKind) { assert_eq ! (self . kind , var . expect_region ()) } }}}
mkitem!{mkimpl!{impl core :: fmt :: Debug for BoundRegion { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self . kind { BoundRegionKind :: Anon => write ! (f , "{:?}" , self . var) , BoundRegionKind :: ClosureEnv => write ! (f , "{:?}.Env" , self . var) , BoundRegionKind :: Named (def) => { write ! (f , "{:?}.Named({:?})" , self . var , def) } BoundRegionKind :: NamedAnon (symbol) => { write ! (f , "{:?}.NamedAnon({:?})" , self . var , symbol) } } } }}}
mkitem!{mkimpl!{impl BoundRegionKind { pub fn is_named (& self , tcx : TyCtxt < '_ >) -> bool { self . get_name (tcx) . is_some () } pub fn get_name (& self , tcx : TyCtxt < '_ >) -> Option < Symbol > { match * self { BoundRegionKind :: Named (def_id) => { let name = tcx . item_name (def_id) ; if name != kw :: UnderscoreLifetime { Some (name) } else { None } } BoundRegionKind :: NamedAnon (name) => Some (name) , _ => None , } } pub fn get_id (& self) -> Option < DefId > { match * self { BoundRegionKind :: Named (id) => Some (id) , _ => None , } } }}}
mkmod!{size_asserts, { 
                getname!(size_asserts);
                getsrc!(size_asserts);
                getpath!(size_asserts);
                get_deps!(size_asserts);
                get_crates!(size_asserts);
                mkinclude!(size_asserts);
                mkuse!{use rustc_data_structures :: static_assert_size ;}
mkuse!{use super :: * ;}
mkitem!{static_assert_size ! (RegionKind <'_ >, 20) ;}
mkitem!{static_assert_size ! (ty :: WithCachedTypeInfo < RegionKind <'_ >>, 48) ;} 
            }}