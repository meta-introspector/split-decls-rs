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
mkuse!{use rustc_errors :: MultiSpan ;}
mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_macros :: { Diagnostic , LintDiagnostic , Subdiagnostic } ;}
mkuse!{use rustc_middle :: ty :: { GenericArg , Ty } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use crate :: diagnostics :: RegionName ;}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (borrowck_move_unsized , code = E0161)] pub (crate) struct MoveUnsized < 'tcx > { pub ty : Ty < 'tcx > , # [primary_span] # [label] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (borrowck_higher_ranked_lifetime_error)] pub (crate) struct HigherRankedLifetimeError { # [subdiagnostic] pub cause : Option < HigherRankedErrorCause > , # [primary_span] pub span : Span , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum HigherRankedErrorCause { # [note (borrowck_could_not_prove)] CouldNotProve { predicate : String } , # [note (borrowck_could_not_normalize)] CouldNotNormalize { value : String } , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (borrowck_higher_ranked_subtype_error)] pub (crate) struct HigherRankedSubtypeError { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (borrowck_generic_does_not_live_long_enough)] pub (crate) struct GenericDoesNotLiveLongEnough { pub kind : String , # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (borrowck_var_does_not_need_mut)] pub (crate) struct VarNeedNotMut { # [suggestion (style = "short" , applicability = "machine-applicable" , code = "")] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (borrowck_var_cannot_escape_closure)] # [note] # [note (borrowck_cannot_escape)] pub (crate) struct FnMutError { # [primary_span] pub span : Span , # [subdiagnostic] pub ty_err : FnMutReturnTypeErr , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum VarHereDenote { # [label (borrowck_var_here_captured)] Captured { # [primary_span] span : Span , } , # [label (borrowck_var_here_defined)] Defined { # [primary_span] span : Span , } , # [label (borrowck_closure_inferred_mut)] FnMutInferred { # [primary_span] span : Span , } , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum FnMutReturnTypeErr { # [label (borrowck_returned_closure_escaped)] ReturnClosure { # [primary_span] span : Span , } , # [label (borrowck_returned_async_block_escaped)] ReturnAsyncBlock { # [primary_span] span : Span , } , # [label (borrowck_returned_ref_escaped)] ReturnRef { # [primary_span] span : Span , } , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (borrowck_lifetime_constraints_error)] pub (crate) struct LifetimeOutliveErr { # [primary_span] pub span : Span , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum LifetimeReturnCategoryErr < 'a > { # [label (borrowck_returned_lifetime_wrong)] WrongReturn { # [primary_span] span : Span , mir_def_name : & 'a str , outlived_fr_name : RegionName , fr_name : & 'a RegionName , } , # [label (borrowck_returned_lifetime_short)] ShortReturn { # [primary_span] span : Span , category_desc : & 'static str , free_region_name : & 'a RegionName , outlived_fr_name : RegionName , } , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum RequireStaticErr { # [note (borrowck_used_impl_require_static)] UsedImpl { # [primary_span] multi_span : MultiSpan , } , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum CaptureVarPathUseCause { # [label (borrowck_borrow_due_to_use_coroutine)] BorrowInCoroutine { # [primary_span] path_span : Span , } , # [label (borrowck_use_due_to_use_coroutine)] UseInCoroutine { # [primary_span] path_span : Span , } , # [label (borrowck_assign_due_to_use_coroutine)] AssignInCoroutine { # [primary_span] path_span : Span , } , # [label (borrowck_assign_part_due_to_use_coroutine)] AssignPartInCoroutine { # [primary_span] path_span : Span , } , # [label (borrowck_borrow_due_to_use_closure)] BorrowInClosure { # [primary_span] path_span : Span , } , # [label (borrowck_use_due_to_use_closure)] UseInClosure { # [primary_span] path_span : Span , } , # [label (borrowck_assign_due_to_use_closure)] AssignInClosure { # [primary_span] path_span : Span , } , # [label (borrowck_assign_part_due_to_use_closure)] AssignPartInClosure { # [primary_span] path_span : Span , } , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum CaptureVarKind { # [label (borrowck_capture_immute)] Immut { # [primary_span] kind_span : Span , } , # [label (borrowck_capture_mut)] Mut { # [primary_span] kind_span : Span , } , # [label (borrowck_capture_move)] Move { # [primary_span] kind_span : Span , } , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum CaptureVarCause { # [label (borrowck_var_borrow_by_use_place_in_coroutine)] BorrowUsePlaceCoroutine { is_single_var : bool , place : String , # [primary_span] var_span : Span , } , # [label (borrowck_var_borrow_by_use_place_in_closure)] BorrowUsePlaceClosure { is_single_var : bool , place : String , # [primary_span] var_span : Span , } , # [label (borrowck_var_borrow_by_use_in_coroutine)] BorrowUseInCoroutine { # [primary_span] var_span : Span , } , # [label (borrowck_var_borrow_by_use_in_closure)] BorrowUseInClosure { # [primary_span] var_span : Span , } , # [label (borrowck_var_move_by_use_in_coroutine)] MoveUseInCoroutine { # [primary_span] var_span : Span , } , # [label (borrowck_var_move_by_use_in_closure)] MoveUseInClosure { # [primary_span] var_span : Span , } , # [label (borrowck_var_first_borrow_by_use_place_in_coroutine)] FirstBorrowUsePlaceCoroutine { place : String , # [primary_span] var_span : Span , } , # [label (borrowck_var_first_borrow_by_use_place_in_closure)] FirstBorrowUsePlaceClosure { place : String , # [primary_span] var_span : Span , } , # [label (borrowck_var_second_borrow_by_use_place_in_coroutine)] SecondBorrowUsePlaceCoroutine { place : String , # [primary_span] var_span : Span , } , # [label (borrowck_var_second_borrow_by_use_place_in_closure)] SecondBorrowUsePlaceClosure { place : String , # [primary_span] var_span : Span , } , # [label (borrowck_var_mutable_borrow_by_use_place_in_closure)] MutableBorrowUsePlaceClosure { place : String , # [primary_span] var_span : Span , } , # [label (borrowck_partial_var_move_by_use_in_coroutine)] PartialMoveUseInCoroutine { # [primary_span] var_span : Span , is_partial : bool , } , # [label (borrowck_partial_var_move_by_use_in_closure)] PartialMoveUseInClosure { # [primary_span] var_span : Span , is_partial : bool , } , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (borrowck_cannot_move_when_borrowed , code = E0505)] pub (crate) struct MoveBorrow < 'a > { pub place : & 'a str , pub borrow_place : & 'a str , pub value_place : & 'a str , # [primary_span] # [label (borrowck_move_label)] pub span : Span , # [label] pub borrow_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (borrowck_opaque_type_lifetime_mismatch)] pub (crate) struct LifetimeMismatchOpaqueParam < 'tcx > { pub arg : GenericArg < 'tcx > , pub prev : GenericArg < 'tcx > , # [primary_span] # [label] # [note] pub span : Span , # [label (borrowck_prev_lifetime_label)] pub prev_span : Span , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum CaptureReasonLabel < 'a > { # [label (borrowck_moved_due_to_call)] Call { # [primary_span] fn_call_span : Span , place_name : & 'a str , is_partial : bool , is_loop_message : bool , } , # [label (borrowck_moved_due_to_usage_in_operator)] OperatorUse { # [primary_span] fn_call_span : Span , place_name : & 'a str , is_partial : bool , is_loop_message : bool , } , # [label (borrowck_moved_due_to_implicit_into_iter_call)] ImplicitCall { # [primary_span] fn_call_span : Span , place_name : & 'a str , is_partial : bool , is_loop_message : bool , } , # [label (borrowck_moved_due_to_method_call)] MethodCall { # [primary_span] fn_call_span : Span , place_name : & 'a str , is_partial : bool , is_loop_message : bool , } , # [label (borrowck_moved_due_to_await)] Await { # [primary_span] fn_call_span : Span , place_name : & 'a str , is_partial : bool , is_loop_message : bool , } , # [label (borrowck_value_moved_here)] MovedHere { # [primary_span] move_span : Span , is_partial : bool , is_move_msg : bool , is_loop_message : bool , } , # [label (borrowck_consider_borrow_type_contents)] BorrowContent { # [primary_span] var_span : Span , } , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum CaptureReasonNote { # [note (borrowck_moved_a_fn_once_in_call)] FnOnceMoveInCall { # [primary_span] var_span : Span , } , # [note (borrowck_calling_operator_moves)] UnOpMoveByOperator { # [primary_span] span : Span , } , # [note (borrowck_calling_operator_moves_lhs)] LhsMoveByOperator { # [primary_span] span : Span , } , # [note (borrowck_func_take_self_moved_place)] FuncTakeSelf { func : String , place_name : String , # [primary_span] span : Span , } , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum CaptureReasonSuggest < 'tcx > { # [suggestion (borrowck_suggest_iterate_over_slice , applicability = "maybe-incorrect" , code = "&" , style = "verbose")] IterateSlice { ty : Ty < 'tcx > , # [primary_span] span : Span , } , # [suggestion (borrowck_suggest_create_fresh_reborrow , applicability = "maybe-incorrect" , code = ".as_mut()" , style = "verbose")] FreshReborrow { # [primary_span] span : Span , } , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum CaptureArgLabel { # [label (borrowck_value_capture_here)] Capture { is_within : bool , # [primary_span] args_span : Span , } , # [label (borrowck_move_out_place_here)] MoveOutPlace { place : String , # [primary_span] args_span : Span , } , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum OnClosureNote < 'a > { # [note (borrowck_closure_invoked_twice)] InvokedTwice { place_name : & 'a str , # [primary_span] span : Span , } , # [note (borrowck_closure_moved_twice)] MovedTwice { place_name : & 'a str , # [primary_span] span : Span , } , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum TypeNoCopy < 'a , 'tcx > { # [label (borrowck_ty_no_impl_copy)] Label { is_partial_move : bool , ty : Ty < 'tcx > , place : & 'a str , # [primary_span] span : Span , } , # [note (borrowck_ty_no_impl_copy)] Note { is_partial_move : bool , ty : Ty < 'tcx > , place : & 'a str } , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (borrowck_simd_intrinsic_arg_const)] pub (crate) struct SimdIntrinsicArgConst { # [primary_span] pub span : Span , pub arg : usize , pub intrinsic : String , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (borrowck_tail_expr_drop_order)] pub (crate) struct TailExprDropOrder { # [label] pub borrowed : Span , }}}