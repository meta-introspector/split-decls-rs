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
mkuse!{use std :: cell :: RefCell ;}
mkuse!{use std :: collections :: BTreeMap ;}
mkuse!{use std :: ops :: { Deref , DerefMut } ;}
mkuse!{use std :: sync :: LazyLock ;}
mkuse!{use private :: Sealed ;}
mkuse!{use rustc_ast :: { AttrStyle , CRATE_NODE_ID , MetaItemLit , NodeId } ;}
mkuse!{use rustc_errors :: { Diag , Diagnostic , Level } ;}
mkuse!{use rustc_feature :: { AttributeTemplate , AttributeType } ;}
mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_hir :: lints :: { AttributeLint , AttributeLintKind } ;}
mkuse!{use rustc_hir :: { AttrPath , CRATE_HIR_ID , HirId } ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_span :: { ErrorGuaranteed , Span , Symbol } ;}
mkuse!{use crate :: AttributeParser ;}
mkuse!{use crate :: attributes :: allow_unstable :: { AllowConstFnUnstableParser , AllowInternalUnstableParser , UnstableFeatureBoundParser , } ;}
mkuse!{use crate :: attributes :: body :: CoroutineParser ;}
mkuse!{use crate :: attributes :: codegen_attrs :: { ColdParser , CoverageParser , ExportNameParser , ForceTargetFeatureParser , NakedParser , NoMangleParser , OptimizeParser , SanitizeParser , TargetFeatureParser , TrackCallerParser , UsedParser , } ;}
mkuse!{use crate :: attributes :: confusables :: ConfusablesParser ;}
mkuse!{use crate :: attributes :: crate_level :: { CrateNameParser , MoveSizeLimitParser , NoCoreParser , NoStdParser , PatternComplexityLimitParser , RecursionLimitParser , TypeLengthLimitParser , } ;}
mkuse!{use crate :: attributes :: deprecation :: DeprecationParser ;}
mkuse!{use crate :: attributes :: dummy :: DummyParser ;}
mkuse!{use crate :: attributes :: inline :: { InlineParser , RustcForceInlineParser } ;}
mkuse!{use crate :: attributes :: link_attrs :: { ExportStableParser , FfiConstParser , FfiPureParser , LinkNameParser , LinkOrdinalParser , LinkParser , LinkSectionParser , LinkageParser , StdInternalSymbolParser , } ;}
mkuse!{use crate :: attributes :: lint_helpers :: { AsPtrParser , AutomaticallyDerivedParser , PassByValueParser , PubTransparentParser , } ;}
mkuse!{use crate :: attributes :: loop_match :: { ConstContinueParser , LoopMatchParser } ;}
mkuse!{use crate :: attributes :: macro_attrs :: { AllowInternalUnsafeParser , MacroEscapeParser , MacroUseParser , } ;}
mkuse!{use crate :: attributes :: must_use :: MustUseParser ;}
mkuse!{use crate :: attributes :: no_implicit_prelude :: NoImplicitPreludeParser ;}
mkuse!{use crate :: attributes :: non_exhaustive :: NonExhaustiveParser ;}
mkuse!{use crate :: attributes :: path :: PathParser as PathAttributeParser ;}
mkuse!{use crate :: attributes :: proc_macro_attrs :: { ProcMacroAttributeParser , ProcMacroDeriveParser , ProcMacroParser , RustcBuiltinMacroParser , } ;}
mkuse!{use crate :: attributes :: prototype :: CustomMirParser ;}
mkuse!{use crate :: attributes :: repr :: { AlignParser , AlignStaticParser , ReprParser } ;}
mkuse!{use crate :: attributes :: rustc_internal :: { RustcLayoutScalarValidRangeEnd , RustcLayoutScalarValidRangeStart , RustcObjectLifetimeDefaultParser , } ;}
mkuse!{use crate :: attributes :: semantics :: MayDangleParser ;}
mkuse!{use crate :: attributes :: stability :: { BodyStabilityParser , ConstStabilityIndirectParser , ConstStabilityParser , StabilityParser , } ;}
mkuse!{use crate :: attributes :: test_attrs :: { IgnoreParser , ShouldPanicParser } ;}
mkuse!{use crate :: attributes :: traits :: { AllowIncoherentImplParser , CoherenceIsCoreParser , CoinductiveParser , ConstTraitParser , DenyExplicitImplParser , DoNotImplementViaObjectParser , FundamentalParser , MarkerParser , ParenSugarParser , PointeeParser , SkipDuringMethodDispatchParser , SpecializationTraitParser , TypeConstParser , UnsafeSpecializationMarkerParser , } ;}
mkuse!{use crate :: attributes :: transparency :: TransparencyParser ;}
mkuse!{use crate :: attributes :: { AttributeParser as _ , Combine , Single , WithoutArgs } ;}
mkuse!{use crate :: parser :: { ArgParser , PathParser } ;}
mkuse!{use crate :: session_diagnostics :: { AttributeParseError , AttributeParseErrorReason , UnknownMetaItem } ;}
mkuse!{use crate :: target_checking :: AllowedTargets ;}
mkitem!{type GroupType < S > = LazyLock < GroupTypeInner < S > > ;}
mkitem!{mkstruct!{pub (super) struct GroupTypeInner < S : Stage > { pub (super) accepters : BTreeMap < & 'static [Symbol] , Vec < GroupTypeInnerAccept < S > > > , pub (super) finalizers : Vec < FinalizeFn < S > > , }}}
mkitem!{mkstruct!{pub (super) struct GroupTypeInnerAccept < S : Stage > { pub (super) template : AttributeTemplate , pub (super) accept_fn : AcceptFn < S > , pub (super) allowed_targets : AllowedTargets , pub (super) attribute_type : AttributeType , }}}
mkitem!{type AcceptFn < S > = Box < dyn for < 'sess , 'a > Fn (& mut AcceptContext < '_ , 'sess , S > , & ArgParser < 'a >) + Send + Sync > ;}
mkitem!{type FinalizeFn < S > = Box < dyn Send + Sync + Fn (& mut FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > > ;}
mkitem!{macro_rules ! attribute_parsers { (pub (crate) static $ name : ident = [$ ($ names : ty) ,* $ (,) ?] ;) => { mod early { use super ::*; type Combine < T > = super :: Combine < T , Early >; type Single < T > = super :: Single < T , Early >; type WithoutArgs < T > = super :: WithoutArgs < T , Early >; attribute_parsers ! (@ [Early] pub (crate) static $ name = [$ ($ names) ,*] ;) ; } mod late { use super ::*; type Combine < T > = super :: Combine < T , Late >; type Single < T > = super :: Single < T , Late >; type WithoutArgs < T > = super :: WithoutArgs < T , Late >; attribute_parsers ! (@ [Late] pub (crate) static $ name = [$ ($ names) ,*] ;) ; } } ; (@ [$ stage : ty] pub (crate) static $ name : ident = [$ ($ names : ty) ,* $ (,) ?] ;) => { pub (crate) static $ name : GroupType <$ stage > = LazyLock :: new (|| { let mut accepts = BTreeMap ::< _ , Vec < GroupTypeInnerAccept <$ stage >>>:: new () ; let mut finalizes = Vec ::< FinalizeFn <$ stage >>:: new () ; $ ({ thread_local ! { static STATE_OBJECT : RefCell <$ names > = RefCell :: new (<$ names >:: default ()) ; } ; for (path , template , accept_fn) in <$ names >:: ATTRIBUTES { accepts . entry (* path) . or_default () . push (GroupTypeInnerAccept { template : * template , accept_fn : Box :: new (| cx , args | { STATE_OBJECT . with_borrow_mut (| s | { accept_fn (s , cx , args) }) }) , allowed_targets : <$ names as crate :: attributes :: AttributeParser <$ stage >>:: ALLOWED_TARGETS , attribute_type : <$ names as crate :: attributes :: AttributeParser <$ stage >>:: TYPE , }) ; } finalizes . push (Box :: new (| cx | { let state = STATE_OBJECT . take () ; state . finalize (cx) })) ; }) * GroupTypeInner { accepters : accepts , finalizers : finalizes } }) ; } ; }}
mkitem!{attribute_parsers ! (pub (crate) static ATTRIBUTE_PARSERS = [AlignParser , AlignStaticParser , BodyStabilityParser , ConfusablesParser , ConstStabilityParser , MacroUseParser , NakedParser , StabilityParser , UsedParser , Combine < AllowConstFnUnstableParser >, Combine < AllowInternalUnstableParser >, Combine < ForceTargetFeatureParser >, Combine < LinkParser >, Combine < ReprParser >, Combine < TargetFeatureParser >, Combine < UnstableFeatureBoundParser >, Single < CoverageParser >, Single < CrateNameParser >, Single < CustomMirParser >, Single < DeprecationParser >, Single < DummyParser >, Single < ExportNameParser >, Single < IgnoreParser >, Single < InlineParser >, Single < LinkNameParser >, Single < LinkOrdinalParser >, Single < LinkSectionParser >, Single < LinkageParser >, Single < MoveSizeLimitParser >, Single < MustUseParser >, Single < OptimizeParser >, Single < PathAttributeParser >, Single < PatternComplexityLimitParser >, Single < ProcMacroDeriveParser >, Single < RecursionLimitParser >, Single < RustcBuiltinMacroParser >, Single < RustcForceInlineParser >, Single < RustcLayoutScalarValidRangeEnd >, Single < RustcLayoutScalarValidRangeStart >, Single < RustcObjectLifetimeDefaultParser >, Single < SanitizeParser >, Single < ShouldPanicParser >, Single < SkipDuringMethodDispatchParser >, Single < TransparencyParser >, Single < TypeLengthLimitParser >, Single < WithoutArgs < AllowIncoherentImplParser >>, Single < WithoutArgs < AllowInternalUnsafeParser >>, Single < WithoutArgs < AsPtrParser >>, Single < WithoutArgs < AutomaticallyDerivedParser >>, Single < WithoutArgs < CoherenceIsCoreParser >>, Single < WithoutArgs < CoinductiveParser >>, Single < WithoutArgs < ColdParser >>, Single < WithoutArgs < ConstContinueParser >>, Single < WithoutArgs < ConstStabilityIndirectParser >>, Single < WithoutArgs < ConstTraitParser >>, Single < WithoutArgs < CoroutineParser >>, Single < WithoutArgs < DenyExplicitImplParser >>, Single < WithoutArgs < DoNotImplementViaObjectParser >>, Single < WithoutArgs < ExportStableParser >>, Single < WithoutArgs < FfiConstParser >>, Single < WithoutArgs < FfiPureParser >>, Single < WithoutArgs < FundamentalParser >>, Single < WithoutArgs < LoopMatchParser >>, Single < WithoutArgs < MacroEscapeParser >>, Single < WithoutArgs < MarkerParser >>, Single < WithoutArgs < MayDangleParser >>, Single < WithoutArgs < NoCoreParser >>, Single < WithoutArgs < NoImplicitPreludeParser >>, Single < WithoutArgs < NoMangleParser >>, Single < WithoutArgs < NoStdParser >>, Single < WithoutArgs < NonExhaustiveParser >>, Single < WithoutArgs < ParenSugarParser >>, Single < WithoutArgs < PassByValueParser >>, Single < WithoutArgs < PointeeParser >>, Single < WithoutArgs < ProcMacroAttributeParser >>, Single < WithoutArgs < ProcMacroParser >>, Single < WithoutArgs < PubTransparentParser >>, Single < WithoutArgs < SpecializationTraitParser >>, Single < WithoutArgs < StdInternalSymbolParser >>, Single < WithoutArgs < TrackCallerParser >>, Single < WithoutArgs < TypeConstParser >>, Single < WithoutArgs < UnsafeSpecializationMarkerParser >>,] ;) ;}
mkmod!{private, { 
                getname!(private);
                getsrc!(private);
                getpath!(private);
                get_deps!(private);
                get_crates!(private);
                mkinclude!(private);
                mkitem!{mktrait!{pub trait Sealed { }}}
mkitem!{mkimpl!{impl Sealed for super :: Early { }}}
mkitem!{mkimpl!{impl Sealed for super :: Late { }}} 
            }}
mkitem!{mktrait!{# [allow (private_interfaces)] pub trait Stage : Sized + 'static + Sealed { type Id : Copy ; fn parsers () -> & 'static GroupType < Self > ; fn emit_err < 'sess > (& self , sess : & 'sess Session , diag : impl for < 'x > Diagnostic < 'x > ,) -> ErrorGuaranteed ; fn should_emit (& self) -> ShouldEmit ; fn id_is_crate_root (id : Self :: Id) -> bool ; }}}
mkitem!{mkimpl!{# [allow (private_interfaces)] impl Stage for Early { type Id = NodeId ; fn parsers () -> & 'static GroupType < Self > { & early :: ATTRIBUTE_PARSERS } fn emit_err < 'sess > (& self , sess : & 'sess Session , diag : impl for < 'x > Diagnostic < 'x > ,) -> ErrorGuaranteed { self . should_emit () . emit_err (sess . dcx () . create_err (diag)) } fn should_emit (& self) -> ShouldEmit { self . emit_errors } fn id_is_crate_root (id : Self :: Id) -> bool { id == CRATE_NODE_ID } }}}
mkitem!{mkimpl!{# [allow (private_interfaces)] impl Stage for Late { type Id = HirId ; fn parsers () -> & 'static GroupType < Self > { & late :: ATTRIBUTE_PARSERS } fn emit_err < 'sess > (& self , tcx : & 'sess Session , diag : impl for < 'x > Diagnostic < 'x > ,) -> ErrorGuaranteed { tcx . dcx () . emit_err (diag) } fn should_emit (& self) -> ShouldEmit { ShouldEmit :: ErrorsAndLints } fn id_is_crate_root (id : Self :: Id) -> bool { id == CRATE_HIR_ID } }}}
mkitem!{mkstruct!{# [doc = " used when parsing attributes for miscellaneous things *before* ast lowering"] pub struct Early { # [doc = " Whether to emit errors or delay them as a bug"] # [doc = " For most attributes, the attribute will be parsed again in the `Late` stage and in this case the errors should be delayed"] # [doc = " But for some, such as `cfg`, the attribute will be removed before the `Late` stage so errors must be emitted"] pub emit_errors : ShouldEmit , }}}
mkitem!{mkstruct!{# [doc = " used when parsing attributes during ast lowering"] pub struct Late ;}}
mkitem!{mkstruct!{# [doc = " Context given to every attribute parser when accepting"] # [doc = ""] # [doc = " Gives [`AttributeParser`]s enough information to create errors, for example."] pub struct AcceptContext < 'f , 'sess , S : Stage > { pub (crate) shared : SharedContext < 'f , 'sess , S > , # [doc = " The span of the attribute currently being parsed"] pub (crate) attr_span : Span , # [doc = " Whether it is an inner or outer attribute"] pub (crate) attr_style : AttrStyle , # [doc = " The expected structure of the attribute."] # [doc = ""] # [doc = " Used in reporting errors to give a hint to users what the attribute *should* look like."] pub (crate) template : & 'f AttributeTemplate , # [doc = " The name of the attribute we're currently accepting."] pub (crate) attr_path : AttrPath , }}}
mkitem!{mkimpl!{impl < 'f , 'sess : 'f , S : Stage > SharedContext < 'f , 'sess , S > { pub (crate) fn emit_err (& self , diag : impl for < 'x > Diagnostic < 'x >) -> ErrorGuaranteed { self . stage . emit_err (& self . sess , diag) } # [doc = " Emit a lint. This method is somewhat special, since lints emitted during attribute parsing"] # [doc = " must be delayed until after HIR is built. This method will take care of the details of"] # [doc = " that."] pub (crate) fn emit_lint (& mut self , lint : AttributeLintKind , span : Span) { if ! matches ! (self . stage . should_emit () , ShouldEmit :: ErrorsAndLints | ShouldEmit :: EarlyFatal { also_emit_lints : true }) { return ; } let id = self . target_id ; (self . emit_lint) (AttributeLint { id , span , kind : lint }) ; } pub (crate) fn warn_unused_duplicate (& mut self , used_span : Span , unused_span : Span) { self . emit_lint (AttributeLintKind :: UnusedDuplicate { this : unused_span , other : used_span , warning : false , } , unused_span ,) } pub (crate) fn warn_unused_duplicate_future_error (& mut self , used_span : Span , unused_span : Span ,) { self . emit_lint (AttributeLintKind :: UnusedDuplicate { this : unused_span , other : used_span , warning : true , } , unused_span ,) } }}}
mkitem!{mkimpl!{impl < 'f , 'sess : 'f , S : Stage > AcceptContext < 'f , 'sess , S > { pub (crate) fn unknown_key (& self , span : Span , found : String , options : & 'static [& 'static str] ,) -> ErrorGuaranteed { self . emit_err (UnknownMetaItem { span , item : found , expected : options }) } # [doc = " error that a string literal was expected."] # [doc = " You can optionally give the literal you did find (which you found not to be a string literal)"] # [doc = " which can make better errors. For example, if the literal was a byte string it will suggest"] # [doc = " removing the `b` prefix."] pub (crate) fn expected_string_literal (& self , span : Span , actual_literal : Option < & MetaItemLit > ,) -> ErrorGuaranteed { self . emit_err (AttributeParseError { span , attr_span : self . attr_span , template : self . template . clone () , attribute : self . attr_path . clone () , reason : AttributeParseErrorReason :: ExpectedStringLiteral { byte_string : actual_literal . and_then (| i | { i . kind . is_bytestr () . then (| | self . sess () . source_map () . start_point (i . span)) }) , } , attr_style : self . attr_style , }) } pub (crate) fn expected_integer_literal (& self , span : Span) -> ErrorGuaranteed { self . emit_err (AttributeParseError { span , attr_span : self . attr_span , template : self . template . clone () , attribute : self . attr_path . clone () , reason : AttributeParseErrorReason :: ExpectedIntegerLiteral , attr_style : self . attr_style , }) } pub (crate) fn expected_list (& self , span : Span) -> ErrorGuaranteed { self . emit_err (AttributeParseError { span , attr_span : self . attr_span , template : self . template . clone () , attribute : self . attr_path . clone () , reason : AttributeParseErrorReason :: ExpectedList , attr_style : self . attr_style , }) } pub (crate) fn expected_no_args (& self , args_span : Span) -> ErrorGuaranteed { self . emit_err (AttributeParseError { span : args_span , attr_span : self . attr_span , template : self . template . clone () , attribute : self . attr_path . clone () , reason : AttributeParseErrorReason :: ExpectedNoArgs , attr_style : self . attr_style , }) } # [doc = " emit an error that a `name` was expected here"] pub (crate) fn expected_identifier (& self , span : Span) -> ErrorGuaranteed { self . emit_err (AttributeParseError { span , attr_span : self . attr_span , template : self . template . clone () , attribute : self . attr_path . clone () , reason : AttributeParseErrorReason :: ExpectedIdentifier , attr_style : self . attr_style , }) } # [doc = " emit an error that a `name = value` pair was expected at this span. The symbol can be given for"] # [doc = " a nicer error message talking about the specific name that was found lacking a value."] pub (crate) fn expected_name_value (& self , span : Span , name : Option < Symbol >) -> ErrorGuaranteed { self . emit_err (AttributeParseError { span , attr_span : self . attr_span , template : self . template . clone () , attribute : self . attr_path . clone () , reason : AttributeParseErrorReason :: ExpectedNameValue (name) , attr_style : self . attr_style , }) } # [doc = " emit an error that a `name = value` pair was found where that name was already seen."] pub (crate) fn duplicate_key (& self , span : Span , key : Symbol) -> ErrorGuaranteed { self . emit_err (AttributeParseError { span , attr_span : self . attr_span , template : self . template . clone () , attribute : self . attr_path . clone () , reason : AttributeParseErrorReason :: DuplicateKey (key) , attr_style : self . attr_style , }) } # [doc = " an error that should be emitted when a [`MetaItemOrLitParser`](crate::parser::MetaItemOrLitParser)"] # [doc = " was expected *not* to be a literal, but instead a meta item."] pub (crate) fn unexpected_literal (& self , span : Span) -> ErrorGuaranteed { self . emit_err (AttributeParseError { span , attr_span : self . attr_span , template : self . template . clone () , attribute : self . attr_path . clone () , reason : AttributeParseErrorReason :: UnexpectedLiteral , attr_style : self . attr_style , }) } pub (crate) fn expected_single_argument (& self , span : Span) -> ErrorGuaranteed { self . emit_err (AttributeParseError { span , attr_span : self . attr_span , template : self . template . clone () , attribute : self . attr_path . clone () , reason : AttributeParseErrorReason :: ExpectedSingleArgument , attr_style : self . attr_style , }) } pub (crate) fn expected_at_least_one_argument (& self , span : Span) -> ErrorGuaranteed { self . emit_err (AttributeParseError { span , attr_span : self . attr_span , template : self . template . clone () , attribute : self . attr_path . clone () , reason : AttributeParseErrorReason :: ExpectedAtLeastOneArgument , attr_style : self . attr_style , }) } # [doc = " produces an error along the lines of `expected one of [foo, meow]`"] pub (crate) fn expected_specific_argument (& self , span : Span , possibilities : & [Symbol] ,) -> ErrorGuaranteed { self . emit_err (AttributeParseError { span , attr_span : self . attr_span , template : self . template . clone () , attribute : self . attr_path . clone () , reason : AttributeParseErrorReason :: ExpectedSpecificArgument { possibilities , strings : false , list : false , } , attr_style : self . attr_style , }) } # [doc = " produces an error along the lines of `expected one of [foo, meow] as an argument`."] # [doc = " i.e. slightly different wording to [`expected_specific_argument`](Self::expected_specific_argument)."] pub (crate) fn expected_specific_argument_and_list (& self , span : Span , possibilities : & [Symbol] ,) -> ErrorGuaranteed { self . emit_err (AttributeParseError { span , attr_span : self . attr_span , template : self . template . clone () , attribute : self . attr_path . clone () , reason : AttributeParseErrorReason :: ExpectedSpecificArgument { possibilities , strings : false , list : true , } , attr_style : self . attr_style , }) } # [doc = " produces an error along the lines of `expected one of [\"foo\", \"meow\"]`"] pub (crate) fn expected_specific_argument_strings (& self , span : Span , possibilities : & [Symbol] ,) -> ErrorGuaranteed { self . emit_err (AttributeParseError { span , attr_span : self . attr_span , template : self . template . clone () , attribute : self . attr_path . clone () , reason : AttributeParseErrorReason :: ExpectedSpecificArgument { possibilities , strings : true , list : false , } , attr_style : self . attr_style , }) } pub (crate) fn warn_empty_attribute (& mut self , span : Span) { self . emit_lint (AttributeLintKind :: EmptyAttribute { first_span : span } , span) ; } }}}
mkitem!{mkimpl!{impl < 'f , 'sess , S : Stage > Deref for AcceptContext < 'f , 'sess , S > { type Target = SharedContext < 'f , 'sess , S > ; fn deref (& self) -> & Self :: Target { & self . shared } }}}
mkitem!{mkimpl!{impl < 'f , 'sess , S : Stage > DerefMut for AcceptContext < 'f , 'sess , S > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . shared } }}}
mkitem!{mkstruct!{# [doc = " Context given to every attribute parser during finalization."] # [doc = ""] # [doc = " Gives [`AttributeParser`](crate::attributes::AttributeParser)s enough information to create"] # [doc = " errors, for example."] pub struct SharedContext < 'p , 'sess , S : Stage > { # [doc = " The parse context, gives access to the session and the"] # [doc = " diagnostics context."] pub (crate) cx : & 'p mut AttributeParser < 'sess , S > , # [doc = " The span of the syntactical component this attribute was applied to"] pub (crate) target_span : Span , # [doc = " The id ([`NodeId`] if `S` is `Early`, [`HirId`] if `S` is `Late`) of the syntactical component this attribute was applied to"] pub (crate) target_id : S :: Id , pub (crate) emit_lint : & 'p mut dyn FnMut (AttributeLint < S :: Id >) , }}}
mkitem!{mkstruct!{# [doc = " Context given to every attribute parser during finalization."] # [doc = ""] # [doc = " Gives [`AttributeParser`](crate::attributes::AttributeParser)s enough information to create"] # [doc = " errors, for example."] pub (crate) struct FinalizeContext < 'p , 'sess , S : Stage > { pub (crate) shared : SharedContext < 'p , 'sess , S > , # [doc = " A list of all attribute on this syntax node."] # [doc = ""] # [doc = " Useful for compatibility checks with other attributes in [`finalize`](crate::attributes::AttributeParser::finalize)"] # [doc = ""] # [doc = " Usually, you should use normal attribute parsing logic instead,"] # [doc = " especially when making a *denylist* of other attributes."] pub (crate) all_attrs : & 'p [PathParser < 'p >] , }}}
mkitem!{mkimpl!{impl < 'p , 'sess : 'p , S : Stage > Deref for FinalizeContext < 'p , 'sess , S > { type Target = SharedContext < 'p , 'sess , S > ; fn deref (& self) -> & Self :: Target { & self . shared } }}}
mkitem!{mkimpl!{impl < 'p , 'sess : 'p , S : Stage > DerefMut for FinalizeContext < 'p , 'sess , S > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . shared } }}}
mkitem!{mkimpl!{impl < 'p , 'sess : 'p , S : Stage > Deref for SharedContext < 'p , 'sess , S > { type Target = AttributeParser < 'sess , S > ; fn deref (& self) -> & Self :: Target { self . cx } }}}
mkitem!{mkimpl!{impl < 'p , 'sess : 'p , S : Stage > DerefMut for SharedContext < 'p , 'sess , S > { fn deref_mut (& mut self) -> & mut Self :: Target { self . cx } }}}
mkitem!{mkenum!{# [derive (PartialEq , Clone , Copy , Debug)] pub enum OmitDoc { Lower , Skip , }}}
mkitem!{mkenum!{# [derive (Copy , Clone , Debug)] pub enum ShouldEmit { # [doc = " The operations will emit errors, and lints, and errors are fatal."] # [doc = ""] # [doc = " Only relevant when early parsing, in late parsing equivalent to `ErrorsAndLints`."] # [doc = " Late parsing is never fatal, and instead tries to emit as many diagnostics as possible."] EarlyFatal { also_emit_lints : bool } , # [doc = " The operation will emit errors and lints."] # [doc = " This is usually what you need."] ErrorsAndLints , # [doc = " The operation will emit *not* errors and lints."] # [doc = " Use this if you are *sure* that this operation will be called at a different time with `ShouldEmit::ErrorsAndLints`."] Nothing , }}}
mkitem!{mkimpl!{impl ShouldEmit { pub (crate) fn emit_err (& self , diag : Diag < '_ >) -> ErrorGuaranteed { match self { ShouldEmit :: EarlyFatal { .. } if diag . level () == Level :: DelayedBug => diag . emit () , ShouldEmit :: EarlyFatal { .. } => diag . upgrade_to_fatal () . emit () , ShouldEmit :: ErrorsAndLints => diag . emit () , ShouldEmit :: Nothing => diag . delay_as_bug () , } } }}}