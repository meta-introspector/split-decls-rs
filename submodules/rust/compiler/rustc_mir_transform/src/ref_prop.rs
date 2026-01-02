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
mkuse!{use std :: borrow :: Cow ;}
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_index :: bit_set :: DenseBitSet ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: mir :: visit :: * ;}
mkuse!{use rustc_middle :: mir :: * ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_mir_dataflow :: Analysis ;}
mkuse!{use rustc_mir_dataflow :: impls :: { MaybeStorageDead , always_storage_live_locals } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use crate :: ssa :: { SsaLocals , StorageLiveLocals } ;}
mkitem!{mkstruct!{#[doc = " Propagate references using SSA analysis."] #[doc = ""] #[doc = " MIR building may produce a lot of borrow-dereference patterns."] #[doc = ""] #[doc = " This pass aims to transform the following pattern:"] #[doc = "   _1 = &raw? mut? PLACE;"] #[doc = "   _3 = *_1;"] #[doc = "   _4 = &raw? mut? *_1;"] #[doc = ""] #[doc = " Into"] #[doc = "   _1 = &raw? mut? PLACE;"] #[doc = "   _3 = PLACE;"] #[doc = "   _4 = &raw? mut? PLACE;"] #[doc = ""] #[doc = " where `PLACE` is a direct or an indirect place expression."] #[doc = ""] #[doc = " There are 3 properties that need to be upheld for this transformation to be legal:"] #[doc = " - place stability: `PLACE` must refer to the same memory wherever it appears;"] #[doc = " - pointer liveness: we must not introduce dereferences of dangling pointers;"] #[doc = " - `&mut` borrow uniqueness."] #[doc = ""] #[doc = " # Stability"] #[doc = ""] #[doc = " If `PLACE` is an indirect projection, if its of the form `(*LOCAL).PROJECTIONS` where:"] #[doc = " - `LOCAL` is SSA;"] #[doc = " - all projections in `PROJECTIONS` have a stable offset (no dereference and no indexing)."] #[doc = ""] #[doc = " If `PLACE` is a direct projection of a local, we consider it as constant if:"] #[doc = " - the local is always live, or it has a single `StorageLive`;"] #[doc = " - all projections have a stable offset."] #[doc = ""] #[doc = " # Liveness"] #[doc = ""] #[doc = " When performing an instantiation, we must take care not to introduce uses of dangling locals."] #[doc = " To ensure this, we walk the body with the `MaybeStorageDead` dataflow analysis:"] #[doc = " - if we want to replace `*x` by reborrow `*y` and `y` may be dead, we allow replacement and"] #[doc = "   mark storage statements on `y` for removal;"] #[doc = " - if we want to replace `*x` by non-reborrow `y` and `y` must be live, we allow replacement;"] #[doc = " - if we want to replace `*x` by non-reborrow `y` and `y` may be dead, we do not replace."] #[doc = ""] #[doc = " # Uniqueness"] #[doc = ""] #[doc = " For `&mut` borrows, we also need to preserve the uniqueness property:"] #[doc = " we must avoid creating a state where we interleave uses of `*_1` and `_2`."] #[doc = " To do it, we only perform full instantiation of mutable borrows:"] #[doc = " we replace either all or none of the occurrences of `*_1`."] #[doc = ""] #[doc = " Some care has to be taken when `_1` is copied in other locals."] #[doc = "   _1 = &raw? mut? _2;"] #[doc = "   _3 = *_1;"] #[doc = "   _4 = _1"] #[doc = "   _5 = *_4"] #[doc = " In such cases, fully instantiating `_1` means fully instantiating all of the copies."] #[doc = ""] #[doc = " For immutable borrows, we do not need to preserve such uniqueness property,"] #[doc = " so we perform all the possible instantiations without removing the `_1 = &_2` statement."] pub (super) struct ReferencePropagation ;}}
mkitem!{mkimpl!{impl < 'tcx > crate :: MirPass < 'tcx > for ReferencePropagation { fn is_enabled (& self , sess : & rustc_session :: Session) -> bool { sess . mir_opt_level () >= 2 } #[instrument (level = "trace" , skip (self , tcx , body))] fn run_pass (& self , tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { debug ! (def_id = ? body . source . def_id ()) ; move_to_copy_pointers (tcx , body) ; while propagate_ssa (tcx , body) { } } fn is_required (& self) -> bool { false } }}}

macro_rules! move_to_copy_pointers_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function move_to_copy_pointers in module {}", module_path!());
    };
}

mkfn!{
    move_to_copy_pointers_introspect!();
    #[doc = " The SSA analysis done by [`SsaLocals`] treats [`Operand::Move`] as a read, even though in"] #[doc = " general [`Operand::Move`] represents pass-by-pointer where the callee can overwrite the"] #[doc = " pointee (Miri always considers the place deinitialized). CopyProp has a similar trick to"] #[doc = " turn [`Operand::Move`] into [`Operand::Copy`] when required for an optimization, but in this"] #[doc = " pass we just turn all moves of pointers into copies because pointers should be by-value anyway."] fn move_to_copy_pointers < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { let mut visitor = MoveToCopyVisitor { tcx , local_decls : & body . local_decls } ; for (bb , data) in body . basic_blocks . as_mut_preserves_cfg () . iter_enumerated_mut () { visitor . visit_basic_block_data (bb , data) ; } struct MoveToCopyVisitor < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , local_decls : & 'a IndexVec < Local , LocalDecl < 'tcx > > , } impl < 'a , 'tcx > MutVisitor < 'tcx > for MoveToCopyVisitor < 'a , 'tcx > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn visit_operand (& mut self , operand : & mut Operand < 'tcx > , loc : Location) { if let Operand :: Move (place) = * operand { if place . ty (self . local_decls , self . tcx) . ty . is_any_ptr () { * operand = Operand :: Copy (place) ; } } self . super_operand (operand , loc) ; } } }
}

macro_rules! propagate_ssa_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function propagate_ssa in module {}", module_path!());
    };
}

mkfn!{
    propagate_ssa_introspect!();
    fn propagate_ssa < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) -> bool { let typing_env = body . typing_env (tcx) ; let ssa = SsaLocals :: new (tcx , body , typing_env) ; let mut replacer = compute_replacement (tcx , body , ssa) ; debug ! (? replacer . targets) ; debug ! (? replacer . allowed_replacements) ; debug ! (? replacer . storage_to_remove) ; replacer . visit_body_preserves_cfg (body) ; if replacer . any_replacement { crate :: simplify :: remove_unused_definitions (body) ; } replacer . any_replacement }
}
mkitem!{mkenum!{#[derive (Copy , Clone , Debug , PartialEq , Eq)] enum Value < 'tcx > { #[doc = " Not a pointer, or we can't know."] Unknown , #[doc = " We know the value to be a pointer to this place."] #[doc = " The boolean indicates whether the reference is mutable, subject the uniqueness rule."] Pointer (Place < 'tcx > , bool) , }}}

macro_rules! compute_replacement_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compute_replacement in module {}", module_path!());
    };
}

mkfn!{
    compute_replacement_introspect!();
    #[doc = " For each local, save the place corresponding to `*local`."] #[instrument (level = "trace" , skip (tcx , body , ssa))] fn compute_replacement < 'tcx > (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , ssa : SsaLocals ,) -> Replacer < 'tcx > { let always_live_locals = always_storage_live_locals (body) ; let storage_live = StorageLiveLocals :: new (body , & always_live_locals) ; let mut maybe_dead = MaybeStorageDead :: new (Cow :: Owned (always_live_locals)) . iterate_to_fixpoint (tcx , body , None) . into_results_cursor (body) ; let mut targets = IndexVec :: from_elem (Value :: Unknown , & body . local_decls) ; let mut storage_to_remove = DenseBitSet :: new_empty (body . local_decls . len ()) ; let fully_replaceable_locals = fully_replaceable_locals (& ssa) ; let is_constant_place = | place : Place < '_ > | { if place . projection . first () == Some (& PlaceElem :: Deref) { ssa . is_ssa (place . local) && place . projection [1 ..] . iter () . all (PlaceElem :: is_stable_offset) } else { storage_live . has_single_storage (place . local) && place . projection [..] . iter () . all (PlaceElem :: is_stable_offset) } } ; let mut can_perform_opt = | target : Place < 'tcx > , loc : Location | { if target . projection . first () == Some (& PlaceElem :: Deref) { storage_to_remove . insert (target . local) ; true } else { maybe_dead . seek_after_primary_effect (loc) ; let maybe_dead = maybe_dead . get () . contains (target . local) ; ! maybe_dead } } ; for (local , rvalue , location) in ssa . assignments (body) { debug ! (? local) ; let Value :: Unknown = targets [local] else { bug ! () } ; let ty = body . local_decls [local] . ty ; if ! ty . is_any_ptr () { debug ! ("not a reference or pointer") ; continue ; } let needs_unique = ty . is_mutable_ptr () ; if needs_unique && ! fully_replaceable_locals . contains (local) { debug ! ("not fully replaceable") ; continue ; } debug ! (? rvalue) ; match rvalue { Rvalue :: Use (Operand :: Copy (place) | Operand :: Move (place)) | Rvalue :: CopyForDeref (place) => { if let Some (rhs) = place . as_local () && ssa . is_ssa (rhs) { let target = targets [rhs] ; if ! needs_unique && matches ! (target , Value :: Pointer (..)) { targets [local] = target ; } else { targets [local] = Value :: Pointer (tcx . mk_place_deref (rhs . into ()) , needs_unique) ; } } } Rvalue :: Ref (_ , _ , place) | Rvalue :: RawPtr (_ , place) => { let mut place = * place ; if place . projection . first () == Some (& PlaceElem :: Deref) && let Value :: Pointer (target , inner_needs_unique) = targets [place . local] && ! inner_needs_unique && can_perform_opt (target , location) { place = target . project_deeper (& place . projection [1 ..] , tcx) ; } assert_ne ! (place . local , local) ; if is_constant_place (place) { targets [local] = Value :: Pointer (place , needs_unique) ; } } _ => { } } } debug ! (? targets) ; let mut finder = ReplacementFinder { targets , can_perform_opt , allowed_replacements : FxHashSet :: default () } ; let reachable_blocks = traversal :: reachable_as_bitset (body) ; for (bb , bbdata) in body . basic_blocks . iter_enumerated () { if reachable_blocks . contains (bb) { finder . visit_basic_block_data (bb , bbdata) ; } } let allowed_replacements = finder . allowed_replacements ; return Replacer { tcx , targets : finder . targets , storage_to_remove , allowed_replacements , any_replacement : false , } ; struct ReplacementFinder < 'tcx , F > { targets : IndexVec < Local , Value < 'tcx > > , can_perform_opt : F , allowed_replacements : FxHashSet < (Local , Location) > , } impl < 'tcx , F > Visitor < 'tcx > for ReplacementFinder < 'tcx , F > where F : FnMut (Place < 'tcx > , Location) -> bool , { fn visit_place (& mut self , place : & Place < 'tcx > , ctxt : PlaceContext , loc : Location) { if matches ! (ctxt , PlaceContext :: NonUse (_)) { return ; } if place . projection . first () != Some (& PlaceElem :: Deref) { return ; } let mut place = place . as_ref () ; loop { if let Value :: Pointer (target , needs_unique) = self . targets [place . local] { let perform_opt = (self . can_perform_opt) (target , loc) ; debug ! (? place , ? target , ? needs_unique , ? perform_opt) ; if let & [PlaceElem :: Deref] = & target . projection [..] { assert ! (perform_opt) ; self . allowed_replacements . insert ((target . local , loc)) ; place . local = target . local ; continue ; } else if perform_opt { self . allowed_replacements . insert ((target . local , loc)) ; } else if needs_unique { self . targets [place . local] = Value :: Unknown ; } } break ; } } } }
}

macro_rules! fully_replaceable_locals_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fully_replaceable_locals in module {}", module_path!());
    };
}

mkfn!{
    fully_replaceable_locals_introspect!();
    #[doc = " Compute the set of locals that can be fully replaced."] #[doc = ""] #[doc = " We consider a local to be replaceable iff it's only used in a `Deref` projection `*_local` or"] #[doc = " non-use position (like storage statements and debuginfo)."] fn fully_replaceable_locals (ssa : & SsaLocals) -> DenseBitSet < Local > { let mut replaceable = DenseBitSet :: new_empty (ssa . num_locals ()) ; for local in ssa . locals () { if ssa . num_direct_uses (local) == 0 { replaceable . insert (local) ; } } ssa . meet_copy_equivalence (& mut replaceable) ; replaceable }
}
mkitem!{mkstruct!{#[doc = " Utility to help performing substitution of `*pattern` by `target`."] struct Replacer < 'tcx > { tcx : TyCtxt < 'tcx > , targets : IndexVec < Local , Value < 'tcx > > , storage_to_remove : DenseBitSet < Local > , allowed_replacements : FxHashSet < (Local , Location) > , any_replacement : bool , }}}
mkitem!{mkimpl!{impl < 'tcx > MutVisitor < 'tcx > for Replacer < 'tcx > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn visit_var_debug_info (& mut self , debuginfo : & mut VarDebugInfo < 'tcx >) { while let VarDebugInfoContents :: Place (ref mut place) = debuginfo . value && place . projection . is_empty () && let Value :: Pointer (target , _) = self . targets [place . local] && target . projection . iter () . all (| p | p . can_use_in_debuginfo ()) { if let Some ((& PlaceElem :: Deref , rest)) = target . projection . split_last () { * place = Place :: from (target . local) . project_deeper (rest , self . tcx) ; self . any_replacement = true ; } else { break ; } } self . super_var_debug_info (debuginfo) ; } fn visit_place (& mut self , place : & mut Place < 'tcx > , ctxt : PlaceContext , loc : Location) { loop { if place . projection . first () != Some (& PlaceElem :: Deref) { return ; } let Value :: Pointer (target , _) = self . targets [place . local] else { return } ; let perform_opt = match ctxt { PlaceContext :: NonUse (NonUseContext :: VarDebugInfo) => { target . projection . iter () . all (| p | p . can_use_in_debuginfo ()) } PlaceContext :: NonUse (_) => true , _ => self . allowed_replacements . contains (& (target . local , loc)) , } ; if ! perform_opt { return ; } * place = target . project_deeper (& place . projection [1 ..] , self . tcx) ; self . any_replacement = true ; } } fn visit_statement (& mut self , stmt : & mut Statement < 'tcx > , loc : Location) { match stmt . kind { StatementKind :: StorageLive (l) | StatementKind :: StorageDead (l) if self . storage_to_remove . contains (l) => { stmt . make_nop () ; } _ => self . super_statement (stmt , loc) , } } }}}