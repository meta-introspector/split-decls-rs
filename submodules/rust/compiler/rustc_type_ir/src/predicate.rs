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
mkuse!{use std :: fmt ;}
mkuse!{use std :: hash :: Hash ;}
mkuse!{use derive_where :: derive_where ;}
mkuse!{#[cfg (feature = "nightly")] use rustc_macros :: { Decodable , Decodable_NoContext , Encodable , Encodable_NoContext , HashStable_NoContext , } ;}
mkuse!{use rustc_type_ir_macros :: { Lift_Generic , TypeFoldable_Generic , TypeVisitable_Generic } ;}
mkuse!{use crate :: inherent :: * ;}
mkuse!{use crate :: lift :: Lift ;}
mkuse!{use crate :: upcast :: { Upcast , UpcastFrom } ;}
mkuse!{use crate :: visit :: TypeVisitableExt as _ ;}
mkuse!{use crate :: { self as ty , Interner } ;}
mkitem!{mkstruct!{#[doc = " `A: 'region`"] #[derive_where (Clone , Hash , PartialEq , Debug ; I : Interner , A)] #[derive_where (Copy ; I : Interner , A : Copy)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic)] #[cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct OutlivesPredicate < I : Interner , A > (pub A , pub I :: Region) ;}}
mkitem!{mkimpl!{impl < I : Interner , A : Eq > Eq for OutlivesPredicate < I , A > { }}}
mkitem!{mkimpl!{impl < I : Interner , U : Interner , A > Lift < U > for OutlivesPredicate < I , A > where A : Lift < U > , I :: Region : Lift < U , Lifted = U :: Region > , { type Lifted = OutlivesPredicate < U , A :: Lifted > ; fn lift_to_interner (self , cx : U) -> Option < Self :: Lifted > { Some (OutlivesPredicate (self . 0 . lift_to_interner (cx) ? , self . 1 . lift_to_interner (cx) ?)) } }}}
mkitem!{mkstruct!{#[doc = " A complete reference to a trait. These take numerous guises in syntax,"] #[doc = " but perhaps the most recognizable form is in a where-clause:"] #[doc = " ```ignore (illustrative)"] #[doc = " T: Foo<U>"] #[doc = " ```"] #[doc = " This would be represented by a trait-reference where the `DefId` is the"] #[doc = " `DefId` for the trait `Foo` and the args define `T` as parameter 0,"] #[doc = " and `U` as parameter 1."] #[doc = ""] #[doc = " Trait references also appear in object types like `Foo<U>`, but in"] #[doc = " that case the `Self` parameter is absent from the generic parameters."] #[derive_where (Clone , Copy , Hash , PartialEq ; I : Interner)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] #[cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct TraitRef < I : Interner > { pub def_id : I :: TraitId , pub args : I :: GenericArgs , #[doc = " This field exists to prevent the creation of `TraitRef` without"] #[doc = " calling [`TraitRef::new_from_args`]."] _use_trait_ref_new_instead : () , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for TraitRef < I > { }}}
mkitem!{mkimpl!{impl < I : Interner > TraitRef < I > { pub fn new_from_args (interner : I , trait_def_id : I :: TraitId , args : I :: GenericArgs) -> Self { interner . debug_assert_args_compatible (trait_def_id . into () , args) ; Self { def_id : trait_def_id , args , _use_trait_ref_new_instead : () } } pub fn new (interner : I , trait_def_id : I :: TraitId , args : impl IntoIterator < Item : Into < I :: GenericArg > > ,) -> Self { let args = interner . mk_args_from_iter (args . into_iter () . map (Into :: into)) ; Self :: new_from_args (interner , trait_def_id , args) } pub fn from_assoc (interner : I , trait_id : I :: TraitId , args : I :: GenericArgs) -> TraitRef < I > { let generics = interner . generics_of (trait_id . into ()) ; TraitRef :: new (interner , trait_id , args . iter () . take (generics . count ())) } #[doc = " Returns a `TraitRef` of the form `P0: Foo<P1..Pn>` where `Pi`"] #[doc = " are the parameters defined on trait."] pub fn identity (interner : I , def_id : I :: TraitId) -> TraitRef < I > { TraitRef :: new_from_args (interner , def_id , I :: GenericArgs :: identity_for_item (interner , def_id . into ()) ,) } pub fn with_replaced_self_ty (self , interner : I , self_ty : I :: Ty) -> Self { TraitRef :: new (interner , self . def_id , [self_ty . into ()] . into_iter () . chain (self . args . iter () . skip (1)) ,) } #[inline] pub fn self_ty (& self) -> I :: Ty { self . args . type_at (0) } }}}
mkitem!{mkimpl!{impl < I : Interner > ty :: Binder < I , TraitRef < I > > { pub fn self_ty (& self) -> ty :: Binder < I , I :: Ty > { self . map_bound_ref (| tr | tr . self_ty ()) } pub fn def_id (& self) -> I :: TraitId { self . skip_binder () . def_id } pub fn to_host_effect_clause (self , cx : I , constness : BoundConstness) -> I :: Clause { self . map_bound (| trait_ref | { ty :: ClauseKind :: HostEffect (HostEffectPredicate { trait_ref , constness }) }) . upcast (cx) } }}}
mkitem!{mkstruct!{#[derive_where (Clone , Copy , Hash , PartialEq ; I : Interner)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] #[cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct TraitPredicate < I : Interner > { pub trait_ref : TraitRef < I > , #[doc = " If polarity is Positive: we are proving that the trait is implemented."] #[doc = ""] #[doc = " If polarity is Negative: we are proving that a negative impl of this trait"] #[doc = " exists. (Note that coherence also checks whether negative impls of supertraits"] #[doc = " exist via a series of predicates.)"] pub polarity : PredicatePolarity , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for TraitPredicate < I > { }}}
mkitem!{mkimpl!{impl < I : Interner > TraitPredicate < I > { pub fn with_replaced_self_ty (self , interner : I , self_ty : I :: Ty) -> Self { Self { trait_ref : self . trait_ref . with_replaced_self_ty (interner , self_ty) , polarity : self . polarity , } } pub fn def_id (self) -> I :: TraitId { self . trait_ref . def_id } pub fn self_ty (self) -> I :: Ty { self . trait_ref . self_ty () } }}}
mkitem!{mkimpl!{impl < I : Interner > ty :: Binder < I , TraitPredicate < I > > { pub fn def_id (self) -> I :: TraitId { self . skip_binder () . def_id () } pub fn self_ty (self) -> ty :: Binder < I , I :: Ty > { self . map_bound (| trait_ref | trait_ref . self_ty ()) } #[inline] pub fn polarity (self) -> PredicatePolarity { self . skip_binder () . polarity } }}}
mkitem!{mkimpl!{impl < I : Interner > UpcastFrom < I , TraitRef < I > > for TraitPredicate < I > { fn upcast_from (from : TraitRef < I > , _tcx : I) -> Self { TraitPredicate { trait_ref : from , polarity : PredicatePolarity :: Positive } } }}}
mkitem!{mkimpl!{impl < I : Interner > UpcastFrom < I , ty :: Binder < I , TraitRef < I > > > for ty :: Binder < I , TraitPredicate < I > > { fn upcast_from (from : ty :: Binder < I , TraitRef < I > > , _tcx : I) -> Self { from . map_bound (| trait_ref | TraitPredicate { trait_ref , polarity : PredicatePolarity :: Positive , }) } }}}
mkitem!{mkimpl!{impl < I : Interner > fmt :: Debug for TraitPredicate < I > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "TraitPredicate({:?}, polarity:{:?})" , self . trait_ref , self . polarity) } }}}
mkitem!{mkenum!{#[derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] #[cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub enum ImplPolarity { #[doc = " `impl Trait for Type`"] Positive , #[doc = " `impl !Trait for Type`"] Negative , #[doc = " `#[rustc_reservation_impl] impl Trait for Type`"] #[doc = ""] #[doc = " This is a \"stability hack\", not a real Rust feature."] #[doc = " See #64631 for details."] Reservation , }}}
mkitem!{mkimpl!{impl fmt :: Display for ImplPolarity { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Positive => f . write_str ("positive") , Self :: Negative => f . write_str ("negative") , Self :: Reservation => f . write_str ("reservation") , } } }}}
mkitem!{mkimpl!{impl ImplPolarity { #[doc = " The polarity marker in front of the impl trait ref if applicable."] pub fn as_str (self) -> & 'static str { match self { Self :: Positive => "" , Self :: Negative => "!" , Self :: Reservation => "" , } } }}}
mkitem!{mkenum!{#[doc = " Polarity for a trait predicate. May either be negative or positive."] #[doc = " Distinguished from [`ImplPolarity`] since we never compute goals with"] #[doc = " \"reservation\" level."] #[derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] #[cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub enum PredicatePolarity { #[doc = " `Type: Trait`"] Positive , #[doc = " `Type: !Trait`"] Negative , }}}
mkitem!{mkimpl!{impl PredicatePolarity { #[doc = " Flips polarity by turning `Positive` into `Negative` and `Negative` into `Positive`."] pub fn flip (& self) -> PredicatePolarity { match self { PredicatePolarity :: Positive => PredicatePolarity :: Negative , PredicatePolarity :: Negative => PredicatePolarity :: Positive , } } }}}
mkitem!{mkimpl!{impl fmt :: Display for PredicatePolarity { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Positive => f . write_str ("positive") , Self :: Negative => f . write_str ("negative") , } } }}}
mkitem!{mkenum!{#[derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] #[cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub enum ExistentialPredicate < I : Interner > { #[doc = " E.g., `Iterator`."] Trait (ExistentialTraitRef < I >) , #[doc = " E.g., `Iterator::Item = T`."] Projection (ExistentialProjection < I >) , #[doc = " E.g., `Send`."] AutoTrait (I :: TraitId) , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for ExistentialPredicate < I > { }}}
mkitem!{mkimpl!{impl < I : Interner > ty :: Binder < I , ExistentialPredicate < I > > { #[doc = " Given an existential predicate like `?Self: PartialEq<u32>` (e.g., derived from `dyn PartialEq<u32>`),"] #[doc = " and a concrete type `self_ty`, returns a full predicate where the existentially quantified variable `?Self`"] #[doc = " has been replaced with `self_ty` (e.g., `self_ty: PartialEq<u32>`, in our example)."] pub fn with_self_ty (& self , cx : I , self_ty : I :: Ty) -> I :: Clause { match self . skip_binder () { ExistentialPredicate :: Trait (tr) => self . rebind (tr) . with_self_ty (cx , self_ty) . upcast (cx) , ExistentialPredicate :: Projection (p) => { self . rebind (p . with_self_ty (cx , self_ty)) . upcast (cx) } ExistentialPredicate :: AutoTrait (did) => { let generics = cx . generics_of (did . into ()) ; let trait_ref = if generics . count () == 1 { ty :: TraitRef :: new (cx , did , [self_ty]) } else { let err_args = GenericArgs :: extend_with_error (cx , did . into () , & [self_ty . into ()]) ; ty :: TraitRef :: new_from_args (cx , did , err_args) } ; self . rebind (trait_ref) . upcast (cx) } } } }}}
mkitem!{mkstruct!{#[doc = " An existential reference to a trait, where `Self` is erased."] #[doc = " For example, the trait object `Trait<'a, 'b, X, Y>` is:"] #[doc = " ```ignore (illustrative)"] #[doc = " exists T. T: Trait<'a, 'b, X, Y>"] #[doc = " ```"] #[doc = " The generic parameters don't include the erased `Self`, only trait"] #[doc = " type and lifetime parameters (`[X, Y]` and `['a, 'b]` above)."] #[derive_where (Clone , Copy , Hash , PartialEq ; I : Interner)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] #[cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct ExistentialTraitRef < I : Interner > { pub def_id : I :: TraitId , pub args : I :: GenericArgs , #[doc = " This field exists to prevent the creation of `ExistentialTraitRef` without"] #[doc = " calling [`ExistentialTraitRef::new_from_args`]."] _use_existential_trait_ref_new_instead : () , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for ExistentialTraitRef < I > { }}}
mkitem!{mkimpl!{impl < I : Interner > ExistentialTraitRef < I > { pub fn new_from_args (interner : I , trait_def_id : I :: TraitId , args : I :: GenericArgs) -> Self { interner . debug_assert_existential_args_compatible (trait_def_id . into () , args) ; Self { def_id : trait_def_id , args , _use_existential_trait_ref_new_instead : () } } pub fn new (interner : I , trait_def_id : I :: TraitId , args : impl IntoIterator < Item : Into < I :: GenericArg > > ,) -> Self { let args = interner . mk_args_from_iter (args . into_iter () . map (Into :: into)) ; Self :: new_from_args (interner , trait_def_id , args) } pub fn erase_self_ty (interner : I , trait_ref : TraitRef < I >) -> ExistentialTraitRef < I > { trait_ref . args . type_at (0) ; ExistentialTraitRef { def_id : trait_ref . def_id , args : interner . mk_args (& trait_ref . args . as_slice () [1 ..]) , _use_existential_trait_ref_new_instead : () , } } #[doc = " Object types don't have a self type specified. Therefore, when"] #[doc = " we convert the principal trait-ref into a normal trait-ref,"] #[doc = " you must give *some* self type. A common choice is `mk_err()`"] #[doc = " or some placeholder type."] pub fn with_self_ty (self , interner : I , self_ty : I :: Ty) -> TraitRef < I > { TraitRef :: new (interner , self . def_id , [self_ty . into ()] . into_iter () . chain (self . args . iter ())) } }}}
mkitem!{mkimpl!{impl < I : Interner > ty :: Binder < I , ExistentialTraitRef < I > > { pub fn def_id (& self) -> I :: TraitId { self . skip_binder () . def_id } #[doc = " Object types don't have a self type specified. Therefore, when"] #[doc = " we convert the principal trait-ref into a normal trait-ref,"] #[doc = " you must give *some* self type. A common choice is `mk_err()`"] #[doc = " or some placeholder type."] pub fn with_self_ty (& self , cx : I , self_ty : I :: Ty) -> ty :: Binder < I , TraitRef < I > > { self . map_bound (| trait_ref | trait_ref . with_self_ty (cx , self_ty)) } }}}
mkitem!{mkstruct!{#[doc = " A `ProjectionPredicate` for an `ExistentialTraitRef`."] #[derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] #[cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct ExistentialProjection < I : Interner > { pub def_id : I :: DefId , pub args : I :: GenericArgs , pub term : I :: Term , #[doc = " This field exists to prevent the creation of `ExistentialProjection`"] #[doc = " without using [`ExistentialProjection::new_from_args`]."] use_existential_projection_new_instead : () , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for ExistentialProjection < I > { }}}
mkitem!{mkimpl!{impl < I : Interner > ExistentialProjection < I > { pub fn new_from_args (interner : I , def_id : I :: DefId , args : I :: GenericArgs , term : I :: Term ,) -> ExistentialProjection < I > { interner . debug_assert_existential_args_compatible (def_id , args) ; Self { def_id , args , term , use_existential_projection_new_instead : () } } pub fn new (interner : I , def_id : I :: DefId , args : impl IntoIterator < Item : Into < I :: GenericArg > > , term : I :: Term ,) -> ExistentialProjection < I > { let args = interner . mk_args_from_iter (args . into_iter () . map (Into :: into)) ; Self :: new_from_args (interner , def_id , args , term) } #[doc = " Extracts the underlying existential trait reference from this projection."] #[doc = " For example, if this is a projection of `exists T. <T as Iterator>::Item == X`,"] #[doc = " then this function would return an `exists T. T: Iterator` existential trait"] #[doc = " reference."] pub fn trait_ref (& self , interner : I) -> ExistentialTraitRef < I > { let def_id = interner . parent (self . def_id) ; let args_count = interner . generics_of (def_id) . count () - 1 ; let args = interner . mk_args (& self . args . as_slice () [.. args_count]) ; ExistentialTraitRef :: new_from_args (interner , def_id . try_into () . unwrap () , args) } pub fn with_self_ty (& self , interner : I , self_ty : I :: Ty) -> ProjectionPredicate < I > { debug_assert ! (! self_ty . has_escaping_bound_vars ()) ; ProjectionPredicate { projection_term : AliasTerm :: new (interner , self . def_id , [self_ty . into ()] . iter () . chain (self . args . iter ()) ,) , term : self . term , } } pub fn erase_self_ty (interner : I , projection_predicate : ProjectionPredicate < I >) -> Self { projection_predicate . projection_term . args . type_at (0) ; Self { def_id : projection_predicate . projection_term . def_id , args : interner . mk_args (& projection_predicate . projection_term . args . as_slice () [1 ..]) , term : projection_predicate . term , use_existential_projection_new_instead : () , } } }}}
mkitem!{mkimpl!{impl < I : Interner > ty :: Binder < I , ExistentialProjection < I > > { pub fn with_self_ty (& self , cx : I , self_ty : I :: Ty) -> ty :: Binder < I , ProjectionPredicate < I > > { self . map_bound (| p | p . with_self_ty (cx , self_ty)) } pub fn item_def_id (& self) -> I :: DefId { self . skip_binder () . def_id } }}}
mkitem!{mkenum!{#[derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] #[cfg_attr (feature = "nightly" , derive (Encodable , Decodable , HashStable_NoContext))] pub enum AliasTermKind { #[doc = " A projection `<Type as Trait>::AssocType`."] #[doc = " Can get normalized away if monomorphic enough."] ProjectionTy , #[doc = " An associated type in an inherent `impl`"] InherentTy , #[doc = " An opaque type (usually from `impl Trait` in type aliases or function return types)"] #[doc = " Can only be normalized away in PostAnalysis mode or its defining scope."] OpaqueTy , #[doc = " A free type alias that actually checks its trait bounds."] #[doc = " Currently only used if the type alias references opaque types."] #[doc = " Can always be normalized away."] FreeTy , #[doc = " An unevaluated anonymous constants."] UnevaluatedConst , #[doc = " An unevaluated const coming from an associated const."] ProjectionConst , #[doc = " A top level const item not part of a trait or impl."] FreeConst , #[doc = " An associated const in an inherent `impl`"] InherentConst , }}}
mkitem!{mkimpl!{impl AliasTermKind { pub fn descr (self) -> & 'static str { match self { AliasTermKind :: ProjectionTy => "associated type" , AliasTermKind :: ProjectionConst => "associated const" , AliasTermKind :: InherentTy => "inherent associated type" , AliasTermKind :: InherentConst => "inherent associated const" , AliasTermKind :: OpaqueTy => "opaque type" , AliasTermKind :: FreeTy => "type alias" , AliasTermKind :: FreeConst => "unevaluated constant" , AliasTermKind :: UnevaluatedConst => "unevaluated constant" , } } pub fn is_type (self) -> bool { match self { AliasTermKind :: ProjectionTy | AliasTermKind :: InherentTy | AliasTermKind :: OpaqueTy | AliasTermKind :: FreeTy => true , AliasTermKind :: UnevaluatedConst | AliasTermKind :: ProjectionConst | AliasTermKind :: InherentConst | AliasTermKind :: FreeConst => false , } } }}}
mkitem!{mkimpl!{impl From < ty :: AliasTyKind > for AliasTermKind { fn from (value : ty :: AliasTyKind) -> Self { match value { ty :: Projection => AliasTermKind :: ProjectionTy , ty :: Opaque => AliasTermKind :: OpaqueTy , ty :: Free => AliasTermKind :: FreeTy , ty :: Inherent => AliasTermKind :: InherentTy , } } }}}
mkitem!{mkstruct!{#[doc = " Represents the unprojected term of a projection goal."] #[doc = ""] #[doc = " * For a projection, this would be `<Ty as Trait<...>>::N<...>`."] #[doc = " * For an inherent projection, this would be `Ty::N<...>`."] #[doc = " * For an opaque type, there is no explicit syntax."] #[derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] #[cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct AliasTerm < I : Interner > { #[doc = " The parameters of the associated or opaque item."] #[doc = ""] #[doc = " For a projection, these are the generic parameters for the trait and the"] #[doc = " GAT parameters, if there are any."] #[doc = ""] #[doc = " For an inherent projection, they consist of the self type and the GAT parameters,"] #[doc = " if there are any."] #[doc = ""] #[doc = " For RPIT the generic parameters are for the generics of the function,"] #[doc = " while for TAIT it is used for the generic parameters of the alias."] pub args : I :: GenericArgs , #[doc = " The `DefId` of the `TraitItem` or `ImplItem` for the associated type `N` depending on whether"] #[doc = " this is a projection or an inherent projection or the `DefId` of the `OpaqueType` item if"] #[doc = " this is an opaque."] #[doc = ""] #[doc = " During codegen, `interner.type_of(def_id)` can be used to get the type of the"] #[doc = " underlying type if the type is an opaque."] #[doc = ""] #[doc = " Note that if this is an associated type, this is not the `DefId` of the"] #[doc = " `TraitRef` containing this associated type, which is in `interner.associated_item(def_id).container`,"] #[doc = " aka. `interner.parent(def_id)`."] pub def_id : I :: DefId , #[doc = " This field exists to prevent the creation of `AliasTerm` without using [`AliasTerm::new_from_args`]."] #[derive_where (skip (Debug))] _use_alias_term_new_instead : () , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for AliasTerm < I > { }}}
mkitem!{mkimpl!{impl < I : Interner > AliasTerm < I > { pub fn new_from_args (interner : I , def_id : I :: DefId , args : I :: GenericArgs) -> AliasTerm < I > { interner . debug_assert_args_compatible (def_id , args) ; AliasTerm { def_id , args , _use_alias_term_new_instead : () } } pub fn new (interner : I , def_id : I :: DefId , args : impl IntoIterator < Item : Into < I :: GenericArg > > ,) -> AliasTerm < I > { let args = interner . mk_args_from_iter (args . into_iter () . map (Into :: into)) ; Self :: new_from_args (interner , def_id , args) } pub fn expect_ty (self , interner : I) -> ty :: AliasTy < I > { match self . kind (interner) { AliasTermKind :: ProjectionTy | AliasTermKind :: InherentTy | AliasTermKind :: OpaqueTy | AliasTermKind :: FreeTy => { } AliasTermKind :: InherentConst | AliasTermKind :: FreeConst | AliasTermKind :: UnevaluatedConst | AliasTermKind :: ProjectionConst => { panic ! ("Cannot turn `UnevaluatedConst` into `AliasTy`") } } ty :: AliasTy { def_id : self . def_id , args : self . args , _use_alias_ty_new_instead : () } } pub fn kind (self , interner : I) -> AliasTermKind { interner . alias_term_kind (self) } pub fn to_term (self , interner : I) -> I :: Term { match self . kind (interner) { AliasTermKind :: ProjectionTy => Ty :: new_alias (interner , ty :: AliasTyKind :: Projection , ty :: AliasTy { def_id : self . def_id , args : self . args , _use_alias_ty_new_instead : () } ,) . into () , AliasTermKind :: InherentTy => Ty :: new_alias (interner , ty :: AliasTyKind :: Inherent , ty :: AliasTy { def_id : self . def_id , args : self . args , _use_alias_ty_new_instead : () } ,) . into () , AliasTermKind :: OpaqueTy => Ty :: new_alias (interner , ty :: AliasTyKind :: Opaque , ty :: AliasTy { def_id : self . def_id , args : self . args , _use_alias_ty_new_instead : () } ,) . into () , AliasTermKind :: FreeTy => Ty :: new_alias (interner , ty :: AliasTyKind :: Free , ty :: AliasTy { def_id : self . def_id , args : self . args , _use_alias_ty_new_instead : () } ,) . into () , AliasTermKind :: FreeConst | AliasTermKind :: InherentConst | AliasTermKind :: UnevaluatedConst | AliasTermKind :: ProjectionConst => I :: Const :: new_unevaluated (interner , ty :: UnevaluatedConst :: new (self . def_id , self . args) ,) . into () , } } }}}
mkitem!{mkimpl!{#[doc = " The following methods work only with (trait) associated term projections."] impl < I : Interner > AliasTerm < I > { pub fn self_ty (self) -> I :: Ty { self . args . type_at (0) } pub fn with_replaced_self_ty (self , interner : I , self_ty : I :: Ty) -> Self { AliasTerm :: new (interner , self . def_id , [self_ty . into ()] . into_iter () . chain (self . args . iter () . skip (1)) ,) } pub fn trait_def_id (self , interner : I) -> I :: TraitId { assert ! (matches ! (self . kind (interner) , AliasTermKind :: ProjectionTy | AliasTermKind :: ProjectionConst) , "expected a projection") ; interner . parent (self . def_id) . try_into () . unwrap () } #[doc = " Extracts the underlying trait reference and own args from this projection."] #[doc = " For example, if this is a projection of `<T as StreamingIterator>::Item<'a>`,"] #[doc = " then this function would return a `T: StreamingIterator` trait reference and"] #[doc = " `['a]` as the own args."] pub fn trait_ref_and_own_args (self , interner : I) -> (TraitRef < I > , I :: GenericArgsSlice) { interner . trait_ref_and_own_args_for_alias (self . def_id , self . args) } #[doc = " Extracts the underlying trait reference from this projection."] #[doc = " For example, if this is a projection of `<T as Iterator>::Item`,"] #[doc = " then this function would return a `T: Iterator` trait reference."] #[doc = ""] #[doc = " WARNING: This will drop the args for generic associated types"] #[doc = " consider calling [Self::trait_ref_and_own_args] to get those"] #[doc = " as well."] pub fn trait_ref (self , interner : I) -> TraitRef < I > { self . trait_ref_and_own_args (interner) . 0 } #[doc = " Extract the own args from this projection."] #[doc = " For example, if this is a projection of `<T as StreamingIterator>::Item<'a>`,"] #[doc = " then this function would return the slice `['a]` as the own args."] pub fn own_args (self , interner : I) -> I :: GenericArgsSlice { self . trait_ref_and_own_args (interner) . 1 } }}}
mkitem!{mkimpl!{#[doc = " The following methods work only with inherent associated term projections."] impl < I : Interner > AliasTerm < I > { #[doc = " Transform the generic parameters to have the given `impl` args as the base and the GAT args on top of that."] #[doc = ""] #[doc = " Does the following transformation:"] #[doc = ""] #[doc = " ```text"] #[doc = " [Self, P_0...P_m] -> [I_0...I_n, P_0...P_m]"] #[doc = ""] #[doc = "     I_i impl args"] #[doc = "     P_j GAT args"] #[doc = " ```"] pub fn rebase_inherent_args_onto_impl (self , impl_args : I :: GenericArgs , interner : I ,) -> I :: GenericArgs { debug_assert ! (matches ! (self . kind (interner) , AliasTermKind :: InherentTy | AliasTermKind :: InherentConst)) ; interner . mk_args_from_iter (impl_args . iter () . chain (self . args . iter () . skip (1))) } }}}
mkitem!{mkimpl!{impl < I : Interner > From < ty :: AliasTy < I > > for AliasTerm < I > { fn from (ty : ty :: AliasTy < I >) -> Self { AliasTerm { args : ty . args , def_id : ty . def_id , _use_alias_term_new_instead : () } } }}}
mkitem!{mkimpl!{impl < I : Interner > From < ty :: UnevaluatedConst < I > > for AliasTerm < I > { fn from (ct : ty :: UnevaluatedConst < I >) -> Self { AliasTerm { args : ct . args , def_id : ct . def , _use_alias_term_new_instead : () } } }}}
mkitem!{mkstruct!{#[doc = " This kind of predicate has no *direct* correspondent in the"] #[doc = " syntax, but it roughly corresponds to the syntactic forms:"] #[doc = ""] #[doc = " 1. `T: TraitRef<..., Item = Type>`"] #[doc = " 2. `<T as TraitRef<...>>::Item == Type` (NYI)"] #[doc = ""] #[doc = " In particular, form #1 is \"desugared\" to the combination of a"] #[doc = " normal trait predicate (`T: TraitRef<...>`) and one of these"] #[doc = " predicates. Form #2 is a broader form in that it also permits"] #[doc = " equality between arbitrary types. Processing an instance of"] #[doc = " Form #2 eventually yields one of these `ProjectionPredicate`"] #[doc = " instances to normalize the LHS."] #[derive_where (Clone , Copy , Hash , PartialEq ; I : Interner)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] #[cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct ProjectionPredicate < I : Interner > { pub projection_term : AliasTerm < I > , pub term : I :: Term , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for ProjectionPredicate < I > { }}}
mkitem!{mkimpl!{impl < I : Interner > ProjectionPredicate < I > { pub fn self_ty (self) -> I :: Ty { self . projection_term . self_ty () } pub fn with_replaced_self_ty (self , interner : I , self_ty : I :: Ty) -> ProjectionPredicate < I > { Self { projection_term : self . projection_term . with_replaced_self_ty (interner , self_ty) , .. self } } pub fn trait_def_id (self , interner : I) -> I :: TraitId { self . projection_term . trait_def_id (interner) } pub fn def_id (self) -> I :: DefId { self . projection_term . def_id } }}}
mkitem!{mkimpl!{impl < I : Interner > ty :: Binder < I , ProjectionPredicate < I > > { #[doc = " Returns the `DefId` of the trait of the associated item being projected."] #[inline] pub fn trait_def_id (& self , cx : I) -> I :: TraitId { self . skip_binder () . projection_term . trait_def_id (cx) } pub fn term (& self) -> ty :: Binder < I , I :: Term > { self . map_bound (| predicate | predicate . term) } #[doc = " The `DefId` of the `TraitItem` for the associated type."] #[doc = ""] #[doc = " Note that this is not the `DefId` of the `TraitRef` containing this"] #[doc = " associated type, which is in `tcx.associated_item(projection_def_id()).container`."] pub fn item_def_id (& self) -> I :: DefId { self . skip_binder () . projection_term . def_id } }}}
mkitem!{mkimpl!{impl < I : Interner > fmt :: Debug for ProjectionPredicate < I > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "ProjectionPredicate({:?}, {:?})" , self . projection_term , self . term) } }}}
mkitem!{mkstruct!{#[doc = " Used by the new solver to normalize an alias. This always expects the `term` to"] #[doc = " be an unconstrained inference variable which is used as the output."] #[derive_where (Clone , Copy , Hash , PartialEq ; I : Interner)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] #[cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct NormalizesTo < I : Interner > { pub alias : AliasTerm < I > , pub term : I :: Term , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for NormalizesTo < I > { }}}
mkitem!{mkimpl!{impl < I : Interner > NormalizesTo < I > { pub fn self_ty (self) -> I :: Ty { self . alias . self_ty () } pub fn with_replaced_self_ty (self , interner : I , self_ty : I :: Ty) -> NormalizesTo < I > { Self { alias : self . alias . with_replaced_self_ty (interner , self_ty) , .. self } } pub fn trait_def_id (self , interner : I) -> I :: TraitId { self . alias . trait_def_id (interner) } pub fn def_id (self) -> I :: DefId { self . alias . def_id } }}}
mkitem!{mkimpl!{impl < I : Interner > fmt :: Debug for NormalizesTo < I > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "NormalizesTo({:?}, {:?})" , self . alias , self . term) } }}}
mkitem!{mkstruct!{#[derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] #[cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub struct HostEffectPredicate < I : Interner > { pub trait_ref : ty :: TraitRef < I > , pub constness : BoundConstness , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for HostEffectPredicate < I > { }}}
mkitem!{mkimpl!{impl < I : Interner > HostEffectPredicate < I > { pub fn self_ty (self) -> I :: Ty { self . trait_ref . self_ty () } pub fn with_replaced_self_ty (self , interner : I , self_ty : I :: Ty) -> Self { Self { trait_ref : self . trait_ref . with_replaced_self_ty (interner , self_ty) , .. self } } pub fn def_id (self) -> I :: TraitId { self . trait_ref . def_id } }}}
mkitem!{mkimpl!{impl < I : Interner > ty :: Binder < I , HostEffectPredicate < I > > { pub fn def_id (self) -> I :: TraitId { self . skip_binder () . def_id () } pub fn self_ty (self) -> ty :: Binder < I , I :: Ty > { self . map_bound (| trait_ref | trait_ref . self_ty ()) } #[inline] pub fn constness (self) -> BoundConstness { self . skip_binder () . constness } }}}
mkitem!{mkstruct!{#[doc = " Encodes that `a` must be a subtype of `b`. The `a_is_expected` flag indicates"] #[doc = " whether the `a` type is the type that we should label as \"expected\" when"] #[doc = " presenting user diagnostics."] #[derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] #[cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct SubtypePredicate < I : Interner > { pub a_is_expected : bool , pub a : I :: Ty , pub b : I :: Ty , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for SubtypePredicate < I > { }}}
mkitem!{mkstruct!{#[doc = " Encodes that we have to coerce *from* the `a` type to the `b` type."] #[derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] #[cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct CoercePredicate < I : Interner > { pub a : I :: Ty , pub b : I :: Ty , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for CoercePredicate < I > { }}}
mkitem!{mkenum!{#[derive (Clone , Copy , Hash , PartialEq , Eq , Debug)] #[cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub enum BoundConstness { #[doc = " `Type: const Trait`"] #[doc = ""] #[doc = " A bound is required to be unconditionally const, even in a runtime function."] Const , #[doc = " `Type: [const] Trait`"] #[doc = ""] #[doc = " Requires resolving to const only when we are in a const context."] Maybe , }}}
mkitem!{mkimpl!{impl BoundConstness { pub fn satisfies (self , goal : BoundConstness) -> bool { match (self , goal) { (BoundConstness :: Const , BoundConstness :: Const | BoundConstness :: Maybe) => true , (BoundConstness :: Maybe , BoundConstness :: Maybe) => true , (BoundConstness :: Maybe , BoundConstness :: Const) => false , } } pub fn as_str (self) -> & 'static str { match self { Self :: Const => "const" , Self :: Maybe => "[const]" , } } }}}
mkitem!{mkimpl!{impl fmt :: Display for BoundConstness { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Const => f . write_str ("const") , Self :: Maybe => f . write_str ("[const]") , } } }}}