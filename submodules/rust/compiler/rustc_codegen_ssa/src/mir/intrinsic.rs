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
mkuse!{use rustc_abi :: WrappingRange ;}
mkuse!{use rustc_middle :: mir :: SourceInfo ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt } ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_session :: config :: OptLevel ;}
mkuse!{use rustc_span :: sym ;}
mkuse!{use super :: FunctionCx ;}
mkuse!{use super :: operand :: OperandRef ;}
mkuse!{use super :: place :: PlaceRef ;}
mkuse!{use crate :: common :: { AtomicRmwBinOp , SynchronizationScope } ;}
mkuse!{use crate :: errors :: InvalidMonomorphization ;}
mkuse!{use crate :: traits :: * ;}
mkuse!{use crate :: { MemFlags , meth , size_of_val } ;}

macro_rules! copy_intrinsic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copy_intrinsic in module {}", module_path!());
    };
}

mkfn!{
    copy_intrinsic_introspect!();
    fn copy_intrinsic < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx , allow_overlap : bool , volatile : bool , ty : Ty < 'tcx > , dst : Bx :: Value , src : Bx :: Value , count : Bx :: Value ,) { let layout = bx . layout_of (ty) ; let size = layout . size ; let align = layout . align . abi ; let size = bx . mul (bx . const_usize (size . bytes ()) , count) ; let flags = if volatile { MemFlags :: VOLATILE } else { MemFlags :: empty () } ; if allow_overlap { bx . memmove (dst , align , src , align , size , flags) ; } else { bx . memcpy (dst , align , src , align , size , flags) ; } }
}

macro_rules! memset_intrinsic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function memset_intrinsic in module {}", module_path!());
    };
}

mkfn!{
    memset_intrinsic_introspect!();
    fn memset_intrinsic < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx , volatile : bool , ty : Ty < 'tcx > , dst : Bx :: Value , val : Bx :: Value , count : Bx :: Value ,) { let layout = bx . layout_of (ty) ; let size = layout . size ; let align = layout . align . abi ; let size = bx . mul (bx . const_usize (size . bytes ()) , count) ; let flags = if volatile { MemFlags :: VOLATILE } else { MemFlags :: empty () } ; bx . memset (dst , val , size , align , flags) ; }
}
mkitem!{mkimpl!{impl < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > FunctionCx < 'a , 'tcx , Bx > { #[doc = " In the `Err` case, returns the instance that should be called instead."] pub fn codegen_intrinsic_call (& mut self , bx : & mut Bx , instance : ty :: Instance < 'tcx > , args : & [OperandRef < 'tcx , Bx :: Value >] , result : PlaceRef < 'tcx , Bx :: Value > , source_info : SourceInfo ,) -> Result < () , ty :: Instance < 'tcx > > { let span = source_info . span ; let name = bx . tcx () . item_name (instance . def_id ()) ; let fn_args = instance . args ; if let sym :: typed_swap_nonoverlapping = name { let pointee_ty = fn_args . type_at (0) ; let pointee_layout = bx . layout_of (pointee_ty) ; if ! bx . is_backend_ref (pointee_layout) || bx . sess () . opts . optimize == OptLevel :: No || bx . sess () . target . arch == "spirv" { let align = pointee_layout . align . abi ; let x_place = args [0] . val . deref (align) ; let y_place = args [1] . val . deref (align) ; bx . typed_place_swap (x_place , y_place , pointee_layout) ; return Ok (()) ; } } let invalid_monomorphization_int_type = | ty | { bx . tcx () . dcx () . emit_err (InvalidMonomorphization :: BasicIntegerType { span , name , ty }) ; } ; let invalid_monomorphization_int_or_ptr_type = | ty | { bx . tcx () . dcx () . emit_err (InvalidMonomorphization :: BasicIntegerOrPtrType { span , name , ty , }) ; } ; let parse_atomic_ordering = | ord : ty :: Value < 'tcx > | { let discr = ord . valtree . unwrap_branch () [0] . unwrap_leaf () ; discr . to_atomic_ordering () } ; if args . is_empty () { match name { sym :: abort | sym :: unreachable | sym :: cold_path | sym :: breakpoint | sym :: assert_zero_valid | sym :: assert_mem_uninitialized_valid | sym :: assert_inhabited | sym :: ub_checks | sym :: contract_checks | sym :: atomic_fence | sym :: atomic_singlethreadfence | sym :: caller_location => { } _ => { span_bug ! (span , "nullary intrinsic {name} must either be in a const block or explicitly opted out because it is inherently a runtime intrinsic
") ; } } } let llval = match name { sym :: abort => { bx . abort () ; return Ok (()) ; } sym :: caller_location => { let location = self . get_caller_location (bx , source_info) ; location . val . store (bx , result) ; return Ok (()) ; } sym :: va_start => bx . va_start (args [0] . immediate ()) , sym :: va_end => bx . va_end (args [0] . immediate ()) , sym :: size_of_val => { let tp_ty = fn_args . type_at (0) ; let (_ , meta) = args [0] . val . pointer_parts () ; let (llsize , _) = size_of_val :: size_and_align_of_dst (bx , tp_ty , meta) ; llsize } sym :: align_of_val => { let tp_ty = fn_args . type_at (0) ; let (_ , meta) = args [0] . val . pointer_parts () ; let (_ , llalign) = size_of_val :: size_and_align_of_dst (bx , tp_ty , meta) ; llalign } sym :: vtable_size | sym :: vtable_align => { let vtable = args [0] . immediate () ; let idx = match name { sym :: vtable_size => ty :: COMMON_VTABLE_ENTRIES_SIZE , sym :: vtable_align => ty :: COMMON_VTABLE_ENTRIES_ALIGN , _ => bug ! () , } ; let value = meth :: VirtualIndex :: from_index (idx) . get_usize (bx , vtable , instance . ty (bx . tcx () , bx . typing_env ()) ,) ; match name { sym :: vtable_size => { let size_bound = bx . data_layout () . ptr_sized_integer () . signed_max () as u128 ; bx . range_metadata (value , WrappingRange { start : 0 , end : size_bound }) ; } sym :: vtable_align => { bx . range_metadata (value , WrappingRange { start : 1 , end : ! 0 }) } _ => { } } value } sym :: arith_offset => { let ty = fn_args . type_at (0) ; let layout = bx . layout_of (ty) ; let ptr = args [0] . immediate () ; let offset = args [1] . immediate () ; bx . gep (bx . backend_type (layout) , ptr , & [offset]) } sym :: copy => { copy_intrinsic (bx , true , false , fn_args . type_at (0) , args [1] . immediate () , args [0] . immediate () , args [2] . immediate () ,) ; return Ok (()) ; } sym :: write_bytes => { memset_intrinsic (bx , false , fn_args . type_at (0) , args [0] . immediate () , args [1] . immediate () , args [2] . immediate () ,) ; return Ok (()) ; } sym :: volatile_copy_nonoverlapping_memory => { copy_intrinsic (bx , false , true , fn_args . type_at (0) , args [0] . immediate () , args [1] . immediate () , args [2] . immediate () ,) ; return Ok (()) ; } sym :: volatile_copy_memory => { copy_intrinsic (bx , true , true , fn_args . type_at (0) , args [0] . immediate () , args [1] . immediate () , args [2] . immediate () ,) ; return Ok (()) ; } sym :: volatile_set_memory => { memset_intrinsic (bx , true , fn_args . type_at (0) , args [0] . immediate () , args [1] . immediate () , args [2] . immediate () ,) ; return Ok (()) ; } sym :: volatile_store => { let dst = args [0] . deref (bx . cx ()) ; args [1] . val . volatile_store (bx , dst) ; return Ok (()) ; } sym :: unaligned_volatile_store => { let dst = args [0] . deref (bx . cx ()) ; args [1] . val . unaligned_volatile_store (bx , dst) ; return Ok (()) ; } sym :: disjoint_bitor => { let a = args [0] . immediate () ; let b = args [1] . immediate () ; bx . or_disjoint (a , b) } sym :: exact_div => { let ty = args [0] . layout . ty ; match int_type_width_signed (ty , bx . tcx ()) { Some ((_width , signed)) => { if signed { bx . exactsdiv (args [0] . immediate () , args [1] . immediate ()) } else { bx . exactudiv (args [0] . immediate () , args [1] . immediate ()) } } None => { bx . tcx () . dcx () . emit_err (InvalidMonomorphization :: BasicIntegerType { span , name , ty , }) ; return Ok (()) ; } } } sym :: fadd_fast | sym :: fsub_fast | sym :: fmul_fast | sym :: fdiv_fast | sym :: frem_fast => { match float_type_width (args [0] . layout . ty) { Some (_width) => match name { sym :: fadd_fast => bx . fadd_fast (args [0] . immediate () , args [1] . immediate ()) , sym :: fsub_fast => bx . fsub_fast (args [0] . immediate () , args [1] . immediate ()) , sym :: fmul_fast => bx . fmul_fast (args [0] . immediate () , args [1] . immediate ()) , sym :: fdiv_fast => bx . fdiv_fast (args [0] . immediate () , args [1] . immediate ()) , sym :: frem_fast => bx . frem_fast (args [0] . immediate () , args [1] . immediate ()) , _ => bug ! () , } , None => { bx . tcx () . dcx () . emit_err (InvalidMonomorphization :: BasicFloatType { span , name , ty : args [0] . layout . ty , }) ; return Ok (()) ; } } } sym :: fadd_algebraic | sym :: fsub_algebraic | sym :: fmul_algebraic | sym :: fdiv_algebraic | sym :: frem_algebraic => match float_type_width (args [0] . layout . ty) { Some (_width) => match name { sym :: fadd_algebraic => { bx . fadd_algebraic (args [0] . immediate () , args [1] . immediate ()) } sym :: fsub_algebraic => { bx . fsub_algebraic (args [0] . immediate () , args [1] . immediate ()) } sym :: fmul_algebraic => { bx . fmul_algebraic (args [0] . immediate () , args [1] . immediate ()) } sym :: fdiv_algebraic => { bx . fdiv_algebraic (args [0] . immediate () , args [1] . immediate ()) } sym :: frem_algebraic => { bx . frem_algebraic (args [0] . immediate () , args [1] . immediate ()) } _ => bug ! () , } , None => { bx . tcx () . dcx () . emit_err (InvalidMonomorphization :: BasicFloatType { span , name , ty : args [0] . layout . ty , }) ; return Ok (()) ; } } , sym :: float_to_int_unchecked => { if float_type_width (args [0] . layout . ty) . is_none () { bx . tcx () . dcx () . emit_err (InvalidMonomorphization :: FloatToIntUnchecked { span , ty : args [0] . layout . ty , }) ; return Ok (()) ; } let Some ((_width , signed)) = int_type_width_signed (result . layout . ty , bx . tcx ()) else { bx . tcx () . dcx () . emit_err (InvalidMonomorphization :: FloatToIntUnchecked { span , ty : result . layout . ty , }) ; return Ok (()) ; } ; if signed { bx . fptosi (args [0] . immediate () , bx . backend_type (result . layout)) } else { bx . fptoui (args [0] . immediate () , bx . backend_type (result . layout)) } } sym :: atomic_load => { let ty = fn_args . type_at (0) ; if ! (int_type_width_signed (ty , bx . tcx ()) . is_some () || ty . is_raw_ptr ()) { invalid_monomorphization_int_or_ptr_type (ty) ; return Ok (()) ; } let ordering = fn_args . const_at (1) . to_value () ; let layout = bx . layout_of (ty) ; let source = args [0] . immediate () ; bx . atomic_load (bx . backend_type (layout) , source , parse_atomic_ordering (ordering) , layout . size ,) } sym :: atomic_store => { let ty = fn_args . type_at (0) ; if ! (int_type_width_signed (ty , bx . tcx ()) . is_some () || ty . is_raw_ptr ()) { invalid_monomorphization_int_or_ptr_type (ty) ; return Ok (()) ; } let ordering = fn_args . const_at (1) . to_value () ; let size = bx . layout_of (ty) . size ; let val = args [1] . immediate () ; let ptr = args [0] . immediate () ; bx . atomic_store (val , ptr , parse_atomic_ordering (ordering) , size) ; return Ok (()) ; } sym :: atomic_cxchg | sym :: atomic_cxchgweak => { let ty = fn_args . type_at (0) ; if ! (int_type_width_signed (ty , bx . tcx ()) . is_some () || ty . is_raw_ptr ()) { invalid_monomorphization_int_or_ptr_type (ty) ; return Ok (()) ; } let succ_ordering = fn_args . const_at (1) . to_value () ; let fail_ordering = fn_args . const_at (2) . to_value () ; let weak = name == sym :: atomic_cxchgweak ; let dst = args [0] . immediate () ; let cmp = args [1] . immediate () ; let src = args [2] . immediate () ; let (val , success) = bx . atomic_cmpxchg (dst , cmp , src , parse_atomic_ordering (succ_ordering) , parse_atomic_ordering (fail_ordering) , weak ,) ; let val = bx . from_immediate (val) ; let success = bx . from_immediate (success) ; let dest = result . project_field (bx , 0) ; bx . store_to_place (val , dest . val) ; let dest = result . project_field (bx , 1) ; bx . store_to_place (success , dest . val) ; return Ok (()) ; } sym :: atomic_max | sym :: atomic_min => { let atom_op = if name == sym :: atomic_max { AtomicRmwBinOp :: AtomicMax } else { AtomicRmwBinOp :: AtomicMin } ; let ty = fn_args . type_at (0) ; if matches ! (ty . kind () , ty :: Int (_)) { let ordering = fn_args . const_at (1) . to_value () ; let ptr = args [0] . immediate () ; let val = args [1] . immediate () ; bx . atomic_rmw (atom_op , ptr , val , parse_atomic_ordering (ordering) , false ,) } else { invalid_monomorphization_int_type (ty) ; return Ok (()) ; } } sym :: atomic_umax | sym :: atomic_umin => { let atom_op = if name == sym :: atomic_umax { AtomicRmwBinOp :: AtomicUMax } else { AtomicRmwBinOp :: AtomicUMin } ; let ty = fn_args . type_at (0) ; if matches ! (ty . kind () , ty :: Uint (_)) { let ordering = fn_args . const_at (1) . to_value () ; let ptr = args [0] . immediate () ; let val = args [1] . immediate () ; bx . atomic_rmw (atom_op , ptr , val , parse_atomic_ordering (ordering) , false ,) } else { invalid_monomorphization_int_type (ty) ; return Ok (()) ; } } sym :: atomic_xchg => { let ty = fn_args . type_at (0) ; let ordering = fn_args . const_at (1) . to_value () ; if int_type_width_signed (ty , bx . tcx ()) . is_some () || ty . is_raw_ptr () { let ptr = args [0] . immediate () ; let val = args [1] . immediate () ; let atomic_op = AtomicRmwBinOp :: AtomicXchg ; bx . atomic_rmw (atomic_op , ptr , val , parse_atomic_ordering (ordering) , ty . is_raw_ptr () ,) } else { invalid_monomorphization_int_or_ptr_type (ty) ; return Ok (()) ; } } sym :: atomic_xadd | sym :: atomic_xsub | sym :: atomic_and | sym :: atomic_nand | sym :: atomic_or | sym :: atomic_xor => { let atom_op = match name { sym :: atomic_xadd => AtomicRmwBinOp :: AtomicAdd , sym :: atomic_xsub => AtomicRmwBinOp :: AtomicSub , sym :: atomic_and => AtomicRmwBinOp :: AtomicAnd , sym :: atomic_nand => AtomicRmwBinOp :: AtomicNand , sym :: atomic_or => AtomicRmwBinOp :: AtomicOr , sym :: atomic_xor => AtomicRmwBinOp :: AtomicXor , _ => unreachable ! () , } ; let ty_mem = fn_args . type_at (0) ; let ty_op = fn_args . type_at (1) ; let ordering = fn_args . const_at (2) . to_value () ; if (int_type_width_signed (ty_mem , bx . tcx ()) . is_some () && ty_op == ty_mem) || (ty_mem . is_raw_ptr () && ty_op == bx . tcx () . types . usize) { let ptr = args [0] . immediate () ; let val = args [1] . immediate () ; bx . atomic_rmw (atom_op , ptr , val , parse_atomic_ordering (ordering) , ty_mem . is_raw_ptr () ,) } else { invalid_monomorphization_int_or_ptr_type (ty_mem) ; return Ok (()) ; } } sym :: atomic_fence => { let ordering = fn_args . const_at (0) . to_value () ; bx . atomic_fence (parse_atomic_ordering (ordering) , SynchronizationScope :: CrossThread) ; return Ok (()) ; } sym :: atomic_singlethreadfence => { let ordering = fn_args . const_at (0) . to_value () ; bx . atomic_fence (parse_atomic_ordering (ordering) , SynchronizationScope :: SingleThread ,) ; return Ok (()) ; } sym :: nontemporal_store => { let dst = args [0] . deref (bx . cx ()) ; args [1] . val . nontemporal_store (bx , dst) ; return Ok (()) ; } sym :: ptr_offset_from | sym :: ptr_offset_from_unsigned => { let ty = fn_args . type_at (0) ; let pointee_size = bx . layout_of (ty) . size ; let a = args [0] . immediate () ; let b = args [1] . immediate () ; let a = bx . ptrtoint (a , bx . type_isize ()) ; let b = bx . ptrtoint (b , bx . type_isize ()) ; let pointee_size = bx . const_usize (pointee_size . bytes ()) ; if name == sym :: ptr_offset_from { let d = bx . sub (a , b) ; bx . exactsdiv (d , pointee_size) } else { let d = bx . unchecked_usub (a , b) ; bx . exactudiv (d , pointee_size) } } sym :: cold_path => { return Ok (()) ; } _ => { return bx . codegen_intrinsic_call (instance , args , result , span) ; } } ; if result . layout . ty . is_bool () { let val = bx . from_immediate (llval) ; bx . store_to_place (val , result . val) ; } else if ! result . layout . ty . is_unit () { bx . store_to_place (llval , result . val) ; } Ok (()) } }}}

macro_rules! int_type_width_signed_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function int_type_width_signed in module {}", module_path!());
    };
}

mkfn!{
    int_type_width_signed_introspect!();
    fn int_type_width_signed (ty : Ty < '_ > , tcx : TyCtxt < '_ >) -> Option < (u64 , bool) > { match ty . kind () { ty :: Int (t) => { Some ((t . bit_width () . unwrap_or (u64 :: from (tcx . sess . target . pointer_width)) , true)) } ty :: Uint (t) => { Some ((t . bit_width () . unwrap_or (u64 :: from (tcx . sess . target . pointer_width)) , false)) } _ => None , } }
}

macro_rules! float_type_width_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function float_type_width in module {}", module_path!());
    };
}

mkfn!{
    float_type_width_introspect!();
    fn float_type_width (ty : Ty < '_ >) -> Option < u64 > { match ty . kind () { ty :: Float (t) => Some (t . bit_width ()) , _ => None , } }
}