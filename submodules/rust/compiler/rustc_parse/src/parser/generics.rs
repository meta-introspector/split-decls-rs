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
mkuse!{use rustc_ast :: { self as ast , AttrVec , DUMMY_NODE_ID , GenericBounds , GenericParam , GenericParamKind , TyKind , WhereClause , token , } ;}
mkuse!{use rustc_errors :: { Applicability , PResult } ;}
mkuse!{use rustc_span :: { Ident , Span , kw , sym } ;}
mkuse!{use thin_vec :: ThinVec ;}
mkuse!{use super :: { ForceCollect , Parser , Trailing , UsePreAttrPos } ;}
mkuse!{use crate :: errors :: { self , MultipleWhereClauses , UnexpectedDefaultValueForLifetimeInGenericParameters , UnexpectedSelfInGenericParameters , WhereClauseBeforeTupleStructBody , WhereClauseBeforeTupleStructBodySugg , } ;}
mkuse!{use crate :: exp ;}
mkitem!{mkenum!{enum PredicateKindOrStructBody { PredicateKind (ast :: WherePredicateKind) , StructBody (ThinVec < ast :: FieldDef >) , }}}
mkitem!{mkimpl!{impl < 'a > Parser < 'a > { # [doc = " Parses bounds of a lifetime parameter `BOUND + BOUND + BOUND`, possibly with trailing `+`."] # [doc = ""] # [doc = " ```text"] # [doc = " BOUND = LT_BOUND (e.g., `'a`)"] # [doc = " ```"] fn parse_lt_param_bounds (& mut self) -> GenericBounds { let mut lifetimes = Vec :: new () ; while self . check_lifetime () { lifetimes . push (ast :: GenericBound :: Outlives (self . expect_lifetime ())) ; if ! self . eat_plus () { break ; } } lifetimes } # [doc = " Matches `typaram = IDENT (`?` unbound)? optbounds ( EQ ty )?`."] fn parse_ty_param (& mut self , preceding_attrs : AttrVec) -> PResult < 'a , GenericParam > { let ident = self . parse_ident () ? ; if self . may_recover () && ident . name . as_str () . to_ascii_lowercase () == kw :: Const . as_str () && self . check_ident () { return self . recover_const_param_with_mistyped_const (preceding_attrs , ident) ; } let mut colon_span = None ; let bounds = if self . eat (exp ! (Colon)) { colon_span = Some (self . prev_token . span) ; if self . token . is_keyword (kw :: Impl) { let impl_span = self . token . span ; let snapshot = self . create_snapshot_for_diagnostic () ; match self . parse_ty () { Ok (p) => { if let TyKind :: ImplTrait (_ , bounds) = & p . kind { let span = impl_span . to (self . token . span . shrink_to_lo ()) ; let mut err = self . dcx () . struct_span_err (span , "expected trait bound, found `impl Trait` type" ,) ; err . span_label (span , "not a trait") ; if let [bound , ..] = & bounds [..] { err . span_suggestion_verbose (impl_span . until (bound . span ()) , "use the trait bounds directly" , String :: new () , Applicability :: MachineApplicable ,) ; } return Err (err) ; } } Err (err) => { err . cancel () ; } } self . restore_snapshot (snapshot) ; } self . parse_generic_bounds () ? } else { Vec :: new () } ; let default = if self . eat (exp ! (Eq)) { Some (self . parse_ty () ?) } else { None } ; Ok (GenericParam { ident , id : ast :: DUMMY_NODE_ID , attrs : preceding_attrs , bounds , kind : GenericParamKind :: Type { default } , is_placeholder : false , colon_span , }) } pub (crate) fn parse_const_param (& mut self , preceding_attrs : AttrVec ,) -> PResult < 'a , GenericParam > { let const_span = self . token . span ; self . expect_keyword (exp ! (Const)) ? ; let ident = self . parse_ident () ? ; self . expect (exp ! (Colon)) ? ; let ty = self . parse_ty () ? ; let default = if self . eat (exp ! (Eq)) { Some (self . parse_const_arg () ?) } else { None } ; let span = if let Some (ref default) = default { const_span . to (default . value . span) } else { const_span . to (ty . span) } ; Ok (GenericParam { ident , id : ast :: DUMMY_NODE_ID , attrs : preceding_attrs , bounds : Vec :: new () , kind : GenericParamKind :: Const { ty , span , default } , is_placeholder : false , colon_span : None , }) } pub (crate) fn recover_const_param_with_mistyped_const (& mut self , preceding_attrs : AttrVec , mistyped_const_ident : Ident ,) -> PResult < 'a , GenericParam > { let ident = self . parse_ident () ? ; self . expect (exp ! (Colon)) ? ; let ty = self . parse_ty () ? ; let default = if self . eat (exp ! (Eq)) { Some (self . parse_const_arg () ?) } else { None } ; let span = if let Some (ref default) = default { mistyped_const_ident . span . to (default . value . span) } else { mistyped_const_ident . span . to (ty . span) } ; self . dcx () . struct_span_err (mistyped_const_ident . span , format ! ("`const` keyword was mistyped as `{}`" , mistyped_const_ident . as_str ()) ,) . with_span_suggestion_verbose (mistyped_const_ident . span , "use the `const` keyword" , kw :: Const , Applicability :: MachineApplicable ,) . emit () ; Ok (GenericParam { ident , id : ast :: DUMMY_NODE_ID , attrs : preceding_attrs , bounds : Vec :: new () , kind : GenericParamKind :: Const { ty , span , default } , is_placeholder : false , colon_span : None , }) } # [doc = " Parse a (possibly empty) list of generic (lifetime, type, const) parameters."] # [doc = ""] # [doc = " ```ebnf"] # [doc = " GenericParams = (GenericParam (\",\" GenericParam)* \",\"?)?"] # [doc = " ```"] pub (super) fn parse_generic_params (& mut self) -> PResult < 'a , ThinVec < ast :: GenericParam > > { let mut params = ThinVec :: new () ; let mut done = false ; while ! done { let attrs = self . parse_outer_attributes () ? ; let param = self . collect_tokens (None , attrs , ForceCollect :: No , | this , attrs | { if this . eat_keyword_noexpect (kw :: SelfUpper) { this . dcx () . emit_err (UnexpectedSelfInGenericParameters { span : this . prev_token . span }) ; let _ = this . eat (exp ! (Comma)) ; } let param = if this . check_lifetime () { let lifetime = this . expect_lifetime () ; let (colon_span , bounds) = if this . eat (exp ! (Colon)) { (Some (this . prev_token . span) , this . parse_lt_param_bounds ()) } else { (None , Vec :: new ()) } ; if this . check_noexpect (& token :: Eq) && this . look_ahead (1 , | t | t . is_lifetime ()) { let lo = this . token . span ; this . bump () ; this . bump () ; let span = lo . to (this . prev_token . span) ; this . dcx () . emit_err (UnexpectedDefaultValueForLifetimeInGenericParameters { span , }) ; } Some (ast :: GenericParam { ident : lifetime . ident , id : lifetime . id , attrs , bounds , kind : ast :: GenericParamKind :: Lifetime , is_placeholder : false , colon_span , }) } else if this . check_keyword (exp ! (Const)) { Some (this . parse_const_param (attrs) ?) } else if this . check_ident () { Some (this . parse_ty_param (attrs) ?) } else if this . token . can_begin_type () { let snapshot = this . create_snapshot_for_diagnostic () ; let lo = this . token . span ; match this . parse_ty_where_predicate_kind () { Ok (_) => { this . dcx () . emit_err (errors :: BadAssocTypeBounds { span : lo . to (this . prev_token . span) , }) ; } Err (err) => { err . cancel () ; this . restore_snapshot (snapshot) ; } } return Ok ((None , Trailing :: No , UsePreAttrPos :: No)) ; } else { if ! attrs . is_empty () { if ! params . is_empty () { this . dcx () . emit_err (errors :: AttrAfterGeneric { span : attrs [0] . span }) ; } else { this . dcx () . emit_err (errors :: AttrWithoutGenerics { span : attrs [0] . span }) ; } } return Ok ((None , Trailing :: No , UsePreAttrPos :: No)) ; } ; if ! this . eat (exp ! (Comma)) { done = true ; } Ok ((param , Trailing :: No , UsePreAttrPos :: No)) }) ? ; if let Some (param) = param { params . push (param) ; } else { break ; } } Ok (params) } # [doc = " Parses a set of optional generic type parameter declarations. Where"] # [doc = " clauses are not parsed here, and must be added later via"] # [doc = " `parse_where_clause()`."] # [doc = ""] # [doc = " matches generics = ( ) | ( < > ) | ( < typaramseq ( , )? > ) | ( < lifetimes ( , )? > )"] # [doc = "                  | ( < lifetimes , typaramseq ( , )? > )"] # [doc = " where   typaramseq = ( typaram ) | ( typaram , typaramseq )"] pub (super) fn parse_generics (& mut self) -> PResult < 'a , ast :: Generics > { if self . eat_noexpect (& token :: PathSep) { self . dcx () . emit_err (errors :: InvalidPathSepInFnDefinition { span : self . prev_token . span }) ; } let span_lo = self . token . span ; let (params , span) = if self . eat_lt () { let params = self . parse_generic_params () ? ; self . expect_gt_or_maybe_suggest_closing_generics (& params) ? ; (params , span_lo . to (self . prev_token . span)) } else { (ThinVec :: new () , self . prev_token . span . shrink_to_hi ()) } ; Ok (ast :: Generics { params , where_clause : WhereClause { has_where_token : false , predicates : ThinVec :: new () , span : self . prev_token . span . shrink_to_hi () , } , span , }) } # [doc = " Parses an experimental fn contract"] # [doc = " (`contract_requires(WWW) contract_ensures(ZZZ)`)"] pub (super) fn parse_contract (& mut self) -> PResult < 'a , Option < Box < ast :: FnContract > > > { let requires = if self . eat_keyword_noexpect (exp ! (ContractRequires) . kw) { self . psess . gated_spans . gate (sym :: contracts_internals , self . prev_token . span) ; let precond = self . parse_expr () ? ; Some (precond) } else { None } ; let ensures = if self . eat_keyword_noexpect (exp ! (ContractEnsures) . kw) { self . psess . gated_spans . gate (sym :: contracts_internals , self . prev_token . span) ; let postcond = self . parse_expr () ? ; Some (postcond) } else { None } ; if requires . is_none () && ensures . is_none () { Ok (None) } else { Ok (Some (Box :: new (ast :: FnContract { requires , ensures }))) } } # [doc = " Parses an optional where-clause."] # [doc = ""] # [doc = " ```ignore (only-for-syntax-highlight)"] # [doc = " where T : Trait<U, V> + 'b, 'a : 'b"] # [doc = " ```"] pub (super) fn parse_where_clause (& mut self) -> PResult < 'a , WhereClause > { self . parse_where_clause_common (None) . map (| (clause , _) | clause) } pub (super) fn parse_struct_where_clause (& mut self , struct_name : Ident , body_insertion_point : Span ,) -> PResult < 'a , (WhereClause , Option < ThinVec < ast :: FieldDef > >) > { self . parse_where_clause_common (Some ((struct_name , body_insertion_point))) } fn parse_where_clause_common (& mut self , struct_ : Option < (Ident , Span) > ,) -> PResult < 'a , (WhereClause , Option < ThinVec < ast :: FieldDef > >) > { let mut where_clause = WhereClause { has_where_token : false , predicates : ThinVec :: new () , span : self . prev_token . span . shrink_to_hi () , } ; let mut tuple_struct_body = None ; if ! self . eat_keyword (exp ! (Where)) { return Ok ((where_clause , None)) ; } if self . eat_noexpect (& token :: Colon) { let colon_span = self . prev_token . span ; self . dcx () . struct_span_err (colon_span , "unexpected colon after `where`") . with_span_suggestion_short (colon_span , "remove the colon" , "" , Applicability :: MachineApplicable ,) . emit () ; } where_clause . has_where_token = true ; let where_lo = self . prev_token . span ; if self . choose_generics_over_qpath (0) { let generics = self . parse_generics () ? ; self . dcx () . emit_err (errors :: WhereOnGenerics { span : generics . span }) ; } loop { let where_sp = where_lo . to (self . prev_token . span) ; let attrs = self . parse_outer_attributes () ? ; let pred_lo = self . token . span ; let predicate = self . collect_tokens (None , attrs , ForceCollect :: No , | this , attrs | { for attr in & attrs { self . psess . gated_spans . gate (sym :: where_clause_attrs , attr . span) ; } let kind = if this . check_lifetime () && this . look_ahead (1 , | t | ! t . is_like_plus ()) { let lifetime = this . expect_lifetime () ; this . expect (exp ! (Colon)) ? ; let bounds = this . parse_lt_param_bounds () ; Some (ast :: WherePredicateKind :: RegionPredicate (ast :: WhereRegionPredicate { lifetime , bounds , })) } else if this . check_type () { match this . parse_ty_where_predicate_kind_or_recover_tuple_struct_body (struct_ , pred_lo , where_sp ,) ? { PredicateKindOrStructBody :: PredicateKind (kind) => Some (kind) , PredicateKindOrStructBody :: StructBody (body) => { tuple_struct_body = Some (body) ; None } } } else { None } ; let predicate = kind . map (| kind | ast :: WherePredicate { attrs , kind , id : DUMMY_NODE_ID , span : pred_lo . to (this . prev_token . span) , is_placeholder : false , }) ; Ok ((predicate , Trailing :: No , UsePreAttrPos :: No)) }) ? ; match predicate { Some (predicate) => where_clause . predicates . push (predicate) , None => break , } let prev_token = self . prev_token . span ; let ate_comma = self . eat (exp ! (Comma)) ; if self . eat_keyword_noexpect (kw :: Where) { self . dcx () . emit_err (MultipleWhereClauses { span : self . token . span , previous : pred_lo , between : prev_token . shrink_to_hi () . to (self . prev_token . span) , }) ; } else if ! ate_comma { break ; } } where_clause . span = where_lo . to (self . prev_token . span) ; Ok ((where_clause , tuple_struct_body)) } fn parse_ty_where_predicate_kind_or_recover_tuple_struct_body (& mut self , struct_ : Option < (Ident , Span) > , pred_lo : Span , where_sp : Span ,) -> PResult < 'a , PredicateKindOrStructBody > { let mut snapshot = None ; if let Some (struct_) = struct_ && self . may_recover () && self . token == token :: OpenParen { snapshot = Some ((struct_ , self . create_snapshot_for_diagnostic ())) ; } ; match self . parse_ty_where_predicate_kind () { Ok (pred) => Ok (PredicateKindOrStructBody :: PredicateKind (pred)) , Err (type_err) => { let Some (((struct_name , body_insertion_point) , mut snapshot)) = snapshot else { return Err (type_err) ; } ; match snapshot . parse_tuple_struct_body () { Ok (body) if matches ! (snapshot . token . kind , token :: Semi | token :: Eof) || snapshot . token . can_begin_item () => { type_err . cancel () ; let body_sp = pred_lo . to (snapshot . prev_token . span) ; let map = self . psess . source_map () ; self . dcx () . emit_err (WhereClauseBeforeTupleStructBody { span : where_sp , name : struct_name . span , body : body_sp , sugg : map . span_to_snippet (body_sp) . ok () . map (| body | { WhereClauseBeforeTupleStructBodySugg { left : body_insertion_point . shrink_to_hi () , snippet : body , right : map . end_point (where_sp) . to (body_sp) , } }) , }) ; self . restore_snapshot (snapshot) ; Ok (PredicateKindOrStructBody :: StructBody (body)) } Ok (_) => Err (type_err) , Err (body_err) => { body_err . cancel () ; Err (type_err) } } } } } fn parse_ty_where_predicate_kind (& mut self) -> PResult < 'a , ast :: WherePredicateKind > { let (bound_vars , _) = self . parse_higher_ranked_binder () ? ; let ty = self . parse_ty_for_where_clause () ? ; if self . eat (exp ! (Colon)) { let bounds = self . parse_generic_bounds () ? ; Ok (ast :: WherePredicateKind :: BoundPredicate (ast :: WhereBoundPredicate { bound_generic_params : bound_vars , bounded_ty : ty , bounds , })) } else if self . eat (exp ! (Eq)) || self . eat (exp ! (EqEq)) { let rhs_ty = self . parse_ty () ? ; Ok (ast :: WherePredicateKind :: EqPredicate (ast :: WhereEqPredicate { lhs_ty : ty , rhs_ty })) } else { self . maybe_recover_bounds_doubled_colon (& ty) ? ; self . unexpected_any () } } pub (super) fn choose_generics_over_qpath (& self , start : usize) -> bool { self . look_ahead (start , | t | t == & token :: Lt) && (self . look_ahead (start + 1 , | t | t == & token :: Pound || t == & token :: Gt) || self . look_ahead (start + 1 , | t | t . is_lifetime () || t . is_ident ()) && self . look_ahead (start + 2 , | t | { matches ! (t . kind , token :: Gt | token :: Comma | token :: Colon | token :: Eq) || t . kind == token :: Question }) || self . is_keyword_ahead (start + 1 , & [kw :: Const])) } }}}