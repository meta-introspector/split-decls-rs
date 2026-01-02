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
mkuse!{use std :: borrow :: Cow ;}
mkuse!{use std :: mem ;}
mkuse!{use std :: ops :: Bound ;}
mkuse!{use rustc_ast :: AsmMacro ;}
mkuse!{use rustc_data_structures :: stack :: ensure_sufficient_stack ;}
mkuse!{use rustc_errors :: DiagArgValue ;}
mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: { self as hir , BindingMode , ByRef , HirId , Mutability , find_attr } ;}
mkuse!{use rustc_middle :: middle :: codegen_fn_attrs :: { TargetFeature , TargetFeatureKind } ;}
mkuse!{use rustc_middle :: mir :: BorrowKind ;}
mkuse!{use rustc_middle :: span_bug ;}
mkuse!{use rustc_middle :: thir :: visit :: Visitor ;}
mkuse!{use rustc_middle :: thir :: * ;}
mkuse!{use rustc_middle :: ty :: print :: with_no_trimmed_paths ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt } ;}
mkuse!{use rustc_session :: lint :: Level ;}
mkuse!{use rustc_session :: lint :: builtin :: { DEPRECATED_SAFE_2024 , UNSAFE_OP_IN_UNSAFE_FN , UNUSED_UNSAFE } ;}
mkuse!{use rustc_span :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_span :: { Span , Symbol , sym } ;}
mkuse!{use crate :: builder :: ExprCategory ;}
mkuse!{use crate :: errors :: * ;}
mkitem!{mkstruct!{struct UnsafetyVisitor < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , thir : & 'a Thir < 'tcx > , #[doc = " The `HirId` of the current scope, which would be the `HirId`"] #[doc = " of the current HIR node, modulo adjustments. Used for lint levels."] hir_context : HirId , #[doc = " The current \"safety context\". This notably tracks whether we are in an"] #[doc = " `unsafe` block, and whether it has been used."] safety_context : SafetyContext , #[doc = " The `#[target_feature]` attributes of the body. Used for checking"] #[doc = " calls to functions with `#[target_feature]` (RFC 2396)."] body_target_features : & 'tcx [TargetFeature] , #[doc = " When inside the LHS of an assignment to a field, this is the type"] #[doc = " of the LHS and the span of the assignment expression."] assignment_info : Option < Ty < 'tcx > > , in_union_destructure : bool , typing_env : ty :: TypingEnv < 'tcx > , inside_adt : bool , warnings : & 'a mut Vec < UnusedUnsafeWarning > , #[doc = " Flag to ensure that we only suggest wrapping the entire function body in"] #[doc = " an unsafe block once."] suggest_unsafe_block : bool , }}}
mkitem!{mkimpl!{impl < 'tcx > UnsafetyVisitor < '_ , 'tcx > { fn in_safety_context (& mut self , safety_context : SafetyContext , f : impl FnOnce (& mut Self)) { let prev_context = mem :: replace (& mut self . safety_context , safety_context) ; f (self) ; let safety_context = mem :: replace (& mut self . safety_context , prev_context) ; if let SafetyContext :: UnsafeBlock { used , span , hir_id , nested_used_blocks } = safety_context { if ! used { self . warn_unused_unsafe (hir_id , span , None) ; if let SafetyContext :: UnsafeBlock { nested_used_blocks : ref mut prev_nested_used_blocks , .. } = self . safety_context { prev_nested_used_blocks . extend (nested_used_blocks) ; } } else { for block in nested_used_blocks { self . warn_unused_unsafe (block . hir_id , block . span , Some (UnusedUnsafeEnclosing :: Block { span : self . tcx . sess . source_map () . guess_head_span (span) , }) ,) ; } match self . safety_context { SafetyContext :: UnsafeBlock { nested_used_blocks : ref mut prev_nested_used_blocks , .. } => { prev_nested_used_blocks . push (NestedUsedBlock { hir_id , span }) ; } _ => () , } } } } fn emit_deprecated_safe_fn_call (& self , span : Span , kind : & UnsafeOpKind) -> bool { match kind { & UnsafeOpKind :: CallToUnsafeFunction (Some (id)) if ! span . at_least_rust_2024 () && let Some (attr) = self . tcx . get_attr (id , sym :: rustc_deprecated_safe_2024) => { let suggestion = attr . meta_item_list () . unwrap_or_default () . into_iter () . find (| item | item . has_name (sym :: audit_that)) . map (| item | { item . value_str () . expect ("`#[rustc_deprecated_safe_2024(audit_that)]` must have a string value" ,) }) ; let sm = self . tcx . sess . source_map () ; let guarantee = suggestion . as_ref () . map (| suggestion | format ! ("that {}" , suggestion)) . unwrap_or_else (| | String :: from ("its unsafe preconditions")) ; let suggestion = suggestion . and_then (| suggestion | { sm . indentation_before (span) . map (| indent | { format ! ("{}// TODO: Audit that {}.\n" , indent , suggestion) }) }) . unwrap_or_default () ; self . tcx . emit_node_span_lint (DEPRECATED_SAFE_2024 , self . hir_context , span , CallToDeprecatedSafeFnRequiresUnsafe { span , function : with_no_trimmed_paths ! (self . tcx . def_path_str (id)) , guarantee , sub : CallToDeprecatedSafeFnRequiresUnsafeSub { start_of_line_suggestion : suggestion , start_of_line : sm . span_extend_to_line (span) . shrink_to_lo () , left : span . shrink_to_lo () , right : span . shrink_to_hi () , } , } ,) ; true } _ => false , } } fn requires_unsafe (& mut self , span : Span , kind : UnsafeOpKind) { let unsafe_op_in_unsafe_fn_allowed = self . unsafe_op_in_unsafe_fn_allowed () ; match self . safety_context { SafetyContext :: BuiltinUnsafeBlock => { } SafetyContext :: UnsafeBlock { ref mut used , .. } => { * used = true ; } SafetyContext :: UnsafeFn if unsafe_op_in_unsafe_fn_allowed => { } SafetyContext :: UnsafeFn => { let deprecated_safe_fn = self . emit_deprecated_safe_fn_call (span , & kind) ; if ! deprecated_safe_fn { kind . emit_unsafe_op_in_unsafe_fn_lint (self . tcx , self . hir_context , span , self . suggest_unsafe_block ,) ; self . suggest_unsafe_block = false ; } } SafetyContext :: Safe => { let deprecated_safe_fn = self . emit_deprecated_safe_fn_call (span , & kind) ; if ! deprecated_safe_fn { kind . emit_requires_unsafe_err (self . tcx , span , self . hir_context , unsafe_op_in_unsafe_fn_allowed ,) ; } } } } fn warn_unused_unsafe (& mut self , hir_id : HirId , block_span : Span , enclosing_unsafe : Option < UnusedUnsafeEnclosing > ,) { self . warnings . push (UnusedUnsafeWarning { hir_id , block_span , enclosing_unsafe }) ; } #[doc = " Whether the `unsafe_op_in_unsafe_fn` lint is `allow`ed at the current HIR node."] fn unsafe_op_in_unsafe_fn_allowed (& self) -> bool { self . tcx . lint_level_at_node (UNSAFE_OP_IN_UNSAFE_FN , self . hir_context) . level == Level :: Allow } #[doc = " Handle closures/coroutines/inline-consts, which is unsafecked with their parent body."] fn visit_inner_body (& mut self , def : LocalDefId) { if let Ok ((inner_thir , expr)) = self . tcx . thir_body (def) { self . tcx . ensure_done () . mir_built (def) ; let inner_thir = if self . tcx . sess . opts . unstable_opts . no_steal_thir { & inner_thir . borrow () } else { & inner_thir . steal () } ; let hir_context = self . tcx . local_def_id_to_hir_id (def) ; let safety_context = mem :: replace (& mut self . safety_context , SafetyContext :: Safe) ; let mut inner_visitor = UnsafetyVisitor { tcx : self . tcx , thir : inner_thir , hir_context , safety_context , body_target_features : self . body_target_features , assignment_info : self . assignment_info , in_union_destructure : false , typing_env : self . typing_env , inside_adt : false , warnings : self . warnings , suggest_unsafe_block : self . suggest_unsafe_block , } ; for param in & inner_thir . params { if let Some (param_pat) = param . pat . as_deref () { inner_visitor . visit_pat (param_pat) ; } } inner_visitor . visit_expr (& inner_thir [expr]) ; self . safety_context = inner_visitor . safety_context ; } } }}}
mkitem!{mkstruct!{struct LayoutConstrainedPlaceVisitor < 'a , 'tcx > { found : bool , thir : & 'a Thir < 'tcx > , tcx : TyCtxt < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > LayoutConstrainedPlaceVisitor < 'a , 'tcx > { fn new (thir : & 'a Thir < 'tcx > , tcx : TyCtxt < 'tcx >) -> Self { Self { found : false , thir , tcx } } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Visitor < 'a , 'tcx > for LayoutConstrainedPlaceVisitor < 'a , 'tcx > { fn thir (& self) -> & 'a Thir < 'tcx > { self . thir } fn visit_expr (& mut self , expr : & 'a Expr < 'tcx >) { match expr . kind { ExprKind :: Field { lhs , .. } => { if let ty :: Adt (adt_def , _) = self . thir [lhs] . ty . kind () { if (Bound :: Unbounded , Bound :: Unbounded) != self . tcx . layout_scalar_valid_range (adt_def . did ()) { self . found = true ; } } visit :: walk_expr (self , expr) ; } ExprKind :: Deref { .. } => { } ref kind if ExprCategory :: of (kind) . is_none_or (| cat | cat == ExprCategory :: Place) => { visit :: walk_expr (self , expr) ; } _ => { } } } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Visitor < 'a , 'tcx > for UnsafetyVisitor < 'a , 'tcx > { fn thir (& self) -> & 'a Thir < 'tcx > { self . thir } fn visit_block (& mut self , block : & 'a Block) { match block . safety_mode { BlockSafety :: BuiltinUnsafe => { self . in_safety_context (SafetyContext :: BuiltinUnsafeBlock , | this | { visit :: walk_block (this , block) }) ; } BlockSafety :: ExplicitUnsafe (hir_id) => { let used = matches ! (self . tcx . lint_level_at_node (UNUSED_UNSAFE , hir_id) . level , Level :: Allow) ; self . in_safety_context (SafetyContext :: UnsafeBlock { span : block . span , hir_id , used , nested_used_blocks : Vec :: new () , } , | this | visit :: walk_block (this , block) ,) ; } BlockSafety :: Safe => { visit :: walk_block (self , block) ; } } } fn visit_pat (& mut self , pat : & 'a Pat < 'tcx >) { if self . in_union_destructure { match pat . kind { PatKind :: Missing => unreachable ! () , PatKind :: Binding { .. } | PatKind :: Constant { .. } | PatKind :: Variant { .. } | PatKind :: Leaf { .. } | PatKind :: Deref { .. } | PatKind :: DerefPattern { .. } | PatKind :: Range { .. } | PatKind :: Slice { .. } | PatKind :: Array { .. } | PatKind :: Never => { self . requires_unsafe (pat . span , AccessToUnionField) ; return ; } PatKind :: Wild | PatKind :: Or { .. } | PatKind :: ExpandedConstant { .. } | PatKind :: AscribeUserType { .. } | PatKind :: Error (_) => { } } } ; match & pat . kind { PatKind :: Leaf { subpatterns , .. } => { if let ty :: Adt (adt_def , ..) = pat . ty . kind () { for pat in subpatterns { if adt_def . non_enum_variant () . fields [pat . field] . safety . is_unsafe () { self . requires_unsafe (pat . pattern . span , UseOfUnsafeField) ; } } if adt_def . is_union () { let old_in_union_destructure = std :: mem :: replace (& mut self . in_union_destructure , true) ; visit :: walk_pat (self , pat) ; self . in_union_destructure = old_in_union_destructure ; } else if (Bound :: Unbounded , Bound :: Unbounded) != self . tcx . layout_scalar_valid_range (adt_def . did ()) { let old_inside_adt = std :: mem :: replace (& mut self . inside_adt , true) ; visit :: walk_pat (self , pat) ; self . inside_adt = old_inside_adt ; } else { visit :: walk_pat (self , pat) ; } } else { visit :: walk_pat (self , pat) ; } } PatKind :: Variant { adt_def , args : _ , variant_index , subpatterns } => { for pat in subpatterns { let field = & pat . field ; if adt_def . variant (* variant_index) . fields [* field] . safety . is_unsafe () { self . requires_unsafe (pat . pattern . span , UseOfUnsafeField) ; } } visit :: walk_pat (self , pat) ; } PatKind :: Binding { mode : BindingMode (ByRef :: Yes (rm) , _) , ty , .. } => { if self . inside_adt { let ty :: Ref (_ , ty , _) = ty . kind () else { span_bug ! (pat . span , "ByRef::Yes in pattern, but found non-reference type {}" , ty) ; } ; match rm { Mutability :: Not => { if ! ty . is_freeze (self . tcx , self . typing_env) { self . requires_unsafe (pat . span , BorrowOfLayoutConstrainedField) ; } } Mutability :: Mut { .. } => { self . requires_unsafe (pat . span , MutationOfLayoutConstrainedField) ; } } } visit :: walk_pat (self , pat) ; } PatKind :: Deref { .. } | PatKind :: DerefPattern { .. } => { let old_inside_adt = std :: mem :: replace (& mut self . inside_adt , false) ; visit :: walk_pat (self , pat) ; self . inside_adt = old_inside_adt ; } PatKind :: ExpandedConstant { def_id , .. } => { if let Some (def) = def_id . as_local () && matches ! (self . tcx . def_kind (def_id) , DefKind :: InlineConst) { self . visit_inner_body (def) ; } visit :: walk_pat (self , pat) ; } _ => { visit :: walk_pat (self , pat) ; } } } fn visit_expr (& mut self , expr : & 'a Expr < 'tcx >) { match expr . kind { ExprKind :: Field { .. } | ExprKind :: VarRef { .. } | ExprKind :: UpvarRef { .. } | ExprKind :: Scope { .. } | ExprKind :: Cast { .. } => { } ExprKind :: RawBorrow { .. } | ExprKind :: Adt { .. } | ExprKind :: Array { .. } | ExprKind :: Binary { .. } | ExprKind :: Block { .. } | ExprKind :: Borrow { .. } | ExprKind :: Literal { .. } | ExprKind :: NamedConst { .. } | ExprKind :: NonHirLiteral { .. } | ExprKind :: ZstLiteral { .. } | ExprKind :: ConstParam { .. } | ExprKind :: ConstBlock { .. } | ExprKind :: Deref { .. } | ExprKind :: Index { .. } | ExprKind :: NeverToAny { .. } | ExprKind :: PlaceTypeAscription { .. } | ExprKind :: ValueTypeAscription { .. } | ExprKind :: PlaceUnwrapUnsafeBinder { .. } | ExprKind :: ValueUnwrapUnsafeBinder { .. } | ExprKind :: WrapUnsafeBinder { .. } | ExprKind :: PointerCoercion { .. } | ExprKind :: Repeat { .. } | ExprKind :: StaticRef { .. } | ExprKind :: ThreadLocalRef { .. } | ExprKind :: Tuple { .. } | ExprKind :: Unary { .. } | ExprKind :: Call { .. } | ExprKind :: ByUse { .. } | ExprKind :: Assign { .. } | ExprKind :: AssignOp { .. } | ExprKind :: Break { .. } | ExprKind :: Closure { .. } | ExprKind :: Continue { .. } | ExprKind :: ConstContinue { .. } | ExprKind :: Return { .. } | ExprKind :: Become { .. } | ExprKind :: Yield { .. } | ExprKind :: Loop { .. } | ExprKind :: LoopMatch { .. } | ExprKind :: Let { .. } | ExprKind :: Match { .. } | ExprKind :: Box { .. } | ExprKind :: If { .. } | ExprKind :: InlineAsm { .. } | ExprKind :: OffsetOf { .. } | ExprKind :: LogicalOp { .. } | ExprKind :: Use { .. } => { self . assignment_info = None ; } } ; match expr . kind { ExprKind :: Scope { value , lint_level : LintLevel :: Explicit (hir_id) , region_scope : _ } => { let prev_id = self . hir_context ; self . hir_context = hir_id ; ensure_sufficient_stack (| | { self . visit_expr (& self . thir [value]) ; }) ; self . hir_context = prev_id ; return ; } ExprKind :: Call { fun , ty : _ , args : _ , from_hir_call : _ , fn_span : _ } => { let fn_ty = self . thir [fun] . ty ; let sig = fn_ty . fn_sig (self . tcx) ; let (callee_features , safe_target_features) : (& [_] , _) = match fn_ty . kind () { ty :: FnDef (func_id , ..) => { let cg_attrs = self . tcx . codegen_fn_attrs (func_id) ; (& cg_attrs . target_features , cg_attrs . safe_target_features) } _ => (& [] , false) , } ; if sig . safety () . is_unsafe () && ! safe_target_features { let func_id = if let ty :: FnDef (func_id , _) = fn_ty . kind () { Some (* func_id) } else { None } ; self . requires_unsafe (expr . span , CallToUnsafeFunction (func_id)) ; } else if let & ty :: FnDef (func_did , _) = fn_ty . kind () { if ! self . tcx . is_target_feature_call_safe (callee_features , self . body_target_features) { let missing : Vec < _ > = callee_features . iter () . copied () . filter (| feature | { feature . kind == TargetFeatureKind :: Enabled && ! self . body_target_features . iter () . any (| body_feature | body_feature . name == feature . name) }) . map (| feature | feature . name) . collect () ; let build_enabled = self . tcx . sess . target_features . iter () . copied () . filter (| feature | missing . contains (feature)) . collect () ; self . requires_unsafe (expr . span , CallToFunctionWith { function : func_did , missing , build_enabled } ,) ; } } } ExprKind :: RawBorrow { arg , .. } => { if let ExprKind :: Scope { value : arg , .. } = self . thir [arg] . kind && let ExprKind :: Deref { arg } = self . thir [arg] . kind { visit :: walk_expr (self , & self . thir [arg]) ; return ; } } ExprKind :: Deref { arg } => { if let ExprKind :: StaticRef { def_id , .. } | ExprKind :: ThreadLocalRef (def_id) = self . thir [arg] . kind { if self . tcx . is_mutable_static (def_id) { self . requires_unsafe (expr . span , UseOfMutableStatic) ; } else if self . tcx . is_foreign_item (def_id) { match self . tcx . def_kind (def_id) { DefKind :: Static { safety : hir :: Safety :: Safe , .. } => { } _ => self . requires_unsafe (expr . span , UseOfExternStatic) , } } } else if self . thir [arg] . ty . is_raw_ptr () { self . requires_unsafe (expr . span , DerefOfRawPointer) ; } } ExprKind :: InlineAsm (box InlineAsmExpr { asm_macro : asm_macro @ (AsmMacro :: Asm | AsmMacro :: NakedAsm) , ref operands , template : _ , options : _ , line_spans : _ , }) => { if let AsmMacro :: Asm = asm_macro { self . requires_unsafe (expr . span , UseOfInlineAssembly) ; } for op in & * * operands { use rustc_middle :: thir :: InlineAsmOperand :: * ; match op { In { expr , reg : _ } | Out { expr : Some (expr) , reg : _ , late : _ } | InOut { expr , reg : _ , late : _ } => self . visit_expr (& self . thir () [* expr]) , SplitInOut { in_expr , out_expr , reg : _ , late : _ } => { self . visit_expr (& self . thir () [* in_expr]) ; if let Some (out_expr) = out_expr { self . visit_expr (& self . thir () [* out_expr]) ; } } Out { expr : None , reg : _ , late : _ } | Const { value : _ , span : _ } | SymFn { value : _ } | SymStatic { def_id : _ } => { } Label { block } => { self . in_safety_context (SafetyContext :: Safe , | this | { visit :: walk_block (this , & this . thir () [* block]) }) ; } } } return ; } ExprKind :: Adt (box AdtExpr { adt_def , variant_index , args : _ , user_ty : _ , fields : _ , base : _ , }) => { if adt_def . variant (variant_index) . has_unsafe_fields () { self . requires_unsafe (expr . span , InitializingTypeWithUnsafeField) } match self . tcx . layout_scalar_valid_range (adt_def . did ()) { (Bound :: Unbounded , Bound :: Unbounded) => { } _ => self . requires_unsafe (expr . span , InitializingTypeWith) , } } ExprKind :: Closure (box ClosureExpr { closure_id , args : _ , upvars : _ , movability : _ , fake_reads : _ , }) => { self . visit_inner_body (closure_id) ; } ExprKind :: ConstBlock { did , args : _ } => { let def_id = did . expect_local () ; self . visit_inner_body (def_id) ; } ExprKind :: Field { lhs , variant_index , name } => { let lhs = & self . thir [lhs] ; if let ty :: Adt (adt_def , _) = lhs . ty . kind () { if adt_def . variant (variant_index) . fields [name] . safety . is_unsafe () { self . requires_unsafe (expr . span , UseOfUnsafeField) ; } else if adt_def . is_union () { if let Some (assigned_ty) = self . assignment_info { if assigned_ty . needs_drop (self . tcx , self . typing_env) { assert ! (self . tcx . dcx () . has_errors () . is_some () , "union fields that need dropping should be impossible: {assigned_ty}") ; } } else { self . requires_unsafe (expr . span , AccessToUnionField) ; } } } } ExprKind :: Assign { lhs , rhs } | ExprKind :: AssignOp { lhs , rhs , .. } => { let lhs = & self . thir [lhs] ; let mut visitor = LayoutConstrainedPlaceVisitor :: new (self . thir , self . tcx) ; visit :: walk_expr (& mut visitor , lhs) ; if visitor . found { self . requires_unsafe (expr . span , MutationOfLayoutConstrainedField) ; } if matches ! (expr . kind , ExprKind :: Assign { .. }) { self . assignment_info = Some (lhs . ty) ; visit :: walk_expr (self , lhs) ; self . assignment_info = None ; visit :: walk_expr (self , & self . thir () [rhs]) ; return ; } } ExprKind :: Borrow { borrow_kind , arg } => { let mut visitor = LayoutConstrainedPlaceVisitor :: new (self . thir , self . tcx) ; visit :: walk_expr (& mut visitor , expr) ; if visitor . found { match borrow_kind { BorrowKind :: Fake (_) | BorrowKind :: Shared if ! self . thir [arg] . ty . is_freeze (self . tcx , self . typing_env) => { self . requires_unsafe (expr . span , BorrowOfLayoutConstrainedField) } BorrowKind :: Mut { .. } => { self . requires_unsafe (expr . span , MutationOfLayoutConstrainedField) } BorrowKind :: Fake (_) | BorrowKind :: Shared => { } } } } ExprKind :: PlaceUnwrapUnsafeBinder { .. } | ExprKind :: ValueUnwrapUnsafeBinder { .. } | ExprKind :: WrapUnsafeBinder { .. } => { self . requires_unsafe (expr . span , UnsafeBinderCast) ; } _ => { } } visit :: walk_expr (self , expr) ; } }}}
mkitem!{mkenum!{#[derive (Clone)] enum SafetyContext { Safe , BuiltinUnsafeBlock , UnsafeFn , UnsafeBlock { span : Span , hir_id : HirId , used : bool , nested_used_blocks : Vec < NestedUsedBlock > } , }}}
mkitem!{mkstruct!{#[derive (Clone , Copy)] struct NestedUsedBlock { hir_id : HirId , span : Span , }}}
mkitem!{mkstruct!{struct UnusedUnsafeWarning { hir_id : HirId , block_span : Span , enclosing_unsafe : Option < UnusedUnsafeEnclosing > , }}}
mkitem!{mkenum!{#[derive (Clone , PartialEq)] enum UnsafeOpKind { CallToUnsafeFunction (Option < DefId >) , UseOfInlineAssembly , InitializingTypeWith , InitializingTypeWithUnsafeField , UseOfMutableStatic , UseOfExternStatic , UseOfUnsafeField , DerefOfRawPointer , AccessToUnionField , MutationOfLayoutConstrainedField , BorrowOfLayoutConstrainedField , CallToFunctionWith { function : DefId , #[doc = " Target features enabled in callee's `#[target_feature]` but missing in"] #[doc = " caller's `#[target_feature]`."] missing : Vec < Symbol > , #[doc = " Target features in `missing` that are enabled at compile time"] #[doc = " (e.g., with `-C target-feature`)."] build_enabled : Vec < Symbol > , } , UnsafeBinderCast , }}}
mkuse!{use UnsafeOpKind :: * ;}
mkitem!{mkimpl!{impl UnsafeOpKind { fn emit_unsafe_op_in_unsafe_fn_lint (& self , tcx : TyCtxt < '_ > , hir_id : HirId , span : Span , suggest_unsafe_block : bool ,) { if tcx . hir_opt_delegation_sig_id (hir_id . owner . def_id) . is_some () { return ; } let parent_id = tcx . hir_get_parent_item (hir_id) ; let parent_owner = tcx . hir_owner_node (parent_id) ; let should_suggest = parent_owner . fn_sig () . is_some_and (| sig | { matches ! (sig . header . safety , hir :: HeaderSafety :: Normal (hir :: Safety :: Unsafe)) }) ; let unsafe_not_inherited_note = if should_suggest { suggest_unsafe_block . then (| | { let body_span = tcx . hir_body (parent_owner . body_id () . unwrap ()) . value . span ; UnsafeNotInheritedLintNote { signature_span : tcx . def_span (parent_id . def_id) , body_span , } }) } else { None } ; match self { CallToUnsafeFunction (Some (did)) => tcx . emit_node_span_lint (UNSAFE_OP_IN_UNSAFE_FN , hir_id , span , UnsafeOpInUnsafeFnCallToUnsafeFunctionRequiresUnsafe { span , function : with_no_trimmed_paths ! (tcx . def_path_str (* did)) , unsafe_not_inherited_note , } ,) , CallToUnsafeFunction (None) => tcx . emit_node_span_lint (UNSAFE_OP_IN_UNSAFE_FN , hir_id , span , UnsafeOpInUnsafeFnCallToUnsafeFunctionRequiresUnsafeNameless { span , unsafe_not_inherited_note , } ,) , UseOfInlineAssembly => tcx . emit_node_span_lint (UNSAFE_OP_IN_UNSAFE_FN , hir_id , span , UnsafeOpInUnsafeFnUseOfInlineAssemblyRequiresUnsafe { span , unsafe_not_inherited_note , } ,) , InitializingTypeWith => tcx . emit_node_span_lint (UNSAFE_OP_IN_UNSAFE_FN , hir_id , span , UnsafeOpInUnsafeFnInitializingTypeWithRequiresUnsafe { span , unsafe_not_inherited_note , } ,) , InitializingTypeWithUnsafeField => tcx . emit_node_span_lint (UNSAFE_OP_IN_UNSAFE_FN , hir_id , span , UnsafeOpInUnsafeFnInitializingTypeWithUnsafeFieldRequiresUnsafe { span , unsafe_not_inherited_note , } ,) , UseOfMutableStatic => tcx . emit_node_span_lint (UNSAFE_OP_IN_UNSAFE_FN , hir_id , span , UnsafeOpInUnsafeFnUseOfMutableStaticRequiresUnsafe { span , unsafe_not_inherited_note , } ,) , UseOfExternStatic => tcx . emit_node_span_lint (UNSAFE_OP_IN_UNSAFE_FN , hir_id , span , UnsafeOpInUnsafeFnUseOfExternStaticRequiresUnsafe { span , unsafe_not_inherited_note , } ,) , UseOfUnsafeField => tcx . emit_node_span_lint (UNSAFE_OP_IN_UNSAFE_FN , hir_id , span , UnsafeOpInUnsafeFnUseOfUnsafeFieldRequiresUnsafe { span , unsafe_not_inherited_note , } ,) , DerefOfRawPointer => tcx . emit_node_span_lint (UNSAFE_OP_IN_UNSAFE_FN , hir_id , span , UnsafeOpInUnsafeFnDerefOfRawPointerRequiresUnsafe { span , unsafe_not_inherited_note , } ,) , AccessToUnionField => tcx . emit_node_span_lint (UNSAFE_OP_IN_UNSAFE_FN , hir_id , span , UnsafeOpInUnsafeFnAccessToUnionFieldRequiresUnsafe { span , unsafe_not_inherited_note , } ,) , MutationOfLayoutConstrainedField => tcx . emit_node_span_lint (UNSAFE_OP_IN_UNSAFE_FN , hir_id , span , UnsafeOpInUnsafeFnMutationOfLayoutConstrainedFieldRequiresUnsafe { span , unsafe_not_inherited_note , } ,) , BorrowOfLayoutConstrainedField => tcx . emit_node_span_lint (UNSAFE_OP_IN_UNSAFE_FN , hir_id , span , UnsafeOpInUnsafeFnBorrowOfLayoutConstrainedFieldRequiresUnsafe { span , unsafe_not_inherited_note , } ,) , CallToFunctionWith { function , missing , build_enabled } => tcx . emit_node_span_lint (UNSAFE_OP_IN_UNSAFE_FN , hir_id , span , UnsafeOpInUnsafeFnCallToFunctionWithRequiresUnsafe { span , function : with_no_trimmed_paths ! (tcx . def_path_str (* function)) , missing_target_features : DiagArgValue :: StrListSepByAnd (missing . iter () . map (| feature | Cow :: from (feature . to_string ())) . collect () ,) , missing_target_features_count : missing . len () , note : ! build_enabled . is_empty () , build_target_features : DiagArgValue :: StrListSepByAnd (build_enabled . iter () . map (| feature | Cow :: from (feature . to_string ())) . collect () ,) , build_target_features_count : build_enabled . len () , unsafe_not_inherited_note , } ,) , UnsafeBinderCast => tcx . emit_node_span_lint (UNSAFE_OP_IN_UNSAFE_FN , hir_id , span , UnsafeOpInUnsafeFnUnsafeBinderCastRequiresUnsafe { span , unsafe_not_inherited_note , } ,) , } } fn emit_requires_unsafe_err (& self , tcx : TyCtxt < '_ > , span : Span , hir_context : HirId , unsafe_op_in_unsafe_fn_allowed : bool ,) { let note_non_inherited = tcx . hir_parent_iter (hir_context) . find (| (id , node) | { if let hir :: Node :: Expr (block) = node && let hir :: ExprKind :: Block (block , _) = block . kind && let hir :: BlockCheckMode :: UnsafeBlock (_) = block . rules { true } else if let Some (sig) = tcx . hir_fn_sig_by_hir_id (* id) && matches ! (sig . header . safety , hir :: HeaderSafety :: Normal (hir :: Safety :: Unsafe)) { true } else { false } }) ; let unsafe_not_inherited_note = if let Some ((id , _)) = note_non_inherited { let span = tcx . hir_span (id) ; let span = tcx . sess . source_map () . guess_head_span (span) ; Some (UnsafeNotInheritedNote { span }) } else { None } ; let dcx = tcx . dcx () ; match self { CallToUnsafeFunction (Some (did)) if unsafe_op_in_unsafe_fn_allowed => { dcx . emit_err (CallToUnsafeFunctionRequiresUnsafeUnsafeOpInUnsafeFnAllowed { span , unsafe_not_inherited_note , function : tcx . def_path_str (* did) , }) ; } CallToUnsafeFunction (Some (did)) => { dcx . emit_err (CallToUnsafeFunctionRequiresUnsafe { span , unsafe_not_inherited_note , function : tcx . def_path_str (* did) , }) ; } CallToUnsafeFunction (None) if unsafe_op_in_unsafe_fn_allowed => { dcx . emit_err (CallToUnsafeFunctionRequiresUnsafeNamelessUnsafeOpInUnsafeFnAllowed { span , unsafe_not_inherited_note , }) ; } CallToUnsafeFunction (None) => { dcx . emit_err (CallToUnsafeFunctionRequiresUnsafeNameless { span , unsafe_not_inherited_note , }) ; } UseOfInlineAssembly if unsafe_op_in_unsafe_fn_allowed => { dcx . emit_err (UseOfInlineAssemblyRequiresUnsafeUnsafeOpInUnsafeFnAllowed { span , unsafe_not_inherited_note , }) ; } UseOfInlineAssembly => { dcx . emit_err (UseOfInlineAssemblyRequiresUnsafe { span , unsafe_not_inherited_note }) ; } InitializingTypeWith if unsafe_op_in_unsafe_fn_allowed => { dcx . emit_err (InitializingTypeWithRequiresUnsafeUnsafeOpInUnsafeFnAllowed { span , unsafe_not_inherited_note , }) ; } InitializingTypeWith => { dcx . emit_err (InitializingTypeWithRequiresUnsafe { span , unsafe_not_inherited_note , }) ; } InitializingTypeWithUnsafeField if unsafe_op_in_unsafe_fn_allowed => { dcx . emit_err (InitializingTypeWithUnsafeFieldRequiresUnsafeUnsafeOpInUnsafeFnAllowed { span , unsafe_not_inherited_note , } ,) ; } InitializingTypeWithUnsafeField => { dcx . emit_err (InitializingTypeWithUnsafeFieldRequiresUnsafe { span , unsafe_not_inherited_note , }) ; } UseOfMutableStatic if unsafe_op_in_unsafe_fn_allowed => { dcx . emit_err (UseOfMutableStaticRequiresUnsafeUnsafeOpInUnsafeFnAllowed { span , unsafe_not_inherited_note , }) ; } UseOfMutableStatic => { dcx . emit_err (UseOfMutableStaticRequiresUnsafe { span , unsafe_not_inherited_note }) ; } UseOfExternStatic if unsafe_op_in_unsafe_fn_allowed => { dcx . emit_err (UseOfExternStaticRequiresUnsafeUnsafeOpInUnsafeFnAllowed { span , unsafe_not_inherited_note , }) ; } UseOfExternStatic => { dcx . emit_err (UseOfExternStaticRequiresUnsafe { span , unsafe_not_inherited_note }) ; } UseOfUnsafeField if unsafe_op_in_unsafe_fn_allowed => { dcx . emit_err (UseOfUnsafeFieldRequiresUnsafeUnsafeOpInUnsafeFnAllowed { span , unsafe_not_inherited_note , }) ; } UseOfUnsafeField => { dcx . emit_err (UseOfUnsafeFieldRequiresUnsafe { span , unsafe_not_inherited_note }) ; } DerefOfRawPointer if unsafe_op_in_unsafe_fn_allowed => { dcx . emit_err (DerefOfRawPointerRequiresUnsafeUnsafeOpInUnsafeFnAllowed { span , unsafe_not_inherited_note , }) ; } DerefOfRawPointer => { dcx . emit_err (DerefOfRawPointerRequiresUnsafe { span , unsafe_not_inherited_note }) ; } AccessToUnionField if unsafe_op_in_unsafe_fn_allowed => { dcx . emit_err (AccessToUnionFieldRequiresUnsafeUnsafeOpInUnsafeFnAllowed { span , unsafe_not_inherited_note , }) ; } AccessToUnionField => { dcx . emit_err (AccessToUnionFieldRequiresUnsafe { span , unsafe_not_inherited_note }) ; } MutationOfLayoutConstrainedField if unsafe_op_in_unsafe_fn_allowed => { dcx . emit_err (MutationOfLayoutConstrainedFieldRequiresUnsafeUnsafeOpInUnsafeFnAllowed { span , unsafe_not_inherited_note , } ,) ; } MutationOfLayoutConstrainedField => { dcx . emit_err (MutationOfLayoutConstrainedFieldRequiresUnsafe { span , unsafe_not_inherited_note , }) ; } BorrowOfLayoutConstrainedField if unsafe_op_in_unsafe_fn_allowed => { dcx . emit_err (BorrowOfLayoutConstrainedFieldRequiresUnsafeUnsafeOpInUnsafeFnAllowed { span , unsafe_not_inherited_note , } ,) ; } BorrowOfLayoutConstrainedField => { dcx . emit_err (BorrowOfLayoutConstrainedFieldRequiresUnsafe { span , unsafe_not_inherited_note , }) ; } CallToFunctionWith { function , missing , build_enabled } if unsafe_op_in_unsafe_fn_allowed => { dcx . emit_err (CallToFunctionWithRequiresUnsafeUnsafeOpInUnsafeFnAllowed { span , missing_target_features : DiagArgValue :: StrListSepByAnd (missing . iter () . map (| feature | Cow :: from (feature . to_string ())) . collect () ,) , missing_target_features_count : missing . len () , note : ! build_enabled . is_empty () , build_target_features : DiagArgValue :: StrListSepByAnd (build_enabled . iter () . map (| feature | Cow :: from (feature . to_string ())) . collect () ,) , build_target_features_count : build_enabled . len () , unsafe_not_inherited_note , function : tcx . def_path_str (* function) , }) ; } CallToFunctionWith { function , missing , build_enabled } => { dcx . emit_err (CallToFunctionWithRequiresUnsafe { span , missing_target_features : DiagArgValue :: StrListSepByAnd (missing . iter () . map (| feature | Cow :: from (feature . to_string ())) . collect () ,) , missing_target_features_count : missing . len () , note : ! build_enabled . is_empty () , build_target_features : DiagArgValue :: StrListSepByAnd (build_enabled . iter () . map (| feature | Cow :: from (feature . to_string ())) . collect () ,) , build_target_features_count : build_enabled . len () , unsafe_not_inherited_note , function : tcx . def_path_str (* function) , }) ; } UnsafeBinderCast if unsafe_op_in_unsafe_fn_allowed => { dcx . emit_err (UnsafeBinderCastRequiresUnsafeUnsafeOpInUnsafeFnAllowed { span , unsafe_not_inherited_note , }) ; } UnsafeBinderCast => { dcx . emit_err (UnsafeBinderCastRequiresUnsafe { span , unsafe_not_inherited_note }) ; } } } }}}

macro_rules! check_unsafety_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_unsafety in module {}", module_path!());
    };
}

mkfn!{
    check_unsafety_introspect!();
    pub (crate) fn check_unsafety (tcx : TyCtxt < '_ > , def : LocalDefId) { assert ! (! tcx . is_typeck_child (def . to_def_id ())) ; if find_attr ! (tcx . get_all_attrs (def) , AttributeKind :: CustomMir (..) => ()) . is_some () { return ; } let Ok ((thir , expr)) = tcx . thir_body (def) else { return } ; tcx . ensure_done () . mir_built (def) ; let thir = if tcx . sess . opts . unstable_opts . no_steal_thir { & thir . borrow () } else { & thir . steal () } ; let hir_id = tcx . local_def_id_to_hir_id (def) ; let safety_context = tcx . hir_fn_sig_by_hir_id (hir_id) . map_or (SafetyContext :: Safe , | fn_sig | { match fn_sig . header . safety { hir :: HeaderSafety :: SafeTargetFeatures => SafetyContext :: Safe , hir :: HeaderSafety :: Normal (safety) => match safety { hir :: Safety :: Unsafe => SafetyContext :: UnsafeFn , hir :: Safety :: Safe => SafetyContext :: Safe , } , } }) ; let body_target_features = & tcx . body_codegen_attrs (def . to_def_id ()) . target_features ; let mut warnings = Vec :: new () ; let mut visitor = UnsafetyVisitor { tcx , thir , safety_context , hir_context : hir_id , body_target_features , assignment_info : None , in_union_destructure : false , typing_env : ty :: TypingEnv :: non_body_analysis (tcx , def) , inside_adt : false , warnings : & mut warnings , suggest_unsafe_block : true , } ; for param in & thir . params { if let Some (param_pat) = param . pat . as_deref () { visitor . visit_pat (param_pat) ; } } visitor . visit_expr (& thir [expr]) ; warnings . sort_by_key (| w | w . block_span) ; for UnusedUnsafeWarning { hir_id , block_span , enclosing_unsafe } in warnings { let block_span = tcx . sess . source_map () . guess_head_span (block_span) ; tcx . emit_node_span_lint (UNUSED_UNSAFE , hir_id , block_span , UnusedUnsafe { span : block_span , enclosing : enclosing_unsafe } ,) ; } }
}