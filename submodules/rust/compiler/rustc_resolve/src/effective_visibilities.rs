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
mkuse!{use std :: mem ;}
mkuse!{use rustc_ast :: visit :: Visitor ;}
mkuse!{use rustc_ast :: { Crate , EnumDef , ast , visit } ;}
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_hir :: def_id :: { CRATE_DEF_ID , LocalDefId } ;}
mkuse!{use rustc_middle :: middle :: privacy :: { EffectiveVisibilities , EffectiveVisibility , Level } ;}
mkuse!{use rustc_middle :: ty :: Visibility ;}
mkuse!{use tracing :: info ;}
mkuse!{use crate :: { NameBinding , NameBindingKind , Resolver } ;}
mkitem!{mkenum!{# [derive (Clone , Copy)] enum ParentId < 'ra > { Def (LocalDefId) , Import (NameBinding < 'ra >) , }}}
mkitem!{mkimpl!{impl ParentId < '_ > { fn level (self) -> Level { match self { ParentId :: Def (_) => Level :: Direct , ParentId :: Import (_) => Level :: Reexported , } } }}}
mkitem!{mkstruct!{pub (crate) struct EffectiveVisibilitiesVisitor < 'a , 'ra , 'tcx > { r : & 'a mut Resolver < 'ra , 'tcx > , def_effective_visibilities : EffectiveVisibilities , # [doc = " While walking import chains we need to track effective visibilities per-binding, and def id"] # [doc = " keys in `Resolver::effective_visibilities` are not enough for that, because multiple"] # [doc = " bindings can correspond to a single def id in imports. So we keep a separate table."] import_effective_visibilities : EffectiveVisibilities < NameBinding < 'ra > > , current_private_vis : Visibility , changed : bool , }}}
mkitem!{mkimpl!{impl Resolver < '_ , '_ > { fn nearest_normal_mod (& self , def_id : LocalDefId) -> LocalDefId { self . get_nearest_non_block_module (def_id . to_def_id ()) . nearest_parent_mod () . expect_local () } fn private_vis_import (& self , binding : NameBinding < '_ >) -> Visibility { let NameBindingKind :: Import { import , .. } = binding . kind else { unreachable ! () } ; Visibility :: Restricted (import . id () . map (| id | self . nearest_normal_mod (self . local_def_id (id))) . unwrap_or (CRATE_DEF_ID) ,) } fn private_vis_def (& self , def_id : LocalDefId) -> Visibility { let normal_mod_id = self . nearest_normal_mod (def_id) ; if normal_mod_id == def_id { Visibility :: Restricted (self . tcx . local_parent (def_id)) } else { Visibility :: Restricted (normal_mod_id) } } }}}
mkitem!{mkimpl!{impl < 'a , 'ra , 'tcx > EffectiveVisibilitiesVisitor < 'a , 'ra , 'tcx > { # [doc = " Fills the `Resolver::effective_visibilities` table with public & exported items"] # [doc = " For now, this doesn't resolve macros (FIXME) and cannot resolve Impl, as we"] # [doc = " need access to a TyCtxt for that. Returns the set of ambiguous re-exports."] pub (crate) fn compute_effective_visibilities < 'c > (r : & 'a mut Resolver < 'ra , 'tcx > , krate : & 'c Crate ,) -> FxHashSet < NameBinding < 'ra > > { let mut visitor = EffectiveVisibilitiesVisitor { r , def_effective_visibilities : Default :: default () , import_effective_visibilities : Default :: default () , current_private_vis : Visibility :: Restricted (CRATE_DEF_ID) , changed : true , } ; visitor . def_effective_visibilities . update_root () ; visitor . set_bindings_effective_visibilities (CRATE_DEF_ID) ; while visitor . changed { visitor . changed = false ; visit :: walk_crate (& mut visitor , krate) ; } visitor . r . effective_visibilities = visitor . def_effective_visibilities ; let mut exported_ambiguities = FxHashSet :: default () ; for (binding , eff_vis) in visitor . import_effective_visibilities . iter () { let NameBindingKind :: Import { import , .. } = binding . kind else { unreachable ! () } ; if ! binding . is_ambiguity_recursive () { if let Some (node_id) = import . id () { r . effective_visibilities . update_eff_vis (r . local_def_id (node_id) , eff_vis , r . tcx) } } else if binding . ambiguity . is_some () && eff_vis . is_public_at_level (Level :: Reexported) { exported_ambiguities . insert (* binding) ; } } info ! ("resolve::effective_visibilities: {:#?}" , r . effective_visibilities) ; exported_ambiguities } # [doc = " Update effective visibilities of bindings in the given module,"] # [doc = " including their whole reexport chains."] fn set_bindings_effective_visibilities (& mut self , module_id : LocalDefId) { let module = self . r . expect_module (module_id . to_def_id ()) ; for (_ , name_resolution) in self . r . resolutions (module) . borrow () . iter () { let Some (mut binding) = name_resolution . borrow () . binding () else { continue ; } ; let is_ambiguity = | binding : NameBinding < 'ra > , warn : bool | binding . ambiguity . is_some () && ! warn ; let mut parent_id = ParentId :: Def (module_id) ; let mut warn_ambiguity = binding . warn_ambiguity ; while let NameBindingKind :: Import { binding : nested_binding , .. } = binding . kind { self . update_import (binding , parent_id) ; if is_ambiguity (binding , warn_ambiguity) { break ; } parent_id = ParentId :: Import (binding) ; binding = nested_binding ; warn_ambiguity |= nested_binding . warn_ambiguity ; } if ! is_ambiguity (binding , warn_ambiguity) && let Some (def_id) = binding . res () . opt_def_id () . and_then (| id | id . as_local ()) { self . update_def (def_id , binding . vis . expect_local () , parent_id) ; } } } fn effective_vis_or_private (& mut self , parent_id : ParentId < 'ra >) -> EffectiveVisibility { * match parent_id { ParentId :: Def (def_id) => self . def_effective_visibilities . effective_vis_or_private (def_id , | | self . r . private_vis_def (def_id)) , ParentId :: Import (binding) => self . import_effective_visibilities . effective_vis_or_private (binding , | | self . r . private_vis_import (binding)) , } } # [doc = " All effective visibilities for a node are larger or equal than private visibility"] # [doc = " for that node (see `check_invariants` in middle/privacy.rs)."] # [doc = " So if either parent or nominal visibility is the same as private visibility, then"] # [doc = " `min(parent_vis, nominal_vis) <= private_vis`, and the update logic is guaranteed"] # [doc = " to not update anything and we can skip it."] # [doc = ""] # [doc = " We are checking this condition only if the correct value of private visibility is"] # [doc = " cheaply available, otherwise it doesn't make sense performance-wise."] # [doc = ""] # [doc = " `None` is returned if the update can be skipped,"] # [doc = " and cheap private visibility is returned otherwise."] fn may_update (& self , nominal_vis : Visibility , parent_id : ParentId < '_ > ,) -> Option < Option < Visibility > > { match parent_id { ParentId :: Def (def_id) => (nominal_vis != self . current_private_vis && self . r . tcx . local_visibility (def_id) != self . current_private_vis) . then_some (Some (self . current_private_vis)) , ParentId :: Import (_) => Some (None) , } } fn update_import (& mut self , binding : NameBinding < 'ra > , parent_id : ParentId < 'ra >) { let nominal_vis = binding . vis . expect_local () ; let Some (cheap_private_vis) = self . may_update (nominal_vis , parent_id) else { return } ; let inherited_eff_vis = self . effective_vis_or_private (parent_id) ; let tcx = self . r . tcx ; self . changed |= self . import_effective_visibilities . update (binding , Some (nominal_vis) , | | cheap_private_vis . unwrap_or_else (| | self . r . private_vis_import (binding)) , inherited_eff_vis , parent_id . level () , tcx ,) ; } fn update_def (& mut self , def_id : LocalDefId , nominal_vis : Visibility , parent_id : ParentId < 'ra > ,) { let Some (cheap_private_vis) = self . may_update (nominal_vis , parent_id) else { return } ; let inherited_eff_vis = self . effective_vis_or_private (parent_id) ; let tcx = self . r . tcx ; self . changed |= self . def_effective_visibilities . update (def_id , Some (nominal_vis) , | | cheap_private_vis . unwrap_or_else (| | self . r . private_vis_def (def_id)) , inherited_eff_vis , parent_id . level () , tcx ,) ; } fn update_field (& mut self , def_id : LocalDefId , parent_id : LocalDefId) { self . update_def (def_id , self . r . tcx . local_visibility (def_id) , ParentId :: Def (parent_id)) ; } }}}
mkitem!{mkimpl!{impl < 'a , 'ra , 'tcx > Visitor < 'a > for EffectiveVisibilitiesVisitor < 'a , 'ra , 'tcx > { fn visit_item (& mut self , item : & 'a ast :: Item) { let def_id = self . r . local_def_id (item . id) ; match item . kind { ast :: ItemKind :: Impl (..) => return , ast :: ItemKind :: MacCall (..) | ast :: ItemKind :: DelegationMac (..) => panic ! ("ast::ItemKind::MacCall encountered, this should not anymore appear at this stage") , ast :: ItemKind :: Mod (..) => { let prev_private_vis = mem :: replace (& mut self . current_private_vis , Visibility :: Restricted (def_id)) ; self . set_bindings_effective_visibilities (def_id) ; visit :: walk_item (self , item) ; self . current_private_vis = prev_private_vis ; } ast :: ItemKind :: Enum (_ , _ , EnumDef { ref variants }) => { self . set_bindings_effective_visibilities (def_id) ; for variant in variants { let variant_def_id = self . r . local_def_id (variant . id) ; for field in variant . data . fields () { self . update_field (self . r . local_def_id (field . id) , variant_def_id) ; } } } ast :: ItemKind :: Struct (_ , _ , ref def) | ast :: ItemKind :: Union (_ , _ , ref def) => { for field in def . fields () { self . update_field (self . r . local_def_id (field . id) , def_id) ; } } ast :: ItemKind :: Trait (..) => { self . set_bindings_effective_visibilities (def_id) ; } ast :: ItemKind :: ExternCrate (..) | ast :: ItemKind :: Use (..) | ast :: ItemKind :: Static (..) | ast :: ItemKind :: Const (..) | ast :: ItemKind :: GlobalAsm (..) | ast :: ItemKind :: TyAlias (..) | ast :: ItemKind :: TraitAlias (..) | ast :: ItemKind :: MacroDef (..) | ast :: ItemKind :: ForeignMod (..) | ast :: ItemKind :: Fn (..) | ast :: ItemKind :: Delegation (..) => return , } } }}}