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
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_hir :: intravisit ;}
mkuse!{use rustc_hir :: intravisit :: Visitor ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: util :: { CheckRegions , NotUniqueParam } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt , TypeSuperVisitable , TypeVisitable , TypeVisitor } ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use tracing :: { instrument , trace } ;}
mkuse!{use crate :: errors :: { DuplicateArg , NotParam } ;}
mkitem!{mkstruct!{struct OpaqueTypeCollector < 'tcx > { tcx : TyCtxt < 'tcx > , opaques : Vec < LocalDefId > , # [doc = " The `DefId` of the item which we are collecting opaque types for."] item : LocalDefId , # [doc = " Avoid infinite recursion due to recursive declarations."] seen : FxHashSet < LocalDefId > , span : Option < Span > , mode : CollectionMode , }}}
mkitem!{mkenum!{enum CollectionMode { # [doc = " For impl trait in assoc types we only permit collecting them from"] # [doc = " associated types of the same impl block."] ImplTraitInAssocTypes , # [doc = " When collecting for an explicit `#[define_opaque]` attribute, find all TAITs"] Taits , # [doc = " The default case, only collect RPITs and AsyncFn return types, as these are"] # [doc = " always defined by the current item."] RpitAndAsyncFnOnly , }}}
mkitem!{mkimpl!{impl < 'tcx > OpaqueTypeCollector < 'tcx > { fn new (tcx : TyCtxt < 'tcx > , item : LocalDefId) -> Self { let mode = match tcx . def_kind (item) { DefKind :: AssocConst | DefKind :: AssocFn | DefKind :: AssocTy => { CollectionMode :: ImplTraitInAssocTypes } DefKind :: TyAlias => CollectionMode :: Taits , _ => CollectionMode :: RpitAndAsyncFnOnly , } ; Self { tcx , opaques : Vec :: new () , item , seen : Default :: default () , span : None , mode } } fn span (& self) -> Span { self . span . unwrap_or_else (| | { self . tcx . def_ident_span (self . item) . unwrap_or_else (| | self . tcx . def_span (self . item)) }) } fn visit_spanned (& mut self , span : Span , value : impl TypeVisitable < TyCtxt < 'tcx > >) { let old = self . span ; self . span = Some (span) ; value . visit_with (self) ; self . span = old ; } fn parent_impl_trait_ref (& self) -> Option < ty :: TraitRef < 'tcx > > { let parent = self . parent () ? ; if matches ! (self . tcx . def_kind (parent) , DefKind :: Impl { .. }) { Some (self . tcx . impl_trait_ref (parent) ? . instantiate_identity ()) } else { None } } fn parent (& self) -> Option < LocalDefId > { match self . tcx . def_kind (self . item) { DefKind :: AssocFn | DefKind :: AssocTy | DefKind :: AssocConst => { Some (self . tcx . local_parent (self . item)) } _ => None , } } # [instrument (level = "trace" , skip (self))] fn collect_taits_declared_in_body (& mut self) { let body = self . tcx . hir_body_owned_by (self . item) . value ; struct TaitInBodyFinder < 'a , 'tcx > { collector : & 'a mut OpaqueTypeCollector < 'tcx > , } impl < 'v > intravisit :: Visitor < 'v > for TaitInBodyFinder < '_ , '_ > { # [instrument (level = "trace" , skip (self))] fn visit_nested_item (& mut self , id : rustc_hir :: ItemId) { let id = id . owner_id . def_id ; if let DefKind :: TyAlias = self . collector . tcx . def_kind (id) { let items = self . collector . tcx . opaque_types_defined_by (id) ; self . collector . opaques . extend (items) ; } } # [instrument (level = "trace" , skip (self))] fn visit_nested_body (& mut self , id : rustc_hir :: BodyId) { let body = self . collector . tcx . hir_body (id) ; self . visit_body (body) ; } } TaitInBodyFinder { collector : self } . visit_expr (body) ; } # [instrument (level = "debug" , skip (self))] fn visit_opaque_ty (& mut self , alias_ty : ty :: AliasTy < 'tcx >) { if ! self . seen . insert (alias_ty . def_id . expect_local ()) { return ; } match self . tcx . local_opaque_ty_origin (alias_ty . def_id . expect_local ()) { rustc_hir :: OpaqueTyOrigin :: FnReturn { .. } | rustc_hir :: OpaqueTyOrigin :: AsyncFn { .. } => { } rustc_hir :: OpaqueTyOrigin :: TyAlias { in_assoc_ty , .. } => match self . mode { CollectionMode :: ImplTraitInAssocTypes => { if ! in_assoc_ty { return ; } } CollectionMode :: Taits => { if in_assoc_ty { return ; } } CollectionMode :: RpitAndAsyncFnOnly => return , } , } trace ! (? alias_ty , "adding") ; self . opaques . push (alias_ty . def_id . expect_local ()) ; let parent_count = self . tcx . generics_of (alias_ty . def_id) . parent_count ; match self . tcx . uses_unique_generic_params (& alias_ty . args [.. parent_count] , CheckRegions :: FromFunction) { Ok (()) => { for (pred , span) in self . tcx . explicit_item_bounds (alias_ty . def_id) . iter_identity_copied () { trace ! (? pred) ; self . visit_spanned (span , pred) ; } } Err (NotUniqueParam :: NotParam (arg)) => { self . tcx . dcx () . emit_err (NotParam { arg , span : self . span () , opaque_span : self . tcx . def_span (alias_ty . def_id) , }) ; } Err (NotUniqueParam :: DuplicateParam (arg)) => { self . tcx . dcx () . emit_err (DuplicateArg { arg , span : self . span () , opaque_span : self . tcx . def_span (alias_ty . def_id) , }) ; } } } # [doc = " Checks the `#[define_opaque]` attributes on items and collects opaques to define"] # [doc = " from the referenced types."] # [instrument (level = "trace" , skip (self))] fn collect_taits_from_defines_attr (& mut self) { let hir_id = self . tcx . local_def_id_to_hir_id (self . item) ; if ! hir_id . is_owner () { return ; } let Some (defines) = self . tcx . hir_attr_map (hir_id . owner) . define_opaque else { return ; } ; for & (span , define) in defines { trace ! (? define) ; let mode = std :: mem :: replace (& mut self . mode , CollectionMode :: Taits) ; let n = self . opaques . len () ; super :: sig_types :: walk_types (self . tcx , define , self) ; if n == self . opaques . len () { self . tcx . dcx () . span_err (span , "item does not contain any opaque types") ; } self . mode = mode ; } self . mode = CollectionMode :: RpitAndAsyncFnOnly ; } }}}
mkitem!{mkimpl!{impl < 'tcx > super :: sig_types :: SpannedTypeVisitor < 'tcx > for OpaqueTypeCollector < 'tcx > { # [instrument (skip (self) , ret , level = "trace")] fn visit (& mut self , span : Span , value : impl TypeVisitable < TyCtxt < 'tcx > >) { self . visit_spanned (span , value) ; } }}}
mkitem!{mkimpl!{impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for OpaqueTypeCollector < 'tcx > { # [instrument (skip (self) , ret , level = "trace")] fn visit_ty (& mut self , t : Ty < 'tcx >) { t . super_visit_with (self) ; match * t . kind () { ty :: Alias (ty :: Opaque , alias_ty) if alias_ty . def_id . is_local () => { self . visit_opaque_ty (alias_ty) ; } ty :: Alias (ty :: Free , alias_ty) if let Some (def_id) = alias_ty . def_id . as_local () => { if ! self . seen . insert (def_id) { return ; } self . tcx . type_of (alias_ty . def_id) . instantiate (self . tcx , alias_ty . args) . visit_with (self) ; } ty :: Alias (ty :: Projection , alias_ty) => { if let Some (impl_trait_ref) = self . parent_impl_trait_ref () { if alias_ty . trait_ref (self . tcx) == impl_trait_ref { let parent = self . parent () . expect ("we should have a parent here") ; for & assoc in self . tcx . associated_items (parent) . in_definition_order () { trace ! (? assoc) ; if assoc . expect_trait_impl () != Ok (alias_ty . def_id) { continue ; } if ! assoc . defaultness (self . tcx) . is_final () { continue ; } if ! self . seen . insert (assoc . def_id . expect_local ()) { return ; } let alias_args = alias_ty . args . rebase_onto (self . tcx , impl_trait_ref . def_id , ty :: GenericArgs :: identity_for_item (self . tcx , parent) ,) ; if self . tcx . check_args_compatible (assoc . def_id , alias_args) { self . tcx . type_of (assoc . def_id) . instantiate (self . tcx , alias_args) . visit_with (self) ; return ; } else { self . tcx . dcx () . span_delayed_bug (self . tcx . def_span (assoc . def_id) , "item had incorrect args" ,) ; } } } } else if let Some (ty :: ImplTraitInTraitData :: Trait { fn_def_id , .. }) = self . tcx . opt_rpitit_info (alias_ty . def_id) && fn_def_id == self . item . into () { let ty = self . tcx . type_of (alias_ty . def_id) . instantiate (self . tcx , alias_ty . args) ; let ty :: Alias (ty :: Opaque , alias_ty) = * ty . kind () else { bug ! ("{ty:?}") } ; self . visit_opaque_ty (alias_ty) ; } } _ => trace ! (kind =? t . kind ()) , } } }}}

macro_rules! opaque_types_defined_by_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function opaque_types_defined_by in module {}", module_path!());
    };
}

mkfn!{
    opaque_types_defined_by_introspect!();
    fn opaque_types_defined_by < 'tcx > (tcx : TyCtxt < 'tcx > , item : LocalDefId ,) -> & 'tcx ty :: List < LocalDefId > { let kind = tcx . def_kind (item) ; trace ! (? kind) ; let mut collector = OpaqueTypeCollector :: new (tcx , item) ; collector . collect_taits_from_defines_attr () ; super :: sig_types :: walk_types (tcx , item , & mut collector) ; match kind { DefKind :: AssocFn | DefKind :: Fn | DefKind :: Static { .. } | DefKind :: Const | DefKind :: AssocConst | DefKind :: AnonConst => { collector . collect_taits_declared_in_body () ; } DefKind :: Closure | DefKind :: InlineConst | DefKind :: SyntheticCoroutineBody => { collector . opaques . extend (tcx . opaque_types_defined_by (tcx . local_parent (item))) ; } DefKind :: AssocTy | DefKind :: TyAlias | DefKind :: GlobalAsm => { } DefKind :: OpaqueTy | DefKind :: Mod | DefKind :: Struct | DefKind :: Union | DefKind :: Enum | DefKind :: Variant | DefKind :: Trait | DefKind :: ForeignTy | DefKind :: TraitAlias | DefKind :: TyParam | DefKind :: ConstParam | DefKind :: Ctor (_ , _) | DefKind :: Macro (_) | DefKind :: ExternCrate | DefKind :: Use | DefKind :: ForeignMod | DefKind :: Field | DefKind :: LifetimeParam | DefKind :: Impl { .. } => { span_bug ! (tcx . def_span (item) , "`opaque_types_defined_by` not defined for {} `{item:?}`" , kind . descr (item . to_def_id ())) ; } } tcx . mk_local_def_ids (& collector . opaques) }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (super) fn provide (providers : & mut Providers) { * providers = Providers { opaque_types_defined_by , .. * providers } ; }
}