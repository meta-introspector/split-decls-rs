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
mkuse!{use std :: fmt :: Write ;}
mkuse!{use std :: hash :: Hasher ;}
mkuse!{use std :: iter ;}
mkuse!{use std :: ops :: Range ;}
mkuse!{use rustc_abi :: { ExternAbi , Integer } ;}
mkuse!{use rustc_data_structures :: base_n :: ToBaseN ;}
mkuse!{use rustc_data_structures :: fx :: FxHashMap ;}
mkuse!{use rustc_data_structures :: intern :: Interned ;}
mkuse!{use rustc_data_structures :: stable_hasher :: StableHasher ;}
mkuse!{use rustc_hashes :: Hash64 ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def :: CtorKind ;}
mkuse!{use rustc_hir :: def_id :: { CrateNum , DefId } ;}
mkuse!{use rustc_hir :: definitions :: { DefPathData , DisambiguatedDefPathData } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: layout :: IntegerExt ;}
mkuse!{use rustc_middle :: ty :: print :: { Print , PrintError , Printer } ;}
mkuse!{use rustc_middle :: ty :: { self , FloatTy , GenericArg , GenericArgKind , Instance , IntTy , ReifyReason , Ty , TyCtxt , TypeVisitable , TypeVisitableExt , UintTy , } ;}
mkuse!{use rustc_span :: sym ;}

macro_rules! mangle_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mangle in module {}", module_path!());
    };
}

mkfn!{
    mangle_introspect!();
    pub (super) fn mangle < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , instantiating_crate : Option < CrateNum > , is_exportable : bool ,) -> String { let def_id = instance . def_id () ; let args = tcx . normalize_erasing_regions (ty :: TypingEnv :: fully_monomorphized () , instance . args) ; let prefix = "_R" ; let mut p : V0SymbolMangler < '_ > = V0SymbolMangler { tcx , start_offset : prefix . len () , is_exportable , paths : FxHashMap :: default () , types : FxHashMap :: default () , consts : FxHashMap :: default () , binders : vec ! [] , out : String :: from (prefix) , } ; let shim_kind = match instance . def { ty :: InstanceKind :: ThreadLocalShim (_) => Some ("tls") , ty :: InstanceKind :: VTableShim (_) => Some ("vtable") , ty :: InstanceKind :: ReifyShim (_ , None) => Some ("reify") , ty :: InstanceKind :: ReifyShim (_ , Some (ReifyReason :: FnPtr)) => Some ("reify_fnptr") , ty :: InstanceKind :: ReifyShim (_ , Some (ReifyReason :: Vtable)) => Some ("reify_vtable") , ty :: InstanceKind :: ConstructCoroutineInClosureShim { receiver_by_ref : true , .. } => { Some ("by_move") } ty :: InstanceKind :: ConstructCoroutineInClosureShim { receiver_by_ref : false , .. } => { Some ("by_ref") } ty :: InstanceKind :: FutureDropPollShim (_ , _ , _) => Some ("drop") , _ => None , } ; if let ty :: InstanceKind :: AsyncDropGlue (_ , ty) = instance . def { let ty :: Coroutine (_ , cor_args) = ty . kind () else { bug ! () ; } ; let drop_ty = cor_args . first () . unwrap () . expect_ty () ; p . print_def_path (def_id , tcx . mk_args (& [GenericArg :: from (drop_ty)])) . unwrap () } else if let Some (shim_kind) = shim_kind { p . path_append_ns (| p | p . print_def_path (def_id , args) , 'S' , 0 , shim_kind) . unwrap () } else { p . print_def_path (def_id , args) . unwrap () } ; if let Some (instantiating_crate) = instantiating_crate { p . print_def_path (instantiating_crate . as_def_id () , & []) . unwrap () ; } std :: mem :: take (& mut p . out) }
}

macro_rules! mangle_internal_symbol_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mangle_internal_symbol in module {}", module_path!());
    };
}

mkfn!{
    mangle_internal_symbol_introspect!();
    pub fn mangle_internal_symbol < 'tcx > (tcx : TyCtxt < 'tcx > , item_name : & str) -> String { match item_name { "rust_eh_personality" => return item_name . to_owned () , "__isPlatformVersionAtLeast" | "__isOSVersionAtLeast" => return item_name . to_owned () , _ => { } } let prefix = "_R" ; let mut p : V0SymbolMangler < '_ > = V0SymbolMangler { tcx , start_offset : prefix . len () , is_exportable : false , paths : FxHashMap :: default () , types : FxHashMap :: default () , consts : FxHashMap :: default () , binders : vec ! [] , out : String :: from (prefix) , } ; p . path_append_ns (| p | { p . push ("C") ; p . push_disambiguator ({ let mut hasher = StableHasher :: new () ; hasher . write (tcx . sess . cfg_version . as_bytes ()) ; let hash : Hash64 = hasher . finish () ; hash . as_u64 () }) ; p . push_ident ("__rustc") ; Ok (()) } , 'v' , 0 , item_name ,) . unwrap () ; std :: mem :: take (& mut p . out) }
}

macro_rules! mangle_typeid_for_trait_ref_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mangle_typeid_for_trait_ref in module {}", module_path!());
    };
}

mkfn!{
    mangle_typeid_for_trait_ref_introspect!();
    pub (super) fn mangle_typeid_for_trait_ref < 'tcx > (tcx : TyCtxt < 'tcx > , trait_ref : ty :: ExistentialTraitRef < 'tcx > ,) -> String { let mut p = V0SymbolMangler { tcx , start_offset : 0 , is_exportable : false , paths : FxHashMap :: default () , types : FxHashMap :: default () , consts : FxHashMap :: default () , binders : vec ! [] , out : String :: new () , } ; p . print_def_path (trait_ref . def_id , & []) . unwrap () ; std :: mem :: take (& mut p . out) }
}
mkitem!{mkstruct!{struct BinderLevel { #[doc = " The range of distances from the root of what's"] #[doc = " being printed, to the lifetimes in a binder."] #[doc = " Specifically, a `BrAnon` lifetime has depth"] #[doc = " `lifetime_depths.start + index`, going away from the"] #[doc = " the root and towards its use site, as the var index increases."] #[doc = " This is used to flatten rustc's pairing of `BrAnon`"] #[doc = " (intra-binder disambiguation) with a `DebruijnIndex`"] #[doc = " (binder addressing), to \"true\" de Bruijn indices,"] #[doc = " by subtracting the depth of a certain lifetime, from"] #[doc = " the innermost depth at its use site."] lifetime_depths : Range < u32 > , }}}
mkitem!{mkstruct!{struct V0SymbolMangler < 'tcx > { tcx : TyCtxt < 'tcx > , binders : Vec < BinderLevel > , out : String , is_exportable : bool , #[doc = " The length of the prefix in `out` (e.g. 2 for `_R`)."] start_offset : usize , #[doc = " The values are start positions in `out`, in bytes."] paths : FxHashMap < (DefId , & 'tcx [GenericArg < 'tcx >]) , usize > , types : FxHashMap < Ty < 'tcx > , usize > , consts : FxHashMap < ty :: Const < 'tcx > , usize > , }}}
mkitem!{mkimpl!{impl < 'tcx > V0SymbolMangler < 'tcx > { fn push (& mut self , s : & str) { self . out . push_str (s) ; } #[doc = " Push a `_`-terminated base 62 integer, using the format"] #[doc = " specified in the RFC as `<base-62-number>`, that is:"] #[doc = " * `x = 0` is encoded as just the `\"_\"` terminator"] #[doc = " * `x > 0` is encoded as `x - 1` in base 62, followed by `\"_\"`,"] #[doc = "   e.g. `1` becomes `\"0_\"`, `62` becomes `\"Z_\"`, etc."] fn push_integer_62 (& mut self , x : u64) { push_integer_62 (x , & mut self . out) } #[doc = " Push a `tag`-prefixed base 62 integer, when larger than `0`, that is:"] #[doc = " * `x = 0` is encoded as `\"\"` (nothing)"] #[doc = " * `x > 0` is encoded as the `tag` followed by `push_integer_62(x - 1)`"] #[doc = "   e.g. `1` becomes `tag + \"_\"`, `2` becomes `tag + \"0_\"`, etc."] fn push_opt_integer_62 (& mut self , tag : & str , x : u64) { if let Some (x) = x . checked_sub (1) { self . push (tag) ; self . push_integer_62 (x) ; } } fn push_disambiguator (& mut self , dis : u64) { self . push_opt_integer_62 ("s" , dis) ; } fn push_ident (& mut self , ident : & str) { push_ident (ident , & mut self . out) } fn path_append_ns (& mut self , print_prefix : impl FnOnce (& mut Self) -> Result < () , PrintError > , ns : char , disambiguator : u64 , name : & str ,) -> Result < () , PrintError > { self . push ("N") ; self . out . push (ns) ; print_prefix (self) ? ; self . push_disambiguator (disambiguator) ; self . push_ident (name) ; Ok (()) } fn print_backref (& mut self , i : usize) -> Result < () , PrintError > { self . push ("B") ; self . push_integer_62 ((i - self . start_offset) as u64) ; Ok (()) } fn wrap_binder < T > (& mut self , value : & ty :: Binder < 'tcx , T > , print_value : impl FnOnce (& mut Self , & T) -> Result < () , PrintError > ,) -> Result < () , PrintError > where T : TypeVisitable < TyCtxt < 'tcx > > , { let mut lifetime_depths = self . binders . last () . map (| b | b . lifetime_depths . end) . map_or (0 .. 0 , | i | i .. i) ; let lifetimes = value . bound_vars () . iter () . filter (| var | matches ! (var , ty :: BoundVariableKind :: Region (..))) . count () as u32 ; self . push_opt_integer_62 ("G" , lifetimes as u64) ; lifetime_depths . end += lifetimes ; self . binders . push (BinderLevel { lifetime_depths }) ; print_value (self , value . as_ref () . skip_binder ()) ? ; self . binders . pop () ; Ok (()) } fn print_pat (& mut self , pat : ty :: Pattern < 'tcx >) -> Result < () , std :: fmt :: Error > { Ok (match * pat { ty :: PatternKind :: Range { start , end } => { let consts = [start , end] ; for ct in consts { Ty :: new_array_with_const_len (self . tcx , self . tcx . types . unit , ct) . print (self) ? ; } } ty :: PatternKind :: Or (patterns) => { for pat in patterns { self . print_pat (pat) ? ; } } }) } }}}
mkitem!{mkimpl!{impl < 'tcx > Printer < 'tcx > for V0SymbolMangler < 'tcx > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn print_def_path (& mut self , def_id : DefId , args : & 'tcx [GenericArg < 'tcx >] ,) -> Result < () , PrintError > { if let Some (& i) = self . paths . get (& (def_id , args)) { return self . print_backref (i) ; } let start = self . out . len () ; self . default_print_def_path (def_id , args) ? ; if ! args . iter () . any (| k | k . has_escaping_bound_vars ()) { self . paths . insert ((def_id , args) , start) ; } Ok (()) } fn print_impl_path (& mut self , impl_def_id : DefId , args : & 'tcx [GenericArg < 'tcx >] ,) -> Result < () , PrintError > { let key = self . tcx . def_key (impl_def_id) ; let parent_def_id = DefId { index : key . parent . unwrap () , .. impl_def_id } ; let self_ty = self . tcx . type_of (impl_def_id) ; let impl_trait_ref = self . tcx . impl_trait_ref (impl_def_id) ; let generics = self . tcx . generics_of (impl_def_id) ; let (typing_env , mut self_ty , mut impl_trait_ref) = if generics . count () > args . len () || & args [.. generics . count ()] == self . tcx . erase_and_anonymize_regions (ty :: GenericArgs :: identity_for_item (self . tcx , impl_def_id ,)) . as_slice () { (ty :: TypingEnv :: post_analysis (self . tcx , impl_def_id) , self_ty . instantiate_identity () , impl_trait_ref . map (| impl_trait_ref | impl_trait_ref . instantiate_identity ()) ,) } else { assert ! (! args . has_non_region_param () && ! args . has_free_regions () , "should not be mangling partially substituted \
                polymorphic instance: {impl_def_id:?} {args:?}") ; (ty :: TypingEnv :: fully_monomorphized () , self_ty . instantiate (self . tcx , args) , impl_trait_ref . map (| impl_trait_ref | impl_trait_ref . instantiate (self . tcx , args)) ,) } ; match & mut impl_trait_ref { Some (impl_trait_ref) => { assert_eq ! (impl_trait_ref . self_ty () , self_ty) ; * impl_trait_ref = self . tcx . normalize_erasing_regions (typing_env , * impl_trait_ref) ; self_ty = impl_trait_ref . self_ty () ; } None => { self_ty = self . tcx . normalize_erasing_regions (typing_env , self_ty) ; } } self . push (match impl_trait_ref { Some (_) => "X" , None => "M" , }) ; if impl_trait_ref . is_some () && args . iter () . any (| a | a . has_non_region_param ()) { self . print_path_with_generic_args (| this | { this . path_append_ns (| p | p . print_def_path (parent_def_id , & []) , 'I' , key . disambiguated_data . disambiguator as u64 , "" ,) } , args ,) ? ; } else { let exported_impl_order = self . tcx . stable_order_of_exportable_impls (impl_def_id . krate) ; let disambiguator = match self . is_exportable { true => exported_impl_order [& impl_def_id] as u64 , false => { exported_impl_order . len () as u64 + key . disambiguated_data . disambiguator as u64 } } ; self . push_disambiguator (disambiguator) ; self . print_def_path (parent_def_id , & []) ? ; } self_ty . print (self) ? ; if let Some (trait_ref) = impl_trait_ref { self . print_def_path (trait_ref . def_id , trait_ref . args) ? ; } Ok (()) } fn print_region (& mut self , region : ty :: Region < '_ >) -> Result < () , PrintError > { let i = match region . kind () { ty :: ReErased => 0 , ty :: ReBound (debruijn , ty :: BoundRegion { var , kind : ty :: BoundRegionKind :: Anon }) => { let binder = & self . binders [self . binders . len () - 1 - debruijn . index ()] ; let depth = binder . lifetime_depths . start + var . as_u32 () ; 1 + (self . binders . last () . unwrap () . lifetime_depths . end - 1 - depth) } _ => bug ! ("symbol_names: non-erased region `{:?}`" , region) , } ; self . push ("L") ; self . push_integer_62 (i as u64) ; Ok (()) } fn print_type (& mut self , ty : Ty < 'tcx >) -> Result < () , PrintError > { let basic_type = match ty . kind () { ty :: Bool => "b" , ty :: Char => "c" , ty :: Str => "e" , ty :: Int (IntTy :: I8) => "a" , ty :: Int (IntTy :: I16) => "s" , ty :: Int (IntTy :: I32) => "l" , ty :: Int (IntTy :: I64) => "x" , ty :: Int (IntTy :: I128) => "n" , ty :: Int (IntTy :: Isize) => "i" , ty :: Uint (UintTy :: U8) => "h" , ty :: Uint (UintTy :: U16) => "t" , ty :: Uint (UintTy :: U32) => "m" , ty :: Uint (UintTy :: U64) => "y" , ty :: Uint (UintTy :: U128) => "o" , ty :: Uint (UintTy :: Usize) => "j" , ty :: Float (FloatTy :: F16) => "C3f16" , ty :: Float (FloatTy :: F32) => "f" , ty :: Float (FloatTy :: F64) => "d" , ty :: Float (FloatTy :: F128) => "C4f128" , ty :: Never => "z" , ty :: Tuple (_) if ty . is_unit () => "u" , ty :: Param (_) => "p" , _ => "" , } ; if ! basic_type . is_empty () { self . push (basic_type) ; return Ok (()) ; } if let Some (& i) = self . types . get (& ty) { return self . print_backref (i) ; } let start = self . out . len () ; match * ty . kind () { ty :: Bool | ty :: Char | ty :: Str | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Never => { unreachable ! () } ty :: Tuple (_) if ty . is_unit () => unreachable ! () , ty :: Param (_) => unreachable ! () , ty :: Bound (..) | ty :: Placeholder (_) | ty :: Infer (_) | ty :: Error (_) => bug ! () , ty :: Ref (r , ty , mutbl) => { self . push (match mutbl { hir :: Mutability :: Not => "R" , hir :: Mutability :: Mut => "Q" , }) ; if ! r . is_erased () { r . print (self) ? ; } ty . print (self) ? ; } ty :: RawPtr (ty , mutbl) => { self . push (match mutbl { hir :: Mutability :: Not => "P" , hir :: Mutability :: Mut => "O" , }) ; ty . print (self) ? ; } ty :: Pat (ty , pat) => { self . push ("T") ; ty . print (self) ? ; self . print_pat (pat) ? ; self . push ("E") ; } ty :: Array (ty , len) => { self . push ("A") ; ty . print (self) ? ; self . print_const (len) ? ; } ty :: Slice (ty) => { self . push ("S") ; ty . print (self) ? ; } ty :: Tuple (tys) => { self . push ("T") ; for ty in tys . iter () { ty . print (self) ? ; } self . push ("E") ; } ty :: Adt (ty :: AdtDef (Interned (& ty :: AdtDefData { did : def_id , .. } , _)) , args) | ty :: FnDef (def_id , args) | ty :: Closure (def_id , args) | ty :: CoroutineClosure (def_id , args) | ty :: Coroutine (def_id , args) => { self . print_def_path (def_id , args) ? ; } ty :: Alias (ty :: Projection , ty :: AliasTy { def_id , args , .. }) => { self . print_def_path (def_id , args) ? ; } ty :: Foreign (def_id) => { self . print_def_path (def_id , & []) ? ; } ty :: FnPtr (sig_tys , hdr) => { let sig = sig_tys . with (hdr) ; self . push ("F") ; self . wrap_binder (& sig , | p , sig | { if sig . safety . is_unsafe () { p . push ("U") ; } match sig . abi { ExternAbi :: Rust => { } ExternAbi :: C { unwind : false } => p . push ("KC") , abi => { p . push ("K") ; let name = abi . as_str () ; if name . contains ('-') { p . push_ident (& name . replace ('-' , "_")) ; } else { p . push_ident (name) ; } } } for & ty in sig . inputs () { ty . print (p) ? ; } if sig . c_variadic { p . push ("v") ; } p . push ("E") ; sig . output () . print (p) }) ? ; } ty :: UnsafeBinder (..) => todo ! () , ty :: Dynamic (predicates , r , kind) => { self . push (match kind { ty :: Dyn => "D" , }) ; self . print_dyn_existential (predicates) ? ; r . print (self) ? ; } ty :: Alias (..) => bug ! ("symbol_names: unexpected alias") , ty :: CoroutineWitness (..) => bug ! ("symbol_names: unexpected `CoroutineWitness`") , } if ! ty . has_escaping_bound_vars () { self . types . insert (ty , start) ; } Ok (()) } fn print_dyn_existential (& mut self , predicates : & 'tcx ty :: List < ty :: PolyExistentialPredicate < 'tcx > > ,) -> Result < () , PrintError > { self . wrap_binder (& predicates [0] , | p , _ | { for predicate in predicates . iter () { match predicate . as_ref () . skip_binder () { ty :: ExistentialPredicate :: Trait (trait_ref) => { let dummy_self = Ty :: new_fresh (p . tcx , 0) ; let trait_ref = trait_ref . with_self_ty (p . tcx , dummy_self) ; p . print_def_path (trait_ref . def_id , trait_ref . args) ? ; } ty :: ExistentialPredicate :: Projection (projection) => { let name = p . tcx . associated_item (projection . def_id) . name () ; p . push ("p") ; p . push_ident (name . as_str ()) ; match projection . term . kind () { ty :: TermKind :: Ty (ty) => ty . print (p) , ty :: TermKind :: Const (c) => c . print (p) , } ? ; } ty :: ExistentialPredicate :: AutoTrait (def_id) => { p . print_def_path (* def_id , & []) ? ; } } } Ok (()) }) ? ; self . push ("E") ; Ok (()) } fn print_const (& mut self , ct : ty :: Const < 'tcx >) -> Result < () , PrintError > { let cv = match ct . kind () { ty :: ConstKind :: Value (cv) => cv , ty :: ConstKind :: Param (_) => { self . push ("p") ; return Ok (()) ; } ty :: ConstKind :: Unevaluated (ty :: UnevaluatedConst { def , args , .. }) => { return self . print_def_path (def , args) ; } ty :: ConstKind :: Expr (_) | ty :: ConstKind :: Infer (_) | ty :: ConstKind :: Bound (..) | ty :: ConstKind :: Placeholder (_) | ty :: ConstKind :: Error (_) => bug ! () , } ; if let Some (& i) = self . consts . get (& ct) { self . print_backref (i) ? ; return Ok (()) ; } let ty :: Value { ty : ct_ty , valtree } = cv ; let start = self . out . len () ; match ct_ty . kind () { ty :: Uint (_) | ty :: Int (_) | ty :: Bool | ty :: Char => { ct_ty . print (self) ? ; let mut bits = cv . try_to_bits (self . tcx , ty :: TypingEnv :: fully_monomorphized ()) . expect ("expected const to be monomorphic") ; if let ty :: Int (ity) = ct_ty . kind () { let val = Integer :: from_int_ty (& self . tcx , * ity) . size () . sign_extend (bits) as i128 ; if val < 0 { self . push ("n") ; } bits = val . unsigned_abs () ; } let _ = write ! (self . out , "{bits:x}_") ; } ty :: Str => { let tcx = self . tcx () ; let ref_ty = Ty :: new_imm_ref (tcx , tcx . lifetimes . re_static , ct_ty) ; let cv = ty :: Value { ty : ref_ty , valtree } ; let slice = cv . try_to_raw_bytes (tcx) . unwrap_or_else (| | { bug ! ("expected to get raw bytes from valtree {:?} for type {:}" , valtree , ct_ty) }) ; let s = std :: str :: from_utf8 (slice) . expect ("non utf8 str from MIR interpreter") ; self . push ("e") ; for byte in s . bytes () { let _ = write ! (self . out , "{byte:02x}") ; } self . push ("_") ; } ty :: Ref (_ , _ , mutbl) => { self . push (match mutbl { hir :: Mutability :: Not => "R" , hir :: Mutability :: Mut => "Q" , }) ; let pointee_ty = ct_ty . builtin_deref (true) . expect ("tried to dereference on non-ptr type") ; let dereferenced_const = ty :: Const :: new_value (self . tcx , valtree , pointee_ty) ; dereferenced_const . print (self) ? ; } ty :: Array (..) | ty :: Tuple (..) | ty :: Adt (..) | ty :: Slice (_) => { let contents = self . tcx . destructure_const (ct) ; let fields = contents . fields . iter () . copied () ; let print_field_list = | this : & mut Self | { for field in fields . clone () { field . print (this) ? ; } this . push ("E") ; Ok (()) } ; match * ct_ty . kind () { ty :: Array (..) | ty :: Slice (_) => { self . push ("A") ; print_field_list (self) ? ; } ty :: Tuple (..) => { self . push ("T") ; print_field_list (self) ? ; } ty :: Adt (def , args) => { let variant_idx = contents . variant . expect ("destructed const of adt without variant idx") ; let variant_def = & def . variant (variant_idx) ; self . push ("V") ; self . print_def_path (variant_def . def_id , args) ? ; match variant_def . ctor_kind () { Some (CtorKind :: Const) => { self . push ("U") ; } Some (CtorKind :: Fn) => { self . push ("T") ; print_field_list (self) ? ; } None => { self . push ("S") ; for (field_def , field) in iter :: zip (& variant_def . fields , fields) { let disambiguated_field = self . tcx . def_key (field_def . did) . disambiguated_data ; let field_name = disambiguated_field . data . get_opt_name () ; self . push_disambiguator (disambiguated_field . disambiguator as u64 ,) ; self . push_ident (field_name . unwrap () . as_str ()) ; field . print (self) ? ; } self . push ("E") ; } } } _ => unreachable ! () , } } _ => { bug ! ("symbol_names: unsupported constant of type `{}` ({:?})" , ct_ty , ct) ; } } if ! ct . has_escaping_bound_vars () { self . consts . insert (ct , start) ; } Ok (()) } fn print_crate_name (& mut self , cnum : CrateNum) -> Result < () , PrintError > { self . push ("C") ; if ! self . is_exportable { let stable_crate_id = self . tcx . def_path_hash (cnum . as_def_id ()) . stable_crate_id () ; self . push_disambiguator (stable_crate_id . as_u64 ()) ; } let name = self . tcx . crate_name (cnum) ; self . push_ident (name . as_str ()) ; Ok (()) } fn print_path_with_qualified (& mut self , self_ty : Ty < 'tcx > , trait_ref : Option < ty :: TraitRef < 'tcx > > ,) -> Result < () , PrintError > { assert ! (trait_ref . is_some ()) ; let trait_ref = trait_ref . unwrap () ; self . push ("Y") ; self_ty . print (self) ? ; self . print_def_path (trait_ref . def_id , trait_ref . args) } fn print_path_with_impl (& mut self , _ : impl FnOnce (& mut Self) -> Result < () , PrintError > , _ : Ty < 'tcx > , _ : Option < ty :: TraitRef < 'tcx > > ,) -> Result < () , PrintError > { unreachable ! () } fn print_path_with_simple (& mut self , print_prefix : impl FnOnce (& mut Self) -> Result < () , PrintError > , disambiguated_data : & DisambiguatedDefPathData ,) -> Result < () , PrintError > { let ns = match disambiguated_data . data { DefPathData :: ForeignMod => return print_prefix (self) , DefPathData :: TypeNs (_) => 't' , DefPathData :: ValueNs (_) => 'v' , DefPathData :: Closure => 'C' , DefPathData :: Ctor => 'c' , DefPathData :: AnonConst => 'k' , DefPathData :: OpaqueTy => 'i' , DefPathData :: SyntheticCoroutineBody => 's' , DefPathData :: NestedStatic => 'n' , DefPathData :: CrateRoot | DefPathData :: Use | DefPathData :: GlobalAsm | DefPathData :: Impl | DefPathData :: MacroNs (_) | DefPathData :: LifetimeNs (_) | DefPathData :: OpaqueLifetime (_) | DefPathData :: AnonAssocTy (..) => { bug ! ("symbol_names: unexpected DefPathData: {:?}" , disambiguated_data . data) } } ; let name = disambiguated_data . data . get_opt_name () ; self . path_append_ns (print_prefix , ns , disambiguated_data . disambiguator as u64 , name . unwrap_or (sym :: empty) . as_str () ,) } fn print_path_with_generic_args (& mut self , print_prefix : impl FnOnce (& mut Self) -> Result < () , PrintError > , args : & [GenericArg < 'tcx >] ,) -> Result < () , PrintError > { let print_regions = args . iter () . any (| arg | match arg . kind () { GenericArgKind :: Lifetime (r) => ! r . is_erased () , _ => false , }) ; let args = args . iter () . cloned () . filter (| arg | match arg . kind () { GenericArgKind :: Lifetime (_) => print_regions , _ => true , }) ; if args . clone () . next () . is_none () { return print_prefix (self) ; } self . push ("I") ; print_prefix (self) ? ; for arg in args { match arg . kind () { GenericArgKind :: Lifetime (lt) => { lt . print (self) ? ; } GenericArgKind :: Type (ty) => { ty . print (self) ? ; } GenericArgKind :: Const (c) => { self . push ("K") ; c . print (self) ? ; } } } self . push ("E") ; Ok (()) } }}}

macro_rules! push_integer_62_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function push_integer_62 in module {}", module_path!());
    };
}

mkfn!{
    push_integer_62_introspect!();
    #[doc = " Push a `_`-terminated base 62 integer, using the format"] #[doc = " specified in the RFC as `<base-62-number>`, that is:"] #[doc = " * `x = 0` is encoded as just the `\"_\"` terminator"] #[doc = " * `x > 0` is encoded as `x - 1` in base 62, followed by `\"_\"`,"] #[doc = "   e.g. `1` becomes `\"0_\"`, `62` becomes `\"Z_\"`, etc."] pub (crate) fn push_integer_62 (x : u64 , output : & mut String) { if let Some (x) = x . checked_sub (1) { output . push_str (& x . to_base (62)) ; } output . push ('_') ; }
}

macro_rules! encode_integer_62_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function encode_integer_62 in module {}", module_path!());
    };
}

mkfn!{
    encode_integer_62_introspect!();
    pub (crate) fn encode_integer_62 (x : u64) -> String { let mut output = String :: new () ; push_integer_62 (x , & mut output) ; output }
}

macro_rules! push_ident_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function push_ident in module {}", module_path!());
    };
}

mkfn!{
    push_ident_introspect!();
    pub (crate) fn push_ident (ident : & str , output : & mut String) { let mut use_punycode = false ; for b in ident . bytes () { match b { b'_' | b'a' ..= b'z' | b'A' ..= b'Z' | b'0' ..= b'9' => { } 0x80 ..= 0xff => use_punycode = true , _ => bug ! ("symbol_names: bad byte {} in ident {:?}" , b , ident) , } } let punycode_string ; let ident = if use_punycode { output . push ('u') ; let mut punycode_bytes = match punycode :: encode (ident) { Ok (s) => s . into_bytes () , Err (()) => bug ! ("symbol_names: punycode encoding failed for ident {:?}" , ident) , } ; if let Some (c) = punycode_bytes . iter_mut () . rfind (| & & mut c | c == b'-') { * c = b'_' ; } punycode_string = String :: from_utf8 (punycode_bytes) . unwrap () ; & punycode_string } else { ident } ; let _ = write ! (output , "{}" , ident . len ()) ; if let Some ('_' | '0' ..= '9') = ident . chars () . next () { output . push ('_') ; } output . push_str (ident) ; }
}