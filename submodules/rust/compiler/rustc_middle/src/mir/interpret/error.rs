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
mkuse!{use std :: any :: Any ;}
mkuse!{use std :: backtrace :: Backtrace ;}
mkuse!{use std :: borrow :: Cow ;}
mkuse!{use std :: { convert , fmt , mem , ops } ;}
mkuse!{use either :: Either ;}
mkuse!{use rustc_abi :: { Align , Size , VariantIdx , WrappingRange } ;}
mkuse!{use rustc_data_structures :: sync :: Lock ;}
mkuse!{use rustc_errors :: { DiagArgName , DiagArgValue , DiagMessage , ErrorGuaranteed , IntoDiagArg } ;}
mkuse!{use rustc_macros :: { HashStable , TyDecodable , TyEncodable } ;}
mkuse!{use rustc_session :: CtfeBacktrace ;}
mkuse!{use rustc_span :: def_id :: DefId ;}
mkuse!{use rustc_span :: { DUMMY_SP , Span , Symbol } ;}
mkuse!{use super :: { AllocId , AllocRange , ConstAllocation , Pointer , Scalar } ;}
mkuse!{use crate :: error ;}
mkuse!{use crate :: mir :: { ConstAlloc , ConstValue } ;}
mkuse!{use crate :: ty :: { self , Mutability , Ty , TyCtxt , ValTree , layout , tls } ;}
mkitem!{mkenum!{# [derive (Debug , Copy , Clone , PartialEq , Eq , HashStable , TyEncodable , TyDecodable)] pub enum ErrorHandled { # [doc = " Already reported an error for this evaluation, and the compilation is"] # [doc = " *guaranteed* to fail. Warnings/lints *must not* produce `Reported`."] Reported (ReportedErrorInfo , Span) , # [doc = " Don't emit an error, the evaluation failed because the MIR was generic"] # [doc = " and the args didn't fully monomorphize it."] TooGeneric (Span) , }}}
mkitem!{mkimpl!{impl From < ReportedErrorInfo > for ErrorHandled { # [inline] fn from (error : ReportedErrorInfo) -> ErrorHandled { ErrorHandled :: Reported (error , DUMMY_SP) } }}}
mkitem!{mkimpl!{impl ErrorHandled { pub (crate) fn with_span (self , span : Span) -> Self { match self { ErrorHandled :: Reported (err , _span) => ErrorHandled :: Reported (err , span) , ErrorHandled :: TooGeneric (_span) => ErrorHandled :: TooGeneric (span) , } } pub fn emit_note (& self , tcx : TyCtxt < '_ >) { match self { & ErrorHandled :: Reported (err , span) => { if ! err . allowed_in_infallible && ! span . is_dummy () { tcx . dcx () . emit_note (error :: ErroneousConstant { span }) ; } } & ErrorHandled :: TooGeneric (_) => { } } } }}}
mkitem!{mkstruct!{# [derive (Debug , Copy , Clone , PartialEq , Eq , HashStable , TyEncodable , TyDecodable)] pub struct ReportedErrorInfo { error : ErrorGuaranteed , # [doc = " Whether this error is allowed to show up even in otherwise \"infallible\" promoteds."] # [doc = " This is for things like overflows during size computation or resource exhaustion."] allowed_in_infallible : bool , }}}
mkitem!{mkimpl!{impl ReportedErrorInfo { # [inline] pub fn const_eval_error (error : ErrorGuaranteed) -> ReportedErrorInfo { ReportedErrorInfo { allowed_in_infallible : false , error } } # [doc = " Use this when the error that led to this is *not* a const-eval error"] # [doc = " (e.g., a layout or type checking error)."] # [inline] pub fn non_const_eval_error (error : ErrorGuaranteed) -> ReportedErrorInfo { ReportedErrorInfo { allowed_in_infallible : true , error } } # [doc = " Use this when the error that led to this *is* a const-eval error, but"] # [doc = " we do allow it to occur in infallible constants (e.g., resource exhaustion)."] # [inline] pub fn allowed_in_infallible (error : ErrorGuaranteed) -> ReportedErrorInfo { ReportedErrorInfo { allowed_in_infallible : true , error } } pub fn is_allowed_in_infallible (& self) -> bool { self . allowed_in_infallible } }}}
mkitem!{mkimpl!{impl From < ReportedErrorInfo > for ErrorGuaranteed { # [inline] fn from (val : ReportedErrorInfo) -> Self { val . error } }}}
mkitem!{mkenum!{# [doc = " An error type for the `const_to_valtree` query. Some error should be reported with a \"use-site span\","] # [doc = " which means the query cannot emit the error, so those errors are represented as dedicated variants here."] # [derive (Debug , Copy , Clone , PartialEq , Eq , HashStable , TyEncodable , TyDecodable)] pub enum ValTreeCreationError < 'tcx > { # [doc = " The constant is too big to be valtree'd."] NodesOverflow , # [doc = " The constant references mutable or external memory, so it cannot be valtree'd."] InvalidConst , # [doc = " Values of this type, or this particular value, are not supported as valtrees."] NonSupportedType (Ty < 'tcx >) , # [doc = " The error has already been handled by const evaluation."] ErrorHandled (ErrorHandled) , }}}
mkitem!{mkimpl!{impl < 'tcx > From < ErrorHandled > for ValTreeCreationError < 'tcx > { fn from (err : ErrorHandled) -> Self { ValTreeCreationError :: ErrorHandled (err) } }}}
mkitem!{mkimpl!{impl < 'tcx > From < InterpErrorInfo < 'tcx > > for ValTreeCreationError < 'tcx > { fn from (err : InterpErrorInfo < 'tcx >) -> Self { let (_kind , backtrace) = err . into_parts () ; backtrace . print_backtrace () ; ValTreeCreationError :: InvalidConst } }}}
mkitem!{mkimpl!{impl < 'tcx > ValTreeCreationError < 'tcx > { pub (crate) fn with_span (self , span : Span) -> Self { use ValTreeCreationError :: * ; match self { ErrorHandled (handled) => ErrorHandled (handled . with_span (span)) , other => other , } } }}}
mkitem!{pub type EvalToAllocationRawResult < 'tcx > = Result < ConstAlloc < 'tcx > , ErrorHandled > ;}
mkitem!{pub type EvalStaticInitializerRawResult < 'tcx > = Result < ConstAllocation < 'tcx > , ErrorHandled > ;}
mkitem!{pub type EvalToConstValueResult < 'tcx > = Result < ConstValue , ErrorHandled > ;}
mkitem!{pub type EvalToValTreeResult < 'tcx > = Result < ValTree < 'tcx > , ValTreeCreationError < 'tcx > > ;}
mkitem!{# [cfg (target_pointer_width = "64")] rustc_data_structures :: static_assert_size ! (InterpErrorInfo <'_ >, 8) ;}
mkitem!{mkstruct!{# [doc = " Packages the kind of error we got from the const code interpreter"] # [doc = " up with a Rust-level backtrace of where the error occurred."] # [doc = " These should always be constructed by calling `.into()` on"] # [doc = " an `InterpError`. In `rustc_mir::interpret`, we have `throw_err_*`"] # [doc = " macros for this."] # [doc = ""] # [doc = " Interpreter errors must *not* be silently discarded (that will lead to a panic). Instead,"] # [doc = " explicitly call `discard_err` if this is really the right thing to do. Note that if"] # [doc = " this happens during const-eval or in Miri, it could lead to a UB error being lost!"] # [derive (Debug)] pub struct InterpErrorInfo < 'tcx > (Box < InterpErrorInfoInner < 'tcx > >) ;}}
mkitem!{mkstruct!{# [derive (Debug)] struct InterpErrorInfoInner < 'tcx > { kind : InterpErrorKind < 'tcx > , backtrace : InterpErrorBacktrace , }}}
mkitem!{mkstruct!{# [derive (Debug)] pub struct InterpErrorBacktrace { backtrace : Option < Box < Backtrace > > , }}}
mkitem!{mkimpl!{impl InterpErrorBacktrace { pub fn new () -> InterpErrorBacktrace { let capture_backtrace = tls :: with_opt (| tcx | { if let Some (tcx) = tcx { * Lock :: borrow (& tcx . sess . ctfe_backtrace) } else { CtfeBacktrace :: Disabled } }) ; let backtrace = match capture_backtrace { CtfeBacktrace :: Disabled => None , CtfeBacktrace :: Capture => Some (Box :: new (Backtrace :: force_capture ())) , CtfeBacktrace :: Immediate => { let backtrace = Backtrace :: force_capture () ; print_backtrace (& backtrace) ; None } } ; InterpErrorBacktrace { backtrace } } pub fn print_backtrace (& self) { if let Some (backtrace) = self . backtrace . as_ref () { print_backtrace (backtrace) ; } } }}}
mkitem!{mkimpl!{impl < 'tcx > InterpErrorInfo < 'tcx > { pub fn into_parts (self) -> (InterpErrorKind < 'tcx > , InterpErrorBacktrace) { let InterpErrorInfo (box InterpErrorInfoInner { kind , backtrace }) = self ; (kind , backtrace) } pub fn into_kind (self) -> InterpErrorKind < 'tcx > { self . 0 . kind } pub fn from_parts (kind : InterpErrorKind < 'tcx > , backtrace : InterpErrorBacktrace) -> Self { Self (Box :: new (InterpErrorInfoInner { kind , backtrace })) } # [inline] pub fn kind (& self) -> & InterpErrorKind < 'tcx > { & self . 0 . kind } }}}

macro_rules! print_backtrace_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function print_backtrace in module {}", module_path!());
    };
}

mkfn!{
    print_backtrace_introspect!();
    fn print_backtrace (backtrace : & Backtrace) { eprintln ! ("\n\nAn error occurred in the MIR interpreter:\n{backtrace}") ; }
}
mkitem!{mkimpl!{impl From < ErrorHandled > for InterpErrorInfo < '_ > { fn from (err : ErrorHandled) -> Self { InterpErrorKind :: InvalidProgram (match err { ErrorHandled :: Reported (r , _span) => InvalidProgramInfo :: AlreadyReported (r) , ErrorHandled :: TooGeneric (_span) => InvalidProgramInfo :: TooGeneric , }) . into () } }}}
mkitem!{mkimpl!{impl < 'tcx > From < InterpErrorKind < 'tcx > > for InterpErrorInfo < 'tcx > { fn from (kind : InterpErrorKind < 'tcx >) -> Self { InterpErrorInfo (Box :: new (InterpErrorInfoInner { kind , backtrace : InterpErrorBacktrace :: new () , })) } }}}
mkitem!{mkenum!{# [doc = " Error information for when the program we executed turned out not to actually be a valid"] # [doc = " program. This cannot happen in stand-alone Miri (except for layout errors that are only detect"] # [doc = " during monomorphization), but it can happen during CTFE/ConstProp where we work on generic code"] # [doc = " or execution does not have all information available."] # [derive (Debug)] pub enum InvalidProgramInfo < 'tcx > { # [doc = " Resolution can fail if we are in a too generic context."] TooGeneric , # [doc = " Abort in case errors are already reported."] AlreadyReported (ReportedErrorInfo) , # [doc = " An error occurred during layout computation."] Layout (layout :: LayoutError < 'tcx >) , }}}
mkitem!{mkenum!{# [doc = " Details of why a pointer had to be in-bounds."] # [derive (Debug , Copy , Clone)] pub enum CheckInAllocMsg { # [doc = " We are accessing memory."] MemoryAccess , # [doc = " We are doing pointer arithmetic."] InboundsPointerArithmetic , # [doc = " None of the above -- generic/unspecific inbounds test."] Dereferenceable , }}}
mkitem!{mkenum!{# [doc = " Details of which pointer is not aligned."] # [derive (Debug , Copy , Clone)] pub enum CheckAlignMsg { # [doc = " The accessed pointer did not have proper alignment."] AccessedPtr , # [doc = " The access occurred with a place that was based on a misaligned pointer."] BasedOn , }}}
mkitem!{mkenum!{# [derive (Debug , Copy , Clone)] pub enum InvalidMetaKind { # [doc = " Size of a `[T]` is too big"] SliceTooBig , # [doc = " Size of a DST is too big"] TooBig , }}}
mkitem!{mkimpl!{impl IntoDiagArg for InvalidMetaKind { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Borrowed (match self { InvalidMetaKind :: SliceTooBig => "slice_too_big" , InvalidMetaKind :: TooBig => "too_big" , })) } }}}
mkitem!{mkstruct!{# [doc = " Details of an access to uninitialized bytes / bad pointer bytes where it is not allowed."] # [derive (Debug , Clone , Copy)] pub struct BadBytesAccess { # [doc = " Range of the original memory access."] pub access : AllocRange , # [doc = " Range of the bad memory that was encountered. (Might not be maximal.)"] pub bad : AllocRange , }}}
mkitem!{mkstruct!{# [doc = " Information about a size mismatch."] # [derive (Debug)] pub struct ScalarSizeMismatch { pub target_size : u64 , pub data_size : u64 , }}}
mkitem!{mkstruct!{# [doc = " Information about a misaligned pointer."] # [derive (Copy , Clone , Hash , PartialEq , Eq , Debug)] pub struct Misalignment { pub has : Align , pub required : Align , }}}
mkitem!{macro_rules ! impl_into_diag_arg_through_debug { ($ ($ ty : ty) ,*$ (,) ?) => { $ (impl IntoDiagArg for $ ty { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (format ! ("{self:?}"))) } }) * } }}
mkitem!{impl_into_diag_arg_through_debug ! { AllocId , Pointer < AllocId >, AllocRange , }}
mkitem!{mkenum!{# [doc = " Error information for when the program caused Undefined Behavior."] # [derive (Debug)] pub enum UndefinedBehaviorInfo < 'tcx > { # [doc = " Free-form case. Only for errors that are never caught! Used by miri"] Ub (String) , # [doc = " A custom (free-form) fluent-translated error, created by `err_ub_custom!`."] Custom (crate :: error :: CustomSubdiagnostic < 'tcx >) , # [doc = " Validation error."] ValidationError (ValidationErrorInfo < 'tcx >) , # [doc = " Unreachable code was executed."] Unreachable , # [doc = " A slice/array index projection went out-of-bounds."] BoundsCheckFailed { len : u64 , index : u64 } , # [doc = " Something was divided by 0 (x / 0)."] DivisionByZero , # [doc = " Something was \"remainded\" by 0 (x % 0)."] RemainderByZero , # [doc = " Signed division overflowed (INT_MIN / -1)."] DivisionOverflow , # [doc = " Signed remainder overflowed (INT_MIN % -1)."] RemainderOverflow , # [doc = " Overflowing inbounds pointer arithmetic."] PointerArithOverflow , # [doc = " Overflow in arithmetic that may not overflow."] ArithOverflow { intrinsic : Symbol } , # [doc = " Shift by too much."] ShiftOverflow { intrinsic : Symbol , shift_amount : Either < u128 , i128 > } , # [doc = " Invalid metadata in a wide pointer"] InvalidMeta (InvalidMetaKind) , # [doc = " Reading a C string that does not end within its allocation."] UnterminatedCString (Pointer < AllocId >) , # [doc = " Using a pointer after it got freed."] PointerUseAfterFree (AllocId , CheckInAllocMsg) , # [doc = " Used a pointer outside the bounds it is valid for."] PointerOutOfBounds { alloc_id : AllocId , alloc_size : Size , ptr_offset : i64 , # [doc = " The size of the memory range that was expected to be in-bounds."] inbounds_size : i64 , msg : CheckInAllocMsg , } , # [doc = " Using an integer as a pointer in the wrong way."] DanglingIntPointer { addr : u64 , # [doc = " The size of the memory range that was expected to be in-bounds (or 0 if we need an"] # [doc = " allocation but not any actual memory there, e.g. for function pointers)."] inbounds_size : i64 , msg : CheckInAllocMsg , } , # [doc = " Used a pointer with bad alignment."] AlignmentCheckFailed (Misalignment , CheckAlignMsg) , # [doc = " Writing to read-only memory."] WriteToReadOnly (AllocId) , # [doc = " Trying to access the data behind a function pointer."] DerefFunctionPointer (AllocId) , # [doc = " Trying to access the data behind a vtable pointer."] DerefVTablePointer (AllocId) , # [doc = " Trying to access the actual type id."] DerefTypeIdPointer (AllocId) , # [doc = " Using a non-boolean `u8` as bool."] InvalidBool (u8) , # [doc = " Using a non-character `u32` as character."] InvalidChar (u32) , # [doc = " The tag of an enum does not encode an actual discriminant."] InvalidTag (Scalar < AllocId >) , # [doc = " Using a pointer-not-to-a-function as function pointer."] InvalidFunctionPointer (Pointer < AllocId >) , # [doc = " Using a pointer-not-to-a-vtable as vtable pointer."] InvalidVTablePointer (Pointer < AllocId >) , # [doc = " Using a vtable for the wrong trait."] InvalidVTableTrait { # [doc = " The vtable that was actually referenced by the wide pointer metadata."] vtable_dyn_type : & 'tcx ty :: List < ty :: PolyExistentialPredicate < 'tcx > > , # [doc = " The vtable that was expected at the point in MIR that it was accessed."] expected_dyn_type : & 'tcx ty :: List < ty :: PolyExistentialPredicate < 'tcx > > , } , # [doc = " Using a string that is not valid UTF-8,"] InvalidStr (std :: str :: Utf8Error) , # [doc = " Using uninitialized data where it is not allowed."] InvalidUninitBytes (Option < (AllocId , BadBytesAccess) >) , # [doc = " Working with a local that is not currently live."] DeadLocal , # [doc = " Data size is not equal to target size."] ScalarSizeMismatch (ScalarSizeMismatch) , # [doc = " A discriminant of an uninhabited enum variant is written."] UninhabitedEnumVariantWritten (VariantIdx) , # [doc = " An uninhabited enum variant is projected."] UninhabitedEnumVariantRead (Option < VariantIdx >) , # [doc = " Trying to set discriminant to the niched variant, but the value does not match."] InvalidNichedEnumVariantWritten { enum_ty : Ty < 'tcx > } , # [doc = " ABI-incompatible argument types."] AbiMismatchArgument { # [doc = " The index of the argument whose type is wrong."] arg_idx : usize , caller_ty : Ty < 'tcx > , callee_ty : Ty < 'tcx > , } , # [doc = " ABI-incompatible return types."] AbiMismatchReturn { caller_ty : Ty < 'tcx > , callee_ty : Ty < 'tcx > } , }}}
mkitem!{mkenum!{# [derive (Debug , Clone , Copy)] pub enum PointerKind { Ref (Mutability) , Box , }}}
mkitem!{mkimpl!{impl IntoDiagArg for PointerKind { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (match self { Self :: Ref (_) => "ref" , Self :: Box => "box" , } . into () ,) } }}}
mkitem!{mkstruct!{# [derive (Debug)] pub struct ValidationErrorInfo < 'tcx > { pub path : Option < String > , pub kind : ValidationErrorKind < 'tcx > , }}}
mkitem!{mkenum!{# [derive (Debug)] pub enum ExpectedKind { Reference , Box , RawPtr , InitScalar , Bool , Char , Float , Int , FnPtr , EnumTag , Str , }}}
mkitem!{mkimpl!{impl From < PointerKind > for ExpectedKind { fn from (x : PointerKind) -> ExpectedKind { match x { PointerKind :: Box => ExpectedKind :: Box , PointerKind :: Ref (_) => ExpectedKind :: Reference , } } }}}
mkitem!{mkenum!{# [derive (Debug)] pub enum ValidationErrorKind < 'tcx > { PointerAsInt { expected : ExpectedKind , } , PartialPointer , PtrToUninhabited { ptr_kind : PointerKind , ty : Ty < 'tcx > , } , MutableRefToImmutable , UnsafeCellInImmutable , MutableRefInConst , NullFnPtr , NeverVal , NullablePtrOutOfRange { range : WrappingRange , max_value : u128 , } , PtrOutOfRange { range : WrappingRange , max_value : u128 , } , OutOfRange { value : String , range : WrappingRange , max_value : u128 , } , UninhabitedVal { ty : Ty < 'tcx > , } , InvalidEnumTag { value : String , } , UninhabitedEnumVariant , Uninit { expected : ExpectedKind , } , InvalidVTablePtr { value : String , } , InvalidMetaWrongTrait { # [doc = " The vtable that was actually referenced by the wide pointer metadata."] vtable_dyn_type : & 'tcx ty :: List < ty :: PolyExistentialPredicate < 'tcx > > , # [doc = " The vtable that was expected at the point in MIR that it was accessed."] expected_dyn_type : & 'tcx ty :: List < ty :: PolyExistentialPredicate < 'tcx > > , } , InvalidMetaSliceTooLarge { ptr_kind : PointerKind , } , InvalidMetaTooLarge { ptr_kind : PointerKind , } , UnalignedPtr { ptr_kind : PointerKind , required_bytes : u64 , found_bytes : u64 , } , NullPtr { ptr_kind : PointerKind , } , DanglingPtrNoProvenance { ptr_kind : PointerKind , pointer : String , } , DanglingPtrOutOfBounds { ptr_kind : PointerKind , } , DanglingPtrUseAfterFree { ptr_kind : PointerKind , } , InvalidBool { value : String , } , InvalidChar { value : String , } , InvalidFnPtr { value : String , } , }}}
mkitem!{mkenum!{# [doc = " Error information for when the program did something that might (or might not) be correct"] # [doc = " to do according to the Rust spec, but due to limitations in the interpreter, the"] # [doc = " operation could not be carried out. These limitations can differ between CTFE and the"] # [doc = " Miri engine, e.g., CTFE does not support dereferencing pointers at integral addresses."] # [derive (Debug)] pub enum UnsupportedOpInfo { # [doc = " Free-form case. Only for errors that are never caught! Used by Miri."] Unsupported (String) , # [doc = " Unsized local variables."] UnsizedLocal , # [doc = " Extern type field with an indeterminate offset."] ExternTypeField , # [doc = " Attempting to read or copy parts of a pointer to somewhere else; without knowing absolute"] # [doc = " addresses, the resulting state cannot be represented by the CTFE interpreter."] ReadPartialPointer (Pointer < AllocId >) , # [doc = " Encountered a pointer where we needed an integer."] ReadPointerAsInt (Option < (AllocId , BadBytesAccess) >) , # [doc = " Accessing thread local statics"] ThreadLocalStatic (DefId) , # [doc = " Accessing an unsupported extern static."] ExternStatic (DefId) , }}}
mkitem!{mkenum!{# [doc = " Error information for when the program exhausted the resources granted to it"] # [doc = " by the interpreter."] # [derive (Debug)] pub enum ResourceExhaustionInfo { # [doc = " The stack grew too big."] StackFrameLimitReached , # [doc = " There is not enough memory (on the host) to perform an allocation."] MemoryExhausted , # [doc = " The address space (of the target) is full."] AddressSpaceFull , # [doc = " The compiler got an interrupt signal (a user ran out of patience)."] Interrupted , }}}
mkitem!{mktrait!{# [doc = " A trait for machine-specific errors (or other \"machine stop\" conditions)."] pub trait MachineStopType : Any + fmt :: Debug + Send { # [doc = " The diagnostic message for this error"] fn diagnostic_message (& self) -> DiagMessage ; # [doc = " Add diagnostic arguments by passing name and value pairs to `adder`, which are passed to"] # [doc = " fluent for formatting the translated diagnostic message."] fn add_args (self : Box < Self > , adder : & mut dyn FnMut (DiagArgName , DiagArgValue)) ; }}}
mkitem!{mkimpl!{impl dyn MachineStopType { # [inline (always)] pub fn downcast_ref < T : Any > (& self) -> Option < & T > { let x : & dyn Any = self ; x . downcast_ref () } }}}
mkitem!{mkenum!{# [derive (Debug)] pub enum InterpErrorKind < 'tcx > { # [doc = " The program caused undefined behavior."] UndefinedBehavior (UndefinedBehaviorInfo < 'tcx >) , # [doc = " The program did something the interpreter does not support (some of these *might* be UB"] # [doc = " but the interpreter is not sure)."] Unsupported (UnsupportedOpInfo) , # [doc = " The program was invalid (ill-typed, bad MIR, not sufficiently monomorphized, ...)."] InvalidProgram (InvalidProgramInfo < 'tcx >) , # [doc = " The program exhausted the interpreter's resources (stack/heap too big,"] # [doc = " execution takes too long, ...)."] ResourceExhaustion (ResourceExhaustionInfo) , # [doc = " Stop execution for a machine-controlled reason. This is never raised by"] # [doc = " the core engine itself."] MachineStop (Box < dyn MachineStopType >) , }}}
mkitem!{mkimpl!{impl InterpErrorKind < '_ > { # [doc = " Some errors do string formatting even if the error is never printed."] # [doc = " To avoid performance issues, there are places where we want to be sure to never raise these formatting errors,"] # [doc = " so this method lets us detect them and `bug!` on unexpected errors."] pub fn formatted_string (& self) -> bool { matches ! (self , InterpErrorKind :: Unsupported (UnsupportedOpInfo :: Unsupported (_)) | InterpErrorKind :: UndefinedBehavior (UndefinedBehaviorInfo :: ValidationError { .. }) | InterpErrorKind :: UndefinedBehavior (UndefinedBehaviorInfo :: Ub (_))) } }}}
mkitem!{# [macro_export] macro_rules ! err_unsup { ($ ($ tt : tt) *) => { $ crate :: mir :: interpret :: InterpErrorKind :: Unsupported ($ crate :: mir :: interpret :: UnsupportedOpInfo ::$ ($ tt) *) } ; }}
mkitem!{# [macro_export] macro_rules ! err_unsup_format { ($ ($ tt : tt) *) => { $ crate :: err_unsup ! (Unsupported (format ! ($ ($ tt) *))) } ; }}
mkitem!{# [macro_export] macro_rules ! err_inval { ($ ($ tt : tt) *) => { $ crate :: mir :: interpret :: InterpErrorKind :: InvalidProgram ($ crate :: mir :: interpret :: InvalidProgramInfo ::$ ($ tt) *) } ; }}
mkitem!{# [macro_export] macro_rules ! err_ub { ($ ($ tt : tt) *) => { $ crate :: mir :: interpret :: InterpErrorKind :: UndefinedBehavior ($ crate :: mir :: interpret :: UndefinedBehaviorInfo ::$ ($ tt) *) } ; }}
mkitem!{# [macro_export] macro_rules ! err_ub_format { ($ ($ tt : tt) *) => { $ crate :: err_ub ! (Ub (format ! ($ ($ tt) *))) } ; }}
mkitem!{# [macro_export] macro_rules ! err_ub_custom { ($ msg : expr $ (, $ ($ name : ident = $ value : expr) ,* $ (,) ?) ?) => { { $ (let ($ ($ name ,) *) = ($ ($ value ,) *) ;) ? $ crate :: err_ub ! (Custom ($ crate :: error :: CustomSubdiagnostic { msg : || $ msg , add_args : Box :: new (move | mut set_arg | { $ ($ (set_arg (stringify ! ($ name) . into () , rustc_errors :: IntoDiagArg :: into_diag_arg ($ name , & mut None)) ;) *) ? }) })) } } ; }}
mkitem!{# [macro_export] macro_rules ! err_exhaust { ($ ($ tt : tt) *) => { $ crate :: mir :: interpret :: InterpErrorKind :: ResourceExhaustion ($ crate :: mir :: interpret :: ResourceExhaustionInfo ::$ ($ tt) *) } ; }}
mkitem!{# [macro_export] macro_rules ! err_machine_stop { ($ ($ tt : tt) *) => { $ crate :: mir :: interpret :: InterpErrorKind :: MachineStop (Box :: new ($ ($ tt) *)) } ; }}
mkitem!{# [macro_export] macro_rules ! throw_unsup { ($ ($ tt : tt) *) => { do yeet $ crate :: err_unsup ! ($ ($ tt) *) } ; }}
mkitem!{# [macro_export] macro_rules ! throw_unsup_format { ($ ($ tt : tt) *) => { do yeet $ crate :: err_unsup_format ! ($ ($ tt) *) } ; }}
mkitem!{# [macro_export] macro_rules ! throw_inval { ($ ($ tt : tt) *) => { do yeet $ crate :: err_inval ! ($ ($ tt) *) } ; }}
mkitem!{# [macro_export] macro_rules ! throw_ub { ($ ($ tt : tt) *) => { do yeet $ crate :: err_ub ! ($ ($ tt) *) } ; }}
mkitem!{# [macro_export] macro_rules ! throw_ub_format { ($ ($ tt : tt) *) => { do yeet $ crate :: err_ub_format ! ($ ($ tt) *) } ; }}
mkitem!{# [macro_export] macro_rules ! throw_ub_custom { ($ ($ tt : tt) *) => { do yeet $ crate :: err_ub_custom ! ($ ($ tt) *) } ; }}
mkitem!{# [macro_export] macro_rules ! throw_exhaust { ($ ($ tt : tt) *) => { do yeet $ crate :: err_exhaust ! ($ ($ tt) *) } ; }}
mkitem!{# [macro_export] macro_rules ! throw_machine_stop { ($ ($ tt : tt) *) => { do yeet $ crate :: err_machine_stop ! ($ ($ tt) *) } ; }}
mkitem!{mkstruct!{# [doc = " Guard type that panics on drop."] # [derive (Debug)] struct Guard ;}}
mkitem!{mkimpl!{impl Drop for Guard { fn drop (& mut self) { if ! std :: thread :: panicking () { panic ! ("an interpreter error got improperly discarded; use `discard_err()` if this is intentional") ; } } }}}
mkitem!{mkstruct!{# [doc = " The result type used by the interpreter. This is a newtype around `Result`"] # [doc = " to block access to operations like `ok()` that discard UB errors."] # [doc = ""] # [doc = " We also make things panic if this type is ever implicitly dropped."] # [derive (Debug)] # [must_use] pub struct InterpResult < 'tcx , T = () > { res : Result < T , InterpErrorInfo < 'tcx > > , guard : Guard , }}}
mkitem!{mkimpl!{impl < 'tcx , T > ops :: Try for InterpResult < 'tcx , T > { type Output = T ; type Residual = InterpResult < 'tcx , convert :: Infallible > ; # [inline] fn from_output (output : Self :: Output) -> Self { InterpResult :: new (Ok (output)) } # [inline] fn branch (self) -> ops :: ControlFlow < Self :: Residual , Self :: Output > { match self . disarm () { Ok (v) => ops :: ControlFlow :: Continue (v) , Err (e) => ops :: ControlFlow :: Break (InterpResult :: new (Err (e))) , } } }}}
mkitem!{mkimpl!{impl < 'tcx , T > ops :: Residual < T > for InterpResult < 'tcx , convert :: Infallible > { type TryType = InterpResult < 'tcx , T > ; }}}
mkitem!{mkimpl!{impl < 'tcx , T > ops :: FromResidual for InterpResult < 'tcx , T > { # [inline] # [track_caller] fn from_residual (residual : InterpResult < 'tcx , convert :: Infallible >) -> Self { match residual . disarm () { Err (e) => Self :: new (Err (e)) , } } }}}
mkitem!{mkimpl!{impl < 'tcx , T > ops :: FromResidual < ops :: Yeet < InterpErrorKind < 'tcx > > > for InterpResult < 'tcx , T > { # [inline] fn from_residual (ops :: Yeet (e) : ops :: Yeet < InterpErrorKind < 'tcx > >) -> Self { Self :: new (Err (e . into ())) } }}}
mkitem!{mkimpl!{impl < 'tcx , T , E : Into < InterpErrorInfo < 'tcx > > > ops :: FromResidual < Result < convert :: Infallible , E > > for InterpResult < 'tcx , T > { # [inline] fn from_residual (residual : Result < convert :: Infallible , E >) -> Self { match residual { Err (e) => Self :: new (Err (e . into ())) , } } }}}
mkitem!{mkimpl!{impl < 'tcx , T , E : Into < InterpErrorInfo < 'tcx > > > From < Result < T , E > > for InterpResult < 'tcx , T > { # [inline] fn from (value : Result < T , E >) -> Self { Self :: new (value . map_err (| e | e . into ())) } }}}
mkitem!{mkimpl!{impl < 'tcx , T , V : FromIterator < T > > FromIterator < InterpResult < 'tcx , T > > for InterpResult < 'tcx , V > { fn from_iter < I : IntoIterator < Item = InterpResult < 'tcx , T > > > (iter : I) -> Self { Self :: new (iter . into_iter () . map (| x | x . disarm ()) . collect ()) } }}}
mkitem!{mkimpl!{impl < 'tcx , T > InterpResult < 'tcx , T > { # [inline (always)] fn new (res : Result < T , InterpErrorInfo < 'tcx > >) -> Self { Self { res , guard : Guard } } # [inline (always)] fn disarm (self) -> Result < T , InterpErrorInfo < 'tcx > > { mem :: forget (self . guard) ; self . res } # [doc = " Discard the error information in this result. Only use this if ignoring Undefined Behavior is okay!"] # [inline] pub fn discard_err (self) -> Option < T > { self . disarm () . ok () } # [doc = " Look at the `Result` wrapped inside of this."] # [doc = " Must only be used to report the error!"] # [inline] pub fn report_err (self) -> Result < T , InterpErrorInfo < 'tcx > > { self . disarm () } # [inline] pub fn map < U > (self , f : impl FnOnce (T) -> U) -> InterpResult < 'tcx , U > { InterpResult :: new (self . disarm () . map (f)) } # [inline] pub fn map_err_info (self , f : impl FnOnce (InterpErrorInfo < 'tcx >) -> InterpErrorInfo < 'tcx > ,) -> InterpResult < 'tcx , T > { InterpResult :: new (self . disarm () . map_err (f)) } # [inline] pub fn map_err_kind (self , f : impl FnOnce (InterpErrorKind < 'tcx >) -> InterpErrorKind < 'tcx > ,) -> InterpResult < 'tcx , T > { InterpResult :: new (self . disarm () . map_err (| mut e | { e . 0 . kind = f (e . 0 . kind) ; e })) } # [inline] pub fn inspect_err_kind (self , f : impl FnOnce (& InterpErrorKind < 'tcx >)) -> InterpResult < 'tcx , T > { InterpResult :: new (self . disarm () . inspect_err (| e | f (& e . 0 . kind))) } # [inline] # [track_caller] pub fn unwrap (self) -> T { self . disarm () . unwrap () } # [inline] # [track_caller] pub fn unwrap_or_else (self , f : impl FnOnce (InterpErrorInfo < 'tcx >) -> T) -> T { self . disarm () . unwrap_or_else (f) } # [inline] # [track_caller] pub fn expect (self , msg : & str) -> T { self . disarm () . expect (msg) } # [inline] pub fn and_then < U > (self , f : impl FnOnce (T) -> InterpResult < 'tcx , U >) -> InterpResult < 'tcx , U > { InterpResult :: new (self . disarm () . and_then (| t | f (t) . disarm ())) } # [doc = " Returns success if both `self` and `other` succeed, while ensuring we don't"] # [doc = " accidentally drop an error."] # [doc = ""] # [doc = " If both are an error, `self` will be reported."] # [inline] pub fn and < U > (self , other : InterpResult < 'tcx , U >) -> InterpResult < 'tcx , (T , U) > { match self . disarm () { Ok (t) => interp_ok ((t , other ?)) , Err (e) => { drop (other . disarm ()) ; InterpResult :: new (Err (e)) } } } }}}

macro_rules! interp_ok_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function interp_ok in module {}", module_path!());
    };
}

mkfn!{
    interp_ok_introspect!();
    # [inline (always)] pub fn interp_ok < 'tcx , T > (x : T) -> InterpResult < 'tcx , T > { InterpResult :: new (Ok (x)) }
}