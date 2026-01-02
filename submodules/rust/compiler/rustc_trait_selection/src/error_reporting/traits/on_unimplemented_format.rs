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
mkuse!{use std :: fmt ;}
mkuse!{use std :: ops :: Range ;}
mkuse!{use errors :: * ;}
mkuse!{use rustc_middle :: ty :: print :: TraitRefPrintSugared ;}
mkuse!{use rustc_middle :: ty :: { GenericParamDefKind , TyCtxt } ;}
mkuse!{use rustc_parse_format :: { Argument , FormatSpec , ParseError , ParseMode , Parser , Piece as RpfPiece , Position , } ;}
mkuse!{use rustc_session :: lint :: builtin :: MALFORMED_DIAGNOSTIC_FORMAT_LITERALS ;}
mkuse!{use rustc_span :: def_id :: DefId ;}
mkuse!{use rustc_span :: { InnerSpan , Span , Symbol , kw , sym } ;}
mkitem!{mkstruct!{# [doc = " Like [std::fmt::Arguments] this is a string that has been parsed into \"pieces\","] # [doc = " either as string pieces or dynamic arguments."] # [derive (Debug)] pub struct FormatString { # [allow (dead_code , reason = "Debug impl")] input : Symbol , span : Span , pieces : Vec < Piece > , # [doc = " The formatting string was parsed successfully but with warnings"] pub warnings : Vec < FormatWarning > , }}}
mkitem!{mkenum!{# [derive (Debug)] enum Piece { Lit (String) , Arg (FormatArg) , }}}
mkitem!{mkenum!{# [derive (Debug)] enum FormatArg { GenericParam { generic_param : Symbol , } , SelfUpper , # [doc = " `{This}` or `{TraitName}`"] This , # [doc = " The sugared form of the trait"] Trait , # [doc = " what we're in, like a function, method, closure etc."] ItemContext , # [doc = " What the user typed, if it doesn't match anything we can use."] AsIs (String) , }}}
mkitem!{mkenum!{pub enum Ctx < 'tcx > { RustcOnUnimplemented { tcx : TyCtxt < 'tcx > , trait_def_id : DefId } , DiagnosticOnUnimplemented { tcx : TyCtxt < 'tcx > , trait_def_id : DefId } , }}}
mkitem!{mkenum!{# [derive (Debug)] pub enum FormatWarning { UnknownParam { argument_name : Symbol , span : Span } , PositionalArgument { span : Span , help : String } , InvalidSpecifier { name : String , span : Span } , FutureIncompat { span : Span , help : String } , }}}
mkitem!{mkimpl!{impl FormatWarning { pub fn emit_warning < 'tcx > (& self , tcx : TyCtxt < 'tcx > , item_def_id : DefId) { match * self { FormatWarning :: UnknownParam { argument_name , span } => { let this = tcx . item_ident (item_def_id) ; if let Some (item_def_id) = item_def_id . as_local () { tcx . emit_node_span_lint (MALFORMED_DIAGNOSTIC_FORMAT_LITERALS , tcx . local_def_id_to_hir_id (item_def_id) , span , UnknownFormatParameterForOnUnimplementedAttr { argument_name , trait_name : this , } ,) ; } } FormatWarning :: PositionalArgument { span , .. } => { if let Some (item_def_id) = item_def_id . as_local () { tcx . emit_node_span_lint (MALFORMED_DIAGNOSTIC_FORMAT_LITERALS , tcx . local_def_id_to_hir_id (item_def_id) , span , DisallowedPositionalArgument ,) ; } } FormatWarning :: InvalidSpecifier { span , .. } => { if let Some (item_def_id) = item_def_id . as_local () { tcx . emit_node_span_lint (MALFORMED_DIAGNOSTIC_FORMAT_LITERALS , tcx . local_def_id_to_hir_id (item_def_id) , span , InvalidFormatSpecifier ,) ; } } FormatWarning :: FutureIncompat { .. } => { } } } }}}
mkitem!{mkstruct!{# [doc = " Arguments to fill a [FormatString] with."] # [doc = ""] # [doc = " For example, given a"] # [doc = " ```rust,ignore (just an example)"] # [doc = ""] # [doc = " #[rustc_on_unimplemented("] # [doc = "     on(all(from_desugaring = \"QuestionMark\"),"] # [doc = "         message = \"the `?` operator can only be used in {ItemContext} \\"] # [doc = "                     that returns `Result` or `Option` \\"] # [doc = "                     (or another type that implements `{FromResidual}`)\","] # [doc = "         label = \"cannot use the `?` operator in {ItemContext} that returns `{Self}`\","] # [doc = "         parent_label = \"this function should return `Result` or `Option` to accept `?`\""] # [doc = "     ),"] # [doc = " )]"] # [doc = " pub trait FromResidual<R = <Self as Try>::Residual> {"] # [doc = "    ..."] # [doc = " }"] # [doc = ""] # [doc = " async fn an_async_function() -> u32 {"] # [doc = "     let x: Option<u32> = None;"] # [doc = "     x?; //~ ERROR the `?` operator"] # [doc = "     22"] # [doc = " }"] # [doc = "  ```"] # [doc = " it will look like this:"] # [doc = ""] # [doc = " ```rust,ignore (just an example)"] # [doc = " FormatArgs {"] # [doc = "     this: \"FromResidual\","] # [doc = "     trait_sugared: \"FromResidual<Option<Infallible>>\","] # [doc = "     item_context: \"an async function\","] # [doc = "     generic_args: [(\"Self\", \"u32\"), (\"R\", \"Option<Infallible>\")],"] # [doc = " }"] # [doc = " ```"] # [derive (Debug)] pub struct FormatArgs < 'tcx > { pub this : String , pub trait_sugared : TraitRefPrintSugared < 'tcx > , pub item_context : & 'static str , pub generic_args : Vec < (Symbol , String) > , }}}
mkitem!{mkimpl!{impl FormatString { pub fn span (& self) -> Span { self . span } pub fn parse < 'tcx > (input : Symbol , snippet : Option < String > , span : Span , ctx : & Ctx < 'tcx > ,) -> Result < Self , ParseError > { let s = input . as_str () ; let mut parser = Parser :: new (s , None , snippet , false , ParseMode :: Diagnostic) ; let pieces : Vec < _ > = parser . by_ref () . collect () ; if let Some (err) = parser . errors . into_iter () . next () { return Err (err) ; } let mut warnings = Vec :: new () ; let pieces = pieces . into_iter () . map (| piece | match piece { RpfPiece :: Lit (lit) => Piece :: Lit (lit . into ()) , RpfPiece :: NextArgument (arg) => { warn_on_format_spec (& arg . format , & mut warnings , span , parser . is_source_literal) ; let arg = parse_arg (& arg , ctx , & mut warnings , span , parser . is_source_literal) ; Piece :: Arg (arg) } }) . collect () ; Ok (FormatString { input , pieces , span , warnings }) } pub fn format (& self , args : & FormatArgs < '_ >) -> String { let mut ret = String :: new () ; for piece in & self . pieces { match piece { Piece :: Lit (s) | Piece :: Arg (FormatArg :: AsIs (s)) => ret . push_str (& s) , Piece :: Arg (FormatArg :: GenericParam { generic_param }) => { let value = match args . generic_args . iter () . find (| (p , _) | p == generic_param) { Some ((_ , val)) => val . to_string () , None => generic_param . to_string () , } ; ret . push_str (& value) ; } Piece :: Arg (FormatArg :: SelfUpper) => { let slf = match args . generic_args . iter () . find (| (p , _) | * p == kw :: SelfUpper) { Some ((_ , val)) => val . to_string () , None => "Self" . to_string () , } ; ret . push_str (& slf) ; } Piece :: Arg (FormatArg :: This) => ret . push_str (& args . this) , Piece :: Arg (FormatArg :: Trait) => { let _ = fmt :: write (& mut ret , format_args ! ("{}" , & args . trait_sugared)) ; } Piece :: Arg (FormatArg :: ItemContext) => ret . push_str (args . item_context) , } } ret } }}}

macro_rules! parse_arg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_arg in module {}", module_path!());
    };
}

mkfn!{
    parse_arg_introspect!();
    fn parse_arg < 'tcx > (arg : & Argument < '_ > , ctx : & Ctx < 'tcx > , warnings : & mut Vec < FormatWarning > , input_span : Span , is_source_literal : bool ,) -> FormatArg { let (Ctx :: RustcOnUnimplemented { tcx , trait_def_id } | Ctx :: DiagnosticOnUnimplemented { tcx , trait_def_id }) = ctx ; let span = slice_span (input_span , arg . position_span . clone () , is_source_literal) ; match arg . position { Position :: ArgumentNamed (name) => match (ctx , Symbol :: intern (name)) { (Ctx :: RustcOnUnimplemented { .. } , sym :: ItemContext) => FormatArg :: ItemContext , (Ctx :: RustcOnUnimplemented { .. } , sym :: This) => FormatArg :: This , (Ctx :: RustcOnUnimplemented { .. } , sym :: Trait) => FormatArg :: Trait , (Ctx :: RustcOnUnimplemented { .. } | Ctx :: DiagnosticOnUnimplemented { .. } , kw :: SelfUpper ,) => FormatArg :: SelfUpper , (Ctx :: RustcOnUnimplemented { .. } | Ctx :: DiagnosticOnUnimplemented { .. } , generic_param ,) if tcx . generics_of (trait_def_id) . own_params . iter () . any (| param | { ! matches ! (param . kind , GenericParamDefKind :: Lifetime) && param . name == generic_param }) => { FormatArg :: GenericParam { generic_param } } (_ , argument_name) => { warnings . push (FormatWarning :: UnknownParam { argument_name , span }) ; FormatArg :: AsIs (format ! ("{{{}}}" , argument_name . as_str ())) } } , Position :: ArgumentIs (idx) => { warnings . push (FormatWarning :: PositionalArgument { span , help : format ! ("use `{{{idx}}}` to print a number in braces") , }) ; FormatArg :: AsIs (format ! ("{{{idx}}}")) } Position :: ArgumentImplicitlyIs (_) => { warnings . push (FormatWarning :: PositionalArgument { span , help : String :: from ("use `{{}}` to print empty braces") , }) ; FormatArg :: AsIs (String :: from ("{}")) } } }
}

macro_rules! warn_on_format_spec_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function warn_on_format_spec in module {}", module_path!());
    };
}

mkfn!{
    warn_on_format_spec_introspect!();
    # [doc = " `#[rustc_on_unimplemented]` and `#[diagnostic::...]` don't actually do anything"] # [doc = " with specifiers, so emit a warning if they are used."] fn warn_on_format_spec (spec : & FormatSpec < '_ > , warnings : & mut Vec < FormatWarning > , input_span : Span , is_source_literal : bool ,) { if spec . ty != "" { let span = spec . ty_span . as_ref () . map (| inner | slice_span (input_span , inner . clone () , is_source_literal)) . unwrap_or (input_span) ; warnings . push (FormatWarning :: InvalidSpecifier { span , name : spec . ty . into () }) } }
}

macro_rules! slice_span_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function slice_span in module {}", module_path!());
    };
}

mkfn!{
    slice_span_introspect!();
    fn slice_span (input : Span , Range { start , end } : Range < usize > , is_source_literal : bool) -> Span { if is_source_literal { input . from_inner (InnerSpan { start , end }) } else { input } }
}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                mkuse!{use rustc_macros :: LintDiagnostic ;}
mkuse!{use rustc_span :: Ident ;}
mkuse!{use super :: * ;}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (trait_selection_unknown_format_parameter_for_on_unimplemented_attr)] # [help] pub struct UnknownFormatParameterForOnUnimplementedAttr { pub argument_name : Symbol , pub trait_name : Ident , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (trait_selection_disallowed_positional_argument)] # [help] pub struct DisallowedPositionalArgument ;}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (trait_selection_invalid_format_specifier)] # [help] pub struct InvalidFormatSpecifier ;}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (trait_selection_missing_options_for_on_unimplemented_attr)] # [help] pub struct MissingOptionsForOnUnimplementedAttr ;}} 
            }}