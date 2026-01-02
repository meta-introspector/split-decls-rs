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
mkuse!{use std :: assert_matches :: debug_assert_matches ;}
mkuse!{use either :: { Left , Right } ;}
mkuse!{use rustc_abi :: { Align , HasDataLayout , Size , TargetDataLayout } ;}
mkuse!{use rustc_errors :: DiagCtxtHandle ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_hir :: limit :: Limit ;}
mkuse!{use rustc_middle :: mir :: interpret :: { ErrorHandled , InvalidMetaKind , ReportedErrorInfo } ;}
mkuse!{use rustc_middle :: query :: TyCtxtAt ;}
mkuse!{use rustc_middle :: ty :: layout :: { self , FnAbiError , FnAbiOf , FnAbiOfHelpers , FnAbiRequest , LayoutError , LayoutOf , LayoutOfHelpers , TyAndLayout , } ;}
mkuse!{use rustc_middle :: ty :: { self , GenericArgsRef , Ty , TyCtxt , TypeFoldable , TypingEnv , Variance } ;}
mkuse!{use rustc_middle :: { mir , span_bug } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use rustc_target :: callconv :: FnAbi ;}
mkuse!{use tracing :: { debug , trace } ;}
mkuse!{use super :: { Frame , FrameInfo , GlobalId , InterpErrorInfo , InterpErrorKind , InterpResult , MPlaceTy , Machine , MemPlaceMeta , Memory , OpTy , Place , PlaceTy , PointerArithmetic , Projectable , Provenance , err_inval , interp_ok , throw_inval , throw_ub , throw_ub_custom , } ;}
mkuse!{use crate :: { ReportErrorExt , enter_trace_span , fluent_generated as fluent , util } ;}
mkitem!{mkstruct!{pub struct InterpCx < 'tcx , M : Machine < 'tcx > > { #[doc = " Stores the `Machine` instance."] #[doc = ""] #[doc = " Note: the stack is provided by the machine."] pub machine : M , #[doc = " The results of the type checker, from rustc."] #[doc = " The span in this is the \"root\" of the evaluation, i.e., the const"] #[doc = " we are evaluating (if this is CTFE)."] pub tcx : TyCtxtAt < 'tcx > , #[doc = " The current context in case we're evaluating in a"] #[doc = " polymorphic context. This always uses `ty::TypingMode::PostAnalysis`."] pub (super) typing_env : ty :: TypingEnv < 'tcx > , #[doc = " The virtual memory system."] pub memory : Memory < 'tcx , M > , #[doc = " The recursion limit (cached from `tcx.recursion_limit(())`)"] pub recursion_limit : Limit , }}}
mkitem!{mkimpl!{impl < 'tcx , M : Machine < 'tcx > > HasDataLayout for InterpCx < 'tcx , M > { #[inline] fn data_layout (& self) -> & TargetDataLayout { & self . tcx . data_layout } }}}
mkitem!{mkimpl!{impl < 'tcx , M > layout :: HasTyCtxt < 'tcx > for InterpCx < 'tcx , M > where M : Machine < 'tcx > , { #[inline] fn tcx (& self) -> TyCtxt < 'tcx > { * self . tcx } }}}
mkitem!{mkimpl!{impl < 'tcx , M > layout :: HasTypingEnv < 'tcx > for InterpCx < 'tcx , M > where M : Machine < 'tcx > , { fn typing_env (& self) -> ty :: TypingEnv < 'tcx > { self . typing_env } }}}
mkitem!{mkimpl!{impl < 'tcx , M : Machine < 'tcx > > LayoutOfHelpers < 'tcx > for InterpCx < 'tcx , M > { type LayoutOfResult = Result < TyAndLayout < 'tcx > , InterpErrorKind < 'tcx > > ; #[inline] fn layout_tcx_at_span (& self) -> Span { self . tcx . span } #[inline] fn handle_layout_err (& self , err : LayoutError < 'tcx > , _ : Span , _ : Ty < 'tcx > ,) -> InterpErrorKind < 'tcx > { err_inval ! (Layout (err)) } }}}
mkitem!{mkimpl!{impl < 'tcx , M : Machine < 'tcx > > FnAbiOfHelpers < 'tcx > for InterpCx < 'tcx , M > { type FnAbiOfResult = Result < & 'tcx FnAbi < 'tcx , Ty < 'tcx > > , InterpErrorKind < 'tcx > > ; fn handle_fn_abi_err (& self , err : FnAbiError < 'tcx > , _span : Span , _fn_abi_request : FnAbiRequest < 'tcx > ,) -> InterpErrorKind < 'tcx > { match err { FnAbiError :: Layout (err) => err_inval ! (Layout (err)) , } } }}}
mkitem!{mkimpl!{impl < 'tcx , M : Machine < 'tcx > > InterpCx < 'tcx , M > { #[doc = " This inherent method takes priority over the trait method with the same name in LayoutOf,"] #[doc = " and allows wrapping the actual [LayoutOf::layout_of] with a tracing span."] #[doc = " See [LayoutOf::layout_of] for the original documentation."] #[inline (always)] pub fn layout_of (& self , ty : Ty < 'tcx >) -> < Self as LayoutOfHelpers < 'tcx > > :: LayoutOfResult { let _trace = enter_trace_span ! (M , layouting :: layout_of , ty = ? ty . kind ()) ; LayoutOf :: layout_of (self , ty) } #[doc = " This inherent method takes priority over the trait method with the same name in FnAbiOf,"] #[doc = " and allows wrapping the actual [FnAbiOf::fn_abi_of_fn_ptr] with a tracing span."] #[doc = " See [FnAbiOf::fn_abi_of_fn_ptr] for the original documentation."] #[inline (always)] pub fn fn_abi_of_fn_ptr (& self , sig : ty :: PolyFnSig < 'tcx > , extra_args : & 'tcx ty :: List < Ty < 'tcx > > ,) -> < Self as FnAbiOfHelpers < 'tcx > > :: FnAbiOfResult { let _trace = enter_trace_span ! (M , layouting :: fn_abi_of_fn_ptr , ? sig , ? extra_args) ; FnAbiOf :: fn_abi_of_fn_ptr (self , sig , extra_args) } #[doc = " This inherent method takes priority over the trait method with the same name in FnAbiOf,"] #[doc = " and allows wrapping the actual [FnAbiOf::fn_abi_of_instance] with a tracing span."] #[doc = " See [FnAbiOf::fn_abi_of_instance] for the original documentation."] #[inline (always)] pub fn fn_abi_of_instance (& self , instance : ty :: Instance < 'tcx > , extra_args : & 'tcx ty :: List < Ty < 'tcx > > ,) -> < Self as FnAbiOfHelpers < 'tcx > > :: FnAbiOfResult { let _trace = enter_trace_span ! (M , layouting :: fn_abi_of_instance , ? instance , ? extra_args) ; FnAbiOf :: fn_abi_of_instance (self , instance , extra_args) } }}}

macro_rules! mir_assign_valid_types_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mir_assign_valid_types in module {}", module_path!());
    };
}

mkfn!{
    mir_assign_valid_types_introspect!();
    #[doc = " Test if it is valid for a MIR assignment to assign `src`-typed place to `dest`-typed value."] #[doc = " This test should be symmetric, as it is primarily about layout compatibility."] pub (super) fn mir_assign_valid_types < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : TypingEnv < 'tcx > , src : TyAndLayout < 'tcx > , dest : TyAndLayout < 'tcx > ,) -> bool { if util :: relate_types (tcx , typing_env , Variance :: Covariant , src . ty , dest . ty) { if cfg ! (debug_assertions) || src . ty != dest . ty { assert_eq ! (src . layout , dest . layout) ; } true } else { false } }
}

macro_rules! from_known_layout_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function from_known_layout in module {}", module_path!());
    };
}

mkfn!{
    from_known_layout_introspect!();
    #[doc = " Use the already known layout if given (but sanity check in debug mode),"] #[doc = " or compute the layout."] #[cfg_attr (not (debug_assertions) , inline (always))] pub (super) fn from_known_layout < 'tcx > (tcx : TyCtxtAt < 'tcx > , typing_env : TypingEnv < 'tcx > , known_layout : Option < TyAndLayout < 'tcx > > , compute : impl FnOnce () -> InterpResult < 'tcx , TyAndLayout < 'tcx > > ,) -> InterpResult < 'tcx , TyAndLayout < 'tcx > > { match known_layout { None => compute () , Some (known_layout) => { if cfg ! (debug_assertions) { let check_layout = compute () ? ; if ! mir_assign_valid_types (tcx . tcx , typing_env , check_layout , known_layout) { span_bug ! (tcx . span , "expected type differs from actual type.\nexpected: {}\nactual: {}" , known_layout . ty , check_layout . ty ,) ; } } interp_ok (known_layout) } } }
}

macro_rules! format_interp_error_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function format_interp_error in module {}", module_path!());
    };
}

mkfn!{
    format_interp_error_introspect!();
    #[doc = " Turn the given error into a human-readable string. Expects the string to be printed, so if"] #[doc = " `RUSTC_CTFE_BACKTRACE` is set this will show a backtrace of the rustc internals that"] #[doc = " triggered the error."] #[doc = ""] #[doc = " This is NOT the preferred way to render an error; use `report` from `const_eval` instead."] #[doc = " However, this is useful when error messages appear in ICEs."] pub fn format_interp_error < 'tcx > (dcx : DiagCtxtHandle < '_ > , e : InterpErrorInfo < 'tcx >) -> String { let (e , backtrace) = e . into_parts () ; backtrace . print_backtrace () ; #[allow (rustc :: untranslatable_diagnostic)] let mut diag = dcx . struct_allow ("") ; let msg = e . diagnostic_message () ; e . add_args (& mut diag) ; let s = dcx . eagerly_translate_to_string (msg , diag . args . iter ()) ; diag . cancel () ; s }
}
mkitem!{mkimpl!{impl < 'tcx , M : Machine < 'tcx > > InterpCx < 'tcx , M > { pub fn new (tcx : TyCtxt < 'tcx > , root_span : Span , typing_env : ty :: TypingEnv < 'tcx > , machine : M ,) -> Self { debug_assert_matches ! (typing_env . typing_mode , ty :: TypingMode :: PostAnalysis) ; InterpCx { machine , tcx : tcx . at (root_span) , typing_env , memory : Memory :: new () , recursion_limit : tcx . recursion_limit () , } } #[doc = " Returns the span of the currently executed statement/terminator."] #[doc = " This is the span typically used for error reporting."] #[inline (always)] pub fn cur_span (& self) -> Span { self . stack () . last () . map_or (self . tcx . span , | f | f . current_span ()) } pub (crate) fn stack (& self) -> & [Frame < 'tcx , M :: Provenance , M :: FrameExtra >] { M :: stack (self) } #[inline (always)] pub (crate) fn stack_mut (& mut self) -> & mut Vec < Frame < 'tcx , M :: Provenance , M :: FrameExtra > > { M :: stack_mut (self) } #[inline (always)] pub fn frame_idx (& self) -> usize { let stack = self . stack () ; assert ! (! stack . is_empty ()) ; stack . len () - 1 } #[inline (always)] pub fn frame (& self) -> & Frame < 'tcx , M :: Provenance , M :: FrameExtra > { self . stack () . last () . expect ("no call frames exist") } #[inline (always)] pub fn frame_mut (& mut self) -> & mut Frame < 'tcx , M :: Provenance , M :: FrameExtra > { self . stack_mut () . last_mut () . expect ("no call frames exist") } #[inline (always)] pub fn body (& self) -> & 'tcx mir :: Body < 'tcx > { self . frame () . body } #[inline] pub fn type_is_freeze (& self , ty : Ty < 'tcx >) -> bool { ty . is_freeze (* self . tcx , self . typing_env) } pub fn load_mir (& self , instance : ty :: InstanceKind < 'tcx > , promoted : Option < mir :: Promoted > ,) -> InterpResult < 'tcx , & 'tcx mir :: Body < 'tcx > > { trace ! ("load mir(instance={:?}, promoted={:?})" , instance , promoted) ; let body = if let Some (promoted) = promoted { let def = instance . def_id () ; & self . tcx . promoted_mir (def) [promoted] } else { M :: load_mir (self , instance) } ; if let Some (err) = body . tainted_by_errors { throw_inval ! (AlreadyReported (ReportedErrorInfo :: non_const_eval_error (err))) ; } interp_ok (body) } #[doc = " Call this on things you got out of the MIR (so it is as generic as the current"] #[doc = " stack frame), to bring it into the proper environment for this interpreter."] pub fn instantiate_from_current_frame_and_normalize_erasing_regions < T : TypeFoldable < TyCtxt < 'tcx > > , > (& self , value : T ,) -> Result < T , ErrorHandled > { self . instantiate_from_frame_and_normalize_erasing_regions (self . frame () , value) } #[doc = " Call this on things you got out of the MIR (so it is as generic as the provided"] #[doc = " stack frame), to bring it into the proper environment for this interpreter."] pub fn instantiate_from_frame_and_normalize_erasing_regions < T : TypeFoldable < TyCtxt < 'tcx > > > (& self , frame : & Frame < 'tcx , M :: Provenance , M :: FrameExtra > , value : T ,) -> Result < T , ErrorHandled > { let _trace = enter_trace_span ! (M , "instantiate_from_frame_and_normalize_erasing_regions" , % frame . instance) ; frame . instance . try_instantiate_mir_and_normalize_erasing_regions (* self . tcx , self . typing_env , ty :: EarlyBinder :: bind (value) ,) . map_err (| _ | ErrorHandled :: TooGeneric (self . cur_span ())) } #[doc = " The `args` are assumed to already be in our interpreter \"universe\"."] pub (super) fn resolve (& self , def : DefId , args : GenericArgsRef < 'tcx > ,) -> InterpResult < 'tcx , ty :: Instance < 'tcx > > { let _trace = enter_trace_span ! (M , resolve :: try_resolve , def = ? def) ; trace ! ("resolve: {:?}, {:#?}" , def , args) ; trace ! ("typing_env: {:#?}" , self . typing_env) ; trace ! ("args: {:#?}" , args) ; match ty :: Instance :: try_resolve (* self . tcx , self . typing_env , def , args) { Ok (Some (instance)) => interp_ok (instance) , Ok (None) => throw_inval ! (TooGeneric) , Err (error_guaranteed) => throw_inval ! (AlreadyReported (ReportedErrorInfo :: non_const_eval_error (error_guaranteed))) , } } #[doc = " Walks up the callstack from the intrinsic's callsite, searching for the first callsite in a"] #[doc = " frame which is not `#[track_caller]`. This matches the `caller_location` intrinsic,"] #[doc = " and is primarily intended for the panic machinery."] pub (crate) fn find_closest_untracked_caller_location (& self) -> Span { for frame in self . stack () . iter () . rev () { debug ! ("find_closest_untracked_caller_location: checking frame {:?}" , frame . instance) ; let loc = frame . loc . left () . unwrap () ; let mut source_info = * frame . body . source_info (loc) ; let block = & frame . body . basic_blocks [loc . block] ; if loc . statement_index == block . statements . len () { debug ! ("find_closest_untracked_caller_location: got terminator {:?} ({:?})" , block . terminator () , block . terminator () . kind ,) ; if let mir :: TerminatorKind :: Call { fn_span , .. } = block . terminator () . kind { source_info . span = fn_span ; } } let caller_location = if frame . instance . def . requires_caller_location (* self . tcx) { Some (Err (())) } else { None } ; if let Ok (span) = frame . body . caller_location_span (source_info , caller_location , * self . tcx , Ok) { return span ; } } span_bug ! (self . cur_span () , "no non-`#[track_caller]` frame found") } #[doc = " Returns the actual dynamic size and alignment of the place at the given type."] #[doc = " Only the \"meta\" (metadata) part of the place matters."] #[doc = " This can fail to provide an answer for extern types."] pub (super) fn size_and_align_from_meta (& self , metadata : & MemPlaceMeta < M :: Provenance > , layout : & TyAndLayout < 'tcx > ,) -> InterpResult < 'tcx , Option < (Size , Align) > > { if layout . is_sized () { return interp_ok (Some ((layout . size , layout . align . abi))) ; } match layout . ty . kind () { ty :: Adt (..) | ty :: Tuple (..) => { assert ! (! layout . ty . is_simd ()) ; assert ! (layout . fields . count () > 0) ; trace ! ("DST layout: {:?}" , layout) ; let unsized_offset_unadjusted = layout . fields . offset (layout . fields . count () - 1) ; let sized_align = layout . align . abi ; let field = layout . field (self , layout . fields . count () - 1) ; let Some ((unsized_size , mut unsized_align)) = self . size_and_align_from_meta (metadata , & field) ? else { return interp_ok (None) ; } ; if let ty :: Adt (def , _) = layout . ty . kind () && let Some (packed) = def . repr () . pack { unsized_align = unsized_align . min (packed) ; } let full_align = sized_align . max (unsized_align) ; let unsized_offset_adjusted = unsized_offset_unadjusted . align_to (unsized_align) ; let full_size = (unsized_offset_adjusted + unsized_size) . align_to (full_align) ; assert_eq ! (full_size , (unsized_offset_unadjusted + unsized_size) . align_to (full_align)) ; if full_size > self . max_size_of_val () { throw_ub ! (InvalidMeta (InvalidMetaKind :: TooBig)) ; } interp_ok (Some ((full_size , full_align))) } ty :: Dynamic (expected_trait , _ , ty :: Dyn) => { let vtable = metadata . unwrap_meta () . to_pointer (self) ? ; interp_ok (Some (self . get_vtable_size_and_align (vtable , Some (expected_trait)) ?)) } ty :: Slice (_) | ty :: Str => { let len = metadata . unwrap_meta () . to_target_usize (self) ? ; let elem = layout . field (self , 0) ; let size = elem . size . bytes () . saturating_mul (len) ; let size = Size :: from_bytes (size) ; if size > self . max_size_of_val () { throw_ub ! (InvalidMeta (InvalidMetaKind :: SliceTooBig)) ; } interp_ok (Some ((size , elem . align . abi))) } ty :: Foreign (_) => interp_ok (None) , _ => span_bug ! (self . cur_span () , "size_and_align_of::<{}> not supported" , layout . ty) , } } #[inline] pub fn size_and_align_of_val (& self , val : & impl Projectable < 'tcx , M :: Provenance > ,) -> InterpResult < 'tcx , Option < (Size , Align) > > { self . size_and_align_from_meta (& val . meta () , & val . layout ()) } #[doc = " Jump to the given block."] #[inline] pub fn go_to_block (& mut self , target : mir :: BasicBlock) { self . frame_mut () . loc = Left (mir :: Location { block : target , statement_index : 0 }) ; } #[doc = " *Return* to the given `target` basic block."] #[doc = " Do *not* use for unwinding! Use `unwind_to_block` instead."] #[doc = ""] #[doc = " If `target` is `None`, that indicates the function cannot return, so we raise UB."] pub fn return_to_block (& mut self , target : Option < mir :: BasicBlock >) -> InterpResult < 'tcx > { if let Some (target) = target { self . go_to_block (target) ; interp_ok (()) } else { throw_ub ! (Unreachable) } } #[doc = " *Unwind* to the given `target` basic block."] #[doc = " Do *not* use for returning! Use `return_to_block` instead."] #[doc = ""] #[doc = " If `target` is `UnwindAction::Continue`, that indicates the function does not need cleanup"] #[doc = " during unwinding, and we will just keep propagating that upwards."] #[doc = ""] #[doc = " If `target` is `UnwindAction::Unreachable`, that indicates the function does not allow"] #[doc = " unwinding, and doing so is UB."] #[cold] pub fn unwind_to_block (& mut self , target : mir :: UnwindAction) -> InterpResult < 'tcx > { self . frame_mut () . loc = match target { mir :: UnwindAction :: Cleanup (block) => Left (mir :: Location { block , statement_index : 0 }) , mir :: UnwindAction :: Continue => Right (self . frame_mut () . body . span) , mir :: UnwindAction :: Unreachable => { throw_ub_custom ! (fluent :: const_eval_unreachable_unwind) ; } mir :: UnwindAction :: Terminate (reason) => { self . frame_mut () . loc = Right (self . frame_mut () . body . span) ; M :: unwind_terminate (self , reason) ? ; return interp_ok (()) ; } } ; interp_ok (()) } #[doc = " Call a query that can return `ErrorHandled`. Should be used for statics and other globals."] #[doc = " (`mir::Const`/`ty::Const` have `eval` methods that can be used directly instead.)"] pub fn ctfe_query < T > (& self , query : impl FnOnce (TyCtxtAt < 'tcx >) -> Result < T , ErrorHandled > ,) -> Result < T , ErrorHandled > { query (self . tcx . at (self . cur_span ())) . map_err (| err | { err . emit_note (* self . tcx) ; err }) } pub fn eval_global (& self , instance : ty :: Instance < 'tcx > ,) -> InterpResult < 'tcx , MPlaceTy < 'tcx , M :: Provenance > > { let gid = GlobalId { instance , promoted : None } ; let val = if self . tcx . is_static (gid . instance . def_id ()) { let alloc_id = self . tcx . reserve_and_set_static_alloc (gid . instance . def_id ()) ; let ty = instance . ty (self . tcx . tcx , self . typing_env) ; mir :: ConstAlloc { alloc_id , ty } } else { self . ctfe_query (| tcx | tcx . eval_to_allocation_raw (self . typing_env . as_query_input (gid))) ? } ; self . raw_const_to_mplace (val) } pub fn eval_mir_constant (& self , val : & mir :: Const < 'tcx > , span : Span , layout : Option < TyAndLayout < 'tcx > > ,) -> InterpResult < 'tcx , OpTy < 'tcx , M :: Provenance > > { let _trace = enter_trace_span ! (M , const_eval :: eval_mir_constant , ? val) ; let const_val = val . eval (* self . tcx , self . typing_env , span) . map_err (| err | { if M :: ALL_CONSTS_ARE_PRECHECKED { match err { ErrorHandled :: TooGeneric (..) => { } , ErrorHandled :: Reported (reported , span) => { if reported . is_allowed_in_infallible () { } else { span_bug ! (span , "interpret const eval failure of {val:?} which is not in required_consts") ; } } } } err . emit_note (* self . tcx) ; err }) ? ; self . const_val_to_op (const_val , val . ty () , layout) } #[must_use] pub fn dump_place (& self , place : & PlaceTy < 'tcx , M :: Provenance >) -> PlacePrinter < '_ , 'tcx , M > { PlacePrinter { ecx : self , place : * place . place () } } #[must_use] pub fn generate_stacktrace (& self) -> Vec < FrameInfo < 'tcx > > { Frame :: generate_stacktrace_from_stack (self . stack ()) } pub fn adjust_nan < F1 , F2 > (& self , f : F2 , inputs : & [F1]) -> F2 where F1 : rustc_apfloat :: Float + rustc_apfloat :: FloatConvert < F2 > , F2 : rustc_apfloat :: Float , { if f . is_nan () { M :: generate_nan (self , inputs) } else { f } } }}}
mkitem!{mkstruct!{#[doc (hidden)] #[doc = " Helper struct for the `dump_place` function."] pub struct PlacePrinter < 'a , 'tcx , M : Machine < 'tcx > > { ecx : & 'a InterpCx < 'tcx , M > , place : Place < M :: Provenance > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx , M : Machine < 'tcx > > std :: fmt :: Debug for PlacePrinter < 'a , 'tcx , M > { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self . place { Place :: Local { local , offset , locals_addr } => { debug_assert_eq ! (locals_addr , self . ecx . frame () . locals_addr ()) ; let mut allocs = Vec :: new () ; write ! (fmt , "{local:?}") ? ; if let Some (offset) = offset { write ! (fmt , "+{:#x}" , offset . bytes ()) ? ; } write ! (fmt , ":") ? ; self . ecx . frame () . locals [local] . print (& mut allocs , fmt) ? ; write ! (fmt , ": {:?}" , self . ecx . dump_allocs (allocs . into_iter () . flatten () . collect ())) } Place :: Ptr (mplace) => match mplace . ptr . provenance . and_then (Provenance :: get_alloc_id) { Some (alloc_id) => { write ! (fmt , "by ref {:?}: {:?}" , mplace . ptr , self . ecx . dump_alloc (alloc_id)) } ptr => write ! (fmt , " integral by ref: {ptr:?}") , } , } } }}}