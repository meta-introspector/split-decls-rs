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
mkuse!{use std :: collections :: BTreeMap ;}
mkuse!{use std :: fmt ;}
mkuse!{use Context :: * ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_hir :: intravisit :: { self , Visitor } ;}
mkuse!{use rustc_hir :: { Destination , Node , find_attr } ;}
mkuse!{use rustc_middle :: hir :: nested_filter ;}
mkuse!{use rustc_middle :: span_bug ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_span :: hygiene :: DesugaringKind ;}
mkuse!{use rustc_span :: { BytePos , Span } ;}
mkuse!{use crate :: errors :: { BreakInsideClosure , BreakInsideCoroutine , BreakNonLoop , ConstContinueBadLabel , ContinueLabeledBlock , OutsideLoop , OutsideLoopSuggestion , UnlabeledCfInWhileCondition , UnlabeledInLabeledBlock , } ;}
mkitem!{mkenum!{#[doc = " The context in which a block is encountered."] #[derive (Clone , Copy , Debug , PartialEq)] enum Context { Normal , Fn , Loop (hir :: LoopSource) , Closure (Span) , Coroutine { coroutine_span : Span , kind : hir :: CoroutineDesugaring , source : hir :: CoroutineSource , } , UnlabeledBlock (Span) , UnlabeledIfBlock (Span) , LabeledBlock , #[doc = " E.g. The labeled block inside `['_'; 'block: { break 'block 1 + 2; }]`."] AnonConst , #[doc = " E.g. `const { ... }`."] ConstBlock , #[doc = " E.g. `#[loop_match] loop { state = 'label: { /* ... */ } }`."] LoopMatch { #[doc = " The destination pointing to the labeled block (not to the loop itself)."] labeled_block : Destination , } , }}}
mkitem!{mkstruct!{#[derive (Clone)] struct BlockInfo { name : String , spans : Vec < Span > , suggs : Vec < Span > , }}}
mkitem!{mkenum!{#[derive (PartialEq)] enum BreakContextKind { Break , Continue , }}}
mkitem!{mkimpl!{impl fmt :: Display for BreakContextKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { BreakContextKind :: Break => "break" , BreakContextKind :: Continue => "continue" , } . fmt (f) } }}}
mkitem!{mkstruct!{#[derive (Clone)] struct CheckLoopVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , cx_stack : Vec < Context > , block_breaks : BTreeMap < Span , BlockInfo > , }}}

macro_rules! check_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check in module {}", module_path!());
    };
}

mkfn!{
    check_introspect!();
    pub (crate) fn check < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId , body : & 'tcx hir :: Body < 'tcx >) { let mut check = CheckLoopVisitor { tcx , cx_stack : vec ! [Normal] , block_breaks : Default :: default () } ; let cx = match tcx . def_kind (def_id) { DefKind :: AnonConst => AnonConst , _ => Fn , } ; check . with_context (cx , | v | v . visit_body (body)) ; check . report_outside_loop_error () ; }
}
mkitem!{mkimpl!{impl < 'hir > Visitor < 'hir > for CheckLoopVisitor < 'hir > { type NestedFilter = nested_filter :: OnlyBodies ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_anon_const (& mut self , _ : & 'hir hir :: AnonConst) { } fn visit_inline_const (& mut self , c : & 'hir hir :: ConstBlock) { self . with_context (ConstBlock , | v | intravisit :: walk_inline_const (v , c)) ; } fn visit_expr (& mut self , e : & 'hir hir :: Expr < 'hir >) { match e . kind { hir :: ExprKind :: If (cond , then , else_opt) => { self . visit_expr (cond) ; let get_block = | ck_loop : & CheckLoopVisitor < 'hir > , expr : & hir :: Expr < 'hir > | -> Option < & hir :: Block < 'hir > > { if let hir :: ExprKind :: Block (b , None) = expr . kind && matches ! (ck_loop . cx_stack . last () , Some (& Normal) | Some (& AnonConst) | Some (& UnlabeledBlock (_)) | Some (& UnlabeledIfBlock (_))) { Some (b) } else { None } } ; if let Some (b) = get_block (self , then) { self . with_context (UnlabeledIfBlock (b . span . shrink_to_lo ()) , | v | { v . visit_block (b) }) ; } else { self . visit_expr (then) ; } if let Some (else_expr) = else_opt { if let Some (b) = get_block (self , else_expr) { self . with_context (UnlabeledIfBlock (b . span . shrink_to_lo ()) , | v | { v . visit_block (b) }) ; } else { self . visit_expr (else_expr) ; } } } hir :: ExprKind :: Loop (ref b , _ , source , _) => { let cx = match self . is_loop_match (e , b) { Some (labeled_block) => LoopMatch { labeled_block } , None => Loop (source) , } ; self . with_context (cx , | v | v . visit_block (b)) ; } hir :: ExprKind :: Closure (& hir :: Closure { ref fn_decl , body , fn_decl_span , kind , .. }) => { let cx = match kind { hir :: ClosureKind :: Coroutine (hir :: CoroutineKind :: Desugared (kind , source)) => { Coroutine { coroutine_span : fn_decl_span , kind , source } } _ => Closure (fn_decl_span) , } ; self . visit_fn_decl (fn_decl) ; self . with_context (cx , | v | v . visit_nested_body (body)) ; } hir :: ExprKind :: Block (ref b , Some (_label)) => { self . with_context (LabeledBlock , | v | v . visit_block (b)) ; } hir :: ExprKind :: Block (ref b , None) if matches ! (self . cx_stack . last () , Some (& Fn) | Some (& ConstBlock)) => { self . with_context (Normal , | v | v . visit_block (b)) ; } hir :: ExprKind :: Block (ref b @ hir :: Block { rules : hir :: BlockCheckMode :: DefaultBlock , .. } , None ,) if matches ! (self . cx_stack . last () , Some (& Normal) | Some (& AnonConst) | Some (& UnlabeledBlock (_))) => { self . with_context (UnlabeledBlock (b . span . shrink_to_lo ()) , | v | v . visit_block (b)) ; } hir :: ExprKind :: Break (break_destination , ref opt_expr) => { if let Some (e) = opt_expr { self . visit_expr (e) ; } if self . require_label_in_labeled_block (e . span , & break_destination , "break") { return ; } let loop_id = match break_destination . target_id { Ok (loop_id) => Some (loop_id) , Err (hir :: LoopIdError :: OutsideLoopScope) => None , Err (hir :: LoopIdError :: UnlabeledCfInWhileCondition) => { self . tcx . dcx () . emit_err (UnlabeledCfInWhileCondition { span : e . span , cf_type : "break" , }) ; None } Err (hir :: LoopIdError :: UnresolvedLabel) => None , } ; if find_attr ! (self . tcx . hir_attrs (e . hir_id) , AttributeKind :: ConstContinue (_)) { let Some (label) = break_destination . label else { let span = e . span ; self . tcx . dcx () . emit_fatal (ConstContinueBadLabel { span }) ; } ; let is_target_label = | cx : & Context | match cx { Context :: LoopMatch { labeled_block } => { assert ! (labeled_block . target_id . is_ok ()) ; break_destination . target_id == labeled_block . target_id } _ => false , } ; if ! self . cx_stack . iter () . rev () . any (is_target_label) { let span = label . ident . span ; self . tcx . dcx () . emit_fatal (ConstContinueBadLabel { span }) ; } } if let Some (Node :: Block (_)) = loop_id . map (| id | self . tcx . hir_node (id)) { return ; } if let Some (break_expr) = opt_expr { let (head , loop_label , loop_kind) = if let Some (loop_id) = loop_id { match self . tcx . hir_expect_expr (loop_id) . kind { hir :: ExprKind :: Loop (_ , label , source , sp) => { (Some (sp) , label , Some (source)) } ref r => { span_bug ! (e . span , "break label resolved to a non-loop: {:?}" , r) } } } else { (None , None , None) } ; match loop_kind { None | Some (hir :: LoopSource :: Loop) => () , Some (kind) => { let suggestion = format ! ("break{}" , break_destination . label . map_or_else (String :: new , | l | format ! (" {}" , l . ident))) ; self . tcx . dcx () . emit_err (BreakNonLoop { span : e . span , head , kind : kind . name () , suggestion , loop_label , break_label : break_destination . label , break_expr_kind : & break_expr . kind , break_expr_span : break_expr . span , }) ; } } } let sp_lo = e . span . with_lo (e . span . lo () + BytePos ("break" . len () as u32)) ; let label_sp = match break_destination . label { Some (label) => sp_lo . with_hi (label . ident . span . hi ()) , None => sp_lo . shrink_to_lo () , } ; self . require_break_cx (BreakContextKind :: Break , e . span , label_sp , self . cx_stack . len () - 1 ,) ; } hir :: ExprKind :: Continue (destination) => { self . require_label_in_labeled_block (e . span , & destination , "continue") ; match destination . target_id { Ok (loop_id) => { if let Node :: Block (block) = self . tcx . hir_node (loop_id) { self . tcx . dcx () . emit_err (ContinueLabeledBlock { span : e . span , block_span : block . span , }) ; } } Err (hir :: LoopIdError :: UnlabeledCfInWhileCondition) => { self . tcx . dcx () . emit_err (UnlabeledCfInWhileCondition { span : e . span , cf_type : "continue" , }) ; } Err (_) => { } } self . require_break_cx (BreakContextKind :: Continue , e . span , e . span , self . cx_stack . len () - 1 ,) } _ => intravisit :: walk_expr (self , e) , } } }}}
mkitem!{mkimpl!{impl < 'hir > CheckLoopVisitor < 'hir > { fn with_context < F > (& mut self , cx : Context , f : F) where F : FnOnce (& mut CheckLoopVisitor < 'hir >) , { self . cx_stack . push (cx) ; f (self) ; self . cx_stack . pop () ; } fn require_break_cx (& mut self , br_cx_kind : BreakContextKind , span : Span , break_span : Span , cx_pos : usize ,) { match self . cx_stack [cx_pos] { LabeledBlock | Loop (_) | LoopMatch { .. } => { } Closure (closure_span) => { self . tcx . dcx () . emit_err (BreakInsideClosure { span , closure_span , name : & br_cx_kind . to_string () , }) ; } Coroutine { coroutine_span , kind , source } => { let kind = match kind { hir :: CoroutineDesugaring :: Async => "async" , hir :: CoroutineDesugaring :: Gen => "gen" , hir :: CoroutineDesugaring :: AsyncGen => "async gen" , } ; let source = match source { hir :: CoroutineSource :: Block => "block" , hir :: CoroutineSource :: Closure => "closure" , hir :: CoroutineSource :: Fn => "function" , } ; self . tcx . dcx () . emit_err (BreakInsideCoroutine { span , coroutine_span , name : & br_cx_kind . to_string () , kind , source , }) ; } UnlabeledBlock (block_span) if br_cx_kind == BreakContextKind :: Break && block_span . eq_ctxt (break_span) => { let block = self . block_breaks . entry (block_span) . or_insert_with (| | BlockInfo { name : br_cx_kind . to_string () , spans : vec ! [] , suggs : vec ! [] , }) ; block . spans . push (span) ; block . suggs . push (break_span) ; } UnlabeledIfBlock (_) if br_cx_kind == BreakContextKind :: Break => { self . require_break_cx (br_cx_kind , span , break_span , cx_pos - 1) ; } Normal | AnonConst | Fn | UnlabeledBlock (_) | UnlabeledIfBlock (_) | ConstBlock => { self . tcx . dcx () . emit_err (OutsideLoop { spans : vec ! [span] , name : & br_cx_kind . to_string () , is_break : br_cx_kind == BreakContextKind :: Break , suggestion : None , }) ; } } } fn require_label_in_labeled_block (& self , span : Span , label : & Destination , cf_type : & str ,) -> bool { if ! span . is_desugaring (DesugaringKind :: QuestionMark) && self . cx_stack . last () == Some (& LabeledBlock) && label . label . is_none () { self . tcx . dcx () . emit_err (UnlabeledInLabeledBlock { span , cf_type }) ; return true ; } false } fn report_outside_loop_error (& self) { for (s , block) in & self . block_breaks { self . tcx . dcx () . emit_err (OutsideLoop { spans : block . spans . clone () , name : & block . name , is_break : true , suggestion : Some (OutsideLoopSuggestion { block_span : * s , break_spans : block . suggs . clone () , }) , }) ; } } #[doc = " Is this a loop annotated with `#[loop_match]` that looks syntactically sound?"] fn is_loop_match (& self , e : & 'hir hir :: Expr < 'hir > , body : & 'hir hir :: Block < 'hir > ,) -> Option < Destination > { if ! find_attr ! (self . tcx . hir_attrs (e . hir_id) , AttributeKind :: LoopMatch (_)) { return None ; } let loop_body_expr = match body . stmts { [] => match body . expr { Some (expr) => expr , None => return None , } , [single] if body . expr . is_none () => match single . kind { hir :: StmtKind :: Expr (expr) | hir :: StmtKind :: Semi (expr) => expr , _ => return None , } , [..] => return None , } ; let hir :: ExprKind :: Assign (_ , rhs_expr , _) = loop_body_expr . kind else { return None } ; let hir :: ExprKind :: Block (block , label) = rhs_expr . kind else { return None } ; Some (Destination { label , target_id : Ok (block . hir_id) }) } }}}