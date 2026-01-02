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
mkuse!{use std :: borrow :: Cow ;}
mkuse!{use rustc_data_structures :: intern :: Interned ;}
mkuse!{use rustc_error_messages :: MultiSpan ;}
mkuse!{use rustc_macros :: { HashStable , TyDecodable , TyEncodable } ;}
mkuse!{use rustc_type_ir :: walk :: TypeWalker ;}
mkuse!{use rustc_type_ir :: { self as ir , TypeFlags , WithCachedTypeInfo } ;}
mkuse!{use crate :: ty :: { self , Ty , TyCtxt } ;}
mkmod!{int, { 
                getname!(int);
                getsrc!(int);
                getpath!(int);
                get_deps!(int);
                get_crates!(int);
                mkinclude!(int);
                 
            }}
mkmod!{kind, { 
                getname!(kind);
                getsrc!(kind);
                getpath!(kind);
                get_deps!(kind);
                get_crates!(kind);
                mkinclude!(kind);
                 
            }}
mkmod!{valtree, { 
                getname!(valtree);
                getsrc!(valtree);
                getpath!(valtree);
                get_deps!(valtree);
                get_crates!(valtree);
                mkinclude!(valtree);
                 
            }}
mkuse!{pub use int :: * ;}
mkuse!{pub use kind :: * ;}
mkuse!{use rustc_span :: { DUMMY_SP , ErrorGuaranteed } ;}
mkuse!{pub use valtree :: * ;}
mkitem!{pub type ConstKind < 'tcx > = ir :: ConstKind < TyCtxt < 'tcx > > ;}
mkitem!{pub type UnevaluatedConst < 'tcx > = ir :: UnevaluatedConst < TyCtxt < 'tcx > > ;}
mkitem!{# [cfg (target_pointer_width = "64")] rustc_data_structures :: static_assert_size ! (ConstKind <'_ >, 24) ;}
mkitem!{mkstruct!{# [derive (Copy , Clone , PartialEq , Eq , Hash , HashStable)] # [rustc_pass_by_value] pub struct Const < 'tcx > (pub (super) Interned < 'tcx , WithCachedTypeInfo < ConstKind < 'tcx > > >) ;}}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: inherent :: IntoKind for Const < 'tcx > { type Kind = ConstKind < 'tcx > ; fn kind (self) -> ConstKind < 'tcx > { self . kind () } }}}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: Flags for Const < 'tcx > { fn flags (& self) -> TypeFlags { self . 0 . flags } fn outer_exclusive_binder (& self) -> rustc_type_ir :: DebruijnIndex { self . 0 . outer_exclusive_binder } }}}
mkitem!{mkimpl!{impl < 'tcx > Const < 'tcx > { # [inline] pub fn kind (self) -> ConstKind < 'tcx > { let a : & ConstKind < 'tcx > = self . 0 . 0 ; * a } # [inline] pub fn flags (self) -> TypeFlags { self . 0 . flags } # [inline] pub fn outer_exclusive_binder (self) -> ty :: DebruijnIndex { self . 0 . outer_exclusive_binder } # [inline] pub fn new (tcx : TyCtxt < 'tcx > , kind : ty :: ConstKind < 'tcx >) -> Const < 'tcx > { tcx . mk_ct_from_kind (kind) } # [inline] pub fn new_param (tcx : TyCtxt < 'tcx > , param : ty :: ParamConst) -> Const < 'tcx > { Const :: new (tcx , ty :: ConstKind :: Param (param)) } # [inline] pub fn new_var (tcx : TyCtxt < 'tcx > , infer : ty :: ConstVid) -> Const < 'tcx > { Const :: new (tcx , ty :: ConstKind :: Infer (ty :: InferConst :: Var (infer))) } # [inline] pub fn new_fresh (tcx : TyCtxt < 'tcx > , fresh : u32) -> Const < 'tcx > { Const :: new (tcx , ty :: ConstKind :: Infer (ty :: InferConst :: Fresh (fresh))) } # [inline] pub fn new_infer (tcx : TyCtxt < 'tcx > , infer : ty :: InferConst) -> Const < 'tcx > { Const :: new (tcx , ty :: ConstKind :: Infer (infer)) } # [inline] pub fn new_bound (tcx : TyCtxt < 'tcx > , debruijn : ty :: DebruijnIndex , bound_const : ty :: BoundConst ,) -> Const < 'tcx > { Const :: new (tcx , ty :: ConstKind :: Bound (debruijn , bound_const)) } # [inline] pub fn new_placeholder (tcx : TyCtxt < 'tcx > , placeholder : ty :: PlaceholderConst) -> Const < 'tcx > { Const :: new (tcx , ty :: ConstKind :: Placeholder (placeholder)) } # [inline] pub fn new_unevaluated (tcx : TyCtxt < 'tcx > , uv : ty :: UnevaluatedConst < 'tcx >) -> Const < 'tcx > { tcx . debug_assert_args_compatible (uv . def , uv . args) ; Const :: new (tcx , ty :: ConstKind :: Unevaluated (uv)) } # [inline] pub fn new_value (tcx : TyCtxt < 'tcx > , valtree : ty :: ValTree < 'tcx > , ty : Ty < 'tcx >) -> Const < 'tcx > { Const :: new (tcx , ty :: ConstKind :: Value (ty :: Value { ty , valtree })) } # [inline] pub fn new_expr (tcx : TyCtxt < 'tcx > , expr : ty :: Expr < 'tcx >) -> Const < 'tcx > { Const :: new (tcx , ty :: ConstKind :: Expr (expr)) } # [inline] pub fn new_error (tcx : TyCtxt < 'tcx > , e : ty :: ErrorGuaranteed) -> Const < 'tcx > { Const :: new (tcx , ty :: ConstKind :: Error (e)) } # [doc = " Like [Ty::new_error] but for constants."] # [track_caller] pub fn new_misc_error (tcx : TyCtxt < 'tcx >) -> Const < 'tcx > { Const :: new_error_with_message (tcx , DUMMY_SP , "ty::ConstKind::Error constructed but no error reported" ,) } # [doc = " Like [Ty::new_error_with_message] but for constants."] # [track_caller] pub fn new_error_with_message < S : Into < MultiSpan > > (tcx : TyCtxt < 'tcx > , span : S , msg : impl Into < Cow < 'static , str > > ,) -> Const < 'tcx > { let reported = tcx . dcx () . span_delayed_bug (span , msg) ; Const :: new_error (tcx , reported) } pub fn is_trivially_wf (self) -> bool { match self . kind () { ty :: ConstKind :: Param (_) | ty :: ConstKind :: Placeholder (_) | ty :: ConstKind :: Bound (..) => { true } ty :: ConstKind :: Infer (_) | ty :: ConstKind :: Unevaluated (..) | ty :: ConstKind :: Value (_) | ty :: ConstKind :: Error (_) | ty :: ConstKind :: Expr (_) => false , } } }}}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: inherent :: Const < TyCtxt < 'tcx > > for Const < 'tcx > { fn new_infer (tcx : TyCtxt < 'tcx > , infer : ty :: InferConst) -> Self { Const :: new_infer (tcx , infer) } fn new_var (tcx : TyCtxt < 'tcx > , vid : ty :: ConstVid) -> Self { Const :: new_var (tcx , vid) } fn new_bound (interner : TyCtxt < 'tcx > , debruijn : ty :: DebruijnIndex , bound_const : ty :: BoundConst ,) -> Self { Const :: new_bound (interner , debruijn , bound_const) } fn new_anon_bound (tcx : TyCtxt < 'tcx > , debruijn : ty :: DebruijnIndex , var : ty :: BoundVar) -> Self { Const :: new_bound (tcx , debruijn , ty :: BoundConst { var }) } fn new_placeholder (tcx : TyCtxt < 'tcx > , placeholder : ty :: PlaceholderConst) -> Self { Const :: new_placeholder (tcx , placeholder) } fn new_unevaluated (interner : TyCtxt < 'tcx > , uv : ty :: UnevaluatedConst < 'tcx >) -> Self { Const :: new_unevaluated (interner , uv) } fn new_expr (interner : TyCtxt < 'tcx > , expr : ty :: Expr < 'tcx >) -> Self { Const :: new_expr (interner , expr) } fn new_error (interner : TyCtxt < 'tcx > , guar : ErrorGuaranteed) -> Self { Const :: new_error (interner , guar) } }}}
mkitem!{mkimpl!{impl < 'tcx > Const < 'tcx > { # [doc = " Creates a constant with the given integer value and interns it."] # [inline] pub fn from_bits (tcx : TyCtxt < 'tcx > , bits : u128 , typing_env : ty :: TypingEnv < 'tcx > , ty : Ty < 'tcx > ,) -> Self { let size = tcx . layout_of (typing_env . as_query_input (ty)) . unwrap_or_else (| e | panic ! ("could not compute layout for {ty:?}: {e:?}")) . size ; ty :: Const :: new_value (tcx , ty :: ValTree :: from_scalar_int (tcx , ScalarInt :: try_from_uint (bits , size) . unwrap ()) , ty ,) } # [inline] # [doc = " Creates an interned zst constant."] pub fn zero_sized (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx >) -> Self { ty :: Const :: new_value (tcx , ty :: ValTree :: zst (tcx) , ty) } # [inline] # [doc = " Creates an interned bool constant."] pub fn from_bool (tcx : TyCtxt < 'tcx > , v : bool) -> Self { Self :: from_bits (tcx , v as u128 , ty :: TypingEnv :: fully_monomorphized () , tcx . types . bool) } # [inline] # [doc = " Creates an interned usize constant."] pub fn from_target_usize (tcx : TyCtxt < 'tcx > , n : u64) -> Self { Self :: from_bits (tcx , n as u128 , ty :: TypingEnv :: fully_monomorphized () , tcx . types . usize) } # [doc = " Panics if `self.kind != ty::ConstKind::Value`."] pub fn to_value (self) -> ty :: Value < 'tcx > { match self . kind () { ty :: ConstKind :: Value (cv) => cv , _ => bug ! ("expected ConstKind::Value, got {:?}" , self . kind ()) , } } # [doc = " Attempts to convert to a value."] # [doc = ""] # [doc = " Note that this does not evaluate the constant."] pub fn try_to_value (self) -> Option < ty :: Value < 'tcx > > { match self . kind () { ty :: ConstKind :: Value (cv) => Some (cv) , _ => None , } } # [doc = " Convenience method to extract the value of a usize constant,"] # [doc = " useful to get the length of an array type."] # [doc = ""] # [doc = " Note that this does not evaluate the constant."] # [inline] pub fn try_to_target_usize (self , tcx : TyCtxt < 'tcx >) -> Option < u64 > { self . try_to_value () ? . try_to_target_usize (tcx) } pub fn is_ct_infer (self) -> bool { matches ! (self . kind () , ty :: ConstKind :: Infer (_)) } # [doc = " Iterator that walks `self` and any types reachable from"] # [doc = " `self`, in depth-first order. Note that just walks the types"] # [doc = " that appear in `self`, it does not descend into the fields of"] # [doc = " structs or variants. For example:"] # [doc = ""] # [doc = " ```text"] # [doc = " isize => { isize }"] # [doc = " Foo<Bar<isize>> => { Foo<Bar<isize>>, Bar<isize>, isize }"] # [doc = " [isize] => { [isize], isize }"] # [doc = " ```"] pub fn walk (self) -> TypeWalker < TyCtxt < 'tcx > > { TypeWalker :: new (self . into ()) } }}}
mkitem!{mkenum!{# [derive (Copy , Clone , PartialEq , Eq , Hash , Debug , TyEncodable , TyDecodable , HashStable)] pub enum AnonConstKind { # [doc = " `feature(generic_const_exprs)` anon consts are allowed to use arbitrary generic parameters in scope"] GCE , # [doc = " stable `min_const_generics` anon consts are not allowed to use any generic parameters"] MCG , # [doc = " anon consts used as the length of a repeat expr are syntactically allowed to use generic parameters"] # [doc = " but must not depend on the actual instantiation. See #76200 for more information"] RepeatExprCount , # [doc = " anon consts outside of the type system, e.g. enum discriminants"] NonTypeSystem , }}}