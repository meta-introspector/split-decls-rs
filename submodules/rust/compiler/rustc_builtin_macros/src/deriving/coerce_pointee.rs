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
mkuse!{use ast :: HasAttrs ;}
mkuse!{use rustc_ast :: mut_visit :: MutVisitor ;}
mkuse!{use rustc_ast :: visit :: BoundKind ;}
mkuse!{use rustc_ast :: { self as ast , GenericArg , GenericBound , GenericParamKind , Generics , ItemKind , MetaItem , TraitBoundModifiers , VariantData , WherePredicate , } ;}
mkuse!{use rustc_data_structures :: flat_map_in_place :: FlatMapInPlace ;}
mkuse!{use rustc_errors :: E0802 ;}
mkuse!{use rustc_expand :: base :: { Annotatable , ExtCtxt } ;}
mkuse!{use rustc_macros :: Diagnostic ;}
mkuse!{use rustc_span :: { Ident , Span , Symbol , sym } ;}
mkuse!{use thin_vec :: { ThinVec , thin_vec } ;}
mkuse!{use crate :: errors ;}
mkitem!{macro_rules ! path { ($ span : expr , $ ($ part : ident) ::*) => { vec ! [$ (Ident :: new (sym ::$ part , $ span) ,) *] } }}

macro_rules! expand_deriving_coerce_pointee_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_deriving_coerce_pointee in module {}", module_path!());
    };
}

mkfn!{
    expand_deriving_coerce_pointee_introspect!();
    pub (crate) fn expand_deriving_coerce_pointee (cx : & ExtCtxt < '_ > , span : Span , _mitem : & MetaItem , item : & Annotatable , push : & mut dyn FnMut (Annotatable) , _is_const : bool ,) { item . visit_with (& mut DetectNonGenericPointeeAttr { cx }) ; let (name_ident , generics) = if let Annotatable :: Item (aitem) = item && let ItemKind :: Struct (ident , g , struct_data) = & aitem . kind { if ! matches ! (struct_data , VariantData :: Struct { fields , recovered : _ } | VariantData :: Tuple (fields , _) if ! fields . is_empty ()) { cx . dcx () . emit_err (RequireOneField { span }) ; return ; } (* ident , g) } else { cx . dcx () . emit_err (RequireTransparent { span }) ; return ; } ; let self_params : Vec < _ > = generics . params . iter () . map (| p | match p . kind { GenericParamKind :: Lifetime => GenericArg :: Lifetime (cx . lifetime (p . span () , p . ident)) , GenericParamKind :: Type { .. } => GenericArg :: Type (cx . ty_ident (p . span () , p . ident)) , GenericParamKind :: Const { .. } => GenericArg :: Const (cx . const_ident (p . span () , p . ident)) , }) . collect () ; let type_params : Vec < _ > = generics . params . iter () . enumerate () . filter_map (| (idx , p) | { if let GenericParamKind :: Type { .. } = p . kind { Some ((idx , p . span () , p . attrs () . iter () . any (| attr | attr . has_name (sym :: pointee)))) } else { None } }) . collect () ; let pointee_param_idx = if type_params . is_empty () { cx . dcx () . emit_err (RequireOneGeneric { span }) ; return ; } else if type_params . len () == 1 { type_params [0] . 0 } else { let mut pointees = type_params . iter () . filter_map (| & (idx , span , is_pointee) | is_pointee . then_some ((idx , span))) ; match (pointees . next () , pointees . next ()) { (Some ((idx , _span)) , None) => idx , (None , _) => { cx . dcx () . emit_err (RequireOnePointee { span }) ; return ; } (Some ((_ , one)) , Some ((_ , another))) => { cx . dcx () . emit_err (TooManyPointees { one , another }) ; return ; } } } ; let path = cx . path_all (span , false , vec ! [name_ident] , self_params . clone ()) ; let self_type = cx . ty_path (path) ; let attrs = thin_vec ! [cx . attr_word (sym :: automatically_derived , span) ,] ; { let trait_path = cx . path_all (span , true , path ! (span , core :: marker :: CoercePointeeValidated) , vec ! []) ; let trait_ref = cx . trait_ref (trait_path) ; push (Annotatable :: Item (cx . item (span , attrs . clone () , ast :: ItemKind :: Impl (ast :: Impl { generics : Generics { params : generics . params . iter () . map (| p | match & p . kind { GenericParamKind :: Lifetime => { cx . lifetime_param (p . span () , p . ident , p . bounds . clone ()) } GenericParamKind :: Type { default : _ } => { cx . typaram (p . span () , p . ident , p . bounds . clone () , None) } GenericParamKind :: Const { ty , span : _ , default : _ } => cx . const_param (p . span () , p . ident , p . bounds . clone () , ty . clone () , None ,) , }) . collect () , where_clause : generics . where_clause . clone () , span : generics . span , } , of_trait : Some (Box :: new (ast :: TraitImplHeader { safety : ast :: Safety :: Default , polarity : ast :: ImplPolarity :: Positive , defaultness : ast :: Defaultness :: Final , constness : ast :: Const :: No , trait_ref , })) , self_ty : self_type . clone () , items : ThinVec :: new () , }) ,) ,)) ; } let mut add_impl_block = | generics , trait_symbol , trait_args | { let mut parts = path ! (span , core :: ops) ; parts . push (Ident :: new (trait_symbol , span)) ; let trait_path = cx . path_all (span , true , parts , trait_args) ; let trait_ref = cx . trait_ref (trait_path) ; let item = cx . item (span , attrs . clone () , ast :: ItemKind :: Impl (ast :: Impl { generics , of_trait : Some (Box :: new (ast :: TraitImplHeader { safety : ast :: Safety :: Default , polarity : ast :: ImplPolarity :: Positive , defaultness : ast :: Defaultness :: Final , constness : ast :: Const :: No , trait_ref , })) , self_ty : self_type . clone () , items : ThinVec :: new () , }) ,) ; push (Annotatable :: Item (item)) ; } ; let s_ty = cx . ty_ident (span , Ident :: new (sym :: __S , span)) ; let mut alt_self_params = self_params ; alt_self_params [pointee_param_idx] = GenericArg :: Type (s_ty . clone ()) ; let alt_self_type = cx . ty_path (cx . path_all (span , false , vec ! [name_ident] , alt_self_params)) ; let mut impl_generics = generics . clone () ; let pointee_ty_ident = generics . params [pointee_param_idx] . ident ; let mut self_bounds ; { let pointee = & mut impl_generics . params [pointee_param_idx] ; self_bounds = pointee . bounds . clone () ; if ! contains_maybe_sized_bound (& self_bounds) && ! contains_maybe_sized_bound_on_pointee (& generics . where_clause . predicates , pointee_ty_ident . name ,) { cx . dcx () . emit_err (RequiresMaybeSized { span : pointee_ty_ident . span , name : pointee_ty_ident , }) ; return ; } let arg = GenericArg :: Type (s_ty . clone ()) ; let unsize = cx . path_all (span , true , path ! (span , core :: marker :: Unsize) , vec ! [arg]) ; pointee . bounds . push (cx . trait_bound (unsize , false)) ; pointee . attrs . retain (| attr | ! attr . has_name (sym :: pointee)) ; } for (idx , (params , orig_params)) in impl_generics . params . iter_mut () . zip (& generics . params) . enumerate () { match & mut params . kind { ast :: GenericParamKind :: Const { default , .. } => * default = None , ast :: GenericParamKind :: Type { default } => * default = None , ast :: GenericParamKind :: Lifetime => { } } if idx != pointee_param_idx { for bound in & orig_params . bounds { let mut bound = bound . clone () ; let mut substitution = TypeSubstitution { from_name : pointee_ty_ident . name , to_ty : & s_ty , rewritten : false , } ; substitution . visit_param_bound (& mut bound , BoundKind :: Bound) ; if substitution . rewritten { params . bounds . push (bound) ; } } } } { let mut substitution = TypeSubstitution { from_name : pointee_ty_ident . name , to_ty : & s_ty , rewritten : false } ; for bound in & mut self_bounds { substitution . visit_param_bound (bound , BoundKind :: Bound) ; } } for predicate in & generics . where_clause . predicates { if let ast :: WherePredicateKind :: BoundPredicate (bound) = & predicate . kind { let mut substitution = TypeSubstitution { from_name : pointee_ty_ident . name , to_ty : & s_ty , rewritten : false , } ; let mut kind = ast :: WherePredicateKind :: BoundPredicate (bound . clone ()) ; substitution . visit_where_predicate_kind (& mut kind) ; if substitution . rewritten { let predicate = ast :: WherePredicate { attrs : predicate . attrs . clone () , kind , span : predicate . span , id : ast :: DUMMY_NODE_ID , is_placeholder : false , } ; impl_generics . where_clause . predicates . push (predicate) ; } } } let extra_param = cx . typaram (span , Ident :: new (sym :: __S , span) , self_bounds , None) ; impl_generics . params . insert (pointee_param_idx + 1 , extra_param) ; let gen_args = vec ! [GenericArg :: Type (alt_self_type)] ; add_impl_block (impl_generics . clone () , sym :: DispatchFromDyn , gen_args . clone ()) ; add_impl_block (impl_generics . clone () , sym :: CoerceUnsized , gen_args) ; }
}

macro_rules! contains_maybe_sized_bound_on_pointee_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function contains_maybe_sized_bound_on_pointee in module {}", module_path!());
    };
}

mkfn!{
    contains_maybe_sized_bound_on_pointee_introspect!();
    fn contains_maybe_sized_bound_on_pointee (predicates : & [WherePredicate] , pointee : Symbol) -> bool { for bound in predicates { if let ast :: WherePredicateKind :: BoundPredicate (bound) = & bound . kind && bound . bounded_ty . kind . is_simple_path () . is_some_and (| name | name == pointee) { for bound in & bound . bounds { if is_maybe_sized_bound (bound) { return true ; } } } } false }
}

macro_rules! is_maybe_sized_bound_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_maybe_sized_bound in module {}", module_path!());
    };
}

mkfn!{
    is_maybe_sized_bound_introspect!();
    fn is_maybe_sized_bound (bound : & GenericBound) -> bool { if let GenericBound :: Trait (trait_ref) = bound && let TraitBoundModifiers { polarity : ast :: BoundPolarity :: Maybe (_) , .. } = trait_ref . modifiers && is_sized_marker (& trait_ref . trait_ref . path) { true } else { false } }
}

macro_rules! contains_maybe_sized_bound_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function contains_maybe_sized_bound in module {}", module_path!());
    };
}

mkfn!{
    contains_maybe_sized_bound_introspect!();
    fn contains_maybe_sized_bound (bounds : & [GenericBound]) -> bool { bounds . iter () . any (is_maybe_sized_bound) }
}

macro_rules! path_segment_is_exact_match_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function path_segment_is_exact_match in module {}", module_path!());
    };
}

mkfn!{
    path_segment_is_exact_match_introspect!();
    fn path_segment_is_exact_match (path_segments : & [ast :: PathSegment] , syms : & [Symbol]) -> bool { path_segments . iter () . zip (syms) . all (| (segment , & symbol) | segment . ident . name == symbol) }
}

macro_rules! is_sized_marker_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_sized_marker in module {}", module_path!());
    };
}

mkfn!{
    is_sized_marker_introspect!();
    fn is_sized_marker (path : & ast :: Path) -> bool { const CORE_UNSIZE : [Symbol ; 3] = [sym :: core , sym :: marker , sym :: Sized] ; const STD_UNSIZE : [Symbol ; 3] = [sym :: std , sym :: marker , sym :: Sized] ; if path . segments . len () == 4 && path . is_global () { path_segment_is_exact_match (& path . segments [1 ..] , & CORE_UNSIZE) || path_segment_is_exact_match (& path . segments [1 ..] , & STD_UNSIZE) } else if path . segments . len () == 3 { path_segment_is_exact_match (& path . segments , & CORE_UNSIZE) || path_segment_is_exact_match (& path . segments , & STD_UNSIZE) } else { * path == sym :: Sized } }
}
mkitem!{mkstruct!{struct TypeSubstitution < 'a > { from_name : Symbol , to_ty : & 'a ast :: Ty , rewritten : bool , }}}
mkitem!{mkimpl!{impl < 'a > ast :: mut_visit :: MutVisitor for TypeSubstitution < 'a > { fn visit_ty (& mut self , ty : & mut ast :: Ty) { if let Some (name) = ty . kind . is_simple_path () && name == self . from_name { * ty = self . to_ty . clone () ; self . rewritten = true ; } else { ast :: mut_visit :: walk_ty (self , ty) ; } } fn visit_where_predicate_kind (& mut self , kind : & mut ast :: WherePredicateKind) { match kind { rustc_ast :: WherePredicateKind :: BoundPredicate (bound) => { bound . bound_generic_params . flat_map_in_place (| param | self . flat_map_generic_param (param)) ; self . visit_ty (& mut bound . bounded_ty) ; for bound in & mut bound . bounds { self . visit_param_bound (bound , BoundKind :: Bound) } } rustc_ast :: WherePredicateKind :: RegionPredicate (_) | rustc_ast :: WherePredicateKind :: EqPredicate (_) => { } } } }}}
mkitem!{mkstruct!{struct DetectNonGenericPointeeAttr < 'a , 'b > { cx : & 'a ExtCtxt < 'b > , }}}
mkitem!{mkimpl!{impl < 'a , 'b > rustc_ast :: visit :: Visitor < 'a > for DetectNonGenericPointeeAttr < 'a , 'b > { fn visit_attribute (& mut self , attr : & 'a rustc_ast :: Attribute) -> Self :: Result { if attr . has_name (sym :: pointee) { self . cx . dcx () . emit_err (errors :: NonGenericPointee { span : attr . span }) ; } } fn visit_generic_param (& mut self , param : & 'a rustc_ast :: GenericParam) -> Self :: Result { let mut error_on_pointee = AlwaysErrorOnGenericParam { cx : self . cx } ; match & param . kind { GenericParamKind :: Type { default } => { rustc_ast :: visit :: visit_opt ! (error_on_pointee , visit_ty , default) ; } GenericParamKind :: Const { .. } | GenericParamKind :: Lifetime => { rustc_ast :: visit :: walk_generic_param (& mut error_on_pointee , param) ; } } } fn visit_ty (& mut self , t : & 'a rustc_ast :: Ty) -> Self :: Result { let mut error_on_pointee = AlwaysErrorOnGenericParam { cx : self . cx } ; error_on_pointee . visit_ty (t) } }}}
mkitem!{mkstruct!{struct AlwaysErrorOnGenericParam < 'a , 'b > { cx : & 'a ExtCtxt < 'b > , }}}
mkitem!{mkimpl!{impl < 'a , 'b > rustc_ast :: visit :: Visitor < 'a > for AlwaysErrorOnGenericParam < 'a , 'b > { fn visit_attribute (& mut self , attr : & 'a rustc_ast :: Attribute) -> Self :: Result { if attr . has_name (sym :: pointee) { self . cx . dcx () . emit_err (errors :: NonGenericPointee { span : attr . span }) ; } } }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_coerce_pointee_requires_transparent , code = E0802)] struct RequireTransparent { # [primary_span] span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_coerce_pointee_requires_one_field , code = E0802)] struct RequireOneField { # [primary_span] span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_coerce_pointee_requires_one_generic , code = E0802)] struct RequireOneGeneric { # [primary_span] span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_coerce_pointee_requires_one_pointee , code = E0802)] struct RequireOnePointee { # [primary_span] span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_coerce_pointee_too_many_pointees , code = E0802)] struct TooManyPointees { # [primary_span] one : Span , # [label] another : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_coerce_pointee_requires_maybe_sized , code = E0802)] struct RequiresMaybeSized { # [primary_span] span : Span , name : Ident , }}}