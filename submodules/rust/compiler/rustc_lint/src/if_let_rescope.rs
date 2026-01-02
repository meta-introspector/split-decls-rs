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
mkuse!{use std :: iter :: repeat ;}
mkuse!{use std :: ops :: ControlFlow ;}
mkuse!{use hir :: intravisit :: { self , Visitor } ;}
mkuse!{use rustc_ast :: Recovered ;}
mkuse!{use rustc_errors :: { Applicability , Diag , EmissionGuarantee , Subdiagnostic , SuggestionStyle } ;}
mkuse!{use rustc_hir :: { self as hir , HirIdSet } ;}
mkuse!{use rustc_macros :: { LintDiagnostic , Subdiagnostic } ;}
mkuse!{use rustc_middle :: ty :: adjustment :: Adjust ;}
mkuse!{use rustc_middle :: ty :: significant_drop_order :: { extract_component_with_significant_dtor , ty_dtor_span , } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt } ;}
mkuse!{use rustc_session :: lint :: { FutureIncompatibilityReason , LintId } ;}
mkuse!{use rustc_session :: { declare_lint , impl_lint_pass } ;}
mkuse!{use rustc_span :: edition :: Edition ;}
mkuse!{use rustc_span :: { DUMMY_SP , Span } ;}
mkuse!{use smallvec :: SmallVec ;}
mkuse!{use crate :: { LateContext , LateLintPass } ;}
mkitem!{declare_lint ! { # [doc = " The `if_let_rescope` lint detects cases where a temporary value with"] # [doc = " significant drop is generated on the right hand side of `if let`"] # [doc = " and suggests a rewrite into `match` when possible."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,edition2021"] # [doc = " #![warn(if_let_rescope)]"] # [doc = " #![allow(unused_variables)]"] # [doc = ""] # [doc = " struct Droppy;"] # [doc = " impl Drop for Droppy {"] # [doc = "     fn drop(&mut self) {"] # [doc = "         // Custom destructor, including this `drop` implementation, is considered"] # [doc = "         // significant."] # [doc = "         // Rust does not check whether this destructor emits side-effects that can"] # [doc = "         // lead to observable change in program semantics, when the drop order changes."] # [doc = "         // Rust biases to be on the safe side, so that you can apply discretion whether"] # [doc = "         // this change indeed breaches any contract or specification that your code needs"] # [doc = "         // to honour."] # [doc = "         println!(\"dropped\");"] # [doc = "     }"] # [doc = " }"] # [doc = " impl Droppy {"] # [doc = "     fn get(&self) -> Option<u8> {"] # [doc = "         None"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     if let Some(value) = Droppy.get() {"] # [doc = "         // do something"] # [doc = "     } else {"] # [doc = "         // do something else"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " With Edition 2024, temporaries generated while evaluating `if let`s"] # [doc = " will be dropped before the `else` block."] # [doc = " This lint captures a possible change in runtime behaviour due to"] # [doc = " a change in sequence of calls to significant `Drop::drop` destructors."] # [doc = ""] # [doc = " A significant [`Drop::drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html)"] # [doc = " destructor here refers to an explicit, arbitrary implementation of the `Drop` trait on the type"] # [doc = " with exceptions including `Vec`, `Box`, `Rc`, `BTreeMap` and `HashMap`"] # [doc = " that are marked by the compiler otherwise so long that the generic types have"] # [doc = " no significant destructor recursively."] # [doc = " In other words, a type has a significant drop destructor when it has a `Drop` implementation"] # [doc = " or its destructor invokes a significant destructor on a type."] # [doc = " Since we cannot completely reason about the change by just inspecting the existence of"] # [doc = " a significant destructor, this lint remains only a suggestion and is set to `allow` by default."] # [doc = ""] # [doc = " Whenever possible, a rewrite into an equivalent `match` expression that"] # [doc = " observe the same order of calls to such destructors is proposed by this lint."] # [doc = " Authors may take their own discretion whether the rewrite suggestion shall be"] # [doc = " accepted, or rejected to continue the use of the `if let` expression."] pub IF_LET_RESCOPE , Allow , "`if let` assigns a shorter lifetime to temporary values being pattern-matched against in Edition 2024 and \
    rewriting in `match` is an option to preserve the semantics up to Edition 2021" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: EditionSemanticsChange (Edition :: Edition2024) , reference : "<https://doc.rust-lang.org/edition-guide/rust-2024/temporary-if-let-scope.html>" , } ; }}
mkitem!{mkstruct!{# [doc = " Lint for potential change in program semantics of `if let`s"] # [derive (Default)] pub (crate) struct IfLetRescope { skip : HirIdSet , }}}

macro_rules! expr_parent_is_else_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expr_parent_is_else in module {}", module_path!());
    };
}

mkfn!{
    expr_parent_is_else_introspect!();
    fn expr_parent_is_else (tcx : TyCtxt < '_ > , hir_id : hir :: HirId) -> bool { let Some ((_ , hir :: Node :: Expr (expr))) = tcx . hir_parent_iter (hir_id) . next () else { return false ; } ; let hir :: ExprKind :: If (_cond , _conseq , Some (alt)) = expr . kind else { return false } ; alt . hir_id == hir_id }
}

macro_rules! expr_parent_is_stmt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expr_parent_is_stmt in module {}", module_path!());
    };
}

mkfn!{
    expr_parent_is_stmt_introspect!();
    fn expr_parent_is_stmt (tcx : TyCtxt < '_ > , hir_id : hir :: HirId) -> bool { let mut parents = tcx . hir_parent_iter (hir_id) ; let stmt = match parents . next () { Some ((_ , hir :: Node :: Stmt (stmt))) => stmt , Some ((_ , hir :: Node :: Block (_) | hir :: Node :: Arm (_))) => return true , _ => return false , } ; let (hir :: StmtKind :: Semi (expr) | hir :: StmtKind :: Expr (expr)) = stmt . kind else { return false } ; expr . hir_id == hir_id }
}

macro_rules! match_head_needs_bracket_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function match_head_needs_bracket in module {}", module_path!());
    };
}

mkfn!{
    match_head_needs_bracket_introspect!();
    fn match_head_needs_bracket (tcx : TyCtxt < '_ > , expr : & hir :: Expr < '_ >) -> bool { expr_parent_is_else (tcx , expr . hir_id) && matches ! (expr . kind , hir :: ExprKind :: If (..)) }
}
mkitem!{mkimpl!{impl IfLetRescope { fn probe_if_cascade < 'tcx > (& mut self , cx : & LateContext < 'tcx > , mut expr : & 'tcx hir :: Expr < 'tcx >) { if self . skip . contains (& expr . hir_id) { return ; } let tcx = cx . tcx ; let source_map = tcx . sess . source_map () ; let expr_end = match expr . kind { hir :: ExprKind :: If (_cond , conseq , None) => conseq . span . shrink_to_hi () , hir :: ExprKind :: If (_cond , _conseq , Some (alt)) => alt . span . shrink_to_hi () , _ => return , } ; let mut seen_dyn = false ; let mut add_bracket_to_match_head = match_head_needs_bracket (tcx , expr) ; let mut significant_droppers = vec ! [] ; let mut lifetime_ends = vec ! [] ; let mut closing_brackets = 0 ; let mut alt_heads = vec ! [] ; let mut match_heads = vec ! [] ; let mut consequent_heads = vec ! [] ; let mut destructors = vec ! [] ; let mut first_if_to_lint = None ; let mut first_if_to_rewrite = false ; let mut empty_alt = false ; while let hir :: ExprKind :: If (cond , conseq , alt) = expr . kind { self . skip . insert (expr . hir_id) ; if let hir :: ExprKind :: Let (& hir :: LetExpr { span , pat , init , ty : ty_ascription , recovered : Recovered :: No , }) = cond . kind { let if_let_pat = source_map . span_take_while (expr . span , | & ch | ch == '(' || ch . is_whitespace ()) . between (init . span) ; let before_conseq = conseq . span . shrink_to_lo () ; let lifetime_end = source_map . end_point (conseq . span) ; if let ControlFlow :: Break ((drop_span , drop_tys)) = (FindSignificantDropper { cx }) . check_if_let_scrutinee (init) { destructors . extend (drop_tys . into_iter () . filter_map (| ty | { if let Some (span) = ty_dtor_span (tcx , ty) { Some (DestructorLabel { span , dtor_kind : "concrete" }) } else if matches ! (ty . kind () , ty :: Dynamic (..)) { if seen_dyn { None } else { seen_dyn = true ; Some (DestructorLabel { span : DUMMY_SP , dtor_kind : "dyn" }) } } else { None } })) ; first_if_to_lint = first_if_to_lint . or_else (| | Some ((span , expr . hir_id))) ; significant_droppers . push (drop_span) ; lifetime_ends . push (lifetime_end) ; if ty_ascription . is_some () || ! expr . span . can_be_used_for_suggestions () || ! pat . span . can_be_used_for_suggestions () || ! if_let_pat . can_be_used_for_suggestions () || ! before_conseq . can_be_used_for_suggestions () { } else if let Ok (pat) = source_map . span_to_snippet (pat . span) { let emit_suggestion = | alt_span | { first_if_to_rewrite = true ; if add_bracket_to_match_head { closing_brackets += 2 ; match_heads . push (SingleArmMatchBegin :: WithOpenBracket (if_let_pat)) ; } else { closing_brackets += 1 ; match_heads . push (SingleArmMatchBegin :: WithoutOpenBracket (if_let_pat)) ; } consequent_heads . push (ConsequentRewrite { span : before_conseq , pat }) ; if let Some (alt_span) = alt_span { alt_heads . push (AltHead (alt_span)) ; } } ; if let Some (alt) = alt { let alt_head = conseq . span . between (alt . span) ; if alt_head . can_be_used_for_suggestions () { emit_suggestion (Some (alt_head)) ; } } else { emit_suggestion (None) ; empty_alt = true ; break ; } } } } add_bracket_to_match_head = true ; if let Some (alt) = alt { expr = alt ; } else { break ; } } if let Some ((span , hir_id)) = first_if_to_lint { tcx . emit_node_span_lint (IF_LET_RESCOPE , hir_id , span , IfLetRescopeLint { destructors , significant_droppers , lifetime_ends , rewrite : first_if_to_rewrite . then_some (IfLetRescopeRewrite { match_heads , consequent_heads , closing_brackets : ClosingBrackets { span : expr_end , count : closing_brackets , empty_alt , } , alt_heads , }) , } ,) ; } } }}}
mkitem!{impl_lint_pass ! (IfLetRescope => [IF_LET_RESCOPE]) ;}
mkitem!{mkimpl!{impl < 'tcx > LateLintPass < 'tcx > for IfLetRescope { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx >) { if expr . span . edition () . at_least_rust_2024 () || cx . tcx . lints_that_dont_need_to_run (()) . contains (& LintId :: of (IF_LET_RESCOPE)) { return ; } if let hir :: ExprKind :: Loop (block , _label , hir :: LoopSource :: While , _span) = expr . kind && let Some (value) = block . expr && let hir :: ExprKind :: If (cond , _conseq , _alt) = value . kind && let hir :: ExprKind :: Let (..) = cond . kind { self . skip . insert (value . hir_id) ; return ; } if expr_parent_is_stmt (cx . tcx , expr . hir_id) && matches ! (expr . kind , hir :: ExprKind :: If (_cond , _conseq , None)) { return ; } self . probe_if_cascade (cx , expr) ; } }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (lint_if_let_rescope)] struct IfLetRescopeLint { # [subdiagnostic] destructors : Vec < DestructorLabel > , # [label] significant_droppers : Vec < Span > , # [help] lifetime_ends : Vec < Span > , # [subdiagnostic] rewrite : Option < IfLetRescopeRewrite > , }}}
mkitem!{mkstruct!{struct IfLetRescopeRewrite { match_heads : Vec < SingleArmMatchBegin > , consequent_heads : Vec < ConsequentRewrite > , closing_brackets : ClosingBrackets , alt_heads : Vec < AltHead > , }}}
mkitem!{mkimpl!{impl Subdiagnostic for IfLetRescopeRewrite { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { let mut suggestions = vec ! [] ; for match_head in self . match_heads { match match_head { SingleArmMatchBegin :: WithOpenBracket (span) => { suggestions . push ((span , "{ match " . into ())) } SingleArmMatchBegin :: WithoutOpenBracket (span) => { suggestions . push ((span , "match " . into ())) } } } for ConsequentRewrite { span , pat } in self . consequent_heads { suggestions . push ((span , format ! ("{{ {pat} => "))) ; } for AltHead (span) in self . alt_heads { suggestions . push ((span , " _ => " . into ())) ; } let closing_brackets = self . closing_brackets ; suggestions . push ((closing_brackets . span , closing_brackets . empty_alt . then_some (" _ => {}" . chars ()) . into_iter () . flatten () . chain (repeat ('}') . take (closing_brackets . count)) . collect () ,)) ; let msg = diag . eagerly_translate (crate :: fluent_generated :: lint_suggestion) ; diag . multipart_suggestion_with_style (msg , suggestions , Applicability :: MachineApplicable , SuggestionStyle :: ShowCode ,) ; } }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (lint_if_let_dtor)] struct DestructorLabel { # [primary_span] span : Span , dtor_kind : & 'static str , }}}
mkitem!{mkstruct!{struct AltHead (Span) ;}}
mkitem!{mkstruct!{struct ConsequentRewrite { span : Span , pat : String , }}}
mkitem!{mkstruct!{struct ClosingBrackets { span : Span , count : usize , empty_alt : bool , }}}
mkitem!{mkenum!{enum SingleArmMatchBegin { WithOpenBracket (Span) , WithoutOpenBracket (Span) , }}}
mkitem!{mkstruct!{struct FindSignificantDropper < 'a , 'tcx > { cx : & 'a LateContext < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > FindSignificantDropper < '_ , 'tcx > { # [doc = " Check the scrutinee of an `if let` to see if it promotes any temporary values"] # [doc = " that would change drop order in edition 2024. Specifically, it checks the value"] # [doc = " of the scrutinee itself, and also recurses into the expression to find any ref"] # [doc = " exprs (or autoref) which would promote temporaries that would be scoped to the"] # [doc = " end of this `if`."] fn check_if_let_scrutinee (& mut self , init : & 'tcx hir :: Expr < 'tcx > ,) -> ControlFlow < (Span , SmallVec < [Ty < 'tcx > ; 4] >) > { self . check_promoted_temp_with_drop (init) ? ; self . visit_expr (init) } # [doc = " Check that an expression is not a promoted temporary with a significant"] # [doc = " drop impl."] # [doc = ""] # [doc = " An expression is a promoted temporary if it has an addr taken (i.e. `&expr` or autoref)"] # [doc = " or is the scrutinee of the `if let`, *and* the expression is not a place"] # [doc = " expr, and it has a significant drop."] fn check_promoted_temp_with_drop (& self , expr : & 'tcx hir :: Expr < 'tcx > ,) -> ControlFlow < (Span , SmallVec < [Ty < 'tcx > ; 4] >) > { if expr . is_place_expr (| base | { self . cx . typeck_results () . adjustments () . get (base . hir_id) . is_some_and (| x | x . iter () . any (| adj | matches ! (adj . kind , Adjust :: Deref (_)))) }) { return ControlFlow :: Continue (()) ; } let drop_tys = extract_component_with_significant_dtor (self . cx . tcx , self . cx . typing_env () , self . cx . typeck_results () . expr_ty (expr) ,) ; if drop_tys . is_empty () { return ControlFlow :: Continue (()) ; } ControlFlow :: Break ((expr . span , drop_tys)) } }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for FindSignificantDropper < '_ , 'tcx > { type Result = ControlFlow < (Span , SmallVec < [Ty < 'tcx > ; 4] >) > ; fn visit_block (& mut self , b : & 'tcx hir :: Block < 'tcx >) -> Self :: Result { if let Some (expr) = b . expr { self . visit_expr (expr) } else { ControlFlow :: Continue (()) } } fn visit_expr (& mut self , expr : & 'tcx hir :: Expr < 'tcx >) -> Self :: Result { for adj in self . cx . typeck_results () . expr_adjustments (expr) { match adj . kind { Adjust :: Deref (_) => break , Adjust :: Borrow (_) => { self . check_promoted_temp_with_drop (expr) ? ; } _ => { } } } match expr . kind { hir :: ExprKind :: AddrOf (_ , _ , expr) => { self . check_promoted_temp_with_drop (expr) ? ; intravisit :: walk_expr (self , expr) } hir :: ExprKind :: Index (expr , _ , _) | hir :: ExprKind :: Field (expr , _) => { self . check_promoted_temp_with_drop (expr) ? ; intravisit :: walk_expr (self , expr) } hir :: ExprKind :: If (..) => ControlFlow :: Continue (()) , hir :: ExprKind :: Match (scrut , _ , _) => self . visit_expr (scrut) , hir :: ExprKind :: DropTemps (_) => ControlFlow :: Continue (()) , _ => intravisit :: walk_expr (self , expr) , } } }}}