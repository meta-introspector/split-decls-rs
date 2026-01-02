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
mkuse!{use cranelift_codegen :: isa :: TargetFrontendConfig ;}
mkuse!{use cranelift_frontend :: { FunctionBuilder , FunctionBuilderContext } ;}
mkuse!{use rustc_abi :: { Float , Integer , Primitive } ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_middle :: ty :: TypeFoldable ;}
mkuse!{use rustc_middle :: ty :: layout :: { self , FnAbiError , FnAbiOfHelpers , FnAbiRequest , LayoutError , LayoutOfHelpers , } ;}
mkuse!{use rustc_span :: source_map :: Spanned ;}
mkuse!{use rustc_target :: callconv :: FnAbi ;}
mkuse!{use rustc_target :: spec :: { HasTargetSpec , Target } ;}
mkuse!{use crate :: constant :: ConstantCx ;}
mkuse!{use crate :: debuginfo :: FunctionDebugContext ;}
mkuse!{use crate :: prelude :: * ;}

macro_rules! pointer_ty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pointer_ty in module {}", module_path!());
    };
}

mkfn!{
    pointer_ty_introspect!();
    pub (crate) fn pointer_ty (tcx : TyCtxt < '_ >) -> types :: Type { match tcx . data_layout . pointer_size () . bits () { 16 => types :: I16 , 32 => types :: I32 , 64 => types :: I64 , bits => bug ! ("ptr_sized_integer: unknown pointer bit size {}" , bits) , } }
}

macro_rules! scalar_to_clif_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function scalar_to_clif_type in module {}", module_path!());
    };
}

mkfn!{
    scalar_to_clif_type_introspect!();
    pub (crate) fn scalar_to_clif_type (tcx : TyCtxt < '_ > , scalar : Scalar) -> Type { match scalar . primitive () { Primitive :: Int (int , _sign) => match int { Integer :: I8 => types :: I8 , Integer :: I16 => types :: I16 , Integer :: I32 => types :: I32 , Integer :: I64 => types :: I64 , Integer :: I128 => types :: I128 , } , Primitive :: Float (float) => match float { Float :: F16 => types :: F16 , Float :: F32 => types :: F32 , Float :: F64 => types :: F64 , Float :: F128 => types :: F128 , } , Primitive :: Pointer (_) => pointer_ty (tcx) , } }
}

macro_rules! clif_type_from_ty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clif_type_from_ty in module {}", module_path!());
    };
}

mkfn!{
    clif_type_from_ty_introspect!();
    fn clif_type_from_ty < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx >) -> Option < types :: Type > { Some (match ty . kind () { ty :: Bool => types :: I8 , ty :: Uint (size) => match size { UintTy :: U8 => types :: I8 , UintTy :: U16 => types :: I16 , UintTy :: U32 => types :: I32 , UintTy :: U64 => types :: I64 , UintTy :: U128 => types :: I128 , UintTy :: Usize => pointer_ty (tcx) , } , ty :: Int (size) => match size { IntTy :: I8 => types :: I8 , IntTy :: I16 => types :: I16 , IntTy :: I32 => types :: I32 , IntTy :: I64 => types :: I64 , IntTy :: I128 => types :: I128 , IntTy :: Isize => pointer_ty (tcx) , } , ty :: Char => types :: I32 , ty :: Float (size) => match size { FloatTy :: F16 => types :: F16 , FloatTy :: F32 => types :: F32 , FloatTy :: F64 => types :: F64 , FloatTy :: F128 => types :: F128 , } , ty :: FnPtr (..) => pointer_ty (tcx) , ty :: RawPtr (pointee_ty , _) | ty :: Ref (_ , pointee_ty , _) => { if tcx . type_has_metadata (* pointee_ty , ty :: TypingEnv :: fully_monomorphized ()) { return None ; } else { pointer_ty (tcx) } } ty :: Param (_) => bug ! ("ty param {:?}" , ty) , _ => return None , }) }
}

macro_rules! clif_pair_type_from_ty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clif_pair_type_from_ty in module {}", module_path!());
    };
}

mkfn!{
    clif_pair_type_from_ty_introspect!();
    fn clif_pair_type_from_ty < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > ,) -> Option < (types :: Type , types :: Type) > { Some (match ty . kind () { ty :: Tuple (types) if types . len () == 2 => { (clif_type_from_ty (tcx , types [0]) ? , clif_type_from_ty (tcx , types [1]) ?) } ty :: RawPtr (pointee_ty , _) | ty :: Ref (_ , pointee_ty , _) => { if tcx . type_has_metadata (* pointee_ty , ty :: TypingEnv :: fully_monomorphized ()) { (pointer_ty (tcx) , pointer_ty (tcx)) } else { return None ; } } _ => return None , }) }
}

macro_rules! codegen_icmp_imm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function codegen_icmp_imm in module {}", module_path!());
    };
}

mkfn!{
    codegen_icmp_imm_introspect!();
    pub (crate) fn codegen_icmp_imm (fx : & mut FunctionCx < '_ , '_ , '_ > , intcc : IntCC , lhs : Value , rhs : i128 ,) -> Value { let lhs_ty = fx . bcx . func . dfg . value_type (lhs) ; if lhs_ty == types :: I128 { let (lhs_lsb , lhs_msb) = fx . bcx . ins () . isplit (lhs) ; let (rhs_lsb , rhs_msb) = (rhs as u128 as u64 as i64 , (rhs as u128 >> 64) as u64 as i64) ; match intcc { IntCC :: Equal => { let lsb_eq = fx . bcx . ins () . icmp_imm (IntCC :: Equal , lhs_lsb , rhs_lsb) ; let msb_eq = fx . bcx . ins () . icmp_imm (IntCC :: Equal , lhs_msb , rhs_msb) ; fx . bcx . ins () . band (lsb_eq , msb_eq) } IntCC :: NotEqual => { let lsb_ne = fx . bcx . ins () . icmp_imm (IntCC :: NotEqual , lhs_lsb , rhs_lsb) ; let msb_ne = fx . bcx . ins () . icmp_imm (IntCC :: NotEqual , lhs_msb , rhs_msb) ; fx . bcx . ins () . bor (lsb_ne , msb_ne) } _ => { let msb_eq = fx . bcx . ins () . icmp_imm (IntCC :: Equal , lhs_msb , rhs_msb) ; let lsb_cc = fx . bcx . ins () . icmp_imm (intcc , lhs_lsb , rhs_lsb) ; let msb_cc = fx . bcx . ins () . icmp_imm (intcc , lhs_msb , rhs_msb) ; fx . bcx . ins () . select (msb_eq , lsb_cc , msb_cc) } } } else { let rhs = rhs as i64 ; fx . bcx . ins () . icmp_imm (intcc , lhs , rhs) } }
}

macro_rules! codegen_bitcast_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function codegen_bitcast in module {}", module_path!());
    };
}

mkfn!{
    codegen_bitcast_introspect!();
    pub (crate) fn codegen_bitcast (fx : & mut FunctionCx < '_ , '_ , '_ > , dst_ty : Type , val : Value) -> Value { let mut flags = MemFlags :: new () ; flags . set_endianness (match fx . tcx . data_layout . endian { rustc_abi :: Endian :: Big => cranelift_codegen :: ir :: Endianness :: Big , rustc_abi :: Endian :: Little => cranelift_codegen :: ir :: Endianness :: Little , }) ; fx . bcx . ins () . bitcast (dst_ty , flags , val) }
}

macro_rules! type_zero_value_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_zero_value in module {}", module_path!());
    };
}

mkfn!{
    type_zero_value_introspect!();
    pub (crate) fn type_zero_value (bcx : & mut FunctionBuilder < '_ > , ty : Type) -> Value { if ty == types :: I128 { let zero = bcx . ins () . iconst (types :: I64 , 0) ; bcx . ins () . iconcat (zero , zero) } else { bcx . ins () . iconst (ty , 0) } }
}

macro_rules! type_min_max_value_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_min_max_value in module {}", module_path!());
    };
}

mkfn!{
    type_min_max_value_introspect!();
    pub (crate) fn type_min_max_value (bcx : & mut FunctionBuilder < '_ > , ty : Type , signed : bool ,) -> (Value , Value) { assert ! (ty . is_int ()) ; if ty == types :: I128 { if signed { let min = i128 :: MIN as u128 ; let min_lsb = bcx . ins () . iconst (types :: I64 , min as u64 as i64) ; let min_msb = bcx . ins () . iconst (types :: I64 , (min >> 64) as u64 as i64) ; let min = bcx . ins () . iconcat (min_lsb , min_msb) ; let max = i128 :: MAX as u128 ; let max_lsb = bcx . ins () . iconst (types :: I64 , max as u64 as i64) ; let max_msb = bcx . ins () . iconst (types :: I64 , (max >> 64) as u64 as i64) ; let max = bcx . ins () . iconcat (max_lsb , max_msb) ; return (min , max) ; } else { let min_half = bcx . ins () . iconst (types :: I64 , 0) ; let min = bcx . ins () . iconcat (min_half , min_half) ; let max_half = bcx . ins () . iconst (types :: I64 , u64 :: MAX as i64) ; let max = bcx . ins () . iconcat (max_half , max_half) ; return (min , max) ; } } let min = match (ty , signed) { (types :: I8 , false) | (types :: I16 , false) | (types :: I32 , false) | (types :: I64 , false) => { 0i64 } (types :: I8 , true) => i64 :: from (i8 :: MIN as u8) , (types :: I16 , true) => i64 :: from (i16 :: MIN as u16) , (types :: I32 , true) => i64 :: from (i32 :: MIN as u32) , (types :: I64 , true) => i64 :: MIN , _ => unreachable ! () , } ; let max = match (ty , signed) { (types :: I8 , false) => i64 :: from (u8 :: MAX) , (types :: I16 , false) => i64 :: from (u16 :: MAX) , (types :: I32 , false) => i64 :: from (u32 :: MAX) , (types :: I64 , false) => u64 :: MAX as i64 , (types :: I8 , true) => i64 :: from (i8 :: MAX as u8) , (types :: I16 , true) => i64 :: from (i16 :: MAX as u16) , (types :: I32 , true) => i64 :: from (i32 :: MAX as u32) , (types :: I64 , true) => i64 :: MAX , _ => unreachable ! () , } ; let (min , max) = (bcx . ins () . iconst (ty , min) , bcx . ins () . iconst (ty , max)) ; (min , max) }
}

macro_rules! type_sign_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_sign in module {}", module_path!());
    };
}

mkfn!{
    type_sign_introspect!();
    pub (crate) fn type_sign (ty : Ty < '_ >) -> bool { match ty . kind () { ty :: Ref (..) | ty :: RawPtr (..) | ty :: FnPtr (..) | ty :: Char | ty :: Uint (..) | ty :: Bool => false , ty :: Int (..) => true , ty :: Float (..) => false , _ => panic ! ("{}" , ty) , } }
}

macro_rules! create_wrapper_function_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_wrapper_function in module {}", module_path!());
    };
}

mkfn!{
    create_wrapper_function_introspect!();
    pub (crate) fn create_wrapper_function (module : & mut dyn Module , sig : Signature , wrapper_name : & str , callee_name : & str ,) { let wrapper_func_id = module . declare_function (wrapper_name , Linkage :: Export , & sig) . unwrap () ; let callee_func_id = module . declare_function (callee_name , Linkage :: Import , & sig) . unwrap () ; let mut ctx = Context :: new () ; ctx . func . signature = sig ; { let mut func_ctx = FunctionBuilderContext :: new () ; let mut bcx = FunctionBuilder :: new (& mut ctx . func , & mut func_ctx) ; let block = bcx . create_block () ; bcx . switch_to_block (block) ; let func = & mut bcx . func . stencil ; let args = func . signature . params . iter () . map (| param | func . dfg . append_block_param (block , param . value_type)) . collect :: < Vec < Value > > () ; let callee_func_ref = module . declare_func_in_func (callee_func_id , & mut bcx . func) ; let call_inst = bcx . ins () . call (callee_func_ref , & args) ; let results = bcx . inst_results (call_inst) . to_vec () ; bcx . ins () . return_ (& results) ; bcx . seal_all_blocks () ; bcx . finalize () ; } module . define_function (wrapper_func_id , & mut ctx) . unwrap () ; }
}
mkitem!{mkstruct!{pub (crate) struct FunctionCx < 'm , 'clif , 'tcx : 'm > { pub (crate) cx : & 'clif mut crate :: CodegenCx , pub (crate) module : & 'm mut dyn Module , pub (crate) tcx : TyCtxt < 'tcx > , pub (crate) target_config : TargetFrontendConfig , pub (crate) pointer_type : Type , pub (crate) constants_cx : ConstantCx , pub (crate) func_debug_cx : Option < FunctionDebugContext > , pub (crate) instance : Instance < 'tcx > , pub (crate) symbol_name : String , pub (crate) mir : & 'tcx Body < 'tcx > , pub (crate) fn_abi : & 'tcx FnAbi < 'tcx , Ty < 'tcx > > , pub (crate) bcx : FunctionBuilder < 'clif > , pub (crate) block_map : IndexVec < BasicBlock , Block > , pub (crate) local_map : IndexVec < Local , CPlace < 'tcx > > , #[doc = " When `#[track_caller]` is used, the implicit caller location is stored in this variable."] pub (crate) caller_location : Option < CValue < 'tcx > > , pub (crate) clif_comments : crate :: pretty_clif :: CommentWriter , #[doc = " This should only be accessed by `CPlace::new_var`."] pub (crate) next_ssa_var : u32 , }}}
mkitem!{mkimpl!{impl < 'tcx > LayoutOfHelpers < 'tcx > for FunctionCx < '_ , '_ , 'tcx > { #[inline] fn handle_layout_err (& self , err : LayoutError < 'tcx > , span : Span , ty : Ty < 'tcx >) -> ! { FullyMonomorphizedLayoutCx (self . tcx) . handle_layout_err (err , span , ty) } }}}
mkitem!{mkimpl!{impl < 'tcx > FnAbiOfHelpers < 'tcx > for FunctionCx < '_ , '_ , 'tcx > { #[inline] fn handle_fn_abi_err (& self , err : FnAbiError < 'tcx > , span : Span , fn_abi_request : FnAbiRequest < 'tcx > ,) -> ! { FullyMonomorphizedLayoutCx (self . tcx) . handle_fn_abi_err (err , span , fn_abi_request) } }}}
mkitem!{mkimpl!{impl < 'tcx > layout :: HasTyCtxt < 'tcx > for FunctionCx < '_ , '_ , 'tcx > { fn tcx < 'b > (& 'b self) -> TyCtxt < 'tcx > { self . tcx } }}}
mkitem!{mkimpl!{impl < 'tcx > rustc_abi :: HasDataLayout for FunctionCx < '_ , '_ , 'tcx > { fn data_layout (& self) -> & rustc_abi :: TargetDataLayout { & self . tcx . data_layout } }}}
mkitem!{mkimpl!{impl < 'tcx > layout :: HasTypingEnv < 'tcx > for FunctionCx < '_ , '_ , 'tcx > { fn typing_env (& self) -> ty :: TypingEnv < 'tcx > { ty :: TypingEnv :: fully_monomorphized () } }}}
mkitem!{mkimpl!{impl < 'tcx > HasTargetSpec for FunctionCx < '_ , '_ , 'tcx > { fn target_spec (& self) -> & Target { & self . tcx . sess . target } }}}
mkitem!{mkimpl!{impl < 'tcx > FunctionCx < '_ , '_ , 'tcx > { pub (crate) fn monomorphize < T > (& self , value : T) -> T where T : TypeFoldable < TyCtxt < 'tcx > > + Copy , { self . instance . instantiate_mir_and_normalize_erasing_regions (self . tcx , ty :: TypingEnv :: fully_monomorphized () , ty :: EarlyBinder :: bind (value) ,) } pub (crate) fn clif_type (& self , ty : Ty < 'tcx >) -> Option < Type > { clif_type_from_ty (self . tcx , ty) } pub (crate) fn clif_pair_type (& self , ty : Ty < 'tcx >) -> Option < (Type , Type) > { clif_pair_type_from_ty (self . tcx , ty) } pub (crate) fn get_block (& self , bb : BasicBlock) -> Block { * self . block_map . get (bb) . unwrap () } pub (crate) fn get_local_place (& mut self , local : Local) -> CPlace < 'tcx > { * self . local_map . get (local) . unwrap_or_else (| | { panic ! ("Local {:?} doesn't exist" , local) ; }) } pub (crate) fn create_stack_slot (& mut self , size : u32 , align : u32) -> Pointer { assert ! (size % align == 0 , "size must be a multiple of alignment (size={size}, align={align})") ; let abi_align = if self . tcx . sess . target . arch == "s390x" { 8 } else { 16 } ; if align <= abi_align { let stack_slot = self . bcx . create_sized_stack_slot (StackSlotData { kind : StackSlotKind :: ExplicitSlot , size : (size + abi_align - 1) / abi_align * abi_align , align_shift : 4 , }) ; Pointer :: stack_slot (stack_slot) } else { let stack_slot = self . bcx . create_sized_stack_slot (StackSlotData { kind : StackSlotKind :: ExplicitSlot , size : (size + align) / abi_align * abi_align , align_shift : 4 , }) ; let base_ptr = self . bcx . ins () . stack_addr (self . pointer_type , stack_slot , 0) ; let misalign_offset = self . bcx . ins () . band_imm (base_ptr , i64 :: from (align - 1)) ; let realign_offset = self . bcx . ins () . irsub_imm (misalign_offset , i64 :: from (align)) ; Pointer :: new (self . bcx . ins () . iadd (base_ptr , realign_offset)) } } pub (crate) fn set_debug_loc (& mut self , source_info : mir :: SourceInfo) { if let Some (debug_context) = & mut self . cx . debug_context { let (file_id , line , column) = debug_context . get_span_loc (self . tcx , self . mir . span , source_info . span) ; let source_loc = self . func_debug_cx . as_mut () . unwrap () . add_dbg_loc (file_id , line , column) ; self . bcx . set_srcloc (source_loc) ; } } pub (crate) fn get_caller_location (& mut self , source_info : mir :: SourceInfo) -> CValue < 'tcx > { self . mir . caller_location_span (source_info , self . caller_location , self . tcx , | span | { let const_loc = self . tcx . span_as_caller_location (span) ; crate :: constant :: codegen_const_value (self , const_loc , self . tcx . caller_location_ty ()) }) } pub (crate) fn anonymous_str (& mut self , msg : & str) -> Value { let mut data = DataDescription :: new () ; data . define (msg . as_bytes () . to_vec () . into_boxed_slice ()) ; let msg_id = self . module . declare_anonymous_data (false , false) . unwrap () ; let _ = self . module . define_data (msg_id , & data) ; let local_msg_id = self . module . declare_data_in_func (msg_id , self . bcx . func) ; if self . clif_comments . enabled () { self . add_comment (local_msg_id , msg) ; } self . bcx . ins () . global_value (self . pointer_type , local_msg_id) } }}}
mkitem!{mkstruct!{pub (crate) struct FullyMonomorphizedLayoutCx < 'tcx > (pub (crate) TyCtxt < 'tcx >) ;}}
mkitem!{mkimpl!{impl < 'tcx > LayoutOfHelpers < 'tcx > for FullyMonomorphizedLayoutCx < 'tcx > { #[inline] fn handle_layout_err (& self , err : LayoutError < 'tcx > , span : Span , ty : Ty < 'tcx >) -> ! { if let LayoutError :: SizeOverflow (_) | LayoutError :: ReferencesError (_) = err { self . 0 . sess . dcx () . span_fatal (span , err . to_string ()) } else { self . 0 . sess . dcx () . span_fatal (span , format ! ("failed to get layout for `{}`: {}" , ty , err)) } } }}}
mkitem!{mkimpl!{impl < 'tcx > FnAbiOfHelpers < 'tcx > for FullyMonomorphizedLayoutCx < 'tcx > { #[inline] fn handle_fn_abi_err (& self , err : FnAbiError < 'tcx > , span : Span , fn_abi_request : FnAbiRequest < 'tcx > ,) -> ! { if let FnAbiError :: Layout (LayoutError :: SizeOverflow (_)) = err { self . 0 . sess . dcx () . emit_fatal (Spanned { span , node : err }) } else { match fn_abi_request { FnAbiRequest :: OfFnPtr { sig , extra_args } => { span_bug ! (span , "`fn_abi_of_fn_ptr({sig}, {extra_args:?})` failed: {err:?}") ; } FnAbiRequest :: OfInstance { instance , extra_args } => { span_bug ! (span , "`fn_abi_of_instance({instance}, {extra_args:?})` failed: {err:?}") ; } } } } }}}
mkitem!{mkimpl!{impl < 'tcx > layout :: HasTyCtxt < 'tcx > for FullyMonomorphizedLayoutCx < 'tcx > { fn tcx < 'b > (& 'b self) -> TyCtxt < 'tcx > { self . 0 } }}}
mkitem!{mkimpl!{impl < 'tcx > rustc_abi :: HasDataLayout for FullyMonomorphizedLayoutCx < 'tcx > { fn data_layout (& self) -> & rustc_abi :: TargetDataLayout { & self . 0 . data_layout } }}}
mkitem!{mkimpl!{impl < 'tcx > layout :: HasTypingEnv < 'tcx > for FullyMonomorphizedLayoutCx < 'tcx > { fn typing_env (& self) -> ty :: TypingEnv < 'tcx > { ty :: TypingEnv :: fully_monomorphized () } }}}
mkitem!{mkimpl!{impl < 'tcx > HasTargetSpec for FullyMonomorphizedLayoutCx < 'tcx > { fn target_spec (& self) -> & Target { & self . 0 . sess . target } }}}