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
mkuse!{use rustc_abi :: FieldIdx ;}
mkuse!{use rustc_ast :: InlineAsmTemplatePiece ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexSet ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_hir :: { self as hir , LangItem } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: { self , Article , FloatTy , IntTy , Ty , TyCtxt , TypeVisitableExt , UintTy } ;}
mkuse!{use rustc_session :: lint ;}
mkuse!{use rustc_span :: def_id :: LocalDefId ;}
mkuse!{use rustc_span :: { Span , Symbol , sym } ;}
mkuse!{use rustc_target :: asm :: { InlineAsmReg , InlineAsmRegClass , InlineAsmRegOrRegClass , InlineAsmType , ModifierInfo , } ;}
mkuse!{use rustc_trait_selection :: infer :: InferCtxtExt ;}
mkuse!{use crate :: FnCtxt ;}
mkuse!{use crate :: errors :: RegisterTypeUnstable ;}
mkitem!{mkstruct!{pub (crate) struct InlineAsmCtxt < 'a , 'tcx > { target_features : & 'tcx FxIndexSet < Symbol > , fcx : & 'a FnCtxt < 'a , 'tcx > , }}}
mkitem!{mkenum!{enum NonAsmTypeReason < 'tcx > { UnevaluatedSIMDArrayLength (DefId , ty :: Const < 'tcx >) , Invalid (Ty < 'tcx >) , InvalidElement (DefId , Ty < 'tcx >) , NotSizedPtr (Ty < 'tcx >) , EmptySIMDArray (Ty < 'tcx >) , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > InlineAsmCtxt < 'a , 'tcx > { pub (crate) fn new (fcx : & 'a FnCtxt < 'a , 'tcx > , def_id : LocalDefId) -> Self { InlineAsmCtxt { target_features : fcx . tcx . asm_target_features (def_id) , fcx } } fn tcx (& self) -> TyCtxt < 'tcx > { self . fcx . tcx } fn expr_ty (& self , expr : & hir :: Expr < 'tcx >) -> Ty < 'tcx > { let ty = self . fcx . typeck_results . borrow () . expr_ty_adjusted (expr) ; let ty = self . fcx . try_structurally_resolve_type (expr . span , ty) ; if ty . has_non_region_infer () { Ty :: new_misc_error (self . tcx ()) } else { self . tcx () . erase_and_anonymize_regions (ty) } } fn is_thin_ptr_ty (& self , span : Span , ty : Ty < 'tcx >) -> bool { if self . fcx . type_is_sized_modulo_regions (self . fcx . param_env , ty) { return true ; } if let ty :: Foreign (..) = self . fcx . try_structurally_resolve_type (span , ty) . kind () { return true ; } false } fn get_asm_ty (& self , span : Span , ty : Ty < 'tcx > ,) -> Result < InlineAsmType , NonAsmTypeReason < 'tcx > > { let asm_ty_isize = match self . tcx () . sess . target . pointer_width { 16 => InlineAsmType :: I16 , 32 => InlineAsmType :: I32 , 64 => InlineAsmType :: I64 , width => bug ! ("unsupported pointer width: {width}") , } ; match * ty . kind () { ty :: Int (IntTy :: I8) | ty :: Uint (UintTy :: U8) => Ok (InlineAsmType :: I8) , ty :: Int (IntTy :: I16) | ty :: Uint (UintTy :: U16) => Ok (InlineAsmType :: I16) , ty :: Int (IntTy :: I32) | ty :: Uint (UintTy :: U32) => Ok (InlineAsmType :: I32) , ty :: Int (IntTy :: I64) | ty :: Uint (UintTy :: U64) => Ok (InlineAsmType :: I64) , ty :: Int (IntTy :: I128) | ty :: Uint (UintTy :: U128) => Ok (InlineAsmType :: I128) , ty :: Int (IntTy :: Isize) | ty :: Uint (UintTy :: Usize) => Ok (asm_ty_isize) , ty :: Float (FloatTy :: F16) => Ok (InlineAsmType :: F16) , ty :: Float (FloatTy :: F32) => Ok (InlineAsmType :: F32) , ty :: Float (FloatTy :: F64) => Ok (InlineAsmType :: F64) , ty :: Float (FloatTy :: F128) => Ok (InlineAsmType :: F128) , ty :: FnPtr (..) => Ok (asm_ty_isize) , ty :: RawPtr (elem_ty , _) => { if self . is_thin_ptr_ty (span , elem_ty) { Ok (asm_ty_isize) } else { Err (NonAsmTypeReason :: NotSizedPtr (ty)) } } ty :: Adt (adt , args) if adt . repr () . simd () => { let fields = & adt . non_enum_variant () . fields ; if fields . is_empty () { return Err (NonAsmTypeReason :: EmptySIMDArray (ty)) ; } let field = & fields [FieldIdx :: ZERO] ; let elem_ty = field . ty (self . tcx () , args) ; let (size , ty) = match * elem_ty . kind () { ty :: Array (ty , len) => { let len = if self . fcx . next_trait_solver () { self . fcx . try_structurally_resolve_const (span , len) } else { self . fcx . tcx . normalize_erasing_regions (self . fcx . typing_env (self . fcx . param_env) , len ,) } ; if let Some (len) = len . try_to_target_usize (self . tcx ()) { (len , ty) } else { return Err (NonAsmTypeReason :: UnevaluatedSIMDArrayLength (field . did , len ,)) ; } } _ => (fields . len () as u64 , elem_ty) , } ; match ty . kind () { ty :: Int (IntTy :: I8) | ty :: Uint (UintTy :: U8) => Ok (InlineAsmType :: VecI8 (size)) , ty :: Int (IntTy :: I16) | ty :: Uint (UintTy :: U16) => Ok (InlineAsmType :: VecI16 (size)) , ty :: Int (IntTy :: I32) | ty :: Uint (UintTy :: U32) => Ok (InlineAsmType :: VecI32 (size)) , ty :: Int (IntTy :: I64) | ty :: Uint (UintTy :: U64) => Ok (InlineAsmType :: VecI64 (size)) , ty :: Int (IntTy :: I128) | ty :: Uint (UintTy :: U128) => { Ok (InlineAsmType :: VecI128 (size)) } ty :: Int (IntTy :: Isize) | ty :: Uint (UintTy :: Usize) => { Ok (match self . tcx () . sess . target . pointer_width { 16 => InlineAsmType :: VecI16 (size) , 32 => InlineAsmType :: VecI32 (size) , 64 => InlineAsmType :: VecI64 (size) , width => bug ! ("unsupported pointer width: {width}") , }) } ty :: Float (FloatTy :: F16) => Ok (InlineAsmType :: VecF16 (size)) , ty :: Float (FloatTy :: F32) => Ok (InlineAsmType :: VecF32 (size)) , ty :: Float (FloatTy :: F64) => Ok (InlineAsmType :: VecF64 (size)) , ty :: Float (FloatTy :: F128) => Ok (InlineAsmType :: VecF128 (size)) , _ => Err (NonAsmTypeReason :: InvalidElement (field . did , ty)) , } } ty :: Infer (_) => bug ! ("unexpected infer ty in asm operand") , _ => Err (NonAsmTypeReason :: Invalid (ty)) , } } fn check_asm_operand_type (& self , idx : usize , reg : InlineAsmRegOrRegClass , expr : & 'tcx hir :: Expr < 'tcx > , template : & [InlineAsmTemplatePiece] , is_input : bool , tied_input : Option < (& 'tcx hir :: Expr < 'tcx > , Option < InlineAsmType >) > ,) -> Option < InlineAsmType > { let ty = self . expr_ty (expr) ; if ty . has_non_region_infer () { bug ! ("inference variable in asm operand ty: {:?} {:?}" , expr , ty) ; } let asm_ty = match * ty . kind () { ty :: Never if is_input => return None , _ if ty . references_error () => return None , ty :: Adt (adt , args) if self . tcx () . is_lang_item (adt . did () , LangItem :: MaybeUninit) => { let fields = & adt . non_enum_variant () . fields ; let ty = fields [FieldIdx :: ONE] . ty (self . tcx () , args) ; let ty :: Adt (ty , args) = ty . kind () else { unreachable ! ("expected first field of `MaybeUninit` to be an ADT") } ; assert ! (ty . is_manually_drop () , "expected first field of `MaybeUninit` to be `ManuallyDrop`") ; let fields = & ty . non_enum_variant () . fields ; let ty = fields [FieldIdx :: ZERO] . ty (self . tcx () , args) ; self . get_asm_ty (expr . span , ty) } _ => self . get_asm_ty (expr . span , ty) , } ; let asm_ty = match asm_ty { Ok (asm_ty) => asm_ty , Err (reason) => { match reason { NonAsmTypeReason :: UnevaluatedSIMDArrayLength (did , len) => { let msg = format ! ("cannot evaluate SIMD vector length `{len}`") ; self . fcx . dcx () . struct_span_err (self . tcx () . def_span (did) , msg) . with_span_note (expr . span , "SIMD vector length needs to be known statically for use in `asm!`" ,) . emit () ; } NonAsmTypeReason :: Invalid (ty) => { let msg = format ! ("cannot use value of type `{ty}` for inline assembly") ; self . fcx . dcx () . struct_span_err (expr . span , msg) . with_note ("only integers, floats, SIMD vectors, pointers and function pointers \
                            can be used as arguments for inline assembly" ,) . emit () ; } NonAsmTypeReason :: NotSizedPtr (ty) => { let msg = format ! ("cannot use value of unsized pointer type `{ty}` for inline assembly") ; self . fcx . dcx () . struct_span_err (expr . span , msg) . with_note ("only sized pointers can be used in inline assembly") . emit () ; } NonAsmTypeReason :: InvalidElement (did , ty) => { let msg = format ! ("cannot use SIMD vector with element type `{ty}` for inline assembly") ; self . fcx . dcx () . struct_span_err (self . tcx () . def_span (did) , msg) . with_span_note (expr . span , "only integers, floats, SIMD vectors, pointers and function pointers \
                            can be used as arguments for inline assembly" ,) . emit () ; } NonAsmTypeReason :: EmptySIMDArray (ty) => { let msg = format ! ("use of empty SIMD vector `{ty}`") ; self . fcx . dcx () . struct_span_err (expr . span , msg) . emit () ; } } return None ; } } ; if ! self . fcx . type_is_copy_modulo_regions (self . fcx . param_env , ty) { let msg = "arguments for inline assembly must be copyable" ; self . fcx . dcx () . struct_span_err (expr . span , msg) . with_note (format ! ("`{ty}` does not implement the Copy trait")) . emit () ; } if let Some ((in_expr , Some (in_asm_ty))) = tied_input { if in_asm_ty != asm_ty { let msg = "incompatible types for asm inout argument" ; let in_expr_ty = self . expr_ty (in_expr) ; self . fcx . dcx () . struct_span_err (vec ! [in_expr . span , expr . span] , msg) . with_span_label (in_expr . span , format ! ("type `{in_expr_ty}`")) . with_span_label (expr . span , format ! ("type `{ty}`")) . with_note ("asm inout arguments must have the same type, \
                        unless they are both pointers or integers of the same size" ,) . emit () ; } return Some (asm_ty) ; } let asm_arch = self . tcx () . sess . asm_arch . unwrap () ; let allow_experimental_reg = self . tcx () . features () . asm_experimental_reg () ; let reg_class = reg . reg_class () ; let supported_tys = reg_class . supported_types (asm_arch , allow_experimental_reg) ; let Some ((_ , feature)) = supported_tys . iter () . find (| & & (t , _) | t == asm_ty) else { let mut err = if ! allow_experimental_reg && reg_class . supported_types (asm_arch , true) . iter () . any (| & (t , _) | t == asm_ty) { self . tcx () . sess . create_feature_err (RegisterTypeUnstable { span : expr . span , ty } , sym :: asm_experimental_reg ,) } else { let msg = format ! ("type `{ty}` cannot be used with this register class") ; let mut err = self . fcx . dcx () . struct_span_err (expr . span , msg) ; let supported_tys : Vec < _ > = supported_tys . iter () . map (| (t , _) | t . to_string ()) . collect () ; err . note (format ! ("register class `{}` supports these types: {}" , reg_class . name () , supported_tys . join (", ") ,)) ; err } ; if let Some (suggest) = reg_class . suggest_class (asm_arch , asm_ty) { err . help (format ! ("consider using the `{}` register class instead" , suggest . name ())) ; } err . emit () ; return Some (asm_ty) ; } ; if let Some (feature) = feature { if ! self . target_features . contains (feature) { let msg = format ! ("`{feature}` target feature is not enabled") ; self . fcx . dcx () . struct_span_err (expr . span , msg) . with_note (format ! ("this is required to use type `{}` with register class `{}`" , ty , reg_class . name () ,)) . emit () ; return Some (asm_ty) ; } } if let Some (ModifierInfo { modifier : suggested_modifier , result : suggested_result , size : suggested_size , }) = reg_class . suggest_modifier (asm_arch , asm_ty) { let mut spans = vec ! [] ; for piece in template { if let & InlineAsmTemplatePiece :: Placeholder { operand_idx , modifier , span } = piece { if operand_idx == idx && modifier . is_none () { spans . push (span) ; } } } if ! spans . is_empty () { let ModifierInfo { modifier : default_modifier , result : default_result , size : default_size , } = reg_class . default_modifier (asm_arch) . unwrap () ; self . tcx () . node_span_lint (lint :: builtin :: ASM_SUB_REGISTER , expr . hir_id , spans , | lint | { lint . primary_message ("formatting may not be suitable for sub-register argument") ; lint . span_label (expr . span , "for this argument") ; lint . help (format ! ("use `{{{idx}:{suggested_modifier}}}` to have the register formatted as `{suggested_result}` (for {suggested_size}-bit values)" ,)) ; lint . help (format ! ("or use `{{{idx}:{default_modifier}}}` to keep the default formatting of `{default_result}` (for {default_size}-bit values)" ,)) ; } ,) ; } } Some (asm_ty) } pub (crate) fn check_asm (& self , asm : & hir :: InlineAsm < 'tcx >) { let Some (asm_arch) = self . tcx () . sess . asm_arch else { self . fcx . dcx () . delayed_bug ("target architecture does not support asm") ; return ; } ; let allow_experimental_reg = self . tcx () . features () . asm_experimental_reg () ; for (idx , & (op , op_sp)) in asm . operands . iter () . enumerate () { if let Some (reg) = op . reg () { if let InlineAsmRegOrRegClass :: Reg (reg) = reg { if let InlineAsmReg :: Err = reg { continue ; } if let Err (msg) = reg . validate (asm_arch , self . tcx () . sess . relocation_model () , self . target_features , & self . tcx () . sess . target , op . is_clobber () ,) { let msg = format ! ("cannot use register `{}`: {}" , reg . name () , msg) ; self . fcx . dcx () . span_err (op_sp , msg) ; continue ; } } if ! op . is_clobber () { let mut missing_required_features = vec ! [] ; let reg_class = reg . reg_class () ; if let InlineAsmRegClass :: Err = reg_class { continue ; } for & (_ , feature) in reg_class . supported_types (asm_arch , allow_experimental_reg) { match feature { Some (feature) => { if self . target_features . contains (& feature) { missing_required_features . clear () ; break ; } else { missing_required_features . push (feature) ; } } None => { missing_required_features . clear () ; break ; } } } missing_required_features . sort_unstable () ; missing_required_features . dedup () ; match & missing_required_features [..] { [] => { } [feature] => { let msg = format ! ("register class `{}` requires the `{}` target feature" , reg_class . name () , feature) ; self . fcx . dcx () . span_err (op_sp , msg) ; continue ; } features => { let msg = format ! ("register class `{}` requires at least one of the following target features: {}" , reg_class . name () , features . iter () . map (| f | f . as_str ()) . intersperse (", ") . collect ::< String > () ,) ; self . fcx . dcx () . span_err (op_sp , msg) ; continue ; } } } } match op { hir :: InlineAsmOperand :: In { reg , expr } => { self . check_asm_operand_type (idx , reg , expr , asm . template , true , None) ; } hir :: InlineAsmOperand :: Out { reg , late : _ , expr } => { if let Some (expr) = expr { self . check_asm_operand_type (idx , reg , expr , asm . template , false , None) ; } } hir :: InlineAsmOperand :: InOut { reg , late : _ , expr } => { self . check_asm_operand_type (idx , reg , expr , asm . template , false , None) ; } hir :: InlineAsmOperand :: SplitInOut { reg , late : _ , in_expr , out_expr } => { let in_ty = self . check_asm_operand_type (idx , reg , in_expr , asm . template , true , None) ; if let Some (out_expr) = out_expr { self . check_asm_operand_type (idx , reg , out_expr , asm . template , false , Some ((in_expr , in_ty)) ,) ; } } hir :: InlineAsmOperand :: Const { anon_const } => { let ty = self . expr_ty (self . tcx () . hir_body (anon_const . body) . value) ; match ty . kind () { ty :: Error (_) => { } _ if ty . is_integral () => { } _ => { self . fcx . dcx () . struct_span_err (op_sp , "invalid type for `const` operand") . with_span_label (self . tcx () . def_span (anon_const . def_id) , format ! ("is {} `{}`" , ty . kind () . article () , ty) ,) . with_help ("`const` operands must be of an integer type") . emit () ; } } } hir :: InlineAsmOperand :: SymFn { expr } => { let ty = self . expr_ty (expr) ; match ty . kind () { ty :: FnDef (..) => { } ty :: Error (_) => { } _ => { self . fcx . dcx () . struct_span_err (op_sp , "invalid `sym` operand") . with_span_label (expr . span , format ! ("is {} `{}`" , ty . kind () . article () , ty) ,) . with_help ("`sym` operands must refer to either a function or a static" ,) . emit () ; } } } hir :: InlineAsmOperand :: SymStatic { .. } => { } hir :: InlineAsmOperand :: Label { .. } => { } } } } }}}