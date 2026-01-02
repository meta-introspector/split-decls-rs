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
mkuse!{use std :: { mem , slice } ;}
mkuse!{use rustc_ast :: visit :: { self , Visitor } ;}
mkuse!{use rustc_ast :: { self as ast , HasNodeId , NodeId , attr } ;}
mkuse!{use rustc_ast_pretty :: pprust ;}
mkuse!{use rustc_attr_parsing :: AttributeParser ;}
mkuse!{use rustc_errors :: DiagCtxtHandle ;}
mkuse!{use rustc_expand :: base :: { ExtCtxt , ResolverExpand } ;}
mkuse!{use rustc_expand :: expand :: { AstFragment , ExpansionConfig } ;}
mkuse!{use rustc_feature :: Features ;}
mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_span :: hygiene :: AstPass ;}
mkuse!{use rustc_span :: source_map :: SourceMap ;}
mkuse!{use rustc_span :: { DUMMY_SP , Ident , Span , Symbol , kw , sym } ;}
mkuse!{use smallvec :: smallvec ;}
mkuse!{use thin_vec :: { ThinVec , thin_vec } ;}
mkuse!{use crate :: errors ;}
mkitem!{mkstruct!{struct ProcMacroDerive { id : NodeId , trait_name : Symbol , function_ident : Ident , span : Span , attrs : ThinVec < Symbol > , }}}
mkitem!{mkstruct!{struct ProcMacroDef { id : NodeId , function_ident : Ident , span : Span , }}}
mkitem!{mkenum!{enum ProcMacro { Derive (ProcMacroDerive) , Attr (ProcMacroDef) , Bang (ProcMacroDef) , }}}
mkitem!{mkstruct!{struct CollectProcMacros < 'a > { macros : Vec < ProcMacro > , in_root : bool , dcx : DiagCtxtHandle < 'a > , session : & 'a Session , source_map : & 'a SourceMap , is_proc_macro_crate : bool , is_test_crate : bool , }}}

macro_rules! inject_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function inject in module {}", module_path!());
    };
}

mkfn!{
    inject_introspect!();
    pub fn inject (krate : & mut ast :: Crate , sess : & Session , features : & Features , resolver : & mut dyn ResolverExpand , is_proc_macro_crate : bool , has_proc_macro_decls : bool , is_test_crate : bool , dcx : DiagCtxtHandle < '_ > ,) { let ecfg = ExpansionConfig :: default (sym :: proc_macro , features) ; let mut cx = ExtCtxt :: new (sess , ecfg , resolver , None) ; let mut collect = CollectProcMacros { macros : Vec :: new () , in_root : true , dcx , session : sess , source_map : sess . source_map () , is_proc_macro_crate , is_test_crate , } ; if has_proc_macro_decls || is_proc_macro_crate { visit :: walk_crate (& mut collect , krate) ; } let macros = collect . macros ; if ! is_proc_macro_crate { return ; } if is_test_crate { return ; } let decls = mk_decls (& mut cx , & macros) ; krate . items . push (decls) ; }
}
mkitem!{mkimpl!{impl < 'a > CollectProcMacros < 'a > { fn check_not_pub_in_root (& self , vis : & ast :: Visibility , sp : Span) { if self . is_proc_macro_crate && self . in_root && vis . kind . is_pub () { self . dcx . emit_err (errors :: ProcMacro { span : sp }) ; } } fn collect_custom_derive (& mut self , item : & 'a ast :: Item , function_ident : Ident , attr : & 'a ast :: Attribute ,) { let Some (rustc_hir :: Attribute :: Parsed (AttributeKind :: ProcMacroDerive { trait_name , helper_attrs , .. })) = AttributeParser :: parse_limited (self . session , slice :: from_ref (attr) , sym :: proc_macro_derive , item . span , item . node_id () , None ,) else { return ; } ; if self . in_root && item . vis . kind . is_pub () { self . macros . push (ProcMacro :: Derive (ProcMacroDerive { id : item . id , span : item . span , trait_name , function_ident , attrs : helper_attrs , })) ; } else { let msg = if ! self . in_root { "functions tagged with `#[proc_macro_derive]` must \
                 currently reside in the root of the crate" } else { "functions tagged with `#[proc_macro_derive]` must be `pub`" } ; self . dcx . span_err (self . source_map . guess_head_span (item . span) , msg) ; } } fn collect_attr_proc_macro (& mut self , item : & 'a ast :: Item , function_ident : Ident) { if self . in_root && item . vis . kind . is_pub () { self . macros . push (ProcMacro :: Attr (ProcMacroDef { id : item . id , span : item . span , function_ident , })) ; } else { let msg = if ! self . in_root { "functions tagged with `#[proc_macro_attribute]` must \
                 currently reside in the root of the crate" } else { "functions tagged with `#[proc_macro_attribute]` must be `pub`" } ; self . dcx . span_err (self . source_map . guess_head_span (item . span) , msg) ; } } fn collect_bang_proc_macro (& mut self , item : & 'a ast :: Item , function_ident : Ident) { if self . in_root && item . vis . kind . is_pub () { self . macros . push (ProcMacro :: Bang (ProcMacroDef { id : item . id , span : item . span , function_ident , })) ; } else { let msg = if ! self . in_root { "functions tagged with `#[proc_macro]` must \
                 currently reside in the root of the crate" } else { "functions tagged with `#[proc_macro]` must be `pub`" } ; self . dcx . span_err (self . source_map . guess_head_span (item . span) , msg) ; } } }}}
mkitem!{mkimpl!{impl < 'a > Visitor < 'a > for CollectProcMacros < 'a > { fn visit_item (& mut self , item : & 'a ast :: Item) { if let ast :: ItemKind :: MacroDef (..) = item . kind { if self . is_proc_macro_crate && attr :: contains_name (& item . attrs , sym :: macro_export) { self . dcx . emit_err (errors :: ExportMacroRules { span : self . source_map . guess_head_span (item . span) , }) ; } } let mut found_attr : Option < & 'a ast :: Attribute > = None ; for attr in & item . attrs { if attr . is_proc_macro_attr () { if let Some (prev_attr) = found_attr { let prev_item = prev_attr . get_normal_item () ; let item = attr . get_normal_item () ; let path_str = pprust :: path_to_string (& item . path) ; let msg = if item . path . segments [0] . ident . name == prev_item . path . segments [0] . ident . name { format ! ("only one `#[{path_str}]` attribute is allowed on any given function" ,) } else { format ! ("`#[{}]` and `#[{}]` attributes cannot both be applied
                            to the same function" , path_str , pprust :: path_to_string (& prev_item . path) ,) } ; self . dcx . struct_span_err (attr . span , msg) . with_span_label (prev_attr . span , "previous attribute here") . emit () ; return ; } found_attr = Some (attr) ; } } let Some (attr) = found_attr else { self . check_not_pub_in_root (& item . vis , self . source_map . guess_head_span (item . span)) ; let prev_in_root = mem :: replace (& mut self . in_root , false) ; visit :: walk_item (self , item) ; self . in_root = prev_in_root ; return ; } ; let fn_ident = if let ast :: ItemKind :: Fn (fn_) = & item . kind { fn_ . ident } else { return ; } ; if self . is_test_crate { return ; } if ! self . is_proc_macro_crate { self . dcx . create_err (errors :: AttributeOnlyUsableWithCrateType { span : attr . span , path : & pprust :: path_to_string (& attr . get_normal_item () . path) , }) . emit () ; return ; } if attr . has_name (sym :: proc_macro_derive) { self . collect_custom_derive (item , fn_ident , attr) ; } else if attr . has_name (sym :: proc_macro_attribute) { self . collect_attr_proc_macro (item , fn_ident) ; } else if attr . has_name (sym :: proc_macro) { self . collect_bang_proc_macro (item , fn_ident) ; } ; let prev_in_root = mem :: replace (& mut self . in_root , false) ; visit :: walk_item (self , item) ; self . in_root = prev_in_root ; } }}}

macro_rules! mk_decls_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mk_decls in module {}", module_path!());
    };
}

mkfn!{
    mk_decls_introspect!();
    fn mk_decls (cx : & mut ExtCtxt < '_ > , macros : & [ProcMacro]) -> Box < ast :: Item > { let expn_id = cx . resolver . expansion_for_ast_pass (DUMMY_SP , AstPass :: ProcMacroHarness , & [sym :: rustc_attrs , sym :: proc_macro_internals] , None ,) ; let span = DUMMY_SP . with_def_site_ctxt (expn_id . to_expn_id ()) ; let proc_macro = Ident :: new (sym :: proc_macro , span) ; let krate = cx . item (span , ast :: AttrVec :: new () , ast :: ItemKind :: ExternCrate (None , proc_macro)) ; let bridge = Ident :: new (sym :: bridge , span) ; let client = Ident :: new (sym :: client , span) ; let proc_macro_ty = Ident :: new (sym :: ProcMacro , span) ; let custom_derive = Ident :: new (sym :: custom_derive , span) ; let attr = Ident :: new (sym :: attr , span) ; let bang = Ident :: new (sym :: bang , span) ; let decls = macros . iter () . map (| m | { let harness_span = span ; let span = match m { ProcMacro :: Derive (m) => m . span , ProcMacro :: Attr (m) | ProcMacro :: Bang (m) => m . span , } ; let local_path = | cx : & ExtCtxt < '_ > , ident | cx . expr_path (cx . path (span , vec ! [ident])) ; let proc_macro_ty_method_path = | cx : & ExtCtxt < '_ > , method | { cx . expr_path (cx . path (span . with_ctxt (harness_span . ctxt ()) , vec ! [proc_macro , bridge , client , proc_macro_ty , method] ,)) } ; match m { ProcMacro :: Derive (cd) => { cx . resolver . declare_proc_macro (cd . id) ; cx . expr_call (harness_span , proc_macro_ty_method_path (cx , custom_derive) , thin_vec ! [cx . expr_str (span , cd . trait_name) , cx . expr_array_ref (span , cd . attrs . iter () . map (|& s | cx . expr_str (span , s)) . collect ::< ThinVec < _ >> () ,) , local_path (cx , cd . function_ident) ,] ,) } ProcMacro :: Attr (ca) | ProcMacro :: Bang (ca) => { cx . resolver . declare_proc_macro (ca . id) ; let ident = match m { ProcMacro :: Attr (_) => attr , ProcMacro :: Bang (_) => bang , ProcMacro :: Derive (_) => unreachable ! () , } ; cx . expr_call (harness_span , proc_macro_ty_method_path (cx , ident) , thin_vec ! [cx . expr_str (span , ca . function_ident . name) , local_path (cx , ca . function_ident) ,] ,) } } }) . collect () ; let mut decls_static = cx . item_static (span , Ident :: new (sym :: _DECLS , span) , cx . ty_ref (span , cx . ty (span , ast :: TyKind :: Slice (cx . ty_path (cx . path (span , vec ! [proc_macro , bridge , client , proc_macro_ty])) ,) ,) , None , ast :: Mutability :: Not ,) , ast :: Mutability :: Not , cx . expr_array_ref (span , decls) ,) ; decls_static . attrs . extend ([cx . attr_word (sym :: rustc_proc_macro_decls , span) , cx . attr_word (sym :: used , span) , cx . attr_nested_word (sym :: allow , sym :: deprecated , span) ,]) ; let block = cx . expr_block (cx . block (span , thin_vec ! [cx . stmt_item (span , krate) , cx . stmt_item (span , decls_static)]) ,) ; let anon_constant = cx . item_const (span , Ident :: new (kw :: Underscore , span) , cx . ty (span , ast :: TyKind :: Tup (ThinVec :: new ())) , block ,) ; let items = AstFragment :: Items (smallvec ! [anon_constant]) ; cx . monotonic_expander () . fully_expand_fragment (items) . make_items () . pop () . unwrap () }
}