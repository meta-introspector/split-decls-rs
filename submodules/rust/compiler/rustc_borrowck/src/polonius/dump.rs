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
mkuse!{use std :: io ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashSet , FxIndexMap , FxIndexSet } ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_middle :: mir :: pretty :: { MirDumper , PassWhere , PrettyPrintMirOptions } ;}
mkuse!{use rustc_middle :: mir :: { Body , Location } ;}
mkuse!{use rustc_middle :: ty :: { RegionVid , TyCtxt } ;}
mkuse!{use rustc_mir_dataflow :: points :: PointIndex ;}
mkuse!{use rustc_session :: config :: MirIncludeSpans ;}
mkuse!{use crate :: borrow_set :: BorrowSet ;}
mkuse!{use crate :: constraints :: OutlivesConstraint ;}
mkuse!{use crate :: polonius :: { LocalizedOutlivesConstraint , LocalizedOutlivesConstraintSet , PoloniusDiagnosticsContext , } ;}
mkuse!{use crate :: region_infer :: values :: LivenessValues ;}
mkuse!{use crate :: type_check :: Locations ;}
mkuse!{use crate :: { BorrowckInferCtxt , ClosureRegionRequirements , RegionInferenceContext } ;}

macro_rules! dump_polonius_mir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dump_polonius_mir in module {}", module_path!());
    };
}

mkfn!{
    dump_polonius_mir_introspect!();
    # [doc = " `-Zdump-mir=polonius` dumps MIR annotated with NLL and polonius specific information."] pub (crate) fn dump_polonius_mir < 'tcx > (infcx : & BorrowckInferCtxt < 'tcx > , body : & Body < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , closure_region_requirements : & Option < ClosureRegionRequirements < 'tcx > > , borrow_set : & BorrowSet < 'tcx > , polonius_diagnostics : Option < & PoloniusDiagnosticsContext > ,) { let tcx = infcx . tcx ; if ! tcx . sess . opts . unstable_opts . polonius . is_next_enabled () { return ; } let Some (dumper) = MirDumper :: new (tcx , "polonius" , body) else { return } ; let polonius_diagnostics = polonius_diagnostics . expect ("missing diagnostics context with `-Zpolonius=next`") ; let extra_data = & | pass_where , out : & mut dyn io :: Write | { emit_polonius_mir (tcx , regioncx , closure_region_requirements , borrow_set , & polonius_diagnostics . localized_outlives_constraints , pass_where , out ,) } ; let options = PrettyPrintMirOptions { include_extra_comments : matches ! (tcx . sess . opts . unstable_opts . mir_include_spans , MirIncludeSpans :: On | MirIncludeSpans :: Nll) , } ; let dumper = dumper . set_extra_data (extra_data) . set_options (options) ; let _ : io :: Result < () > = try { let mut file = dumper . create_dump_file ("html" , body) ? ; emit_polonius_dump (& dumper , body , regioncx , borrow_set , & polonius_diagnostics . localized_outlives_constraints , & mut file ,) ? ; } ; }
}

macro_rules! emit_polonius_dump_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_polonius_dump in module {}", module_path!());
    };
}

mkfn!{
    emit_polonius_dump_introspect!();
    # [doc = " The polonius dump consists of:"] # [doc = " - the NLL MIR"] # [doc = " - the list of polonius localized constraints"] # [doc = " - a mermaid graph of the CFG"] # [doc = " - a mermaid graph of the NLL regions and the constraints between them"] # [doc = " - a mermaid graph of the NLL SCCs and the constraints between them"] fn emit_polonius_dump < 'tcx > (dumper : & MirDumper < '_ , '_ , 'tcx > , body : & Body < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , borrow_set : & BorrowSet < 'tcx > , localized_outlives_constraints : & LocalizedOutlivesConstraintSet , out : & mut dyn io :: Write ,) -> io :: Result < () > { writeln ! (out , "<!DOCTYPE html>") ? ; writeln ! (out , "<html>") ? ; writeln ! (out , "<head><title>Polonius MIR dump</title></head>") ? ; writeln ! (out , "<body>") ? ; writeln ! (out , "<div>") ? ; writeln ! (out , "Raw MIR dump") ? ; writeln ! (out , "<pre><code>") ? ; emit_html_mir (dumper , body , out) ? ; writeln ! (out , "</code></pre>") ? ; writeln ! (out , "</div>") ? ; writeln ! (out , "<div>") ? ; writeln ! (out , "Polonius constraint graph") ? ; writeln ! (out , "<pre class='mermaid'>") ? ; let edge_count = emit_mermaid_constraint_graph (borrow_set , regioncx . liveness_constraints () , & localized_outlives_constraints , out ,) ? ; writeln ! (out , "</pre>") ? ; writeln ! (out , "</div>") ? ; writeln ! (out , "<div>") ? ; writeln ! (out , "Control-flow graph") ? ; writeln ! (out , "<pre class='mermaid'>") ? ; emit_mermaid_cfg (body , out) ? ; writeln ! (out , "</pre>") ? ; writeln ! (out , "</div>") ? ; writeln ! (out , "<div>") ? ; writeln ! (out , "NLL regions") ? ; writeln ! (out , "<pre class='mermaid'>") ? ; emit_mermaid_nll_regions (dumper . tcx () , regioncx , out) ? ; writeln ! (out , "</pre>") ? ; writeln ! (out , "</div>") ? ; writeln ! (out , "<div>") ? ; writeln ! (out , "NLL SCCs") ? ; writeln ! (out , "<pre class='mermaid'>") ? ; emit_mermaid_nll_sccs (dumper . tcx () , regioncx , out) ? ; writeln ! (out , "</pre>") ? ; writeln ! (out , "</div>") ? ; writeln ! (out , "<script src='https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js'></script>") ? ; writeln ! (out , "<script>") ? ; writeln ! (out , "mermaid.initialize({{ startOnLoad: false, maxEdges: {} }});" , edge_count . max (100) ,) ? ; writeln ! (out , "mermaid.run({{ querySelector: '.mermaid' }})") ? ; writeln ! (out , "</script>") ? ; writeln ! (out , "</body>") ? ; writeln ! (out , "</html>") ? ; Ok (()) }
}

macro_rules! emit_html_mir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_html_mir in module {}", module_path!());
    };
}

mkfn!{
    emit_html_mir_introspect!();
    # [doc = " Emits the polonius MIR, as escaped HTML."] fn emit_html_mir < 'tcx > (dumper : & MirDumper < '_ , '_ , 'tcx > , body : & Body < 'tcx > , out : & mut dyn io :: Write ,) -> io :: Result < () > { let mut buffer = Vec :: new () ; dumper . dump_mir_to_writer (body , & mut buffer) ? ; let buffer = String :: from_utf8_lossy (& buffer) ; for ch in buffer . chars () { let escaped = match ch { '>' => "&gt;" , '<' => "&lt;" , '&' => "&amp;" , '\'' => "&#39;" , '"' => "&quot;" , _ => { write ! (out , "{}" , ch) ? ; continue ; } } ; write ! (out , "{}" , escaped) ? ; } Ok (()) }
}

macro_rules! emit_polonius_mir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_polonius_mir in module {}", module_path!());
    };
}

mkfn!{
    emit_polonius_mir_introspect!();
    # [doc = " Produces the actual NLL + Polonius MIR sections to emit during the dumping process."] fn emit_polonius_mir < 'tcx > (tcx : TyCtxt < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , closure_region_requirements : & Option < ClosureRegionRequirements < 'tcx > > , borrow_set : & BorrowSet < 'tcx > , localized_outlives_constraints : & LocalizedOutlivesConstraintSet , pass_where : PassWhere , out : & mut dyn io :: Write ,) -> io :: Result < () > { crate :: nll :: emit_nll_mir (tcx , regioncx , closure_region_requirements , borrow_set , pass_where , out ,) ? ; let liveness = regioncx . liveness_constraints () ; match pass_where { PassWhere :: BeforeCFG => { if localized_outlives_constraints . outlives . len () > 0 { writeln ! (out , "| Localized constraints") ? ; for constraint in & localized_outlives_constraints . outlives { let LocalizedOutlivesConstraint { source , from , target , to } = constraint ; let from = liveness . location_from_point (* from) ; let to = liveness . location_from_point (* to) ; writeln ! (out , "| {source:?} at {from:?} -> {target:?} at {to:?}") ? ; } writeln ! (out , "|") ? ; } } _ => { } } Ok (()) }
}

macro_rules! emit_mermaid_cfg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_mermaid_cfg in module {}", module_path!());
    };
}

mkfn!{
    emit_mermaid_cfg_introspect!();
    # [doc = " Emits a mermaid flowchart of the CFG blocks and edges, similar to the graphviz version."] fn emit_mermaid_cfg (body : & Body < '_ > , out : & mut dyn io :: Write) -> io :: Result < () > { use rustc_middle :: mir :: { TerminatorEdges , TerminatorKind } ; writeln ! (out , "flowchart TD") ? ; for (block_idx , block) in body . basic_blocks . iter_enumerated () { let block_idx = block_idx . as_usize () ; let cleanup = if block . is_cleanup { " (cleanup)" } else { "" } ; writeln ! (out , "{block_idx}[\"bb{block_idx}{cleanup}\"]") ? ; } for (block_idx , block) in body . basic_blocks . iter_enumerated () { let block_idx = block_idx . as_usize () ; let terminator = block . terminator () ; match terminator . edges () { TerminatorEdges :: None => { } TerminatorEdges :: Single (bb) => { writeln ! (out , "{block_idx} --> {}" , bb . as_usize ()) ? ; } TerminatorEdges :: Double (bb1 , bb2) => { if matches ! (terminator . kind , TerminatorKind :: FalseEdge { .. }) { writeln ! (out , "{block_idx} --> {}" , bb1 . as_usize ()) ? ; writeln ! (out , "{block_idx} -- imaginary --> {}" , bb2 . as_usize ()) ? ; } else { writeln ! (out , "{block_idx} --> {}" , bb1 . as_usize ()) ? ; writeln ! (out , "{block_idx} -- unwind --> {}" , bb2 . as_usize ()) ? ; } } TerminatorEdges :: AssignOnReturn { return_ , cleanup , .. } => { for to_idx in return_ { writeln ! (out , "{block_idx} --> {}" , to_idx . as_usize ()) ? ; } if let Some (to_idx) = cleanup { writeln ! (out , "{block_idx} -- unwind --> {}" , to_idx . as_usize ()) ? ; } } TerminatorEdges :: SwitchInt { targets , .. } => { for to_idx in targets . all_targets () { writeln ! (out , "{block_idx} --> {}" , to_idx . as_usize ()) ? ; } } } } Ok (()) }
}

macro_rules! render_region_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function render_region in module {}", module_path!());
    };
}

mkfn!{
    render_region_introspect!();
    # [doc = " Emits a region's label: index, universe, external name."] fn render_region < 'tcx > (tcx : TyCtxt < 'tcx > , region : RegionVid , regioncx : & RegionInferenceContext < 'tcx > , out : & mut dyn io :: Write ,) -> io :: Result < () > { let def = regioncx . region_definition (region) ; let universe = def . universe ; write ! (out , "'{}" , region . as_usize ()) ? ; if ! universe . is_root () { write ! (out , "/{universe:?}") ? ; } if let Some (name) = def . external_name . and_then (| e | e . get_name (tcx)) { write ! (out , " ({name})") ? ; } Ok (()) }
}

macro_rules! emit_mermaid_nll_regions_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_mermaid_nll_regions in module {}", module_path!());
    };
}

mkfn!{
    emit_mermaid_nll_regions_introspect!();
    # [doc = " Emits a mermaid flowchart of the NLL regions and the outlives constraints between them, similar"] # [doc = " to the graphviz version."] fn emit_mermaid_nll_regions < 'tcx > (tcx : TyCtxt < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , out : & mut dyn io :: Write ,) -> io :: Result < () > { writeln ! (out , "flowchart TD") ? ; for region in regioncx . definitions . indices () { write ! (out , "{}[\"" , region . as_usize ()) ? ; render_region (tcx , region , regioncx , out) ? ; writeln ! (out , "\"]") ? ; } let edges : FxHashSet < _ > = regioncx . outlives_constraints () . map (| c | (c . sup , c . sub)) . collect () ; let constraint_key = | c : & OutlivesConstraint < '_ > | { let min = c . sup . min (c . sub) ; let max = c . sup . max (c . sub) ; (min , max) } ; let mut ordered_edges : Vec < _ > = regioncx . outlives_constraints () . collect () ; ordered_edges . sort_by_key (| c | constraint_key (c)) ; ordered_edges . dedup_by_key (| c | constraint_key (c)) ; for outlives in ordered_edges { write ! (out , "{} " , outlives . sup . as_usize ()) ? ; if edges . contains (& (outlives . sub , outlives . sup)) { write ! (out , "&lt;") ? ; } write ! (out , "-- ") ? ; match outlives . locations { Locations :: All (_) => write ! (out , "All") ? , Locations :: Single (location) => write ! (out , "{:?}" , location) ? , } writeln ! (out , " --> {}" , outlives . sub . as_usize ()) ? ; } Ok (()) }
}

macro_rules! emit_mermaid_nll_sccs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_mermaid_nll_sccs in module {}", module_path!());
    };
}

mkfn!{
    emit_mermaid_nll_sccs_introspect!();
    # [doc = " Emits a mermaid flowchart of the NLL SCCs and the outlives constraints between them, similar"] # [doc = " to the graphviz version."] fn emit_mermaid_nll_sccs < 'tcx > (tcx : TyCtxt < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , out : & mut dyn io :: Write ,) -> io :: Result < () > { writeln ! (out , "flowchart TD") ? ; let mut nodes_per_scc : IndexVec < _ , _ > = regioncx . constraint_sccs () . all_sccs () . map (| _ | Vec :: new ()) . collect () ; for region in regioncx . definitions . indices () { let scc = regioncx . constraint_sccs () . scc (region) ; nodes_per_scc [scc] . push (region) ; } for (scc , regions) in nodes_per_scc . iter_enumerated () { write ! (out , "{scc}[\"SCC({scc}) = {{" , scc = scc . as_usize ()) ? ; for (idx , & region) in regions . iter () . enumerate () { render_region (tcx , region , regioncx , out) ? ; if idx < regions . len () - 1 { write ! (out , ",") ? ; } } writeln ! (out , "}}\"]") ? ; } let edges = regioncx . constraint_sccs () . all_sccs () . flat_map (| source | { regioncx . constraint_sccs () . successors (source) . iter () . map (move | & target | (source , target)) }) ; for (source , target) in edges { writeln ! (out , "{} --> {}" , source . as_usize () , target . as_usize ()) ? ; } Ok (()) }
}

macro_rules! emit_mermaid_constraint_graph_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_mermaid_constraint_graph in module {}", module_path!());
    };
}

mkfn!{
    emit_mermaid_constraint_graph_introspect!();
    # [doc = " Emits a mermaid flowchart of the polonius localized outlives constraints, with subgraphs per"] # [doc = " region, and loan introductions."] fn emit_mermaid_constraint_graph < 'tcx > (borrow_set : & BorrowSet < 'tcx > , liveness : & LivenessValues , localized_outlives_constraints : & LocalizedOutlivesConstraintSet , out : & mut dyn io :: Write ,) -> io :: Result < usize > { let location_name = | location : Location | { format ! ("BB{}_{}" , location . block . index () , location . statement_index) } ; let region_name = | region : RegionVid | format ! ("'{}" , region . index ()) ; let node_name = | region : RegionVid , point : PointIndex | { let location = liveness . location_from_point (point) ; format ! ("{}_{}" , region_name (region) , location_name (location)) } ; writeln ! (out , "flowchart TD") ? ; writeln ! (out , "    subgraph \"Loans\"") ? ; for loan_idx in 0 .. borrow_set . len () { writeln ! (out , "        L{loan_idx}") ? ; } writeln ! (out , "    end\n") ? ; for (loan_idx , loan) in borrow_set . iter_enumerated () { writeln ! (out , "    L{} --> {}_{}" , loan_idx . index () , region_name (loan . region) , location_name (loan . reserve_location) ,) ? ; } writeln ! (out , "") ? ; let mut points_per_region : FxIndexMap < RegionVid , FxIndexSet < PointIndex > > = FxIndexMap :: default () ; for constraint in & localized_outlives_constraints . outlives { points_per_region . entry (constraint . source) . or_default () . insert (constraint . from) ; points_per_region . entry (constraint . target) . or_default () . insert (constraint . to) ; } for (region , points) in points_per_region { writeln ! (out , "    subgraph \"{}\"" , region_name (region)) ? ; for point in points { writeln ! (out , "        {}" , node_name (region , point)) ? ; } writeln ! (out , "    end\n") ? ; } for constraint in & localized_outlives_constraints . outlives { writeln ! (out , "    {} --> {}" , node_name (constraint . source , constraint . from) , node_name (constraint . target , constraint . to) ,) ? ; } let edge_count = borrow_set . len () + localized_outlives_constraints . outlives . len () ; Ok (edge_count) }
}