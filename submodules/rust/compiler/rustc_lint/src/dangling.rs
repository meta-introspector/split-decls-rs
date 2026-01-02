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
mkuse!{use rustc_ast :: visit :: { visit_opt , walk_list } ;}
mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_hir :: def :: Res ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_hir :: intravisit :: { FnKind , Visitor , walk_expr } ;}
mkuse!{use rustc_hir :: { Block , Body , Expr , ExprKind , FnDecl , FnRetTy , LangItem , TyKind , find_attr } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt } ;}
mkuse!{use rustc_session :: { declare_lint , impl_lint_pass } ;}
mkuse!{use rustc_span :: { Span , sym } ;}
mkuse!{use crate :: lints :: { DanglingPointersFromLocals , DanglingPointersFromTemporaries } ;}
mkuse!{use crate :: { LateContext , LateLintPass } ;}
mkitem!{declare_lint ! { #[doc = " The `dangling_pointers_from_temporaries` lint detects getting a pointer to data"] #[doc = " of a temporary that will immediately get dropped."] #[doc = ""] #[doc = " ### Example"] #[doc = ""] #[doc = " ```rust"] #[doc = " # #![allow(unused)]"] #[doc = " # unsafe fn use_data(ptr: *const u8) { }"] #[doc = " fn gather_and_use(bytes: impl Iterator<Item = u8>) {"] #[doc = "     let x: *const u8 = bytes.collect::<Vec<u8>>().as_ptr();"] #[doc = "     unsafe { use_data(x) }"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " {{produces}}"] #[doc = ""] #[doc = " ### Explanation"] #[doc = ""] #[doc = " Getting a pointer from a temporary value will not prolong its lifetime,"] #[doc = " which means that the value can be dropped and the allocation freed"] #[doc = " while the pointer still exists, making the pointer dangling."] #[doc = " This is not an error (as far as the type system is concerned)"] #[doc = " but probably is not what the user intended either."] #[doc = ""] #[doc = " If you need stronger guarantees, consider using references instead,"] #[doc = " as they are statically verified by the borrow-checker to never dangle."] pub DANGLING_POINTERS_FROM_TEMPORARIES , Warn , "detects getting a pointer from a temporary" }}
mkitem!{declare_lint ! { #[doc = " The `dangling_pointers_from_locals` lint detects getting a pointer to data"] #[doc = " of a local that will be dropped at the end of the function."] #[doc = ""] #[doc = " ### Example"] #[doc = ""] #[doc = " ```rust"] #[doc = " fn f() -> *const u8 {"] #[doc = "     let x = 0;"] #[doc = "     &x // returns a dangling ptr to `x`"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " {{produces}}"] #[doc = ""] #[doc = " ### Explanation"] #[doc = ""] #[doc = " Returning a pointer from a local value will not prolong its lifetime,"] #[doc = " which means that the value can be dropped and the allocation freed"] #[doc = " while the pointer still exists, making the pointer dangling."] #[doc = " This is not an error (as far as the type system is concerned)"] #[doc = " but probably is not what the user intended either."] #[doc = ""] #[doc = " If you need stronger guarantees, consider using references instead,"] #[doc = " as they are statically verified by the borrow-checker to never dangle."] pub DANGLING_POINTERS_FROM_LOCALS , Warn , "detects returning a pointer from a local variable" }}
mkitem!{mkstruct!{#[doc = " FIXME: false negatives (i.e. the lint is not emitted when it should be)"] #[doc = " 1. Ways to get a temporary that are not recognized:"] #[doc = "    - `owning_temporary.field`"] #[doc = "    - `owning_temporary[index]`"] #[doc = " 2. No checks for ref-to-ptr conversions:"] #[doc = "    - `&raw [mut] temporary`"] #[doc = "    - `&temporary as *(const|mut) _`"] #[doc = "    - `ptr::from_ref(&temporary)` and friends"] #[derive (Clone , Copy , Default)] pub (crate) struct DanglingPointers ;}}
mkitem!{impl_lint_pass ! (DanglingPointers => [DANGLING_POINTERS_FROM_TEMPORARIES , DANGLING_POINTERS_FROM_LOCALS]) ;}
mkitem!{mkimpl!{impl < 'tcx > LateLintPass < 'tcx > for DanglingPointers { fn check_fn (& mut self , cx : & LateContext < 'tcx > , fn_kind : FnKind < 'tcx > , fn_decl : & 'tcx FnDecl < 'tcx > , body : & 'tcx Body < 'tcx > , _ : Span , def_id : LocalDefId ,) { DanglingPointerSearcher { cx , inside_call_args : false } . visit_body (body) ; if let FnRetTy :: Return (ret_ty) = & fn_decl . output && let TyKind :: Ptr (_) = ret_ty . kind { let ty = match cx . tcx . type_of (def_id) . instantiate_identity () . kind () { ty :: FnDef (..) => cx . tcx . fn_sig (def_id) . instantiate_identity () , ty :: Closure (_ , args) => args . as_closure () . sig () , _ => return , } ; let ty = ty . output () ; let ty = cx . tcx . instantiate_bound_regions_with_erased (ty) ; let inner_ty = match ty . kind () { ty :: RawPtr (inner_ty , _) => * inner_ty , _ => return , } ; if cx . tcx . layout_of (cx . typing_env () . as_query_input (inner_ty)) . is_ok_and (| layout | ! layout . is_1zst ()) { let dcx = & DanglingPointerLocalContext { body : def_id , fn_ret : ty , fn_ret_span : ret_ty . span , fn_ret_inner : inner_ty , fn_kind : match fn_kind { FnKind :: ItemFn (..) => "function" , FnKind :: Method (..) => "method" , FnKind :: Closure => "closure" , } , } ; DanglingPointerReturnSearcher { cx , dcx } . visit_body (body) ; if let ExprKind :: Block (block , None) = & body . value . kind && let innermost_block = block . innermost_block () && let Some (expr) = innermost_block . expr { lint_addr_of_local (cx , dcx , expr) ; } } } } }}}
mkitem!{mkstruct!{struct DanglingPointerLocalContext < 'tcx > { body : LocalDefId , fn_ret : Ty < 'tcx > , fn_ret_span : Span , fn_ret_inner : Ty < 'tcx > , fn_kind : & 'static str , }}}
mkitem!{mkstruct!{struct DanglingPointerReturnSearcher < 'lcx , 'tcx > { cx : & 'lcx LateContext < 'tcx > , dcx : & 'lcx DanglingPointerLocalContext < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for DanglingPointerReturnSearcher < '_ , 'tcx > { fn visit_expr (& mut self , expr : & 'tcx Expr < 'tcx >) -> Self :: Result { if let ExprKind :: Ret (Some (expr)) = expr . kind { lint_addr_of_local (self . cx , self . dcx , expr) ; } walk_expr (self , expr) } }}}

macro_rules! lint_addr_of_local_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lint_addr_of_local in module {}", module_path!());
    };
}

mkfn!{
    lint_addr_of_local_introspect!();
    #[doc = " Look for `&<path_to_local_in_same_body>` pattern and emit lint for it"] fn lint_addr_of_local < 'a > (cx : & LateContext < 'a > , dcx : & DanglingPointerLocalContext < 'a > , expr : & 'a Expr < 'a > ,) { let (inner , _) = super :: utils :: peel_casts (cx , expr) ; if let ExprKind :: AddrOf (_ , _ , inner_of) = inner . kind && let ExprKind :: Path (ref qpath) = inner_of . peel_blocks () . kind && let Res :: Local (from) = cx . qpath_res (qpath , inner_of . hir_id) && cx . tcx . hir_enclosing_body_owner (from) == dcx . body { cx . tcx . emit_node_span_lint (DANGLING_POINTERS_FROM_LOCALS , expr . hir_id , expr . span , DanglingPointersFromLocals { ret_ty : dcx . fn_ret , ret_ty_span : dcx . fn_ret_span , fn_kind : dcx . fn_kind , local_var : cx . tcx . hir_span (from) , local_var_name : cx . tcx . hir_ident (from) , local_var_ty : dcx . fn_ret_inner , created_at : (expr . hir_id != inner . hir_id) . then_some (inner . span) , } ,) ; } }
}
mkitem!{mkstruct!{#[doc = " This produces a dangling pointer:"] #[doc = " ```ignore (example)"] #[doc = " let ptr = CString::new(\"hello\").unwrap().as_ptr();"] #[doc = " foo(ptr)"] #[doc = " ```"] #[doc = ""] #[doc = " But this does not:"] #[doc = " ```ignore (example)"] #[doc = " foo(CString::new(\"hello\").unwrap().as_ptr())"] #[doc = " ```"] #[doc = ""] #[doc = " But this does:"] #[doc = " ```ignore (example)"] #[doc = " foo({ let ptr = CString::new(\"hello\").unwrap().as_ptr(); ptr })"] #[doc = " ```"] #[doc = ""] #[doc = " So we have to keep track of when we are inside of a function/method call argument."] struct DanglingPointerSearcher < 'lcx , 'tcx > { cx : & 'lcx LateContext < 'tcx > , #[doc = " Keeps track of whether we are inside of function/method call arguments,"] #[doc = " where this lint should not be emitted."] #[doc = ""] #[doc = " See [the main doc][`Self`] for examples."] inside_call_args : bool , }}}
mkitem!{mkimpl!{impl Visitor < '_ > for DanglingPointerSearcher < '_ , '_ > { fn visit_expr (& mut self , expr : & Expr < '_ >) -> Self :: Result { if ! self . inside_call_args { lint_expr (self . cx , expr) } match expr . kind { ExprKind :: Call (lhs , args) | ExprKind :: MethodCall (_ , lhs , args , _) => { self . visit_expr (lhs) ; self . with_inside_call_args (true , | this | walk_list ! (this , visit_expr , args)) } ExprKind :: Block (& Block { stmts , expr , .. } , _) => { self . with_inside_call_args (false , | this | walk_list ! (this , visit_stmt , stmts)) ; visit_opt ! (self , visit_expr , expr) } _ => walk_expr (self , expr) , } } }}}
mkitem!{mkimpl!{impl DanglingPointerSearcher < '_ , '_ > { fn with_inside_call_args < R > (& mut self , inside_call_args : bool , callback : impl FnOnce (& mut Self) -> R ,) -> R { let old = core :: mem :: replace (& mut self . inside_call_args , inside_call_args) ; let result = callback (self) ; self . inside_call_args = old ; result } }}}

macro_rules! lint_expr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lint_expr in module {}", module_path!());
    };
}

mkfn!{
    lint_expr_introspect!();
    fn lint_expr (cx : & LateContext < '_ > , expr : & Expr < '_ >) { if let ExprKind :: MethodCall (method , receiver , _args , _span) = expr . kind && is_temporary_rvalue (receiver) && let ty = cx . typeck_results () . expr_ty (receiver) && owns_allocation (cx . tcx , ty) && let Some (fn_id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) && find_attr ! (cx . tcx . get_all_attrs (fn_id) , AttributeKind :: AsPtr (_)) { cx . tcx . emit_node_span_lint (DANGLING_POINTERS_FROM_TEMPORARIES , expr . hir_id , method . ident . span , DanglingPointersFromTemporaries { callee : method . ident , ty , ptr_span : method . ident . span , temporary_span : receiver . span , } ,) } }
}

macro_rules! is_temporary_rvalue_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_temporary_rvalue in module {}", module_path!());
    };
}

mkfn!{
    is_temporary_rvalue_introspect!();
    fn is_temporary_rvalue (expr : & Expr < '_ >) -> bool { match expr . kind { ExprKind :: ConstBlock (..) | ExprKind :: Repeat (..) | ExprKind :: Lit (..) => false , ExprKind :: Path (..) => false , ExprKind :: Call (..) | ExprKind :: MethodCall (..) | ExprKind :: Use (..) | ExprKind :: Binary (..) => true , ExprKind :: If (..) | ExprKind :: Loop (..) | ExprKind :: Match (..) | ExprKind :: Block (..) => true , ExprKind :: Index (..) | ExprKind :: Field (..) | ExprKind :: Unary (..) => false , ExprKind :: Struct (..) => true , ExprKind :: Array (..) => false , ExprKind :: Break (..) | ExprKind :: Continue (..) | ExprKind :: Ret (..) | ExprKind :: Become (..) => { false } ExprKind :: Assign (..) | ExprKind :: AssignOp (..) | ExprKind :: Yield (..) => false , ExprKind :: AddrOf (..) | ExprKind :: OffsetOf (..) | ExprKind :: InlineAsm (..) => false , ExprKind :: Cast (..) | ExprKind :: Closure (..) | ExprKind :: Tup (..) | ExprKind :: DropTemps (..) | ExprKind :: Let (..) => false , ExprKind :: UnsafeBinderCast (..) => false , ExprKind :: Type (..) | ExprKind :: Err (..) => false , } }
}

macro_rules! owns_allocation_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function owns_allocation in module {}", module_path!());
    };
}

mkfn!{
    owns_allocation_introspect!();
    fn owns_allocation (tcx : TyCtxt < '_ > , ty : Ty < '_ >) -> bool { if ty . is_array () { true } else if let Some (inner) = ty . boxed_ty () { inner . is_slice () || inner . is_str () || inner . ty_adt_def () . is_some_and (| def | tcx . is_lang_item (def . did () , LangItem :: CStr)) || owns_allocation (tcx , inner) } else if let Some (def) = ty . ty_adt_def () { for lang_item in [LangItem :: String , LangItem :: MaybeUninit , LangItem :: UnsafeCell] { if tcx . is_lang_item (def . did () , lang_item) { return true ; } } tcx . get_diagnostic_name (def . did ()) . is_some_and (| name | { matches ! (name , sym :: cstring_type | sym :: Vec | sym :: Cell | sym :: SyncUnsafeCell) }) } else { false } }
}