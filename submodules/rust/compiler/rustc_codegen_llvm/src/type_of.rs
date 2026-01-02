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
mkuse!{use rustc_abi :: Primitive :: { Float , Int , Pointer } ;}
mkuse!{use rustc_abi :: { Align , BackendRepr , FieldsShape , Scalar , Size , Variants } ;}
mkuse!{use rustc_codegen_ssa :: traits :: * ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: layout :: { LayoutOf , TyAndLayout } ;}
mkuse!{use rustc_middle :: ty :: print :: { with_no_trimmed_paths , with_no_visible_paths } ;}
mkuse!{use rustc_middle :: ty :: { self , CoroutineArgsExt , Ty , TypeVisitableExt } ;}
mkuse!{use rustc_span :: { DUMMY_SP , Span } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: common :: * ;}
mkuse!{use crate :: type_ :: Type ;}

macro_rules! uncached_llvm_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function uncached_llvm_type in module {}", module_path!());
    };
}

mkfn!{
    uncached_llvm_type_introspect!();
    fn uncached_llvm_type < 'a , 'tcx > (cx : & CodegenCx < 'a , 'tcx > , layout : TyAndLayout < 'tcx > , defer : & mut Option < (& 'a Type , TyAndLayout < 'tcx >) > ,) -> & 'a Type { match layout . backend_repr { BackendRepr :: Scalar (_) => bug ! ("handled elsewhere") , BackendRepr :: SimdVector { element , count } => { let element = layout . scalar_llvm_type_at (cx , element) ; return cx . type_vector (element , count) ; } BackendRepr :: Memory { .. } | BackendRepr :: ScalarPair (..) => { } } let name = match layout . ty . kind () { ty :: Adt (..) | ty :: Closure (..) | ty :: CoroutineClosure (..) | ty :: Foreign (..) | ty :: Coroutine (..) | ty :: Str if ! cx . sess () . fewer_names () => { let mut name = with_no_visible_paths ! (with_no_trimmed_paths ! (layout . ty . to_string ())) ; if let (& ty :: Adt (def , _) , & Variants :: Single { index }) = (layout . ty . kind () , & layout . variants) { if def . is_enum () { write ! (& mut name , "::{}" , def . variant (index) . name) . unwrap () ; } } if let (& ty :: Coroutine (_ , _) , & Variants :: Single { index }) = (layout . ty . kind () , & layout . variants) { write ! (& mut name , "::{}" , ty :: CoroutineArgs :: variant_name (index)) . unwrap () ; } Some (name) } _ => None , } ; match layout . fields { FieldsShape :: Primitive | FieldsShape :: Union (_) => { let fill = cx . type_padding_filler (layout . size , layout . align . abi) ; let packed = false ; match name { None => cx . type_struct (& [fill] , packed) , Some (ref name) => { let llty = cx . type_named_struct (name) ; cx . set_struct_body (llty , & [fill] , packed) ; llty } } } FieldsShape :: Array { count , .. } => cx . type_array (layout . field (cx , 0) . llvm_type (cx) , count) , FieldsShape :: Arbitrary { .. } => match name { None => { let (llfields , packed) = struct_llfields (cx , layout) ; cx . type_struct (& llfields , packed) } Some (ref name) => { let llty = cx . type_named_struct (name) ; * defer = Some ((llty , layout)) ; llty } } , } }
}

macro_rules! struct_llfields_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function struct_llfields in module {}", module_path!());
    };
}

mkfn!{
    struct_llfields_introspect!();
    fn struct_llfields < 'a , 'tcx > (cx : & CodegenCx < 'a , 'tcx > , layout : TyAndLayout < 'tcx > ,) -> (Vec < & 'a Type > , bool) { debug ! ("struct_llfields: {:#?}" , layout) ; let field_count = layout . fields . count () ; let mut packed = false ; let mut offset = Size :: ZERO ; let mut prev_effective_align = layout . align . abi ; let mut result : Vec < _ > = Vec :: with_capacity (1 + field_count * 2) ; for i in layout . fields . index_by_increasing_offset () { let target_offset = layout . fields . offset (i as usize) ; let field = layout . field (cx , i) ; let effective_field_align = layout . align . abi . min (field . align . abi) . restrict_for_offset (target_offset) ; packed |= effective_field_align < field . align . abi ; debug ! ("struct_llfields: {}: {:?} offset: {:?} target_offset: {:?} \
                effective_field_align: {}" , i , field , offset , target_offset , effective_field_align . bytes ()) ; assert ! (target_offset >= offset) ; let padding = target_offset - offset ; if padding != Size :: ZERO { let padding_align = prev_effective_align . min (effective_field_align) ; assert_eq ! (offset . align_to (padding_align) + padding , target_offset) ; result . push (cx . type_padding_filler (padding , padding_align)) ; debug ! ("    padding before: {:?}" , padding) ; } result . push (field . llvm_type (cx)) ; offset = target_offset + field . size ; prev_effective_align = effective_field_align ; } if layout . is_sized () && field_count > 0 { if offset > layout . size { bug ! ("layout: {:#?} stride: {:?} offset: {:?}" , layout , layout . size , offset) ; } let padding = layout . size - offset ; if padding != Size :: ZERO { let padding_align = prev_effective_align ; assert_eq ! (offset . align_to (padding_align) + padding , layout . size) ; debug ! ("struct_llfields: pad_bytes: {:?} offset: {:?} stride: {:?}" , padding , offset , layout . size) ; result . push (cx . type_padding_filler (padding , padding_align)) ; } } else { debug ! ("struct_llfields: offset: {:?} stride: {:?}" , offset , layout . size) ; } (result , packed) }
}
mkitem!{mkimpl!{impl < 'a , 'tcx > CodegenCx < 'a , 'tcx > { pub (crate) fn align_of (& self , ty : Ty < 'tcx >) -> Align { self . layout_of (ty) . align . abi } pub (crate) fn size_of (& self , ty : Ty < 'tcx >) -> Size { self . layout_of (ty) . size } pub (crate) fn size_and_align_of (& self , ty : Ty < 'tcx >) -> (Size , Align) { self . spanned_size_and_align_of (ty , DUMMY_SP) } pub (crate) fn spanned_size_and_align_of (& self , ty : Ty < 'tcx > , span : Span) -> (Size , Align) { let layout = self . spanned_layout_of (ty , span) ; (layout . size , layout . align . abi) } }}}
mkitem!{mktrait!{pub (crate) trait LayoutLlvmExt < 'tcx > { fn is_llvm_immediate (& self) -> bool ; fn is_llvm_scalar_pair (& self) -> bool ; fn llvm_type < 'a > (& self , cx : & CodegenCx < 'a , 'tcx >) -> & 'a Type ; fn immediate_llvm_type < 'a > (& self , cx : & CodegenCx < 'a , 'tcx >) -> & 'a Type ; fn scalar_llvm_type_at < 'a > (& self , cx : & CodegenCx < 'a , 'tcx > , scalar : Scalar) -> & 'a Type ; fn scalar_pair_element_llvm_type < 'a > (& self , cx : & CodegenCx < 'a , 'tcx > , index : usize , immediate : bool ,) -> & 'a Type ; }}}
mkitem!{mkimpl!{impl < 'tcx > LayoutLlvmExt < 'tcx > for TyAndLayout < 'tcx > { fn is_llvm_immediate (& self) -> bool { match self . backend_repr { BackendRepr :: Scalar (_) | BackendRepr :: SimdVector { .. } => true , BackendRepr :: ScalarPair (..) | BackendRepr :: Memory { .. } => false , } } fn is_llvm_scalar_pair (& self) -> bool { match self . backend_repr { BackendRepr :: ScalarPair (..) => true , BackendRepr :: Scalar (_) | BackendRepr :: SimdVector { .. } | BackendRepr :: Memory { .. } => false , } } # [doc = " Gets the LLVM type corresponding to a Rust type, i.e., `rustc_middle::ty::Ty`."] # [doc = " The pointee type of the pointer in `PlaceRef` is always this type."] # [doc = " For sized types, it is also the right LLVM type for an `alloca`"] # [doc = " containing a value of that type, and most immediates (except `bool`)."] # [doc = " Unsized types, however, are represented by a \"minimal unit\", e.g."] # [doc = " `[T]` becomes `T`, while `str` and `Trait` turn into `i8` - this"] # [doc = " is useful for indexing slices, as `&[T]`'s data pointer is `T*`."] # [doc = " If the type is an unsized struct, the regular layout is generated,"] # [doc = " with the innermost trailing unsized field using the \"minimal unit\""] # [doc = " of that field's type - this is useful for taking the address of"] # [doc = " that field and ensuring the struct has the right alignment."] fn llvm_type < 'a > (& self , cx : & CodegenCx < 'a , 'tcx >) -> & 'a Type { if let BackendRepr :: Scalar (scalar) = self . backend_repr { if let Some (& llty) = cx . scalar_lltypes . borrow () . get (& self . ty) { return llty ; } let llty = self . scalar_llvm_type_at (cx , scalar) ; cx . scalar_lltypes . borrow_mut () . insert (self . ty , llty) ; return llty ; } let variant_index = match self . variants { Variants :: Single { index } => Some (index) , _ => None , } ; if let Some (llty) = cx . type_lowering . borrow () . get (& (self . ty , variant_index)) { return llty ; } debug ! ("llvm_type({:#?})" , self) ; assert ! (! self . ty . has_escaping_bound_vars () , "{:?} has escaping bound vars" , self . ty) ; let normal_ty = cx . tcx . erase_and_anonymize_regions (self . ty) ; let mut defer = None ; let llty = if self . ty != normal_ty { let mut layout = cx . layout_of (normal_ty) ; if let Some (v) = variant_index { layout = layout . for_variant (cx , v) ; } layout . llvm_type (cx) } else { uncached_llvm_type (cx , * self , & mut defer) } ; debug ! ("--> mapped {:#?} to llty={:?}" , self , llty) ; cx . type_lowering . borrow_mut () . insert ((self . ty , variant_index) , llty) ; if let Some ((llty , layout)) = defer { let (llfields , packed) = struct_llfields (cx , layout) ; cx . set_struct_body (llty , & llfields , packed) ; } llty } fn immediate_llvm_type < 'a > (& self , cx : & CodegenCx < 'a , 'tcx >) -> & 'a Type { match self . backend_repr { BackendRepr :: Scalar (scalar) => { if scalar . is_bool () { return cx . type_i1 () ; } } BackendRepr :: ScalarPair (..) => { return cx . type_struct (& [self . scalar_pair_element_llvm_type (cx , 0 , true) , self . scalar_pair_element_llvm_type (cx , 1 , true) ,] , false ,) ; } _ => { } } ; self . llvm_type (cx) } fn scalar_llvm_type_at < 'a > (& self , cx : & CodegenCx < 'a , 'tcx > , scalar : Scalar) -> & 'a Type { match scalar . primitive () { Int (i , _) => cx . type_from_integer (i) , Float (f) => cx . type_from_float (f) , Pointer (address_space) => cx . type_ptr_ext (address_space) , } } fn scalar_pair_element_llvm_type < 'a > (& self , cx : & CodegenCx < 'a , 'tcx > , index : usize , immediate : bool ,) -> & 'a Type { let BackendRepr :: ScalarPair (a , b) = self . backend_repr else { bug ! ("TyAndLayout::scalar_pair_element_llty({:?}): not applicable" , self) ; } ; let scalar = [a , b] [index] ; if immediate && scalar . is_bool () { return cx . type_i1 () ; } self . scalar_llvm_type_at (cx , scalar) } }}}