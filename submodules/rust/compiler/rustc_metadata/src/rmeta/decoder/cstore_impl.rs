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
mkuse!{use std :: any :: Any ;}
mkuse!{use std :: mem ;}
mkuse!{use std :: sync :: Arc ;}
mkuse!{use rustc_hir :: attrs :: Deprecation ;}
mkuse!{use rustc_hir :: def :: { CtorKind , DefKind } ;}
mkuse!{use rustc_hir :: def_id :: { CrateNum , DefId , DefIdMap , LOCAL_CRATE } ;}
mkuse!{use rustc_hir :: definitions :: { DefKey , DefPath , DefPathHash } ;}
mkuse!{use rustc_middle :: arena :: ArenaAllocatable ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: metadata :: ModChild ;}
mkuse!{use rustc_middle :: middle :: exported_symbols :: ExportedSymbol ;}
mkuse!{use rustc_middle :: middle :: stability :: DeprecationEntry ;}
mkuse!{use rustc_middle :: query :: { ExternProviders , LocalCrate } ;}
mkuse!{use rustc_middle :: ty :: fast_reject :: SimplifiedType ;}
mkuse!{use rustc_middle :: ty :: { self , TyCtxt } ;}
mkuse!{use rustc_middle :: util :: Providers ;}
mkuse!{use rustc_session :: cstore :: { CrateStore , ExternCrate } ;}
mkuse!{use rustc_session :: { Session , StableCrateId } ;}
mkuse!{use rustc_span :: hygiene :: ExpnId ;}
mkuse!{use rustc_span :: { Span , Symbol , kw } ;}
mkuse!{use super :: { Decodable , DecodeContext , DecodeIterator } ;}
mkuse!{use crate :: creader :: { CStore , LoadedMacro } ;}
mkuse!{use crate :: rmeta :: AttrFlags ;}
mkuse!{use crate :: rmeta :: table :: IsDefault ;}
mkuse!{use crate :: { foreign_modules , native_libs } ;}
mkitem!{mktrait!{trait ProcessQueryValue < 'tcx , T > { fn process_decoded (self , _tcx : TyCtxt < 'tcx > , _err : impl Fn () -> !) -> T ; }}}
mkitem!{mkimpl!{impl < T > ProcessQueryValue < '_ , T > for T { #[inline (always)] fn process_decoded (self , _tcx : TyCtxt < '_ > , _err : impl Fn () -> !) -> T { self } }}}
mkitem!{mkimpl!{impl < 'tcx , T > ProcessQueryValue < 'tcx , ty :: EarlyBinder < 'tcx , T > > for T { #[inline (always)] fn process_decoded (self , _tcx : TyCtxt < '_ > , _err : impl Fn () -> !) -> ty :: EarlyBinder < 'tcx , T > { ty :: EarlyBinder :: bind (self) } }}}
mkitem!{mkimpl!{impl < T > ProcessQueryValue < '_ , T > for Option < T > { #[inline (always)] fn process_decoded (self , _tcx : TyCtxt < '_ > , err : impl Fn () -> !) -> T { if let Some (value) = self { value } else { err () } } }}}
mkitem!{mkimpl!{impl < 'tcx , T : ArenaAllocatable < 'tcx > > ProcessQueryValue < 'tcx , & 'tcx T > for Option < T > { #[inline (always)] fn process_decoded (self , tcx : TyCtxt < 'tcx > , err : impl Fn () -> !) -> & 'tcx T { if let Some (value) = self { tcx . arena . alloc (value) } else { err () } } }}}
mkitem!{mkimpl!{impl < T , E > ProcessQueryValue < '_ , Result < Option < T > , E > > for Option < T > { #[inline (always)] fn process_decoded (self , _tcx : TyCtxt < '_ > , _err : impl Fn () -> !) -> Result < Option < T > , E > { Ok (self) } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx , T : Copy + Decodable < DecodeContext < 'a , 'tcx > > > ProcessQueryValue < 'tcx , & 'tcx [T] > for Option < DecodeIterator < 'a , 'tcx , T > > { #[inline (always)] fn process_decoded (self , tcx : TyCtxt < 'tcx > , err : impl Fn () -> !) -> & 'tcx [T] { if let Some (iter) = self { tcx . arena . alloc_from_iter (iter) } else { err () } } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx , T : Copy + Decodable < DecodeContext < 'a , 'tcx > > > ProcessQueryValue < 'tcx , Option < & 'tcx [T] > > for Option < DecodeIterator < 'a , 'tcx , T > > { #[inline (always)] fn process_decoded (self , tcx : TyCtxt < 'tcx > , _err : impl Fn () -> !) -> Option < & 'tcx [T] > { if let Some (iter) = self { Some (& * tcx . arena . alloc_from_iter (iter)) } else { None } } }}}
mkitem!{mkimpl!{impl ProcessQueryValue < '_ , Option < DeprecationEntry > > for Option < Deprecation > { #[inline (always)] fn process_decoded (self , _tcx : TyCtxt < '_ > , _err : impl Fn () -> !) -> Option < DeprecationEntry > { self . map (DeprecationEntry :: external) } }}}
mkitem!{macro_rules ! provide_one { ($ tcx : ident , $ def_id : ident , $ other : ident , $ cdata : ident , $ name : ident => { table }) => { provide_one ! { $ tcx , $ def_id , $ other , $ cdata , $ name => { $ cdata . root . tables .$ name . get ($ cdata , $ def_id . index) . map (| lazy | lazy . decode (($ cdata , $ tcx))) . process_decoded ($ tcx , || panic ! ("{:?} does not have a {:?}" , $ def_id , stringify ! ($ name))) } } } ; ($ tcx : ident , $ def_id : ident , $ other : ident , $ cdata : ident , $ name : ident => { table_defaulted_array }) => { provide_one ! { $ tcx , $ def_id , $ other , $ cdata , $ name => { let lazy = $ cdata . root . tables .$ name . get ($ cdata , $ def_id . index) ; let value = if lazy . is_default () { & [] as & [_] } else { $ tcx . arena . alloc_from_iter (lazy . decode (($ cdata , $ tcx))) } ; value . process_decoded ($ tcx , || panic ! ("{:?} does not have a {:?}" , $ def_id , stringify ! ($ name))) } } } ; ($ tcx : ident , $ def_id : ident , $ other : ident , $ cdata : ident , $ name : ident => { table_direct }) => { provide_one ! { $ tcx , $ def_id , $ other , $ cdata , $ name => { $ cdata . root . tables .$ name . get ($ cdata , $ def_id . index) . process_decoded ($ tcx , || panic ! ("{:?} does not have a {:?}" , $ def_id , stringify ! ($ name))) } } } ; ($ tcx : ident , $ def_id : ident , $ other : ident , $ cdata : ident , $ name : ident => $ compute : block) => { fn $ name <'tcx > ($ tcx : TyCtxt <'tcx >, def_id_arg : rustc_middle :: query :: queries ::$ name :: Key <'tcx >,) -> rustc_middle :: query :: queries ::$ name :: ProvidedValue <'tcx > { let _prof_timer = $ tcx . prof . generic_activity (concat ! ("metadata_decode_entry_" , stringify ! ($ name))) ; #[allow (unused_variables)] let ($ def_id , $ other) = def_id_arg . into_args () ; assert ! (!$ def_id . is_local ()) ; use rustc_middle :: dep_graph :: dep_kinds ; if dep_kinds ::$ name != dep_kinds :: crate_hash && $ tcx . dep_graph . is_fully_enabled () { $ tcx . ensure_ok () . crate_hash ($ def_id . krate) ; } let cdata = rustc_data_structures :: sync :: FreezeReadGuard :: map (CStore :: from_tcx ($ tcx) , | c | { c . get_crate_data ($ def_id . krate) . cdata }) ; let $ cdata = crate :: creader :: CrateMetadataRef { cdata : & cdata , cstore : & CStore :: from_tcx ($ tcx) , } ; $ compute } } ; }}
mkitem!{macro_rules ! provide { ($ tcx : ident , $ def_id : ident , $ other : ident , $ cdata : ident , $ ($ name : ident => { $ ($ compute : tt) * }) *) => { fn provide_extern (providers : & mut ExternProviders) { $ (provide_one ! { $ tcx , $ def_id , $ other , $ cdata , $ name => { $ ($ compute) * } }) * * providers = ExternProviders { $ ($ name ,) * ..* providers } ; } } }}
mkitem!{mktrait!{trait IntoArgs { type Other ; fn into_args (self) -> (DefId , Self :: Other) ; }}}
mkitem!{mkimpl!{impl IntoArgs for DefId { type Other = () ; fn into_args (self) -> (DefId , ()) { (self , ()) } }}}
mkitem!{mkimpl!{impl IntoArgs for CrateNum { type Other = () ; fn into_args (self) -> (DefId , ()) { (self . as_def_id () , ()) } }}}
mkitem!{mkimpl!{impl IntoArgs for (CrateNum , DefId) { type Other = DefId ; fn into_args (self) -> (DefId , DefId) { (self . 0 . as_def_id () , self . 1) } }}}
mkitem!{mkimpl!{impl < 'tcx > IntoArgs for ty :: InstanceKind < 'tcx > { type Other = () ; fn into_args (self) -> (DefId , ()) { (self . def_id () , ()) } }}}
mkitem!{mkimpl!{impl IntoArgs for (CrateNum , SimplifiedType) { type Other = SimplifiedType ; fn into_args (self) -> (DefId , SimplifiedType) { (self . 0 . as_def_id () , self . 1) } }}}
mkitem!{provide ! { tcx , def_id , other , cdata , explicit_item_bounds => { table_defaulted_array } explicit_item_self_bounds => { table_defaulted_array } explicit_predicates_of => { table } generics_of => { table } inferred_outlives_of => { table_defaulted_array } explicit_super_predicates_of => { table_defaulted_array } explicit_implied_predicates_of => { table_defaulted_array } type_of => { table } type_alias_is_lazy => { table_direct } variances_of => { table } fn_sig => { table } codegen_fn_attrs => { table } impl_trait_header => { table } const_param_default => { table } object_lifetime_default => { table } thir_abstract_const => { table } optimized_mir => { table } mir_for_ctfe => { table } closure_saved_names_of_captured_variables => { table } mir_coroutine_witnesses => { table } promoted_mir => { table } def_span => { table } def_ident_span => { table } lookup_stability => { table } lookup_const_stability => { table } lookup_default_body_stability => { table } lookup_deprecation_entry => { table } params_in_repr => { table } def_kind => { cdata . def_kind (def_id . index) } impl_parent => { table } defaultness => { table_direct } constness => { table_direct } const_conditions => { table } explicit_implied_const_bounds => { table_defaulted_array } coerce_unsized_info => { Ok (cdata . root . tables . coerce_unsized_info . get (cdata , def_id . index) . map (| lazy | lazy . decode ((cdata , tcx))) . process_decoded (tcx , || panic ! ("{def_id:?} does not have coerce_unsized_info"))) } mir_const_qualif => { table } rendered_const => { table } rendered_precise_capturing_args => { table } asyncness => { table_direct } fn_arg_idents => { table } coroutine_kind => { table_direct } coroutine_for_closure => { table } coroutine_by_move_body_def_id => { table } eval_static_initializer => { Ok (cdata . root . tables . eval_static_initializer . get (cdata , def_id . index) . map (| lazy | lazy . decode ((cdata , tcx))) . unwrap_or_else (|| panic ! ("{def_id:?} does not have eval_static_initializer"))) } trait_def => { table } deduced_param_attrs => { cdata . root . tables . deduced_param_attrs . get (cdata , def_id . index) . map (| lazy | { &* tcx . arena . alloc_from_iter (lazy . decode ((cdata , tcx))) }) . unwrap_or_default () } opaque_ty_origin => { table } assumed_wf_types_for_rpitit => { table } collect_return_position_impl_trait_in_trait_tys => { Ok (cdata . root . tables . trait_impl_trait_tys . get (cdata , def_id . index) . map (| lazy | lazy . decode ((cdata , tcx))) . process_decoded (tcx , || panic ! ("{def_id:?} does not have trait_impl_trait_tys"))) } associated_types_for_impl_traits_in_trait_or_impl => { table } visibility => { cdata . get_visibility (def_id . index) } adt_def => { cdata . get_adt_def (def_id . index , tcx) } adt_destructor => { table } adt_async_destructor => { table } associated_item_def_ids => { tcx . arena . alloc_from_iter (cdata . get_associated_item_or_field_def_ids (def_id . index)) } associated_item => { cdata . get_associated_item (def_id . index , tcx . sess) } inherent_impls => { cdata . get_inherent_implementations_for_type (tcx , def_id . index) } attrs_for_def => { tcx . arena . alloc_from_iter (cdata . get_item_attrs (def_id . index , tcx . sess)) } is_mir_available => { cdata . is_item_mir_available (def_id . index) } is_ctfe_mir_available => { cdata . is_ctfe_mir_available (def_id . index) } cross_crate_inlinable => { table_direct } dylib_dependency_formats => { cdata . get_dylib_dependency_formats (tcx) } is_private_dep => { cdata . private_dep } is_panic_runtime => { cdata . root . panic_runtime } is_compiler_builtins => { cdata . root . compiler_builtins } has_global_allocator => { cdata . root . has_global_allocator } has_alloc_error_handler => { cdata . root . has_alloc_error_handler } has_panic_handler => { cdata . root . has_panic_handler } is_profiler_runtime => { cdata . root . profiler_runtime } required_panic_strategy => { cdata . root . required_panic_strategy } panic_in_drop_strategy => { cdata . root . panic_in_drop_strategy } extern_crate => { cdata . extern_crate . map (| c | &* tcx . arena . alloc (c)) } is_no_builtins => { cdata . root . no_builtins } symbol_mangling_version => { cdata . root . symbol_mangling_version } specialization_enabled_in => { cdata . root . specialization_enabled_in } reachable_non_generics => { let reachable_non_generics = tcx . exported_non_generic_symbols (cdata . cnum) . iter () . filter_map (|& (exported_symbol , export_info) | { if let ExportedSymbol :: NonGeneric (def_id) = exported_symbol { Some ((def_id , export_info)) } else { None } }) . collect () ; reachable_non_generics } native_libraries => { cdata . get_native_libraries (tcx . sess) . collect () } foreign_modules => { cdata . get_foreign_modules (tcx . sess) . map (| m | (m . def_id , m)) . collect () } crate_hash => { cdata . root . header . hash } crate_host_hash => { cdata . host_hash } crate_name => { cdata . root . header . name } num_extern_def_ids => { cdata . num_def_ids () } extra_filename => { cdata . root . extra_filename . clone () } traits => { tcx . arena . alloc_from_iter (cdata . get_traits ()) } trait_impls_in_crate => { tcx . arena . alloc_from_iter (cdata . get_trait_impls ()) } implementations_of_trait => { cdata . get_implementations_of_trait (tcx , other) } crate_incoherent_impls => { cdata . get_incoherent_impls (tcx , other) } dep_kind => { cdata . dep_kind } module_children => { tcx . arena . alloc_from_iter (cdata . get_module_children (def_id . index , tcx . sess)) } lib_features => { cdata . get_lib_features () } stability_implications => { cdata . get_stability_implications (tcx) . iter () . copied () . collect () } stripped_cfg_items => { cdata . get_stripped_cfg_items (cdata . cnum , tcx) } intrinsic_raw => { cdata . get_intrinsic (def_id . index) } defined_lang_items => { cdata . get_lang_items (tcx) } diagnostic_items => { cdata . get_diagnostic_items () } missing_lang_items => { cdata . get_missing_lang_items (tcx) } missing_extern_crate_item => { matches ! (cdata . extern_crate , Some (extern_crate) if ! extern_crate . is_direct ()) } used_crate_source => { Arc :: clone (& cdata . source) } debugger_visualizers => { cdata . get_debugger_visualizers () } exportable_items => { tcx . arena . alloc_from_iter (cdata . get_exportable_items ()) } stable_order_of_exportable_impls => { tcx . arena . alloc (cdata . get_stable_order_of_exportable_impls () . collect ()) } exported_non_generic_symbols => { cdata . exported_non_generic_symbols (tcx) } exported_generic_symbols => { cdata . exported_generic_symbols (tcx) } crate_extern_paths => { cdata . source () . paths () . cloned () . collect () } expn_that_defined => { cdata . get_expn_that_defined (def_id . index , tcx . sess) } default_field => { cdata . get_default_field (def_id . index) } is_doc_hidden => { cdata . get_attr_flags (def_id . index) . contains (AttrFlags :: IS_DOC_HIDDEN) } doc_link_resolutions => { tcx . arena . alloc (cdata . get_doc_link_resolutions (def_id . index)) } doc_link_traits_in_scope => { tcx . arena . alloc_from_iter (cdata . get_doc_link_traits_in_scope (def_id . index)) } anon_const_kind => { table } }}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (in crate :: rmeta) fn provide (providers : & mut Providers) { provide_cstore_hooks (providers) ; providers . queries = rustc_middle :: query :: Providers { allocator_kind : | tcx , () | CStore :: from_tcx (tcx) . allocator_kind () , alloc_error_handler_kind : | tcx , () | CStore :: from_tcx (tcx) . alloc_error_handler_kind () , is_private_dep : | _tcx , LocalCrate | false , native_library : | tcx , id | { tcx . native_libraries (id . krate) . iter () . filter (| lib | native_libs :: relevant_lib (tcx . sess , lib)) . find (| lib | { let Some (fm_id) = lib . foreign_module else { return false ; } ; let map = tcx . foreign_modules (id . krate) ; map . get (& fm_id) . expect ("failed to find foreign module") . foreign_items . contains (& id) }) } , native_libraries : native_libs :: collect , foreign_modules : foreign_modules :: collect , visible_parent_map : | tcx , () | { use std :: collections :: hash_map :: Entry ; use std :: collections :: vec_deque :: VecDeque ; let mut visible_parent_map : DefIdMap < DefId > = Default :: default () ; let mut fallback_map : Vec < (DefId , DefId) > = Default :: default () ; let bfs_queue = & mut VecDeque :: new () ; for & cnum in tcx . crates (()) { if tcx . missing_extern_crate_item (cnum) { continue ; } bfs_queue . push_back (cnum . as_def_id ()) ; } let mut add_child = | bfs_queue : & mut VecDeque < _ > , child : & ModChild , parent : DefId | { if ! child . vis . is_public () { return ; } if let Some (def_id) = child . res . opt_def_id () { if child . ident . name == kw :: Underscore { fallback_map . push ((def_id , parent)) ; return ; } if tcx . is_doc_hidden (parent) { fallback_map . push ((def_id , parent)) ; return ; } match visible_parent_map . entry (def_id) { Entry :: Occupied (mut entry) => { if def_id . is_local () && entry . get () . is_local () { entry . insert (parent) ; } } Entry :: Vacant (entry) => { entry . insert (parent) ; if child . res . module_like_def_id () . is_some () { bfs_queue . push_back (def_id) ; } } } } } ; while let Some (def) = bfs_queue . pop_front () { for child in tcx . module_children (def) . iter () { add_child (bfs_queue , child , def) ; } } for (child , parent) in fallback_map { visible_parent_map . entry (child) . or_insert (parent) ; } visible_parent_map } , dependency_formats : | tcx , () | Arc :: new (crate :: dependency_format :: calculate (tcx)) , has_global_allocator : | tcx , LocalCrate | CStore :: from_tcx (tcx) . has_global_allocator () , has_alloc_error_handler : | tcx , LocalCrate | CStore :: from_tcx (tcx) . has_alloc_error_handler () , postorder_cnums : | tcx , () | { tcx . arena . alloc_from_iter (CStore :: from_tcx (tcx) . crate_dependencies_in_postorder (LOCAL_CRATE) . into_iter () ,) } , crates : | tcx , () | { tcx . untracked () . cstore . freeze () ; tcx . arena . alloc_from_iter (CStore :: from_tcx (tcx) . iter_crate_data () . map (| (cnum , _) | cnum)) } , used_crates : | tcx , () | { tcx . untracked () . cstore . freeze () ; tcx . arena . alloc_from_iter (CStore :: from_tcx (tcx) . iter_crate_data () . filter_map (| (cnum , data) | data . used () . then_some (cnum)) ,) } , .. providers . queries } ; provide_extern (& mut providers . extern_queries) ; }
}
mkitem!{mkimpl!{impl CStore { pub fn ctor_untracked (& self , def : DefId) -> Option < (CtorKind , DefId) > { self . get_crate_data (def . krate) . get_ctor (def . index) } pub fn load_macro_untracked (& self , id : DefId , tcx : TyCtxt < '_ >) -> LoadedMacro { let sess = tcx . sess ; let _prof_timer = sess . prof . generic_activity ("metadata_load_macro") ; let data = self . get_crate_data (id . krate) ; if data . root . is_proc_macro_crate () { LoadedMacro :: ProcMacro (data . load_proc_macro (id . index , tcx)) } else { LoadedMacro :: MacroDef { def : data . get_macro (id . index , sess) , ident : data . item_ident (id . index , sess) , attrs : data . get_item_attrs (id . index , sess) . collect () , span : data . get_span (id . index , sess) , edition : data . root . edition , } } } pub fn def_span_untracked (& self , def_id : DefId , sess : & Session) -> Span { self . get_crate_data (def_id . krate) . get_span (def_id . index , sess) } pub fn def_kind_untracked (& self , def : DefId) -> DefKind { self . get_crate_data (def . krate) . def_kind (def . index) } pub fn expn_that_defined_untracked (& self , def_id : DefId , sess : & Session) -> ExpnId { self . get_crate_data (def_id . krate) . get_expn_that_defined (def_id . index , sess) } #[doc = " Only public-facing way to traverse all the definitions in a non-local crate."] #[doc = " Critically useful for this third-party project: <https://github.com/hacspec/hacspec>."] #[doc = " See <https://github.com/rust-lang/rust/pull/85889> for context."] pub fn num_def_ids_untracked (& self , cnum : CrateNum) -> usize { self . get_crate_data (cnum) . num_def_ids () } pub fn get_proc_macro_quoted_span_untracked (& self , cnum : CrateNum , id : usize , sess : & Session ,) -> Span { self . get_crate_data (cnum) . get_proc_macro_quoted_span (id , sess) } pub fn set_used_recursively (& mut self , cnum : CrateNum) { let cmeta = self . get_crate_data_mut (cnum) ; if ! cmeta . used { cmeta . used = true ; let dependencies = mem :: take (& mut cmeta . dependencies) ; for & dep_cnum in & dependencies { self . set_used_recursively (dep_cnum) ; } self . get_crate_data_mut (cnum) . dependencies = dependencies ; } } #[doc = " Track how an extern crate has been loaded. Called after resolving an import in the local crate."] #[doc = ""] #[doc = " * the `name` is for [`Self::set_resolved_extern_crate_name`] saving `--extern name=`"] #[doc = " * `extern_crate` is for diagnostics"] pub (crate) fn update_extern_crate (& mut self , cnum : CrateNum , name : Symbol , extern_crate : ExternCrate ,) { debug_assert_eq ! (extern_crate . dependency_of , LOCAL_CRATE , "this function should not be called on transitive dependencies") ; self . set_resolved_extern_crate_name (name , cnum) ; self . update_transitive_extern_crate_diagnostics (cnum , extern_crate) ; } #[doc = " `CrateMetadata` uses `ExternCrate` only for diagnostics"] fn update_transitive_extern_crate_diagnostics (& mut self , cnum : CrateNum , extern_crate : ExternCrate ,) { let cmeta = self . get_crate_data_mut (cnum) ; if cmeta . update_extern_crate_diagnostics (extern_crate) { let extern_crate = ExternCrate { dependency_of : cnum , .. extern_crate } ; let dependencies = mem :: take (& mut cmeta . dependencies) ; for & dep_cnum in & dependencies { self . update_transitive_extern_crate_diagnostics (dep_cnum , extern_crate) ; } self . get_crate_data_mut (cnum) . dependencies = dependencies ; } } }}}
mkitem!{mkimpl!{impl CrateStore for CStore { fn as_any (& self) -> & dyn Any { self } fn untracked_as_any (& mut self) -> & mut dyn Any { self } fn crate_name (& self , cnum : CrateNum) -> Symbol { self . get_crate_data (cnum) . root . header . name } fn stable_crate_id (& self , cnum : CrateNum) -> StableCrateId { self . get_crate_data (cnum) . root . stable_crate_id } #[doc = " Returns the `DefKey` for a given `DefId`. This indicates the"] #[doc = " parent `DefId` as well as some idea of what kind of data the"] #[doc = " `DefId` refers to."] fn def_key (& self , def : DefId) -> DefKey { self . get_crate_data (def . krate) . def_key (def . index) } fn def_path (& self , def : DefId) -> DefPath { self . get_crate_data (def . krate) . def_path (def . index) } fn def_path_hash (& self , def : DefId) -> DefPathHash { self . get_crate_data (def . krate) . def_path_hash (def . index) } }}}

macro_rules! provide_cstore_hooks_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide_cstore_hooks in module {}", module_path!());
    };
}

mkfn!{
    provide_cstore_hooks_introspect!();
    fn provide_cstore_hooks (providers : & mut Providers) { providers . hooks . def_path_hash_to_def_id_extern = | tcx , hash , stable_crate_id | { let cstore = CStore :: from_tcx (tcx) ; let cnum = * tcx . untracked () . stable_crate_ids . read () . get (& stable_crate_id) . unwrap_or_else (| | bug ! ("uninterned StableCrateId: {stable_crate_id:?}")) ; assert_ne ! (cnum , LOCAL_CRATE) ; let def_index = cstore . get_crate_data (cnum) . def_path_hash_to_def_index (hash) ; DefId { krate : cnum , index : def_index } } ; providers . hooks . expn_hash_to_expn_id = | tcx , cnum , index_guess , hash | { let cstore = CStore :: from_tcx (tcx) ; cstore . get_crate_data (cnum) . expn_hash_to_expn_id (tcx . sess , index_guess , hash) } ; providers . hooks . import_source_files = | tcx , cnum | { let cstore = CStore :: from_tcx (tcx) ; let cdata = cstore . get_crate_data (cnum) ; for file_index in 0 .. cdata . root . source_map . size () { cdata . imported_source_file (file_index as u32 , tcx . sess) ; } } ; }
}