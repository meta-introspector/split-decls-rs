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
mkuse!{use rustc_abi :: { FieldIdx , VariantIdx } ;}
mkuse!{use rustc_middle :: mir :: interpret :: Scalar ;}
mkuse!{use rustc_middle :: mir :: * ;}
mkuse!{use rustc_middle :: thir :: * ;}
mkuse!{use rustc_middle :: ty ;}
mkuse!{use rustc_middle :: ty :: cast :: mir_cast_kind ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use rustc_span :: source_map :: Spanned ;}
mkuse!{use super :: { PResult , ParseCtxt , parse_by_kind } ;}
mkuse!{use crate :: builder :: custom :: ParseError ;}
mkuse!{use crate :: builder :: expr :: as_constant :: as_constant_inner ;}
mkitem!{mkimpl!{impl < 'a , 'tcx > ParseCtxt < 'a , 'tcx > { pub (crate) fn parse_statement (& self , expr_id : ExprId) -> PResult < StatementKind < 'tcx > > { parse_by_kind ! (self , expr_id , _ , "statement" , @ call (mir_storage_live , args) => { Ok (StatementKind :: StorageLive (self . parse_local (args [0]) ?)) } , @ call (mir_storage_dead , args) => { Ok (StatementKind :: StorageDead (self . parse_local (args [0]) ?)) } , @ call (mir_assume , args) => { let op = self . parse_operand (args [0]) ?; Ok (StatementKind :: Intrinsic (Box :: new (NonDivergingIntrinsic :: Assume (op)))) } , @ call (mir_deinit , args) => { Ok (StatementKind :: Deinit (Box :: new (self . parse_place (args [0]) ?))) } , @ call (mir_retag , args) => { Ok (StatementKind :: Retag (RetagKind :: Default , Box :: new (self . parse_place (args [0]) ?))) } , @ call (mir_set_discriminant , args) => { let place = self . parse_place (args [0]) ?; let var = self . parse_integer_literal (args [1]) ? as u32 ; Ok (StatementKind :: SetDiscriminant { place : Box :: new (place) , variant_index : VariantIdx :: from_u32 (var) , }) } , ExprKind :: Assign { lhs , rhs } => { let lhs = self . parse_place (* lhs) ?; let rhs = self . parse_rvalue (* rhs) ?; Ok (StatementKind :: Assign (Box :: new ((lhs , rhs)))) } ,) } pub (crate) fn parse_terminator (& self , expr_id : ExprId) -> PResult < TerminatorKind < 'tcx > > { parse_by_kind ! (self , expr_id , expr , "terminator" , @ call (mir_return , _args) => { Ok (TerminatorKind :: Return) } , @ call (mir_goto , args) => { Ok (TerminatorKind :: Goto { target : self . parse_block (args [0]) ? }) } , @ call (mir_unreachable , _args) => { Ok (TerminatorKind :: Unreachable) } , @ call (mir_unwind_resume , _args) => { Ok (TerminatorKind :: UnwindResume) } , @ call (mir_unwind_terminate , args) => { Ok (TerminatorKind :: UnwindTerminate (self . parse_unwind_terminate_reason (args [0]) ?)) } , @ call (mir_drop , args) => { Ok (TerminatorKind :: Drop { place : self . parse_place (args [0]) ?, target : self . parse_return_to (args [1]) ?, unwind : self . parse_unwind_action (args [2]) ?, replace : false , drop : None , async_fut : None , }) } , @ call (mir_call , args) => { self . parse_call (args) } , @ call (mir_tail_call , args) => { self . parse_tail_call (args) } , ExprKind :: Match { scrutinee , arms , .. } => { let discr = self . parse_operand (* scrutinee) ?; self . parse_match (arms , expr . span) . map (| t | TerminatorKind :: SwitchInt { discr , targets : t }) } ,) } fn parse_unwind_terminate_reason (& self , expr_id : ExprId) -> PResult < UnwindTerminateReason > { parse_by_kind ! (self , expr_id , _ , "unwind terminate reason" , @ variant (mir_unwind_terminate_reason , Abi) => { Ok (UnwindTerminateReason :: Abi) } , @ variant (mir_unwind_terminate_reason , InCleanup) => { Ok (UnwindTerminateReason :: InCleanup) } ,) } fn parse_unwind_action (& self , expr_id : ExprId) -> PResult < UnwindAction > { parse_by_kind ! (self , expr_id , _ , "unwind action" , @ call (mir_unwind_continue , _args) => { Ok (UnwindAction :: Continue) } , @ call (mir_unwind_unreachable , _args) => { Ok (UnwindAction :: Unreachable) } , @ call (mir_unwind_terminate , args) => { Ok (UnwindAction :: Terminate (self . parse_unwind_terminate_reason (args [0]) ?)) } , @ call (mir_unwind_cleanup , args) => { Ok (UnwindAction :: Cleanup (self . parse_block (args [0]) ?)) } ,) } fn parse_return_to (& self , expr_id : ExprId) -> PResult < BasicBlock > { parse_by_kind ! (self , expr_id , _ , "return block" , @ call (mir_return_to , args) => { self . parse_block (args [0]) } ,) } fn parse_match (& self , arms : & [ArmId] , span : Span) -> PResult < SwitchTargets > { let Some ((otherwise , rest)) = arms . split_last () else { return Err (ParseError { span , item_description : "no arms" . to_string () , expected : "at least one arm" . to_string () , }) ; } ; let otherwise = & self . thir [* otherwise] ; let PatKind :: Wild = otherwise . pattern . kind else { return Err (ParseError { span : otherwise . span , item_description : format ! ("{:?}" , otherwise . pattern . kind) , expected : "wildcard pattern" . to_string () , }) ; } ; let otherwise = self . parse_block (otherwise . body) ? ; let mut values = Vec :: new () ; let mut targets = Vec :: new () ; for arm in rest { let arm = & self . thir [* arm] ; let value = match arm . pattern . kind { PatKind :: Constant { value } => value , PatKind :: ExpandedConstant { ref subpattern , def_id : _ } if let PatKind :: Constant { value } = subpattern . kind => { value } _ => { return Err (ParseError { span : arm . pattern . span , item_description : format ! ("{:?}" , arm . pattern . kind) , expected : "constant pattern" . to_string () , }) ; } } ; values . push (value . valtree . unwrap_leaf () . to_bits_unchecked ()) ; targets . push (self . parse_block (arm . body) ?) ; } Ok (SwitchTargets :: new (values . into_iter () . zip (targets) , otherwise)) } fn parse_call (& self , args : & [ExprId]) -> PResult < TerminatorKind < 'tcx > > { let (destination , call) = parse_by_kind ! (self , args [0] , _ , "function call" , ExprKind :: Assign { lhs , rhs } => (* lhs , * rhs) ,) ; let destination = self . parse_place (destination) ? ; let target = self . parse_return_to (args [1]) ? ; let unwind = self . parse_unwind_action (args [2]) ? ; parse_by_kind ! (self , call , _ , "function call" , ExprKind :: Call { fun , args , from_hir_call , fn_span , .. } => { let fun = self . parse_operand (* fun) ?; let args = args . iter () . map (| arg | Ok (Spanned { node : self . parse_operand (* arg) ?, span : self . thir . exprs [* arg] . span })) . collect ::< PResult < Box < [_] >>> () ?; Ok (TerminatorKind :: Call { func : fun , args , destination , target : Some (target) , unwind , call_source : if * from_hir_call { CallSource :: Normal } else { CallSource :: OverloadedOperator } , fn_span : * fn_span , }) } ,) } fn parse_tail_call (& self , args : & [ExprId]) -> PResult < TerminatorKind < 'tcx > > { parse_by_kind ! (self , args [0] , _ , "tail call" , ExprKind :: Call { fun , args , fn_span , .. } => { let fun = self . parse_operand (* fun) ?; let args = args . iter () . map (| arg | Ok (Spanned { node : self . parse_operand (* arg) ?, span : self . thir . exprs [* arg] . span })) . collect ::< PResult < Box < [_] >>> () ?; Ok (TerminatorKind :: TailCall { func : fun , args , fn_span : * fn_span , }) } ,) } fn parse_rvalue (& self , expr_id : ExprId) -> PResult < Rvalue < 'tcx > > { parse_by_kind ! (self , expr_id , expr , "rvalue" , @ call (mir_discriminant , args) => self . parse_place (args [0]) . map (Rvalue :: Discriminant) , @ call (mir_cast_transmute , args) => { let source = self . parse_operand (args [0]) ?; Ok (Rvalue :: Cast (CastKind :: Transmute , source , expr . ty)) } , @ call (mir_cast_ptr_to_ptr , args) => { let source = self . parse_operand (args [0]) ?; Ok (Rvalue :: Cast (CastKind :: PtrToPtr , source , expr . ty)) } , @ call (mir_checked , args) => { parse_by_kind ! (self , args [0] , _ , "binary op" , ExprKind :: Binary { op , lhs , rhs } => { if let Some (op_with_overflow) = op . wrapping_to_overflowing () { Ok (Rvalue :: BinaryOp (op_with_overflow , Box :: new ((self . parse_operand (* lhs) ?, self . parse_operand (* rhs) ?)))) } else { Err (self . expr_error (expr_id , "No WithOverflow form of this operator")) } } ,) } , @ call (mir_offset , args) => { let ptr = self . parse_operand (args [0]) ?; let offset = self . parse_operand (args [1]) ?; Ok (Rvalue :: BinaryOp (BinOp :: Offset , Box :: new ((ptr , offset)))) } , @ call (mir_len , args) => Ok (Rvalue :: Len (self . parse_place (args [0]) ?)) , @ call (mir_ptr_metadata , args) => Ok (Rvalue :: UnaryOp (UnOp :: PtrMetadata , self . parse_operand (args [0]) ?)) , @ call (mir_copy_for_deref , args) => Ok (Rvalue :: CopyForDeref (self . parse_place (args [0]) ?)) , ExprKind :: Borrow { borrow_kind , arg } => Ok (Rvalue :: Ref (self . tcx . lifetimes . re_erased , * borrow_kind , self . parse_place (* arg) ?)) , ExprKind :: RawBorrow { mutability , arg } => Ok (Rvalue :: RawPtr ((* mutability) . into () , self . parse_place (* arg) ?)) , ExprKind :: Binary { op , lhs , rhs } => Ok (Rvalue :: BinaryOp (* op , Box :: new ((self . parse_operand (* lhs) ?, self . parse_operand (* rhs) ?)))) , ExprKind :: Unary { op , arg } => Ok (Rvalue :: UnaryOp (* op , self . parse_operand (* arg) ?)) , ExprKind :: Repeat { value , count } => Ok (Rvalue :: Repeat (self . parse_operand (* value) ?, * count)) , ExprKind :: Cast { source } => { let source = self . parse_operand (* source) ?; let source_ty = source . ty (self . body . local_decls () , self . tcx) ; let cast_kind = mir_cast_kind (source_ty , expr . ty) ; Ok (Rvalue :: Cast (cast_kind , source , expr . ty)) } , ExprKind :: Tuple { fields } => Ok (Rvalue :: Aggregate (Box :: new (AggregateKind :: Tuple) , fields . iter () . map (| e | self . parse_operand (* e)) . collect ::< Result < _ , _ >> () ?)) , ExprKind :: Array { fields } => { let elem_ty = expr . ty . builtin_index () . expect ("ty must be an array") ; Ok (Rvalue :: Aggregate (Box :: new (AggregateKind :: Array (elem_ty)) , fields . iter () . map (| e | self . parse_operand (* e)) . collect ::< Result < _ , _ >> () ?)) } , ExprKind :: Adt (box AdtExpr { adt_def , variant_index , args , fields , .. }) => { let is_union = adt_def . is_union () ; let active_field_index = is_union . then (|| fields [0] . name) ; Ok (Rvalue :: Aggregate (Box :: new (AggregateKind :: Adt (adt_def . did () , * variant_index , args , None , active_field_index)) , fields . iter () . map (| f | self . parse_operand (f . expr)) . collect ::< Result < _ , _ >> () ?)) } , _ => self . parse_operand (expr_id) . map (Rvalue :: Use) ,) } pub (crate) fn parse_operand (& self , expr_id : ExprId) -> PResult < Operand < 'tcx > > { parse_by_kind ! (self , expr_id , expr , "operand" , @ call (mir_move , args) => self . parse_place (args [0]) . map (Operand :: Move) , @ call (mir_static , args) => self . parse_static (args [0]) , @ call (mir_static_mut , args) => self . parse_static (args [0]) , ExprKind :: Literal { .. } | ExprKind :: NamedConst { .. } | ExprKind :: NonHirLiteral { .. } | ExprKind :: ZstLiteral { .. } | ExprKind :: ConstParam { .. } | ExprKind :: ConstBlock { .. } => { Ok (Operand :: Constant (Box :: new (as_constant_inner (expr , | _ | None , self . tcx)))) } , _ => self . parse_place (expr_id) . map (Operand :: Copy) ,) } fn parse_place (& self , expr_id : ExprId) -> PResult < Place < 'tcx > > { self . parse_place_inner (expr_id) . map (| (x , _) | x) } fn parse_place_inner (& self , expr_id : ExprId) -> PResult < (Place < 'tcx > , PlaceTy < 'tcx >) > { let (parent , proj) = parse_by_kind ! (self , expr_id , expr , "place" , @ call (mir_field , args) => { let (parent , place_ty) = self . parse_place_inner (args [0]) ?; let field = FieldIdx :: from_u32 (self . parse_integer_literal (args [1]) ? as u32) ; let field_ty = PlaceTy :: field_ty (self . tcx , place_ty . ty , place_ty . variant_index , field) ; let proj = PlaceElem :: Field (field , field_ty) ; let place = parent . project_deeper (& [proj] , self . tcx) ; return Ok ((place , PlaceTy :: from_ty (field_ty))) ; } , @ call (mir_variant , args) => { (args [0] , PlaceElem :: Downcast (None , VariantIdx :: from_u32 (self . parse_integer_literal (args [1]) ? as u32))) } , ExprKind :: Deref { arg } => { parse_by_kind ! (self , * arg , _ , "does not matter" , @ call (mir_make_place , args) => return self . parse_place_inner (args [0]) , _ => (* arg , PlaceElem :: Deref) ,) } , ExprKind :: Index { lhs , index } => (* lhs , PlaceElem :: Index (self . parse_local (* index) ?)) , ExprKind :: Field { lhs , name : field , .. } => (* lhs , PlaceElem :: Field (* field , expr . ty)) , _ => { let place = self . parse_local (expr_id) . map (Place :: from) ?; return Ok ((place , PlaceTy :: from_ty (expr . ty))) } ,) ; let (parent , ty) = self . parse_place_inner (parent) ? ; let place = parent . project_deeper (& [proj] , self . tcx) ; let ty = ty . projection_ty (self . tcx , proj) ; Ok ((place , ty)) } fn parse_local (& self , expr_id : ExprId) -> PResult < Local > { parse_by_kind ! (self , expr_id , _ , "local" , ExprKind :: VarRef { id } => Ok (self . local_map [id]) ,) } fn parse_block (& self , expr_id : ExprId) -> PResult < BasicBlock > { parse_by_kind ! (self , expr_id , _ , "basic block" , ExprKind :: VarRef { id } => Ok (self . block_map [id]) ,) } fn parse_static (& self , expr_id : ExprId) -> PResult < Operand < 'tcx > > { let expr_id = parse_by_kind ! (self , expr_id , _ , "static" , ExprKind :: Deref { arg } => * arg ,) ; parse_by_kind ! (self , expr_id , expr , "static" , ExprKind :: StaticRef { alloc_id , ty , .. } => { let const_val = ConstValue :: Scalar (Scalar :: from_pointer ((* alloc_id) . into () , & self . tcx)) ; let const_ = Const :: Val (const_val , * ty) ; Ok (Operand :: Constant (Box :: new (ConstOperand { span : expr . span , user_ty : None , const_ }))) } ,) } fn parse_integer_literal (& self , expr_id : ExprId) -> PResult < u128 > { parse_by_kind ! (self , expr_id , expr , "constant" , ExprKind :: Literal { .. } | ExprKind :: NamedConst { .. } | ExprKind :: NonHirLiteral { .. } | ExprKind :: ConstBlock { .. } => Ok ({ let value = as_constant_inner (expr , | _ | None , self . tcx) ; value . const_ . eval_bits (self . tcx , self . typing_env) }) ,) } }}}