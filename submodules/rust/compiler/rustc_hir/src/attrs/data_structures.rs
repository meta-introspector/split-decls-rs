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
mkuse!{use std :: path :: PathBuf ;}
mkuse!{pub use ReprAttr :: * ;}
mkuse!{use rustc_abi :: Align ;}
mkuse!{use rustc_ast :: token :: CommentKind ;}
mkuse!{use rustc_ast :: { AttrStyle , ast } ;}
mkuse!{use rustc_error_messages :: { DiagArgValue , IntoDiagArg } ;}
mkuse!{use rustc_macros :: { Decodable , Encodable , HashStable_Generic , PrintAttribute } ;}
mkuse!{use rustc_span :: def_id :: DefId ;}
mkuse!{use rustc_span :: hygiene :: Transparency ;}
mkuse!{use rustc_span :: { Ident , Span , Symbol } ;}
mkuse!{pub use rustc_target :: spec :: SanitizerSet ;}
mkuse!{use thin_vec :: ThinVec ;}
mkuse!{use crate :: attrs :: pretty_printing :: PrintAttribute ;}
mkuse!{use crate :: limit :: Limit ;}
mkuse!{use crate :: { DefaultBodyStability , PartialConstStability , RustcVersion , Stability } ;}
mkitem!{mkenum!{# [derive (Copy , Clone , PartialEq , Encodable , Decodable , Debug , HashStable_Generic , PrintAttribute)] pub enum InlineAttr { None , Hint , Always , Never , # [doc = " `#[rustc_force_inline]` forces inlining to happen in the MIR inliner - it reports an error"] # [doc = " if the inlining cannot happen. It is limited to only free functions so that the calls"] # [doc = " can always be resolved."] Force { attr_span : Span , reason : Option < Symbol > , } , }}}
mkitem!{mkimpl!{impl InlineAttr { pub fn always (& self) -> bool { match self { InlineAttr :: Always | InlineAttr :: Force { .. } => true , InlineAttr :: None | InlineAttr :: Hint | InlineAttr :: Never => false , } } }}}
mkitem!{mkenum!{# [derive (Clone , Encodable , Decodable , Debug , PartialEq , Eq , HashStable_Generic)] pub enum InstructionSetAttr { ArmA32 , ArmT32 , }}}
mkitem!{mkenum!{# [derive (Copy , Clone , Debug , PartialEq , Eq , Default , PrintAttribute)] # [derive (Encodable , Decodable , HashStable_Generic)] pub enum OptimizeAttr { # [doc = " No `#[optimize(..)]` attribute"] # [default] Default , # [doc = " `#[optimize(none)]`"] DoNotOptimize , # [doc = " `#[optimize(speed)]`"] Speed , # [doc = " `#[optimize(size)]`"] Size , }}}
mkitem!{mkimpl!{impl OptimizeAttr { pub fn do_not_optimize (& self) -> bool { matches ! (self , Self :: DoNotOptimize) } }}}
mkitem!{mkenum!{# [derive (PartialEq , Debug , Encodable , Decodable , Copy , Clone , HashStable_Generic , PrintAttribute)] pub enum ReprAttr { ReprInt (IntType) , ReprRust , ReprC , ReprPacked (Align) , ReprSimd , ReprTransparent , ReprAlign (Align) , }}}
mkitem!{mkenum!{pub enum TransparencyError { UnknownTransparency (Symbol , Span) , MultipleTransparencyAttrs (Span , Span) , }}}
mkitem!{mkenum!{# [derive (Eq , PartialEq , Debug , Copy , Clone)] # [derive (Encodable , Decodable , HashStable_Generic , PrintAttribute)] pub enum IntType { SignedInt (ast :: IntTy) , UnsignedInt (ast :: UintTy) , }}}
mkitem!{mkstruct!{# [derive (Copy , Debug , Encodable , Decodable , Clone , HashStable_Generic , PrintAttribute)] pub struct Deprecation { pub since : DeprecatedSince , # [doc = " The note to issue a reason."] pub note : Option < Symbol > , # [doc = " A text snippet used to completely replace any use of the deprecated item in an expression."] # [doc = ""] # [doc = " This is currently unstable."] pub suggestion : Option < Symbol > , }}}
mkitem!{mkenum!{# [doc = " Release in which an API is deprecated."] # [derive (Copy , Debug , Encodable , Decodable , Clone , HashStable_Generic , PrintAttribute)] pub enum DeprecatedSince { RustcVersion (RustcVersion) , # [doc = " Deprecated in the future (\"to be determined\")."] Future , # [doc = " `feature(staged_api)` is off. Deprecation versions outside the standard"] # [doc = " library are allowed to be arbitrary strings, for better or worse."] NonStandard (Symbol) , # [doc = " Deprecation version is unspecified but optional."] Unspecified , # [doc = " Failed to parse a deprecation version, or the deprecation version is"] # [doc = " unspecified and required. An error has already been emitted."] Err , }}}
mkitem!{mkenum!{# [doc = " Successfully-parsed value of a `#[coverage(..)]` attribute."] # [derive (Copy , Debug , Eq , PartialEq , Encodable , Decodable , Clone)] # [derive (HashStable_Generic , PrintAttribute)] pub enum CoverageAttrKind { On , Off , }}}
mkitem!{mkimpl!{impl Deprecation { # [doc = " Whether an item marked with #[deprecated(since = \"X\")] is currently"] # [doc = " deprecated (i.e., whether X is not greater than the current rustc"] # [doc = " version)."] pub fn is_in_effect (& self) -> bool { match self . since { DeprecatedSince :: RustcVersion (since) => since <= RustcVersion :: CURRENT , DeprecatedSince :: Future => false , DeprecatedSince :: NonStandard (_) => true , DeprecatedSince :: Unspecified | DeprecatedSince :: Err => true , } } pub fn is_since_rustc_version (& self) -> bool { matches ! (self . since , DeprecatedSince :: RustcVersion (_)) } }}}
mkitem!{mkenum!{# [doc = " There are three valid forms of the attribute:"] # [doc = " `#[used]`, which is semantically equivalent to `#[used(linker)]` except that the latter is currently unstable."] # [doc = " `#[used(compiler)]`"] # [doc = " `#[used(linker)]`"] # [derive (Encodable , Decodable , Copy , Clone , Debug , PartialEq , Eq , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub enum UsedBy { Compiler , Linker , }}}
mkitem!{mkenum!{# [derive (Encodable , Decodable , Clone , Debug , PartialEq , Eq , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub enum MacroUseArgs { UseAll , UseSpecific (ThinVec < Ident >) , }}}
mkitem!{mkimpl!{impl Default for MacroUseArgs { fn default () -> Self { Self :: UseSpecific (ThinVec :: new ()) } }}}
mkitem!{mkstruct!{# [derive (Debug , Clone , Encodable , Decodable , HashStable_Generic)] pub struct StrippedCfgItem < ModId = DefId > { pub parent_module : ModId , pub ident : Ident , pub cfg : (CfgEntry , Span) , }}}
mkitem!{mkimpl!{impl < ModId > StrippedCfgItem < ModId > { pub fn map_mod_id < New > (self , f : impl FnOnce (ModId) -> New) -> StrippedCfgItem < New > { StrippedCfgItem { parent_module : f (self . parent_module) , ident : self . ident , cfg : self . cfg } } }}}
mkitem!{mkenum!{# [derive (Encodable , Decodable , Clone , Debug , PartialEq , Eq , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub enum CfgEntry { All (ThinVec < CfgEntry > , Span) , Any (ThinVec < CfgEntry > , Span) , Not (Box < CfgEntry > , Span) , Bool (bool , Span) , NameValue { name : Symbol , name_span : Span , value : Option < (Symbol , Span) > , span : Span } , Version (Option < RustcVersion > , Span) , }}}
mkitem!{mkenum!{# [doc = " Possible values for the `#[linkage]` attribute, allowing to specify the"] # [doc = " linkage type for a `MonoItem`."] # [doc = ""] # [doc = " See <https://llvm.org/docs/LangRef.html#linkage-types> for more details about these variants."] # [derive (Encodable , Decodable , Clone , Copy , Debug , PartialEq , Eq , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub enum Linkage { AvailableExternally , Common , ExternalWeak , External , Internal , LinkOnceAny , LinkOnceODR , WeakAny , WeakODR , }}}
mkitem!{mkenum!{# [derive (Clone , Copy , Decodable , Debug , Encodable , PartialEq)] # [derive (HashStable_Generic , PrintAttribute)] pub enum MirDialect { Analysis , Built , Runtime , }}}
mkitem!{mkimpl!{impl IntoDiagArg for MirDialect { fn into_diag_arg (self , _path : & mut Option < PathBuf >) -> DiagArgValue { let arg = match self { MirDialect :: Analysis => "analysis" , MirDialect :: Built => "built" , MirDialect :: Runtime => "runtime" , } ; DiagArgValue :: Str (Cow :: Borrowed (arg)) } }}}
mkitem!{mkenum!{# [derive (Clone , Copy , Decodable , Debug , Encodable , PartialEq)] # [derive (HashStable_Generic , PrintAttribute)] pub enum MirPhase { Initial , PostCleanup , Optimized , }}}
mkitem!{mkimpl!{impl IntoDiagArg for MirPhase { fn into_diag_arg (self , _path : & mut Option < PathBuf >) -> DiagArgValue { let arg = match self { MirPhase :: Initial => "initial" , MirPhase :: PostCleanup => "post-cleanup" , MirPhase :: Optimized => "optimized" , } ; DiagArgValue :: Str (Cow :: Borrowed (arg)) } }}}
mkitem!{mkenum!{# [doc = " Different ways that the PE Format can decorate a symbol name."] # [doc = " From <https://docs.microsoft.com/en-us/windows/win32/debug/pe-format#import-name-type>"] # [derive (Copy , Clone , Debug , Encodable , Decodable , HashStable_Generic , PartialEq , Eq , PrintAttribute)] pub enum PeImportNameType { # [doc = " IMPORT_ORDINAL"] # [doc = " Uses the ordinal (i.e., a number) rather than the name."] Ordinal (u16) , # [doc = " Same as IMPORT_NAME"] # [doc = " Name is decorated with all prefixes and suffixes."] Decorated , # [doc = " Same as IMPORT_NAME_NOPREFIX"] # [doc = " Prefix (e.g., the leading `_` or `@`) is skipped, but suffix is kept."] NoPrefix , # [doc = " Same as IMPORT_NAME_UNDECORATE"] # [doc = " Prefix (e.g., the leading `_` or `@`) and suffix (the first `@` and all"] # [doc = " trailing characters) are skipped."] Undecorated , }}}
mkitem!{mkenum!{# [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Encodable , Decodable , PrintAttribute)] # [derive (HashStable_Generic)] pub enum NativeLibKind { # [doc = " Static library (e.g. `libfoo.a` on Linux or `foo.lib` on Windows/MSVC)"] Static { # [doc = " Whether to bundle objects from static library into produced rlib"] bundle : Option < bool > , # [doc = " Whether to link static library without throwing any object files away"] whole_archive : Option < bool > , } , # [doc = " Dynamic library (e.g. `libfoo.so` on Linux)"] # [doc = " or an import library corresponding to a dynamic library (e.g. `foo.lib` on Windows/MSVC)."] Dylib { # [doc = " Whether the dynamic library will be linked only if it satisfies some undefined symbols"] as_needed : Option < bool > , } , # [doc = " Dynamic library (e.g. `foo.dll` on Windows) without a corresponding import library."] # [doc = " On Linux, it refers to a generated shared library stub."] RawDylib , # [doc = " A macOS-specific kind of dynamic libraries."] Framework { # [doc = " Whether the framework will be linked only if it satisfies some undefined symbols"] as_needed : Option < bool > , } , # [doc = " Argument which is passed to linker, relative order with libraries and other arguments"] # [doc = " is preserved"] LinkArg , # [doc = " Module imported from WebAssembly"] WasmImportModule , # [doc = " The library kind wasn't specified, `Dylib` is currently used as a default."] Unspecified , }}}
mkitem!{mkimpl!{impl NativeLibKind { pub fn has_modifiers (& self) -> bool { match self { NativeLibKind :: Static { bundle , whole_archive } => { bundle . is_some () || whole_archive . is_some () } NativeLibKind :: Dylib { as_needed } | NativeLibKind :: Framework { as_needed } => { as_needed . is_some () } NativeLibKind :: RawDylib | NativeLibKind :: Unspecified | NativeLibKind :: LinkArg | NativeLibKind :: WasmImportModule => false , } } pub fn is_statically_included (& self) -> bool { matches ! (self , NativeLibKind :: Static { .. }) } pub fn is_dllimport (& self) -> bool { matches ! (self , NativeLibKind :: Dylib { .. } | NativeLibKind :: RawDylib | NativeLibKind :: Unspecified) } }}}
mkitem!{mkstruct!{# [derive (Debug , Encodable , Decodable , Clone , HashStable_Generic , PrintAttribute)] pub struct LinkEntry { pub span : Span , pub kind : NativeLibKind , pub name : Symbol , pub cfg : Option < CfgEntry > , pub verbatim : Option < bool > , pub import_name_type : Option < (PeImportNameType , Span) > , }}}
mkitem!{mkenum!{# [doc = " Represents parsed *built-in* inert attributes."] # [doc = ""] # [doc = " ## Overview"] # [doc = " These attributes are markers that guide the compilation process and are never expanded into other code."] # [doc = " They persist throughout the compilation phases, from AST to HIR and beyond."] # [doc = ""] # [doc = " ## Attribute Processing"] # [doc = " While attributes are initially parsed by [`rustc_parse`] into [`ast::Attribute`], they still contain raw token streams"] # [doc = " because different attributes have different internal structures. This enum represents the final,"] # [doc = " fully parsed form of these attributes, where each variant contains all the information and"] # [doc = " structure relevant for the specific attribute."] # [doc = ""] # [doc = " Some attributes can be applied multiple times to the same item, and they are \"collapsed\" into a single"] # [doc = " semantic attribute. For example:"] # [doc = " ```rust"] # [doc = " #[repr(C)]"] # [doc = " #[repr(packed)]"] # [doc = " struct S { }"] # [doc = " ```"] # [doc = " This is equivalent to `#[repr(C, packed)]` and results in a single [`AttributeKind::Repr`] containing"] # [doc = " both `C` and `packed` annotations. This collapsing happens during parsing and is reflected in the"] # [doc = " data structures defined in this enum."] # [doc = ""] # [doc = " ## Usage"] # [doc = " These parsed attributes are used throughout the compiler to:"] # [doc = " - Control code generation (e.g., `#[repr]`)"] # [doc = " - Mark API stability (`#[stable]`, `#[unstable]`)"] # [doc = " - Provide documentation (`#[doc]`)"] # [doc = " - Guide compiler behavior (e.g., `#[allow_internal_unstable]`)"] # [doc = ""] # [doc = " ## Note on Attribute Organization"] # [doc = " Some attributes like `InlineAttr`, `OptimizeAttr`, and `InstructionSetAttr` are defined separately"] # [doc = " from this enum because they are used in specific compiler phases (like code generation) and don't"] # [doc = " need to persist throughout the entire compilation process. They are typically processed and"] # [doc = " converted into their final form earlier in the compilation pipeline."] # [doc = ""] # [doc = " For example:"] # [doc = " - `InlineAttr` is used during code generation to control function inlining"] # [doc = " - `OptimizeAttr` is used to control optimization levels"] # [doc = " - `InstructionSetAttr` is used for target-specific code generation"] # [doc = ""] # [doc = " These attributes are handled by their respective compiler passes in the [`rustc_codegen_ssa`] crate"] # [doc = " and don't need to be preserved in the same way as the attributes in this enum."] # [doc = ""] # [doc = " For more details on attribute parsing, see the [`rustc_attr_parsing`] crate."] # [doc = ""] # [doc = " [`rustc_parse`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_parse/index.html"] # [doc = " [`rustc_codegen_ssa`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_codegen_ssa/index.html"] # [doc = " [`rustc_attr_parsing`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_attr_parsing/index.html"] # [derive (Clone , Debug , HashStable_Generic , Encodable , Decodable , PrintAttribute)] pub enum AttributeKind { # [doc = " Represents `#[align(N)]`."] Align { align : Align , span : Span } , # [doc = " Represents `#[rustc_allow_const_fn_unstable]`."] AllowConstFnUnstable (ThinVec < Symbol > , Span) , # [doc = " Represents `#[rustc_allow_incoherent_impl]`."] AllowIncoherentImpl (Span) , # [doc = " Represents `#[allow_internal_unsafe]`."] AllowInternalUnsafe (Span) , # [doc = " Represents `#[allow_internal_unstable]`."] AllowInternalUnstable (ThinVec < (Symbol , Span) > , Span) , # [doc = " Represents `#[rustc_as_ptr]` (used by the `dangling_pointers_from_temporaries` lint)."] AsPtr (Span) , # [doc = " Represents `#[automatically_derived]`"] AutomaticallyDerived (Span) , # [doc = " Represents `#[rustc_default_body_unstable]`."] BodyStability { stability : DefaultBodyStability , # [doc = " Span of the `#[rustc_default_body_unstable(...)]` attribute"] span : Span , } , # [doc = " Represents `#[rustc_coherence_is_core]`."] CoherenceIsCore , # [doc = " Represents `#[rustc_coinductive]`."] Coinductive (Span) , # [doc = " Represents `#[cold]`."] Cold (Span) , # [doc = " Represents `#[rustc_confusables]`."] Confusables { symbols : ThinVec < Symbol > , first_span : Span , } , # [doc = " Represents `#[const_continue]`."] ConstContinue (Span) , # [doc = " Represents `#[rustc_const_stable]` and `#[rustc_const_unstable]`."] ConstStability { stability : PartialConstStability , # [doc = " Span of the `#[rustc_const_stable(...)]` or `#[rustc_const_unstable(...)]` attribute"] span : Span , } , # [doc = " Represents `#[rustc_const_stable_indirect]`."] ConstStabilityIndirect , # [doc = " Represents `#[const_trait]`."] ConstTrait (Span) , # [doc = " Represents `#[coroutine]`."] Coroutine (Span) , # [doc = " Represents `#[coverage(..)]`."] Coverage (Span , CoverageAttrKind) , # [doc = " Represents `#[crate_name = ...]`"] CrateName { name : Symbol , name_span : Span , attr_span : Span , style : AttrStyle } , # [doc = " Represents `#[custom_mir]`."] CustomMir (Option < (MirDialect , Span) > , Option < (MirPhase , Span) > , Span) , # [doc = "Represents `#[rustc_deny_explicit_impl]`."] DenyExplicitImpl (Span) , # [doc = " Represents [`#[deprecated]`](https://doc.rust-lang.org/stable/reference/attributes/diagnostics.html#the-deprecated-attribute)."] Deprecation { deprecation : Deprecation , span : Span } , # [doc = " Represents `#[rustc_do_not_implement_via_object]`."] DoNotImplementViaObject (Span) , # [doc = " Represents [`#[doc]`](https://doc.rust-lang.org/stable/rustdoc/write-documentation/the-doc-attribute.html)."] DocComment { style : AttrStyle , kind : CommentKind , span : Span , comment : Symbol } , # [doc = " Represents `#[rustc_dummy]`."] Dummy , # [doc = " Represents [`#[export_name]`](https://doc.rust-lang.org/reference/abi.html#the-export_name-attribute)."] ExportName { # [doc = " The name to export this item with."] # [doc = " It may not contain \\0 bytes as it will be converted to a null-terminated string."] name : Symbol , span : Span , } , # [doc = " Represents `#[export_stable]`."] ExportStable , # [doc = " Represents `#[ffi_const]`."] FfiConst (Span) , # [doc = " Represents `#[ffi_pure]`."] FfiPure (Span) , # [doc = " Represents `#[fundamental]`."] Fundamental , # [doc = " Represents `#[ignore]`"] Ignore { span : Span , # [doc = " ignore can optionally have a reason: `#[ignore = \"reason this is ignored\"]`"] reason : Option < Symbol > , } , # [doc = " Represents `#[inline]` and `#[rustc_force_inline]`."] Inline (InlineAttr , Span) , # [doc = " Represents `#[link]`."] Link (ThinVec < LinkEntry > , Span) , # [doc = " Represents `#[link_name]`."] LinkName { name : Symbol , span : Span } , # [doc = " Represents `#[link_ordinal]`."] LinkOrdinal { ordinal : u16 , span : Span } , # [doc = " Represents [`#[link_section]`](https://doc.rust-lang.org/reference/abi.html#the-link_section-attribute)"] LinkSection { name : Symbol , span : Span } , # [doc = " Represents `#[linkage]`."] Linkage (Linkage , Span) , # [doc = " Represents `#[loop_match]`."] LoopMatch (Span) , # [doc = " Represents `#[macro_escape]`."] MacroEscape (Span) , # [doc = " Represents `#[rustc_macro_transparency]`."] MacroTransparency (Transparency) , # [doc = " Represents `#[macro_use]`."] MacroUse { span : Span , arguments : MacroUseArgs } , # [doc = " Represents `#[marker]`."] Marker (Span) , # [doc = " Represents [`#[may_dangle]`](https://std-dev-guide.rust-lang.org/tricky/may-dangle.html)."] MayDangle (Span) , # [doc = " Represents `#[move_size_limit]`"] MoveSizeLimit { attr_span : Span , limit_span : Span , limit : Limit } , # [doc = " Represents `#[must_use]`."] MustUse { span : Span , # [doc = " must_use can optionally have a reason: `#[must_use = \"reason this must be used\"]`"] reason : Option < Symbol > , } , # [doc = " Represents `#[naked]`"] Naked (Span) , # [doc = " Represents `#[no_core]`"] NoCore (Span) , # [doc = " Represents `#[no_implicit_prelude]`"] NoImplicitPrelude (Span) , # [doc = " Represents `#[no_mangle]`"] NoMangle (Span) , # [doc = " Represents `#[no_std]`"] NoStd (Span) , # [doc = " Represents `#[non_exhaustive]`"] NonExhaustive (Span) , # [doc = " Represents `#[optimize(size|speed)]`"] Optimize (OptimizeAttr , Span) , # [doc = " Represents `#[rustc_paren_sugar]`."] ParenSugar (Span) , # [doc = " Represents `#[rustc_pass_by_value]` (used by the `rustc_pass_by_value` lint)."] PassByValue (Span) , # [doc = " Represents `#[path]`"] Path (Symbol , Span) , # [doc = " Represents `#[pattern_complexity_limit]`"] PatternComplexityLimit { attr_span : Span , limit_span : Span , limit : Limit } , # [doc = " Represents `#[pointee]`"] Pointee (Span) , # [doc = " Represents `#[proc_macro]`"] ProcMacro (Span) , # [doc = " Represents `#[proc_macro_attribute]`"] ProcMacroAttribute (Span) , # [doc = " Represents `#[proc_macro_derive]`"] ProcMacroDerive { trait_name : Symbol , helper_attrs : ThinVec < Symbol > , span : Span } , # [doc = " Represents `#[rustc_pub_transparent]` (used by the `repr_transparent_external_private_fields` lint)."] PubTransparent (Span) , # [doc = " Represents [`#[recursion_limit]`](https://doc.rust-lang.org/reference/attributes/limits.html#the-recursion_limit-attribute)"] RecursionLimit { attr_span : Span , limit_span : Span , limit : Limit } , # [doc = " Represents [`#[repr]`](https://doc.rust-lang.org/stable/reference/type-layout.html#representations)."] Repr { reprs : ThinVec < (ReprAttr , Span) > , first_span : Span } , # [doc = " Represents `#[rustc_builtin_macro]`."] RustcBuiltinMacro { builtin_name : Option < Symbol > , helper_attrs : ThinVec < Symbol > , span : Span } , # [doc = " Represents `#[rustc_layout_scalar_valid_range_end]`."] RustcLayoutScalarValidRangeEnd (Box < u128 > , Span) , # [doc = " Represents `#[rustc_layout_scalar_valid_range_start]`."] RustcLayoutScalarValidRangeStart (Box < u128 > , Span) , # [doc = " Represents `#[rustc_object_lifetime_default]`."] RustcObjectLifetimeDefault , # [doc = " Represents `#[sanitize]`"] # [doc = ""] # [doc = " the on set and off set are distjoint since there's a third option: unset."] # [doc = " a node may not set the sanitizer setting in which case it inherits from parents."] Sanitize { on_set : SanitizerSet , off_set : SanitizerSet , span : Span } , # [doc = " Represents `#[should_panic]`"] ShouldPanic { reason : Option < Symbol > , span : Span } , # [doc = " Represents `#[rustc_skip_during_method_dispatch]`."] SkipDuringMethodDispatch { array : bool , boxed_slice : bool , span : Span } , # [doc = " Represents `#[rustc_specialization_trait]`."] SpecializationTrait (Span) , # [doc = " Represents `#[stable]`, `#[unstable]` and `#[rustc_allowed_through_unstable_modules]`."] Stability { stability : Stability , # [doc = " Span of the attribute."] span : Span , } , # [doc = " Represents `#[rustc_std_internal_symbol]`."] StdInternalSymbol (Span) , # [doc = " Represents `#[target_feature(enable = \"...\")]` and"] # [doc = " `#[unsafe(force_target_feature(enable = \"...\")]`."] TargetFeature { features : ThinVec < (Symbol , Span) > , attr_span : Span , was_forced : bool } , # [doc = " Represents `#[track_caller]`"] TrackCaller (Span) , # [doc = " Represents `#[type_const]`."] TypeConst (Span) , # [doc = " Represents `#[type_length_limit]`"] TypeLengthLimit { attr_span : Span , limit_span : Span , limit : Limit } , # [doc = " Represents `#[rustc_unsafe_specialization_marker]`."] UnsafeSpecializationMarker (Span) , # [doc = " Represents `#[unstable_feature_bound]`."] UnstableFeatureBound (ThinVec < (Symbol , Span) >) , # [doc = " Represents `#[used]`"] Used { used_by : UsedBy , span : Span } , }}}