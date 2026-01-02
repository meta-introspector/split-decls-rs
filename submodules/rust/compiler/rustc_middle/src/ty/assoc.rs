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
mkuse!{use rustc_data_structures :: sorted_map :: SortedIndexMultiMap ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_hir :: def :: { DefKind , Namespace } ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_hir :: find_attr ;}
mkuse!{use rustc_macros :: { Decodable , Encodable , HashStable } ;}
mkuse!{use rustc_span :: { ErrorGuaranteed , Ident , Symbol } ;}
mkuse!{use super :: { TyCtxt , Visibility } ;}
mkuse!{use crate :: ty ;}
mkitem!{mkenum!{#[derive (Clone , Copy , PartialEq , Eq , Debug , HashStable , Hash , Encodable , Decodable)] pub enum AssocContainer { Trait , InherentImpl , #[doc = " The `DefId` points to the trait item being implemented."] TraitImpl (Result < DefId , ErrorGuaranteed >) , }}}
mkitem!{mkstruct!{#[doc = " Information about an associated item"] #[derive (Copy , Clone , Debug , PartialEq , HashStable , Eq , Hash , Encodable , Decodable)] pub struct AssocItem { pub def_id : DefId , pub kind : AssocKind , pub container : AssocContainer , }}}
mkitem!{mkimpl!{impl AssocItem { pub fn opt_name (& self) -> Option < Symbol > { match self . kind { ty :: AssocKind :: Type { data : AssocTypeData :: Normal (name) } => Some (name) , ty :: AssocKind :: Type { data : AssocTypeData :: Rpitit (_) } => None , ty :: AssocKind :: Const { name } => Some (name) , ty :: AssocKind :: Fn { name , .. } => Some (name) , } } pub fn name (& self) -> Symbol { self . opt_name () . expect ("name of non-Rpitit assoc item") } pub fn ident (& self , tcx : TyCtxt < '_ >) -> Ident { Ident :: new (self . name () , tcx . def_ident_span (self . def_id) . unwrap ()) } #[doc = " Gets the defaultness of the associated item."] #[doc = " To get the default associated type, use the [`type_of`] query on the"] #[doc = " [`DefId`] of the type."] #[doc = ""] #[doc = " [`type_of`]: crate::ty::TyCtxt::type_of"] pub fn defaultness (& self , tcx : TyCtxt < '_ >) -> hir :: Defaultness { match self . container { AssocContainer :: InherentImpl => hir :: Defaultness :: Final , AssocContainer :: Trait | AssocContainer :: TraitImpl (_) => tcx . defaultness (self . def_id) , } } pub fn expect_trait_impl (& self) -> Result < DefId , ErrorGuaranteed > { let AssocContainer :: TraitImpl (trait_item_id) = self . container else { bug ! ("expected item to be in a trait impl: {:?}" , self . def_id) ; } ; trait_item_id } #[doc = " If this is a trait impl item, returns the `DefId` of the trait item this implements."] #[doc = " Otherwise, returns `DefId` for self. Returns an Err in case the trait item was not"] #[doc = " resolved successfully."] pub fn trait_item_or_self (& self) -> Result < DefId , ErrorGuaranteed > { match self . container { AssocContainer :: TraitImpl (id) => id , AssocContainer :: Trait | AssocContainer :: InherentImpl => Ok (self . def_id) , } } pub fn trait_item_def_id (& self) -> Option < DefId > { match self . container { AssocContainer :: TraitImpl (Ok (id)) => Some (id) , _ => None , } } #[inline] pub fn visibility (& self , tcx : TyCtxt < '_ >) -> Visibility < DefId > { tcx . visibility (self . def_id) } #[inline] pub fn container_id (& self , tcx : TyCtxt < '_ >) -> DefId { tcx . parent (self . def_id) } #[inline] pub fn trait_container (& self , tcx : TyCtxt < '_ >) -> Option < DefId > { match self . container { AssocContainer :: InherentImpl | AssocContainer :: TraitImpl (_) => None , AssocContainer :: Trait => Some (tcx . parent (self . def_id)) , } } #[inline] pub fn impl_container (& self , tcx : TyCtxt < '_ >) -> Option < DefId > { match self . container { AssocContainer :: InherentImpl | AssocContainer :: TraitImpl (_) => { Some (tcx . parent (self . def_id)) } AssocContainer :: Trait => None , } } pub fn signature (& self , tcx : TyCtxt < '_ >) -> String { match self . kind { ty :: AssocKind :: Fn { .. } => { tcx . fn_sig (self . def_id) . instantiate_identity () . skip_binder () . to_string () } ty :: AssocKind :: Type { .. } => format ! ("type {};" , self . name ()) , ty :: AssocKind :: Const { name } => { format ! ("const {}: {:?};" , name , tcx . type_of (self . def_id) . instantiate_identity ()) } } } pub fn descr (& self) -> & 'static str { match self . kind { ty :: AssocKind :: Const { .. } => "associated const" , ty :: AssocKind :: Fn { has_self : true , .. } => "method" , ty :: AssocKind :: Fn { has_self : false , .. } => "associated function" , ty :: AssocKind :: Type { .. } => "associated type" , } } pub fn namespace (& self) -> Namespace { match self . kind { ty :: AssocKind :: Type { .. } => Namespace :: TypeNS , ty :: AssocKind :: Const { .. } | ty :: AssocKind :: Fn { .. } => Namespace :: ValueNS , } } pub fn as_def_kind (& self) -> DefKind { match self . kind { AssocKind :: Const { .. } => DefKind :: AssocConst , AssocKind :: Fn { .. } => DefKind :: AssocFn , AssocKind :: Type { .. } => DefKind :: AssocTy , } } pub fn is_type (& self) -> bool { matches ! (self . kind , ty :: AssocKind :: Type { .. }) } pub fn is_fn (& self) -> bool { matches ! (self . kind , ty :: AssocKind :: Fn { .. }) } pub fn is_method (& self) -> bool { matches ! (self . kind , ty :: AssocKind :: Fn { has_self : true , .. }) } pub fn as_tag (& self) -> AssocTag { match self . kind { AssocKind :: Const { .. } => AssocTag :: Const , AssocKind :: Fn { .. } => AssocTag :: Fn , AssocKind :: Type { .. } => AssocTag :: Type , } } pub fn is_impl_trait_in_trait (& self) -> bool { matches ! (self . kind , AssocKind :: Type { data : AssocTypeData :: Rpitit (_) }) } #[doc = " Returns true if:"] #[doc = " - This trait associated item has the `#[type_const]` attribute,"] #[doc = " - If it is in a trait impl, the item from the original trait has this attribute, or"] #[doc = " - It is an inherent assoc const."] pub fn is_type_const_capable (& self , tcx : TyCtxt < '_ >) -> bool { if ! matches ! (self . kind , ty :: AssocKind :: Const { .. }) { return false ; } let def_id = match self . container { AssocContainer :: Trait => self . def_id , AssocContainer :: TraitImpl (Ok (trait_item_did)) => trait_item_did , AssocContainer :: TraitImpl (Err (_)) => return false , AssocContainer :: InherentImpl => return true , } ; find_attr ! (tcx . get_all_attrs (def_id) , AttributeKind :: TypeConst (_)) } }}}
mkitem!{mkenum!{#[derive (Copy , Clone , PartialEq , Debug , HashStable , Eq , Hash , Encodable , Decodable)] pub enum AssocTypeData { Normal (Symbol) , #[doc = " The associated type comes from an RPITIT. It has no name, and the"] #[doc = " `ImplTraitInTraitData` provides additional information about its"] #[doc = " source."] Rpitit (ty :: ImplTraitInTraitData) , }}}
mkitem!{mkenum!{#[derive (Copy , Clone , PartialEq , Debug , HashStable , Eq , Hash , Encodable , Decodable)] pub enum AssocKind { Const { name : Symbol } , Fn { name : Symbol , has_self : bool } , Type { data : AssocTypeData } , }}}
mkitem!{mkimpl!{impl AssocKind { pub fn namespace (& self) -> Namespace { match * self { ty :: AssocKind :: Type { .. } => Namespace :: TypeNS , ty :: AssocKind :: Const { .. } | ty :: AssocKind :: Fn { .. } => Namespace :: ValueNS , } } pub fn as_def_kind (& self) -> DefKind { match self { AssocKind :: Const { .. } => DefKind :: AssocConst , AssocKind :: Fn { .. } => DefKind :: AssocFn , AssocKind :: Type { .. } => DefKind :: AssocTy , } } }}}
mkitem!{mkimpl!{impl std :: fmt :: Display for AssocKind { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { AssocKind :: Fn { has_self : true , .. } => write ! (f , "method") , AssocKind :: Fn { has_self : false , .. } => write ! (f , "associated function") , AssocKind :: Const { .. } => write ! (f , "associated const") , AssocKind :: Type { .. } => write ! (f , "associated type") , } } }}}
mkitem!{mkenum!{#[derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum AssocTag { Const , Fn , Type , }}}
mkitem!{mkstruct!{#[doc = " A list of `ty::AssocItem`s in definition order that allows for efficient lookup by name."] #[doc = ""] #[doc = " When doing lookup by name, we try to postpone hygienic comparison for as long as possible since"] #[doc = " it is relatively expensive. Instead, items are indexed by `Symbol` and hygienic comparison is"] #[doc = " done only on items with the same name."] #[derive (Debug , Clone , PartialEq , HashStable)] pub struct AssocItems { items : SortedIndexMultiMap < u32 , Option < Symbol > , ty :: AssocItem > , }}}
mkitem!{mkimpl!{impl AssocItems { #[doc = " Constructs an `AssociatedItems` map from a series of `ty::AssocItem`s in definition order."] pub fn new (items_in_def_order : impl IntoIterator < Item = ty :: AssocItem >) -> Self { let items = items_in_def_order . into_iter () . map (| item | (item . opt_name () , item)) . collect () ; AssocItems { items } } #[doc = " Returns an iterator over associated items in the order they were defined."] #[doc = ""] #[doc = " New code should avoid relying on definition order. If you need a particular associated item"] #[doc = " for a known trait, make that trait a lang item instead of indexing this array."] pub fn in_definition_order (& self) -> impl '_ + Iterator < Item = & ty :: AssocItem > { self . items . iter () . map (| (_ , v) | v) } pub fn len (& self) -> usize { self . items . len () } #[doc = " Returns an iterator over all associated items with the given name, ignoring hygiene."] #[doc = ""] #[doc = " Panics if `name.is_empty()` returns `true`."] pub fn filter_by_name_unhygienic (& self , name : Symbol ,) -> impl '_ + Iterator < Item = & ty :: AssocItem > { assert ! (! name . is_empty ()) ; self . items . get_by_key (Some (name)) } #[doc = " Returns the associated item with the given identifier and `AssocKind`, if one exists."] #[doc = " The identifier is ignoring hygiene. This is meant to be used for lints and diagnostics."] pub fn filter_by_name_unhygienic_and_kind (& self , name : Symbol , assoc_tag : AssocTag ,) -> impl '_ + Iterator < Item = & ty :: AssocItem > { self . filter_by_name_unhygienic (name) . filter (move | item | item . as_tag () == assoc_tag) } #[doc = " Returns the associated item with the given identifier and `AssocKind`, if one exists."] #[doc = " The identifier is matched hygienically."] pub fn find_by_ident_and_kind (& self , tcx : TyCtxt < '_ > , ident : Ident , assoc_tag : AssocTag , parent_def_id : DefId ,) -> Option < & ty :: AssocItem > { self . filter_by_name_unhygienic (ident . name) . filter (| item | item . as_tag () == assoc_tag) . find (| item | tcx . hygienic_eq (ident , item . ident (tcx) , parent_def_id)) } #[doc = " Returns the associated item with the given identifier in the given `Namespace`, if one"] #[doc = " exists. The identifier is matched hygienically."] pub fn find_by_ident_and_namespace (& self , tcx : TyCtxt < '_ > , ident : Ident , ns : Namespace , parent_def_id : DefId ,) -> Option < & ty :: AssocItem > { self . filter_by_name_unhygienic (ident . name) . filter (| item | item . namespace () == ns) . find (| item | tcx . hygienic_eq (ident , item . ident (tcx) , parent_def_id)) } }}}
mkitem!{mkimpl!{impl < 'tcx > TyCtxt < 'tcx > { #[doc = " Given an `fn_def_id` of a trait or a trait implementation:"] #[doc = ""] #[doc = " if `fn_def_id` is a function defined inside a trait, then it synthesizes"] #[doc = " a new def id corresponding to a new associated type for each return-"] #[doc = " position `impl Trait` in the signature."] #[doc = ""] #[doc = " if `fn_def_id` is a function inside of an impl, then for each synthetic"] #[doc = " associated type generated for the corresponding trait function described"] #[doc = " above, synthesize a corresponding associated type in the impl."] pub fn associated_types_for_impl_traits_in_associated_fn (self , fn_def_id : DefId ,) -> & 'tcx [DefId] { let parent_def_id = self . parent (fn_def_id) ; & self . associated_types_for_impl_traits_in_trait_or_impl (parent_def_id) [& fn_def_id] } }}}