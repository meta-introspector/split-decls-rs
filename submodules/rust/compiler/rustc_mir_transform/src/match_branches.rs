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
mkuse!{use std :: iter ;}
mkuse!{use rustc_abi :: Integer ;}
mkuse!{use rustc_index :: IndexSlice ;}
mkuse!{use rustc_middle :: mir :: * ;}
mkuse!{use rustc_middle :: ty :: layout :: { IntegerExt , TyAndLayout } ;}
mkuse!{use rustc_middle :: ty :: { self , ScalarInt , Ty , TyCtxt } ;}
mkuse!{use tracing :: instrument ;}
mkuse!{use super :: simplify :: simplify_cfg ;}
mkuse!{use crate :: patch :: MirPatch ;}
mkitem!{mkstruct!{pub (super) struct MatchBranchSimplification ;}}
mkitem!{mkimpl!{impl < 'tcx > crate :: MirPass < 'tcx > for MatchBranchSimplification { fn is_enabled (& self , sess : & rustc_session :: Session) -> bool { sess . mir_opt_level () >= 1 } fn run_pass (& self , tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { let typing_env = body . typing_env (tcx) ; let mut apply_patch = false ; let mut patch = MirPatch :: new (body) ; for (bb , bb_data) in body . basic_blocks . iter_enumerated () { match & bb_data . terminator () . kind { TerminatorKind :: SwitchInt { discr : Operand :: Copy (_) | Operand :: Move (_) , targets , .. } if ! targets . all_targets () . contains (& bb) => { } _ => continue , } ; if SimplifyToIf . simplify (tcx , body , & mut patch , bb , typing_env) . is_some () { apply_patch = true ; continue ; } if SimplifyToExp :: default () . simplify (tcx , body , & mut patch , bb , typing_env) . is_some () { apply_patch = true ; continue ; } } if apply_patch { patch . apply (body) ; simplify_cfg (tcx , body) ; } } fn is_required (& self) -> bool { false } }}}
mkitem!{mktrait!{trait SimplifyMatch < 'tcx > { #[doc = " Simplifies a match statement, returning `Some` if the simplification succeeds, `None`"] #[doc = " otherwise. Generic code is written here, and we generally don't need a custom"] #[doc = " implementation."] fn simplify (& mut self , tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , patch : & mut MirPatch < 'tcx > , switch_bb_idx : BasicBlock , typing_env : ty :: TypingEnv < 'tcx > ,) -> Option < () > { let bbs = & body . basic_blocks ; let TerminatorKind :: SwitchInt { discr , targets , .. } = & bbs [switch_bb_idx] . terminator () . kind else { unreachable ! () ; } ; let discr_ty = discr . ty (body . local_decls () , tcx) ; self . can_simplify (tcx , targets , typing_env , bbs , discr_ty) ? ; let discr = discr . clone () ; let source_info = bbs [switch_bb_idx] . terminator () . source_info ; let discr_local = patch . new_temp (discr_ty , source_info . span) ; let (_ , first) = targets . iter () . next () . unwrap () ; let statement_index = bbs [switch_bb_idx] . statements . len () ; let parent_end = Location { block : switch_bb_idx , statement_index } ; patch . add_statement (parent_end , StatementKind :: StorageLive (discr_local)) ; patch . add_assign (parent_end , Place :: from (discr_local) , Rvalue :: Use (discr)) ; self . new_stmts (tcx , targets , typing_env , patch , parent_end , bbs , discr_local , discr_ty) ; patch . add_statement (parent_end , StatementKind :: StorageDead (discr_local)) ; patch . patch_terminator (switch_bb_idx , bbs [first] . terminator () . kind . clone ()) ; Some (()) } #[doc = " Check that the BBs to be simplified satisfies all distinct and"] #[doc = " that the terminator are the same."] #[doc = " There are also conditions for different ways of simplification."] fn can_simplify (& mut self , tcx : TyCtxt < 'tcx > , targets : & SwitchTargets , typing_env : ty :: TypingEnv < 'tcx > , bbs : & IndexSlice < BasicBlock , BasicBlockData < 'tcx > > , discr_ty : Ty < 'tcx > ,) -> Option < () > ; fn new_stmts (& self , tcx : TyCtxt < 'tcx > , targets : & SwitchTargets , typing_env : ty :: TypingEnv < 'tcx > , patch : & mut MirPatch < 'tcx > , parent_end : Location , bbs : & IndexSlice < BasicBlock , BasicBlockData < 'tcx > > , discr_local : Local , discr_ty : Ty < 'tcx > ,) ; }}}
mkitem!{mkstruct!{struct SimplifyToIf ;}}
mkitem!{mkimpl!{#[doc = " If a source block is found that switches between two blocks that are exactly"] #[doc = " the same modulo const bool assignments (e.g., one assigns true another false"] #[doc = " to the same place), merge a target block statements into the source block,"] #[doc = " using Eq / Ne comparison with switch value where const bools value differ."] #[doc = ""] #[doc = " For example:"] #[doc = ""] #[doc = " ```ignore (MIR)"] #[doc = " bb0: {"] #[doc = "     switchInt(move _3) -> [42_isize: bb1, otherwise: bb2];"] #[doc = " }"] #[doc = ""] #[doc = " bb1: {"] #[doc = "     _2 = const true;"] #[doc = "     goto -> bb3;"] #[doc = " }"] #[doc = ""] #[doc = " bb2: {"] #[doc = "     _2 = const false;"] #[doc = "     goto -> bb3;"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " into:"] #[doc = ""] #[doc = " ```ignore (MIR)"] #[doc = " bb0: {"] #[doc = "    _2 = Eq(move _3, const 42_isize);"] #[doc = "    goto -> bb3;"] #[doc = " }"] #[doc = " ```"] impl < 'tcx > SimplifyMatch < 'tcx > for SimplifyToIf { #[instrument (level = "debug" , skip (self , tcx) , ret)] fn can_simplify (& mut self , tcx : TyCtxt < 'tcx > , targets : & SwitchTargets , typing_env : ty :: TypingEnv < 'tcx > , bbs : & IndexSlice < BasicBlock , BasicBlockData < 'tcx > > , _discr_ty : Ty < 'tcx > ,) -> Option < () > { let (first , second) = match targets . all_targets () { & [first , otherwise] => (first , otherwise) , & [first , second , otherwise] if bbs [otherwise] . is_empty_unreachable () => (first , second) , _ => { return None ; } } ; if first == second { return None ; } if bbs [first] . terminator () . kind != bbs [second] . terminator () . kind { return None ; } let first_stmts = & bbs [first] . statements ; let second_stmts = & bbs [second] . statements ; if first_stmts . len () != second_stmts . len () { return None ; } for (f , s) in iter :: zip (first_stmts , second_stmts) { match (& f . kind , & s . kind) { (f_s , s_s) if f_s == s_s => { } (StatementKind :: Assign (box (lhs_f , Rvalue :: Use (Operand :: Constant (f_c)))) , StatementKind :: Assign (box (lhs_s , Rvalue :: Use (Operand :: Constant (s_c)))) ,) if lhs_f == lhs_s && f_c . const_ . ty () . is_bool () && s_c . const_ . ty () . is_bool () && f_c . const_ . try_eval_bool (tcx , typing_env) . is_some () && s_c . const_ . try_eval_bool (tcx , typing_env) . is_some () => { } _ => return None , } } Some (()) } fn new_stmts (& self , tcx : TyCtxt < 'tcx > , targets : & SwitchTargets , typing_env : ty :: TypingEnv < 'tcx > , patch : & mut MirPatch < 'tcx > , parent_end : Location , bbs : & IndexSlice < BasicBlock , BasicBlockData < 'tcx > > , discr_local : Local , discr_ty : Ty < 'tcx > ,) { let ((val , first) , second) = match (targets . all_targets () , targets . all_values ()) { (& [first , otherwise] , & [val]) => ((val , first) , otherwise) , (& [first , second , otherwise] , & [val , _]) if bbs [otherwise] . is_empty_unreachable () => { ((val , first) , second) } _ => unreachable ! () , } ; let first = & bbs [first] ; let second = & bbs [second] ; for (f , s) in iter :: zip (& first . statements , & second . statements) { match (& f . kind , & s . kind) { (f_s , s_s) if f_s == s_s => { patch . add_statement (parent_end , f . kind . clone ()) ; } (StatementKind :: Assign (box (lhs , Rvalue :: Use (Operand :: Constant (f_c)))) , StatementKind :: Assign (box (_ , Rvalue :: Use (Operand :: Constant (s_c)))) ,) => { let f_b = f_c . const_ . try_eval_bool (tcx , typing_env) . unwrap () ; let s_b = s_c . const_ . try_eval_bool (tcx , typing_env) . unwrap () ; if f_b == s_b { patch . add_statement (parent_end , f . kind . clone ()) ; } else { let size = tcx . layout_of (typing_env . as_query_input (discr_ty)) . unwrap () . size ; let const_cmp = Operand :: const_from_scalar (tcx , discr_ty , rustc_const_eval :: interpret :: Scalar :: from_uint (val , size) , rustc_span :: DUMMY_SP ,) ; let op = if f_b { BinOp :: Eq } else { BinOp :: Ne } ; let rhs = Rvalue :: BinaryOp (op , Box :: new ((Operand :: Copy (Place :: from (discr_local)) , const_cmp)) ,) ; patch . add_assign (parent_end , * lhs , rhs) ; } } _ => unreachable ! () , } } } }}}

macro_rules! can_cast_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function can_cast in module {}", module_path!());
    };
}

mkfn!{
    can_cast_introspect!();
    #[doc = " Check if the cast constant using `IntToInt` is equal to the target constant."] fn can_cast (tcx : TyCtxt < '_ > , src_val : impl Into < u128 > , src_layout : TyAndLayout < '_ > , cast_ty : Ty < '_ > , target_scalar : ScalarInt ,) -> bool { let from_scalar = ScalarInt :: try_from_uint (src_val . into () , src_layout . size) . unwrap () ; let v = match src_layout . ty . kind () { ty :: Uint (_) => from_scalar . to_uint (src_layout . size) , ty :: Int (_) => from_scalar . to_int (src_layout . size) as u128 , _ => return false , } ; let size = match * cast_ty . kind () { ty :: Int (t) => Integer :: from_int_ty (& tcx , t) . size () , ty :: Uint (t) => Integer :: from_uint_ty (& tcx , t) . size () , _ => return false , } ; let v = size . truncate (v) ; let cast_scalar = ScalarInt :: try_from_uint (v , size) . unwrap () ; cast_scalar == target_scalar }
}
mkitem!{mkstruct!{#[derive (Default)] struct SimplifyToExp { transform_kinds : Vec < TransformKind > , }}}
mkitem!{mkenum!{#[derive (Clone , Copy , Debug)] enum ExpectedTransformKind < 'a , 'tcx > { #[doc = " Identical statements."] Same (& 'a StatementKind < 'tcx >) , #[doc = " Assignment statements have the same value."] SameByEq { place : & 'a Place < 'tcx > , ty : Ty < 'tcx > , scalar : ScalarInt } , #[doc = " Enum variant comparison type."] Cast { place : & 'a Place < 'tcx > , ty : Ty < 'tcx > } , }}}
mkitem!{mkenum!{enum TransformKind { Same , Cast , }}}
mkitem!{mkimpl!{impl From < ExpectedTransformKind < '_ , '_ > > for TransformKind { fn from (compare_type : ExpectedTransformKind < '_ , '_ >) -> Self { match compare_type { ExpectedTransformKind :: Same (_) => TransformKind :: Same , ExpectedTransformKind :: SameByEq { .. } => TransformKind :: Same , ExpectedTransformKind :: Cast { .. } => TransformKind :: Cast , } } }}}
mkitem!{mkimpl!{#[doc = " If we find that the value of match is the same as the assignment,"] #[doc = " merge a target block statements into the source block,"] #[doc = " using cast to transform different integer types."] #[doc = ""] #[doc = " For example:"] #[doc = ""] #[doc = " ```ignore (MIR)"] #[doc = " bb0: {"] #[doc = "     switchInt(_1) -> [1: bb2, 2: bb3, 3: bb4, otherwise: bb1];"] #[doc = " }"] #[doc = ""] #[doc = " bb1: {"] #[doc = "     unreachable;"] #[doc = " }"] #[doc = ""] #[doc = " bb2: {"] #[doc = "     _0 = const 1_i16;"] #[doc = "     goto -> bb5;"] #[doc = " }"] #[doc = ""] #[doc = " bb3: {"] #[doc = "     _0 = const 2_i16;"] #[doc = "     goto -> bb5;"] #[doc = " }"] #[doc = ""] #[doc = " bb4: {"] #[doc = "     _0 = const 3_i16;"] #[doc = "     goto -> bb5;"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " into:"] #[doc = ""] #[doc = " ```ignore (MIR)"] #[doc = " bb0: {"] #[doc = "    _0 = _3 as i16 (IntToInt);"] #[doc = "    goto -> bb5;"] #[doc = " }"] #[doc = " ```"] impl < 'tcx > SimplifyMatch < 'tcx > for SimplifyToExp { #[instrument (level = "debug" , skip (self , tcx) , ret)] fn can_simplify (& mut self , tcx : TyCtxt < 'tcx > , targets : & SwitchTargets , typing_env : ty :: TypingEnv < 'tcx > , bbs : & IndexSlice < BasicBlock , BasicBlockData < 'tcx > > , discr_ty : Ty < 'tcx > ,) -> Option < () > { if targets . iter () . len () < 2 || targets . iter () . len () > 64 { return None ; } if ! targets . is_distinct () { return None ; } if ! bbs [targets . otherwise ()] . is_empty_unreachable () { return None ; } let mut target_iter = targets . iter () ; let (first_case_val , first_target) = target_iter . next () . unwrap () ; let first_terminator_kind = & bbs [first_target] . terminator () . kind ; if ! targets . iter () . all (| (_ , other_target) | first_terminator_kind == & bbs [other_target] . terminator () . kind) { return None ; } let discr_layout = tcx . layout_of (typing_env . as_query_input (discr_ty)) . unwrap () ; let first_stmts = & bbs [first_target] . statements ; let (second_case_val , second_target) = target_iter . next () . unwrap () ; let second_stmts = & bbs [second_target] . statements ; if first_stmts . len () != second_stmts . len () { return None ; } let mut expected_transform_kinds = Vec :: new () ; for (f , s) in iter :: zip (first_stmts , second_stmts) { let compare_type = match (& f . kind , & s . kind) { (f_s , s_s) if f_s == s_s => ExpectedTransformKind :: Same (f_s) , (StatementKind :: Assign (box (lhs_f , Rvalue :: Use (Operand :: Constant (f_c)))) , StatementKind :: Assign (box (lhs_s , Rvalue :: Use (Operand :: Constant (s_c)))) ,) if lhs_f == lhs_s && f_c . const_ . ty () == s_c . const_ . ty () && f_c . const_ . ty () . is_integral () => { match (f_c . const_ . try_eval_scalar_int (tcx , typing_env) , s_c . const_ . try_eval_scalar_int (tcx , typing_env) ,) { (Some (f) , Some (s)) if f == s => ExpectedTransformKind :: SameByEq { place : lhs_f , ty : f_c . const_ . ty () , scalar : f , } , (Some (f) , Some (s)) if (can_cast (tcx , first_case_val , discr_layout , f_c . const_ . ty () , f ,) && can_cast (tcx , second_case_val , discr_layout , f_c . const_ . ty () , s ,)) => { ExpectedTransformKind :: Cast { place : lhs_f , ty : f_c . const_ . ty () } } _ => { return None ; } } } _ => return None , } ; expected_transform_kinds . push (compare_type) ; } for (other_val , other_target) in target_iter { let other_stmts = & bbs [other_target] . statements ; if expected_transform_kinds . len () != other_stmts . len () { return None ; } for (f , s) in iter :: zip (& expected_transform_kinds , other_stmts) { match (* f , & s . kind) { (ExpectedTransformKind :: Same (f_s) , s_s) if f_s == s_s => { } (ExpectedTransformKind :: SameByEq { place : lhs_f , ty : f_ty , scalar } , StatementKind :: Assign (box (lhs_s , Rvalue :: Use (Operand :: Constant (s_c)))) ,) if lhs_f == lhs_s && s_c . const_ . ty () == f_ty && s_c . const_ . try_eval_scalar_int (tcx , typing_env) == Some (scalar) => { } (ExpectedTransformKind :: Cast { place : lhs_f , ty : f_ty } , StatementKind :: Assign (box (lhs_s , Rvalue :: Use (Operand :: Constant (s_c)))) ,) if let Some (f) = s_c . const_ . try_eval_scalar_int (tcx , typing_env) && lhs_f == lhs_s && s_c . const_ . ty () == f_ty && can_cast (tcx , other_val , discr_layout , f_ty , f) => { } _ => return None , } } } self . transform_kinds = expected_transform_kinds . into_iter () . map (| c | c . into ()) . collect () ; Some (()) } fn new_stmts (& self , _tcx : TyCtxt < 'tcx > , targets : & SwitchTargets , _typing_env : ty :: TypingEnv < 'tcx > , patch : & mut MirPatch < 'tcx > , parent_end : Location , bbs : & IndexSlice < BasicBlock , BasicBlockData < 'tcx > > , discr_local : Local , discr_ty : Ty < 'tcx > ,) { let (_ , first) = targets . iter () . next () . unwrap () ; let first = & bbs [first] ; for (t , s) in iter :: zip (& self . transform_kinds , & first . statements) { match (t , & s . kind) { (TransformKind :: Same , _) => { patch . add_statement (parent_end , s . kind . clone ()) ; } (TransformKind :: Cast , StatementKind :: Assign (box (lhs , Rvalue :: Use (Operand :: Constant (f_c)))) ,) => { let operand = Operand :: Copy (Place :: from (discr_local)) ; let r_val = if f_c . const_ . ty () == discr_ty { Rvalue :: Use (operand) } else { Rvalue :: Cast (CastKind :: IntToInt , operand , f_c . const_ . ty ()) } ; patch . add_assign (parent_end , * lhs , r_val) ; } _ => unreachable ! () , } } } }}}