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
mkuse!{use rustc_index :: IndexSlice ;}
mkuse!{use rustc_middle :: mir :: * ;}
mkuse!{use rustc_middle :: thir :: * ;}
mkuse!{use rustc_middle :: ty :: { self , Ty } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use super :: { PResult , ParseCtxt , ParseError } ;}
mkmod!{instruction, { 
                getname!(instruction);
                getsrc!(instruction);
                getpath!(instruction);
                get_deps!(instruction);
                get_crates!(instruction);
                mkinclude!(instruction);
                 
            }}
mkitem!{#[doc = " Helper macro for parsing custom MIR."] #[doc = ""] #[doc = " Example usage looks something like:"] #[doc = " ```rust,ignore (incomplete example)"] #[doc = " parse_by_kind!("] #[doc = "     self, // : &ParseCtxt"] #[doc = "     expr_id, // what you're matching against"] #[doc = "     \"assignment\", // the thing you're trying to parse"] #[doc = "     @call(\"mir_assign\", args) => { args[0] }, // match invocations of the `mir_assign` special function"] #[doc = "     ExprKind::Assign { lhs, .. } => { lhs }, // match thir assignment expressions"] #[doc = "     // no need for fallthrough case - reasonable error is automatically generated"] #[doc = " )"] #[doc = " ```"] macro_rules ! parse_by_kind { ($ self : ident , $ expr_id : expr , $ expr_name : pat , $ expected : literal , $ (@ call ($ name : ident , $ args : ident) => $ call_expr : expr ,) * $ (@ variant ($ adt : ident , $ variant : ident) => $ variant_expr : expr ,) * $ ($ pat : pat $ (if $ guard : expr) ? => $ expr : expr ,) *) => { { let expr_id = $ self . preparse ($ expr_id) ; let expr = &$ self . thir [expr_id] ; tracing :: debug ! ("Trying to parse {:?} as {}" , expr . kind , $ expected) ; let $ expr_name = expr ; match & expr . kind { $ (ExprKind :: Call { ty , fun : _ , args : $ args , .. } if { match ty . kind () { ty :: FnDef (did , _) => { $ self . tcx . is_diagnostic_item (rustc_span :: sym ::$ name , * did) } _ => false , } } => $ call_expr ,) * $ (ExprKind :: Adt (box AdtExpr { adt_def , variant_index , .. }) if { $ self . tcx . is_diagnostic_item (rustc_span :: sym ::$ adt , adt_def . did ()) && adt_def . variants () [* variant_index] . name == rustc_span :: sym ::$ variant } => $ variant_expr ,) * $ ($ pat $ (if $ guard) ? => $ expr ,) * #[allow (unreachable_patterns)] _ => return Err ($ self . expr_error (expr_id , $ expected)) } } } ; }}
mkuse!{pub (crate) use parse_by_kind ;}
mkitem!{mkimpl!{impl < 'a , 'tcx > ParseCtxt < 'a , 'tcx > { #[doc = " Expressions should only ever be matched on after preparsing them. This removes extra scopes"] #[doc = " we don't care about."] fn preparse (& self , expr_id : ExprId) -> ExprId { let expr = & self . thir [expr_id] ; match expr . kind { ExprKind :: Scope { value , .. } => self . preparse (value) , _ => expr_id , } } fn statement_as_expr (& self , stmt_id : StmtId) -> PResult < ExprId > { match & self . thir [stmt_id] . kind { StmtKind :: Expr { expr , .. } => Ok (* expr) , kind @ StmtKind :: Let { pattern , .. } => Err (ParseError { span : pattern . span , item_description : format ! ("{kind:?}") , expected : "expression" . to_string () , }) , } } pub (crate) fn parse_args (& mut self , params : & IndexSlice < ParamId , Param < 'tcx > >) -> PResult < () > { for param in params . iter () { let (var , span) = { let pat = param . pat . as_ref () . unwrap () ; match & pat . kind { PatKind :: Binding { var , .. } => (* var , pat . span) , _ => { return Err (ParseError { span : pat . span , item_description : format ! ("{:?}" , pat . kind) , expected : "local" . to_string () , }) ; } } } ; let decl = LocalDecl :: new (param . ty , span) ; let local = self . body . local_decls . push (decl) ; self . local_map . insert (var , local) ; } Ok (()) } #[doc = " Bodies are of the form:"] #[doc = ""] #[doc = " ```text"] #[doc = " {"] #[doc = "     let bb1: BasicBlock;"] #[doc = "     let bb2: BasicBlock;"] #[doc = "     {"] #[doc = "         let RET: _;"] #[doc = "         let local1;"] #[doc = "         let local2;"] #[doc = ""] #[doc = "         {"] #[doc = "             { // entry block"] #[doc = "                 statement1;"] #[doc = "                 terminator1"] #[doc = "             };"] #[doc = ""] #[doc = "             bb1 = {"] #[doc = "                 statement2;"] #[doc = "                 terminator2"] #[doc = "             };"] #[doc = ""] #[doc = "             bb2 = {"] #[doc = "                 statement3;"] #[doc = "                 terminator3"] #[doc = "             }"] #[doc = ""] #[doc = "             RET"] #[doc = "         }"] #[doc = "     }"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " This allows us to easily parse the basic blocks declarations, local declarations, and"] #[doc = " basic block definitions in order."] pub (crate) fn parse_body (& mut self , expr_id : ExprId) -> PResult < () > { let body = parse_by_kind ! (self , expr_id , _ , "whole body" , ExprKind :: Block { block } => self . thir [* block] . expr . unwrap () ,) ; let (block_decls , rest) = parse_by_kind ! (self , body , _ , "body with block decls" , ExprKind :: Block { block } => { let block = & self . thir [* block] ; (& block . stmts , block . expr . unwrap ()) } ,) ; self . parse_block_decls (block_decls . iter () . copied ()) ? ; let (local_decls , rest) = parse_by_kind ! (self , rest , _ , "body with local decls" , ExprKind :: Block { block } => { let block = & self . thir [* block] ; (& block . stmts , block . expr . unwrap ()) } ,) ; self . parse_local_decls (local_decls . iter () . copied ()) ? ; let (debuginfo , rest) = parse_by_kind ! (self , rest , _ , "body with debuginfo" , ExprKind :: Block { block } => { let block = & self . thir [* block] ; (& block . stmts , block . expr . unwrap ()) } ,) ; self . parse_debuginfo (debuginfo . iter () . copied ()) ? ; let block_defs = parse_by_kind ! (self , rest , _ , "body with block defs" , ExprKind :: Block { block } => & self . thir [* block] . stmts ,) ; for (i , block_def) in block_defs . iter () . enumerate () { let is_cleanup = self . body . basic_blocks_mut () [BasicBlock :: from_usize (i)] . is_cleanup ; let block = self . parse_block_def (self . statement_as_expr (* block_def) ? , is_cleanup) ? ; self . body . basic_blocks_mut () [BasicBlock :: from_usize (i)] = block ; } Ok (()) } fn parse_block_decls (& mut self , stmts : impl Iterator < Item = StmtId >) -> PResult < () > { for stmt in stmts { self . parse_basic_block_decl (stmt) ? ; } Ok (()) } fn parse_basic_block_decl (& mut self , stmt : StmtId) -> PResult < () > { match & self . thir [stmt] . kind { StmtKind :: Let { pattern , initializer : Some (initializer) , .. } => { let (var , ..) = self . parse_var (pattern) ? ; let data = BasicBlockData :: new (None , parse_by_kind ! (self , * initializer , _ , "basic block declaration" , @ variant (mir_basic_block , Normal) => false , @ variant (mir_basic_block , Cleanup) => true ,) ,) ; let block = self . body . basic_blocks_mut () . push (data) ; self . block_map . insert (var , block) ; Ok (()) } _ => Err (self . stmt_error (stmt , "let statement with an initializer")) , } } fn parse_local_decls (& mut self , mut stmts : impl Iterator < Item = StmtId >) -> PResult < () > { let (ret_var , ..) = self . parse_let_statement (stmts . next () . unwrap ()) ? ; self . local_map . insert (ret_var , Local :: ZERO) ; for stmt in stmts { let (var , ty , span) = self . parse_let_statement (stmt) ? ; let decl = LocalDecl :: new (ty , span) ; let local = self . body . local_decls . push (decl) ; self . local_map . insert (var , local) ; } Ok (()) } fn parse_debuginfo (& mut self , stmts : impl Iterator < Item = StmtId >) -> PResult < () > { for stmt in stmts { let stmt = & self . thir [stmt] ; let expr = match stmt . kind { StmtKind :: Let { span , .. } => { return Err (ParseError { span , item_description : format ! ("{:?}" , stmt) , expected : "debuginfo" . to_string () , }) ; } StmtKind :: Expr { expr , .. } => expr , } ; let span = self . thir [expr] . span ; let (name , operand) = parse_by_kind ! (self , expr , _ , "debuginfo" , @ call (mir_debuginfo , args) => { (args [0] , args [1]) } ,) ; let name = parse_by_kind ! (self , name , _ , "debuginfo" , ExprKind :: Literal { lit , neg : false } => lit ,) ; let Some (name) = name . node . str () else { return Err (ParseError { span , item_description : format ! ("{:?}" , name) , expected : "string" . to_string () , }) ; } ; let operand = self . parse_operand (operand) ? ; let value = match operand { Operand :: Constant (c) => VarDebugInfoContents :: Const (* c) , Operand :: Copy (p) | Operand :: Move (p) => VarDebugInfoContents :: Place (p) , } ; let dbginfo = VarDebugInfo { name , source_info : SourceInfo { span , scope : self . source_scope } , composite : None , argument_index : None , value , } ; self . body . var_debug_info . push (dbginfo) ; } Ok (()) } fn parse_let_statement (& mut self , stmt_id : StmtId) -> PResult < (LocalVarId , Ty < 'tcx > , Span) > { let pattern = match & self . thir [stmt_id] . kind { StmtKind :: Let { pattern , .. } => pattern , StmtKind :: Expr { expr , .. } => { return Err (self . expr_error (* expr , "let statement")) ; } } ; self . parse_var (pattern) } fn parse_var (& mut self , mut pat : & Pat < 'tcx >) -> PResult < (LocalVarId , Ty < 'tcx > , Span) > { loop { match & pat . kind { PatKind :: Binding { var , ty , .. } => break Ok ((* var , * ty , pat . span)) , PatKind :: AscribeUserType { subpattern , .. } => { pat = subpattern ; } _ => { break Err (ParseError { span : pat . span , item_description : format ! ("{:?}" , pat . kind) , expected : "local" . to_string () , }) ; } } } } fn parse_block_def (& self , expr_id : ExprId , is_cleanup : bool) -> PResult < BasicBlockData < 'tcx > > { let block = parse_by_kind ! (self , expr_id , _ , "basic block" , ExprKind :: Block { block } => & self . thir [* block] ,) ; let mut data = BasicBlockData :: new (None , is_cleanup) ; for stmt_id in & * block . stmts { let stmt = self . statement_as_expr (* stmt_id) ? ; let span = self . thir [stmt] . span ; let statement = self . parse_statement (stmt) ? ; data . statements . push (Statement :: new (SourceInfo { span , scope : self . source_scope } , statement)) ; } let Some (trailing) = block . expr else { return Err (self . expr_error (expr_id , "terminator")) } ; let span = self . thir [trailing] . span ; let terminator = self . parse_terminator (trailing) ? ; data . terminator = Some (Terminator { source_info : SourceInfo { span , scope : self . source_scope } , kind : terminator , }) ; Ok (data) } }}}