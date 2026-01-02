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
mkuse!{use rustc_abi :: FIRST_VARIANT ;}
mkuse!{use rustc_data_structures :: stack :: ensure_sufficient_stack ;}
mkuse!{use rustc_data_structures :: unord :: { UnordMap , UnordSet } ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: find_attr ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: { self , AdtDef , Instance , Ty , TyCtxt } ;}
mkuse!{use rustc_session :: declare_lint ;}
mkuse!{use rustc_span :: { Span , Symbol } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use crate :: lints :: { BuiltinClashingExtern , BuiltinClashingExternSub } ;}
mkuse!{use crate :: { LintVec , types } ;}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { clashing_extern_declarations , .. * providers } ; }
}

macro_rules! get_lints_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_lints in module {}", module_path!());
    };
}

mkfn!{
    get_lints_introspect!();
    pub (crate) fn get_lints () -> LintVec { vec ! [CLASHING_EXTERN_DECLARATIONS] }
}

macro_rules! clashing_extern_declarations_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clashing_extern_declarations in module {}", module_path!());
    };
}

mkfn!{
    clashing_extern_declarations_introspect!();
    fn clashing_extern_declarations (tcx : TyCtxt < '_ > , () : ()) { let mut lint = ClashingExternDeclarations :: new () ; for id in tcx . hir_crate_items (()) . foreign_items () { lint . check_foreign_item (tcx , id) ; } }
}
mkitem!{declare_lint ! { # [doc = " The `clashing_extern_declarations` lint detects when an `extern fn`"] # [doc = " has been declared with the same name but different types."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " mod m {"] # [doc = "     unsafe extern \"C\" {"] # [doc = "         fn foo();"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " unsafe extern \"C\" {"] # [doc = "     fn foo(_: u32);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Because two symbols of the same name cannot be resolved to two"] # [doc = " different functions at link time, and one function cannot possibly"] # [doc = " have two types, a clashing extern declaration is almost certainly a"] # [doc = " mistake. Check to make sure that the `extern` definitions are correct"] # [doc = " and equivalent, and possibly consider unifying them in one location."] # [doc = ""] # [doc = " This lint does not run between crates because a project may have"] # [doc = " dependencies which both rely on the same extern function, but declare"] # [doc = " it in a different (but valid) way. For example, they may both declare"] # [doc = " an opaque type for one or more of the arguments (which would end up"] # [doc = " distinct types), or use types that are valid conversions in the"] # [doc = " language the `extern fn` is defined in. In these cases, the compiler"] # [doc = " can't say that the clashing declaration is incorrect."] pub CLASHING_EXTERN_DECLARATIONS , Warn , "detects when an extern fn has been declared with the same name but different types" }}
mkitem!{mkstruct!{struct ClashingExternDeclarations { # [doc = " Map of function symbol name to the first-seen hir id for that symbol name.. If seen_decls"] # [doc = " contains an entry for key K, it means a symbol with name K has been seen by this lint and"] # [doc = " the symbol should be reported as a clashing declaration."] seen_decls : UnordMap < Symbol , hir :: OwnerId > , }}}
mkitem!{mkenum!{# [doc = " Differentiate between whether the name for an extern decl came from the link_name attribute or"] # [doc = " just from declaration itself. This is important because we don't want to report clashes on"] # [doc = " symbol name if they don't actually clash because one or the other links against a symbol with a"] # [doc = " different name."] enum SymbolName { # [doc = " The name of the symbol + the span of the annotation which introduced the link name."] Link (Symbol , Span) , # [doc = " No link name, so just the name of the symbol."] Normal (Symbol) , }}}
mkitem!{mkimpl!{impl SymbolName { fn get_name (& self) -> Symbol { match self { SymbolName :: Link (s , _) | SymbolName :: Normal (s) => * s , } } }}}
mkitem!{mkimpl!{impl ClashingExternDeclarations { pub (crate) fn new () -> Self { ClashingExternDeclarations { seen_decls : Default :: default () } } # [doc = " Insert a new foreign item into the seen set. If a symbol with the same name already exists"] # [doc = " for the item, return its HirId without updating the set."] fn insert (& mut self , tcx : TyCtxt < '_ > , fi : hir :: ForeignItemId) -> Option < hir :: OwnerId > { let did = fi . owner_id . to_def_id () ; let instance = Instance :: new_raw (did , ty :: List :: identity_for_item (tcx , did)) ; let name = Symbol :: intern (tcx . symbol_name (instance) . name) ; if let Some (& existing_id) = self . seen_decls . get (& name) { Some (existing_id) } else { self . seen_decls . insert (name , fi . owner_id) } } # [instrument (level = "trace" , skip (self , tcx))] fn check_foreign_item < 'tcx > (& mut self , tcx : TyCtxt < 'tcx > , this_fi : hir :: ForeignItemId) { let DefKind :: Fn = tcx . def_kind (this_fi . owner_id) else { return } ; let Some (existing_did) = self . insert (tcx , this_fi) else { return } ; let existing_decl_ty = tcx . type_of (existing_did) . skip_binder () ; let this_decl_ty = tcx . type_of (this_fi . owner_id) . instantiate_identity () ; debug ! ("ClashingExternDeclarations: Comparing existing {:?}: {:?} to this {:?}: {:?}" , existing_did , existing_decl_ty , this_fi . owner_id , this_decl_ty) ; if ! structurally_same_type (tcx , ty :: TypingEnv :: non_body_analysis (tcx , this_fi . owner_id) , existing_decl_ty , this_decl_ty ,) { let orig = name_of_extern_decl (tcx , existing_did) ; let this = tcx . item_name (this_fi . owner_id . to_def_id ()) ; let orig = orig . get_name () ; let previous_decl_label = get_relevant_span (tcx , existing_did) ; let mismatch_label = get_relevant_span (tcx , this_fi . owner_id) ; let sub = BuiltinClashingExternSub { tcx , expected : existing_decl_ty , found : this_decl_ty } ; let decorator = if orig == this { BuiltinClashingExtern :: SameName { this , orig , previous_decl_label , mismatch_label , sub , } } else { BuiltinClashingExtern :: DiffName { this , orig , previous_decl_label , mismatch_label , sub , } } ; tcx . emit_node_span_lint (CLASHING_EXTERN_DECLARATIONS , this_fi . hir_id () , mismatch_label , decorator ,) ; } } }}}

macro_rules! name_of_extern_decl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function name_of_extern_decl in module {}", module_path!());
    };
}

mkfn!{
    name_of_extern_decl_introspect!();
    # [doc = " Get the name of the symbol that's linked against for a given extern declaration. That is,"] # [doc = " the name specified in a #[link_name = ...] attribute if one was specified, else, just the"] # [doc = " symbol's name."] fn name_of_extern_decl (tcx : TyCtxt < '_ > , fi : hir :: OwnerId) -> SymbolName { if let Some ((overridden_link_name , overridden_link_name_span)) = tcx . codegen_fn_attrs (fi) . symbol_name . map (| overridden_link_name | { (overridden_link_name , find_attr ! (tcx . get_all_attrs (fi) , AttributeKind :: LinkName { span , .. } => * span) . unwrap () ,) }) { SymbolName :: Link (overridden_link_name , overridden_link_name_span) } else { SymbolName :: Normal (tcx . item_name (fi . to_def_id ())) } }
}

macro_rules! get_relevant_span_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_relevant_span in module {}", module_path!());
    };
}

mkfn!{
    get_relevant_span_introspect!();
    # [doc = " We want to ensure that we use spans for both decls that include where the"] # [doc = " name was defined, whether that was from the link_name attribute or not."] fn get_relevant_span (tcx : TyCtxt < '_ > , fi : hir :: OwnerId) -> Span { match name_of_extern_decl (tcx , fi) { SymbolName :: Normal (_) => tcx . def_span (fi) , SymbolName :: Link (_ , annot_span) => annot_span , } }
}

macro_rules! structurally_same_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function structurally_same_type in module {}", module_path!());
    };
}

mkfn!{
    structurally_same_type_introspect!();
    # [doc = " Checks whether two types are structurally the same enough that the declarations shouldn't"] # [doc = " clash. We need this so we don't emit a lint when two modules both declare an extern struct,"] # [doc = " with the same members (as the declarations shouldn't clash)."] fn structurally_same_type < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , a : Ty < 'tcx > , b : Ty < 'tcx > ,) -> bool { let mut seen_types = UnordSet :: default () ; let result = structurally_same_type_impl (& mut seen_types , tcx , typing_env , a , b) ; if cfg ! (debug_assertions) && result { let a_layout = tcx . layout_of (typing_env . as_query_input (a)) . unwrap () ; let b_layout = tcx . layout_of (typing_env . as_query_input (b)) . unwrap () ; assert_eq ! (a_layout . backend_repr , b_layout . backend_repr) ; assert_eq ! (a_layout . size , b_layout . size) ; assert_eq ! (a_layout . align , b_layout . align) ; } result }
}

macro_rules! structurally_same_type_impl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function structurally_same_type_impl in module {}", module_path!());
    };
}

mkfn!{
    structurally_same_type_impl_introspect!();
    fn structurally_same_type_impl < 'tcx > (seen_types : & mut UnordSet < (Ty < 'tcx > , Ty < 'tcx >) > , tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , a : Ty < 'tcx > , b : Ty < 'tcx > ,) -> bool { debug ! ("structurally_same_type_impl(tcx, a = {:?}, b = {:?})" , a , b) ; let non_transparent_ty = | mut ty : Ty < 'tcx > | -> Ty < 'tcx > { loop { if let ty :: Adt (def , args) = * ty . kind () { let is_transparent = def . repr () . transparent () ; let is_non_null = types :: nonnull_optimization_guaranteed (tcx , def) ; debug ! (? ty , is_transparent , is_non_null) ; if is_transparent && ! is_non_null { debug_assert_eq ! (def . variants () . len () , 1) ; let v = & def . variant (FIRST_VARIANT) ; if let Some (field) = types :: transparent_newtype_field (tcx , v) { ty = field . ty (tcx , args) ; continue ; } } } debug ! ("non_transparent_ty -> {:?}" , ty) ; return ty ; } } ; let a = non_transparent_ty (a) ; let b = non_transparent_ty (b) ; if ! seen_types . insert ((a , b)) { true } else if a == b { true } else { let is_primitive_or_pointer = | ty : Ty < 'tcx > | ty . is_primitive () || matches ! (ty . kind () , ty :: RawPtr (..) | ty :: Ref (..)) ; ensure_sufficient_stack (| | { match (a . kind () , b . kind ()) { (& ty :: Adt (a_def , a_gen_args) , & ty :: Adt (b_def , b_gen_args)) => { if ! (a_def . repr () . c () && b_def . repr () . c ()) { return false ; } let repr_characteristica = | def : AdtDef < 'tcx > | (def . repr () . pack , def . repr () . align , def . repr () . simd ()) ; if repr_characteristica (a_def) != repr_characteristica (b_def) { return false ; } let a_fields = a_def . variants () . iter () . flat_map (| v | v . fields . iter ()) ; let b_fields = b_def . variants () . iter () . flat_map (| v | v . fields . iter ()) ; a_fields . eq_by (b_fields , | & ty :: FieldDef { did : a_did , .. } , & ty :: FieldDef { did : b_did , .. } | { structurally_same_type_impl (seen_types , tcx , typing_env , tcx . type_of (a_did) . instantiate (tcx , a_gen_args) , tcx . type_of (b_did) . instantiate (tcx , b_gen_args) ,) } ,) } (ty :: Array (a_ty , a_len) , ty :: Array (b_ty , b_len)) => { a_len == b_len && structurally_same_type_impl (seen_types , tcx , typing_env , * a_ty , * b_ty) } (ty :: Slice (a_ty) , ty :: Slice (b_ty)) => { structurally_same_type_impl (seen_types , tcx , typing_env , * a_ty , * b_ty) } (ty :: RawPtr (a_ty , a_mutbl) , ty :: RawPtr (b_ty , b_mutbl)) => { a_mutbl == b_mutbl && structurally_same_type_impl (seen_types , tcx , typing_env , * a_ty , * b_ty) } (ty :: Ref (_a_region , a_ty , a_mut) , ty :: Ref (_b_region , b_ty , b_mut)) => { a_mut == b_mut && structurally_same_type_impl (seen_types , tcx , typing_env , * a_ty , * b_ty) } (ty :: FnDef (..) , ty :: FnDef (..)) => { let a_poly_sig = a . fn_sig (tcx) ; let b_poly_sig = b . fn_sig (tcx) ; let a_sig = tcx . instantiate_bound_regions_with_erased (a_poly_sig) ; let b_sig = tcx . instantiate_bound_regions_with_erased (b_poly_sig) ; (a_sig . abi , a_sig . safety , a_sig . c_variadic) == (b_sig . abi , b_sig . safety , b_sig . c_variadic) && a_sig . inputs () . iter () . eq_by (b_sig . inputs () . iter () , | a , b | { structurally_same_type_impl (seen_types , tcx , typing_env , * a , * b) }) && structurally_same_type_impl (seen_types , tcx , typing_env , a_sig . output () , b_sig . output () ,) } (ty :: Tuple (..) , ty :: Tuple (..)) => { false } (ty :: Dynamic (..) , ty :: Dynamic (..)) | (ty :: Error (..) , ty :: Error (..)) | (ty :: Closure (..) , ty :: Closure (..)) | (ty :: Coroutine (..) , ty :: Coroutine (..)) | (ty :: CoroutineWitness (..) , ty :: CoroutineWitness (..)) | (ty :: Alias (ty :: Projection , ..) , ty :: Alias (ty :: Projection , ..)) | (ty :: Alias (ty :: Inherent , ..) , ty :: Alias (ty :: Inherent , ..)) | (ty :: Alias (ty :: Opaque , ..) , ty :: Alias (ty :: Opaque , ..)) => false , (ty :: Bool , ty :: Bool) | (ty :: Char , ty :: Char) | (ty :: Never , ty :: Never) | (ty :: Str , ty :: Str) => unreachable ! () , (ty :: Adt (..) | ty :: Pat (..) , _) if is_primitive_or_pointer (b) => { if let Some (a_inner) = types :: repr_nullable_ptr (tcx , typing_env , a) { a_inner == b } else { false } } (_ , ty :: Adt (..) | ty :: Pat (..)) if is_primitive_or_pointer (a) => { if let Some (b_inner) = types :: repr_nullable_ptr (tcx , typing_env , b) { b_inner == a } else { false } } _ => false , } }) } }
}