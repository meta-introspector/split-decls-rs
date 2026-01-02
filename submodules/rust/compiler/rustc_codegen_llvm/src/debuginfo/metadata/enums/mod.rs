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
mkuse!{use rustc_abi :: { FieldIdx , TagEncoding , VariantIdx , Variants } ;}
mkuse!{use rustc_codegen_ssa :: debuginfo :: type_names :: { compute_debuginfo_type_name , cpp_like_debuginfo } ;}
mkuse!{use rustc_codegen_ssa :: debuginfo :: { tag_base_type , wants_c_like_enum_debuginfo } ;}
mkuse!{use rustc_codegen_ssa :: traits :: MiscCodegenMethods ;}
mkuse!{use rustc_hir :: def :: CtorKind ;}
mkuse!{use rustc_index :: IndexSlice ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: mir :: CoroutineLayout ;}
mkuse!{use rustc_middle :: ty :: layout :: { LayoutOf , TyAndLayout } ;}
mkuse!{use rustc_middle :: ty :: { self , AdtDef , CoroutineArgs , CoroutineArgsExt , Ty , VariantDef } ;}
mkuse!{use rustc_span :: { Span , Symbol } ;}
mkuse!{use super :: type_map :: { DINodeCreationResult , UniqueTypeId } ;}
mkuse!{use super :: { SmallVec , size_and_align_of } ;}
mkuse!{use crate :: common :: { AsCCharPtr , CodegenCx } ;}
mkuse!{use crate :: debuginfo :: metadata :: type_map :: { self , Stub } ;}
mkuse!{use crate :: debuginfo :: metadata :: { UNKNOWN_LINE_NUMBER , build_field_di_node , build_generic_type_param_di_nodes , file_metadata_from_def_id , type_di_node , unknown_file_metadata , } ;}
mkuse!{use crate :: debuginfo :: utils :: { DIB , create_DIArray , get_namespace_for_item } ;}
mkuse!{use crate :: llvm :: debuginfo :: { DIFlags , DIType } ;}
mkuse!{use crate :: llvm :: { self } ;}
mkmod!{cpp_like, { 
                getname!(cpp_like);
                getsrc!(cpp_like);
                getpath!(cpp_like);
                get_deps!(cpp_like);
                get_crates!(cpp_like);
                mkinclude!(cpp_like);
                 
            }}
mkmod!{native, { 
                getname!(native);
                getsrc!(native);
                getpath!(native);
                get_deps!(native);
                get_crates!(native);
                mkinclude!(native);
                 
            }}

macro_rules! build_enum_type_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_enum_type_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_enum_type_di_node_introspect!();
    pub (super) fn build_enum_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , unique_type_id : UniqueTypeId < 'tcx > , span : Span ,) -> DINodeCreationResult < 'll > { let enum_type = unique_type_id . expect_ty () ; let & ty :: Adt (enum_adt_def , _) = enum_type . kind () else { bug ! ("build_enum_type_di_node() called with non-enum type: `{:?}`" , enum_type) } ; let enum_type_and_layout = cx . spanned_layout_of (enum_type , span) ; if wants_c_like_enum_debuginfo (cx . tcx , enum_type_and_layout) { return build_c_style_enum_di_node (cx , enum_adt_def , enum_type_and_layout) ; } if cpp_like_debuginfo (cx . tcx) { cpp_like :: build_enum_type_di_node (cx , unique_type_id) } else { native :: build_enum_type_di_node (cx , unique_type_id) } }
}

macro_rules! build_coroutine_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_coroutine_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_coroutine_di_node_introspect!();
    pub (super) fn build_coroutine_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , unique_type_id : UniqueTypeId < 'tcx > ,) -> DINodeCreationResult < 'll > { if cpp_like_debuginfo (cx . tcx) { cpp_like :: build_coroutine_di_node (cx , unique_type_id) } else { native :: build_coroutine_di_node (cx , unique_type_id) } }
}

macro_rules! build_c_style_enum_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_c_style_enum_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_c_style_enum_di_node_introspect!();
    # [doc = " Build the debuginfo node for a C-style enum, i.e. an enum the variants of which have no fields."] # [doc = ""] # [doc = " The resulting debuginfo will be a DW_TAG_enumeration_type."] fn build_c_style_enum_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , enum_adt_def : AdtDef < 'tcx > , enum_type_and_layout : TyAndLayout < 'tcx > ,) -> DINodeCreationResult < 'll > { let containing_scope = get_namespace_for_item (cx , enum_adt_def . did ()) ; let enum_adt_def_id = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { Some (enum_adt_def . did ()) } else { None } ; DINodeCreationResult { di_node : build_enumeration_type_di_node (cx , & compute_debuginfo_type_name (cx . tcx , enum_type_and_layout . ty , false) , tag_base_type (cx . tcx , enum_type_and_layout) , enum_adt_def . discriminants (cx . tcx) . map (| (variant_index , discr) | { let name = Cow :: from (enum_adt_def . variant (variant_index) . name . as_str ()) ; (name , discr . val) }) , enum_adt_def_id , containing_scope ,) , already_stored_in_typemap : false , } }
}

macro_rules! build_enumeration_type_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_enumeration_type_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_enumeration_type_di_node_introspect!();
    # [doc = " Build a DW_TAG_enumeration_type debuginfo node, with the given base type and variants."] # [doc = " This is a helper function and does not register anything in the type map by itself."] # [doc = ""] # [doc = " `variants` is an iterator of (discr-value, variant-name)."] fn build_enumeration_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , type_name : & str , base_type : Ty < 'tcx > , enumerators : impl Iterator < Item = (Cow < 'tcx , str > , u128) > , def_id : Option < rustc_span :: def_id :: DefId > , containing_scope : & 'll DIType ,) -> & 'll DIType { let is_unsigned = match base_type . kind () { ty :: Int (_) => false , ty :: Uint (_) => true , _ => bug ! ("build_enumeration_type_di_node() called with non-integer tag type.") , } ; let (size , align) = cx . size_and_align_of (base_type) ; let enumerator_di_nodes : SmallVec < Option < & 'll DIType > > = enumerators . map (| (name , value) | unsafe { let value = [value as u64 , (value >> 64) as u64] ; Some (llvm :: LLVMRustDIBuilderCreateEnumerator (DIB (cx) , name . as_c_char_ptr () , name . len () , value . as_ptr () , size . bits () as libc :: c_uint , is_unsigned ,)) }) . collect () ; let (file_metadata , line_number) = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { file_metadata_from_def_id (cx , def_id) } else { (unknown_file_metadata (cx) , UNKNOWN_LINE_NUMBER) } ; unsafe { llvm :: LLVMRustDIBuilderCreateEnumerationType (DIB (cx) , containing_scope , type_name . as_c_char_ptr () , type_name . len () , file_metadata , line_number , size . bits () , align . bits () as u32 , create_DIArray (DIB (cx) , & enumerator_di_nodes [..]) , type_di_node (cx , base_type) , true ,) } }
}

macro_rules! build_enum_variant_struct_type_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_enum_variant_struct_type_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_enum_variant_struct_type_di_node_introspect!();
    # [doc = " Build the debuginfo node for the struct type describing a single variant of an enum."] # [doc = ""] # [doc = " ```txt"] # [doc = "       DW_TAG_structure_type              (top-level type for enum)"] # [doc = "         DW_TAG_variant_part              (variant part)"] # [doc = "           DW_AT_discr                    (reference to discriminant DW_TAG_member)"] # [doc = "           DW_TAG_member                  (discriminant member)"] # [doc = "           DW_TAG_variant                 (variant 1)"] # [doc = "           DW_TAG_variant                 (variant 2)"] # [doc = "           DW_TAG_variant                 (variant 3)"] # [doc = "  --->   DW_TAG_structure_type            (type of variant 1)"] # [doc = "  --->   DW_TAG_structure_type            (type of variant 2)"] # [doc = "  --->   DW_TAG_structure_type            (type of variant 3)"] # [doc = " ```"] # [doc = ""] # [doc = " In CPP-like mode, we have the exact same descriptions for each variant too:"] # [doc = ""] # [doc = " ```txt"] # [doc = "       DW_TAG_union_type              (top-level type for enum)"] # [doc = "         DW_TAG_member                    (member for variant 1)"] # [doc = "         DW_TAG_member                    (member for variant 2)"] # [doc = "         DW_TAG_member                    (member for variant 3)"] # [doc = "  --->   DW_TAG_structure_type            (type of variant 1)"] # [doc = "  --->   DW_TAG_structure_type            (type of variant 2)"] # [doc = "  --->   DW_TAG_structure_type            (type of variant 3)"] # [doc = "         DW_TAG_enumeration_type          (type of tag)"] # [doc = " ```"] # [doc = ""] # [doc = " The node looks like:"] # [doc = ""] # [doc = " ```txt"] # [doc = " DW_TAG_structure_type"] # [doc = "   DW_AT_name                  <name-of-variant>"] # [doc = "   DW_AT_byte_size             0x00000010"] # [doc = "   DW_AT_alignment             0x00000008"] # [doc = "   DW_TAG_member"] # [doc = "     DW_AT_name                  <name-of-field-0>"] # [doc = "     DW_AT_type                  <0x0000018e>"] # [doc = "     DW_AT_alignment             0x00000004"] # [doc = "     DW_AT_data_member_location  4"] # [doc = "   DW_TAG_member"] # [doc = "     DW_AT_name                  <name-of-field-1>"] # [doc = "     DW_AT_type                  <0x00000195>"] # [doc = "     DW_AT_alignment             0x00000008"] # [doc = "     DW_AT_data_member_location  8"] # [doc = "   ..."] # [doc = " ```"] # [doc = ""] # [doc = " The type of a variant is always a struct type with the name of the variant"] # [doc = " and a DW_TAG_member for each field (but not the discriminant)."] fn build_enum_variant_struct_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , enum_type_and_layout : TyAndLayout < 'tcx > , enum_type_di_node : & 'll DIType , variant_index : VariantIdx , variant_def : & VariantDef , variant_layout : TyAndLayout < 'tcx > , di_flags : DIFlags ,) -> & 'll DIType { assert_eq ! (variant_layout . ty , enum_type_and_layout . ty) ; let def_location = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { Some (file_metadata_from_def_id (cx , Some (variant_def . def_id))) } else { None } ; type_map :: build_type_with_children (cx , type_map :: stub (cx , Stub :: Struct , UniqueTypeId :: for_enum_variant_struct_type (cx . tcx , enum_type_and_layout . ty , variant_index ,) , variant_def . name . as_str () , def_location , size_and_align_of (enum_type_and_layout) , Some (enum_type_di_node) , di_flags ,) , | cx , struct_type_di_node | { (0 .. variant_layout . fields . count ()) . map (| field_index | { let field_name = if variant_def . ctor_kind () != Some (CtorKind :: Fn) { let field = & variant_def . fields [FieldIdx :: from_usize (field_index)] ; Cow :: from (field . name . as_str ()) } else { super :: tuple_field_name (field_index) } ; let field_layout = variant_layout . field (cx , field_index) ; build_field_di_node (cx , struct_type_di_node , & field_name , field_layout , variant_layout . fields . offset (field_index) , di_flags , type_di_node (cx , field_layout . ty) , None ,) }) . collect :: < SmallVec < _ > > () } , | cx | build_generic_type_param_di_nodes (cx , enum_type_and_layout . ty) ,) . di_node }
}

macro_rules! build_coroutine_variant_struct_type_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_coroutine_variant_struct_type_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_coroutine_variant_struct_type_di_node_introspect!();
    # [doc = " Build the struct type for describing a single coroutine state."] # [doc = " See [build_coroutine_variant_struct_type_di_node]."] # [doc = ""] # [doc = " ```txt"] # [doc = ""] # [doc = "       DW_TAG_structure_type              (top-level type for enum)"] # [doc = "         DW_TAG_variant_part              (variant part)"] # [doc = "           DW_AT_discr                    (reference to discriminant DW_TAG_member)"] # [doc = "           DW_TAG_member                  (discriminant member)"] # [doc = "           DW_TAG_variant                 (variant 1)"] # [doc = "           DW_TAG_variant                 (variant 2)"] # [doc = "           DW_TAG_variant                 (variant 3)"] # [doc = "  --->   DW_TAG_structure_type            (type of variant 1)"] # [doc = "  --->   DW_TAG_structure_type            (type of variant 2)"] # [doc = "  --->   DW_TAG_structure_type            (type of variant 3)"] # [doc = ""] # [doc = " ```"] fn build_coroutine_variant_struct_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , variant_index : VariantIdx , coroutine_type_and_layout : TyAndLayout < 'tcx > , coroutine_type_di_node : & 'll DIType , coroutine_layout : & CoroutineLayout < 'tcx > , common_upvar_names : & IndexSlice < FieldIdx , Symbol > ,) -> & 'll DIType { let variant_name = CoroutineArgs :: variant_name (variant_index) ; let unique_type_id = UniqueTypeId :: for_enum_variant_struct_type (cx . tcx , coroutine_type_and_layout . ty , variant_index ,) ; let variant_layout = coroutine_type_and_layout . for_variant (cx , variant_index) ; let coroutine_args = match coroutine_type_and_layout . ty . kind () { ty :: Coroutine (_ , args) => args . as_coroutine () , _ => unreachable ! () , } ; type_map :: build_type_with_children (cx , type_map :: stub (cx , Stub :: Struct , unique_type_id , & variant_name , None , size_and_align_of (coroutine_type_and_layout) , Some (coroutine_type_di_node) , DIFlags :: FlagZero ,) , | cx , variant_struct_type_di_node | { let state_specific_fields : SmallVec < _ > = (0 .. variant_layout . fields . count ()) . map (| field_index | { let coroutine_saved_local = coroutine_layout . variant_fields [variant_index] [FieldIdx :: from_usize (field_index)] ; let field_name_maybe = coroutine_layout . field_names [coroutine_saved_local] ; let field_name = field_name_maybe . as_ref () . map (| s | Cow :: from (s . as_str ())) . unwrap_or_else (| | super :: tuple_field_name (field_index)) ; let field_type = variant_layout . field (cx , field_index) . ty ; build_field_di_node (cx , variant_struct_type_di_node , & field_name , cx . layout_of (field_type) , variant_layout . fields . offset (field_index) , DIFlags :: FlagZero , type_di_node (cx , field_type) , None ,) }) . collect () ; let common_fields : SmallVec < _ > = coroutine_args . prefix_tys () . iter () . zip (common_upvar_names) . enumerate () . map (| (index , (upvar_ty , upvar_name)) | { build_field_di_node (cx , variant_struct_type_di_node , upvar_name . as_str () , cx . layout_of (upvar_ty) , coroutine_type_and_layout . fields . offset (index) , DIFlags :: FlagZero , type_di_node (cx , upvar_ty) , None ,) }) . collect () ; state_specific_fields . into_iter () . chain (common_fields) . collect () } , | cx | build_generic_type_param_di_nodes (cx , coroutine_type_and_layout . ty) ,) . di_node }
}
mkitem!{mkenum!{# [derive (Copy , Clone)] enum DiscrResult { NoDiscriminant , Value (u128) , Range (u128 , u128) , }}}
mkitem!{mkimpl!{impl DiscrResult { fn opt_single_val (& self) -> Option < u128 > { if let Self :: Value (d) = * self { Some (d) } else { None } } }}}

macro_rules! compute_discriminant_value_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compute_discriminant_value in module {}", module_path!());
    };
}

mkfn!{
    compute_discriminant_value_introspect!();
    # [doc = " Returns the discriminant value corresponding to the variant index."] # [doc = ""] # [doc = " Will return `None` if there is less than two variants (because then the enum won't have)"] # [doc = " a tag, and if this is the untagged variant of a niche-layout enum (because then there is no"] # [doc = " single discriminant value)."] fn compute_discriminant_value < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , enum_type_and_layout : TyAndLayout < 'tcx > , variant_index : VariantIdx ,) -> DiscrResult { match enum_type_and_layout . layout . variants () { & Variants :: Single { .. } | & Variants :: Empty => DiscrResult :: NoDiscriminant , & Variants :: Multiple { tag_encoding : TagEncoding :: Direct , .. } => DiscrResult :: Value (enum_type_and_layout . ty . discriminant_for_variant (cx . tcx , variant_index) . unwrap () . val ,) , & Variants :: Multiple { tag_encoding : TagEncoding :: Niche { ref niche_variants , niche_start , untagged_variant } , tag , .. } => { if variant_index == untagged_variant { let valid_range = enum_type_and_layout . for_variant (cx , variant_index) . largest_niche . as_ref () . unwrap () . valid_range ; let min = valid_range . start . min (valid_range . end) ; let min = tag . size (cx) . truncate (min) ; let max = valid_range . start . max (valid_range . end) ; let max = tag . size (cx) . truncate (max) ; DiscrResult :: Range (min , max) } else { let value = (variant_index . as_u32 () as u128) . wrapping_sub (niche_variants . start () . as_u32 () as u128) . wrapping_add (niche_start) ; let value = tag . size (cx) . truncate (value) ; DiscrResult :: Value (value) } } } }
}