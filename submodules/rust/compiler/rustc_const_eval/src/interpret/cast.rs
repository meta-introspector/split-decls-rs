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
mkuse!{use std :: assert_matches :: assert_matches ;}
mkuse!{use rustc_abi :: { FieldIdx , Integer } ;}
mkuse!{use rustc_apfloat :: ieee :: { Double , Half , Quad , Single } ;}
mkuse!{use rustc_apfloat :: { Float , FloatConvert } ;}
mkuse!{use rustc_middle :: mir :: CastKind ;}
mkuse!{use rustc_middle :: mir :: interpret :: { InterpResult , PointerArithmetic , Scalar } ;}
mkuse!{use rustc_middle :: ty :: adjustment :: PointerCoercion ;}
mkuse!{use rustc_middle :: ty :: layout :: { IntegerExt , TyAndLayout } ;}
mkuse!{use rustc_middle :: ty :: { self , FloatTy , Ty } ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use tracing :: trace ;}
mkuse!{use super :: util :: ensure_monomorphic_enough ;}
mkuse!{use super :: { FnVal , ImmTy , Immediate , InterpCx , Machine , OpTy , PlaceTy , err_inval , interp_ok , throw_ub , throw_ub_custom , } ;}
mkuse!{use crate :: interpret :: Writeable ;}
mkuse!{use crate :: { enter_trace_span , fluent_generated as fluent } ;}
mkitem!{mkimpl!{impl < 'tcx , M : Machine < 'tcx > > InterpCx < 'tcx , M > { pub fn cast (& mut self , src : & OpTy < 'tcx , M :: Provenance > , cast_kind : CastKind , cast_ty : Ty < 'tcx > , dest : & PlaceTy < 'tcx , M :: Provenance > ,) -> InterpResult < 'tcx > { let cast_layout = if cast_ty == dest . layout . ty { dest . layout } else { self . layout_of (cast_ty) ? } ; match cast_kind { CastKind :: PointerCoercion (PointerCoercion :: Unsize , _) => { self . unsize_into (src , cast_layout , dest) ? ; } CastKind :: PointerExposeProvenance => { let src = self . read_immediate (src) ? ; let res = self . pointer_expose_provenance_cast (& src , cast_layout) ? ; self . write_immediate (* res , dest) ? ; } CastKind :: PointerWithExposedProvenance => { let src = self . read_immediate (src) ? ; let res = self . pointer_with_exposed_provenance_cast (& src , cast_layout) ? ; self . write_immediate (* res , dest) ? ; } CastKind :: IntToInt | CastKind :: IntToFloat => { let src = self . read_immediate (src) ? ; let res = self . int_to_int_or_float (& src , cast_layout) ? ; self . write_immediate (* res , dest) ? ; } CastKind :: FloatToFloat | CastKind :: FloatToInt => { let src = self . read_immediate (src) ? ; let res = self . float_to_float_or_int (& src , cast_layout) ? ; self . write_immediate (* res , dest) ? ; } CastKind :: FnPtrToPtr | CastKind :: PtrToPtr => { let src = self . read_immediate (src) ? ; let res = self . ptr_to_ptr (& src , cast_layout) ? ; self . write_immediate (* res , dest) ? ; } CastKind :: PointerCoercion (PointerCoercion :: MutToConstPointer | PointerCoercion :: ArrayToPointer , _ ,) => { bug ! ("{cast_kind:?} casts are for borrowck only, not runtime MIR") ; } CastKind :: PointerCoercion (PointerCoercion :: ReifyFnPointer , _) => { ensure_monomorphic_enough (* self . tcx , src . layout . ty) ? ; match * src . layout . ty . kind () { ty :: FnDef (def_id , args) => { let instance = { let _trace = enter_trace_span ! (M , resolve :: resolve_for_fn_ptr , ? def_id) ; ty :: Instance :: resolve_for_fn_ptr (* self . tcx , self . typing_env , def_id , args ,) . ok_or_else (| | err_inval ! (TooGeneric)) ? } ; let fn_ptr = self . fn_ptr (FnVal :: Instance (instance)) ; self . write_pointer (fn_ptr , dest) ? ; } _ => span_bug ! (self . cur_span () , "reify fn pointer on {}" , src . layout . ty) , } } CastKind :: PointerCoercion (PointerCoercion :: UnsafeFnPointer , _) => { let src = self . read_immediate (src) ? ; match cast_ty . kind () { ty :: FnPtr (..) => { self . write_immediate (* src , dest) ? ; } _ => span_bug ! (self . cur_span () , "fn to unsafe fn cast on {}" , cast_ty) , } } CastKind :: PointerCoercion (PointerCoercion :: ClosureFnPointer (_) , _) => { ensure_monomorphic_enough (* self . tcx , src . layout . ty) ? ; match * src . layout . ty . kind () { ty :: Closure (def_id , args) => { let instance = { let _trace = enter_trace_span ! (M , resolve :: resolve_closure , ? def_id) ; ty :: Instance :: resolve_closure (* self . tcx , def_id , args , ty :: ClosureKind :: FnOnce ,) } ; let fn_ptr = self . fn_ptr (FnVal :: Instance (instance)) ; self . write_pointer (fn_ptr , dest) ? ; } _ => span_bug ! (self . cur_span () , "closure fn pointer on {}" , src . layout . ty) , } } CastKind :: Transmute => { assert ! (src . layout . is_sized ()) ; assert ! (dest . layout . is_sized ()) ; assert_eq ! (cast_ty , dest . layout . ty) ; if src . layout . size != dest . layout . size { throw_ub_custom ! (fluent :: const_eval_invalid_transmute , src_bytes = src . layout . size . bytes () , dest_bytes = dest . layout . size . bytes () , src = src . layout . ty , dest = dest . layout . ty ,) ; } self . copy_op_allow_transmute (src , dest) ? ; } } interp_ok (()) } #[doc = " Handles 'IntToInt' and 'IntToFloat' casts."] pub fn int_to_int_or_float (& self , src : & ImmTy < 'tcx , M :: Provenance > , cast_to : TyAndLayout < 'tcx > ,) -> InterpResult < 'tcx , ImmTy < 'tcx , M :: Provenance > > { assert ! (src . layout . ty . is_integral () || src . layout . ty . is_char () || src . layout . ty . is_bool ()) ; assert ! (cast_to . ty . is_floating_point () || cast_to . ty . is_integral () || cast_to . ty . is_char ()) ; interp_ok (ImmTy :: from_scalar (self . cast_from_int_like (src . to_scalar () , src . layout , cast_to . ty) ? , cast_to ,)) } #[doc = " Handles 'FloatToFloat' and 'FloatToInt' casts."] pub fn float_to_float_or_int (& self , src : & ImmTy < 'tcx , M :: Provenance > , cast_to : TyAndLayout < 'tcx > ,) -> InterpResult < 'tcx , ImmTy < 'tcx , M :: Provenance > > { let ty :: Float (fty) = src . layout . ty . kind () else { bug ! ("FloatToFloat/FloatToInt cast: source type {} is not a float type" , src . layout . ty) } ; let val = match fty { FloatTy :: F16 => self . cast_from_float (src . to_scalar () . to_f16 () ? , cast_to . ty) , FloatTy :: F32 => self . cast_from_float (src . to_scalar () . to_f32 () ? , cast_to . ty) , FloatTy :: F64 => self . cast_from_float (src . to_scalar () . to_f64 () ? , cast_to . ty) , FloatTy :: F128 => self . cast_from_float (src . to_scalar () . to_f128 () ? , cast_to . ty) , } ; interp_ok (ImmTy :: from_scalar (val , cast_to)) } #[doc = " Handles 'FnPtrToPtr' and 'PtrToPtr' casts."] pub fn ptr_to_ptr (& self , src : & ImmTy < 'tcx , M :: Provenance > , cast_to : TyAndLayout < 'tcx > ,) -> InterpResult < 'tcx , ImmTy < 'tcx , M :: Provenance > > { assert ! (src . layout . ty . is_any_ptr ()) ; assert ! (cast_to . ty . is_raw_ptr ()) ; if cast_to . size == src . layout . size { return interp_ok (ImmTy :: from_immediate (* * src , cast_to)) ; } else { assert_eq ! (src . layout . size , 2 * self . pointer_size ()) ; assert_eq ! (cast_to . size , self . pointer_size ()) ; assert ! (src . layout . ty . is_raw_ptr ()) ; return match * * src { Immediate :: ScalarPair (data , _) => interp_ok (ImmTy :: from_scalar (data , cast_to)) , Immediate :: Scalar (..) => span_bug ! (self . cur_span () , "{:?} input to a fat-to-thin cast ({} -> {})" , * src , src . layout . ty , cast_to . ty) , Immediate :: Uninit => throw_ub ! (InvalidUninitBytes (None)) , } ; } } pub fn pointer_expose_provenance_cast (& mut self , src : & ImmTy < 'tcx , M :: Provenance > , cast_to : TyAndLayout < 'tcx > ,) -> InterpResult < 'tcx , ImmTy < 'tcx , M :: Provenance > > { assert_matches ! (src . layout . ty . kind () , ty :: RawPtr (_ , _) | ty :: FnPtr (..)) ; assert ! (cast_to . ty . is_integral ()) ; let scalar = src . to_scalar () ; let ptr = scalar . to_pointer (self) ? ; match ptr . into_pointer_or_addr () { Ok (ptr) => M :: expose_provenance (self , ptr . provenance) ? , Err (_) => { } } ; interp_ok (ImmTy :: from_scalar (self . cast_from_int_like (scalar , src . layout , cast_to . ty) ? , cast_to ,)) } pub fn pointer_with_exposed_provenance_cast (& self , src : & ImmTy < 'tcx , M :: Provenance > , cast_to : TyAndLayout < 'tcx > ,) -> InterpResult < 'tcx , ImmTy < 'tcx , M :: Provenance > > { assert ! (src . layout . ty . is_integral ()) ; assert_matches ! (cast_to . ty . kind () , ty :: RawPtr (_ , _)) ; let scalar = src . to_scalar () ; let addr = self . cast_from_int_like (scalar , src . layout , self . tcx . types . usize) ? ; let addr = addr . to_target_usize (self) ? ; let ptr = M :: ptr_from_addr_cast (self , addr) ? ; interp_ok (ImmTy :: from_scalar (Scalar :: from_maybe_pointer (ptr , self) , cast_to)) } #[doc = " Low-level cast helper function. This works directly on scalars and can take 'int-like' input"] #[doc = " type (basically everything with a scalar layout) to int/float/char types."] fn cast_from_int_like (& self , scalar : Scalar < M :: Provenance > , src_layout : TyAndLayout < 'tcx > , cast_ty : Ty < 'tcx > ,) -> InterpResult < 'tcx , Scalar < M :: Provenance > > { let signed = src_layout . backend_repr . is_signed () ; let v = match src_layout . ty . kind () { ty :: Uint (_) | ty :: RawPtr (..) | ty :: FnPtr (..) => scalar . to_uint (src_layout . size) ? , ty :: Int (_) => scalar . to_int (src_layout . size) ? as u128 , ty :: Bool => scalar . to_bool () ? . into () , ty :: Char => scalar . to_char () ? . into () , _ => span_bug ! (self . cur_span () , "invalid int-like cast from {}" , src_layout . ty) , } ; interp_ok (match * cast_ty . kind () { ty :: Int (_) | ty :: Uint (_) => { let size = match * cast_ty . kind () { ty :: Int (t) => Integer :: from_int_ty (self , t) . size () , ty :: Uint (t) => Integer :: from_uint_ty (self , t) . size () , _ => bug ! () , } ; let v = size . truncate (v) ; Scalar :: from_uint (v , size) } ty :: Float (fty) if signed => { let v = v as i128 ; match fty { FloatTy :: F16 => Scalar :: from_f16 (Half :: from_i128 (v) . value) , FloatTy :: F32 => Scalar :: from_f32 (Single :: from_i128 (v) . value) , FloatTy :: F64 => Scalar :: from_f64 (Double :: from_i128 (v) . value) , FloatTy :: F128 => Scalar :: from_f128 (Quad :: from_i128 (v) . value) , } } ty :: Float (fty) => match fty { FloatTy :: F16 => Scalar :: from_f16 (Half :: from_u128 (v) . value) , FloatTy :: F32 => Scalar :: from_f32 (Single :: from_u128 (v) . value) , FloatTy :: F64 => Scalar :: from_f64 (Double :: from_u128 (v) . value) , FloatTy :: F128 => Scalar :: from_f128 (Quad :: from_u128 (v) . value) , } , ty :: Char => Scalar :: from_u32 (u8 :: try_from (v) . unwrap () . into ()) , _ => span_bug ! (self . cur_span () , "invalid int to {} cast" , cast_ty) , }) } #[doc = " Low-level cast helper function. Converts an apfloat `f` into int or float types."] fn cast_from_float < F > (& self , f : F , dest_ty : Ty < 'tcx >) -> Scalar < M :: Provenance > where F : Float + Into < Scalar < M :: Provenance > > + FloatConvert < Half > + FloatConvert < Single > + FloatConvert < Double > + FloatConvert < Quad > , { match * dest_ty . kind () { ty :: Uint (t) => { let size = Integer :: from_uint_ty (self , t) . size () ; let v = f . to_u128 (size . bits_usize ()) . value ; Scalar :: from_uint (v , size) } ty :: Int (t) => { let size = Integer :: from_int_ty (self , t) . size () ; let v = f . to_i128 (size . bits_usize ()) . value ; Scalar :: from_int (v , size) } ty :: Float (fty) => match fty { FloatTy :: F16 => { Scalar :: from_f16 (self . adjust_nan (f . convert (& mut false) . value , & [f])) } FloatTy :: F32 => { Scalar :: from_f32 (self . adjust_nan (f . convert (& mut false) . value , & [f])) } FloatTy :: F64 => { Scalar :: from_f64 (self . adjust_nan (f . convert (& mut false) . value , & [f])) } FloatTy :: F128 => { Scalar :: from_f128 (self . adjust_nan (f . convert (& mut false) . value , & [f])) } } , _ => span_bug ! (self . cur_span () , "invalid float to {} cast" , dest_ty) , } } #[doc = " `src` is a *pointer to* a `source_ty`, and in `dest` we should store a pointer to th same"] #[doc = " data at type `cast_ty`."] fn unsize_into_ptr (& mut self , src : & OpTy < 'tcx , M :: Provenance > , dest : & impl Writeable < 'tcx , M :: Provenance > , source_ty : Ty < 'tcx > , cast_ty : Ty < 'tcx > ,) -> InterpResult < 'tcx > { let (src_pointee_ty , dest_pointee_ty) = self . tcx . struct_lockstep_tails_for_codegen (source_ty , cast_ty , self . typing_env) ; match (src_pointee_ty . kind () , dest_pointee_ty . kind ()) { (& ty :: Array (_ , length) , & ty :: Slice (_)) => { let ptr = self . read_pointer (src) ? ; let val = Immediate :: new_slice (ptr , length . try_to_target_usize (* self . tcx) . expect ("expected monomorphic const in const eval") , self ,) ; self . write_immediate (val , dest) } (ty :: Dynamic (data_a , _ , ty :: Dyn) , ty :: Dynamic (data_b , _ , ty :: Dyn)) => { let val = self . read_immediate (src) ? ; if data_a == data_b { return self . write_immediate (* val , dest) ; } let (old_data , old_vptr) = val . to_scalar_pair () ; let old_data = old_data . to_pointer (self) ? ; let old_vptr = old_vptr . to_pointer (self) ? ; let ty = self . get_ptr_vtable_ty (old_vptr , Some (data_a)) ? ; let vptr_entry_idx = self . tcx . supertrait_vtable_slot ((src_pointee_ty , dest_pointee_ty)) ; let vtable_entries = self . vtable_entries (data_a . principal () , ty) ; if let Some (entry_idx) = vptr_entry_idx { let Some (& ty :: VtblEntry :: TraitVPtr (upcast_trait_ref)) = vtable_entries . get (entry_idx) else { span_bug ! (self . cur_span () , "invalid vtable entry index in {} -> {} upcast" , src_pointee_ty , dest_pointee_ty) ; } ; let erased_trait_ref = ty :: ExistentialTraitRef :: erase_self_ty (* self . tcx , upcast_trait_ref) ; assert_eq ! (data_b . principal () . map (| b | { self . tcx . normalize_erasing_late_bound_regions (self . typing_env , b) }) , Some (erased_trait_ref) ,) ; } else { let vtable_entries_b = self . vtable_entries (data_b . principal () , ty) ; assert ! (& vtable_entries [.. vtable_entries_b . len ()] == vtable_entries_b) ; } ; let new_vptr = self . get_vtable_ptr (ty , data_b) ? ; self . write_immediate (Immediate :: new_dyn_trait (old_data , new_vptr , self) , dest) } (_ , & ty :: Dynamic (data , _ , ty :: Dyn)) => { let vtable = self . get_vtable_ptr (src_pointee_ty , data) ? ; let ptr = self . read_pointer (src) ? ; let val = Immediate :: new_dyn_trait (ptr , vtable , & * self . tcx) ; self . write_immediate (val , dest) } _ => { ensure_monomorphic_enough (* self . tcx , src . layout . ty) ? ; ensure_monomorphic_enough (* self . tcx , cast_ty) ? ; span_bug ! (self . cur_span () , "invalid pointer unsizing {} -> {}" , src . layout . ty , cast_ty) } } } pub fn unsize_into (& mut self , src : & OpTy < 'tcx , M :: Provenance > , cast_ty : TyAndLayout < 'tcx > , dest : & impl Writeable < 'tcx , M :: Provenance > ,) -> InterpResult < 'tcx > { trace ! ("Unsizing {:?} of type {} into {}" , * src , src . layout . ty , cast_ty . ty) ; match (src . layout . ty . kind () , cast_ty . ty . kind ()) { (& ty :: Ref (_ , s , _) , & ty :: Ref (_ , c , _) | & ty :: RawPtr (c , _)) | (& ty :: RawPtr (s , _) , & ty :: RawPtr (c , _)) => self . unsize_into_ptr (src , dest , s , c) , (& ty :: Adt (def_a , _) , & ty :: Adt (def_b , _)) => { assert_eq ! (def_a , def_b) ; let mut found_cast_field = false ; for i in 0 .. src . layout . fields . count () { let cast_ty_field = cast_ty . field (self , i) ; let i = FieldIdx :: from_usize (i) ; let src_field = self . project_field (src , i) ? ; let dst_field = self . project_field (dest , i) ? ; if src_field . layout . is_1zst () && cast_ty_field . is_1zst () { } else if src_field . layout . ty == cast_ty_field . ty { self . copy_op (& src_field , & dst_field) ? ; } else { if found_cast_field { span_bug ! (self . cur_span () , "unsize_into: more than one field to cast") ; } found_cast_field = true ; self . unsize_into (& src_field , cast_ty_field , & dst_field) ? ; } } interp_ok (()) } _ => { ensure_monomorphic_enough (* self . tcx , src . layout . ty) ? ; ensure_monomorphic_enough (* self . tcx , cast_ty . ty) ? ; span_bug ! (self . cur_span () , "unsize_into: invalid conversion: {:?} -> {:?}" , src . layout , dest . layout ()) } } } }}}