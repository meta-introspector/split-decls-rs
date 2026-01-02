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
mkuse!{use rustc_ast as ast ;}
mkuse!{use rustc_ast :: visit :: { self , Visitor } ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashMap , FxIndexMap , FxIndexSet } ;}
mkuse!{use rustc_data_structures :: unord :: UnordSet ;}
mkuse!{use rustc_errors :: MultiSpan ;}
mkuse!{use rustc_hir :: def :: { DefKind , Res } ;}
mkuse!{use rustc_session :: lint :: BuiltinLintDiag ;}
mkuse!{use rustc_session :: lint :: builtin :: { MACRO_USE_EXTERN_CRATE , UNUSED_EXTERN_CRATES , UNUSED_IMPORTS , UNUSED_QUALIFICATIONS , } ;}
mkuse!{use rustc_span :: { DUMMY_SP , Ident , Macros20NormalizedIdent , Span , kw } ;}
mkuse!{use crate :: imports :: { Import , ImportKind } ;}
mkuse!{use crate :: { LexicalScopeBinding , NameBindingKind , Resolver , module_to_string } ;}
mkitem!{mkstruct!{struct UnusedImport { use_tree : ast :: UseTree , use_tree_id : ast :: NodeId , item_span : Span , unused : UnordSet < ast :: NodeId > , }}}
mkitem!{mkimpl!{impl UnusedImport { fn add (& mut self , id : ast :: NodeId) { self . unused . insert (id) ; } }}}
mkitem!{mkstruct!{struct UnusedImportCheckVisitor < 'a , 'ra , 'tcx > { r : & 'a mut Resolver < 'ra , 'tcx > , # [doc = " All the (so far) unused imports, grouped path list"] unused_imports : FxIndexMap < ast :: NodeId , UnusedImport > , extern_crate_items : Vec < ExternCrateToLint > , base_use_tree : Option < & 'a ast :: UseTree > , base_id : ast :: NodeId , item_span : Span , }}}
mkitem!{mkstruct!{struct ExternCrateToLint { id : ast :: NodeId , # [doc = " Span from the item"] span : Span , # [doc = " Span to use to suggest complete removal."] span_with_attributes : Span , # [doc = " Span of the visibility, if any."] vis_span : Span , # [doc = " Whether the item has attrs."] has_attrs : bool , # [doc = " Name used to refer to the crate."] ident : Ident , # [doc = " Whether the statement renames the crate `extern crate orig_name as new_name;`."] renames : bool , }}}
mkitem!{mkimpl!{impl < 'a , 'ra , 'tcx > UnusedImportCheckVisitor < 'a , 'ra , 'tcx > { fn check_import (& mut self , id : ast :: NodeId) { let used = self . r . used_imports . contains (& id) ; let def_id = self . r . local_def_id (id) ; if ! used { if self . r . maybe_unused_trait_imports . contains (& def_id) { return ; } self . unused_import (self . base_id) . add (id) ; } else { self . r . maybe_unused_trait_imports . swap_remove (& def_id) ; if let Some (i) = self . unused_imports . get_mut (& self . base_id) { i . unused . remove (& id) ; } } } fn check_use_tree (& mut self , use_tree : & 'a ast :: UseTree , id : ast :: NodeId) { if self . r . effective_visibilities . is_exported (self . r . local_def_id (id)) { self . check_import_as_underscore (use_tree , id) ; return ; } if let ast :: UseTreeKind :: Nested { ref items , .. } = use_tree . kind { if items . is_empty () { self . unused_import (self . base_id) . add (id) ; } } else { self . check_import (id) ; } } fn unused_import (& mut self , id : ast :: NodeId) -> & mut UnusedImport { let use_tree_id = self . base_id ; let use_tree = self . base_use_tree . unwrap () . clone () ; let item_span = self . item_span ; self . unused_imports . entry (id) . or_insert_with (| | UnusedImport { use_tree , use_tree_id , item_span , unused : Default :: default () , }) } fn check_import_as_underscore (& mut self , item : & ast :: UseTree , id : ast :: NodeId) { match item . kind { ast :: UseTreeKind :: Simple (Some (ident)) => { if ident . name == kw :: Underscore && ! self . r . import_res_map . get (& id) . is_some_and (| per_ns | { matches ! (per_ns . type_ns , Some (Res :: Def (DefKind :: Trait | DefKind :: TraitAlias , _))) }) { self . unused_import (self . base_id) . add (id) ; } } ast :: UseTreeKind :: Nested { ref items , .. } => self . check_imports_as_underscore (items) , _ => { } } } fn check_imports_as_underscore (& mut self , items : & [(ast :: UseTree , ast :: NodeId)]) { for (item , id) in items { self . check_import_as_underscore (item , * id) ; } } fn report_unused_extern_crate_items (& mut self , maybe_unused_extern_crates : FxHashMap < ast :: NodeId , Span > ,) { let tcx = self . r . tcx () ; for extern_crate in & self . extern_crate_items { let warn_if_unused = ! extern_crate . ident . name . as_str () . starts_with ('_') ; if warn_if_unused { if let Some (& span) = maybe_unused_extern_crates . get (& extern_crate . id) { self . r . lint_buffer . buffer_lint (UNUSED_EXTERN_CRATES , extern_crate . id , span , BuiltinLintDiag :: UnusedExternCrate { span : extern_crate . span , removal_span : extern_crate . span_with_attributes , } ,) ; continue ; } } if ! tcx . sess . at_least_rust_2018 () { continue ; } if extern_crate . has_attrs { continue ; } if extern_crate . renames { continue ; } if self . r . extern_prelude . get (& Macros20NormalizedIdent :: new (extern_crate . ident)) . is_none_or (| entry | entry . introduced_by_item ()) { continue ; } let module = self . r . get_nearest_non_block_module (self . r . local_def_id (extern_crate . id) . to_def_id ()) ; if module . no_implicit_prelude { continue ; } let vis_span = extern_crate . vis_span . find_ancestor_inside (extern_crate . span) . unwrap_or (extern_crate . vis_span) ; let ident_span = extern_crate . ident . span . find_ancestor_inside (extern_crate . span) . unwrap_or (extern_crate . ident . span) ; self . r . lint_buffer . buffer_lint (UNUSED_EXTERN_CRATES , extern_crate . id , extern_crate . span , BuiltinLintDiag :: ExternCrateNotIdiomatic { vis_span , ident_span } ,) ; } } }}}
mkitem!{mkimpl!{impl < 'a , 'ra , 'tcx > Visitor < 'a > for UnusedImportCheckVisitor < 'a , 'ra , 'tcx > { fn visit_item (& mut self , item : & 'a ast :: Item) { self . item_span = item . span_with_attributes () ; match & item . kind { ast :: ItemKind :: Use (..) if item . span . is_dummy () => return , ast :: ItemKind :: Use (use_tree) => { self . base_id = item . id ; self . base_use_tree = Some (use_tree) ; self . check_use_tree (use_tree , item . id) ; } & ast :: ItemKind :: ExternCrate (orig_name , ident) => { self . extern_crate_items . push (ExternCrateToLint { id : item . id , span : item . span , vis_span : item . vis . span , span_with_attributes : item . span_with_attributes () , has_attrs : ! item . attrs . is_empty () , ident , renames : orig_name . is_some () , }) ; } _ => { } } visit :: walk_item (self , item) ; } fn visit_nested_use_tree (& mut self , use_tree : & 'a ast :: UseTree , id : ast :: NodeId) { self . check_use_tree (use_tree , id) ; visit :: walk_use_tree (self , use_tree) ; } }}}
mkitem!{mkenum!{enum UnusedSpanResult { Used , Unused { spans : Vec < Span > , remove : Span } , PartialUnused { spans : Vec < Span > , remove : Vec < Span > } , }}}

macro_rules! calc_unused_spans_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function calc_unused_spans in module {}", module_path!());
    };
}

mkfn!{
    calc_unused_spans_introspect!();
    fn calc_unused_spans (unused_import : & UnusedImport , use_tree : & ast :: UseTree , use_tree_id : ast :: NodeId ,) -> UnusedSpanResult { let full_span = if unused_import . use_tree . span == use_tree . span { unused_import . item_span } else { use_tree . span } ; match use_tree . kind { ast :: UseTreeKind :: Simple (..) | ast :: UseTreeKind :: Glob => { if unused_import . unused . contains (& use_tree_id) { UnusedSpanResult :: Unused { spans : vec ! [use_tree . span] , remove : full_span } } else { UnusedSpanResult :: Used } } ast :: UseTreeKind :: Nested { items : ref nested , span : tree_span } => { if nested . is_empty () { return UnusedSpanResult :: Unused { spans : vec ! [use_tree . span] , remove : full_span } ; } let mut unused_spans = Vec :: new () ; let mut to_remove = Vec :: new () ; let mut used_children = 0 ; let mut contains_self = false ; let mut previous_unused = false ; for (pos , (use_tree , use_tree_id)) in nested . iter () . enumerate () { let remove = match calc_unused_spans (unused_import , use_tree , * use_tree_id) { UnusedSpanResult :: Used => { used_children += 1 ; None } UnusedSpanResult :: Unused { mut spans , remove } => { unused_spans . append (& mut spans) ; Some (remove) } UnusedSpanResult :: PartialUnused { mut spans , remove : mut to_remove_extra } => { used_children += 1 ; unused_spans . append (& mut spans) ; to_remove . append (& mut to_remove_extra) ; None } } ; if let Some (remove) = remove { let remove_span = if nested . len () == 1 { remove } else if pos == nested . len () - 1 || used_children > 0 { nested [pos - 1] . 0 . span . shrink_to_hi () . to (use_tree . span) } else { use_tree . span . to (nested [pos + 1] . 0 . span . shrink_to_lo ()) } ; if previous_unused && ! to_remove . is_empty () { let previous = to_remove . pop () . unwrap () ; to_remove . push (previous . to (remove_span)) ; } else { to_remove . push (remove_span) ; } } contains_self |= use_tree . prefix == kw :: SelfLower && matches ! (use_tree . kind , ast :: UseTreeKind :: Simple (_)) && ! unused_import . unused . contains (& use_tree_id) ; previous_unused = remove . is_some () ; } if unused_spans . is_empty () { UnusedSpanResult :: Used } else if used_children == 0 { UnusedSpanResult :: Unused { spans : unused_spans , remove : full_span } } else { if used_children == 1 && ! contains_self { to_remove . push (tree_span . shrink_to_lo () . to (nested . first () . unwrap () . 0 . span . shrink_to_lo ()) ,) ; to_remove . push (nested . last () . unwrap () . 0 . span . shrink_to_hi () . to (tree_span . shrink_to_hi ()) ,) ; } UnusedSpanResult :: PartialUnused { spans : unused_spans , remove : to_remove } } } } }
}
mkitem!{mkimpl!{impl Resolver < '_ , '_ > { pub (crate) fn check_unused (& mut self , krate : & ast :: Crate) { let tcx = self . tcx ; let mut maybe_unused_extern_crates = FxHashMap :: default () ; for import in self . potentially_unused_imports . iter () { match import . kind { _ if import . vis . is_public () || import . span . is_dummy () || self . import_use_map . contains_key (import) => { if let ImportKind :: MacroUse { .. } = import . kind { if ! import . span . is_dummy () { self . lint_buffer . buffer_lint (MACRO_USE_EXTERN_CRATE , import . root_id , import . span , BuiltinLintDiag :: MacroUseDeprecated ,) ; } } } ImportKind :: ExternCrate { id , .. } => { let def_id = self . local_def_id (id) ; if self . extern_crate_map . get (& def_id) . is_none_or (| & cnum | { ! tcx . is_compiler_builtins (cnum) && ! tcx . is_panic_runtime (cnum) && ! tcx . has_global_allocator (cnum) && ! tcx . has_panic_handler (cnum) }) { maybe_unused_extern_crates . insert (id , import . span) ; } } ImportKind :: MacroUse { .. } => { self . lint_buffer . buffer_lint (UNUSED_IMPORTS , import . root_id , import . span , BuiltinLintDiag :: UnusedMacroUse ,) ; } _ => { } } } let mut visitor = UnusedImportCheckVisitor { r : self , unused_imports : Default :: default () , extern_crate_items : Default :: default () , base_use_tree : None , base_id : ast :: DUMMY_NODE_ID , item_span : DUMMY_SP , } ; visit :: walk_crate (& mut visitor , krate) ; visitor . report_unused_extern_crate_items (maybe_unused_extern_crates) ; for unused in visitor . unused_imports . values () { let (spans , remove_spans) = match calc_unused_spans (unused , & unused . use_tree , unused . use_tree_id) { UnusedSpanResult :: Used => continue , UnusedSpanResult :: Unused { spans , remove } => (spans , vec ! [remove]) , UnusedSpanResult :: PartialUnused { spans , remove } => (spans , remove) , } ; let ms = MultiSpan :: from_spans (spans) ; let mut span_snippets = ms . primary_spans () . iter () . filter_map (| span | tcx . sess . source_map () . span_to_snippet (* span) . ok ()) . map (| s | format ! ("`{s}`")) . collect :: < Vec < String > > () ; span_snippets . sort () ; let remove_whole_use = remove_spans . len () == 1 && remove_spans [0] == unused . item_span ; let num_to_remove = ms . primary_spans () . len () ; let test_module_span = if tcx . sess . is_test_crate () { None } else { let parent_module = visitor . r . get_nearest_non_block_module (visitor . r . local_def_id (unused . use_tree_id) . to_def_id () ,) ; match module_to_string (parent_module) { Some (module) if module == "test" || module == "tests" || module . starts_with ("test_") || module . starts_with ("tests_") || module . ends_with ("_test") || module . ends_with ("_tests") => { Some (parent_module . span) } _ => None , } } ; visitor . r . lint_buffer . buffer_lint (UNUSED_IMPORTS , unused . use_tree_id , ms , BuiltinLintDiag :: UnusedImports { remove_whole_use , num_to_remove , remove_spans , test_module_span , span_snippets , } ,) ; } let unused_imports = visitor . unused_imports ; let mut check_redundant_imports = FxIndexSet :: default () ; for module in self . arenas . local_modules () . iter () { for (_key , resolution) in self . resolutions (* module) . borrow () . iter () { if let Some (binding) = resolution . borrow () . best_binding () && let NameBindingKind :: Import { import , .. } = binding . kind && let ImportKind :: Single { id , .. } = import . kind { if let Some (unused_import) = unused_imports . get (& import . root_id) && unused_import . unused . contains (& id) { continue ; } check_redundant_imports . insert (import) ; } } } let mut redundant_imports = UnordSet :: default () ; for import in check_redundant_imports { if self . check_for_redundant_imports (import) && let Some (id) = import . id () { redundant_imports . insert (id) ; } } for unn_qua in & self . potentially_unnecessary_qualifications { if let LexicalScopeBinding :: Item (name_binding) = unn_qua . binding && let NameBindingKind :: Import { import , .. } = name_binding . kind && (is_unused_import (import , & unused_imports) || is_redundant_import (import , & redundant_imports)) { continue ; } self . lint_buffer . buffer_lint (UNUSED_QUALIFICATIONS , unn_qua . node_id , unn_qua . path_span , BuiltinLintDiag :: UnusedQualifications { removal_span : unn_qua . removal_span } ,) ; } fn is_redundant_import (import : Import < '_ > , redundant_imports : & UnordSet < ast :: NodeId > ,) -> bool { if let Some (id) = import . id () && redundant_imports . contains (& id) { return true ; } false } fn is_unused_import (import : Import < '_ > , unused_imports : & FxIndexMap < ast :: NodeId , UnusedImport > ,) -> bool { if let Some (unused_import) = unused_imports . get (& import . root_id) && let Some (id) = import . id () && unused_import . unused . contains (& id) { return true ; } false } } }}}