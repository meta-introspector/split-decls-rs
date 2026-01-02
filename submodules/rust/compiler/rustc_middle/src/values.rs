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
mkuse!{use std :: collections :: VecDeque ;}
mkuse!{use std :: fmt :: Write ;}
mkuse!{use std :: ops :: ControlFlow ;}
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_errors :: { Applicability , MultiSpan , pluralize , struct_span_code_err } ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def :: { DefKind , Res } ;}
mkuse!{use rustc_query_system :: Value ;}
mkuse!{use rustc_query_system :: query :: { CycleError , report_cycle } ;}
mkuse!{use rustc_span :: def_id :: LocalDefId ;}
mkuse!{use rustc_span :: { ErrorGuaranteed , Span } ;}
mkuse!{use crate :: dep_graph :: dep_kinds ;}
mkuse!{use crate :: query :: plumbing :: CyclePlaceholder ;}
mkuse!{use crate :: ty :: { self , Representability , Ty , TyCtxt } ;}
mkitem!{mkimpl!{impl < 'tcx > Value < TyCtxt < 'tcx > > for Ty < '_ > { fn from_cycle_error (tcx : TyCtxt < 'tcx > , _ : & CycleError , guar : ErrorGuaranteed) -> Self { unsafe { std :: mem :: transmute :: < Ty < 'tcx > , Ty < '_ > > (Ty :: new_error (tcx , guar)) } } }}}
mkitem!{mkimpl!{impl < 'tcx > Value < TyCtxt < 'tcx > > for Result < ty :: EarlyBinder < '_ , Ty < '_ > > , CyclePlaceholder > { fn from_cycle_error (_tcx : TyCtxt < 'tcx > , _ : & CycleError , guar : ErrorGuaranteed) -> Self { Err (CyclePlaceholder (guar)) } }}}
mkitem!{mkimpl!{impl < 'tcx > Value < TyCtxt < 'tcx > > for ty :: SymbolName < '_ > { fn from_cycle_error (tcx : TyCtxt < 'tcx > , _ : & CycleError , _guar : ErrorGuaranteed) -> Self { unsafe { std :: mem :: transmute :: < ty :: SymbolName < 'tcx > , ty :: SymbolName < '_ > > (ty :: SymbolName :: new (tcx , "<error>" ,)) } } }}}
mkitem!{mkimpl!{impl < 'tcx > Value < TyCtxt < 'tcx > > for ty :: Binder < '_ , ty :: FnSig < '_ > > { fn from_cycle_error (tcx : TyCtxt < 'tcx > , cycle_error : & CycleError , guar : ErrorGuaranteed ,) -> Self { let err = Ty :: new_error (tcx , guar) ; let arity = if let Some (frame) = cycle_error . cycle . get (0) && frame . query . dep_kind == dep_kinds :: fn_sig && let Some (def_id) = frame . query . def_id && let Some (node) = tcx . hir_get_if_local (def_id) && let Some (sig) = node . fn_sig () { sig . decl . inputs . len () } else { tcx . dcx () . abort_if_errors () ; unreachable ! () } ; let fn_sig = ty :: Binder :: dummy (tcx . mk_fn_sig (std :: iter :: repeat (err) . take (arity) , err , false , rustc_hir :: Safety :: Safe , rustc_abi :: ExternAbi :: Rust ,)) ; unsafe { std :: mem :: transmute :: < ty :: PolyFnSig < 'tcx > , ty :: Binder < '_ , ty :: FnSig < '_ > > > (fn_sig) } } }}}
mkitem!{mkimpl!{impl < 'tcx > Value < TyCtxt < 'tcx > > for Representability { fn from_cycle_error (tcx : TyCtxt < 'tcx > , cycle_error : & CycleError , _guar : ErrorGuaranteed ,) -> Self { let mut item_and_field_ids = Vec :: new () ; let mut representable_ids = FxHashSet :: default () ; for info in & cycle_error . cycle { if info . query . dep_kind == dep_kinds :: representability && let Some (field_id) = info . query . def_id && let Some (field_id) = field_id . as_local () && let Some (DefKind :: Field) = info . query . info . def_kind { let parent_id = tcx . parent (field_id . to_def_id ()) ; let item_id = match tcx . def_kind (parent_id) { DefKind :: Variant => tcx . parent (parent_id) , _ => parent_id , } ; item_and_field_ids . push ((item_id . expect_local () , field_id)) ; } } for info in & cycle_error . cycle { if info . query . dep_kind == dep_kinds :: representability_adt_ty && let Some (def_id) = info . query . def_id_for_ty_in_cycle && let Some (def_id) = def_id . as_local () && ! item_and_field_ids . iter () . any (| & (id , _) | id == def_id) { representable_ids . insert (def_id) ; } } let guar = recursive_type_error (tcx , item_and_field_ids , & representable_ids) ; Representability :: Infinite (guar) } }}}
mkitem!{mkimpl!{impl < 'tcx > Value < TyCtxt < 'tcx > > for ty :: EarlyBinder < '_ , Ty < '_ > > { fn from_cycle_error (tcx : TyCtxt < 'tcx > , cycle_error : & CycleError , guar : ErrorGuaranteed ,) -> Self { ty :: EarlyBinder :: bind (Ty :: from_cycle_error (tcx , cycle_error , guar)) } }}}
mkitem!{mkimpl!{impl < 'tcx > Value < TyCtxt < 'tcx > > for ty :: EarlyBinder < '_ , ty :: Binder < '_ , ty :: FnSig < '_ > > > { fn from_cycle_error (tcx : TyCtxt < 'tcx > , cycle_error : & CycleError , guar : ErrorGuaranteed ,) -> Self { ty :: EarlyBinder :: bind (ty :: Binder :: from_cycle_error (tcx , cycle_error , guar)) } }}}
mkitem!{mkimpl!{impl < 'tcx > Value < TyCtxt < 'tcx > > for & [ty :: Variance] { fn from_cycle_error (tcx : TyCtxt < 'tcx > , cycle_error : & CycleError , _guar : ErrorGuaranteed ,) -> Self { search_for_cycle_permutation (& cycle_error . cycle , | cycle | { if let Some (frame) = cycle . get (0) && frame . query . dep_kind == dep_kinds :: variances_of && let Some (def_id) = frame . query . def_id { let n = tcx . generics_of (def_id) . own_params . len () ; ControlFlow :: Break (vec ! [ty :: Bivariant ; n] . leak ()) } else { ControlFlow :: Continue (()) } } , | | { span_bug ! (cycle_error . usage . as_ref () . unwrap () . 0 , "only `variances_of` returns `&[ty::Variance]`") } ,) } }}}

macro_rules! search_for_cycle_permutation_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function search_for_cycle_permutation in module {}", module_path!());
    };
}

mkfn!{
    search_for_cycle_permutation_introspect!();
    fn search_for_cycle_permutation < Q , T > (cycle : & [Q] , try_cycle : impl Fn (& mut VecDeque < & Q >) -> ControlFlow < T , () > , otherwise : impl FnOnce () -> T ,) -> T { let mut cycle : VecDeque < _ > = cycle . iter () . collect () ; for _ in 0 .. cycle . len () { match try_cycle (& mut cycle) { ControlFlow :: Continue (_) => { cycle . rotate_left (1) ; } ControlFlow :: Break (t) => return t , } } otherwise () }
}
mkitem!{mkimpl!{impl < 'tcx , T > Value < TyCtxt < 'tcx > > for Result < T , & '_ ty :: layout :: LayoutError < '_ > > { fn from_cycle_error (tcx : TyCtxt < 'tcx > , cycle_error : & CycleError , _guar : ErrorGuaranteed ,) -> Self { let diag = search_for_cycle_permutation (& cycle_error . cycle , | cycle | { if cycle [0] . query . dep_kind == dep_kinds :: layout_of && let Some (def_id) = cycle [0] . query . def_id_for_ty_in_cycle && let Some (def_id) = def_id . as_local () && let def_kind = tcx . def_kind (def_id) && matches ! (def_kind , DefKind :: Closure) && let Some (coroutine_kind) = tcx . coroutine_kind (def_id) { let span = if coroutine_kind . is_fn_like () { tcx . def_span (tcx . local_parent (def_id)) } else { tcx . def_span (def_id) } ; let mut diag = struct_span_code_err ! (tcx . sess . dcx () , span , E0733 , "recursion in {} {} requires boxing" , tcx . def_kind_descr_article (def_kind , def_id . to_def_id ()) , tcx . def_kind_descr (def_kind , def_id . to_def_id ()) ,) ; for (i , frame) in cycle . iter () . enumerate () { if frame . query . dep_kind != dep_kinds :: layout_of { continue ; } let Some (frame_def_id) = frame . query . def_id_for_ty_in_cycle else { continue ; } ; let Some (frame_coroutine_kind) = tcx . coroutine_kind (frame_def_id) else { continue ; } ; let frame_span = frame . query . info . default_span (cycle [(i + 1) % cycle . len ()] . span) ; if frame_span . is_dummy () { continue ; } if i == 0 { diag . span_label (frame_span , "recursive call here") ; } else { let coroutine_span : Span = if frame_coroutine_kind . is_fn_like () { tcx . def_span (tcx . parent (frame_def_id)) } else { tcx . def_span (frame_def_id) } ; let mut multispan = MultiSpan :: from_span (coroutine_span) ; multispan . push_span_label (frame_span , "...leading to this recursive call") ; diag . span_note (multispan , format ! ("which leads to this {}" , tcx . def_descr (frame_def_id)) ,) ; } } if matches ! (coroutine_kind , hir :: CoroutineKind :: Desugared (hir :: CoroutineDesugaring :: Async , _)) { diag . note ("a recursive `async fn` call must introduce indirection such as `Box::pin` to avoid an infinitely sized future") ; } ControlFlow :: Break (diag) } else { ControlFlow :: Continue (()) } } , | | report_cycle (tcx . sess , cycle_error) ,) ; let guar = diag . emit () ; Err (Box :: leak (Box :: new (ty :: layout :: LayoutError :: Cycle (guar)))) } }}}

macro_rules! recursive_type_error_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function recursive_type_error in module {}", module_path!());
    };
}

mkfn!{
    recursive_type_error_introspect!();
    pub fn recursive_type_error (tcx : TyCtxt < '_ > , mut item_and_field_ids : Vec < (LocalDefId , LocalDefId) > , representable_ids : & FxHashSet < LocalDefId > ,) -> ErrorGuaranteed { const ITEM_LIMIT : usize = 5 ; let start_index = item_and_field_ids . iter () . enumerate () . min_by_key (| & (_ , & (id , _)) | tcx . def_span (id)) . unwrap () . 0 ; item_and_field_ids . rotate_left (start_index) ; let cycle_len = item_and_field_ids . len () ; let show_cycle_len = cycle_len . min (ITEM_LIMIT) ; let mut err_span = MultiSpan :: from_spans (item_and_field_ids [.. show_cycle_len] . iter () . map (| (id , _) | tcx . def_span (id . to_def_id ())) . collect () ,) ; let mut suggestion = Vec :: with_capacity (show_cycle_len * 2) ; for i in 0 .. show_cycle_len { let (_ , field_id) = item_and_field_ids [i] ; let (next_item_id , _) = item_and_field_ids [(i + 1) % cycle_len] ; let hir :: Node :: Field (field) = tcx . hir_node_by_def_id (field_id) else { bug ! ("expected field") } ; let mut found = Vec :: new () ; find_item_ty_spans (tcx , field . ty , next_item_id , & mut found , representable_ids) ; if found . is_empty () { found . push (field . ty . span) ; } for span in found { err_span . push_span_label (span , "recursive without indirection") ; suggestion . push ((span . shrink_to_lo () , "Box<" . to_string ())) ; suggestion . push ((span . shrink_to_hi () , ">" . to_string ())) ; } } let items_list = { let mut s = String :: new () ; for (i , & (item_id , _)) in item_and_field_ids . iter () . enumerate () { let path = tcx . def_path_str (item_id) ; write ! (& mut s , "`{path}`") . unwrap () ; if i == (ITEM_LIMIT - 1) && cycle_len > ITEM_LIMIT { write ! (& mut s , " and {} more" , cycle_len - 5) . unwrap () ; break ; } if cycle_len > 1 && i < cycle_len - 2 { s . push_str (", ") ; } else if cycle_len > 1 && i == cycle_len - 2 { s . push_str (" and ") } } s } ; struct_span_code_err ! (tcx . dcx () , err_span , E0072 , "recursive type{} {} {} infinite size" , pluralize ! (cycle_len) , items_list , pluralize ! ("has" , cycle_len) ,) . with_multipart_suggestion ("insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle" , suggestion , Applicability :: HasPlaceholders ,) . emit () }
}

macro_rules! find_item_ty_spans_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_item_ty_spans in module {}", module_path!());
    };
}

mkfn!{
    find_item_ty_spans_introspect!();
    fn find_item_ty_spans (tcx : TyCtxt < '_ > , ty : & hir :: Ty < '_ > , needle : LocalDefId , spans : & mut Vec < Span > , seen_representable : & FxHashSet < LocalDefId > ,) { match ty . kind { hir :: TyKind :: Path (hir :: QPath :: Resolved (_ , path)) => { if let Res :: Def (kind , def_id) = path . res && matches ! (kind , DefKind :: Enum | DefKind :: Struct | DefKind :: Union) { let check_params = def_id . as_local () . is_none_or (| def_id | { if def_id == needle { spans . push (ty . span) ; } seen_representable . contains (& def_id) }) ; if check_params && let Some (args) = path . segments . last () . unwrap () . args { let params_in_repr = tcx . params_in_repr (def_id) ; for (i , arg) in args . args . iter () . enumerate () . take (params_in_repr . domain_size ()) { if let hir :: GenericArg :: Type (ty) = arg && params_in_repr . contains (i as u32) { find_item_ty_spans (tcx , ty . as_unambig_ty () , needle , spans , seen_representable ,) ; } } } } } hir :: TyKind :: Array (ty , _) => find_item_ty_spans (tcx , ty , needle , spans , seen_representable) , hir :: TyKind :: Tup (tys) => { tys . iter () . for_each (| ty | find_item_ty_spans (tcx , ty , needle , spans , seen_representable)) } _ => { } } }
}