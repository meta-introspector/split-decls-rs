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
mkuse!{use std :: assert_matches :: assert_matches ;}
mkuse!{use std :: ops :: ControlFlow ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_hir :: intravisit :: { self , Visitor , VisitorExt } ;}
mkuse!{use rustc_hir :: { self as hir , AmbigArg , GenericParamKind , HirId , Node } ;}
mkuse!{use rustc_middle :: span_bug ;}
mkuse!{use rustc_middle :: ty :: { self , TyCtxt } ;}
mkuse!{use rustc_session :: lint ;}
mkuse!{use rustc_span :: { Span , Symbol , kw } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use crate :: delegation :: inherit_generics_for_delegation_item ;}
mkuse!{use crate :: middle :: resolve_bound_vars as rbv ;}

macro_rules! generics_of_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function generics_of in module {}", module_path!());
    };
}

mkfn!{
    generics_of_introspect!();
    # [instrument (level = "debug" , skip (tcx) , ret)] pub (super) fn generics_of (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> ty :: Generics { use rustc_hir :: * ; if let Some (ty :: ImplTraitInTraitData :: Trait { fn_def_id , opaque_def_id }) = tcx . opt_rpitit_info (def_id . to_def_id ()) { debug ! ("RPITIT fn_def_id={fn_def_id:?} opaque_def_id={opaque_def_id:?}") ; let trait_def_id = tcx . parent (fn_def_id) ; let opaque_ty_generics = tcx . generics_of (opaque_def_id) ; let opaque_ty_parent_count = opaque_ty_generics . parent_count ; let mut own_params = opaque_ty_generics . own_params . clone () ; let parent_generics = tcx . generics_of (trait_def_id) ; let parent_count = parent_generics . parent_count + parent_generics . own_params . len () ; let mut trait_fn_params = tcx . generics_of (fn_def_id) . own_params . clone () ; for param in & mut own_params { param . index = param . index + parent_count as u32 + trait_fn_params . len () as u32 - opaque_ty_parent_count as u32 ; } trait_fn_params . extend (own_params) ; own_params = trait_fn_params ; let param_def_id_to_index = own_params . iter () . map (| param | (param . def_id , param . index)) . collect () ; return ty :: Generics { parent : Some (trait_def_id) , parent_count , own_params , param_def_id_to_index , has_self : opaque_ty_generics . has_self , has_late_bound_regions : opaque_ty_generics . has_late_bound_regions , } ; } let hir_id = tcx . local_def_id_to_hir_id (def_id) ; let node = tcx . hir_node (hir_id) ; if let Some (sig) = node . fn_sig () && let Some (sig_id) = sig . decl . opt_delegation_sig_id () { return inherit_generics_for_delegation_item (tcx , def_id , sig_id) ; } let parent_def_id = match node { Node :: ImplItem (_) | Node :: TraitItem (_) | Node :: Variant (_) | Node :: Ctor (..) | Node :: Field (_) => { let parent_id = tcx . hir_get_parent_item (hir_id) ; Some (parent_id . to_def_id ()) } Node :: AnonConst (_) => { let parent_did = tcx . parent (def_id . to_def_id ()) ; let parent_did = if let DefKind :: AnonConst = tcx . def_kind (parent_did) { parent_did } else { tcx . hir_get_parent_item (hir_id) . to_def_id () } ; debug ! (? parent_did) ; let mut in_param_ty = false ; for (_parent , node) in tcx . hir_parent_iter (hir_id) { if let Some (generics) = node . generics () { let mut visitor = AnonConstInParamTyDetector { in_param_ty : false , ct : hir_id } ; in_param_ty = visitor . visit_generics (generics) . is_break () ; break ; } } match tcx . anon_const_kind (def_id) { ty :: AnonConstKind :: MCG => None , ty :: AnonConstKind :: RepeatExprCount => Some (parent_did) , ty :: AnonConstKind :: GCE if in_param_ty => None , ty :: AnonConstKind :: GCE if let Some (param_id) = tcx . hir_opt_const_param_default_param_def_id (hir_id) => { let generics = tcx . generics_of (parent_did) ; let param_def_idx = generics . param_def_id_to_index [& param_id . to_def_id ()] ; let own_params = generics . params_to (param_def_idx as usize , tcx) . to_owned () ; let param_def_id_to_index = own_params . iter () . map (| param | (param . def_id , param . index)) . collect () ; return ty :: Generics { parent : generics . parent , parent_count : generics . parent_count , own_params , param_def_id_to_index , has_self : generics . has_self , has_late_bound_regions : generics . has_late_bound_regions , } ; } ty :: AnonConstKind :: GCE => Some (parent_did) , ty :: AnonConstKind :: NonTypeSystem if matches ! (tcx . parent_hir_node (hir_id) , Node :: TyPat (_) | Node :: Field (_)) => { Some (parent_did) } ty :: AnonConstKind :: NonTypeSystem => None , } } Node :: ConstBlock (_) | Node :: Expr (& hir :: Expr { kind : hir :: ExprKind :: Closure { .. } , .. }) => { Some (tcx . typeck_root_def_id (def_id . to_def_id ())) } Node :: OpaqueTy (& hir :: OpaqueTy { origin : hir :: OpaqueTyOrigin :: FnReturn { parent : fn_def_id , in_trait_or_impl } | hir :: OpaqueTyOrigin :: AsyncFn { parent : fn_def_id , in_trait_or_impl } , .. }) => { if in_trait_or_impl . is_some () { assert_matches ! (tcx . def_kind (fn_def_id) , DefKind :: AssocFn) ; } else { assert_matches ! (tcx . def_kind (fn_def_id) , DefKind :: AssocFn | DefKind :: Fn) ; } Some (fn_def_id . to_def_id ()) } Node :: OpaqueTy (& hir :: OpaqueTy { origin : hir :: OpaqueTyOrigin :: TyAlias { parent , in_assoc_ty } , .. }) => { if in_assoc_ty { assert_matches ! (tcx . def_kind (parent) , DefKind :: AssocTy) ; } else { assert_matches ! (tcx . def_kind (parent) , DefKind :: TyAlias) ; } debug ! ("generics_of: parent of opaque ty {:?} is {:?}" , def_id , parent) ; Some (parent . to_def_id ()) } Node :: Item (_) | Node :: ForeignItem (_) => None , Node :: GenericParam (_) => None , Node :: Synthetic => span_bug ! (tcx . def_span (def_id) , "synthetic HIR should have its `generics_of` explicitly fed") , _ => span_bug ! (tcx . def_span (def_id) , "generics_of: unexpected node kind {node:?}") , } ; let opt_self = if let Node :: Item (item) = node && let ItemKind :: Trait (..) | ItemKind :: TraitAlias (..) = item . kind { Some (ty :: GenericParamDef { index : 0 , name : kw :: SelfUpper , def_id : def_id . to_def_id () , pure_wrt_drop : false , kind : ty :: GenericParamDefKind :: Type { has_default : false , synthetic : false } , }) } else { None } ; let param_default_policy = param_default_policy (node) ; let hir_generics = node . generics () . unwrap_or (hir :: Generics :: empty ()) ; let has_self = opt_self . is_some () ; let mut parent_has_self = false ; let mut own_start = has_self as u32 ; let parent_count = parent_def_id . map_or (0 , | def_id | { let generics = tcx . generics_of (def_id) ; assert ! (! has_self) ; parent_has_self = generics . has_self ; own_start = generics . count () as u32 ; generics . parent_count + generics . own_params . len () }) ; let mut own_params : Vec < _ > = Vec :: with_capacity (hir_generics . params . len () + has_self as usize) ; if let Some (opt_self) = opt_self { own_params . push (opt_self) ; } let early_lifetimes = super :: early_bound_lifetimes_from_generics (tcx , hir_generics) ; own_params . extend (early_lifetimes . enumerate () . map (| (i , param) | ty :: GenericParamDef { name : param . name . ident () . name , index : own_start + i as u32 , def_id : param . def_id . to_def_id () , pure_wrt_drop : param . pure_wrt_drop , kind : ty :: GenericParamDefKind :: Lifetime , })) ; let type_start = own_start - has_self as u32 + own_params . len () as u32 ; let mut i : u32 = 0 ; let mut next_index = | | { let prev = i ; i += 1 ; prev + type_start } ; own_params . extend (hir_generics . params . iter () . filter_map (| param | { const MESSAGE : & str = "defaults for generic parameters are not allowed here" ; let kind = match param . kind { GenericParamKind :: Lifetime { .. } => return None , GenericParamKind :: Type { default , synthetic } => { if default . is_some () { match param_default_policy . expect ("no policy for generic param default") { ParamDefaultPolicy :: Allowed => { } ParamDefaultPolicy :: FutureCompatForbidden => { tcx . node_span_lint (lint :: builtin :: INVALID_TYPE_PARAM_DEFAULT , param . hir_id , param . span , | lint | { lint . primary_message (MESSAGE) ; } ,) ; } ParamDefaultPolicy :: Forbidden => { tcx . dcx () . span_err (param . span , MESSAGE) ; } } } ty :: GenericParamDefKind :: Type { has_default : default . is_some () , synthetic } } GenericParamKind :: Const { ty : _ , default } => { if default . is_some () { match param_default_policy . expect ("no policy for generic param default") { ParamDefaultPolicy :: Allowed => { } ParamDefaultPolicy :: FutureCompatForbidden | ParamDefaultPolicy :: Forbidden => { tcx . dcx () . span_err (param . span , MESSAGE) ; } } } ty :: GenericParamDefKind :: Const { has_default : default . is_some () } } } ; Some (ty :: GenericParamDef { index : next_index () , name : param . name . ident () . name , def_id : param . def_id . to_def_id () , pure_wrt_drop : param . pure_wrt_drop , kind , }) })) ; if let Node :: Expr (& hir :: Expr { kind : hir :: ExprKind :: Closure (hir :: Closure { kind , .. }) , .. }) = node { let dummy_args = match kind { ClosureKind :: Closure => & ["<closure_kind>" , "<closure_signature>" , "<upvars>"] [..] , ClosureKind :: Coroutine (_) => { & ["<coroutine_kind>" , "<resume_ty>" , "<yield_ty>" , "<return_ty>" , "<upvars>"] [..] } ClosureKind :: CoroutineClosure (_) => & ["<closure_kind>" , "<closure_signature_parts>" , "<upvars>" , "<bound_captures_by_ref>" ,] [..] , } ; own_params . extend (dummy_args . iter () . map (| & arg | ty :: GenericParamDef { index : next_index () , name : Symbol :: intern (arg) , def_id : def_id . to_def_id () , pure_wrt_drop : false , kind : ty :: GenericParamDefKind :: Type { has_default : false , synthetic : false } , })) ; } if let Node :: ConstBlock (_) = node { own_params . push (ty :: GenericParamDef { index : next_index () , name : rustc_span :: sym :: const_ty_placeholder , def_id : def_id . to_def_id () , pure_wrt_drop : false , kind : ty :: GenericParamDefKind :: Type { has_default : false , synthetic : false } , }) ; } if let Node :: OpaqueTy (& hir :: OpaqueTy { .. }) = node { assert ! (own_params . is_empty ()) ; let lifetimes = tcx . opaque_captured_lifetimes (def_id) ; debug ! (? lifetimes) ; own_params . extend (lifetimes . iter () . map (| & (_ , param) | ty :: GenericParamDef { name : tcx . item_name (param . to_def_id ()) , index : next_index () , def_id : param . to_def_id () , pure_wrt_drop : false , kind : ty :: GenericParamDefKind :: Lifetime , })) } let param_def_id_to_index = own_params . iter () . map (| param | (param . def_id , param . index)) . collect () ; ty :: Generics { parent : parent_def_id , parent_count , own_params , param_def_id_to_index , has_self : has_self || parent_has_self , has_late_bound_regions : has_late_bound_regions (tcx , node) , } }
}
mkitem!{mkenum!{# [derive (Clone , Copy)] enum ParamDefaultPolicy { Allowed , # [doc = " Tracked in <https://github.com/rust-lang/rust/issues/36887>."] FutureCompatForbidden , Forbidden , }}}

macro_rules! param_default_policy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function param_default_policy in module {}", module_path!());
    };
}

mkfn!{
    param_default_policy_introspect!();
    fn param_default_policy (node : Node < '_ >) -> Option < ParamDefaultPolicy > { use rustc_hir :: * ; Some (match node { Node :: Item (item) => match item . kind { ItemKind :: Trait (..) | ItemKind :: TraitAlias (..) | ItemKind :: TyAlias (..) | ItemKind :: Enum (..) | ItemKind :: Struct (..) | ItemKind :: Union (..) => ParamDefaultPolicy :: Allowed , ItemKind :: Fn { .. } | ItemKind :: Impl (_) => ParamDefaultPolicy :: FutureCompatForbidden , ItemKind :: Const (..) => ParamDefaultPolicy :: Forbidden , _ => return None , } , Node :: TraitItem (item) => match item . kind { TraitItemKind :: Const (..) | TraitItemKind :: Type (..) => ParamDefaultPolicy :: Forbidden , TraitItemKind :: Fn (..) => ParamDefaultPolicy :: FutureCompatForbidden , } , Node :: ImplItem (item) => match item . kind { ImplItemKind :: Const (..) | ImplItemKind :: Type (..) => ParamDefaultPolicy :: Forbidden , ImplItemKind :: Fn (..) => ParamDefaultPolicy :: FutureCompatForbidden , } , Node :: ForeignItem (_) => ParamDefaultPolicy :: Forbidden , Node :: OpaqueTy (..) => ParamDefaultPolicy :: Allowed , _ => return None , }) }
}

macro_rules! has_late_bound_regions_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function has_late_bound_regions in module {}", module_path!());
    };
}

mkfn!{
    has_late_bound_regions_introspect!();
    fn has_late_bound_regions < 'tcx > (tcx : TyCtxt < 'tcx > , node : Node < 'tcx >) -> Option < Span > { struct LateBoundRegionsDetector < 'tcx > { tcx : TyCtxt < 'tcx > , outer_index : ty :: DebruijnIndex , } impl < 'tcx > Visitor < 'tcx > for LateBoundRegionsDetector < 'tcx > { type Result = ControlFlow < Span > ; fn visit_ty (& mut self , ty : & 'tcx hir :: Ty < 'tcx , AmbigArg >) -> ControlFlow < Span > { match ty . kind { hir :: TyKind :: FnPtr (..) => { self . outer_index . shift_in (1) ; let res = intravisit :: walk_ty (self , ty) ; self . outer_index . shift_out (1) ; res } hir :: TyKind :: UnsafeBinder (_) => { self . outer_index . shift_in (1) ; let res = intravisit :: walk_ty (self , ty) ; self . outer_index . shift_out (1) ; res } _ => intravisit :: walk_ty (self , ty) , } } fn visit_poly_trait_ref (& mut self , tr : & 'tcx hir :: PolyTraitRef < 'tcx >) -> ControlFlow < Span > { self . outer_index . shift_in (1) ; let res = intravisit :: walk_poly_trait_ref (self , tr) ; self . outer_index . shift_out (1) ; res } fn visit_lifetime (& mut self , lt : & 'tcx hir :: Lifetime) -> ControlFlow < Span > { match self . tcx . named_bound_var (lt . hir_id) { Some (rbv :: ResolvedArg :: StaticLifetime | rbv :: ResolvedArg :: EarlyBound (..)) => { ControlFlow :: Continue (()) } Some (rbv :: ResolvedArg :: LateBound (debruijn , _ , _)) if debruijn < self . outer_index => { ControlFlow :: Continue (()) } Some (rbv :: ResolvedArg :: LateBound (..) | rbv :: ResolvedArg :: Free (..) | rbv :: ResolvedArg :: Error (_) ,) | None => ControlFlow :: Break (lt . ident . span) , } } } fn has_late_bound_regions < 'tcx > (tcx : TyCtxt < 'tcx > , generics : & 'tcx hir :: Generics < 'tcx > , decl : & 'tcx hir :: FnDecl < 'tcx > ,) -> Option < Span > { let mut visitor = LateBoundRegionsDetector { tcx , outer_index : ty :: INNERMOST } ; for param in generics . params { if let GenericParamKind :: Lifetime { .. } = param . kind { if tcx . is_late_bound (param . hir_id) { return Some (param . span) ; } } } visitor . visit_fn_decl (decl) . break_value () } let decl = node . fn_decl () ? ; let generics = node . generics () ? ; has_late_bound_regions (tcx , generics , decl) }
}
mkitem!{mkstruct!{struct AnonConstInParamTyDetector { in_param_ty : bool , ct : HirId , }}}
mkitem!{mkimpl!{impl < 'v > Visitor < 'v > for AnonConstInParamTyDetector { type Result = ControlFlow < () > ; fn visit_generic_param (& mut self , p : & 'v hir :: GenericParam < 'v >) -> Self :: Result { if let GenericParamKind :: Const { ty , default : _ } = p . kind { let prev = self . in_param_ty ; self . in_param_ty = true ; let res = self . visit_ty_unambig (ty) ; self . in_param_ty = prev ; res } else { ControlFlow :: Continue (()) } } fn visit_anon_const (& mut self , c : & 'v hir :: AnonConst) -> Self :: Result { if self . in_param_ty && self . ct == c . hir_id { return ControlFlow :: Break (()) ; } intravisit :: walk_anon_const (self , c) } }}}