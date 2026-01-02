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
mkuse!{use std :: cmp ;}
mkuse!{use std :: marker :: PhantomData ;}
mkuse!{use std :: ops :: Range ;}
mkuse!{use rustc_data_structures :: undo_log :: Rollback ;}
mkuse!{use rustc_data_structures :: { snapshot_vec as sv , unify as ut } ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyVid } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: infer :: InferCtxtUndoLogs ;}
mkitem!{mkenum!{# [doc = " Represents a single undo-able action that affects a type inference variable."] # [derive (Clone)] pub (crate) enum UndoLog < 'tcx > { EqRelation (sv :: UndoLog < ut :: Delegate < TyVidEqKey < 'tcx > > >) , SubRelation (sv :: UndoLog < ut :: Delegate < TyVidSubKey > >) , }}}
mkitem!{mkimpl!{# [doc = " Convert from a specific kind of undo to the more general UndoLog"] impl < 'tcx > From < sv :: UndoLog < ut :: Delegate < TyVidEqKey < 'tcx > > > > for UndoLog < 'tcx > { fn from (l : sv :: UndoLog < ut :: Delegate < TyVidEqKey < 'tcx > > >) -> Self { UndoLog :: EqRelation (l) } }}}
mkitem!{mkimpl!{# [doc = " Convert from a specific kind of undo to the more general UndoLog"] impl < 'tcx > From < sv :: UndoLog < ut :: Delegate < TyVidSubKey > > > for UndoLog < 'tcx > { fn from (l : sv :: UndoLog < ut :: Delegate < TyVidSubKey > >) -> Self { UndoLog :: SubRelation (l) } }}}
mkitem!{mkimpl!{impl < 'tcx > Rollback < sv :: UndoLog < ut :: Delegate < TyVidEqKey < 'tcx > > > > for TypeVariableStorage < 'tcx > { fn reverse (& mut self , undo : sv :: UndoLog < ut :: Delegate < TyVidEqKey < 'tcx > > >) { self . eq_relations . reverse (undo) } }}}
mkitem!{mkimpl!{impl < 'tcx > Rollback < sv :: UndoLog < ut :: Delegate < TyVidSubKey > > > for TypeVariableStorage < 'tcx > { fn reverse (& mut self , undo : sv :: UndoLog < ut :: Delegate < TyVidSubKey > >) { self . sub_unification_table . reverse (undo) } }}}
mkitem!{mkimpl!{impl < 'tcx > Rollback < UndoLog < 'tcx > > for TypeVariableStorage < 'tcx > { fn reverse (& mut self , undo : UndoLog < 'tcx >) { match undo { UndoLog :: EqRelation (undo) => self . eq_relations . reverse (undo) , UndoLog :: SubRelation (undo) => self . sub_unification_table . reverse (undo) , } } }}}
mkitem!{mkstruct!{# [derive (Clone , Default)] pub (crate) struct TypeVariableStorage < 'tcx > { # [doc = " The origins of each type variable."] values : IndexVec < TyVid , TypeVariableData > , # [doc = " Two variables are unified in `eq_relations` when we have a"] # [doc = " constraint `?X == ?Y`. This table also stores, for each key,"] # [doc = " the known value."] eq_relations : ut :: UnificationTableStorage < TyVidEqKey < 'tcx > > , # [doc = " Only used by `-Znext-solver` and for diagnostics. Tracks whether"] # [doc = " type variables are related via subtyping at all, ignoring which of"] # [doc = " the two is the subtype."] # [doc = ""] # [doc = " When reporting ambiguity errors, we sometimes want to"] # [doc = " treat all inference vars which are subtypes of each"] # [doc = " others as if they are equal. For this case we compute"] # [doc = " the transitive closure of our subtype obligations here."] # [doc = ""] # [doc = " E.g. when encountering ambiguity errors, we want to suggest"] # [doc = " specifying some method argument or to add a type annotation"] # [doc = " to a local variable. Because subtyping cannot change the"] # [doc = " shape of a type, it's fine if the cause of the ambiguity error"] # [doc = " is only related to the suggested variable via subtyping."] # [doc = ""] # [doc = " Even for something like `let x = returns_arg(); x.method();` the"] # [doc = " type of `x` is only a supertype of the argument of `returns_arg`. We"] # [doc = " still want to suggest specifying the type of the argument."] sub_unification_table : ut :: UnificationTableStorage < TyVidSubKey > , }}}
mkitem!{mkstruct!{pub (crate) struct TypeVariableTable < 'a , 'tcx > { storage : & 'a mut TypeVariableStorage < 'tcx > , undo_log : & 'a mut InferCtxtUndoLogs < 'tcx > , }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug)] pub struct TypeVariableOrigin { pub span : Span , # [doc = " `DefId` of the type parameter this was instantiated for, if any."] # [doc = ""] # [doc = " This should only be used for diagnostics."] pub param_def_id : Option < DefId > , }}}
mkitem!{mkstruct!{# [derive (Clone)] pub (crate) struct TypeVariableData { origin : TypeVariableOrigin , }}}
mkitem!{mkenum!{# [derive (Copy , Clone , Debug)] pub (crate) enum TypeVariableValue < 'tcx > { Known { value : Ty < 'tcx > } , Unknown { universe : ty :: UniverseIndex } , }}}
mkitem!{mkimpl!{impl < 'tcx > TypeVariableValue < 'tcx > { # [doc = " If this value is known, returns the type it is known to be."] # [doc = " Otherwise, `None`."] pub (crate) fn known (& self) -> Option < Ty < 'tcx > > { match * self { TypeVariableValue :: Unknown { .. } => None , TypeVariableValue :: Known { value } => Some (value) , } } pub (crate) fn is_unknown (& self) -> bool { match * self { TypeVariableValue :: Unknown { .. } => true , TypeVariableValue :: Known { .. } => false , } } }}}
mkitem!{mkimpl!{impl < 'tcx > TypeVariableStorage < 'tcx > { # [inline] pub (crate) fn with_log < 'a > (& 'a mut self , undo_log : & 'a mut InferCtxtUndoLogs < 'tcx > ,) -> TypeVariableTable < 'a , 'tcx > { TypeVariableTable { storage : self , undo_log } } # [inline] pub (crate) fn eq_relations_ref (& self) -> & ut :: UnificationTableStorage < TyVidEqKey < 'tcx > > { & self . eq_relations } pub (super) fn finalize_rollback (& mut self) { debug_assert ! (self . values . len () >= self . eq_relations . len ()) ; self . values . truncate (self . eq_relations . len ()) ; } }}}
mkitem!{mkimpl!{impl < 'tcx > TypeVariableTable < '_ , 'tcx > { # [doc = " Returns the origin that was given when `vid` was created."] # [doc = ""] # [doc = " Note that this function does not return care whether"] # [doc = " `vid` has been unified with something else or not."] pub (crate) fn var_origin (& self , vid : ty :: TyVid) -> TypeVariableOrigin { self . storage . values [vid] . origin } # [doc = " Records that `a == b`."] # [doc = ""] # [doc = " Precondition: neither `a` nor `b` are known."] pub (crate) fn equate (& mut self , a : ty :: TyVid , b : ty :: TyVid) { debug_assert ! (self . probe (a) . is_unknown ()) ; debug_assert ! (self . probe (b) . is_unknown ()) ; self . eq_relations () . union (a , b) ; self . sub_unification_table () . union (a , b) ; } # [doc = " Records that `a` and `b` are related via subtyping. We don't track"] # [doc = " which of the two is the subtype."] # [doc = ""] # [doc = " Precondition: neither `a` nor `b` are known."] pub (crate) fn sub_unify (& mut self , a : ty :: TyVid , b : ty :: TyVid) { debug_assert ! (self . probe (a) . is_unknown ()) ; debug_assert ! (self . probe (b) . is_unknown ()) ; self . sub_unification_table () . union (a , b) ; } # [doc = " Instantiates `vid` with the type `ty`."] # [doc = ""] # [doc = " Precondition: `vid` must not have been previously instantiated."] pub (crate) fn instantiate (& mut self , vid : ty :: TyVid , ty : Ty < 'tcx >) { let vid = self . root_var (vid) ; debug_assert ! (! ty . is_ty_var () , "instantiating ty var with var: {vid:?} {ty:?}") ; debug_assert ! (self . probe (vid) . is_unknown ()) ; debug_assert ! (self . eq_relations () . probe_value (vid) . is_unknown () , "instantiating type variable `{vid:?}` twice: new-value = {ty:?}, old-value={:?}" , self . eq_relations () . probe_value (vid)) ; self . eq_relations () . union_value (vid , TypeVariableValue :: Known { value : ty }) ; } # [doc = " Creates a new type variable."] # [doc = ""] # [doc = " - `diverging`: indicates if this is a \"diverging\" type"] # [doc = "   variable, e.g.,  one created as the type of a `return`"] # [doc = "   expression. The code in this module doesn't care if a"] # [doc = "   variable is diverging, but the main Rust type-checker will"] # [doc = "   sometimes \"unify\" such variables with the `!` or `()` types."] # [doc = " - `origin`: indicates *why* the type variable was created."] # [doc = "   The code in this module doesn't care, but it can be useful"] # [doc = "   for improving error messages."] pub (crate) fn new_var (& mut self , universe : ty :: UniverseIndex , origin : TypeVariableOrigin ,) -> ty :: TyVid { let eq_key = self . eq_relations () . new_key (TypeVariableValue :: Unknown { universe }) ; let sub_key = self . sub_unification_table () . new_key (()) ; debug_assert_eq ! (eq_key . vid , sub_key . vid) ; let index = self . storage . values . push (TypeVariableData { origin }) ; debug_assert_eq ! (eq_key . vid , index) ; debug ! ("new_var(index={:?}, universe={:?}, origin={:?})" , eq_key . vid , universe , origin) ; index } # [doc = " Returns the number of type variables created thus far."] pub (crate) fn num_vars (& self) -> usize { self . storage . values . len () } # [doc = " Returns the \"root\" variable of `vid` in the `eq_relations`"] # [doc = " equivalence table. All type variables that have been equated"] # [doc = " will yield the same root variable (per the union-find"] # [doc = " algorithm), so `root_var(a) == root_var(b)` implies that `a =="] # [doc = " b` (transitively)."] pub (crate) fn root_var (& mut self , vid : ty :: TyVid) -> ty :: TyVid { self . eq_relations () . find (vid) . vid } # [doc = " Returns the \"root\" variable of `vid` in the `sub_unification_table`"] # [doc = " equivalence table. All type variables that have been are related via"] # [doc = " equality or subtyping will yield the same root variable (per the"] # [doc = " union-find algorithm), so `sub_unification_table_root_var(a)"] # [doc = " == sub_unification_table_root_var(b)` implies that:"] # [doc = " ```text"] # [doc = " exists X. (a <: X || X <: a) && (b <: X || X <: b)"] # [doc = " ```"] pub (crate) fn sub_unification_table_root_var (& mut self , vid : ty :: TyVid) -> ty :: TyVid { self . sub_unification_table () . find (vid) . vid } # [doc = " Retrieves the type to which `vid` has been instantiated, if"] # [doc = " any."] pub (crate) fn probe (& mut self , vid : ty :: TyVid) -> TypeVariableValue < 'tcx > { self . inlined_probe (vid) } # [doc = " An always-inlined variant of `probe`, for very hot call sites."] # [inline (always)] pub (crate) fn inlined_probe (& mut self , vid : ty :: TyVid) -> TypeVariableValue < 'tcx > { self . eq_relations () . inlined_probe_value (vid) } # [inline] fn eq_relations (& mut self) -> super :: UnificationTable < '_ , 'tcx , TyVidEqKey < 'tcx > > { self . storage . eq_relations . with_log (self . undo_log) } # [inline] fn sub_unification_table (& mut self) -> super :: UnificationTable < '_ , 'tcx , TyVidSubKey > { self . storage . sub_unification_table . with_log (self . undo_log) } # [doc = " Returns a range of the type variables created during the snapshot."] pub (crate) fn vars_since_snapshot (& mut self , value_count : usize ,) -> (Range < TyVid > , Vec < TypeVariableOrigin >) { let range = TyVid :: from_usize (value_count) .. TyVid :: from_usize (self . num_vars ()) ; (range . clone () , range . map (| index | self . var_origin (index)) . collect ()) } # [doc = " Returns indices of all variables that are not yet"] # [doc = " instantiated."] pub (crate) fn unresolved_variables (& mut self) -> Vec < ty :: TyVid > { (0 .. self . num_vars ()) . filter_map (| i | { let vid = ty :: TyVid :: from_usize (i) ; match self . probe (vid) { TypeVariableValue :: Unknown { .. } => Some (vid) , TypeVariableValue :: Known { .. } => None , } }) . collect () } }}}
mkitem!{mkstruct!{# [doc = " These structs (a newtyped TyVid) are used as the unification key"] # [doc = " for the `eq_relations`; they carry a `TypeVariableValue` along"] # [doc = " with them."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub (crate) struct TyVidEqKey < 'tcx > { vid : ty :: TyVid , phantom : PhantomData < TypeVariableValue < 'tcx > > , }}}
mkitem!{mkimpl!{impl < 'tcx > From < ty :: TyVid > for TyVidEqKey < 'tcx > { # [inline] fn from (vid : ty :: TyVid) -> Self { TyVidEqKey { vid , phantom : PhantomData } } }}}
mkitem!{mkimpl!{impl < 'tcx > ut :: UnifyKey for TyVidEqKey < 'tcx > { type Value = TypeVariableValue < 'tcx > ; # [inline (always)] fn index (& self) -> u32 { self . vid . as_u32 () } # [inline] fn from_index (i : u32) -> Self { TyVidEqKey :: from (ty :: TyVid :: from_u32 (i)) } fn tag () -> & 'static str { "TyVidEqKey" } fn order_roots (a : Self , _ : & Self :: Value , b : Self , _ : & Self :: Value) -> Option < (Self , Self) > { if a . vid . as_u32 () < b . vid . as_u32 () { Some ((a , b)) } else { Some ((b , a)) } } }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug , PartialEq , Eq)] pub (crate) struct TyVidSubKey { vid : ty :: TyVid , }}}
mkitem!{mkimpl!{impl From < ty :: TyVid > for TyVidSubKey { # [inline] fn from (vid : ty :: TyVid) -> Self { TyVidSubKey { vid } } }}}
mkitem!{mkimpl!{impl ut :: UnifyKey for TyVidSubKey { type Value = () ; # [inline] fn index (& self) -> u32 { self . vid . as_u32 () } # [inline] fn from_index (i : u32) -> TyVidSubKey { TyVidSubKey { vid : ty :: TyVid :: from_u32 (i) } } fn tag () -> & 'static str { "TyVidSubKey" } }}}
mkitem!{mkimpl!{impl < 'tcx > ut :: UnifyValue for TypeVariableValue < 'tcx > { type Error = ut :: NoError ; fn unify_values (value1 : & Self , value2 : & Self) -> Result < Self , ut :: NoError > { match (value1 , value2) { (& TypeVariableValue :: Known { .. } , & TypeVariableValue :: Known { .. }) => { bug ! ("equating two type variables, both of which have known types") } (& TypeVariableValue :: Known { .. } , & TypeVariableValue :: Unknown { .. }) => Ok (* value1) , (& TypeVariableValue :: Unknown { .. } , & TypeVariableValue :: Known { .. }) => Ok (* value2) , (& TypeVariableValue :: Unknown { universe : universe1 } , & TypeVariableValue :: Unknown { universe : universe2 } ,) => { let universe = cmp :: min (universe1 , universe2) ; Ok (TypeVariableValue :: Unknown { universe }) } } } }}}