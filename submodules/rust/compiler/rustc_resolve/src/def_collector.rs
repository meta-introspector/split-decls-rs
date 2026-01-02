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
mkuse!{use rustc_ast :: visit :: FnKind ;}
mkuse!{use rustc_ast :: * ;}
mkuse!{use rustc_attr_parsing :: { AttributeParser , Early , OmitDoc , ShouldEmit } ;}
mkuse!{use rustc_expand :: expand :: AstFragment ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: Target ;}
mkuse!{use rustc_hir :: def :: { CtorKind , CtorOf , DefKind } ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_middle :: span_bug ;}
mkuse!{use rustc_span :: hygiene :: LocalExpnId ;}
mkuse!{use rustc_span :: { Span , Symbol , sym } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: { ImplTraitContext , InvocationParent , Resolver } ;}

macro_rules! collect_definitions_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_definitions in module {}", module_path!());
    };
}

mkfn!{
    collect_definitions_introspect!();
    pub (crate) fn collect_definitions (resolver : & mut Resolver < '_ , '_ > , fragment : & AstFragment , expansion : LocalExpnId ,) { let invocation_parent = resolver . invocation_parents [& expansion] ; let mut visitor = DefCollector { resolver , expansion , invocation_parent } ; fragment . visit_with (& mut visitor) ; }
}
mkitem!{mkstruct!{# [doc = " Creates `DefId`s for nodes in the AST."] struct DefCollector < 'a , 'ra , 'tcx > { resolver : & 'a mut Resolver < 'ra , 'tcx > , invocation_parent : InvocationParent , expansion : LocalExpnId , }}}
mkitem!{mkimpl!{impl < 'a , 'ra , 'tcx > DefCollector < 'a , 'ra , 'tcx > { fn create_def (& mut self , node_id : NodeId , name : Option < Symbol > , def_kind : DefKind , span : Span ,) -> LocalDefId { let parent_def = self . invocation_parent . parent_def ; debug ! ("create_def(node_id={:?}, def_kind={:?}, parent_def={:?})" , node_id , def_kind , parent_def) ; self . resolver . create_def (parent_def , node_id , name , def_kind , self . expansion . to_expn_id () , span . with_parent (None) ,) . def_id () } fn with_parent < F : FnOnce (& mut Self) > (& mut self , parent_def : LocalDefId , f : F) { let orig_parent_def = mem :: replace (& mut self . invocation_parent . parent_def , parent_def) ; f (self) ; self . invocation_parent . parent_def = orig_parent_def ; } fn with_impl_trait < F : FnOnce (& mut Self) > (& mut self , impl_trait_context : ImplTraitContext , f : F ,) { let orig_itc = mem :: replace (& mut self . invocation_parent . impl_trait_context , impl_trait_context) ; f (self) ; self . invocation_parent . impl_trait_context = orig_itc ; } fn collect_field (& mut self , field : & 'a FieldDef , index : Option < usize >) { let index = | this : & Self | { index . unwrap_or_else (| | { let node_id = NodeId :: placeholder_from_expn_id (this . expansion) ; this . resolver . placeholder_field_indices [& node_id] }) } ; if field . is_placeholder { let old_index = self . resolver . placeholder_field_indices . insert (field . id , index (self)) ; assert ! (old_index . is_none () , "placeholder field index is reset for a node ID") ; self . visit_macro_invoc (field . id) ; } else { let name = field . ident . map_or_else (| | sym :: integer (index (self)) , | ident | ident . name) ; let def = self . create_def (field . id , Some (name) , DefKind :: Field , field . span) ; self . with_parent (def , | this | visit :: walk_field_def (this , field)) ; } } fn visit_macro_invoc (& mut self , id : NodeId) { let id = id . placeholder_to_expn_id () ; let old_parent = self . resolver . invocation_parents . insert (id , self . invocation_parent) ; assert ! (old_parent . is_none () , "parent `LocalDefId` is reset for an invocation") ; } }}}
mkitem!{mkimpl!{impl < 'a , 'ra , 'tcx > visit :: Visitor < 'a > for DefCollector < 'a , 'ra , 'tcx > { fn visit_item (& mut self , i : & 'a Item) { let mut opt_macro_data = None ; let def_kind = match & i . kind { ItemKind :: Impl (i) => DefKind :: Impl { of_trait : i . of_trait . is_some () } , ItemKind :: ForeignMod (..) => DefKind :: ForeignMod , ItemKind :: Mod (..) => DefKind :: Mod , ItemKind :: Trait (..) => DefKind :: Trait , ItemKind :: TraitAlias (..) => DefKind :: TraitAlias , ItemKind :: Enum (..) => DefKind :: Enum , ItemKind :: Struct (..) => DefKind :: Struct , ItemKind :: Union (..) => DefKind :: Union , ItemKind :: ExternCrate (..) => DefKind :: ExternCrate , ItemKind :: TyAlias (..) => DefKind :: TyAlias , ItemKind :: Static (s) => DefKind :: Static { safety : hir :: Safety :: Safe , mutability : s . mutability , nested : false , } , ItemKind :: Const (..) => DefKind :: Const , ItemKind :: Fn (..) | ItemKind :: Delegation (..) => DefKind :: Fn , ItemKind :: MacroDef (ident , def) => { let edition = i . span . edition () ; let mut parser = AttributeParser :: < '_ , Early > :: new (& self . resolver . tcx . sess , self . resolver . tcx . features () , Vec :: new () , Early { emit_errors : ShouldEmit :: Nothing } ,) ; let attrs = parser . parse_attribute_list (& i . attrs , i . span , i . id , Target :: MacroDef , OmitDoc :: Skip , std :: convert :: identity , | _l | { } ,) ; let macro_data = self . resolver . compile_macro (def , * ident , & attrs , i . span , i . id , edition) ; let macro_kinds = macro_data . ext . macro_kinds () ; opt_macro_data = Some (macro_data) ; DefKind :: Macro (macro_kinds) } ItemKind :: GlobalAsm (..) => DefKind :: GlobalAsm , ItemKind :: Use (use_tree) => { self . create_def (i . id , None , DefKind :: Use , use_tree . span) ; return visit :: walk_item (self , i) ; } ItemKind :: MacCall (..) | ItemKind :: DelegationMac (..) => { return self . visit_macro_invoc (i . id) ; } } ; let def_id = self . create_def (i . id , i . kind . ident () . map (| ident | ident . name) , def_kind , i . span) ; if let Some (macro_data) = opt_macro_data { self . resolver . new_local_macro (def_id , macro_data) ; } self . with_parent (def_id , | this | { this . with_impl_trait (ImplTraitContext :: Existential , | this | { match i . kind { ItemKind :: Struct (_ , _ , ref struct_def) | ItemKind :: Union (_ , _ , ref struct_def) => { if let Some ((ctor_kind , ctor_node_id)) = CtorKind :: from_ast (struct_def) { this . create_def (ctor_node_id , None , DefKind :: Ctor (CtorOf :: Struct , ctor_kind) , i . span ,) ; } } _ => { } } visit :: walk_item (this , i) ; }) }) ; } fn visit_fn (& mut self , fn_kind : FnKind < 'a > , span : Span , _ : NodeId) { match fn_kind { FnKind :: Fn (_ctxt , _vis , Fn { sig : FnSig { header , decl , span : _ } , ident , generics , contract , body , .. } ,) if let Some (coroutine_kind) = header . coroutine_kind => { self . visit_ident (ident) ; self . visit_fn_header (header) ; self . visit_generics (generics) ; if let Some (contract) = contract { self . visit_contract (contract) ; } let FnDecl { inputs , output } = & * * decl ; for param in inputs { self . visit_param (param) ; } let (return_id , return_span) = coroutine_kind . return_id () ; let return_def = self . create_def (return_id , None , DefKind :: OpaqueTy , return_span) ; self . with_parent (return_def , | this | this . visit_fn_ret_ty (output)) ; if let Some (body) = body { let closure_def = self . create_def (coroutine_kind . closure_id () , None , DefKind :: Closure , span) ; self . with_parent (closure_def , | this | this . visit_block (body)) ; } } FnKind :: Closure (binder , Some (coroutine_kind) , decl , body) => { self . visit_closure_binder (binder) ; visit :: walk_fn_decl (self , decl) ; let coroutine_def = self . create_def (coroutine_kind . closure_id () , None , DefKind :: Closure , span) ; self . with_parent (coroutine_def , | this | this . visit_expr (body)) ; } _ => visit :: walk_fn (self , fn_kind) , } } fn visit_nested_use_tree (& mut self , use_tree : & 'a UseTree , id : NodeId) { self . create_def (id , None , DefKind :: Use , use_tree . span) ; visit :: walk_use_tree (self , use_tree) ; } fn visit_foreign_item (& mut self , fi : & 'a ForeignItem) { let (ident , def_kind) = match fi . kind { ForeignItemKind :: Static (box StaticItem { ident , ty : _ , mutability , expr : _ , safety , define_opaque : _ , }) => { let safety = match safety { ast :: Safety :: Unsafe (_) | ast :: Safety :: Default => hir :: Safety :: Unsafe , ast :: Safety :: Safe (_) => hir :: Safety :: Safe , } ; (ident , DefKind :: Static { safety , mutability , nested : false }) } ForeignItemKind :: Fn (box Fn { ident , .. }) => (ident , DefKind :: Fn) , ForeignItemKind :: TyAlias (box TyAlias { ident , .. }) => (ident , DefKind :: ForeignTy) , ForeignItemKind :: MacCall (_) => return self . visit_macro_invoc (fi . id) , } ; let def = self . create_def (fi . id , Some (ident . name) , def_kind , fi . span) ; self . with_parent (def , | this | visit :: walk_item (this , fi)) ; } fn visit_variant (& mut self , v : & 'a Variant) { if v . is_placeholder { return self . visit_macro_invoc (v . id) ; } let def = self . create_def (v . id , Some (v . ident . name) , DefKind :: Variant , v . span) ; self . with_parent (def , | this | { if let Some ((ctor_kind , ctor_node_id)) = CtorKind :: from_ast (& v . data) { this . create_def (ctor_node_id , None , DefKind :: Ctor (CtorOf :: Variant , ctor_kind) , v . span ,) ; } visit :: walk_variant (this , v) }) ; } fn visit_where_predicate (& mut self , pred : & 'a WherePredicate) { if pred . is_placeholder { self . visit_macro_invoc (pred . id) } else { visit :: walk_where_predicate (self , pred) } } fn visit_variant_data (& mut self , data : & 'a VariantData) { for (index , field) in data . fields () . iter () . enumerate () { self . collect_field (field , Some (index)) ; } } fn visit_generic_param (& mut self , param : & 'a GenericParam) { if param . is_placeholder { self . visit_macro_invoc (param . id) ; return ; } let def_kind = match param . kind { GenericParamKind :: Lifetime { .. } => DefKind :: LifetimeParam , GenericParamKind :: Type { .. } => DefKind :: TyParam , GenericParamKind :: Const { .. } => DefKind :: ConstParam , } ; self . create_def (param . id , Some (param . ident . name) , def_kind , param . ident . span) ; self . with_impl_trait (ImplTraitContext :: Universal , | this | { visit :: walk_generic_param (this , param) }) ; } fn visit_assoc_item (& mut self , i : & 'a AssocItem , ctxt : visit :: AssocCtxt) { let (ident , def_kind) = match & i . kind { AssocItemKind :: Fn (box Fn { ident , .. }) | AssocItemKind :: Delegation (box Delegation { ident , .. }) => (* ident , DefKind :: AssocFn) , AssocItemKind :: Const (box ConstItem { ident , .. }) => (* ident , DefKind :: AssocConst) , AssocItemKind :: Type (box TyAlias { ident , .. }) => (* ident , DefKind :: AssocTy) , AssocItemKind :: MacCall (..) | AssocItemKind :: DelegationMac (..) => { return self . visit_macro_invoc (i . id) ; } } ; let def = self . create_def (i . id , Some (ident . name) , def_kind , i . span) ; self . with_parent (def , | this | visit :: walk_assoc_item (this , i , ctxt)) ; } fn visit_pat (& mut self , pat : & 'a Pat) { match pat . kind { PatKind :: MacCall (..) => self . visit_macro_invoc (pat . id) , _ => visit :: walk_pat (self , pat) , } } fn visit_anon_const (& mut self , constant : & 'a AnonConst) { let parent = self . create_def (constant . id , None , DefKind :: AnonConst , constant . value . span) ; self . with_parent (parent , | this | visit :: walk_anon_const (this , constant)) ; } fn visit_expr (& mut self , expr : & 'a Expr) { let parent_def = match expr . kind { ExprKind :: MacCall (..) => return self . visit_macro_invoc (expr . id) , ExprKind :: Closure (..) | ExprKind :: Gen (..) => { self . create_def (expr . id , None , DefKind :: Closure , expr . span) } ExprKind :: ConstBlock (ref constant) => { for attr in & expr . attrs { visit :: walk_attribute (self , attr) ; } let def = self . create_def (constant . id , None , DefKind :: InlineConst , constant . value . span) ; self . with_parent (def , | this | visit :: walk_anon_const (this , constant)) ; return ; } _ => self . invocation_parent . parent_def , } ; self . with_parent (parent_def , | this | visit :: walk_expr (this , expr)) } fn visit_ty (& mut self , ty : & 'a Ty) { match ty . kind { TyKind :: MacCall (..) => self . visit_macro_invoc (ty . id) , TyKind :: ImplTrait (opaque_id , _) => { let name = * self . resolver . impl_trait_names . get (& ty . id) . unwrap_or_else (| | span_bug ! (ty . span , "expected this opaque to be named")) ; let kind = match self . invocation_parent . impl_trait_context { ImplTraitContext :: Universal => DefKind :: TyParam , ImplTraitContext :: Existential => DefKind :: OpaqueTy , ImplTraitContext :: InBinding => return visit :: walk_ty (self , ty) , } ; let id = self . create_def (opaque_id , Some (name) , kind , ty . span) ; match self . invocation_parent . impl_trait_context { ImplTraitContext :: Universal => visit :: walk_ty (self , ty) , ImplTraitContext :: Existential => { self . with_parent (id , | this | visit :: walk_ty (this , ty)) } ImplTraitContext :: InBinding => unreachable ! () , } ; } _ => visit :: walk_ty (self , ty) , } } fn visit_stmt (& mut self , stmt : & 'a Stmt) { match stmt . kind { StmtKind :: MacCall (..) => self . visit_macro_invoc (stmt . id) , StmtKind :: Let (ref local) => self . with_impl_trait (ImplTraitContext :: InBinding , | this | { visit :: walk_local (this , local) }) , _ => visit :: walk_stmt (self , stmt) , } } fn visit_arm (& mut self , arm : & 'a Arm) { if arm . is_placeholder { self . visit_macro_invoc (arm . id) } else { visit :: walk_arm (self , arm) } } fn visit_expr_field (& mut self , f : & 'a ExprField) { if f . is_placeholder { self . visit_macro_invoc (f . id) } else { visit :: walk_expr_field (self , f) } } fn visit_pat_field (& mut self , fp : & 'a PatField) { if fp . is_placeholder { self . visit_macro_invoc (fp . id) } else { visit :: walk_pat_field (self , fp) } } fn visit_param (& mut self , p : & 'a Param) { if p . is_placeholder { self . visit_macro_invoc (p . id) } else { self . with_impl_trait (ImplTraitContext :: Universal , | this | visit :: walk_param (this , p)) } } fn visit_field_def (& mut self , field : & 'a FieldDef) { self . collect_field (field , None) ; } fn visit_crate (& mut self , krate : & 'a Crate) { if krate . is_placeholder { self . visit_macro_invoc (krate . id) } else { visit :: walk_crate (self , krate) } } fn visit_attribute (& mut self , attr : & 'a Attribute) -> Self :: Result { let orig_in_attr = mem :: replace (& mut self . invocation_parent . in_attr , true) ; visit :: walk_attribute (self , attr) ; self . invocation_parent . in_attr = orig_in_attr ; } fn visit_inline_asm (& mut self , asm : & 'a InlineAsm) { let InlineAsm { asm_macro : _ , template : _ , template_strs : _ , operands , clobber_abis : _ , options : _ , line_spans : _ , } = asm ; for (op , _span) in operands { match op { InlineAsmOperand :: In { expr , reg : _ } | InlineAsmOperand :: Out { expr : Some (expr) , reg : _ , late : _ } | InlineAsmOperand :: InOut { expr , reg : _ , late : _ } => { self . visit_expr (expr) ; } InlineAsmOperand :: Out { expr : None , reg : _ , late : _ } => { } InlineAsmOperand :: SplitInOut { in_expr , out_expr , reg : _ , late : _ } => { self . visit_expr (in_expr) ; if let Some (expr) = out_expr { self . visit_expr (expr) ; } } InlineAsmOperand :: Const { anon_const } => { let def = self . create_def (anon_const . id , None , DefKind :: InlineConst , anon_const . value . span ,) ; self . with_parent (def , | this | visit :: walk_anon_const (this , anon_const)) ; } InlineAsmOperand :: Sym { sym } => self . visit_inline_asm_sym (sym) , InlineAsmOperand :: Label { block } => self . visit_block (block) , } } } }}}