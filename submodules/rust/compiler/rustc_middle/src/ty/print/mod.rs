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
mkuse!{use hir :: def :: Namespace ;}
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_data_structures :: sso :: SsoHashSet ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def_id :: { CrateNum , DefId } ;}
mkuse!{use rustc_hir :: definitions :: { DefPathData , DisambiguatedDefPathData } ;}
mkuse!{use tracing :: { debug , instrument , trace } ;}
mkuse!{use crate :: ty :: { self , GenericArg , Ty , TyCtxt } ;}
mkmod!{pretty, { 
                getname!(pretty);
                getsrc!(pretty);
                getpath!(pretty);
                get_deps!(pretty);
                get_crates!(pretty);
                mkinclude!(pretty);
                 
            }}
mkuse!{pub use self :: pretty :: * ;}
mkuse!{use super :: Lift ;}
mkitem!{pub type PrintError = std :: fmt :: Error ;}
mkitem!{mktrait!{pub trait Print < 'tcx , P > { fn print (& self , p : & mut P) -> Result < () , PrintError > ; }}}
mkitem!{mktrait!{# [doc = " A trait that \"prints\" user-facing type system entities: paths, types, lifetimes, constants,"] # [doc = " etc. \"Printing\" here means building up a representation of the entity's path, usually as a"] # [doc = " `String` (e.g. \"std::io::Read\") or a `Vec<Symbol>` (e.g. `[sym::std, sym::io, sym::Read]`). The"] # [doc = " representation is built up by appending one or more pieces. The specific details included in"] # [doc = " the built-up representation depend on the purpose of the printer. The more advanced printers"] # [doc = " also rely on the `PrettyPrinter` sub-trait."] pub trait Printer < 'tcx > : Sized { fn tcx < 'a > (& 'a self) -> TyCtxt < 'tcx > ; # [doc = " Appends a representation of an entity with a normal path, e.g. \"std::io::Read\"."] fn print_def_path (& mut self , def_id : DefId , args : & 'tcx [GenericArg < 'tcx >] ,) -> Result < () , PrintError > { self . default_print_def_path (def_id , args) } # [doc = " Like `print_def_path`, but for `DefPathData::Impl`."] fn print_impl_path (& mut self , impl_def_id : DefId , args : & 'tcx [GenericArg < 'tcx >] ,) -> Result < () , PrintError > { let tcx = self . tcx () ; let self_ty = tcx . type_of (impl_def_id) ; let impl_trait_ref = tcx . impl_trait_ref (impl_def_id) ; let (self_ty , impl_trait_ref) = if tcx . generics_of (impl_def_id) . count () <= args . len () { (self_ty . instantiate (tcx , args) , impl_trait_ref . map (| impl_trait_ref | impl_trait_ref . instantiate (tcx , args)) ,) } else { (self_ty . instantiate_identity () , impl_trait_ref . map (| impl_trait_ref | impl_trait_ref . instantiate_identity ()) ,) } ; self . default_print_impl_path (impl_def_id , self_ty , impl_trait_ref) } # [doc = " Appends a representation of a region."] fn print_region (& mut self , region : ty :: Region < 'tcx >) -> Result < () , PrintError > ; # [doc = " Appends a representation of a type."] fn print_type (& mut self , ty : Ty < 'tcx >) -> Result < () , PrintError > ; # [doc = " Appends a representation of a list of `PolyExistentialPredicate`s."] fn print_dyn_existential (& mut self , predicates : & 'tcx ty :: List < ty :: PolyExistentialPredicate < 'tcx > > ,) -> Result < () , PrintError > ; # [doc = " Appends a representation of a const."] fn print_const (& mut self , ct : ty :: Const < 'tcx >) -> Result < () , PrintError > ; # [doc = " Appends a representation of a crate name, e.g. `std`, or even ``."] fn print_crate_name (& mut self , cnum : CrateNum) -> Result < () , PrintError > ; # [doc = " Appends a representation of a (full or partial) simple path, in two parts. `print_prefix`,"] # [doc = " when called, appends the representation of the leading segments. The rest of the method"] # [doc = " appends the representation of the final segment, the details of which are in"] # [doc = " `disambiguated_data`."] # [doc = ""] # [doc = " E.g. `std::io` + `Read` -> `std::io::Read`."] fn print_path_with_simple (& mut self , print_prefix : impl FnOnce (& mut Self) -> Result < () , PrintError > , disambiguated_data : & DisambiguatedDefPathData ,) -> Result < () , PrintError > ; # [doc = " Similar to `print_path_with_simple`, but the final segment is an `impl` segment."] # [doc = ""] # [doc = " E.g. `slice` + `<impl [T]>` -> `slice::<impl [T]>`, which may then be further appended to,"] # [doc = " giving a longer path representation such as `slice::<impl [T]>::to_vec_in::ConvertVec`."] fn print_path_with_impl (& mut self , print_prefix : impl FnOnce (& mut Self) -> Result < () , PrintError > , self_ty : Ty < 'tcx > , trait_ref : Option < ty :: TraitRef < 'tcx > > ,) -> Result < () , PrintError > ; # [doc = " Appends a representation of a path ending in generic args, in two parts. `print_prefix`,"] # [doc = " when called, appends the leading segments. The rest of the method appends the"] # [doc = " representation of the generic args. (Some printers choose to skip appending the generic"] # [doc = " args.)"] # [doc = ""] # [doc = " E.g. `ImplementsTraitForUsize` + `<usize>` -> `ImplementsTraitForUsize<usize>`."] fn print_path_with_generic_args (& mut self , print_prefix : impl FnOnce (& mut Self) -> Result < () , PrintError > , args : & [GenericArg < 'tcx >] ,) -> Result < () , PrintError > ; # [doc = " Appends a representation of a qualified path segment, e.g. `<OsString as From<&T>>`."] # [doc = " If `trait_ref` is `None`, it may fall back to simpler forms, e.g. `<Vec<T>>` or just `Foo`."] fn print_path_with_qualified (& mut self , self_ty : Ty < 'tcx > , trait_ref : Option < ty :: TraitRef < 'tcx > > ,) -> Result < () , PrintError > ; fn print_coroutine_with_kind (& mut self , def_id : DefId , parent_args : & 'tcx [GenericArg < 'tcx >] , kind : Ty < 'tcx > ,) -> Result < () , PrintError > { self . print_path_with_generic_args (| p | p . print_def_path (def_id , parent_args) , & [kind . into ()]) } # [instrument (skip (self) , level = "debug")] fn default_print_def_path (& mut self , def_id : DefId , args : & 'tcx [GenericArg < 'tcx >] ,) -> Result < () , PrintError > { let key = self . tcx () . def_key (def_id) ; debug ! (? key) ; match key . disambiguated_data . data { DefPathData :: CrateRoot => { assert ! (key . parent . is_none ()) ; self . print_crate_name (def_id . krate) } DefPathData :: Impl => self . print_impl_path (def_id , args) , _ => { let parent_def_id = DefId { index : key . parent . unwrap () , .. def_id } ; let mut parent_args = args ; let mut trait_qualify_parent = false ; if ! args . is_empty () { let generics = self . tcx () . generics_of (def_id) ; parent_args = & args [.. generics . parent_count . min (args . len ())] ; match key . disambiguated_data . data { DefPathData :: Closure => { if let Some (hir :: CoroutineKind :: Desugared (_ , hir :: CoroutineSource :: Closure ,)) = self . tcx () . coroutine_kind (def_id) && args . len () > parent_args . len () { return self . print_coroutine_with_kind (def_id , parent_args , args [parent_args . len ()] . expect_ty () ,) ; } else { } } DefPathData :: SyntheticCoroutineBody => { } DefPathData :: AnonConst => { } _ => { if ! generics . is_own_empty () && args . len () >= generics . count () { let args = generics . own_args_no_defaults (self . tcx () , args) ; return self . print_path_with_generic_args (| p | p . print_def_path (def_id , parent_args) , args ,) ; } } } trait_qualify_parent = generics . has_self && generics . parent == Some (parent_def_id) && parent_args . len () == generics . parent_count && self . tcx () . generics_of (parent_def_id) . parent_count == 0 ; } self . print_path_with_simple (| p : & mut Self | { if trait_qualify_parent { let trait_ref = ty :: TraitRef :: new (p . tcx () , parent_def_id , parent_args . iter () . copied () ,) ; p . print_path_with_qualified (trait_ref . self_ty () , Some (trait_ref)) } else { p . print_def_path (parent_def_id , parent_args) } } , & key . disambiguated_data ,) } } } fn default_print_impl_path (& mut self , impl_def_id : DefId , self_ty : Ty < 'tcx > , impl_trait_ref : Option < ty :: TraitRef < 'tcx > > ,) -> Result < () , PrintError > { debug ! ("default_print_impl_path: impl_def_id={:?}, self_ty={}, impl_trait_ref={:?}" , impl_def_id , self_ty , impl_trait_ref) ; let key = self . tcx () . def_key (impl_def_id) ; let parent_def_id = DefId { index : key . parent . unwrap () , .. impl_def_id } ; let in_self_mod = match characteristic_def_id_of_type (self_ty) { None => false , Some (ty_def_id) => self . tcx () . parent (ty_def_id) == parent_def_id , } ; let in_trait_mod = match impl_trait_ref { None => false , Some (trait_ref) => self . tcx () . parent (trait_ref . def_id) == parent_def_id , } ; if ! in_self_mod && ! in_trait_mod { self . print_path_with_impl (| p | p . print_def_path (parent_def_id , & []) , self_ty , impl_trait_ref ,) } else { self . print_path_with_qualified (self_ty , impl_trait_ref) } } }}}

macro_rules! characteristic_def_id_of_type_cached_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function characteristic_def_id_of_type_cached in module {}", module_path!());
    };
}

mkfn!{
    characteristic_def_id_of_type_cached_introspect!();
    # [doc = " As a heuristic, when we see an impl, if we see that the"] # [doc = " 'self type' is a type defined in the same module as the impl,"] # [doc = " we can omit including the path to the impl itself. This"] # [doc = " function tries to find a \"characteristic `DefId`\" for a"] # [doc = " type. It's just a heuristic so it makes some questionable"] # [doc = " decisions and we may want to adjust it later."] # [doc = ""] # [doc = " Visited set is needed to avoid full iteration over"] # [doc = " deeply nested tuples that have no DefId."] fn characteristic_def_id_of_type_cached < 'a > (ty : Ty < 'a > , visited : & mut SsoHashSet < Ty < 'a > > ,) -> Option < DefId > { match * ty . kind () { ty :: Adt (adt_def , _) => Some (adt_def . did ()) , ty :: Dynamic (data , ..) => data . principal_def_id () , ty :: Pat (subty , _) | ty :: Array (subty , _) | ty :: Slice (subty) => { characteristic_def_id_of_type_cached (subty , visited) } ty :: RawPtr (ty , _) => characteristic_def_id_of_type_cached (ty , visited) , ty :: Ref (_ , ty , _) => characteristic_def_id_of_type_cached (ty , visited) , ty :: Tuple (tys) => tys . iter () . find_map (| ty | { if visited . insert (ty) { return characteristic_def_id_of_type_cached (ty , visited) ; } return None ; }) , ty :: FnDef (def_id , _) | ty :: Closure (def_id , _) | ty :: CoroutineClosure (def_id , _) | ty :: Coroutine (def_id , _) | ty :: CoroutineWitness (def_id , _) | ty :: Foreign (def_id) => Some (def_id) , ty :: Bool | ty :: Char | ty :: Int (_) | ty :: Uint (_) | ty :: Str | ty :: FnPtr (..) | ty :: UnsafeBinder (_) | ty :: Alias (..) | ty :: Placeholder (..) | ty :: Param (_) | ty :: Infer (_) | ty :: Bound (..) | ty :: Error (_) | ty :: Never | ty :: Float (_) => None , } }
}

macro_rules! characteristic_def_id_of_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function characteristic_def_id_of_type in module {}", module_path!());
    };
}

mkfn!{
    characteristic_def_id_of_type_introspect!();
    pub fn characteristic_def_id_of_type (ty : Ty < '_ >) -> Option < DefId > { characteristic_def_id_of_type_cached (ty , & mut SsoHashSet :: new ()) }
}
mkitem!{mkimpl!{impl < 'tcx , P : Printer < 'tcx > > Print < 'tcx , P > for ty :: Region < 'tcx > { fn print (& self , p : & mut P) -> Result < () , PrintError > { p . print_region (* self) } }}}
mkitem!{mkimpl!{impl < 'tcx , P : Printer < 'tcx > > Print < 'tcx , P > for Ty < 'tcx > { fn print (& self , p : & mut P) -> Result < () , PrintError > { p . print_type (* self) } }}}
mkitem!{mkimpl!{impl < 'tcx , P : Printer < 'tcx > + std :: fmt :: Write > Print < 'tcx , P > for ty :: Instance < 'tcx > { fn print (& self , cx : & mut P) -> Result < () , PrintError > { cx . print_def_path (self . def_id () , self . args) ? ; match self . def { ty :: InstanceKind :: Item (_) => { } ty :: InstanceKind :: VTableShim (_) => cx . write_str (" - shim(vtable)") ? , ty :: InstanceKind :: ReifyShim (_ , None) => cx . write_str (" - shim(reify)") ? , ty :: InstanceKind :: ReifyShim (_ , Some (ty :: ReifyReason :: FnPtr)) => { cx . write_str (" - shim(reify-fnptr)") ? } ty :: InstanceKind :: ReifyShim (_ , Some (ty :: ReifyReason :: Vtable)) => { cx . write_str (" - shim(reify-vtable)") ? } ty :: InstanceKind :: ThreadLocalShim (_) => cx . write_str (" - shim(tls)") ? , ty :: InstanceKind :: Intrinsic (_) => cx . write_str (" - intrinsic") ? , ty :: InstanceKind :: Virtual (_ , num) => cx . write_str (& format ! (" - virtual#{num}")) ? , ty :: InstanceKind :: FnPtrShim (_ , ty) => cx . write_str (& format ! (" - shim({ty})")) ? , ty :: InstanceKind :: ClosureOnceShim { .. } => cx . write_str (" - shim") ? , ty :: InstanceKind :: ConstructCoroutineInClosureShim { .. } => cx . write_str (" - shim") ? , ty :: InstanceKind :: DropGlue (_ , None) => cx . write_str (" - shim(None)") ? , ty :: InstanceKind :: DropGlue (_ , Some (ty)) => { cx . write_str (& format ! (" - shim(Some({ty}))")) ? } ty :: InstanceKind :: CloneShim (_ , ty) => cx . write_str (& format ! (" - shim({ty})")) ? , ty :: InstanceKind :: FnPtrAddrShim (_ , ty) => cx . write_str (& format ! (" - shim({ty})")) ? , ty :: InstanceKind :: FutureDropPollShim (_ , proxy_ty , impl_ty) => { cx . write_str (& format ! (" - dropshim({proxy_ty}-{impl_ty})")) ? } ty :: InstanceKind :: AsyncDropGlue (_ , ty) => cx . write_str (& format ! (" - shim({ty})")) ? , ty :: InstanceKind :: AsyncDropGlueCtorShim (_ , ty) => { cx . write_str (& format ! (" - shim(Some({ty}))")) ? } } ; Ok (()) } }}}
mkitem!{mkimpl!{impl < 'tcx , P : Printer < 'tcx > > Print < 'tcx , P > for & 'tcx ty :: List < ty :: PolyExistentialPredicate < 'tcx > > { fn print (& self , p : & mut P) -> Result < () , PrintError > { p . print_dyn_existential (self) } }}}
mkitem!{mkimpl!{impl < 'tcx , P : Printer < 'tcx > > Print < 'tcx , P > for ty :: Const < 'tcx > { fn print (& self , p : & mut P) -> Result < () , PrintError > { p . print_const (* self) } }}}
mkitem!{mkimpl!{impl < T > rustc_type_ir :: ir_print :: IrPrint < T > for TyCtxt < '_ > where T : Copy + for < 'a , 'tcx > Lift < TyCtxt < 'tcx > , Lifted : Print < 'tcx , FmtPrinter < 'a , 'tcx > > > , { fn print (t : & T , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { ty :: tls :: with (| tcx | { let mut p = FmtPrinter :: new (tcx , Namespace :: TypeNS) ; tcx . lift (* t) . expect ("could not lift for printing") . print (& mut p) ? ; fmt . write_str (& p . into_buffer ()) ? ; Ok (()) }) } fn print_debug (t : & T , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { with_no_trimmed_paths ! (Self :: print (t , fmt)) } }}}