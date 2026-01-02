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
mkuse!{use rustc_data_structures :: fx :: FxIndexMap ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_type_ir :: data_structures :: DelayedMap ;}
mkuse!{use crate :: ty :: { self , Binder , BoundConst , BoundTy , Ty , TyCtxt , TypeFoldable , TypeFolder , TypeSuperFoldable , TypeVisitableExt , } ;}
mkitem!{mkstruct!{pub struct BottomUpFolder < 'tcx , F , G , H > where F : FnMut (Ty < 'tcx >) -> Ty < 'tcx > , G : FnMut (ty :: Region < 'tcx >) -> ty :: Region < 'tcx > , H : FnMut (ty :: Const < 'tcx >) -> ty :: Const < 'tcx > , { pub tcx : TyCtxt < 'tcx > , pub ty_op : F , pub lt_op : G , pub ct_op : H , }}}
mkitem!{mkimpl!{impl < 'tcx , F , G , H > TypeFolder < TyCtxt < 'tcx > > for BottomUpFolder < 'tcx , F , G , H > where F : FnMut (Ty < 'tcx >) -> Ty < 'tcx > , G : FnMut (ty :: Region < 'tcx >) -> ty :: Region < 'tcx > , H : FnMut (ty :: Const < 'tcx >) -> ty :: Const < 'tcx > , { fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } fn fold_ty (& mut self , ty : Ty < 'tcx >) -> Ty < 'tcx > { let t = ty . super_fold_with (self) ; (self . ty_op) (t) } fn fold_region (& mut self , r : ty :: Region < 'tcx >) -> ty :: Region < 'tcx > { (self . lt_op) (r) } fn fold_const (& mut self , ct : ty :: Const < 'tcx >) -> ty :: Const < 'tcx > { let ct = ct . super_fold_with (self) ; (self . ct_op) (ct) } }}}
mkitem!{mktrait!{# [doc = " A delegate used when instantiating bound vars."] # [doc = ""] # [doc = " Any implementation must make sure that each bound variable always"] # [doc = " gets mapped to the same result. `BoundVarReplacer` caches by using"] # [doc = " a `DelayedMap` which does not cache the first few types it encounters."] pub trait BoundVarReplacerDelegate < 'tcx > { fn replace_region (& mut self , br : ty :: BoundRegion) -> ty :: Region < 'tcx > ; fn replace_ty (& mut self , bt : ty :: BoundTy) -> Ty < 'tcx > ; fn replace_const (& mut self , bc : ty :: BoundConst) -> ty :: Const < 'tcx > ; }}}
mkitem!{mkstruct!{# [doc = " A simple delegate taking 3 mutable functions. The used functions must"] # [doc = " always return the same result for each bound variable, no matter how"] # [doc = " frequently they are called."] pub struct FnMutDelegate < 'a , 'tcx > { pub regions : & 'a mut (dyn FnMut (ty :: BoundRegion) -> ty :: Region < 'tcx > + 'a) , pub types : & 'a mut (dyn FnMut (ty :: BoundTy) -> Ty < 'tcx > + 'a) , pub consts : & 'a mut (dyn FnMut (ty :: BoundConst) -> ty :: Const < 'tcx > + 'a) , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > BoundVarReplacerDelegate < 'tcx > for FnMutDelegate < 'a , 'tcx > { fn replace_region (& mut self , br : ty :: BoundRegion) -> ty :: Region < 'tcx > { (self . regions) (br) } fn replace_ty (& mut self , bt : ty :: BoundTy) -> Ty < 'tcx > { (self . types) (bt) } fn replace_const (& mut self , bc : ty :: BoundConst) -> ty :: Const < 'tcx > { (self . consts) (bc) } }}}
mkitem!{mkstruct!{# [doc = " Replaces the escaping bound vars (late bound regions or bound types) in a type."] struct BoundVarReplacer < 'tcx , D > { tcx : TyCtxt < 'tcx > , # [doc = " As with `RegionFolder`, represents the index of a binder *just outside*"] # [doc = " the ones we have visited."] current_index : ty :: DebruijnIndex , delegate : D , # [doc = " This cache only tracks the `DebruijnIndex` and assumes that it does not matter"] # [doc = " for the delegate how often its methods get used."] cache : DelayedMap < (ty :: DebruijnIndex , Ty < 'tcx >) , Ty < 'tcx > > , }}}
mkitem!{mkimpl!{impl < 'tcx , D : BoundVarReplacerDelegate < 'tcx > > BoundVarReplacer < 'tcx , D > { fn new (tcx : TyCtxt < 'tcx > , delegate : D) -> Self { BoundVarReplacer { tcx , current_index : ty :: INNERMOST , delegate , cache : Default :: default () } } }}}
mkitem!{mkimpl!{impl < 'tcx , D > TypeFolder < TyCtxt < 'tcx > > for BoundVarReplacer < 'tcx , D > where D : BoundVarReplacerDelegate < 'tcx > , { fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } fn fold_binder < T : TypeFoldable < TyCtxt < 'tcx > > > (& mut self , t : ty :: Binder < 'tcx , T > ,) -> ty :: Binder < 'tcx , T > { self . current_index . shift_in (1) ; let t = t . super_fold_with (self) ; self . current_index . shift_out (1) ; t } fn fold_ty (& mut self , t : Ty < 'tcx >) -> Ty < 'tcx > { match * t . kind () { ty :: Bound (debruijn , bound_ty) if debruijn == self . current_index => { let ty = self . delegate . replace_ty (bound_ty) ; debug_assert ! (! ty . has_vars_bound_above (ty :: INNERMOST)) ; ty :: shift_vars (self . tcx , ty , self . current_index . as_u32 ()) } _ => { if ! t . has_vars_bound_at_or_above (self . current_index) { t } else if let Some (& t) = self . cache . get (& (self . current_index , t)) { t } else { let res = t . super_fold_with (self) ; assert ! (self . cache . insert ((self . current_index , t) , res)) ; res } } } } fn fold_region (& mut self , r : ty :: Region < 'tcx >) -> ty :: Region < 'tcx > { match r . kind () { ty :: ReBound (debruijn , br) if debruijn == self . current_index => { let region = self . delegate . replace_region (br) ; if let ty :: ReBound (debruijn1 , br) = region . kind () { assert_eq ! (debruijn1 , ty :: INNERMOST) ; ty :: Region :: new_bound (self . tcx , debruijn , br) } else { region } } _ => r , } } fn fold_const (& mut self , ct : ty :: Const < 'tcx >) -> ty :: Const < 'tcx > { match ct . kind () { ty :: ConstKind :: Bound (debruijn , bound_const) if debruijn == self . current_index => { let ct = self . delegate . replace_const (bound_const) ; debug_assert ! (! ct . has_vars_bound_above (ty :: INNERMOST)) ; ty :: shift_vars (self . tcx , ct , self . current_index . as_u32 ()) } _ => ct . super_fold_with (self) , } } fn fold_predicate (& mut self , p : ty :: Predicate < 'tcx >) -> ty :: Predicate < 'tcx > { if p . has_vars_bound_at_or_above (self . current_index) { p . super_fold_with (self) } else { p } } fn fold_clauses (& mut self , c : ty :: Clauses < 'tcx >) -> ty :: Clauses < 'tcx > { if c . has_vars_bound_at_or_above (self . current_index) { c . super_fold_with (self) } else { c } } }}}
mkitem!{mkimpl!{impl < 'tcx > TyCtxt < 'tcx > { # [doc = " Replaces all regions bound by the given `Binder` with the"] # [doc = " results returned by the closure; the closure is expected to"] # [doc = " return a free region (relative to this binder), and hence the"] # [doc = " binder is removed in the return type. The closure is invoked"] # [doc = " once for each unique `BoundRegionKind`; multiple references to the"] # [doc = " same `BoundRegionKind` will reuse the previous result. A map is"] # [doc = " returned at the end with each bound region and the free region"] # [doc = " that replaced it."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This method only replaces late bound regions. Any types or"] # [doc = " constants bound by `value` will cause an ICE."] pub fn instantiate_bound_regions < T , F > (self , value : Binder < 'tcx , T > , mut fld_r : F ,) -> (T , FxIndexMap < ty :: BoundRegion , ty :: Region < 'tcx > >) where F : FnMut (ty :: BoundRegion) -> ty :: Region < 'tcx > , T : TypeFoldable < TyCtxt < 'tcx > > , { let mut region_map = FxIndexMap :: default () ; let real_fld_r = | br : ty :: BoundRegion | * region_map . entry (br) . or_insert_with (| | fld_r (br)) ; let value = self . instantiate_bound_regions_uncached (value , real_fld_r) ; (value , region_map) } pub fn instantiate_bound_regions_uncached < T , F > (self , value : Binder < 'tcx , T > , mut replace_regions : F ,) -> T where F : FnMut (ty :: BoundRegion) -> ty :: Region < 'tcx > , T : TypeFoldable < TyCtxt < 'tcx > > , { let value = value . skip_binder () ; if ! value . has_escaping_bound_vars () { value } else { let delegate = FnMutDelegate { regions : & mut replace_regions , types : & mut | b | bug ! ("unexpected bound ty in binder: {b:?}") , consts : & mut | b | bug ! ("unexpected bound ct in binder: {b:?}") , } ; let mut replacer = BoundVarReplacer :: new (self , delegate) ; value . fold_with (& mut replacer) } } # [doc = " Replaces all escaping bound vars. The `fld_r` closure replaces escaping"] # [doc = " bound regions; the `fld_t` closure replaces escaping bound types and the `fld_c`"] # [doc = " closure replaces escaping bound consts."] pub fn replace_escaping_bound_vars_uncached < T : TypeFoldable < TyCtxt < 'tcx > > > (self , value : T , delegate : impl BoundVarReplacerDelegate < 'tcx > ,) -> T { if ! value . has_escaping_bound_vars () { value } else { let mut replacer = BoundVarReplacer :: new (self , delegate) ; value . fold_with (& mut replacer) } } # [doc = " Replaces all types or regions bound by the given `Binder`. The `fld_r`"] # [doc = " closure replaces bound regions, the `fld_t` closure replaces bound"] # [doc = " types, and `fld_c` replaces bound constants."] pub fn replace_bound_vars_uncached < T : TypeFoldable < TyCtxt < 'tcx > > > (self , value : Binder < 'tcx , T > , delegate : impl BoundVarReplacerDelegate < 'tcx > ,) -> T { self . replace_escaping_bound_vars_uncached (value . skip_binder () , delegate) } # [doc = " Replaces any late-bound regions bound in `value` with"] # [doc = " free variants attached to `all_outlive_scope`."] pub fn liberate_late_bound_regions < T > (self , all_outlive_scope : DefId , value : ty :: Binder < 'tcx , T > ,) -> T where T : TypeFoldable < TyCtxt < 'tcx > > , { self . instantiate_bound_regions_uncached (value , | br | { let kind = ty :: LateParamRegionKind :: from_bound (br . var , br . kind) ; ty :: Region :: new_late_param (self , all_outlive_scope , kind) }) } pub fn shift_bound_var_indices < T > (self , bound_vars : usize , value : T) -> T where T : TypeFoldable < TyCtxt < 'tcx > > , { let shift_bv = | bv : ty :: BoundVar | bv + bound_vars ; self . replace_escaping_bound_vars_uncached (value , FnMutDelegate { regions : & mut | r : ty :: BoundRegion | { ty :: Region :: new_bound (self , ty :: INNERMOST , ty :: BoundRegion { var : shift_bv (r . var) , kind : r . kind } ,) } , types : & mut | t : ty :: BoundTy | { Ty :: new_bound (self , ty :: INNERMOST , ty :: BoundTy { var : shift_bv (t . var) , kind : t . kind } ,) } , consts : & mut | c | { ty :: Const :: new_bound (self , ty :: INNERMOST , ty :: BoundConst { var : shift_bv (c . var) } ,) } , } ,) } # [doc = " Replaces any late-bound regions bound in `value` with `'erased`. Useful in codegen but also"] # [doc = " method lookup and a few other places where precise region relationships are not required."] pub fn instantiate_bound_regions_with_erased < T > (self , value : Binder < 'tcx , T >) -> T where T : TypeFoldable < TyCtxt < 'tcx > > , { self . instantiate_bound_regions (value , | _ | self . lifetimes . re_erased) . 0 } # [doc = " Anonymize all bound variables in `value`, this is mostly used to improve caching."] pub fn anonymize_bound_vars < T > (self , value : Binder < 'tcx , T >) -> Binder < 'tcx , T > where T : TypeFoldable < TyCtxt < 'tcx > > , { struct Anonymize < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , map : & 'a mut FxIndexMap < ty :: BoundVar , ty :: BoundVariableKind > , } impl < 'tcx > BoundVarReplacerDelegate < 'tcx > for Anonymize < '_ , 'tcx > { fn replace_region (& mut self , br : ty :: BoundRegion) -> ty :: Region < 'tcx > { let entry = self . map . entry (br . var) ; let index = entry . index () ; let var = ty :: BoundVar :: from_usize (index) ; let kind = entry . or_insert_with (| | ty :: BoundVariableKind :: Region (ty :: BoundRegionKind :: Anon)) . expect_region () ; let br = ty :: BoundRegion { var , kind } ; ty :: Region :: new_bound (self . tcx , ty :: INNERMOST , br) } fn replace_ty (& mut self , bt : ty :: BoundTy) -> Ty < 'tcx > { let entry = self . map . entry (bt . var) ; let index = entry . index () ; let var = ty :: BoundVar :: from_usize (index) ; let kind = entry . or_insert_with (| | ty :: BoundVariableKind :: Ty (ty :: BoundTyKind :: Anon)) . expect_ty () ; Ty :: new_bound (self . tcx , ty :: INNERMOST , BoundTy { var , kind }) } fn replace_const (& mut self , bc : ty :: BoundConst) -> ty :: Const < 'tcx > { let entry = self . map . entry (bc . var) ; let index = entry . index () ; let var = ty :: BoundVar :: from_usize (index) ; let () = entry . or_insert_with (| | ty :: BoundVariableKind :: Const) . expect_const () ; ty :: Const :: new_bound (self . tcx , ty :: INNERMOST , BoundConst { var }) } } let mut map = Default :: default () ; let delegate = Anonymize { tcx : self , map : & mut map } ; let inner = self . replace_escaping_bound_vars_uncached (value . skip_binder () , delegate) ; let bound_vars = self . mk_bound_variable_kinds_from_iter (map . into_values ()) ; Binder :: bind_with_vars (inner , bound_vars) } }}}