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
mkuse!{use std :: fmt ;}
mkuse!{use rustc_errors :: ErrorGuaranteed ;}
mkuse!{use rustc_infer :: infer :: canonical :: Canonical ;}
mkuse!{use rustc_infer :: infer :: outlives :: env :: RegionBoundPairs ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: mir :: { Body , ConstraintCategory } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt , TypeFoldable , Upcast } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use rustc_span :: def_id :: DefId ;}
mkuse!{use rustc_trait_selection :: solve :: NoSolution ;}
mkuse!{use rustc_trait_selection :: traits :: ObligationCause ;}
mkuse!{use rustc_trait_selection :: traits :: query :: type_op :: custom :: CustomTypeOp ;}
mkuse!{use rustc_trait_selection :: traits :: query :: type_op :: { self , TypeOpOutput } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use super :: { Locations , NormalizeLocation , TypeChecker } ;}
mkuse!{use crate :: BorrowckInferCtxt ;}
mkuse!{use crate :: diagnostics :: ToUniverseInfo ;}
mkuse!{use crate :: type_check :: { MirTypeckRegionConstraints , constraint_conversion } ;}
mkuse!{use crate :: universal_regions :: UniversalRegions ;}

macro_rules! fully_perform_op_raw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fully_perform_op_raw in module {}", module_path!());
    };
}

mkfn!{
    fully_perform_op_raw_introspect!();
    # [instrument (skip (infcx , constraints , op) , level = "trace")] pub (crate) fn fully_perform_op_raw < 'tcx , R : fmt :: Debug , Op > (infcx : & BorrowckInferCtxt < 'tcx > , body : & Body < 'tcx > , universal_regions : & UniversalRegions < 'tcx > , region_bound_pairs : & RegionBoundPairs < 'tcx > , known_type_outlives_obligations : & [ty :: PolyTypeOutlivesPredicate < 'tcx >] , constraints : & mut MirTypeckRegionConstraints < 'tcx > , locations : Locations , category : ConstraintCategory < 'tcx > , op : Op ,) -> Result < R , ErrorGuaranteed > where Op : type_op :: TypeOp < 'tcx , Output = R > , Op :: ErrorInfo : ToUniverseInfo < 'tcx > , { let old_universe = infcx . universe () ; let TypeOpOutput { output , constraints : query_constraints , error_info } = op . fully_perform (infcx , infcx . root_def_id , locations . span (body)) ? ; if cfg ! (debug_assertions) { let data = infcx . take_and_reset_region_constraints () ; if ! data . is_empty () { panic ! ("leftover region constraints: {data:#?}") ; } } debug ! (? output , ? query_constraints) ; if let Some (data) = query_constraints { constraint_conversion :: ConstraintConversion :: new (infcx , universal_regions , region_bound_pairs , known_type_outlives_obligations , locations , locations . span (body) , category , constraints ,) . convert_all (data) ; } let universe = infcx . universe () ; if old_universe != universe && let Some (error_info) = error_info { let universe_info = error_info . to_universe_info (old_universe) ; for u in (old_universe + 1) ..= universe { constraints . universe_causes . insert (u , universe_info . clone ()) ; } } Ok (output) }
}
mkitem!{mkimpl!{impl < 'a , 'tcx > TypeChecker < 'a , 'tcx > { # [doc = " Given some operation `op` that manipulates types, proves"] # [doc = " predicates, or otherwise uses the inference context, executes"] # [doc = " `op` and then executes all the further obligations that `op`"] # [doc = " returns. This will yield a set of outlives constraints amongst"] # [doc = " regions which are extracted and stored as having occurred at"] # [doc = " `locations`."] # [doc = ""] # [doc = " **Any `rustc_infer::infer` operations that might generate region"] # [doc = " constraints should occur within this method so that those"] # [doc = " constraints can be properly localized!**"] # [instrument (skip (self , op) , level = "trace")] pub (super) fn fully_perform_op < R : fmt :: Debug , Op > (& mut self , locations : Locations , category : ConstraintCategory < 'tcx > , op : Op ,) -> Result < R , ErrorGuaranteed > where Op : type_op :: TypeOp < 'tcx , Output = R > , Op :: ErrorInfo : ToUniverseInfo < 'tcx > , { fully_perform_op_raw (self . infcx , self . body , self . universal_regions , self . region_bound_pairs , self . known_type_outlives_obligations , self . constraints , locations , category , op ,) } pub (super) fn instantiate_canonical < T > (& mut self , span : Span , canonical : & Canonical < 'tcx , T > ,) -> T where T : TypeFoldable < TyCtxt < 'tcx > > , { let (instantiated , _) = self . infcx . instantiate_canonical (span , canonical) ; instantiated } # [instrument (skip (self) , level = "debug")] pub (super) fn prove_trait_ref (& mut self , trait_ref : ty :: TraitRef < 'tcx > , locations : Locations , category : ConstraintCategory < 'tcx > ,) { self . prove_predicate (ty :: Binder :: dummy (ty :: PredicateKind :: Clause (ty :: ClauseKind :: Trait (ty :: TraitPredicate { trait_ref , polarity : ty :: PredicatePolarity :: Positive } ,))) , locations , category ,) ; } # [instrument (level = "debug" , skip (self))] pub (super) fn normalize_and_prove_instantiated_predicates (& mut self , _def_id : DefId , instantiated_predicates : ty :: InstantiatedPredicates < 'tcx > , locations : Locations ,) { for (predicate , span) in instantiated_predicates { debug ! (? span , ? predicate) ; let category = ConstraintCategory :: Predicate (span) ; let predicate = self . normalize_with_category (predicate , locations , category) ; self . prove_predicate (predicate , locations , category) ; } } pub (super) fn prove_predicates (& mut self , predicates : impl IntoIterator < Item : Upcast < TyCtxt < 'tcx > , ty :: Predicate < 'tcx > > + std :: fmt :: Debug > , locations : Locations , category : ConstraintCategory < 'tcx > ,) { for predicate in predicates { self . prove_predicate (predicate , locations , category) ; } } # [instrument (skip (self) , level = "debug")] pub (super) fn prove_predicate (& mut self , predicate : impl Upcast < TyCtxt < 'tcx > , ty :: Predicate < 'tcx > > + std :: fmt :: Debug , locations : Locations , category : ConstraintCategory < 'tcx > ,) { let param_env = self . infcx . param_env ; let predicate = predicate . upcast (self . tcx ()) ; let _ : Result < _ , ErrorGuaranteed > = self . fully_perform_op (locations , category , param_env . and (type_op :: prove_predicate :: ProvePredicate { predicate }) ,) ; } pub (super) fn normalize < T > (& mut self , value : T , location : impl NormalizeLocation) -> T where T : type_op :: normalize :: Normalizable < 'tcx > + fmt :: Display + Copy + 'tcx , { self . normalize_with_category (value , location , ConstraintCategory :: Boring) } pub (super) fn deeply_normalize < T > (& mut self , value : T , location : impl NormalizeLocation) -> T where T : type_op :: normalize :: Normalizable < 'tcx > + fmt :: Display + Copy + 'tcx , { let result : Result < _ , ErrorGuaranteed > = self . fully_perform_op (location . to_locations () , ConstraintCategory :: Boring , self . infcx . param_env . and (type_op :: normalize :: DeeplyNormalize { value }) ,) ; result . unwrap_or (value) } # [instrument (skip (self) , level = "debug")] pub (super) fn normalize_with_category < T > (& mut self , value : T , location : impl NormalizeLocation , category : ConstraintCategory < 'tcx > ,) -> T where T : type_op :: normalize :: Normalizable < 'tcx > + fmt :: Display + Copy + 'tcx , { let param_env = self . infcx . param_env ; let result : Result < _ , ErrorGuaranteed > = self . fully_perform_op (location . to_locations () , category , param_env . and (type_op :: normalize :: Normalize { value }) ,) ; result . unwrap_or (value) } # [instrument (skip (self) , level = "debug")] pub (super) fn struct_tail (& mut self , ty : Ty < 'tcx > , location : impl NormalizeLocation ,) -> Ty < 'tcx > { let tcx = self . tcx () ; if self . infcx . next_trait_solver () { let body = self . body ; let param_env = self . infcx . param_env ; self . fully_perform_op (location . to_locations () , ConstraintCategory :: Boring , CustomTypeOp :: new (| ocx | { let structurally_normalize = | ty | { ocx . structurally_normalize_ty (& ObligationCause :: misc (location . to_locations () . span (body) , body . source . def_id () . expect_local () ,) , param_env , ty ,) . unwrap_or_else (| _ | bug ! ("struct tail should have been computable, since we computed it in HIR")) } ; let tail = tcx . struct_tail_raw (ty , structurally_normalize , | | { } ,) ; Ok (tail) } , "normalizing struct tail" ,) ,) . unwrap_or_else (| guar | Ty :: new_error (tcx , guar)) } else { let mut normalize = | ty | self . normalize (ty , location) ; let tail = tcx . struct_tail_raw (ty , & mut normalize , | | { }) ; normalize (tail) } } # [instrument (skip (self) , level = "debug")] pub (super) fn structurally_resolve (& mut self , ty : Ty < 'tcx > , location : impl NormalizeLocation ,) -> Ty < 'tcx > { if self . infcx . next_trait_solver () { let body = self . body ; let param_env = self . infcx . param_env ; self . fully_perform_op (location . to_locations () , ConstraintCategory :: Boring , CustomTypeOp :: new (| ocx | { ocx . structurally_normalize_ty (& ObligationCause :: misc (location . to_locations () . span (body) , body . source . def_id () . expect_local () ,) , param_env , ty ,) . map_err (| _ | NoSolution) } , "normalizing struct tail" ,) ,) . unwrap_or_else (| guar | Ty :: new_error (self . tcx () , guar)) } else { self . normalize (ty , location) } } # [instrument (skip (self) , level = "debug")] pub (super) fn ascribe_user_type (& mut self , mir_ty : Ty < 'tcx > , user_ty : ty :: UserType < 'tcx > , span : Span ,) { let _ : Result < _ , ErrorGuaranteed > = self . fully_perform_op (Locations :: All (span) , ConstraintCategory :: Boring , self . infcx . param_env . and (type_op :: ascribe_user_type :: AscribeUserType { mir_ty , user_ty }) ,) ; } # [doc = " *Incorrectly* skips the WF checks we normally do in `ascribe_user_type`."] # [doc = ""] # [doc = " FIXME(#104478, #104477): This is a hack for backward-compatibility."] # [instrument (skip (self) , level = "debug")] pub (super) fn ascribe_user_type_skip_wf (& mut self , mir_ty : Ty < 'tcx > , user_ty : ty :: UserType < 'tcx > , span : Span ,) { let ty :: UserTypeKind :: Ty (user_ty) = user_ty . kind else { bug ! () } ; if let ty :: Infer (_) = user_ty . kind () { self . eq_types (user_ty , mir_ty , Locations :: All (span) , ConstraintCategory :: Boring) . unwrap () ; return ; } let mir_ty = self . normalize (mir_ty , Locations :: All (span)) ; let cause = ObligationCause :: dummy_with_span (span) ; let param_env = self . infcx . param_env ; let _ : Result < _ , ErrorGuaranteed > = self . fully_perform_op (Locations :: All (span) , ConstraintCategory :: Boring , type_op :: custom :: CustomTypeOp :: new (| ocx | { let user_ty = ocx . normalize (& cause , param_env , user_ty) ; ocx . eq (& cause , param_env , user_ty , mir_ty) ? ; Ok (()) } , "ascribe_user_type_skip_wf" ,) ,) ; } }}}