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
mkuse!{use core :: ops :: ControlFlow ;}
mkuse!{use rustc_ast as ast ;}
mkuse!{use rustc_ast :: visit :: visit_opt ;}
mkuse!{use rustc_ast :: { EnumDef , VariantData , attr } ;}
mkuse!{use rustc_expand :: base :: { Annotatable , DummyResult , ExtCtxt } ;}
mkuse!{use rustc_span :: { ErrorGuaranteed , Ident , Span , kw , sym } ;}
mkuse!{use smallvec :: SmallVec ;}
mkuse!{use thin_vec :: { ThinVec , thin_vec } ;}
mkuse!{use crate :: deriving :: generic :: ty :: * ;}
mkuse!{use crate :: deriving :: generic :: * ;}
mkuse!{use crate :: errors ;}

macro_rules! expand_deriving_default_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_deriving_default in module {}", module_path!());
    };
}

mkfn!{
    expand_deriving_default_introspect!();
    pub (crate) fn expand_deriving_default (cx : & ExtCtxt < '_ > , span : Span , mitem : & ast :: MetaItem , item : & Annotatable , push : & mut dyn FnMut (Annotatable) , is_const : bool ,) { item . visit_with (& mut DetectNonVariantDefaultAttr { cx }) ; let trait_def = TraitDef { span , path : Path :: new (vec ! [kw :: Default , sym :: Default]) , skip_path_as_bound : has_a_default_variant (item) , needs_copy_as_bound_if_packed : false , additional_bounds : Vec :: new () , supports_unions : false , methods : vec ! [MethodDef { name : kw :: Default , generics : Bounds :: empty () , explicit_self : false , nonself_args : Vec :: new () , ret_ty : Self_ , attributes : thin_vec ! [cx . attr_word (sym :: inline , span)] , fieldless_variants_strategy : FieldlessVariantsStrategy :: Default , combine_substructure : combine_substructure (Box :: new (| cx , trait_span , substr | { match substr . fields { StaticStruct (_ , fields) => { default_struct_substructure (cx , trait_span , substr , fields) } StaticEnum (enum_def) => { default_enum_substructure (cx , trait_span , enum_def , item . span ()) } _ => cx . dcx () . span_bug (trait_span , "method in `derive(Default)`") , } })) , }] , associated_types : Vec :: new () , is_const , is_staged_api_crate : cx . ecfg . features . staged_api () , } ; trait_def . expand (cx , mitem , item , push) }
}

macro_rules! default_call_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function default_call in module {}", module_path!());
    };
}

mkfn!{
    default_call_introspect!();
    fn default_call (cx : & ExtCtxt < '_ > , span : Span) -> Box < ast :: Expr > { let default_ident = cx . std_path (& [kw :: Default , sym :: Default , kw :: Default]) ; cx . expr_call_global (span , default_ident , ThinVec :: new ()) }
}

macro_rules! default_struct_substructure_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function default_struct_substructure in module {}", module_path!());
    };
}

mkfn!{
    default_struct_substructure_introspect!();
    fn default_struct_substructure (cx : & ExtCtxt < '_ > , trait_span : Span , substr : & Substructure < '_ > , summary : & StaticFields ,) -> BlockOrExpr { let expr = match summary { Unnamed (_ , IsTuple :: No) => cx . expr_ident (trait_span , substr . type_ident) , Unnamed (fields , IsTuple :: Yes) => { let exprs = fields . iter () . map (| sp | default_call (cx , * sp)) . collect () ; cx . expr_call_ident (trait_span , substr . type_ident , exprs) } Named (fields) => { let default_fields = fields . iter () . map (| (ident , span , default_val) | { let value = match default_val { None => default_call (cx , * span) , Some (val) => { cx . expr (val . value . span , ast :: ExprKind :: ConstBlock (val . clone ())) } } ; cx . field_imm (* span , * ident , value) }) . collect () ; cx . expr_struct_ident (trait_span , substr . type_ident , default_fields) } } ; BlockOrExpr :: new_expr (expr) }
}

macro_rules! default_enum_substructure_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function default_enum_substructure in module {}", module_path!());
    };
}

mkfn!{
    default_enum_substructure_introspect!();
    fn default_enum_substructure (cx : & ExtCtxt < '_ > , trait_span : Span , enum_def : & EnumDef , item_span : Span ,) -> BlockOrExpr { let expr = match try { let default_variant = extract_default_variant (cx , enum_def , trait_span , item_span) ? ; validate_default_attribute (cx , default_variant) ? ; default_variant } { Ok (default_variant) => { match & default_variant . data { VariantData :: Unit (_) => cx . expr_path (cx . path (default_variant . span , vec ! [Ident :: new (kw :: SelfUpper , default_variant . span) , default_variant . ident] ,)) , VariantData :: Struct { fields , .. } => { let default_fields = fields . iter () . map (| field | { cx . field_imm (field . span , field . ident . unwrap () , match & field . default { None => default_call (cx , field . span) , Some (val) => cx . expr (val . value . span , ast :: ExprKind :: ConstBlock (val . clone ()) ,) , } ,) }) . collect () ; let path = cx . path (default_variant . span , vec ! [Ident :: new (kw :: SelfUpper , default_variant . span) , default_variant . ident ,] ,) ; cx . expr_struct (default_variant . span , path , default_fields) } VariantData :: Tuple (..) => { cx . dcx () . bug ("encountered tuple variant annotated with `#[default]`") } } } Err (guar) => DummyResult :: raw_expr (trait_span , Some (guar)) , } ; BlockOrExpr :: new_expr (expr) }
}

macro_rules! extract_default_variant_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function extract_default_variant in module {}", module_path!());
    };
}

mkfn!{
    extract_default_variant_introspect!();
    fn extract_default_variant < 'a > (cx : & ExtCtxt < '_ > , enum_def : & 'a EnumDef , trait_span : Span , item_span : Span ,) -> Result < & 'a rustc_ast :: Variant , ErrorGuaranteed > { let default_variants : SmallVec < [_ ; 1] > = enum_def . variants . iter () . filter (| variant | attr :: contains_name (& variant . attrs , kw :: Default)) . collect () ; let variant = match default_variants . as_slice () { [variant] => variant , [] => { let possible_defaults = enum_def . variants . iter () . filter (| variant | matches ! (variant . data , VariantData :: Unit (..))) . filter (| variant | ! attr :: contains_name (& variant . attrs , sym :: non_exhaustive)) ; let suggs = possible_defaults . map (| v | errors :: NoDefaultVariantSugg { span : v . span . shrink_to_lo () }) . collect () ; let guar = cx . dcx () . emit_err (errors :: NoDefaultVariant { span : trait_span , item_span , suggs }) ; return Err (guar) ; } [first , rest @ ..] => { let suggs = default_variants . iter () . filter_map (| variant | { let keep = attr :: find_by_name (& variant . attrs , kw :: Default) ? . span ; let spans : Vec < Span > = default_variants . iter () . flat_map (| v | { attr :: filter_by_name (& v . attrs , kw :: Default) . filter_map (| attr | (attr . span != keep) . then_some (attr . span)) }) . collect () ; (! spans . is_empty ()) . then_some (errors :: MultipleDefaultsSugg { spans , ident : variant . ident }) }) . collect () ; let guar = cx . dcx () . emit_err (errors :: MultipleDefaults { span : trait_span , first : first . span , additional : rest . iter () . map (| v | v . span) . collect () , suggs , }) ; return Err (guar) ; } } ; if cx . ecfg . features . default_field_values () && let VariantData :: Struct { fields , .. } = & variant . data && fields . iter () . all (| f | f . default . is_some ()) && ! fields . is_empty () { } else if ! matches ! (variant . data , VariantData :: Unit (..)) { let post = if cx . ecfg . features . default_field_values () { " or variants where every field has a default value" } else { "" } ; let guar = cx . dcx () . emit_err (errors :: NonUnitDefault { span : variant . ident . span , post }) ; return Err (guar) ; } if let Some (non_exhaustive_attr) = attr :: find_by_name (& variant . attrs , sym :: non_exhaustive) { let guar = cx . dcx () . emit_err (errors :: NonExhaustiveDefault { span : variant . ident . span , non_exhaustive : non_exhaustive_attr . span , }) ; return Err (guar) ; } Ok (variant) }
}

macro_rules! validate_default_attribute_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function validate_default_attribute in module {}", module_path!());
    };
}

mkfn!{
    validate_default_attribute_introspect!();
    fn validate_default_attribute (cx : & ExtCtxt < '_ > , default_variant : & rustc_ast :: Variant ,) -> Result < () , ErrorGuaranteed > { let attrs : SmallVec < [_ ; 1] > = attr :: filter_by_name (& default_variant . attrs , kw :: Default) . collect () ; let attr = match attrs . as_slice () { [attr] => attr , [] => cx . dcx () . bug ("this method must only be called with a variant that has a `#[default]` attribute" ,) , [first , rest @ ..] => { let sugg = errors :: MultipleDefaultAttrsSugg { spans : rest . iter () . map (| attr | attr . span) . collect () , } ; let guar = cx . dcx () . emit_err (errors :: MultipleDefaultAttrs { span : default_variant . ident . span , first : first . span , first_rest : rest [0] . span , rest : rest . iter () . map (| attr | attr . span) . collect :: < Vec < _ > > () . into () , only_one : rest . len () == 1 , sugg , }) ; return Err (guar) ; } } ; if ! attr . is_word () { let guar = cx . dcx () . emit_err (errors :: DefaultHasArg { span : attr . span }) ; return Err (guar) ; } Ok (()) }
}
mkitem!{mkstruct!{struct DetectNonVariantDefaultAttr < 'a , 'b > { cx : & 'a ExtCtxt < 'b > , }}}
mkitem!{mkimpl!{impl < 'a , 'b > rustc_ast :: visit :: Visitor < 'a > for DetectNonVariantDefaultAttr < 'a , 'b > { fn visit_attribute (& mut self , attr : & 'a rustc_ast :: Attribute) { if attr . has_name (kw :: Default) { let post = if self . cx . ecfg . features . default_field_values () { " or variants where every field has a default value" } else { "" } ; self . cx . dcx () . emit_err (errors :: NonUnitDefault { span : attr . span , post }) ; } rustc_ast :: visit :: walk_attribute (self , attr) ; } fn visit_variant (& mut self , v : & 'a rustc_ast :: Variant) { self . visit_ident (& v . ident) ; self . visit_vis (& v . vis) ; self . visit_variant_data (& v . data) ; visit_opt ! (self , visit_anon_const , & v . disr_expr) ; for attr in & v . attrs { rustc_ast :: visit :: walk_attribute (self , attr) ; } } }}}

macro_rules! has_a_default_variant_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function has_a_default_variant in module {}", module_path!());
    };
}

mkfn!{
    has_a_default_variant_introspect!();
    fn has_a_default_variant (item : & Annotatable) -> bool { struct HasDefaultAttrOnVariant ; impl < 'ast > rustc_ast :: visit :: Visitor < 'ast > for HasDefaultAttrOnVariant { type Result = ControlFlow < () > ; fn visit_variant (& mut self , v : & 'ast rustc_ast :: Variant) -> ControlFlow < () > { if v . attrs . iter () . any (| attr | attr . has_name (kw :: Default)) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } } } item . visit_with (& mut HasDefaultAttrOnVariant) . is_break () }
}