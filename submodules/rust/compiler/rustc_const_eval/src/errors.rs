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
mkuse!{use std :: borrow :: Cow ;}
mkuse!{use std :: fmt :: Write ;}
mkuse!{use either :: Either ;}
mkuse!{use rustc_abi :: WrappingRange ;}
mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_errors :: { Diag , DiagArgValue , DiagMessage , Diagnostic , EmissionGuarantee , Level , MultiSpan , Subdiagnostic , } ;}
mkuse!{use rustc_hir :: ConstContext ;}
mkuse!{use rustc_macros :: { Diagnostic , LintDiagnostic , Subdiagnostic } ;}
mkuse!{use rustc_middle :: mir :: interpret :: { CtfeProvenance , ExpectedKind , InterpErrorKind , InvalidMetaKind , InvalidProgramInfo , Misalignment , Pointer , PointerKind , ResourceExhaustionInfo , UndefinedBehaviorInfo , UnsupportedOpInfo , ValidationErrorInfo , } ;}
mkuse!{use rustc_middle :: ty :: { self , Mutability , Ty } ;}
mkuse!{use rustc_span :: { Span , Symbol } ;}
mkuse!{use crate :: fluent_generated as fluent ;}
mkuse!{use crate :: interpret :: InternKind ;}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_dangling_ptr_in_final)] pub (crate) struct DanglingPtrInFinal { # [primary_span] pub span : Span , pub kind : InternKind , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_nested_static_in_thread_local)] pub (crate) struct NestedStaticInThreadLocal { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_mutable_ptr_in_final)] pub (crate) struct MutablePtrInFinal { # [primary_span] pub span : Span , pub kind : InternKind , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_const_heap_ptr_in_final)] # [note] pub (crate) struct ConstHeapPtrInFinal { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_partial_pointer_in_final)] # [note] pub (crate) struct PartialPtrInFinal { # [primary_span] pub span : Span , pub kind : InternKind , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_unstable_in_stable_exposed)] pub (crate) struct UnstableInStableExposed { pub gate : String , # [primary_span] pub span : Span , # [help (const_eval_is_function_call)] pub is_function_call : bool , # [doc = " Need to duplicate the field so that fluent also provides it as a variable..."] pub is_function_call2 : bool , # [suggestion (const_eval_unstable_sugg , code = "#[rustc_const_unstable(feature = \"...\", issue = \"...\")]\n" , applicability = "has-placeholders")] pub attr_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_thread_local_access , code = E0625)] pub (crate) struct ThreadLocalAccessErr { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_raw_ptr_to_int)] # [note] # [note (const_eval_note2)] pub (crate) struct RawPtrToIntErr { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_raw_ptr_comparison)] # [note] pub (crate) struct RawPtrComparisonErr { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_panic_non_str)] pub (crate) struct PanicNonStrErr { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_unallowed_fn_pointer_call)] pub (crate) struct UnallowedFnPointerCall { # [primary_span] pub span : Span , pub kind : ConstContext , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_unstable_const_fn)] pub (crate) struct UnstableConstFn { # [primary_span] pub span : Span , pub def_path : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_unstable_const_trait)] pub (crate) struct UnstableConstTrait { # [primary_span] pub span : Span , pub def_path : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_unstable_intrinsic)] pub (crate) struct UnstableIntrinsic { # [primary_span] pub span : Span , pub name : Symbol , pub feature : Symbol , # [suggestion (const_eval_unstable_intrinsic_suggestion , code = "#![feature({feature})]\n" , applicability = "machine-applicable")] pub suggestion : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_unmarked_const_item_exposed)] # [help] pub (crate) struct UnmarkedConstItemExposed { # [primary_span] pub span : Span , pub def_path : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_unmarked_intrinsic_exposed)] # [help] pub (crate) struct UnmarkedIntrinsicExposed { # [primary_span] pub span : Span , pub def_path : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_mutable_borrow_escaping , code = E0764)] # [note] # [note (const_eval_note2)] # [help] pub (crate) struct MutableBorrowEscaping { # [primary_span] # [label] pub span : Span , pub kind : ConstContext , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_non_const_fmt_macro_call , code = E0015)] pub (crate) struct NonConstFmtMacroCall { # [primary_span] pub span : Span , pub kind : ConstContext , pub non_or_conditionally : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_non_const_fn_call , code = E0015)] pub (crate) struct NonConstFnCall { # [primary_span] pub span : Span , pub def_path_str : String , pub def_descr : & 'static str , pub kind : ConstContext , pub non_or_conditionally : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_non_const_intrinsic)] pub (crate) struct NonConstIntrinsic { # [primary_span] pub span : Span , pub name : Symbol , pub kind : ConstContext , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_unallowed_op_in_const_context)] pub (crate) struct UnallowedOpInConstContext { # [primary_span] pub span : Span , pub msg : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_unallowed_heap_allocations , code = E0010)] pub (crate) struct UnallowedHeapAllocations { # [primary_span] # [label] pub span : Span , pub kind : ConstContext , # [note (const_eval_teach_note)] pub teach : bool , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_unallowed_inline_asm , code = E0015)] pub (crate) struct UnallowedInlineAsm { # [primary_span] pub span : Span , pub kind : ConstContext , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_interior_mutable_borrow_escaping , code = E0492)] # [note] # [note (const_eval_note2)] # [help] pub (crate) struct InteriorMutableBorrowEscaping { # [primary_span] # [label] pub span : Span , pub kind : ConstContext , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (const_eval_long_running)] # [note] pub struct LongRunning { # [help] pub item_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_long_running)] pub struct LongRunningWarn { # [primary_span] # [label] pub span : Span , # [help] pub item_span : Span , pub force_duplicate : usize , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (const_eval_non_const_impl)] pub (crate) struct NonConstImplNote { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Clone)] pub struct FrameNote { pub span : Span , pub times : i32 , pub where_ : & 'static str , pub instance : String , pub has_label : bool , }}}
mkitem!{mkimpl!{impl Subdiagnostic for FrameNote { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { diag . arg ("times" , self . times) ; diag . arg ("where_" , self . where_) ; diag . arg ("instance" , self . instance) ; let mut span : MultiSpan = self . span . into () ; if self . has_label && ! self . span . is_dummy () { span . push_span_label (self . span , fluent :: const_eval_frame_note_last) ; } let msg = diag . eagerly_translate (fluent :: const_eval_frame_note) ; diag . remove_arg ("times") ; diag . remove_arg ("where_") ; diag . remove_arg ("instance") ; diag . span_note (span , msg) ; } }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (const_eval_raw_bytes)] pub struct RawBytesNote { pub size : u64 , pub align : u64 , pub bytes : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_non_const_match_eq , code = E0015)] # [note] pub struct NonConstMatchEq < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , pub kind : ConstContext , pub non_or_conditionally : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_non_const_for_loop_into_iter , code = E0015)] pub struct NonConstForLoopIntoIter < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , pub kind : ConstContext , pub non_or_conditionally : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_non_const_question_branch , code = E0015)] pub struct NonConstQuestionBranch < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , pub kind : ConstContext , pub non_or_conditionally : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_non_const_question_from_residual , code = E0015)] pub struct NonConstQuestionFromResidual < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , pub kind : ConstContext , pub non_or_conditionally : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_non_const_try_block_from_output , code = E0015)] pub struct NonConstTryBlockFromOutput < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , pub kind : ConstContext , pub non_or_conditionally : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_non_const_await , code = E0015)] pub struct NonConstAwait < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , pub kind : ConstContext , pub non_or_conditionally : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_non_const_closure , code = E0015)] pub struct NonConstClosure { # [primary_span] pub span : Span , pub kind : ConstContext , # [subdiagnostic] pub note : Option < NonConstClosureNote > , pub non_or_conditionally : & 'static str , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub enum NonConstClosureNote { # [note (const_eval_closure_fndef_not_const)] FnDef { # [primary_span] span : Span , } , # [note (const_eval_fn_ptr_call)] FnPtr , # [note (const_eval_closure_call)] Closure , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [multipart_suggestion (const_eval_consider_dereferencing , applicability = "machine-applicable")] pub struct ConsiderDereferencing { pub deref : String , # [suggestion_part (code = "{deref}")] pub span : Span , # [suggestion_part (code = "{deref}")] pub rhs_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_non_const_operator , code = E0015)] pub struct NonConstOperator { # [primary_span] pub span : Span , pub kind : ConstContext , # [subdiagnostic] pub sugg : Option < ConsiderDereferencing > , pub non_or_conditionally : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_non_const_deref_coercion , code = E0015)] # [note] pub struct NonConstDerefCoercion < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , pub kind : ConstContext , pub target_ty : Ty < 'tcx > , # [note (const_eval_target_note)] pub deref_target : Option < Span > , pub non_or_conditionally : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (const_eval_live_drop , code = E0493)] pub struct LiveDrop < 'tcx > { # [primary_span] # [label] pub span : Span , pub kind : ConstContext , pub dropped_ty : Ty < 'tcx > , # [label (const_eval_dropped_at_label)] pub dropped_at : Span , }}}
mkitem!{mktrait!{pub trait ReportErrorExt { # [doc = " Returns the diagnostic message for this error."] fn diagnostic_message (& self) -> DiagMessage ; fn add_args < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) ; fn debug (self) -> String where Self : Sized , { ty :: tls :: with (move | tcx | { let dcx = tcx . dcx () ; let mut diag = dcx . struct_allow (DiagMessage :: Str (String :: new () . into ())) ; let message = self . diagnostic_message () ; self . add_args (& mut diag) ; let s = dcx . eagerly_translate_to_string (message , diag . args . iter ()) ; diag . cancel () ; s }) } }}}
mkitem!{mkimpl!{impl < 'a > ReportErrorExt for UndefinedBehaviorInfo < 'a > { fn diagnostic_message (& self) -> DiagMessage { use UndefinedBehaviorInfo :: * ; use crate :: fluent_generated :: * ; match self { Ub (msg) => msg . clone () . into () , Custom (x) => (x . msg) () , ValidationError (e) => e . diagnostic_message () , Unreachable => const_eval_unreachable , BoundsCheckFailed { .. } => const_eval_bounds_check_failed , DivisionByZero => const_eval_division_by_zero , RemainderByZero => const_eval_remainder_by_zero , DivisionOverflow => const_eval_division_overflow , RemainderOverflow => const_eval_remainder_overflow , PointerArithOverflow => const_eval_pointer_arithmetic_overflow , ArithOverflow { .. } => const_eval_overflow_arith , ShiftOverflow { .. } => const_eval_overflow_shift , InvalidMeta (InvalidMetaKind :: SliceTooBig) => const_eval_invalid_meta_slice , InvalidMeta (InvalidMetaKind :: TooBig) => const_eval_invalid_meta , UnterminatedCString (_) => const_eval_unterminated_c_string , PointerUseAfterFree (_ , _) => const_eval_pointer_use_after_free , PointerOutOfBounds { .. } => const_eval_pointer_out_of_bounds , DanglingIntPointer { addr : 0 , .. } => const_eval_dangling_null_pointer , DanglingIntPointer { .. } => const_eval_dangling_int_pointer , AlignmentCheckFailed { .. } => const_eval_alignment_check_failed , WriteToReadOnly (_) => const_eval_write_to_read_only , DerefFunctionPointer (_) => const_eval_deref_function_pointer , DerefVTablePointer (_) => const_eval_deref_vtable_pointer , DerefTypeIdPointer (_) => const_eval_deref_typeid_pointer , InvalidBool (_) => const_eval_invalid_bool , InvalidChar (_) => const_eval_invalid_char , InvalidTag (_) => const_eval_invalid_tag , InvalidFunctionPointer (_) => const_eval_invalid_function_pointer , InvalidVTablePointer (_) => const_eval_invalid_vtable_pointer , InvalidVTableTrait { .. } => const_eval_invalid_vtable_trait , InvalidStr (_) => const_eval_invalid_str , InvalidUninitBytes (None) => const_eval_invalid_uninit_bytes_unknown , InvalidUninitBytes (Some (_)) => const_eval_invalid_uninit_bytes , DeadLocal => const_eval_dead_local , ScalarSizeMismatch (_) => const_eval_scalar_size_mismatch , UninhabitedEnumVariantWritten (_) => const_eval_uninhabited_enum_variant_written , UninhabitedEnumVariantRead (_) => const_eval_uninhabited_enum_variant_read , InvalidNichedEnumVariantWritten { .. } => { const_eval_invalid_niched_enum_variant_written } AbiMismatchArgument { .. } => const_eval_incompatible_arg_types , AbiMismatchReturn { .. } => const_eval_incompatible_return_types , } } fn add_args < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { use UndefinedBehaviorInfo :: * ; match self { Ub (_) => { } Custom (custom) => { (custom . add_args) (& mut | name , value | { diag . arg (name , value) ; }) ; } ValidationError (e) => e . add_args (diag) , Unreachable | DivisionByZero | RemainderByZero | DivisionOverflow | RemainderOverflow | PointerArithOverflow | InvalidMeta (InvalidMetaKind :: SliceTooBig) | InvalidMeta (InvalidMetaKind :: TooBig) | InvalidUninitBytes (None) | DeadLocal | UninhabitedEnumVariantWritten (_) | UninhabitedEnumVariantRead (_) => { } ArithOverflow { intrinsic } => { diag . arg ("intrinsic" , intrinsic) ; } ShiftOverflow { intrinsic , shift_amount } => { diag . arg ("intrinsic" , intrinsic) ; diag . arg ("shift_amount" , match shift_amount { Either :: Left (v) => v . to_string () , Either :: Right (v) => v . to_string () , } ,) ; } BoundsCheckFailed { len , index } => { diag . arg ("len" , len) ; diag . arg ("index" , index) ; } UnterminatedCString (ptr) | InvalidFunctionPointer (ptr) | InvalidVTablePointer (ptr) => { diag . arg ("pointer" , ptr) ; } InvalidVTableTrait { expected_dyn_type , vtable_dyn_type } => { diag . arg ("expected_dyn_type" , expected_dyn_type . to_string ()) ; diag . arg ("vtable_dyn_type" , vtable_dyn_type . to_string ()) ; } PointerUseAfterFree (alloc_id , msg) => { diag . arg ("alloc_id" , alloc_id) . arg ("operation" , format ! ("{:?}" , msg)) ; } PointerOutOfBounds { alloc_id , alloc_size , ptr_offset , inbounds_size , msg } => { diag . arg ("alloc_size" , alloc_size . bytes ()) ; diag . arg ("pointer" , { let mut out = format ! ("{:?}" , alloc_id) ; if ptr_offset > 0 { write ! (out , "+{:#x}" , ptr_offset) . unwrap () ; } else if ptr_offset < 0 { write ! (out , "-{:#x}" , ptr_offset . unsigned_abs ()) . unwrap () ; } out }) ; diag . arg ("inbounds_size" , inbounds_size) ; diag . arg ("inbounds_size_is_neg" , inbounds_size < 0) ; diag . arg ("inbounds_size_abs" , inbounds_size . unsigned_abs ()) ; diag . arg ("ptr_offset" , ptr_offset) ; diag . arg ("ptr_offset_is_neg" , ptr_offset < 0) ; diag . arg ("ptr_offset_abs" , ptr_offset . unsigned_abs ()) ; diag . arg ("alloc_size_minus_ptr_offset" , alloc_size . bytes () . saturating_sub (ptr_offset as u64) ,) ; diag . arg ("operation" , format ! ("{:?}" , msg)) ; } DanglingIntPointer { addr , inbounds_size , msg } => { if addr != 0 { diag . arg ("pointer" , Pointer :: < Option < CtfeProvenance > > :: without_provenance (addr) . to_string () ,) ; } diag . arg ("inbounds_size" , inbounds_size) ; diag . arg ("inbounds_size_is_neg" , inbounds_size < 0) ; diag . arg ("inbounds_size_abs" , inbounds_size . unsigned_abs ()) ; diag . arg ("operation" , format ! ("{:?}" , msg)) ; } AlignmentCheckFailed (Misalignment { required , has } , msg) => { diag . arg ("required" , required . bytes ()) ; diag . arg ("has" , has . bytes ()) ; diag . arg ("msg" , format ! ("{msg:?}")) ; } WriteToReadOnly (alloc) | DerefFunctionPointer (alloc) | DerefVTablePointer (alloc) | DerefTypeIdPointer (alloc) => { diag . arg ("allocation" , alloc) ; } InvalidBool (b) => { diag . arg ("value" , format ! ("{b:02x}")) ; } InvalidChar (c) => { diag . arg ("value" , format ! ("{c:08x}")) ; } InvalidTag (tag) => { diag . arg ("tag" , format ! ("{tag:x}")) ; } InvalidStr (err) => { diag . arg ("err" , format ! ("{err}")) ; } InvalidUninitBytes (Some ((alloc , info))) => { diag . arg ("alloc" , alloc) ; diag . arg ("access" , info . access) ; diag . arg ("uninit" , info . bad) ; } ScalarSizeMismatch (info) => { diag . arg ("target_size" , info . target_size) ; diag . arg ("data_size" , info . data_size) ; } InvalidNichedEnumVariantWritten { enum_ty } => { diag . arg ("ty" , enum_ty) ; } AbiMismatchArgument { arg_idx , caller_ty , callee_ty } => { diag . arg ("arg_idx" , arg_idx + 1) ; diag . arg ("caller_ty" , caller_ty) ; diag . arg ("callee_ty" , callee_ty) ; } AbiMismatchReturn { caller_ty , callee_ty } => { diag . arg ("caller_ty" , caller_ty) ; diag . arg ("callee_ty" , callee_ty) ; } } } }}}
mkitem!{mkimpl!{impl < 'tcx > ReportErrorExt for ValidationErrorInfo < 'tcx > { fn diagnostic_message (& self) -> DiagMessage { use rustc_middle :: mir :: interpret :: ValidationErrorKind :: * ; use crate :: fluent_generated :: * ; match self . kind { PtrToUninhabited { ptr_kind : PointerKind :: Box , .. } => { const_eval_validation_box_to_uninhabited } PtrToUninhabited { ptr_kind : PointerKind :: Ref (_) , .. } => { const_eval_validation_ref_to_uninhabited } PointerAsInt { .. } => const_eval_validation_pointer_as_int , PartialPointer => const_eval_validation_partial_pointer , MutableRefToImmutable => const_eval_validation_mutable_ref_to_immutable , MutableRefInConst => const_eval_validation_mutable_ref_in_const , NullFnPtr => const_eval_validation_null_fn_ptr , NeverVal => const_eval_validation_never_val , NullablePtrOutOfRange { .. } => const_eval_validation_nullable_ptr_out_of_range , PtrOutOfRange { .. } => const_eval_validation_ptr_out_of_range , OutOfRange { .. } => const_eval_validation_out_of_range , UnsafeCellInImmutable => const_eval_validation_unsafe_cell , UninhabitedVal { .. } => const_eval_validation_uninhabited_val , InvalidEnumTag { .. } => const_eval_validation_invalid_enum_tag , UninhabitedEnumVariant => const_eval_validation_uninhabited_enum_variant , Uninit { .. } => const_eval_validation_uninit , InvalidVTablePtr { .. } => const_eval_validation_invalid_vtable_ptr , InvalidMetaWrongTrait { .. } => const_eval_validation_invalid_vtable_trait , InvalidMetaSliceTooLarge { ptr_kind : PointerKind :: Box } => { const_eval_validation_invalid_box_slice_meta } InvalidMetaSliceTooLarge { ptr_kind : PointerKind :: Ref (_) } => { const_eval_validation_invalid_ref_slice_meta } InvalidMetaTooLarge { ptr_kind : PointerKind :: Box } => { const_eval_validation_invalid_box_meta } InvalidMetaTooLarge { ptr_kind : PointerKind :: Ref (_) } => { const_eval_validation_invalid_ref_meta } UnalignedPtr { ptr_kind : PointerKind :: Ref (_) , .. } => { const_eval_validation_unaligned_ref } UnalignedPtr { ptr_kind : PointerKind :: Box , .. } => const_eval_validation_unaligned_box , NullPtr { ptr_kind : PointerKind :: Box } => const_eval_validation_null_box , NullPtr { ptr_kind : PointerKind :: Ref (_) } => const_eval_validation_null_ref , DanglingPtrNoProvenance { ptr_kind : PointerKind :: Box , .. } => { const_eval_validation_dangling_box_no_provenance } DanglingPtrNoProvenance { ptr_kind : PointerKind :: Ref (_) , .. } => { const_eval_validation_dangling_ref_no_provenance } DanglingPtrOutOfBounds { ptr_kind : PointerKind :: Box } => { const_eval_validation_dangling_box_out_of_bounds } DanglingPtrOutOfBounds { ptr_kind : PointerKind :: Ref (_) } => { const_eval_validation_dangling_ref_out_of_bounds } DanglingPtrUseAfterFree { ptr_kind : PointerKind :: Box } => { const_eval_validation_dangling_box_use_after_free } DanglingPtrUseAfterFree { ptr_kind : PointerKind :: Ref (_) } => { const_eval_validation_dangling_ref_use_after_free } InvalidBool { .. } => const_eval_validation_invalid_bool , InvalidChar { .. } => const_eval_validation_invalid_char , InvalidFnPtr { .. } => const_eval_validation_invalid_fn_ptr , } } fn add_args < G : EmissionGuarantee > (self , err : & mut Diag < '_ , G >) { use rustc_middle :: mir :: interpret :: ValidationErrorKind :: * ; use crate :: fluent_generated as fluent ; if let PointerAsInt { .. } | PartialPointer = self . kind { err . help (fluent :: const_eval_ptr_as_bytes_1) ; err . help (fluent :: const_eval_ptr_as_bytes_2) ; } let message = if let Some (path) = self . path { err . dcx . eagerly_translate_to_string (fluent :: const_eval_validation_front_matter_invalid_value_with_path , [("path" . into () , DiagArgValue :: Str (path . into ()))] . iter () . map (| (a , b) | (a , b)) ,) } else { err . dcx . eagerly_translate_to_string (fluent :: const_eval_validation_front_matter_invalid_value , [] . into_iter () ,) } ; err . arg ("front_matter" , message) ; fn add_range_arg < G : EmissionGuarantee > (r : WrappingRange , max_hi : u128 , err : & mut Diag < '_ , G > ,) { let WrappingRange { start : lo , end : hi } = r ; assert ! (hi <= max_hi) ; let msg = if lo > hi { fluent :: const_eval_range_wrapping } else if lo == hi { fluent :: const_eval_range_singular } else if lo == 0 { assert ! (hi < max_hi , "should not be printing if the range covers everything") ; fluent :: const_eval_range_upper } else if hi == max_hi { assert ! (lo > 0 , "should not be printing if the range covers everything") ; fluent :: const_eval_range_lower } else { fluent :: const_eval_range } ; let args = [("lo" . into () , DiagArgValue :: Str (lo . to_string () . into ())) , ("hi" . into () , DiagArgValue :: Str (hi . to_string () . into ())) ,] ; let args = args . iter () . map (| (a , b) | (a , b)) ; let message = err . dcx . eagerly_translate_to_string (msg , args) ; err . arg ("in_range" , message) ; } match self . kind { PtrToUninhabited { ty , .. } | UninhabitedVal { ty } => { err . arg ("ty" , ty) ; } PointerAsInt { expected } | Uninit { expected } => { let msg = match expected { ExpectedKind :: Reference => fluent :: const_eval_validation_expected_ref , ExpectedKind :: Box => fluent :: const_eval_validation_expected_box , ExpectedKind :: RawPtr => fluent :: const_eval_validation_expected_raw_ptr , ExpectedKind :: InitScalar => fluent :: const_eval_validation_expected_init_scalar , ExpectedKind :: Bool => fluent :: const_eval_validation_expected_bool , ExpectedKind :: Char => fluent :: const_eval_validation_expected_char , ExpectedKind :: Float => fluent :: const_eval_validation_expected_float , ExpectedKind :: Int => fluent :: const_eval_validation_expected_int , ExpectedKind :: FnPtr => fluent :: const_eval_validation_expected_fn_ptr , ExpectedKind :: EnumTag => fluent :: const_eval_validation_expected_enum_tag , ExpectedKind :: Str => fluent :: const_eval_validation_expected_str , } ; let msg = err . dcx . eagerly_translate_to_string (msg , [] . into_iter ()) ; err . arg ("expected" , msg) ; } InvalidEnumTag { value } | InvalidVTablePtr { value } | InvalidBool { value } | InvalidChar { value } | InvalidFnPtr { value } => { err . arg ("value" , value) ; } NullablePtrOutOfRange { range , max_value } | PtrOutOfRange { range , max_value } => { add_range_arg (range , max_value , err) } OutOfRange { range , max_value , value } => { err . arg ("value" , value) ; add_range_arg (range , max_value , err) ; } UnalignedPtr { required_bytes , found_bytes , .. } => { err . arg ("required_bytes" , required_bytes) ; err . arg ("found_bytes" , found_bytes) ; } DanglingPtrNoProvenance { pointer , .. } => { err . arg ("pointer" , pointer) ; } InvalidMetaWrongTrait { vtable_dyn_type , expected_dyn_type } => { err . arg ("vtable_dyn_type" , vtable_dyn_type . to_string ()) ; err . arg ("expected_dyn_type" , expected_dyn_type . to_string ()) ; } NullPtr { .. } | MutableRefToImmutable | MutableRefInConst | NullFnPtr | NeverVal | UnsafeCellInImmutable | InvalidMetaSliceTooLarge { .. } | InvalidMetaTooLarge { .. } | DanglingPtrUseAfterFree { .. } | DanglingPtrOutOfBounds { .. } | UninhabitedEnumVariant | PartialPointer => { } } } }}}
mkitem!{mkimpl!{impl ReportErrorExt for UnsupportedOpInfo { fn diagnostic_message (& self) -> DiagMessage { use crate :: fluent_generated :: * ; match self { UnsupportedOpInfo :: Unsupported (s) => s . clone () . into () , UnsupportedOpInfo :: ExternTypeField => const_eval_extern_type_field , UnsupportedOpInfo :: UnsizedLocal => const_eval_unsized_local , UnsupportedOpInfo :: ReadPartialPointer (_) => const_eval_partial_pointer_read , UnsupportedOpInfo :: ReadPointerAsInt (_) => const_eval_read_pointer_as_int , UnsupportedOpInfo :: ThreadLocalStatic (_) => const_eval_thread_local_static , UnsupportedOpInfo :: ExternStatic (_) => const_eval_extern_static , } } fn add_args < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { use UnsupportedOpInfo :: * ; use crate :: fluent_generated :: * ; if let ReadPointerAsInt (_) | ReadPartialPointer (_) = self { diag . help (const_eval_ptr_as_bytes_1) ; diag . help (const_eval_ptr_as_bytes_2) ; } match self { UnsizedLocal | UnsupportedOpInfo :: ExternTypeField | Unsupported (_) | ReadPointerAsInt (_) => { } ReadPartialPointer (ptr) => { diag . arg ("ptr" , ptr) ; } ThreadLocalStatic (did) | ExternStatic (did) => rustc_middle :: ty :: tls :: with (| tcx | { diag . arg ("did" , tcx . def_path_str (did)) ; }) , } } }}}
mkitem!{mkimpl!{impl < 'tcx > ReportErrorExt for InterpErrorKind < 'tcx > { fn diagnostic_message (& self) -> DiagMessage { match self { InterpErrorKind :: UndefinedBehavior (ub) => ub . diagnostic_message () , InterpErrorKind :: Unsupported (e) => e . diagnostic_message () , InterpErrorKind :: InvalidProgram (e) => e . diagnostic_message () , InterpErrorKind :: ResourceExhaustion (e) => e . diagnostic_message () , InterpErrorKind :: MachineStop (e) => e . diagnostic_message () , } } fn add_args < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { match self { InterpErrorKind :: UndefinedBehavior (ub) => ub . add_args (diag) , InterpErrorKind :: Unsupported (e) => e . add_args (diag) , InterpErrorKind :: InvalidProgram (e) => e . add_args (diag) , InterpErrorKind :: ResourceExhaustion (e) => e . add_args (diag) , InterpErrorKind :: MachineStop (e) => e . add_args (& mut | name , value | { diag . arg (name , value) ; }) , } } }}}
mkitem!{mkimpl!{impl < 'tcx > ReportErrorExt for InvalidProgramInfo < 'tcx > { fn diagnostic_message (& self) -> DiagMessage { use crate :: fluent_generated :: * ; match self { InvalidProgramInfo :: TooGeneric => const_eval_too_generic , InvalidProgramInfo :: AlreadyReported (_) => const_eval_already_reported , InvalidProgramInfo :: Layout (e) => e . diagnostic_message () , } } fn add_args < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { match self { InvalidProgramInfo :: TooGeneric | InvalidProgramInfo :: AlreadyReported (_) => { } InvalidProgramInfo :: Layout (e) => { let dummy_level = Level :: Bug ; let dummy_diag : Diag < '_ , () > = e . into_diagnostic () . into_diag (diag . dcx , dummy_level) ; for (name , val) in dummy_diag . args . iter () { diag . arg (name . clone () , val . clone ()) ; } dummy_diag . cancel () ; } } } }}}
mkitem!{mkimpl!{impl ReportErrorExt for ResourceExhaustionInfo { fn diagnostic_message (& self) -> DiagMessage { use crate :: fluent_generated :: * ; match self { ResourceExhaustionInfo :: StackFrameLimitReached => const_eval_stack_frame_limit_reached , ResourceExhaustionInfo :: MemoryExhausted => const_eval_memory_exhausted , ResourceExhaustionInfo :: AddressSpaceFull => const_eval_address_space_full , ResourceExhaustionInfo :: Interrupted => const_eval_interrupted , } } fn add_args < G : EmissionGuarantee > (self , _ : & mut Diag < '_ , G >) { } }}}
mkitem!{mkimpl!{impl rustc_errors :: IntoDiagArg for InternKind { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Borrowed (match self { InternKind :: Static (Mutability :: Not) => "static" , InternKind :: Static (Mutability :: Mut) => "static_mut" , InternKind :: Constant => "const" , InternKind :: Promoted => "promoted" , })) } }}}