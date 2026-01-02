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
mkuse!{use rustc_data_structures :: stack :: ensure_sufficient_stack ;}
mkuse!{use tracing :: { debug , instrument , trace } ;}
mkmod!{query_context, { 
                getname!(query_context);
                getsrc!(query_context);
                getpath!(query_context);
                get_deps!(query_context);
                get_crates!(query_context);
                mkinclude!(query_context);
                 
            }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkuse!{use crate :: layout :: { self , Def , Dfa , Reference , Tree , dfa , union } ;}
mkuse!{use crate :: maybe_transmutable :: query_context :: QueryContext ;}
mkuse!{use crate :: { Answer , Condition , Map , Reason } ;}
mkitem!{mkstruct!{pub (crate) struct MaybeTransmutableQuery < L , C > where C : QueryContext , { src : L , dst : L , assume : crate :: Assume , context : C , }}}
mkitem!{mkimpl!{impl < L , C > MaybeTransmutableQuery < L , C > where C : QueryContext , { pub (crate) fn new (src : L , dst : L , assume : crate :: Assume , context : C) -> Self { Self { src , dst , assume , context } } }}}
mkmod!{rustc, { 
                getname!(rustc);
                getsrc!(rustc);
                getpath!(rustc);
                get_deps!(rustc);
                get_crates!(rustc);
                mkinclude!(rustc);
                mkuse!{use rustc_middle :: ty :: layout :: LayoutCx ;}
mkuse!{use rustc_middle :: ty :: { Ty , TyCtxt , TypingEnv } ;}
mkuse!{use super :: * ;}
mkuse!{use crate :: layout :: tree :: rustc :: Err ;}
mkitem!{mkimpl!{impl < 'tcx > MaybeTransmutableQuery < Ty < 'tcx > , TyCtxt < 'tcx > > { # [doc = " This method begins by converting `src` and `dst` from `Ty`s to `Tree`s,"] # [doc = " then computes an answer using those trees."] # [instrument (level = "debug" , skip (self) , fields (src = ? self . src , dst = ? self . dst))] pub (crate) fn answer (self ,) -> Answer < < TyCtxt < 'tcx > as QueryContext > :: Region , < TyCtxt < 'tcx > as QueryContext > :: Type > { let Self { src , dst , assume , context } = self ; let layout_cx = LayoutCx :: new (context , TypingEnv :: fully_monomorphized ()) ; let src = Tree :: from_ty (src , layout_cx) ; let dst = Tree :: from_ty (dst , layout_cx) ; match (src , dst) { (Err (Err :: TypeError (_)) , _) | (_ , Err (Err :: TypeError (_))) => { Answer :: No (Reason :: TypeError) } (Err (Err :: UnknownLayout) , _) => Answer :: No (Reason :: SrcLayoutUnknown) , (_ , Err (Err :: UnknownLayout)) => Answer :: No (Reason :: DstLayoutUnknown) , (Err (Err :: NotYetSupported) , _) => Answer :: No (Reason :: SrcIsNotYetSupported) , (_ , Err (Err :: NotYetSupported)) => Answer :: No (Reason :: DstIsNotYetSupported) , (Err (Err :: SizeOverflow) , _) => Answer :: No (Reason :: SrcSizeOverflow) , (_ , Err (Err :: SizeOverflow)) => Answer :: No (Reason :: DstSizeOverflow) , (Ok (src) , Ok (dst)) => MaybeTransmutableQuery { src , dst , assume , context } . answer () , } } }}} 
            }}
mkitem!{mkimpl!{impl < C > MaybeTransmutableQuery < Tree < < C as QueryContext > :: Def , < C as QueryContext > :: Region , < C as QueryContext > :: Type > , C , > where C : QueryContext , { # [doc = " Answers whether a `Tree` is transmutable into another `Tree`."] # [doc = ""] # [doc = " This method begins by de-def'ing `src` and `dst`, and prunes private paths from `dst`,"] # [doc = " then converts `src` and `dst` to `Dfa`s, and computes an answer using those DFAs."] # [inline (always)] # [instrument (level = "debug" , skip (self) , fields (src = ? self . src , dst = ? self . dst))] pub (crate) fn answer (self) -> Answer < < C as QueryContext > :: Region , < C as QueryContext > :: Type > { let Self { src , dst , assume , context } = self ; let src = src . prune (& | _def | false) ; if src . is_inhabited () && ! dst . is_inhabited () { return Answer :: No (Reason :: DstUninhabited) ; } trace ! (? src , "pruned src") ; let dst = if assume . safety { dst . prune (& | _def | false) } else { dst . prune (& | def | def . has_safety_invariants ()) } ; trace ! (? dst , "pruned dst") ; let src = match Dfa :: from_tree (src) { Ok (src) => src , Err (layout :: Uninhabited) => return Answer :: Yes , } ; let dst = match Dfa :: from_tree (dst) { Ok (dst) => dst , Err (layout :: Uninhabited) => return Answer :: No (Reason :: DstMayHaveSafetyInvariants) , } ; MaybeTransmutableQuery { src , dst , assume , context } . answer () } }}}
mkitem!{mkimpl!{impl < C > MaybeTransmutableQuery < Dfa < < C as QueryContext > :: Region , < C as QueryContext > :: Type > , C > where C : QueryContext , { # [doc = " Answers whether a `Dfa` is transmutable into another `Dfa`."] pub (crate) fn answer (self) -> Answer < < C as QueryContext > :: Region , < C as QueryContext > :: Type > { self . answer_memo (& mut Map :: default () , self . src . start , self . dst . start) } # [inline (always)] # [instrument (level = "debug" , skip (self))] fn answer_memo (& self , cache : & mut Map < (dfa :: State , dfa :: State) , Answer < < C as QueryContext > :: Region , < C as QueryContext > :: Type > , > , src_state : dfa :: State , dst_state : dfa :: State ,) -> Answer < < C as QueryContext > :: Region , < C as QueryContext > :: Type > { if let Some (answer) = cache . get (& (src_state , dst_state)) { answer . clone () } else { let answer = ensure_sufficient_stack (| | self . answer_impl (cache , src_state , dst_state)) ; if let Some (..) = cache . insert ((src_state , dst_state) , answer . clone ()) { panic ! ("failed to correctly cache transmutability") } answer } } fn answer_impl (& self , cache : & mut Map < (dfa :: State , dfa :: State) , Answer < < C as QueryContext > :: Region , < C as QueryContext > :: Type > , > , src_state : dfa :: State , dst_state : dfa :: State ,) -> Answer < < C as QueryContext > :: Region , < C as QueryContext > :: Type > { debug ! (? src_state , ? dst_state) ; debug ! (src = ? self . src) ; debug ! (dst = ? self . dst) ; debug ! (src_transitions_len = self . src . transitions . len () , dst_transitions_len = self . dst . transitions . len ()) ; if dst_state == self . dst . accept { Answer :: Yes } else if src_state == self . src . accept { if let Some (dst_state_prime) = self . dst . get_uninit_edge_dst (dst_state) { self . answer_memo (cache , src_state , dst_state_prime) } else { Answer :: No (Reason :: DstIsTooBig) } } else { let src_quantifier = if self . assume . validity { Quantifier :: ThereExists } else { Quantifier :: ForAll } ; let bytes_answer = src_quantifier . apply (union (self . src . bytes_from (src_state) , self . dst . bytes_from (dst_state)) . filter_map (| (_range , (src_state_prime , dst_state_prime)) | { match (src_state_prime , dst_state_prime) { (None , _) => None , (Some (_) , None) => Some (Answer :: No (Reason :: DstIsBitIncompatible)) , (Some (src_state_prime) , Some (dst_state_prime)) => { Some (self . answer_memo (cache , src_state_prime , dst_state_prime)) } } } ,) ,) ; debug ! (? bytes_answer) ; match bytes_answer { Answer :: No (_) if ! self . assume . validity => return bytes_answer , Answer :: Yes if self . assume . validity => return bytes_answer , _ => { } } ; let refs_answer = src_quantifier . apply (self . src . refs_from (src_state) . map (| (src_ref , src_state_prime) | { Quantifier :: ThereExists . apply (self . dst . refs_from (dst_state) . map (| (dst_ref , dst_state_prime) | { if ! src_ref . is_mut && dst_ref . is_mut { Answer :: No (Reason :: DstIsMoreUnique) } else if ! self . assume . alignment && src_ref . referent_align < dst_ref . referent_align { Answer :: No (Reason :: DstHasStricterAlignment { src_min_align : src_ref . referent_align , dst_min_align : dst_ref . referent_align , }) } else if dst_ref . referent_size > src_ref . referent_size { Answer :: No (Reason :: DstRefIsTooBig { src : src_ref . referent , src_size : src_ref . referent_size , dst : dst_ref . referent , dst_size : dst_ref . referent_size , }) } else { let mut conditions = Vec :: with_capacity (4) ; let mut is_transmutable = | src : Reference < _ , _ > , dst : Reference < _ , _ > | { conditions . push (Condition :: Transmutable { src : src . referent , dst : dst . referent , }) ; if ! self . assume . lifetimes { conditions . push (Condition :: Outlives { long : src . region , short : dst . region , }) ; } } ; is_transmutable (src_ref , dst_ref) ; if dst_ref . is_mut { is_transmutable (dst_ref , src_ref) ; } else { conditions . push (Condition :: Immutable { ty : dst_ref . referent }) ; } Answer :: If (Condition :: IfAll (conditions)) . and (self . answer_memo (cache , src_state_prime , dst_state_prime ,)) } } ,)) }) ,) ; if self . assume . validity { bytes_answer . or (refs_answer) } else { bytes_answer . and (refs_answer) } } } }}}
mkitem!{mkimpl!{impl < R , T > Answer < R , T > { fn and (self , rhs : Answer < R , T >) -> Answer < R , T > { let lhs = self ; match (lhs , rhs) { (Answer :: No (Reason :: DstIsBitIncompatible) , Answer :: No (reason)) | (Answer :: No (reason) , Answer :: No (_)) | (Answer :: No (reason) , _) | (_ , Answer :: No (reason)) => Answer :: No (reason) , | (Answer :: Yes , other) | (other , Answer :: Yes) => other , (Answer :: If (Condition :: IfAll (mut lhs)) , Answer :: If (Condition :: IfAll (ref mut rhs))) => { lhs . append (rhs) ; Answer :: If (Condition :: IfAll (lhs)) } (Answer :: If (cond) , Answer :: If (Condition :: IfAll (mut conds))) | (Answer :: If (Condition :: IfAll (mut conds)) , Answer :: If (cond)) => { conds . push (cond) ; Answer :: If (Condition :: IfAll (conds)) } (Answer :: If (lhs) , Answer :: If (rhs)) => Answer :: If (Condition :: IfAll (vec ! [lhs , rhs])) , } } fn or (self , rhs : Answer < R , T >) -> Answer < R , T > { let lhs = self ; match (lhs , rhs) { (Answer :: No (Reason :: DstIsBitIncompatible) , Answer :: No (reason)) | (Answer :: No (reason) , Answer :: No (_)) => Answer :: No (reason) , (Answer :: No (_) , other) | (other , Answer :: No (_)) => other . or (Answer :: Yes) , (Answer :: Yes , other) | (other , Answer :: Yes) => other , (Answer :: If (Condition :: IfAny (mut lhs)) , Answer :: If (Condition :: IfAny (ref mut rhs))) => { lhs . append (rhs) ; Answer :: If (Condition :: IfAny (lhs)) } (Answer :: If (cond) , Answer :: If (Condition :: IfAny (mut conds))) | (Answer :: If (Condition :: IfAny (mut conds)) , Answer :: If (cond)) => { conds . push (cond) ; Answer :: If (Condition :: IfAny (conds)) } (Answer :: If (lhs) , Answer :: If (rhs)) => Answer :: If (Condition :: IfAny (vec ! [lhs , rhs])) , } } }}}
mkitem!{mkenum!{enum Quantifier { ThereExists , ForAll , }}}
mkitem!{mkimpl!{impl Quantifier { fn apply < R , T , I > (& self , iter : I) -> Answer < R , T > where R : layout :: Region , T : layout :: Type , I : IntoIterator < Item = Answer < R , T > > , { use std :: ops :: ControlFlow :: { Break , Continue } ; let (init , try_fold_f) : (_ , fn (_ , _) -> _) = match self { Self :: ThereExists => { (Answer :: No (Reason :: DstIsBitIncompatible) , | accum : Answer < R , T > , next | match accum . or (next) { Answer :: Yes => Break (Answer :: Yes) , maybe => Continue (maybe) , }) } Self :: ForAll => (Answer :: Yes , | accum : Answer < R , T > , next | { let answer = accum . and (next) ; match answer { Answer :: No (_) => Break (answer) , maybe => Continue (maybe) , } }) , } ; let (Continue (result) | Break (result)) = iter . into_iter () . try_fold (init , try_fold_f) ; result } }}}