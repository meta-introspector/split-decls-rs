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
mkuse!{use rustc_macros :: HashStable ;}
mkuse!{use smallvec :: SmallVec ;}
mkuse!{use tracing :: instrument ;}
mkuse!{use crate :: ty :: context :: TyCtxt ;}
mkuse!{use crate :: ty :: { self , DefId , OpaqueTypeKey , Ty , TypingEnv } ;}
mkitem!{mkenum!{#[doc = " Represents whether some type is inhabited in a given context."] #[doc = " Examples of uninhabited types are `!`, `enum Void {}`, or a struct"] #[doc = " containing either of those types."] #[doc = " A type's inhabitedness may depend on the `ParamEnv` as well as what types"] #[doc = " are visible in the current module."] #[derive (Clone , Copy , Debug , PartialEq , HashStable)] pub enum InhabitedPredicate < 'tcx > { #[doc = " Inhabited"] True , #[doc = " Uninhabited"] False , #[doc = " Uninhabited when a const value is non-zero. This occurs when there is an"] #[doc = " array of uninhabited items, but the array is inhabited if it is empty."] ConstIsZero (ty :: Const < 'tcx >) , #[doc = " Uninhabited if within a certain module. This occurs when an uninhabited"] #[doc = " type has restricted visibility."] NotInModule (DefId) , #[doc = " Inhabited if some generic type is inhabited."] #[doc = " These are replaced by calling [`Self::instantiate`]."] GenericType (Ty < 'tcx >) , #[doc = " Inhabited if either we don't know the hidden type or we know it and it is inhabited."] OpaqueType (OpaqueTypeKey < 'tcx >) , #[doc = " A AND B"] And (& 'tcx [InhabitedPredicate < 'tcx > ; 2]) , #[doc = " A OR B"] Or (& 'tcx [InhabitedPredicate < 'tcx > ; 2]) , }}}
mkitem!{mkimpl!{impl < 'tcx > InhabitedPredicate < 'tcx > { #[doc = " Returns true if the corresponding type is inhabited in the given `ParamEnv` and module."] pub fn apply (self , tcx : TyCtxt < 'tcx > , typing_env : TypingEnv < 'tcx > , module_def_id : DefId ,) -> bool { self . apply_revealing_opaque (tcx , typing_env , module_def_id , & | _ | None) } #[doc = " Returns true if the corresponding type is inhabited in the given `ParamEnv` and module,"] #[doc = " revealing opaques when possible."] pub fn apply_revealing_opaque (self , tcx : TyCtxt < 'tcx > , typing_env : TypingEnv < 'tcx > , module_def_id : DefId , reveal_opaque : & impl Fn (OpaqueTypeKey < 'tcx >) -> Option < Ty < 'tcx > > ,) -> bool { let Ok (result) = self . apply_inner :: < ! > (tcx , typing_env , & mut Default :: default () , & | id | Ok (tcx . is_descendant_of (module_def_id , id)) , reveal_opaque ,) ; result } #[doc = " Same as `apply`, but returns `None` if self contains a module predicate"] pub fn apply_any_module (self , tcx : TyCtxt < 'tcx > , typing_env : TypingEnv < 'tcx >) -> Option < bool > { self . apply_inner (tcx , typing_env , & mut Default :: default () , & | _ | Err (()) , & | _ | None) . ok () } #[doc = " Same as `apply`, but `NotInModule(_)` predicates yield `false`. That is,"] #[doc = " privately uninhabited types are considered always uninhabited."] pub fn apply_ignore_module (self , tcx : TyCtxt < 'tcx > , typing_env : TypingEnv < 'tcx >) -> bool { let Ok (result) = self . apply_inner :: < ! > (tcx , typing_env , & mut Default :: default () , & | _ | Ok (true) , & | _ | { None }) ; result } #[instrument (level = "debug" , skip (tcx , typing_env , in_module , reveal_opaque) , ret)] fn apply_inner < E : std :: fmt :: Debug > (self , tcx : TyCtxt < 'tcx > , typing_env : TypingEnv < 'tcx > , eval_stack : & mut SmallVec < [Ty < 'tcx > ; 1] > , in_module : & impl Fn (DefId) -> Result < bool , E > , reveal_opaque : & impl Fn (OpaqueTypeKey < 'tcx >) -> Option < Ty < 'tcx > > ,) -> Result < bool , E > { match self { Self :: False => Ok (false) , Self :: True => Ok (true) , Self :: ConstIsZero (const_) => match const_ . try_to_target_usize (tcx) { None | Some (0) => Ok (true) , Some (1 ..) => Ok (false) , } , Self :: NotInModule (id) => in_module (id) . map (| in_mod | ! in_mod) , Self :: GenericType (t) => { let normalized_pred = tcx . try_normalize_erasing_regions (typing_env , t) . map_or (self , | t | t . inhabited_predicate (tcx)) ; match normalized_pred { Self :: GenericType (_) => Ok (true) , pred => { if eval_stack . contains (& t) { return Ok (true) ; } eval_stack . push (t) ; let ret = pred . apply_inner (tcx , typing_env , eval_stack , in_module , reveal_opaque) ; eval_stack . pop () ; ret } } } Self :: OpaqueType (key) => match reveal_opaque (key) { None => Ok (true) , Some (t) => { if eval_stack . contains (& t) { return Ok (true) ; } eval_stack . push (t) ; let ret = t . inhabited_predicate (tcx) . apply_inner (tcx , typing_env , eval_stack , in_module , reveal_opaque ,) ; eval_stack . pop () ; ret } } , Self :: And ([a , b]) => try_and (a , b , | x | { x . apply_inner (tcx , typing_env , eval_stack , in_module , reveal_opaque) }) , Self :: Or ([a , b]) => try_or (a , b , | x | { x . apply_inner (tcx , typing_env , eval_stack , in_module , reveal_opaque) }) , } } pub fn and (self , tcx : TyCtxt < 'tcx > , other : Self) -> Self { self . reduce_and (tcx , other) . unwrap_or_else (| | Self :: And (tcx . arena . alloc ([self , other]))) } pub fn or (self , tcx : TyCtxt < 'tcx > , other : Self) -> Self { self . reduce_or (tcx , other) . unwrap_or_else (| | Self :: Or (tcx . arena . alloc ([self , other]))) } pub fn all (tcx : TyCtxt < 'tcx > , iter : impl IntoIterator < Item = Self >) -> Self { let mut result = Self :: True ; for pred in iter { if matches ! (pred , Self :: False) { return Self :: False ; } result = result . and (tcx , pred) ; } result } pub fn any (tcx : TyCtxt < 'tcx > , iter : impl IntoIterator < Item = Self >) -> Self { let mut result = Self :: False ; for pred in iter { if matches ! (pred , Self :: True) { return Self :: True ; } result = result . or (tcx , pred) ; } result } fn reduce_and (self , tcx : TyCtxt < 'tcx > , other : Self) -> Option < Self > { match (self , other) { (Self :: True , a) | (a , Self :: True) => Some (a) , (Self :: False , _) | (_ , Self :: False) => Some (Self :: False) , (Self :: ConstIsZero (a) , Self :: ConstIsZero (b)) if a == b => Some (Self :: ConstIsZero (a)) , (Self :: NotInModule (a) , Self :: NotInModule (b)) if a == b => Some (Self :: NotInModule (a)) , (Self :: NotInModule (a) , Self :: NotInModule (b)) if tcx . is_descendant_of (a , b) => { Some (Self :: NotInModule (b)) } (Self :: NotInModule (a) , Self :: NotInModule (b)) if tcx . is_descendant_of (b , a) => { Some (Self :: NotInModule (a)) } (Self :: GenericType (a) , Self :: GenericType (b)) if a == b => Some (Self :: GenericType (a)) , (Self :: And (& [a , b]) , c) | (c , Self :: And (& [a , b])) => { if let Some (ac) = a . reduce_and (tcx , c) { Some (ac . and (tcx , b)) } else if let Some (bc) = b . reduce_and (tcx , c) { Some (Self :: And (tcx . arena . alloc ([a , bc]))) } else { None } } _ => None , } } fn reduce_or (self , tcx : TyCtxt < 'tcx > , other : Self) -> Option < Self > { match (self , other) { (Self :: True , _) | (_ , Self :: True) => Some (Self :: True) , (Self :: False , a) | (a , Self :: False) => Some (a) , (Self :: ConstIsZero (a) , Self :: ConstIsZero (b)) if a == b => Some (Self :: ConstIsZero (a)) , (Self :: NotInModule (a) , Self :: NotInModule (b)) if a == b => Some (Self :: NotInModule (a)) , (Self :: NotInModule (a) , Self :: NotInModule (b)) if tcx . is_descendant_of (a , b) => { Some (Self :: NotInModule (a)) } (Self :: NotInModule (a) , Self :: NotInModule (b)) if tcx . is_descendant_of (b , a) => { Some (Self :: NotInModule (b)) } (Self :: GenericType (a) , Self :: GenericType (b)) if a == b => Some (Self :: GenericType (a)) , (Self :: Or (& [a , b]) , c) | (c , Self :: Or (& [a , b])) => { if let Some (ac) = a . reduce_or (tcx , c) { Some (ac . or (tcx , b)) } else if let Some (bc) = b . reduce_or (tcx , c) { Some (Self :: Or (tcx . arena . alloc ([a , bc]))) } else { None } } _ => None , } } #[doc = " Replaces generic types with its corresponding predicate"] pub fn instantiate (self , tcx : TyCtxt < 'tcx > , args : ty :: GenericArgsRef < 'tcx >) -> Self { self . instantiate_opt (tcx , args) . unwrap_or (self) } #[doc = " Same as [`Self::instantiate`], but if there is no generics to"] #[doc = " instantiate, returns `None`. This is useful because it lets us avoid"] #[doc = " allocating a recursive copy of everything when the result is unchanged."] #[doc = ""] #[doc = " Only used to implement `instantiate` itself."] fn instantiate_opt (self , tcx : TyCtxt < 'tcx > , args : ty :: GenericArgsRef < 'tcx >) -> Option < Self > { match self { Self :: ConstIsZero (c) => { let c = ty :: EarlyBinder :: bind (c) . instantiate (tcx , args) ; let pred = match c . try_to_target_usize (tcx) { Some (0) => Self :: True , Some (1 ..) => Self :: False , None => Self :: ConstIsZero (c) , } ; Some (pred) } Self :: GenericType (t) => { Some (ty :: EarlyBinder :: bind (t) . instantiate (tcx , args) . inhabited_predicate (tcx)) } Self :: And (& [a , b]) => match a . instantiate_opt (tcx , args) { None => b . instantiate_opt (tcx , args) . map (| b | a . and (tcx , b)) , Some (InhabitedPredicate :: False) => Some (InhabitedPredicate :: False) , Some (a) => Some (a . and (tcx , b . instantiate_opt (tcx , args) . unwrap_or (b))) , } , Self :: Or (& [a , b]) => match a . instantiate_opt (tcx , args) { None => b . instantiate_opt (tcx , args) . map (| b | a . or (tcx , b)) , Some (InhabitedPredicate :: True) => Some (InhabitedPredicate :: True) , Some (a) => Some (a . or (tcx , b . instantiate_opt (tcx , args) . unwrap_or (b))) , } , Self :: True | Self :: False | Self :: NotInModule (_) => None , Self :: OpaqueType (_) => { bug ! ("unexpected OpaqueType in InhabitedPredicate") ; } } } }}}

macro_rules! try_and_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_and in module {}", module_path!());
    };
}

mkfn!{
    try_and_introspect!();
    fn try_and < T , E > (a : T , b : T , mut f : impl FnMut (T) -> Result < bool , E >) -> Result < bool , E > { let a = f (a) ; if matches ! (a , Ok (false)) { return Ok (false) ; } match (a , f (b)) { (_ , Ok (false)) | (Ok (false) , _) => Ok (false) , (Ok (true) , Ok (true)) => Ok (true) , (Err (e) , _) | (_ , Err (e)) => Err (e) , } }
}

macro_rules! try_or_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_or in module {}", module_path!());
    };
}

mkfn!{
    try_or_introspect!();
    fn try_or < T , E > (a : T , b : T , mut f : impl FnMut (T) -> Result < bool , E >) -> Result < bool , E > { let a = f (a) ; if matches ! (a , Ok (true)) { return Ok (true) ; } match (a , f (b)) { (_ , Ok (true)) | (Ok (true) , _) => Ok (true) , (Ok (false) , Ok (false)) => Ok (false) , (Err (e) , _) | (_ , Err (e)) => Err (e) , } }
}