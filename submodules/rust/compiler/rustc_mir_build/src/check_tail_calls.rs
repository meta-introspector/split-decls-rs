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
mkuse!{use rustc_abi :: ExternAbi ;}
mkuse!{use rustc_data_structures :: stack :: ensure_sufficient_stack ;}
mkuse!{use rustc_errors :: Applicability ;}
mkuse!{use rustc_hir :: LangItem ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: CRATE_DEF_ID ;}
mkuse!{use rustc_middle :: span_bug ;}
mkuse!{use rustc_middle :: thir :: visit :: { self , Visitor } ;}
mkuse!{use rustc_middle :: thir :: { BodyTy , Expr , ExprId , ExprKind , Thir } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt } ;}
mkuse!{use rustc_span :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_span :: { DUMMY_SP , ErrorGuaranteed , Span } ;}

macro_rules! check_tail_calls_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_tail_calls in module {}", module_path!());
    };
}

mkfn!{
    check_tail_calls_introspect!();
    pub (crate) fn check_tail_calls (tcx : TyCtxt < '_ > , def : LocalDefId) -> Result < () , ErrorGuaranteed > { let (thir , expr) = tcx . thir_body (def) ? ; let thir = & thir . borrow () ; if thir . exprs . is_empty () { return Ok (()) ; } let is_closure = matches ! (tcx . def_kind (def) , DefKind :: Closure) ; let caller_ty = tcx . type_of (def) . skip_binder () ; let mut visitor = TailCallCkVisitor { tcx , thir , found_errors : Ok (()) , typing_env : ty :: TypingEnv :: non_body_analysis (tcx , def) , is_closure , caller_ty , } ; visitor . visit_expr (& thir [expr]) ; visitor . found_errors }
}
mkitem!{mkstruct!{struct TailCallCkVisitor < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , thir : & 'a Thir < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , #[doc = " Whatever the currently checked body is one of a closure"] is_closure : bool , #[doc = " The result of the checks, `Err(_)` if there was a problem with some"] #[doc = " tail call, `Ok(())` if all of them were fine."] found_errors : Result < () , ErrorGuaranteed > , #[doc = " Type of the caller function."] caller_ty : Ty < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > TailCallCkVisitor < '_ , 'tcx > { fn check_tail_call (& mut self , call : & Expr < '_ > , expr : & Expr < '_ >) { if self . is_closure { self . report_in_closure (expr) ; return ; } let BodyTy :: Fn (caller_sig) = self . thir . body_type else { span_bug ! (call . span , "`become` outside of functions should have been disallowed by hir_typeck") } ; let caller_sig = self . tcx . erase_and_anonymize_regions (caller_sig) ; let ExprKind :: Scope { value , .. } = call . kind else { span_bug ! (call . span , "expected scope, found: {call:?}") } ; let value = & self . thir [value] ; if matches ! (value . kind , ExprKind :: Binary { .. } | ExprKind :: Unary { .. } | ExprKind :: AssignOp { .. } | ExprKind :: Index { .. }) { self . report_builtin_op (call , expr) ; return ; } let ExprKind :: Call { ty , fun , ref args , from_hir_call , fn_span } = value . kind else { self . report_non_call (value , expr) ; return ; } ; if ! from_hir_call { self . report_op (ty , args , fn_span , expr) ; } if let & ty :: FnDef (did , args) = ty . kind () { let parent = self . tcx . parent (did) ; if self . tcx . fn_trait_kind_from_def_id (parent) . is_some () && let Some (this) = args . first () && let Some (this) = this . as_type () { if this . is_closure () { self . report_calling_closure (& self . thir [fun] , args [1] . as_type () . unwrap () , expr) ; } else { self . report_nonfn_callee (fn_span , self . thir [fun] . span , this) ; } return ; } ; if self . tcx . intrinsic (did) . is_some () { self . report_calling_intrinsic (expr) ; } } let (ty :: FnDef (..) | ty :: FnPtr (..)) = ty . kind () else { self . report_nonfn_callee (fn_span , self . thir [fun] . span , ty) ; return ; } ; let callee_sig = self . tcx . normalize_erasing_late_bound_regions (self . typing_env , ty . fn_sig (self . tcx)) ; if caller_sig . abi != callee_sig . abi { self . report_abi_mismatch (expr . span , caller_sig . abi , callee_sig . abi) ; } if caller_sig . inputs_and_output != callee_sig . inputs_and_output { self . report_signature_mismatch (expr . span , self . tcx . liberate_late_bound_regions (CRATE_DEF_ID . to_def_id () , self . caller_ty . fn_sig (self . tcx) ,) , self . tcx . liberate_late_bound_regions (CRATE_DEF_ID . to_def_id () , ty . fn_sig (self . tcx)) ,) ; } { let caller_needs_location = self . needs_location (self . caller_ty) ; if caller_needs_location { self . report_track_caller_caller (expr . span) ; } } if caller_sig . c_variadic { self . report_c_variadic_caller (expr . span) ; } if callee_sig . c_variadic { self . report_c_variadic_callee (expr . span) ; } } #[doc = " Returns true if function of type `ty` needs location argument"] #[doc = " (i.e. if a function is marked as `#[track_caller]`)."] #[doc = ""] #[doc = " Panics if the function's instance can't be immediately resolved."] fn needs_location (& self , ty : Ty < 'tcx >) -> bool { if let & ty :: FnDef (did , substs) = ty . kind () { let instance = ty :: Instance :: expect_resolve (self . tcx , self . typing_env , did , substs , DUMMY_SP) ; instance . def . requires_caller_location (self . tcx) } else { false } } fn report_in_closure (& mut self , expr : & Expr < '_ >) { let err = self . tcx . dcx () . span_err (expr . span , "`become` is not allowed in closures") ; self . found_errors = Err (err) ; } fn report_builtin_op (& mut self , value : & Expr < '_ > , expr : & Expr < '_ >) { let err = self . tcx . dcx () . struct_span_err (value . span , "`become` does not support operators") . with_note ("using `become` on a builtin operator is not useful") . with_span_suggestion (value . span . until (expr . span) , "try using `return` instead" , "return " , Applicability :: MachineApplicable ,) . emit () ; self . found_errors = Err (err) ; } fn report_op (& mut self , fun_ty : Ty < '_ > , args : & [ExprId] , fn_span : Span , expr : & Expr < '_ >) { let mut err = self . tcx . dcx () . struct_span_err (fn_span , "`become` does not support operators") ; if let & ty :: FnDef (did , _substs) = fun_ty . kind () && let parent = self . tcx . parent (did) && matches ! (self . tcx . def_kind (parent) , DefKind :: Trait) && let Some (method) = op_trait_as_method_name (self . tcx , parent) { match args { & [arg] => { let arg = & self . thir [arg] ; err . multipart_suggestion ("try using the method directly" , vec ! [(fn_span . shrink_to_lo () . until (arg . span) , "(" . to_owned ()) , (arg . span . shrink_to_hi () , format ! (").{method}()")) ,] , Applicability :: MaybeIncorrect ,) ; } & [lhs , rhs] => { let lhs = & self . thir [lhs] ; let rhs = & self . thir [rhs] ; err . multipart_suggestion ("try using the method directly" , vec ! [(lhs . span . shrink_to_lo () , format ! ("(")) , (lhs . span . between (rhs . span) , format ! (").{method}(")) , (rhs . span . between (expr . span . shrink_to_hi ()) , ")" . to_owned ()) ,] , Applicability :: MaybeIncorrect ,) ; } _ => span_bug ! (expr . span , "operator with more than 2 args? {args:?}") , } } self . found_errors = Err (err . emit ()) ; } fn report_non_call (& mut self , value : & Expr < '_ > , expr : & Expr < '_ >) { let err = self . tcx . dcx () . struct_span_err (value . span , "`become` requires a function call") . with_span_note (value . span , "not a function call") . with_span_suggestion (value . span . until (expr . span) , "try using `return` instead" , "return " , Applicability :: MaybeIncorrect ,) . emit () ; self . found_errors = Err (err) ; } fn report_calling_closure (& mut self , fun : & Expr < '_ > , tupled_args : Ty < '_ > , expr : & Expr < '_ >) { let underscored_args = match tupled_args . kind () { ty :: Tuple (tys) if tys . is_empty () => "" . to_owned () , ty :: Tuple (tys) => std :: iter :: repeat ("_, ") . take (tys . len () - 1) . chain (["_"]) . collect () , _ => "_" . to_owned () , } ; let err = self . tcx . dcx () . struct_span_err (expr . span , "tail calling closures directly is not allowed") . with_multipart_suggestion ("try casting the closure to a function pointer type" , vec ! [(fun . span . shrink_to_lo () , "(" . to_owned ()) , (fun . span . shrink_to_hi () , format ! (" as fn({underscored_args}) -> _)")) ,] , Applicability :: MaybeIncorrect ,) . emit () ; self . found_errors = Err (err) ; } fn report_calling_intrinsic (& mut self , expr : & Expr < '_ >) { let err = self . tcx . dcx () . struct_span_err (expr . span , "tail calling intrinsics is not allowed") . emit () ; self . found_errors = Err (err) ; } fn report_nonfn_callee (& mut self , call_sp : Span , fun_sp : Span , ty : Ty < '_ >) { let mut err = self . tcx . dcx () . struct_span_err (call_sp , "tail calls can only be performed with function definitions or pointers" ,) . with_note (format ! ("callee has type `{ty}`")) ; let mut ty = ty ; let mut refs = 0 ; while ty . is_box () || ty . is_ref () { ty = ty . builtin_deref (false) . unwrap () ; refs += 1 ; } if refs > 0 && ty . is_fn () { let thing = if ty . is_fn_ptr () { "pointer" } else { "definition" } ; let derefs = std :: iter :: once ('(') . chain (std :: iter :: repeat_n ('*' , refs)) . collect :: < String > () ; err . multipart_suggestion (format ! ("consider dereferencing the expression to get a function {thing}") , vec ! [(fun_sp . shrink_to_lo () , derefs) , (fun_sp . shrink_to_hi () , ")" . to_owned ())] , Applicability :: MachineApplicable ,) ; } let err = err . emit () ; self . found_errors = Err (err) ; } fn report_abi_mismatch (& mut self , sp : Span , caller_abi : ExternAbi , callee_abi : ExternAbi) { let err = self . tcx . dcx () . struct_span_err (sp , "mismatched function ABIs") . with_note ("`become` requires caller and callee to have the same ABI") . with_note (format ! ("caller ABI is `{caller_abi}`, while callee ABI is `{callee_abi}`")) . emit () ; self . found_errors = Err (err) ; } fn report_signature_mismatch (& mut self , sp : Span , caller_sig : ty :: FnSig < '_ > , callee_sig : ty :: FnSig < '_ > ,) { let err = self . tcx . dcx () . struct_span_err (sp , "mismatched signatures") . with_note ("`become` requires caller and callee to have matching signatures") . with_note (format ! ("caller signature: `{caller_sig}`")) . with_note (format ! ("callee signature: `{callee_sig}`")) . emit () ; self . found_errors = Err (err) ; } fn report_track_caller_caller (& mut self , sp : Span) { let err = self . tcx . dcx () . struct_span_err (sp , "a function marked with `#[track_caller]` cannot perform a tail-call" ,) . emit () ; self . found_errors = Err (err) ; } fn report_c_variadic_caller (& mut self , sp : Span) { let err = self . tcx . dcx () . struct_span_err (sp , "tail-calls are not allowed in c-variadic functions") . emit () ; self . found_errors = Err (err) ; } fn report_c_variadic_callee (& mut self , sp : Span) { let err = self . tcx . dcx () . struct_span_err (sp , "c-variadic functions can't be tail-called") . emit () ; self . found_errors = Err (err) ; } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Visitor < 'a , 'tcx > for TailCallCkVisitor < 'a , 'tcx > { fn thir (& self) -> & 'a Thir < 'tcx > { & self . thir } fn visit_expr (& mut self , expr : & 'a Expr < 'tcx >) { ensure_sufficient_stack (| | { if let ExprKind :: Become { value } = expr . kind { let call = & self . thir [value] ; self . check_tail_call (call , expr) ; } visit :: walk_expr (self , expr) ; }) ; } }}}

macro_rules! op_trait_as_method_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function op_trait_as_method_name in module {}", module_path!());
    };
}

mkfn!{
    op_trait_as_method_name_introspect!();
    fn op_trait_as_method_name (tcx : TyCtxt < '_ > , trait_did : DefId) -> Option < & 'static str > { let m = match tcx . as_lang_item (trait_did) ? { LangItem :: Add => "add" , LangItem :: Sub => "sub" , LangItem :: Mul => "mul" , LangItem :: Div => "div" , LangItem :: Rem => "rem" , LangItem :: Neg => "neg" , LangItem :: Not => "not" , LangItem :: BitXor => "bitxor" , LangItem :: BitAnd => "bitand" , LangItem :: BitOr => "bitor" , LangItem :: Shl => "shl" , LangItem :: Shr => "shr" , LangItem :: AddAssign => "add_assign" , LangItem :: SubAssign => "sub_assign" , LangItem :: MulAssign => "mul_assign" , LangItem :: DivAssign => "div_assign" , LangItem :: RemAssign => "rem_assign" , LangItem :: BitXorAssign => "bitxor_assign" , LangItem :: BitAndAssign => "bitand_assign" , LangItem :: BitOrAssign => "bitor_assign" , LangItem :: ShlAssign => "shl_assign" , LangItem :: ShrAssign => "shr_assign" , LangItem :: Index => "index" , LangItem :: IndexMut => "index_mut" , _ => return None , } ; Some (m) }
}