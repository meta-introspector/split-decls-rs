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
mkuse!{use rustc_data_structures :: fx :: FxIndexMap ;}
mkuse!{use rustc_hir :: intravisit :: { self , Visitor } ;}
mkuse!{use rustc_hir :: { self as hir , LifetimeSource } ;}
mkuse!{use rustc_session :: { declare_lint , declare_lint_pass } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use tracing :: instrument ;}
mkuse!{use crate :: { LateContext , LateLintPass , LintContext , lints } ;}
mkitem!{declare_lint ! { #[doc = " The `mismatched_lifetime_syntaxes` lint detects when the same"] #[doc = " lifetime is referred to by different syntaxes between function"] #[doc = " arguments and return values."] #[doc = ""] #[doc = " The three kinds of syntaxes are:"] #[doc = ""] #[doc = " 1. Named lifetimes. These are references (`&'a str`) or paths"] #[doc = "    (`Person<'a>`) that use a lifetime with a name, such as"] #[doc = "    `'static` or `'a`."] #[doc = ""] #[doc = " 2. Elided lifetimes. These are references with no explicit"] #[doc = "    lifetime (`&str`), references using the anonymous lifetime"] #[doc = "    (`&'_ str`), and paths using the anonymous lifetime"] #[doc = "    (`Person<'_>`)."] #[doc = ""] #[doc = " 3. Hidden lifetimes. These are paths that do not contain any"] #[doc = "    visual indication that it contains a lifetime (`Person`)."] #[doc = ""] #[doc = " ### Example"] #[doc = ""] #[doc = " ```rust,compile_fail"] #[doc = " #![deny(mismatched_lifetime_syntaxes)]"] #[doc = ""] #[doc = " pub fn mixing_named_with_elided(v: &'static u8) -> &u8 {"] #[doc = "     v"] #[doc = " }"] #[doc = ""] #[doc = " struct Person<'a> {"] #[doc = "     name: &'a str,"] #[doc = " }"] #[doc = ""] #[doc = " pub fn mixing_hidden_with_elided(v: Person) -> Person<'_> {"] #[doc = "     v"] #[doc = " }"] #[doc = ""] #[doc = " struct Foo;"] #[doc = ""] #[doc = " impl Foo {"] #[doc = "     // Lifetime elision results in the output lifetime becoming"] #[doc = "     // `'static`, which is not what was intended."] #[doc = "     pub fn get_mut(&'static self, x: &mut u8) -> &mut u8 {"] #[doc = "         unsafe { &mut *(x as *mut _) }"] #[doc = "     }"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " {{produces}}"] #[doc = ""] #[doc = " ### Explanation"] #[doc = ""] #[doc = " Lifetime elision is useful because it frees you from having to"] #[doc = " give each lifetime its own name and show the relation of input"] #[doc = " and output lifetimes for common cases. However, a lifetime"] #[doc = " that uses inconsistent syntax between related arguments and"] #[doc = " return values is more confusing."] #[doc = ""] #[doc = " In certain `unsafe` code, lifetime elision combined with"] #[doc = " inconsistent lifetime syntax may result in unsound code."] pub MISMATCHED_LIFETIME_SYNTAXES , Warn , "detects when a lifetime uses different syntax between arguments and return values" }}
mkitem!{declare_lint_pass ! (LifetimeSyntax => [MISMATCHED_LIFETIME_SYNTAXES]) ;}
mkitem!{mkimpl!{impl < 'tcx > LateLintPass < 'tcx > for LifetimeSyntax { #[instrument (skip_all)] fn check_fn (& mut self , cx : & LateContext < 'tcx > , _ : hir :: intravisit :: FnKind < 'tcx > , fd : & 'tcx hir :: FnDecl < 'tcx > , _ : & 'tcx hir :: Body < 'tcx > , _ : rustc_span :: Span , _ : rustc_span :: def_id :: LocalDefId ,) { check_fn_like (cx , fd) ; } #[instrument (skip_all)] fn check_trait_item (& mut self , cx : & LateContext < 'tcx > , ti : & 'tcx hir :: TraitItem < 'tcx >) { match ti . kind { hir :: TraitItemKind :: Const (..) => { } hir :: TraitItemKind :: Fn (fn_sig , _trait_fn) => check_fn_like (cx , fn_sig . decl) , hir :: TraitItemKind :: Type (..) => { } } } #[instrument (skip_all)] fn check_foreign_item (& mut self , cx : & LateContext < 'tcx > , fi : & 'tcx rustc_hir :: ForeignItem < 'tcx > ,) { match fi . kind { hir :: ForeignItemKind :: Fn (fn_sig , _idents , _generics) => check_fn_like (cx , fn_sig . decl) , hir :: ForeignItemKind :: Static (..) => { } hir :: ForeignItemKind :: Type => { } } } }}}

macro_rules! check_fn_like_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_fn_like in module {}", module_path!());
    };
}

mkfn!{
    check_fn_like_introspect!();
    fn check_fn_like < 'tcx > (cx : & LateContext < 'tcx > , fd : & 'tcx hir :: FnDecl < 'tcx >) { let mut input_map = Default :: default () ; let mut output_map = Default :: default () ; for input in fd . inputs { LifetimeInfoCollector :: collect (input , & mut input_map) ; } if let hir :: FnRetTy :: Return (output) = fd . output { LifetimeInfoCollector :: collect (output , & mut output_map) ; } report_mismatches (cx , & input_map , & output_map) ; }
}

macro_rules! report_mismatches_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_mismatches in module {}", module_path!());
    };
}

mkfn!{
    report_mismatches_introspect!();
    #[instrument (skip_all)] fn report_mismatches < 'tcx > (cx : & LateContext < 'tcx > , inputs : & LifetimeInfoMap < 'tcx > , outputs : & LifetimeInfoMap < 'tcx > ,) { for (resolved_lifetime , output_info) in outputs { if let Some (input_info) = inputs . get (resolved_lifetime) { if ! lifetimes_use_matched_syntax (input_info , output_info) { emit_mismatch_diagnostic (cx , input_info , output_info) ; } } } }
}
mkitem!{mkenum!{#[derive (Debug , Copy , Clone , PartialEq)] enum LifetimeSyntaxCategory { Hidden , Elided , Named , }}}
mkitem!{mkimpl!{impl LifetimeSyntaxCategory { fn new (syntax_source : (hir :: LifetimeSyntax , LifetimeSource)) -> Option < Self > { use LifetimeSource :: * ; use hir :: LifetimeSyntax :: * ; match syntax_source { (Implicit , Reference) | (ExplicitAnonymous , Reference) | (ExplicitAnonymous , Path { .. }) | (ExplicitAnonymous , OutlivesBound | PreciseCapturing) => { Some (Self :: Elided) } (Implicit , Path { .. }) => { Some (Self :: Hidden) } (ExplicitBound , Reference) | (ExplicitBound , Path { .. }) | (ExplicitBound , OutlivesBound | PreciseCapturing) => { Some (Self :: Named) } (Implicit , OutlivesBound | PreciseCapturing) | (_ , Other) => { None } } } }}}
mkitem!{mkstruct!{#[derive (Debug , Default)] pub struct LifetimeSyntaxCategories < T > { pub hidden : T , pub elided : T , pub named : T , }}}
mkitem!{mkimpl!{impl < T > LifetimeSyntaxCategories < T > { fn select (& mut self , category : LifetimeSyntaxCategory) -> & mut T { use LifetimeSyntaxCategory :: * ; match category { Elided => & mut self . elided , Hidden => & mut self . hidden , Named => & mut self . named , } } }}}
mkitem!{mkimpl!{impl < T > LifetimeSyntaxCategories < Vec < T > > { pub fn len (& self) -> LifetimeSyntaxCategories < usize > { LifetimeSyntaxCategories { hidden : self . hidden . len () , elided : self . elided . len () , named : self . named . len () , } } pub fn iter_unnamed (& self) -> impl Iterator < Item = & T > { let Self { hidden , elided , named : _ } = self ; [hidden . iter () , elided . iter ()] . into_iter () . flatten () } }}}
mkitem!{mkimpl!{impl std :: ops :: Add for LifetimeSyntaxCategories < usize > { type Output = Self ; fn add (self , rhs : Self) -> Self :: Output { Self { hidden : self . hidden + rhs . hidden , elided : self . elided + rhs . elided , named : self . named + rhs . named , } } }}}

macro_rules! lifetimes_use_matched_syntax_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lifetimes_use_matched_syntax in module {}", module_path!());
    };
}

mkfn!{
    lifetimes_use_matched_syntax_introspect!();
    fn lifetimes_use_matched_syntax (input_info : & [Info < '_ >] , output_info : & [Info < '_ >]) -> bool { let mut syntax_counts = LifetimeSyntaxCategories :: < usize > :: default () ; for info in input_info . iter () . chain (output_info) { if let Some (category) = info . lifetime_syntax_category () { * syntax_counts . select (category) += 1 ; } } tracing :: debug ! (? syntax_counts) ; matches ! (syntax_counts , LifetimeSyntaxCategories { hidden : _ , elided : 0 , named : 0 } | LifetimeSyntaxCategories { hidden : 0 , elided : _ , named : 0 } | LifetimeSyntaxCategories { hidden : 0 , elided : 0 , named : _ }) }
}

macro_rules! emit_mismatch_diagnostic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_mismatch_diagnostic in module {}", module_path!());
    };
}

mkfn!{
    emit_mismatch_diagnostic_introspect!();
    fn emit_mismatch_diagnostic < 'tcx > (cx : & LateContext < 'tcx > , input_info : & [Info < '_ >] , output_info : & [Info < '_ >] ,) { let mut bound_lifetime = None ; let mut suggest_change_to_explicit_bound = Vec :: new () ; let mut suggest_change_to_mixed_implicit = Vec :: new () ; let mut suggest_change_to_mixed_explicit_anonymous = Vec :: new () ; let mut suggest_change_to_implicit = Vec :: new () ; let mut suggest_change_to_explicit_anonymous = Vec :: new () ; let mut allow_suggesting_implicit = true ; let mut saw_a_reference = false ; let mut saw_a_path = false ; for info in input_info . iter () . chain (output_info) { use LifetimeSource :: * ; use hir :: LifetimeSyntax :: * ; let syntax_source = info . syntax_source () ; if let (_ , Other) = syntax_source { continue ; } if let (ExplicitBound , _) = syntax_source { bound_lifetime = Some (info) ; } match syntax_source { (Implicit , Reference) => { suggest_change_to_explicit_anonymous . push (info) ; suggest_change_to_explicit_bound . push (info) ; } (ExplicitAnonymous , Reference) => { suggest_change_to_implicit . push (info) ; suggest_change_to_explicit_bound . push (info) ; } (Implicit , Path { .. }) => { suggest_change_to_mixed_explicit_anonymous . push (info) ; suggest_change_to_explicit_anonymous . push (info) ; suggest_change_to_explicit_bound . push (info) ; } (ExplicitAnonymous , Path { .. }) => { suggest_change_to_explicit_bound . push (info) ; } (ExplicitBound , Reference) => { suggest_change_to_implicit . push (info) ; suggest_change_to_mixed_implicit . push (info) ; suggest_change_to_explicit_anonymous . push (info) ; } (ExplicitBound , Path { .. }) => { suggest_change_to_mixed_explicit_anonymous . push (info) ; suggest_change_to_explicit_anonymous . push (info) ; } (Implicit , OutlivesBound | PreciseCapturing) => { panic ! ("This syntax / source combination is not possible") ; } (ExplicitAnonymous , OutlivesBound | PreciseCapturing) => { suggest_change_to_explicit_bound . push (info) ; } (ExplicitBound , OutlivesBound | PreciseCapturing) => { suggest_change_to_mixed_explicit_anonymous . push (info) ; suggest_change_to_explicit_anonymous . push (info) ; } (_ , Other) => { panic ! ("This syntax / source combination has already been skipped") ; } } if matches ! (syntax_source , (_ , Path { .. } | OutlivesBound | PreciseCapturing)) { allow_suggesting_implicit = false ; } match syntax_source { (_ , Reference) => saw_a_reference = true , (_ , Path { .. }) => saw_a_path = true , _ => { } } } let categorize = | infos : & [Info < '_ >] | { let mut categories = LifetimeSyntaxCategories :: < Vec < _ > > :: default () ; for info in infos { if let Some (category) = info . lifetime_syntax_category () { categories . select (category) . push (info . reporting_span ()) ; } } categories } ; let inputs = categorize (input_info) ; let outputs = categorize (output_info) ; let make_implicit_suggestions = | infos : & [& Info < '_ >] | infos . iter () . map (| i | i . removing_span ()) . collect :: < Vec < _ > > () ; let explicit_bound_suggestion = bound_lifetime . map (| info | { build_mismatch_suggestion (info . lifetime_name () , & suggest_change_to_explicit_bound) }) ; let is_bound_static = bound_lifetime . is_some_and (| info | info . is_static ()) ; tracing :: debug ! (? bound_lifetime , ? explicit_bound_suggestion , ? is_bound_static) ; let should_suggest_mixed = (saw_a_reference && saw_a_path) && (! suggest_change_to_mixed_implicit . is_empty () || ! suggest_change_to_mixed_explicit_anonymous . is_empty ()) && ! is_bound_static ; let mixed_suggestion = should_suggest_mixed . then (| | { let implicit_suggestions = make_implicit_suggestions (& suggest_change_to_mixed_implicit) ; let explicit_anonymous_suggestions = suggest_change_to_mixed_explicit_anonymous . iter () . map (| info | info . suggestion ("'_")) . collect () ; lints :: MismatchedLifetimeSyntaxesSuggestion :: Mixed { implicit_suggestions , explicit_anonymous_suggestions , optional_alternative : false , } }) ; tracing :: debug ! (? suggest_change_to_mixed_implicit , ? suggest_change_to_mixed_explicit_anonymous , ? mixed_suggestion ,) ; let should_suggest_implicit = ! suggest_change_to_implicit . is_empty () && allow_suggesting_implicit && ! is_bound_static ; let implicit_suggestion = should_suggest_implicit . then (| | { let suggestions = make_implicit_suggestions (& suggest_change_to_implicit) ; lints :: MismatchedLifetimeSyntaxesSuggestion :: Implicit { suggestions , optional_alternative : false , } }) ; tracing :: debug ! (? should_suggest_implicit , ? suggest_change_to_implicit , allow_suggesting_implicit , ? implicit_suggestion ,) ; let should_suggest_explicit_anonymous = ! suggest_change_to_explicit_anonymous . is_empty () && ! is_bound_static ; let explicit_anonymous_suggestion = should_suggest_explicit_anonymous . then (| | build_mismatch_suggestion ("'_" , & suggest_change_to_explicit_anonymous)) ; tracing :: debug ! (? should_suggest_explicit_anonymous , ? suggest_change_to_explicit_anonymous , ? explicit_anonymous_suggestion ,) ; let mut suggestions = Vec :: new () ; suggestions . extend (explicit_bound_suggestion) ; suggestions . extend (mixed_suggestion) ; suggestions . extend (implicit_suggestion) ; suggestions . extend (explicit_anonymous_suggestion) ; cx . emit_span_lint (MISMATCHED_LIFETIME_SYNTAXES , inputs . iter_unnamed () . chain (outputs . iter_unnamed ()) . copied () . collect :: < Vec < _ > > () , lints :: MismatchedLifetimeSyntaxes { inputs , outputs , suggestions } ,) ; }
}

macro_rules! build_mismatch_suggestion_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_mismatch_suggestion in module {}", module_path!());
    };
}

mkfn!{
    build_mismatch_suggestion_introspect!();
    fn build_mismatch_suggestion (lifetime_name : & str , infos : & [& Info < '_ >] ,) -> lints :: MismatchedLifetimeSyntaxesSuggestion { let lifetime_name = lifetime_name . to_owned () ; let suggestions = infos . iter () . map (| info | info . suggestion (& lifetime_name)) . collect () ; lints :: MismatchedLifetimeSyntaxesSuggestion :: Explicit { lifetime_name , suggestions , optional_alternative : false , } }
}
mkitem!{mkstruct!{#[derive (Debug)] struct Info < 'tcx > { type_span : Span , referenced_type_span : Option < Span > , lifetime : & 'tcx hir :: Lifetime , }}}
mkitem!{mkimpl!{impl < 'tcx > Info < 'tcx > { fn syntax_source (& self) -> (hir :: LifetimeSyntax , LifetimeSource) { (self . lifetime . syntax , self . lifetime . source) } fn lifetime_syntax_category (& self) -> Option < LifetimeSyntaxCategory > { LifetimeSyntaxCategory :: new (self . syntax_source ()) } fn lifetime_name (& self) -> & str { self . lifetime . ident . as_str () } fn is_static (& self) -> bool { self . lifetime . is_static () } #[doc = " When reporting a lifetime that is implicit, we expand the span"] #[doc = " to include the type. Otherwise we end up pointing at nothing,"] #[doc = " which is a bit confusing."] fn reporting_span (& self) -> Span { if self . lifetime . is_implicit () { self . type_span } else { self . lifetime . ident . span } } #[doc = " When removing an explicit lifetime from a reference,"] #[doc = " we want to remove the whitespace after the lifetime."] #[doc = ""] #[doc = " ```rust"] #[doc = " fn x(a: &'_ u8) {}"] #[doc = " ```"] #[doc = ""] #[doc = " Should become:"] #[doc = ""] #[doc = " ```rust"] #[doc = " fn x(a: &u8) {}"] #[doc = " ```"] fn removing_span (& self) -> Span { let mut span = self . suggestion ("'dummy") . 0 ; if let Some (referenced_type_span) = self . referenced_type_span { span = span . until (referenced_type_span) ; } span } fn suggestion (& self , lifetime_name : & str) -> (Span , String) { self . lifetime . suggestion (lifetime_name) } }}}
mkitem!{type LifetimeInfoMap < 'tcx > = FxIndexMap < & 'tcx hir :: LifetimeKind , Vec < Info < 'tcx > > > ;}
mkitem!{mkstruct!{struct LifetimeInfoCollector < 'a , 'tcx > { type_span : Span , referenced_type_span : Option < Span > , map : & 'a mut LifetimeInfoMap < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > LifetimeInfoCollector < 'a , 'tcx > { fn collect (ty : & 'tcx hir :: Ty < 'tcx > , map : & 'a mut LifetimeInfoMap < 'tcx >) { let mut this = Self { type_span : ty . span , referenced_type_span : None , map } ; intravisit :: walk_unambig_ty (& mut this , ty) ; } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Visitor < 'tcx > for LifetimeInfoCollector < 'a , 'tcx > { #[instrument (skip (self))] fn visit_lifetime (& mut self , lifetime : & 'tcx hir :: Lifetime) { let type_span = self . type_span ; let referenced_type_span = self . referenced_type_span ; let info = Info { type_span , referenced_type_span , lifetime } ; self . map . entry (& lifetime . kind) . or_default () . push (info) ; } #[instrument (skip (self))] fn visit_ty (& mut self , ty : & 'tcx hir :: Ty < 'tcx , hir :: AmbigArg >) -> Self :: Result { let old_type_span = self . type_span ; let old_referenced_type_span = self . referenced_type_span ; self . type_span = ty . span ; if let hir :: TyKind :: Ref (_ , ty) = ty . kind { self . referenced_type_span = Some (ty . ty . span) ; } intravisit :: walk_ty (self , ty) ; self . type_span = old_type_span ; self . referenced_type_span = old_referenced_type_span ; } }}}