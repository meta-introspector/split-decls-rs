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
mkuse!{use std :: borrow :: Cow ;}
mkuse!{use std :: fmt ;}
mkuse!{use std :: hash :: Hash ;}
mkuse!{use rustc_data_structures :: base_n :: { BaseNString , CASE_INSENSITIVE , ToBaseN } ;}
mkuse!{use rustc_data_structures :: fingerprint :: Fingerprint ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexMap ;}
mkuse!{use rustc_data_structures :: stable_hasher :: { HashStable , StableHasher , ToStableHashKey } ;}
mkuse!{use rustc_data_structures :: unord :: UnordMap ;}
mkuse!{use rustc_hashes :: Hash128 ;}
mkuse!{use rustc_hir :: ItemId ;}
mkuse!{use rustc_hir :: attrs :: { InlineAttr , Linkage } ;}
mkuse!{use rustc_hir :: def_id :: { CrateNum , DefId , DefIdSet , LOCAL_CRATE } ;}
mkuse!{use rustc_macros :: { HashStable , TyDecodable , TyEncodable } ;}
mkuse!{use rustc_query_system :: ich :: StableHashingContext ;}
mkuse!{use rustc_session :: config :: OptLevel ;}
mkuse!{use rustc_span :: { Span , Symbol } ;}
mkuse!{use rustc_target :: spec :: SymbolVisibility ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: dep_graph :: { DepNode , WorkProduct , WorkProductId } ;}
mkuse!{use crate :: middle :: codegen_fn_attrs :: CodegenFnAttrFlags ;}
mkuse!{use crate :: ty :: { self , GenericArgs , Instance , InstanceKind , SymbolName , Ty , TyCtxt } ;}
mkitem!{mkenum!{#[doc = " Describes how a monomorphization will be instantiated in object files."] #[derive (PartialEq)] pub enum InstantiationMode { #[doc = " There will be exactly one instance of the given MonoItem. It will have"] #[doc = " external linkage so that it can be linked to from other codegen units."] GloballyShared { #[doc = " In some compilation scenarios we may decide to take functions that"] #[doc = " are typically `LocalCopy` and instead move them to `GloballyShared`"] #[doc = " to avoid codegenning them a bunch of times. In this situation,"] #[doc = " however, our local copy may conflict with other crates also"] #[doc = " inlining the same function."] #[doc = ""] #[doc = " This flag indicates that this situation is occurring, and informs"] #[doc = " symbol name calculation that some extra mangling is needed to"] #[doc = " avoid conflicts. Note that this may eventually go away entirely if"] #[doc = " ThinLTO enables us to *always* have a globally shared instance of a"] #[doc = " function within one crate's compilation."] may_conflict : bool , } , #[doc = " Each codegen unit containing a reference to the given MonoItem will"] #[doc = " have its own private copy of the function (with internal linkage)."] LocalCopy , }}}
mkitem!{mkenum!{#[derive (PartialEq , Eq , Clone , Copy , Debug , Hash , HashStable , TyEncodable , TyDecodable)] pub enum MonoItem < 'tcx > { Fn (Instance < 'tcx >) , Static (DefId) , GlobalAsm (ItemId) , }}}

macro_rules! opt_incr_drop_glue_mode_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function opt_incr_drop_glue_mode in module {}", module_path!());
    };
}

mkfn!{
    opt_incr_drop_glue_mode_introspect!();
    fn opt_incr_drop_glue_mode < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx >) -> InstantiationMode { let ty :: Adt (adt_def , _) = ty . kind () else { return InstantiationMode :: LocalCopy ; } ; let Some (dtor) = adt_def . destructor (tcx) else { if adt_def . is_enum () { return InstantiationMode :: LocalCopy ; } else { return InstantiationMode :: GloballyShared { may_conflict : true } ; } } ; if tcx . cross_crate_inlinable (dtor . did) { InstantiationMode :: LocalCopy } else { InstantiationMode :: GloballyShared { may_conflict : true } } }
}
mkitem!{mkimpl!{impl < 'tcx > MonoItem < 'tcx > { #[doc = " Returns `true` if the mono item is user-defined (i.e. not compiler-generated, like shims)."] pub fn is_user_defined (& self) -> bool { match * self { MonoItem :: Fn (instance) => matches ! (instance . def , InstanceKind :: Item (..)) , MonoItem :: Static (..) | MonoItem :: GlobalAsm (..) => true , } } pub fn size_estimate (& self , tcx : TyCtxt < 'tcx >) -> usize { match * self { MonoItem :: Fn (instance) => tcx . size_estimate (instance) , MonoItem :: Static (_) | MonoItem :: GlobalAsm (_) => 1 , } } pub fn is_generic_fn (& self) -> bool { match self { MonoItem :: Fn (instance) => instance . args . non_erasable_generics () . next () . is_some () , MonoItem :: Static (..) | MonoItem :: GlobalAsm (..) => false , } } pub fn symbol_name (& self , tcx : TyCtxt < 'tcx >) -> SymbolName < 'tcx > { match * self { MonoItem :: Fn (instance) => tcx . symbol_name (instance) , MonoItem :: Static (def_id) => tcx . symbol_name (Instance :: mono (tcx , def_id)) , MonoItem :: GlobalAsm (item_id) => { SymbolName :: new (tcx , & format ! ("global_asm_{:?}" , item_id . owner_id)) } } } pub fn instantiation_mode (& self , tcx : TyCtxt < 'tcx >) -> InstantiationMode { let instance = match * self { MonoItem :: Fn (instance) => instance , MonoItem :: Static (..) | MonoItem :: GlobalAsm (..) => { return InstantiationMode :: GloballyShared { may_conflict : false } ; } } ; if tcx . is_entrypoint (instance . def_id ()) { return InstantiationMode :: GloballyShared { may_conflict : false } ; } let codegen_fn_attrs = tcx . codegen_instance_attrs (instance . def) ; if codegen_fn_attrs . contains_extern_indicator () || codegen_fn_attrs . flags . contains (CodegenFnAttrFlags :: NAKED) { return InstantiationMode :: GloballyShared { may_conflict : false } ; } if let InstanceKind :: DropGlue (_ , Some (ty)) = instance . def { if tcx . sess . opts . optimize == OptLevel :: No { return InstantiationMode :: GloballyShared { may_conflict : false } ; } if tcx . sess . opts . incremental . is_none () { return InstantiationMode :: LocalCopy ; } return opt_incr_drop_glue_mode (tcx , ty) ; } if ! tcx . cross_crate_inlinable (instance . def_id ()) && ! instance . def . requires_inline (tcx) { return InstantiationMode :: GloballyShared { may_conflict : false } ; } if tcx . sess . link_dead_code () { return InstantiationMode :: GloballyShared { may_conflict : true } ; } if codegen_fn_attrs . inline . always () { return InstantiationMode :: LocalCopy ; } if let InlineAttr :: Never = codegen_fn_attrs . inline && self . is_generic_fn () { return InstantiationMode :: GloballyShared { may_conflict : true } ; } match tcx . sess . opts . optimize { OptLevel :: No => InstantiationMode :: GloballyShared { may_conflict : true } , _ => InstantiationMode :: LocalCopy , } } pub fn explicit_linkage (& self , tcx : TyCtxt < 'tcx >) -> Option < Linkage > { let instance_kind = match * self { MonoItem :: Fn (ref instance) => instance . def , MonoItem :: Static (def_id) => InstanceKind :: Item (def_id) , MonoItem :: GlobalAsm (..) => return None , } ; tcx . codegen_instance_attrs (instance_kind) . linkage } #[doc = " Returns `true` if this instance is instantiable - whether it has no unsatisfied"] #[doc = " predicates."] #[doc = ""] #[doc = " In order to codegen an item, all of its predicates must hold, because"] #[doc = " otherwise the item does not make sense. Type-checking ensures that"] #[doc = " the predicates of every item that is *used by* a valid item *do*"] #[doc = " hold, so we can rely on that."] #[doc = ""] #[doc = " However, we codegen collector roots (reachable items) and functions"] #[doc = " in vtables when they are seen, even if they are not used, and so they"] #[doc = " might not be instantiable. For example, a programmer can define this"] #[doc = " public function:"] #[doc = ""] #[doc = "     pub fn foo<'a>(s: &'a mut ()) where &'a mut (): Clone {"] #[doc = "         <&mut () as Clone>::clone(&s);"] #[doc = "     }"] #[doc = ""] #[doc = " That function can't be codegened, because the method `<&mut () as Clone>::clone`"] #[doc = " does not exist. Luckily for us, that function can't ever be used,"] #[doc = " because that would require for `&'a mut (): Clone` to hold, so we"] #[doc = " can just not emit any code, or even a linker reference for it."] #[doc = ""] #[doc = " Similarly, if a vtable method has such a signature, and therefore can't"] #[doc = " be used, we can just not emit it and have a placeholder (a null pointer,"] #[doc = " which will never be accessed) in its place."] pub fn is_instantiable (& self , tcx : TyCtxt < 'tcx >) -> bool { debug ! ("is_instantiable({:?})" , self) ; let (def_id , args) = match * self { MonoItem :: Fn (ref instance) => (instance . def_id () , instance . args) , MonoItem :: Static (def_id) => (def_id , GenericArgs :: empty ()) , MonoItem :: GlobalAsm (..) => return true , } ; ! tcx . instantiate_and_check_impossible_predicates ((def_id , & args)) } pub fn local_span (& self , tcx : TyCtxt < 'tcx >) -> Option < Span > { match * self { MonoItem :: Fn (Instance { def , .. }) => def . def_id () . as_local () , MonoItem :: Static (def_id) => def_id . as_local () , MonoItem :: GlobalAsm (item_id) => Some (item_id . owner_id . def_id) , } . map (| def_id | tcx . def_span (def_id)) } pub fn codegen_dep_node (& self , tcx : TyCtxt < 'tcx >) -> DepNode { crate :: dep_graph :: make_compile_mono_item (tcx , self) } #[doc = " Returns the item's `CrateNum`"] pub fn krate (& self) -> CrateNum { match self { MonoItem :: Fn (instance) => instance . def_id () . krate , MonoItem :: Static (def_id) => def_id . krate , MonoItem :: GlobalAsm (..) => LOCAL_CRATE , } } #[doc = " Returns the item's `DefId`"] pub fn def_id (& self) -> DefId { match * self { MonoItem :: Fn (Instance { def , .. }) => def . def_id () , MonoItem :: Static (def_id) => def_id , MonoItem :: GlobalAsm (item_id) => item_id . owner_id . to_def_id () , } } }}}
mkitem!{mkimpl!{impl < 'tcx > fmt :: Display for MonoItem < 'tcx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { MonoItem :: Fn (instance) => write ! (f , "fn {instance}") , MonoItem :: Static (def_id) => { write ! (f , "static {}" , Instance :: new_raw (def_id , GenericArgs :: empty ())) } MonoItem :: GlobalAsm (..) => write ! (f , "global_asm") , } } }}}
mkitem!{mkimpl!{impl ToStableHashKey < StableHashingContext < '_ > > for MonoItem < '_ > { type KeyType = Fingerprint ; fn to_stable_hash_key (& self , hcx : & StableHashingContext < '_ >) -> Self :: KeyType { let mut hasher = StableHasher :: new () ; self . hash_stable (& mut hcx . clone () , & mut hasher) ; hasher . finish () } }}}
mkitem!{mkstruct!{#[derive (Debug , HashStable , Copy , Clone)] pub struct MonoItemPartitions < 'tcx > { pub codegen_units : & 'tcx [CodegenUnit < 'tcx >] , pub all_mono_items : & 'tcx DefIdSet , }}}
mkitem!{mkstruct!{#[derive (Debug , HashStable)] pub struct CodegenUnit < 'tcx > { #[doc = " A name for this CGU. Incremental compilation requires that"] #[doc = " name be unique amongst **all** crates. Therefore, it should"] #[doc = " contain something unique to this crate (e.g., a module path)"] #[doc = " as well as the crate name and disambiguator."] name : Symbol , items : FxIndexMap < MonoItem < 'tcx > , MonoItemData > , size_estimate : usize , primary : bool , #[doc = " True if this is CGU is used to hold code coverage information for dead code,"] #[doc = " false otherwise."] is_code_coverage_dead_code_cgu : bool , }}}
mkitem!{mkstruct!{#[doc = " Auxiliary info about a `MonoItem`."] #[derive (Copy , Clone , PartialEq , Debug , HashStable)] pub struct MonoItemData { #[doc = " A cached copy of the result of `MonoItem::instantiation_mode`, where"] #[doc = " `GloballyShared` maps to `false` and `LocalCopy` maps to `true`."] pub inlined : bool , pub linkage : Linkage , pub visibility : Visibility , #[doc = " A cached copy of the result of `MonoItem::size_estimate`."] pub size_estimate : usize , }}}
mkitem!{mkenum!{#[doc = " Specifies the symbol visibility with regards to dynamic linking."] #[doc = ""] #[doc = " Visibility doesn't have any effect when linkage is internal."] #[doc = ""] #[doc = " DSO means dynamic shared object, that is a dynamically linked executable or dylib."] #[derive (Copy , Clone , PartialEq , Debug , HashStable)] pub enum Visibility { #[doc = " Export the symbol from the DSO and apply overrides of the symbol by outside DSOs to within"] #[doc = " the DSO if the object file format supports this."] Default , #[doc = " Hide the symbol outside of the defining DSO even when external linkage is used to export it"] #[doc = " from the object file."] Hidden , #[doc = " Export the symbol from the DSO, but don't apply overrides of the symbol by outside DSOs to"] #[doc = " within the DSO. Equivalent to default visibility with object file formats that don't support"] #[doc = " overriding exported symbols by another DSO."] Protected , }}}
mkitem!{mkimpl!{impl From < SymbolVisibility > for Visibility { fn from (value : SymbolVisibility) -> Self { match value { SymbolVisibility :: Hidden => Visibility :: Hidden , SymbolVisibility :: Protected => Visibility :: Protected , SymbolVisibility :: Interposable => Visibility :: Default , } } }}}
mkitem!{mkimpl!{impl < 'tcx > CodegenUnit < 'tcx > { #[inline] pub fn new (name : Symbol) -> CodegenUnit < 'tcx > { CodegenUnit { name , items : Default :: default () , size_estimate : 0 , primary : false , is_code_coverage_dead_code_cgu : false , } } pub fn name (& self) -> Symbol { self . name } pub fn set_name (& mut self , name : Symbol) { self . name = name ; } pub fn is_primary (& self) -> bool { self . primary } pub fn make_primary (& mut self) { self . primary = true ; } pub fn items (& self) -> & FxIndexMap < MonoItem < 'tcx > , MonoItemData > { & self . items } pub fn items_mut (& mut self) -> & mut FxIndexMap < MonoItem < 'tcx > , MonoItemData > { & mut self . items } pub fn is_code_coverage_dead_code_cgu (& self) -> bool { self . is_code_coverage_dead_code_cgu } #[doc = " Marks this CGU as the one used to contain code coverage information for dead code."] pub fn make_code_coverage_dead_code_cgu (& mut self) { self . is_code_coverage_dead_code_cgu = true ; } pub fn mangle_name (human_readable_name : & str) -> BaseNString { let mut hasher = StableHasher :: new () ; human_readable_name . hash (& mut hasher) ; let hash : Hash128 = hasher . finish () ; hash . as_u128 () . to_base_fixed_len (CASE_INSENSITIVE) } pub fn shorten_name (human_readable_name : & str) -> Cow < '_ , str > { const MAX_CGU_NAME_LENGTH : usize = 200 ; const TRUNCATED_NAME_PREFIX : & str = "-trunc-" ; if human_readable_name . len () > MAX_CGU_NAME_LENGTH { let mangled_name = Self :: mangle_name (human_readable_name) ; let truncate_to = human_readable_name . floor_char_boundary (MAX_CGU_NAME_LENGTH - TRUNCATED_NAME_PREFIX . len () - mangled_name . len () ,) ; format ! ("{}{}{}" , & human_readable_name [.. truncate_to] , TRUNCATED_NAME_PREFIX , mangled_name) . into () } else { human_readable_name . into () } } pub fn compute_size_estimate (& mut self) { self . size_estimate = self . items . values () . map (| data | data . size_estimate) . sum () ; } #[doc = " Should only be called if [`compute_size_estimate`] has previously been called."] #[doc = ""] #[doc = " [`compute_size_estimate`]: Self::compute_size_estimate"] #[inline] pub fn size_estimate (& self) -> usize { assert ! (self . items . is_empty () || self . size_estimate != 0) ; self . size_estimate } pub fn contains_item (& self , item : & MonoItem < 'tcx >) -> bool { self . items () . contains_key (item) } pub fn work_product_id (& self) -> WorkProductId { WorkProductId :: from_cgu_name (self . name () . as_str ()) } pub fn previous_work_product (& self , tcx : TyCtxt < '_ >) -> WorkProduct { let work_product_id = self . work_product_id () ; tcx . dep_graph . previous_work_product (& work_product_id) . unwrap_or_else (| | panic ! ("Could not find work-product for CGU `{}`" , self . name ())) } pub fn items_in_deterministic_order (& self , tcx : TyCtxt < 'tcx > ,) -> Vec < (MonoItem < 'tcx > , MonoItemData) > { #[derive (PartialEq , Eq , PartialOrd , Ord)] struct ItemSortKey < 'tcx > (Option < Span > , SymbolName < 'tcx >) ; fn local_item_id < 'tcx > (item : MonoItem < 'tcx >) -> Option < DefId > { match item { MonoItem :: Fn (ref instance) => match instance . def { InstanceKind :: Item (def) => def . as_local () . map (| _ | def) , InstanceKind :: VTableShim (..) | InstanceKind :: ReifyShim (..) | InstanceKind :: Intrinsic (..) | InstanceKind :: FnPtrShim (..) | InstanceKind :: Virtual (..) | InstanceKind :: ClosureOnceShim { .. } | InstanceKind :: ConstructCoroutineInClosureShim { .. } | InstanceKind :: DropGlue (..) | InstanceKind :: CloneShim (..) | InstanceKind :: ThreadLocalShim (..) | InstanceKind :: FnPtrAddrShim (..) | InstanceKind :: AsyncDropGlue (..) | InstanceKind :: FutureDropPollShim (..) | InstanceKind :: AsyncDropGlueCtorShim (..) => None , } , MonoItem :: Static (def_id) => def_id . as_local () . map (| _ | def_id) , MonoItem :: GlobalAsm (item_id) => Some (item_id . owner_id . def_id . to_def_id ()) , } } fn item_sort_key < 'tcx > (tcx : TyCtxt < 'tcx > , item : MonoItem < 'tcx >) -> ItemSortKey < 'tcx > { ItemSortKey (local_item_id (item) . map (| def_id | tcx . def_span (def_id) . find_ancestor_not_from_macro ()) . flatten () , item . symbol_name (tcx) ,) } let mut items : Vec < _ > = self . items () . iter () . map (| (& i , & data) | (i , data)) . collect () ; if ! tcx . sess . opts . unstable_opts . codegen_source_order { items . sort_by_cached_key (| & (i , _) | i . symbol_name (tcx)) ; } else { items . sort_by_cached_key (| & (i , _) | item_sort_key (tcx , i)) ; } items } pub fn codegen_dep_node (& self , tcx : TyCtxt < 'tcx >) -> DepNode { crate :: dep_graph :: make_compile_codegen_unit (tcx , self . name ()) } }}}
mkitem!{mkimpl!{impl ToStableHashKey < StableHashingContext < '_ > > for CodegenUnit < '_ > { type KeyType = String ; fn to_stable_hash_key (& self , _ : & StableHashingContext < '_ >) -> Self :: KeyType { self . name . to_string () } }}}
mkitem!{mkstruct!{pub struct CodegenUnitNameBuilder < 'tcx > { tcx : TyCtxt < 'tcx > , cache : UnordMap < CrateNum , String > , }}}
mkitem!{mkimpl!{impl < 'tcx > CodegenUnitNameBuilder < 'tcx > { pub fn new (tcx : TyCtxt < 'tcx >) -> Self { CodegenUnitNameBuilder { tcx , cache : Default :: default () } } #[doc = " CGU names should fulfill the following requirements:"] #[doc = " - They should be able to act as a file name on any kind of file system"] #[doc = " - They should not collide with other CGU names, even for different versions"] #[doc = "   of the same crate."] #[doc = ""] #[doc = " Consequently, we don't use special characters except for '.' and '-' and we"] #[doc = " prefix each name with the crate-name and crate-disambiguator."] #[doc = ""] #[doc = " This function will build CGU names of the form:"] #[doc = ""] #[doc = " ```text"] #[doc = " <crate-name>.<crate-disambiguator>[-in-<local-crate-id>](-<component>)*[.<special-suffix>]"] #[doc = " <local-crate-id> = <local-crate-name>.<local-crate-disambiguator>"] #[doc = " ```"] #[doc = ""] #[doc = " The '.' before `<special-suffix>` makes sure that names with a special"] #[doc = " suffix can never collide with a name built out of regular Rust"] #[doc = " identifiers (e.g., module paths)."] pub fn build_cgu_name < I , C , S > (& mut self , cnum : CrateNum , components : I , special_suffix : Option < S > ,) -> Symbol where I : IntoIterator < Item = C > , C : fmt :: Display , S : fmt :: Display , { let cgu_name = self . build_cgu_name_no_mangle (cnum , components , special_suffix) ; if self . tcx . sess . opts . unstable_opts . human_readable_cgu_names { Symbol :: intern (& CodegenUnit :: shorten_name (cgu_name . as_str ())) } else { Symbol :: intern (& CodegenUnit :: mangle_name (cgu_name . as_str ())) } } #[doc = " Same as `CodegenUnit::build_cgu_name()` but will never mangle the"] #[doc = " resulting name."] pub fn build_cgu_name_no_mangle < I , C , S > (& mut self , cnum : CrateNum , components : I , special_suffix : Option < S > ,) -> Symbol where I : IntoIterator < Item = C > , C : fmt :: Display , S : fmt :: Display , { use std :: fmt :: Write ; let mut cgu_name = String :: with_capacity (64) ; let tcx = self . tcx ; let crate_prefix = self . cache . entry (cnum) . or_insert_with (| | { let local_crate_id = if cnum != LOCAL_CRATE { let local_stable_crate_id = tcx . stable_crate_id (LOCAL_CRATE) ; format ! ("-in-{}.{:08x}" , tcx . crate_name (LOCAL_CRATE) , local_stable_crate_id) } else { String :: new () } ; let stable_crate_id = tcx . stable_crate_id (LOCAL_CRATE) ; format ! ("{}.{:08x}{}" , tcx . crate_name (cnum) , stable_crate_id , local_crate_id) }) ; write ! (cgu_name , "{crate_prefix}") . unwrap () ; for component in components { write ! (cgu_name , "-{component}") . unwrap () ; } if let Some (special_suffix) = special_suffix { write ! (cgu_name , ".{special_suffix}") . unwrap () ; } Symbol :: intern (& cgu_name) } }}}
mkitem!{mkenum!{#[doc = " See module-level docs of `rustc_monomorphize::collector` on some context for \"mentioned\" items."] #[derive (Copy , Clone , Debug , PartialEq , Eq , Hash , HashStable)] pub enum CollectionMode { #[doc = " Collect items that are used, i.e., actually needed for codegen."] #[doc = ""] #[doc = " Which items are used can depend on optimization levels, as MIR optimizations can remove"] #[doc = " uses."] UsedItems , #[doc = " Collect items that are mentioned. The goal of this mode is that it is independent of"] #[doc = " optimizations: the set of \"mentioned\" items is computed before optimizations are run."] #[doc = ""] #[doc = " The exact contents of this set are *not* a stable guarantee. (For instance, it is currently"] #[doc = " computed after drop-elaboration. If we ever do some optimizations even in debug builds, we"] #[doc = " might decide to run them before computing mentioned items.) The key property of this set is"] #[doc = " that it is optimization-independent."] MentionedItems , }}}