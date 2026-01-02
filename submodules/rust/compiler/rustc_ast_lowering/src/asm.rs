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
mkuse!{use std :: fmt :: Write ;}
mkuse!{use rustc_ast :: * ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashMap , FxHashSet , FxIndexMap } ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def :: { DefKind , Res } ;}
mkuse!{use rustc_session :: parse :: feature_err ;}
mkuse!{use rustc_span :: { Span , sym } ;}
mkuse!{use rustc_target :: asm ;}
mkuse!{use super :: LoweringContext ;}
mkuse!{use super :: errors :: { AbiSpecifiedMultipleTimes , AttSyntaxOnlyX86 , ClobberAbiNotSupported , InlineAsmUnsupportedTarget , InvalidAbiClobberAbi , InvalidAsmTemplateModifierConst , InvalidAsmTemplateModifierLabel , InvalidAsmTemplateModifierRegClass , InvalidAsmTemplateModifierRegClassSub , InvalidAsmTemplateModifierSym , InvalidRegister , InvalidRegisterClass , RegisterClassOnlyClobber , RegisterClassOnlyClobberStable , RegisterConflict , } ;}
mkuse!{use crate :: { AllowReturnTypeNotation , ImplTraitContext , ImplTraitPosition , ParamMode , ResolverAstLoweringExt , fluent_generated as fluent , } ;}
mkitem!{mkimpl!{impl < 'a , 'hir > LoweringContext < 'a , 'hir > { pub (crate) fn lower_inline_asm (& mut self , sp : Span , asm : & InlineAsm ,) -> & 'hir hir :: InlineAsm < 'hir > { let asm_arch = if self . tcx . sess . opts . actually_rustdoc { None } else { self . tcx . sess . asm_arch } ; if asm_arch . is_none () && ! self . tcx . sess . opts . actually_rustdoc { self . dcx () . emit_err (InlineAsmUnsupportedTarget { span : sp }) ; } if let Some (asm_arch) = asm_arch { let is_stable = matches ! (asm_arch , asm :: InlineAsmArch :: X86 | asm :: InlineAsmArch :: X86_64 | asm :: InlineAsmArch :: Arm | asm :: InlineAsmArch :: AArch64 | asm :: InlineAsmArch :: Arm64EC | asm :: InlineAsmArch :: RiscV32 | asm :: InlineAsmArch :: RiscV64 | asm :: InlineAsmArch :: LoongArch32 | asm :: InlineAsmArch :: LoongArch64 | asm :: InlineAsmArch :: S390x) ; if ! is_stable && ! self . tcx . features () . asm_experimental_arch () { feature_err (& self . tcx . sess , sym :: asm_experimental_arch , sp , fluent :: ast_lowering_unstable_inline_assembly ,) . emit () ; } } let allow_experimental_reg = self . tcx . features () . asm_experimental_reg () ; if asm . options . contains (InlineAsmOptions :: ATT_SYNTAX) && ! matches ! (asm_arch , Some (asm :: InlineAsmArch :: X86 | asm :: InlineAsmArch :: X86_64)) && ! self . tcx . sess . opts . actually_rustdoc { self . dcx () . emit_err (AttSyntaxOnlyX86 { span : sp }) ; } if asm . options . contains (InlineAsmOptions :: MAY_UNWIND) && ! self . tcx . features () . asm_unwind () { feature_err (& self . tcx . sess , sym :: asm_unwind , sp , fluent :: ast_lowering_unstable_may_unwind ,) . emit () ; } let mut clobber_abis = FxIndexMap :: default () ; if let Some (asm_arch) = asm_arch { for (abi_name , abi_span) in & asm . clobber_abis { match asm :: InlineAsmClobberAbi :: parse (asm_arch , & self . tcx . sess . target , & self . tcx . sess . unstable_target_features , * abi_name ,) { Ok (abi) => { match clobber_abis . get (& abi) { Some ((prev_name , prev_sp)) => { let source_map = self . tcx . sess . source_map () ; let equivalent = source_map . span_to_snippet (* prev_sp) != source_map . span_to_snippet (* abi_span) ; self . dcx () . emit_err (AbiSpecifiedMultipleTimes { abi_span : * abi_span , prev_name : * prev_name , prev_span : * prev_sp , equivalent , }) ; } None => { clobber_abis . insert (abi , (* abi_name , * abi_span)) ; } } } Err (& []) => { self . dcx () . emit_err (ClobberAbiNotSupported { abi_span : * abi_span }) ; } Err (supported_abis) => { let mut abis = format ! ("`{}`" , supported_abis [0]) ; for m in & supported_abis [1 ..] { let _ = write ! (abis , ", `{m}`") ; } self . dcx () . emit_err (InvalidAbiClobberAbi { abi_span : * abi_span , supported_abis : abis , }) ; } } } } let sess = self . tcx . sess ; let mut operands : Vec < _ > = asm . operands . iter () . map (| (op , op_sp) | { let lower_reg = | & reg : & _ | match reg { InlineAsmRegOrRegClass :: Reg (reg) => { asm :: InlineAsmRegOrRegClass :: Reg (if let Some (asm_arch) = asm_arch { asm :: InlineAsmReg :: parse (asm_arch , reg) . unwrap_or_else (| error | { self . dcx () . emit_err (InvalidRegister { op_span : * op_sp , reg , error , }) ; asm :: InlineAsmReg :: Err }) } else { asm :: InlineAsmReg :: Err }) } InlineAsmRegOrRegClass :: RegClass (reg_class) => { asm :: InlineAsmRegOrRegClass :: RegClass (if let Some (asm_arch) = asm_arch { asm :: InlineAsmRegClass :: parse (asm_arch , reg_class) . unwrap_or_else (| supported_register_classes | { let mut register_classes = format ! ("`{}`" , supported_register_classes [0]) ; for m in & supported_register_classes [1 ..] { let _ = write ! (register_classes , ", `{m}`") ; } self . dcx () . emit_err (InvalidRegisterClass { op_span : * op_sp , reg_class , supported_register_classes : register_classes , }) ; asm :: InlineAsmRegClass :: Err } ,) } else { asm :: InlineAsmRegClass :: Err }) } } ; let op = match op { InlineAsmOperand :: In { reg , expr } => hir :: InlineAsmOperand :: In { reg : lower_reg (reg) , expr : self . lower_expr (expr) , } , InlineAsmOperand :: Out { reg , late , expr } => hir :: InlineAsmOperand :: Out { reg : lower_reg (reg) , late : * late , expr : expr . as_ref () . map (| expr | self . lower_expr (expr)) , } , InlineAsmOperand :: InOut { reg , late , expr } => hir :: InlineAsmOperand :: InOut { reg : lower_reg (reg) , late : * late , expr : self . lower_expr (expr) , } , InlineAsmOperand :: SplitInOut { reg , late , in_expr , out_expr } => { hir :: InlineAsmOperand :: SplitInOut { reg : lower_reg (reg) , late : * late , in_expr : self . lower_expr (in_expr) , out_expr : out_expr . as_ref () . map (| expr | self . lower_expr (expr)) , } } InlineAsmOperand :: Const { anon_const } => hir :: InlineAsmOperand :: Const { anon_const : self . lower_const_block (anon_const) , } , InlineAsmOperand :: Sym { sym } => { let static_def_id = self . resolver . get_partial_res (sym . id) . and_then (| res | res . full_res ()) . and_then (| res | match res { Res :: Def (DefKind :: Static { .. } , def_id) => Some (def_id) , _ => None , }) ; if let Some (def_id) = static_def_id { let path = self . lower_qpath (sym . id , & sym . qself , & sym . path , ParamMode :: Optional , AllowReturnTypeNotation :: No , ImplTraitContext :: Disallowed (ImplTraitPosition :: Path) , None ,) ; hir :: InlineAsmOperand :: SymStatic { path , def_id } } else { let expr = Expr { id : sym . id , kind : ExprKind :: Path (sym . qself . clone () , sym . path . clone ()) , span : * op_sp , attrs : AttrVec :: new () , tokens : None , } ; hir :: InlineAsmOperand :: SymFn { expr : self . lower_expr (& expr) } } } InlineAsmOperand :: Label { block } => { hir :: InlineAsmOperand :: Label { block : self . lower_block (block , false) } } } ; (op , self . lower_span (* op_sp)) }) . collect () ; for p in & asm . template { if let InlineAsmTemplatePiece :: Placeholder { operand_idx , modifier : Some (modifier) , span : placeholder_span , } = * p { let op_sp = asm . operands [operand_idx] . 1 ; match & operands [operand_idx] . 0 { hir :: InlineAsmOperand :: In { reg , .. } | hir :: InlineAsmOperand :: Out { reg , .. } | hir :: InlineAsmOperand :: InOut { reg , .. } | hir :: InlineAsmOperand :: SplitInOut { reg , .. } => { let class = reg . reg_class () ; if class == asm :: InlineAsmRegClass :: Err { continue ; } let valid_modifiers = class . valid_modifiers (asm_arch . unwrap ()) ; if ! valid_modifiers . contains (& modifier) { let sub = if ! valid_modifiers . is_empty () { let mut mods = format ! ("`{}`" , valid_modifiers [0]) ; for m in & valid_modifiers [1 ..] { let _ = write ! (mods , ", `{m}`") ; } InvalidAsmTemplateModifierRegClassSub :: SupportModifier { class_name : class . name () , modifiers : mods , } } else { InvalidAsmTemplateModifierRegClassSub :: DoesNotSupportModifier { class_name : class . name () , } } ; self . dcx () . emit_err (InvalidAsmTemplateModifierRegClass { placeholder_span , op_span : op_sp , sub , }) ; } } hir :: InlineAsmOperand :: Const { .. } => { self . dcx () . emit_err (InvalidAsmTemplateModifierConst { placeholder_span , op_span : op_sp , }) ; } hir :: InlineAsmOperand :: SymFn { .. } | hir :: InlineAsmOperand :: SymStatic { .. } => { self . dcx () . emit_err (InvalidAsmTemplateModifierSym { placeholder_span , op_span : op_sp , }) ; } hir :: InlineAsmOperand :: Label { .. } => { self . dcx () . emit_err (InvalidAsmTemplateModifierLabel { placeholder_span , op_span : op_sp , }) ; } } } } let mut used_input_regs = FxHashMap :: default () ; let mut used_output_regs = FxHashMap :: default () ; for (idx , & (ref op , op_sp)) in operands . iter () . enumerate () { if let Some (reg) = op . reg () { let reg_class = reg . reg_class () ; if reg_class == asm :: InlineAsmRegClass :: Err { continue ; } if reg_class . is_clobber_only (asm_arch . unwrap () , allow_experimental_reg) && ! op . is_clobber () { if allow_experimental_reg || reg_class . is_clobber_only (asm_arch . unwrap () , true) { self . dcx () . emit_err (RegisterClassOnlyClobber { op_span : op_sp , reg_class_name : reg_class . name () , }) ; } else { self . tcx . sess . create_feature_err (RegisterClassOnlyClobberStable { op_span : op_sp , reg_class_name : reg_class . name () , } , sym :: asm_experimental_reg ,) . emit () ; } continue ; } if let asm :: InlineAsmRegOrRegClass :: Reg (reg) = reg { let (input , output) = match op { hir :: InlineAsmOperand :: In { .. } => (true , false) , hir :: InlineAsmOperand :: Out { late , .. } => (! late , true) , hir :: InlineAsmOperand :: InOut { .. } | hir :: InlineAsmOperand :: SplitInOut { .. } => (true , true) , hir :: InlineAsmOperand :: Const { .. } | hir :: InlineAsmOperand :: SymFn { .. } | hir :: InlineAsmOperand :: SymStatic { .. } | hir :: InlineAsmOperand :: Label { .. } => { unreachable ! ("{op:?} is not a register operand") ; } } ; let mut skip = false ; let mut check = | used_regs : & mut FxHashMap < asm :: InlineAsmReg , usize > , input , r : asm :: InlineAsmReg | { match used_regs . entry (r) { Entry :: Occupied (o) => { if skip { return ; } skip = true ; let idx2 = * o . get () ; let (ref op2 , op_sp2) = operands [idx2] ; let in_out = match (op , op2) { (hir :: InlineAsmOperand :: In { .. } , hir :: InlineAsmOperand :: Out { late , .. } ,) | (hir :: InlineAsmOperand :: Out { late , .. } , hir :: InlineAsmOperand :: In { .. } ,) => { assert ! (!* late) ; let out_op_sp = if input { op_sp2 } else { op_sp } ; Some (out_op_sp) } _ => None , } ; let reg_str = | idx | -> & str { let (op , _) : & (InlineAsmOperand , Span) = & asm . operands [idx] ; if let Some (ast :: InlineAsmRegOrRegClass :: Reg (reg_sym)) = op . reg () { reg_sym . as_str () } else { unreachable ! ("{op:?} is not a register operand") ; } } ; self . dcx () . emit_err (RegisterConflict { op_span1 : op_sp , op_span2 : op_sp2 , reg1_name : reg_str (idx) , reg2_name : reg_str (idx2) , in_out , }) ; } Entry :: Vacant (v) => { if r == reg { v . insert (idx) ; } } } } ; let mut overlapping_with = vec ! [] ; reg . overlapping_regs (| r | { overlapping_with . push (r) ; }) ; for r in overlapping_with { if input { check (& mut used_input_regs , true , r) ; } if output { check (& mut used_output_regs , false , r) ; } } } } } let mut clobbered = FxHashSet :: default () ; for (abi , (_ , abi_span)) in clobber_abis { for & clobber in abi . clobbered_regs () { if clobbered . contains (& clobber) { continue ; } let mut overlapping_with = vec ! [] ; clobber . overlapping_regs (| reg | { overlapping_with . push (reg) ; }) ; let output_used = overlapping_with . iter () . any (| reg | used_output_regs . contains_key (& reg)) ; if ! output_used { operands . push ((hir :: InlineAsmOperand :: Out { reg : asm :: InlineAsmRegOrRegClass :: Reg (clobber) , late : true , expr : None , } , self . lower_span (abi_span) ,)) ; clobbered . insert (clobber) ; } } } if let Some ((_ , op_sp)) = operands . iter () . find (| (op , _) | matches ! (op , hir :: InlineAsmOperand :: Label { .. })) { let output_operand_used = operands . iter () . any (| (op , _) | { matches ! (op , hir :: InlineAsmOperand :: Out { expr : Some (_) , .. } | hir :: InlineAsmOperand :: InOut { .. } | hir :: InlineAsmOperand :: SplitInOut { out_expr : Some (_) , .. }) }) ; if output_operand_used && ! self . tcx . features () . asm_goto_with_outputs () { feature_err (sess , sym :: asm_goto_with_outputs , * op_sp , fluent :: ast_lowering_unstable_inline_assembly_label_operand_with_outputs ,) . emit () ; } } let operands = self . arena . alloc_from_iter (operands) ; let template = self . arena . alloc_from_iter (asm . template . iter () . cloned ()) ; let template_strs = self . arena . alloc_from_iter (asm . template_strs . iter () . map (| (sym , snippet , span) | (* sym , * snippet , self . lower_span (* span))) ,) ; let line_spans = self . arena . alloc_from_iter (asm . line_spans . iter () . map (| span | self . lower_span (* span))) ; let hir_asm = hir :: InlineAsm { asm_macro : asm . asm_macro , template , template_strs , operands , options : asm . options , line_spans , } ; self . arena . alloc (hir_asm) } }}}