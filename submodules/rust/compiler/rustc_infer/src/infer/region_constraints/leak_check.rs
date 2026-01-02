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
mkuse!{use rustc_data_structures :: fx :: FxIndexMap ;}
mkuse!{use rustc_data_structures :: graph :: scc :: Sccs ;}
mkuse!{use rustc_data_structures :: graph :: vec_graph :: VecGraph ;}
mkuse!{use rustc_index :: Idx ;}
mkuse!{use rustc_middle :: span_bug ;}
mkuse!{use rustc_middle :: ty :: error :: TypeError ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use super :: * ;}
mkuse!{use crate :: infer :: relate :: RelateResult ;}
mkuse!{use crate :: infer :: snapshot :: CombinedSnapshot ;}
mkitem!{mkimpl!{impl < 'tcx > RegionConstraintCollector < '_ , 'tcx > { # [doc = " Searches new universes created during `snapshot`, looking for"] # [doc = " placeholders that may \"leak\" out from the universes they are contained"] # [doc = " in. If any leaking placeholders are found, then an `Err` is returned"] # [doc = " (typically leading to the snapshot being reversed). This algorithm"] # [doc = " only looks at placeholders which cannot be named by `outer_universe`,"] # [doc = " as this is the universe we're currently checking for a leak."] # [doc = ""] # [doc = " The leak check *used* to be the only way we had to handle higher-ranked"] # [doc = " obligations. Now that we have integrated universes into the region"] # [doc = " solvers, this is no longer the case, but we retain the leak check for"] # [doc = " backwards compatibility purposes. In particular, it lets us make \"early\""] # [doc = " decisions about whether a region error will be reported that are used in"] # [doc = " coherence and elsewhere -- see #56105 and #59490 for more details. The"] # [doc = " eventual fate of the leak checker is not yet settled."] # [doc = ""] # [doc = " The leak checker works by searching for the following error patterns:"] # [doc = ""] # [doc = " * P1: P2, where P1 != P2"] # [doc = " * P1: R, where R is in some universe that cannot name P1"] # [doc = ""] # [doc = " The idea here is that each of these patterns represents something that"] # [doc = " the region solver would eventually report as an error, so we can detect"] # [doc = " the error early. There is a fly in the ointment, though, in that this is"] # [doc = " not entirely true. In particular, in the future, we may extend the"] # [doc = " environment with implied bounds or other info about how placeholders"] # [doc = " relate to regions in outer universes. In that case, `P1: R` for example"] # [doc = " might become solvable."] # [doc = ""] # [doc = " # Summary of the implementation"] # [doc = ""] # [doc = " The leak checks as follows. First, we construct a graph where `R2: R1`"] # [doc = " implies `R2 -> R1`, and we compute the SCCs."] # [doc = ""] # [doc = " For each SCC S, we compute:"] # [doc = ""] # [doc = " * what placeholder P it must be equal to, if any"] # [doc = "   * if there are multiple placeholders that must be equal, report an error because `P1: P2`"] # [doc = " * the minimum universe of its constituents"] # [doc = ""] # [doc = " Then we walk the SCCs in dependency order and compute"] # [doc = ""] # [doc = " * what placeholder they must outlive transitively"] # [doc = "   * if they must also be equal to a placeholder, report an error because `P1: P2`"] # [doc = " * minimum universe U of all SCCs they must outlive"] # [doc = "   * if they must also be equal to a placeholder P, and U cannot name P, report an error, as"] # [doc = "     that indicates `P: R` and `R` is in an incompatible universe"] # [doc = ""] # [doc = " To improve performance and for the old trait solver caching to be sound, this takes"] # [doc = " an optional snapshot in which case we only look at region constraints added in that"] # [doc = " snapshot. If we were to not do that the `leak_check` during evaluation can rely on"] # [doc = " region constraints added outside of that evaluation. As that is not reflected in the"] # [doc = " cache key this would be unsound."] # [doc = ""] # [doc = " # Historical note"] # [doc = ""] # [doc = " Older variants of the leak check used to report errors for these"] # [doc = " patterns, but we no longer do:"] # [doc = ""] # [doc = " * R: P1, even if R cannot name P1, because R = 'static is a valid sol'n"] # [doc = " * R: P1, R: P2, as above"] # [instrument (level = "debug" , skip (self , tcx , only_consider_snapshot) , ret)] pub fn leak_check (self , tcx : TyCtxt < 'tcx > , outer_universe : ty :: UniverseIndex , max_universe : ty :: UniverseIndex , only_consider_snapshot : Option < & CombinedSnapshot < 'tcx > > ,) -> RelateResult < 'tcx , () > { if outer_universe == max_universe { return Ok (()) ; } let mini_graph = MiniGraph :: new (& self , only_consider_snapshot) ; let mut leak_check = LeakCheck :: new (tcx , outer_universe , max_universe , mini_graph , self) ; leak_check . assign_placeholder_values () ? ; leak_check . propagate_scc_value () ? ; Ok (()) } }}}
mkitem!{mkstruct!{struct LeakCheck < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , outer_universe : ty :: UniverseIndex , mini_graph : MiniGraph < 'tcx > , rcc : RegionConstraintCollector < 'a , 'tcx > , scc_placeholders : IndexVec < LeakCheckScc , Option < ty :: PlaceholderRegion > > , scc_universes : IndexVec < LeakCheckScc , SccUniverse < 'tcx > > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > LeakCheck < 'a , 'tcx > { fn new (tcx : TyCtxt < 'tcx > , outer_universe : ty :: UniverseIndex , max_universe : ty :: UniverseIndex , mini_graph : MiniGraph < 'tcx > , rcc : RegionConstraintCollector < 'a , 'tcx > ,) -> Self { let dummy_scc_universe = SccUniverse { universe : max_universe , region : None } ; let num_sccs = mini_graph . sccs . num_sccs () ; Self { tcx , outer_universe , mini_graph , rcc , scc_placeholders : IndexVec :: from_elem_n (None , num_sccs) , scc_universes : IndexVec :: from_elem_n (dummy_scc_universe , num_sccs) , } } # [doc = " Compute what placeholders (if any) each SCC must be equal to."] # [doc = " Also compute the minimum universe of all the regions in each SCC."] fn assign_placeholder_values (& mut self) -> RelateResult < 'tcx , () > { for (region , leak_check_node) in & self . mini_graph . nodes { let scc = self . mini_graph . sccs . scc (* leak_check_node) ; let universe = self . rcc . universe (* region) ; debug ! ("assign_placeholder_values: scc={:?} universe={:?} region={:?}" , scc , universe , region) ; self . scc_universes [scc] . take_min (universe , * region) ; if let ty :: RePlaceholder (placeholder) = region . kind () { if self . outer_universe . cannot_name (placeholder . universe) { match self . scc_placeholders [scc] { Some (p) => { assert_ne ! (p , placeholder) ; return Err (self . placeholder_error (p , placeholder)) ; } None => { self . scc_placeholders [scc] = Some (placeholder) ; } } } } } Ok (()) } # [doc = " For each SCC S, iterate over each successor S1 where `S: S1`:"] # [doc = ""] # [doc = " * Compute"] # [doc = " Iterate over each SCC `S` and ensure that, for each `S1` where `S1: S`,"] # [doc = " `universe(S) <= universe(S1)`. This executes after"] # [doc = " `assign_placeholder_values`, so `universe(S)` is already the minimum"] # [doc = " universe of any of its direct constituents."] fn propagate_scc_value (& mut self) -> RelateResult < 'tcx , () > { for scc1 in self . mini_graph . sccs . all_sccs () { debug ! ("propagate_scc_value: scc={:?} with universe {:?}" , scc1 , self . scc_universes [scc1]) ; let mut scc1_universe = self . scc_universes [scc1] ; let mut succ_bound = None ; for & scc2 in self . mini_graph . sccs . successors (scc1) { let SccUniverse { universe : scc2_universe , region : scc2_region } = self . scc_universes [scc2] ; scc1_universe . take_min (scc2_universe , scc2_region . unwrap ()) ; if let Some (b) = self . scc_placeholders [scc2] { succ_bound = Some (b) ; } } self . scc_universes [scc1] = scc1_universe ; if let Some (scc1_placeholder) = self . scc_placeholders [scc1] { debug ! ("propagate_scc_value: scc1={:?} placeholder={:?} scc1_universe={:?}" , scc1 , scc1_placeholder , scc1_universe) ; if scc1_universe . universe . cannot_name (scc1_placeholder . universe) { return Err (self . error (scc1_placeholder , scc1_universe . region . unwrap ())) ; } if let Some (scc2_placeholder) = succ_bound { assert_ne ! (scc1_placeholder , scc2_placeholder) ; return Err (self . placeholder_error (scc1_placeholder , scc2_placeholder)) ; } } else { self . scc_placeholders [scc1] = succ_bound ; } } Ok (()) } fn placeholder_error (& self , placeholder1 : ty :: PlaceholderRegion , placeholder2 : ty :: PlaceholderRegion ,) -> TypeError < 'tcx > { self . error (placeholder1 , ty :: Region :: new_placeholder (self . tcx , placeholder2)) } fn error (& self , placeholder : ty :: PlaceholderRegion , other_region : ty :: Region < 'tcx > ,) -> TypeError < 'tcx > { debug ! ("error: placeholder={:?}, other_region={:?}" , placeholder , other_region) ; TypeError :: RegionsInsufficientlyPolymorphic (placeholder . bound , other_region) } }}}
mkitem!{mkstruct!{# [doc = " Tracks the \"minimum universe\" for each SCC, along with some region that"] # [doc = " caused it to change."] # [derive (Copy , Clone , Debug)] struct SccUniverse < 'tcx > { # [doc = " For some SCC S, the minimum universe of:"] # [doc = ""] # [doc = " * each region R in S"] # [doc = " * each SCC S1 such that S: S1"] universe : ty :: UniverseIndex , # [doc = " Some region that caused `universe` to be what it is."] region : Option < ty :: Region < 'tcx > > , }}}
mkitem!{mkimpl!{impl < 'tcx > SccUniverse < 'tcx > { # [doc = " If `universe` is less than our current universe, then update"] # [doc = " `self.universe` and `self.region`."] fn take_min (& mut self , universe : ty :: UniverseIndex , region : ty :: Region < 'tcx >) { if universe < self . universe || self . region . is_none () { self . universe = universe ; self . region = Some (region) ; } } }}}
mkitem!{rustc_index :: newtype_index ! { # [orderable] # [debug_format = "LeakCheckNode({})"] struct LeakCheckNode { } }}
mkitem!{rustc_index :: newtype_index ! { # [orderable] # [debug_format = "LeakCheckScc({})"] struct LeakCheckScc { } }}
mkitem!{mkstruct!{# [doc = " Represents the graph of constraints. For each `R1: R2` constraint we create"] # [doc = " an edge `R1 -> R2` in the graph."] struct MiniGraph < 'tcx > { # [doc = " Map from a region to the index of the node in the graph."] nodes : FxIndexMap < ty :: Region < 'tcx > , LeakCheckNode > , # [doc = " Map from node index to SCC, and stores the successors of each SCC. All"] # [doc = " the regions in the same SCC are equal to one another, and if `S1 -> S2`,"] # [doc = " then `S1: S2`."] sccs : Sccs < LeakCheckNode , LeakCheckScc > , }}}
mkitem!{mkimpl!{impl < 'tcx > MiniGraph < 'tcx > { fn new (region_constraints : & RegionConstraintCollector < '_ , 'tcx > , only_consider_snapshot : Option < & CombinedSnapshot < 'tcx > > ,) -> Self { let mut nodes = FxIndexMap :: default () ; let mut edges = Vec :: new () ; Self :: iterate_region_constraints (region_constraints , only_consider_snapshot , | target , source | { let source_node = Self :: add_node (& mut nodes , source) ; let target_node = Self :: add_node (& mut nodes , target) ; edges . push ((source_node , target_node)) ; } ,) ; let graph = VecGraph :: < _ , false > :: new (nodes . len () , edges) ; let sccs = Sccs :: new (& graph) ; Self { nodes , sccs } } # [doc = " Invokes `each_edge(R1, R2)` for each edge where `R2: R1`"] fn iterate_region_constraints (region_constraints : & RegionConstraintCollector < '_ , 'tcx > , only_consider_snapshot : Option < & CombinedSnapshot < 'tcx > > , mut each_edge : impl FnMut (ty :: Region < 'tcx > , ty :: Region < 'tcx >) ,) { if let Some (snapshot) = only_consider_snapshot { for undo_entry in region_constraints . undo_log . region_constraints_in_snapshot (& snapshot . undo_snapshot) { match undo_entry { & AddConstraint (i) => { let c = region_constraints . data () . constraints [i] . 0 ; each_edge (c . sub , c . sup) ; } & AddVerify (i) => span_bug ! (region_constraints . data () . verifys [i] . origin . span () , "we never add verifications while doing higher-ranked things" ,) , & AddCombination (..) | & AddVar (..) => { } } } } else { region_constraints . data () . constraints . iter () . for_each (| (c , _) | each_edge (c . sub , c . sup)) } } fn add_node (nodes : & mut FxIndexMap < ty :: Region < 'tcx > , LeakCheckNode > , r : ty :: Region < 'tcx > ,) -> LeakCheckNode { let l = nodes . len () ; * nodes . entry (r) . or_insert (LeakCheckNode :: new (l)) } }}}