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
mkuse!{use rustc_ast :: visit :: BoundKind ;}
mkuse!{use rustc_ast :: { self as ast , NodeId , visit as ast_visit } ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashMap , FxHashSet } ;}
mkuse!{use rustc_data_structures :: thousands :: usize_with_underscores ;}
mkuse!{use rustc_hir :: { self as hir , AmbigArg , HirId , intravisit as hir_visit } ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use rustc_span :: def_id :: LocalDefId ;}
mkitem!{mkstruct!{struct NodeStats { count : usize , size : usize , }}}
mkitem!{mkimpl!{impl NodeStats { fn new () -> NodeStats { NodeStats { count : 0 , size : 0 } } fn accum_size (& self) -> usize { self . count * self . size } }}}
mkitem!{mkstruct!{struct Node { stats : NodeStats , subnodes : FxHashMap < & 'static str , NodeStats > , }}}
mkitem!{mkimpl!{impl Node { fn new () -> Node { Node { stats : NodeStats :: new () , subnodes : FxHashMap :: default () } } }}}
mkitem!{mkstruct!{#[doc = " This type measures the size of AST and HIR nodes, by implementing the AST"] #[doc = " and HIR `Visitor` traits. But we don't measure every visited type because"] #[doc = " that could cause double counting."] #[doc = ""] #[doc = " For example, `ast::Visitor` has `visit_ident`, but `Ident`s are always"] #[doc = " stored inline within other AST nodes, so we don't implement `visit_ident`"] #[doc = " here. In contrast, we do implement `visit_expr` because `ast::Expr` is"] #[doc = " always stored as `Box<ast::Expr>`, and every such expression should be"] #[doc = " measured separately."] #[doc = ""] #[doc = " In general, a `visit_foo` method should be implemented here if the"] #[doc = " corresponding `Foo` type is always stored on its own, e.g.: `Box<Foo>`,"] #[doc = " `Box<Foo>`, `Vec<Foo>`, `Box<[Foo]>`."] #[doc = ""] #[doc = " There are some types in the AST and HIR tree that the visitors do not have"] #[doc = " a `visit_*` method for, and so we cannot measure these, which is"] #[doc = " unfortunate."] struct StatCollector < 'k > { tcx : Option < TyCtxt < 'k > > , nodes : FxHashMap < & 'static str , Node > , seen : FxHashSet < HirId > , }}}

macro_rules! print_hir_stats_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function print_hir_stats in module {}", module_path!());
    };
}

mkfn!{
    print_hir_stats_introspect!();
    pub fn print_hir_stats (tcx : TyCtxt < '_ >) { let mut collector = StatCollector { tcx : Some (tcx) , nodes : FxHashMap :: default () , seen : FxHashSet :: default () } ; tcx . hir_walk_toplevel_module (& mut collector) ; tcx . hir_walk_attributes (& mut collector) ; collector . print (tcx , "HIR STATS" , "hir-stats") ; }
}

macro_rules! print_ast_stats_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function print_ast_stats in module {}", module_path!());
    };
}

mkfn!{
    print_ast_stats_introspect!();
    pub fn print_ast_stats (tcx : TyCtxt < '_ > , krate : & ast :: Crate) { use rustc_ast :: visit :: Visitor ; let mut collector = StatCollector { tcx : None , nodes : FxHashMap :: default () , seen : FxHashSet :: default () } ; collector . visit_crate (krate) ; collector . print (tcx , "POST EXPANSION AST STATS" , "ast-stats") ; }
}
mkitem!{mkimpl!{impl < 'k > StatCollector < 'k > { fn record < T > (& mut self , label : & 'static str , id : Option < HirId > , val : & T) { self . record_inner (label , None , id , val) ; } fn record_variant < T > (& mut self , label1 : & 'static str , label2 : & 'static str , id : Option < HirId > , val : & T ,) { self . record_inner (label1 , Some (label2) , id , val) ; } fn record_inner < T > (& mut self , label1 : & 'static str , label2 : Option < & 'static str > , id : Option < HirId > , val : & T ,) { if id . is_some_and (| x | ! self . seen . insert (x)) { return ; } let node = self . nodes . entry (label1) . or_insert (Node :: new ()) ; node . stats . count += 1 ; node . stats . size = size_of_val (val) ; if let Some (label2) = label2 { let subnode = node . subnodes . entry (label2) . or_insert (NodeStats :: new ()) ; subnode . count += 1 ; subnode . size = size_of_val (val) ; } } fn print (& self , tcx : TyCtxt < '_ > , title : & str , prefix : & str) { use std :: fmt :: Write ; #[allow (rustc :: potential_query_instability)] let mut nodes : Vec < _ > = self . nodes . iter () . collect () ; nodes . sort_by_cached_key (| (label , node) | (node . stats . accum_size () , label . to_owned ())) ; nodes . reverse () ; let name_w = 18 ; let acc_size1_w = 10 ; let acc_size2_w = 8 ; let acc_size_w = acc_size1_w + acc_size2_w ; let count_w = 14 ; let item_size_w = 14 ; let banner_w = name_w + acc_size_w + count_w + item_size_w ; let total_size = nodes . iter () . map (| (_ , node) | node . stats . accum_size ()) . sum () ; let total_count = nodes . iter () . map (| (_ , node) | node . stats . count) . sum () ; let mut s = String :: new () ; _ = writeln ! (s , "{prefix} {}" , "=" . repeat (banner_w)) ; _ = writeln ! (s , "{prefix} {title}: {}" , tcx . crate_name (hir :: def_id :: LOCAL_CRATE)) ; _ = writeln ! (s , "{prefix} {:<name_w$}{:>acc_size_w$}{:>count_w$}{:>item_size_w$}" , "Name" , "Accumulated Size" , "Count" , "Item Size") ; _ = writeln ! (s , "{prefix} {}" , "-" . repeat (banner_w)) ; let percent = | m , n | (m * 100) as f64 / n as f64 ; for (label , node) in nodes { let size = node . stats . accum_size () ; _ = writeln ! (s , "{prefix} {:<name_w$}{:>acc_size1_w$} ({:4.1}%){:>count_w$}{:>item_size_w$}" , label , usize_with_underscores (size) , percent (size , total_size) , usize_with_underscores (node . stats . count) , usize_with_underscores (node . stats . size)) ; if ! node . subnodes . is_empty () { #[allow (rustc :: potential_query_instability)] let mut subnodes : Vec < _ > = node . subnodes . iter () . collect () ; subnodes . sort_by_cached_key (| (label , subnode) | { (subnode . accum_size () , label . to_owned ()) }) ; for (label , subnode) in subnodes { let size = subnode . accum_size () ; _ = writeln ! (s , "{prefix} - {:<name_w$}{:>acc_size1_w$} ({:4.1}%){:>count_w$}" , label , usize_with_underscores (size) , percent (size , total_size) , usize_with_underscores (subnode . count) ,) ; } } } _ = writeln ! (s , "{prefix} {}" , "-" . repeat (banner_w)) ; _ = writeln ! (s , "{prefix} {:<name_w$}{:>acc_size1_w$}{:>acc_size2_w$}{:>count_w$}" , "Total" , usize_with_underscores (total_size) , "" , usize_with_underscores (total_count) ,) ; _ = writeln ! (s , "{prefix} {}" , "=" . repeat (banner_w)) ; eprint ! ("{s}") ; } }}}
mkitem!{macro_rules ! record_variants { (($ self : ident , $ val : expr , $ kind : expr , $ id : expr , $ mod : ident , $ ty : ty , $ tykind : ident) , [$ ($ variant : ident) ,*]) => { match $ kind { $ ($ mod ::$ tykind ::$ variant { .. } => { $ self . record_variant (stringify ! ($ ty) , stringify ! ($ variant) , $ id , $ val) }) * } } ; }}
mkitem!{mkimpl!{impl < 'v > hir_visit :: Visitor < 'v > for StatCollector < 'v > { fn visit_param (& mut self , param : & 'v hir :: Param < 'v >) { self . record ("Param" , Some (param . hir_id) , param) ; hir_visit :: walk_param (self , param) } fn visit_nested_item (& mut self , id : hir :: ItemId) { let nested_item = self . tcx . unwrap () . hir_item (id) ; self . visit_item (nested_item) } fn visit_nested_trait_item (& mut self , trait_item_id : hir :: TraitItemId) { let nested_trait_item = self . tcx . unwrap () . hir_trait_item (trait_item_id) ; self . visit_trait_item (nested_trait_item) } fn visit_nested_impl_item (& mut self , impl_item_id : hir :: ImplItemId) { let nested_impl_item = self . tcx . unwrap () . hir_impl_item (impl_item_id) ; self . visit_impl_item (nested_impl_item) } fn visit_nested_foreign_item (& mut self , id : hir :: ForeignItemId) { let nested_foreign_item = self . tcx . unwrap () . hir_foreign_item (id) ; self . visit_foreign_item (nested_foreign_item) ; } fn visit_nested_body (& mut self , body_id : hir :: BodyId) { let nested_body = self . tcx . unwrap () . hir_body (body_id) ; self . visit_body (nested_body) } fn visit_item (& mut self , i : & 'v hir :: Item < 'v >) { record_variants ! ((self , i , i . kind , Some (i . hir_id ()) , hir , Item , ItemKind) , [ExternCrate , Use , Static , Const , Fn , Macro , Mod , ForeignMod , GlobalAsm , TyAlias , Enum , Struct , Union , Trait , TraitAlias , Impl]) ; hir_visit :: walk_item (self , i) } fn visit_body (& mut self , b : & hir :: Body < 'v >) { self . record ("Body" , None , b) ; hir_visit :: walk_body (self , b) ; } fn visit_mod (& mut self , m : & 'v hir :: Mod < 'v > , _s : Span , _n : HirId) { self . record ("Mod" , None , m) ; hir_visit :: walk_mod (self , m) } fn visit_foreign_item (& mut self , i : & 'v hir :: ForeignItem < 'v >) { record_variants ! ((self , i , i . kind , Some (i . hir_id ()) , hir , ForeignItem , ForeignItemKind) , [Fn , Static , Type]) ; hir_visit :: walk_foreign_item (self , i) } fn visit_local (& mut self , l : & 'v hir :: LetStmt < 'v >) { self . record ("Local" , Some (l . hir_id) , l) ; hir_visit :: walk_local (self , l) } fn visit_block (& mut self , b : & 'v hir :: Block < 'v >) { self . record ("Block" , Some (b . hir_id) , b) ; hir_visit :: walk_block (self , b) } fn visit_stmt (& mut self , s : & 'v hir :: Stmt < 'v >) { record_variants ! ((self , s , s . kind , Some (s . hir_id) , hir , Stmt , StmtKind) , [Let , Item , Expr , Semi]) ; hir_visit :: walk_stmt (self , s) } fn visit_arm (& mut self , a : & 'v hir :: Arm < 'v >) { self . record ("Arm" , Some (a . hir_id) , a) ; hir_visit :: walk_arm (self , a) } fn visit_pat (& mut self , p : & 'v hir :: Pat < 'v >) { record_variants ! ((self , p , p . kind , Some (p . hir_id) , hir , Pat , PatKind) , [Missing , Wild , Binding , Struct , TupleStruct , Or , Never , Tuple , Box , Deref , Ref , Expr , Guard , Range , Slice , Err]) ; hir_visit :: walk_pat (self , p) } fn visit_pat_field (& mut self , f : & 'v hir :: PatField < 'v >) { self . record ("PatField" , Some (f . hir_id) , f) ; hir_visit :: walk_pat_field (self , f) } fn visit_expr (& mut self , e : & 'v hir :: Expr < 'v >) { record_variants ! ((self , e , e . kind , Some (e . hir_id) , hir , Expr , ExprKind) , [ConstBlock , Array , Call , MethodCall , Use , Tup , Binary , Unary , Lit , Cast , Type , DropTemps , Let , If , Loop , Match , Closure , Block , Assign , AssignOp , Field , Index , Path , AddrOf , Break , Continue , Ret , Become , InlineAsm , OffsetOf , Struct , Repeat , Yield , UnsafeBinderCast , Err]) ; hir_visit :: walk_expr (self , e) } fn visit_expr_field (& mut self , f : & 'v hir :: ExprField < 'v >) { self . record ("ExprField" , Some (f . hir_id) , f) ; hir_visit :: walk_expr_field (self , f) } fn visit_ty (& mut self , t : & 'v hir :: Ty < 'v , AmbigArg >) { record_variants ! ((self , t , t . kind , Some (t . hir_id) , hir , Ty , TyKind) , [InferDelegation , Slice , Array , Ptr , Ref , FnPtr , UnsafeBinder , Never , Tup , Path , OpaqueDef , TraitAscription , TraitObject , Typeof , Infer , Pat , Err]) ; hir_visit :: walk_ty (self , t) } fn visit_generic_param (& mut self , p : & 'v hir :: GenericParam < 'v >) { self . record ("GenericParam" , Some (p . hir_id) , p) ; hir_visit :: walk_generic_param (self , p) } fn visit_generics (& mut self , g : & 'v hir :: Generics < 'v >) { self . record ("Generics" , None , g) ; hir_visit :: walk_generics (self , g) } fn visit_where_predicate (& mut self , p : & 'v hir :: WherePredicate < 'v >) { record_variants ! ((self , p , p . kind , Some (p . hir_id) , hir , WherePredicate , WherePredicateKind) , [BoundPredicate , RegionPredicate , EqPredicate]) ; hir_visit :: walk_where_predicate (self , p) } fn visit_fn (& mut self , fk : hir_visit :: FnKind < 'v > , fd : & 'v hir :: FnDecl < 'v > , b : hir :: BodyId , _ : Span , id : LocalDefId ,) { self . record ("FnDecl" , None , fd) ; hir_visit :: walk_fn (self , fk , fd , b , id) } fn visit_use (& mut self , p : & 'v hir :: UsePath < 'v > , _hir_id : HirId) { self . record ("Path" , None , p) ; let hir :: Path { span : _ , res : _ , segments } = * p ; ast_visit :: walk_list ! (self , visit_path_segment , segments) ; } fn visit_trait_item (& mut self , ti : & 'v hir :: TraitItem < 'v >) { record_variants ! ((self , ti , ti . kind , Some (ti . hir_id ()) , hir , TraitItem , TraitItemKind) , [Const , Fn , Type]) ; hir_visit :: walk_trait_item (self , ti) } fn visit_trait_item_ref (& mut self , ti : & 'v hir :: TraitItemId) { self . record ("TraitItemId" , Some (ti . hir_id ()) , ti) ; hir_visit :: walk_trait_item_ref (self , * ti) } fn visit_impl_item (& mut self , ii : & 'v hir :: ImplItem < 'v >) { record_variants ! ((self , ii , ii . kind , Some (ii . hir_id ()) , hir , ImplItem , ImplItemKind) , [Const , Fn , Type]) ; hir_visit :: walk_impl_item (self , ii) } fn visit_foreign_item_ref (& mut self , fi : & 'v hir :: ForeignItemId) { self . record ("ForeignItemId" , Some (fi . hir_id ()) , fi) ; hir_visit :: walk_foreign_item_ref (self , * fi) } fn visit_impl_item_ref (& mut self , ii : & 'v hir :: ImplItemId) { self . record ("ImplItemId" , Some (ii . hir_id ()) , ii) ; hir_visit :: walk_impl_item_ref (self , * ii) } fn visit_param_bound (& mut self , b : & 'v hir :: GenericBound < 'v >) { record_variants ! ((self , b , b , None , hir , GenericBound , GenericBound) , [Trait , Outlives , Use]) ; hir_visit :: walk_param_bound (self , b) } fn visit_field_def (& mut self , s : & 'v hir :: FieldDef < 'v >) { self . record ("FieldDef" , Some (s . hir_id) , s) ; hir_visit :: walk_field_def (self , s) } fn visit_variant (& mut self , v : & 'v hir :: Variant < 'v >) { self . record ("Variant" , None , v) ; hir_visit :: walk_variant (self , v) } fn visit_generic_arg (& mut self , ga : & 'v hir :: GenericArg < 'v >) { record_variants ! ((self , ga , ga , Some (ga . hir_id ()) , hir , GenericArg , GenericArg) , [Lifetime , Type , Const , Infer]) ; match ga { hir :: GenericArg :: Lifetime (lt) => self . visit_lifetime (lt) , hir :: GenericArg :: Type (ty) => self . visit_ty (ty) , hir :: GenericArg :: Const (ct) => self . visit_const_arg (ct) , hir :: GenericArg :: Infer (inf) => self . visit_id (inf . hir_id) , } } fn visit_lifetime (& mut self , lifetime : & 'v hir :: Lifetime) { self . record ("Lifetime" , Some (lifetime . hir_id) , lifetime) ; hir_visit :: walk_lifetime (self , lifetime) } fn visit_path (& mut self , path : & hir :: Path < 'v > , _id : HirId) { self . record ("Path" , None , path) ; hir_visit :: walk_path (self , path) } fn visit_path_segment (& mut self , path_segment : & 'v hir :: PathSegment < 'v >) { self . record ("PathSegment" , None , path_segment) ; hir_visit :: walk_path_segment (self , path_segment) } fn visit_generic_args (& mut self , ga : & 'v hir :: GenericArgs < 'v >) { self . record ("GenericArgs" , None , ga) ; hir_visit :: walk_generic_args (self , ga) } fn visit_assoc_item_constraint (& mut self , constraint : & 'v hir :: AssocItemConstraint < 'v >) { self . record ("AssocItemConstraint" , Some (constraint . hir_id) , constraint) ; hir_visit :: walk_assoc_item_constraint (self , constraint) } fn visit_attribute (& mut self , attr : & 'v hir :: Attribute) { self . record ("Attribute" , None , attr) ; } fn visit_inline_asm (& mut self , asm : & 'v hir :: InlineAsm < 'v > , id : HirId) { self . record ("InlineAsm" , None , asm) ; hir_visit :: walk_inline_asm (self , asm , id) ; } }}}
mkitem!{mkimpl!{impl < 'v > ast_visit :: Visitor < 'v > for StatCollector < 'v > { fn visit_foreign_item (& mut self , i : & 'v ast :: ForeignItem) { record_variants ! ((self , i , i . kind , None , ast , ForeignItem , ForeignItemKind) , [Static , Fn , TyAlias , MacCall]) ; ast_visit :: walk_item (self , i) } fn visit_item (& mut self , i : & 'v ast :: Item) { record_variants ! ((self , i , i . kind , None , ast , Item , ItemKind) , [ExternCrate , Use , Static , Const , Fn , Mod , ForeignMod , GlobalAsm , TyAlias , Enum , Struct , Union , Trait , TraitAlias , Impl , MacCall , MacroDef , Delegation , DelegationMac]) ; ast_visit :: walk_item (self , i) } fn visit_local (& mut self , l : & 'v ast :: Local) { self . record ("Local" , None , l) ; ast_visit :: walk_local (self , l) } fn visit_block (& mut self , b : & 'v ast :: Block) { self . record ("Block" , None , b) ; ast_visit :: walk_block (self , b) } fn visit_stmt (& mut self , s : & 'v ast :: Stmt) { record_variants ! ((self , s , s . kind , None , ast , Stmt , StmtKind) , [Let , Item , Expr , Semi , Empty , MacCall]) ; ast_visit :: walk_stmt (self , s) } fn visit_param (& mut self , p : & 'v ast :: Param) { self . record ("Param" , None , p) ; ast_visit :: walk_param (self , p) } fn visit_arm (& mut self , a : & 'v ast :: Arm) { self . record ("Arm" , None , a) ; ast_visit :: walk_arm (self , a) } fn visit_pat (& mut self , p : & 'v ast :: Pat) { record_variants ! ((self , p , p . kind , None , ast , Pat , PatKind) , [Missing , Wild , Ident , Struct , TupleStruct , Or , Path , Tuple , Box , Deref , Ref , Expr , Range , Slice , Rest , Never , Guard , Paren , MacCall , Err]) ; ast_visit :: walk_pat (self , p) } fn visit_expr (& mut self , e : & 'v ast :: Expr) { #[rustfmt :: skip] record_variants ! ((self , e , e . kind , None , ast , Expr , ExprKind) , [Array , ConstBlock , Call , MethodCall , Tup , Binary , Unary , Lit , Cast , Type , Let , If , While , ForLoop , Loop , Match , Closure , Block , Await , Use , TryBlock , Assign , AssignOp , Field , Index , Range , Underscore , Path , AddrOf , Break , Continue , Ret , InlineAsm , FormatArgs , OffsetOf , MacCall , Struct , Repeat , Paren , Try , Yield , Yeet , Become , IncludedBytes , Gen , UnsafeBinderCast , Err , Dummy]) ; ast_visit :: walk_expr (self , e) } fn visit_ty (& mut self , t : & 'v ast :: Ty) { record_variants ! ((self , t , t . kind , None , ast , Ty , TyKind) , [Slice , Array , Ptr , Ref , PinnedRef , FnPtr , UnsafeBinder , Never , Tup , Path , Pat , TraitObject , ImplTrait , Paren , Typeof , Infer , ImplicitSelf , MacCall , CVarArgs , Dummy , Err]) ; ast_visit :: walk_ty (self , t) } fn visit_generic_param (& mut self , g : & 'v ast :: GenericParam) { self . record ("GenericParam" , None , g) ; ast_visit :: walk_generic_param (self , g) } fn visit_where_predicate (& mut self , p : & 'v ast :: WherePredicate) { record_variants ! ((self , p , & p . kind , None , ast , WherePredicate , WherePredicateKind) , [BoundPredicate , RegionPredicate , EqPredicate]) ; ast_visit :: walk_where_predicate (self , p) } fn visit_fn (& mut self , fk : ast_visit :: FnKind < 'v > , _ : Span , _ : NodeId) { self . record ("FnDecl" , None , fk . decl ()) ; ast_visit :: walk_fn (self , fk) } fn visit_assoc_item (& mut self , i : & 'v ast :: AssocItem , ctxt : ast_visit :: AssocCtxt) { record_variants ! ((self , i , i . kind , None , ast , AssocItem , AssocItemKind) , [Const , Fn , Type , MacCall , Delegation , DelegationMac]) ; ast_visit :: walk_assoc_item (self , i , ctxt) ; } fn visit_param_bound (& mut self , b : & 'v ast :: GenericBound , _ctxt : BoundKind) { record_variants ! ((self , b , b , None , ast , GenericBound , GenericBound) , [Trait , Outlives , Use]) ; ast_visit :: walk_param_bound (self , b) } fn visit_field_def (& mut self , s : & 'v ast :: FieldDef) { self . record ("FieldDef" , None , s) ; ast_visit :: walk_field_def (self , s) } fn visit_variant (& mut self , v : & 'v ast :: Variant) { self . record ("Variant" , None , v) ; ast_visit :: walk_variant (self , v) } fn visit_path_segment (& mut self , path_segment : & 'v ast :: PathSegment) { self . record ("PathSegment" , None , path_segment) ; ast_visit :: walk_path_segment (self , path_segment) } fn visit_generic_args (& mut self , g : & 'v ast :: GenericArgs) { record_variants ! ((self , g , g , None , ast , GenericArgs , GenericArgs) , [AngleBracketed , Parenthesized , ParenthesizedElided]) ; ast_visit :: walk_generic_args (self , g) } fn visit_attribute (& mut self , attr : & 'v ast :: Attribute) { record_variants ! ((self , attr , attr . kind , None , ast , Attribute , AttrKind) , [Normal , DocComment]) ; ast_visit :: walk_attribute (self , attr) } fn visit_expr_field (& mut self , f : & 'v ast :: ExprField) { self . record ("ExprField" , None , f) ; ast_visit :: walk_expr_field (self , f) } fn visit_crate (& mut self , krate : & 'v ast :: Crate) { self . record ("Crate" , None , krate) ; ast_visit :: walk_crate (self , krate) } fn visit_inline_asm (& mut self , asm : & 'v ast :: InlineAsm) { self . record ("InlineAsm" , None , asm) ; ast_visit :: walk_inline_asm (self , asm) } }}}