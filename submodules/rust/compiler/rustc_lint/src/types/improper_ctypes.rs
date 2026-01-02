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
mkuse!{use std :: iter ;}
mkuse!{use std :: ops :: ControlFlow ;}
mkuse!{use bitflags :: bitflags ;}
mkuse!{use rustc_abi :: VariantIdx ;}
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_errors :: DiagMessage ;}
mkuse!{use rustc_hir :: def :: CtorKind ;}
mkuse!{use rustc_hir :: intravisit :: VisitorExt ;}
mkuse!{use rustc_hir :: { self as hir , AmbigArg } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: { self , Adt , AdtDef , AdtKind , GenericArgsRef , Ty , TyCtxt , TypeSuperVisitable , TypeVisitable , TypeVisitableExt , } ;}
mkuse!{use rustc_session :: { declare_lint , declare_lint_pass } ;}
mkuse!{use rustc_span :: def_id :: LocalDefId ;}
mkuse!{use rustc_span :: { Span , sym } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use super :: repr_nullable_ptr ;}
mkuse!{use crate :: lints :: { ImproperCTypes , UsesPowerAlignment } ;}
mkuse!{use crate :: { LateContext , LateLintPass , LintContext , fluent_generated as fluent } ;}
mkitem!{declare_lint ! { # [doc = " The `improper_ctypes` lint detects incorrect use of types in foreign"] # [doc = " modules."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " unsafe extern \"C\" {"] # [doc = "     static STATIC: String;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " The compiler has several checks to verify that types used in `extern`"] # [doc = " blocks are safe and follow certain rules to ensure proper"] # [doc = " compatibility with the foreign interfaces. This lint is issued when it"] # [doc = " detects a probable mistake in a definition. The lint usually should"] # [doc = " provide a description of the issue, along with possibly a hint on how"] # [doc = " to resolve it."] IMPROPER_CTYPES , Warn , "proper use of libc types in foreign modules" }}
mkitem!{declare_lint ! { # [doc = " The `improper_ctypes_definitions` lint detects incorrect use of"] # [doc = " [`extern` function] definitions."] # [doc = ""] # [doc = " [`extern` function]: https://doc.rust-lang.org/reference/items/functions.html#extern-function-qualifier"] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #![allow(unused)]"] # [doc = " pub extern \"C\" fn str_type(p: &str) { }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " There are many parameter and return types that may be specified in an"] # [doc = " `extern` function that are not compatible with the given ABI. This"] # [doc = " lint is an alert that these types should not be used. The lint usually"] # [doc = " should provide a description of the issue, along with possibly a hint"] # [doc = " on how to resolve it."] IMPROPER_CTYPES_DEFINITIONS , Warn , "proper use of libc types in foreign item definitions" }}
mkitem!{declare_lint ! { # [doc = " The `uses_power_alignment` lint detects specific `repr(C)`"] # [doc = " aggregates on AIX."] # [doc = " In its platform C ABI, AIX uses the \"power\" (as in PowerPC) alignment"] # [doc = " rule (detailed in https://www.ibm.com/docs/en/xl-c-and-cpp-aix/16.1?topic=data-using-alignment-modes#alignment),"] # [doc = " which can also be set for XLC by `#pragma align(power)` or"] # [doc = " `-qalign=power`. Aggregates with a floating-point type as the"] # [doc = " recursively first field (as in \"at offset 0\") modify the layout of"] # [doc = " *subsequent* fields of the associated structs to use an alignment value"] # [doc = " where the floating-point type is aligned on a 4-byte boundary."] # [doc = ""] # [doc = " Effectively, subsequent floating-point fields act as-if they are `repr(packed(4))`. This"] # [doc = " would be unsound to do in a `repr(C)` type without all the restrictions that come with"] # [doc = " `repr(packed)`. Rust instead chooses a layout that maintains soundness of Rust code, at the"] # [doc = " expense of incompatibility with C code."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,ignore (fails on non-powerpc64-ibm-aix)"] # [doc = " #[repr(C)]"] # [doc = " pub struct Floats {"] # [doc = "     a: f64,"] # [doc = "     b: u8,"] # [doc = "     c: f64,"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " This will produce:"] # [doc = ""] # [doc = " ```text"] # [doc = " warning: repr(C) does not follow the power alignment rule. This may affect platform C ABI compatibility for this type"] # [doc = "  --> <source>:5:3"] # [doc = "   |"] # [doc = " 5 |   c: f64,"] # [doc = "   |   ^^^^^^"] # [doc = "   |"] # [doc = "   = note: `#[warn(uses_power_alignment)]` on by default"] # [doc = " ```"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " The power alignment rule specifies that the above struct has the"] # [doc = " following alignment:"] # [doc = "  - offset_of!(Floats, a) == 0"] # [doc = "  - offset_of!(Floats, b) == 8"] # [doc = "  - offset_of!(Floats, c) == 12"] # [doc = ""] # [doc = " However, Rust currently aligns `c` at `offset_of!(Floats, c) == 16`."] # [doc = " Using offset 12 would be unsound since `f64` generally must be 8-aligned on this target."] # [doc = " Thus, a warning is produced for the above struct."] USES_POWER_ALIGNMENT , Warn , "Structs do not follow the power alignment rule under repr(C)" }}
mkitem!{declare_lint_pass ! (ImproperCTypesLint => [IMPROPER_CTYPES , IMPROPER_CTYPES_DEFINITIONS , USES_POWER_ALIGNMENT]) ;}

macro_rules! check_non_exhaustive_variant_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_non_exhaustive_variant in module {}", module_path!());
    };
}

mkfn!{
    check_non_exhaustive_variant_introspect!();
    # [doc = " Check a variant of a non-exhaustive enum for improper ctypes"] # [doc = ""] # [doc = " We treat `#[non_exhaustive] enum` as \"ensure that code will compile if new variants are added\"."] # [doc = " This includes linting, on a best-effort basis. There are valid additions that are unlikely."] # [doc = ""] # [doc = " Adding a data-carrying variant to an existing C-like enum that is passed to C is \"unlikely\","] # [doc = " so we don't need the lint to account for it."] # [doc = " e.g. going from enum Foo { A, B, C } to enum Foo { A, B, C, D(u32) }."] pub (crate) fn check_non_exhaustive_variant (non_exhaustive_variant_list : bool , variant : & ty :: VariantDef ,) -> ControlFlow < DiagMessage , () > { if non_exhaustive_variant_list { if variant_has_complex_ctor (variant) { return ControlFlow :: Break (fluent :: lint_improper_ctypes_non_exhaustive) ; } } if variant . field_list_has_applicable_non_exhaustive () { return ControlFlow :: Break (fluent :: lint_improper_ctypes_non_exhaustive_variant) ; } ControlFlow :: Continue (()) }
}

macro_rules! variant_has_complex_ctor_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function variant_has_complex_ctor in module {}", module_path!());
    };
}

mkfn!{
    variant_has_complex_ctor_introspect!();
    fn variant_has_complex_ctor (variant : & ty :: VariantDef) -> bool { ! matches ! (variant . ctor_kind () , Some (CtorKind :: Const)) }
}

macro_rules! check_arg_for_power_alignment_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_arg_for_power_alignment in module {}", module_path!());
    };
}

mkfn!{
    check_arg_for_power_alignment_introspect!();
    # [doc = " Per-struct-field function that checks if a struct definition follows"] # [doc = " the Power alignment Rule (see the `check_struct_for_power_alignment` function)."] fn check_arg_for_power_alignment < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { let tcx = cx . tcx ; assert ! (tcx . sess . target . os == "aix") ; if ty . is_floating_point () && ty . primitive_size (tcx) . bytes () > 4 { return true ; } else if let Adt (adt_def , _) = ty . kind () && adt_def . is_struct () && adt_def . repr () . c () && ! adt_def . repr () . packed () && adt_def . repr () . align . is_none () { let struct_variant = adt_def . variant (VariantIdx :: ZERO) ; for struct_field in & struct_variant . fields { let field_ty = tcx . type_of (struct_field . did) . instantiate_identity () ; if check_arg_for_power_alignment (cx , field_ty) { return true ; } } } return false ; }
}

macro_rules! check_struct_for_power_alignment_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_struct_for_power_alignment in module {}", module_path!());
    };
}

mkfn!{
    check_struct_for_power_alignment_introspect!();
    # [doc = " Check a struct definition for respect of the Power alignment Rule (as in PowerPC),"] # [doc = " which should be respected in the \"aix\" target OS."] # [doc = " To do so, we must follow one of the two following conditions:"] # [doc = " - The first field of the struct must be floating-point type that"] # [doc = "    is greater than 4-bytes."] # [doc = "  - The first field of the struct must be an aggregate whose"] # [doc = "    recursively first field is a floating-point type greater than"] # [doc = "    4 bytes."] fn check_struct_for_power_alignment < 'tcx > (cx : & LateContext < 'tcx > , item : & 'tcx hir :: Item < 'tcx > , adt_def : AdtDef < 'tcx > ,) { let tcx = cx . tcx ; if adt_def . repr () . c () && ! adt_def . repr () . packed () && adt_def . repr () . align . is_none () && tcx . sess . target . os == "aix" && ! adt_def . all_fields () . next () . is_none () { let struct_variant_data = item . expect_struct () . 2 ; for field_def in struct_variant_data . fields () . iter () . skip (1) { let def_id = field_def . def_id ; let ty = tcx . type_of (def_id) . instantiate_identity () ; if check_arg_for_power_alignment (cx , ty) { cx . emit_span_lint (USES_POWER_ALIGNMENT , field_def . span , UsesPowerAlignment) ; } } } }
}
mkitem!{mkenum!{# [derive (Clone , Copy)] enum CItemKind { Declaration , Definition , }}}
mkitem!{mkenum!{enum FfiResult < 'tcx > { FfiSafe , FfiPhantom (Ty < 'tcx >) , FfiUnsafe { ty : Ty < 'tcx > , reason : DiagMessage , help : Option < DiagMessage > } , }}}
mkitem!{# [doc = " The result when a type has been checked but perhaps not completely. `None` indicates that"] # [doc = " FFI safety/unsafety has not yet been determined, `Some(res)` indicates that the safety/unsafety"] # [doc = " in the `FfiResult` is final."] type PartialFfiResult < 'tcx > = Option < FfiResult < 'tcx > > ;}
mkitem!{bitflags ! { # [derive (Clone , Copy , Debug , PartialEq , Eq)] struct VisitorState : u8 { # [doc = " For use in (externally-linked) static variables."] const STATIC = 0b000001 ; # [doc = " For use in functions in general."] const FUNC = 0b000010 ; # [doc = " For variables in function returns (implicitly: not for static variables)."] const FN_RETURN = 0b000100 ; # [doc = " For variables in functions/variables which are defined in rust."] const DEFINED = 0b001000 ; # [doc = " For times where we are only defining the type of something"] # [doc = " (struct/enum/union definitions, FnPtrs)."] const THEORETICAL = 0b010000 ; } }}
mkitem!{mkimpl!{impl VisitorState { const STATIC_TY : Self = Self :: STATIC ; const ARGUMENT_TY_IN_DEFINITION : Self = Self :: from_bits (Self :: FUNC . bits () | Self :: DEFINED . bits ()) . unwrap () ; const RETURN_TY_IN_DEFINITION : Self = Self :: from_bits (Self :: FUNC . bits () | Self :: FN_RETURN . bits () | Self :: DEFINED . bits ()) . unwrap () ; const ARGUMENT_TY_IN_DECLARATION : Self = Self :: FUNC ; const RETURN_TY_IN_DECLARATION : Self = Self :: from_bits (Self :: FUNC . bits () | Self :: FN_RETURN . bits ()) . unwrap () ; const ARGUMENT_TY_IN_FNPTR : Self = Self :: from_bits (Self :: FUNC . bits () | Self :: THEORETICAL . bits ()) . unwrap () ; const RETURN_TY_IN_FNPTR : Self = Self :: from_bits (Self :: FUNC . bits () | Self :: THEORETICAL . bits () | Self :: FN_RETURN . bits ()) . unwrap () ; # [doc = " Get the proper visitor state for a given function's arguments."] fn argument_from_fnmode (fn_mode : CItemKind) -> Self { match fn_mode { CItemKind :: Definition => VisitorState :: ARGUMENT_TY_IN_DEFINITION , CItemKind :: Declaration => VisitorState :: ARGUMENT_TY_IN_DECLARATION , } } # [doc = " Get the proper visitor state for a given function's return type."] fn return_from_fnmode (fn_mode : CItemKind) -> Self { match fn_mode { CItemKind :: Definition => VisitorState :: RETURN_TY_IN_DEFINITION , CItemKind :: Declaration => VisitorState :: RETURN_TY_IN_DECLARATION , } } # [doc = " Whether the type is used in a function."] fn is_in_function (self) -> bool { let ret = self . contains (Self :: FUNC) ; if ret { debug_assert ! (! self . contains (Self :: STATIC)) ; } ret } # [doc = " Whether the type is used (directly or not) in a function, in return position."] fn is_in_function_return (self) -> bool { let ret = self . contains (Self :: FN_RETURN) ; if ret { debug_assert ! (self . is_in_function ()) ; } ret } # [doc = " Whether the type is used (directly or not) in a defined function."] # [doc = " In other words, whether or not we allow non-FFI-safe types behind a C pointer,"] # [doc = " to be treated as an opaque type on the other side of the FFI boundary."] fn is_in_defined_function (self) -> bool { self . contains (Self :: DEFINED) && self . is_in_function () } # [doc = " Whether the type is used (directly or not) in a function pointer type."] # [doc = " Here, we also allow non-FFI-safe types behind a C pointer,"] # [doc = " to be treated as an opaque type on the other side of the FFI boundary."] fn is_in_fnptr (self) -> bool { self . contains (Self :: THEORETICAL) && self . is_in_function () } # [doc = " Whether we can expect type parameters and co in a given type."] fn can_expect_ty_params (self) -> bool { self . contains (Self :: THEORETICAL) || self . is_in_defined_function () } }}}
mkitem!{mkstruct!{# [doc = " Visitor used to recursively traverse MIR types and evaluate FFI-safety."] # [doc = " It uses ``check_*`` methods as entrypoints to be called elsewhere,"] # [doc = " and ``visit_*`` methods to recurse."] struct ImproperCTypesVisitor < 'a , 'tcx > { cx : & 'a LateContext < 'tcx > , # [doc = " To prevent problems with recursive types,"] # [doc = " add a types-in-check cache."] cache : FxHashSet < Ty < 'tcx > > , # [doc = " The original type being checked, before we recursed"] # [doc = " to any other types it contains."] base_ty : Ty < 'tcx > , base_fn_mode : CItemKind , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > ImproperCTypesVisitor < 'a , 'tcx > { fn new (cx : & 'a LateContext < 'tcx > , base_ty : Ty < 'tcx > , base_fn_mode : CItemKind) -> Self { Self { cx , base_ty , base_fn_mode , cache : FxHashSet :: default () } } # [doc = " Checks if the given field's type is \"ffi-safe\"."] fn check_field_type_for_ffi (& mut self , state : VisitorState , field : & ty :: FieldDef , args : GenericArgsRef < 'tcx > ,) -> FfiResult < 'tcx > { let field_ty = field . ty (self . cx . tcx , args) ; let field_ty = self . cx . tcx . try_normalize_erasing_regions (self . cx . typing_env () , field_ty) . unwrap_or (field_ty) ; self . visit_type (state , field_ty) } # [doc = " Checks if the given `VariantDef`'s field types are \"ffi-safe\"."] fn check_variant_for_ffi (& mut self , state : VisitorState , ty : Ty < 'tcx > , def : ty :: AdtDef < 'tcx > , variant : & ty :: VariantDef , args : GenericArgsRef < 'tcx > ,) -> FfiResult < 'tcx > { use FfiResult :: * ; let transparent_with_all_zst_fields = if def . repr () . transparent () { if let Some (field) = super :: transparent_newtype_field (self . cx . tcx , variant) { match self . check_field_type_for_ffi (state , field , args) { FfiUnsafe { ty , .. } if ty . is_unit () => () , r => return r , } false } else { true } } else { false } ; let mut all_phantom = ! variant . fields . is_empty () ; for field in & variant . fields { all_phantom &= match self . check_field_type_for_ffi (state , field , args) { FfiSafe => false , FfiUnsafe { ty , .. } if ty . is_unit () => false , FfiPhantom (..) => true , r @ FfiUnsafe { .. } => return r , } } if all_phantom { FfiPhantom (ty) } else if transparent_with_all_zst_fields { FfiUnsafe { ty , reason : fluent :: lint_improper_ctypes_struct_zst , help : None } } else { FfiSafe } } # [doc = " Checks if the given type is \"ffi-safe\" (has a stable, well-defined"] # [doc = " representation which can be exported to C code)."] fn visit_type (& mut self , state : VisitorState , ty : Ty < 'tcx >) -> FfiResult < 'tcx > { use FfiResult :: * ; let tcx = self . cx . tcx ; if ! self . cache . insert (ty) { return FfiSafe ; } match * ty . kind () { ty :: Adt (def , args) => { if let Some (boxed) = ty . boxed_ty () && (state . is_in_defined_function () || (state . is_in_fnptr () && matches ! (self . base_fn_mode , CItemKind :: Definition))) { if boxed . is_sized (tcx , self . cx . typing_env ()) { return FfiSafe ; } else { return FfiUnsafe { ty , reason : fluent :: lint_improper_ctypes_box , help : None , } ; } } if def . is_phantom_data () { return FfiPhantom (ty) ; } match def . adt_kind () { AdtKind :: Struct | AdtKind :: Union => { if let Some (sym :: cstring_type | sym :: cstr_type) = tcx . get_diagnostic_name (def . did ()) && ! self . base_ty . is_mutable_ptr () { return FfiUnsafe { ty , reason : fluent :: lint_improper_ctypes_cstr_reason , help : Some (fluent :: lint_improper_ctypes_cstr_help) , } ; } if ! def . repr () . c () && ! def . repr () . transparent () { return FfiUnsafe { ty , reason : if def . is_struct () { fluent :: lint_improper_ctypes_struct_layout_reason } else { fluent :: lint_improper_ctypes_union_layout_reason } , help : if def . is_struct () { Some (fluent :: lint_improper_ctypes_struct_layout_help) } else { Some (fluent :: lint_improper_ctypes_union_layout_help) } , } ; } if def . non_enum_variant () . field_list_has_applicable_non_exhaustive () { return FfiUnsafe { ty , reason : if def . is_struct () { fluent :: lint_improper_ctypes_struct_non_exhaustive } else { fluent :: lint_improper_ctypes_union_non_exhaustive } , help : None , } ; } if def . non_enum_variant () . fields . is_empty () { return FfiUnsafe { ty , reason : if def . is_struct () { fluent :: lint_improper_ctypes_struct_fieldless_reason } else { fluent :: lint_improper_ctypes_union_fieldless_reason } , help : if def . is_struct () { Some (fluent :: lint_improper_ctypes_struct_fieldless_help) } else { Some (fluent :: lint_improper_ctypes_union_fieldless_help) } , } ; } self . check_variant_for_ffi (state , ty , def , def . non_enum_variant () , args) } AdtKind :: Enum => { if def . variants () . is_empty () { return FfiSafe ; } if ! def . repr () . c () && ! def . repr () . transparent () && def . repr () . int . is_none () { if let Some (ty) = repr_nullable_ptr (self . cx . tcx , self . cx . typing_env () , ty) { return self . visit_type (state , ty) ; } return FfiUnsafe { ty , reason : fluent :: lint_improper_ctypes_enum_repr_reason , help : Some (fluent :: lint_improper_ctypes_enum_repr_help) , } ; } let non_exhaustive = def . variant_list_has_applicable_non_exhaustive () ; let ret = def . variants () . iter () . try_for_each (| variant | { check_non_exhaustive_variant (non_exhaustive , variant) . map_break (| reason | FfiUnsafe { ty , reason , help : None }) ? ; match self . check_variant_for_ffi (state , ty , def , variant , args) { FfiSafe => ControlFlow :: Continue (()) , r => ControlFlow :: Break (r) , } }) ; if let ControlFlow :: Break (result) = ret { return result ; } FfiSafe } } } ty :: Char => FfiUnsafe { ty , reason : fluent :: lint_improper_ctypes_char_reason , help : Some (fluent :: lint_improper_ctypes_char_help) , } , ty :: Pat (base , ..) => self . visit_type (state , base) , ty :: Bool | ty :: Int (..) | ty :: Uint (..) | ty :: Float (..) | ty :: Never => FfiSafe , ty :: Slice (_) => FfiUnsafe { ty , reason : fluent :: lint_improper_ctypes_slice_reason , help : Some (fluent :: lint_improper_ctypes_slice_help) , } , ty :: Dynamic (..) => { FfiUnsafe { ty , reason : fluent :: lint_improper_ctypes_dyn , help : None } } ty :: Str => FfiUnsafe { ty , reason : fluent :: lint_improper_ctypes_str_reason , help : Some (fluent :: lint_improper_ctypes_str_help) , } , ty :: Tuple (..) => FfiUnsafe { ty , reason : fluent :: lint_improper_ctypes_tuple_reason , help : Some (fluent :: lint_improper_ctypes_tuple_help) , } , ty :: RawPtr (ty , _) | ty :: Ref (_ , ty , _) if { (state . is_in_defined_function () || state . is_in_fnptr ()) && ty . is_sized (self . cx . tcx , self . cx . typing_env ()) } => { FfiSafe } ty :: RawPtr (ty , _) if match ty . kind () { ty :: Tuple (tuple) => tuple . is_empty () , _ => false , } => { FfiSafe } ty :: RawPtr (ty , _) | ty :: Ref (_ , ty , _) => self . visit_type (state , ty) , ty :: Array (inner_ty , _) => self . visit_type (state , inner_ty) , ty :: FnPtr (sig_tys , hdr) => { let sig = sig_tys . with (hdr) ; if sig . abi () . is_rustic_abi () { return FfiUnsafe { ty , reason : fluent :: lint_improper_ctypes_fnptr_reason , help : Some (fluent :: lint_improper_ctypes_fnptr_help) , } ; } let sig = tcx . instantiate_bound_regions_with_erased (sig) ; for arg in sig . inputs () { match self . visit_type (VisitorState :: ARGUMENT_TY_IN_FNPTR , * arg) { FfiSafe => { } r => return r , } } let ret_ty = sig . output () ; if ret_ty . is_unit () { return FfiSafe ; } self . visit_type (VisitorState :: RETURN_TY_IN_FNPTR , ret_ty) } ty :: Foreign (..) => FfiSafe , ty :: Alias (ty :: Opaque , ..) => { FfiUnsafe { ty , reason : fluent :: lint_improper_ctypes_opaque , help : None } } ty :: Param (..) | ty :: Alias (ty :: Projection | ty :: Inherent , ..) if state . can_expect_ty_params () => { FfiSafe } ty :: UnsafeBinder (_) => todo ! ("FIXME(unsafe_binder)") , ty :: Param (..) | ty :: Alias (ty :: Projection | ty :: Inherent | ty :: Free , ..) | ty :: Infer (..) | ty :: Bound (..) | ty :: Error (_) | ty :: Closure (..) | ty :: CoroutineClosure (..) | ty :: Coroutine (..) | ty :: CoroutineWitness (..) | ty :: Placeholder (..) | ty :: FnDef (..) => bug ! ("unexpected type in foreign function: {:?}" , ty) , } } fn visit_for_opaque_ty (& mut self , ty : Ty < 'tcx >) -> PartialFfiResult < 'tcx > { struct ProhibitOpaqueTypes ; impl < 'tcx > ty :: TypeVisitor < TyCtxt < 'tcx > > for ProhibitOpaqueTypes { type Result = ControlFlow < Ty < 'tcx > > ; fn visit_ty (& mut self , ty : Ty < 'tcx >) -> Self :: Result { if ! ty . has_opaque_types () { return ControlFlow :: Continue (()) ; } if let ty :: Alias (ty :: Opaque , ..) = ty . kind () { ControlFlow :: Break (ty) } else { ty . super_visit_with (self) } } } if let Some (ty) = self . cx . tcx . try_normalize_erasing_regions (self . cx . typing_env () , ty) . unwrap_or (ty) . visit_with (& mut ProhibitOpaqueTypes) . break_value () { Some (FfiResult :: FfiUnsafe { ty , reason : fluent :: lint_improper_ctypes_opaque , help : None , }) } else { None } } # [doc = " Check if the type is array and emit an unsafe type lint."] fn check_for_array_ty (& mut self , ty : Ty < 'tcx >) -> PartialFfiResult < 'tcx > { if let ty :: Array (..) = ty . kind () { Some (FfiResult :: FfiUnsafe { ty , reason : fluent :: lint_improper_ctypes_array_reason , help : Some (fluent :: lint_improper_ctypes_array_help) , }) } else { None } } # [doc = " Determine the FFI-safety of a single (MIR) type, given the context of how it is used."] fn check_type (& mut self , state : VisitorState , ty : Ty < 'tcx >) -> FfiResult < 'tcx > { if let Some (res) = self . visit_for_opaque_ty (ty) { return res ; } let ty = self . cx . tcx . try_normalize_erasing_regions (self . cx . typing_env () , ty) . unwrap_or (ty) ; if state . is_in_function () { if let Some (res) = self . check_for_array_ty (ty) { return res ; } } if state . is_in_function_return () && ty . is_unit () { return FfiResult :: FfiSafe ; } self . visit_type (state , ty) } }}}
mkitem!{mkimpl!{impl < 'tcx > ImproperCTypesLint { # [doc = " Find any fn-ptr types with external ABIs in `ty`, and FFI-checks them."] # [doc = " For example, `Option<extern \"C\" fn()>` FFI-checks `extern \"C\" fn()`."] fn check_type_for_external_abi_fnptr (& mut self , cx : & LateContext < 'tcx > , state : VisitorState , hir_ty : & hir :: Ty < 'tcx > , ty : Ty < 'tcx > , fn_mode : CItemKind ,) { struct FnPtrFinder < 'tcx > { spans : Vec < Span > , tys : Vec < Ty < 'tcx > > , } impl < 'tcx > hir :: intravisit :: Visitor < '_ > for FnPtrFinder < 'tcx > { fn visit_ty (& mut self , ty : & '_ hir :: Ty < '_ , AmbigArg >) { debug ! (? ty) ; if let hir :: TyKind :: FnPtr (hir :: FnPtrTy { abi , .. }) = ty . kind && ! abi . is_rustic_abi () { self . spans . push (ty . span) ; } hir :: intravisit :: walk_ty (self , ty) } } impl < 'tcx > ty :: TypeVisitor < TyCtxt < 'tcx > > for FnPtrFinder < 'tcx > { type Result = () ; fn visit_ty (& mut self , ty : Ty < 'tcx >) -> Self :: Result { if let ty :: FnPtr (_ , hdr) = ty . kind () && ! hdr . abi . is_rustic_abi () { self . tys . push (ty) ; } ty . super_visit_with (self) } } let mut visitor = FnPtrFinder { spans : Vec :: new () , tys : Vec :: new () } ; ty . visit_with (& mut visitor) ; visitor . visit_ty_unambig (hir_ty) ; let all_types = iter :: zip (visitor . tys . drain (..) , visitor . spans . drain (..)) ; for (fn_ptr_ty , span) in all_types { let mut visitor = ImproperCTypesVisitor :: new (cx , fn_ptr_ty , fn_mode) ; let ffi_res = visitor . check_type (state , fn_ptr_ty) ; self . process_ffi_result (cx , span , ffi_res , fn_mode) ; } } # [doc = " Regardless of a function's need to be \"ffi-safe\", look for fn-ptr argument/return types"] # [doc = " that need to be checked for ffi-safety."] fn check_fn_for_external_abi_fnptr (& mut self , cx : & LateContext < 'tcx > , fn_mode : CItemKind , def_id : LocalDefId , decl : & 'tcx hir :: FnDecl < '_ > ,) { let sig = cx . tcx . fn_sig (def_id) . instantiate_identity () ; let sig = cx . tcx . instantiate_bound_regions_with_erased (sig) ; for (input_ty , input_hir) in iter :: zip (sig . inputs () , decl . inputs) { let state = VisitorState :: argument_from_fnmode (fn_mode) ; self . check_type_for_external_abi_fnptr (cx , state , input_hir , * input_ty , fn_mode) ; } if let hir :: FnRetTy :: Return (ret_hir) = decl . output { let state = VisitorState :: return_from_fnmode (fn_mode) ; self . check_type_for_external_abi_fnptr (cx , state , ret_hir , sig . output () , fn_mode) ; } } # [doc = " For a local definition of a #[repr(C)] struct/enum/union, check that it is indeed FFI-safe."] fn check_reprc_adt (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx hir :: Item < 'tcx > , adt_def : AdtDef < 'tcx > ,) { debug_assert ! (adt_def . repr () . c () && ! adt_def . repr () . packed () && adt_def . repr () . align . is_none ()) ; check_struct_for_power_alignment (cx , item , adt_def) ; } fn check_foreign_static (& mut self , cx : & LateContext < 'tcx > , id : hir :: OwnerId , span : Span) { let ty = cx . tcx . type_of (id) . instantiate_identity () ; let mut visitor = ImproperCTypesVisitor :: new (cx , ty , CItemKind :: Declaration) ; let ffi_res = visitor . check_type (VisitorState :: STATIC_TY , ty) ; self . process_ffi_result (cx , span , ffi_res , CItemKind :: Declaration) ; } # [doc = " Check if a function's argument types and result type are \"ffi-safe\"."] fn check_foreign_fn (& mut self , cx : & LateContext < 'tcx > , fn_mode : CItemKind , def_id : LocalDefId , decl : & 'tcx hir :: FnDecl < '_ > ,) { let sig = cx . tcx . fn_sig (def_id) . instantiate_identity () ; let sig = cx . tcx . instantiate_bound_regions_with_erased (sig) ; for (input_ty , input_hir) in iter :: zip (sig . inputs () , decl . inputs) { let state = VisitorState :: argument_from_fnmode (fn_mode) ; let mut visitor = ImproperCTypesVisitor :: new (cx , * input_ty , fn_mode) ; let ffi_res = visitor . check_type (state , * input_ty) ; self . process_ffi_result (cx , input_hir . span , ffi_res , fn_mode) ; } if let hir :: FnRetTy :: Return (ret_hir) = decl . output { let state = VisitorState :: return_from_fnmode (fn_mode) ; let mut visitor = ImproperCTypesVisitor :: new (cx , sig . output () , fn_mode) ; let ffi_res = visitor . check_type (state , sig . output ()) ; self . process_ffi_result (cx , ret_hir . span , ffi_res , fn_mode) ; } } fn process_ffi_result (& self , cx : & LateContext < 'tcx > , sp : Span , res : FfiResult < 'tcx > , fn_mode : CItemKind ,) { match res { FfiResult :: FfiSafe => { } FfiResult :: FfiPhantom (ty) => { self . emit_ffi_unsafe_type_lint (cx , ty , sp , fluent :: lint_improper_ctypes_only_phantomdata , None , fn_mode ,) ; } FfiResult :: FfiUnsafe { ty , reason , help } => { self . emit_ffi_unsafe_type_lint (cx , ty , sp , reason , help , fn_mode) ; } } } fn emit_ffi_unsafe_type_lint (& self , cx : & LateContext < 'tcx > , ty : Ty < 'tcx > , sp : Span , note : DiagMessage , help : Option < DiagMessage > , fn_mode : CItemKind ,) { let lint = match fn_mode { CItemKind :: Declaration => IMPROPER_CTYPES , CItemKind :: Definition => IMPROPER_CTYPES_DEFINITIONS , } ; let desc = match fn_mode { CItemKind :: Declaration => "block" , CItemKind :: Definition => "fn" , } ; let span_note = if let ty :: Adt (def , _) = ty . kind () && let Some (sp) = cx . tcx . hir_span_if_local (def . did ()) { Some (sp) } else { None } ; cx . emit_span_lint (lint , sp , ImproperCTypes { ty , desc , label : sp , help , note , span_note }) ; } }}}
mkitem!{mkimpl!{# [doc = " `ImproperCTypesDefinitions` checks items outside of foreign items (e.g. stuff that isn't in"] # [doc = " `extern \"C\" { }` blocks):"] # [doc = ""] # [doc = " - `extern \"<abi>\" fn` definitions are checked in the same way as the"] # [doc = "   `ImproperCtypesDeclarations` visitor checks functions if `<abi>` is external (e.g. \"C\")."] # [doc = " - All other items which contain types (e.g. other functions, struct definitions, etc) are"] # [doc = "   checked for extern fn-ptrs with external ABIs."] impl < 'tcx > LateLintPass < 'tcx > for ImproperCTypesLint { fn check_foreign_item (& mut self , cx : & LateContext < 'tcx > , it : & hir :: ForeignItem < 'tcx >) { let abi = cx . tcx . hir_get_foreign_abi (it . hir_id ()) ; match it . kind { hir :: ForeignItemKind :: Fn (sig , _ , _) => { if ! abi . is_rustic_abi () { self . check_foreign_fn (cx , CItemKind :: Declaration , it . owner_id . def_id , sig . decl) ; } else { self . check_fn_for_external_abi_fnptr (cx , CItemKind :: Declaration , it . owner_id . def_id , sig . decl ,) ; } } hir :: ForeignItemKind :: Static (ty , _ , _) if ! abi . is_rustic_abi () => { self . check_foreign_static (cx , it . owner_id , ty . span) ; } hir :: ForeignItemKind :: Static (..) | hir :: ForeignItemKind :: Type => () , } } fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx hir :: Item < 'tcx >) { match item . kind { hir :: ItemKind :: Static (_ , _ , ty , _) | hir :: ItemKind :: Const (_ , _ , ty , _) | hir :: ItemKind :: TyAlias (_ , _ , ty) => { self . check_type_for_external_abi_fnptr (cx , VisitorState :: STATIC_TY , ty , cx . tcx . type_of (item . owner_id) . instantiate_identity () , CItemKind :: Definition ,) ; } hir :: ItemKind :: Fn { .. } => { } hir :: ItemKind :: Struct (..) | hir :: ItemKind :: Union (..) | hir :: ItemKind :: Enum (..) => { let adt_def : AdtDef < 'tcx > = cx . tcx . adt_def (item . owner_id . to_def_id ()) ; if adt_def . repr () . c () && ! adt_def . repr () . packed () && adt_def . repr () . align . is_none () { self . check_reprc_adt (cx , item , adt_def) ; } } hir :: ItemKind :: Impl (..) | hir :: ItemKind :: TraitAlias (..) | hir :: ItemKind :: Trait (..) | hir :: ItemKind :: GlobalAsm { .. } | hir :: ItemKind :: ForeignMod { .. } | hir :: ItemKind :: Mod (..) | hir :: ItemKind :: Macro (..) | hir :: ItemKind :: Use (..) | hir :: ItemKind :: ExternCrate (..) => { } } } fn check_field_def (& mut self , cx : & LateContext < 'tcx > , field : & 'tcx hir :: FieldDef < 'tcx >) { self . check_type_for_external_abi_fnptr (cx , VisitorState :: STATIC_TY , field . ty , cx . tcx . type_of (field . def_id) . instantiate_identity () , CItemKind :: Definition ,) ; } fn check_fn (& mut self , cx : & LateContext < 'tcx > , kind : hir :: intravisit :: FnKind < 'tcx > , decl : & 'tcx hir :: FnDecl < '_ > , _ : & 'tcx hir :: Body < '_ > , _ : Span , id : LocalDefId ,) { use hir :: intravisit :: FnKind ; let abi = match kind { FnKind :: ItemFn (_ , _ , header , ..) => header . abi , FnKind :: Method (_ , sig , ..) => sig . header . abi , _ => return , } ; if ! abi . is_rustic_abi () { self . check_foreign_fn (cx , CItemKind :: Definition , id , decl) ; } else { self . check_fn_for_external_abi_fnptr (cx , CItemKind :: Definition , id , decl) ; } } }}}