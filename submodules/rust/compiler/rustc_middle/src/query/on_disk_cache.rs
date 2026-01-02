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
mkuse!{use std :: collections :: hash_map :: Entry ;}
mkuse!{use std :: mem ;}
mkuse!{use std :: sync :: Arc ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashMap , FxIndexMap , FxIndexSet } ;}
mkuse!{use rustc_data_structures :: memmap :: Mmap ;}
mkuse!{use rustc_data_structures :: sync :: { HashMapExt , Lock , RwLock } ;}
mkuse!{use rustc_data_structures :: unhash :: UnhashMap ;}
mkuse!{use rustc_data_structures :: unord :: { UnordMap , UnordSet } ;}
mkuse!{use rustc_hir :: def_id :: { CrateNum , DefId , DefIndex , LOCAL_CRATE , LocalDefId , StableCrateId } ;}
mkuse!{use rustc_hir :: definitions :: DefPathHash ;}
mkuse!{use rustc_index :: { Idx , IndexVec } ;}
mkuse!{use rustc_macros :: { Decodable , Encodable } ;}
mkuse!{use rustc_query_system :: query :: QuerySideEffect ;}
mkuse!{use rustc_serialize :: opaque :: { FileEncodeResult , FileEncoder , IntEncodedWithFixedSize , MemDecoder } ;}
mkuse!{use rustc_serialize :: { Decodable , Decoder , Encodable , Encoder } ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_span :: hygiene :: { ExpnId , HygieneDecodeContext , HygieneEncodeContext , SyntaxContext , SyntaxContextKey , } ;}
mkuse!{use rustc_span :: source_map :: Spanned ;}
mkuse!{use rustc_span :: { BytePos , ByteSymbol , CachingSourceMapView , ExpnData , ExpnHash , Pos , RelativeBytePos , SourceFile , Span , SpanDecoder , SpanEncoder , StableSourceFileId , Symbol , } ;}
mkuse!{use crate :: dep_graph :: { DepNodeIndex , SerializedDepNodeIndex } ;}
mkuse!{use crate :: mir :: interpret :: { AllocDecodingSession , AllocDecodingState } ;}
mkuse!{use crate :: mir :: mono :: MonoItem ;}
mkuse!{use crate :: mir :: { self , interpret } ;}
mkuse!{use crate :: ty :: codec :: { RefDecodable , TyDecoder , TyEncoder } ;}
mkuse!{use crate :: ty :: { self , Ty , TyCtxt } ;}
mkitem!{const TAG_FILE_FOOTER : u128 = 0xC0FFEE_C0FFEE_C0FFEE_C0FFEE_C0FFEE ;}
mkitem!{const TAG_FULL_SPAN : u8 = 0 ;}
mkitem!{const TAG_PARTIAL_SPAN : u8 = 1 ;}
mkitem!{const TAG_RELATIVE_SPAN : u8 = 2 ;}
mkitem!{const TAG_SYNTAX_CONTEXT : u8 = 0 ;}
mkitem!{const TAG_EXPN_DATA : u8 = 1 ;}
mkitem!{const SYMBOL_STR : u8 = 0 ;}
mkitem!{const SYMBOL_OFFSET : u8 = 1 ;}
mkitem!{const SYMBOL_PREDEFINED : u8 = 2 ;}
mkitem!{mkstruct!{#[doc = " Provides an interface to incremental compilation data cached from the"] #[doc = " previous compilation session. This data will eventually include the results"] #[doc = " of a few selected queries (like `typeck` and `mir_optimized`) and"] #[doc = " any side effects that have been emitted during a query."] pub struct OnDiskCache { serialized_data : RwLock < Option < Mmap > > , current_side_effects : Lock < FxIndexMap < DepNodeIndex , QuerySideEffect > > , file_index_to_stable_id : FxHashMap < SourceFileIndex , EncodedSourceFileId > , file_index_to_file : Lock < FxHashMap < SourceFileIndex , Arc < SourceFile > > > , query_result_index : FxHashMap < SerializedDepNodeIndex , AbsoluteBytePos > , prev_side_effects_index : FxHashMap < SerializedDepNodeIndex , AbsoluteBytePos > , alloc_decoding_state : AllocDecodingState , syntax_contexts : FxHashMap < u32 , AbsoluteBytePos > , expn_data : UnhashMap < ExpnHash , AbsoluteBytePos > , hygiene_context : HygieneDecodeContext , foreign_expn_data : UnhashMap < ExpnHash , u32 > , }}}
mkitem!{mkstruct!{#[derive (Encodable , Decodable)] struct Footer { file_index_to_stable_id : FxHashMap < SourceFileIndex , EncodedSourceFileId > , query_result_index : EncodedDepNodeIndex , side_effects_index : EncodedDepNodeIndex , interpret_alloc_index : Vec < u64 > , syntax_contexts : FxHashMap < u32 , AbsoluteBytePos > , expn_data : UnhashMap < ExpnHash , AbsoluteBytePos > , foreign_expn_data : UnhashMap < ExpnHash , u32 > , }}}
mkitem!{pub type EncodedDepNodeIndex = Vec < (SerializedDepNodeIndex , AbsoluteBytePos) > ;}
mkitem!{mkstruct!{#[derive (Copy , Clone , PartialEq , Eq , Hash , Debug , Encodable , Decodable)] struct SourceFileIndex (u32) ;}}
mkitem!{mkstruct!{#[derive (Copy , Clone , Debug , Hash , Eq , PartialEq , Encodable , Decodable)] pub struct AbsoluteBytePos (u64) ;}}
mkitem!{mkimpl!{impl AbsoluteBytePos { #[inline] pub fn new (pos : usize) -> AbsoluteBytePos { AbsoluteBytePos (pos . try_into () . expect ("Incremental cache file size overflowed u64.")) } #[inline] fn to_usize (self) -> usize { self . 0 as usize } }}}
mkitem!{mkstruct!{#[derive (Encodable , Decodable , Clone , Debug)] struct EncodedSourceFileId { stable_source_file_id : StableSourceFileId , stable_crate_id : StableCrateId , }}}
mkitem!{mkimpl!{impl EncodedSourceFileId { #[inline] fn new (tcx : TyCtxt < '_ > , file : & SourceFile) -> EncodedSourceFileId { EncodedSourceFileId { stable_source_file_id : file . stable_id , stable_crate_id : tcx . stable_crate_id (file . cnum) , } } }}}
mkitem!{mkimpl!{impl OnDiskCache { #[doc = " Creates a new `OnDiskCache` instance from the serialized data in `data`."] #[doc = ""] #[doc = " The serialized cache has some basic integrity checks, if those checks indicate that the"] #[doc = " on-disk data is corrupt, an error is returned."] pub fn new (sess : & Session , data : Mmap , start_pos : usize) -> Result < Self , () > { assert ! (sess . opts . incremental . is_some ()) ; let mut decoder = MemDecoder :: new (& data , start_pos) ? ; let footer_pos = decoder . with_position (decoder . len () - IntEncodedWithFixedSize :: ENCODED_SIZE , | decoder | { IntEncodedWithFixedSize :: decode (decoder) . 0 as usize }) ; let footer : Footer = decoder . with_position (footer_pos , | decoder | decode_tagged (decoder , TAG_FILE_FOOTER)) ; Ok (Self { serialized_data : RwLock :: new (Some (data)) , file_index_to_stable_id : footer . file_index_to_stable_id , file_index_to_file : Default :: default () , current_side_effects : Default :: default () , query_result_index : footer . query_result_index . into_iter () . collect () , prev_side_effects_index : footer . side_effects_index . into_iter () . collect () , alloc_decoding_state : AllocDecodingState :: new (footer . interpret_alloc_index) , syntax_contexts : footer . syntax_contexts , expn_data : footer . expn_data , foreign_expn_data : footer . foreign_expn_data , hygiene_context : Default :: default () , }) } pub fn new_empty () -> Self { Self { serialized_data : RwLock :: new (None) , file_index_to_stable_id : Default :: default () , file_index_to_file : Default :: default () , current_side_effects : Default :: default () , query_result_index : Default :: default () , prev_side_effects_index : Default :: default () , alloc_decoding_state : AllocDecodingState :: new (Vec :: new ()) , syntax_contexts : FxHashMap :: default () , expn_data : UnhashMap :: default () , foreign_expn_data : UnhashMap :: default () , hygiene_context : Default :: default () , } } #[doc = " Execute all cache promotions and release the serialized backing Mmap."] #[doc = ""] #[doc = " Cache promotions require invoking queries, which needs to read the serialized data."] #[doc = " In order to serialize the new on-disk cache, the former on-disk cache file needs to be"] #[doc = " deleted, hence we won't be able to refer to its memmapped data."] pub fn drop_serialized_data (& self , tcx : TyCtxt < '_ >) { tcx . dep_graph . exec_cache_promotions (tcx) ; * self . serialized_data . write () = None ; } pub fn serialize (& self , tcx : TyCtxt < '_ > , encoder : FileEncoder) -> FileEncodeResult { tcx . dep_graph . with_ignore (| | { let (file_to_file_index , file_index_to_stable_id) = { let files = tcx . sess . source_map () . files () ; let mut file_to_file_index = FxHashMap :: with_capacity_and_hasher (files . len () , Default :: default ()) ; let mut file_index_to_stable_id = FxHashMap :: with_capacity_and_hasher (files . len () , Default :: default ()) ; for (index , file) in files . iter () . enumerate () { let index = SourceFileIndex (index as u32) ; let file_ptr : * const SourceFile = & raw const * * file ; file_to_file_index . insert (file_ptr , index) ; let source_file_id = EncodedSourceFileId :: new (tcx , file) ; file_index_to_stable_id . insert (index , source_file_id) ; } (file_to_file_index , file_index_to_stable_id) } ; let hygiene_encode_context = HygieneEncodeContext :: default () ; let mut encoder = CacheEncoder { tcx , encoder , type_shorthands : Default :: default () , predicate_shorthands : Default :: default () , interpret_allocs : Default :: default () , source_map : CachingSourceMapView :: new (tcx . sess . source_map ()) , file_to_file_index , hygiene_context : & hygiene_encode_context , symbol_index_table : Default :: default () , } ; let mut query_result_index = EncodedDepNodeIndex :: new () ; tcx . sess . time ("encode_query_results" , | | { let enc = & mut encoder ; let qri = & mut query_result_index ; (tcx . query_system . fns . encode_query_results) (tcx , enc , qri) ; }) ; let side_effects_index : EncodedDepNodeIndex = self . current_side_effects . borrow () . iter () . map (| (dep_node_index , side_effect) | { let pos = AbsoluteBytePos :: new (encoder . position ()) ; let dep_node_index = SerializedDepNodeIndex :: new (dep_node_index . index ()) ; encoder . encode_tagged (dep_node_index , side_effect) ; (dep_node_index , pos) }) . collect () ; let interpret_alloc_index = { let mut interpret_alloc_index = Vec :: new () ; let mut n = 0 ; loop { let new_n = encoder . interpret_allocs . len () ; if n == new_n { break ; } interpret_alloc_index . reserve (new_n - n) ; for idx in n .. new_n { let id = encoder . interpret_allocs [idx] ; let pos : u64 = encoder . position () . try_into () . unwrap () ; interpret_alloc_index . push (pos) ; interpret :: specialized_encode_alloc_id (& mut encoder , tcx , id) ; } n = new_n ; } interpret_alloc_index } ; let mut syntax_contexts = FxHashMap :: default () ; let mut expn_data = UnhashMap :: default () ; let mut foreign_expn_data = UnhashMap :: default () ; hygiene_encode_context . encode (& mut encoder , | encoder , index , ctxt_data | { let pos = AbsoluteBytePos :: new (encoder . position ()) ; encoder . encode_tagged (TAG_SYNTAX_CONTEXT , ctxt_data) ; syntax_contexts . insert (index , pos) ; } , | encoder , expn_id , data , hash | { if expn_id . krate == LOCAL_CRATE { let pos = AbsoluteBytePos :: new (encoder . position ()) ; encoder . encode_tagged (TAG_EXPN_DATA , data) ; expn_data . insert (hash , pos) ; } else { foreign_expn_data . insert (hash , expn_id . local_id . as_u32 ()) ; } } ,) ; let footer_pos = encoder . position () as u64 ; encoder . encode_tagged (TAG_FILE_FOOTER , & Footer { file_index_to_stable_id , query_result_index , side_effects_index , interpret_alloc_index , syntax_contexts , expn_data , foreign_expn_data , } ,) ; IntEncodedWithFixedSize (footer_pos) . encode (& mut encoder . encoder) ; encoder . finish () }) } #[doc = " Loads a `QuerySideEffect` created during the previous compilation session."] pub fn load_side_effect (& self , tcx : TyCtxt < '_ > , dep_node_index : SerializedDepNodeIndex ,) -> Option < QuerySideEffect > { let side_effect : Option < QuerySideEffect > = self . load_indexed (tcx , dep_node_index , & self . prev_side_effects_index) ; side_effect } #[doc = " Stores a `QuerySideEffect` emitted during the current compilation session."] #[doc = " Anything stored like this will be available via `load_side_effect` in"] #[doc = " the next compilation session."] pub fn store_side_effect (& self , dep_node_index : DepNodeIndex , side_effect : QuerySideEffect) { let mut current_side_effects = self . current_side_effects . borrow_mut () ; let prev = current_side_effects . insert (dep_node_index , side_effect) ; debug_assert ! (prev . is_none ()) ; } #[doc = " Return whether the cached query result can be decoded."] #[inline] pub fn loadable_from_disk (& self , dep_node_index : SerializedDepNodeIndex) -> bool { self . query_result_index . contains_key (& dep_node_index) } #[doc = " Returns the cached query result if there is something in the cache for"] #[doc = " the given `SerializedDepNodeIndex`; otherwise returns `None`."] pub fn try_load_query_result < 'tcx , T > (& self , tcx : TyCtxt < 'tcx > , dep_node_index : SerializedDepNodeIndex ,) -> Option < T > where T : for < 'a > Decodable < CacheDecoder < 'a , 'tcx > > , { let opt_value = self . load_indexed (tcx , dep_node_index , & self . query_result_index) ; debug_assert_eq ! (opt_value . is_some () , self . loadable_from_disk (dep_node_index)) ; opt_value } fn load_indexed < 'tcx , T > (& self , tcx : TyCtxt < 'tcx > , dep_node_index : SerializedDepNodeIndex , index : & FxHashMap < SerializedDepNodeIndex , AbsoluteBytePos > ,) -> Option < T > where T : for < 'a > Decodable < CacheDecoder < 'a , 'tcx > > , { let pos = index . get (& dep_node_index) . cloned () ? ; let value = self . with_decoder (tcx , pos , | decoder | decode_tagged (decoder , dep_node_index)) ; Some (value) } fn with_decoder < 'a , 'tcx , T , F : for < 's > FnOnce (& mut CacheDecoder < 's , 'tcx >) -> T > (& self , tcx : TyCtxt < 'tcx > , pos : AbsoluteBytePos , f : F ,) -> T where T : Decodable < CacheDecoder < 'a , 'tcx > > , { let serialized_data = self . serialized_data . read () ; let mut decoder = CacheDecoder { tcx , opaque : MemDecoder :: new (serialized_data . as_deref () . unwrap_or (& []) , pos . to_usize ()) . unwrap () , file_index_to_file : & self . file_index_to_file , file_index_to_stable_id : & self . file_index_to_stable_id , alloc_decoding_session : self . alloc_decoding_state . new_decoding_session () , syntax_contexts : & self . syntax_contexts , expn_data : & self . expn_data , foreign_expn_data : & self . foreign_expn_data , hygiene_context : & self . hygiene_context , } ; f (& mut decoder) } }}}
mkitem!{mkstruct!{#[doc = " A decoder that can read from the incremental compilation cache. It is similar to the one"] #[doc = " we use for crate metadata decoding in that it can rebase spans and eventually"] #[doc = " will also handle things that contain `Ty` instances."] pub struct CacheDecoder < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , opaque : MemDecoder < 'a > , file_index_to_file : & 'a Lock < FxHashMap < SourceFileIndex , Arc < SourceFile > > > , file_index_to_stable_id : & 'a FxHashMap < SourceFileIndex , EncodedSourceFileId > , alloc_decoding_session : AllocDecodingSession < 'a > , syntax_contexts : & 'a FxHashMap < u32 , AbsoluteBytePos > , expn_data : & 'a UnhashMap < ExpnHash , AbsoluteBytePos > , foreign_expn_data : & 'a UnhashMap < ExpnHash , u32 > , hygiene_context : & 'a HygieneDecodeContext , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > CacheDecoder < 'a , 'tcx > { #[inline] fn file_index_to_file (& self , index : SourceFileIndex) -> Arc < SourceFile > { let CacheDecoder { tcx , file_index_to_file , file_index_to_stable_id , .. } = * self ; Arc :: clone (file_index_to_file . borrow_mut () . entry (index) . or_insert_with (| | { let source_file_id = & file_index_to_stable_id [& index] ; let source_file_cnum = tcx . stable_crate_id_to_crate_num (source_file_id . stable_crate_id) ; if source_file_cnum != LOCAL_CRATE { self . tcx . import_source_files (source_file_cnum) ; } tcx . sess . source_map () . source_file_by_stable_id (source_file_id . stable_source_file_id) . expect ("failed to lookup `SourceFile` in new context") })) } #[inline] fn decode_symbol_or_byte_symbol < S > (& mut self , new_from_index : impl Fn (u32) -> S , read_and_intern_str_or_byte_str_this : impl Fn (& mut Self) -> S , read_and_intern_str_or_byte_str_opaque : impl Fn (& mut MemDecoder < 'a >) -> S ,) -> S { let tag = self . read_u8 () ; match tag { SYMBOL_STR => read_and_intern_str_or_byte_str_this (self) , SYMBOL_OFFSET => { let pos = self . read_usize () ; self . opaque . with_position (pos , | d | read_and_intern_str_or_byte_str_opaque (d)) } SYMBOL_PREDEFINED => new_from_index (self . read_u32 ()) , _ => unreachable ! () , } } }}}

macro_rules! decode_tagged_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function decode_tagged in module {}", module_path!());
    };
}

mkfn!{
    decode_tagged_introspect!();
    fn decode_tagged < D , T , V > (decoder : & mut D , expected_tag : T) -> V where T : Decodable < D > + Eq + std :: fmt :: Debug , V : Decodable < D > , D : Decoder , { let start_pos = decoder . position () ; let actual_tag = T :: decode (decoder) ; assert_eq ! (actual_tag , expected_tag) ; let value = V :: decode (decoder) ; let end_pos = decoder . position () ; let expected_len : u64 = Decodable :: decode (decoder) ; assert_eq ! ((end_pos - start_pos) as u64 , expected_len) ; value }
}
mkitem!{mkimpl!{impl < 'a , 'tcx > TyDecoder < 'tcx > for CacheDecoder < 'a , 'tcx > { const CLEAR_CROSS_CRATE : bool = false ; #[inline] fn interner (& self) -> TyCtxt < 'tcx > { self . tcx } fn cached_ty_for_shorthand < F > (& mut self , shorthand : usize , or_insert_with : F) -> Ty < 'tcx > where F : FnOnce (& mut Self) -> Ty < 'tcx > , { let tcx = self . tcx ; let cache_key = ty :: CReaderCacheKey { cnum : None , pos : shorthand } ; if let Some (& ty) = tcx . ty_rcache . borrow () . get (& cache_key) { return ty ; } let ty = or_insert_with (self) ; tcx . ty_rcache . borrow_mut () . insert_same (cache_key , ty) ; ty } fn with_position < F , R > (& mut self , pos : usize , f : F) -> R where F : FnOnce (& mut Self) -> R , { debug_assert ! (pos < self . opaque . len ()) ; let new_opaque = self . opaque . split_at (pos) ; let old_opaque = mem :: replace (& mut self . opaque , new_opaque) ; let r = f (self) ; self . opaque = old_opaque ; r } fn decode_alloc_id (& mut self) -> interpret :: AllocId { let alloc_decoding_session = self . alloc_decoding_session ; alloc_decoding_session . decode_alloc_id (self) } }}}
mkitem!{crate :: implement_ty_decoder ! (CacheDecoder <'a , 'tcx >) ;}
mkitem!{mkimpl!{impl < 'a , 'tcx > Decodable < CacheDecoder < 'a , 'tcx > > for Vec < u8 > { fn decode (d : & mut CacheDecoder < 'a , 'tcx >) -> Self { Decodable :: decode (& mut d . opaque) } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > SpanDecoder for CacheDecoder < 'a , 'tcx > { fn decode_syntax_context (& mut self) -> SyntaxContext { let syntax_contexts = self . syntax_contexts ; rustc_span :: hygiene :: decode_syntax_context (self , self . hygiene_context , | this , id | { let pos = syntax_contexts . get (& id) . unwrap () ; this . with_position (pos . to_usize () , | decoder | { let data : SyntaxContextKey = decode_tagged (decoder , TAG_SYNTAX_CONTEXT) ; data }) }) } fn decode_expn_id (& mut self) -> ExpnId { let hash = ExpnHash :: decode (self) ; if hash . is_root () { return ExpnId :: root () ; } if let Some (expn_id) = ExpnId :: from_hash (hash) { return expn_id ; } let krate = self . tcx . stable_crate_id_to_crate_num (hash . stable_crate_id ()) ; let expn_id = if krate == LOCAL_CRATE { let pos = self . expn_data . get (& hash) . unwrap_or_else (| | panic ! ("Bad hash {:?} (map {:?})" , hash , self . expn_data)) ; let data : ExpnData = self . with_position (pos . to_usize () , | decoder | decode_tagged (decoder , TAG_EXPN_DATA)) ; let expn_id = rustc_span :: hygiene :: register_local_expn_id (data , hash) ; #[cfg (debug_assertions)] { use rustc_data_structures :: stable_hasher :: { HashStable , StableHasher } ; let local_hash = self . tcx . with_stable_hashing_context (| mut hcx | { let mut hasher = StableHasher :: new () ; expn_id . expn_data () . hash_stable (& mut hcx , & mut hasher) ; hasher . finish () }) ; debug_assert_eq ! (hash . local_hash () , local_hash) ; } expn_id } else { let index_guess = self . foreign_expn_data [& hash] ; self . tcx . expn_hash_to_expn_id (krate , index_guess , hash) } ; debug_assert_eq ! (expn_id . krate , krate) ; expn_id } fn decode_span (& mut self) -> Span { let ctxt = SyntaxContext :: decode (self) ; let parent = Option :: < LocalDefId > :: decode (self) ; let tag : u8 = Decodable :: decode (self) ; let (lo , hi) = match tag { TAG_PARTIAL_SPAN => (BytePos (0) , BytePos (0)) , TAG_RELATIVE_SPAN => { let dlo = u32 :: decode (self) ; let dto = u32 :: decode (self) ; let enclosing = self . tcx . source_span_untracked (parent . unwrap ()) . data_untracked () ; (enclosing . lo + BytePos :: from_u32 (dlo) , enclosing . lo + BytePos :: from_u32 (dto)) } TAG_FULL_SPAN => { let file_lo_index = SourceFileIndex :: decode (self) ; let line_lo = usize :: decode (self) ; let col_lo = RelativeBytePos :: decode (self) ; let len = BytePos :: decode (self) ; let file_lo = self . file_index_to_file (file_lo_index) ; let lo = file_lo . lines () [line_lo - 1] + col_lo ; let lo = file_lo . absolute_position (lo) ; let hi = lo + len ; (lo , hi) } _ => unreachable ! () , } ; Span :: new (lo , hi , ctxt , parent) } fn decode_symbol (& mut self) -> Symbol { self . decode_symbol_or_byte_symbol (Symbol :: new , | this | Symbol :: intern (this . read_str ()) , | opaque | Symbol :: intern (opaque . read_str ()) ,) } fn decode_byte_symbol (& mut self) -> ByteSymbol { self . decode_symbol_or_byte_symbol (ByteSymbol :: new , | this | ByteSymbol :: intern (this . read_byte_str ()) , | opaque | ByteSymbol :: intern (opaque . read_byte_str ()) ,) } fn decode_crate_num (& mut self) -> CrateNum { let stable_id = StableCrateId :: decode (self) ; let cnum = self . tcx . stable_crate_id_to_crate_num (stable_id) ; cnum } fn decode_def_index (& mut self) -> DefIndex { panic ! ("trying to decode `DefIndex` outside the context of a `DefId`") } fn decode_def_id (& mut self) -> DefId { let def_path_hash = DefPathHash :: decode (self) ; match self . tcx . def_path_hash_to_def_id (def_path_hash) { Some (r) => r , None => panic ! ("Failed to convert DefPathHash {def_path_hash:?}") , } } fn decode_attr_id (& mut self) -> rustc_span :: AttrId { panic ! ("cannot decode `AttrId` with `CacheDecoder`") ; } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Decodable < CacheDecoder < 'a , 'tcx > > for & 'tcx UnordSet < LocalDefId > { #[inline] fn decode (d : & mut CacheDecoder < 'a , 'tcx >) -> Self { RefDecodable :: decode (d) } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Decodable < CacheDecoder < 'a , 'tcx > > for & 'tcx UnordMap < DefId , ty :: EarlyBinder < 'tcx , Ty < 'tcx > > > { #[inline] fn decode (d : & mut CacheDecoder < 'a , 'tcx >) -> Self { RefDecodable :: decode (d) } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Decodable < CacheDecoder < 'a , 'tcx > > for & 'tcx IndexVec < mir :: Promoted , mir :: Body < 'tcx > > { #[inline] fn decode (d : & mut CacheDecoder < 'a , 'tcx >) -> Self { RefDecodable :: decode (d) } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Decodable < CacheDecoder < 'a , 'tcx > > for & 'tcx [(ty :: Clause < 'tcx > , Span)] { #[inline] fn decode (d : & mut CacheDecoder < 'a , 'tcx >) -> Self { RefDecodable :: decode (d) } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Decodable < CacheDecoder < 'a , 'tcx > > for & 'tcx [rustc_ast :: InlineAsmTemplatePiece] { #[inline] fn decode (d : & mut CacheDecoder < 'a , 'tcx >) -> Self { RefDecodable :: decode (d) } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Decodable < CacheDecoder < 'a , 'tcx > > for & 'tcx [Spanned < MonoItem < 'tcx > >] { #[inline] fn decode (d : & mut CacheDecoder < 'a , 'tcx >) -> Self { RefDecodable :: decode (d) } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Decodable < CacheDecoder < 'a , 'tcx > > for & 'tcx crate :: traits :: specialization_graph :: Graph { #[inline] fn decode (d : & mut CacheDecoder < 'a , 'tcx >) -> Self { RefDecodable :: decode (d) } }}}
mkitem!{macro_rules ! impl_ref_decoder { (<$ tcx : tt > $ ($ ty : ty ,) *) => { $ (impl <'a , $ tcx > Decodable < CacheDecoder <'a , $ tcx >> for &$ tcx [$ ty] { #[inline] fn decode (d : & mut CacheDecoder <'a , $ tcx >) -> Self { RefDecodable :: decode (d) } }) * } ; }}
mkitem!{impl_ref_decoder ! { <'tcx > Span , rustc_hir :: Attribute , rustc_span :: Ident , ty :: Variance , rustc_span :: def_id :: DefId , rustc_span :: def_id :: LocalDefId , (rustc_middle :: middle :: exported_symbols :: ExportedSymbol <'tcx >, rustc_middle :: middle :: exported_symbols :: SymbolExportInfo) , ty :: DeducedParamAttrs , }}
mkitem!{mkstruct!{#[doc = " An encoder that can write to the incremental compilation cache."] pub struct CacheEncoder < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , encoder : FileEncoder , type_shorthands : FxHashMap < Ty < 'tcx > , usize > , predicate_shorthands : FxHashMap < ty :: PredicateKind < 'tcx > , usize > , interpret_allocs : FxIndexSet < interpret :: AllocId > , source_map : CachingSourceMapView < 'tcx > , file_to_file_index : FxHashMap < * const SourceFile , SourceFileIndex > , hygiene_context : & 'a HygieneEncodeContext , symbol_index_table : FxHashMap < u32 , usize > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > CacheEncoder < 'a , 'tcx > { #[inline] fn source_file_index (& mut self , source_file : Arc < SourceFile >) -> SourceFileIndex { self . file_to_file_index [& (& raw const * source_file)] } #[doc = " Encode something with additional information that allows to do some"] #[doc = " sanity checks when decoding the data again. This method will first"] #[doc = " encode the specified tag, then the given value, then the number of"] #[doc = " bytes taken up by tag and value. On decoding, we can then verify that"] #[doc = " we get the expected tag and read the expected number of bytes."] pub fn encode_tagged < T : Encodable < Self > , V : Encodable < Self > > (& mut self , tag : T , value : & V) { let start_pos = self . position () ; tag . encode (self) ; value . encode (self) ; let end_pos = self . position () ; ((end_pos - start_pos) as u64) . encode (self) ; } fn encode_symbol_or_byte_symbol (& mut self , index : u32 , emit_str_or_byte_str : impl Fn (& mut Self) ,) { if Symbol :: is_predefined (index) { self . encoder . emit_u8 (SYMBOL_PREDEFINED) ; self . encoder . emit_u32 (index) ; } else { match self . symbol_index_table . entry (index) { Entry :: Vacant (o) => { self . encoder . emit_u8 (SYMBOL_STR) ; let pos = self . encoder . position () ; o . insert (pos) ; emit_str_or_byte_str (self) ; } Entry :: Occupied (o) => { let x = * o . get () ; self . emit_u8 (SYMBOL_OFFSET) ; self . emit_usize (x) ; } } } } #[inline] fn finish (mut self) -> FileEncodeResult { self . encoder . finish () } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > SpanEncoder for CacheEncoder < 'a , 'tcx > { fn encode_syntax_context (& mut self , syntax_context : SyntaxContext) { rustc_span :: hygiene :: raw_encode_syntax_context (syntax_context , self . hygiene_context , self) ; } fn encode_expn_id (& mut self , expn_id : ExpnId) { self . hygiene_context . schedule_expn_data_for_encoding (expn_id) ; expn_id . expn_hash () . encode (self) ; } fn encode_span (& mut self , span : Span) { let span_data = span . data_untracked () ; span_data . ctxt . encode (self) ; span_data . parent . encode (self) ; if span_data . is_dummy () { return TAG_PARTIAL_SPAN . encode (self) ; } if let Some (parent) = span_data . parent { let enclosing = self . tcx . source_span_untracked (parent) . data_untracked () ; if enclosing . contains (span_data) { TAG_RELATIVE_SPAN . encode (self) ; (span_data . lo - enclosing . lo) . to_u32 () . encode (self) ; (span_data . hi - enclosing . lo) . to_u32 () . encode (self) ; return ; } } let pos = self . source_map . byte_pos_to_line_and_col (span_data . lo) ; let partial_span = match & pos { Some ((file_lo , _ , _)) => ! file_lo . contains (span_data . hi) , None => true , } ; if partial_span { return TAG_PARTIAL_SPAN . encode (self) ; } let (file_lo , line_lo , col_lo) = pos . unwrap () ; let len = span_data . hi - span_data . lo ; let source_file_index = self . source_file_index (file_lo) ; TAG_FULL_SPAN . encode (self) ; source_file_index . encode (self) ; line_lo . encode (self) ; col_lo . encode (self) ; len . encode (self) ; } fn encode_symbol (& mut self , sym : Symbol) { self . encode_symbol_or_byte_symbol (sym . as_u32 () , | this | this . emit_str (sym . as_str ())) ; } fn encode_byte_symbol (& mut self , byte_sym : ByteSymbol) { self . encode_symbol_or_byte_symbol (byte_sym . as_u32 () , | this | { this . emit_byte_str (byte_sym . as_byte_str ()) }) ; } fn encode_crate_num (& mut self , crate_num : CrateNum) { self . tcx . stable_crate_id (crate_num) . encode (self) ; } fn encode_def_id (& mut self , def_id : DefId) { self . tcx . def_path_hash (def_id) . encode (self) ; } fn encode_def_index (& mut self , _def_index : DefIndex) { bug ! ("encoding `DefIndex` without context") ; } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > TyEncoder < 'tcx > for CacheEncoder < 'a , 'tcx > { const CLEAR_CROSS_CRATE : bool = false ; #[inline] fn position (& self) -> usize { self . encoder . position () } #[inline] fn type_shorthands (& mut self) -> & mut FxHashMap < Ty < 'tcx > , usize > { & mut self . type_shorthands } #[inline] fn predicate_shorthands (& mut self) -> & mut FxHashMap < ty :: PredicateKind < 'tcx > , usize > { & mut self . predicate_shorthands } #[inline] fn encode_alloc_id (& mut self , alloc_id : & interpret :: AllocId) { let (index , _) = self . interpret_allocs . insert_full (* alloc_id) ; index . encode (self) ; } }}}
mkitem!{macro_rules ! encoder_methods { ($ ($ name : ident ($ ty : ty) ;) *) => { #[inline] $ (fn $ name (& mut self , value : $ ty) { self . encoder .$ name (value) }) * } }}
mkitem!{mkimpl!{impl < 'a , 'tcx > Encoder for CacheEncoder < 'a , 'tcx > { encoder_methods ! { emit_usize (usize) ; emit_u128 (u128) ; emit_u64 (u64) ; emit_u32 (u32) ; emit_u16 (u16) ; emit_u8 (u8) ; emit_isize (isize) ; emit_i128 (i128) ; emit_i64 (i64) ; emit_i32 (i32) ; emit_i16 (i16) ; emit_raw_bytes (& [u8]) ; } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Encodable < CacheEncoder < 'a , 'tcx > > for [u8] { fn encode (& self , e : & mut CacheEncoder < 'a , 'tcx >) { self . encode (& mut e . encoder) ; } }}}