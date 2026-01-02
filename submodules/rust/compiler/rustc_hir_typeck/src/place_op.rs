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
mkuse!{use rustc_errors :: Applicability ;}
mkuse!{use rustc_hir_analysis :: autoderef :: Autoderef ;}
mkuse!{use rustc_infer :: infer :: InferOk ;}
mkuse!{use rustc_infer :: traits :: { Obligation , ObligationCauseCode } ;}
mkuse!{use rustc_middle :: span_bug ;}
mkuse!{use rustc_middle :: ty :: adjustment :: { Adjust , Adjustment , AllowTwoPhase , AutoBorrow , AutoBorrowMutability , OverloadedDeref , PointerCoercion , } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty } ;}
mkuse!{use rustc_span :: { Span , sym } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use { rustc_ast as ast , rustc_hir as hir } ;}
mkuse!{use crate :: method :: MethodCallee ;}
mkuse!{use crate :: { FnCtxt , PlaceOp } ;}
mkitem!{mkimpl!{impl < 'a , 'tcx > FnCtxt < 'a , 'tcx > { # [doc = " Type-check `*oprnd_expr` with `oprnd_expr` type-checked already."] pub (super) fn lookup_derefing (& self , expr : & hir :: Expr < '_ > , oprnd_expr : & 'tcx hir :: Expr < 'tcx > , oprnd_ty : Ty < 'tcx > ,) -> Option < Ty < 'tcx > > { if let Some (ty) = oprnd_ty . builtin_deref (true) { return Some (ty) ; } let ok = self . try_overloaded_deref (expr . span , oprnd_ty) ? ; let method = self . register_infer_ok_obligations (ok) ; if let ty :: Ref (_ , _ , hir :: Mutability :: Not) = method . sig . inputs () [0] . kind () { self . apply_adjustments (oprnd_expr , vec ! [Adjustment { kind : Adjust :: Borrow (AutoBorrow :: Ref (AutoBorrowMutability :: Not)) , target : method . sig . inputs () [0] , }] ,) ; } else { span_bug ! (expr . span , "input to deref is not a ref?") ; } let ty = self . make_overloaded_place_return_type (method) ; self . write_method_call_and_enforce_effects (expr . hir_id , expr . span , method) ; Some (ty) } # [doc = " Type-check `*base_expr[index_expr]` with `base_expr` and `index_expr` type-checked already."] pub (super) fn lookup_indexing (& self , expr : & hir :: Expr < '_ > , base_expr : & 'tcx hir :: Expr < 'tcx > , base_ty : Ty < 'tcx > , index_expr : & 'tcx hir :: Expr < 'tcx > , idx_ty : Ty < 'tcx > ,) -> Option < (Ty < 'tcx > , Ty < 'tcx >) > { let mut autoderef = self . autoderef (base_expr . span , base_ty) ; let mut result = None ; while result . is_none () && autoderef . next () . is_some () { result = self . try_index_step (expr , base_expr , & autoderef , idx_ty , index_expr) ; } self . register_predicates (autoderef . into_obligations ()) ; result } fn negative_index (& self , ty : Ty < 'tcx > , span : Span , base_expr : & hir :: Expr < '_ > ,) -> Option < (Ty < 'tcx > , Ty < 'tcx >) > { let ty = self . resolve_vars_if_possible (ty) ; let mut err = self . dcx () . struct_span_err (span , format ! ("negative integers cannot be used to index on a `{ty}`") ,) ; err . span_label (span , format ! ("cannot use a negative integer for indexing on `{ty}`")) ; if let (hir :: ExprKind :: Path (..) , Ok (snippet)) = (& base_expr . kind , self . tcx . sess . source_map () . span_to_snippet (base_expr . span)) { err . span_suggestion_verbose (span . shrink_to_lo () , format ! ("to access an element starting from the end of the `{ty}`, compute the index" ,) , format ! ("{snippet}.len() ") , Applicability :: MachineApplicable ,) ; } let reported = err . emit () ; Some ((Ty :: new_error (self . tcx , reported) , Ty :: new_error (self . tcx , reported))) } # [doc = " To type-check `base_expr[index_expr]`, we progressively autoderef"] # [doc = " (and otherwise adjust) `base_expr`, looking for a type which either"] # [doc = " supports builtin indexing or overloaded indexing."] # [doc = " This loop implements one step in that search; the autoderef loop"] # [doc = " is implemented by `lookup_indexing`."] fn try_index_step (& self , expr : & hir :: Expr < '_ > , base_expr : & hir :: Expr < '_ > , autoderef : & Autoderef < 'a , 'tcx > , index_ty : Ty < 'tcx > , index_expr : & hir :: Expr < '_ > ,) -> Option < (Ty < 'tcx > , Ty < 'tcx >) > { let adjusted_ty = self . structurally_resolve_type (autoderef . span () , autoderef . final_ty ()) ; debug ! ("try_index_step(expr={:?}, base_expr={:?}, adjusted_ty={:?}, \
             index_ty={:?})" , expr , base_expr , adjusted_ty , index_ty) ; if let hir :: ExprKind :: Unary (hir :: UnOp :: Neg , hir :: Expr { kind : hir :: ExprKind :: Lit (hir :: Lit { node : ast :: LitKind :: Int (..) , .. }) , .. } ,) = index_expr . kind { match adjusted_ty . kind () { ty :: Adt (def , _) if self . tcx . is_diagnostic_item (sym :: Vec , def . did ()) => { return self . negative_index (adjusted_ty , index_expr . span , base_expr) ; } ty :: Slice (_) | ty :: Array (_ , _) => { return self . negative_index (adjusted_ty , index_expr . span , base_expr) ; } _ => { } } } for unsize in [false , true] { let mut self_ty = adjusted_ty ; if unsize { if let ty :: Array (element_ty , ct) = * adjusted_ty . kind () { self . register_predicate (Obligation :: new (self . tcx , self . cause (base_expr . span , ObligationCauseCode :: ArrayLen (adjusted_ty)) , self . param_env , ty :: ClauseKind :: ConstArgHasType (ct , self . tcx . types . usize) ,)) ; self_ty = Ty :: new_slice (self . tcx , element_ty) ; } else { continue ; } } let input_ty = self . next_ty_var (base_expr . span) ; let method = self . try_overloaded_place_op (expr . span , self_ty , Some (input_ty) , PlaceOp :: Index) ; if let Some (result) = method { debug ! ("try_index_step: success, using overloaded indexing") ; let method = self . register_infer_ok_obligations (result) ; let mut adjustments = self . adjust_steps (autoderef) ; if let ty :: Ref (region , _ , hir :: Mutability :: Not) = method . sig . inputs () [0] . kind () { adjustments . push (Adjustment { kind : Adjust :: Borrow (AutoBorrow :: Ref (AutoBorrowMutability :: Not)) , target : Ty :: new_imm_ref (self . tcx , * region , adjusted_ty) , }) ; } else { span_bug ! (expr . span , "input to index is not a ref?") ; } if unsize { adjustments . push (Adjustment { kind : Adjust :: Pointer (PointerCoercion :: Unsize) , target : method . sig . inputs () [0] , }) ; } self . apply_adjustments (base_expr , adjustments) ; self . write_method_call_and_enforce_effects (expr . hir_id , expr . span , method) ; return Some ((input_ty , self . make_overloaded_place_return_type (method))) ; } } None } # [doc = " Try to resolve an overloaded place op. We only deal with the immutable"] # [doc = " variant here (Deref/Index). In some contexts we would need the mutable"] # [doc = " variant (DerefMut/IndexMut); those would be later converted by"] # [doc = " `convert_place_derefs_to_mutable`."] pub (super) fn try_overloaded_place_op (& self , span : Span , base_ty : Ty < 'tcx > , opt_rhs_ty : Option < Ty < 'tcx > > , op : PlaceOp ,) -> Option < InferOk < 'tcx , MethodCallee < 'tcx > > > { debug ! ("try_overloaded_place_op({:?},{:?},{:?})" , span , base_ty , op) ; let (Some (imm_tr) , imm_op) = (match op { PlaceOp :: Deref => (self . tcx . lang_items () . deref_trait () , sym :: deref) , PlaceOp :: Index => (self . tcx . lang_items () . index_trait () , sym :: index) , }) else { return None ; } ; self . lookup_method_for_operator (self . misc (span) , imm_op , imm_tr , base_ty , opt_rhs_ty) } fn try_mutable_overloaded_place_op (& self , span : Span , base_ty : Ty < 'tcx > , opt_rhs_ty : Option < Ty < 'tcx > > , op : PlaceOp ,) -> Option < InferOk < 'tcx , MethodCallee < 'tcx > > > { debug ! ("try_mutable_overloaded_place_op({:?},{:?},{:?})" , span , base_ty , op) ; let (Some (mut_tr) , mut_op) = (match op { PlaceOp :: Deref => (self . tcx . lang_items () . deref_mut_trait () , sym :: deref_mut) , PlaceOp :: Index => (self . tcx . lang_items () . index_mut_trait () , sym :: index_mut) , }) else { return None ; } ; self . lookup_method_for_operator (self . misc (span) , mut_op , mut_tr , base_ty , opt_rhs_ty) } # [doc = " Convert auto-derefs, indices, etc of an expression from `Deref` and `Index`"] # [doc = " into `DerefMut` and `IndexMut` respectively."] # [doc = ""] # [doc = " This is a second pass of typechecking derefs/indices. We need this because we do not"] # [doc = " always know whether a place needs to be mutable or not in the first pass."] # [doc = " This happens whether there is an implicit mutable reborrow, e.g. when the type"] # [doc = " is used as the receiver of a method call."] pub (crate) fn convert_place_derefs_to_mutable (& self , expr : & hir :: Expr < '_ >) { let mut exprs = vec ! [expr] ; while let hir :: ExprKind :: Field (expr , _) | hir :: ExprKind :: Index (expr , _ , _) | hir :: ExprKind :: Unary (hir :: UnOp :: Deref , expr) = exprs . last () . unwrap () . kind { exprs . push (expr) ; } debug ! ("convert_place_derefs_to_mutable: exprs={:?}" , exprs) ; let mut inside_union = false ; for (i , & expr) in exprs . iter () . rev () . enumerate () { debug ! ("convert_place_derefs_to_mutable: i={} expr={:?}" , i , expr) ; let mut source = self . node_ty (expr . hir_id) ; if matches ! (expr . kind , hir :: ExprKind :: Unary (hir :: UnOp :: Deref , _)) { inside_union = false ; } if source . is_union () { inside_union = true ; } let previous_adjustments = self . typeck_results . borrow_mut () . adjustments_mut () . remove (expr . hir_id) ; if let Some (mut adjustments) = previous_adjustments { for adjustment in & mut adjustments { if let Adjust :: Deref (Some (ref mut deref)) = adjustment . kind && let Some (ok) = self . try_mutable_overloaded_place_op (expr . span , source , None , PlaceOp :: Deref ,) { let method = self . register_infer_ok_obligations (ok) ; let ty :: Ref (_ , _ , mutbl) = * method . sig . output () . kind () else { span_bug ! (self . tcx . def_span (method . def_id) , "expected DerefMut to return a &mut") ; } ; * deref = OverloadedDeref { mutbl , span : deref . span } ; self . enforce_context_effects (None , expr . span , method . def_id , method . args) ; if inside_union && source . ty_adt_def () . is_some_and (| adt | adt . is_manually_drop ()) { self . dcx () . struct_span_err (expr . span , "not automatically applying `DerefMut` on `ManuallyDrop` union field" ,) . with_help ("writing to this reference calls the destructor for the old value" ,) . with_help ("add an explicit `*` if that is desired, or call `ptr::write` to not run the destructor") . emit () ; } } source = adjustment . target ; } self . typeck_results . borrow_mut () . adjustments_mut () . insert (expr . hir_id , adjustments) ; } match expr . kind { hir :: ExprKind :: Index (base_expr , ..) => { self . convert_place_op_to_mutable (PlaceOp :: Index , expr , base_expr) ; } hir :: ExprKind :: Unary (hir :: UnOp :: Deref , base_expr) => { self . convert_place_op_to_mutable (PlaceOp :: Deref , expr , base_expr) ; } _ => { } } } } fn convert_place_op_to_mutable (& self , op : PlaceOp , expr : & hir :: Expr < '_ > , base_expr : & hir :: Expr < '_ > ,) { debug ! ("convert_place_op_to_mutable({:?}, {:?}, {:?})" , op , expr , base_expr) ; if ! self . typeck_results . borrow () . is_method_call (expr) { debug ! ("convert_place_op_to_mutable - builtin, nothing to do") ; return ; } let base_ty = self . typeck_results . borrow () . expr_ty_adjusted (base_expr) . builtin_deref (false) . expect ("place op takes something that is not a ref") ; let arg_ty = match op { PlaceOp :: Deref => None , PlaceOp :: Index => { Some (self . typeck_results . borrow () . node_args (expr . hir_id) . type_at (1)) } } ; let method = self . try_mutable_overloaded_place_op (expr . span , base_ty , arg_ty , op) ; let method = match method { Some (ok) => self . register_infer_ok_obligations (ok) , None => return , } ; debug ! ("convert_place_op_to_mutable: method={:?}" , method) ; self . write_method_call_and_enforce_effects (expr . hir_id , expr . span , method) ; let ty :: Ref (region , _ , hir :: Mutability :: Mut) = method . sig . inputs () [0] . kind () else { span_bug ! (expr . span , "input to mutable place op is not a mut ref?") ; } ; let base_expr_ty = self . node_ty (base_expr . hir_id) ; if let Some (adjustments) = self . typeck_results . borrow_mut () . adjustments_mut () . get_mut (base_expr . hir_id) { let mut source = base_expr_ty ; for adjustment in & mut adjustments [..] { if let Adjust :: Borrow (AutoBorrow :: Ref (..)) = adjustment . kind { debug ! ("convert_place_op_to_mutable: converting autoref {:?}" , adjustment) ; let mutbl = AutoBorrowMutability :: Mut { allow_two_phase_borrow : AllowTwoPhase :: No , } ; adjustment . kind = Adjust :: Borrow (AutoBorrow :: Ref (mutbl)) ; adjustment . target = Ty :: new_ref (self . tcx , * region , source , mutbl . into ()) ; } source = adjustment . target ; } if let [.. , Adjustment { kind : Adjust :: Borrow (AutoBorrow :: Ref (..)) , .. } , Adjustment { kind : Adjust :: Pointer (PointerCoercion :: Unsize) , ref mut target } ,] = adjustments [..] { * target = method . sig . inputs () [0] ; } } } }}}