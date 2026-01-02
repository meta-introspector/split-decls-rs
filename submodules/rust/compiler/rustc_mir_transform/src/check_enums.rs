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
mkuse!{use rustc_abi :: { Scalar , Size , TagEncoding , Variants , WrappingRange } ;}
mkuse!{use rustc_hir :: LangItem ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: mir :: visit :: Visitor ;}
mkuse!{use rustc_middle :: mir :: * ;}
mkuse!{use rustc_middle :: ty :: layout :: PrimitiveExt ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt , TypingEnv } ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use tracing :: debug ;}
mkitem!{mkstruct!{#[doc = " This pass inserts checks for a valid enum discriminant where they are most"] #[doc = " likely to find UB, because checking everywhere like Miri would generate too"] #[doc = " much MIR."] pub (super) struct CheckEnums ;}}
mkitem!{mkimpl!{impl < 'tcx > crate :: MirPass < 'tcx > for CheckEnums { fn is_enabled (& self , sess : & Session) -> bool { sess . ub_checks () } fn run_pass (& self , tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { if tcx . lang_items () . get (LangItem :: PanicImpl) . is_none () { return ; } let typing_env = body . typing_env (tcx) ; let basic_blocks = body . basic_blocks . as_mut () ; let local_decls = & mut body . local_decls ; for block in basic_blocks . indices () . rev () { for statement_index in (0 .. basic_blocks [block] . statements . len ()) . rev () { let location = Location { block , statement_index } ; let statement = & basic_blocks [block] . statements [statement_index] ; let source_info = statement . source_info ; let mut finder = EnumFinder :: new (tcx , local_decls , typing_env) ; finder . visit_statement (statement , location) ; for check in finder . into_found_enums () { debug ! ("Inserting enum check") ; let new_block = split_block (basic_blocks , location) ; match check { EnumCheckType :: Direct { op_size , .. } | EnumCheckType :: WithNiche { op_size , .. } if op_size . bytes () == 0 => { tcx . dcx () . span_delayed_bug (source_info . span , "cannot build enum discriminant from zero-sized type" ,) ; basic_blocks [block] . terminator = Some (Terminator { source_info , kind : TerminatorKind :: Goto { target : new_block } , }) ; } EnumCheckType :: Direct { source_op , discr , op_size , valid_discrs } => { insert_direct_enum_check (tcx , local_decls , basic_blocks , block , source_op , discr , op_size , valid_discrs , source_info , new_block ,) } EnumCheckType :: Uninhabited => insert_uninhabited_enum_check (tcx , local_decls , & mut basic_blocks [block] , source_info , new_block ,) , EnumCheckType :: WithNiche { source_op , discr , op_size , offset , valid_range , } => insert_niche_check (tcx , local_decls , & mut basic_blocks [block] , source_op , valid_range , discr , op_size , offset , source_info , new_block ,) , } } } } } fn is_required (& self) -> bool { true } }}}
mkitem!{mkenum!{#[doc = " Represent the different kind of enum checks we can insert."] enum EnumCheckType < 'tcx > { #[doc = " We know we try to create an uninhabited enum from an inhabited variant."] Uninhabited , #[doc = " We know the enum does no niche optimizations and can thus easily compute"] #[doc = " the valid discriminants."] Direct { source_op : Operand < 'tcx > , discr : TyAndSize < 'tcx > , op_size : Size , valid_discrs : Vec < u128 > , } , #[doc = " We try to construct an enum that has a niche."] WithNiche { source_op : Operand < 'tcx > , discr : TyAndSize < 'tcx > , op_size : Size , offset : Size , valid_range : WrappingRange , } , }}}
mkitem!{mkstruct!{#[derive (Debug , Copy , Clone)] struct TyAndSize < 'tcx > { pub ty : Ty < 'tcx > , pub size : Size , }}}
mkitem!{mkstruct!{#[doc = " A [Visitor] that finds the construction of enums and evaluates which checks"] #[doc = " we should apply."] struct EnumFinder < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , local_decls : & 'a mut LocalDecls < 'tcx > , typing_env : TypingEnv < 'tcx > , enums : Vec < EnumCheckType < 'tcx > > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > EnumFinder < 'a , 'tcx > { fn new (tcx : TyCtxt < 'tcx > , local_decls : & 'a mut LocalDecls < 'tcx > , typing_env : TypingEnv < 'tcx > ,) -> Self { EnumFinder { tcx , local_decls , typing_env , enums : Vec :: new () } } #[doc = " Returns the found enum creations and which checks should be inserted."] fn into_found_enums (self) -> Vec < EnumCheckType < 'tcx > > { self . enums } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Visitor < 'tcx > for EnumFinder < 'a , 'tcx > { fn visit_rvalue (& mut self , rvalue : & Rvalue < 'tcx > , location : Location) { if let Rvalue :: Cast (CastKind :: Transmute , op , ty) = rvalue { let ty :: Adt (adt_def , _) = ty . kind () else { return ; } ; if ! adt_def . is_enum () { return ; } let Ok (enum_layout) = self . tcx . layout_of (self . typing_env . as_query_input (* ty)) else { return ; } ; let Ok (op_layout) = self . tcx . layout_of (self . typing_env . as_query_input (op . ty (self . local_decls , self . tcx))) else { return ; } ; match enum_layout . variants { Variants :: Empty if op_layout . is_uninhabited () => return , Variants :: Empty => { self . enums . push (EnumCheckType :: Uninhabited) ; } Variants :: Single { .. } => { } Variants :: Multiple { tag_encoding : TagEncoding :: Direct , tag : Scalar :: Initialized { value , .. } , .. } => { let valid_discrs = adt_def . discriminants (self . tcx) . map (| (_ , discr) | discr . val) . collect () ; let discr = TyAndSize { ty : value . to_int_ty (self . tcx) , size : value . size (& self . tcx) } ; self . enums . push (EnumCheckType :: Direct { source_op : op . to_copy () , discr , op_size : op_layout . size , valid_discrs , }) ; } Variants :: Multiple { tag_encoding : TagEncoding :: Niche { .. } , tag : Scalar :: Initialized { value , valid_range , .. } , tag_field , .. } => { let discr = TyAndSize { ty : value . to_int_ty (self . tcx) , size : value . size (& self . tcx) } ; self . enums . push (EnumCheckType :: WithNiche { source_op : op . to_copy () , discr , op_size : op_layout . size , offset : enum_layout . fields . offset (tag_field . as_usize ()) , valid_range , }) ; } _ => return , } self . super_rvalue (rvalue , location) ; } } }}}

macro_rules! split_block_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function split_block in module {}", module_path!());
    };
}

mkfn!{
    split_block_introspect!();
    fn split_block (basic_blocks : & mut IndexVec < BasicBlock , BasicBlockData < '_ > > , location : Location ,) -> BasicBlock { let block_data = & mut basic_blocks [location . block] ; let new_block = BasicBlockData :: new_stmts (block_data . statements . split_off (location . statement_index) , block_data . terminator . take () , block_data . is_cleanup ,) ; basic_blocks . push (new_block) }
}

macro_rules! insert_discr_cast_to_u128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function insert_discr_cast_to_u128 in module {}", module_path!());
    };
}

mkfn!{
    insert_discr_cast_to_u128_introspect!();
    #[doc = " Inserts the cast of an operand (any type) to a u128 value that holds the discriminant value."] fn insert_discr_cast_to_u128 < 'tcx > (tcx : TyCtxt < 'tcx > , local_decls : & mut IndexVec < Local , LocalDecl < 'tcx > > , block_data : & mut BasicBlockData < 'tcx > , source_op : Operand < 'tcx > , discr : TyAndSize < 'tcx > , op_size : Size , offset : Option < Size > , source_info : SourceInfo ,) -> Place < 'tcx > { let get_ty_for_size = | tcx : TyCtxt < 'tcx > , size : Size | -> Ty < 'tcx > { match size . bytes () { 1 => tcx . types . u8 , 2 => tcx . types . u16 , 4 => tcx . types . u32 , 8 => tcx . types . u64 , 16 => tcx . types . u128 , invalid => bug ! ("Found discriminant with invalid size, has {} bytes" , invalid) , } } ; let (cast_kind , discr_ty_bits) = if discr . size . bytes () < op_size . bytes () { let mu = Ty :: new_maybe_uninit (tcx , tcx . types . u8) ; let array_len = op_size . bytes () ; let mu_array_ty = Ty :: new_array (tcx , mu , array_len) ; let mu_array = local_decls . push (LocalDecl :: with_source_info (mu_array_ty , source_info)) . into () ; let rvalue = Rvalue :: Cast (CastKind :: Transmute , source_op , mu_array_ty) ; block_data . statements . push (Statement :: new (source_info , StatementKind :: Assign (Box :: new ((mu_array , rvalue))))) ; let offset = offset . unwrap_or (Size :: ZERO) ; let smaller_mu_array = mu_array . project_deeper (& [ProjectionElem :: Subslice { from : offset . bytes () , to : offset . bytes () + discr . size . bytes () , from_end : false , }] , tcx ,) ; (CastKind :: Transmute , Operand :: Copy (smaller_mu_array)) } else { let operand_int_ty = get_ty_for_size (tcx , op_size) ; let op_as_int = local_decls . push (LocalDecl :: with_source_info (operand_int_ty , source_info)) . into () ; let rvalue = Rvalue :: Cast (CastKind :: Transmute , source_op , operand_int_ty) ; block_data . statements . push (Statement :: new (source_info , StatementKind :: Assign (Box :: new ((op_as_int , rvalue))) ,)) ; (CastKind :: IntToInt , Operand :: Copy (op_as_int)) } ; let rvalue = Rvalue :: Cast (cast_kind , discr_ty_bits , discr . ty) ; let discr_in_discr_ty = local_decls . push (LocalDecl :: with_source_info (discr . ty , source_info)) . into () ; block_data . statements . push (Statement :: new (source_info , StatementKind :: Assign (Box :: new ((discr_in_discr_ty , rvalue))) ,)) ; let const_u128 = Ty :: new_uint (tcx , ty :: UintTy :: U128) ; let rvalue = Rvalue :: Cast (CastKind :: IntToInt , Operand :: Copy (discr_in_discr_ty) , const_u128) ; let discr = local_decls . push (LocalDecl :: with_source_info (const_u128 , source_info)) . into () ; block_data . statements . push (Statement :: new (source_info , StatementKind :: Assign (Box :: new ((discr , rvalue))))) ; discr }
}

macro_rules! insert_direct_enum_check_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function insert_direct_enum_check in module {}", module_path!());
    };
}

mkfn!{
    insert_direct_enum_check_introspect!();
    fn insert_direct_enum_check < 'tcx > (tcx : TyCtxt < 'tcx > , local_decls : & mut IndexVec < Local , LocalDecl < 'tcx > > , basic_blocks : & mut IndexVec < BasicBlock , BasicBlockData < 'tcx > > , current_block : BasicBlock , source_op : Operand < 'tcx > , discr : TyAndSize < 'tcx > , op_size : Size , discriminants : Vec < u128 > , source_info : SourceInfo , new_block : BasicBlock ,) { let invalid_discr_block_data = BasicBlockData :: new (None , false) ; let invalid_discr_block = basic_blocks . push (invalid_discr_block_data) ; let block_data = & mut basic_blocks [current_block] ; let discr_place = insert_discr_cast_to_u128 (tcx , local_decls , block_data , source_op , discr , op_size , None , source_info ,) ; let mask = discr . size . unsigned_int_max () ; let discr_masked = local_decls . push (LocalDecl :: with_source_info (tcx . types . u128 , source_info)) . into () ; let rvalue = Rvalue :: BinaryOp (BinOp :: BitAnd , Box :: new ((Operand :: Copy (discr_place) , Operand :: Constant (Box :: new (ConstOperand { span : source_info . span , user_ty : None , const_ : Const :: Val (ConstValue :: from_u128 (mask) , tcx . types . u128) , })) ,)) ,) ; block_data . statements . push (Statement :: new (source_info , StatementKind :: Assign (Box :: new ((discr_masked , rvalue))))) ; block_data . terminator = Some (Terminator { source_info , kind : TerminatorKind :: SwitchInt { discr : Operand :: Copy (discr_masked) , targets : SwitchTargets :: new (discriminants . into_iter () . map (| discr_val | (discr . size . truncate (discr_val) , new_block)) , invalid_discr_block ,) , } , }) ; basic_blocks [invalid_discr_block] . terminator = Some (Terminator { source_info , kind : TerminatorKind :: Assert { cond : Operand :: Constant (Box :: new (ConstOperand { span : source_info . span , user_ty : None , const_ : Const :: Val (ConstValue :: from_bool (false) , tcx . types . bool) , })) , expected : true , target : new_block , msg : Box :: new (AssertKind :: InvalidEnumConstruction (Operand :: Copy (discr_masked))) , unwind : UnwindAction :: Unreachable , } , }) ; }
}

macro_rules! insert_uninhabited_enum_check_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function insert_uninhabited_enum_check in module {}", module_path!());
    };
}

mkfn!{
    insert_uninhabited_enum_check_introspect!();
    fn insert_uninhabited_enum_check < 'tcx > (tcx : TyCtxt < 'tcx > , local_decls : & mut IndexVec < Local , LocalDecl < 'tcx > > , block_data : & mut BasicBlockData < 'tcx > , source_info : SourceInfo , new_block : BasicBlock ,) { let is_ok : Place < '_ > = local_decls . push (LocalDecl :: with_source_info (tcx . types . bool , source_info)) . into () ; block_data . statements . push (Statement :: new (source_info , StatementKind :: Assign (Box :: new ((is_ok , Rvalue :: Use (Operand :: Constant (Box :: new (ConstOperand { span : source_info . span , user_ty : None , const_ : Const :: Val (ConstValue :: from_bool (false) , tcx . types . bool) , }))) ,))) ,)) ; block_data . terminator = Some (Terminator { source_info , kind : TerminatorKind :: Assert { cond : Operand :: Copy (is_ok) , expected : true , target : new_block , msg : Box :: new (AssertKind :: InvalidEnumConstruction (Operand :: Constant (Box :: new (ConstOperand { span : source_info . span , user_ty : None , const_ : Const :: Val (ConstValue :: from_u128 (0) , tcx . types . u128) , } ,)))) , unwind : UnwindAction :: Unreachable , } , }) ; }
}

macro_rules! insert_niche_check_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function insert_niche_check in module {}", module_path!());
    };
}

mkfn!{
    insert_niche_check_introspect!();
    fn insert_niche_check < 'tcx > (tcx : TyCtxt < 'tcx > , local_decls : & mut IndexVec < Local , LocalDecl < 'tcx > > , block_data : & mut BasicBlockData < 'tcx > , source_op : Operand < 'tcx > , valid_range : WrappingRange , discr : TyAndSize < 'tcx > , op_size : Size , offset : Size , source_info : SourceInfo , new_block : BasicBlock ,) { let discr = insert_discr_cast_to_u128 (tcx , local_decls , block_data , source_op , discr , op_size , Some (offset) , source_info ,) ; let start_const = Operand :: Constant (Box :: new (ConstOperand { span : source_info . span , user_ty : None , const_ : Const :: Val (ConstValue :: from_u128 (valid_range . start) , tcx . types . u128) , })) ; let end_start_diff_const = Operand :: Constant (Box :: new (ConstOperand { span : source_info . span , user_ty : None , const_ : Const :: Val (ConstValue :: from_u128 (u128 :: wrapping_sub (valid_range . end , valid_range . start)) , tcx . types . u128 ,) , })) ; let discr_diff : Place < '_ > = local_decls . push (LocalDecl :: with_source_info (tcx . types . u128 , source_info)) . into () ; block_data . statements . push (Statement :: new (source_info , StatementKind :: Assign (Box :: new ((discr_diff , Rvalue :: BinaryOp (BinOp :: Sub , Box :: new ((Operand :: Copy (discr) , start_const))) ,))) ,)) ; let is_ok : Place < '_ > = local_decls . push (LocalDecl :: with_source_info (tcx . types . bool , source_info)) . into () ; block_data . statements . push (Statement :: new (source_info , StatementKind :: Assign (Box :: new ((is_ok , Rvalue :: BinaryOp (BinOp :: Le , Box :: new ((Operand :: Copy (discr_diff) , end_start_diff_const)) ,) ,))) ,)) ; block_data . terminator = Some (Terminator { source_info , kind : TerminatorKind :: Assert { cond : Operand :: Copy (is_ok) , expected : true , target : new_block , msg : Box :: new (AssertKind :: InvalidEnumConstruction (Operand :: Copy (discr))) , unwind : UnwindAction :: Unreachable , } , }) ; }
}