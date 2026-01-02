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
mkuse!{use std :: ops :: Range ;}
mkuse!{use rustc_abi :: { Align , HasDataLayout , Primitive , Scalar , Size , WrappingRange } ;}
mkuse!{use rustc_codegen_ssa :: common ;}
mkuse!{use rustc_codegen_ssa :: traits :: * ;}
mkuse!{use rustc_hir :: LangItem ;}
mkuse!{use rustc_hir :: attrs :: Linkage ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LOCAL_CRATE } ;}
mkuse!{use rustc_middle :: middle :: codegen_fn_attrs :: { CodegenFnAttrFlags , CodegenFnAttrs } ;}
mkuse!{use rustc_middle :: mir :: interpret :: { Allocation , ConstAllocation , ErrorHandled , InitChunk , Pointer , Scalar as InterpScalar , read_target_uint , } ;}
mkuse!{use rustc_middle :: mir :: mono :: MonoItem ;}
mkuse!{use rustc_middle :: ty :: layout :: { HasTypingEnv , LayoutOf } ;}
mkuse!{use rustc_middle :: ty :: { self , Instance } ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use tracing :: { debug , instrument , trace } ;}
mkuse!{use crate :: common :: CodegenCx ;}
mkuse!{use crate :: errors :: SymbolAlreadyDefined ;}
mkuse!{use crate :: type_ :: Type ;}
mkuse!{use crate :: type_of :: LayoutLlvmExt ;}
mkuse!{use crate :: value :: Value ;}
mkuse!{use crate :: { base , debuginfo , llvm } ;}

macro_rules! const_alloc_to_llvm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function const_alloc_to_llvm in module {}", module_path!());
    };
}

mkfn!{
    const_alloc_to_llvm_introspect!();
    pub (crate) fn const_alloc_to_llvm < 'll > (cx : & CodegenCx < 'll , '_ > , alloc : & Allocation , is_static : bool ,) -> & 'll Value { if ! is_static { assert ! (alloc . len () != 0) ; } let mut llvals = Vec :: with_capacity (alloc . provenance () . ptrs () . len () + 1) ; let dl = cx . data_layout () ; let pointer_size = dl . pointer_size () ; let pointer_size_bytes = pointer_size . bytes () as usize ; fn append_chunks_of_init_and_uninit_bytes < 'll , 'a , 'b > (llvals : & mut Vec < & 'll Value > , cx : & 'a CodegenCx < 'll , 'b > , alloc : & 'a Allocation , range : Range < usize > ,) { let chunks = alloc . init_mask () . range_as_init_chunks (range . clone () . into ()) ; let chunk_to_llval = move | chunk | match chunk { InitChunk :: Init (range) => { let range = (range . start . bytes () as usize) .. (range . end . bytes () as usize) ; let bytes = alloc . inspect_with_uninit_and_ptr_outside_interpreter (range) ; cx . const_bytes (bytes) } InitChunk :: Uninit (range) => { let len = range . end . bytes () - range . start . bytes () ; cx . const_undef (cx . type_array (cx . type_i8 () , len)) } } ; let max = cx . sess () . opts . unstable_opts . uninit_const_chunk_threshold ; let allow_uninit_chunks = chunks . clone () . take (max . saturating_add (1)) . count () <= max ; if allow_uninit_chunks { llvals . extend (chunks . map (chunk_to_llval)) ; } else { let bytes = alloc . inspect_with_uninit_and_ptr_outside_interpreter (range) ; llvals . push (cx . const_bytes (bytes)) ; } } let mut next_offset = 0 ; for & (offset , prov) in alloc . provenance () . ptrs () . iter () { let offset = offset . bytes () ; assert_eq ! (offset as usize as u64 , offset) ; let offset = offset as usize ; if offset > next_offset { append_chunks_of_init_and_uninit_bytes (& mut llvals , cx , alloc , next_offset .. offset) ; } let ptr_offset = read_target_uint (dl . endian , alloc . inspect_with_uninit_and_ptr_outside_interpreter (offset .. (offset + pointer_size_bytes) ,) ,) . expect ("const_alloc_to_llvm: could not read relocation pointer") as u64 ; let address_space = cx . tcx . global_alloc (prov . alloc_id ()) . address_space (cx) ; llvals . push (cx . scalar_to_backend (InterpScalar :: from_pointer (Pointer :: new (prov , Size :: from_bytes (ptr_offset)) , & cx . tcx) , Scalar :: Initialized { value : Primitive :: Pointer (address_space) , valid_range : WrappingRange :: full (pointer_size) , } , cx . type_ptr_ext (address_space) ,)) ; next_offset = offset + pointer_size_bytes ; } if alloc . len () >= next_offset { let range = next_offset .. alloc . len () ; append_chunks_of_init_and_uninit_bytes (& mut llvals , cx , alloc , range) ; } if let & [data] = & * llvals { data } else { cx . const_struct (& llvals , true) } }
}

macro_rules! codegen_static_initializer_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function codegen_static_initializer in module {}", module_path!());
    };
}

mkfn!{
    codegen_static_initializer_introspect!();
    fn codegen_static_initializer < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , def_id : DefId ,) -> Result < (& 'll Value , ConstAllocation < 'tcx >) , ErrorHandled > { let alloc = cx . tcx . eval_static_initializer (def_id) ? ; Ok ((const_alloc_to_llvm (cx , alloc . inner () , true) , alloc)) }
}

macro_rules! set_global_alignment_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_global_alignment in module {}", module_path!());
    };
}

mkfn!{
    set_global_alignment_introspect!();
    fn set_global_alignment < 'll > (cx : & CodegenCx < 'll , '_ > , gv : & 'll Value , mut align : Align) { if let Some (min_global) = cx . sess () . target . min_global_align { align = Ord :: max (align , min_global) ; } llvm :: set_alignment (gv , align) ; }
}

macro_rules! check_and_apply_linkage_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_and_apply_linkage in module {}", module_path!());
    };
}

mkfn!{
    check_and_apply_linkage_introspect!();
    fn check_and_apply_linkage < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , attrs : & CodegenFnAttrs , llty : & 'll Type , sym : & str , def_id : DefId ,) -> & 'll Value { if let Some (linkage) = attrs . import_linkage { debug ! ("get_static: sym={} linkage={:?}" , sym , linkage) ; let g1 = if matches ! (attrs . import_linkage , Some (Linkage :: ExternalWeak)) { let instance = Instance :: mono (cx . tcx , def_id) ; if let ty :: Adt (struct_def , args) = instance . ty (cx . tcx , cx . typing_env ()) . kind () && cx . tcx . is_lang_item (struct_def . did () , LangItem :: Option) && let ty :: FnPtr (sig , header) = args . type_at (0) . kind () { let fn_sig = sig . with (* header) ; let fn_abi = cx . fn_abi_of_fn_ptr (fn_sig , ty :: List :: empty ()) ; cx . declare_fn (sym , & fn_abi , None) } else { cx . declare_global (sym , cx . type_i8 ()) } } else { cx . declare_global (sym , cx . type_i8 ()) } ; llvm :: set_linkage (g1 , base :: linkage_to_llvm (linkage)) ; let real_name = format ! ("_rust_extern_with_linkage_{:016x}_{sym}" , cx . tcx . stable_crate_id (LOCAL_CRATE)) ; let g2 = cx . define_global (& real_name , llty) . unwrap_or_else (| | { cx . sess () . dcx () . emit_fatal (SymbolAlreadyDefined { span : cx . tcx . def_span (def_id) , symbol_name : sym , }) }) ; llvm :: set_linkage (g2 , llvm :: Linkage :: InternalLinkage) ; llvm :: set_initializer (g2 , g1) ; g2 } else if cx . tcx . sess . target . arch == "x86" && common :: is_mingw_gnu_toolchain (& cx . tcx . sess . target) && let Some (dllimport) = crate :: common :: get_dllimport (cx . tcx , def_id , sym) { cx . declare_global (& common :: i686_decorated_name (dllimport , true , true , false) , llty) } else { cx . declare_global (sym , llty) } }
}
mkitem!{mkimpl!{impl < 'll > CodegenCx < 'll , '_ > { pub (crate) fn const_bitcast (& self , val : & 'll Value , ty : & 'll Type) -> & 'll Value { unsafe { llvm :: LLVMConstBitCast (val , ty) } } pub (crate) fn const_pointercast (& self , val : & 'll Value , ty : & 'll Type) -> & 'll Value { unsafe { llvm :: LLVMConstPointerCast (val , ty) } } # [doc = " Create a global variable."] # [doc = ""] # [doc = " The returned global variable is a pointer in the default address space for globals."] # [doc = " Fails if a symbol with the given name already exists."] pub (crate) fn static_addr_of_mut (& self , cv : & 'll Value , align : Align , kind : Option < & str > ,) -> & 'll Value { let gv = match kind { Some (kind) if ! self . tcx . sess . fewer_names () => { let name = self . generate_local_symbol_name (kind) ; let gv = self . define_global (& name , self . val_ty (cv)) . unwrap_or_else (| | { bug ! ("symbol `{}` is already defined" , name) ; }) ; llvm :: set_linkage (gv , llvm :: Linkage :: PrivateLinkage) ; gv } _ => self . define_private_global (self . val_ty (cv)) , } ; llvm :: set_initializer (gv , cv) ; set_global_alignment (self , gv , align) ; llvm :: set_unnamed_address (gv , llvm :: UnnamedAddr :: Global) ; gv } # [doc = " Create a global constant."] # [doc = ""] # [doc = " The returned global variable is a pointer in the default address space for globals."] pub (crate) fn static_addr_of_impl (& self , cv : & 'll Value , align : Align , kind : Option < & str > ,) -> & 'll Value { if let Some (& gv) = self . const_globals . borrow () . get (& cv) { unsafe { let llalign = align . bytes () as u32 ; if llalign > llvm :: LLVMGetAlignment (gv) { llvm :: LLVMSetAlignment (gv , llalign) ; } } return gv ; } let gv = self . static_addr_of_mut (cv , align , kind) ; llvm :: set_global_constant (gv , true) ; self . const_globals . borrow_mut () . insert (cv , gv) ; gv } # [instrument (level = "debug" , skip (self))] pub (crate) fn get_static (& self , def_id : DefId) -> & 'll Value { let instance = Instance :: mono (self . tcx , def_id) ; trace ! (? instance) ; let DefKind :: Static { nested , .. } = self . tcx . def_kind (def_id) else { bug ! () } ; let llty = if nested { self . type_i8 () } else { let ty = instance . ty (self . tcx , self . typing_env ()) ; trace ! (? ty) ; self . layout_of (ty) . llvm_type (self) } ; self . get_static_inner (def_id , llty) } # [instrument (level = "debug" , skip (self , llty))] fn get_static_inner (& self , def_id : DefId , llty : & 'll Type) -> & 'll Value { let instance = Instance :: mono (self . tcx , def_id) ; if let Some (& g) = self . instances . borrow () . get (& instance) { trace ! ("used cached value") ; return g ; } let defined_in_current_codegen_unit = self . codegen_unit . items () . contains_key (& MonoItem :: Static (def_id)) ; assert ! (! defined_in_current_codegen_unit , "consts::get_static() should always hit the cache for \
                 statics defined in the same CGU, but did not for `{def_id:?}`") ; let sym = self . tcx . symbol_name (instance) . name ; let fn_attrs = self . tcx . codegen_fn_attrs (def_id) ; debug ! (? sym , ? fn_attrs) ; let g = if def_id . is_local () && ! self . tcx . is_foreign_item (def_id) { if let Some (g) = self . get_declared_value (sym) { if self . val_ty (g) != self . type_ptr () { span_bug ! (self . tcx . def_span (def_id) , "Conflicting types for static") ; } } let g = self . declare_global (sym , llty) ; if ! self . tcx . is_reachable_non_generic (def_id) { llvm :: set_visibility (g , llvm :: Visibility :: Hidden) ; } g } else { check_and_apply_linkage (self , fn_attrs , llty , sym , def_id) } ; if fn_attrs . flags . contains (CodegenFnAttrFlags :: THREAD_LOCAL) { llvm :: set_thread_local_mode (g , self . tls_model) ; } let dso_local = self . assume_dso_local (g , true) ; if ! def_id . is_local () { let needs_dll_storage_attr = self . use_dll_storage_attrs && ! self . tcx . is_foreign_item (def_id) && ! dso_local && ! self . tcx . sess . opts . cg . linker_plugin_lto . enabled () ; assert ! (! (self . tcx . sess . opts . cg . linker_plugin_lto . enabled () && self . tcx . sess . target . is_like_windows && self . tcx . sess . opts . cg . prefer_dynamic)) ; if needs_dll_storage_attr { if ! self . tcx . is_codegened_item (def_id) { llvm :: set_dllimport_storage_class (g) ; } } } if self . use_dll_storage_attrs && let Some (library) = self . tcx . native_library (def_id) && library . kind . is_dllimport () { llvm :: set_dllimport_storage_class (g) ; } self . instances . borrow_mut () . insert (instance , g) ; g } fn codegen_static_item (& mut self , def_id : DefId) { assert ! (llvm :: LLVMGetInitializer (self . instances . borrow () . get (& Instance :: mono (self . tcx , def_id)) . unwrap ()) . is_none ()) ; let attrs = self . tcx . codegen_fn_attrs (def_id) ; let Ok ((v , alloc)) = codegen_static_initializer (self , def_id) else { return ; } ; let alloc = alloc . inner () ; let val_llty = self . val_ty (v) ; let g = self . get_static_inner (def_id , val_llty) ; let llty = self . get_type_of_global (g) ; let g = if val_llty == llty { g } else { let name = String :: from_utf8 (llvm :: get_value_name (g)) . expect ("we declare our statics with a utf8-valid name") ; llvm :: set_value_name (g , b"") ; let linkage = llvm :: get_linkage (g) ; let visibility = llvm :: get_visibility (g) ; let new_g = self . declare_global (& name , val_llty) ; llvm :: set_linkage (new_g , linkage) ; llvm :: set_visibility (new_g , visibility) ; self . renamed_statics . borrow_mut () . insert (def_id , new_g) ; self . statics_to_rauw . borrow_mut () . push ((g , new_g)) ; new_g } ; set_global_alignment (self , g , alloc . align) ; llvm :: set_initializer (g , v) ; self . assume_dso_local (g , true) ; if alloc . mutability . is_not () { llvm :: set_global_constant (g , true) ; } debuginfo :: build_global_var_di_node (self , def_id , g) ; if attrs . flags . contains (CodegenFnAttrFlags :: THREAD_LOCAL) { llvm :: set_thread_local_mode (g , self . tls_model) ; } if self . tcx . sess . target . is_like_wasm && attrs . link_section . map (| link_section | ! link_section . as_str () . starts_with (".init_array")) . unwrap_or (true) { if let Some (section) = attrs . link_section { let section = self . create_metadata (section . as_str () . as_bytes ()) ; assert ! (alloc . provenance () . ptrs () . is_empty ()) ; let bytes = alloc . inspect_with_uninit_and_ptr_outside_interpreter (0 .. alloc . len ()) ; let alloc = self . create_metadata (bytes) ; let data = [section , alloc] ; let meta = unsafe { llvm :: LLVMMDNodeInContext2 (self . llcx , data . as_ptr () , data . len ()) } ; let val = self . get_metadata_value (meta) ; unsafe { llvm :: LLVMAddNamedMetadataOperand (self . llmod , c"wasm.custom_sections" . as_ptr () , val ,) } ; } } else { base :: set_link_section (g , attrs) ; } base :: set_variable_sanitizer_attrs (g , attrs) ; if attrs . flags . contains (CodegenFnAttrFlags :: USED_COMPILER) { assert ! (! attrs . flags . contains (CodegenFnAttrFlags :: USED_LINKER)) ; self . add_compiler_used_global (g) ; } if attrs . flags . contains (CodegenFnAttrFlags :: USED_LINKER) { assert ! (! attrs . flags . contains (CodegenFnAttrFlags :: USED_COMPILER)) ; self . add_used_global (g) ; } } # [doc = " Add a global value to a list to be stored in the `llvm.used` variable, an array of ptr."] pub (crate) fn add_used_global (& mut self , global : & 'll Value) { self . used_statics . push (global) ; } # [doc = " Add a global value to a list to be stored in the `llvm.compiler.used` variable,"] # [doc = " an array of ptr."] pub (crate) fn add_compiler_used_global (& mut self , global : & 'll Value) { self . compiler_used_statics . push (global) ; } }}}
mkitem!{mkimpl!{impl < 'll > StaticCodegenMethods for CodegenCx < 'll , '_ > { # [doc = " Get a pointer to a global variable."] # [doc = ""] # [doc = " The pointer will always be in the default address space. If global variables default to a"] # [doc = " different address space, an addrspacecast is inserted."] fn static_addr_of (& self , cv : & 'll Value , align : Align , kind : Option < & str >) -> & 'll Value { let gv = self . static_addr_of_impl (cv , align , kind) ; self . const_pointercast (gv , self . type_ptr ()) } fn codegen_static (& mut self , def_id : DefId) { self . codegen_static_item (def_id) } }}}