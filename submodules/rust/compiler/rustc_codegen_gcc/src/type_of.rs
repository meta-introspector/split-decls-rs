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
mkuse!{use std :: fmt :: Write ;}
mkuse!{use gccjit :: { Struct , Type } ;}
mkuse!{use rustc_abi as abi ;}
mkuse!{use rustc_abi :: Primitive :: * ;}
mkuse!{use rustc_abi :: { BackendRepr , FieldsShape , Integer , PointeeInfo , Reg , Size , TyAbiInterface , Variants , } ;}
mkuse!{use rustc_codegen_ssa :: traits :: { BaseTypeCodegenMethods , DerivedTypeCodegenMethods , LayoutTypeCodegenMethods , } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: layout :: { LayoutOf , TyAndLayout } ;}
mkuse!{use rustc_middle :: ty :: print :: with_no_trimmed_paths ;}
mkuse!{use rustc_middle :: ty :: { self , CoroutineArgsExt , Ty , TypeVisitableExt } ;}
mkuse!{use rustc_target :: callconv :: { CastTarget , FnAbi } ;}
mkuse!{use crate :: abi :: { FnAbiGcc , FnAbiGccExt , GccType } ;}
mkuse!{use crate :: context :: CodegenCx ;}
mkuse!{use crate :: type_ :: struct_fields ;}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > CodegenCx < 'gcc , 'tcx > { fn type_from_unsigned_integer (& self , i : Integer) -> Type < 'gcc > { use Integer :: * ; match i { I8 => self . type_u8 () , I16 => self . type_u16 () , I32 => self . type_u32 () , I64 => self . type_u64 () , I128 => self . type_u128 () , } } # [cfg (feature = "master")] pub fn type_int_from_ty (& self , t : ty :: IntTy) -> Type < 'gcc > { match t { ty :: IntTy :: Isize => self . type_isize () , ty :: IntTy :: I8 => self . type_i8 () , ty :: IntTy :: I16 => self . type_i16 () , ty :: IntTy :: I32 => self . type_i32 () , ty :: IntTy :: I64 => self . type_i64 () , ty :: IntTy :: I128 => self . type_i128 () , } } # [cfg (feature = "master")] pub fn type_uint_from_ty (& self , t : ty :: UintTy) -> Type < 'gcc > { match t { ty :: UintTy :: Usize => self . type_isize () , ty :: UintTy :: U8 => self . type_i8 () , ty :: UintTy :: U16 => self . type_i16 () , ty :: UintTy :: U32 => self . type_i32 () , ty :: UintTy :: U64 => self . type_i64 () , ty :: UintTy :: U128 => self . type_i128 () , } } }}}

macro_rules! uncached_gcc_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function uncached_gcc_type in module {}", module_path!());
    };
}

mkfn!{
    uncached_gcc_type_introspect!();
    fn uncached_gcc_type < 'gcc , 'tcx > (cx : & CodegenCx < 'gcc , 'tcx > , layout : TyAndLayout < 'tcx > , defer : & mut Option < (Struct < 'gcc > , TyAndLayout < 'tcx >) > ,) -> Type < 'gcc > { match layout . backend_repr { BackendRepr :: Scalar (_) => bug ! ("handled elsewhere") , BackendRepr :: SimdVector { ref element , count } => { let element = layout . scalar_gcc_type_at (cx , element , Size :: ZERO) ; let element = if element . get_pointee () . is_some () { cx . usize_type } else { element } ; return cx . context . new_vector_type (element , count) ; } BackendRepr :: ScalarPair (..) => { return cx . type_struct (& [layout . scalar_pair_element_gcc_type (cx , 0) , layout . scalar_pair_element_gcc_type (cx , 1) ,] , false ,) ; } BackendRepr :: Memory { .. } => { } } let name = match * layout . ty . kind () { ty :: Adt (..) | ty :: Closure (..) | ty :: CoroutineClosure (..) | ty :: Foreign (..) | ty :: Coroutine (..) | ty :: Str if ! cx . sess () . fewer_names () => { let mut name = with_no_trimmed_paths ! (layout . ty . to_string ()) ; if let (& ty :: Adt (def , _) , & Variants :: Single { index }) = (layout . ty . kind () , & layout . variants) && def . is_enum () && ! def . variants () . is_empty () { write ! (& mut name , "::{}" , def . variant (index) . name) . unwrap () ; } if let (& ty :: Coroutine (_ , _) , & Variants :: Single { index }) = (layout . ty . kind () , & layout . variants) { write ! (& mut name , "::{}" , ty :: CoroutineArgs :: variant_name (index)) . unwrap () ; } Some (name) } ty :: Adt (..) => { Some (String :: new ()) } _ => None , } ; match layout . fields { FieldsShape :: Primitive | FieldsShape :: Union (_) => { let fill = cx . type_padding_filler (layout . size , layout . align . abi) ; let packed = false ; match name { None => cx . type_struct (& [fill] , packed) , Some (ref name) => { let gcc_type = cx . type_named_struct (name) ; cx . set_struct_body (gcc_type , & [fill] , packed) ; gcc_type . as_type () } } } FieldsShape :: Array { count , .. } => cx . type_array (layout . field (cx , 0) . gcc_type (cx) , count) , FieldsShape :: Arbitrary { .. } => match name { None => { let (gcc_fields , packed) = struct_fields (cx , layout) ; cx . type_struct (& gcc_fields , packed) } Some (ref name) => { let gcc_type = cx . type_named_struct (name) ; * defer = Some ((gcc_type , layout)) ; gcc_type . as_type () } } , } }
}
mkitem!{mktrait!{pub trait LayoutGccExt < 'tcx > { fn is_gcc_immediate (& self) -> bool ; fn is_gcc_scalar_pair (& self) -> bool ; fn gcc_type < 'gcc > (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> Type < 'gcc > ; fn immediate_gcc_type < 'gcc > (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> Type < 'gcc > ; fn scalar_gcc_type_at < 'gcc > (& self , cx : & CodegenCx < 'gcc , 'tcx > , scalar : & abi :: Scalar , offset : Size ,) -> Type < 'gcc > ; fn scalar_pair_element_gcc_type < 'gcc > (& self , cx : & CodegenCx < 'gcc , 'tcx > , index : usize ,) -> Type < 'gcc > ; fn pointee_info_at < 'gcc > (& self , cx : & CodegenCx < 'gcc , 'tcx > , offset : Size ,) -> Option < PointeeInfo > ; }}}
mkitem!{mkimpl!{impl < 'tcx > LayoutGccExt < 'tcx > for TyAndLayout < 'tcx > { fn is_gcc_immediate (& self) -> bool { match self . backend_repr { BackendRepr :: Scalar (_) | BackendRepr :: SimdVector { .. } => true , BackendRepr :: ScalarPair (..) | BackendRepr :: Memory { .. } => false , } } fn is_gcc_scalar_pair (& self) -> bool { match self . backend_repr { BackendRepr :: ScalarPair (..) => true , BackendRepr :: Scalar (_) | BackendRepr :: SimdVector { .. } | BackendRepr :: Memory { .. } => false , } } # [doc = " Gets the GCC type corresponding to a Rust type, i.e., `rustc_middle::ty::Ty`."] # [doc = " The pointee type of the pointer in `PlaceRef` is always this type."] # [doc = " For sized types, it is also the right LLVM type for an `alloca`"] # [doc = " containing a value of that type, and most immediates (except `bool`)."] # [doc = " Unsized types, however, are represented by a \"minimal unit\", e.g."] # [doc = " `[T]` becomes `T`, while `str` and `Trait` turn into `i8` - this"] # [doc = " is useful for indexing slices, as `&[T]`'s data pointer is `T*`."] # [doc = " If the type is an unsized struct, the regular layout is generated,"] # [doc = " with the innermost trailing unsized field using the \"minimal unit\""] # [doc = " of that field's type - this is useful for taking the address of"] # [doc = " that field and ensuring the struct has the right alignment."] fn gcc_type < 'gcc > (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> Type < 'gcc > { use rustc_middle :: ty :: layout :: FnAbiOf ; if let BackendRepr :: Scalar (ref scalar) = self . backend_repr { if let Some (& ty) = cx . scalar_types . borrow () . get (& self . ty) { return ty ; } let ty = match * self . ty . kind () { ty :: FnPtr (sig_tys , hdr) => cx . fn_ptr_backend_type (cx . fn_abi_of_fn_ptr (sig_tys . with (hdr) , ty :: List :: empty ())) , _ => self . scalar_gcc_type_at (cx , scalar , Size :: ZERO) , } ; cx . scalar_types . borrow_mut () . insert (self . ty , ty) ; return ty ; } let variant_index = match self . variants { Variants :: Single { index } => Some (index) , _ => None , } ; let cached_type = cx . types . borrow () . get (& (self . ty , variant_index)) . cloned () ; if let Some (ty) = cached_type { return ty ; } assert ! (! self . ty . has_escaping_bound_vars () , "{:?} has escaping bound vars" , self . ty) ; let normal_ty = cx . tcx . erase_and_anonymize_regions (self . ty) ; let mut defer = None ; let ty = if self . ty != normal_ty { let mut layout = cx . layout_of (normal_ty) ; if let Some (v) = variant_index { layout = layout . for_variant (cx , v) ; } layout . gcc_type (cx) } else { uncached_gcc_type (cx , * self , & mut defer) } ; cx . types . borrow_mut () . insert ((self . ty , variant_index) , ty) ; if let Some ((deferred_ty , layout)) = defer { let (fields , packed) = struct_fields (cx , layout) ; cx . set_struct_body (deferred_ty , & fields , packed) ; } ty } fn immediate_gcc_type < 'gcc > (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> Type < 'gcc > { if let BackendRepr :: Scalar (ref scalar) = self . backend_repr && scalar . is_bool () { return cx . type_i1 () ; } self . gcc_type (cx) } fn scalar_gcc_type_at < 'gcc > (& self , cx : & CodegenCx < 'gcc , 'tcx > , scalar : & abi :: Scalar , offset : Size ,) -> Type < 'gcc > { match scalar . primitive () { Int (i , true) => cx . type_from_integer (i) , Int (i , false) => cx . type_from_unsigned_integer (i) , Float (f) => cx . type_from_float (f) , Pointer (address_space) => { let pointee = if let Some (pointee) = self . pointee_info_at (cx , offset) { cx . type_pointee_for_align (pointee . align) } else { cx . type_i8 () } ; cx . type_ptr_to_ext (pointee , address_space) } } } fn scalar_pair_element_gcc_type < 'gcc > (& self , cx : & CodegenCx < 'gcc , 'tcx > , index : usize ,) -> Type < 'gcc > { let (a , b) = match self . backend_repr { BackendRepr :: ScalarPair (ref a , ref b) => (a , b) , _ => bug ! ("TyAndLayout::scalar_pair_element_llty({:?}): not applicable" , self) , } ; let scalar = [a , b] [index] ; if scalar . is_bool () { return cx . type_i1 () ; } let offset = if index == 0 { Size :: ZERO } else { a . size (cx) . align_to (b . align (cx) . abi) } ; self . scalar_gcc_type_at (cx , scalar , offset) } fn pointee_info_at < 'a > (& self , cx : & CodegenCx < 'a , 'tcx > , offset : Size) -> Option < PointeeInfo > { if let Some (& pointee) = cx . pointee_infos . borrow () . get (& (self . ty , offset)) { return pointee ; } let result = Ty :: ty_and_layout_pointee_info_at (* self , cx , offset) ; cx . pointee_infos . borrow_mut () . insert ((self . ty , offset) , result) ; result } }}}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > LayoutTypeCodegenMethods < 'tcx > for CodegenCx < 'gcc , 'tcx > { fn backend_type (& self , layout : TyAndLayout < 'tcx >) -> Type < 'gcc > { layout . gcc_type (self) } fn immediate_backend_type (& self , layout : TyAndLayout < 'tcx >) -> Type < 'gcc > { layout . immediate_gcc_type (self) } fn is_backend_immediate (& self , layout : TyAndLayout < 'tcx >) -> bool { layout . is_gcc_immediate () } fn is_backend_scalar_pair (& self , layout : TyAndLayout < 'tcx >) -> bool { layout . is_gcc_scalar_pair () } fn scalar_pair_element_backend_type (& self , layout : TyAndLayout < 'tcx > , index : usize , _immediate : bool ,) -> Type < 'gcc > { layout . scalar_pair_element_gcc_type (self , index) } fn cast_backend_type (& self , ty : & CastTarget) -> Type < 'gcc > { ty . gcc_type (self) } fn fn_ptr_backend_type (& self , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > >) -> Type < 'gcc > { fn_abi . ptr_to_gcc_type (self) } fn reg_backend_type (& self , _ty : & Reg) -> Type < 'gcc > { unimplemented ! () ; } fn fn_decl_backend_type (& self , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > >) -> Type < 'gcc > { let FnAbiGcc { return_type , arguments_type , is_c_variadic , .. } = fn_abi . gcc_type (self) ; self . context . new_function_pointer_type (None , return_type , & arguments_type , is_c_variadic) } }}}