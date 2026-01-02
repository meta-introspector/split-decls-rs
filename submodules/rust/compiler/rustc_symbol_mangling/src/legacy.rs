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
mkuse!{use std :: fmt :: { self , Write } ;}
mkuse!{use std :: mem :: { self , discriminant } ;}
mkuse!{use rustc_data_structures :: stable_hasher :: { HashStable , StableHasher } ;}
mkuse!{use rustc_hashes :: Hash64 ;}
mkuse!{use rustc_hir :: def_id :: { CrateNum , DefId } ;}
mkuse!{use rustc_hir :: definitions :: { DefPathData , DisambiguatedDefPathData } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: print :: { PrettyPrinter , Print , PrintError , Printer } ;}
mkuse!{use rustc_middle :: ty :: { self , GenericArg , GenericArgKind , Instance , ReifyReason , Ty , TyCtxt , TypeVisitableExt , } ;}
mkuse!{use tracing :: debug ;}

macro_rules! mangle_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mangle in module {}", module_path!());
    };
}

mkfn!{
    mangle_introspect!();
    pub (super) fn mangle < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , instantiating_crate : Option < CrateNum > ,) -> String { let def_id = instance . def_id () ; let mut ty_def_id = def_id ; let instance_ty ; loop { let key = tcx . def_key (ty_def_id) ; match key . disambiguated_data . data { DefPathData :: TypeNs (_) | DefPathData :: ValueNs (_) | DefPathData :: Closure | DefPathData :: SyntheticCoroutineBody => { instance_ty = tcx . type_of (ty_def_id) . instantiate_identity () ; debug ! (? instance_ty) ; break ; } _ => { ty_def_id . index = key . parent . unwrap_or_else (| | { bug ! ("finding type for {:?}, encountered def-id {:?} with no \
                         parent" , def_id , ty_def_id) ; }) ; } } } let instance_ty = tcx . erase_and_anonymize_regions (instance_ty) ; let hash = get_symbol_hash (tcx , instance , instance_ty , instantiating_crate) ; let mut p = LegacySymbolMangler { tcx , path : SymbolPath :: new () , keep_within_component : false } ; p . print_def_path (def_id , if let ty :: InstanceKind :: DropGlue (_ , _) | ty :: InstanceKind :: AsyncDropGlueCtorShim (_ , _) | ty :: InstanceKind :: FutureDropPollShim (_ , _ , _) = instance . def { & * instance . args } else if let ty :: InstanceKind :: AsyncDropGlue (_ , ty) = instance . def { let ty :: Coroutine (_ , cor_args) = ty . kind () else { bug ! () ; } ; let drop_ty = cor_args . first () . unwrap () . expect_ty () ; tcx . mk_args (& [GenericArg :: from (drop_ty)]) } else { & [] } ,) . unwrap () ; match instance . def { ty :: InstanceKind :: ThreadLocalShim (..) => { p . write_str ("{{tls-shim}}") . unwrap () ; } ty :: InstanceKind :: VTableShim (..) => { p . write_str ("{{vtable-shim}}") . unwrap () ; } ty :: InstanceKind :: ReifyShim (_ , reason) => { p . write_str ("{{reify-shim") . unwrap () ; match reason { Some (ReifyReason :: FnPtr) => p . write_str ("-fnptr") . unwrap () , Some (ReifyReason :: Vtable) => p . write_str ("-vtable") . unwrap () , None => () , } p . write_str ("}}") . unwrap () ; } ty :: InstanceKind :: ConstructCoroutineInClosureShim { receiver_by_ref , .. } => { p . write_str (if receiver_by_ref { "{{by-move-shim}}" } else { "{{by-ref-shim}}" }) . unwrap () ; } _ => { } } if let ty :: InstanceKind :: FutureDropPollShim (..) = instance . def { let _ = p . write_str ("{{drop-shim}}") ; } p . path . finish (hash) }
}

macro_rules! get_symbol_hash_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_symbol_hash in module {}", module_path!());
    };
}

mkfn!{
    get_symbol_hash_introspect!();
    fn get_symbol_hash < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , item_type : Ty < 'tcx > , instantiating_crate : Option < CrateNum > ,) -> Hash64 { let def_id = instance . def_id () ; let args = instance . args ; debug ! ("get_symbol_hash(def_id={:?}, parameters={:?})" , def_id , args) ; tcx . with_stable_hashing_context (| mut hcx | { let mut hasher = StableHasher :: new () ; tcx . def_path_hash (def_id) . hash_stable (& mut hcx , & mut hasher) ; assert ! (! item_type . has_erasable_regions ()) ; hcx . while_hashing_spans (false , | hcx | { item_type . hash_stable (hcx , & mut hasher) ; if let ty :: FnDef (..) = item_type . kind () { item_type . fn_sig (tcx) . hash_stable (hcx , & mut hasher) ; } args . hash_stable (hcx , & mut hasher) ; if let Some (instantiating_crate) = instantiating_crate { tcx . def_path_hash (instantiating_crate . as_def_id ()) . stable_crate_id () . hash_stable (hcx , & mut hasher) ; } discriminant (& instance . def) . hash_stable (hcx , & mut hasher) ; }) ; hasher . finish :: < Hash64 > () }) }
}
mkitem!{mkstruct!{#[derive (Debug)] struct SymbolPath { result : String , temp_buf : String , }}}
mkitem!{mkimpl!{impl SymbolPath { fn new () -> Self { let mut result = SymbolPath { result : String :: with_capacity (64) , temp_buf : String :: with_capacity (16) } ; result . result . push_str ("_ZN") ; result } fn finalize_pending_component (& mut self) { if ! self . temp_buf . is_empty () { let _ = write ! (self . result , "{}{}" , self . temp_buf . len () , self . temp_buf) ; self . temp_buf . clear () ; } } fn finish (mut self , hash : Hash64) -> String { self . finalize_pending_component () ; let _ = write ! (self . result , "17h{hash:016x}E") ; self . result } }}}
mkitem!{mkstruct!{struct LegacySymbolMangler < 'tcx > { tcx : TyCtxt < 'tcx > , path : SymbolPath , keep_within_component : bool , }}}
mkitem!{mkimpl!{impl < 'tcx > Printer < 'tcx > for LegacySymbolMangler < 'tcx > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn print_region (& mut self , _region : ty :: Region < '_ >) -> Result < () , PrintError > { Ok (()) } fn print_type (& mut self , ty : Ty < 'tcx >) -> Result < () , PrintError > { match * ty . kind () { ty :: FnDef (def_id , args) | ty :: Alias (ty :: Projection | ty :: Opaque , ty :: AliasTy { def_id , args , .. }) | ty :: Closure (def_id , args) | ty :: CoroutineClosure (def_id , args) | ty :: Coroutine (def_id , args) => self . print_def_path (def_id , args) , ty :: Array (ty , size) => { self . write_str ("[") ? ; self . print_type (ty) ? ; self . write_str ("; ") ? ; if let Some (size) = size . try_to_target_usize (self . tcx ()) { write ! (self , "{size}") ? } else if let ty :: ConstKind :: Param (param) = size . kind () { param . print (self) ? } else { self . write_str ("_") ? } self . write_str ("]") ? ; Ok (()) } ty :: Alias (ty :: Inherent , _) => panic ! ("unexpected inherent projection") , _ => self . pretty_print_type (ty) , } } fn print_dyn_existential (& mut self , predicates : & 'tcx ty :: List < ty :: PolyExistentialPredicate < 'tcx > > ,) -> Result < () , PrintError > { let mut first = true ; for p in predicates { if ! first { write ! (self , "+") ? ; } first = false ; p . print (self) ? ; } Ok (()) } fn print_const (& mut self , ct : ty :: Const < 'tcx >) -> Result < () , PrintError > { match ct . kind () { ty :: ConstKind :: Value (cv) if cv . ty . is_integral () => { let scalar = cv . valtree . unwrap_leaf () ; let signed = matches ! (cv . ty . kind () , ty :: Int (_)) ; write ! (self , "{:#?}" , ty :: ConstInt :: new (scalar , signed , cv . ty . is_ptr_sized_integral ())) ? ; } _ => self . write_str ("_") ? , } Ok (()) } fn print_crate_name (& mut self , cnum : CrateNum) -> Result < () , PrintError > { self . write_str (self . tcx . crate_name (cnum) . as_str ()) ? ; Ok (()) } fn print_path_with_qualified (& mut self , self_ty : Ty < 'tcx > , trait_ref : Option < ty :: TraitRef < 'tcx > > ,) -> Result < () , PrintError > { match self_ty . kind () { ty :: FnDef (..) | ty :: Alias (..) | ty :: Closure (..) | ty :: CoroutineClosure (..) | ty :: Coroutine (..) if trait_ref . is_none () => { self . print_type (self_ty) } _ => self . pretty_print_path_with_qualified (self_ty , trait_ref) , } } fn print_path_with_impl (& mut self , print_prefix : impl FnOnce (& mut Self) -> Result < () , PrintError > , self_ty : Ty < 'tcx > , trait_ref : Option < ty :: TraitRef < 'tcx > > ,) -> Result < () , PrintError > { self . pretty_print_path_with_impl (| cx | { print_prefix (cx) ? ; if cx . keep_within_component { cx . write_str ("::") ? ; } else { cx . path . finalize_pending_component () ; } Ok (()) } , self_ty , trait_ref ,) } fn print_path_with_simple (& mut self , print_prefix : impl FnOnce (& mut Self) -> Result < () , PrintError > , disambiguated_data : & DisambiguatedDefPathData ,) -> Result < () , PrintError > { print_prefix (self) ? ; if let DefPathData :: ForeignMod | DefPathData :: Ctor = disambiguated_data . data { return Ok (()) ; } if self . keep_within_component { self . write_str ("::") ? ; } else { self . path . finalize_pending_component () ; } write ! (self , "{}" , disambiguated_data . data) ? ; Ok (()) } fn print_path_with_generic_args (& mut self , print_prefix : impl FnOnce (& mut Self) -> Result < () , PrintError > , args : & [GenericArg < 'tcx >] ,) -> Result < () , PrintError > { print_prefix (self) ? ; let args = args . iter () . cloned () . filter (| arg | ! matches ! (arg . kind () , GenericArgKind :: Lifetime (_))) ; if args . clone () . next () . is_some () { self . generic_delimiters (| cx | cx . comma_sep (args)) } else { Ok (()) } } fn print_impl_path (& mut self , impl_def_id : DefId , args : & 'tcx [GenericArg < 'tcx >] ,) -> Result < () , PrintError > { let self_ty = self . tcx . type_of (impl_def_id) ; let impl_trait_ref = self . tcx . impl_trait_ref (impl_def_id) ; let generics = self . tcx . generics_of (impl_def_id) ; let (typing_env , mut self_ty , mut impl_trait_ref) = if generics . count () > args . len () || & args [.. generics . count ()] == self . tcx . erase_and_anonymize_regions (ty :: GenericArgs :: identity_for_item (self . tcx , impl_def_id ,)) . as_slice () { (ty :: TypingEnv :: post_analysis (self . tcx , impl_def_id) , self_ty . instantiate_identity () , impl_trait_ref . map (| impl_trait_ref | impl_trait_ref . instantiate_identity ()) ,) } else { assert ! (! args . has_non_region_param () , "should not be mangling partially substituted \
                polymorphic instance: {impl_def_id:?} {args:?}") ; (ty :: TypingEnv :: fully_monomorphized () , self_ty . instantiate (self . tcx , args) , impl_trait_ref . map (| impl_trait_ref | impl_trait_ref . instantiate (self . tcx , args)) ,) } ; match & mut impl_trait_ref { Some (impl_trait_ref) => { assert_eq ! (impl_trait_ref . self_ty () , self_ty) ; * impl_trait_ref = self . tcx . normalize_erasing_regions (typing_env , * impl_trait_ref) ; self_ty = impl_trait_ref . self_ty () ; } None => { self_ty = self . tcx . normalize_erasing_regions (typing_env , self_ty) ; } } self . default_print_impl_path (impl_def_id , self_ty , impl_trait_ref) } }}}
mkitem!{mkimpl!{impl < 'tcx > PrettyPrinter < 'tcx > for LegacySymbolMangler < 'tcx > { fn should_print_optional_region (& self , _region : ty :: Region < '_ >) -> bool { false } fn comma_sep < T > (& mut self , mut elems : impl Iterator < Item = T >) -> Result < () , PrintError > where T : Print < 'tcx , Self > , { if let Some (first) = elems . next () { first . print (self) ? ; for elem in elems { self . write_str (",") ? ; elem . print (self) ? ; } } Ok (()) } fn generic_delimiters (& mut self , f : impl FnOnce (& mut Self) -> Result < () , PrintError > ,) -> Result < () , PrintError > { write ! (self , "<") ? ; let kept_within_component = mem :: replace (& mut self . keep_within_component , true) ; f (self) ? ; self . keep_within_component = kept_within_component ; write ! (self , ">") ? ; Ok (()) } }}}
mkitem!{mkimpl!{impl fmt :: Write for LegacySymbolMangler < '_ > { fn write_str (& mut self , s : & str) -> fmt :: Result { for c in s . chars () { if self . path . temp_buf . is_empty () { match c { 'a' ..= 'z' | 'A' ..= 'Z' | '_' => { } _ => { self . path . temp_buf . push ('_') ; } } } match c { '@' => self . path . temp_buf . push_str ("$SP$") , '*' => self . path . temp_buf . push_str ("$BP$") , '&' => self . path . temp_buf . push_str ("$RF$") , '<' => self . path . temp_buf . push_str ("$LT$") , '>' => self . path . temp_buf . push_str ("$GT$") , '(' => self . path . temp_buf . push_str ("$LP$") , ')' => self . path . temp_buf . push_str ("$RP$") , ',' => self . path . temp_buf . push_str ("$C$") , '-' | ':' | '.' if self . tcx . has_strict_asm_symbol_naming () => { self . path . temp_buf . push ('$') } '-' | ':' => self . path . temp_buf . push ('.') , 'm' if self . path . temp_buf . ends_with (".llv") => self . path . temp_buf . push_str ("$u6d$") , 'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' | '.' | '$' => self . path . temp_buf . push (c) , _ => { self . path . temp_buf . push ('$') ; for c in c . escape_unicode () . skip (1) { match c { '{' => { } '}' => self . path . temp_buf . push ('$') , c => self . path . temp_buf . push (c) , } } } } } Ok (()) } }}}