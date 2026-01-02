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
mkuse!{use rustc_abi :: { BackendRepr , FieldIdx , VariantIdx } ;}
mkuse!{use rustc_data_structures :: stack :: ensure_sufficient_stack ;}
mkuse!{use rustc_middle :: mir :: interpret :: { EvalToValTreeResult , GlobalId , ValTreeCreationError } ;}
mkuse!{use rustc_middle :: ty :: layout :: { LayoutCx , TyAndLayout } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt } ;}
mkuse!{use rustc_middle :: { bug , mir } ;}
mkuse!{use rustc_span :: DUMMY_SP ;}
mkuse!{use tracing :: { debug , instrument , trace } ;}
mkuse!{use super :: VALTREE_MAX_NODES ;}
mkuse!{use super :: eval_queries :: { mk_eval_cx_to_read_const_val , op_to_const } ;}
mkuse!{use super :: machine :: CompileTimeInterpCx ;}
mkuse!{use crate :: const_eval :: CanAccessMutGlobal ;}
mkuse!{use crate :: interpret :: { ImmTy , Immediate , InternKind , MPlaceTy , MemPlaceMeta , MemoryKind , PlaceTy , Projectable , Scalar , intern_const_alloc_recursive , } ;}

macro_rules! branches_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function branches in module {}", module_path!());
    };
}

mkfn!{
    branches_introspect!();
    #[instrument (skip (ecx) , level = "debug")] fn branches < 'tcx > (ecx : & CompileTimeInterpCx < 'tcx > , place : & MPlaceTy < 'tcx > , field_count : usize , variant : Option < VariantIdx > , num_nodes : & mut usize ,) -> EvalToValTreeResult < 'tcx > { let place = match variant { Some (variant) => ecx . project_downcast (place , variant) . unwrap () , None => place . clone () , } ; debug ! (? place) ; let mut branches = Vec :: with_capacity (field_count + variant . is_some () as usize) ; if let Some (variant) = variant { branches . push (ty :: ValTree :: from_scalar_int (* ecx . tcx , variant . as_u32 () . into ())) ; } for i in 0 .. field_count { let field = ecx . project_field (& place , FieldIdx :: from_usize (i)) . unwrap () ; let valtree = const_to_valtree_inner (ecx , & field , num_nodes) ? ; branches . push (valtree) ; } if branches . len () == 0 { * num_nodes += 1 ; } Ok (ty :: ValTree :: from_branches (* ecx . tcx , branches)) }
}

macro_rules! slice_branches_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function slice_branches in module {}", module_path!());
    };
}

mkfn!{
    slice_branches_introspect!();
    #[instrument (skip (ecx) , level = "debug")] fn slice_branches < 'tcx > (ecx : & CompileTimeInterpCx < 'tcx > , place : & MPlaceTy < 'tcx > , num_nodes : & mut usize ,) -> EvalToValTreeResult < 'tcx > { let n = place . len (ecx) . unwrap_or_else (| _ | panic ! ("expected to use len of place {place:?}")) ; let mut elems = Vec :: with_capacity (n as usize) ; for i in 0 .. n { let place_elem = ecx . project_index (place , i) . unwrap () ; let valtree = const_to_valtree_inner (ecx , & place_elem , num_nodes) ? ; elems . push (valtree) ; } Ok (ty :: ValTree :: from_branches (* ecx . tcx , elems)) }
}

macro_rules! const_to_valtree_inner_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function const_to_valtree_inner in module {}", module_path!());
    };
}

mkfn!{
    const_to_valtree_inner_introspect!();
    #[instrument (skip (ecx) , level = "debug")] fn const_to_valtree_inner < 'tcx > (ecx : & CompileTimeInterpCx < 'tcx > , place : & MPlaceTy < 'tcx > , num_nodes : & mut usize ,) -> EvalToValTreeResult < 'tcx > { let tcx = * ecx . tcx ; let ty = place . layout . ty ; debug ! ("ty kind: {:?}" , ty . kind ()) ; if * num_nodes >= VALTREE_MAX_NODES { return Err (ValTreeCreationError :: NodesOverflow) ; } match ty . kind () { ty :: FnDef (..) => { * num_nodes += 1 ; Ok (ty :: ValTree :: zst (tcx)) } ty :: Bool | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Char => { let val = ecx . read_immediate (place) . report_err () ? ; let val = val . to_scalar_int () . unwrap () ; * num_nodes += 1 ; Ok (ty :: ValTree :: from_scalar_int (tcx , val)) } ty :: Pat (base , ..) => { let mut place = place . clone () ; place . layout = ecx . layout_of (* base) . unwrap () ; ensure_sufficient_stack (| | const_to_valtree_inner (ecx , & place , num_nodes)) } , ty :: RawPtr (_ , _) => { let val = ecx . read_immediate (place) . report_err () ? ; if matches ! (val . layout . backend_repr , BackendRepr :: ScalarPair (..)) { return Err (ValTreeCreationError :: NonSupportedType (ty)) ; } let val = val . to_scalar () ; let Ok (val) = val . try_to_scalar_int () else { return Err (ValTreeCreationError :: NonSupportedType (ty)) ; } ; Ok (ty :: ValTree :: from_scalar_int (tcx , val)) } ty :: FnPtr (..) => Err (ValTreeCreationError :: NonSupportedType (ty)) , ty :: Ref (_ , _ , _) => { let derefd_place = ecx . deref_pointer (place) . report_err () ? ; const_to_valtree_inner (ecx , & derefd_place , num_nodes) } ty :: Str | ty :: Slice (_) | ty :: Array (_ , _) => { slice_branches (ecx , place , num_nodes) } ty :: Dynamic (..) => Err (ValTreeCreationError :: NonSupportedType (ty)) , ty :: Tuple (elem_tys) => { branches (ecx , place , elem_tys . len () , None , num_nodes) } ty :: Adt (def , _) => { if def . is_union () { return Err (ValTreeCreationError :: NonSupportedType (ty)) ; } else if def . variants () . is_empty () { bug ! ("uninhabited types should have errored and never gotten converted to valtree") } let variant = ecx . read_discriminant (place) . report_err () ? ; branches (ecx , place , def . variant (variant) . fields . len () , def . is_enum () . then_some (variant) , num_nodes) } ty :: Never | ty :: Error (_) | ty :: Foreign (..) | ty :: Infer (ty :: FreshIntTy (_)) | ty :: Infer (ty :: FreshFloatTy (_)) | ty :: Alias (..) | ty :: Param (_) | ty :: Bound (..) | ty :: Placeholder (..) | ty :: Infer (_) | ty :: Closure (..) | ty :: CoroutineClosure (..) | ty :: Coroutine (..) | ty :: CoroutineWitness (..) | ty :: UnsafeBinder (_) => Err (ValTreeCreationError :: NonSupportedType (ty)) , } }
}

macro_rules! reconstruct_place_meta_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function reconstruct_place_meta in module {}", module_path!());
    };
}

mkfn!{
    reconstruct_place_meta_introspect!();
    #[doc = " Valtrees don't store the `MemPlaceMeta` that all dynamically sized values have in the interpreter."] #[doc = " This function reconstructs it."] fn reconstruct_place_meta < 'tcx > (layout : TyAndLayout < 'tcx > , valtree : ty :: ValTree < 'tcx > , tcx : TyCtxt < 'tcx > ,) -> MemPlaceMeta { if layout . is_sized () { return MemPlaceMeta :: None ; } let mut last_valtree = valtree ; let tail = tcx . struct_tail_raw (layout . ty , | ty | ty , | | { let branches = last_valtree . unwrap_branch () ; last_valtree = * branches . last () . unwrap () ; debug ! (? branches , ? last_valtree) ; } ,) ; match tail . kind () { ty :: Slice (..) | ty :: Str => { } _ => bug ! ("unsized tail of a valtree must be Slice or Str") , } ; let num_elems = last_valtree . unwrap_branch () . len () ; MemPlaceMeta :: Meta (Scalar :: from_target_usize (num_elems as u64 , & tcx)) }
}

macro_rules! create_valtree_place_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_valtree_place in module {}", module_path!());
    };
}

mkfn!{
    create_valtree_place_introspect!();
    #[instrument (skip (ecx) , level = "debug" , ret)] fn create_valtree_place < 'tcx > (ecx : & mut CompileTimeInterpCx < 'tcx > , layout : TyAndLayout < 'tcx > , valtree : ty :: ValTree < 'tcx > ,) -> MPlaceTy < 'tcx > { let meta = reconstruct_place_meta (layout , valtree , ecx . tcx . tcx) ; ecx . allocate_dyn (layout , MemoryKind :: Stack , meta) . unwrap () }
}

macro_rules! eval_to_valtree_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function eval_to_valtree in module {}", module_path!());
    };
}

mkfn!{
    eval_to_valtree_introspect!();
    #[doc = " Evaluates a constant and turns it into a type-level constant value."] pub (crate) fn eval_to_valtree < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , cid : GlobalId < 'tcx > ,) -> EvalToValTreeResult < 'tcx > { debug_assert_eq ! (typing_env . typing_mode , ty :: TypingMode :: PostAnalysis) ; let const_alloc = tcx . eval_to_allocation_raw (typing_env . as_query_input (cid)) ? ; let ecx = mk_eval_cx_to_read_const_val (tcx , DUMMY_SP , typing_env , CanAccessMutGlobal :: No ,) ; let place = ecx . raw_const_to_mplace (const_alloc) . unwrap () ; debug ! (? place) ; let mut num_nodes = 0 ; const_to_valtree_inner (& ecx , & place , & mut num_nodes) }
}

macro_rules! valtree_to_const_value_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function valtree_to_const_value in module {}", module_path!());
    };
}

mkfn!{
    valtree_to_const_value_introspect!();
    #[doc = " Converts a `ValTree` to a `ConstValue`, which is needed after mir"] #[doc = " construction has finished."] #[instrument (skip (tcx) , level = "debug" , ret)] pub fn valtree_to_const_value < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , cv : ty :: Value < 'tcx > ,) -> mir :: ConstValue { match * cv . ty . kind () { ty :: FnDef (..) => { assert ! (cv . valtree . is_zst ()) ; mir :: ConstValue :: ZeroSized } ty :: Bool | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Char | ty :: RawPtr (_ , _) => { mir :: ConstValue :: Scalar (Scalar :: Int (cv . valtree . unwrap_leaf ())) } ty :: Pat (ty , _) => { let cv = ty :: Value { valtree : cv . valtree , ty } ; valtree_to_const_value (tcx , typing_env , cv) } ty :: Ref (_ , inner_ty , _) => { let mut ecx = mk_eval_cx_to_read_const_val (tcx , DUMMY_SP , typing_env , CanAccessMutGlobal :: No) ; let imm = valtree_to_ref (& mut ecx , cv . valtree , inner_ty) ; let imm = ImmTy :: from_immediate (imm , tcx . layout_of (typing_env . as_query_input (cv . ty)) . unwrap () ,) ; op_to_const (& ecx , & imm . into () , false) } ty :: Tuple (_) | ty :: Array (_ , _) | ty :: Adt (..) => { let layout = tcx . layout_of (typing_env . as_query_input (cv . ty)) . unwrap () ; if layout . is_zst () { return mir :: ConstValue :: ZeroSized ; } if layout . backend_repr . is_scalar () && (matches ! (cv . ty . kind () , ty :: Tuple (_)) || matches ! (cv . ty . kind () , ty :: Adt (def , _) if def . is_struct ())) { let branches = cv . valtree . unwrap_branch () ; for (i , & inner_valtree) in branches . iter () . enumerate () { let field = layout . field (& LayoutCx :: new (tcx , typing_env) , i) ; if ! field . is_zst () { let cv = ty :: Value { valtree : inner_valtree , ty : field . ty } ; return valtree_to_const_value (tcx , typing_env , cv) ; } } bug ! ("could not find non-ZST field during in {layout:#?}") ; } let mut ecx = mk_eval_cx_to_read_const_val (tcx , DUMMY_SP , typing_env , CanAccessMutGlobal :: No) ; let place = create_valtree_place (& mut ecx , layout , cv . valtree) ; valtree_into_mplace (& mut ecx , & place , cv . valtree) ; dump_place (& ecx , & place) ; intern_const_alloc_recursive (& mut ecx , InternKind :: Constant , & place) . unwrap () ; op_to_const (& ecx , & place . into () , false) } ty :: Never | ty :: Error (_) | ty :: Foreign (..) | ty :: Infer (ty :: FreshIntTy (_)) | ty :: Infer (ty :: FreshFloatTy (_)) | ty :: Alias (..) | ty :: Param (_) | ty :: Bound (..) | ty :: Placeholder (..) | ty :: Infer (_) | ty :: Closure (..) | ty :: CoroutineClosure (..) | ty :: Coroutine (..) | ty :: CoroutineWitness (..) | ty :: FnPtr (..) | ty :: Str | ty :: Slice (_) | ty :: Dynamic (..) | ty :: UnsafeBinder (_) => { bug ! ("no ValTree should have been created for type {:?}" , cv . ty . kind ()) } } }
}

macro_rules! valtree_to_ref_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function valtree_to_ref in module {}", module_path!());
    };
}

mkfn!{
    valtree_to_ref_introspect!();
    #[doc = " Put a valtree into memory and return a reference to that."] fn valtree_to_ref < 'tcx > (ecx : & mut CompileTimeInterpCx < 'tcx > , valtree : ty :: ValTree < 'tcx > , pointee_ty : Ty < 'tcx > ,) -> Immediate { let pointee_place = create_valtree_place (ecx , ecx . layout_of (pointee_ty) . unwrap () , valtree) ; debug ! (? pointee_place) ; valtree_into_mplace (ecx , & pointee_place , valtree) ; dump_place (ecx , & pointee_place) ; intern_const_alloc_recursive (ecx , InternKind :: Constant , & pointee_place) . unwrap () ; pointee_place . to_ref (& ecx . tcx) }
}

macro_rules! valtree_into_mplace_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function valtree_into_mplace in module {}", module_path!());
    };
}

mkfn!{
    valtree_into_mplace_introspect!();
    #[instrument (skip (ecx) , level = "debug")] fn valtree_into_mplace < 'tcx > (ecx : & mut CompileTimeInterpCx < 'tcx > , place : & MPlaceTy < 'tcx > , valtree : ty :: ValTree < 'tcx > ,) { let ty = place . layout . ty ; match ty . kind () { ty :: FnDef (_ , _) => { } ty :: Bool | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Char | ty :: RawPtr (..) => { let scalar_int = valtree . unwrap_leaf () ; debug ! ("writing trivial valtree {:?} to place {:?}" , scalar_int , place) ; ecx . write_immediate (Immediate :: Scalar (scalar_int . into ()) , place) . unwrap () ; } ty :: Ref (_ , inner_ty , _) => { let imm = valtree_to_ref (ecx , valtree , * inner_ty) ; debug ! (? imm) ; ecx . write_immediate (imm , place) . unwrap () ; } ty :: Adt (_ , _) | ty :: Tuple (_) | ty :: Array (_ , _) | ty :: Str | ty :: Slice (_) => { let branches = valtree . unwrap_branch () ; let (place_adjusted , branches , variant_idx) = match ty . kind () { ty :: Adt (def , _) if def . is_enum () => { let scalar_int = branches [0] . unwrap_leaf () ; let variant_idx = VariantIdx :: from_u32 (scalar_int . to_u32 ()) ; let variant = def . variant (variant_idx) ; debug ! (? variant) ; (ecx . project_downcast (place , variant_idx) . unwrap () , & branches [1 ..] , Some (variant_idx) ,) } _ => (place . clone () , branches , None) , } ; debug ! (? place_adjusted , ? branches) ; for (i , inner_valtree) in branches . iter () . enumerate () { debug ! (? i , ? inner_valtree) ; let place_inner = match ty . kind () { ty :: Str | ty :: Slice (_) | ty :: Array (..) => { ecx . project_index (place , i as u64) . unwrap () } _ => ecx . project_field (& place_adjusted , FieldIdx :: from_usize (i)) . unwrap () , } ; debug ! (? place_inner) ; valtree_into_mplace (ecx , & place_inner , * inner_valtree) ; dump_place (ecx , & place_inner) ; } debug ! ("dump of place_adjusted:") ; dump_place (ecx , & place_adjusted) ; if let Some (variant_idx) = variant_idx { ecx . write_discriminant (variant_idx , place) . unwrap () ; } debug ! ("dump of place after writing discriminant:") ; dump_place (ecx , place) ; } _ => bug ! ("shouldn't have created a ValTree for {:?}" , ty) , } }
}

macro_rules! dump_place_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dump_place in module {}", module_path!());
    };
}

mkfn!{
    dump_place_introspect!();
    fn dump_place < 'tcx > (ecx : & CompileTimeInterpCx < 'tcx > , place : & MPlaceTy < 'tcx >) { trace ! ("{:?}" , ecx . dump_place (& PlaceTy :: from (place . clone ()))) ; }
}