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
mkuse!{use rustc_ast :: TraitObjectSyntax ;}
mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_errors :: { Diag , EmissionGuarantee , ErrorGuaranteed , StashKey , Suggestions } ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def :: { DefKind , Namespace , Res } ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_lint_defs :: Applicability ;}
mkuse!{use rustc_lint_defs :: builtin :: BARE_TRAIT_OBJECTS ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use rustc_span :: edit_distance :: find_best_match_for_name ;}
mkuse!{use rustc_trait_selection :: error_reporting :: traits :: suggestions :: NextTypeParamName ;}
mkuse!{use super :: HirTyLowerer ;}
mkitem!{mkimpl!{impl < 'tcx > dyn HirTyLowerer < 'tcx > + '_ { # [doc = " Prohibit or lint against *bare* trait object types depending on the edition."] # [doc = ""] # [doc = " *Bare* trait object types are ones that aren't preceded by the keyword `dyn`."] # [doc = " In edition 2021 and onward we emit a hard error for them."] pub (super) fn prohibit_or_lint_bare_trait_object_ty (& self , self_ty : & hir :: Ty < '_ > ,) -> Option < ErrorGuaranteed > { let tcx = self . tcx () ; let poly_trait_ref = if let hir :: TyKind :: TraitObject ([poly_trait_ref , ..] , tagged_ptr) = self_ty . kind && let TraitObjectSyntax :: None = tagged_ptr . tag () { poly_trait_ref } else { return None ; } ; let in_path = match tcx . parent_hir_node (self_ty . hir_id) { hir :: Node :: Ty (hir :: Ty { kind : hir :: TyKind :: Path (hir :: QPath :: TypeRelative (qself , _)) , .. }) | hir :: Node :: Expr (hir :: Expr { kind : hir :: ExprKind :: Path (hir :: QPath :: TypeRelative (qself , _)) , .. }) | hir :: Node :: PatExpr (hir :: PatExpr { kind : hir :: PatExprKind :: Path (hir :: QPath :: TypeRelative (qself , _)) , .. }) if qself . hir_id == self_ty . hir_id => true , _ => false , } ; let needs_bracket = in_path && ! tcx . sess . source_map () . span_to_prev_source (self_ty . span) . ok () . is_some_and (| s | s . trim_end () . ends_with ('<')) ; let is_global = poly_trait_ref . trait_ref . path . is_global () ; let mut sugg = vec ! [(self_ty . span . shrink_to_lo () , format ! ("{}dyn {}" , if needs_bracket { "<" } else { "" } , if is_global { "(" } else { "" } ,) ,)] ; if is_global || needs_bracket { sugg . push ((self_ty . span . shrink_to_hi () , format ! ("{}{}" , if is_global { ")" } else { "" } , if needs_bracket { ">" } else { "" } ,) ,)) ; } if self_ty . span . edition () . at_least_rust_2021 () { let mut diag = rustc_errors :: struct_span_code_err ! (self . dcx () , self_ty . span , E0782 , "{}" , "expected a type, found a trait") ; if self_ty . span . can_be_used_for_suggestions () && poly_trait_ref . trait_ref . trait_def_id () . is_some () && ! self . maybe_suggest_impl_trait (self_ty , & mut diag) && ! self . maybe_suggest_dyn_trait (self_ty , sugg , & mut diag) { self . maybe_suggest_add_generic_impl_trait (self_ty , & mut diag) ; } self . maybe_suggest_blanket_trait_impl (self_ty , & mut diag) ; self . maybe_suggest_assoc_ty_bound (self_ty , & mut diag) ; self . maybe_suggest_typoed_method (self_ty , poly_trait_ref . trait_ref . trait_def_id () , & mut diag ,) ; if let Some (mut sugg) = self . dcx () . steal_non_err (self_ty . span , StashKey :: AssociatedTypeSuggestion) && let Suggestions :: Enabled (ref mut s1) = diag . suggestions && let Suggestions :: Enabled (ref mut s2) = sugg . suggestions { s1 . append (s2) ; sugg . cancel () ; } Some (diag . emit ()) } else { tcx . node_span_lint (BARE_TRAIT_OBJECTS , self_ty . hir_id , self_ty . span , | lint | { lint . primary_message ("trait objects without an explicit `dyn` are deprecated") ; if self_ty . span . can_be_used_for_suggestions () { lint . multipart_suggestion_verbose ("if this is a dyn-compatible trait, use `dyn`" , sugg , Applicability :: MachineApplicable ,) ; } self . maybe_suggest_blanket_trait_impl (self_ty , lint) ; }) ; None } } # [doc = " For a struct or enum with an invalid bare trait object field, suggest turning"] # [doc = " it into a generic type bound."] fn maybe_suggest_add_generic_impl_trait (& self , self_ty : & hir :: Ty < '_ > , diag : & mut Diag < '_ > ,) -> bool { let tcx = self . tcx () ; let parent_hir_id = tcx . parent_hir_id (self_ty . hir_id) ; let parent_item = tcx . hir_get_parent_item (self_ty . hir_id) . def_id ; let generics = match tcx . hir_node_by_def_id (parent_item) { hir :: Node :: Item (hir :: Item { kind : hir :: ItemKind :: Struct (_ , generics , variant) , .. }) => { if ! variant . fields () . iter () . any (| field | field . hir_id == parent_hir_id) { return false ; } generics } hir :: Node :: Item (hir :: Item { kind : hir :: ItemKind :: Enum (_ , generics , def) , .. }) => { if ! def . variants . iter () . flat_map (| variant | variant . data . fields () . iter ()) . any (| field | field . hir_id == parent_hir_id) { return false ; } generics } _ => return false , } ; let Ok (rendered_ty) = tcx . sess . source_map () . span_to_snippet (self_ty . span) else { return false ; } ; let param = "TUV" . chars () . map (| c | c . to_string ()) . chain ((0 ..) . map (| i | format ! ("P{i}"))) . find (| s | ! generics . params . iter () . any (| param | param . name . ident () . as_str () == s)) . expect ("we definitely can find at least one param name to generate") ; let mut sugg = vec ! [(self_ty . span , param . to_string ())] ; if let Some (insertion_span) = generics . span_for_param_suggestion () { sugg . push ((insertion_span , format ! (", {param}: {}" , rendered_ty))) ; } else { sugg . push ((generics . where_clause_span , format ! ("<{param}: {}>" , rendered_ty))) ; } diag . multipart_suggestion_verbose ("you might be missing a type parameter" , sugg , Applicability :: MachineApplicable ,) ; true } # [doc = " Make sure that we are in the condition to suggest the blanket implementation."] fn maybe_suggest_blanket_trait_impl < G : EmissionGuarantee > (& self , self_ty : & hir :: Ty < '_ > , diag : & mut Diag < '_ , G > ,) { let tcx = self . tcx () ; let parent_id = tcx . hir_get_parent_item (self_ty . hir_id) . def_id ; if let hir :: Node :: Item (hir :: Item { kind : hir :: ItemKind :: Impl (hir :: Impl { self_ty : impl_self_ty , of_trait , generics , .. }) , .. }) = tcx . hir_node_by_def_id (parent_id) && self_ty . hir_id == impl_self_ty . hir_id { let Some (of_trait) = of_trait else { diag . span_suggestion_verbose (impl_self_ty . span . shrink_to_hi () , "you might have intended to implement this trait for a given type" , format ! (" for /* Type */") , Applicability :: HasPlaceholders ,) ; return ; } ; if ! of_trait . trait_ref . trait_def_id () . is_some_and (| def_id | def_id . is_local ()) { return ; } let of_trait_span = of_trait . trait_ref . path . span ; let Ok (of_trait_name) = tcx . sess . source_map () . span_to_snippet (of_trait_span) else { return ; } ; let Ok (impl_trait_name) = self . tcx () . sess . source_map () . span_to_snippet (self_ty . span) else { return ; } ; let sugg = self . add_generic_param_suggestion (generics , self_ty . span , & impl_trait_name) ; diag . multipart_suggestion (format ! ("alternatively use a blanket implementation to implement `{of_trait_name}` for \
                     all types that also implement `{impl_trait_name}`") , sugg , Applicability :: MaybeIncorrect ,) ; } } # [doc = " Try our best to approximate when adding `dyn` would be helpful for a bare"] # [doc = " trait object."] # [doc = ""] # [doc = " Right now, this is if the type is either directly nested in another ty,"] # [doc = " or if it's in the tail field within a struct. This approximates what the"] # [doc = " user would've gotten on edition 2015, except for the case where we have"] # [doc = " an *obvious* knock-on `Sized` error."] fn maybe_suggest_dyn_trait (& self , self_ty : & hir :: Ty < '_ > , sugg : Vec < (Span , String) > , diag : & mut Diag < '_ > ,) -> bool { let tcx = self . tcx () ; match tcx . parent_hir_node (self_ty . hir_id) { hir :: Node :: Ty (_) | hir :: Node :: Expr (_) | hir :: Node :: PatExpr (_) | hir :: Node :: PathSegment (_) | hir :: Node :: AssocItemConstraint (_) | hir :: Node :: TraitRef (_) | hir :: Node :: Item (_) | hir :: Node :: WherePredicate (_) => { } hir :: Node :: Field (field) => { if let hir :: Node :: Item (hir :: Item { kind : hir :: ItemKind :: Struct (_ , _ , variant) , .. }) = tcx . parent_hir_node (field . hir_id) && variant . fields () . last () . is_some_and (| tail_field | tail_field . hir_id == field . hir_id) { } else { return false ; } } _ => return false , } diag . multipart_suggestion_verbose ("you can add the `dyn` keyword if you want a trait object" , sugg , Applicability :: MachineApplicable ,) ; true } fn add_generic_param_suggestion (& self , generics : & hir :: Generics < '_ > , self_ty_span : Span , impl_trait_name : & str ,) -> Vec < (Span , String) > { let param_name = generics . params . next_type_param_name (None) ; let add_generic_sugg = if let Some (span) = generics . span_for_param_suggestion () { (span , format ! (", {param_name}: {impl_trait_name}")) } else { (generics . span , format ! ("<{param_name}: {impl_trait_name}>")) } ; vec ! [(self_ty_span , param_name) , add_generic_sugg] } # [doc = " Make sure that we are in the condition to suggest `impl Trait`."] fn maybe_suggest_impl_trait (& self , self_ty : & hir :: Ty < '_ > , diag : & mut Diag < '_ >) -> bool { let tcx = self . tcx () ; let parent_id = tcx . hir_get_parent_item (self_ty . hir_id) . def_id ; let (sig , generics) = match tcx . hir_node_by_def_id (parent_id) { hir :: Node :: Item (hir :: Item { kind : hir :: ItemKind :: Fn { sig , generics , .. } , .. }) => (sig , generics) , hir :: Node :: TraitItem (hir :: TraitItem { kind : hir :: TraitItemKind :: Fn (sig , _) , generics , .. }) => (sig , generics) , hir :: Node :: ImplItem (hir :: ImplItem { kind : hir :: ImplItemKind :: Fn (sig , _) , generics , .. }) => (sig , generics) , _ => return false , } ; let Ok (trait_name) = tcx . sess . source_map () . span_to_snippet (self_ty . span) else { return false ; } ; let impl_sugg = vec ! [(self_ty . span . shrink_to_lo () , "impl " . to_string ())] ; let is_dyn_compatible = match self_ty . kind { hir :: TyKind :: TraitObject (objects , ..) => { objects . iter () . all (| o | match o . trait_ref . path . res { Res :: Def (DefKind :: Trait , id) => tcx . is_dyn_compatible (id) , _ => false , }) } _ => false , } ; let borrowed = matches ! (tcx . parent_hir_node (self_ty . hir_id) , hir :: Node :: Ty (hir :: Ty { kind : hir :: TyKind :: Ref (..) , .. })) ; if let hir :: FnRetTy :: Return (ty) = sig . decl . output && ty . peel_refs () . hir_id == self_ty . hir_id { let pre = if ! is_dyn_compatible { format ! ("`{trait_name}` is dyn-incompatible, ") } else { String :: new () } ; let msg = format ! ("{pre}use `impl {trait_name}` to return an opaque type, as long as you return a \
                 single underlying type" ,) ; diag . multipart_suggestion_verbose (msg , impl_sugg , Applicability :: MachineApplicable) ; if is_dyn_compatible { let suggestion = if borrowed { vec ! [(ty . span , format ! ("Box<dyn {trait_name}>"))] } else { vec ! [(ty . span . shrink_to_lo () , "Box<dyn " . to_string ()) , (ty . span . shrink_to_hi () , ">" . to_string ()) ,] } ; diag . multipart_suggestion_verbose ("alternatively, you can return an owned trait object" , suggestion , Applicability :: MachineApplicable ,) ; } return true ; } for ty in sig . decl . inputs { if ty . peel_refs () . hir_id != self_ty . hir_id { continue ; } let sugg = self . add_generic_param_suggestion (generics , self_ty . span , & trait_name) ; diag . multipart_suggestion_verbose (format ! ("use a new generic type parameter, constrained by `{trait_name}`") , sugg , Applicability :: MachineApplicable ,) ; diag . multipart_suggestion_verbose ("you can also use an opaque type, but users won't be able to specify the type \
                 parameter when calling the `fn`, having to rely exclusively on type inference" , impl_sugg , Applicability :: MachineApplicable ,) ; if ! is_dyn_compatible { diag . note (format ! ("`{trait_name}` is dyn-incompatible, otherwise a trait object could be used")) ; } else { let (dyn_str , paren_dyn_str) = if borrowed { ("dyn " , "(dyn ") } else { ("&dyn " , "&(dyn ") } ; let sugg = if let hir :: TyKind :: TraitObject ([_ , _ , ..] , _) = self_ty . kind { vec ! [(self_ty . span . shrink_to_lo () , paren_dyn_str . to_string ()) , (self_ty . span . shrink_to_hi () , ")" . to_string ()) ,] } else { vec ! [(self_ty . span . shrink_to_lo () , dyn_str . to_string ())] } ; diag . multipart_suggestion_verbose (format ! ("alternatively, use a trait object to accept any type that implements \
                         `{trait_name}`, accessing its methods at runtime using dynamic dispatch" ,) , sugg , Applicability :: MachineApplicable ,) ; } return true ; } false } fn maybe_suggest_assoc_ty_bound (& self , self_ty : & hir :: Ty < '_ > , diag : & mut Diag < '_ >) { let mut parents = self . tcx () . hir_parent_iter (self_ty . hir_id) ; if let Some ((c_hir_id , hir :: Node :: AssocItemConstraint (constraint))) = parents . next () && let Some (obj_ty) = constraint . ty () && let Some ((_ , hir :: Node :: TraitRef (trait_ref))) = parents . next () { if let Some ((_ , hir :: Node :: Ty (ty))) = parents . next () && let hir :: TyKind :: TraitObject (..) = ty . kind { return ; } if trait_ref . path . segments . iter () . find_map (| seg | { seg . args . filter (| args | args . constraints . iter () . any (| c | c . hir_id == c_hir_id)) }) . is_none_or (| args | args . parenthesized != hir :: GenericArgsParentheses :: No) { return ; } let lo = if constraint . gen_args . span_ext . is_dummy () { constraint . ident . span } else { constraint . gen_args . span_ext } ; let hi = obj_ty . span ; if ! lo . eq_ctxt (hi) { return ; } diag . span_suggestion_verbose (lo . between (hi) , "you might have meant to write a bound here" , ": " , Applicability :: MaybeIncorrect ,) ; } } fn maybe_suggest_typoed_method (& self , self_ty : & hir :: Ty < '_ > , trait_def_id : Option < DefId > , diag : & mut Diag < '_ > ,) { let tcx = self . tcx () ; let Some (trait_def_id) = trait_def_id else { return ; } ; let hir :: Node :: Expr (hir :: Expr { kind : hir :: ExprKind :: Path (hir :: QPath :: TypeRelative (path_ty , segment)) , .. }) = tcx . parent_hir_node (self_ty . hir_id) else { return ; } ; if path_ty . hir_id != self_ty . hir_id { return ; } let names : Vec < _ > = tcx . associated_items (trait_def_id) . in_definition_order () . filter (| assoc | assoc . namespace () == Namespace :: ValueNS) . map (| cand | cand . name ()) . collect () ; if let Some (typo) = find_best_match_for_name (& names , segment . ident . name , None) { diag . span_suggestion_verbose (segment . ident . span , format ! ("you may have misspelled this associated item, causing `{}` \
                    to be interpreted as a type rather than a trait" , tcx . item_name (trait_def_id) ,) , typo , Applicability :: MaybeIncorrect ,) ; } } }}}