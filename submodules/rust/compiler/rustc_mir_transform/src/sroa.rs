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
mkuse!{use rustc_abi :: FieldIdx ;}
mkuse!{use rustc_data_structures :: flat_map_in_place :: FlatMapInPlace ;}
mkuse!{use rustc_hir :: LangItem ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_index :: bit_set :: { DenseBitSet , GrowableBitSet } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: mir :: visit :: * ;}
mkuse!{use rustc_middle :: mir :: * ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt } ;}
mkuse!{use rustc_mir_dataflow :: value_analysis :: { excluded_locals , iter_fields } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use crate :: patch :: MirPatch ;}
mkitem!{mkstruct!{pub (super) struct ScalarReplacementOfAggregates ;}}
mkitem!{mkimpl!{impl < 'tcx > crate :: MirPass < 'tcx > for ScalarReplacementOfAggregates { fn is_enabled (& self , sess : & rustc_session :: Session) -> bool { sess . mir_opt_level () >= 2 } #[instrument (level = "debug" , skip (self , tcx , body))] fn run_pass (& self , tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { debug ! (def_id = ? body . source . def_id ()) ; if tcx . type_of (body . source . def_id ()) . instantiate_identity () . is_coroutine () { return ; } let mut excluded = excluded_locals (body) ; let typing_env = body . typing_env (tcx) ; loop { debug ! (? excluded) ; let escaping = escaping_locals (tcx , & excluded , body) ; debug ! (? escaping) ; let replacements = compute_flattening (tcx , typing_env , body , escaping) ; debug ! (? replacements) ; let all_dead_locals = replace_flattened_locals (tcx , body , replacements) ; if ! all_dead_locals . is_empty () { excluded . union (& all_dead_locals) ; excluded = { let mut growable = GrowableBitSet :: from (excluded) ; growable . ensure (body . local_decls . len ()) ; growable . into () } ; } else { break ; } } } fn is_required (& self) -> bool { false } }}}

macro_rules! escaping_locals_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function escaping_locals in module {}", module_path!());
    };
}

mkfn!{
    escaping_locals_introspect!();
    #[doc = " Identify all locals that are not eligible for SROA."] #[doc = ""] #[doc = " There are 3 cases:"] #[doc = " - the aggregated local is used or passed to other code (function parameters and arguments);"] #[doc = " - the locals is a union or an enum;"] #[doc = " - the local's address is taken, and thus the relative addresses of the fields are observable to"] #[doc = "   client code."] fn escaping_locals < 'tcx > (tcx : TyCtxt < 'tcx > , excluded : & DenseBitSet < Local > , body : & Body < 'tcx > ,) -> DenseBitSet < Local > { let is_excluded_ty = | ty : Ty < 'tcx > | { if ty . is_union () || ty . is_enum () { return true ; } if let ty :: Adt (def , _args) = ty . kind () && (def . repr () . simd () || tcx . is_lang_item (def . did () , LangItem :: DynMetadata)) { return true ; } false } ; let mut set = DenseBitSet :: new_empty (body . local_decls . len ()) ; set . insert_range (RETURN_PLACE ..= Local :: from_usize (body . arg_count)) ; for (local , decl) in body . local_decls () . iter_enumerated () { if excluded . contains (local) || is_excluded_ty (decl . ty) { set . insert (local) ; } } let mut visitor = EscapeVisitor { set } ; visitor . visit_body (body) ; return visitor . set ; struct EscapeVisitor { set : DenseBitSet < Local > , } impl < 'tcx > Visitor < 'tcx > for EscapeVisitor { fn visit_local (& mut self , local : Local , _ : PlaceContext , _ : Location) { self . set . insert (local) ; } fn visit_place (& mut self , place : & Place < 'tcx > , context : PlaceContext , location : Location) { if let & [PlaceElem :: Field (..) , ..] = & place . projection [..] { return ; } self . super_place (place , context , location) ; } fn visit_assign (& mut self , lvalue : & Place < 'tcx > , rvalue : & Rvalue < 'tcx > , location : Location ,) { if lvalue . as_local () . is_some () { match rvalue { Rvalue :: Aggregate (..) | Rvalue :: Use (..) => { self . visit_rvalue (rvalue , location) ; return ; } _ => { } } } self . super_assign (lvalue , rvalue , location) } fn visit_statement (& mut self , statement : & Statement < 'tcx > , location : Location) { match statement . kind { StatementKind :: StorageLive (..) | StatementKind :: StorageDead (..) | StatementKind :: Deinit (..) => return , _ => self . super_statement (statement , location) , } } fn visit_var_debug_info (& mut self , _ : & VarDebugInfo < 'tcx >) { } } }
}
mkitem!{mkstruct!{#[derive (Default , Debug)] struct ReplacementMap < 'tcx > { #[doc = " Pre-computed list of all \"new\" locals for each \"old\" local. This is used to expand storage"] #[doc = " and deinit statement and debuginfo."] fragments : IndexVec < Local , Option < IndexVec < FieldIdx , Option < (Ty < 'tcx > , Local) > > > > , }}}
mkitem!{mkimpl!{impl < 'tcx > ReplacementMap < 'tcx > { fn replace_place (& self , tcx : TyCtxt < 'tcx > , place : PlaceRef < 'tcx >) -> Option < Place < 'tcx > > { let & [PlaceElem :: Field (f , _) , ref rest @ ..] = place . projection else { return None ; } ; let fields = self . fragments [place . local] . as_ref () ? ; let (_ , new_local) = fields [f] ? ; Some (Place { local : new_local , projection : tcx . mk_place_elems (rest) }) } fn place_fragments (& self , place : Place < 'tcx > ,) -> Option < impl Iterator < Item = (FieldIdx , Ty < 'tcx > , Local) > > { let local = place . as_local () ? ; let fields = self . fragments [local] . as_ref () ? ; Some (fields . iter_enumerated () . filter_map (| (field , & opt_ty_local) | { let (ty , local) = opt_ty_local ? ; Some ((field , ty , local)) })) } }}}

macro_rules! compute_flattening_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compute_flattening in module {}", module_path!());
    };
}

mkfn!{
    compute_flattening_introspect!();
    #[doc = " Compute the replacement of flattened places into locals."] #[doc = ""] #[doc = " For each eligible place, we assign a new local to each accessed field."] #[doc = " The replacement will be done later in `ReplacementVisitor`."] fn compute_flattening < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , body : & mut Body < 'tcx > , escaping : DenseBitSet < Local > ,) -> ReplacementMap < 'tcx > { let mut fragments = IndexVec :: from_elem (None , & body . local_decls) ; for local in body . local_decls . indices () { if escaping . contains (local) { continue ; } let decl = body . local_decls [local] . clone () ; let ty = decl . ty ; iter_fields (ty , tcx , typing_env , | variant , field , field_ty | { if variant . is_some () { return ; } ; let new_local = body . local_decls . push (LocalDecl { ty : field_ty , user_ty : None , .. decl . clone () }) ; fragments . get_or_insert_with (local , IndexVec :: new) . insert (field , (field_ty , new_local)) ; }) ; } ReplacementMap { fragments } }
}

macro_rules! replace_flattened_locals_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function replace_flattened_locals in module {}", module_path!());
    };
}

mkfn!{
    replace_flattened_locals_introspect!();
    #[doc = " Perform the replacement computed by `compute_flattening`."] fn replace_flattened_locals < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx > , replacements : ReplacementMap < 'tcx > ,) -> DenseBitSet < Local > { let mut all_dead_locals = DenseBitSet :: new_empty (replacements . fragments . len ()) ; for (local , replacements) in replacements . fragments . iter_enumerated () { if replacements . is_some () { all_dead_locals . insert (local) ; } } debug ! (? all_dead_locals) ; if all_dead_locals . is_empty () { return all_dead_locals ; } let mut visitor = ReplacementVisitor { tcx , local_decls : & body . local_decls , replacements : & replacements , all_dead_locals , patch : MirPatch :: new (body) , } ; for (bb , data) in body . basic_blocks . as_mut_preserves_cfg () . iter_enumerated_mut () { visitor . visit_basic_block_data (bb , data) ; } for scope in & mut body . source_scopes { visitor . visit_source_scope_data (scope) ; } for (index , annotation) in body . user_type_annotations . iter_enumerated_mut () { visitor . visit_user_type_annotation (index , annotation) ; } visitor . expand_var_debug_info (& mut body . var_debug_info) ; let ReplacementVisitor { patch , all_dead_locals , .. } = visitor ; patch . apply (body) ; all_dead_locals }
}
mkitem!{mkstruct!{struct ReplacementVisitor < 'tcx , 'll > { tcx : TyCtxt < 'tcx > , #[doc = " This is only used to compute the type for `VarDebugInfoFragment`."] local_decls : & 'll LocalDecls < 'tcx > , #[doc = " Work to do."] replacements : & 'll ReplacementMap < 'tcx > , #[doc = " This is used to check that we are not leaving references to replaced locals behind."] all_dead_locals : DenseBitSet < Local > , patch : MirPatch < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > ReplacementVisitor < 'tcx , '_ > { #[instrument (level = "trace" , skip (self))] fn expand_var_debug_info (& mut self , var_debug_info : & mut Vec < VarDebugInfo < 'tcx > >) { var_debug_info . flat_map_in_place (| mut var_debug_info | { let place = match var_debug_info . value { VarDebugInfoContents :: Const (_) => return vec ! [var_debug_info] , VarDebugInfoContents :: Place (ref mut place) => place , } ; if let Some (repl) = self . replacements . replace_place (self . tcx , place . as_ref ()) { * place = repl ; return vec ! [var_debug_info] ; } let Some (parts) = self . replacements . place_fragments (* place) else { return vec ! [var_debug_info] ; } ; let ty = place . ty (self . local_decls , self . tcx) . ty ; parts . map (| (field , field_ty , replacement_local) | { let mut var_debug_info = var_debug_info . clone () ; let composite = var_debug_info . composite . get_or_insert_with (| | { Box :: new (VarDebugInfoFragment { ty , projection : Vec :: new () }) }) ; composite . projection . push (PlaceElem :: Field (field , field_ty)) ; var_debug_info . value = VarDebugInfoContents :: Place (replacement_local . into ()) ; var_debug_info }) . collect () }) ; } }}}
mkitem!{mkimpl!{impl < 'tcx , 'll > MutVisitor < 'tcx > for ReplacementVisitor < 'tcx , 'll > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn visit_place (& mut self , place : & mut Place < 'tcx > , context : PlaceContext , location : Location) { if let Some (repl) = self . replacements . replace_place (self . tcx , place . as_ref ()) { * place = repl } else { self . super_place (place , context , location) } } #[instrument (level = "trace" , skip (self))] fn visit_statement (& mut self , statement : & mut Statement < 'tcx > , location : Location) { match statement . kind { StatementKind :: StorageLive (l) => { if let Some (final_locals) = self . replacements . place_fragments (l . into ()) { for (_ , _ , fl) in final_locals { self . patch . add_statement (location , StatementKind :: StorageLive (fl)) ; } statement . make_nop () ; } return ; } StatementKind :: StorageDead (l) => { if let Some (final_locals) = self . replacements . place_fragments (l . into ()) { for (_ , _ , fl) in final_locals { self . patch . add_statement (location , StatementKind :: StorageDead (fl)) ; } statement . make_nop () ; } return ; } StatementKind :: Deinit (box place) => { if let Some (final_locals) = self . replacements . place_fragments (place) { for (_ , _ , fl) in final_locals { self . patch . add_statement (location , StatementKind :: Deinit (Box :: new (fl . into ()))) ; } statement . make_nop () ; return ; } } StatementKind :: Assign (box (place , Rvalue :: Aggregate (_ , ref mut operands))) => { if let Some (local) = place . as_local () && let Some (final_locals) = & self . replacements . fragments [local] { let operands = std :: mem :: take (operands) ; for (& opt_ty_local , mut operand) in final_locals . iter () . zip (operands) { if let Some ((_ , new_local)) = opt_ty_local { self . visit_operand (& mut operand , location) ; let rvalue = Rvalue :: Use (operand) ; self . patch . add_statement (location , StatementKind :: Assign (Box :: new ((new_local . into () , rvalue))) ,) ; } } statement . make_nop () ; return ; } } StatementKind :: Assign (box (place , Rvalue :: Use (Operand :: Constant (_)))) => { if let Some (final_locals) = self . replacements . place_fragments (place) { let location = location . successor_within_block () ; for (field , ty , new_local) in final_locals { let rplace = self . tcx . mk_place_field (place , field , ty) ; let rvalue = Rvalue :: Use (Operand :: Move (rplace)) ; self . patch . add_statement (location , StatementKind :: Assign (Box :: new ((new_local . into () , rvalue))) ,) ; } return ; } } StatementKind :: Assign (box (lhs , Rvalue :: Use (ref op))) => { let (rplace , copy) = match * op { Operand :: Copy (rplace) => (rplace , true) , Operand :: Move (rplace) => (rplace , false) , Operand :: Constant (_) => bug ! () , } ; if let Some (final_locals) = self . replacements . place_fragments (lhs) { for (field , ty , new_local) in final_locals { let rplace = self . tcx . mk_place_field (rplace , field , ty) ; debug ! (? rplace) ; let rplace = self . replacements . replace_place (self . tcx , rplace . as_ref ()) . unwrap_or (rplace) ; debug ! (? rplace) ; let rvalue = if copy { Rvalue :: Use (Operand :: Copy (rplace)) } else { Rvalue :: Use (Operand :: Move (rplace)) } ; self . patch . add_statement (location , StatementKind :: Assign (Box :: new ((new_local . into () , rvalue))) ,) ; } statement . make_nop () ; return ; } } _ => { } } self . super_statement (statement , location) } fn visit_local (& mut self , local : & mut Local , _ : PlaceContext , _ : Location) { assert ! (! self . all_dead_locals . contains (* local)) ; } }}}