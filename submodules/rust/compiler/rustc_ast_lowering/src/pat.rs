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
mkuse!{use std :: sync :: Arc ;}
mkuse!{use rustc_ast :: * ;}
mkuse!{use rustc_data_structures :: stack :: ensure_sufficient_stack ;}
mkuse!{use rustc_hir :: def :: { DefKind , Res } ;}
mkuse!{use rustc_hir :: { self as hir , LangItem , Target } ;}
mkuse!{use rustc_middle :: span_bug ;}
mkuse!{use rustc_span :: source_map :: { Spanned , respan } ;}
mkuse!{use rustc_span :: { DesugaringKind , Ident , Span } ;}
mkuse!{use super :: errors :: { ArbitraryExpressionInPattern , ExtraDoubleDot , MisplacedDoubleDot , SubTupleBinding , } ;}
mkuse!{use super :: { ImplTraitContext , LoweringContext , ParamMode , ResolverAstLoweringExt } ;}
mkuse!{use crate :: { AllowReturnTypeNotation , ImplTraitPosition } ;}
mkitem!{mkimpl!{impl < 'a , 'hir > LoweringContext < 'a , 'hir > { pub (crate) fn lower_pat (& mut self , pattern : & Pat) -> & 'hir hir :: Pat < 'hir > { self . arena . alloc (self . lower_pat_mut (pattern)) } fn lower_pat_mut (& mut self , mut pattern : & Pat) -> hir :: Pat < 'hir > { ensure_sufficient_stack (| | { let pat_hir_id = self . lower_node_id (pattern . id) ; let node = loop { match & pattern . kind { PatKind :: Missing => break hir :: PatKind :: Missing , PatKind :: Wild => break hir :: PatKind :: Wild , PatKind :: Never => break hir :: PatKind :: Never , PatKind :: Ident (binding_mode , ident , sub) => { let lower_sub = | this : & mut Self | sub . as_ref () . map (| s | this . lower_pat (s)) ; break self . lower_pat_ident (pattern , * binding_mode , * ident , pat_hir_id , lower_sub ,) ; } PatKind :: Expr (e) => { break hir :: PatKind :: Expr (self . lower_expr_within_pat (e , false)) ; } PatKind :: TupleStruct (qself , path , pats) => { let qpath = self . lower_qpath (pattern . id , qself , path , ParamMode :: Optional , AllowReturnTypeNotation :: No , ImplTraitContext :: Disallowed (ImplTraitPosition :: Path) , None ,) ; let (pats , ddpos) = self . lower_pat_tuple (pats , "tuple struct") ; break hir :: PatKind :: TupleStruct (qpath , pats , ddpos) ; } PatKind :: Or (pats) => { break hir :: PatKind :: Or (self . arena . alloc_from_iter (pats . iter () . map (| x | self . lower_pat_mut (x))) ,) ; } PatKind :: Path (qself , path) => { let qpath = self . lower_qpath (pattern . id , qself , path , ParamMode :: Optional , AllowReturnTypeNotation :: No , ImplTraitContext :: Disallowed (ImplTraitPosition :: Path) , None ,) ; let kind = hir :: PatExprKind :: Path (qpath) ; let span = self . lower_span (pattern . span) ; let expr = hir :: PatExpr { hir_id : pat_hir_id , span , kind } ; let expr = self . arena . alloc (expr) ; return hir :: Pat { hir_id : self . next_id () , kind : hir :: PatKind :: Expr (expr) , span , default_binding_modes : true , } ; } PatKind :: Struct (qself , path , fields , etc) => { let qpath = self . lower_qpath (pattern . id , qself , path , ParamMode :: Optional , AllowReturnTypeNotation :: No , ImplTraitContext :: Disallowed (ImplTraitPosition :: Path) , None ,) ; let fs = self . arena . alloc_from_iter (fields . iter () . map (| f | { let hir_id = self . lower_node_id (f . id) ; self . lower_attrs (hir_id , & f . attrs , f . span , Target :: PatField) ; hir :: PatField { hir_id , ident : self . lower_ident (f . ident) , pat : self . lower_pat (& f . pat) , is_shorthand : f . is_shorthand , span : self . lower_span (f . span) , } })) ; break hir :: PatKind :: Struct (qpath , fs , match etc { ast :: PatFieldsRest :: Rest (sp) => Some (self . lower_span (* sp)) , ast :: PatFieldsRest :: Recovered (_) => Some (Span :: default ()) , _ => None , } ,) ; } PatKind :: Tuple (pats) => { let (pats , ddpos) = self . lower_pat_tuple (pats , "tuple") ; break hir :: PatKind :: Tuple (pats , ddpos) ; } PatKind :: Box (inner) => { break hir :: PatKind :: Box (self . lower_pat (inner)) ; } PatKind :: Deref (inner) => { break hir :: PatKind :: Deref (self . lower_pat (inner)) ; } PatKind :: Ref (inner , mutbl) => { break hir :: PatKind :: Ref (self . lower_pat (inner) , * mutbl) ; } PatKind :: Range (e1 , e2 , Spanned { node : end , .. }) => { break hir :: PatKind :: Range (e1 . as_deref () . map (| e | self . lower_expr_within_pat (e , true)) , e2 . as_deref () . map (| e | self . lower_expr_within_pat (e , true)) , self . lower_range_end (end , e2 . is_some ()) ,) ; } PatKind :: Guard (inner , cond) => { break hir :: PatKind :: Guard (self . lower_pat (inner) , self . lower_expr (cond)) ; } PatKind :: Slice (pats) => break self . lower_pat_slice (pats) , PatKind :: Rest => { break self . ban_illegal_rest_pat (pattern . span) ; } PatKind :: Paren (inner) => pattern = inner , PatKind :: MacCall (_) => panic ! ("{:?} shouldn't exist here" , pattern . span) , PatKind :: Err (guar) => break hir :: PatKind :: Err (* guar) , } } ; self . pat_with_node_id_of (pattern , node , pat_hir_id) }) } fn lower_pat_tuple (& mut self , pats : & [Box < Pat >] , ctx : & str ,) -> (& 'hir [hir :: Pat < 'hir >] , hir :: DotDotPos) { let mut elems = Vec :: with_capacity (pats . len ()) ; let mut rest = None ; let mut iter = pats . iter () . enumerate () ; for (idx , pat) in iter . by_ref () { match & pat . kind { PatKind :: Rest => { rest = Some ((idx , pat . span)) ; break ; } PatKind :: Ident (_ , ident , Some (sub)) if sub . is_rest () => { let sp = pat . span ; self . dcx () . emit_err (SubTupleBinding { span : sp , ident_name : ident . name , ident : * ident , ctx , }) ; } _ => { } } elems . push (self . lower_pat_mut (pat)) ; } for (_ , pat) in iter { if pat . is_rest () { self . ban_extra_rest_pat (pat . span , rest . unwrap () . 1 , ctx) ; } else { elems . push (self . lower_pat_mut (pat)) ; } } (self . arena . alloc_from_iter (elems) , hir :: DotDotPos :: new (rest . map (| (ddpos , _) | ddpos))) } #[doc = " Lower a slice pattern of form `[pat_0, ..., pat_n]` into"] #[doc = " `hir::PatKind::Slice(before, slice, after)`."] #[doc = ""] #[doc = " When encountering `($binding_mode $ident @)? ..` (`slice`),"] #[doc = " this is interpreted as a sub-slice pattern semantically."] #[doc = " Patterns that follow, which are not like `slice` -- or an error occurs, are in `after`."] fn lower_pat_slice (& mut self , pats : & [Box < Pat >]) -> hir :: PatKind < 'hir > { let mut before = Vec :: new () ; let mut after = Vec :: new () ; let mut slice = None ; let mut prev_rest_span = None ; let lower_rest_sub = | this : & mut Self , pat : & Pat , & ann , & ident , sub : & Pat | { let sub_hir_id = this . lower_node_id (sub . id) ; let lower_sub = | this : & mut Self | Some (this . pat_wild_with_node_id_of (sub , sub_hir_id)) ; let pat_hir_id = this . lower_node_id (pat . id) ; let node = this . lower_pat_ident (pat , ann , ident , pat_hir_id , lower_sub) ; this . pat_with_node_id_of (pat , node , pat_hir_id) } ; let mut iter = pats . iter () ; for pat in iter . by_ref () { match & pat . kind { PatKind :: Rest => { prev_rest_span = Some (pat . span) ; let hir_id = self . lower_node_id (pat . id) ; slice = Some (self . pat_wild_with_node_id_of (pat , hir_id)) ; break ; } PatKind :: Ident (ann , ident , Some (sub)) if sub . is_rest () => { prev_rest_span = Some (sub . span) ; slice = Some (self . arena . alloc (lower_rest_sub (self , pat , ann , ident , sub))) ; break ; } _ => before . push (self . lower_pat_mut (pat)) , } } for pat in iter { let rest_span = match & pat . kind { PatKind :: Rest => Some (pat . span) , PatKind :: Ident (ann , ident , Some (sub)) if sub . is_rest () => { after . push (lower_rest_sub (self , pat , ann , ident , sub)) ; Some (sub . span) } _ => None , } ; if let Some (rest_span) = rest_span { self . ban_extra_rest_pat (rest_span , prev_rest_span . unwrap () , "slice") ; } else { after . push (self . lower_pat_mut (pat)) ; } } hir :: PatKind :: Slice (self . arena . alloc_from_iter (before) , slice , self . arena . alloc_from_iter (after) ,) } fn lower_pat_ident (& mut self , p : & Pat , annotation : BindingMode , ident : Ident , hir_id : hir :: HirId , lower_sub : impl FnOnce (& mut Self) -> Option < & 'hir hir :: Pat < 'hir > > ,) -> hir :: PatKind < 'hir > { match self . resolver . get_partial_res (p . id) . map (| d | d . expect_full_res ()) { res @ (None | Some (Res :: Local (_))) => { let binding_id = match res { Some (Res :: Local (id)) => { if id == p . id { self . ident_and_label_to_local_id . insert (id , hir_id . local_id) ; hir_id } else { hir :: HirId { owner : self . current_hir_id_owner , local_id : self . ident_and_label_to_local_id [& id] , } } } _ => { self . ident_and_label_to_local_id . insert (p . id , hir_id . local_id) ; hir_id } } ; hir :: PatKind :: Binding (annotation , binding_id , self . lower_ident (ident) , lower_sub (self) ,) } Some (res) => { let res = self . lower_res (res) ; let span = self . lower_span (ident . span) ; hir :: PatKind :: Expr (self . arena . alloc (hir :: PatExpr { kind : hir :: PatExprKind :: Path (hir :: QPath :: Resolved (None , self . arena . alloc (hir :: Path { span , res , segments : arena_vec ! [self ; hir :: PathSegment :: new (self . lower_ident (ident) , self . next_id () , res)] , }) ,)) , hir_id : self . next_id () , span , })) } } } fn pat_wild_with_node_id_of (& mut self , p : & Pat , hir_id : hir :: HirId) -> & 'hir hir :: Pat < 'hir > { self . arena . alloc (self . pat_with_node_id_of (p , hir :: PatKind :: Wild , hir_id)) } #[doc = " Construct a `Pat` with the `HirId` of `p.id` already lowered."] fn pat_with_node_id_of (& mut self , p : & Pat , kind : hir :: PatKind < 'hir > , hir_id : hir :: HirId ,) -> hir :: Pat < 'hir > { hir :: Pat { hir_id , kind , span : self . lower_span (p . span) , default_binding_modes : true } } #[doc = " Emit a friendly error for extra `..` patterns in a tuple/tuple struct/slice pattern."] pub (crate) fn ban_extra_rest_pat (& self , sp : Span , prev_sp : Span , ctx : & str) { self . dcx () . emit_err (ExtraDoubleDot { span : sp , prev_span : prev_sp , ctx }) ; } #[doc = " Used to ban the `..` pattern in places it shouldn't be semantically."] fn ban_illegal_rest_pat (& self , sp : Span) -> hir :: PatKind < 'hir > { self . dcx () . emit_err (MisplacedDoubleDot { span : sp }) ; hir :: PatKind :: Wild } fn lower_range_end (& mut self , e : & RangeEnd , has_end : bool) -> hir :: RangeEnd { match * e { RangeEnd :: Excluded if has_end => hir :: RangeEnd :: Excluded , RangeEnd :: Excluded | RangeEnd :: Included (_) => hir :: RangeEnd :: Included , } } #[doc = " Matches `'-' lit | lit (cf. parser::Parser::parse_literal_maybe_minus)`,"] #[doc = " or paths for ranges."] fn lower_expr_within_pat (& mut self , expr : & Expr , allow_paths : bool ,) -> & 'hir hir :: PatExpr < 'hir > { let span = self . lower_span (expr . span) ; let err = | guar | hir :: PatExprKind :: Lit { lit : respan (span , LitKind :: Err (guar)) , negated : false } ; let kind = match & expr . kind { ExprKind :: Lit (lit) => { hir :: PatExprKind :: Lit { lit : self . lower_lit (lit , span) , negated : false } } ExprKind :: ConstBlock (c) => hir :: PatExprKind :: ConstBlock (self . lower_const_block (c)) , ExprKind :: IncludedBytes (byte_sym) => hir :: PatExprKind :: Lit { lit : respan (span , LitKind :: ByteStr (* byte_sym , StrStyle :: Cooked)) , negated : false , } , ExprKind :: Err (guar) => err (* guar) , ExprKind :: Dummy => span_bug ! (span , "lowered ExprKind::Dummy") , ExprKind :: Path (qself , path) if allow_paths => hir :: PatExprKind :: Path (self . lower_qpath (expr . id , qself , path , ParamMode :: Optional , AllowReturnTypeNotation :: No , ImplTraitContext :: Disallowed (ImplTraitPosition :: Path) , None ,)) , ExprKind :: Unary (UnOp :: Neg , inner) if let ExprKind :: Lit (lit) = & inner . kind => { hir :: PatExprKind :: Lit { lit : self . lower_lit (lit , span) , negated : true } } _ => { let pattern_from_macro = expr . is_approximately_pattern () ; let guar = self . dcx () . emit_err (ArbitraryExpressionInPattern { span , pattern_from_macro_note : pattern_from_macro , }) ; err (guar) } } ; self . arena . alloc (hir :: PatExpr { hir_id : self . lower_node_id (expr . id) , span , kind }) } pub (crate) fn lower_ty_pat (& mut self , pattern : & TyPat , base_type : Span ,) -> & 'hir hir :: TyPat < 'hir > { self . arena . alloc (self . lower_ty_pat_mut (pattern , base_type)) } fn lower_ty_pat_mut (& mut self , pattern : & TyPat , base_type : Span) -> hir :: TyPat < 'hir > { let pat_hir_id = self . lower_node_id (pattern . id) ; let node = match & pattern . kind { TyPatKind :: Range (e1 , e2 , Spanned { node : end , span }) => hir :: TyPatKind :: Range (e1 . as_deref () . map (| e | self . lower_anon_const_to_const_arg (e)) . unwrap_or_else (| | { self . lower_ty_pat_range_end (hir :: LangItem :: RangeMin , span . shrink_to_lo () , base_type ,) }) , e2 . as_deref () . map (| e | match end { RangeEnd :: Included (..) => self . lower_anon_const_to_const_arg (e) , RangeEnd :: Excluded => self . lower_excluded_range_end (e) , }) . unwrap_or_else (| | { self . lower_ty_pat_range_end (hir :: LangItem :: RangeMax , span . shrink_to_hi () , base_type ,) }) ,) , TyPatKind :: Or (variants) => { hir :: TyPatKind :: Or (self . arena . alloc_from_iter (variants . iter () . map (| pat | self . lower_ty_pat_mut (pat , base_type)) ,)) } TyPatKind :: Err (guar) => hir :: TyPatKind :: Err (* guar) , } ; hir :: TyPat { hir_id : pat_hir_id , kind : node , span : self . lower_span (pattern . span) } } #[doc = " Lowers the range end of an exclusive range (`2..5`) to an inclusive range 2..=(5 - 1)."] #[doc = " This way the type system doesn't have to handle the distinction between inclusive/exclusive ranges."] fn lower_excluded_range_end (& mut self , e : & AnonConst) -> & 'hir hir :: ConstArg < 'hir > { let span = self . lower_span (e . value . span) ; let unstable_span = self . mark_span_with_reason (DesugaringKind :: PatTyRange , span , Some (Arc :: clone (& self . allow_pattern_type)) ,) ; let anon_const = self . with_new_scopes (span , | this | { let def_id = this . local_def_id (e . id) ; let hir_id = this . lower_node_id (e . id) ; let body = this . lower_body (| this | { let kind = hir :: ExprKind :: Path (this . make_lang_item_qpath (hir :: LangItem :: RangeSub , unstable_span , None ,)) ; let fn_def = this . arena . alloc (hir :: Expr { hir_id : this . next_id () , kind , span }) ; let args = this . arena . alloc ([this . lower_expr_mut (& e . value)]) ; (& [] , hir :: Expr { hir_id : this . next_id () , kind : hir :: ExprKind :: Call (fn_def , args) , span , } ,) }) ; hir :: AnonConst { def_id , hir_id , body , span } }) ; self . arena . alloc (hir :: ConstArg { hir_id : self . next_id () , kind : hir :: ConstArgKind :: Anon (self . arena . alloc (anon_const)) , }) } #[doc = " When a range has no end specified (`1..` or `1..=`) or no start specified (`..5` or `..=5`),"] #[doc = " we instead use a constant of the MAX/MIN of the type."] #[doc = " This way the type system does not have to handle the lack of a start/end."] fn lower_ty_pat_range_end (& mut self , lang_item : LangItem , span : Span , base_type : Span ,) -> & 'hir hir :: ConstArg < 'hir > { let node_id = self . next_node_id () ; let def_id = self . create_def (node_id , None , DefKind :: AnonConst , span) ; let hir_id = self . lower_node_id (node_id) ; let unstable_span = self . mark_span_with_reason (DesugaringKind :: PatTyRange , self . lower_span (span) , Some (Arc :: clone (& self . allow_pattern_type)) ,) ; let span = self . lower_span (base_type) ; let path_expr = hir :: Expr { hir_id : self . next_id () , kind : hir :: ExprKind :: Path (self . make_lang_item_qpath (lang_item , unstable_span , None)) , span , } ; let ct = self . with_new_scopes (span , | this | { self . arena . alloc (hir :: AnonConst { def_id , hir_id , body : this . lower_body (| _this | (& [] , path_expr)) , span , }) }) ; let hir_id = self . next_id () ; self . arena . alloc (hir :: ConstArg { kind : hir :: ConstArgKind :: Anon (ct) , hir_id }) } }}}