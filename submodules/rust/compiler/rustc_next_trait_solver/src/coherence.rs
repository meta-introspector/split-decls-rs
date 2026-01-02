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
mkuse!{use std :: fmt :: Debug ;}
mkuse!{use std :: ops :: ControlFlow ;}
mkuse!{use derive_where :: derive_where ;}
mkuse!{use rustc_type_ir :: inherent :: * ;}
mkuse!{use rustc_type_ir :: { self as ty , InferCtxtLike , Interner , TrivialTypeTraversalImpls , TypeVisitable , TypeVisitableExt , TypeVisitor , } ;}
mkuse!{use tracing :: instrument ;}
mkitem!{mkenum!{#[doc = " Whether we do the orphan check relative to this crate or to some remote crate."] #[derive (Copy , Clone , Debug)] pub enum InCrate { Local { mode : OrphanCheckMode } , Remote , }}}
mkitem!{mkenum!{#[derive (Copy , Clone , Debug)] pub enum OrphanCheckMode { #[doc = " Proper orphan check."] Proper , #[doc = " Improper orphan check for backward compatibility."] #[doc = ""] #[doc = " In this mode, type params inside projections are considered to be covered"] #[doc = " even if the projection may normalize to a type that doesn't actually cover"] #[doc = " them. This is unsound. See also [#124559] and [#99554]."] #[doc = ""] #[doc = " [#124559]: https://github.com/rust-lang/rust/issues/124559"] #[doc = " [#99554]: https://github.com/rust-lang/rust/issues/99554"] Compat , }}}
mkitem!{mkenum!{#[derive (Debug , Copy , Clone)] pub enum Conflict { Upstream , Downstream , }}}

macro_rules! trait_ref_is_knowable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trait_ref_is_knowable in module {}", module_path!());
    };
}

mkfn!{
    trait_ref_is_knowable_introspect!();
    #[doc = " Returns whether all impls which would apply to the `trait_ref`"] #[doc = " e.g. `Ty: Trait<Arg>` are already known in the local crate."] #[doc = ""] #[doc = " This both checks whether any downstream or sibling crates could"] #[doc = " implement it and whether an upstream crate can add this impl"] #[doc = " without breaking backwards compatibility."] #[instrument (level = "debug" , skip (infcx , lazily_normalize_ty) , ret)] pub fn trait_ref_is_knowable < Infcx , I , E > (infcx : & Infcx , trait_ref : ty :: TraitRef < I > , mut lazily_normalize_ty : impl FnMut (I :: Ty) -> Result < I :: Ty , E > ,) -> Result < Result < () , Conflict > , E > where Infcx : InferCtxtLike < Interner = I > , I : Interner , E : Debug , { if orphan_check_trait_ref (infcx , trait_ref , InCrate :: Remote , & mut lazily_normalize_ty) ? . is_ok () { return Ok (Err (Conflict :: Downstream)) ; } if trait_ref_is_local_or_fundamental (infcx . cx () , trait_ref) { return Ok (Ok (())) ; } if orphan_check_trait_ref (infcx , trait_ref , InCrate :: Local { mode : OrphanCheckMode :: Proper } , & mut lazily_normalize_ty ,) ? . is_ok () { Ok (Ok (())) } else { Ok (Err (Conflict :: Upstream)) } }
}

macro_rules! trait_ref_is_local_or_fundamental_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trait_ref_is_local_or_fundamental in module {}", module_path!());
    };
}

mkfn!{
    trait_ref_is_local_or_fundamental_introspect!();
    pub fn trait_ref_is_local_or_fundamental < I : Interner > (tcx : I , trait_ref : ty :: TraitRef < I >) -> bool { trait_ref . def_id . is_local () || tcx . trait_is_fundamental (trait_ref . def_id) }
}
mkitem!{TrivialTypeTraversalImpls ! { IsFirstInputType , }}
mkitem!{mkenum!{#[derive (Debug , Copy , Clone)] pub enum IsFirstInputType { No , Yes , }}}
mkitem!{mkimpl!{impl From < bool > for IsFirstInputType { fn from (b : bool) -> IsFirstInputType { match b { false => IsFirstInputType :: No , true => IsFirstInputType :: Yes , } } }}}
mkitem!{mkenum!{#[derive_where (Debug ; I : Interner , T : Debug)] pub enum OrphanCheckErr < I : Interner , T > { NonLocalInputType (Vec < (I :: Ty , IsFirstInputType) >) , UncoveredTyParams (UncoveredTyParams < I , T >) , }}}
mkitem!{mkstruct!{#[derive_where (Debug ; I : Interner , T : Debug)] pub struct UncoveredTyParams < I : Interner , T > { pub uncovered : T , pub local_ty : Option < I :: Ty > , }}}

macro_rules! orphan_check_trait_ref_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function orphan_check_trait_ref in module {}", module_path!());
    };
}

mkfn!{
    orphan_check_trait_ref_introspect!();
    #[doc = " Checks whether a trait-ref is potentially implementable by a crate."] #[doc = ""] #[doc = " The current rule is that a trait-ref orphan checks in a crate C:"] #[doc = ""] #[doc = " 1. Order the parameters in the trait-ref in generic parameters order"] #[doc = " - Self first, others linearly (e.g., `<U as Foo<V, W>>` is U < V < W)."] #[doc = " 2. Of these type parameters, there is at least one type parameter"] #[doc = "    in which, walking the type as a tree, you can reach a type local"] #[doc = "    to C where all types in-between are fundamental types. Call the"] #[doc = "    first such parameter the \"local key parameter\"."] #[doc = "     - e.g., `Box<LocalType>` is OK, because you can visit LocalType"] #[doc = "       going through `Box`, which is fundamental."] #[doc = "     - similarly, `FundamentalPair<Vec<()>, Box<LocalType>>` is OK for"] #[doc = "       the same reason."] #[doc = "     - but (knowing that `Vec<T>` is non-fundamental, and assuming it's"] #[doc = "       not local), `Vec<LocalType>` is bad, because `Vec<->` is between"] #[doc = "       the local type and the type parameter."] #[doc = " 3. Before this local type, no generic type parameter of the impl must"] #[doc = "    be reachable through fundamental types."] #[doc = "     - e.g. `impl<T> Trait<LocalType> for Vec<T>` is fine, as `Vec` is not fundamental."] #[doc = "     - while `impl<T> Trait<LocalType> for Box<T>` results in an error, as `T` is"] #[doc = "       reachable through the fundamental type `Box`."] #[doc = " 4. Every type in the local key parameter not known in C, going"] #[doc = "    through the parameter's type tree, must appear only as a subtree of"] #[doc = "    a type local to C, with only fundamental types between the type"] #[doc = "    local to C and the local key parameter."] #[doc = "     - e.g., `Vec<LocalType<T>>>` (or equivalently `Box<Vec<LocalType<T>>>`)"] #[doc = "     is bad, because the only local type with `T` as a subtree is"] #[doc = "     `LocalType<T>`, and `Vec<->` is between it and the type parameter."] #[doc = "     - similarly, `FundamentalPair<LocalType<T>, T>` is bad, because"] #[doc = "     the second occurrence of `T` is not a subtree of *any* local type."] #[doc = "     - however, `LocalType<Vec<T>>` is OK, because `T` is a subtree of"] #[doc = "     `LocalType<Vec<T>>`, which is local and has no types between it and"] #[doc = "     the type parameter."] #[doc = ""] #[doc = " The orphan rules actually serve several different purposes:"] #[doc = ""] #[doc = " 1. They enable link-safety - i.e., 2 mutually-unknowing crates (where"] #[doc = "    every type local to one crate is unknown in the other) can't implement"] #[doc = "    the same trait-ref. This follows because it can be seen that no such"] #[doc = "    type can orphan-check in 2 such crates."] #[doc = ""] #[doc = "    To check that a local impl follows the orphan rules, we check it in"] #[doc = "    InCrate::Local mode, using type parameters for the \"generic\" types."] #[doc = ""] #[doc = "    In InCrate::Local mode the orphan check succeeds if the current crate"] #[doc = "    is definitely allowed to implement the given trait (no false positives)."] #[doc = ""] #[doc = " 2. They ground negative reasoning for coherence. If a user wants to"] #[doc = "    write both a conditional blanket impl and a specific impl, we need to"] #[doc = "    make sure they do not overlap. For example, if we write"] #[doc = "    ```ignore (illustrative)"] #[doc = "    impl<T> IntoIterator for Vec<T>"] #[doc = "    impl<T: Iterator> IntoIterator for T"] #[doc = "    ```"] #[doc = "    We need to be able to prove that `Vec<$0>: !Iterator` for every type $0."] #[doc = "    We can observe that this holds in the current crate, but we need to make"] #[doc = "    sure this will also hold in all unknown crates (both \"independent\" crates,"] #[doc = "    which we need for link-safety, and also child crates, because we don't want"] #[doc = "    child crates to get error for impl conflicts in a *dependency*)."] #[doc = ""] #[doc = "    For that, we only allow negative reasoning if, for every assignment to the"] #[doc = "    inference variables, every unknown crate would get an orphan error if they"] #[doc = "    try to implement this trait-ref. To check for this, we use InCrate::Remote"] #[doc = "    mode. That is sound because we already know all the impls from known crates."] #[doc = ""] #[doc = "    In InCrate::Remote mode the orphan check succeeds if a foreign crate"] #[doc = "    *could* implement the given trait (no false negatives)."] #[doc = ""] #[doc = " 3. For non-`#[fundamental]` traits, they guarantee that parent crates can"] #[doc = "    add \"non-blanket\" impls without breaking negative reasoning in dependent"] #[doc = "    crates. This is the \"rebalancing coherence\" (RFC 1023) restriction."] #[doc = ""] #[doc = "    For that, we only allow a crate to perform negative reasoning on"] #[doc = "    non-local-non-`#[fundamental]` if there's a local key parameter as per (2)."] #[doc = ""] #[doc = "    Because we never perform negative reasoning generically (coherence does"] #[doc = "    not involve type parameters), this can be interpreted as doing the full"] #[doc = "    orphan check (using InCrate::Local mode), instantiating non-local known"] #[doc = "    types for all inference variables."] #[doc = ""] #[doc = "    This allows for crates to future-compatibly add impls as long as they"] #[doc = "    can't apply to types with a key parameter in a child crate - applying"] #[doc = "    the rules, this basically means that every type parameter in the impl"] #[doc = "    must appear behind a non-fundamental type (because this is not a"] #[doc = "    type-system requirement, crate owners might also go for \"semantic"] #[doc = "    future-compatibility\" involving things such as sealed traits, but"] #[doc = "    the above requirement is sufficient, and is necessary in \"open world\""] #[doc = "    cases)."] #[doc = ""] #[doc = " Note that this function is never called for types that have both type"] #[doc = " parameters and inference variables."] #[instrument (level = "trace" , skip (infcx , lazily_normalize_ty) , ret)] pub fn orphan_check_trait_ref < Infcx , I , E : Debug > (infcx : & Infcx , trait_ref : ty :: TraitRef < I > , in_crate : InCrate , lazily_normalize_ty : impl FnMut (I :: Ty) -> Result < I :: Ty , E > ,) -> Result < Result < () , OrphanCheckErr < I , I :: Ty > > , E > where Infcx : InferCtxtLike < Interner = I > , I : Interner , E : Debug , { if trait_ref . has_param () { panic ! ("orphan check only expects inference variables: {trait_ref:?}") ; } let mut checker = OrphanChecker :: new (infcx , in_crate , lazily_normalize_ty) ; Ok (match trait_ref . visit_with (& mut checker) { ControlFlow :: Continue (()) => Err (OrphanCheckErr :: NonLocalInputType (checker . non_local_tys)) , ControlFlow :: Break (residual) => match residual { OrphanCheckEarlyExit :: NormalizationFailure (err) => return Err (err) , OrphanCheckEarlyExit :: UncoveredTyParam (ty) => { checker . search_first_local_ty = true ; let local_ty = match trait_ref . visit_with (& mut checker) { ControlFlow :: Break (OrphanCheckEarlyExit :: LocalTy (local_ty)) => Some (local_ty) , _ => None , } ; Err (OrphanCheckErr :: UncoveredTyParams (UncoveredTyParams { uncovered : ty , local_ty , })) } OrphanCheckEarlyExit :: LocalTy (_) => Ok (()) , } , }) }
}
mkitem!{mkstruct!{struct OrphanChecker < 'a , Infcx , I : Interner , F > { infcx : & 'a Infcx , in_crate : InCrate , in_self_ty : bool , lazily_normalize_ty : F , #[doc = " Ignore orphan check failures and exclusively search for the first local type."] search_first_local_ty : bool , non_local_tys : Vec < (I :: Ty , IsFirstInputType) > , }}}
mkitem!{mkimpl!{impl < 'a , Infcx , I , F , E > OrphanChecker < 'a , Infcx , I , F > where Infcx : InferCtxtLike < Interner = I > , I : Interner , F : FnOnce (I :: Ty) -> Result < I :: Ty , E > , { fn new (infcx : & 'a Infcx , in_crate : InCrate , lazily_normalize_ty : F) -> Self { OrphanChecker { infcx , in_crate , in_self_ty : true , lazily_normalize_ty , search_first_local_ty : false , non_local_tys : Vec :: new () , } } fn found_non_local_ty (& mut self , t : I :: Ty) -> ControlFlow < OrphanCheckEarlyExit < I , E > > { self . non_local_tys . push ((t , self . in_self_ty . into ())) ; ControlFlow :: Continue (()) } fn found_uncovered_ty_param (& mut self , ty : I :: Ty) -> ControlFlow < OrphanCheckEarlyExit < I , E > > { if self . search_first_local_ty { return ControlFlow :: Continue (()) ; } ControlFlow :: Break (OrphanCheckEarlyExit :: UncoveredTyParam (ty)) } fn def_id_is_local (& mut self , def_id : impl DefId < I >) -> bool { match self . in_crate { InCrate :: Local { .. } => def_id . is_local () , InCrate :: Remote => false , } } }}}
mkitem!{mkenum!{enum OrphanCheckEarlyExit < I : Interner , E > { NormalizationFailure (E) , UncoveredTyParam (I :: Ty) , LocalTy (I :: Ty) , }}}
mkitem!{mkimpl!{impl < 'a , Infcx , I , F , E > TypeVisitor < I > for OrphanChecker < 'a , Infcx , I , F > where Infcx : InferCtxtLike < Interner = I > , I : Interner , F : FnMut (I :: Ty) -> Result < I :: Ty , E > , { type Result = ControlFlow < OrphanCheckEarlyExit < I , E > > ; fn visit_region (& mut self , _r : I :: Region) -> Self :: Result { ControlFlow :: Continue (()) } fn visit_ty (& mut self , ty : I :: Ty) -> Self :: Result { let ty = self . infcx . shallow_resolve (ty) ; let ty = match (self . lazily_normalize_ty) (ty) { Ok (norm_ty) if norm_ty . is_ty_var () => ty , Ok (norm_ty) => norm_ty , Err (err) => return ControlFlow :: Break (OrphanCheckEarlyExit :: NormalizationFailure (err)) , } ; let result = match ty . kind () { ty :: Bool | ty :: Char | ty :: Int (..) | ty :: Uint (..) | ty :: Float (..) | ty :: Str | ty :: FnDef (..) | ty :: Pat (..) | ty :: FnPtr (..) | ty :: Array (..) | ty :: Slice (..) | ty :: RawPtr (..) | ty :: Never | ty :: Tuple (..) | ty :: UnsafeBinder (_) => self . found_non_local_ty (ty) , ty :: Param (..) => panic ! ("unexpected ty param") , ty :: Placeholder (..) | ty :: Bound (..) | ty :: Infer (..) => { match self . in_crate { InCrate :: Local { .. } => self . found_uncovered_ty_param (ty) , InCrate :: Remote => ControlFlow :: Break (OrphanCheckEarlyExit :: LocalTy (ty)) , } } ty :: Alias (kind , _) => { if ty . has_type_flags (ty :: TypeFlags :: HAS_TY_PLACEHOLDER | ty :: TypeFlags :: HAS_TY_BOUND | ty :: TypeFlags :: HAS_TY_INFER ,) { match self . in_crate { InCrate :: Local { mode } => match kind { ty :: Projection => { if let OrphanCheckMode :: Compat = mode { ControlFlow :: Continue (()) } else { self . found_uncovered_ty_param (ty) } } _ => self . found_uncovered_ty_param (ty) , } , InCrate :: Remote => { ControlFlow :: Break (OrphanCheckEarlyExit :: LocalTy (ty)) } } } else { self . found_non_local_ty (ty) } } ty :: Ref (_ , ty , _) => ty . visit_with (self) , ty :: Adt (def , args) => { if self . def_id_is_local (def . def_id ()) { ControlFlow :: Break (OrphanCheckEarlyExit :: LocalTy (ty)) } else if def . is_fundamental () { args . visit_with (self) } else { self . found_non_local_ty (ty) } } ty :: Foreign (def_id) => { if self . def_id_is_local (def_id) { ControlFlow :: Break (OrphanCheckEarlyExit :: LocalTy (ty)) } else { self . found_non_local_ty (ty) } } ty :: Dynamic (tt , ..) => { let principal = tt . principal () . map (| p | p . def_id ()) ; if principal . is_some_and (| p | self . def_id_is_local (p)) { ControlFlow :: Break (OrphanCheckEarlyExit :: LocalTy (ty)) } else { self . found_non_local_ty (ty) } } ty :: Error (_) => ControlFlow :: Break (OrphanCheckEarlyExit :: LocalTy (ty)) , ty :: Closure (did , ..) => { if self . def_id_is_local (did) { ControlFlow :: Break (OrphanCheckEarlyExit :: LocalTy (ty)) } else { self . found_non_local_ty (ty) } } ty :: CoroutineClosure (did , ..) => { if self . def_id_is_local (did) { ControlFlow :: Break (OrphanCheckEarlyExit :: LocalTy (ty)) } else { self . found_non_local_ty (ty) } } ty :: Coroutine (did , ..) => { if self . def_id_is_local (did) { ControlFlow :: Break (OrphanCheckEarlyExit :: LocalTy (ty)) } else { self . found_non_local_ty (ty) } } ty :: CoroutineWitness (..) => ControlFlow :: Break (OrphanCheckEarlyExit :: LocalTy (ty)) , } ; self . in_self_ty = false ; result } #[doc = " All possible values for a constant parameter already exist"] #[doc = " in the crate defining the trait, so they are always non-local[^1]."] #[doc = ""] #[doc = " Because there's no way to have an impl where the first local"] #[doc = " generic argument is a constant, we also don't have to fail"] #[doc = " the orphan check when encountering a parameter or a generic constant."] #[doc = ""] #[doc = " This means that we can completely ignore constants during the orphan check."] #[doc = ""] #[doc = " See `tests/ui/coherence/const-generics-orphan-check-ok.rs` for examples."] #[doc = ""] #[doc = " [^1]: This might not hold for function pointers or trait objects in the future."] #[doc = " As these should be quite rare as const arguments and especially rare as impl"] #[doc = " parameters, allowing uncovered const parameters in impls seems more useful"] #[doc = " than allowing `impl<T> Trait<local_fn_ptr, T> for i32` to compile."] fn visit_const (& mut self , _c : I :: Const) -> Self :: Result { ControlFlow :: Continue (()) } }}}