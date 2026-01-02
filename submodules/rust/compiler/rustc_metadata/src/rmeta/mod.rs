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
mkuse!{use std :: marker :: PhantomData ;}
mkuse!{use std :: num :: NonZero ;}
mkuse!{pub (crate) use decoder :: { CrateMetadata , CrateNumMap , MetadataBlob , TargetModifiers } ;}
mkuse!{use decoder :: { DecodeContext , Metadata } ;}
mkuse!{use def_path_hash_map :: DefPathHashMapRef ;}
mkuse!{use encoder :: EncodeContext ;}
mkuse!{pub use encoder :: { EncodedMetadata , encode_metadata , rendered_const } ;}
mkuse!{pub (crate) use parameterized :: ParameterizedOverTcx ;}
mkuse!{use rustc_abi :: { FieldIdx , ReprOptions , VariantIdx } ;}
mkuse!{use rustc_data_structures :: fx :: FxHashMap ;}
mkuse!{use rustc_data_structures :: svh :: Svh ;}
mkuse!{use rustc_hir :: attrs :: StrippedCfgItem ;}
mkuse!{use rustc_hir :: def :: { CtorKind , DefKind , DocLinkResMap , MacroKinds } ;}
mkuse!{use rustc_hir :: def_id :: { CrateNum , DefId , DefIdMap , DefIndex , DefPathHash , StableCrateId } ;}
mkuse!{use rustc_hir :: definitions :: DefKey ;}
mkuse!{use rustc_hir :: lang_items :: LangItem ;}
mkuse!{use rustc_hir :: { PreciseCapturingArgKind , attrs } ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_index :: bit_set :: DenseBitSet ;}
mkuse!{use rustc_macros :: { Decodable , Encodable , MetadataDecodable , MetadataEncodable , TyDecodable , TyEncodable , } ;}
mkuse!{use rustc_middle :: metadata :: ModChild ;}
mkuse!{use rustc_middle :: middle :: codegen_fn_attrs :: CodegenFnAttrs ;}
mkuse!{use rustc_middle :: middle :: debugger_visualizer :: DebuggerVisualizerFile ;}
mkuse!{use rustc_middle :: middle :: exported_symbols :: { ExportedSymbol , SymbolExportInfo } ;}
mkuse!{use rustc_middle :: middle :: lib_features :: FeatureStability ;}
mkuse!{use rustc_middle :: middle :: resolve_bound_vars :: ObjectLifetimeDefault ;}
mkuse!{use rustc_middle :: mir ;}
mkuse!{use rustc_middle :: ty :: fast_reject :: SimplifiedType ;}
mkuse!{use rustc_middle :: ty :: { self , DeducedParamAttrs , Ty , TyCtxt , UnusedGenericParams } ;}
mkuse!{use rustc_middle :: util :: Providers ;}
mkuse!{use rustc_serialize :: opaque :: FileEncoder ;}
mkuse!{use rustc_session :: config :: { SymbolManglingVersion , TargetModifier } ;}
mkuse!{use rustc_session :: cstore :: { CrateDepKind , ForeignModule , LinkagePreference , NativeLib } ;}
mkuse!{use rustc_span :: edition :: Edition ;}
mkuse!{use rustc_span :: hygiene :: { ExpnIndex , MacroKind , SyntaxContextKey } ;}
mkuse!{use rustc_span :: { self , ExpnData , ExpnHash , ExpnId , Ident , Span , Symbol } ;}
mkuse!{use rustc_target :: spec :: { PanicStrategy , TargetTuple } ;}
mkuse!{use table :: TableBuilder ;}
mkuse!{use { rustc_ast as ast , rustc_hir as hir } ;}
mkuse!{use crate :: creader :: CrateMetadataRef ;}
mkmod!{decoder, { 
                getname!(decoder);
                getsrc!(decoder);
                getpath!(decoder);
                get_deps!(decoder);
                get_crates!(decoder);
                mkinclude!(decoder);
                 
            }}
mkmod!{def_path_hash_map, { 
                getname!(def_path_hash_map);
                getsrc!(def_path_hash_map);
                getpath!(def_path_hash_map);
                get_deps!(def_path_hash_map);
                get_crates!(def_path_hash_map);
                mkinclude!(def_path_hash_map);
                 
            }}
mkmod!{encoder, { 
                getname!(encoder);
                getsrc!(encoder);
                getpath!(encoder);
                get_deps!(encoder);
                get_crates!(encoder);
                mkinclude!(encoder);
                 
            }}
mkmod!{parameterized, { 
                getname!(parameterized);
                getsrc!(parameterized);
                getpath!(parameterized);
                get_deps!(parameterized);
                get_crates!(parameterized);
                mkinclude!(parameterized);
                 
            }}
mkmod!{table, { 
                getname!(table);
                getsrc!(table);
                getpath!(table);
                get_deps!(table);
                get_crates!(table);
                mkinclude!(table);
                 
            }}

macro_rules! rustc_version_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rustc_version in module {}", module_path!());
    };
}

mkfn!{
    rustc_version_introspect!();
    pub (crate) fn rustc_version (cfg_version : & 'static str) -> String { format ! ("rustc {cfg_version}") }
}
mkitem!{#[doc = " Metadata encoding version."] #[doc = " N.B., increment this if you change the format of metadata such that"] #[doc = " the rustc version can't be found to compare with `rustc_version()`."] const METADATA_VERSION : u8 = 10 ;}
mkitem!{#[doc = " Metadata header which includes `METADATA_VERSION`."] #[doc = ""] #[doc = " This header is followed by the length of the compressed data, then"] #[doc = " the position of the `CrateRoot`, which is encoded as a 64-bit little-endian"] #[doc = " unsigned integer, and further followed by the rustc version string."] pub const METADATA_HEADER : & [u8] = & [b'r' , b'u' , b's' , b't' , 0 , 0 , 0 , METADATA_VERSION] ;}
mkitem!{mkstruct!{#[doc = " A value of type T referred to by its absolute position"] #[doc = " in the metadata, and which can be decoded lazily."] #[doc = ""] #[doc = " Metadata is effective a tree, encoded in post-order,"] #[doc = " and with the root's position written next to the header."] #[doc = " That means every single `LazyValue` points to some previous"] #[doc = " location in the metadata and is part of a larger node."] #[doc = ""] #[doc = " The first `LazyValue` in a node is encoded as the backwards"] #[doc = " distance from the position where the containing node"] #[doc = " starts and where the `LazyValue` points to, while the rest"] #[doc = " use the forward distance from the previous `LazyValue`."] #[doc = " Distances start at 1, as 0-byte nodes are invalid."] #[doc = " Also invalid are nodes being referred in a different"] #[doc = " order than they were encoded in."] #[must_use] struct LazyValue < T > { position : NonZero < usize > , _marker : PhantomData < fn () -> T > , }}}
mkitem!{mkimpl!{impl < T > LazyValue < T > { fn from_position (position : NonZero < usize >) -> LazyValue < T > { LazyValue { position , _marker : PhantomData } } }}}
mkitem!{mkstruct!{#[doc = " A list of lazily-decoded values."] #[doc = ""] #[doc = " Unlike `LazyValue<Vec<T>>`, the length is encoded next to the"] #[doc = " position, not at the position, which means that the length"] #[doc = " doesn't need to be known before encoding all the elements."] #[doc = ""] #[doc = " If the length is 0, no position is encoded, but otherwise,"] #[doc = " the encoding is that of `LazyArray`, with the distinction that"] #[doc = " the minimal distance the length of the sequence, i.e."] #[doc = " it's assumed there's no 0-byte element in the sequence."] struct LazyArray < T > { position : NonZero < usize > , num_elems : usize , _marker : PhantomData < fn () -> T > , }}}
mkitem!{mkimpl!{impl < T > Default for LazyArray < T > { fn default () -> LazyArray < T > { LazyArray :: from_position_and_num_elems (NonZero :: new (1) . unwrap () , 0) } }}}
mkitem!{mkimpl!{impl < T > LazyArray < T > { fn from_position_and_num_elems (position : NonZero < usize > , num_elems : usize) -> LazyArray < T > { LazyArray { position , num_elems , _marker : PhantomData } } }}}
mkitem!{mkstruct!{#[doc = " A list of lazily-decoded values, with the added capability of random access."] #[doc = ""] #[doc = " Random-access table (i.e. offering constant-time `get`/`set`), similar to"] #[doc = " `LazyArray<T>`, but without requiring encoding or decoding all the values"] #[doc = " eagerly and in-order."] struct LazyTable < I , T > { position : NonZero < usize > , #[doc = " The encoded size of the elements of a table is selected at runtime to drop"] #[doc = " trailing zeroes. This is the number of bytes used for each table element."] width : usize , #[doc = " How many elements are in the table."] len : usize , _marker : PhantomData < fn (I) -> T > , }}}
mkitem!{mkimpl!{impl < I , T > LazyTable < I , T > { fn from_position_and_encoded_size (position : NonZero < usize > , width : usize , len : usize ,) -> LazyTable < I , T > { LazyTable { position , width , len , _marker : PhantomData } } }}}
mkitem!{mkimpl!{impl < T > Copy for LazyValue < T > { }}}
mkitem!{mkimpl!{impl < T > Clone for LazyValue < T > { fn clone (& self) -> Self { * self } }}}
mkitem!{mkimpl!{impl < T > Copy for LazyArray < T > { }}}
mkitem!{mkimpl!{impl < T > Clone for LazyArray < T > { fn clone (& self) -> Self { * self } }}}
mkitem!{mkimpl!{impl < I , T > Copy for LazyTable < I , T > { }}}
mkitem!{mkimpl!{impl < I , T > Clone for LazyTable < I , T > { fn clone (& self) -> Self { * self } }}}
mkitem!{mkenum!{#[doc = " Encoding / decoding state for `Lazy`s (`LazyValue`, `LazyArray`, and `LazyTable`)."] #[derive (Copy , Clone , PartialEq , Eq , Debug)] enum LazyState { #[doc = " Outside of a metadata node."] NoNode , #[doc = " Inside a metadata node, and before any `Lazy`s."] #[doc = " The position is that of the node itself."] NodeStart (NonZero < usize >) , #[doc = " Inside a metadata node, with a previous `Lazy`s."] #[doc = " The position is where that previous `Lazy` would start."] Previous (NonZero < usize >) , }}}
mkitem!{type SyntaxContextTable = LazyTable < u32 , Option < LazyValue < SyntaxContextKey > > > ;}
mkitem!{type ExpnDataTable = LazyTable < ExpnIndex , Option < LazyValue < ExpnData > > > ;}
mkitem!{type ExpnHashTable = LazyTable < ExpnIndex , Option < LazyValue < ExpnHash > > > ;}
mkitem!{mkstruct!{#[derive (MetadataEncodable , MetadataDecodable)] pub (crate) struct ProcMacroData { proc_macro_decls_static : DefIndex , stability : Option < hir :: Stability > , macros : LazyArray < DefIndex > , }}}
mkitem!{mkstruct!{#[doc = " Serialized crate metadata."] #[doc = ""] #[doc = " This contains just enough information to determine if we should load the `CrateRoot` or not."] #[doc = " Prefer [`CrateRoot`] whenever possible to avoid ICEs when using `omit-git-hash` locally."] #[doc = " See #76720 for more details."] #[doc = ""] #[doc = " If you do modify this struct, also bump the [`METADATA_VERSION`] constant."] #[derive (MetadataEncodable , MetadataDecodable)] pub (crate) struct CrateHeader { pub (crate) triple : TargetTuple , pub (crate) hash : Svh , pub (crate) name : Symbol , #[doc = " Whether this is the header for a proc-macro crate."] #[doc = ""] #[doc = " This is separate from [`ProcMacroData`] to avoid having to update [`METADATA_VERSION`] every"] #[doc = " time ProcMacroData changes."] pub (crate) is_proc_macro_crate : bool , #[doc = " Whether this crate metadata section is just a stub."] #[doc = " Stubs do not contain the full metadata (it will be typically stored"] #[doc = " in a separate rmeta file)."] #[doc = ""] #[doc = " This is used inside rlibs and dylibs when using `-Zembed-metadata=no`."] pub (crate) is_stub : bool , }}}
mkitem!{mkstruct!{#[doc = " Serialized `.rmeta` data for a crate."] #[doc = ""] #[doc = " When compiling a proc-macro crate, we encode many of"] #[doc = " the `LazyArray<T>` fields as `Lazy::empty()`. This serves two purposes:"] #[doc = ""] #[doc = " 1. We avoid performing unnecessary work. Proc-macro crates can only"] #[doc = " export proc-macros functions, which are compiled into a shared library."] #[doc = " As a result, a large amount of the information we normally store"] #[doc = " (e.g. optimized MIR) is unneeded by downstream crates."] #[doc = " 2. We avoid serializing invalid `CrateNum`s. When we deserialize"] #[doc = " a proc-macro crate, we don't load any of its dependencies (since we"] #[doc = " just need to invoke a native function from the shared library)."] #[doc = " This means that any foreign `CrateNum`s that we serialize cannot be"] #[doc = " deserialized, since we will not know how to map them into the current"] #[doc = " compilation session. If we were to serialize a proc-macro crate like"] #[doc = " a normal crate, much of what we serialized would be unusable in addition"] #[doc = " to being unused."] #[derive (MetadataEncodable , MetadataDecodable)] pub (crate) struct CrateRoot { #[doc = " A header used to detect if this is the right crate to load."] header : CrateHeader , extra_filename : String , stable_crate_id : StableCrateId , required_panic_strategy : Option < PanicStrategy > , panic_in_drop_strategy : PanicStrategy , edition : Edition , has_global_allocator : bool , has_alloc_error_handler : bool , has_panic_handler : bool , has_default_lib_allocator : bool , crate_deps : LazyArray < CrateDep > , dylib_dependency_formats : LazyArray < Option < LinkagePreference > > , lib_features : LazyArray < (Symbol , FeatureStability) > , stability_implications : LazyArray < (Symbol , Symbol) > , lang_items : LazyArray < (DefIndex , LangItem) > , lang_items_missing : LazyArray < LangItem > , stripped_cfg_items : LazyArray < StrippedCfgItem < DefIndex > > , diagnostic_items : LazyArray < (Symbol , DefIndex) > , native_libraries : LazyArray < NativeLib > , foreign_modules : LazyArray < ForeignModule > , traits : LazyArray < DefIndex > , impls : LazyArray < TraitImpls > , incoherent_impls : LazyArray < IncoherentImpls > , interpret_alloc_index : LazyArray < u64 > , proc_macro_data : Option < ProcMacroData > , tables : LazyTables , debugger_visualizers : LazyArray < DebuggerVisualizerFile > , exportable_items : LazyArray < DefIndex > , stable_order_of_exportable_impls : LazyArray < (DefIndex , usize) > , exported_non_generic_symbols : LazyArray < (ExportedSymbol < 'static > , SymbolExportInfo) > , exported_generic_symbols : LazyArray < (ExportedSymbol < 'static > , SymbolExportInfo) > , syntax_contexts : SyntaxContextTable , expn_data : ExpnDataTable , expn_hashes : ExpnHashTable , def_path_hash_map : LazyValue < DefPathHashMapRef < 'static > > , source_map : LazyTable < u32 , Option < LazyValue < rustc_span :: SourceFile > > > , target_modifiers : LazyArray < TargetModifier > , compiler_builtins : bool , needs_allocator : bool , needs_panic_runtime : bool , no_builtins : bool , panic_runtime : bool , profiler_runtime : bool , symbol_mangling_version : SymbolManglingVersion , specialization_enabled_in : bool , }}}
mkitem!{mkstruct!{#[doc = " On-disk representation of `DefId`."] #[doc = " This creates a type-safe way to enforce that we remap the CrateNum between the on-disk"] #[doc = " representation and the compilation session."] #[derive (Copy , Clone)] pub (crate) struct RawDefId { krate : u32 , index : u32 , }}}
mkitem!{mkimpl!{impl From < DefId > for RawDefId { fn from (val : DefId) -> Self { RawDefId { krate : val . krate . as_u32 () , index : val . index . as_u32 () } } }}}
mkitem!{mkimpl!{impl RawDefId { #[doc = " This exists so that `provide_one!` is happy"] fn decode (self , meta : (CrateMetadataRef < '_ > , TyCtxt < '_ >)) -> DefId { self . decode_from_cdata (meta . 0) } fn decode_from_cdata (self , cdata : CrateMetadataRef < '_ >) -> DefId { let krate = CrateNum :: from_u32 (self . krate) ; let krate = cdata . map_encoded_cnum_to_current (krate) ; DefId { krate , index : DefIndex :: from_u32 (self . index) } } }}}
mkitem!{mkstruct!{#[derive (Encodable , Decodable)] pub (crate) struct CrateDep { pub name : Symbol , pub hash : Svh , pub host_hash : Option < Svh > , pub kind : CrateDepKind , pub extra_filename : String , pub is_private : bool , }}}
mkitem!{mkstruct!{#[derive (MetadataEncodable , MetadataDecodable)] pub (crate) struct TraitImpls { trait_id : (u32 , DefIndex) , impls : LazyArray < (DefIndex , Option < SimplifiedType >) > , }}}
mkitem!{mkstruct!{#[derive (MetadataEncodable , MetadataDecodable)] pub (crate) struct IncoherentImpls { self_ty : SimplifiedType , impls : LazyArray < DefIndex > , }}}
mkitem!{#[doc = " Define `LazyTables` and `TableBuilders` at the same time."] macro_rules ! define_tables { (- defaulted : $ ($ name1 : ident : Table <$ IDX1 : ty , $ T1 : ty >,) + - optional : $ ($ name2 : ident : Table <$ IDX2 : ty , $ T2 : ty >,) +) => { #[derive (MetadataEncodable , MetadataDecodable)] pub (crate) struct LazyTables { $ ($ name1 : LazyTable <$ IDX1 , $ T1 >,) + $ ($ name2 : LazyTable <$ IDX2 , Option <$ T2 >>,) + } #[derive (Default)] struct TableBuilders { $ ($ name1 : TableBuilder <$ IDX1 , $ T1 >,) + $ ($ name2 : TableBuilder <$ IDX2 , Option <$ T2 >>,) + } impl TableBuilders { fn encode (& self , buf : & mut FileEncoder) -> LazyTables { LazyTables { $ ($ name1 : self .$ name1 . encode (buf) ,) + $ ($ name2 : self .$ name2 . encode (buf) ,) + } } } } }}
mkitem!{define_tables ! { - defaulted : intrinsic : Table < DefIndex , Option < LazyValue < ty :: IntrinsicDef >>>, is_macro_rules : Table < DefIndex , bool >, type_alias_is_lazy : Table < DefIndex , bool >, attr_flags : Table < DefIndex , AttrFlags >, def_path_hashes : Table < DefIndex , u64 >, explicit_item_bounds : Table < DefIndex , LazyArray < (ty :: Clause <'static >, Span) >>, explicit_item_self_bounds : Table < DefIndex , LazyArray < (ty :: Clause <'static >, Span) >>, inferred_outlives_of : Table < DefIndex , LazyArray < (ty :: Clause <'static >, Span) >>, explicit_super_predicates_of : Table < DefIndex , LazyArray < (ty :: Clause <'static >, Span) >>, explicit_implied_predicates_of : Table < DefIndex , LazyArray < (ty :: Clause <'static >, Span) >>, explicit_implied_const_bounds : Table < DefIndex , LazyArray < (ty :: PolyTraitRef <'static >, Span) >>, inherent_impls : Table < DefIndex , LazyArray < DefIndex >>, opt_rpitit_info : Table < DefIndex , Option < LazyValue < ty :: ImplTraitInTraitData >>>, module_children_reexports : Table < DefIndex , LazyArray < ModChild >>, cross_crate_inlinable : Table < DefIndex , bool >, - optional : attributes : Table < DefIndex , LazyArray < hir :: Attribute >>, module_children_non_reexports : Table < DefIndex , LazyArray < DefIndex >>, associated_item_or_field_def_ids : Table < DefIndex , LazyArray < DefIndex >>, def_kind : Table < DefIndex , DefKind >, visibility : Table < DefIndex , LazyValue < ty :: Visibility < DefIndex >>>, safety : Table < DefIndex , hir :: Safety >, def_span : Table < DefIndex , LazyValue < Span >>, def_ident_span : Table < DefIndex , LazyValue < Span >>, lookup_stability : Table < DefIndex , LazyValue < hir :: Stability >>, lookup_const_stability : Table < DefIndex , LazyValue < hir :: ConstStability >>, lookup_default_body_stability : Table < DefIndex , LazyValue < hir :: DefaultBodyStability >>, lookup_deprecation_entry : Table < DefIndex , LazyValue < attrs :: Deprecation >>, explicit_predicates_of : Table < DefIndex , LazyValue < ty :: GenericPredicates <'static >>>, generics_of : Table < DefIndex , LazyValue < ty :: Generics >>, type_of : Table < DefIndex , LazyValue < ty :: EarlyBinder <'static , Ty <'static >>>>, variances_of : Table < DefIndex , LazyArray < ty :: Variance >>, fn_sig : Table < DefIndex , LazyValue < ty :: EarlyBinder <'static , ty :: PolyFnSig <'static >>>>, codegen_fn_attrs : Table < DefIndex , LazyValue < CodegenFnAttrs >>, impl_trait_header : Table < DefIndex , LazyValue < ty :: ImplTraitHeader <'static >>>, const_param_default : Table < DefIndex , LazyValue < ty :: EarlyBinder <'static , rustc_middle :: ty :: Const <'static >>>>, object_lifetime_default : Table < DefIndex , LazyValue < ObjectLifetimeDefault >>, optimized_mir : Table < DefIndex , LazyValue < mir :: Body <'static >>>, mir_for_ctfe : Table < DefIndex , LazyValue < mir :: Body <'static >>>, closure_saved_names_of_captured_variables : Table < DefIndex , LazyValue < IndexVec < FieldIdx , Symbol >>>, mir_coroutine_witnesses : Table < DefIndex , LazyValue < mir :: CoroutineLayout <'static >>>, promoted_mir : Table < DefIndex , LazyValue < IndexVec < mir :: Promoted , mir :: Body <'static >>>>, thir_abstract_const : Table < DefIndex , LazyValue < ty :: EarlyBinder <'static , ty :: Const <'static >>>>, impl_parent : Table < DefIndex , RawDefId >, constness : Table < DefIndex , hir :: Constness >, const_conditions : Table < DefIndex , LazyValue < ty :: ConstConditions <'static >>>, defaultness : Table < DefIndex , hir :: Defaultness >, coerce_unsized_info : Table < DefIndex , LazyValue < ty :: adjustment :: CoerceUnsizedInfo >>, mir_const_qualif : Table < DefIndex , LazyValue < mir :: ConstQualifs >>, rendered_const : Table < DefIndex , LazyValue < String >>, rendered_precise_capturing_args : Table < DefIndex , LazyArray < PreciseCapturingArgKind < Symbol , Symbol >>>, asyncness : Table < DefIndex , ty :: Asyncness >, fn_arg_idents : Table < DefIndex , LazyArray < Option < Ident >>>, coroutine_kind : Table < DefIndex , hir :: CoroutineKind >, coroutine_for_closure : Table < DefIndex , RawDefId >, adt_destructor : Table < DefIndex , LazyValue < ty :: Destructor >>, adt_async_destructor : Table < DefIndex , LazyValue < ty :: AsyncDestructor >>, coroutine_by_move_body_def_id : Table < DefIndex , RawDefId >, eval_static_initializer : Table < DefIndex , LazyValue < mir :: interpret :: ConstAllocation <'static >>>, trait_def : Table < DefIndex , LazyValue < ty :: TraitDef >>, expn_that_defined : Table < DefIndex , LazyValue < ExpnId >>, default_fields : Table < DefIndex , LazyValue < DefId >>, params_in_repr : Table < DefIndex , LazyValue < DenseBitSet < u32 >>>, repr_options : Table < DefIndex , LazyValue < ReprOptions >>, def_keys : Table < DefIndex , LazyValue < DefKey >>, proc_macro_quoted_spans : Table < usize , LazyValue < Span >>, variant_data : Table < DefIndex , LazyValue < VariantData >>, assoc_container : Table < DefIndex , LazyValue < ty :: AssocContainer >>, macro_definition : Table < DefIndex , LazyValue < ast :: DelimArgs >>, proc_macro : Table < DefIndex , MacroKind >, deduced_param_attrs : Table < DefIndex , LazyArray < DeducedParamAttrs >>, trait_impl_trait_tys : Table < DefIndex , LazyValue < DefIdMap < ty :: EarlyBinder <'static , Ty <'static >>>>>, doc_link_resolutions : Table < DefIndex , LazyValue < DocLinkResMap >>, doc_link_traits_in_scope : Table < DefIndex , LazyArray < DefId >>, assumed_wf_types_for_rpitit : Table < DefIndex , LazyArray < (Ty <'static >, Span) >>, opaque_ty_origin : Table < DefIndex , LazyValue < hir :: OpaqueTyOrigin < DefId >>>, anon_const_kind : Table < DefIndex , LazyValue < ty :: AnonConstKind >>, associated_types_for_impl_traits_in_trait_or_impl : Table < DefIndex , LazyValue < DefIdMap < Vec < DefId >>>>, }}
mkitem!{mkstruct!{#[derive (TyEncodable , TyDecodable)] struct VariantData { idx : VariantIdx , discr : ty :: VariantDiscr , #[doc = " If this is unit or tuple-variant/struct, then this is the index of the ctor id."] ctor : Option < (CtorKind , DefIndex) > , is_non_exhaustive : bool , }}}
mkitem!{bitflags :: bitflags ! { #[derive (Default)] pub struct AttrFlags : u8 { const IS_DOC_HIDDEN = 1 << 0 ; } }}
mkitem!{mkstruct!{#[doc = " A span tag byte encodes a bunch of data, so that we can cut out a few extra bytes from span"] #[doc = " encodings (which are very common, for example, libcore has ~650,000 unique spans and over 1.1"] #[doc = " million references to prior-written spans)."] #[doc = ""] #[doc = " The byte format is split into several parts:"] #[doc = ""] #[doc = " [ a a a a a c d d ]"] #[doc = ""] #[doc = " `a` bits represent the span length. We have 5 bits, so we can store lengths up to 30 inline, with"] #[doc = " an all-1s pattern representing that the length is stored separately."] #[doc = ""] #[doc = " `c` represents whether the span context is zero (and then it is not stored as a separate varint)"] #[doc = " for direct span encodings, and whether the offset is absolute or relative otherwise (zero for"] #[doc = " absolute)."] #[doc = ""] #[doc = " d bits represent the kind of span we are storing (local, foreign, partial, indirect)."] #[derive (Encodable , Decodable , Copy , Clone)] struct SpanTag (u8) ;}}
mkitem!{mkenum!{#[derive (Debug , Copy , Clone , PartialEq , Eq)] enum SpanKind { Local = 0b00 , Foreign = 0b01 , Partial = 0b10 , Indirect = 0b11 , }}}
mkitem!{mkimpl!{impl SpanTag { fn new (kind : SpanKind , context : rustc_span :: SyntaxContext , length : usize) -> SpanTag { let mut data = 0u8 ; data |= kind as u8 ; if context . is_root () { data |= 0b100 ; } let all_1s_len = (0xffu8 << 3) >> 3 ; if length < all_1s_len as usize { data |= (length as u8) << 3 ; } else { data |= all_1s_len << 3 ; } SpanTag (data) } fn indirect (relative : bool , length_bytes : u8) -> SpanTag { let mut tag = SpanTag (SpanKind :: Indirect as u8) ; if relative { tag . 0 |= 0b100 ; } assert ! (length_bytes <= 8) ; tag . 0 |= length_bytes << 3 ; tag } fn kind (self) -> SpanKind { let masked = self . 0 & 0b11 ; match masked { 0b00 => SpanKind :: Local , 0b01 => SpanKind :: Foreign , 0b10 => SpanKind :: Partial , 0b11 => SpanKind :: Indirect , _ => unreachable ! () , } } fn is_relative_offset (self) -> bool { debug_assert_eq ! (self . kind () , SpanKind :: Indirect) ; self . 0 & 0b100 != 0 } fn context (self) -> Option < rustc_span :: SyntaxContext > { if self . 0 & 0b100 != 0 { Some (rustc_span :: SyntaxContext :: root ()) } else { None } } fn length (self) -> Option < rustc_span :: BytePos > { let all_1s_len = (0xffu8 << 3) >> 3 ; let len = self . 0 >> 3 ; if len != all_1s_len { Some (rustc_span :: BytePos (u32 :: from (len))) } else { None } } }}}
mkitem!{const SYMBOL_STR : u8 = 0 ;}
mkitem!{const SYMBOL_OFFSET : u8 = 1 ;}
mkitem!{const SYMBOL_PREDEFINED : u8 = 2 ;}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub fn provide (providers : & mut Providers) { encoder :: provide (providers) ; decoder :: provide (providers) ; }
}