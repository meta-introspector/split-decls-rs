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
mkuse!{use std :: ops :: ControlFlow ;}
mkuse!{use rustc_abi :: ExternAbi ;}
mkuse!{use rustc_data_structures :: fx :: { FxIndexMap , FxIndexSet } ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_hir :: find_attr ;}
mkuse!{use rustc_hir :: intravisit :: { self , Visitor } ;}
mkuse!{use rustc_middle :: hir :: nested_filter ;}
mkuse!{use rustc_middle :: middle :: privacy :: { EffectiveVisibility , Level } ;}
mkuse!{use rustc_middle :: query :: { LocalCrate , Providers } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt , TypeSuperVisitable , TypeVisitable , TypeVisitor , Visibility , } ;}
mkuse!{use rustc_session :: config :: CrateType ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use crate :: errors :: UnexportableItem ;}
mkitem!{mkstruct!{struct ExportableItemCollector < 'tcx > { tcx : TyCtxt < 'tcx > , exportable_items : FxIndexSet < DefId > , in_exportable_mod : bool , seen_exportable_in_mod : bool , }}}
mkitem!{mkimpl!{impl < 'tcx > ExportableItemCollector < 'tcx > { fn new (tcx : TyCtxt < 'tcx >) -> ExportableItemCollector < 'tcx > { ExportableItemCollector { tcx , exportable_items : Default :: default () , in_exportable_mod : false , seen_exportable_in_mod : false , } } fn report_wrong_site (& self , def_id : LocalDefId) { let def_descr = self . tcx . def_descr (def_id . to_def_id ()) ; self . tcx . dcx () . emit_err (UnexportableItem :: Item { descr : & format ! ("{}" , def_descr) , span : self . tcx . def_span (def_id) , }) ; } fn item_is_exportable (& self , def_id : LocalDefId) -> bool { let has_attr = find_attr ! (self . tcx . get_all_attrs (def_id) , AttributeKind :: ExportStable) ; if ! self . in_exportable_mod && ! has_attr { return false ; } let visibilities = self . tcx . effective_visibilities (()) ; let is_pub = visibilities . is_directly_public (def_id) ; if has_attr && ! is_pub { let vis = visibilities . effective_vis (def_id) . cloned () . unwrap_or_else (| | { EffectiveVisibility :: from_vis (Visibility :: Restricted (self . tcx . parent_module_from_def_id (def_id) . to_local_def_id () ,)) }) ; let vis = vis . at_level (Level :: Direct) ; let span = self . tcx . def_span (def_id) ; self . tcx . dcx () . emit_err (UnexportableItem :: PrivItem { vis_note : span , vis_descr : & vis . to_string (def_id , self . tcx) , span , }) ; return false ; } is_pub && (has_attr || self . in_exportable_mod) } fn add_exportable (& mut self , def_id : LocalDefId) { self . seen_exportable_in_mod = true ; self . exportable_items . insert (def_id . to_def_id ()) ; } fn walk_item_with_mod (& mut self , item : & 'tcx hir :: Item < 'tcx >) { let def_id = item . hir_id () . owner . def_id ; let old_exportable_mod = self . in_exportable_mod ; if find_attr ! (self . tcx . get_all_attrs (def_id) , AttributeKind :: ExportStable) { self . in_exportable_mod = true ; } let old_seen_exportable_in_mod = std :: mem :: replace (& mut self . seen_exportable_in_mod , false) ; intravisit :: walk_item (self , item) ; if self . seen_exportable_in_mod || self . in_exportable_mod { self . exportable_items . insert (def_id . to_def_id ()) ; } self . seen_exportable_in_mod = old_seen_exportable_in_mod ; self . in_exportable_mod = old_exportable_mod ; } }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for ExportableItemCollector < 'tcx > { type NestedFilter = nested_filter :: All ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_item (& mut self , item : & 'tcx hir :: Item < 'tcx >) { let def_id = item . hir_id () . owner . def_id ; match item . kind { hir :: ItemKind :: Mod (..) => { self . walk_item_with_mod (item) ; return ; } hir :: ItemKind :: Impl (impl_) if impl_ . of_trait . is_none () => { self . walk_item_with_mod (item) ; return ; } _ => { } } if ! self . item_is_exportable (def_id) { return ; } match item . kind { hir :: ItemKind :: Fn { .. } | hir :: ItemKind :: Struct (..) | hir :: ItemKind :: Enum (..) | hir :: ItemKind :: Union (..) | hir :: ItemKind :: TyAlias (..) => { self . add_exportable (def_id) ; } hir :: ItemKind :: Use (path , _) => { for res in path . res . present_items () { if let Some (res_id) = res . opt_def_id () && let Some (res_id) = res_id . as_local () { self . add_exportable (res_id) ; } } } hir :: ItemKind :: Mod (..) => unreachable ! () , hir :: ItemKind :: Impl (impl_) if impl_ . of_trait . is_none () => { unreachable ! () ; } _ => self . report_wrong_site (def_id) , } } fn visit_impl_item (& mut self , item : & 'tcx hir :: ImplItem < 'tcx >) { let def_id = item . hir_id () . owner . def_id ; if ! self . item_is_exportable (def_id) { return ; } match item . kind { hir :: ImplItemKind :: Fn (..) | hir :: ImplItemKind :: Type (..) => { self . add_exportable (def_id) ; } _ => self . report_wrong_site (def_id) , } } fn visit_foreign_item (& mut self , item : & 'tcx hir :: ForeignItem < 'tcx >) { let def_id = item . hir_id () . owner . def_id ; if ! self . item_is_exportable (def_id) { self . report_wrong_site (def_id) ; } } fn visit_trait_item (& mut self , item : & 'tcx hir :: TraitItem < 'tcx >) { let def_id = item . hir_id () . owner . def_id ; if ! self . item_is_exportable (def_id) { self . report_wrong_site (def_id) ; } } }}}
mkitem!{mkstruct!{struct ExportableItemsChecker < 'tcx , 'a > { tcx : TyCtxt < 'tcx > , exportable_items : & 'a FxIndexSet < DefId > , item_id : DefId , }}}
mkitem!{mkimpl!{impl < 'tcx , 'a > ExportableItemsChecker < 'tcx , 'a > { fn check (& mut self) { match self . tcx . def_kind (self . item_id) { DefKind :: Fn | DefKind :: AssocFn => self . check_fn () , DefKind :: Enum | DefKind :: Struct | DefKind :: Union => self . check_ty () , _ => { } } } fn check_fn (& mut self) { let def_id = self . item_id . expect_local () ; let span = self . tcx . def_span (def_id) ; if self . tcx . generics_of (def_id) . requires_monomorphization (self . tcx) { self . tcx . dcx () . emit_err (UnexportableItem :: GenericFn (span)) ; return ; } let sig = self . tcx . fn_sig (def_id) . instantiate_identity () . skip_binder () ; if ! matches ! (sig . abi , ExternAbi :: C { .. }) { self . tcx . dcx () . emit_err (UnexportableItem :: FnAbi (span)) ; return ; } let sig = self . tcx . try_normalize_erasing_regions (ty :: TypingEnv :: non_body_analysis (self . tcx , def_id) , sig) . unwrap_or (sig) ; let hir_id = self . tcx . local_def_id_to_hir_id (def_id) ; let decl = self . tcx . hir_fn_decl_by_hir_id (hir_id) . unwrap () ; for (input_ty , input_hir) in iter :: zip (sig . inputs () , decl . inputs) { self . check_nested_types_are_exportable (* input_ty , input_hir . span) ; } if let hir :: FnRetTy :: Return (ret_hir) = decl . output { self . check_nested_types_are_exportable (sig . output () , ret_hir . span) ; } } fn check_ty (& mut self) { let ty = self . tcx . type_of (self . item_id) . skip_binder () ; if let ty :: Adt (adt_def , _) = ty . kind () { if ! adt_def . repr () . inhibit_struct_field_reordering () { self . tcx . dcx () . emit_err (UnexportableItem :: TypeRepr (self . tcx . def_span (self . item_id))) ; } for variant in adt_def . variants () { for field in & variant . fields { if ! field . vis . is_public () { self . tcx . dcx () . emit_err (UnexportableItem :: AdtWithPrivFields { span : self . tcx . def_span (self . item_id) , vis_note : self . tcx . def_span (field . did) , field_name : field . name . as_str () , }) ; } } } } } fn check_nested_types_are_exportable (& mut self , ty : Ty < 'tcx > , ty_span : Span) { let res = ty . visit_with (self) ; if let Some (err_cause) = res . break_value () { self . tcx . dcx () . emit_err (UnexportableItem :: TypeInInterface { span : self . tcx . def_span (self . item_id) , desc : self . tcx . def_descr (self . item_id) , ty : & format ! ("{}" , err_cause) , ty_span , }) ; } } }}}
mkitem!{mkimpl!{impl < 'tcx , 'a > TypeVisitor < TyCtxt < 'tcx > > for ExportableItemsChecker < 'tcx , 'a > { type Result = ControlFlow < Ty < 'tcx > > ; fn visit_ty (& mut self , ty : Ty < 'tcx >) -> Self :: Result { match ty . kind () { ty :: Adt (adt_def , _) => { let did = adt_def . did () ; let exportable = if did . is_local () { self . exportable_items . contains (& did) } else { self . tcx . is_exportable (did) } ; if ! exportable { return ControlFlow :: Break (ty) ; } for variant in adt_def . variants () { for field in & variant . fields { let field_ty = self . tcx . type_of (field . did) . instantiate_identity () ; field_ty . visit_with (self) ? ; } } return ty . super_visit_with (self) ; } ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Bool | ty :: Char | ty :: Error (_) => { } ty :: Array (_ , _) | ty :: Ref (_ , _ , _) | ty :: Param (_) | ty :: Closure (_ , _) | ty :: Dynamic (_ , _ , _) | ty :: Coroutine (_ , _) | ty :: Foreign (_) | ty :: Str | ty :: Tuple (_) | ty :: Pat (..) | ty :: Slice (_) | ty :: RawPtr (_ , _) | ty :: FnDef (_ , _) | ty :: FnPtr (_ , _) | ty :: CoroutineClosure (_ , _) | ty :: CoroutineWitness (_ , _) | ty :: Never | ty :: UnsafeBinder (_) | ty :: Alias (ty :: AliasTyKind :: Opaque , _) => { return ControlFlow :: Break (ty) ; } ty :: Alias (..) | ty :: Infer (_) | ty :: Placeholder (_) | ty :: Bound (..) => unreachable ! () , } ControlFlow :: Continue (()) } }}}

macro_rules! exportable_items_provider_local_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exportable_items_provider_local in module {}", module_path!());
    };
}

mkfn!{
    exportable_items_provider_local_introspect!();
    #[doc = " Exportable items:"] #[doc = ""] #[doc = " 1. Structs/enums/unions with a stable representation (e.g. repr(i32) or repr(C))."] #[doc = " 2. Primitive types."] #[doc = " 3. Non-generic functions with a stable ABI (e.g. extern \"C\") for which every user"] #[doc = "    defined type used in the signature is also marked as `#[export]`."] fn exportable_items_provider_local < 'tcx > (tcx : TyCtxt < 'tcx > , _ : LocalCrate) -> & 'tcx [DefId] { if ! tcx . crate_types () . contains (& CrateType :: Sdylib) && ! tcx . is_sdylib_interface_build () { return & [] ; } let mut visitor = ExportableItemCollector :: new (tcx) ; tcx . hir_walk_toplevel_module (& mut visitor) ; let exportable_items = visitor . exportable_items ; for item_id in exportable_items . iter () { let mut validator = ExportableItemsChecker { tcx , exportable_items : & exportable_items , item_id : * item_id } ; validator . check () ; } tcx . arena . alloc_from_iter (exportable_items . into_iter ()) }
}
mkitem!{mkstruct!{struct ImplsOrderVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , order : FxIndexMap < DefId , usize > , }}}
mkitem!{mkimpl!{impl < 'tcx > ImplsOrderVisitor < 'tcx > { fn new (tcx : TyCtxt < 'tcx >) -> ImplsOrderVisitor < 'tcx > { ImplsOrderVisitor { tcx , order : Default :: default () } } }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for ImplsOrderVisitor < 'tcx > { type NestedFilter = nested_filter :: All ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_item (& mut self , item : & 'tcx hir :: Item < 'tcx >) { if let hir :: ItemKind :: Impl (impl_) = item . kind && impl_ . of_trait . is_none () && self . tcx . is_exportable (item . owner_id . def_id . to_def_id ()) { self . order . insert (item . owner_id . def_id . to_def_id () , self . order . len ()) ; } intravisit :: walk_item (self , item) ; } }}}

macro_rules! stable_order_of_exportable_impls_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stable_order_of_exportable_impls in module {}", module_path!());
    };
}

mkfn!{
    stable_order_of_exportable_impls_introspect!();
    #[doc = " During symbol mangling rustc uses a special index to distinguish between two impls of"] #[doc = " the same type in the same module(See `DisambiguatedDefPathData`). For exportable items"] #[doc = " we cannot use the current approach because it is dependent on the compiler's"] #[doc = " implementation."] #[doc = ""] #[doc = " In order to make disambiguation independent of the compiler version we can assign an"] #[doc = " id to each impl according to the relative order of elements in the source code."] fn stable_order_of_exportable_impls < 'tcx > (tcx : TyCtxt < 'tcx > , _ : LocalCrate ,) -> & 'tcx FxIndexMap < DefId , usize > { if ! tcx . crate_types () . contains (& CrateType :: Sdylib) && ! tcx . is_sdylib_interface_build () { return tcx . arena . alloc (FxIndexMap :: < DefId , usize > :: default ()) ; } let mut vis = ImplsOrderVisitor :: new (tcx) ; tcx . hir_walk_toplevel_module (& mut vis) ; tcx . arena . alloc (vis . order) }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { exportable_items : exportable_items_provider_local , stable_order_of_exportable_impls , .. * providers } ; }
}