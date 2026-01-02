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
mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_errors :: { Applicability , Diag , ElidedLifetimeInPathSubdiag , EmissionGuarantee , IntoDiagArg , MultiSpan , Subdiagnostic , } ;}
mkuse!{use rustc_macros :: { Diagnostic , Subdiagnostic } ;}
mkuse!{use rustc_span :: { Ident , Span , Symbol } ;}
mkuse!{use crate :: late :: PatternSource ;}
mkuse!{use crate :: { Res , fluent_generated as fluent } ;}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_generic_params_from_outer_item , code = E0401)] pub (crate) struct GenericParamsFromOuterItem { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) label : Option < GenericParamsFromOuterItemLabel > , # [label (resolve_refer_to_type_directly)] pub (crate) refer_to_type_directly : Option < Span > , # [subdiagnostic] pub (crate) sugg : Option < GenericParamsFromOuterItemSugg > , # [subdiagnostic] pub (crate) static_or_const : Option < GenericParamsFromOuterItemStaticOrConst > , pub (crate) is_self : bool , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum GenericParamsFromOuterItemStaticOrConst { # [note (resolve_generic_params_from_outer_item_static)] Static , # [note (resolve_generic_params_from_outer_item_const)] Const , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum GenericParamsFromOuterItemLabel { # [label (resolve_generic_params_from_outer_item_self_ty_param)] SelfTyParam (# [primary_span] Span) , # [label (resolve_generic_params_from_outer_item_self_ty_alias)] SelfTyAlias (# [primary_span] Span) , # [label (resolve_generic_params_from_outer_item_ty_param)] TyParam (# [primary_span] Span) , # [label (resolve_generic_params_from_outer_item_const_param)] ConstParam (# [primary_span] Span) , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [suggestion (resolve_suggestion , code = "{snippet}" , applicability = "maybe-incorrect")] pub (crate) struct GenericParamsFromOuterItemSugg { # [primary_span] pub (crate) span : Span , pub (crate) snippet : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_name_is_already_used_as_generic_parameter , code = E0403)] pub (crate) struct NameAlreadyUsedInParameterList { # [primary_span] # [label] pub (crate) span : Span , # [label (resolve_first_use_of_name)] pub (crate) first_use_span : Span , pub (crate) name : Ident , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_method_not_member_of_trait , code = E0407)] pub (crate) struct MethodNotMemberOfTrait { # [primary_span] # [label] pub (crate) span : Span , pub (crate) method : Ident , pub (crate) trait_ : String , # [subdiagnostic] pub (crate) sub : Option < AssociatedFnWithSimilarNameExists > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [suggestion (resolve_associated_fn_with_similar_name_exists , code = "{candidate}" , applicability = "maybe-incorrect")] pub (crate) struct AssociatedFnWithSimilarNameExists { # [primary_span] pub (crate) span : Span , pub (crate) candidate : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_type_not_member_of_trait , code = E0437)] pub (crate) struct TypeNotMemberOfTrait { # [primary_span] # [label] pub (crate) span : Span , pub (crate) type_ : Ident , pub (crate) trait_ : String , # [subdiagnostic] pub (crate) sub : Option < AssociatedTypeWithSimilarNameExists > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [suggestion (resolve_associated_type_with_similar_name_exists , code = "{candidate}" , applicability = "maybe-incorrect")] pub (crate) struct AssociatedTypeWithSimilarNameExists { # [primary_span] pub (crate) span : Span , pub (crate) candidate : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_const_not_member_of_trait , code = E0438)] pub (crate) struct ConstNotMemberOfTrait { # [primary_span] # [label] pub (crate) span : Span , pub (crate) const_ : Ident , pub (crate) trait_ : String , # [subdiagnostic] pub (crate) sub : Option < AssociatedConstWithSimilarNameExists > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [suggestion (resolve_associated_const_with_similar_name_exists , code = "{candidate}" , applicability = "maybe-incorrect")] pub (crate) struct AssociatedConstWithSimilarNameExists { # [primary_span] pub (crate) span : Span , pub (crate) candidate : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_variable_bound_with_different_mode , code = E0409)] pub (crate) struct VariableBoundWithDifferentMode { # [primary_span] # [label] pub (crate) span : Span , # [label (resolve_first_binding_span)] pub (crate) first_binding_span : Span , pub (crate) variable_name : Ident , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_ident_bound_more_than_once_in_parameter_list , code = E0415)] pub (crate) struct IdentifierBoundMoreThanOnceInParameterList { # [primary_span] # [label] pub (crate) span : Span , pub (crate) identifier : Ident , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_ident_bound_more_than_once_in_same_pattern , code = E0416)] pub (crate) struct IdentifierBoundMoreThanOnceInSamePattern { # [primary_span] # [label] pub (crate) span : Span , pub (crate) identifier : Ident , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_undeclared_label , code = E0426)] pub (crate) struct UndeclaredLabel { # [primary_span] # [label] pub (crate) span : Span , pub (crate) name : Symbol , # [subdiagnostic] pub (crate) sub_reachable : Option < LabelWithSimilarNameReachable > , # [subdiagnostic] pub (crate) sub_reachable_suggestion : Option < TryUsingSimilarlyNamedLabel > , # [subdiagnostic] pub (crate) sub_unreachable : Option < UnreachableLabelWithSimilarNameExists > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [label (resolve_label_with_similar_name_reachable)] pub (crate) struct LabelWithSimilarNameReachable (# [primary_span] pub (crate) Span) ;}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [suggestion (resolve_try_using_similarly_named_label , code = "{ident_name}" , applicability = "maybe-incorrect")] pub (crate) struct TryUsingSimilarlyNamedLabel { # [primary_span] pub (crate) span : Span , pub (crate) ident_name : Symbol , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [label (resolve_unreachable_label_with_similar_name_exists)] pub (crate) struct UnreachableLabelWithSimilarNameExists { # [primary_span] pub (crate) ident_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_self_import_can_only_appear_once_in_the_list , code = E0430)] pub (crate) struct SelfImportCanOnlyAppearOnceInTheList { # [primary_span] # [label] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_self_import_only_in_import_list_with_non_empty_prefix , code = E0431)] pub (crate) struct SelfImportOnlyInImportListWithNonEmptyPrefix { # [primary_span] # [label] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_cannot_capture_dynamic_environment_in_fn_item , code = E0434)] # [help] pub (crate) struct CannotCaptureDynamicEnvironmentInFnItem { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_attempt_to_use_non_constant_value_in_constant , code = E0435)] pub (crate) struct AttemptToUseNonConstantValueInConstant < 'a > { # [primary_span] pub (crate) span : Span , # [subdiagnostic] pub (crate) with : Option < AttemptToUseNonConstantValueInConstantWithSuggestion < 'a > > , # [subdiagnostic] pub (crate) with_label : Option < AttemptToUseNonConstantValueInConstantLabelWithSuggestion > , # [subdiagnostic] pub (crate) without : Option < AttemptToUseNonConstantValueInConstantWithoutSuggestion < 'a > > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [multipart_suggestion (resolve_attempt_to_use_non_constant_value_in_constant_with_suggestion , style = "verbose" , applicability = "has-placeholders")] pub (crate) struct AttemptToUseNonConstantValueInConstantWithSuggestion < 'a > { # [suggestion_part (code = "{suggestion} ")] pub (crate) span : Span , pub (crate) suggestion : & 'a str , # [suggestion_part (code = ": /* Type */")] pub (crate) type_span : Option < Span > , pub (crate) current : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [label (resolve_attempt_to_use_non_constant_value_in_constant_label_with_suggestion)] pub (crate) struct AttemptToUseNonConstantValueInConstantLabelWithSuggestion { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [label (resolve_attempt_to_use_non_constant_value_in_constant_without_suggestion)] pub (crate) struct AttemptToUseNonConstantValueInConstantWithoutSuggestion < 'a > { # [primary_span] pub (crate) ident_span : Span , pub (crate) suggestion : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_self_imports_only_allowed_within , code = E0429)] pub (crate) struct SelfImportsOnlyAllowedWithin { # [primary_span] pub (crate) span : Span , # [subdiagnostic] pub (crate) suggestion : Option < SelfImportsOnlyAllowedWithinSuggestion > , # [subdiagnostic] pub (crate) mpart_suggestion : Option < SelfImportsOnlyAllowedWithinMultipartSuggestion > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [suggestion (resolve_self_imports_only_allowed_within_suggestion , code = "" , applicability = "machine-applicable")] pub (crate) struct SelfImportsOnlyAllowedWithinSuggestion { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [multipart_suggestion (resolve_self_imports_only_allowed_within_multipart_suggestion , applicability = "machine-applicable")] pub (crate) struct SelfImportsOnlyAllowedWithinMultipartSuggestion { # [suggestion_part (code = "{{")] pub (crate) multipart_start : Span , # [suggestion_part (code = "}}")] pub (crate) multipart_end : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_binding_shadows_something_unacceptable , code = E0530)] pub (crate) struct BindingShadowsSomethingUnacceptable < 'a > { # [primary_span] # [label] pub (crate) span : Span , pub (crate) shadowing_binding : PatternSource , pub (crate) shadowed_binding : Res , pub (crate) article : & 'a str , # [subdiagnostic] pub (crate) sub_suggestion : Option < BindingShadowsSomethingUnacceptableSuggestion > , # [label (resolve_label_shadowed_binding)] pub (crate) shadowed_binding_span : Span , pub (crate) participle : & 'a str , pub (crate) name : Symbol , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [suggestion (resolve_binding_shadows_something_unacceptable_suggestion , code = "{name}(..)" , applicability = "unspecified")] pub (crate) struct BindingShadowsSomethingUnacceptableSuggestion { # [primary_span] pub (crate) span : Span , pub (crate) name : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_forward_declared_generic_param , code = E0128)] pub (crate) struct ForwardDeclaredGenericParam { # [primary_span] # [label] pub (crate) span : Span , pub (crate) param : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_forward_declared_generic_in_const_param_ty)] pub (crate) struct ForwardDeclaredGenericInConstParamTy { # [primary_span] # [label] pub (crate) span : Span , pub (crate) param : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_param_in_ty_of_const_param , code = E0770)] pub (crate) struct ParamInTyOfConstParam { # [primary_span] # [label] pub (crate) span : Span , pub (crate) name : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_self_in_generic_param_default , code = E0735)] pub (crate) struct SelfInGenericParamDefault { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_self_in_const_generic_ty)] pub (crate) struct SelfInConstGenericTy { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_param_in_non_trivial_anon_const)] pub (crate) struct ParamInNonTrivialAnonConst { # [primary_span] # [label] pub (crate) span : Span , pub (crate) name : Symbol , # [subdiagnostic] pub (crate) param_kind : ParamKindInNonTrivialAnonConst , # [subdiagnostic] pub (crate) help : Option < ParamInNonTrivialAnonConstHelp > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [help (resolve_param_in_non_trivial_anon_const_help)] pub (crate) struct ParamInNonTrivialAnonConstHelp ;}}
mkitem!{mkenum!{# [derive (Debug)] # [derive (Subdiagnostic)] pub (crate) enum ParamKindInNonTrivialAnonConst { # [note (resolve_type_param_in_non_trivial_anon_const)] Type , # [help (resolve_const_param_in_non_trivial_anon_const)] Const { name : Symbol } , # [note (resolve_lifetime_param_in_non_trivial_anon_const)] Lifetime , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_unreachable_label , code = E0767)] # [note] pub (crate) struct UnreachableLabel { # [primary_span] # [label] pub (crate) span : Span , pub (crate) name : Symbol , # [label (resolve_label_definition_span)] pub (crate) definition_span : Span , # [subdiagnostic] pub (crate) sub_suggestion : Option < UnreachableLabelSubSuggestion > , # [subdiagnostic] pub (crate) sub_suggestion_label : Option < UnreachableLabelSubLabel > , # [subdiagnostic] pub (crate) sub_unreachable_label : Option < UnreachableLabelSubLabelUnreachable > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [suggestion (resolve_unreachable_label_suggestion_use_similarly_named , code = "{ident_name}" , applicability = "maybe-incorrect")] pub (crate) struct UnreachableLabelSubSuggestion { # [primary_span] pub (crate) span : Span , pub (crate) ident_name : Symbol , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [label (resolve_unreachable_label_similar_name_reachable)] pub (crate) struct UnreachableLabelSubLabel { # [primary_span] pub (crate) ident_span : Span , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [label (resolve_unreachable_label_similar_name_unreachable)] pub (crate) struct UnreachableLabelSubLabelUnreachable { # [primary_span] pub (crate) ident_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_invalid_asm_sym)] # [help] pub (crate) struct InvalidAsmSym { # [primary_span] # [label] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_lowercase_self)] pub (crate) struct LowercaseSelf { # [primary_span] # [suggestion (code = "Self" , applicability = "maybe-incorrect" , style = "short")] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Debug)] # [derive (Diagnostic)] # [diag (resolve_binding_in_never_pattern)] pub (crate) struct BindingInNeverPattern { # [primary_span] # [suggestion (code = "_" , applicability = "machine-applicable" , style = "short")] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_trait_impl_duplicate , code = E0201)] pub (crate) struct TraitImplDuplicate { # [primary_span] # [label] pub (crate) span : Span , # [label (resolve_old_span_label)] pub (crate) old_span : Span , # [label (resolve_trait_item_span)] pub (crate) trait_item_span : Span , pub (crate) name : Ident , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_relative_2018)] pub (crate) struct Relative2018 { # [primary_span] pub (crate) span : Span , # [suggestion (code = "crate::{path_str}" , applicability = "maybe-incorrect")] pub (crate) path_span : Span , pub (crate) path_str : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_ancestor_only , code = E0742)] pub (crate) struct AncestorOnly (# [primary_span] pub (crate) Span) ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_expected_module_found , code = E0577)] pub (crate) struct ExpectedModuleFound { # [primary_span] # [label] pub (crate) span : Span , pub (crate) res : Res , pub (crate) path_str : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_indeterminate , code = E0578)] pub (crate) struct Indeterminate (# [primary_span] pub (crate) Span) ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_tool_module_imported)] pub (crate) struct ToolModuleImported { # [primary_span] pub (crate) span : Span , # [note] pub (crate) import : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_module_only)] pub (crate) struct ModuleOnly (# [primary_span] pub (crate) Span) ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_macro_expected_found)] pub (crate) struct MacroExpectedFound < 'a > { # [primary_span] # [label] pub (crate) span : Span , pub (crate) found : & 'a str , pub (crate) article : & 'static str , pub (crate) expected : & 'a str , pub (crate) macro_path : & 'a str , # [subdiagnostic] pub (crate) remove_surrounding_derive : Option < RemoveSurroundingDerive > , # [subdiagnostic] pub (crate) add_as_non_derive : Option < AddAsNonDerive < 'a > > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [help (resolve_remove_surrounding_derive)] pub (crate) struct RemoveSurroundingDerive { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [help (resolve_add_as_non_derive)] pub (crate) struct AddAsNonDerive < 'a > { pub (crate) macro_path : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_proc_macro_same_crate)] pub (crate) struct ProcMacroSameCrate { # [primary_span] pub (crate) span : Span , # [help] pub (crate) is_test : bool , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_imported_crate)] pub (crate) struct CrateImported { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_macro_use_extern_crate_self)] pub (crate) struct MacroUseExternCrateSelf { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_accessible_unsure)] # [note] pub (crate) struct CfgAccessibleUnsure { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Debug)] # [derive (Diagnostic)] # [diag (resolve_param_in_enum_discriminant)] pub (crate) struct ParamInEnumDiscriminant { # [primary_span] # [label] pub (crate) span : Span , pub (crate) name : Symbol , # [subdiagnostic] pub (crate) param_kind : ParamKindInEnumDiscriminant , }}}
mkitem!{mkenum!{# [derive (Debug)] # [derive (Subdiagnostic)] pub (crate) enum ParamKindInEnumDiscriminant { # [note (resolve_type_param_in_enum_discriminant)] Type , # [note (resolve_const_param_in_enum_discriminant)] Const , # [note (resolve_lifetime_param_in_enum_discriminant)] Lifetime , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [label (resolve_change_import_binding)] pub (crate) struct ChangeImportBinding { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [suggestion (resolve_change_import_binding , code = "{suggestion}" , applicability = "maybe-incorrect")] pub (crate) struct ChangeImportBindingSuggestion { # [primary_span] pub (crate) span : Span , pub (crate) suggestion : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_imports_cannot_refer_to)] pub (crate) struct ImportsCannotReferTo < 'a > { # [primary_span] pub (crate) span : Span , pub (crate) what : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_cannot_find_ident_in_this_scope)] pub (crate) struct CannotFindIdentInThisScope < 'a > { # [primary_span] pub (crate) span : Span , pub (crate) expected : & 'a str , pub (crate) ident : Ident , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (resolve_explicit_unsafe_traits)] pub (crate) struct ExplicitUnsafeTraits { # [primary_span] pub (crate) span : Span , pub (crate) ident : Ident , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (resolve_macro_defined_later)] pub (crate) struct MacroDefinedLater { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [label (resolve_consider_move_macro_position)] pub (crate) struct MacroSuggMovePosition { # [primary_span] pub (crate) span : Span , pub (crate) ident : Ident , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum MacroRulesNot { # [label (resolve_macro_cannot_use_as_fn_like)] Func { # [primary_span] span : Span , ident : Ident , } , # [label (resolve_macro_cannot_use_as_attr)] Attr { # [primary_span] span : Span , ident : Ident , } , # [label (resolve_macro_cannot_use_as_derive)] Derive { # [primary_span] span : Span , ident : Ident , } , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (resolve_missing_macro_rules_name)] pub (crate) struct MaybeMissingMacroRulesName { # [primary_span] pub (crate) spans : MultiSpan , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [help (resolve_added_macro_use)] pub (crate) struct AddedMacroUse ;}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [suggestion (resolve_consider_adding_a_derive , code = "{suggestion}" , applicability = "maybe-incorrect")] pub (crate) struct ConsiderAddingADerive { # [primary_span] pub (crate) span : Span , pub (crate) suggestion : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_cannot_determine_import_resolution)] pub (crate) struct CannotDetermineImportResolution { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_cannot_determine_macro_resolution)] # [note] pub (crate) struct CannotDetermineMacroResolution { # [primary_span] pub (crate) span : Span , pub (crate) kind : & 'static str , pub (crate) path : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_cannot_be_reexported_private , code = E0364)] pub (crate) struct CannotBeReexportedPrivate { # [primary_span] pub (crate) span : Span , pub (crate) ident : Ident , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_cannot_be_reexported_crate_public , code = E0364)] pub (crate) struct CannotBeReexportedCratePublic { # [primary_span] pub (crate) span : Span , pub (crate) ident : Ident , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_cannot_be_reexported_private , code = E0365)] # [note (resolve_consider_declaring_with_pub)] pub (crate) struct CannotBeReexportedPrivateNS { # [primary_span] # [label (resolve_reexport_of_private)] pub (crate) span : Span , pub (crate) ident : Ident , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_cannot_be_reexported_crate_public , code = E0365)] # [note (resolve_consider_declaring_with_pub)] pub (crate) struct CannotBeReexportedCratePublicNS { # [primary_span] # [label (resolve_reexport_of_crate_public)] pub (crate) span : Span , pub (crate) ident : Ident , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [help (resolve_consider_adding_macro_export)] pub (crate) struct ConsiderAddingMacroExport { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [suggestion (resolve_consider_marking_as_pub_crate , code = "pub(crate)" , applicability = "maybe-incorrect")] pub (crate) struct ConsiderMarkingAsPubCrate { # [primary_span] pub (crate) vis_span : Span , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (resolve_consider_marking_as_pub)] pub (crate) struct ConsiderMarkingAsPub { # [primary_span] pub (crate) span : Span , pub (crate) ident : Ident , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_cannot_glob_import_possible_crates)] pub (crate) struct CannotGlobImportAllCrates { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [suggestion (resolve_unexpected_res_change_ty_to_const_param_sugg , code = "const " , style = "verbose")] pub (crate) struct UnexpectedResChangeTyToConstParamSugg { # [primary_span] pub span : Span , # [applicability] pub applicability : Applicability , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [suggestion (resolve_unexpected_res_use_at_op_in_slice_pat_with_range_sugg , code = "{snippet}" , applicability = "maybe-incorrect" , style = "verbose")] pub (crate) struct UnexpectedResUseAtOpInSlicePatWithRangeSugg { # [primary_span] pub span : Span , pub ident : Ident , pub snippet : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_extern_crate_loading_macro_not_at_crate_root , code = E0468)] pub (crate) struct ExternCrateLoadingMacroNotAtCrateRoot { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_extern_crate_self_requires_renaming)] pub (crate) struct ExternCrateSelfRequiresRenaming { # [primary_span] # [suggestion (code = "extern crate self as name;" , applicability = "has-placeholders")] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_macro_use_name_already_in_use)] # [note] pub (crate) struct MacroUseNameAlreadyInUse { # [primary_span] pub (crate) span : Span , pub (crate) name : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_imported_macro_not_found , code = E0469)] pub (crate) struct ImportedMacroNotFound { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_macro_extern_deprecated)] pub (crate) struct MacroExternDeprecated { # [primary_span] pub (crate) span : Span , # [help] pub inner_attribute : bool , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_arguments_macro_use_not_allowed)] pub (crate) struct ArgumentsMacroUseNotAllowed { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_unnamed_crate_root_import)] pub (crate) struct UnnamedCrateRootImport { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_macro_expanded_extern_crate_cannot_shadow_extern_arguments)] pub (crate) struct MacroExpandedExternCrateCannotShadowExternArguments { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_elided_anonymous_lifetime_report_error , code = E0637)] pub (crate) struct ElidedAnonymousLifetimeReportError { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) suggestion : Option < ElidedAnonymousLifetimeReportErrorSuggestion > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_lending_iterator_report_error)] pub (crate) struct LendingIteratorReportError { # [primary_span] pub (crate) lifetime : Span , # [note] pub (crate) ty : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_anonymous_lifetime_non_gat_report_error)] pub (crate) struct AnonymousLifetimeNonGatReportError { # [primary_span] # [label] pub (crate) lifetime : Span , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [multipart_suggestion (resolve_elided_anonymous_lifetime_report_error_suggestion , applicability = "machine-applicable")] pub (crate) struct ElidedAnonymousLifetimeReportErrorSuggestion { # [suggestion_part (code = "for<'a> ")] pub (crate) lo : Span , # [suggestion_part (code = "'a ")] pub (crate) hi : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_explicit_anonymous_lifetime_report_error , code = E0637)] pub (crate) struct ExplicitAnonymousLifetimeReportError { # [primary_span] # [label] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_implicit_elided_lifetimes_not_allowed_here , code = E0726)] pub (crate) struct ImplicitElidedLifetimeNotAllowedHere { # [primary_span] pub (crate) span : Span , # [subdiagnostic] pub (crate) subdiag : ElidedLifetimeInPathSubdiag , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_underscore_lifetime_is_reserved , code = E0637)] # [help] pub (crate) struct UnderscoreLifetimeIsReserved { # [primary_span] # [label] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_static_lifetime_is_reserved , code = E0262)] pub (crate) struct StaticLifetimeIsReserved { # [primary_span] # [label] pub (crate) span : Span , pub (crate) lifetime : Ident , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_variable_is_not_bound_in_all_patterns , code = E0408)] pub (crate) struct VariableIsNotBoundInAllPatterns { # [primary_span] pub (crate) multispan : MultiSpan , pub (crate) name : Ident , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic , Debug , Clone)] # [label (resolve_pattern_doesnt_bind_name)] pub (crate) struct PatternDoesntBindName { # [primary_span] pub (crate) span : Span , pub (crate) name : Ident , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic , Debug , Clone)] # [label (resolve_variable_not_in_all_patterns)] pub (crate) struct VariableNotInAllPatterns { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [multipart_suggestion (resolve_variable_is_a_typo , applicability = "maybe-incorrect" , style = "verbose")] pub (crate) struct PatternBindingTypo { # [suggestion_part (code = "{typo}")] pub (crate) spans : Vec < Span > , pub (crate) typo : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_name_defined_multiple_time)] # [note] pub (crate) struct NameDefinedMultipleTime { # [primary_span] pub (crate) span : Span , pub (crate) name : Symbol , pub (crate) descr : & 'static str , pub (crate) container : & 'static str , # [subdiagnostic] pub (crate) label : NameDefinedMultipleTimeLabel , # [subdiagnostic] pub (crate) old_binding_label : Option < NameDefinedMultipleTimeOldBindingLabel > , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum NameDefinedMultipleTimeLabel { # [label (resolve_name_defined_multiple_time_reimported)] Reimported { # [primary_span] span : Span , } , # [label (resolve_name_defined_multiple_time_redefined)] Redefined { # [primary_span] span : Span , } , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum NameDefinedMultipleTimeOldBindingLabel { # [label (resolve_name_defined_multiple_time_old_binding_import)] Import { # [primary_span] span : Span , old_kind : & 'static str , } , # [label (resolve_name_defined_multiple_time_old_binding_definition)] Definition { # [primary_span] span : Span , old_kind : & 'static str , } , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_is_private , code = E0603)] pub (crate) struct IsPrivate < 'a > { # [primary_span] # [label] pub (crate) span : Span , pub (crate) ident_descr : & 'a str , pub (crate) ident : Ident , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_generic_arguments_in_macro_path)] pub (crate) struct GenericArgumentsInMacroPath { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_attributes_starting_with_rustc_are_reserved)] pub (crate) struct AttributesStartingWithRustcAreReserved { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_cannot_use_through_an_import)] pub (crate) struct CannotUseThroughAnImport { # [primary_span] pub (crate) span : Span , pub (crate) article : & 'static str , pub (crate) descr : & 'static str , # [note] pub (crate) binding_span : Option < Span > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_name_reserved_in_attribute_namespace)] pub (crate) struct NameReservedInAttributeNamespace { # [primary_span] pub (crate) span : Span , pub (crate) ident : Ident , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_cannot_find_builtin_macro_with_name)] pub (crate) struct CannotFindBuiltinMacroWithName { # [primary_span] pub (crate) span : Span , pub (crate) ident : Ident , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_tool_was_already_registered)] pub (crate) struct ToolWasAlreadyRegistered { # [primary_span] pub (crate) span : Span , pub (crate) tool : Ident , # [label] pub (crate) old_ident_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_tool_only_accepts_identifiers)] pub (crate) struct ToolOnlyAcceptsIdentifiers { # [primary_span] # [label] pub (crate) span : Span , pub (crate) tool : Symbol , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum DefinedHere { # [label (resolve_similarly_named_defined_here)] SimilarlyNamed { # [primary_span] span : Span , candidate_descr : & 'static str , candidate : Symbol , } , # [label (resolve_single_item_defined_here)] SingleItem { # [primary_span] span : Span , candidate_descr : & 'static str , candidate : Symbol , } , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [label (resolve_outer_ident_is_not_publicly_reexported)] pub (crate) struct OuterIdentIsNotPubliclyReexported { # [primary_span] pub (crate) span : Span , pub (crate) outer_ident_descr : & 'static str , pub (crate) outer_ident : Ident , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [label (resolve_constructor_private_if_any_field_private)] pub (crate) struct ConstructorPrivateIfAnyFieldPrivate { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [multipart_suggestion (resolve_consider_making_the_field_public , applicability = "maybe-incorrect" , style = "verbose")] pub (crate) struct ConsiderMakingTheFieldPublic { # [suggestion_part (code = "pub ")] pub (crate) spans : Vec < Span > , pub (crate) number_of_fields : usize , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum ImportIdent { # [suggestion (resolve_suggestion_import_ident_through_reexport , code = "{path}" , applicability = "machine-applicable" , style = "verbose")] ThroughReExport { # [primary_span] span : Span , ident : Ident , path : String , } , # [suggestion (resolve_suggestion_import_ident_directly , code = "{path}" , applicability = "machine-applicable" , style = "verbose")] Directly { # [primary_span] span : Span , ident : Ident , path : String , } , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (resolve_note_and_refers_to_the_item_defined_here)] pub (crate) struct NoteAndRefersToTheItemDefinedHere < 'a > { # [primary_span] pub (crate) span : MultiSpan , pub (crate) binding_descr : & 'a str , pub (crate) binding_name : Ident , pub (crate) first : bool , pub (crate) dots : bool , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [suggestion (resolve_remove_unnecessary_import , code = "" , applicability = "maybe-incorrect")] pub (crate) struct RemoveUnnecessaryImport { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [suggestion (resolve_remove_unnecessary_import , code = "" , applicability = "maybe-incorrect" , style = "tool-only")] pub (crate) struct ToolOnlyRemoveUnnecessaryImport { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (resolve_ident_imported_here_but_it_is_desc)] pub (crate) struct IdentImporterHereButItIsDesc < 'a > { # [primary_span] pub (crate) span : Span , pub (crate) imported_ident : Ident , pub (crate) imported_ident_desc : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (resolve_ident_in_scope_but_it_is_desc)] pub (crate) struct IdentInScopeButItIsDesc < 'a > { pub (crate) imported_ident : Ident , pub (crate) imported_ident_desc : & 'a str , }}}
mkitem!{mkstruct!{pub (crate) struct FoundItemConfigureOut { pub (crate) span : Span , pub (crate) item_was : ItemWas , }}}
mkitem!{mkenum!{pub (crate) enum ItemWas { BehindFeature { feature : Symbol , span : Span } , CfgOut { span : Span } , }}}
mkitem!{mkimpl!{impl Subdiagnostic for FoundItemConfigureOut { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { let mut multispan : MultiSpan = self . span . into () ; match self . item_was { ItemWas :: BehindFeature { feature , span } => { let key = "feature" . into () ; let value = feature . into_diag_arg (& mut None) ; let msg = diag . dcx . eagerly_translate_to_string (fluent :: resolve_item_was_behind_feature , [(& key , & value)] . into_iter () ,) ; multispan . push_span_label (span , msg) ; } ItemWas :: CfgOut { span } => { multispan . push_span_label (span , fluent :: resolve_item_was_cfg_out) ; } } diag . span_note (multispan , fluent :: resolve_found_an_item_configured_out) ; } }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (resolve_trait_impl_mismatch)] pub (crate) struct TraitImplMismatch { # [primary_span] # [label] pub (crate) span : Span , pub (crate) name : Ident , pub (crate) kind : & 'static str , pub (crate) trait_path : String , # [label (resolve_trait_impl_mismatch_label_item)] pub (crate) trait_item_span : Span , }}}