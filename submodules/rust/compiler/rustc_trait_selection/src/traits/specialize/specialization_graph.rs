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
mkuse!{use rustc_errors :: ErrorGuaranteed ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_macros :: extension ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{pub use rustc_middle :: traits :: specialization_graph :: * ;}
mkuse!{use rustc_middle :: ty :: fast_reject :: { self , SimplifiedType , TreatParams } ;}
mkuse!{use rustc_middle :: ty :: { self , TyCtxt , TypeVisitableExt } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use super :: OverlapError ;}
mkuse!{use crate :: traits ;}
mkitem!{mkenum!{# [derive (Copy , Clone , Debug)] pub enum FutureCompatOverlapErrorKind { LeakCheck , }}}
mkitem!{mkstruct!{# [derive (Debug)] pub struct FutureCompatOverlapError < 'tcx > { pub error : OverlapError < 'tcx > , pub kind : FutureCompatOverlapErrorKind , }}}
mkitem!{mkenum!{# [doc = " The result of attempting to insert an impl into a group of children."] # [derive (Debug)] enum Inserted < 'tcx > { # [doc = " The impl was inserted as a new child in this group of children."] BecameNewSibling (Option < FutureCompatOverlapError < 'tcx > >) , # [doc = " The impl should replace existing impls [X1, ..], because the impl specializes X1, X2, etc."] ReplaceChildren (Vec < DefId >) , # [doc = " The impl is a specialization of an existing child."] ShouldRecurseOn (DefId) , }}}
mkitem!{mkimpl!{# [extension (trait ChildrenExt <'tcx >)] impl < 'tcx > Children { # [doc = " Insert an impl into this set of children without comparing to any existing impls."] fn insert_blindly (& mut self , tcx : TyCtxt < 'tcx > , impl_def_id : DefId) { let trait_ref = tcx . impl_trait_ref (impl_def_id) . unwrap () . skip_binder () ; if let Some (st) = fast_reject :: simplify_type (tcx , trait_ref . self_ty () , TreatParams :: InstantiateWithInfer) { debug ! ("insert_blindly: impl_def_id={:?} st={:?}" , impl_def_id , st) ; self . non_blanket_impls . entry (st) . or_default () . push (impl_def_id) } else { debug ! ("insert_blindly: impl_def_id={:?} st=None" , impl_def_id) ; self . blanket_impls . push (impl_def_id) } } # [doc = " Removes an impl from this set of children. Used when replacing"] # [doc = " an impl with a parent. The impl must be present in the list of"] # [doc = " children already."] fn remove_existing (& mut self , tcx : TyCtxt < 'tcx > , impl_def_id : DefId) { let trait_ref = tcx . impl_trait_ref (impl_def_id) . unwrap () . skip_binder () ; let vec : & mut Vec < DefId > ; if let Some (st) = fast_reject :: simplify_type (tcx , trait_ref . self_ty () , TreatParams :: InstantiateWithInfer) { debug ! ("remove_existing: impl_def_id={:?} st={:?}" , impl_def_id , st) ; vec = self . non_blanket_impls . get_mut (& st) . unwrap () ; } else { debug ! ("remove_existing: impl_def_id={:?} st=None" , impl_def_id) ; vec = & mut self . blanket_impls ; } let index = vec . iter () . position (| d | * d == impl_def_id) . unwrap () ; vec . remove (index) ; } # [doc = " Attempt to insert an impl into this set of children, while comparing for"] # [doc = " specialization relationships."] # [instrument (level = "debug" , skip (self , tcx) , ret)] fn insert (& mut self , tcx : TyCtxt < 'tcx > , impl_def_id : DefId , simplified_self : Option < SimplifiedType > , overlap_mode : OverlapMode ,) -> Result < Inserted < 'tcx > , OverlapError < 'tcx > > { let mut last_lint = None ; let mut replace_children = Vec :: new () ; let possible_siblings = match simplified_self { Some (st) => PotentialSiblings :: Filtered (filtered_children (self , st)) , None => PotentialSiblings :: Unfiltered (iter_children (self)) , } ; for possible_sibling in possible_siblings { debug ! (? possible_sibling) ; let create_overlap_error = | overlap : traits :: coherence :: OverlapResult < 'tcx > | { let trait_ref = overlap . impl_header . trait_ref . unwrap () ; let self_ty = trait_ref . self_ty () ; OverlapError { with_impl : possible_sibling , trait_ref , self_ty : self_ty . has_concrete_skeleton () . then_some (self_ty) , intercrate_ambiguity_causes : overlap . intercrate_ambiguity_causes , involves_placeholder : overlap . involves_placeholder , overflowing_predicates : overlap . overflowing_predicates , } } ; let report_overlap_error = | overlap : traits :: coherence :: OverlapResult < 'tcx > , last_lint : & mut _ | { let should_err = traits :: overlapping_impls (tcx , possible_sibling , impl_def_id , traits :: SkipLeakCheck :: default () , overlap_mode ,) . is_some () ; let error = create_overlap_error (overlap) ; if should_err { Err (error) } else { * last_lint = Some (FutureCompatOverlapError { error , kind : FutureCompatOverlapErrorKind :: LeakCheck , }) ; Ok ((false , false)) } } ; let last_lint_mut = & mut last_lint ; let (le , ge) = traits :: overlapping_impls (tcx , possible_sibling , impl_def_id , traits :: SkipLeakCheck :: Yes , overlap_mode ,) . map_or (Ok ((false , false)) , | overlap | { if let Some (overlap_kind) = tcx . impls_are_allowed_to_overlap (impl_def_id , possible_sibling) { match overlap_kind { ty :: ImplOverlapKind :: Permitted { marker : _ } => { } } return Ok ((false , false)) ; } let le = tcx . specializes ((impl_def_id , possible_sibling)) ; let ge = tcx . specializes ((possible_sibling , impl_def_id)) ; if le == ge { report_overlap_error (overlap , last_lint_mut) } else { Ok ((le , ge)) } }) ? ; if le && ! ge { debug ! ("descending as child of TraitRef {:?}" , tcx . impl_trait_ref (possible_sibling) . unwrap () . instantiate_identity ()) ; return Ok (Inserted :: ShouldRecurseOn (possible_sibling)) ; } else if ge && ! le { debug ! ("placing as parent of TraitRef {:?}" , tcx . impl_trait_ref (possible_sibling) . unwrap () . instantiate_identity ()) ; replace_children . push (possible_sibling) ; } else { } } if ! replace_children . is_empty () { return Ok (Inserted :: ReplaceChildren (replace_children)) ; } debug ! ("placing as new sibling") ; self . insert_blindly (tcx , impl_def_id) ; Ok (Inserted :: BecameNewSibling (last_lint)) } }}}

macro_rules! iter_children_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function iter_children in module {}", module_path!());
    };
}

mkfn!{
    iter_children_introspect!();
    fn iter_children (children : & Children) -> impl Iterator < Item = DefId > { let nonblanket = children . non_blanket_impls . iter () . flat_map (| (_ , v) | v . iter ()) ; children . blanket_impls . iter () . chain (nonblanket) . cloned () }
}

macro_rules! filtered_children_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function filtered_children in module {}", module_path!());
    };
}

mkfn!{
    filtered_children_introspect!();
    fn filtered_children (children : & mut Children , st : SimplifiedType) -> impl Iterator < Item = DefId > { let nonblanket = children . non_blanket_impls . entry (st) . or_default () . iter () ; children . blanket_impls . iter () . chain (nonblanket) . cloned () }
}
mkitem!{mkenum!{enum PotentialSiblings < I , J > where I : Iterator < Item = DefId > , J : Iterator < Item = DefId > , { Unfiltered (I) , Filtered (J) , }}}
mkitem!{mkimpl!{impl < I , J > Iterator for PotentialSiblings < I , J > where I : Iterator < Item = DefId > , J : Iterator < Item = DefId > , { type Item = DefId ; fn next (& mut self) -> Option < Self :: Item > { match * self { PotentialSiblings :: Unfiltered (ref mut iter) => iter . next () , PotentialSiblings :: Filtered (ref mut iter) => iter . next () , } } }}}
mkitem!{mkimpl!{# [extension (pub trait GraphExt <'tcx >)] impl < 'tcx > Graph { # [doc = " Insert a local impl into the specialization graph. If an existing impl"] # [doc = " conflicts with it (has overlap, but neither specializes the other),"] # [doc = " information about the area of overlap is returned in the `Err`."] fn insert (& mut self , tcx : TyCtxt < 'tcx > , impl_def_id : DefId , overlap_mode : OverlapMode ,) -> Result < Option < FutureCompatOverlapError < 'tcx > > , OverlapError < 'tcx > > { assert ! (impl_def_id . is_local ()) ; let trait_ref = tcx . impl_trait_ref (impl_def_id) . unwrap () . skip_binder () ; let trait_def_id = trait_ref . def_id ; debug ! ("insert({:?}): inserting TraitRef {:?} into specialization graph" , impl_def_id , trait_ref) ; if trait_ref . references_error () { debug ! ("insert: inserting dummy node for erroneous TraitRef {:?}, \
                 impl_def_id={:?}, trait_def_id={:?}" , trait_ref , impl_def_id , trait_def_id) ; self . parent . insert (impl_def_id , trait_def_id) ; self . children . entry (trait_def_id) . or_default () . insert_blindly (tcx , impl_def_id) ; return Ok (None) ; } let mut parent = trait_def_id ; let mut last_lint = None ; let simplified = fast_reject :: simplify_type (tcx , trait_ref . self_ty () , TreatParams :: InstantiateWithInfer) ; loop { let insert_result = self . children . entry (parent) . or_default () . insert (tcx , impl_def_id , simplified , overlap_mode ,) ? ; match insert_result { Inserted :: BecameNewSibling (opt_lint) => { last_lint = opt_lint ; break ; } Inserted :: ReplaceChildren (grand_children_to_be) => { { let siblings = self . children . get_mut (& parent) . unwrap () ; for & grand_child_to_be in & grand_children_to_be { siblings . remove_existing (tcx , grand_child_to_be) ; } siblings . insert_blindly (tcx , impl_def_id) ; } for & grand_child_to_be in & grand_children_to_be { self . parent . insert (grand_child_to_be , impl_def_id) ; } self . parent . insert (impl_def_id , parent) ; for & grand_child_to_be in & grand_children_to_be { self . children . entry (impl_def_id) . or_default () . insert_blindly (tcx , grand_child_to_be) ; } break ; } Inserted :: ShouldRecurseOn (new_parent) => { parent = new_parent ; } } } self . parent . insert (impl_def_id , parent) ; Ok (last_lint) } # [doc = " Insert cached metadata mapping from a child impl back to its parent."] fn record_impl_from_cstore (& mut self , tcx : TyCtxt < 'tcx > , parent : DefId , child : DefId) { if self . parent . insert (child , parent) . is_some () { bug ! ("When recording an impl from the crate store, information about its parent \
                 was already present.") ; } self . children . entry (parent) . or_default () . insert_blindly (tcx , child) ; } }}}

macro_rules! assoc_def_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function assoc_def in module {}", module_path!());
    };
}

mkfn!{
    assoc_def_introspect!();
    # [doc = " Locate the definition of an associated type in the specialization hierarchy,"] # [doc = " starting from the given impl."] pub (crate) fn assoc_def (tcx : TyCtxt < '_ > , impl_def_id : DefId , assoc_def_id : DefId ,) -> Result < LeafDef , ErrorGuaranteed > { let trait_def_id = tcx . trait_id_of_impl (impl_def_id) . unwrap () ; let trait_def = tcx . trait_def (trait_def_id) ; if let Some (& impl_item_id) = tcx . impl_item_implementor_ids (impl_def_id) . get (& assoc_def_id) { if let Some (impl_def_id) = impl_def_id . as_local () { tcx . ensure_ok () . enforce_impl_non_lifetime_params_are_constrained (impl_def_id) ? ; } let item = tcx . associated_item (impl_item_id) ; let impl_node = Node :: Impl (impl_def_id) ; return Ok (LeafDef { item , defining_node : impl_node , finalizing_node : if item . defaultness (tcx) . is_default () { None } else { Some (impl_node) } , }) ; } let ancestors = trait_def . ancestors (tcx , impl_def_id) ? ; if let Some (assoc_item) = ancestors . leaf_def (tcx , assoc_def_id) { if let ty :: AssocContainer :: TraitImpl (_) = assoc_item . item . container && let Some (impl_def_id) = assoc_item . item . container_id (tcx) . as_local () { tcx . ensure_ok () . enforce_impl_non_lifetime_params_are_constrained (impl_def_id) ? ; } Ok (assoc_item) } else { bug ! ("No associated type `{}` for {}" , tcx . item_name (assoc_def_id) , tcx . def_path_str (impl_def_id)) } }
}