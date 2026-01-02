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
mkuse!{use std :: fmt ;}
mkuse!{pub use LitKind :: * ;}
mkuse!{pub use NtExprKind :: * ;}
mkuse!{pub use NtPatKind :: * ;}
mkuse!{pub use TokenKind :: * ;}
mkuse!{use rustc_macros :: { Decodable , Encodable , HashStable_Generic } ;}
mkuse!{use rustc_span :: edition :: Edition ;}
mkuse!{use rustc_span :: symbol :: IdentPrintMode ;}
mkuse!{use rustc_span :: { DUMMY_SP , ErrorGuaranteed , Span , kw , sym } ;}
mkuse!{# [allow (clippy :: useless_attribute)] # [allow (hidden_glob_reexports)] use rustc_span :: { Ident , Symbol } ;}
mkuse!{use crate :: ast ;}
mkuse!{use crate :: util :: case :: Case ;}
mkitem!{mkenum!{# [derive (Clone , Copy , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub enum CommentKind { Line , Block , }}}
mkitem!{mkenum!{# [derive (Copy , Clone , PartialEq , Debug , Encodable , Decodable , HashStable_Generic)] pub enum InvisibleOrigin { MetaVar (MetaVarKind) , ProcMacro , }}}
mkitem!{mkimpl!{impl InvisibleOrigin { # [inline] pub fn skip (& self) -> bool { match self { InvisibleOrigin :: MetaVar (_) => false , InvisibleOrigin :: ProcMacro => true , } } }}}
mkitem!{mkenum!{# [doc = " Annoyingly similar to `NonterminalKind`, but the slight differences are important."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Encodable , Decodable , Hash , HashStable_Generic)] pub enum MetaVarKind { Item , Block , Stmt , Pat (NtPatKind) , Expr { kind : NtExprKind , can_begin_literal_maybe_minus : bool , can_begin_string_literal : bool , } , Ty { is_path : bool , } , Ident , Lifetime , Literal , Meta { # [doc = " Will `AttrItem::meta` succeed on this, if reparsed?"] has_meta_form : bool , } , Path , Vis , TT , }}}
mkitem!{mkimpl!{impl fmt :: Display for MetaVarKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let sym = match self { MetaVarKind :: Item => sym :: item , MetaVarKind :: Block => sym :: block , MetaVarKind :: Stmt => sym :: stmt , MetaVarKind :: Pat (PatParam { inferred : true } | PatWithOr) => sym :: pat , MetaVarKind :: Pat (PatParam { inferred : false }) => sym :: pat_param , MetaVarKind :: Expr { kind : Expr2021 { inferred : true } | Expr , .. } => sym :: expr , MetaVarKind :: Expr { kind : Expr2021 { inferred : false } , .. } => sym :: expr_2021 , MetaVarKind :: Ty { .. } => sym :: ty , MetaVarKind :: Ident => sym :: ident , MetaVarKind :: Lifetime => sym :: lifetime , MetaVarKind :: Literal => sym :: literal , MetaVarKind :: Meta { .. } => sym :: meta , MetaVarKind :: Path => sym :: path , MetaVarKind :: Vis => sym :: vis , MetaVarKind :: TT => sym :: tt , } ; write ! (f , "{sym}") } }}}
mkitem!{mkenum!{# [doc = " Describes how a sequence of token trees is delimited."] # [doc = " Cannot use `proc_macro::Delimiter` directly because this"] # [doc = " structure should implement some additional traits."] # [derive (Copy , Clone , Debug , PartialEq , Encodable , Decodable , HashStable_Generic)] pub enum Delimiter { # [doc = " `( ... )`"] Parenthesis , # [doc = " `{ ... }`"] Brace , # [doc = " `[ ... ]`"] Bracket , # [doc = " `∅ ... ∅`"] # [doc = " An invisible delimiter, that may, for example, appear around tokens coming from a"] # [doc = " \"macro variable\" `$var`. It is important to preserve operator priorities in cases like"] # [doc = " `$var * 3` where `$var` is `1 + 2`."] # [doc = " Invisible delimiters might not survive roundtrip of a token stream through a string."] Invisible (InvisibleOrigin) , }}}
mkitem!{mkimpl!{impl Delimiter { # [inline] pub fn skip (& self) -> bool { match self { Delimiter :: Parenthesis | Delimiter :: Bracket | Delimiter :: Brace => false , Delimiter :: Invisible (origin) => origin . skip () , } } pub fn eq_ignoring_invisible_origin (& self , other : & Delimiter) -> bool { match (self , other) { (Delimiter :: Parenthesis , Delimiter :: Parenthesis) => true , (Delimiter :: Brace , Delimiter :: Brace) => true , (Delimiter :: Bracket , Delimiter :: Bracket) => true , (Delimiter :: Invisible (_) , Delimiter :: Invisible (_)) => true , _ => false , } } pub fn as_open_token_kind (& self) -> TokenKind { match * self { Delimiter :: Parenthesis => OpenParen , Delimiter :: Brace => OpenBrace , Delimiter :: Bracket => OpenBracket , Delimiter :: Invisible (origin) => OpenInvisible (origin) , } } pub fn as_close_token_kind (& self) -> TokenKind { match * self { Delimiter :: Parenthesis => CloseParen , Delimiter :: Brace => CloseBrace , Delimiter :: Bracket => CloseBracket , Delimiter :: Invisible (origin) => CloseInvisible (origin) , } } }}}
mkitem!{mkenum!{# [derive (Clone , Copy , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub enum LitKind { Bool , Byte , Char , Integer , Float , Str , StrRaw (u8) , ByteStr , ByteStrRaw (u8) , CStr , CStrRaw (u8) , Err (ErrorGuaranteed) , }}}
mkitem!{mkstruct!{# [doc = " A literal token."] # [derive (Clone , Copy , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub struct Lit { pub kind : LitKind , pub symbol : Symbol , pub suffix : Option < Symbol > , }}}
mkitem!{mkimpl!{impl Lit { pub fn new (kind : LitKind , symbol : Symbol , suffix : Option < Symbol >) -> Lit { Lit { kind , symbol , suffix } } # [doc = " Returns `true` if this is semantically a float literal. This includes"] # [doc = " ones like `1f32` that have an `Integer` kind but a float suffix."] pub fn is_semantic_float (& self) -> bool { match self . kind { LitKind :: Float => true , LitKind :: Integer => match self . suffix { Some (sym) => sym == sym :: f32 || sym == sym :: f64 , None => false , } , _ => false , } } # [doc = " Keep this in sync with `Token::can_begin_literal_maybe_minus` and"] # [doc = " `Parser::eat_token_lit` (excluding unary negation)."] pub fn from_token (token : & Token) -> Option < Lit > { match token . uninterpolate () . kind { Ident (name , IdentIsRaw :: No) if name . is_bool_lit () => Some (Lit :: new (Bool , name , None)) , Literal (token_lit) => Some (token_lit) , OpenInvisible (InvisibleOrigin :: MetaVar (MetaVarKind :: Literal | MetaVarKind :: Expr { .. } ,)) => { panic ! ("from_token metavar") ; } _ => None , } } }}}
mkitem!{mkimpl!{impl fmt :: Display for Lit { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let Lit { kind , symbol , suffix } = * self ; match kind { Byte => write ! (f , "b'{symbol}'") ? , Char => write ! (f , "'{symbol}'") ? , Str => write ! (f , "\"{symbol}\"") ? , StrRaw (n) => write ! (f , "r{delim}\"{string}\"{delim}" , delim = "#" . repeat (n as usize) , string = symbol) ? , ByteStr => write ! (f , "b\"{symbol}\"") ? , ByteStrRaw (n) => write ! (f , "br{delim}\"{string}\"{delim}" , delim = "#" . repeat (n as usize) , string = symbol) ? , CStr => write ! (f , "c\"{symbol}\"") ? , CStrRaw (n) => { write ! (f , "cr{delim}\"{symbol}\"{delim}" , delim = "#" . repeat (n as usize)) ? } Integer | Float | Bool | Err (_) => write ! (f , "{symbol}") ? , } if let Some (suffix) = suffix { write ! (f , "{suffix}") ? ; } Ok (()) } }}}
mkitem!{mkimpl!{impl LitKind { # [doc = " An English article for the literal token kind."] pub fn article (self) -> & 'static str { match self { Integer | Err (_) => "an" , _ => "a" , } } pub fn descr (self) -> & 'static str { match self { Bool => "boolean" , Byte => "byte" , Char => "char" , Integer => "integer" , Float => "float" , Str | StrRaw (..) => "string" , ByteStr | ByteStrRaw (..) => "byte string" , CStr | CStrRaw (..) => "C string" , Err (_) => "error" , } } pub (crate) fn may_have_suffix (self) -> bool { matches ! (self , Integer | Float | Err (_)) } }}}

macro_rules! ident_can_begin_expr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ident_can_begin_expr in module {}", module_path!());
    };
}

mkfn!{
    ident_can_begin_expr_introspect!();
    pub fn ident_can_begin_expr (name : Symbol , span : Span , is_raw : IdentIsRaw) -> bool { let ident_token = Token :: new (Ident (name , is_raw) , span) ; ! ident_token . is_reserved_ident () || ident_token . is_path_segment_keyword () || [kw :: Async , kw :: Do , kw :: Box , kw :: Break , kw :: Const , kw :: Continue , kw :: False , kw :: For , kw :: Gen , kw :: If , kw :: Let , kw :: Loop , kw :: Match , kw :: Move , kw :: Return , kw :: True , kw :: Try , kw :: Unsafe , kw :: While , kw :: Yield , kw :: Safe , kw :: Static ,] . contains (& name) }
}

macro_rules! ident_can_begin_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ident_can_begin_type in module {}", module_path!());
    };
}

mkfn!{
    ident_can_begin_type_introspect!();
    fn ident_can_begin_type (name : Symbol , span : Span , is_raw : IdentIsRaw) -> bool { let ident_token = Token :: new (Ident (name , is_raw) , span) ; ! ident_token . is_reserved_ident () || ident_token . is_path_segment_keyword () || [kw :: Underscore , kw :: For , kw :: Impl , kw :: Fn , kw :: Unsafe , kw :: Extern , kw :: Typeof , kw :: Dyn] . contains (& name) }
}
mkitem!{mkenum!{# [derive (PartialEq , Encodable , Decodable , Debug , Copy , Clone , HashStable_Generic)] pub enum IdentIsRaw { No , Yes , }}}
mkitem!{mkimpl!{impl IdentIsRaw { pub fn to_print_mode_ident (self) -> IdentPrintMode { match self { IdentIsRaw :: No => IdentPrintMode :: Normal , IdentIsRaw :: Yes => IdentPrintMode :: RawIdent , } } pub fn to_print_mode_lifetime (self) -> IdentPrintMode { match self { IdentIsRaw :: No => IdentPrintMode :: Normal , IdentIsRaw :: Yes => IdentPrintMode :: RawLifetime , } } }}}
mkitem!{mkimpl!{impl From < bool > for IdentIsRaw { fn from (b : bool) -> Self { if b { Self :: Yes } else { Self :: No } } }}}
mkitem!{mkenum!{# [derive (Clone , Copy , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub enum TokenKind { # [doc = " `=`"] Eq , # [doc = " `<`"] Lt , # [doc = " `<=`"] Le , # [doc = " `==`"] EqEq , # [doc = " `!=`"] Ne , # [doc = " `>=`"] Ge , # [doc = " `>`"] Gt , # [doc = " `&&`"] AndAnd , # [doc = " `||`"] OrOr , # [doc = " `!`"] Bang , # [doc = " `~`"] Tilde , Plus , Minus , Star , Slash , Percent , Caret , And , Or , Shl , Shr , PlusEq , MinusEq , StarEq , SlashEq , PercentEq , CaretEq , AndEq , OrEq , ShlEq , ShrEq , # [doc = " `@`"] At , # [doc = " `.`"] Dot , # [doc = " `..`"] DotDot , # [doc = " `...`"] DotDotDot , # [doc = " `..=`"] DotDotEq , # [doc = " `,`"] Comma , # [doc = " `;`"] Semi , # [doc = " `:`"] Colon , # [doc = " `::`"] PathSep , # [doc = " `->`"] RArrow , # [doc = " `<-`"] LArrow , # [doc = " `=>`"] FatArrow , # [doc = " `#`"] Pound , # [doc = " `$`"] Dollar , # [doc = " `?`"] Question , # [doc = " Used by proc macros for representing lifetimes, not generated by lexer right now."] SingleQuote , # [doc = " `(`"] OpenParen , # [doc = " `)`"] CloseParen , # [doc = " `{`"] OpenBrace , # [doc = " `}`"] CloseBrace , # [doc = " `[`"] OpenBracket , # [doc = " `]`"] CloseBracket , # [doc = " Invisible opening delimiter, produced by a macro."] OpenInvisible (InvisibleOrigin) , # [doc = " Invisible closing delimiter, produced by a macro."] CloseInvisible (InvisibleOrigin) , Literal (Lit) , # [doc = " Identifier token."] # [doc = " Do not forget about `NtIdent` when you want to match on identifiers."] # [doc = " It's recommended to use `Token::{ident,uninterpolate}` and"] # [doc = " `Parser::token_uninterpolated_span` to treat regular and interpolated"] # [doc = " identifiers in the same way."] Ident (Symbol , IdentIsRaw) , # [doc = " This identifier (and its span) is the identifier passed to the"] # [doc = " declarative macro. The span in the surrounding `Token` is the span of"] # [doc = " the `ident` metavariable in the macro's RHS."] NtIdent (Ident , IdentIsRaw) , # [doc = " Lifetime identifier token."] # [doc = " Do not forget about `NtLifetime` when you want to match on lifetime identifiers."] # [doc = " It's recommended to use `Token::{ident,uninterpolate}` and"] # [doc = " `Parser::token_uninterpolated_span` to treat regular and interpolated"] # [doc = " identifiers in the same way."] Lifetime (Symbol , IdentIsRaw) , # [doc = " This identifier (and its span) is the lifetime passed to the"] # [doc = " declarative macro. The span in the surrounding `Token` is the span of"] # [doc = " the `lifetime` metavariable in the macro's RHS."] NtLifetime (Ident , IdentIsRaw) , # [doc = " A doc comment token."] # [doc = " `Symbol` is the doc comment's data excluding its \"quotes\" (`///`, `/**`, etc)"] # [doc = " similarly to symbols in string literal tokens."] DocComment (CommentKind , ast :: AttrStyle , Symbol) , # [doc = " End Of File"] Eof , }}}
mkitem!{mkstruct!{# [derive (Clone , Copy , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub struct Token { pub kind : TokenKind , pub span : Span , }}}
mkitem!{mkimpl!{impl TokenKind { pub fn lit (kind : LitKind , symbol : Symbol , suffix : Option < Symbol >) -> TokenKind { Literal (Lit :: new (kind , symbol , suffix)) } # [doc = " An approximation to proc-macro-style single-character operators used by"] # [doc = " rustc parser. If the operator token can be broken into two tokens, the"] # [doc = " first of which has `n` (1 or 2) chars, then this function performs that"] # [doc = " operation, otherwise it returns `None`."] pub fn break_two_token_op (& self , n : u32) -> Option < (TokenKind , TokenKind) > { assert ! (n == 1 || n == 2) ; Some (match (self , n) { (Le , 1) => (Lt , Eq) , (EqEq , 1) => (Eq , Eq) , (Ne , 1) => (Bang , Eq) , (Ge , 1) => (Gt , Eq) , (AndAnd , 1) => (And , And) , (OrOr , 1) => (Or , Or) , (Shl , 1) => (Lt , Lt) , (Shr , 1) => (Gt , Gt) , (PlusEq , 1) => (Plus , Eq) , (MinusEq , 1) => (Minus , Eq) , (StarEq , 1) => (Star , Eq) , (SlashEq , 1) => (Slash , Eq) , (PercentEq , 1) => (Percent , Eq) , (CaretEq , 1) => (Caret , Eq) , (AndEq , 1) => (And , Eq) , (OrEq , 1) => (Or , Eq) , (ShlEq , 1) => (Lt , Le) , (ShlEq , 2) => (Shl , Eq) , (ShrEq , 1) => (Gt , Ge) , (ShrEq , 2) => (Shr , Eq) , (DotDot , 1) => (Dot , Dot) , (DotDotDot , 1) => (Dot , DotDot) , (DotDotDot , 2) => (DotDot , Dot) , (DotDotEq , 2) => (DotDot , Eq) , (PathSep , 1) => (Colon , Colon) , (RArrow , 1) => (Minus , Gt) , (LArrow , 1) => (Lt , Minus) , (FatArrow , 1) => (Eq , Gt) , _ => return None , }) } # [doc = " Returns tokens that are likely to be typed accidentally instead of the current token."] # [doc = " Enables better error recovery when the wrong token is found."] pub fn similar_tokens (& self) -> & [TokenKind] { match self { Comma => & [Dot , Lt , Semi] , Semi => & [Colon , Comma] , Colon => & [Semi] , FatArrow => & [Eq , RArrow , Ge , Gt] , _ => & [] , } } pub fn should_end_const_arg (& self) -> bool { matches ! (self , Gt | Ge | Shr | ShrEq) } pub fn is_delim (& self) -> bool { self . open_delim () . is_some () || self . close_delim () . is_some () } pub fn open_delim (& self) -> Option < Delimiter > { match * self { OpenParen => Some (Delimiter :: Parenthesis) , OpenBrace => Some (Delimiter :: Brace) , OpenBracket => Some (Delimiter :: Bracket) , OpenInvisible (origin) => Some (Delimiter :: Invisible (origin)) , _ => None , } } pub fn close_delim (& self) -> Option < Delimiter > { match * self { CloseParen => Some (Delimiter :: Parenthesis) , CloseBrace => Some (Delimiter :: Brace) , CloseBracket => Some (Delimiter :: Bracket) , CloseInvisible (origin) => Some (Delimiter :: Invisible (origin)) , _ => None , } } pub fn is_close_delim_or_eof (& self) -> bool { match self { CloseParen | CloseBrace | CloseBracket | CloseInvisible (_) | Eof => true , _ => false , } } }}}
mkitem!{mkimpl!{impl Token { pub fn new (kind : TokenKind , span : Span) -> Self { Token { kind , span } } # [doc = " Some token that will be thrown away later."] pub fn dummy () -> Self { Token :: new (TokenKind :: Question , DUMMY_SP) } # [doc = " Recovers a `Token` from an `Ident`. This creates a raw identifier if necessary."] pub fn from_ast_ident (ident : Ident) -> Self { Token :: new (Ident (ident . name , ident . is_raw_guess () . into ()) , ident . span) } pub fn is_range_separator (& self) -> bool { [DotDot , DotDotDot , DotDotEq] . contains (& self . kind) } pub fn is_punct (& self) -> bool { match self . kind { Eq | Lt | Le | EqEq | Ne | Ge | Gt | AndAnd | OrOr | Bang | Tilde | Plus | Minus | Star | Slash | Percent | Caret | And | Or | Shl | Shr | PlusEq | MinusEq | StarEq | SlashEq | PercentEq | CaretEq | AndEq | OrEq | ShlEq | ShrEq | At | Dot | DotDot | DotDotDot | DotDotEq | Comma | Semi | Colon | PathSep | RArrow | LArrow | FatArrow | Pound | Dollar | Question | SingleQuote => true , OpenParen | CloseParen | OpenBrace | CloseBrace | OpenBracket | CloseBracket | OpenInvisible (_) | CloseInvisible (_) | Literal (..) | DocComment (..) | Ident (..) | NtIdent (..) | Lifetime (..) | NtLifetime (..) | Eof => false , } } pub fn is_like_plus (& self) -> bool { matches ! (self . kind , Plus | PlusEq) } # [doc = " Returns `true` if the token can appear at the start of an expression."] # [doc = ""] # [doc = " **NB**: Take care when modifying this function, since it will change"] # [doc = " the stable set of tokens that are allowed to match an expr nonterminal."] pub fn can_begin_expr (& self) -> bool { match self . uninterpolate () . kind { Ident (name , is_raw) => ident_can_begin_expr (name , self . span , is_raw) , OpenParen | OpenBrace | OpenBracket | Literal (..) | Bang | Minus | Star | Or | OrOr | And | AndAnd | DotDot | DotDotDot | DotDotEq | Lt | Shl | PathSep | Lifetime (..) | Pound => true , OpenInvisible (InvisibleOrigin :: MetaVar (MetaVarKind :: Block | MetaVarKind :: Expr { .. } | MetaVarKind :: Literal | MetaVarKind :: Path)) => true , _ => false , } } # [doc = " Returns `true` if the token can appear at the start of a pattern."] # [doc = ""] # [doc = " Shamelessly borrowed from `can_begin_expr`, only used for diagnostics right now."] pub fn can_begin_pattern (& self , pat_kind : NtPatKind) -> bool { match & self . uninterpolate () . kind { Ident (..) | NtIdent (..) | OpenParen | OpenBracket | And | Minus | AndAnd | Literal (_) | DotDot | DotDotDot | PathSep | Lt | Shl => true , Or => matches ! (pat_kind , PatWithOr) , OpenInvisible (InvisibleOrigin :: MetaVar (MetaVarKind :: Expr { .. } | MetaVarKind :: Literal | MetaVarKind :: Meta { .. } | MetaVarKind :: Pat (_) | MetaVarKind :: Path | MetaVarKind :: Ty { .. })) => true , _ => false , } } # [doc = " Returns `true` if the token can appear at the start of a type."] pub fn can_begin_type (& self) -> bool { match self . uninterpolate () . kind { Ident (name , is_raw) => ident_can_begin_type (name , self . span , is_raw) , OpenParen | OpenBracket | Bang | Star | And | AndAnd | Question | Lifetime (..) | Lt | Shl | PathSep => true , OpenInvisible (InvisibleOrigin :: MetaVar (MetaVarKind :: Ty { .. } | MetaVarKind :: Path)) => true , _ => false , } } # [doc = " Returns `true` if the token can appear at the start of a const param."] pub fn can_begin_const_arg (& self) -> bool { match self . kind { OpenBrace | Literal (..) | Minus => true , Ident (name , IdentIsRaw :: No) if name . is_bool_lit () => true , OpenInvisible (InvisibleOrigin :: MetaVar (MetaVarKind :: Expr { .. } | MetaVarKind :: Block | MetaVarKind :: Literal ,)) => true , _ => false , } } # [doc = " Returns `true` if the token can appear at the start of an item."] pub fn can_begin_item (& self) -> bool { match self . kind { Ident (name , _) => [kw :: Fn , kw :: Use , kw :: Struct , kw :: Enum , kw :: Pub , kw :: Trait , kw :: Extern , kw :: Impl , kw :: Unsafe , kw :: Const , kw :: Safe , kw :: Static , kw :: Union , kw :: Macro , kw :: Mod , kw :: Type ,] . contains (& name) , _ => false , } } # [doc = " Returns `true` if the token is any literal."] pub fn is_lit (& self) -> bool { matches ! (self . kind , Literal (..)) } # [doc = " Returns `true` if the token is any literal, a minus (which can prefix a literal,"] # [doc = " for example a '-42', or one of the boolean idents)."] # [doc = ""] # [doc = " In other words, would this token be a valid start of `parse_literal_maybe_minus`?"] # [doc = ""] # [doc = " Keep this in sync with `Lit::from_token` and `Parser::eat_token_lit`"] # [doc = " (excluding unary negation)."] pub fn can_begin_literal_maybe_minus (& self) -> bool { match self . uninterpolate () . kind { Literal (..) | Minus => true , Ident (name , IdentIsRaw :: No) if name . is_bool_lit () => true , OpenInvisible (InvisibleOrigin :: MetaVar (mv_kind)) => match mv_kind { MetaVarKind :: Literal => true , MetaVarKind :: Expr { can_begin_literal_maybe_minus , .. } => { can_begin_literal_maybe_minus } _ => false , } , _ => false , } } pub fn can_begin_string_literal (& self) -> bool { match self . uninterpolate () . kind { Literal (..) => true , OpenInvisible (InvisibleOrigin :: MetaVar (mv_kind)) => match mv_kind { MetaVarKind :: Literal => true , MetaVarKind :: Expr { can_begin_string_literal , .. } => can_begin_string_literal , _ => false , } , _ => false , } } # [doc = " A convenience function for matching on identifiers during parsing."] # [doc = " Turns interpolated identifier (`$i: ident`) or lifetime (`$l: lifetime`) token"] # [doc = " into the regular identifier or lifetime token it refers to,"] # [doc = " otherwise returns the original token."] pub fn uninterpolate (& self) -> Cow < '_ , Token > { match self . kind { NtIdent (ident , is_raw) => Cow :: Owned (Token :: new (Ident (ident . name , is_raw) , ident . span)) , NtLifetime (ident , is_raw) => { Cow :: Owned (Token :: new (Lifetime (ident . name , is_raw) , ident . span)) } _ => Cow :: Borrowed (self) , } } # [doc = " Returns an identifier if this token is an identifier."] # [inline] pub fn ident (& self) -> Option < (Ident , IdentIsRaw) > { match self . kind { Ident (name , is_raw) => Some ((Ident :: new (name , self . span) , is_raw)) , NtIdent (ident , is_raw) => Some ((ident , is_raw)) , _ => None , } } # [doc = " Returns a lifetime identifier if this token is a lifetime."] # [inline] pub fn lifetime (& self) -> Option < (Ident , IdentIsRaw) > { match self . kind { Lifetime (name , is_raw) => Some ((Ident :: new (name , self . span) , is_raw)) , NtLifetime (ident , is_raw) => Some ((ident , is_raw)) , _ => None , } } # [doc = " Returns `true` if the token is an identifier."] pub fn is_ident (& self) -> bool { self . ident () . is_some () } # [doc = " Returns `true` if the token is a lifetime."] pub fn is_lifetime (& self) -> bool { self . lifetime () . is_some () } # [doc = " Returns `true` if the token is an identifier whose name is the given"] # [doc = " string slice."] pub fn is_ident_named (& self , name : Symbol) -> bool { self . ident () . is_some_and (| (ident , _) | ident . name == name) } # [doc = " Is this a pre-parsed expression dropped into the token stream"] # [doc = " (which happens while parsing the result of macro expansion)?"] pub fn is_metavar_expr (& self) -> bool { matches ! (self . is_metavar_seq () , Some (MetaVarKind :: Expr { .. } | MetaVarKind :: Literal | MetaVarKind :: Path | MetaVarKind :: Block)) } # [doc = " Are we at a block from a metavar (`$b:block`)?"] pub fn is_metavar_block (& self) -> bool { matches ! (self . is_metavar_seq () , Some (MetaVarKind :: Block)) } # [doc = " Returns `true` if the token is either the `mut` or `const` keyword."] pub fn is_mutability (& self) -> bool { self . is_keyword (kw :: Mut) || self . is_keyword (kw :: Const) } pub fn is_qpath_start (& self) -> bool { self == & Lt || self == & Shl } pub fn is_path_start (& self) -> bool { self == & PathSep || self . is_qpath_start () || matches ! (self . is_metavar_seq () , Some (MetaVarKind :: Path)) || self . is_path_segment_keyword () || self . is_non_reserved_ident () } # [doc = " Returns `true` if the token is a given keyword, `kw`."] pub fn is_keyword (& self , kw : Symbol) -> bool { self . is_non_raw_ident_where (| id | id . name == kw) } # [doc = " Returns `true` if the token is a given keyword, `kw` or if `case` is `Insensitive` and this"] # [doc = " token is an identifier equal to `kw` ignoring the case."] pub fn is_keyword_case (& self , kw : Symbol , case : Case) -> bool { self . is_keyword (kw) || (case == Case :: Insensitive && self . is_non_raw_ident_where (| id | { id . name . as_str () . eq_ignore_ascii_case (kw . as_str ()) })) } pub fn is_path_segment_keyword (& self) -> bool { self . is_non_raw_ident_where (Ident :: is_path_segment_keyword) } # [doc = " Returns true for reserved identifiers used internally for elided lifetimes,"] # [doc = " unnamed method parameters, crate root module, error recovery etc."] pub fn is_special_ident (& self) -> bool { self . is_non_raw_ident_where (Ident :: is_special) } # [doc = " Returns `true` if the token is a keyword used in the language."] pub fn is_used_keyword (& self) -> bool { self . is_non_raw_ident_where (Ident :: is_used_keyword) } # [doc = " Returns `true` if the token is a keyword reserved for possible future use."] pub fn is_unused_keyword (& self) -> bool { self . is_non_raw_ident_where (Ident :: is_unused_keyword) } # [doc = " Returns `true` if the token is either a special identifier or a keyword."] pub fn is_reserved_ident (& self) -> bool { self . is_non_raw_ident_where (Ident :: is_reserved) } pub fn is_non_reserved_ident (& self) -> bool { self . ident () . is_some_and (| (id , raw) | raw == IdentIsRaw :: Yes || ! Ident :: is_reserved (id)) } # [doc = " Returns `true` if the token is the identifier `true` or `false`."] pub fn is_bool_lit (& self) -> bool { self . is_non_raw_ident_where (| id | id . name . is_bool_lit ()) } pub fn is_numeric_lit (& self) -> bool { matches ! (self . kind , Literal (Lit { kind : LitKind :: Integer , .. }) | Literal (Lit { kind : LitKind :: Float , .. })) } # [doc = " Returns `true` if the token is the integer literal."] pub fn is_integer_lit (& self) -> bool { matches ! (self . kind , Literal (Lit { kind : LitKind :: Integer , .. })) } # [doc = " Returns `true` if the token is a non-raw identifier for which `pred` holds."] pub fn is_non_raw_ident_where (& self , pred : impl FnOnce (Ident) -> bool) -> bool { match self . ident () { Some ((id , IdentIsRaw :: No)) => pred (id) , _ => false , } } # [doc = " Is this an invisible open delimiter at the start of a token sequence"] # [doc = " from an expanded metavar?"] pub fn is_metavar_seq (& self) -> Option < MetaVarKind > { match self . kind { OpenInvisible (InvisibleOrigin :: MetaVar (kind)) => Some (kind) , _ => None , } } pub fn glue (& self , joint : & Token) -> Option < Token > { let kind = match (& self . kind , & joint . kind) { (Eq , Eq) => EqEq , (Eq , Gt) => FatArrow , (Eq , _) => return None , (Lt , Eq) => Le , (Lt , Lt) => Shl , (Lt , Le) => ShlEq , (Lt , Minus) => LArrow , (Lt , _) => return None , (Gt , Eq) => Ge , (Gt , Gt) => Shr , (Gt , Ge) => ShrEq , (Gt , _) => return None , (Bang , Eq) => Ne , (Bang , _) => return None , (Plus , Eq) => PlusEq , (Plus , _) => return None , (Minus , Eq) => MinusEq , (Minus , Gt) => RArrow , (Minus , _) => return None , (Star , Eq) => StarEq , (Star , _) => return None , (Slash , Eq) => SlashEq , (Slash , _) => return None , (Percent , Eq) => PercentEq , (Percent , _) => return None , (Caret , Eq) => CaretEq , (Caret , _) => return None , (And , Eq) => AndEq , (And , And) => AndAnd , (And , _) => return None , (Or , Eq) => OrEq , (Or , Or) => OrOr , (Or , _) => return None , (Shl , Eq) => ShlEq , (Shl , _) => return None , (Shr , Eq) => ShrEq , (Shr , _) => return None , (Dot , Dot) => DotDot , (Dot , DotDot) => DotDotDot , (Dot , _) => return None , (DotDot , Dot) => DotDotDot , (DotDot , Eq) => DotDotEq , (DotDot , _) => return None , (Colon , Colon) => PathSep , (Colon , _) => return None , (SingleQuote , Ident (name , is_raw)) => { Lifetime (Symbol :: intern (& format ! ("'{name}")) , * is_raw) } (SingleQuote , _) => return None , (Le | EqEq | Ne | Ge | AndAnd | OrOr | Tilde | PlusEq | MinusEq | StarEq | SlashEq | PercentEq | CaretEq | AndEq | OrEq | ShlEq | ShrEq | At | DotDotDot | DotDotEq | Comma | Semi | PathSep | RArrow | LArrow | FatArrow | Pound | Dollar | Question | OpenParen | CloseParen | OpenBrace | CloseBrace | OpenBracket | CloseBracket | OpenInvisible (_) | CloseInvisible (_) | Literal (..) | Ident (..) | NtIdent (..) | Lifetime (..) | NtLifetime (..) | DocComment (..) | Eof , _ ,) => { return None ; } } ; Some (Token :: new (kind , self . span . to (joint . span))) } }}}
mkitem!{mkimpl!{impl PartialEq < TokenKind > for Token { # [inline] fn eq (& self , rhs : & TokenKind) -> bool { self . kind == * rhs } }}}
mkitem!{mkenum!{# [derive (Debug , Copy , Clone , PartialEq , Eq , Encodable , Decodable , Hash , HashStable_Generic)] pub enum NtPatKind { PatWithOr , PatParam { inferred : bool } , }}}
mkitem!{mkenum!{# [derive (Debug , Copy , Clone , PartialEq , Eq , Encodable , Decodable , Hash , HashStable_Generic)] pub enum NtExprKind { Expr , Expr2021 { inferred : bool } , }}}
mkitem!{mkenum!{# [doc = " A macro nonterminal, known in documentation as a fragment specifier."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Encodable , Decodable , Hash , HashStable_Generic)] pub enum NonterminalKind { Item , Block , Stmt , Pat (NtPatKind) , Expr (NtExprKind) , Ty , Ident , Lifetime , Literal , Meta , Path , Vis , TT , }}}
mkitem!{mkimpl!{impl NonterminalKind { # [doc = " The `edition` closure is used to get the edition for the given symbol. Doing"] # [doc = " `span.edition()` is expensive, so we do it lazily."] pub fn from_symbol (symbol : Symbol , edition : impl FnOnce () -> Edition ,) -> Option < NonterminalKind > { Some (match symbol { sym :: item => NonterminalKind :: Item , sym :: block => NonterminalKind :: Block , sym :: stmt => NonterminalKind :: Stmt , sym :: pat => { if edition () . at_least_rust_2021 () { NonterminalKind :: Pat (PatWithOr) } else { NonterminalKind :: Pat (PatParam { inferred : true }) } } sym :: pat_param => NonterminalKind :: Pat (PatParam { inferred : false }) , sym :: expr => { if edition () . at_least_rust_2024 () { NonterminalKind :: Expr (Expr) } else { NonterminalKind :: Expr (Expr2021 { inferred : true }) } } sym :: expr_2021 => NonterminalKind :: Expr (Expr2021 { inferred : false }) , sym :: ty => NonterminalKind :: Ty , sym :: ident => NonterminalKind :: Ident , sym :: lifetime => NonterminalKind :: Lifetime , sym :: literal => NonterminalKind :: Literal , sym :: meta => NonterminalKind :: Meta , sym :: path => NonterminalKind :: Path , sym :: vis => NonterminalKind :: Vis , sym :: tt => NonterminalKind :: TT , _ => return None , }) } fn symbol (self) -> Symbol { match self { NonterminalKind :: Item => sym :: item , NonterminalKind :: Block => sym :: block , NonterminalKind :: Stmt => sym :: stmt , NonterminalKind :: Pat (PatParam { inferred : true } | PatWithOr) => sym :: pat , NonterminalKind :: Pat (PatParam { inferred : false }) => sym :: pat_param , NonterminalKind :: Expr (Expr2021 { inferred : true } | Expr) => sym :: expr , NonterminalKind :: Expr (Expr2021 { inferred : false }) => sym :: expr_2021 , NonterminalKind :: Ty => sym :: ty , NonterminalKind :: Ident => sym :: ident , NonterminalKind :: Lifetime => sym :: lifetime , NonterminalKind :: Literal => sym :: literal , NonterminalKind :: Meta => sym :: meta , NonterminalKind :: Path => sym :: path , NonterminalKind :: Vis => sym :: vis , NonterminalKind :: TT => sym :: tt , } } }}}
mkitem!{mkimpl!{impl fmt :: Display for NonterminalKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . symbol ()) } }}}
mkmod!{size_asserts, { 
                getname!(size_asserts);
                getsrc!(size_asserts);
                getpath!(size_asserts);
                get_deps!(size_asserts);
                get_crates!(size_asserts);
                mkinclude!(size_asserts);
                mkuse!{use rustc_data_structures :: static_assert_size ;}
mkuse!{use super :: * ;}
mkitem!{static_assert_size ! (Lit , 12) ;}
mkitem!{static_assert_size ! (LitKind , 2) ;}
mkitem!{static_assert_size ! (Token , 24) ;}
mkitem!{static_assert_size ! (TokenKind , 16) ;} 
            }}