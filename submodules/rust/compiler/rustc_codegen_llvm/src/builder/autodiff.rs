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
mkuse!{use std :: ptr ;}
mkuse!{use rustc_ast :: expand :: autodiff_attrs :: { AutoDiffAttrs , DiffActivity , DiffMode } ;}
mkuse!{use rustc_codegen_ssa :: common :: TypeKind ;}
mkuse!{use rustc_codegen_ssa :: traits :: { BaseTypeCodegenMethods , BuilderMethods } ;}
mkuse!{use rustc_middle :: ty :: { PseudoCanonicalInput , Ty , TyCtxt , TypingEnv } ;}
mkuse!{use rustc_middle :: { bug , ty } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: builder :: { Builder , PlaceRef , UNNAMED } ;}
mkuse!{use crate :: context :: SimpleCx ;}
mkuse!{use crate :: declare :: declare_simple_fn ;}
mkuse!{use crate :: llvm ;}
mkuse!{use crate :: llvm :: { Metadata , TRUE , Type } ;}
mkuse!{use crate :: value :: Value ;}

macro_rules! adjust_activity_to_abi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function adjust_activity_to_abi in module {}", module_path!());
    };
}

mkfn!{
    adjust_activity_to_abi_introspect!();
    pub (crate) fn adjust_activity_to_abi < 'tcx > (tcx : TyCtxt < 'tcx > , fn_ty : Ty < 'tcx > , da : & mut Vec < DiffActivity > ,) { if ! matches ! (fn_ty . kind () , ty :: FnDef (..)) { bug ! ("expected fn def for autodiff, got {:?}" , fn_ty) ; } let sig = fn_ty . fn_sig (tcx) . skip_binder () ; let mut new_activities = vec ! [] ; let mut new_positions = vec ! [] ; for (i , ty) in sig . inputs () . iter () . enumerate () { if let Some (inner_ty) = ty . builtin_deref (true) { if inner_ty . is_slice () { let sty = match inner_ty . builtin_index () { Some (sty) => sty , None => { panic ! ("slice element type unknown") ; } } ; let pci = PseudoCanonicalInput { typing_env : TypingEnv :: fully_monomorphized () , value : sty , } ; let layout = tcx . layout_of (pci) ; let elem_size = match layout { Ok (layout) => layout . size , Err (_) => { bug ! ("autodiff failed to compute slice element size") ; } } ; let elem_size : u32 = elem_size . bytes () as u32 ; if ! da . is_empty () { let activity = match da [i] { DiffActivity :: DualOnly | DiffActivity :: Dual | DiffActivity :: Dualv | DiffActivity :: DuplicatedOnly | DiffActivity :: Duplicated => { DiffActivity :: FakeActivitySize (Some (elem_size)) } DiffActivity :: Const => DiffActivity :: Const , _ => bug ! ("unexpected activity for ptr/ref") , } ; new_activities . push (activity) ; new_positions . push (i + 1) ; } continue ; } } } for _ in 0 .. new_activities . len () { let pos = new_positions . pop () . unwrap () ; let activity = new_activities . pop () . unwrap () ; da . insert (pos , activity) ; } }
}

macro_rules! match_args_from_caller_to_enzyme_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function match_args_from_caller_to_enzyme in module {}", module_path!());
    };
}

mkfn!{
    match_args_from_caller_to_enzyme_introspect!();
    fn match_args_from_caller_to_enzyme < 'll , 'tcx > (cx : & SimpleCx < 'll > , builder : & mut Builder < '_ , 'll , 'tcx > , width : u32 , args : & mut Vec < & 'll llvm :: Value > , inputs : & [DiffActivity] , outer_args : & [& 'll llvm :: Value] ,) { debug ! ("matching autodiff arguments") ; let mut outer_pos : usize = 0 ; let mut activity_pos = 0 ; let enzyme_const = cx . create_metadata (b"enzyme_const") ; let enzyme_out = cx . create_metadata (b"enzyme_out") ; let enzyme_dup = cx . create_metadata (b"enzyme_dup") ; let enzyme_dupv = cx . create_metadata (b"enzyme_dupv") ; let enzyme_dupnoneed = cx . create_metadata (b"enzyme_dupnoneed") ; let enzyme_dupnoneedv = cx . create_metadata (b"enzyme_dupnoneedv") ; while activity_pos < inputs . len () { let diff_activity = inputs [activity_pos as usize] ; let (activity , duplicated) : (& Metadata , bool) = match diff_activity { DiffActivity :: None => panic ! ("not a valid input activity") , DiffActivity :: Const => (enzyme_const , false) , DiffActivity :: Active => (enzyme_out , false) , DiffActivity :: ActiveOnly => (enzyme_out , false) , DiffActivity :: Dual => (enzyme_dup , true) , DiffActivity :: Dualv => (enzyme_dupv , true) , DiffActivity :: DualOnly => (enzyme_dupnoneed , true) , DiffActivity :: DualvOnly => (enzyme_dupnoneedv , true) , DiffActivity :: Duplicated => (enzyme_dup , true) , DiffActivity :: DuplicatedOnly => (enzyme_dupnoneed , true) , DiffActivity :: FakeActivitySize (_) => (enzyme_const , false) , } ; let outer_arg = outer_args [outer_pos] ; args . push (cx . get_metadata_value (activity)) ; if matches ! (diff_activity , DiffActivity :: Dualv) { let next_outer_arg = outer_args [outer_pos + 1] ; let elem_bytes_size : u64 = match inputs [activity_pos + 1] { DiffActivity :: FakeActivitySize (Some (s)) => s . into () , _ => bug ! ("incorrect Dualv handling recognized.") , } ; let mul = unsafe { llvm :: LLVMBuildMul (builder . llbuilder , cx . get_const_int (cx . type_i64 () , elem_bytes_size) , next_outer_arg , UNNAMED ,) } ; args . push (mul) ; } args . push (outer_arg) ; if duplicated { let next_outer_arg = outer_args [outer_pos + 1] ; let next_outer_ty = cx . val_ty (next_outer_arg) ; let slice = { if activity_pos + 1 >= inputs . len () { false } else { let next_activity = inputs [activity_pos + 1] ; matches ! (next_activity , DiffActivity :: FakeActivitySize (_)) } } ; if slice { assert_eq ! (cx . type_kind (next_outer_ty) , TypeKind :: Integer) ; let iterations = if matches ! (diff_activity , DiffActivity :: Dualv) { 1 } else { width as usize } ; for i in 0 .. iterations { let next_outer_arg2 = outer_args [outer_pos + 2 * (i + 1)] ; let next_outer_ty2 = cx . val_ty (next_outer_arg2) ; assert_eq ! (cx . type_kind (next_outer_ty2) , TypeKind :: Pointer) ; let next_outer_arg3 = outer_args [outer_pos + 2 * (i + 1) + 1] ; let next_outer_ty3 = cx . val_ty (next_outer_arg3) ; assert_eq ! (cx . type_kind (next_outer_ty3) , TypeKind :: Integer) ; args . push (next_outer_arg2) ; } args . push (cx . get_metadata_value (enzyme_const)) ; args . push (next_outer_arg) ; outer_pos += 2 + 2 * iterations ; activity_pos += 2 ; } else { if matches ! (diff_activity , DiffActivity :: Duplicated | DiffActivity :: DuplicatedOnly) { assert_eq ! (cx . type_kind (next_outer_ty) , TypeKind :: Pointer) ; } args . push (next_outer_arg) ; outer_pos += 2 ; activity_pos += 1 ; for _ in 1 .. width { let next_outer_arg = outer_args [outer_pos] ; args . push (next_outer_arg) ; outer_pos += 1 ; } } } else { outer_pos += 1 ; activity_pos += 1 ; } } }
}

macro_rules! generate_enzyme_call_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function generate_enzyme_call in module {}", module_path!());
    };
}

mkfn!{
    generate_enzyme_call_introspect!();
    #[doc = " When differentiating `fn_to_diff`, take a `outer_fn` and generate another"] #[doc = " function with expected naming and calling conventions[^1] which will be"] #[doc = " discovered by the enzyme LLVM pass and its body populated with the differentiated"] #[doc = " `fn_to_diff`. `outer_fn` is then modified to have a call to the generated"] #[doc = " function and handle the differences between the Rust calling convention and"] #[doc = " Enzyme."] #[doc = " [^1]: <https://enzyme.mit.edu/getting_started/CallingConvention/>"] pub (crate) fn generate_enzyme_call < 'll , 'tcx > (builder : & mut Builder < '_ , 'll , 'tcx > , cx : & SimpleCx < 'll > , fn_to_diff : & 'll Value , outer_name : & str , ret_ty : & 'll Type , fn_args : & [& 'll Value] , attrs : AutoDiffAttrs , dest : PlaceRef < 'tcx , & 'll Value > ,) { let mut ad_name : String = match attrs . mode { DiffMode :: Forward => "__enzyme_fwddiff" , DiffMode :: Reverse => "__enzyme_autodiff" , _ => panic ! ("logic bug in autodiff, unrecognized mode") , } . to_string () ; ad_name . push_str (outer_name) ; let enzyme_ty = unsafe { llvm :: LLVMFunctionType (ret_ty , ptr :: null () , 0 , TRUE) } ; let cc = unsafe { llvm :: LLVMGetFunctionCallConv (fn_to_diff) } ; let ad_fn = declare_simple_fn (cx , & ad_name , llvm :: CallConv :: try_from (cc) . expect ("invalid callconv") , llvm :: UnnamedAddr :: No , llvm :: Visibility :: Default , enzyme_ty ,) ; let num_args = llvm :: LLVMCountParams (& fn_to_diff) ; let mut args = Vec :: with_capacity (num_args as usize + 1) ; args . push (fn_to_diff) ; let enzyme_primal_ret = cx . create_metadata (b"enzyme_primal_return") ; if matches ! (attrs . ret_activity , DiffActivity :: Dual | DiffActivity :: Active) { args . push (cx . get_metadata_value (enzyme_primal_ret)) ; } if attrs . width > 1 { let enzyme_width = cx . create_metadata (b"enzyme_width") ; args . push (cx . get_metadata_value (enzyme_width)) ; args . push (cx . get_const_int (cx . type_i64 () , attrs . width as u64)) ; } match_args_from_caller_to_enzyme (& cx , builder , attrs . width , & mut args , & attrs . input_activity , fn_args ,) ; let call = builder . call (enzyme_ty , None , None , ad_fn , & args , None , None) ; builder . store_to_place (call , dest . val) ; }
}