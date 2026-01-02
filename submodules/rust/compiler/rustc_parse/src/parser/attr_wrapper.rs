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
mkuse!{use std :: borrow :: Cow ;}
mkuse!{use std :: mem ;}
mkuse!{use rustc_ast :: token :: Token ;}
mkuse!{use rustc_ast :: tokenstream :: { AttrsTarget , LazyAttrTokenStream , NodeRange , ParserRange , Spacing , TokenCursor , } ;}
mkuse!{use rustc_ast :: { self as ast , AttrVec , Attribute , HasAttrs , HasTokens } ;}
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_errors :: PResult ;}
mkuse!{use rustc_session :: parse :: ParseSess ;}
mkuse!{use rustc_span :: { DUMMY_SP , sym } ;}
mkuse!{use thin_vec :: ThinVec ;}
mkuse!{use super :: { Capturing , ForceCollect , Parser , Trailing } ;}
mkitem!{mkstruct!{#[derive (Clone , Debug)] pub (super) struct CollectPos { start_token : (Token , Spacing) , cursor_snapshot : TokenCursor , start_pos : u32 , }}}
mkitem!{mkenum!{pub (super) enum UsePreAttrPos { No , Yes , }}}
mkitem!{mkstruct!{#[doc = " A wrapper type to ensure that the parser handles outer attributes correctly."] #[doc = " When we parse outer attributes, we need to ensure that we capture tokens"] #[doc = " for the attribute target. This allows us to perform cfg-expansion on"] #[doc = " a token stream before we invoke a derive proc-macro."] #[doc = ""] #[doc = " This wrapper prevents direct access to the underlying `ast::AttrVec`."] #[doc = " Parsing code can only get access to the underlying attributes"] #[doc = " by passing an `AttrWrapper` to `collect_tokens`."] #[doc = " This makes it difficult to accidentally construct an AST node"] #[doc = " (which stores an `ast::AttrVec`) without first collecting tokens."] #[doc = ""] #[doc = " This struct has its own module, to ensure that the parser code"] #[doc = " cannot directly access the `attrs` field."] #[derive (Debug , Clone)] pub (super) struct AttrWrapper { attrs : AttrVec , start_pos : Option < u32 > , }}}
mkitem!{mkimpl!{impl AttrWrapper { pub (super) fn new (attrs : AttrVec , start_pos : u32) -> AttrWrapper { AttrWrapper { attrs , start_pos : Some (start_pos) } } pub (super) fn empty () -> AttrWrapper { AttrWrapper { attrs : AttrVec :: new () , start_pos : None } } pub (super) fn take_for_recovery (self , psess : & ParseSess) -> AttrVec { psess . dcx () . span_delayed_bug (self . attrs . get (0) . map (| attr | attr . span) . unwrap_or (DUMMY_SP) , "AttrVec is taken for recovery but no error is produced" ,) ; self . attrs } #[doc = " Prepend `self.attrs` to `attrs`."] pub (super) fn prepend_to_nt_inner (mut self , attrs : & mut AttrVec) { mem :: swap (attrs , & mut self . attrs) ; attrs . extend (self . attrs) ; } pub (super) fn is_empty (& self) -> bool { self . attrs . is_empty () } }}}

macro_rules! has_cfg_or_cfg_attr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function has_cfg_or_cfg_attr in module {}", module_path!());
    };
}

mkfn!{
    has_cfg_or_cfg_attr_introspect!();
    #[doc = " Returns `true` if `attrs` contains a `cfg` or `cfg_attr` attribute"] fn has_cfg_or_cfg_attr (attrs : & [Attribute]) -> bool { attrs . iter () . any (| attr | { attr . ident () . is_some_and (| ident | ident . name == sym :: cfg || ident . name == sym :: cfg_attr) }) }
}
mkitem!{mkimpl!{impl < 'a > Parser < 'a > { pub (super) fn collect_pos (& self) -> CollectPos { CollectPos { start_token : (self . token , self . token_spacing) , cursor_snapshot : self . token_cursor . clone () , start_pos : self . num_bump_calls , } } #[doc = " Parses code with `f`. If appropriate, it records the tokens (in"] #[doc = " `LazyAttrTokenStream` form) that were parsed in the result, accessible"] #[doc = " via the `HasTokens` trait. The `Trailing` part of the callback's"] #[doc = " result indicates if an extra token should be captured, e.g. a comma or"] #[doc = " semicolon. The `UsePreAttrPos` part of the callback's result indicates"] #[doc = " if we should use `pre_attr_pos` as the collection start position (only"] #[doc = " required in a few cases)."] #[doc = ""] #[doc = " The `attrs` passed in are in `AttrWrapper` form, which is opaque. The"] #[doc = " `AttrVec` within is passed to `f`. See the comment on `AttrWrapper` for"] #[doc = " details."] #[doc = ""] #[doc = " `pre_attr_pos` is the position before the outer attributes (or the node"] #[doc = " itself, if no outer attributes are present). It is only needed if `f`"] #[doc = " can return `UsePreAttrPos::Yes`."] #[doc = ""] #[doc = " Note: If your callback consumes an opening delimiter (including the"] #[doc = " case where `self.token` is an opening delimiter on entry to this"] #[doc = " function), you must also consume the corresponding closing delimiter."] #[doc = " E.g. you can consume `something ([{ }])` or `([{}])`, but not `([{}]`."] #[doc = " This restriction isn't a problem in practice, because parsed AST items"] #[doc = " always have matching delimiters."] #[doc = ""] #[doc = " The following example code will be used to explain things in comments"] #[doc = " below. It has an outer attribute and an inner attribute. Parsing it"] #[doc = " involves two calls to this method, one of which is indirectly"] #[doc = " recursive."] #[doc = " ```ignore (fake attributes)"] #[doc = " #[cfg_eval]                         // token pos"] #[doc = " mod m {                             //   0.. 3"] #[doc = "     #[cfg_attr(cond1, attr1)]       //   3..12"] #[doc = "     fn g() {                        //  12..17"] #[doc = "         #![cfg_attr(cond2, attr2)]  //  17..27"] #[doc = "         let _x = 3;                 //  27..32"] #[doc = "     }                               //  32..33"] #[doc = " }                                   //  33..34"] #[doc = " ```"] pub (super) fn collect_tokens < R : HasAttrs + HasTokens > (& mut self , pre_attr_pos : Option < CollectPos > , attrs : AttrWrapper , force_collect : ForceCollect , f : impl FnOnce (& mut Self , AttrVec) -> PResult < 'a , (R , Trailing , UsePreAttrPos) > ,) -> PResult < 'a , R > { let possible_capture_mode = self . capture_cfg ; let needs_collection = matches ! (force_collect , ForceCollect :: Yes) || needs_tokens (& attrs . attrs) || R :: SUPPORTS_CUSTOM_INNER_ATTRS || possible_capture_mode ; if ! needs_collection { return Ok (f (self , attrs . attrs) ? . 0) ; } let mut collect_pos = self . collect_pos () ; let has_outer_attrs = ! attrs . attrs . is_empty () ; let parser_replacements_start = self . capture_state . parser_replacements . len () ; let (mut ret , capture_trailing , use_pre_attr_pos) = { let prev_capturing = mem :: replace (& mut self . capture_state . capturing , Capturing :: Yes) ; let res = f (self , attrs . attrs) ; self . capture_state . capturing = prev_capturing ; res ? } ; let ret_can_hold_tokens = matches ! (ret . tokens_mut () , Some (None)) ; let mut seen_indices = FxHashSet :: default () ; for (i , attr) in ret . attrs () . iter () . enumerate () { let is_unseen = self . capture_state . seen_attrs . insert (attr . id) ; if ! is_unseen { seen_indices . insert (i) ; } } let ret_attrs : Cow < '_ , [Attribute] > = if seen_indices . is_empty () { Cow :: Borrowed (ret . attrs ()) } else { let ret_attrs = ret . attrs () . iter () . enumerate () . filter_map (| (i , attr) | { if seen_indices . contains (& i) { None } else { Some (attr . clone ()) } }) . collect () ; Cow :: Owned (ret_attrs) } ; let definite_capture_mode = self . capture_cfg && matches ! (self . capture_state . capturing , Capturing :: Yes) && has_cfg_or_cfg_attr (& ret_attrs) ; if ! definite_capture_mode && ! ret_can_hold_tokens { return Ok (ret) ; } let needs_collection = matches ! (force_collect , ForceCollect :: Yes) || needs_tokens (& ret_attrs) || definite_capture_mode ; if ! needs_collection { return Ok (ret) ; } if matches ! (use_pre_attr_pos , UsePreAttrPos :: Yes) { collect_pos = pre_attr_pos . unwrap () ; } let parser_replacements_end = self . capture_state . parser_replacements . len () ; assert ! (! (self . break_last_token > 0 && matches ! (capture_trailing , Trailing :: Yes)) , "Cannot have break_last_token > 0 and have trailing token") ; assert ! (self . break_last_token <= 2 , "cannot break token more than twice") ; let end_pos = self . num_bump_calls + capture_trailing as u32 + if self . break_last_token == 0 { 0 } else { 1 } ; let num_calls = end_pos - collect_pos . start_pos ; let mut inner_attr_parser_replacements = Vec :: new () ; for attr in ret_attrs . iter () { if attr . style == ast :: AttrStyle :: Inner { if let Some (inner_attr_parser_range) = self . capture_state . inner_attr_parser_ranges . remove (& attr . id) { inner_attr_parser_replacements . push ((inner_attr_parser_range , None)) ; } else { self . dcx () . span_delayed_bug (attr . span , "Missing token range for attribute") ; } } } let node_replacements = if parser_replacements_start == parser_replacements_end && inner_attr_parser_replacements . is_empty () { ThinVec :: new () } else { self . capture_state . parser_replacements [parser_replacements_start .. parser_replacements_end] . iter () . cloned () . chain (inner_attr_parser_replacements) . map (| (parser_range , data) | { (NodeRange :: new (parser_range , collect_pos . start_pos) , data) }) . collect () } ; let tokens = LazyAttrTokenStream :: new_pending (collect_pos . start_token , collect_pos . cursor_snapshot , num_calls , self . break_last_token , node_replacements ,) ; let mut tokens_used = false ; if definite_capture_mode { assert ! (self . break_last_token == 0 , "Should not have unglued last token with cfg attr") ; let start_pos = if has_outer_attrs { attrs . start_pos . unwrap () } else { collect_pos . start_pos } ; let target = AttrsTarget { attrs : ret_attrs . iter () . cloned () . collect () , tokens : tokens . clone () } ; tokens_used = true ; self . capture_state . parser_replacements . push ((ParserRange (start_pos .. end_pos) , Some (target))) ; } else if matches ! (self . capture_state . capturing , Capturing :: No) { self . capture_state . parser_replacements . clear () ; self . capture_state . inner_attr_parser_ranges . clear () ; self . capture_state . seen_attrs . clear () ; } if let Some (target_tokens @ None) = ret . tokens_mut () { tokens_used = true ; * target_tokens = Some (tokens) ; } assert ! (tokens_used) ; Ok (ret) } }}}

macro_rules! needs_tokens_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function needs_tokens in module {}", module_path!());
    };
}

mkfn!{
    needs_tokens_introspect!();
    #[doc = " Tokens are needed if:"] #[doc = " - any non-single-segment attributes (other than doc comments) are present,"] #[doc = "   e.g. `rustfmt::skip`; or"] #[doc = " - any `cfg_attr` attributes are present; or"] #[doc = " - any single-segment, non-builtin attributes are present, e.g. `derive`,"] #[doc = "   `test`, `global_allocator`."] fn needs_tokens (attrs : & [ast :: Attribute]) -> bool { attrs . iter () . any (| attr | match attr . ident () { None => ! attr . is_doc_comment () , Some (ident) => { ident . name == sym :: cfg_attr || ! rustc_feature :: is_builtin_attr_name (ident . name) } }) }
}