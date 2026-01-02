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
mkuse!{use std :: borrow :: { Borrow , Cow } ;}
mkuse!{use std :: fmt ;}
mkuse!{use std :: hash :: Hash ;}
mkuse!{use rustc_abi :: { Align , Size } ;}
mkuse!{use rustc_ast :: Mutability ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashMap , FxIndexMap , IndexEntry } ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_hir :: { self as hir , CRATE_HIR_ID , LangItem } ;}
mkuse!{use rustc_middle :: mir :: AssertMessage ;}
mkuse!{use rustc_middle :: mir :: interpret :: ReportedErrorInfo ;}
mkuse!{use rustc_middle :: query :: TyCtxtAt ;}
mkuse!{use rustc_middle :: ty :: layout :: { HasTypingEnv , TyAndLayout , ValidityRequirement } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt } ;}
mkuse!{use rustc_middle :: { bug , mir } ;}
mkuse!{use rustc_span :: { Span , Symbol , sym } ;}
mkuse!{use rustc_target :: callconv :: FnAbi ;}
mkuse!{use tracing :: debug ;}
mkuse!{use super :: error :: * ;}
mkuse!{use crate :: errors :: { LongRunning , LongRunningWarn } ;}
mkuse!{use crate :: fluent_generated as fluent ;}
mkuse!{use crate :: interpret :: { self , AllocId , AllocInit , AllocRange , ConstAllocation , CtfeProvenance , FnArg , Frame , GlobalAlloc , ImmTy , InterpCx , InterpResult , OpTy , PlaceTy , Pointer , RangeSet , Scalar , compile_time_machine , err_inval , interp_ok , throw_exhaust , throw_inval , throw_ub , throw_ub_custom , throw_unsup , throw_unsup_format , } ;}
mkitem!{#[doc = " When hitting this many interpreted terminators we emit a deny by default lint"] #[doc = " that notfies the user that their constant takes a long time to evaluate. If that's"] #[doc = " what they intended, they can just allow the lint."] const LINT_TERMINATOR_LIMIT : usize = 2_000_000 ;}
mkitem!{#[doc = " The limit used by `-Z tiny-const-eval-limit`. This smaller limit is useful for internal"] #[doc = " tests not needing to run 30s or more to show some behaviour."] const TINY_LINT_TERMINATOR_LIMIT : usize = 20 ;}
mkitem!{#[doc = " After this many interpreted terminators, we start emitting progress indicators at every"] #[doc = " power of two of interpreted terminators."] const PROGRESS_INDICATOR_START : usize = 4_000_000 ;}
mkitem!{mkstruct!{#[doc = " Extra machine state for CTFE, and the Machine instance."] pub struct CompileTimeMachine < 'tcx > { #[doc = " The number of terminators that have been evaluated."] #[doc = ""] #[doc = " This is used to produce lints informing the user that the compiler is not stuck."] #[doc = " Set to `usize::MAX` to never report anything."] pub (super) num_evaluated_steps : usize , #[doc = " The virtual call stack."] pub (super) stack : Vec < Frame < 'tcx > > , #[doc = " Pattern matching on consts with references would be unsound if those references"] #[doc = " could point to anything mutable. Therefore, when evaluating consts and when constructing valtrees,"] #[doc = " we ensure that only immutable global memory can be accessed."] pub (super) can_access_mut_global : CanAccessMutGlobal , #[doc = " Whether to check alignment during evaluation."] pub (super) check_alignment : CheckAlignment , #[doc = " If `Some`, we are evaluating the initializer of the static with the given `LocalDefId`,"] #[doc = " storing the result in the given `AllocId`."] #[doc = " Used to prevent accesses to a static's base allocation, as that may allow for self-initialization loops."] pub (crate) static_root_ids : Option < (AllocId , LocalDefId) > , #[doc = " A cache of \"data range\" computations for unions (i.e., the offsets of non-padding bytes)."] union_data_ranges : FxHashMap < Ty < 'tcx > , RangeSet > , }}}
mkitem!{mkenum!{#[derive (Copy , Clone)] pub enum CheckAlignment { #[doc = " Ignore all alignment requirements."] #[doc = " This is mainly used in interning."] No , #[doc = " Hard error when dereferencing a misaligned pointer."] Error , }}}
mkitem!{mkenum!{#[derive (Copy , Clone , PartialEq)] pub (crate) enum CanAccessMutGlobal { No , Yes , }}}
mkitem!{mkimpl!{impl From < bool > for CanAccessMutGlobal { fn from (value : bool) -> Self { if value { Self :: Yes } else { Self :: No } } }}}
mkitem!{mkimpl!{impl < 'tcx > CompileTimeMachine < 'tcx > { pub (crate) fn new (can_access_mut_global : CanAccessMutGlobal , check_alignment : CheckAlignment ,) -> Self { CompileTimeMachine { num_evaluated_steps : 0 , stack : Vec :: new () , can_access_mut_global , check_alignment , static_root_ids : None , union_data_ranges : FxHashMap :: default () , } } }}}
mkitem!{mkimpl!{impl < K : Hash + Eq , V > interpret :: AllocMap < K , V > for FxIndexMap < K , V > { #[inline (always)] fn contains_key < Q : ? Sized + Hash + Eq > (& mut self , k : & Q) -> bool where K : Borrow < Q > , { FxIndexMap :: contains_key (self , k) } #[inline (always)] fn contains_key_ref < Q : ? Sized + Hash + Eq > (& self , k : & Q) -> bool where K : Borrow < Q > , { FxIndexMap :: contains_key (self , k) } #[inline (always)] fn insert (& mut self , k : K , v : V) -> Option < V > { FxIndexMap :: insert (self , k , v) } #[inline (always)] fn remove < Q : ? Sized + Hash + Eq > (& mut self , k : & Q) -> Option < V > where K : Borrow < Q > , { FxIndexMap :: swap_remove (self , k) } #[inline (always)] fn filter_map_collect < T > (& self , mut f : impl FnMut (& K , & V) -> Option < T >) -> Vec < T > { self . iter () . filter_map (move | (k , v) | f (k , v)) . collect () } #[inline (always)] fn get_or < E > (& self , k : K , vacant : impl FnOnce () -> Result < V , E >) -> Result < & V , E > { match self . get (& k) { Some (v) => Ok (v) , None => { vacant () ? ; bug ! ("The CTFE machine shouldn't ever need to extend the alloc_map when reading") } } } #[inline (always)] fn get_mut_or < E > (& mut self , k : K , vacant : impl FnOnce () -> Result < V , E >) -> Result < & mut V , E > { match self . entry (k) { IndexEntry :: Occupied (e) => Ok (e . into_mut ()) , IndexEntry :: Vacant (e) => { let v = vacant () ? ; Ok (e . insert (v)) } } } }}}
mkitem!{pub type CompileTimeInterpCx < 'tcx > = InterpCx < 'tcx , CompileTimeMachine < 'tcx > > ;}
mkitem!{mkenum!{#[derive (Debug , PartialEq , Eq , Copy , Clone)] pub enum MemoryKind { Heap { #[doc = " Indicates whether `make_global` was called on this allocation."] #[doc = " If this is `true`, the allocation must be immutable."] was_made_global : bool , } , }}}
mkitem!{mkimpl!{impl fmt :: Display for MemoryKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { MemoryKind :: Heap { was_made_global } => { write ! (f , "heap allocation{}" , if * was_made_global { " (made global)" } else { "" }) } } } }}}
mkitem!{mkimpl!{impl interpret :: MayLeak for MemoryKind { #[inline (always)] fn may_leak (self) -> bool { match self { MemoryKind :: Heap { was_made_global } => was_made_global , } } }}}
mkitem!{mkimpl!{impl interpret :: MayLeak for ! { #[inline (always)] fn may_leak (self) -> bool { self } }}}
mkitem!{mkimpl!{impl < 'tcx > CompileTimeInterpCx < 'tcx > { fn location_triple_for_span (& self , span : Span) -> (Symbol , u32 , u32) { let topmost = span . ctxt () . outer_expn () . expansion_cause () . unwrap_or (span) ; let caller = self . tcx . sess . source_map () . lookup_char_pos (topmost . lo ()) ; use rustc_session :: RemapFileNameExt ; use rustc_session :: config :: RemapPathScopeComponents ; (Symbol :: intern (& caller . file . name . for_scope (self . tcx . sess , RemapPathScopeComponents :: DIAGNOSTICS) . to_string_lossy () ,) , u32 :: try_from (caller . line) . unwrap () , u32 :: try_from (caller . col_display) . unwrap () . checked_add (1) . unwrap () ,) } #[doc = " \"Intercept\" a function call, because we have something special to do for it."] #[doc = " All `#[rustc_do_not_const_check]` functions MUST be hooked here."] #[doc = " If this returns `Some` function, which may be `instance` or a different function with"] #[doc = " compatible arguments, then evaluation should continue with that function."] #[doc = " If this returns `None`, the function call has been handled and the function has returned."] fn hook_special_const_fn (& mut self , instance : ty :: Instance < 'tcx > , args : & [FnArg < 'tcx >] , _dest : & PlaceTy < 'tcx > , _ret : Option < mir :: BasicBlock > ,) -> InterpResult < 'tcx , Option < ty :: Instance < 'tcx > > > { let def_id = instance . def_id () ; if self . tcx . is_lang_item (def_id , LangItem :: PanicDisplay) || self . tcx . is_lang_item (def_id , LangItem :: BeginPanic) { let args = self . copy_fn_args (args) ; assert ! (args . len () == 1) ; let mut msg_place = self . deref_pointer (& args [0]) ? ; while msg_place . layout . ty . is_ref () { msg_place = self . deref_pointer (& msg_place) ? ; } let msg = Symbol :: intern (self . read_str (& msg_place) ?) ; let span = self . find_closest_untracked_caller_location () ; let (file , line , col) = self . location_triple_for_span (span) ; return Err (ConstEvalErrKind :: Panic { msg , file , line , col }) . into () ; } else if self . tcx . is_lang_item (def_id , LangItem :: PanicFmt) { let const_def_id = self . tcx . require_lang_item (LangItem :: ConstPanicFmt , self . tcx . span) ; let new_instance = ty :: Instance :: expect_resolve (* self . tcx , self . typing_env () , const_def_id , instance . args , self . cur_span () ,) ; return interp_ok (Some (new_instance)) ; } interp_ok (Some (instance)) } #[doc = " See documentation on the `ptr_guaranteed_cmp` intrinsic."] #[doc = " Returns `2` if the result is unknown."] #[doc = " Returns `1` if the pointers are guaranteed equal."] #[doc = " Returns `0` if the pointers are guaranteed inequal."] #[doc = ""] #[doc = " Note that this intrinsic is exposed on stable for comparison with null. In other words, any"] #[doc = " change to this function that affects comparison with null is insta-stable!"] fn guaranteed_cmp (& mut self , a : Scalar , b : Scalar) -> InterpResult < 'tcx , u8 > { interp_ok (match (a , b) { (Scalar :: Int (a) , Scalar :: Int (b)) => (a == b) as u8 , (Scalar :: Int (int) , Scalar :: Ptr (ptr , _)) | (Scalar :: Ptr (ptr , _) , Scalar :: Int (int)) => { let int = int . to_target_usize (* self . tcx) ; let offset_ptr = ptr . wrapping_offset (Size :: from_bytes (int . wrapping_neg ()) , self) ; if ! self . scalar_may_be_null (Scalar :: from_pointer (offset_ptr , self)) ? { 0 } else { 2 } } (Scalar :: Ptr (a , _) , Scalar :: Ptr (b , _)) => { let (a_prov , a_offset) = a . prov_and_relative_offset () ; let (b_prov , b_offset) = b . prov_and_relative_offset () ; let a_allocid = a_prov . alloc_id () ; let b_allocid = b_prov . alloc_id () ; let a_info = self . get_alloc_info (a_allocid) ; let b_info = self . get_alloc_info (b_allocid) ; if a_info . align > Align :: ONE && b_info . align > Align :: ONE { let min_align = Ord :: min (a_info . align . bytes () , b_info . align . bytes ()) ; let a_residue = a_offset . bytes () % min_align ; let b_residue = b_offset . bytes () % min_align ; if a_residue != b_residue { return interp_ok (0) ; } } if let (Some (GlobalAlloc :: Static (a_did)) , Some (GlobalAlloc :: Static (b_did))) = (self . tcx . try_get_global_alloc (a_allocid) , self . tcx . try_get_global_alloc (b_allocid) ,) { if a_allocid == b_allocid { debug_assert_eq ! (a_did , b_did , "different static item DefIds had same AllocId? {a_allocid:?} == {b_allocid:?}, {a_did:?} != {b_did:?}") ; (a_offset == b_offset) as u8 } else { debug_assert_ne ! (a_did , b_did , "same static item DefId had two different AllocIds? {a_allocid:?} != {b_allocid:?}, {a_did:?} == {b_did:?}") ; if a_offset < a_info . size && b_offset < b_info . size { 0 } else { 2 } } } else { 2 } } }) } }}}
mkitem!{mkimpl!{impl < 'tcx > CompileTimeMachine < 'tcx > { #[inline (always)] #[doc = " Find the first stack frame that is within the current crate, if any."] #[doc = " Otherwise, return the crate's HirId"] pub fn best_lint_scope (& self , tcx : TyCtxt < 'tcx >) -> hir :: HirId { self . stack . iter () . find_map (| frame | frame . lint_root (tcx)) . unwrap_or (CRATE_HIR_ID) } }}}
mkitem!{mkimpl!{impl < 'tcx > interpret :: Machine < 'tcx > for CompileTimeMachine < 'tcx > { compile_time_machine ! (<'tcx >) ; const PANIC_ON_ALLOC_FAIL : bool = false ; #[inline (always)] fn enforce_alignment (ecx : & InterpCx < 'tcx , Self >) -> bool { matches ! (ecx . machine . check_alignment , CheckAlignment :: Error) } #[inline (always)] fn enforce_validity (ecx : & InterpCx < 'tcx , Self > , layout : TyAndLayout < 'tcx >) -> bool { ecx . tcx . sess . opts . unstable_opts . extra_const_ub_checks || layout . is_uninhabited () } fn load_mir (ecx : & InterpCx < 'tcx , Self > , instance : ty :: InstanceKind < 'tcx > ,) -> & 'tcx mir :: Body < 'tcx > { match instance { ty :: InstanceKind :: Item (def) => ecx . tcx . mir_for_ctfe (def) , _ => ecx . tcx . instance_mir (instance) , } } fn find_mir_or_eval_fn (ecx : & mut InterpCx < 'tcx , Self > , orig_instance : ty :: Instance < 'tcx > , _abi : & FnAbi < 'tcx , Ty < 'tcx > > , args : & [FnArg < 'tcx >] , dest : & PlaceTy < 'tcx > , ret : Option < mir :: BasicBlock > , _unwind : mir :: UnwindAction ,) -> InterpResult < 'tcx , Option < (& 'tcx mir :: Body < 'tcx > , ty :: Instance < 'tcx >) > > { debug ! ("find_mir_or_eval_fn: {:?}" , orig_instance) ; let Some (instance) = ecx . hook_special_const_fn (orig_instance , args , dest , ret) ? else { return interp_ok (None) ; } ; if let ty :: InstanceKind :: Item (def) = instance . def { if ! ecx . tcx . is_const_fn (def) || ecx . tcx . has_attr (def , sym :: rustc_do_not_const_check) { throw_unsup_format ! ("calling non-const function `{}`" , instance) } } interp_ok (Some ((ecx . load_mir (instance . def , None) ? , orig_instance))) } fn panic_nounwind (ecx : & mut InterpCx < 'tcx , Self > , msg : & str) -> InterpResult < 'tcx > { let msg = Symbol :: intern (msg) ; let span = ecx . find_closest_untracked_caller_location () ; let (file , line , col) = ecx . location_triple_for_span (span) ; Err (ConstEvalErrKind :: Panic { msg , file , line , col }) . into () } fn call_intrinsic (ecx : & mut InterpCx < 'tcx , Self > , instance : ty :: Instance < 'tcx > , args : & [OpTy < 'tcx >] , dest : & PlaceTy < 'tcx , Self :: Provenance > , target : Option < mir :: BasicBlock > , _unwind : mir :: UnwindAction ,) -> InterpResult < 'tcx , Option < ty :: Instance < 'tcx > > > { if ecx . eval_intrinsic (instance , args , dest , target) ? { return interp_ok (None) ; } let intrinsic_name = ecx . tcx . item_name (instance . def_id ()) ; match intrinsic_name { sym :: ptr_guaranteed_cmp => { let a = ecx . read_scalar (& args [0]) ? ; let b = ecx . read_scalar (& args [1]) ? ; let cmp = ecx . guaranteed_cmp (a , b) ? ; ecx . write_scalar (Scalar :: from_u8 (cmp) , dest) ? ; } sym :: const_allocate => { let size = ecx . read_scalar (& args [0]) ? . to_target_usize (ecx) ? ; let align = ecx . read_scalar (& args [1]) ? . to_target_usize (ecx) ? ; let align = match Align :: from_bytes (align) { Ok (a) => a , Err (err) => throw_ub_custom ! (fluent :: const_eval_invalid_align_details , name = "const_allocate" , err_kind = err . diag_ident () , align = err . align ()) , } ; let ptr = ecx . allocate_ptr (Size :: from_bytes (size) , align , interpret :: MemoryKind :: Machine (MemoryKind :: Heap { was_made_global : false }) , AllocInit :: Uninit ,) ? ; ecx . write_pointer (ptr , dest) ? ; } sym :: const_deallocate => { let ptr = ecx . read_pointer (& args [0]) ? ; let size = ecx . read_scalar (& args [1]) ? . to_target_usize (ecx) ? ; let align = ecx . read_scalar (& args [2]) ? . to_target_usize (ecx) ? ; let size = Size :: from_bytes (size) ; let align = match Align :: from_bytes (align) { Ok (a) => a , Err (err) => throw_ub_custom ! (fluent :: const_eval_invalid_align_details , name = "const_deallocate" , err_kind = err . diag_ident () , align = err . align ()) , } ; let (alloc_id , _ , _) = ecx . ptr_get_alloc_id (ptr , 0) ? ; let is_allocated_in_another_const = matches ! (ecx . tcx . try_get_global_alloc (alloc_id) , Some (interpret :: GlobalAlloc :: Memory (_))) ; if ! is_allocated_in_another_const { ecx . deallocate_ptr (ptr , Some ((size , align)) , interpret :: MemoryKind :: Machine (MemoryKind :: Heap { was_made_global : false }) ,) ? ; } } sym :: const_make_global => { let ptr = ecx . read_pointer (& args [0]) ? ; ecx . make_const_heap_ptr_global (ptr) ? ; ecx . write_pointer (ptr , dest) ? ; } sym :: is_val_statically_known => ecx . write_scalar (Scalar :: from_bool (false) , dest) ? , sym :: assert_inhabited | sym :: assert_zero_valid | sym :: assert_mem_uninitialized_valid => { let ty = instance . args . type_at (0) ; let requirement = ValidityRequirement :: from_intrinsic (intrinsic_name) . unwrap () ; let should_panic = ! ecx . tcx . check_validity_requirement ((requirement , ecx . typing_env () . as_query_input (ty))) . map_err (| _ | err_inval ! (TooGeneric)) ? ; if should_panic { let layout = ecx . layout_of (ty) ? ; let msg = match requirement { _ if layout . is_uninhabited () => format ! ("aborted execution: attempted to instantiate uninhabited type `{ty}`") , ValidityRequirement :: Inhabited => bug ! ("handled earlier") , ValidityRequirement :: Zero => format ! ("aborted execution: attempted to zero-initialize type `{ty}`, which is invalid") , ValidityRequirement :: UninitMitigated0x01Fill => format ! ("aborted execution: attempted to leave type `{ty}` uninitialized, which is invalid") , ValidityRequirement :: Uninit => bug ! ("assert_uninit_valid doesn't exist") , } ; Self :: panic_nounwind (ecx , & msg) ? ; return interp_ok (None) ; } } _ => { if ecx . tcx . intrinsic (instance . def_id ()) . unwrap () . must_be_overridden { throw_unsup_format ! ("intrinsic `{intrinsic_name}` is not supported at compile-time") ; } return interp_ok (Some (ty :: Instance { def : ty :: InstanceKind :: Item (instance . def_id ()) , args : instance . args , })) ; } } ecx . return_to_block (target) ? ; interp_ok (None) } fn assert_panic (ecx : & mut InterpCx < 'tcx , Self > , msg : & AssertMessage < 'tcx > , _unwind : mir :: UnwindAction ,) -> InterpResult < 'tcx > { use rustc_middle :: mir :: AssertKind :: * ; let eval_to_int = | op | ecx . read_immediate (& ecx . eval_operand (op , None) ?) . map (| x | x . to_const_int ()) ; let err = match msg { BoundsCheck { len , index } => { let len = eval_to_int (len) ? ; let index = eval_to_int (index) ? ; BoundsCheck { len , index } } Overflow (op , l , r) => Overflow (* op , eval_to_int (l) ? , eval_to_int (r) ?) , OverflowNeg (op) => OverflowNeg (eval_to_int (op) ?) , DivisionByZero (op) => DivisionByZero (eval_to_int (op) ?) , RemainderByZero (op) => RemainderByZero (eval_to_int (op) ?) , ResumedAfterReturn (coroutine_kind) => ResumedAfterReturn (* coroutine_kind) , ResumedAfterPanic (coroutine_kind) => ResumedAfterPanic (* coroutine_kind) , ResumedAfterDrop (coroutine_kind) => ResumedAfterDrop (* coroutine_kind) , MisalignedPointerDereference { required , found } => MisalignedPointerDereference { required : eval_to_int (required) ? , found : eval_to_int (found) ? , } , NullPointerDereference => NullPointerDereference , InvalidEnumConstruction (source) => InvalidEnumConstruction (eval_to_int (source) ?) , } ; Err (ConstEvalErrKind :: AssertFailure (err)) . into () } fn binary_ptr_op (_ecx : & InterpCx < 'tcx , Self > , _bin_op : mir :: BinOp , _left : & ImmTy < 'tcx > , _right : & ImmTy < 'tcx > ,) -> InterpResult < 'tcx , ImmTy < 'tcx > > { throw_unsup_format ! ("pointer arithmetic or comparison is not supported at compile-time") ; } fn increment_const_eval_counter (ecx : & mut InterpCx < 'tcx , Self >) -> InterpResult < 'tcx > { if let Some (new_steps) = ecx . machine . num_evaluated_steps . checked_add (1) { let (limit , start) = if ecx . tcx . sess . opts . unstable_opts . tiny_const_eval_limit { (TINY_LINT_TERMINATOR_LIMIT , TINY_LINT_TERMINATOR_LIMIT) } else { (LINT_TERMINATOR_LIMIT , PROGRESS_INDICATOR_START) } ; ecx . machine . num_evaluated_steps = new_steps ; if new_steps == limit { let hir_id = ecx . machine . best_lint_scope (* ecx . tcx) ; let is_error = ecx . tcx . lint_level_at_node (rustc_session :: lint :: builtin :: LONG_RUNNING_CONST_EVAL , hir_id ,) . level . is_error () ; let span = ecx . cur_span () ; ecx . tcx . emit_node_span_lint (rustc_session :: lint :: builtin :: LONG_RUNNING_CONST_EVAL , hir_id , span , LongRunning { item_span : ecx . tcx . span } ,) ; if is_error { let guard = ecx . tcx . dcx () . span_delayed_bug (span , "The deny lint should have already errored") ; throw_inval ! (AlreadyReported (ReportedErrorInfo :: allowed_in_infallible (guard))) ; } } else if new_steps > start && new_steps . is_power_of_two () { let span = ecx . cur_span () ; ecx . tcx . dcx () . emit_warn (LongRunningWarn { span , item_span : ecx . tcx . span , force_duplicate : new_steps , }) ; } } interp_ok (()) } #[inline (always)] fn expose_provenance (_ecx : & InterpCx < 'tcx , Self > , _provenance : Self :: Provenance ,) -> InterpResult < 'tcx > { throw_unsup_format ! ("exposing pointers is not possible at compile-time") } #[inline (always)] fn init_frame (ecx : & mut InterpCx < 'tcx , Self > , frame : Frame < 'tcx > ,) -> InterpResult < 'tcx , Frame < 'tcx > > { if ! ecx . recursion_limit . value_within_limit (ecx . stack () . len () + 1) { throw_exhaust ! (StackFrameLimitReached) } else { interp_ok (frame) } } #[inline (always)] fn stack < 'a > (ecx : & 'a InterpCx < 'tcx , Self > ,) -> & 'a [Frame < 'tcx , Self :: Provenance , Self :: FrameExtra >] { & ecx . machine . stack } #[inline (always)] fn stack_mut < 'a > (ecx : & 'a mut InterpCx < 'tcx , Self > ,) -> & 'a mut Vec < Frame < 'tcx , Self :: Provenance , Self :: FrameExtra > > { & mut ecx . machine . stack } fn before_access_global (_tcx : TyCtxtAt < 'tcx > , machine : & Self , alloc_id : AllocId , alloc : ConstAllocation < 'tcx > , _static_def_id : Option < DefId > , is_write : bool ,) -> InterpResult < 'tcx > { let alloc = alloc . inner () ; if is_write { match alloc . mutability { Mutability :: Not => throw_ub ! (WriteToReadOnly (alloc_id)) , Mutability :: Mut => Err (ConstEvalErrKind :: ModifiedGlobal) . into () , } } else { if machine . can_access_mut_global == CanAccessMutGlobal :: Yes { interp_ok (()) } else if alloc . mutability == Mutability :: Mut { Err (ConstEvalErrKind :: ConstAccessesMutGlobal) . into () } else { assert_eq ! (alloc . mutability , Mutability :: Not) ; interp_ok (()) } } } fn retag_ptr_value (ecx : & mut InterpCx < 'tcx , Self > , _kind : mir :: RetagKind , val : & ImmTy < 'tcx , CtfeProvenance > ,) -> InterpResult < 'tcx , ImmTy < 'tcx , CtfeProvenance > > { if let ty :: Ref (_ , ty , mutbl) = val . layout . ty . kind () && * mutbl == Mutability :: Not && val . to_scalar_and_meta () . 0 . to_pointer (ecx) ? . provenance . is_some_and (| p | ! p . immutable ()) { let is_immutable = ty . is_freeze (* ecx . tcx , ecx . typing_env ()) ; let place = ecx . ref_to_mplace (val) ? ; let new_place = if is_immutable { place . map_provenance (CtfeProvenance :: as_immutable) } else { place . map_provenance (CtfeProvenance :: as_shared_ref) } ; interp_ok (ImmTy :: from_immediate (new_place . to_ref (ecx) , val . layout)) } else { interp_ok (val . clone ()) } } fn before_memory_write (_tcx : TyCtxtAt < 'tcx > , _machine : & mut Self , _alloc_extra : & mut Self :: AllocExtra , _ptr : Pointer < Option < Self :: Provenance > > , (_alloc_id , immutable) : (AllocId , bool) , range : AllocRange ,) -> InterpResult < 'tcx > { if range . size == Size :: ZERO { return interp_ok (()) ; } if immutable { return Err (ConstEvalErrKind :: WriteThroughImmutablePointer) . into () ; } interp_ok (()) } fn before_alloc_access (tcx : TyCtxtAt < 'tcx > , machine : & Self , alloc_id : AllocId ,) -> InterpResult < 'tcx > { if machine . stack . is_empty () { return interp_ok (()) ; } if Some (alloc_id) == machine . static_root_ids . map (| (id , _) | id) { return Err (ConstEvalErrKind :: RecursiveStatic) . into () ; } if machine . static_root_ids . is_some () { if let Some (GlobalAlloc :: Static (def_id)) = tcx . try_get_global_alloc (alloc_id) { if tcx . is_foreign_item (def_id) { throw_unsup ! (ExternStatic (def_id)) ; } tcx . eval_static_initializer (def_id) ? ; } } interp_ok (()) } fn cached_union_data_range < 'e > (ecx : & 'e mut InterpCx < 'tcx , Self > , ty : Ty < 'tcx > , compute_range : impl FnOnce () -> RangeSet ,) -> Cow < 'e , RangeSet > { if ecx . tcx . sess . opts . unstable_opts . extra_const_ub_checks { Cow :: Borrowed (ecx . machine . union_data_ranges . entry (ty) . or_insert_with (compute_range)) } else { Cow :: Owned (compute_range ()) } } fn get_default_alloc_params (& self) -> < Self :: Bytes as mir :: interpret :: AllocBytes > :: AllocParams { } }}}