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
mkuse!{use std :: fmt :: Debug ;}
mkuse!{use std :: hash :: Hash ;}
mkuse!{use std :: io :: Write ;}
mkuse!{use std :: iter ;}
mkuse!{use std :: num :: NonZero ;}
mkuse!{use std :: sync :: Arc ;}
mkuse!{use parking_lot :: { Condvar , Mutex } ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashMap , FxHashSet } ;}
mkuse!{use rustc_errors :: { Diag , DiagCtxtHandle } ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_span :: { DUMMY_SP , Span } ;}
mkuse!{use super :: QueryStackFrameExtra ;}
mkuse!{use crate :: dep_graph :: DepContext ;}
mkuse!{use crate :: error :: CycleStack ;}
mkuse!{use crate :: query :: plumbing :: CycleError ;}
mkuse!{use crate :: query :: { QueryContext , QueryStackFrame } ;}
mkitem!{mkstruct!{#[doc = " Represents a span and a query key."] #[derive (Clone , Debug)] pub struct QueryInfo < I > { #[doc = " The span corresponding to the reason for which this query was required."] pub span : Span , pub query : QueryStackFrame < I > , }}}
mkitem!{mkimpl!{impl < I > QueryInfo < I > { pub (crate) fn lift < Qcx : QueryContext < QueryInfo = I > > (& self , qcx : Qcx ,) -> QueryInfo < QueryStackFrameExtra > { QueryInfo { span : self . span , query : self . query . lift (qcx) } } }}}
mkitem!{pub type QueryMap < I > = FxHashMap < QueryJobId , QueryJobInfo < I > > ;}
mkitem!{mkstruct!{#[doc = " A value uniquely identifying an active query job."] #[derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct QueryJobId (pub NonZero < u64 >) ;}}
mkitem!{mkimpl!{impl QueryJobId { fn query < I : Clone > (self , map : & QueryMap < I >) -> QueryStackFrame < I > { map . get (& self) . unwrap () . query . clone () } fn span < I > (self , map : & QueryMap < I >) -> Span { map . get (& self) . unwrap () . job . span } fn parent < I > (self , map : & QueryMap < I >) -> Option < QueryJobId > { map . get (& self) . unwrap () . job . parent } fn latch < I > (self , map : & QueryMap < I >) -> Option < & QueryLatch < I > > { map . get (& self) . unwrap () . job . latch . as_ref () } }}}
mkitem!{mkstruct!{#[derive (Clone , Debug)] pub struct QueryJobInfo < I > { pub query : QueryStackFrame < I > , pub job : QueryJob < I > , }}}
mkitem!{mkstruct!{#[doc = " Represents an active query job."] #[derive (Debug)] pub struct QueryJob < I > { pub id : QueryJobId , #[doc = " The span corresponding to the reason for which this query was required."] pub span : Span , #[doc = " The parent query job which created this job and is implicitly waiting on it."] pub parent : Option < QueryJobId > , #[doc = " The latch that is used to wait on this job."] latch : Option < QueryLatch < I > > , }}}
mkitem!{mkimpl!{impl < I > Clone for QueryJob < I > { fn clone (& self) -> Self { Self { id : self . id , span : self . span , parent : self . parent , latch : self . latch . clone () } } }}}
mkitem!{mkimpl!{impl < I > QueryJob < I > { #[doc = " Creates a new query job."] #[inline] pub fn new (id : QueryJobId , span : Span , parent : Option < QueryJobId >) -> Self { QueryJob { id , span , parent , latch : None } } pub (super) fn latch (& mut self) -> QueryLatch < I > { if self . latch . is_none () { self . latch = Some (QueryLatch :: new ()) ; } self . latch . as_ref () . unwrap () . clone () } #[doc = " Signals to waiters that the query is complete."] #[doc = ""] #[doc = " This does nothing for single threaded rustc,"] #[doc = " as there are no concurrent jobs which could be waiting on us"] #[inline] pub fn signal_complete (self) { if let Some (latch) = self . latch { latch . set () ; } } }}}
mkitem!{mkimpl!{impl QueryJobId { pub (super) fn find_cycle_in_stack < I : Clone > (& self , query_map : QueryMap < I > , current_job : & Option < QueryJobId > , span : Span ,) -> CycleError < I > { let mut cycle = Vec :: new () ; let mut current_job = Option :: clone (current_job) ; while let Some (job) = current_job { let info = query_map . get (& job) . unwrap () ; cycle . push (QueryInfo { span : info . job . span , query : info . query . clone () }) ; if job == * self { cycle . reverse () ; cycle [0] . span = span ; let usage = info . job . parent . as_ref () . map (| parent | (info . job . span , parent . query (& query_map))) ; return CycleError { usage , cycle } ; } current_job = info . job . parent ; } panic ! ("did not find a cycle") } #[cold] #[inline (never)] pub fn find_dep_kind_root < I : Clone > (& self , query_map : QueryMap < I >) -> (QueryJobInfo < I > , usize) { let mut depth = 1 ; let info = query_map . get (& self) . unwrap () ; let dep_kind = info . query . dep_kind ; let mut current_id = info . job . parent ; let mut last_layout = (info . clone () , depth) ; while let Some (id) = current_id { let info = query_map . get (& id) . unwrap () ; if info . query . dep_kind == dep_kind { depth += 1 ; last_layout = (info . clone () , depth) ; } current_id = info . job . parent ; } last_layout } }}}
mkitem!{mkstruct!{#[derive (Debug)] struct QueryWaiter < I > { query : Option < QueryJobId > , condvar : Condvar , span : Span , cycle : Mutex < Option < CycleError < I > > > , }}}
mkitem!{mkstruct!{#[derive (Debug)] struct QueryLatchInfo < I > { complete : bool , waiters : Vec < Arc < QueryWaiter < I > > > , }}}
mkitem!{mkstruct!{#[derive (Debug)] pub (super) struct QueryLatch < I > { info : Arc < Mutex < QueryLatchInfo < I > > > , }}}
mkitem!{mkimpl!{impl < I > Clone for QueryLatch < I > { fn clone (& self) -> Self { Self { info : Arc :: clone (& self . info) } } }}}
mkitem!{mkimpl!{impl < I > QueryLatch < I > { fn new () -> Self { QueryLatch { info : Arc :: new (Mutex :: new (QueryLatchInfo { complete : false , waiters : Vec :: new () })) , } } #[doc = " Awaits for the query job to complete."] pub (super) fn wait_on (& self , qcx : impl QueryContext , query : Option < QueryJobId > , span : Span ,) -> Result < () , CycleError < I > > { let waiter = Arc :: new (QueryWaiter { query , span , cycle : Mutex :: new (None) , condvar : Condvar :: new () }) ; self . wait_on_inner (qcx , & waiter) ; let mut cycle = waiter . cycle . lock () ; match cycle . take () { None => Ok (()) , Some (cycle) => Err (cycle) , } } #[doc = " Awaits the caller on this latch by blocking the current thread."] fn wait_on_inner (& self , qcx : impl QueryContext , waiter : & Arc < QueryWaiter < I > >) { let mut info = self . info . lock () ; if ! info . complete { info . waiters . push (Arc :: clone (waiter)) ; rustc_thread_pool :: mark_blocked () ; let proxy = qcx . jobserver_proxy () ; proxy . release_thread () ; waiter . condvar . wait (& mut info) ; drop (info) ; proxy . acquire_thread () ; } } #[doc = " Sets the latch and resumes all waiters on it"] fn set (& self) { let mut info = self . info . lock () ; debug_assert ! (! info . complete) ; info . complete = true ; let registry = rustc_thread_pool :: Registry :: current () ; for waiter in info . waiters . drain (..) { rustc_thread_pool :: mark_unblocked (& registry) ; waiter . condvar . notify_one () ; } } #[doc = " Removes a single waiter from the list of waiters."] #[doc = " This is used to break query cycles."] fn extract_waiter (& self , waiter : usize) -> Arc < QueryWaiter < I > > { let mut info = self . info . lock () ; debug_assert ! (! info . complete) ; info . waiters . remove (waiter) } }}}
mkitem!{#[doc = " A resumable waiter of a query. The usize is the index into waiters in the query's latch"] type Waiter = (QueryJobId , usize) ;}

macro_rules! visit_waiters_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function visit_waiters in module {}", module_path!());
    };
}

mkfn!{
    visit_waiters_introspect!();
    #[doc = " Visits all the non-resumable and resumable waiters of a query."] #[doc = " Only waiters in a query are visited."] #[doc = " `visit` is called for every waiter and is passed a query waiting on `query_ref`"] #[doc = " and a span indicating the reason the query waited on `query_ref`."] #[doc = " If `visit` returns Some, this function returns."] #[doc = " For visits of non-resumable waiters it returns the return value of `visit`."] #[doc = " For visits of resumable waiters it returns Some(Some(Waiter)) which has the"] #[doc = " required information to resume the waiter."] #[doc = " If all `visit` calls returns None, this function also returns None."] fn visit_waiters < I , F > (query_map : & QueryMap < I > , query : QueryJobId , mut visit : F ,) -> Option < Option < Waiter > > where F : FnMut (Span , QueryJobId) -> Option < Option < Waiter > > , { if let Some (parent) = query . parent (query_map) && let Some (cycle) = visit (query . span (query_map) , parent) { return Some (cycle) ; } if let Some (latch) = query . latch (query_map) { for (i , waiter) in latch . info . lock () . waiters . iter () . enumerate () { if let Some (waiter_query) = waiter . query { if visit (waiter . span , waiter_query) . is_some () { return Some (Some ((query , i))) ; } } } } None }
}

macro_rules! cycle_check_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cycle_check in module {}", module_path!());
    };
}

mkfn!{
    cycle_check_introspect!();
    #[doc = " Look for query cycles by doing a depth first search starting at `query`."] #[doc = " `span` is the reason for the `query` to execute. This is initially DUMMY_SP."] #[doc = " If a cycle is detected, this initial value is replaced with the span causing"] #[doc = " the cycle."] fn cycle_check < I > (query_map : & QueryMap < I > , query : QueryJobId , span : Span , stack : & mut Vec < (Span , QueryJobId) > , visited : & mut FxHashSet < QueryJobId > ,) -> Option < Option < Waiter > > { if ! visited . insert (query) { return if let Some (p) = stack . iter () . position (| q | q . 1 == query) { stack . drain (0 .. p) ; stack [0] . 0 = span ; Some (None) } else { None } ; } stack . push ((span , query)) ; let r = visit_waiters (query_map , query , | span , successor | { cycle_check (query_map , successor , span , stack , visited) }) ; if r . is_none () { stack . pop () ; } r }
}

macro_rules! connected_to_root_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function connected_to_root in module {}", module_path!());
    };
}

mkfn!{
    connected_to_root_introspect!();
    #[doc = " Finds out if there's a path to the compiler root (aka. code which isn't in a query)"] #[doc = " from `query` without going through any of the queries in `visited`."] #[doc = " This is achieved with a depth first search."] fn connected_to_root < I > (query_map : & QueryMap < I > , query : QueryJobId , visited : & mut FxHashSet < QueryJobId > ,) -> bool { if ! visited . insert (query) { return false ; } if query . parent (query_map) . is_none () { return true ; } visit_waiters (query_map , query , | _ , successor | { connected_to_root (query_map , successor , visited) . then_some (None) }) . is_some () }
}

macro_rules! pick_query_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pick_query in module {}", module_path!());
    };
}

mkfn!{
    pick_query_introspect!();
    fn pick_query < 'a , I : Clone , T , F > (query_map : & QueryMap < I > , queries : & 'a [T] , f : F) -> & 'a T where F : Fn (& T) -> (Span , QueryJobId) , { queries . iter () . min_by_key (| v | { let (span , query) = f (v) ; let hash = query . query (query_map) . hash ; let span_cmp = if span == DUMMY_SP { 1 } else { 0 } ; (span_cmp , hash) }) . unwrap () }
}

macro_rules! remove_cycle_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remove_cycle in module {}", module_path!());
    };
}

mkfn!{
    remove_cycle_introspect!();
    #[doc = " Looks for query cycles starting from the last query in `jobs`."] #[doc = " If a cycle is found, all queries in the cycle is removed from `jobs` and"] #[doc = " the function return true."] #[doc = " If a cycle was not found, the starting query is removed from `jobs` and"] #[doc = " the function returns false."] fn remove_cycle < I : Clone > (query_map : & QueryMap < I > , jobs : & mut Vec < QueryJobId > , wakelist : & mut Vec < Arc < QueryWaiter < I > > > ,) -> bool { let mut visited = FxHashSet :: default () ; let mut stack = Vec :: new () ; if let Some (waiter) = cycle_check (query_map , jobs . pop () . unwrap () , DUMMY_SP , & mut stack , & mut visited) { let (mut spans , queries) : (Vec < _ > , Vec < _ >) = stack . into_iter () . rev () . unzip () ; spans . rotate_right (1) ; let mut stack : Vec < _ > = iter :: zip (spans , queries) . collect () ; for r in & stack { if let Some (pos) = jobs . iter () . position (| j | j == & r . 1) { jobs . remove (pos) ; } } let entry_points = stack . iter () . filter_map (| & (span , query) | { if query . parent (query_map) . is_none () { Some ((span , query , None)) } else { let mut waiters = Vec :: new () ; visit_waiters (query_map , query , | span , waiter | { let mut visited = FxHashSet :: from_iter (stack . iter () . map (| q | q . 1)) ; if connected_to_root (query_map , waiter , & mut visited) { waiters . push ((span , waiter)) ; } None }) ; if waiters . is_empty () { None } else { let waiter = * pick_query (query_map , & waiters , | s | * s) ; Some ((span , query , Some (waiter))) } } }) . collect :: < Vec < (Span , QueryJobId , Option < (Span , QueryJobId) >) > > () ; let (_ , entry_point , usage) = pick_query (query_map , & entry_points , | e | (e . 0 , e . 1)) ; let entry_point_pos = stack . iter () . position (| (_ , query) | query == entry_point) ; if let Some (pos) = entry_point_pos { stack . rotate_left (pos) ; } let usage = usage . as_ref () . map (| (span , query) | (* span , query . query (query_map))) ; let error = CycleError { usage , cycle : stack . iter () . map (| & (s , ref q) | QueryInfo { span : s , query : q . query (query_map) }) . collect () , } ; let (waitee_query , waiter_idx) = waiter . unwrap () ; let waiter = waitee_query . latch (query_map) . unwrap () . extract_waiter (waiter_idx) ; * waiter . cycle . lock () = Some (error) ; wakelist . push (waiter) ; true } else { false } }
}

macro_rules! break_query_cycles_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function break_query_cycles in module {}", module_path!());
    };
}

mkfn!{
    break_query_cycles_introspect!();
    #[doc = " Detects query cycles by using depth first search over all active query jobs."] #[doc = " If a query cycle is found it will break the cycle by finding an edge which"] #[doc = " uses a query latch and then resuming that waiter."] #[doc = " There may be multiple cycles involved in a deadlock, so this searches"] #[doc = " all active queries for cycles before finally resuming all the waiters at once."] pub fn break_query_cycles < I : Clone + Debug > (query_map : QueryMap < I > , registry : & rustc_thread_pool :: Registry ,) { let mut wakelist = Vec :: new () ; #[allow (rustc :: potential_query_instability)] let mut jobs : Vec < QueryJobId > = query_map . keys () . cloned () . collect () ; let mut found_cycle = false ; while jobs . len () > 0 { if remove_cycle (& query_map , & mut jobs , & mut wakelist) { found_cycle = true ; } } if ! found_cycle { panic ! ("deadlock detected as we're unable to find a query cycle to break\n\
            current query map:\n{:#?}" , query_map) ; } for _ in 0 .. wakelist . len () { rustc_thread_pool :: mark_unblocked (registry) ; } for waiter in wakelist . into_iter () { waiter . condvar . notify_one () ; } }
}

macro_rules! report_cycle_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_cycle in module {}", module_path!());
    };
}

mkfn!{
    report_cycle_introspect!();
    #[inline (never)] #[cold] pub fn report_cycle < 'a > (sess : & 'a Session , CycleError { usage , cycle : stack } : & CycleError ,) -> Diag < 'a > { assert ! (! stack . is_empty ()) ; let span = stack [0] . query . info . default_span (stack [1 % stack . len ()] . span) ; let mut cycle_stack = Vec :: new () ; use crate :: error :: StackCount ; let stack_count = if stack . len () == 1 { StackCount :: Single } else { StackCount :: Multiple } ; for i in 1 .. stack . len () { let query = & stack [i] . query ; let span = query . info . default_span (stack [(i + 1) % stack . len ()] . span) ; cycle_stack . push (CycleStack { span , desc : query . info . description . to_owned () }) ; } let mut cycle_usage = None ; if let Some ((span , ref query)) = * usage { cycle_usage = Some (crate :: error :: CycleUsage { span : query . info . default_span (span) , usage : query . info . description . to_string () , }) ; } let alias = if stack . iter () . all (| entry | matches ! (entry . query . info . def_kind , Some (DefKind :: TyAlias))) { Some (crate :: error :: Alias :: Ty) } else if stack . iter () . all (| entry | entry . query . info . def_kind == Some (DefKind :: TraitAlias)) { Some (crate :: error :: Alias :: Trait) } else { None } ; let cycle_diag = crate :: error :: Cycle { span , cycle_stack , stack_bottom : stack [0] . query . info . description . to_owned () , alias , cycle_usage , stack_count , note_span : () , } ; sess . dcx () . create_err (cycle_diag) }
}

macro_rules! print_query_stack_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function print_query_stack in module {}", module_path!());
    };
}

mkfn!{
    print_query_stack_introspect!();
    pub fn print_query_stack < Qcx : QueryContext > (qcx : Qcx , mut current_query : Option < QueryJobId > , dcx : DiagCtxtHandle < '_ > , limit_frames : Option < usize > , mut file : Option < std :: fs :: File > ,) -> usize { let mut count_printed = 0 ; let mut count_total = 0 ; let query_map = match qcx . collect_active_jobs () { Ok (query_map) => query_map , Err (query_map) => query_map , } ; if let Some (ref mut file) = file { let _ = writeln ! (file , "\n\nquery stack during panic:") ; } while let Some (query) = current_query { let Some (query_info) = query_map . get (& query) else { break ; } ; let query_extra = qcx . lift_query_info (& query_info . query . info) ; if Some (count_printed) < limit_frames || limit_frames . is_none () { #[allow (rustc :: diagnostic_outside_of_impl)] #[allow (rustc :: untranslatable_diagnostic)] dcx . struct_failure_note (format ! ("#{} [{:?}] {}" , count_printed , query_info . query . dep_kind , query_extra . description)) . with_span (query_info . job . span) . emit () ; count_printed += 1 ; } if let Some (ref mut file) = file { let _ = writeln ! (file , "#{} [{}] {}" , count_total , qcx . dep_context () . dep_kind_info (query_info . query . dep_kind) . name , query_extra . description) ; } current_query = query_info . job . parent ; count_total += 1 ; } if let Some (ref mut file) = file { let _ = writeln ! (file , "end of query stack") ; } count_total }
}