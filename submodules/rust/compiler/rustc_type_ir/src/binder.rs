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
mkuse!{use std :: marker :: PhantomData ;}
mkuse!{use std :: ops :: { ControlFlow , Deref } ;}
mkuse!{use derive_where :: derive_where ;}
mkuse!{# [cfg (feature = "nightly")] use rustc_macros :: { Decodable_NoContext , Encodable_NoContext , HashStable_NoContext } ;}
mkuse!{use tracing :: instrument ;}
mkuse!{use crate :: data_structures :: SsoHashSet ;}
mkuse!{use crate :: fold :: { FallibleTypeFolder , TypeFoldable , TypeFolder , TypeSuperFoldable } ;}
mkuse!{use crate :: inherent :: * ;}
mkuse!{use crate :: lift :: Lift ;}
mkuse!{use crate :: visit :: { Flags , TypeSuperVisitable , TypeVisitable , TypeVisitableExt , TypeVisitor } ;}
mkuse!{use crate :: { self as ty , Interner } ;}
mkitem!{mkstruct!{# [doc = " `Binder` is a binder for higher-ranked lifetimes or types. It is part of the"] # [doc = " compiler's representation for things like `for<'a> Fn(&'a isize)`"] # [doc = " (which would be represented by the type `PolyTraitRef == Binder<I, TraitRef>`)."] # [doc = ""] # [doc = " See <https://rustc-dev-guide.rust-lang.org/ty_module/instantiating_binders.html>"] # [doc = " for more details."] # [doc = ""] # [doc = " `Decodable` and `Encodable` are implemented for `Binder<T>` using the `impl_binder_encode_decode!` macro."] # [derive_where (Clone , Hash , PartialEq , Debug ; I : Interner , T)] # [derive_where (Copy ; I : Interner , T : Copy)] # [cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] pub struct Binder < I : Interner , T > { value : T , bound_vars : I :: BoundVarKinds , }}}
mkitem!{mkimpl!{impl < I : Interner , T : Eq > Eq for Binder < I , T > { }}}
mkitem!{mkimpl!{impl < I : Interner , U : Interner , T > Lift < U > for Binder < I , T > where T : Lift < U > , I :: BoundVarKinds : Lift < U , Lifted = U :: BoundVarKinds > , { type Lifted = Binder < U , T :: Lifted > ; fn lift_to_interner (self , cx : U) -> Option < Self :: Lifted > { Some (Binder { value : self . value . lift_to_interner (cx) ? , bound_vars : self . bound_vars . lift_to_interner (cx) ? , }) } }}}
mkitem!{# [cfg (feature = "nightly")] macro_rules ! impl_binder_encode_decode { ($ ($ t : ty) ,+ $ (,) ?) => { $ (impl < I : Interner , E : rustc_serialize :: Encoder > rustc_serialize :: Encodable < E > for ty :: Binder < I , $ t > where $ t : rustc_serialize :: Encodable < E >, I :: BoundVarKinds : rustc_serialize :: Encodable < E >, { fn encode (& self , e : & mut E) { self . bound_vars () . encode (e) ; self . as_ref () . skip_binder () . encode (e) ; } } impl < I : Interner , D : rustc_serialize :: Decoder > rustc_serialize :: Decodable < D > for ty :: Binder < I , $ t > where $ t : TypeVisitable < I > + rustc_serialize :: Decodable < D >, I :: BoundVarKinds : rustc_serialize :: Decodable < D >, { fn decode (decoder : & mut D) -> Self { let bound_vars = rustc_serialize :: Decodable :: decode (decoder) ; ty :: Binder :: bind_with_vars (rustc_serialize :: Decodable :: decode (decoder) , bound_vars) } }) * } }}
mkitem!{# [cfg (feature = "nightly")] impl_binder_encode_decode ! { ty :: FnSig < I >, ty :: FnSigTys < I >, ty :: TraitPredicate < I >, ty :: ExistentialPredicate < I >, ty :: TraitRef < I >, ty :: ExistentialTraitRef < I >, ty :: HostEffectPredicate < I >, }}
mkitem!{mkimpl!{impl < I : Interner , T > Binder < I , T > where T : TypeVisitable < I > , { # [doc = " Wraps `value` in a binder, asserting that `value` does not"] # [doc = " contain any bound vars that would be bound by the"] # [doc = " binder. This is commonly used to 'inject' a value T into a"] # [doc = " different binding level."] # [track_caller] pub fn dummy (value : T) -> Binder < I , T > { assert ! (! value . has_escaping_bound_vars () , "`{value:?}` has escaping bound vars, so it cannot be wrapped in a dummy binder.") ; Binder { value , bound_vars : Default :: default () } } pub fn bind_with_vars (value : T , bound_vars : I :: BoundVarKinds) -> Binder < I , T > { if cfg ! (debug_assertions) { let mut validator = ValidateBoundVars :: new (bound_vars) ; let _ = value . visit_with (& mut validator) ; } Binder { value , bound_vars } } }}}
mkitem!{mkimpl!{impl < I : Interner , T : TypeFoldable < I > > TypeFoldable < I > for Binder < I , T > { fn try_fold_with < F : FallibleTypeFolder < I > > (self , folder : & mut F) -> Result < Self , F :: Error > { folder . try_fold_binder (self) } fn fold_with < F : TypeFolder < I > > (self , folder : & mut F) -> Self { folder . fold_binder (self) } }}}
mkitem!{mkimpl!{impl < I : Interner , T : TypeVisitable < I > > TypeVisitable < I > for Binder < I , T > { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { visitor . visit_binder (self) } }}}
mkitem!{mkimpl!{impl < I : Interner , T : TypeFoldable < I > > TypeSuperFoldable < I > for Binder < I , T > { fn try_super_fold_with < F : FallibleTypeFolder < I > > (self , folder : & mut F ,) -> Result < Self , F :: Error > { self . try_map_bound (| t | t . try_fold_with (folder)) } fn super_fold_with < F : TypeFolder < I > > (self , folder : & mut F) -> Self { self . map_bound (| t | t . fold_with (folder)) } }}}
mkitem!{mkimpl!{impl < I : Interner , T : TypeVisitable < I > > TypeSuperVisitable < I > for Binder < I , T > { fn super_visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { self . as_ref () . skip_binder () . visit_with (visitor) } }}}
mkitem!{mkimpl!{impl < I : Interner , T > Binder < I , T > { # [doc = " Returns the value contained inside of this `for<'a>`. Accessing generic args"] # [doc = " in the returned value is generally incorrect."] # [doc = ""] # [doc = " Please read <https://rustc-dev-guide.rust-lang.org/ty_module/instantiating_binders.html>"] # [doc = " before using this function. It is usually better to discharge the binder using"] # [doc = " `no_bound_vars` or `instantiate_bound_regions` or something like that."] # [doc = ""] # [doc = " `skip_binder` is only valid when you are either extracting data that does not reference"] # [doc = " any generic arguments, e.g. a `DefId`, or when you're making sure you only pass the"] # [doc = " value to things which can handle escaping bound vars."] # [doc = ""] # [doc = " See existing uses of `.skip_binder()` in `rustc_trait_selection::traits::select`"] # [doc = " or `rustc_next_trait_solver` for examples."] pub fn skip_binder (self) -> T { self . value } pub fn bound_vars (& self) -> I :: BoundVarKinds { self . bound_vars } pub fn as_ref (& self) -> Binder < I , & T > { Binder { value : & self . value , bound_vars : self . bound_vars } } pub fn as_deref (& self) -> Binder < I , & T :: Target > where T : Deref , { Binder { value : & self . value , bound_vars : self . bound_vars } } pub fn map_bound_ref < F , U : TypeVisitable < I > > (& self , f : F) -> Binder < I , U > where F : FnOnce (& T) -> U , { self . as_ref () . map_bound (f) } pub fn map_bound < F , U : TypeVisitable < I > > (self , f : F) -> Binder < I , U > where F : FnOnce (T) -> U , { let Binder { value , bound_vars } = self ; let value = f (value) ; if cfg ! (debug_assertions) { let mut validator = ValidateBoundVars :: new (bound_vars) ; let _ = value . visit_with (& mut validator) ; } Binder { value , bound_vars } } pub fn try_map_bound < F , U : TypeVisitable < I > , E > (self , f : F) -> Result < Binder < I , U > , E > where F : FnOnce (T) -> Result < U , E > , { let Binder { value , bound_vars } = self ; let value = f (value) ? ; if cfg ! (debug_assertions) { let mut validator = ValidateBoundVars :: new (bound_vars) ; let _ = value . visit_with (& mut validator) ; } Ok (Binder { value , bound_vars }) } # [doc = " Wraps a `value` in a binder, using the same bound variables as the"] # [doc = " current `Binder`. This should not be used if the new value *changes*"] # [doc = " the bound variables. Note: the (old or new) value itself does not"] # [doc = " necessarily need to *name* all the bound variables."] # [doc = ""] # [doc = " This currently doesn't do anything different than `bind`, because we"] # [doc = " don't actually track bound vars. However, semantically, it is different"] # [doc = " because bound vars aren't allowed to change here, whereas they are"] # [doc = " in `bind`. This may be (debug) asserted in the future."] pub fn rebind < U > (& self , value : U) -> Binder < I , U > where U : TypeVisitable < I > , { Binder :: bind_with_vars (value , self . bound_vars) } # [doc = " Unwraps and returns the value within, but only if it contains"] # [doc = " no bound vars at all. (In other words, if this binder --"] # [doc = " and indeed any enclosing binder -- doesn't bind anything at"] # [doc = " all.) Otherwise, returns `None`."] # [doc = ""] # [doc = " (One could imagine having a method that just unwraps a single"] # [doc = " binder, but permits late-bound vars bound by enclosing"] # [doc = " binders, but that would require adjusting the debruijn"] # [doc = " indices, and given the shallow binding structure we often use,"] # [doc = " would not be that useful.)"] pub fn no_bound_vars (self) -> Option < T > where T : TypeVisitable < I > , { if self . value . has_escaping_bound_vars () { None } else { Some (self . skip_binder ()) } } }}}
mkitem!{mkimpl!{impl < I : Interner , T > Binder < I , Option < T > > { pub fn transpose (self) -> Option < Binder < I , T > > { let Binder { value , bound_vars } = self ; value . map (| value | Binder { value , bound_vars }) } }}}
mkitem!{mkimpl!{impl < I : Interner , T : IntoIterator > Binder < I , T > { pub fn iter (self) -> impl Iterator < Item = Binder < I , T :: Item > > { let Binder { value , bound_vars } = self ; value . into_iter () . map (move | value | Binder { value , bound_vars }) } }}}
mkitem!{mkstruct!{pub struct ValidateBoundVars < I : Interner > { bound_vars : I :: BoundVarKinds , binder_index : ty :: DebruijnIndex , visited : SsoHashSet < (ty :: DebruijnIndex , I :: Ty) > , }}}
mkitem!{mkimpl!{impl < I : Interner > ValidateBoundVars < I > { pub fn new (bound_vars : I :: BoundVarKinds) -> Self { ValidateBoundVars { bound_vars , binder_index : ty :: INNERMOST , visited : SsoHashSet :: default () , } } }}}
mkitem!{mkimpl!{impl < I : Interner > TypeVisitor < I > for ValidateBoundVars < I > { type Result = ControlFlow < () > ; fn visit_binder < T : TypeVisitable < I > > (& mut self , t : & Binder < I , T >) -> Self :: Result { self . binder_index . shift_in (1) ; let result = t . super_visit_with (self) ; self . binder_index . shift_out (1) ; result } fn visit_ty (& mut self , t : I :: Ty) -> Self :: Result { if t . outer_exclusive_binder () < self . binder_index || ! self . visited . insert ((self . binder_index , t)) { return ControlFlow :: Break (()) ; } match t . kind () { ty :: Bound (debruijn , bound_ty) if debruijn == self . binder_index => { let idx = bound_ty . var () . as_usize () ; if self . bound_vars . len () <= idx { panic ! ("Not enough bound vars: {:?} not found in {:?}" , t , self . bound_vars) ; } bound_ty . assert_eq (self . bound_vars . get (idx) . unwrap ()) ; } _ => { } } ; t . super_visit_with (self) } fn visit_const (& mut self , c : I :: Const) -> Self :: Result { if c . outer_exclusive_binder () < self . binder_index { return ControlFlow :: Break (()) ; } match c . kind () { ty :: ConstKind :: Bound (debruijn , bound_const) if debruijn == self . binder_index => { let idx = bound_const . var () . as_usize () ; if self . bound_vars . len () <= idx { panic ! ("Not enough bound vars: {:?} not found in {:?}" , c , self . bound_vars) ; } bound_const . assert_eq (self . bound_vars . get (idx) . unwrap ()) ; } _ => { } } ; c . super_visit_with (self) } fn visit_region (& mut self , r : I :: Region) -> Self :: Result { match r . kind () { ty :: ReBound (index , br) if index == self . binder_index => { let idx = br . var () . as_usize () ; if self . bound_vars . len () <= idx { panic ! ("Not enough bound vars: {:?} not found in {:?}" , r , self . bound_vars) ; } br . assert_eq (self . bound_vars . get (idx) . unwrap ()) ; } _ => () , } ; ControlFlow :: Continue (()) } }}}
mkitem!{mkstruct!{# [doc = " Similar to [`Binder`] except that it tracks early bound generics, i.e. `struct Foo<T>(T)`"] # [doc = " needs `T` instantiated immediately. This type primarily exists to avoid forgetting to call"] # [doc = " `instantiate`."] # [doc = ""] # [doc = " See <https://rustc-dev-guide.rust-lang.org/ty_module/early_binder.html> for more details."] # [derive_where (Clone , PartialEq , Ord , Hash , Debug ; I : Interner , T)] # [derive_where (PartialOrd ; I : Interner , T : Ord)] # [derive_where (Copy ; I : Interner , T : Copy)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub struct EarlyBinder < I : Interner , T > { value : T , # [derive_where (skip (Debug))] _tcx : PhantomData < fn () -> I > , }}}
mkitem!{mkimpl!{impl < I : Interner , T : Eq > Eq for EarlyBinder < I , T > { }}}
mkitem!{mkimpl!{# [doc = " For early binders, you should first call `instantiate` before using any visitors."] # [cfg (feature = "nightly")] impl < I : Interner , T > ! TypeFoldable < I > for ty :: EarlyBinder < I , T > { }}}
mkitem!{mkimpl!{# [doc = " For early binders, you should first call `instantiate` before using any visitors."] # [cfg (feature = "nightly")] impl < I : Interner , T > ! TypeVisitable < I > for ty :: EarlyBinder < I , T > { }}}
mkitem!{mkimpl!{impl < I : Interner , T > EarlyBinder < I , T > { pub fn bind (value : T) -> EarlyBinder < I , T > { EarlyBinder { value , _tcx : PhantomData } } pub fn as_ref (& self) -> EarlyBinder < I , & T > { EarlyBinder { value : & self . value , _tcx : PhantomData } } pub fn map_bound_ref < F , U > (& self , f : F) -> EarlyBinder < I , U > where F : FnOnce (& T) -> U , { self . as_ref () . map_bound (f) } pub fn map_bound < F , U > (self , f : F) -> EarlyBinder < I , U > where F : FnOnce (T) -> U , { let value = f (self . value) ; EarlyBinder { value , _tcx : PhantomData } } pub fn try_map_bound < F , U , E > (self , f : F) -> Result < EarlyBinder < I , U > , E > where F : FnOnce (T) -> Result < U , E > , { let value = f (self . value) ? ; Ok (EarlyBinder { value , _tcx : PhantomData }) } pub fn rebind < U > (& self , value : U) -> EarlyBinder < I , U > { EarlyBinder { value , _tcx : PhantomData } } # [doc = " Skips the binder and returns the \"bound\" value. Accessing generic args"] # [doc = " in the returned value is generally incorrect."] # [doc = ""] # [doc = " Please read <https://rustc-dev-guide.rust-lang.org/ty_module/early_binder.html>"] # [doc = " before using this function."] # [doc = ""] # [doc = " Only use this to extract data that does not depend on generic parameters, e.g."] # [doc = " to get the `DefId` of the inner value or the number of arguments ofan `FnSig`,"] # [doc = " or while making sure to only pass the value to functions which are explicitly"] # [doc = " set up to handle these uninstantiated generic parameters."] # [doc = ""] # [doc = " To skip the binder on `x: &EarlyBinder<I, T>` to obtain `&T`, leverage"] # [doc = " [`EarlyBinder::as_ref`](EarlyBinder::as_ref): `x.as_ref().skip_binder()`."] # [doc = ""] # [doc = " See also [`Binder::skip_binder`](Binder::skip_binder), which is"] # [doc = " the analogous operation on [`Binder`]."] pub fn skip_binder (self) -> T { self . value } }}}
mkitem!{mkimpl!{impl < I : Interner , T > EarlyBinder < I , Option < T > > { pub fn transpose (self) -> Option < EarlyBinder < I , T > > { self . value . map (| value | EarlyBinder { value , _tcx : PhantomData }) } }}}
mkitem!{mkimpl!{impl < I : Interner , Iter : IntoIterator > EarlyBinder < I , Iter > where Iter :: Item : TypeFoldable < I > , { pub fn iter_instantiated < A > (self , cx : I , args : A) -> IterInstantiated < I , Iter , A > where A : SliceLike < Item = I :: GenericArg > , { IterInstantiated { it : self . value . into_iter () , cx , args } } # [doc = " Similar to [`instantiate_identity`](EarlyBinder::instantiate_identity),"] # [doc = " but on an iterator of `TypeFoldable` values."] pub fn iter_identity (self) -> Iter :: IntoIter { self . value . into_iter () } }}}
mkitem!{mkstruct!{pub struct IterInstantiated < I : Interner , Iter : IntoIterator , A > { it : Iter :: IntoIter , cx : I , args : A , }}}
mkitem!{mkimpl!{impl < I : Interner , Iter : IntoIterator , A > Iterator for IterInstantiated < I , Iter , A > where Iter :: Item : TypeFoldable < I > , A : SliceLike < Item = I :: GenericArg > , { type Item = Iter :: Item ; fn next (& mut self) -> Option < Self :: Item > { Some (EarlyBinder { value : self . it . next () ? , _tcx : PhantomData } . instantiate (self . cx , self . args) ,) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }}}
mkitem!{mkimpl!{impl < I : Interner , Iter : IntoIterator , A > DoubleEndedIterator for IterInstantiated < I , Iter , A > where Iter :: IntoIter : DoubleEndedIterator , Iter :: Item : TypeFoldable < I > , A : SliceLike < Item = I :: GenericArg > , { fn next_back (& mut self) -> Option < Self :: Item > { Some (EarlyBinder { value : self . it . next_back () ? , _tcx : PhantomData } . instantiate (self . cx , self . args) ,) } }}}
mkitem!{mkimpl!{impl < I : Interner , Iter : IntoIterator , A > ExactSizeIterator for IterInstantiated < I , Iter , A > where Iter :: IntoIter : ExactSizeIterator , Iter :: Item : TypeFoldable < I > , A : SliceLike < Item = I :: GenericArg > , { }}}
mkitem!{mkimpl!{impl < 's , I : Interner , Iter : IntoIterator > EarlyBinder < I , Iter > where Iter :: Item : Deref , < Iter :: Item as Deref > :: Target : Copy + TypeFoldable < I > , { pub fn iter_instantiated_copied (self , cx : I , args : & 's [I :: GenericArg] ,) -> IterInstantiatedCopied < 's , I , Iter > { IterInstantiatedCopied { it : self . value . into_iter () , cx , args } } # [doc = " Similar to [`instantiate_identity`](EarlyBinder::instantiate_identity),"] # [doc = " but on an iterator of values that deref to a `TypeFoldable`."] pub fn iter_identity_copied (self) -> IterIdentityCopied < Iter > { IterIdentityCopied { it : self . value . into_iter () } } }}}
mkitem!{mkstruct!{pub struct IterInstantiatedCopied < 'a , I : Interner , Iter : IntoIterator > { it : Iter :: IntoIter , cx : I , args : & 'a [I :: GenericArg] , }}}
mkitem!{mkimpl!{impl < I : Interner , Iter : IntoIterator > Iterator for IterInstantiatedCopied < '_ , I , Iter > where Iter :: Item : Deref , < Iter :: Item as Deref > :: Target : Copy + TypeFoldable < I > , { type Item = < Iter :: Item as Deref > :: Target ; fn next (& mut self) -> Option < Self :: Item > { self . it . next () . map (| value | { EarlyBinder { value : * value , _tcx : PhantomData } . instantiate (self . cx , self . args) }) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }}}
mkitem!{mkimpl!{impl < I : Interner , Iter : IntoIterator > DoubleEndedIterator for IterInstantiatedCopied < '_ , I , Iter > where Iter :: IntoIter : DoubleEndedIterator , Iter :: Item : Deref , < Iter :: Item as Deref > :: Target : Copy + TypeFoldable < I > , { fn next_back (& mut self) -> Option < Self :: Item > { self . it . next_back () . map (| value | { EarlyBinder { value : * value , _tcx : PhantomData } . instantiate (self . cx , self . args) }) } }}}
mkitem!{mkimpl!{impl < I : Interner , Iter : IntoIterator > ExactSizeIterator for IterInstantiatedCopied < '_ , I , Iter > where Iter :: IntoIter : ExactSizeIterator , Iter :: Item : Deref , < Iter :: Item as Deref > :: Target : Copy + TypeFoldable < I > , { }}}
mkitem!{mkstruct!{pub struct IterIdentityCopied < Iter : IntoIterator > { it : Iter :: IntoIter , }}}
mkitem!{mkimpl!{impl < Iter : IntoIterator > Iterator for IterIdentityCopied < Iter > where Iter :: Item : Deref , < Iter :: Item as Deref > :: Target : Copy , { type Item = < Iter :: Item as Deref > :: Target ; fn next (& mut self) -> Option < Self :: Item > { self . it . next () . map (| i | * i) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }}}
mkitem!{mkimpl!{impl < Iter : IntoIterator > DoubleEndedIterator for IterIdentityCopied < Iter > where Iter :: IntoIter : DoubleEndedIterator , Iter :: Item : Deref , < Iter :: Item as Deref > :: Target : Copy , { fn next_back (& mut self) -> Option < Self :: Item > { self . it . next_back () . map (| i | * i) } }}}
mkitem!{mkimpl!{impl < Iter : IntoIterator > ExactSizeIterator for IterIdentityCopied < Iter > where Iter :: IntoIter : ExactSizeIterator , Iter :: Item : Deref , < Iter :: Item as Deref > :: Target : Copy , { }}}
mkitem!{mkstruct!{pub struct EarlyBinderIter < I , T > { t : T , _tcx : PhantomData < I > , }}}
mkitem!{mkimpl!{impl < I : Interner , T : IntoIterator > EarlyBinder < I , T > { pub fn transpose_iter (self) -> EarlyBinderIter < I , T :: IntoIter > { EarlyBinderIter { t : self . value . into_iter () , _tcx : PhantomData } } }}}
mkitem!{mkimpl!{impl < I : Interner , T : Iterator > Iterator for EarlyBinderIter < I , T > { type Item = EarlyBinder < I , T :: Item > ; fn next (& mut self) -> Option < Self :: Item > { self . t . next () . map (| value | EarlyBinder { value , _tcx : PhantomData }) } fn size_hint (& self) -> (usize , Option < usize >) { self . t . size_hint () } }}}
mkitem!{mkimpl!{impl < I : Interner , T : TypeFoldable < I > > ty :: EarlyBinder < I , T > { pub fn instantiate < A > (self , cx : I , args : A) -> T where A : SliceLike < Item = I :: GenericArg > , { if args . is_empty () { assert ! (! self . value . has_param () , "{:?} has parameters, but no args were provided in instantiate" , self . value ,) ; return self . value ; } let mut folder = ArgFolder { cx , args : args . as_slice () , binders_passed : 0 } ; self . value . fold_with (& mut folder) } # [doc = " Makes the identity replacement `T0 => T0, ..., TN => TN`."] # [doc = " Conceptually, this converts universally bound variables into placeholders"] # [doc = " when inside of a given item."] # [doc = ""] # [doc = " For example, consider `for<T> fn foo<T>(){ .. }`:"] # [doc = " - Outside of `foo`, `T` is bound (represented by the presence of `EarlyBinder`)."] # [doc = " - Inside of the body of `foo`, we treat `T` as a placeholder by calling"] # [doc = " `instantiate_identity` to discharge the `EarlyBinder`."] pub fn instantiate_identity (self) -> T { self . value } # [doc = " Returns the inner value, but only if it contains no bound vars."] pub fn no_bound_vars (self) -> Option < T > { if ! self . value . has_param () { Some (self . value) } else { None } } }}}
mkitem!{mkstruct!{struct ArgFolder < 'a , I : Interner > { cx : I , args : & 'a [I :: GenericArg] , # [doc = " Number of region binders we have passed through while doing the instantiation"] binders_passed : u32 , }}}
mkitem!{mkimpl!{impl < 'a , I : Interner > TypeFolder < I > for ArgFolder < 'a , I > { # [inline] fn cx (& self) -> I { self . cx } fn fold_binder < T : TypeFoldable < I > > (& mut self , t : ty :: Binder < I , T >) -> ty :: Binder < I , T > { self . binders_passed += 1 ; let t = t . super_fold_with (self) ; self . binders_passed -= 1 ; t } fn fold_region (& mut self , r : I :: Region) -> I :: Region { match r . kind () { ty :: ReEarlyParam (data) => { let rk = self . args . get (data . index () as usize) . map (| arg | arg . kind ()) ; match rk { Some (ty :: GenericArgKind :: Lifetime (lt)) => self . shift_region_through_binders (lt) , Some (other) => self . region_param_expected (data , r , other) , None => self . region_param_out_of_range (data , r) , } } ty :: ReBound (..) | ty :: ReLateParam (_) | ty :: ReStatic | ty :: RePlaceholder (_) | ty :: ReErased | ty :: ReError (_) => r , ty :: ReVar (_) => panic ! ("unexpected region: {r:?}") , } } fn fold_ty (& mut self , t : I :: Ty) -> I :: Ty { if ! t . has_param () { return t ; } match t . kind () { ty :: Param (p) => self . ty_for_param (p , t) , _ => t . super_fold_with (self) , } } fn fold_const (& mut self , c : I :: Const) -> I :: Const { if let ty :: ConstKind :: Param (p) = c . kind () { self . const_for_param (p , c) } else { c . super_fold_with (self) } } fn fold_predicate (& mut self , p : I :: Predicate) -> I :: Predicate { if p . has_param () { p . super_fold_with (self) } else { p } } fn fold_clauses (& mut self , c : I :: Clauses) -> I :: Clauses { if c . has_param () { c . super_fold_with (self) } else { c } } }}}
mkitem!{mkimpl!{impl < 'a , I : Interner > ArgFolder < 'a , I > { fn ty_for_param (& self , p : I :: ParamTy , source_ty : I :: Ty) -> I :: Ty { let opt_ty = self . args . get (p . index () as usize) . map (| arg | arg . kind ()) ; let ty = match opt_ty { Some (ty :: GenericArgKind :: Type (ty)) => ty , Some (kind) => self . type_param_expected (p , source_ty , kind) , None => self . type_param_out_of_range (p , source_ty) , } ; self . shift_vars_through_binders (ty) } # [cold] # [inline (never)] fn type_param_expected (& self , p : I :: ParamTy , ty : I :: Ty , kind : ty :: GenericArgKind < I >) -> ! { panic ! ("expected type for `{:?}` ({:?}/{}) but found {:?} when instantiating, args={:?}" , p , ty , p . index () , kind , self . args ,) } # [cold] # [inline (never)] fn type_param_out_of_range (& self , p : I :: ParamTy , ty : I :: Ty) -> ! { panic ! ("type parameter `{:?}` ({:?}/{}) out of range when instantiating, args={:?}" , p , ty , p . index () , self . args ,) } fn const_for_param (& self , p : I :: ParamConst , source_ct : I :: Const) -> I :: Const { let opt_ct = self . args . get (p . index () as usize) . map (| arg | arg . kind ()) ; let ct = match opt_ct { Some (ty :: GenericArgKind :: Const (ct)) => ct , Some (kind) => self . const_param_expected (p , source_ct , kind) , None => self . const_param_out_of_range (p , source_ct) , } ; self . shift_vars_through_binders (ct) } # [cold] # [inline (never)] fn const_param_expected (& self , p : I :: ParamConst , ct : I :: Const , kind : ty :: GenericArgKind < I > ,) -> ! { panic ! ("expected const for `{:?}` ({:?}/{}) but found {:?} when instantiating args={:?}" , p , ct , p . index () , kind , self . args ,) } # [cold] # [inline (never)] fn const_param_out_of_range (& self , p : I :: ParamConst , ct : I :: Const) -> ! { panic ! ("const parameter `{:?}` ({:?}/{}) out of range when instantiating args={:?}" , p , ct , p . index () , self . args ,) } # [cold] # [inline (never)] fn region_param_expected (& self , ebr : I :: EarlyParamRegion , r : I :: Region , kind : ty :: GenericArgKind < I > ,) -> ! { panic ! ("expected region for `{:?}` ({:?}/{}) but found {:?} when instantiating args={:?}" , ebr , r , ebr . index () , kind , self . args ,) } # [cold] # [inline (never)] fn region_param_out_of_range (& self , ebr : I :: EarlyParamRegion , r : I :: Region) -> ! { panic ! ("region parameter `{:?}` ({:?}/{}) out of range when instantiating args={:?}" , ebr , r , ebr . index () , self . args ,) } # [doc = " It is sometimes necessary to adjust the De Bruijn indices during instantiation. This occurs"] # [doc = " when we are instantiating a type with escaping bound vars into a context where we have"] # [doc = " passed through binders. That's quite a mouthful. Let's see an example:"] # [doc = ""] # [doc = " ```"] # [doc = " type Func<A> = fn(A);"] # [doc = " type MetaFunc = for<'a> fn(Func<&'a i32>);"] # [doc = " ```"] # [doc = ""] # [doc = " The type `MetaFunc`, when fully expanded, will be"] # [doc = " ```ignore (illustrative)"] # [doc = " for<'a> fn(fn(&'a i32))"] # [doc = " //      ^~ ^~ ^~~"] # [doc = " //      |  |  |"] # [doc = " //      |  |  DebruijnIndex of 2"] # [doc = " //      Binders"] # [doc = " ```"] # [doc = " Here the `'a` lifetime is bound in the outer function, but appears as an argument of the"] # [doc = " inner one. Therefore, that appearance will have a DebruijnIndex of 2, because we must skip"] # [doc = " over the inner binder (remember that we count De Bruijn indices from 1). However, in the"] # [doc = " definition of `MetaFunc`, the binder is not visible, so the type `&'a i32` will have a"] # [doc = " De Bruijn index of 1. It's only during the instantiation that we can see we must increase the"] # [doc = " depth by 1 to account for the binder that we passed through."] # [doc = ""] # [doc = " As a second example, consider this twist:"] # [doc = ""] # [doc = " ```"] # [doc = " type FuncTuple<A> = (A,fn(A));"] # [doc = " type MetaFuncTuple = for<'a> fn(FuncTuple<&'a i32>);"] # [doc = " ```"] # [doc = ""] # [doc = " Here the final type will be:"] # [doc = " ```ignore (illustrative)"] # [doc = " for<'a> fn((&'a i32, fn(&'a i32)))"] # [doc = " //          ^~~         ^~~"] # [doc = " //          |           |"] # [doc = " //   DebruijnIndex of 1 |"] # [doc = " //               DebruijnIndex of 2"] # [doc = " ```"] # [doc = " As indicated in the diagram, here the same type `&'a i32` is instantiated once, but in the"] # [doc = " first case we do not increase the De Bruijn index and in the second case we do. The reason"] # [doc = " is that only in the second case have we passed through a fn binder."] # [instrument (level = "trace" , skip (self) , fields (binders_passed = self . binders_passed) , ret)] fn shift_vars_through_binders < T : TypeFoldable < I > > (& self , val : T) -> T { if self . binders_passed == 0 || ! val . has_escaping_bound_vars () { val } else { ty :: shift_vars (self . cx , val , self . binders_passed) } } fn shift_region_through_binders (& self , region : I :: Region) -> I :: Region { if self . binders_passed == 0 || ! region . has_escaping_bound_vars () { region } else { ty :: shift_region (self . cx , region , self . binders_passed) } } }}}