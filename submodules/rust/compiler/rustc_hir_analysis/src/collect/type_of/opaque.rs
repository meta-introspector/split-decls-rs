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
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_hir :: { self as hir , Expr , ImplItem , Item , Node , TraitItem , def , intravisit } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: hir :: nested_filter ;}
mkuse!{use rustc_middle :: ty :: { self , DefiningScopeKind , Ty , TyCtxt , TypeVisitableExt } ;}
mkuse!{use rustc_trait_selection :: opaque_types :: report_item_does_not_constrain_error ;}
mkuse!{use tracing :: { debug , instrument , trace } ;}
mkuse!{use crate :: errors :: UnconstrainedOpaqueType ;}

macro_rules! find_opaque_ty_constraints_for_impl_trait_in_assoc_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_opaque_ty_constraints_for_impl_trait_in_assoc_type in module {}", module_path!());
    };
}

mkfn!{
    find_opaque_ty_constraints_for_impl_trait_in_assoc_type_introspect!();
    # [doc = " Checks \"defining uses\" of opaque `impl Trait` in associated types."] # [doc = " These can only be defined by associated items of the same trait."] # [instrument (skip (tcx) , level = "debug")] pub (super) fn find_opaque_ty_constraints_for_impl_trait_in_assoc_type (tcx : TyCtxt < '_ > , def_id : LocalDefId , opaque_types_from : DefiningScopeKind ,) -> Ty < '_ > { let mut parent_def_id = def_id ; while tcx . def_kind (parent_def_id) == def :: DefKind :: OpaqueTy { parent_def_id = tcx . local_parent (parent_def_id) ; } let impl_def_id = tcx . local_parent (parent_def_id) ; match tcx . def_kind (impl_def_id) { DefKind :: Impl { .. } => { } other => bug ! ("invalid impl trait in assoc type parent: {other:?}") , } let mut locator = TaitConstraintLocator { def_id , tcx , found : None , opaque_types_from } ; for & assoc_id in tcx . associated_item_def_ids (impl_def_id) { let assoc = tcx . associated_item (assoc_id) ; match assoc . kind { ty :: AssocKind :: Const { .. } | ty :: AssocKind :: Fn { .. } => { locator . check (assoc_id . expect_local ()) } ty :: AssocKind :: Type { .. } => { } } } if let Some (hidden) = locator . found { hidden . ty } else { let guar = tcx . dcx () . emit_err (UnconstrainedOpaqueType { span : tcx . def_span (def_id) , name : tcx . item_ident (parent_def_id . to_def_id ()) , what : "impl" , }) ; Ty :: new_error (tcx , guar) } }
}

macro_rules! find_opaque_ty_constraints_for_tait_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_opaque_ty_constraints_for_tait in module {}", module_path!());
    };
}

mkfn!{
    find_opaque_ty_constraints_for_tait_introspect!();
    # [doc = " Checks \"defining uses\" of opaque `impl Trait` types to ensure that they meet the restrictions"] # [doc = " laid for \"higher-order pattern unification\"."] # [doc = " This ensures that inference is tractable."] # [doc = " In particular, definitions of opaque types can only use other generics as arguments,"] # [doc = " and they cannot repeat an argument. Example:"] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = " type Foo<A, B> = impl Bar<A, B>;"] # [doc = ""] # [doc = " // Okay -- `Foo` is applied to two distinct, generic types."] # [doc = " fn a<T, U>() -> Foo<T, U> { .. }"] # [doc = ""] # [doc = " // Not okay -- `Foo` is applied to `T` twice."] # [doc = " fn b<T>() -> Foo<T, T> { .. }"] # [doc = ""] # [doc = " // Not okay -- `Foo` is applied to a non-generic type."] # [doc = " fn b<T>() -> Foo<T, u32> { .. }"] # [doc = " ```"] # [instrument (skip (tcx) , level = "debug")] pub (super) fn find_opaque_ty_constraints_for_tait (tcx : TyCtxt < '_ > , def_id : LocalDefId , opaque_types_from : DefiningScopeKind ,) -> Ty < '_ > { let mut locator = TaitConstraintLocator { def_id , tcx , found : None , opaque_types_from } ; tcx . hir_walk_toplevel_module (& mut locator) ; if let Some (hidden) = locator . found { hidden . ty } else { let mut parent_def_id = def_id ; while tcx . def_kind (parent_def_id) == def :: DefKind :: OpaqueTy { parent_def_id = tcx . local_parent (parent_def_id) ; } let guar = tcx . dcx () . emit_err (UnconstrainedOpaqueType { span : tcx . def_span (def_id) , name : tcx . item_ident (parent_def_id . to_def_id ()) , what : "crate" , }) ; Ty :: new_error (tcx , guar) } }
}
mkitem!{mkstruct!{struct TaitConstraintLocator < 'tcx > { tcx : TyCtxt < 'tcx > , # [doc = " def_id of the opaque type whose defining uses are being checked"] def_id : LocalDefId , # [doc = " as we walk the defining uses, we are checking that all of them"] # [doc = " define the same hidden type. This variable is set to `Some`"] # [doc = " with the first type that we find, and then later types are"] # [doc = " checked against it (we also carry the span of that first"] # [doc = " type)."] found : Option < ty :: OpaqueHiddenType < 'tcx > > , opaque_types_from : DefiningScopeKind , }}}
mkitem!{mkimpl!{impl < 'tcx > TaitConstraintLocator < 'tcx > { fn insert_found (& mut self , hidden_ty : ty :: OpaqueHiddenType < 'tcx >) { if let Some (prev) = & mut self . found { if hidden_ty . ty != prev . ty { let (Ok (guar) | Err (guar)) = prev . build_mismatch_error (& hidden_ty , self . tcx) . map (| d | d . emit ()) ; prev . ty = Ty :: new_error (self . tcx , guar) ; } } else { self . found = Some (hidden_ty) ; } } fn non_defining_use_in_defining_scope (& mut self , item_def_id : LocalDefId) { assert ! (! self . tcx . next_trait_solver_globally ()) ; let guar = report_item_does_not_constrain_error (self . tcx , item_def_id , self . def_id , None) ; self . insert_found (ty :: OpaqueHiddenType :: new_error (self . tcx , guar)) ; } # [instrument (skip (self) , level = "debug")] fn check (& mut self , item_def_id : LocalDefId) { let tcx = self . tcx ; if ! tcx . has_typeck_results (item_def_id) { debug ! ("no constraint: no typeck results") ; return ; } let opaque_types_defined_by = tcx . opaque_types_defined_by (item_def_id) ; if ! opaque_types_defined_by . contains (& self . def_id) { debug ! ("no constraint: no opaque types defined") ; return ; } let hir_node = tcx . hir_node_by_def_id (item_def_id) ; debug_assert ! (! matches ! (hir_node , Node :: ForeignItem (..)) , "foreign items cannot constrain opaque types" ,) ; if let Some (hir_sig) = hir_node . fn_sig () && hir_sig . decl . output . is_suggestable_infer_ty () . is_some () { let guar = self . tcx . dcx () . span_delayed_bug (hir_sig . decl . output . span () , "inferring return types and opaque types do not mix well" ,) ; self . found = Some (ty :: OpaqueHiddenType :: new_error (tcx , guar)) ; return ; } match self . opaque_types_from { DefiningScopeKind :: HirTypeck => { let tables = tcx . typeck (item_def_id) ; if let Some (guar) = tables . tainted_by_errors { self . insert_found (ty :: OpaqueHiddenType :: new_error (tcx , guar)) ; } else if let Some (& hidden_type) = tables . concrete_opaque_types . get (& self . def_id) { self . insert_found (hidden_type) ; } else { self . non_defining_use_in_defining_scope (item_def_id) ; } } DefiningScopeKind :: MirBorrowck => match tcx . mir_borrowck (item_def_id) { Err (guar) => self . insert_found (ty :: OpaqueHiddenType :: new_error (tcx , guar)) , Ok (concrete_opaque_types) => { if let Some (& hidden_type) = concrete_opaque_types . 0 . get (& self . def_id) { debug ! (? hidden_type , "found constraint") ; self . insert_found (hidden_type) ; } else if let Err (guar) = tcx . type_of_opaque_hir_typeck (self . def_id) . instantiate_identity () . error_reported () { self . insert_found (ty :: OpaqueHiddenType :: new_error (tcx , guar)) ; } else { self . non_defining_use_in_defining_scope (item_def_id) ; } } } , } } }}}
mkitem!{mkimpl!{impl < 'tcx > intravisit :: Visitor < 'tcx > for TaitConstraintLocator < 'tcx > { type NestedFilter = nested_filter :: All ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_expr (& mut self , ex : & 'tcx Expr < 'tcx >) { intravisit :: walk_expr (self , ex) ; } fn visit_item (& mut self , it : & 'tcx Item < 'tcx >) { trace ! (? it . owner_id) ; self . check (it . owner_id . def_id) ; intravisit :: walk_item (self , it) ; } fn visit_impl_item (& mut self , it : & 'tcx ImplItem < 'tcx >) { trace ! (? it . owner_id) ; self . check (it . owner_id . def_id) ; intravisit :: walk_impl_item (self , it) ; } fn visit_trait_item (& mut self , it : & 'tcx TraitItem < 'tcx >) { trace ! (? it . owner_id) ; self . check (it . owner_id . def_id) ; intravisit :: walk_trait_item (self , it) ; } fn visit_foreign_item (& mut self , it : & 'tcx hir :: ForeignItem < 'tcx >) { trace ! (? it . owner_id) ; assert_ne ! (it . owner_id . def_id , self . def_id) ; intravisit :: walk_foreign_item (self , it) ; } }}}

macro_rules! find_opaque_ty_constraints_for_rpit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_opaque_ty_constraints_for_rpit in module {}", module_path!());
    };
}

mkfn!{
    find_opaque_ty_constraints_for_rpit_introspect!();
    pub (super) fn find_opaque_ty_constraints_for_rpit < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId , owner_def_id : LocalDefId , opaque_types_from : DefiningScopeKind ,) -> Ty < 'tcx > { match opaque_types_from { DefiningScopeKind :: HirTypeck => { let tables = tcx . typeck (owner_def_id) ; if let Some (guar) = tables . tainted_by_errors { Ty :: new_error (tcx , guar) } else if let Some (hidden_ty) = tables . concrete_opaque_types . get (& def_id) { hidden_ty . ty } else { assert ! (! tcx . next_trait_solver_globally ()) ; Ty :: new_diverging_default (tcx) } } DefiningScopeKind :: MirBorrowck => match tcx . mir_borrowck (owner_def_id) { Ok (concrete_opaque_types) => { if let Some (hidden_ty) = concrete_opaque_types . 0 . get (& def_id) { hidden_ty . ty } else { let hir_ty = tcx . type_of_opaque_hir_typeck (def_id) . instantiate_identity () ; if let Err (guar) = hir_ty . error_reported () { Ty :: new_error (tcx , guar) } else { assert ! (! tcx . next_trait_solver_globally ()) ; hir_ty } } } Err (guar) => Ty :: new_error (tcx , guar) , } , } }
}