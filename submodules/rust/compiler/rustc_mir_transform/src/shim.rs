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
mkuse!{use std :: assert_matches :: assert_matches ;}
mkuse!{use std :: { fmt , iter } ;}
mkuse!{use rustc_abi :: { ExternAbi , FIRST_VARIANT , FieldIdx , VariantIdx } ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_hir :: lang_items :: LangItem ;}
mkuse!{use rustc_index :: { Idx , IndexVec } ;}
mkuse!{use rustc_middle :: mir :: visit :: { MutVisitor , PlaceContext } ;}
mkuse!{use rustc_middle :: mir :: * ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: { self , CoroutineArgs , CoroutineArgsExt , EarlyBinder , GenericArgs , Ty , TyCtxt , } ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_span :: source_map :: { Spanned , dummy_spanned } ;}
mkuse!{use rustc_span :: { DUMMY_SP , Span } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use crate :: elaborate_drop :: { DropElaborator , DropFlagMode , DropStyle , Unwind , elaborate_drop } ;}
mkuse!{use crate :: patch :: MirPatch ;}
mkuse!{use crate :: { abort_unwinding_calls , add_call_guards , add_moves_for_packed_drops , deref_separator , inline , instsimplify , mentioned_items , pass_manager as pm , remove_noop_landing_pads , run_optimization_passes , simplify , } ;}
mkmod!{async_destructor_ctor, { 
                getname!(async_destructor_ctor);
                getsrc!(async_destructor_ctor);
                getpath!(async_destructor_ctor);
                get_deps!(async_destructor_ctor);
                get_crates!(async_destructor_ctor);
                mkinclude!(async_destructor_ctor);
                 
            }}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (super) fn provide (providers : & mut Providers) { providers . mir_shims = make_shim ; }
}
mkitem!{mkstruct!{struct FixProxyFutureDropVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , replace_to : Local , }}}
mkitem!{mkimpl!{impl < 'tcx > MutVisitor < 'tcx > for FixProxyFutureDropVisitor < 'tcx > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn visit_place (& mut self , place : & mut Place < 'tcx > , _context : PlaceContext , _location : Location ,) { if place . local == Local :: from_u32 (1) { if place . projection . len () == 1 { assert ! (matches ! (place . projection . first () , Some (ProjectionElem :: Field (FieldIdx :: ZERO , _)))) ; * place = Place :: from (self . replace_to) ; } else if place . projection . len () == 2 { assert ! (matches ! (place . projection [0] , ProjectionElem :: Field (FieldIdx :: ZERO , _))) ; assert ! (matches ! (place . projection [1] , ProjectionElem :: Deref)) ; * place = Place :: from (self . replace_to) . project_deeper (& [ProjectionElem :: Deref] , self . tcx) ; } } } }}}

macro_rules! make_shim_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_shim in module {}", module_path!());
    };
}

mkfn!{
    make_shim_introspect!();
    fn make_shim < 'tcx > (tcx : TyCtxt < 'tcx > , instance : ty :: InstanceKind < 'tcx >) -> Body < 'tcx > { debug ! ("make_shim({:?})" , instance) ; let mut result = match instance { ty :: InstanceKind :: Item (..) => bug ! ("item {:?} passed to make_shim" , instance) , ty :: InstanceKind :: VTableShim (def_id) => { let adjustment = Adjustment :: Deref { source : DerefSource :: MutPtr } ; build_call_shim (tcx , instance , Some (adjustment) , CallKind :: Direct (def_id)) } ty :: InstanceKind :: FnPtrShim (def_id , ty) => { let trait_ = tcx . parent (def_id) ; let adjustment = match tcx . fn_trait_kind_from_def_id (trait_) . or_else (| | tcx . async_fn_trait_kind_from_def_id (trait_)) { Some (ty :: ClosureKind :: FnOnce) => Adjustment :: Identity , Some (ty :: ClosureKind :: Fn) => Adjustment :: Deref { source : DerefSource :: ImmRef } , Some (ty :: ClosureKind :: FnMut) => Adjustment :: Deref { source : DerefSource :: MutRef } , None => bug ! ("fn pointer {:?} is not an fn" , ty) , } ; build_call_shim (tcx , instance , Some (adjustment) , CallKind :: Indirect (ty)) } ty :: InstanceKind :: ReifyShim (def_id , _) => { build_call_shim (tcx , instance , None , CallKind :: Direct (def_id)) } ty :: InstanceKind :: ClosureOnceShim { call_once : _ , track_caller : _ } => { let fn_mut = tcx . require_lang_item (LangItem :: FnMut , DUMMY_SP) ; let call_mut = tcx . associated_items (fn_mut) . in_definition_order () . find (| it | it . is_fn ()) . unwrap () . def_id ; build_call_shim (tcx , instance , Some (Adjustment :: RefMut) , CallKind :: Direct (call_mut)) } ty :: InstanceKind :: ConstructCoroutineInClosureShim { coroutine_closure_def_id , receiver_by_ref , } => build_construct_coroutine_by_move_shim (tcx , coroutine_closure_def_id , receiver_by_ref) , ty :: InstanceKind :: DropGlue (def_id , ty) => { if let Some (& ty :: Coroutine (coroutine_def_id , args)) = ty . map (Ty :: kind) { let coroutine_body = tcx . optimized_mir (coroutine_def_id) ; let ty :: Coroutine (_ , id_args) = * tcx . type_of (coroutine_def_id) . skip_binder () . kind () else { bug ! () } ; let body = if id_args . as_coroutine () . kind_ty () == args . as_coroutine () . kind_ty () { coroutine_body . coroutine_drop () . unwrap () } else { assert_eq ! (args . as_coroutine () . kind_ty () . to_opt_closure_kind () . unwrap () , ty :: ClosureKind :: FnOnce) ; tcx . optimized_mir (tcx . coroutine_by_move_body_def_id (coroutine_def_id)) . coroutine_drop () . unwrap () } ; let mut body = EarlyBinder :: bind (body . clone ()) . instantiate (tcx , args) ; debug ! ("make_shim({:?}) = {:?}" , instance , body) ; pm :: run_passes (tcx , & mut body , & [& mentioned_items :: MentionedItems , & abort_unwinding_calls :: AbortUnwindingCalls , & add_call_guards :: CriticalCallEdges ,] , Some (MirPhase :: Runtime (RuntimePhase :: Optimized)) , pm :: Optimizations :: Allowed ,) ; return body ; } build_drop_shim (tcx , def_id , ty) } ty :: InstanceKind :: ThreadLocalShim (..) => build_thread_local_shim (tcx , instance) , ty :: InstanceKind :: CloneShim (def_id , ty) => build_clone_shim (tcx , def_id , ty) , ty :: InstanceKind :: FnPtrAddrShim (def_id , ty) => build_fn_ptr_addr_shim (tcx , def_id , ty) , ty :: InstanceKind :: FutureDropPollShim (def_id , proxy_ty , impl_ty) => { let mut body = async_destructor_ctor :: build_future_drop_poll_shim (tcx , def_id , proxy_ty , impl_ty) ; pm :: run_passes (tcx , & mut body , & [& mentioned_items :: MentionedItems , & abort_unwinding_calls :: AbortUnwindingCalls , & add_call_guards :: CriticalCallEdges ,] , Some (MirPhase :: Runtime (RuntimePhase :: PostCleanup)) , pm :: Optimizations :: Allowed ,) ; run_optimization_passes (tcx , & mut body) ; debug ! ("make_shim({:?}) = {:?}" , instance , body) ; return body ; } ty :: InstanceKind :: AsyncDropGlue (def_id , ty) => { let mut body = async_destructor_ctor :: build_async_drop_shim (tcx , def_id , ty) ; pm :: run_passes (tcx , & mut body , & [& mentioned_items :: MentionedItems , & abort_unwinding_calls :: AbortUnwindingCalls , & add_call_guards :: CriticalCallEdges , & simplify :: SimplifyCfg :: MakeShim , & crate :: coroutine :: StateTransform ,] , Some (MirPhase :: Runtime (RuntimePhase :: PostCleanup)) , pm :: Optimizations :: Allowed ,) ; run_optimization_passes (tcx , & mut body) ; debug ! ("make_shim({:?}) = {:?}" , instance , body) ; return body ; } ty :: InstanceKind :: AsyncDropGlueCtorShim (def_id , ty) => { let body = async_destructor_ctor :: build_async_destructor_ctor_shim (tcx , def_id , ty) ; debug ! ("make_shim({:?}) = {:?}" , instance , body) ; return body ; } ty :: InstanceKind :: Virtual (..) => { bug ! ("InstanceKind::Virtual ({:?}) is for direct calls only" , instance) } ty :: InstanceKind :: Intrinsic (_) => { bug ! ("creating shims from intrinsics ({:?}) is unsupported" , instance) } } ; debug ! ("make_shim({:?}) = untransformed {:?}" , instance , result) ; pm :: run_passes_no_validate (tcx , & mut result , & [& mentioned_items :: MentionedItems , & add_moves_for_packed_drops :: AddMovesForPackedDrops , & deref_separator :: Derefer , & remove_noop_landing_pads :: RemoveNoopLandingPads , & simplify :: SimplifyCfg :: MakeShim , & instsimplify :: InstSimplify :: BeforeInline , & inline :: ForceInline , & abort_unwinding_calls :: AbortUnwindingCalls , & add_call_guards :: CriticalCallEdges ,] , Some (MirPhase :: Runtime (RuntimePhase :: Optimized)) ,) ; debug ! ("make_shim({:?}) = {:?}" , instance , result) ; result }
}
mkitem!{mkenum!{# [derive (Copy , Clone , Debug , PartialEq)] enum DerefSource { # [doc = " `fn shim(&self) { inner(*self )}`."] ImmRef , # [doc = " `fn shim(&mut self) { inner(*self )}`."] MutRef , # [doc = " `fn shim(*mut self) { inner(*self )}`."] MutPtr , }}}
mkitem!{mkenum!{# [derive (Copy , Clone , Debug , PartialEq)] enum Adjustment { # [doc = " Pass the receiver as-is."] Identity , # [doc = " We get passed a reference or a raw pointer to `self` and call the target with `*self`."] # [doc = ""] # [doc = " This either copies `self` (if `Self: Copy`, eg. for function items), or moves out of it"] # [doc = " (for `VTableShim`, which effectively is passed `&own Self`)."] Deref { source : DerefSource } , # [doc = " We get passed `self: Self` and call the target with `&mut self`."] # [doc = ""] # [doc = " In this case we need to ensure that the `Self` is dropped after the call, as the callee"] # [doc = " won't do it for us."] RefMut , }}}
mkitem!{mkenum!{# [derive (Copy , Clone , Debug , PartialEq)] enum CallKind < 'tcx > { # [doc = " Call the `FnPtr` that was passed as the receiver."] Indirect (Ty < 'tcx >) , # [doc = " Call a known `FnDef`."] Direct (DefId) , }}}

macro_rules! local_decls_for_sig_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function local_decls_for_sig in module {}", module_path!());
    };
}

mkfn!{
    local_decls_for_sig_introspect!();
    fn local_decls_for_sig < 'tcx > (sig : & ty :: FnSig < 'tcx > , span : Span ,) -> IndexVec < Local , LocalDecl < 'tcx > > { iter :: once (LocalDecl :: new (sig . output () , span)) . chain (sig . inputs () . iter () . map (| ity | LocalDecl :: new (* ity , span) . immutable ())) . collect () }
}

macro_rules! dropee_emit_retag_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dropee_emit_retag in module {}", module_path!());
    };
}

mkfn!{
    dropee_emit_retag_introspect!();
    fn dropee_emit_retag < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx > , mut dropee_ptr : Place < 'tcx > , span : Span ,) -> Place < 'tcx > { if tcx . sess . opts . unstable_opts . mir_emit_retag { let source_info = SourceInfo :: outermost (span) ; let reborrow = Rvalue :: Ref (tcx . lifetimes . re_erased , BorrowKind :: Mut { kind : MutBorrowKind :: Default } , tcx . mk_place_deref (dropee_ptr) ,) ; let ref_ty = reborrow . ty (body . local_decls () , tcx) ; dropee_ptr = body . local_decls . push (LocalDecl :: new (ref_ty , span)) . into () ; let new_statements = [StatementKind :: Assign (Box :: new ((dropee_ptr , reborrow))) , StatementKind :: Retag (RetagKind :: FnEntry , Box :: new (dropee_ptr)) ,] ; for s in new_statements { body . basic_blocks_mut () [START_BLOCK] . statements . push (Statement :: new (source_info , s)) ; } } dropee_ptr }
}

macro_rules! build_drop_shim_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_drop_shim in module {}", module_path!());
    };
}

mkfn!{
    build_drop_shim_introspect!();
    fn build_drop_shim < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId , ty : Option < Ty < 'tcx > >) -> Body < 'tcx > { debug ! ("build_drop_shim(def_id={:?}, ty={:?})" , def_id , ty) ; assert ! (! matches ! (ty , Some (ty) if ty . is_coroutine ())) ; let args = if let Some (ty) = ty { tcx . mk_args (& [ty . into ()]) } else { GenericArgs :: identity_for_item (tcx , def_id) } ; let sig = tcx . fn_sig (def_id) . instantiate (tcx , args) ; let sig = tcx . instantiate_bound_regions_with_erased (sig) ; let span = tcx . def_span (def_id) ; let source_info = SourceInfo :: outermost (span) ; let return_block = BasicBlock :: new (1) ; let mut blocks = IndexVec :: with_capacity (2) ; let block = | blocks : & mut IndexVec < _ , _ > , kind | { blocks . push (BasicBlockData :: new (Some (Terminator { source_info , kind }) , false)) } ; block (& mut blocks , TerminatorKind :: Goto { target : return_block }) ; block (& mut blocks , TerminatorKind :: Return) ; let source = MirSource :: from_instance (ty :: InstanceKind :: DropGlue (def_id , ty)) ; let mut body = new_body (source , blocks , local_decls_for_sig (& sig , span) , sig . inputs () . len () , span) ; let dropee_ptr = Place :: from (Local :: new (1 + 0)) ; let dropee_ptr = dropee_emit_retag (tcx , & mut body , dropee_ptr , span) ; if ty . is_some () { let patch = { let typing_env = ty :: TypingEnv :: post_analysis (tcx , def_id) ; let mut elaborator = DropShimElaborator { body : & body , patch : MirPatch :: new (& body) , tcx , typing_env , produce_async_drops : false , } ; let dropee = tcx . mk_place_deref (dropee_ptr) ; let resume_block = elaborator . patch . resume_block () ; elaborate_drop (& mut elaborator , source_info , dropee , () , return_block , Unwind :: To (resume_block) , START_BLOCK , None ,) ; elaborator . patch } ; patch . apply (& mut body) ; } body }
}

macro_rules! new_body_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function new_body in module {}", module_path!());
    };
}

mkfn!{
    new_body_introspect!();
    fn new_body < 'tcx > (source : MirSource < 'tcx > , basic_blocks : IndexVec < BasicBlock , BasicBlockData < 'tcx > > , local_decls : IndexVec < Local , LocalDecl < 'tcx > > , arg_count : usize , span : Span ,) -> Body < 'tcx > { let mut body = Body :: new (source , basic_blocks , IndexVec :: from_elem_n (SourceScopeData { span , parent_scope : None , inlined : None , inlined_parent_scope : None , local_data : ClearCrossCrate :: Clear , } , 1 ,) , local_decls , IndexVec :: new () , arg_count , vec ! [] , span , None , None ,) ; body . set_required_consts (Vec :: new ()) ; body }
}
mkitem!{mkstruct!{pub (super) struct DropShimElaborator < 'a , 'tcx > { pub body : & 'a Body < 'tcx > , pub patch : MirPatch < 'tcx > , pub tcx : TyCtxt < 'tcx > , pub typing_env : ty :: TypingEnv < 'tcx > , pub produce_async_drops : bool , }}}
mkitem!{mkimpl!{impl fmt :: Debug for DropShimElaborator < '_ , '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . debug_struct ("DropShimElaborator") . finish_non_exhaustive () } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > DropElaborator < 'a , 'tcx > for DropShimElaborator < 'a , 'tcx > { type Path = () ; fn patch_ref (& self) -> & MirPatch < 'tcx > { & self . patch } fn patch (& mut self) -> & mut MirPatch < 'tcx > { & mut self . patch } fn body (& self) -> & 'a Body < 'tcx > { self . body } fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn typing_env (& self) -> ty :: TypingEnv < 'tcx > { self . typing_env } fn terminator_loc (& self , bb : BasicBlock) -> Location { self . patch . terminator_loc (self . body , bb) } fn allow_async_drops (& self) -> bool { self . produce_async_drops } fn drop_style (& self , _path : Self :: Path , mode : DropFlagMode) -> DropStyle { match mode { DropFlagMode :: Shallow => { DropStyle :: Static } DropFlagMode :: Deep => { DropStyle :: Open } } } fn get_drop_flag (& mut self , _path : Self :: Path) -> Option < Operand < 'tcx > > { None } fn clear_drop_flag (& mut self , _location : Location , _path : Self :: Path , _mode : DropFlagMode) { } fn field_subpath (& self , _path : Self :: Path , _field : FieldIdx) -> Option < Self :: Path > { None } fn deref_subpath (& self , _path : Self :: Path) -> Option < Self :: Path > { None } fn downcast_subpath (& self , _path : Self :: Path , _variant : VariantIdx) -> Option < Self :: Path > { Some (()) } fn array_subpath (& self , _path : Self :: Path , _index : u64 , _size : u64) -> Option < Self :: Path > { None } }}}

macro_rules! build_thread_local_shim_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_thread_local_shim in module {}", module_path!());
    };
}

mkfn!{
    build_thread_local_shim_introspect!();
    fn build_thread_local_shim < 'tcx > (tcx : TyCtxt < 'tcx > , instance : ty :: InstanceKind < 'tcx > ,) -> Body < 'tcx > { let def_id = instance . def_id () ; let span = tcx . def_span (def_id) ; let source_info = SourceInfo :: outermost (span) ; let blocks = IndexVec :: from_raw (vec ! [BasicBlockData :: new_stmts (vec ! [Statement :: new (source_info , StatementKind :: Assign (Box :: new ((Place :: return_place () , Rvalue :: ThreadLocalRef (def_id) ,))) ,)] , Some (Terminator { source_info , kind : TerminatorKind :: Return }) , false ,)]) ; new_body (MirSource :: from_instance (instance) , blocks , IndexVec :: from_raw (vec ! [LocalDecl :: new (tcx . thread_local_ptr_ty (def_id) , span)]) , 0 , span ,) }
}

macro_rules! build_clone_shim_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_clone_shim in module {}", module_path!());
    };
}

mkfn!{
    build_clone_shim_introspect!();
    # [doc = " Builds a `Clone::clone` shim for `self_ty`. Here, `def_id` is `Clone::clone`."] fn build_clone_shim < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId , self_ty : Ty < 'tcx >) -> Body < 'tcx > { debug ! ("build_clone_shim(def_id={:?})" , def_id) ; let mut builder = CloneShimBuilder :: new (tcx , def_id , self_ty) ; let dest = Place :: return_place () ; let src = tcx . mk_place_deref (Place :: from (Local :: new (1 + 0))) ; match self_ty . kind () { ty :: FnDef (..) | ty :: FnPtr (..) => builder . copy_shim () , ty :: Closure (_ , args) => builder . tuple_like_shim (dest , src , args . as_closure () . upvar_tys ()) , ty :: CoroutineClosure (_ , args) => { builder . tuple_like_shim (dest , src , args . as_coroutine_closure () . upvar_tys ()) } ty :: Tuple (..) => builder . tuple_like_shim (dest , src , self_ty . tuple_fields ()) , ty :: Coroutine (coroutine_def_id , args) => { assert_eq ! (tcx . coroutine_movability (* coroutine_def_id) , hir :: Movability :: Movable) ; builder . coroutine_shim (dest , src , * coroutine_def_id , args . as_coroutine ()) } _ => bug ! ("clone shim for `{:?}` which is not `Copy` and is not an aggregate" , self_ty) , } ; builder . into_mir () }
}
mkitem!{mkstruct!{struct CloneShimBuilder < 'tcx > { tcx : TyCtxt < 'tcx > , def_id : DefId , local_decls : IndexVec < Local , LocalDecl < 'tcx > > , blocks : IndexVec < BasicBlock , BasicBlockData < 'tcx > > , span : Span , sig : ty :: FnSig < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > CloneShimBuilder < 'tcx > { fn new (tcx : TyCtxt < 'tcx > , def_id : DefId , self_ty : Ty < 'tcx >) -> Self { let sig = tcx . fn_sig (def_id) . instantiate (tcx , & [self_ty . into ()]) ; let sig = tcx . instantiate_bound_regions_with_erased (sig) ; let span = tcx . def_span (def_id) ; CloneShimBuilder { tcx , def_id , local_decls : local_decls_for_sig (& sig , span) , blocks : IndexVec :: new () , span , sig , } } fn into_mir (self) -> Body < 'tcx > { let source = MirSource :: from_instance (ty :: InstanceKind :: CloneShim (self . def_id , self . sig . inputs_and_output [0] ,)) ; new_body (source , self . blocks , self . local_decls , self . sig . inputs () . len () , self . span) } fn source_info (& self) -> SourceInfo { SourceInfo :: outermost (self . span) } fn block (& mut self , statements : Vec < Statement < 'tcx > > , kind : TerminatorKind < 'tcx > , is_cleanup : bool ,) -> BasicBlock { let source_info = self . source_info () ; self . blocks . push (BasicBlockData :: new_stmts (statements , Some (Terminator { source_info , kind }) , is_cleanup ,)) } # [doc = " Gives the index of an upcoming BasicBlock, with an offset."] # [doc = " offset=0 will give you the index of the next BasicBlock,"] # [doc = " offset=1 will give the index of the next-to-next block,"] # [doc = " offset=-1 will give you the index of the last-created block"] fn block_index_offset (& self , offset : usize) -> BasicBlock { BasicBlock :: new (self . blocks . len () + offset) } fn make_statement (& self , kind : StatementKind < 'tcx >) -> Statement < 'tcx > { Statement :: new (self . source_info () , kind) } fn copy_shim (& mut self) { let rcvr = self . tcx . mk_place_deref (Place :: from (Local :: new (1 + 0))) ; let ret_statement = self . make_statement (StatementKind :: Assign (Box :: new ((Place :: return_place () , Rvalue :: Use (Operand :: Copy (rcvr)) ,)))) ; self . block (vec ! [ret_statement] , TerminatorKind :: Return , false) ; } fn make_place (& mut self , mutability : Mutability , ty : Ty < 'tcx >) -> Place < 'tcx > { let span = self . span ; let mut local = LocalDecl :: new (ty , span) ; if mutability . is_not () { local = local . immutable () ; } Place :: from (self . local_decls . push (local)) } fn make_clone_call (& mut self , dest : Place < 'tcx > , src : Place < 'tcx > , ty : Ty < 'tcx > , next : BasicBlock , cleanup : BasicBlock ,) { let tcx = self . tcx ; let func_ty = Ty :: new_fn_def (tcx , self . def_id , [ty]) ; let func = Operand :: Constant (Box :: new (ConstOperand { span : self . span , user_ty : None , const_ : Const :: zero_sized (func_ty) , })) ; let ref_loc = self . make_place (Mutability :: Not , Ty :: new_imm_ref (tcx , tcx . lifetimes . re_erased , ty)) ; let statement = self . make_statement (StatementKind :: Assign (Box :: new ((ref_loc , Rvalue :: Ref (tcx . lifetimes . re_erased , BorrowKind :: Shared , src) ,)))) ; self . block (vec ! [statement] , TerminatorKind :: Call { func , args : [Spanned { node : Operand :: Move (ref_loc) , span : DUMMY_SP }] . into () , destination : dest , target : Some (next) , unwind : UnwindAction :: Cleanup (cleanup) , call_source : CallSource :: Normal , fn_span : self . span , } , false ,) ; } fn clone_fields < I > (& mut self , dest : Place < 'tcx > , src : Place < 'tcx > , target : BasicBlock , mut unwind : BasicBlock , tys : I ,) -> BasicBlock where I : IntoIterator < Item = Ty < 'tcx > > , { for (i , ity) in tys . into_iter () . enumerate () { let field = FieldIdx :: new (i) ; let src_field = self . tcx . mk_place_field (src , field , ity) ; let dest_field = self . tcx . mk_place_field (dest , field , ity) ; let next_unwind = self . block_index_offset (1) ; let next_block = self . block_index_offset (2) ; self . make_clone_call (dest_field , src_field , ity , next_block , unwind) ; self . block (vec ! [] , TerminatorKind :: Drop { place : dest_field , target : unwind , unwind : UnwindAction :: Terminate (UnwindTerminateReason :: InCleanup) , replace : false , drop : None , async_fut : None , } , true ,) ; unwind = next_unwind ; } self . block (vec ! [] , TerminatorKind :: Goto { target } , false) ; unwind } fn tuple_like_shim < I > (& mut self , dest : Place < 'tcx > , src : Place < 'tcx > , tys : I) where I : IntoIterator < Item = Ty < 'tcx > > , { self . block (vec ! [] , TerminatorKind :: Goto { target : self . block_index_offset (3) } , false) ; let unwind = self . block (vec ! [] , TerminatorKind :: UnwindResume , true) ; let target = self . block (vec ! [] , TerminatorKind :: Return , false) ; let _final_cleanup_block = self . clone_fields (dest , src , target , unwind , tys) ; } fn coroutine_shim (& mut self , dest : Place < 'tcx > , src : Place < 'tcx > , coroutine_def_id : DefId , args : CoroutineArgs < TyCtxt < 'tcx > > ,) { self . block (vec ! [] , TerminatorKind :: Goto { target : self . block_index_offset (3) } , false) ; let unwind = self . block (vec ! [] , TerminatorKind :: UnwindResume , true) ; let switch = self . block (vec ! [] , TerminatorKind :: Unreachable , false) ; let unwind = self . clone_fields (dest , src , switch , unwind , args . upvar_tys ()) ; let target = self . block (vec ! [] , TerminatorKind :: Return , false) ; let unreachable = self . block (vec ! [] , TerminatorKind :: Unreachable , false) ; let mut cases = Vec :: with_capacity (args . state_tys (coroutine_def_id , self . tcx) . count ()) ; for (index , state_tys) in args . state_tys (coroutine_def_id , self . tcx) . enumerate () { let variant_index = VariantIdx :: new (index) ; let dest = self . tcx . mk_place_downcast_unnamed (dest , variant_index) ; let src = self . tcx . mk_place_downcast_unnamed (src , variant_index) ; let clone_block = self . block_index_offset (1) ; let start_block = self . block (vec ! [self . make_statement (StatementKind :: SetDiscriminant { place : Box :: new (Place :: return_place ()) , variant_index , })] , TerminatorKind :: Goto { target : clone_block } , false ,) ; cases . push ((index as u128 , start_block)) ; let _final_cleanup_block = self . clone_fields (dest , src , target , unwind , state_tys) ; } let discr_ty = args . discr_ty (self . tcx) ; let temp = self . make_place (Mutability :: Mut , discr_ty) ; let rvalue = Rvalue :: Discriminant (src) ; let statement = self . make_statement (StatementKind :: Assign (Box :: new ((temp , rvalue)))) ; match & mut self . blocks [switch] { BasicBlockData { statements , terminator : Some (Terminator { kind , .. }) , .. } => { statements . push (statement) ; * kind = TerminatorKind :: SwitchInt { discr : Operand :: Move (temp) , targets : SwitchTargets :: new (cases . into_iter () , unreachable) , } ; } BasicBlockData { terminator : None , .. } => unreachable ! () , } } }}}

macro_rules! build_call_shim_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_call_shim in module {}", module_path!());
    };
}

mkfn!{
    build_call_shim_introspect!();
    # [doc = " Builds a \"call\" shim for `instance`. The shim calls the function specified by `call_kind`,"] # [doc = " first adjusting its first argument according to `rcvr_adjustment`."] # [instrument (level = "debug" , skip (tcx) , ret)] fn build_call_shim < 'tcx > (tcx : TyCtxt < 'tcx > , instance : ty :: InstanceKind < 'tcx > , rcvr_adjustment : Option < Adjustment > , call_kind : CallKind < 'tcx > ,) -> Body < 'tcx > { let (sig_args , untuple_args) = if let ty :: InstanceKind :: FnPtrShim (_ , ty) = instance { let sig = tcx . instantiate_bound_regions_with_erased (ty . fn_sig (tcx)) ; let untuple_args = sig . inputs () ; let arg_tup = Ty :: new_tup (tcx , untuple_args) ; (Some ([ty . into () , arg_tup . into ()]) , Some (untuple_args)) } else { (None , None) } ; let def_id = instance . def_id () ; let sig = tcx . fn_sig (def_id) ; let sig = sig . map_bound (| sig | tcx . instantiate_bound_regions_with_erased (sig)) ; assert_eq ! (sig_args . is_some () , ! instance . has_polymorphic_mir_body ()) ; let mut sig = if let Some (sig_args) = sig_args { sig . instantiate (tcx , & sig_args) } else { sig . instantiate_identity () } ; if let CallKind :: Indirect (fnty) = call_kind { let mut inputs_and_output = sig . inputs_and_output . to_vec () ; assert_eq ! (inputs_and_output . len () , 3) ; let self_arg = & mut inputs_and_output [0] ; * self_arg = match rcvr_adjustment . unwrap () { Adjustment :: Identity => fnty , Adjustment :: Deref { source } => match source { DerefSource :: ImmRef => Ty :: new_imm_ref (tcx , tcx . lifetimes . re_erased , fnty) , DerefSource :: MutRef => Ty :: new_mut_ref (tcx , tcx . lifetimes . re_erased , fnty) , DerefSource :: MutPtr => Ty :: new_mut_ptr (tcx , fnty) , } , Adjustment :: RefMut => bug ! ("`RefMut` is never used with indirect calls: {instance:?}") , } ; sig . inputs_and_output = tcx . mk_type_list (& inputs_and_output) ; } if let ty :: InstanceKind :: VTableShim (..) = instance { let mut inputs_and_output = sig . inputs_and_output . to_vec () ; let self_arg = & mut inputs_and_output [0] ; debug_assert ! (tcx . generics_of (def_id) . has_self && * self_arg == tcx . types . self_param) ; * self_arg = Ty :: new_mut_ptr (tcx , * self_arg) ; sig . inputs_and_output = tcx . mk_type_list (& inputs_and_output) ; } let span = tcx . def_span (def_id) ; debug ! (? sig) ; let mut local_decls = local_decls_for_sig (& sig , span) ; let source_info = SourceInfo :: outermost (span) ; let destination = Place :: return_place () ; let rcvr_place = | | { assert ! (rcvr_adjustment . is_some ()) ; Place :: from (Local :: new (1)) } ; let mut statements = vec ! [] ; let rcvr = rcvr_adjustment . map (| rcvr_adjustment | match rcvr_adjustment { Adjustment :: Identity => Operand :: Move (rcvr_place ()) , Adjustment :: Deref { source : _ } => Operand :: Move (tcx . mk_place_deref (rcvr_place ())) , Adjustment :: RefMut => { let ref_rcvr = local_decls . push (LocalDecl :: new (Ty :: new_mut_ref (tcx , tcx . lifetimes . re_erased , sig . inputs () [0]) , span ,) . immutable () ,) ; let borrow_kind = BorrowKind :: Mut { kind : MutBorrowKind :: Default } ; statements . push (Statement :: new (source_info , StatementKind :: Assign (Box :: new ((Place :: from (ref_rcvr) , Rvalue :: Ref (tcx . lifetimes . re_erased , borrow_kind , rcvr_place ()) ,))) ,)) ; Operand :: Move (Place :: from (ref_rcvr)) } }) ; let (callee , mut args) = match call_kind { CallKind :: Indirect (_) => (rcvr . unwrap () , vec ! []) , CallKind :: Direct (def_id) => { let ty = tcx . type_of (def_id) . instantiate_identity () ; (Operand :: Constant (Box :: new (ConstOperand { span , user_ty : None , const_ : Const :: zero_sized (ty) , })) , rcvr . into_iter () . collect :: < Vec < _ > > () ,) } } ; let mut arg_range = 0 .. sig . inputs () . len () ; if rcvr_adjustment . is_some () { arg_range . start += 1 ; } if untuple_args . is_some () { arg_range . end -= 1 ; } args . extend (arg_range . map (| i | Operand :: Move (Place :: from (Local :: new (1 + i))))) ; if let Some (untuple_args) = untuple_args { let tuple_arg = Local :: new (1 + (sig . inputs () . len () - 1)) ; args . extend (untuple_args . iter () . enumerate () . map (| (i , ity) | { Operand :: Move (tcx . mk_place_field (Place :: from (tuple_arg) , FieldIdx :: new (i) , * ity)) })) ; } let n_blocks = if let Some (Adjustment :: RefMut) = rcvr_adjustment { 5 } else { 2 } ; let mut blocks = IndexVec :: with_capacity (n_blocks) ; let block = | blocks : & mut IndexVec < _ , _ > , statements , kind , is_cleanup | { blocks . push (BasicBlockData :: new_stmts (statements , Some (Terminator { source_info , kind }) , is_cleanup ,)) } ; let args = args . into_iter () . map (| a | Spanned { node : a , span : DUMMY_SP }) . collect () ; block (& mut blocks , statements , TerminatorKind :: Call { func : callee , args , destination , target : Some (BasicBlock :: new (1)) , unwind : if let Some (Adjustment :: RefMut) = rcvr_adjustment { UnwindAction :: Cleanup (BasicBlock :: new (3)) } else { UnwindAction :: Continue } , call_source : CallSource :: Misc , fn_span : span , } , false ,) ; if let Some (Adjustment :: RefMut) = rcvr_adjustment { block (& mut blocks , vec ! [] , TerminatorKind :: Drop { place : rcvr_place () , target : BasicBlock :: new (2) , unwind : UnwindAction :: Continue , replace : false , drop : None , async_fut : None , } , false ,) ; } let stmts = vec ! [] ; block (& mut blocks , stmts , TerminatorKind :: Return , false) ; if let Some (Adjustment :: RefMut) = rcvr_adjustment { block (& mut blocks , vec ! [] , TerminatorKind :: Drop { place : rcvr_place () , target : BasicBlock :: new (4) , unwind : UnwindAction :: Terminate (UnwindTerminateReason :: InCleanup) , replace : false , drop : None , async_fut : None , } , true ,) ; block (& mut blocks , vec ! [] , TerminatorKind :: UnwindResume , true) ; } let mut body = new_body (MirSource :: from_instance (instance) , blocks , local_decls , sig . inputs () . len () , span) ; if let ExternAbi :: RustCall = sig . abi { body . spread_arg = Some (Local :: new (sig . inputs () . len ())) ; } body }
}

macro_rules! build_adt_ctor_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_adt_ctor in module {}", module_path!());
    };
}

mkfn!{
    build_adt_ctor_introspect!();
    pub (super) fn build_adt_ctor (tcx : TyCtxt < '_ > , ctor_id : DefId) -> Body < '_ > { debug_assert ! (tcx . is_constructor (ctor_id)) ; let typing_env = ty :: TypingEnv :: post_analysis (tcx , ctor_id) ; let sig = tcx . fn_sig (ctor_id) . instantiate_identity () . no_bound_vars () . expect ("LBR in ADT constructor signature") ; let sig = tcx . normalize_erasing_regions (typing_env , sig) ; let ty :: Adt (adt_def , args) = sig . output () . kind () else { bug ! ("unexpected type for ADT ctor {:?}" , sig . output ()) ; } ; debug ! ("build_ctor: ctor_id={:?} sig={:?}" , ctor_id , sig) ; let span = tcx . def_span (ctor_id) ; let local_decls = local_decls_for_sig (& sig , span) ; let source_info = SourceInfo :: outermost (span) ; let variant_index = if adt_def . is_enum () { adt_def . variant_index_with_ctor_id (ctor_id) } else { FIRST_VARIANT } ; debug ! ("build_ctor: variant_index={:?}" , variant_index) ; let kind = AggregateKind :: Adt (adt_def . did () , variant_index , args , None , None) ; let variant = adt_def . variant (variant_index) ; let statement = Statement :: new (source_info , StatementKind :: Assign (Box :: new ((Place :: return_place () , Rvalue :: Aggregate (Box :: new (kind) , (0 .. variant . fields . len ()) . map (| idx | Operand :: Move (Place :: from (Local :: new (idx + 1)))) . collect () ,) ,))) ,) ; let start_block = BasicBlockData :: new_stmts (vec ! [statement] , Some (Terminator { source_info , kind : TerminatorKind :: Return }) , false ,) ; let source = MirSource :: item (ctor_id) ; let mut body = new_body (source , IndexVec :: from_elem_n (start_block , 1) , local_decls , sig . inputs () . len () , span ,) ; body . set_mentioned_items (Vec :: new ()) ; crate :: pass_manager :: dump_mir_for_phase_change (tcx , & body) ; body }
}

macro_rules! build_fn_ptr_addr_shim_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_fn_ptr_addr_shim in module {}", module_path!());
    };
}

mkfn!{
    build_fn_ptr_addr_shim_introspect!();
    # [doc = " ```ignore (pseudo-impl)"] # [doc = " impl FnPtr for fn(u32) {"] # [doc = "     fn addr(self) -> usize {"] # [doc = "         self as usize"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] fn build_fn_ptr_addr_shim < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId , self_ty : Ty < 'tcx >) -> Body < 'tcx > { assert_matches ! (self_ty . kind () , ty :: FnPtr (..) , "expected fn ptr, found {self_ty}") ; let span = tcx . def_span (def_id) ; let Some (sig) = tcx . fn_sig (def_id) . instantiate (tcx , & [self_ty . into ()]) . no_bound_vars () else { span_bug ! (span , "FnPtr::addr with bound vars for `{self_ty}`") ; } ; let locals = local_decls_for_sig (& sig , span) ; let source_info = SourceInfo :: outermost (span) ; let rvalue = Rvalue :: Cast (CastKind :: FnPtrToPtr , Operand :: Move (Place :: from (Local :: new (1))) , Ty :: new_imm_ptr (tcx , tcx . types . unit) ,) ; let stmt = Statement :: new (source_info , StatementKind :: Assign (Box :: new ((Place :: return_place () , rvalue))) ,) ; let statements = vec ! [stmt] ; let start_block = BasicBlockData :: new_stmts (statements , Some (Terminator { source_info , kind : TerminatorKind :: Return }) , false ,) ; let source = MirSource :: from_instance (ty :: InstanceKind :: FnPtrAddrShim (def_id , self_ty)) ; new_body (source , IndexVec :: from_elem_n (start_block , 1) , locals , sig . inputs () . len () , span) }
}

macro_rules! build_construct_coroutine_by_move_shim_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_construct_coroutine_by_move_shim in module {}", module_path!());
    };
}

mkfn!{
    build_construct_coroutine_by_move_shim_introspect!();
    fn build_construct_coroutine_by_move_shim < 'tcx > (tcx : TyCtxt < 'tcx > , coroutine_closure_def_id : DefId , receiver_by_ref : bool ,) -> Body < 'tcx > { let mut self_ty = tcx . type_of (coroutine_closure_def_id) . instantiate_identity () ; let mut self_local : Place < 'tcx > = Local :: from_usize (1) . into () ; let ty :: CoroutineClosure (_ , args) = * self_ty . kind () else { bug ! () ; } ; if receiver_by_ref { self_local = tcx . mk_place_deref (self_local) ; self_ty = Ty :: new_imm_ref (tcx , tcx . lifetimes . re_erased , self_ty) ; } let poly_sig = args . as_coroutine_closure () . coroutine_closure_sig () . map_bound (| sig | { tcx . mk_fn_sig ([self_ty] . into_iter () . chain (sig . tupled_inputs_ty . tuple_fields ()) , sig . to_coroutine_given_kind_and_upvars (tcx , args . as_coroutine_closure () . parent_args () , tcx . coroutine_for_closure (coroutine_closure_def_id) , ty :: ClosureKind :: FnOnce , tcx . lifetimes . re_erased , args . as_coroutine_closure () . tupled_upvars_ty () , args . as_coroutine_closure () . coroutine_captures_by_ref_ty () ,) , sig . c_variadic , sig . safety , sig . abi ,) }) ; let sig = tcx . liberate_late_bound_regions (coroutine_closure_def_id , poly_sig) ; let ty :: Coroutine (coroutine_def_id , coroutine_args) = * sig . output () . kind () else { bug ! () ; } ; let span = tcx . def_span (coroutine_closure_def_id) ; let locals = local_decls_for_sig (& sig , span) ; let mut fields = vec ! [] ; for idx in 1 .. sig . inputs () . len () { fields . push (Operand :: Move (Local :: from_usize (idx + 1) . into ())) ; } for (idx , ty) in args . as_coroutine_closure () . upvar_tys () . iter () . enumerate () { if receiver_by_ref { if ! matches ! (ty . kind () , ty :: Ref (_ , _ , hir :: Mutability :: Not)) { tcx . dcx () . delayed_bug (format ! ("field should be captured by immutable ref if we have \
                    an `Fn` instance, but it was: {ty}")) ; } fields . push (Operand :: Copy (tcx . mk_place_field (self_local , FieldIdx :: from_usize (idx) , ty ,))) ; } else { fields . push (Operand :: Move (tcx . mk_place_field (self_local , FieldIdx :: from_usize (idx) , ty ,))) ; } } let source_info = SourceInfo :: outermost (span) ; let rvalue = Rvalue :: Aggregate (Box :: new (AggregateKind :: Coroutine (coroutine_def_id , coroutine_args)) , IndexVec :: from_raw (fields) ,) ; let stmt = Statement :: new (source_info , StatementKind :: Assign (Box :: new ((Place :: return_place () , rvalue))) ,) ; let statements = vec ! [stmt] ; let start_block = BasicBlockData :: new_stmts (statements , Some (Terminator { source_info , kind : TerminatorKind :: Return }) , false ,) ; let source = MirSource :: from_instance (ty :: InstanceKind :: ConstructCoroutineInClosureShim { coroutine_closure_def_id , receiver_by_ref , }) ; let body = new_body (source , IndexVec :: from_elem_n (start_block , 1) , locals , sig . inputs () . len () , span) ; let pass_name = if receiver_by_ref { "coroutine_closure_by_ref" } else { "coroutine_closure_by_move" } ; if let Some (dumper) = MirDumper :: new (tcx , pass_name , & body) { dumper . dump_mir (& body) ; } body }
}