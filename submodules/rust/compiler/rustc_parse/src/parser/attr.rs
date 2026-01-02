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
mkuse!{use rustc_ast as ast ;}
mkuse!{use rustc_ast :: token :: { self , MetaVarKind } ;}
mkuse!{use rustc_ast :: tokenstream :: ParserRange ;}
mkuse!{use rustc_ast :: { Attribute , attr } ;}
mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_errors :: { Diag , PResult } ;}
mkuse!{use rustc_span :: { BytePos , Span } ;}
mkuse!{use thin_vec :: ThinVec ;}
mkuse!{use tracing :: debug ;}
mkuse!{use super :: { AttrWrapper , Capturing , FnParseMode , ForceCollect , Parser , PathStyle , Trailing , UsePreAttrPos , } ;}
mkuse!{use crate :: parser :: FnContext ;}
mkuse!{use crate :: { errors , exp , fluent_generated as fluent } ;}
mkitem!{mkenum!{# [derive (Debug)] pub enum InnerAttrPolicy { Permitted , Forbidden (Option < InnerAttrForbiddenReason >) , }}}
mkitem!{mkenum!{# [derive (Clone , Copy , Debug)] pub enum InnerAttrForbiddenReason { InCodeBlock , AfterOuterDocComment { prev_doc_comment_span : Span } , AfterOuterAttribute { prev_outer_attr_sp : Span } , }}}
mkitem!{mkenum!{enum OuterAttributeType { DocComment , DocBlockComment , Attribute , }}}
mkitem!{mkenum!{# [derive (Clone , Copy , PartialEq , Eq)] pub enum AllowLeadingUnsafe { Yes , No , }}}
mkitem!{mkimpl!{impl < 'a > Parser < 'a > { # [doc = " Parses attributes that appear before an item."] pub (super) fn parse_outer_attributes (& mut self) -> PResult < 'a , AttrWrapper > { let mut outer_attrs = ast :: AttrVec :: new () ; let mut just_parsed_doc_comment = false ; let start_pos = self . num_bump_calls ; loop { let attr = if self . check (exp ! (Pound)) { let prev_outer_attr_sp = outer_attrs . last () . map (| attr : & Attribute | attr . span) ; let inner_error_reason = if just_parsed_doc_comment { Some (InnerAttrForbiddenReason :: AfterOuterDocComment { prev_doc_comment_span : prev_outer_attr_sp . unwrap () , }) } else { prev_outer_attr_sp . map (| prev_outer_attr_sp | { InnerAttrForbiddenReason :: AfterOuterAttribute { prev_outer_attr_sp } }) } ; let inner_parse_policy = InnerAttrPolicy :: Forbidden (inner_error_reason) ; just_parsed_doc_comment = false ; Some (self . parse_attribute (inner_parse_policy) ?) } else if let token :: DocComment (comment_kind , attr_style , data) = self . token . kind { if attr_style != ast :: AttrStyle :: Outer { let span = self . token . span ; let mut err = self . dcx () . struct_span_err (span , fluent :: parse_inner_doc_comment_not_permitted) ; err . code (E0753) ; if let Some (replacement_span) = self . annotate_following_item_if_applicable (& mut err , span , match comment_kind { token :: CommentKind :: Line => OuterAttributeType :: DocComment , token :: CommentKind :: Block => OuterAttributeType :: DocBlockComment , } , true ,) { err . note (fluent :: parse_note) ; err . span_suggestion_verbose (replacement_span , fluent :: parse_suggestion , "" , rustc_errors :: Applicability :: MachineApplicable ,) ; } err . emit () ; } self . bump () ; just_parsed_doc_comment = true ; Some (attr :: mk_doc_comment (& self . psess . attr_id_generator , comment_kind , ast :: AttrStyle :: Outer , data , self . prev_token . span ,)) } else { None } ; if let Some (attr) = attr { if attr . style == ast :: AttrStyle :: Outer { outer_attrs . push (attr) ; } } else { break ; } } Ok (AttrWrapper :: new (outer_attrs , start_pos)) } # [doc = " Matches `attribute = # ! [ meta_item ]`."] # [doc = " `inner_parse_policy` prescribes how to handle inner attributes."] pub fn parse_attribute (& mut self , inner_parse_policy : InnerAttrPolicy ,) -> PResult < 'a , ast :: Attribute > { debug ! ("parse_attribute: inner_parse_policy={:?} self.token={:?}" , inner_parse_policy , self . token) ; let lo = self . token . span ; self . collect_tokens_no_attrs (| this | { let pound_hi = this . token . span . hi () ; assert ! (this . eat (exp ! (Pound)) , "parse_attribute called in non-attribute position") ; let not_lo = this . token . span . lo () ; let style = if this . eat (exp ! (Bang)) { ast :: AttrStyle :: Inner } else { ast :: AttrStyle :: Outer } ; let mut bracket_res = this . expect (exp ! (OpenBracket)) ; if let Err (err) = & mut bracket_res && style == ast :: AttrStyle :: Inner && pound_hi == not_lo { err . note ("the token sequence `#!` here looks like the start of \
                    a shebang interpreter directive but it is not" ,) ; err . help ("if you meant this to be a shebang interpreter directive, \
                    move it to the very start of the file" ,) ; } bracket_res ? ; let item = this . parse_attr_item (ForceCollect :: No) ? ; this . expect (exp ! (CloseBracket)) ? ; let attr_sp = lo . to (this . prev_token . span) ; if style == ast :: AttrStyle :: Inner { this . error_on_forbidden_inner_attr (attr_sp , inner_parse_policy , item . is_valid_for_outer_style () ,) ; } Ok (attr :: mk_attr_from_item (& self . psess . attr_id_generator , item , None , style , attr_sp)) }) } fn annotate_following_item_if_applicable (& self , err : & mut Diag < '_ > , span : Span , attr_type : OuterAttributeType , suggest_to_outer : bool ,) -> Option < Span > { let mut snapshot = self . create_snapshot_for_diagnostic () ; let lo = span . lo () + BytePos (match attr_type { OuterAttributeType :: Attribute => 1 , _ => 2 , }) ; let hi = lo + BytePos (1) ; let replacement_span = span . with_lo (lo) . with_hi (hi) ; if let OuterAttributeType :: DocBlockComment | OuterAttributeType :: DocComment = attr_type { snapshot . bump () ; } loop { if snapshot . token == token :: Pound { if let Err (err) = snapshot . parse_attribute (InnerAttrPolicy :: Permitted) { err . cancel () ; return Some (replacement_span) ; } } else { break ; } } match snapshot . parse_item_common (AttrWrapper :: empty () , true , false , FnParseMode { req_name : | _ | true , context : FnContext :: Free , req_body : true } , ForceCollect :: No ,) { Ok (Some (item)) => { err . arg ("item" , item . kind . descr ()) ; err . span_label (item . span , fluent :: parse_label_does_not_annotate_this) ; if suggest_to_outer { err . span_suggestion_verbose (replacement_span , fluent :: parse_sugg_change_inner_to_outer , match attr_type { OuterAttributeType :: Attribute => "" , OuterAttributeType :: DocBlockComment => "*" , OuterAttributeType :: DocComment => "/" , } , rustc_errors :: Applicability :: MachineApplicable ,) ; } return None ; } Err (item_err) => { item_err . cancel () ; } Ok (None) => { } } Some (replacement_span) } pub (super) fn error_on_forbidden_inner_attr (& self , attr_sp : Span , policy : InnerAttrPolicy , suggest_to_outer : bool ,) { if let InnerAttrPolicy :: Forbidden (reason) = policy { let mut diag = match reason . as_ref () . copied () { Some (InnerAttrForbiddenReason :: AfterOuterDocComment { prev_doc_comment_span }) => { self . dcx () . struct_span_err (attr_sp , fluent :: parse_inner_attr_not_permitted_after_outer_doc_comment ,) . with_span_label (attr_sp , fluent :: parse_label_attr) . with_span_label (prev_doc_comment_span , fluent :: parse_label_prev_doc_comment ,) } Some (InnerAttrForbiddenReason :: AfterOuterAttribute { prev_outer_attr_sp }) => self . dcx () . struct_span_err (attr_sp , fluent :: parse_inner_attr_not_permitted_after_outer_attr ,) . with_span_label (attr_sp , fluent :: parse_label_attr) . with_span_label (prev_outer_attr_sp , fluent :: parse_label_prev_attr) , Some (InnerAttrForbiddenReason :: InCodeBlock) | None => { self . dcx () . struct_span_err (attr_sp , fluent :: parse_inner_attr_not_permitted) } } ; diag . note (fluent :: parse_inner_attr_explanation) ; if self . annotate_following_item_if_applicable (& mut diag , attr_sp , OuterAttributeType :: Attribute , suggest_to_outer ,) . is_some () { diag . note (fluent :: parse_outer_attr_explanation) ; } ; diag . emit () ; } } # [doc = " Parses an inner part of an attribute (the path and following tokens)."] # [doc = " The tokens must be either a delimited token stream, or empty token stream,"] # [doc = " or the \"legacy\" key-value form."] # [doc = "     PATH `(` TOKEN_STREAM `)`"] # [doc = "     PATH `[` TOKEN_STREAM `]`"] # [doc = "     PATH `{` TOKEN_STREAM `}`"] # [doc = "     PATH"] # [doc = "     PATH `=` UNSUFFIXED_LIT"] # [doc = " The delimiters or `=` are still put into the resulting token stream."] pub fn parse_attr_item (& mut self , force_collect : ForceCollect) -> PResult < 'a , ast :: AttrItem > { if let Some (item) = self . eat_metavar_seq_with_matcher (| mv_kind | matches ! (mv_kind , MetaVarKind :: Meta { .. }) , | this | this . parse_attr_item (force_collect) ,) { return Ok (item) ; } self . collect_tokens (None , AttrWrapper :: empty () , force_collect , | this , _empty_attrs | { let is_unsafe = this . eat_keyword (exp ! (Unsafe)) ; let unsafety = if is_unsafe { let unsafe_span = this . prev_token . span ; this . expect (exp ! (OpenParen)) ? ; ast :: Safety :: Unsafe (unsafe_span) } else { ast :: Safety :: Default } ; let path = this . parse_path (PathStyle :: Mod) ? ; let args = this . parse_attr_args () ? ; if is_unsafe { this . expect (exp ! (CloseParen)) ? ; } Ok ((ast :: AttrItem { unsafety , path , args , tokens : None } , Trailing :: No , UsePreAttrPos :: No ,)) }) } # [doc = " Parses attributes that appear after the opening of an item. These should"] # [doc = " be preceded by an exclamation mark, but we accept and warn about one"] # [doc = " terminated by a semicolon."] # [doc = ""] # [doc = " Matches `inner_attrs*`."] pub fn parse_inner_attributes (& mut self) -> PResult < 'a , ast :: AttrVec > { let mut attrs = ast :: AttrVec :: new () ; loop { let start_pos = self . num_bump_calls ; let attr = if self . check (exp ! (Pound)) && self . look_ahead (1 , | t | t == & token :: Bang) { Some (self . parse_attribute (InnerAttrPolicy :: Permitted) ?) } else if let token :: DocComment (comment_kind , attr_style , data) = self . token . kind { if attr_style == ast :: AttrStyle :: Inner { self . bump () ; Some (attr :: mk_doc_comment (& self . psess . attr_id_generator , comment_kind , attr_style , data , self . prev_token . span ,)) } else { None } } else { None } ; if let Some (attr) = attr { if let Capturing :: Yes = self . capture_state . capturing { let end_pos = self . num_bump_calls ; let parser_range = ParserRange (start_pos .. end_pos) ; self . capture_state . inner_attr_parser_ranges . insert (attr . id , parser_range) ; } attrs . push (attr) ; } else { break ; } } Ok (attrs) } pub (crate) fn parse_unsuffixed_meta_item_lit (& mut self) -> PResult < 'a , ast :: MetaItemLit > { let lit = self . parse_meta_item_lit () ? ; debug ! ("checking if {:?} is unsuffixed" , lit) ; if ! lit . kind . is_unsuffixed () { self . dcx () . emit_err (errors :: SuffixedLiteralInAttribute { span : lit . span }) ; } Ok (lit) } # [doc = " Parses `cfg_attr(pred, attr_item_list)` where `attr_item_list` is comma-delimited."] pub fn parse_cfg_attr (& mut self ,) -> PResult < 'a , (ast :: MetaItemInner , Vec < (ast :: AttrItem , Span) >) > { let cfg_predicate = self . parse_meta_item_inner () ? ; self . expect (exp ! (Comma)) ? ; let mut expanded_attrs = Vec :: with_capacity (1) ; while self . token != token :: Eof { let lo = self . token . span ; let item = self . parse_attr_item (ForceCollect :: Yes) ? ; expanded_attrs . push ((item , lo . to (self . prev_token . span))) ; if ! self . eat (exp ! (Comma)) { break ; } } Ok ((cfg_predicate , expanded_attrs)) } # [doc = " Matches `COMMASEP(meta_item_inner)`."] pub fn parse_meta_seq_top (& mut self) -> PResult < 'a , ThinVec < ast :: MetaItemInner > > { let mut nmis = ThinVec :: with_capacity (1) ; while self . token != token :: Eof { nmis . push (self . parse_meta_item_inner () ?) ; if ! self . eat (exp ! (Comma)) { break ; } } Ok (nmis) } # [doc = " Parse a meta item per RFC 1559."] # [doc = ""] # [doc = " ```ebnf"] # [doc = " MetaItem = SimplePath ( '=' UNSUFFIXED_LIT | '(' MetaSeq? ')' )? ;"] # [doc = " MetaSeq = MetaItemInner (',' MetaItemInner)* ','? ;"] # [doc = " ```"] pub fn parse_meta_item (& mut self , unsafe_allowed : AllowLeadingUnsafe ,) -> PResult < 'a , ast :: MetaItem > { if let Some (MetaVarKind :: Meta { has_meta_form }) = self . token . is_metavar_seq () { return if has_meta_form { let attr_item = self . eat_metavar_seq (MetaVarKind :: Meta { has_meta_form : true } , | this | { this . parse_attr_item (ForceCollect :: No) }) . unwrap () ; Ok (attr_item . meta (attr_item . path . span) . unwrap ()) } else { self . unexpected_any () } ; } let lo = self . token . span ; let is_unsafe = if unsafe_allowed == AllowLeadingUnsafe :: Yes { self . eat_keyword (exp ! (Unsafe)) } else { false } ; let unsafety = if is_unsafe { let unsafe_span = self . prev_token . span ; self . expect (exp ! (OpenParen)) ? ; ast :: Safety :: Unsafe (unsafe_span) } else { ast :: Safety :: Default } ; let path = self . parse_path (PathStyle :: Mod) ? ; let kind = self . parse_meta_item_kind () ? ; if is_unsafe { self . expect (exp ! (CloseParen)) ? ; } let span = lo . to (self . prev_token . span) ; Ok (ast :: MetaItem { unsafety , path , kind , span }) } pub (crate) fn parse_meta_item_kind (& mut self) -> PResult < 'a , ast :: MetaItemKind > { Ok (if self . eat (exp ! (Eq)) { ast :: MetaItemKind :: NameValue (self . parse_unsuffixed_meta_item_lit () ?) } else if self . check (exp ! (OpenParen)) { let (list , _) = self . parse_paren_comma_seq (| p | p . parse_meta_item_inner ()) ? ; ast :: MetaItemKind :: List (list) } else { ast :: MetaItemKind :: Word }) } # [doc = " Parse an inner meta item per RFC 1559."] # [doc = ""] # [doc = " ```ebnf"] # [doc = " MetaItemInner = UNSUFFIXED_LIT | MetaItem ;"] # [doc = " ```"] pub fn parse_meta_item_inner (& mut self) -> PResult < 'a , ast :: MetaItemInner > { match self . parse_unsuffixed_meta_item_lit () { Ok (lit) => return Ok (ast :: MetaItemInner :: Lit (lit)) , Err (err) => err . cancel () , } match self . parse_meta_item (AllowLeadingUnsafe :: No) { Ok (mi) => return Ok (ast :: MetaItemInner :: MetaItem (mi)) , Err (err) => err . cancel () , } let mut err = errors :: InvalidMetaItem { span : self . token . span , descr : super :: token_descr (& self . token) , quote_ident_sugg : None , } ; if self . prev_token == token :: Eq && let token :: Ident (..) = self . token . kind { let before = self . token . span . shrink_to_lo () ; while let token :: Ident (..) = self . token . kind { self . bump () ; } err . quote_ident_sugg = Some (errors :: InvalidMetaItemQuoteIdentSugg { before , after : self . prev_token . span . shrink_to_hi () , }) ; } Err (self . dcx () . create_err (err)) } }}}