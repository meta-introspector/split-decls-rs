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
mkuse!{use std :: iter ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexMap ;}
mkuse!{use rustc_errors :: ErrorGuaranteed ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LOCAL_CRATE } ;}
mkuse!{use rustc_macros :: { Decodable , Encodable , HashStable } ;}
mkuse!{use rustc_span :: symbol :: sym ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: query :: LocalCrate ;}
mkuse!{use crate :: traits :: specialization_graph ;}
mkuse!{use crate :: ty :: fast_reject :: { self , SimplifiedType , TreatParams } ;}
mkuse!{use crate :: ty :: { Ident , Ty , TyCtxt } ;}
mkitem!{mkstruct!{#[doc = " A trait's definition with type information."] #[derive (HashStable , Encodable , Decodable)] pub struct TraitDef { pub def_id : DefId , pub safety : hir :: Safety , #[doc = " Whether this trait is `const`."] pub constness : hir :: Constness , #[doc = " If `true`, then this trait had the `#[rustc_paren_sugar]`"] #[doc = " attribute, indicating that it should be used with `Foo()`"] #[doc = " sugar. This is a temporary thing -- eventually any trait will"] #[doc = " be usable with the sugar (or without it)."] pub paren_sugar : bool , pub has_auto_impl : bool , #[doc = " If `true`, then this trait has the `#[marker]` attribute, indicating"] #[doc = " that all its associated items have defaults that cannot be overridden,"] #[doc = " and thus `impl`s of it are allowed to overlap."] pub is_marker : bool , #[doc = " If `true`, then this trait has the `#[rustc_coinductive]` attribute or"] #[doc = " is an auto trait. This indicates that trait solver cycles involving an"] #[doc = " `X: ThisTrait` goal are accepted."] #[doc = ""] #[doc = " In the future all traits should be coinductive, but we need a better"] #[doc = " formal understanding of what exactly that means and should probably"] #[doc = " also have already switched to the new trait solver."] pub is_coinductive : bool , #[doc = " If `true`, then this trait has the `#[fundamental]` attribute. This"] #[doc = " affects how conherence computes whether a trait may have trait implementations"] #[doc = " added in the future."] pub is_fundamental : bool , #[doc = " If `true`, then this trait has the `#[rustc_skip_during_method_dispatch(array)]`"] #[doc = " attribute, indicating that editions before 2021 should not consider this trait"] #[doc = " during method dispatch if the receiver is an array."] pub skip_array_during_method_dispatch : bool , #[doc = " If `true`, then this trait has the `#[rustc_skip_during_method_dispatch(boxed_slice)]`"] #[doc = " attribute, indicating that editions before 2024 should not consider this trait"] #[doc = " during method dispatch if the receiver is a boxed slice."] pub skip_boxed_slice_during_method_dispatch : bool , #[doc = " Used to determine whether the standard library is allowed to specialize"] #[doc = " on this trait."] pub specialization_kind : TraitSpecializationKind , #[doc = " List of functions from `#[rustc_must_implement_one_of]` attribute one of which"] #[doc = " must be implemented."] pub must_implement_one_of : Option < Box < [Ident] > > , #[doc = " Whether to add a builtin `dyn Trait: Trait` implementation."] #[doc = " This is enabled for all traits except ones marked with"] #[doc = " `#[rustc_do_not_implement_via_object]`."] pub implement_via_object : bool , #[doc = " Whether a trait is fully built-in, and any implementation is disallowed."] #[doc = " This only applies to built-in traits, and is marked via"] #[doc = " `#[rustc_deny_explicit_impl]`."] pub deny_explicit_impl : bool , }}}
mkitem!{mkenum!{#[doc = " Whether this trait is treated specially by the standard library"] #[doc = " specialization lint."] #[derive (HashStable , PartialEq , Clone , Copy , Encodable , Decodable)] pub enum TraitSpecializationKind { #[doc = " The default. Specializing on this trait is not allowed."] None , #[doc = " Specializing on this trait is allowed because it doesn't have any"] #[doc = " methods. For example `Sized` or `FusedIterator`."] #[doc = " Applies to traits with the `rustc_unsafe_specialization_marker`"] #[doc = " attribute."] Marker , #[doc = " Specializing on this trait is allowed because all of the impls of this"] #[doc = " trait are \"always applicable\". Always applicable means that if"] #[doc = " `X<'x>: T<'y>` for any lifetimes, then `for<'a, 'b> X<'a>: T<'b>`."] #[doc = " Applies to traits with the `rustc_specialization_trait` attribute."] AlwaysApplicable , }}}
mkitem!{mkstruct!{#[derive (Default , Debug , HashStable)] pub struct TraitImpls { blanket_impls : Vec < DefId > , #[doc = " Impls indexed by their simplified self type, for fast lookup."] non_blanket_impls : FxIndexMap < SimplifiedType , Vec < DefId > > , }}}
mkitem!{mkimpl!{impl TraitImpls { pub fn is_empty (& self) -> bool { self . blanket_impls . is_empty () && self . non_blanket_impls . is_empty () } pub fn blanket_impls (& self) -> & [DefId] { self . blanket_impls . as_slice () } pub fn non_blanket_impls (& self) -> & FxIndexMap < SimplifiedType , Vec < DefId > > { & self . non_blanket_impls } }}}
mkitem!{mkimpl!{impl < 'tcx > TraitDef { pub fn ancestors (& self , tcx : TyCtxt < 'tcx > , of_impl : DefId ,) -> Result < specialization_graph :: Ancestors < 'tcx > , ErrorGuaranteed > { specialization_graph :: ancestors (tcx , self . def_id , of_impl) } }}}
mkitem!{mkimpl!{impl < 'tcx > TyCtxt < 'tcx > { #[doc = " Iterate over every impl that could possibly match the self type `self_ty`."] #[doc = ""] #[doc = " `trait_def_id` MUST BE the `DefId` of a trait."] pub fn for_each_relevant_impl (self , trait_def_id : DefId , self_ty : Ty < 'tcx > , mut f : impl FnMut (DefId) ,) { let impls = self . trait_impls_of (trait_def_id) ; for & impl_def_id in impls . blanket_impls . iter () { f (impl_def_id) ; } if let Some (simp) = fast_reject :: simplify_type (self , self_ty , TreatParams :: AsRigid) { if let Some (impls) = impls . non_blanket_impls . get (& simp) { for & impl_def_id in impls { f (impl_def_id) ; } } } else { for & impl_def_id in impls . non_blanket_impls . values () . flatten () { f (impl_def_id) ; } } } #[doc = " `trait_def_id` MUST BE the `DefId` of a trait."] pub fn non_blanket_impls_for_ty (self , trait_def_id : DefId , self_ty : Ty < 'tcx > ,) -> impl Iterator < Item = DefId > { let impls = self . trait_impls_of (trait_def_id) ; if let Some (simp) = fast_reject :: simplify_type (self , self_ty , TreatParams :: InstantiateWithInfer) { if let Some (impls) = impls . non_blanket_impls . get (& simp) { return impls . iter () . copied () ; } } [] . iter () . copied () } #[doc = " Returns an iterator containing all impls for `trait_def_id`."] #[doc = ""] #[doc = " `trait_def_id` MUST BE the `DefId` of a trait."] pub fn all_impls (self , trait_def_id : DefId) -> impl Iterator < Item = DefId > { let TraitImpls { blanket_impls , non_blanket_impls } = self . trait_impls_of (trait_def_id) ; blanket_impls . iter () . chain (non_blanket_impls . iter () . flat_map (| (_ , v) | v)) . cloned () } }}}

macro_rules! trait_impls_of_provider_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trait_impls_of_provider in module {}", module_path!());
    };
}

mkfn!{
    trait_impls_of_provider_introspect!();
    #[doc = " Query provider for `trait_impls_of`."] pub (super) fn trait_impls_of_provider (tcx : TyCtxt < '_ > , trait_id : DefId) -> TraitImpls { let mut impls = TraitImpls :: default () ; if ! trait_id . is_local () { for & cnum in tcx . crates (()) . iter () { for & (impl_def_id , simplified_self_ty) in tcx . implementations_of_trait ((cnum , trait_id)) . iter () { if let Some (simplified_self_ty) = simplified_self_ty { impls . non_blanket_impls . entry (simplified_self_ty) . or_default () . push (impl_def_id) ; } else { impls . blanket_impls . push (impl_def_id) ; } } } } for & impl_def_id in tcx . local_trait_impls (trait_id) { let impl_def_id = impl_def_id . to_def_id () ; let impl_self_ty = tcx . type_of (impl_def_id) . instantiate_identity () ; if let Some (simplified_self_ty) = fast_reject :: simplify_type (tcx , impl_self_ty , TreatParams :: InstantiateWithInfer) { impls . non_blanket_impls . entry (simplified_self_ty) . or_default () . push (impl_def_id) ; } else { impls . blanket_impls . push (impl_def_id) ; } } impls }
}

macro_rules! incoherent_impls_provider_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function incoherent_impls_provider in module {}", module_path!());
    };
}

mkfn!{
    incoherent_impls_provider_introspect!();
    #[doc = " Query provider for `incoherent_impls`."] pub (super) fn incoherent_impls_provider (tcx : TyCtxt < '_ > , simp : SimplifiedType) -> & [DefId] { if let Some (def_id) = simp . def () && ! tcx . has_attr (def_id , sym :: rustc_has_incoherent_inherent_impls) { return & [] ; } let mut impls = Vec :: new () ; for cnum in iter :: once (LOCAL_CRATE) . chain (tcx . crates (()) . iter () . copied ()) { for & impl_def_id in tcx . crate_incoherent_impls ((cnum , simp)) { impls . push (impl_def_id) } } debug ! (? impls) ; tcx . arena . alloc_slice (& impls) }
}

macro_rules! traits_provider_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function traits_provider in module {}", module_path!());
    };
}

mkfn!{
    traits_provider_introspect!();
    pub (super) fn traits_provider (tcx : TyCtxt < '_ > , _ : LocalCrate) -> & [DefId] { let mut traits = Vec :: new () ; for id in tcx . hir_free_items () { if matches ! (tcx . def_kind (id . owner_id) , DefKind :: Trait | DefKind :: TraitAlias) { traits . push (id . owner_id . to_def_id ()) } } tcx . arena . alloc_slice (& traits) }
}

macro_rules! trait_impls_in_crate_provider_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trait_impls_in_crate_provider in module {}", module_path!());
    };
}

mkfn!{
    trait_impls_in_crate_provider_introspect!();
    pub (super) fn trait_impls_in_crate_provider (tcx : TyCtxt < '_ > , _ : LocalCrate) -> & [DefId] { let mut trait_impls = Vec :: new () ; for id in tcx . hir_free_items () { if matches ! (tcx . def_kind (id . owner_id) , DefKind :: Impl { .. }) && tcx . impl_trait_ref (id . owner_id) . is_some () { trait_impls . push (id . owner_id . to_def_id ()) } } tcx . arena . alloc_slice (& trait_impls) }
}