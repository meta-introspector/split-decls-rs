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
mkuse!{use std :: mem ;}
mkuse!{use ast :: token :: IdentIsRaw ;}
mkuse!{use rustc_ast :: token :: { self , MetaVarKind , Token , TokenKind } ;}
mkuse!{use rustc_ast :: { self as ast , AngleBracketedArg , AngleBracketedArgs , AnonConst , AssocItemConstraint , AssocItemConstraintKind , BlockCheckMode , GenericArg , GenericArgs , Generics , ParenthesizedArgs , Path , PathSegment , QSelf , } ;}
mkuse!{use rustc_errors :: { Applicability , Diag , PResult } ;}
mkuse!{use rustc_span :: { BytePos , Ident , Span , kw , sym } ;}
mkuse!{use thin_vec :: ThinVec ;}
mkuse!{use tracing :: debug ;}
mkuse!{use super :: ty :: { AllowPlus , RecoverQPath , RecoverReturnSign } ;}
mkuse!{use super :: { Parser , Restrictions , TokenType } ;}
mkuse!{use crate :: ast :: { PatKind , TyKind } ;}
mkuse!{use crate :: errors :: { self , AttributeOnEmptyType , AttributeOnGenericArg , FnPathFoundNamedParams , PathFoundAttributeInParams , PathFoundCVariadicParams , PathSingleColon , PathTripleColon , } ;}
mkuse!{use crate :: exp ;}
mkuse!{use crate :: parser :: { CommaRecoveryMode , ExprKind , FnContext , FnParseMode , RecoverColon , RecoverComma , } ;}
mkitem!{mkenum!{#[doc = " Specifies how to parse a path."] #[derive (Copy , Clone , PartialEq)] pub enum PathStyle { #[doc = " In some contexts, notably in expressions, paths with generic arguments are ambiguous"] #[doc = " with something else. For example, in expressions `segment < ....` can be interpreted"] #[doc = " as a comparison and `segment ( ....` can be interpreted as a function call."] #[doc = " In all such contexts the non-path interpretation is preferred by default for practical"] #[doc = " reasons, but the path interpretation can be forced by the disambiguator `::`, e.g."] #[doc = " `x<y>` - comparisons, `x::<y>` - unambiguously a path."] #[doc = ""] #[doc = " Also, a path may never be followed by a `:`. This means that we can eagerly recover if"] #[doc = " we encounter it."] Expr , #[doc = " The same as `Expr`, but may be followed by a `:`."] #[doc = " For example, this code:"] #[doc = " ```rust"] #[doc = " struct S;"] #[doc = ""] #[doc = " let S: S;"] #[doc = " //  ^ Followed by a `:`"] #[doc = " ```"] Pat , #[doc = " In other contexts, notably in types, no ambiguity exists and paths can be written"] #[doc = " without the disambiguator, e.g., `x<y>` - unambiguously a path."] #[doc = " Paths with disambiguators are still accepted, `x::<Y>` - unambiguously a path too."] Type , #[doc = " A path with generic arguments disallowed, e.g., `foo::bar::Baz`, used in imports,"] #[doc = " visibilities or attributes."] #[doc = " Technically, this variant is unnecessary and e.g., `Expr` can be used instead"] #[doc = " (paths in \"mod\" contexts have to be checked later for absence of generic arguments"] #[doc = " anyway, due to macros), but it is used to avoid weird suggestions about expected"] #[doc = " tokens when something goes wrong."] Mod , }}}
mkitem!{mkimpl!{impl PathStyle { fn has_generic_ambiguity (& self) -> bool { matches ! (self , Self :: Expr | Self :: Pat) } }}}
mkitem!{mkimpl!{impl < 'a > Parser < 'a > { #[doc = " Parses a qualified path."] #[doc = " Assumes that the leading `<` has been parsed already."] #[doc = ""] #[doc = " `qualified_path = <type [as trait_ref]>::path`"] #[doc = ""] #[doc = " # Examples"] #[doc = " `<T>::default`"] #[doc = " `<T as U>::a`"] #[doc = " `<T as U>::F::a<S>` (without disambiguator)"] #[doc = " `<T as U>::F::a::<S>` (with disambiguator)"] pub (super) fn parse_qpath (& mut self , style : PathStyle) -> PResult < 'a , (Box < QSelf > , Path) > { let lo = self . prev_token . span ; let ty = self . parse_ty () ? ; let (mut path , path_span) ; if self . eat_keyword (exp ! (As)) { let path_lo = self . token . span ; path = self . parse_path (PathStyle :: Type) ? ; path_span = path_lo . to (self . prev_token . span) ; } else { path_span = self . token . span . to (self . token . span) ; path = ast :: Path { segments : ThinVec :: new () , span : path_span , tokens : None } ; } self . expect (exp ! (Gt)) ? ; if self . unmatched_angle_bracket_count > 0 { self . unmatched_angle_bracket_count -= 1 ; debug ! ("parse_qpath: (decrement) count={:?}" , self . unmatched_angle_bracket_count) ; } let is_import_coupler = self . is_import_coupler () ; if ! is_import_coupler && ! self . recover_colon_before_qpath_proj () { self . expect (exp ! (PathSep)) ? ; } let qself = Box :: new (QSelf { ty , path_span , position : path . segments . len () }) ; if ! is_import_coupler { self . parse_path_segments (& mut path . segments , style , None) ? ; } Ok ((qself , Path { segments : path . segments , span : lo . to (self . prev_token . span) , tokens : None } ,)) } #[doc = " Recover from an invalid single colon, when the user likely meant a qualified path."] #[doc = " We avoid emitting this if not followed by an identifier, as our assumption that the user"] #[doc = " intended this to be a qualified path may not be correct."] #[doc = ""] #[doc = " ```ignore (diagnostics)"] #[doc = " <Bar as Baz<T>>:Qux"] #[doc = "                ^ help: use double colon"] #[doc = " ```"] fn recover_colon_before_qpath_proj (& mut self) -> bool { if ! self . check_noexpect (& TokenKind :: Colon) || self . look_ahead (1 , | t | ! t . is_non_reserved_ident ()) { return false ; } self . bump () ; self . dcx () . struct_span_err (self . prev_token . span , "found single colon before projection in qualified path" ,) . with_span_suggestion (self . prev_token . span , "use double colon" , "::" , Applicability :: MachineApplicable ,) . emit () ; true } pub fn parse_path (& mut self , style : PathStyle) -> PResult < 'a , Path > { self . parse_path_inner (style , None) } #[doc = " Parses simple paths."] #[doc = ""] #[doc = " `path = [::] segment+`"] #[doc = " `segment = ident | ident[::]<args> | ident[::](args) [-> type]`"] #[doc = ""] #[doc = " # Examples"] #[doc = " `a::b::C<D>` (without disambiguator)"] #[doc = " `a::b::C::<D>` (with disambiguator)"] #[doc = " `Fn(Args)` (without disambiguator)"] #[doc = " `Fn::(Args)` (with disambiguator)"] pub (super) fn parse_path_inner (& mut self , style : PathStyle , ty_generics : Option < & Generics > ,) -> PResult < 'a , Path > { let reject_generics_if_mod_style = | parser : & Parser < '_ > , path : Path | { if style == PathStyle :: Mod && path . segments . iter () . any (| segment | segment . args . is_some ()) { let span = path . segments . iter () . filter_map (| segment | segment . args . as_ref ()) . map (| arg | arg . span ()) . collect :: < Vec < _ > > () ; parser . dcx () . emit_err (errors :: GenericsInPath { span }) ; let segments = path . segments . iter () . map (| segment | PathSegment { ident : segment . ident , id : segment . id , args : None }) . collect () ; Path { segments , .. path } } else { path } } ; if let Some (path) = self . eat_metavar_seq (MetaVarKind :: Path , | this | this . parse_path (PathStyle :: Type)) { return Ok (reject_generics_if_mod_style (self , path)) ; } if let Some (path) = self . eat_metavar_seq (MetaVarKind :: Ty { is_path : true } , | this | { this . parse_path (PathStyle :: Type) }) { return Ok (reject_generics_if_mod_style (self , path)) ; } let lo = self . token . span ; let mut segments = ThinVec :: new () ; let mod_sep_ctxt = self . token . span . ctxt () ; if self . eat_path_sep () { segments . push (PathSegment :: path_root (lo . shrink_to_lo () . with_ctxt (mod_sep_ctxt))) ; } self . parse_path_segments (& mut segments , style , ty_generics) ? ; Ok (Path { segments , span : lo . to (self . prev_token . span) , tokens : None }) } pub (super) fn parse_path_segments (& mut self , segments : & mut ThinVec < PathSegment > , style : PathStyle , ty_generics : Option < & Generics > ,) -> PResult < 'a , () > { loop { let segment = self . parse_path_segment (style , ty_generics) ? ; if style . has_generic_ambiguity () { self . check_trailing_angle_brackets (& segment , & [exp ! (PathSep)]) ; } segments . push (segment) ; if self . is_import_coupler () || ! self . eat_path_sep () { if self . may_recover () && style == PathStyle :: Expr && self . token == token :: Colon && self . look_ahead (1 , | token | token . is_non_reserved_ident ()) { if self . token . span . lo () == self . prev_token . span . hi () && self . look_ahead (1 , | token | self . token . span . hi () == token . span . lo ()) { self . bump () ; self . dcx () . emit_err (PathSingleColon { span : self . prev_token . span , suggestion : self . prev_token . span . shrink_to_hi () , }) ; } continue ; } return Ok (()) ; } } } #[doc = " Eat `::` or, potentially, `:::`."] #[must_use] pub (super) fn eat_path_sep (& mut self) -> bool { let result = self . eat (exp ! (PathSep)) ; if result && self . may_recover () { if self . eat_noexpect (& token :: Colon) { self . dcx () . emit_err (PathTripleColon { span : self . prev_token . span }) ; } } result } pub (super) fn parse_path_segment (& mut self , style : PathStyle , ty_generics : Option < & Generics > ,) -> PResult < 'a , PathSegment > { let ident = self . parse_path_segment_ident () ? ; let is_args_start = | token : & Token | { matches ! (token . kind , token :: Lt | token :: Shl | token :: OpenParen | token :: LArrow) } ; let check_args_start = | this : & mut Self | { this . expected_token_types . insert (TokenType :: Lt) ; this . expected_token_types . insert (TokenType :: OpenParen) ; is_args_start (& this . token) } ; Ok (if style == PathStyle :: Type && check_args_start (self) || style != PathStyle :: Mod && self . check_path_sep_and_look_ahead (is_args_start) { if style == PathStyle :: Expr { self . unmatched_angle_bracket_count = 0 ; } let _ = self . eat_path_sep () ; let lo = self . token . span ; let args = if self . eat_lt () { let args = self . parse_angle_args_with_leading_angle_bracket_recovery (style , lo , ty_generics ,) ? ; self . expect_gt () . map_err (| mut err | { if self . token == token :: Colon && self . look_ahead (1 , | token | token . is_non_reserved_ident ()) { err . cancel () ; err = self . dcx () . create_err (PathSingleColon { span : self . token . span , suggestion : self . prev_token . span . shrink_to_hi () , }) ; } else if let Some (arg) = args . iter () . rev () . find (| arg | ! matches ! (arg , AngleBracketedArg :: Constraint (_))) { err . span_suggestion_verbose (arg . span () . shrink_to_hi () , "you might have meant to end the type parameters here" , ">" , Applicability :: MaybeIncorrect ,) ; } err }) ? ; let span = lo . to (self . prev_token . span) ; AngleBracketedArgs { args , span } . into () } else if self . token == token :: OpenParen && self . look_ahead (1 , | t | * t == token :: DotDot) { self . bump () ; self . bump () ; self . expect (exp ! (CloseParen)) ? ; let span = lo . to (self . prev_token . span) ; self . psess . gated_spans . gate (sym :: return_type_notation , span) ; let prev_lo = self . prev_token . span . shrink_to_hi () ; if self . eat_noexpect (& token :: RArrow) { let lo = self . prev_token . span ; let ty = self . parse_ty () ? ; let span = lo . to (ty . span) ; let suggestion = prev_lo . to (ty . span) ; self . dcx () . emit_err (errors :: BadReturnTypeNotationOutput { span , suggestion }) ; } Box :: new (ast :: GenericArgs :: ParenthesizedElided (span)) } else { let prev_token_before_parsing = self . prev_token ; let token_before_parsing = self . token ; let mut snapshot = None ; if self . may_recover () && prev_token_before_parsing == token :: PathSep && (style == PathStyle :: Expr && self . token . can_begin_expr () || style == PathStyle :: Pat && self . token . can_begin_pattern (token :: NtPatKind :: PatParam { inferred : false , })) { snapshot = Some (self . create_snapshot_for_diagnostic ()) ; } let dcx = self . dcx () ; let parse_params_result = self . parse_paren_comma_seq (| p | { let mode = FnParseMode { context : FnContext :: Free , req_name : | _ | false , req_body : false , } ; let param = p . parse_param_general (& mode , false , false) ; param . map (move | param | { if ! matches ! (param . pat . kind , PatKind :: Missing) { dcx . emit_err (FnPathFoundNamedParams { named_param_span : param . pat . span , }) ; } if matches ! (param . ty . kind , TyKind :: CVarArgs) { dcx . emit_err (PathFoundCVariadicParams { span : param . pat . span }) ; } if ! param . attrs . is_empty () { dcx . emit_err (PathFoundAttributeInParams { span : param . attrs [0] . span , }) ; } param . ty }) }) ; let (inputs , _) = match parse_params_result { Ok (output) => output , Err (mut error) if prev_token_before_parsing == token :: PathSep => { error . span_label (prev_token_before_parsing . span . to (token_before_parsing . span) , "while parsing this parenthesized list of type arguments starting here" ,) ; if let Some (mut snapshot) = snapshot { snapshot . recover_fn_call_leading_path_sep (style , prev_token_before_parsing , & mut error ,) } return Err (error) ; } Err (error) => return Err (error) , } ; let inputs_span = lo . to (self . prev_token . span) ; let output = self . parse_ret_ty (AllowPlus :: No , RecoverQPath :: No , RecoverReturnSign :: No) ? ; let span = ident . span . to (self . prev_token . span) ; ParenthesizedArgs { span , inputs , inputs_span , output } . into () } ; PathSegment { ident , args : Some (args) , id : ast :: DUMMY_NODE_ID } } else { PathSegment :: from_ident (ident) } ,) } pub (super) fn parse_path_segment_ident (& mut self) -> PResult < 'a , Ident > { match self . token . ident () { Some ((ident , IdentIsRaw :: No)) if ident . is_path_segment_keyword () => { self . bump () ; Ok (ident) } _ => self . parse_ident () , } } #[doc = " Recover `$path::(...)` as `$path(...)`."] #[doc = ""] #[doc = " ```ignore (diagnostics)"] #[doc = " foo::(420, \"bar\")"] #[doc = "    ^^ remove extra separator to make the function call"] #[doc = " // or"] #[doc = " match x {"] #[doc = "    Foo::(420, \"bar\") => { ... },"] #[doc = "       ^^ remove extra separator to turn this into tuple struct pattern"] #[doc = "    _ => { ... },"] #[doc = " }"] #[doc = " ```"] fn recover_fn_call_leading_path_sep (& mut self , style : PathStyle , prev_token_before_parsing : Token , error : & mut Diag < '_ > ,) { match style { PathStyle :: Expr if let Ok (_) = self . parse_paren_comma_seq (| p | p . parse_expr ()) . map_err (| error | error . cancel ()) => { } PathStyle :: Pat if let Ok (_) = self . parse_paren_comma_seq (| p | { p . parse_pat_allow_top_guard (None , RecoverComma :: No , RecoverColon :: No , CommaRecoveryMode :: LikelyTuple ,) }) . map_err (| error | error . cancel ()) => { } _ => { return ; } } if let token :: PathSep | token :: RArrow = self . token . kind { return ; } error . span_suggestion_verbose (prev_token_before_parsing . span , format ! ("consider removing the `::` here to {}" , match style { PathStyle :: Expr => "call the expression" , PathStyle :: Pat => "turn this into a tuple struct pattern" , _ => { return ; } }) , "" , Applicability :: MaybeIncorrect ,) ; } #[doc = " Parses generic args (within a path segment) with recovery for extra leading angle brackets."] #[doc = " For the purposes of understanding the parsing logic of generic arguments, this function"] #[doc = " can be thought of being the same as just calling `self.parse_angle_args()` if the source"] #[doc = " had the correct amount of leading angle brackets."] #[doc = ""] #[doc = " ```ignore (diagnostics)"] #[doc = " bar::<<<<T as Foo>::Output>();"] #[doc = "      ^^ help: remove extra angle brackets"] #[doc = " ```"] fn parse_angle_args_with_leading_angle_bracket_recovery (& mut self , style : PathStyle , lo : Span , ty_generics : Option < & Generics > ,) -> PResult < 'a , ThinVec < AngleBracketedArg > > { let is_first_invocation = style == PathStyle :: Expr ; let snapshot = is_first_invocation . then (| | self . clone ()) ; self . angle_bracket_nesting += 1 ; debug ! ("parse_generic_args_with_leading_angle_bracket_recovery: (snapshotting)") ; match self . parse_angle_args (ty_generics) { Ok (args) => { self . angle_bracket_nesting -= 1 ; Ok (args) } Err (e) if self . angle_bracket_nesting > 10 => { self . angle_bracket_nesting -= 1 ; e . emit () . raise_fatal () ; } Err (e) if is_first_invocation && self . unmatched_angle_bracket_count > 0 => { self . angle_bracket_nesting -= 1 ; let snapshot = mem :: replace (self , snapshot . unwrap ()) ; let all_angle_brackets = (0 .. snapshot . unmatched_angle_bracket_count) . fold (true , | a , _ | a && self . eat_lt ()) ; if ! all_angle_brackets { let _ = mem :: replace (self , snapshot) ; Err (e) } else { e . cancel () ; debug ! ("parse_generic_args_with_leading_angle_bracket_recovery: (snapshot failure) \
                         snapshot.count={:?}" , snapshot . unmatched_angle_bracket_count ,) ; let span = lo . with_hi (lo . lo () + BytePos (snapshot . unmatched_angle_bracket_count . into ())) ; self . dcx () . emit_err (errors :: UnmatchedAngle { span , plural : snapshot . unmatched_angle_bracket_count > 1 , }) ; self . parse_angle_args (ty_generics) } } Err (e) => { self . angle_bracket_nesting -= 1 ; Err (e) } } } #[doc = " Parses (possibly empty) list of generic arguments / associated item constraints,"] #[doc = " possibly including trailing comma."] pub (super) fn parse_angle_args (& mut self , ty_generics : Option < & Generics > ,) -> PResult < 'a , ThinVec < AngleBracketedArg > > { let mut args = ThinVec :: new () ; while let Some (arg) = self . parse_angle_arg (ty_generics) ? { args . push (arg) ; if ! self . eat (exp ! (Comma)) { if self . check_noexpect (& TokenKind :: Semi) && self . look_ahead (1 , | t | t . is_ident () || t . is_lifetime ()) { self . check (exp ! (Gt)) ; let mut err = self . unexpected () . unwrap_err () ; self . bump () ; err . span_suggestion_verbose (self . prev_token . span . until (self . token . span) , "use a comma to separate type parameters" , ", " , Applicability :: MachineApplicable ,) ; err . emit () ; continue ; } if ! self . token . kind . should_end_const_arg () && self . handle_ambiguous_unbraced_const_arg (& mut args) ? { continue ; } break ; } } Ok (args) } #[doc = " Parses a single argument in the angle arguments `<...>` of a path segment."] fn parse_angle_arg (& mut self , ty_generics : Option < & Generics > ,) -> PResult < 'a , Option < AngleBracketedArg > > { let lo = self . token . span ; let arg = self . parse_generic_arg (ty_generics) ? ; match arg { Some (arg) => { let separated = self . check_noexpect (& token :: Colon) || self . check_noexpect (& token :: Eq) ; if separated && (self . check (exp ! (Colon)) | self . check (exp ! (Eq))) { let arg_span = arg . span () ; let (binder , ident , gen_args) = match self . get_ident_from_generic_arg (& arg) { Ok (ident_gen_args) => ident_gen_args , Err (()) => return Ok (Some (AngleBracketedArg :: Arg (arg))) , } ; if binder { return Err (self . dcx () . struct_span_err (arg_span , "`for<...>` is not allowed on associated type bounds" ,)) ; } let kind = if self . eat (exp ! (Colon)) { AssocItemConstraintKind :: Bound { bounds : self . parse_generic_bounds () ? } } else if self . eat (exp ! (Eq)) { self . parse_assoc_equality_term (ident , gen_args . as_ref () , self . prev_token . span ,) ? } else { unreachable ! () ; } ; let span = lo . to (self . prev_token . span) ; let constraint = AssocItemConstraint { id : ast :: DUMMY_NODE_ID , ident , gen_args , kind , span } ; Ok (Some (AngleBracketedArg :: Constraint (constraint))) } else { if self . prev_token . is_ident () && (self . token . is_ident () || self . look_ahead (1 , | token | token . is_ident ())) { self . check (exp ! (Colon)) ; self . check (exp ! (Eq)) ; } Ok (Some (AngleBracketedArg :: Arg (arg))) } } _ => Ok (None) , } } #[doc = " Parse the term to the right of an associated item equality constraint."] #[doc = ""] #[doc = " That is, parse `$term` in `Item = $term` where `$term` is a type or"] #[doc = " a const expression (wrapped in curly braces if complex)."] fn parse_assoc_equality_term (& mut self , ident : Ident , gen_args : Option < & GenericArgs > , eq : Span ,) -> PResult < 'a , AssocItemConstraintKind > { let arg = self . parse_generic_arg (None) ? ; let span = ident . span . to (self . prev_token . span) ; let term = match arg { Some (GenericArg :: Type (ty)) => ty . into () , Some (GenericArg :: Const (c)) => { self . psess . gated_spans . gate (sym :: associated_const_equality , span) ; c . into () } Some (GenericArg :: Lifetime (lt)) => { let guar = self . dcx () . emit_err (errors :: LifetimeInEqConstraint { span : lt . ident . span , lifetime : lt . ident , binding_label : span , colon_sugg : gen_args . map_or (ident . span , | args | args . span ()) . between (lt . ident . span) , }) ; self . mk_ty (lt . ident . span , ast :: TyKind :: Err (guar)) . into () } None => { let after_eq = eq . shrink_to_hi () ; let before_next = self . token . span . shrink_to_lo () ; let mut err = self . dcx () . struct_span_err (after_eq . to (before_next) , "missing type to the right of `=`") ; if matches ! (self . token . kind , token :: Comma | token :: Gt) { err . span_suggestion (self . psess . source_map () . next_point (eq) . to (before_next) , "to constrain the associated type, add a type after `=`" , " TheType" , Applicability :: HasPlaceholders ,) ; err . span_suggestion (eq . to (before_next) , format ! ("remove the `=` if `{ident}` is a type") , "" , Applicability :: MaybeIncorrect ,) } else { err . span_label (self . token . span , format ! ("expected type, found {}" , super :: token_descr (& self . token)) ,) } ; return Err (err) ; } } ; Ok (AssocItemConstraintKind :: Equality { term }) } #[doc = " We do not permit arbitrary expressions as const arguments. They must be one of:"] #[doc = " - An expression surrounded in `{}`."] #[doc = " - A literal."] #[doc = " - A numeric literal prefixed by `-`."] #[doc = " - A single-segment path."] pub (super) fn expr_is_valid_const_arg (& self , expr : & Box < rustc_ast :: Expr >) -> bool { match & expr . kind { ast :: ExprKind :: Block (_ , _) | ast :: ExprKind :: Lit (_) | ast :: ExprKind :: IncludedBytes (..) => true , ast :: ExprKind :: Unary (ast :: UnOp :: Neg , expr) => { matches ! (expr . kind , ast :: ExprKind :: Lit (_)) } ast :: ExprKind :: Path (None , path) if let [segment] = path . segments . as_slice () && segment . args . is_none () => { true } _ => false , } } #[doc = " Parse a const argument, e.g. `<3>`. It is assumed the angle brackets will be parsed by"] #[doc = " the caller."] pub (super) fn parse_const_arg (& mut self) -> PResult < 'a , AnonConst > { let value = if self . token . kind == token :: OpenBrace { self . parse_expr_block (None , self . token . span , BlockCheckMode :: Default) ? } else { self . handle_unambiguous_unbraced_const_arg () ? } ; Ok (AnonConst { id : ast :: DUMMY_NODE_ID , value }) } #[doc = " Parse a generic argument in a path segment."] #[doc = " This does not include constraints, e.g., `Item = u8`, which is handled in `parse_angle_arg`."] pub (super) fn parse_generic_arg (& mut self , ty_generics : Option < & Generics > ,) -> PResult < 'a , Option < GenericArg > > { let mut attr_span : Option < Span > = None ; if self . token == token :: Pound && self . look_ahead (1 , | t | * t == token :: OpenBracket) { let attrs_wrapper = self . parse_outer_attributes () ? ; let raw_attrs = attrs_wrapper . take_for_recovery (self . psess) ; attr_span = Some (raw_attrs [0] . span . to (raw_attrs . last () . unwrap () . span)) ; } let start = self . token . span ; let arg = if self . check_lifetime () && self . look_ahead (1 , | t | ! t . is_like_plus ()) { GenericArg :: Lifetime (self . expect_lifetime ()) } else if self . check_const_arg () { GenericArg :: Const (self . parse_const_arg () ?) } else if self . check_type () { let mut snapshot = None ; if self . may_recover () && self . token . can_begin_expr () { snapshot = Some (self . create_snapshot_for_diagnostic ()) ; } match self . parse_ty () { Ok (ty) => { if let ast :: TyKind :: Slice (inner_ty) | ast :: TyKind :: Array (inner_ty , _) = & ty . kind && let ast :: TyKind :: Err (_) = inner_ty . kind && let Some (snapshot) = snapshot && let Some (expr) = self . recover_unbraced_const_arg_that_can_begin_ty (snapshot) { return Ok (Some (self . dummy_const_arg_needs_braces (self . dcx () . struct_span_err (expr . span , "invalid const generic expression") , expr . span ,) ,)) ; } GenericArg :: Type (ty) } Err (err) => { if let Some (snapshot) = snapshot && let Some (expr) = self . recover_unbraced_const_arg_that_can_begin_ty (snapshot) { return Ok (Some (self . dummy_const_arg_needs_braces (err , expr . span))) ; } return self . recover_const_arg (start , err) . map (Some) ; } } } else if self . token . is_keyword (kw :: Const) { return self . recover_const_param_declaration (ty_generics) ; } else if let Some (attr_span) = attr_span { let diag = self . dcx () . create_err (AttributeOnEmptyType { span : attr_span }) ; return Err (diag) ; } else { let snapshot = self . create_snapshot_for_diagnostic () ; let attrs = self . parse_outer_attributes () ? ; match self . parse_expr_res (Restrictions :: CONST_EXPR , attrs) { Ok ((expr , _)) => { return Ok (Some (self . dummy_const_arg_needs_braces (self . dcx () . struct_span_err (expr . span , "invalid const generic expression") , expr . span ,))) ; } Err (err) => { self . restore_snapshot (snapshot) ; err . cancel () ; return Ok (None) ; } } } ; if let Some (attr_span) = attr_span { let guar = self . dcx () . emit_err (AttributeOnGenericArg { span : attr_span , fix_span : attr_span . until (arg . span ()) , }) ; return Ok (Some (match arg { GenericArg :: Type (_) => GenericArg :: Type (self . mk_ty (attr_span , TyKind :: Err (guar))) , GenericArg :: Const (_) => { let error_expr = self . mk_expr (attr_span , ExprKind :: Err (guar)) ; GenericArg :: Const (AnonConst { id : ast :: DUMMY_NODE_ID , value : error_expr }) } GenericArg :: Lifetime (lt) => GenericArg :: Lifetime (lt) , })) ; } Ok (Some (arg)) } #[doc = " Given a arg inside of generics, we try to destructure it as if it were the LHS in"] #[doc = " `LHS = ...`, i.e. an associated item binding."] #[doc = " This returns a bool indicating if there are any `for<'a, 'b>` binder args, the"] #[doc = " identifier, and any GAT arguments."] fn get_ident_from_generic_arg (& self , gen_arg : & GenericArg ,) -> Result < (bool , Ident , Option < GenericArgs >) , () > { if let GenericArg :: Type (ty) = gen_arg { if let ast :: TyKind :: Path (qself , path) = & ty . kind && qself . is_none () && let [seg] = path . segments . as_slice () { return Ok ((false , seg . ident , seg . args . as_deref () . cloned ())) ; } else if let ast :: TyKind :: TraitObject (bounds , ast :: TraitObjectSyntax :: None) = & ty . kind && let [ast :: GenericBound :: Trait (trait_ref)] = bounds . as_slice () && trait_ref . modifiers == ast :: TraitBoundModifiers :: NONE && let [seg] = trait_ref . trait_ref . path . segments . as_slice () { return Ok ((true , seg . ident , seg . args . as_deref () . cloned ())) ; } } Err (()) } }}}