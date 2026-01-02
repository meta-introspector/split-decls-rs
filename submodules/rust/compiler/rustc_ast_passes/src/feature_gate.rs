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
mkuse!{use rustc_ast :: visit :: { self , AssocCtxt , FnCtxt , FnKind , Visitor } ;}
mkuse!{use rustc_ast :: { NodeId , PatKind , attr , token } ;}
mkuse!{use rustc_feature :: { AttributeGate , BUILTIN_ATTRIBUTE_MAP , BuiltinAttribute , Features } ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_session :: parse :: { feature_err , feature_warn } ;}
mkuse!{use rustc_span :: source_map :: Spanned ;}
mkuse!{use rustc_span :: { Span , Symbol , sym } ;}
mkuse!{use thin_vec :: ThinVec ;}
mkuse!{use crate :: errors ;}
mkitem!{# [doc = " The common case."] macro_rules ! gate { ($ visitor : expr , $ feature : ident , $ span : expr , $ explain : expr) => { { if !$ visitor . features .$ feature () && !$ span . allows_unstable (sym ::$ feature) { # [allow (rustc :: untranslatable_diagnostic)] feature_err (&$ visitor . sess , sym ::$ feature , $ span , $ explain) . emit () ; } } } ; ($ visitor : expr , $ feature : ident , $ span : expr , $ explain : expr , $ help : expr) => { { if !$ visitor . features .$ feature () && !$ span . allows_unstable (sym ::$ feature) { # [allow (rustc :: diagnostic_outside_of_impl)] # [allow (rustc :: untranslatable_diagnostic)] feature_err (&$ visitor . sess , sym ::$ feature , $ span , $ explain) . with_help ($ help) . emit () ; } } } ; }}
mkitem!{# [doc = " The unusual case, where the `has_feature` condition is non-standard."] macro_rules ! gate_alt { ($ visitor : expr , $ has_feature : expr , $ name : expr , $ span : expr , $ explain : expr) => { { if !$ has_feature && !$ span . allows_unstable ($ name) { # [allow (rustc :: untranslatable_diagnostic)] feature_err (&$ visitor . sess , $ name , $ span , $ explain) . emit () ; } } } ; ($ visitor : expr , $ has_feature : expr , $ name : expr , $ span : expr , $ explain : expr , $ notes : expr) => { { if !$ has_feature && !$ span . allows_unstable ($ name) { # [allow (rustc :: untranslatable_diagnostic)] let mut diag = feature_err (&$ visitor . sess , $ name , $ span , $ explain) ; for note in $ notes { diag . note (* note) ; } diag . emit () ; } } } ; }}
mkitem!{# [doc = " The case involving a multispan."] macro_rules ! gate_multi { ($ visitor : expr , $ feature : ident , $ spans : expr , $ explain : expr) => { { if !$ visitor . features .$ feature () { let spans : Vec < _ > = $ spans . filter (| span | ! span . allows_unstable (sym ::$ feature)) . collect () ; if ! spans . is_empty () { feature_err (&$ visitor . sess , sym ::$ feature , spans , $ explain) . emit () ; } } } } ; }}
mkitem!{# [doc = " The legacy case."] macro_rules ! gate_legacy { ($ visitor : expr , $ feature : ident , $ span : expr , $ explain : expr) => { { if !$ visitor . features .$ feature () && !$ span . allows_unstable (sym ::$ feature) { feature_warn (&$ visitor . sess , sym ::$ feature , $ span , $ explain) ; } } } ; }}

macro_rules! check_attribute_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_attribute in module {}", module_path!());
    };
}

mkfn!{
    check_attribute_introspect!();
    pub fn check_attribute (attr : & ast :: Attribute , sess : & Session , features : & Features) { PostExpansionVisitor { sess , features } . visit_attribute (attr) }
}
mkitem!{mkstruct!{struct PostExpansionVisitor < 'a > { sess : & 'a Session , features : & 'a Features , }}}
mkitem!{mkimpl!{impl < 'a > PostExpansionVisitor < 'a > { # [doc = " Feature gate `impl Trait` inside `type Alias = $type_expr;`."] fn check_impl_trait (& self , ty : & ast :: Ty , in_associated_ty : bool) { struct ImplTraitVisitor < 'a > { vis : & 'a PostExpansionVisitor < 'a > , in_associated_ty : bool , } impl Visitor < '_ > for ImplTraitVisitor < '_ > { fn visit_ty (& mut self , ty : & ast :: Ty) { if let ast :: TyKind :: ImplTrait (..) = ty . kind { if self . in_associated_ty { gate ! (& self . vis , impl_trait_in_assoc_type , ty . span , "`impl Trait` in associated types is unstable") ; } else { gate ! (& self . vis , type_alias_impl_trait , ty . span , "`impl Trait` in type aliases is unstable") ; } } visit :: walk_ty (self , ty) ; } fn visit_anon_const (& mut self , _ : & ast :: AnonConst) -> Self :: Result { } } ImplTraitVisitor { vis : self , in_associated_ty } . visit_ty (ty) ; } fn check_late_bound_lifetime_defs (& self , params : & [ast :: GenericParam]) { let non_lt_param_spans = params . iter () . filter_map (| param | match param . kind { ast :: GenericParamKind :: Lifetime { .. } => None , _ => Some (param . ident . span) , }) ; gate_multi ! (& self , non_lifetime_binders , non_lt_param_spans , crate :: fluent_generated :: ast_passes_forbidden_non_lifetime_param) ; if self . features . non_lifetime_binders () { let const_param_spans : Vec < _ > = params . iter () . filter_map (| param | match param . kind { ast :: GenericParamKind :: Const { .. } => Some (param . ident . span) , _ => None , }) . collect () ; if ! const_param_spans . is_empty () { self . sess . dcx () . emit_err (errors :: ForbiddenConstParam { const_param_spans }) ; } } for param in params { if ! param . bounds . is_empty () { let spans : Vec < _ > = param . bounds . iter () . map (| b | b . span ()) . collect () ; self . sess . dcx () . emit_err (errors :: ForbiddenBound { spans }) ; } } } }}}
mkitem!{mkimpl!{impl < 'a > Visitor < 'a > for PostExpansionVisitor < 'a > { fn visit_attribute (& mut self , attr : & ast :: Attribute) { let attr_info = attr . ident () . and_then (| ident | BUILTIN_ATTRIBUTE_MAP . get (& ident . name)) ; if let Some (BuiltinAttribute { gate : AttributeGate :: Gated { feature , message , check , notes , .. } , .. }) = attr_info { gate_alt ! (self , check (self . features) , * feature , attr . span , * message , * notes) ; } if attr . has_name (sym :: doc) { for meta_item_inner in attr . meta_item_list () . unwrap_or_default () { macro_rules ! gate_doc { ($ ($ s : literal { $ ($ name : ident => $ feature : ident) * }) *) => { $ ($ (if meta_item_inner . has_name (sym ::$ name) { let msg = concat ! ("`#[doc(" , stringify ! ($ name) , ")]` is " , $ s) ; gate ! (self , $ feature , attr . span , msg) ; }) *) * } } gate_doc ! ("experimental" { cfg => doc_cfg cfg_hide => doc_cfg_hide masked => doc_masked notable_trait => doc_notable_trait } "meant for internal use only" { attribute => rustdoc_internals keyword => rustdoc_internals fake_variadic => rustdoc_internals search_unbox => rustdoc_internals }) ; } } } fn visit_item (& mut self , i : & 'a ast :: Item) { match & i . kind { ast :: ItemKind :: ForeignMod (_foreign_module) => { } ast :: ItemKind :: Struct (..) | ast :: ItemKind :: Enum (..) | ast :: ItemKind :: Union (..) => { for attr in attr :: filter_by_name (& i . attrs , sym :: repr) { for item in attr . meta_item_list () . unwrap_or_else (ThinVec :: new) { if item . has_name (sym :: simd) { gate ! (& self , repr_simd , attr . span , "SIMD types are experimental and possibly buggy") ; } } } } ast :: ItemKind :: Impl (ast :: Impl { of_trait : Some (of_trait) , .. }) => { if let ast :: ImplPolarity :: Negative (span) = of_trait . polarity { gate ! (& self , negative_impls , span . to (of_trait . trait_ref . path . span) , "negative trait bounds are not fully implemented; \
                         use marker types for now") ; } if let ast :: Defaultness :: Default (_) = of_trait . defaultness { gate ! (& self , specialization , i . span , "specialization is unstable") ; } } ast :: ItemKind :: Trait (box ast :: Trait { is_auto : ast :: IsAuto :: Yes , .. }) => { gate ! (& self , auto_traits , i . span , "auto traits are experimental and possibly buggy") ; } ast :: ItemKind :: TraitAlias (..) => { gate ! (& self , trait_alias , i . span , "trait aliases are experimental") ; } ast :: ItemKind :: MacroDef (_ , ast :: MacroDef { macro_rules : false , .. }) => { let msg = "`macro` is experimental" ; gate ! (& self , decl_macro , i . span , msg) ; } ast :: ItemKind :: TyAlias (box ast :: TyAlias { ty : Some (ty) , .. }) => { self . check_impl_trait (ty , false) } _ => { } } visit :: walk_item (self , i) ; } fn visit_foreign_item (& mut self , i : & 'a ast :: ForeignItem) { match i . kind { ast :: ForeignItemKind :: Fn (..) | ast :: ForeignItemKind :: Static (..) => { let link_name = attr :: first_attr_value_str_by_name (& i . attrs , sym :: link_name) ; let links_to_llvm = link_name . is_some_and (| val | val . as_str () . starts_with ("llvm.")) ; if links_to_llvm { gate ! (& self , link_llvm_intrinsics , i . span , "linking to LLVM intrinsics is experimental") ; } } ast :: ForeignItemKind :: TyAlias (..) => { gate ! (& self , extern_types , i . span , "extern types are experimental") ; } ast :: ForeignItemKind :: MacCall (..) => { } } visit :: walk_item (self , i) } fn visit_ty (& mut self , ty : & 'a ast :: Ty) { match & ty . kind { ast :: TyKind :: FnPtr (fn_ptr_ty) => { self . check_late_bound_lifetime_defs (& fn_ptr_ty . generic_params) ; } ast :: TyKind :: Never => { gate ! (& self , never_type , ty . span , "the `!` type is experimental") ; } ast :: TyKind :: Pat (..) => { gate ! (& self , pattern_types , ty . span , "pattern types are unstable") ; } _ => { } } visit :: walk_ty (self , ty) } fn visit_generics (& mut self , g : & 'a ast :: Generics) { for predicate in & g . where_clause . predicates { match & predicate . kind { ast :: WherePredicateKind :: BoundPredicate (bound_pred) => { self . check_late_bound_lifetime_defs (& bound_pred . bound_generic_params) ; } _ => { } } } visit :: walk_generics (self , g) ; } fn visit_fn_ret_ty (& mut self , ret_ty : & 'a ast :: FnRetTy) { if let ast :: FnRetTy :: Ty (output_ty) = ret_ty { if let ast :: TyKind :: Never = output_ty . kind { } else { self . visit_ty (output_ty) } } } fn visit_generic_args (& mut self , args : & 'a ast :: GenericArgs) { if let ast :: GenericArgs :: Parenthesized (generic_args) = args && let ast :: FnRetTy :: Ty (ref ty) = generic_args . output && matches ! (ty . kind , ast :: TyKind :: Never) { gate ! (& self , never_type , ty . span , "the `!` type is experimental") ; } visit :: walk_generic_args (self , args) ; } fn visit_expr (& mut self , e : & 'a ast :: Expr) { match e . kind { ast :: ExprKind :: TryBlock (_) => { gate ! (& self , try_blocks , e . span , "`try` expression is experimental") ; } ast :: ExprKind :: Lit (token :: Lit { kind : token :: LitKind :: Float | token :: LitKind :: Integer , suffix , .. }) => match suffix { Some (sym :: f16) => { gate ! (& self , f16 , e . span , "the type `f16` is unstable") } Some (sym :: f128) => { gate ! (& self , f128 , e . span , "the type `f128` is unstable") } _ => () , } , _ => { } } visit :: walk_expr (self , e) } fn visit_pat (& mut self , pattern : & 'a ast :: Pat) { match & pattern . kind { PatKind :: Slice (pats) => { for pat in pats { let inner_pat = match & pat . kind { PatKind :: Ident (.. , Some (pat)) => pat , _ => pat , } ; if let PatKind :: Range (Some (_) , None , Spanned { .. }) = inner_pat . kind { gate ! (& self , half_open_range_patterns_in_slices , pat . span , "`X..` patterns in slices are experimental") ; } } } PatKind :: Box (..) => { gate ! (& self , box_patterns , pattern . span , "box pattern syntax is experimental") ; } _ => { } } visit :: walk_pat (self , pattern) } fn visit_poly_trait_ref (& mut self , t : & 'a ast :: PolyTraitRef) { self . check_late_bound_lifetime_defs (& t . bound_generic_params) ; visit :: walk_poly_trait_ref (self , t) ; } fn visit_fn (& mut self , fn_kind : FnKind < 'a > , span : Span , _ : NodeId) { if let Some (_header) = fn_kind . header () { } if let FnKind :: Closure (ast :: ClosureBinder :: For { generic_params , .. } , ..) = fn_kind { self . check_late_bound_lifetime_defs (generic_params) ; } if fn_kind . ctxt () != Some (FnCtxt :: Foreign) && fn_kind . decl () . c_variadic () { gate ! (& self , c_variadic , span , "C-variadic functions are unstable") ; } visit :: walk_fn (self , fn_kind) } fn visit_assoc_item (& mut self , i : & 'a ast :: AssocItem , ctxt : AssocCtxt) { let is_fn = match & i . kind { ast :: AssocItemKind :: Fn (_) => true , ast :: AssocItemKind :: Type (box ast :: TyAlias { ty , .. }) => { if let (Some (_) , AssocCtxt :: Trait) = (ty , ctxt) { gate ! (& self , associated_type_defaults , i . span , "associated type defaults are unstable") ; } if let Some (ty) = ty { self . check_impl_trait (ty , true) ; } false } _ => false , } ; if let ast :: Defaultness :: Default (_) = i . kind . defaultness () { gate_alt ! (& self , self . features . specialization () || (is_fn && self . features . min_specialization ()) , sym :: specialization , i . span , "specialization is unstable") ; } visit :: walk_assoc_item (self , i , ctxt) } }}}

macro_rules! check_crate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_crate in module {}", module_path!());
    };
}

mkfn!{
    check_crate_introspect!();
    pub fn check_crate (krate : & ast :: Crate , sess : & Session , features : & Features) { maybe_stage_features (sess , features , krate) ; check_incompatible_features (sess , features) ; check_new_solver_banned_features (sess , features) ; let mut visitor = PostExpansionVisitor { sess , features } ; let spans = sess . psess . gated_spans . spans . borrow () ; macro_rules ! gate_all { ($ gate : ident , $ msg : literal) => { if let Some (spans) = spans . get (& sym ::$ gate) { for span in spans { gate ! (& visitor , $ gate , * span , $ msg) ; } } } ; ($ gate : ident , $ msg : literal , $ help : literal) => { if let Some (spans) = spans . get (& sym ::$ gate) { for span in spans { gate ! (& visitor , $ gate , * span , $ msg , $ help) ; } } } ; } gate_all ! (if_let_guard , "`if let` guards are experimental" , "you can write `if matches!(<expr>, <pattern>)` instead of `if let <pattern> = <expr>`") ; gate_all ! (async_trait_bounds , "`async` trait bounds are unstable" , "use the desugared name of the async trait, such as `AsyncFn`") ; gate_all ! (async_for_loop , "`for await` loops are experimental") ; gate_all ! (closure_lifetime_binder , "`for<...>` binders for closures are experimental" , "consider removing `for<...>`") ; gate_all ! (more_qualified_paths , "usage of qualified paths in this context is experimental") ; if let Some (spans) = spans . get (& sym :: yield_expr) { for span in spans { if (! visitor . features . coroutines () && ! span . allows_unstable (sym :: coroutines)) && (! visitor . features . gen_blocks () && ! span . allows_unstable (sym :: gen_blocks)) && (! visitor . features . yield_expr () && ! span . allows_unstable (sym :: yield_expr)) { # [allow (rustc :: untranslatable_diagnostic)] feature_err (& visitor . sess , sym :: yield_expr , * span , "yield syntax is experimental") . emit () ; } } } gate_all ! (gen_blocks , "gen blocks are experimental") ; gate_all ! (const_trait_impl , "const trait impls are experimental") ; gate_all ! (half_open_range_patterns_in_slices , "half-open range patterns in slices are unstable") ; gate_all ! (associated_const_equality , "associated const equality is incomplete") ; gate_all ! (yeet_expr , "`do yeet` expression is experimental") ; gate_all ! (const_closures , "const closures are experimental") ; gate_all ! (builtin_syntax , "`builtin #` syntax is unstable") ; gate_all ! (ergonomic_clones , "ergonomic clones are experimental") ; gate_all ! (explicit_tail_calls , "`become` expression is experimental") ; gate_all ! (generic_const_items , "generic const items are experimental") ; gate_all ! (guard_patterns , "guard patterns are experimental" , "consider using match arm guards") ; gate_all ! (default_field_values , "default values on fields are experimental") ; gate_all ! (fn_delegation , "functions delegation is not yet fully implemented") ; gate_all ! (postfix_match , "postfix match is experimental") ; gate_all ! (mut_ref , "mutable by-reference bindings are experimental") ; gate_all ! (global_registration , "global registration is experimental") ; gate_all ! (return_type_notation , "return type notation is experimental") ; gate_all ! (pin_ergonomics , "pinned reference syntax is experimental") ; gate_all ! (unsafe_fields , "`unsafe` fields are experimental") ; gate_all ! (unsafe_binders , "unsafe binder types are experimental") ; gate_all ! (contracts , "contracts are incomplete") ; gate_all ! (contracts_internals , "contract internal machinery is for internal use only") ; gate_all ! (where_clause_attrs , "attributes in `where` clause are unstable") ; gate_all ! (super_let , "`super let` is experimental") ; gate_all ! (frontmatter , "frontmatters are experimental") ; gate_all ! (coroutines , "coroutine syntax is experimental") ; if ! visitor . features . never_patterns () { if let Some (spans) = spans . get (& sym :: never_patterns) { for & span in spans { if span . allows_unstable (sym :: never_patterns) { continue ; } let sm = sess . source_map () ; if let Ok (snippet) = sm . span_to_snippet (span) && snippet == "!" { # [allow (rustc :: untranslatable_diagnostic)] feature_err (sess , sym :: never_patterns , span , "`!` patterns are experimental") . emit () ; } else { let suggestion = span . shrink_to_hi () ; sess . dcx () . emit_err (errors :: MatchArmWithNoBody { span , suggestion }) ; } } } } if ! visitor . features . negative_bounds () { for & span in spans . get (& sym :: negative_bounds) . iter () . copied () . flatten () { sess . dcx () . emit_err (errors :: NegativeBoundUnsupported { span }) ; } } macro_rules ! gate_all_legacy_dont_use { ($ gate : ident , $ msg : literal) => { for span in spans . get (& sym ::$ gate) . unwrap_or (& vec ! []) { gate_legacy ! (& visitor , $ gate , * span , $ msg) ; } } ; } gate_all_legacy_dont_use ! (box_patterns , "box pattern syntax is experimental") ; gate_all_legacy_dont_use ! (trait_alias , "trait aliases are experimental") ; gate_all_legacy_dont_use ! (decl_macro , "`macro` is experimental") ; gate_all_legacy_dont_use ! (try_blocks , "`try` blocks are unstable") ; gate_all_legacy_dont_use ! (auto_traits , "`auto` traits are unstable") ; visit :: walk_crate (& mut visitor , krate) ; }
}

macro_rules! maybe_stage_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function maybe_stage_features in module {}", module_path!());
    };
}

mkfn!{
    maybe_stage_features_introspect!();
    fn maybe_stage_features (sess : & Session , features : & Features , krate : & ast :: Crate) { if sess . opts . unstable_features . is_nightly_build () { return ; } if features . enabled_features () . is_empty () { return ; } let mut errored = false ; for attr in krate . attrs . iter () . filter (| attr | attr . has_name (sym :: feature)) { let mut err = errors :: FeatureOnNonNightly { span : attr . span , channel : option_"dev" . unwrap_or ("(unknown)") , stable_features : vec ! [] , sugg : None , } ; let mut all_stable = true ; for ident in attr . meta_item_list () . into_iter () . flatten () . flat_map (| nested | nested . ident ()) { let name = ident . name ; let stable_since = features . enabled_lang_features () . iter () . find (| feat | feat . gate_name == name) . map (| feat | feat . stable_since) . flatten () ; if let Some (since) = stable_since { err . stable_features . push (errors :: StableFeature { name , since }) ; } else { all_stable = false ; } } if all_stable { err . sugg = Some (attr . span) ; } sess . dcx () . emit_err (err) ; errored = true ; } assert ! (errored) ; }
}

macro_rules! check_incompatible_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_incompatible_features in module {}", module_path!());
    };
}

mkfn!{
    check_incompatible_features_introspect!();
    fn check_incompatible_features (sess : & Session , features : & Features) { let enabled_lang_features = features . enabled_lang_features () . iter () . map (| feat | (feat . gate_name , feat . attr_sp)) ; let enabled_lib_features = features . enabled_lib_features () . iter () . map (| feat | (feat . gate_name , feat . attr_sp)) ; let enabled_features = enabled_lang_features . chain (enabled_lib_features) ; for (f1 , f2) in rustc_feature :: INCOMPATIBLE_FEATURES . iter () . filter (| (f1 , f2) | features . enabled (* f1) && features . enabled (* f2)) { if let Some ((f1_name , f1_span)) = enabled_features . clone () . find (| (name , _) | name == f1) && let Some ((f2_name , f2_span)) = enabled_features . clone () . find (| (name , _) | name == f2) { let spans = vec ! [f1_span , f2_span] ; sess . dcx () . emit_err (errors :: IncompatibleFeatures { spans , f1 : f1_name , f2 : f2_name }) ; } } }
}

macro_rules! check_new_solver_banned_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_new_solver_banned_features in module {}", module_path!());
    };
}

mkfn!{
    check_new_solver_banned_features_introspect!();
    fn check_new_solver_banned_features (sess : & Session , features : & Features) { if ! sess . opts . unstable_opts . next_solver . globally { return ; } if let Some (gce_span) = features . enabled_lang_features () . iter () . find (| feat | feat . gate_name == sym :: generic_const_exprs) . map (| feat | feat . attr_sp) { # [allow (rustc :: symbol_intern_string_literal)] sess . dcx () . emit_err (errors :: IncompatibleFeatures { spans : vec ! [gce_span] , f1 : Symbol :: intern ("-Znext-solver=globally") , f2 : sym :: generic_const_exprs , }) ; } }
}