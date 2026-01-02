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
mkuse!{use rustc_errors :: Applicability :: { MachineApplicable , MaybeIncorrect } ;}
mkuse!{use rustc_errors :: { Diag , MultiSpan , pluralize } ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: find_attr ;}
mkuse!{use rustc_middle :: traits :: { ObligationCause , ObligationCauseCode } ;}
mkuse!{use rustc_middle :: ty :: error :: { ExpectedFound , TypeError } ;}
mkuse!{use rustc_middle :: ty :: fast_reject :: DeepRejectCtxt ;}
mkuse!{use rustc_middle :: ty :: print :: { FmtPrinter , Printer } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , suggest_constraining_type_param } ;}
mkuse!{use rustc_span :: def_id :: DefId ;}
mkuse!{use rustc_span :: { BytePos , Span , Symbol } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: error_reporting :: TypeErrCtxt ;}
mkuse!{use crate :: infer :: InferCtxtExt ;}
mkitem!{mkimpl!{impl < 'tcx > TypeErrCtxt < '_ , 'tcx > { pub fn note_and_explain_type_err (& self , diag : & mut Diag < '_ > , err : TypeError < 'tcx > , cause : & ObligationCause < 'tcx > , sp : Span , body_owner_def_id : DefId ,) { debug ! ("note_and_explain_type_err err={:?} cause={:?}" , err , cause) ; let tcx = self . tcx ; match err { TypeError :: ArgumentSorts (values , _) | TypeError :: Sorts (values) => { match (* values . expected . kind () , * values . found . kind ()) { (ty :: Closure (..) , ty :: Closure (..)) => { diag . note ("no two closures, even if identical, have the same type") ; diag . help ("consider boxing your closure and/or using it as a trait object") ; } (ty :: Coroutine (def_id1 , ..) , ty :: Coroutine (def_id2 , ..)) if self . tcx . coroutine_is_async (def_id1) && self . tcx . coroutine_is_async (def_id2) => { diag . note ("no two async blocks, even if identical, have the same type") ; diag . help ("consider pinning your async block and casting it to a trait object" ,) ; } (ty :: Alias (ty :: Opaque , ..) , ty :: Alias (ty :: Opaque , ..)) => { diag . note ("distinct uses of `impl Trait` result in different opaque types") ; } (ty :: Float (_) , ty :: Infer (ty :: IntVar (_))) if let Ok (snippet ,) = tcx . sess . source_map () . span_to_snippet (sp) => { if snippet . chars () . all (| c | c . is_digit (10) || c == '-' || c == '_') { diag . span_suggestion_verbose (sp . shrink_to_hi () , "use a float literal" , ".0" , MachineApplicable ,) ; } } (ty :: Param (expected) , ty :: Param (found)) => { let generics = tcx . generics_of (body_owner_def_id) ; let e_span = tcx . def_span (generics . type_param (expected , tcx) . def_id) ; if ! sp . contains (e_span) { diag . span_label (e_span , "expected type parameter") ; } let f_span = tcx . def_span (generics . type_param (found , tcx) . def_id) ; if ! sp . contains (f_span) { diag . span_label (f_span , "found type parameter") ; } diag . note ("a type parameter was expected, but a different one was found; \
                             you might be missing a type parameter or trait bound" ,) ; diag . note ("for more information, visit \
                             https://doc.rust-lang.org/book/ch10-02-traits.html\
                             #traits-as-parameters" ,) ; } (ty :: Alias (ty :: Projection | ty :: Inherent , _) , ty :: Alias (ty :: Projection | ty :: Inherent , _) ,) => { diag . note ("an associated type was expected, but a different one was found") ; } (ty :: Param (p) , ty :: Alias (ty :: Projection , proj)) | (ty :: Alias (ty :: Projection , proj) , ty :: Param (p)) if ! tcx . is_impl_trait_in_trait (proj . def_id) => { let param = tcx . generics_of (body_owner_def_id) . type_param (p , tcx) ; let p_def_id = param . def_id ; let p_span = tcx . def_span (p_def_id) ; let expected = match (values . expected . kind () , values . found . kind ()) { (ty :: Param (_) , _) => "expected " , (_ , ty :: Param (_)) => "found " , _ => "" , } ; if ! sp . contains (p_span) { diag . span_label (p_span , format ! ("{expected}this type parameter")) ; } let parent = p_def_id . as_local () . and_then (| id | { let local_id = tcx . local_def_id_to_hir_id (id) ; let generics = tcx . parent_hir_node (local_id) . generics () ? ; Some ((id , generics)) }) ; let mut note = true ; if let Some ((local_id , generics)) = parent { let (trait_ref , assoc_args) = proj . trait_ref_and_own_args (tcx) ; let item_name = tcx . item_name (proj . def_id) ; let item_args = self . format_generic_args (assoc_args) ; let mut matching_span = None ; let mut matched_end_of_args = false ; for bound in generics . bounds_for_param (local_id) { let potential_spans = bound . bounds . iter () . find_map (| bound | { let bound_trait_path = bound . trait_ref () ? . path ; let def_id = bound_trait_path . res . opt_def_id () ? ; let generic_args = bound_trait_path . segments . iter () . last () . map (| path | path . args ()) ; (def_id == trait_ref . def_id) . then_some ((bound_trait_path . span , generic_args)) }) ; if let Some ((end_of_trait , end_of_args)) = potential_spans { let args_span = end_of_args . and_then (| args | args . span ()) ; matched_end_of_args = args_span . is_some () ; matching_span = args_span . or_else (| | Some (end_of_trait)) . map (| span | span . shrink_to_hi ()) ; break ; } } if matched_end_of_args { let path = format ! (", {item_name}{item_args} = {p}") ; note = ! suggest_constraining_type_param (tcx , generics , diag , & proj . self_ty () . to_string () , & path , None , matching_span ,) ; } else { let path = format ! ("<{item_name}{item_args} = {p}>") ; note = ! suggest_constraining_type_param (tcx , generics , diag , & proj . self_ty () . to_string () , & path , None , matching_span ,) ; } } if note { diag . note ("you might be missing a type parameter or trait bound") ; } } (ty :: Param (p) , ty :: Dynamic (..) | ty :: Alias (ty :: Opaque , ..)) | (ty :: Dynamic (..) | ty :: Alias (ty :: Opaque , ..) , ty :: Param (p)) => { let generics = tcx . generics_of (body_owner_def_id) ; let p_span = tcx . def_span (generics . type_param (p , tcx) . def_id) ; let expected = match (values . expected . kind () , values . found . kind ()) { (ty :: Param (_) , _) => "expected " , (_ , ty :: Param (_)) => "found " , _ => "" , } ; if ! sp . contains (p_span) { diag . span_label (p_span , format ! ("{expected}this type parameter")) ; } diag . help ("type parameters must be constrained to match other types") ; if diag . code . is_some_and (| code | tcx . sess . teach (code)) { diag . help ("given a type parameter `T` and a method `foo`:
```
trait Trait<T> { fn foo(&self) -> T; }
```
the only ways to implement method `foo` are:
- constrain `T` with an explicit type:
```
impl Trait<String> for X {
    fn foo(&self) -> String { String::new() }
}
```
- add a trait bound to `T` and call a method on that trait that returns `Self`:
```
impl<T: std::default::Default> Trait<T> for X {
    fn foo(&self) -> T { <T as std::default::Default>::default() }
}
```
- change `foo` to return an argument of type `T`:
```
impl<T> Trait<T> for X {
    fn foo(&self, x: T) -> T { x }
}
```" ,) ; } diag . note ("for more information, visit \
                             https://doc.rust-lang.org/book/ch10-02-traits.html\
                             #traits-as-parameters" ,) ; } (ty :: Param (p) , ty :: Closure (..) | ty :: CoroutineClosure (..) | ty :: Coroutine (..) ,) => { let generics = tcx . generics_of (body_owner_def_id) ; let p_span = tcx . def_span (generics . type_param (p , tcx) . def_id) ; if ! sp . contains (p_span) { diag . span_label (p_span , "expected this type parameter") ; } diag . help (format ! ("every closure has a distinct type and so could not always match the \
                             caller-chosen type of parameter `{p}`")) ; } (ty :: Param (p) , _) | (_ , ty :: Param (p)) => { let generics = tcx . generics_of (body_owner_def_id) ; let p_span = tcx . def_span (generics . type_param (p , tcx) . def_id) ; let expected = match (values . expected . kind () , values . found . kind ()) { (ty :: Param (_) , _) => "expected " , (_ , ty :: Param (_)) => "found " , _ => "" , } ; if ! sp . contains (p_span) { diag . span_label (p_span , format ! ("{expected}this type parameter")) ; } } (ty :: Alias (ty :: Projection | ty :: Inherent , proj_ty) , _) if ! tcx . is_impl_trait_in_trait (proj_ty . def_id) => { self . expected_projection (diag , proj_ty , values , body_owner_def_id , cause . code () ,) ; } (_ , ty :: Alias (ty :: Projection | ty :: Inherent , proj_ty)) if ! tcx . is_impl_trait_in_trait (proj_ty . def_id) => { let msg = | | { format ! ("consider constraining the associated type `{}` to `{}`" , values . found , values . expected ,) } ; if ! (self . suggest_constraining_opaque_associated_type (diag , msg , proj_ty , values . expected ,) || self . suggest_constraint (diag , & msg , body_owner_def_id , proj_ty , values . expected ,)) { diag . help (msg ()) ; diag . note ("for more information, visit \
                                https://doc.rust-lang.org/book/ch19-03-advanced-traits.html" ,) ; } } (ty :: Dynamic (t , _ , ty :: DynKind :: Dyn) , ty :: Alias (ty :: Opaque , alias)) if let Some (def_id) = t . principal_def_id () && tcx . explicit_item_self_bounds (alias . def_id) . skip_binder () . iter () . any (| (pred , _span) | match pred . kind () . skip_binder () { ty :: ClauseKind :: Trait (trait_predicate) if trait_predicate . polarity == ty :: PredicatePolarity :: Positive => { trait_predicate . def_id () == def_id } _ => false , }) => { diag . help (format ! ("you can box the `{}` to coerce it to `Box<{}>`, but you'll have to \
                             change the expected type as well" , values . found , values . expected ,)) ; } (ty :: Dynamic (t , _ , ty :: DynKind :: Dyn) , _) if let Some (def_id) = t . principal_def_id () => { let mut has_matching_impl = false ; tcx . for_each_relevant_impl (def_id , values . found , | did | { if DeepRejectCtxt :: relate_rigid_infer (tcx) . types_may_unify (values . found , tcx . type_of (did) . skip_binder ()) { has_matching_impl = true ; } }) ; if has_matching_impl { let trait_name = tcx . item_name (def_id) ; diag . help (format ! ("`{}` implements `{trait_name}` so you could box the found value \
                                 and coerce it to the trait object `Box<dyn {trait_name}>`, you \
                                 will have to change the expected type as well" , values . found ,)) ; } } (_ , ty :: Dynamic (t , _ , ty :: DynKind :: Dyn)) if let Some (def_id) = t . principal_def_id () => { let mut has_matching_impl = false ; tcx . for_each_relevant_impl (def_id , values . expected , | did | { if DeepRejectCtxt :: relate_rigid_infer (tcx) . types_may_unify (values . expected , tcx . type_of (did) . skip_binder ()) { has_matching_impl = true ; } }) ; if has_matching_impl { let trait_name = tcx . item_name (def_id) ; diag . help (format ! ("`{}` implements `{trait_name}` so you could change the expected \
                                 type to `Box<dyn {trait_name}>`" , values . expected ,)) ; } } (_ , ty :: Alias (ty :: Opaque , opaque_ty)) | (ty :: Alias (ty :: Opaque , opaque_ty) , _) => { if opaque_ty . def_id . is_local () && matches ! (tcx . def_kind (body_owner_def_id) , DefKind :: Fn | DefKind :: Static { .. } | DefKind :: Const | DefKind :: AssocFn | DefKind :: AssocConst) && matches ! (tcx . opaque_ty_origin (opaque_ty . def_id) , hir :: OpaqueTyOrigin :: TyAlias { .. }) && ! tcx . opaque_types_defined_by (body_owner_def_id . expect_local ()) . contains (& opaque_ty . def_id . expect_local ()) { let sp = tcx . def_ident_span (body_owner_def_id) . unwrap_or_else (| | tcx . def_span (body_owner_def_id)) ; let mut alias_def_id = opaque_ty . def_id ; while let DefKind :: OpaqueTy = tcx . def_kind (alias_def_id) { alias_def_id = tcx . parent (alias_def_id) ; } let opaque_path = tcx . def_path_str (alias_def_id) ; match tcx . opaque_ty_origin (opaque_ty . def_id) { rustc_hir :: OpaqueTyOrigin :: FnReturn { .. } => { } rustc_hir :: OpaqueTyOrigin :: AsyncFn { .. } => { } rustc_hir :: OpaqueTyOrigin :: TyAlias { in_assoc_ty : false , .. } => { diag . span_note (sp , format ! ("this item must have a `#[define_opaque({opaque_path})]` \
                                        attribute to be able to define hidden types") ,) ; } rustc_hir :: OpaqueTyOrigin :: TyAlias { in_assoc_ty : true , .. } => { } } } let ObligationCauseCode :: IfExpression { expr_id , .. } = cause . code () else { return ; } ; let hir :: Node :: Expr (& hir :: Expr { kind : hir :: ExprKind :: If (_ , & hir :: Expr { kind : hir :: ExprKind :: Block (& hir :: Block { expr : Some (then) , .. } , _ ,) , .. } , Some (& hir :: Expr { kind : hir :: ExprKind :: Block (& hir :: Block { expr : Some (else_) , .. } , _ ,) , .. }) ,) , .. }) = self . tcx . hir_node (* expr_id) else { return ; } ; let expected = match values . found . kind () { ty :: Alias (..) => values . expected , _ => values . found , } ; let preds = tcx . explicit_item_self_bounds (opaque_ty . def_id) ; for (pred , _span) in preds . skip_binder () { let ty :: ClauseKind :: Trait (trait_predicate) = pred . kind () . skip_binder () else { continue ; } ; if trait_predicate . polarity != ty :: PredicatePolarity :: Positive { continue ; } let def_id = trait_predicate . def_id () ; let mut impl_def_ids = vec ! [] ; tcx . for_each_relevant_impl (def_id , expected , | did | { impl_def_ids . push (did) }) ; if let [_] = & impl_def_ids [..] { let trait_name = tcx . item_name (def_id) ; diag . multipart_suggestion (format ! ("`{expected}` implements `{trait_name}` so you can box \
                                         both arms and coerce to the trait object \
                                         `Box<dyn {trait_name}>`" ,) , vec ! [(then . span . shrink_to_lo () , "Box::new(" . to_string ()) , (then . span . shrink_to_hi () , format ! (") as Box<dyn {}>" , tcx . def_path_str (def_id)) ,) , (else_ . span . shrink_to_lo () , "Box::new(" . to_string ()) , (else_ . span . shrink_to_hi () , ")" . to_string ()) ,] , MachineApplicable ,) ; } } } (ty :: FnPtr (_ , hdr) , ty :: FnDef (def_id , _)) | (ty :: FnDef (def_id , _) , ty :: FnPtr (_ , hdr)) => { if tcx . fn_sig (def_id) . skip_binder () . safety () < hdr . safety { if ! tcx . codegen_fn_attrs (def_id) . safe_target_features { diag . note ("unsafe functions cannot be coerced into safe function pointers" ,) ; } } } (ty :: Adt (_ , _) , ty :: Adt (def , args)) if let ObligationCauseCode :: IfExpression { expr_id , .. } = cause . code () && let hir :: Node :: Expr (if_expr) = self . tcx . hir_node (* expr_id) && let hir :: ExprKind :: If (_ , then_expr , _) = if_expr . kind && let hir :: ExprKind :: Block (blk , _) = then_expr . kind && let Some (then) = blk . expr && def . is_box () && let boxed_ty = args . type_at (0) && let ty :: Dynamic (t , _ , _) = boxed_ty . kind () && let Some (def_id) = t . principal_def_id () && let mut impl_def_ids = vec ! [] && let _ = tcx . for_each_relevant_impl (def_id , values . expected , | did | { impl_def_ids . push (did) }) && let [_] = & impl_def_ids [..] => { diag . multipart_suggestion (format ! ("`{}` implements `{}` so you can box it to coerce to the trait \
                                 object `{}`" , values . expected , tcx . item_name (def_id) , values . found ,) , vec ! [(then . span . shrink_to_lo () , "Box::new(" . to_string ()) , (then . span . shrink_to_hi () , ")" . to_string ()) ,] , MachineApplicable ,) ; } _ => { } } debug ! ("note_and_explain_type_err expected={:?} ({:?}) found={:?} ({:?})" , values . expected , values . expected . kind () , values . found , values . found . kind () ,) ; } TypeError :: CyclicTy (ty) => { if ty . is_closure () || ty . is_coroutine () || ty . is_coroutine_closure () { diag . note ("closures cannot capture themselves or take themselves as argument;\n\
                         this error may be the result of a recent compiler bug-fix,\n\
                         see issue #46062 <https://github.com/rust-lang/rust/issues/46062>\n\
                         for more information" ,) ; } } TypeError :: TargetFeatureCast (def_id) => { let target_spans = find_attr ! (tcx . get_all_attrs (def_id) , AttributeKind :: TargetFeature { attr_span : span , was_forced : false , .. } => * span) ; diag . note ("functions with `#[target_feature]` can only be coerced to `unsafe` function pointers") ; diag . span_labels (target_spans , "`#[target_feature]` added here") ; } _ => { } } } fn suggest_constraint (& self , diag : & mut Diag < '_ > , msg : impl Fn () -> String , body_owner_def_id : DefId , proj_ty : ty :: AliasTy < 'tcx > , ty : Ty < 'tcx > ,) -> bool { let tcx = self . tcx ; let assoc = tcx . associated_item (proj_ty . def_id) ; let (trait_ref , assoc_args) = proj_ty . trait_ref_and_own_args (tcx) ; let Some (item) = tcx . hir_get_if_local (body_owner_def_id) else { return false ; } ; let Some (hir_generics) = item . generics () else { return false ; } ; let ty :: Param (param_ty) = * proj_ty . self_ty () . kind () else { return false ; } ; let generics = tcx . generics_of (body_owner_def_id) ; let def_id = generics . type_param (param_ty , tcx) . def_id ; let Some (def_id) = def_id . as_local () else { return false ; } ; for pred in hir_generics . bounds_for_param (def_id) { if self . constrain_generic_bound_associated_type_structured_suggestion (diag , trait_ref , pred . bounds , assoc , assoc_args , ty , & msg , false ,) { return true ; } } if (param_ty . index as usize) >= generics . parent_count { return false ; } let hir_id = match item { hir :: Node :: ImplItem (item) => item . hir_id () , hir :: Node :: TraitItem (item) => item . hir_id () , _ => return false , } ; let parent = tcx . hir_get_parent_item (hir_id) . def_id ; self . suggest_constraint (diag , msg , parent . into () , proj_ty , ty) } # [doc = " An associated type was expected and a different type was found."] # [doc = ""] # [doc = " We perform a few different checks to see what we can suggest:"] # [doc = ""] # [doc = "  - In the current item, look for associated functions that return the expected type and"] # [doc = "    suggest calling them. (Not a structured suggestion.)"] # [doc = "  - If any of the item's generic bounds can be constrained, we suggest constraining the"] # [doc = "    associated type to the found type."] # [doc = "  - If the associated type has a default type and was expected inside of a `trait`, we"] # [doc = "    mention that this is disallowed."] # [doc = "  - If all other things fail, and the error is not because of a mismatch between the `trait`"] # [doc = "    and the `impl`, we provide a generic `help` to constrain the assoc type or call an assoc"] # [doc = "    fn that returns the type."] fn expected_projection (& self , diag : & mut Diag < '_ > , proj_ty : ty :: AliasTy < 'tcx > , values : ExpectedFound < Ty < 'tcx > > , body_owner_def_id : DefId , cause_code : & ObligationCauseCode < '_ > ,) { let tcx = self . tcx ; if self . tcx . erase_and_anonymize_regions (values . found) . contains (self . tcx . erase_and_anonymize_regions (values . expected)) { return ; } let msg = | | { format ! ("consider constraining the associated type `{}` to `{}`" , values . expected , values . found) } ; let body_owner = tcx . hir_get_if_local (body_owner_def_id) ; let current_method_ident = body_owner . and_then (| n | n . ident ()) . map (| i | i . name) ; let callable_scope = matches ! (body_owner , Some (hir :: Node :: Item (hir :: Item { kind : hir :: ItemKind :: Fn { .. } , .. }) | hir :: Node :: TraitItem (hir :: TraitItem { kind : hir :: TraitItemKind :: Fn (..) , .. }) | hir :: Node :: ImplItem (hir :: ImplItem { kind : hir :: ImplItemKind :: Fn (..) , .. }) ,)) ; let impl_comparison = matches ! (cause_code , ObligationCauseCode :: CompareImplItem { .. }) ; if impl_comparison { } else { let point_at_assoc_fn = if callable_scope && self . point_at_methods_that_satisfy_associated_type (diag , tcx . parent (proj_ty . def_id) , current_method_ident , proj_ty . def_id , values . expected ,) { true } else { false } ; if self . suggest_constraint (diag , & msg , body_owner_def_id , proj_ty , values . found) || point_at_assoc_fn { return ; } } self . suggest_constraining_opaque_associated_type (diag , & msg , proj_ty , values . found) ; if self . point_at_associated_type (diag , body_owner_def_id , values . found) { return ; } if ! impl_comparison { if callable_scope { diag . help (format ! ("{} or calling a method that returns `{}`" , msg () , values . expected)) ; } else { diag . help (msg ()) ; } diag . note ("for more information, visit \
                 https://doc.rust-lang.org/book/ch19-03-advanced-traits.html" ,) ; } if diag . code . is_some_and (| code | tcx . sess . teach (code)) { diag . help ("given an associated type `T` and a method `foo`:
```
trait Trait {
type T;
fn foo(&self) -> Self::T;
}
```
the only way of implementing method `foo` is to constrain `T` with an explicit associated type:
```
impl Trait for X {
type T = String;
fn foo(&self) -> Self::T { String::new() }
}
```" ,) ; } } # [doc = " When the expected `impl Trait` is not defined in the current item, it will come from"] # [doc = " a return type. This can occur when dealing with `TryStream` (#71035)."] fn suggest_constraining_opaque_associated_type (& self , diag : & mut Diag < '_ > , msg : impl Fn () -> String , proj_ty : ty :: AliasTy < 'tcx > , ty : Ty < 'tcx > ,) -> bool { let tcx = self . tcx ; let assoc = tcx . associated_item (proj_ty . def_id) ; if let ty :: Alias (ty :: Opaque , ty :: AliasTy { def_id , .. }) = * proj_ty . self_ty () . kind () { let opaque_local_def_id = def_id . as_local () ; let opaque_hir_ty = if let Some (opaque_local_def_id) = opaque_local_def_id { tcx . hir_expect_opaque_ty (opaque_local_def_id) } else { return false ; } ; let (trait_ref , assoc_args) = proj_ty . trait_ref_and_own_args (tcx) ; self . constrain_generic_bound_associated_type_structured_suggestion (diag , trait_ref , opaque_hir_ty . bounds , assoc , assoc_args , ty , msg , true ,) } else { false } } fn point_at_methods_that_satisfy_associated_type (& self , diag : & mut Diag < '_ > , assoc_container_id : DefId , current_method_ident : Option < Symbol > , proj_ty_item_def_id : DefId , expected : Ty < 'tcx > ,) -> bool { let tcx = self . tcx ; let items = tcx . associated_items (assoc_container_id) ; let methods : Vec < (Span , String) > = items . in_definition_order () . filter (| item | { item . is_fn () && Some (item . name ()) != current_method_ident && ! tcx . is_doc_hidden (item . def_id) }) . filter_map (| item | { let method = tcx . fn_sig (item . def_id) . instantiate_identity () ; match * method . output () . skip_binder () . kind () { ty :: Alias (ty :: Projection , ty :: AliasTy { def_id : item_def_id , .. }) if item_def_id == proj_ty_item_def_id => { Some ((tcx . def_span (item . def_id) , format ! ("consider calling `{}`" , tcx . def_path_str (item . def_id)) ,)) } _ => None , } }) . collect () ; if ! methods . is_empty () { let mut span : MultiSpan = methods . iter () . map (| (sp , _) | * sp) . collect :: < Vec < Span > > () . into () ; let msg = format ! ("{some} method{s} {are} available that return{r} `{ty}`" , some = if methods . len () == 1 { "a" } else { "some" } , s = pluralize ! (methods . len ()) , are = pluralize ! ("is" , methods . len ()) , r = if methods . len () == 1 { "s" } else { "" } , ty = expected) ; for (sp , label) in methods . into_iter () { span . push_span_label (sp , label) ; } diag . span_help (span , msg) ; return true ; } false } fn point_at_associated_type (& self , diag : & mut Diag < '_ > , body_owner_def_id : DefId , found : Ty < 'tcx > ,) -> bool { let tcx = self . tcx ; let Some (def_id) = body_owner_def_id . as_local () else { return false ; } ; let hir_id = tcx . local_def_id_to_hir_id (def_id) ; let parent_id = tcx . hir_get_parent_item (hir_id) ; let item = tcx . hir_node_by_def_id (parent_id . def_id) ; debug ! ("expected_projection parent item {:?}" , item) ; let param_env = tcx . param_env (body_owner_def_id) ; if let DefKind :: Trait | DefKind :: Impl { .. } = tcx . def_kind (parent_id) { let assoc_items = tcx . associated_items (parent_id) ; for assoc_item in assoc_items . in_definition_order () { if assoc_item . is_type () && let hir :: Defaultness :: Default { has_value : true } = assoc_item . defaultness (tcx) && let assoc_ty = tcx . type_of (assoc_item . def_id) . instantiate_identity () && self . infcx . can_eq (param_env , assoc_ty , found) { let msg = match assoc_item . container { ty :: AssocContainer :: Trait => { "associated type defaults can't be assumed inside the \
                                            trait defining them" } ty :: AssocContainer :: InherentImpl | ty :: AssocContainer :: TraitImpl (_) => { "associated type is `default` and may be overridden" } } ; diag . span_label (tcx . def_span (assoc_item . def_id) , msg) ; return true ; } } } false } # [doc = " Given a slice of `hir::GenericBound`s, if any of them corresponds to the `trait_ref`"] # [doc = " requirement, provide a structured suggestion to constrain it to a given type `ty`."] # [doc = ""] # [doc = " `is_bound_surely_present` indicates whether we know the bound we're looking for is"] # [doc = " inside `bounds`. If that's the case then we can consider `bounds` containing only one"] # [doc = " trait bound as the one we're looking for. This can help in cases where the associated"] # [doc = " type is defined on a supertrait of the one present in the bounds."] fn constrain_generic_bound_associated_type_structured_suggestion (& self , diag : & mut Diag < '_ > , trait_ref : ty :: TraitRef < 'tcx > , bounds : hir :: GenericBounds < '_ > , assoc : ty :: AssocItem , assoc_args : & [ty :: GenericArg < 'tcx >] , ty : Ty < 'tcx > , msg : impl Fn () -> String , is_bound_surely_present : bool ,) -> bool { let trait_bounds = bounds . iter () . filter_map (| bound | match bound { hir :: GenericBound :: Trait (ptr) if ptr . modifiers == hir :: TraitBoundModifiers :: NONE => { Some (ptr) } _ => None , }) ; let matching_trait_bounds = trait_bounds . clone () . filter (| ptr | ptr . trait_ref . trait_def_id () == Some (trait_ref . def_id)) . collect :: < Vec < _ > > () ; let span = match & matching_trait_bounds [..] { & [ptr] => ptr . span , & [] if is_bound_surely_present => match & trait_bounds . collect :: < Vec < _ > > () [..] { & [ptr] => ptr . span , _ => return false , } , _ => return false , } ; self . constrain_associated_type_structured_suggestion (diag , span , assoc , assoc_args , ty , msg) } # [doc = " Given a span corresponding to a bound, provide a structured suggestion to set an"] # [doc = " associated type to a given type `ty`."] fn constrain_associated_type_structured_suggestion (& self , diag : & mut Diag < '_ > , span : Span , assoc : ty :: AssocItem , assoc_args : & [ty :: GenericArg < 'tcx >] , ty : Ty < 'tcx > , msg : impl Fn () -> String ,) -> bool { let tcx = self . tcx ; if let Ok (has_params) = tcx . sess . source_map () . span_to_snippet (span) . map (| snippet | snippet . ends_with ('>')) { let (span , sugg) = if has_params { let pos = span . hi () - BytePos (1) ; let span = Span :: new (pos , pos , span . ctxt () , span . parent ()) ; (span , format ! (", {} = {}" , assoc . ident (tcx) , ty)) } else { let item_args = self . format_generic_args (assoc_args) ; (span . shrink_to_hi () , format ! ("<{}{} = {}>" , assoc . ident (tcx) , item_args , ty)) } ; diag . span_suggestion_verbose (span , msg () , sugg , MaybeIncorrect) ; return true ; } false } pub fn format_generic_args (& self , args : & [ty :: GenericArg < 'tcx >]) -> String { FmtPrinter :: print_string (self . tcx , hir :: def :: Namespace :: TypeNS , | p | { p . print_path_with_generic_args (| _ | Ok (()) , args) }) . expect ("could not write to `String`.") } }}}