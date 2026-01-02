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
mkuse!{use super :: { AdtExpr , AdtExprBase , Arm , Block , ClosureExpr , Expr , ExprKind , InlineAsmExpr , InlineAsmOperand , Pat , PatKind , Stmt , StmtKind , Thir , } ;}
mkuse!{use crate :: thir :: LoopMatchMatchData ;}
mkitem!{mktrait!{# [doc = " Every `walk_*` method uses deconstruction to access fields of structs and"] # [doc = " enums. This will result in a compile error if a field is added, which makes"] # [doc = " it more likely the appropriate visit call will be added for it."] pub trait Visitor < 'thir , 'tcx : 'thir > : Sized { fn thir (& self) -> & 'thir Thir < 'tcx > ; fn visit_expr (& mut self , expr : & 'thir Expr < 'tcx >) { walk_expr (self , expr) ; } fn visit_stmt (& mut self , stmt : & 'thir Stmt < 'tcx >) { walk_stmt (self , stmt) ; } fn visit_block (& mut self , block : & 'thir Block) { walk_block (self , block) ; } fn visit_arm (& mut self , arm : & 'thir Arm < 'tcx >) { walk_arm (self , arm) ; } fn visit_pat (& mut self , pat : & 'thir Pat < 'tcx >) { walk_pat (self , pat) ; } }}}

macro_rules! walk_expr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function walk_expr in module {}", module_path!());
    };
}

mkfn!{
    walk_expr_introspect!();
    pub fn walk_expr < 'thir , 'tcx : 'thir , V : Visitor < 'thir , 'tcx > > (visitor : & mut V , expr : & 'thir Expr < 'tcx > ,) { use ExprKind :: * ; let Expr { kind , ty : _ , temp_lifetime : _ , span : _ } = expr ; match * kind { Scope { value , region_scope : _ , lint_level : _ } => { visitor . visit_expr (& visitor . thir () [value]) } Box { value } => visitor . visit_expr (& visitor . thir () [value]) , If { cond , then , else_opt , if_then_scope : _ } => { visitor . visit_expr (& visitor . thir () [cond]) ; visitor . visit_expr (& visitor . thir () [then]) ; if let Some (else_expr) = else_opt { visitor . visit_expr (& visitor . thir () [else_expr]) ; } } Call { fun , ref args , ty : _ , from_hir_call : _ , fn_span : _ } => { visitor . visit_expr (& visitor . thir () [fun]) ; for & arg in & * * args { visitor . visit_expr (& visitor . thir () [arg]) ; } } ByUse { expr , span : _ } => { visitor . visit_expr (& visitor . thir () [expr]) ; } Deref { arg } => visitor . visit_expr (& visitor . thir () [arg]) , Binary { lhs , rhs , op : _ } | LogicalOp { lhs , rhs , op : _ } => { visitor . visit_expr (& visitor . thir () [lhs]) ; visitor . visit_expr (& visitor . thir () [rhs]) ; } Unary { arg , op : _ } => visitor . visit_expr (& visitor . thir () [arg]) , Cast { source } => visitor . visit_expr (& visitor . thir () [source]) , Use { source } => visitor . visit_expr (& visitor . thir () [source]) , NeverToAny { source } => visitor . visit_expr (& visitor . thir () [source]) , PointerCoercion { source , cast : _ , is_from_as_cast : _ } => { visitor . visit_expr (& visitor . thir () [source]) } Let { expr , ref pat } => { visitor . visit_expr (& visitor . thir () [expr]) ; visitor . visit_pat (pat) ; } Loop { body } => visitor . visit_expr (& visitor . thir () [body]) , LoopMatch { match_data : box LoopMatchMatchData { scrutinee , ref arms , .. } , .. } | Match { scrutinee , ref arms , .. } => { visitor . visit_expr (& visitor . thir () [scrutinee]) ; for & arm in & * * arms { visitor . visit_arm (& visitor . thir () [arm]) ; } } Block { block } => visitor . visit_block (& visitor . thir () [block]) , Assign { lhs , rhs } | AssignOp { lhs , rhs , op : _ } => { visitor . visit_expr (& visitor . thir () [lhs]) ; visitor . visit_expr (& visitor . thir () [rhs]) ; } Field { lhs , variant_index : _ , name : _ } => visitor . visit_expr (& visitor . thir () [lhs]) , Index { lhs , index } => { visitor . visit_expr (& visitor . thir () [lhs]) ; visitor . visit_expr (& visitor . thir () [index]) ; } VarRef { id : _ } | UpvarRef { closure_def_id : _ , var_hir_id : _ } => { } Borrow { arg , borrow_kind : _ } => visitor . visit_expr (& visitor . thir () [arg]) , RawBorrow { arg , mutability : _ } => visitor . visit_expr (& visitor . thir () [arg]) , Break { value , label : _ } => { if let Some (value) = value { visitor . visit_expr (& visitor . thir () [value]) } } Continue { label : _ } => { } ConstContinue { value , label : _ } => visitor . visit_expr (& visitor . thir () [value]) , Return { value } => { if let Some (value) = value { visitor . visit_expr (& visitor . thir () [value]) } } Become { value } => visitor . visit_expr (& visitor . thir () [value]) , ConstBlock { did : _ , args : _ } => { } Repeat { value , count : _ } => { visitor . visit_expr (& visitor . thir () [value]) ; } Array { ref fields } | Tuple { ref fields } => { for & field in & * * fields { visitor . visit_expr (& visitor . thir () [field]) ; } } Adt (box AdtExpr { ref fields , ref base , adt_def : _ , variant_index : _ , args : _ , user_ty : _ , }) => { for field in & * * fields { visitor . visit_expr (& visitor . thir () [field . expr]) ; } if let AdtExprBase :: Base (base) = base { visitor . visit_expr (& visitor . thir () [base . base]) ; } } PlaceTypeAscription { source , user_ty : _ , user_ty_span : _ } | ValueTypeAscription { source , user_ty : _ , user_ty_span : _ } => { visitor . visit_expr (& visitor . thir () [source]) } PlaceUnwrapUnsafeBinder { source } | ValueUnwrapUnsafeBinder { source } | WrapUnsafeBinder { source } => visitor . visit_expr (& visitor . thir () [source]) , Closure (box ClosureExpr { closure_id : _ , args : _ , upvars : _ , movability : _ , fake_reads : _ , }) => { } Literal { lit : _ , neg : _ } => { } NonHirLiteral { lit : _ , user_ty : _ } => { } ZstLiteral { user_ty : _ } => { } NamedConst { def_id : _ , args : _ , user_ty : _ } => { } ConstParam { param : _ , def_id : _ } => { } StaticRef { alloc_id : _ , ty : _ , def_id : _ } => { } InlineAsm (box InlineAsmExpr { asm_macro : _ , ref operands , template : _ , options : _ , line_spans : _ , }) => { for op in & * * operands { use InlineAsmOperand :: * ; match op { In { expr , reg : _ } | Out { expr : Some (expr) , reg : _ , late : _ } | InOut { expr , reg : _ , late : _ } => visitor . visit_expr (& visitor . thir () [* expr]) , SplitInOut { in_expr , out_expr , reg : _ , late : _ } => { visitor . visit_expr (& visitor . thir () [* in_expr]) ; if let Some (out_expr) = out_expr { visitor . visit_expr (& visitor . thir () [* out_expr]) ; } } Out { expr : None , reg : _ , late : _ } | Const { value : _ , span : _ } | SymFn { value : _ } | SymStatic { def_id : _ } => { } Label { block } => visitor . visit_block (& visitor . thir () [* block]) , } } } OffsetOf { container : _ , fields : _ } => { } ThreadLocalRef (_) => { } Yield { value } => visitor . visit_expr (& visitor . thir () [value]) , } }
}

macro_rules! walk_stmt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function walk_stmt in module {}", module_path!());
    };
}

mkfn!{
    walk_stmt_introspect!();
    pub fn walk_stmt < 'thir , 'tcx : 'thir , V : Visitor < 'thir , 'tcx > > (visitor : & mut V , stmt : & 'thir Stmt < 'tcx > ,) { let Stmt { kind } = stmt ; match kind { StmtKind :: Expr { expr , scope : _ } => visitor . visit_expr (& visitor . thir () [* expr]) , StmtKind :: Let { initializer , remainder_scope : _ , init_scope : _ , pattern , lint_level : _ , else_block , span : _ , } => { if let Some (init) = initializer { visitor . visit_expr (& visitor . thir () [* init]) ; } visitor . visit_pat (pattern) ; if let Some (block) = else_block { visitor . visit_block (& visitor . thir () [* block]) } } } }
}

macro_rules! walk_block_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function walk_block in module {}", module_path!());
    };
}

mkfn!{
    walk_block_introspect!();
    pub fn walk_block < 'thir , 'tcx : 'thir , V : Visitor < 'thir , 'tcx > > (visitor : & mut V , block : & 'thir Block ,) { let Block { stmts , expr , targeted_by_break : _ , region_scope : _ , span : _ , safety_mode : _ } = block ; for & stmt in & * stmts { visitor . visit_stmt (& visitor . thir () [stmt]) ; } if let Some (expr) = expr { visitor . visit_expr (& visitor . thir () [* expr]) ; } }
}

macro_rules! walk_arm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function walk_arm in module {}", module_path!());
    };
}

mkfn!{
    walk_arm_introspect!();
    pub fn walk_arm < 'thir , 'tcx : 'thir , V : Visitor < 'thir , 'tcx > > (visitor : & mut V , arm : & 'thir Arm < 'tcx > ,) { let Arm { guard , pattern , body , lint_level : _ , span : _ , scope : _ } = arm ; if let Some (expr) = guard { visitor . visit_expr (& visitor . thir () [* expr]) } visitor . visit_pat (pattern) ; visitor . visit_expr (& visitor . thir () [* body]) ; }
}

macro_rules! walk_pat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function walk_pat in module {}", module_path!());
    };
}

mkfn!{
    walk_pat_introspect!();
    pub fn walk_pat < 'thir , 'tcx : 'thir , V : Visitor < 'thir , 'tcx > > (visitor : & mut V , pat : & 'thir Pat < 'tcx > ,) { for_each_immediate_subpat (pat , | p | visitor . visit_pat (p)) ; }
}

macro_rules! for_each_immediate_subpat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function for_each_immediate_subpat in module {}", module_path!());
    };
}

mkfn!{
    for_each_immediate_subpat_introspect!();
    # [doc = " Invokes `callback` on each immediate subpattern of `pat`, if any."] # [doc = " A building block for assembling THIR pattern visitors."] pub (crate) fn for_each_immediate_subpat < 'a , 'tcx > (pat : & 'a Pat < 'tcx > , mut callback : impl FnMut (& 'a Pat < 'tcx >) ,) { let Pat { kind , ty : _ , span : _ } = pat ; match kind { PatKind :: Missing | PatKind :: Wild | PatKind :: Binding { subpattern : None , .. } | PatKind :: Constant { value : _ } | PatKind :: Range (_) | PatKind :: Never | PatKind :: Error (_) => { } PatKind :: AscribeUserType { subpattern , .. } | PatKind :: Binding { subpattern : Some (subpattern) , .. } | PatKind :: Deref { subpattern } | PatKind :: DerefPattern { subpattern , .. } | PatKind :: ExpandedConstant { subpattern , .. } => callback (subpattern) , PatKind :: Variant { subpatterns , .. } | PatKind :: Leaf { subpatterns } => { for field_pat in subpatterns { callback (& field_pat . pattern) ; } } PatKind :: Slice { prefix , slice , suffix } | PatKind :: Array { prefix , slice , suffix } => { for pat in prefix . iter () . chain (slice . as_deref ()) . chain (suffix) { callback (pat) ; } } PatKind :: Or { pats } => { for pat in pats { callback (pat) ; } } } }
}