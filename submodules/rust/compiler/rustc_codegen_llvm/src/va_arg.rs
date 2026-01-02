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
mkuse!{use rustc_abi :: { Align , BackendRepr , Endian , HasDataLayout , Primitive , Size , TyAndLayout } ;}
mkuse!{use rustc_codegen_ssa :: MemFlags ;}
mkuse!{use rustc_codegen_ssa :: common :: IntPredicate ;}
mkuse!{use rustc_codegen_ssa :: mir :: operand :: OperandRef ;}
mkuse!{use rustc_codegen_ssa :: traits :: { BaseTypeCodegenMethods , BuilderMethods , ConstCodegenMethods , LayoutTypeCodegenMethods , } ;}
mkuse!{use rustc_middle :: ty :: Ty ;}
mkuse!{use rustc_middle :: ty :: layout :: { HasTyCtxt , LayoutOf } ;}
mkuse!{use crate :: builder :: Builder ;}
mkuse!{use crate :: type_ :: Type ;}
mkuse!{use crate :: type_of :: LayoutLlvmExt ;}
mkuse!{use crate :: value :: Value ;}

macro_rules! round_up_to_alignment_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function round_up_to_alignment in module {}", module_path!());
    };
}

mkfn!{
    round_up_to_alignment_introspect!();
    fn round_up_to_alignment < 'll > (bx : & mut Builder < '_ , 'll , '_ > , mut value : & 'll Value , align : Align ,) -> & 'll Value { value = bx . add (value , bx . cx () . const_i32 (align . bytes () as i32 - 1)) ; return bx . and (value , bx . cx () . const_i32 (- (align . bytes () as i32))) ; }
}

macro_rules! round_pointer_up_to_alignment_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function round_pointer_up_to_alignment in module {}", module_path!());
    };
}

mkfn!{
    round_pointer_up_to_alignment_introspect!();
    fn round_pointer_up_to_alignment < 'll > (bx : & mut Builder < '_ , 'll , '_ > , addr : & 'll Value , align : Align , ptr_ty : & 'll Type ,) -> & 'll Value { let ptr = bx . inbounds_ptradd (addr , bx . const_i32 (align . bytes () as i32 - 1)) ; bx . call_intrinsic ("llvm.ptrmask" , & [ptr_ty , bx . type_i32 ()] , & [ptr , bx . const_int (bx . isize_ty , - (align . bytes () as isize) as i64)] ,) }
}

macro_rules! emit_direct_ptr_va_arg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_direct_ptr_va_arg in module {}", module_path!());
    };
}

mkfn!{
    emit_direct_ptr_va_arg_introspect!();
    fn emit_direct_ptr_va_arg < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , list : OperandRef < 'tcx , & 'll Value > , size : Size , align : Align , slot_size : Align , allow_higher_align : bool , force_right_adjust : bool ,) -> (& 'll Value , Align) { let va_list_ty = bx . type_ptr () ; let va_list_addr = list . immediate () ; let ptr_align_abi = bx . tcx () . data_layout . pointer_align () . abi ; let ptr = bx . load (va_list_ty , va_list_addr , ptr_align_abi) ; let (addr , addr_align) = if allow_higher_align && align > slot_size { (round_pointer_up_to_alignment (bx , ptr , align , bx . type_ptr ()) , align) } else { (ptr , slot_size) } ; let aligned_size = size . align_to (slot_size) . bytes () as i32 ; let full_direct_size = bx . cx () . const_i32 (aligned_size) ; let next = bx . inbounds_ptradd (addr , full_direct_size) ; bx . store (next , va_list_addr , ptr_align_abi) ; if size . bytes () < slot_size . bytes () && bx . tcx () . sess . target . endian == Endian :: Big && force_right_adjust { let adjusted_size = bx . cx () . const_i32 ((slot_size . bytes () - size . bytes ()) as i32) ; let adjusted = bx . inbounds_ptradd (addr , adjusted_size) ; (adjusted , addr_align) } else { (addr , addr_align) } }
}
mkitem!{mkenum!{enum PassMode { Direct , Indirect , }}}
mkitem!{mkenum!{enum SlotSize { Bytes8 = 8 , Bytes4 = 4 , }}}
mkitem!{mkenum!{enum AllowHigherAlign { No , Yes , }}}
mkitem!{mkenum!{enum ForceRightAdjust { No , Yes , }}}

macro_rules! emit_ptr_va_arg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_ptr_va_arg in module {}", module_path!());
    };
}

mkfn!{
    emit_ptr_va_arg_introspect!();
    fn emit_ptr_va_arg < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , list : OperandRef < 'tcx , & 'll Value > , target_ty : Ty < 'tcx > , pass_mode : PassMode , slot_size : SlotSize , allow_higher_align : AllowHigherAlign , force_right_adjust : ForceRightAdjust ,) -> & 'll Value { let indirect = matches ! (pass_mode , PassMode :: Indirect) ; let allow_higher_align = matches ! (allow_higher_align , AllowHigherAlign :: Yes) ; let force_right_adjust = matches ! (force_right_adjust , ForceRightAdjust :: Yes) ; let slot_size = Align :: from_bytes (slot_size as u64) . unwrap () ; let layout = bx . cx . layout_of (target_ty) ; let (llty , size , align) = if indirect { (bx . cx . layout_of (Ty :: new_imm_ptr (bx . cx . tcx , target_ty)) . llvm_type (bx . cx) , bx . cx . data_layout () . pointer_size () , bx . cx . data_layout () . pointer_align () ,) } else { (layout . llvm_type (bx . cx) , layout . size , layout . align) } ; let (addr , addr_align) = emit_direct_ptr_va_arg (bx , list , size , align . abi , slot_size , allow_higher_align , force_right_adjust ,) ; if indirect { let tmp_ret = bx . load (llty , addr , addr_align) ; bx . load (bx . cx . layout_of (target_ty) . llvm_type (bx . cx) , tmp_ret , align . abi) } else { bx . load (llty , addr , addr_align) } }
}

macro_rules! emit_aapcs_va_arg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_aapcs_va_arg in module {}", module_path!());
    };
}

mkfn!{
    emit_aapcs_va_arg_introspect!();
    fn emit_aapcs_va_arg < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , list : OperandRef < 'tcx , & 'll Value > , target_ty : Ty < 'tcx > ,) -> & 'll Value { let dl = bx . cx . data_layout () ; let va_list_addr = list . immediate () ; let ptr_offset = 8 ; let i32_offset = 4 ; let gr_top = bx . inbounds_ptradd (va_list_addr , bx . cx . const_usize (ptr_offset)) ; let vr_top = bx . inbounds_ptradd (va_list_addr , bx . cx . const_usize (2 * ptr_offset)) ; let gr_offs = bx . inbounds_ptradd (va_list_addr , bx . cx . const_usize (3 * ptr_offset)) ; let vr_offs = bx . inbounds_ptradd (va_list_addr , bx . cx . const_usize (3 * ptr_offset + i32_offset)) ; let layout = bx . cx . layout_of (target_ty) ; let maybe_reg = bx . append_sibling_block ("va_arg.maybe_reg") ; let in_reg = bx . append_sibling_block ("va_arg.in_reg") ; let on_stack = bx . append_sibling_block ("va_arg.on_stack") ; let end = bx . append_sibling_block ("va_arg.end") ; let zero = bx . const_i32 (0) ; let offset_align = Align :: from_bytes (4) . unwrap () ; let gr_type = target_ty . is_any_ptr () || target_ty . is_integral () ; let (reg_off , reg_top , slot_size) = if gr_type { let nreg = layout . size . bytes () . div_ceil (8) ; (gr_offs , gr_top , nreg * 8) } else { let nreg = layout . size . bytes () . div_ceil (16) ; (vr_offs , vr_top , nreg * 16) } ; let mut reg_off_v = bx . load (bx . type_i32 () , reg_off , offset_align) ; let use_stack = bx . icmp (IntPredicate :: IntSGE , reg_off_v , zero) ; bx . cond_br (use_stack , on_stack , maybe_reg) ; bx . switch_to_block (maybe_reg) ; if gr_type && layout . align . abi . bytes () > 8 { reg_off_v = bx . add (reg_off_v , bx . const_i32 (15)) ; reg_off_v = bx . and (reg_off_v , bx . const_i32 (- 16)) ; } let new_reg_off_v = bx . add (reg_off_v , bx . const_i32 (slot_size as i32)) ; bx . store (new_reg_off_v , reg_off , offset_align) ; let use_stack = bx . icmp (IntPredicate :: IntSGT , new_reg_off_v , zero) ; bx . cond_br (use_stack , on_stack , in_reg) ; bx . switch_to_block (in_reg) ; let top_type = bx . type_ptr () ; let top = bx . load (top_type , reg_top , dl . pointer_align () . abi) ; let mut reg_addr = bx . ptradd (top , reg_off_v) ; if bx . tcx () . sess . target . endian == Endian :: Big && layout . size . bytes () != slot_size { let offset = bx . const_i32 ((slot_size - layout . size . bytes ()) as i32) ; reg_addr = bx . ptradd (reg_addr , offset) ; } let reg_type = layout . llvm_type (bx) ; let reg_value = bx . load (reg_type , reg_addr , layout . align . abi) ; bx . br (end) ; bx . switch_to_block (on_stack) ; let stack_value = emit_ptr_va_arg (bx , list , target_ty , PassMode :: Direct , SlotSize :: Bytes8 , AllowHigherAlign :: Yes , ForceRightAdjust :: No ,) ; bx . br (end) ; bx . switch_to_block (end) ; let val = bx . phi (layout . immediate_llvm_type (bx) , & [reg_value , stack_value] , & [in_reg , on_stack]) ; val }
}

macro_rules! emit_powerpc_va_arg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_powerpc_va_arg in module {}", module_path!());
    };
}

mkfn!{
    emit_powerpc_va_arg_introspect!();
    fn emit_powerpc_va_arg < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , list : OperandRef < 'tcx , & 'll Value > , target_ty : Ty < 'tcx > ,) -> & 'll Value { let dl = bx . cx . data_layout () ; let va_list_addr = list . immediate () ; let layout = { let mut layout = bx . cx . layout_of (target_ty) ; while let Some ((_ , inner)) = layout . non_1zst_field (bx . cx) { layout = inner ; } layout } ; let target = & bx . cx . tcx . sess . target ; let is_soft_float_abi = target . abi == "softfloat" ; assert ! (! is_soft_float_abi) ; let is_indirect = false ; let (is_i64 , is_int , is_f64) = match layout . layout . backend_repr () { BackendRepr :: Scalar (scalar) => match scalar . primitive () { rustc_abi :: Primitive :: Int (integer , _) => (integer . size () . bits () == 64 , true , false) , rustc_abi :: Primitive :: Float (float) => (false , false , float . size () . bits () == 64) , rustc_abi :: Primitive :: Pointer (_) => (false , true , false) , } , _ => unreachable ! ("all instances of VaArgSafe are represented as scalars") , } ; let num_regs_addr = if is_int || is_soft_float_abi { va_list_addr } else { bx . inbounds_ptradd (va_list_addr , bx . const_usize (1)) } ; let mut num_regs = bx . load (bx . type_i8 () , num_regs_addr , dl . i8_align . abi) ; if is_i64 || (is_f64 && is_soft_float_abi) { num_regs = bx . add (num_regs , bx . const_u8 (1)) ; num_regs = bx . and (num_regs , bx . const_u8 (0b1111_1110)) ; } let max_regs = 8u8 ; let use_regs = bx . icmp (IntPredicate :: IntULT , num_regs , bx . const_u8 (max_regs)) ; let ptr_align_abi = bx . tcx () . data_layout . pointer_align () . abi ; let in_reg = bx . append_sibling_block ("va_arg.in_reg") ; let in_mem = bx . append_sibling_block ("va_arg.in_mem") ; let end = bx . append_sibling_block ("va_arg.end") ; bx . cond_br (use_regs , in_reg , in_mem) ; let reg_addr = { bx . switch_to_block (in_reg) ; let reg_safe_area_ptr = bx . inbounds_ptradd (va_list_addr , bx . cx . const_usize (1 + 1 + 2 + 4)) ; let mut reg_addr = bx . load (bx . type_ptr () , reg_safe_area_ptr , ptr_align_abi) ; if ! is_int && ! is_soft_float_abi { reg_addr = bx . inbounds_ptradd (reg_addr , bx . cx . const_usize (32)) } let reg_size = if is_int || is_soft_float_abi { 4 } else { 8 } ; let reg_offset = bx . mul (num_regs , bx . cx () . const_u8 (reg_size)) ; let reg_addr = bx . inbounds_ptradd (reg_addr , reg_offset) ; let reg_incr = if is_i64 || (is_f64 && is_soft_float_abi) { 2 } else { 1 } ; let new_num_regs = bx . add (num_regs , bx . cx . const_u8 (reg_incr)) ; bx . store (new_num_regs , num_regs_addr , dl . i8_align . abi) ; bx . br (end) ; reg_addr } ; let mem_addr = { bx . switch_to_block (in_mem) ; bx . store (bx . const_u8 (max_regs) , num_regs_addr , dl . i8_align . abi) ; let overflow_area_align = Align :: from_bytes (4) . unwrap () ; let size = if ! is_indirect { layout . layout . size . align_to (overflow_area_align) } else { dl . pointer_size () } ; let overflow_area_ptr = bx . inbounds_ptradd (va_list_addr , bx . cx . const_usize (1 + 1 + 2)) ; let mut overflow_area = bx . load (bx . type_ptr () , overflow_area_ptr , ptr_align_abi) ; if layout . layout . align . abi > overflow_area_align { overflow_area = round_pointer_up_to_alignment (bx , overflow_area , layout . layout . align . abi , bx . type_ptr () ,) ; } let mem_addr = overflow_area ; overflow_area = bx . inbounds_ptradd (overflow_area , bx . const_usize (size . bytes ())) ; bx . store (overflow_area , overflow_area_ptr , ptr_align_abi) ; bx . br (end) ; mem_addr } ; bx . switch_to_block (end) ; let val_addr = bx . phi (bx . type_ptr () , & [reg_addr , mem_addr] , & [in_reg , in_mem]) ; let val_type = layout . llvm_type (bx) ; let val_addr = if is_indirect { bx . load (bx . cx . type_ptr () , val_addr , ptr_align_abi) } else { val_addr } ; bx . load (val_type , val_addr , layout . align . abi) }
}

macro_rules! emit_s390x_va_arg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_s390x_va_arg in module {}", module_path!());
    };
}

mkfn!{
    emit_s390x_va_arg_introspect!();
    fn emit_s390x_va_arg < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , list : OperandRef < 'tcx , & 'll Value > , target_ty : Ty < 'tcx > ,) -> & 'll Value { let dl = bx . cx . data_layout () ; let va_list_addr = list . immediate () ; let i64_offset = 8 ; let ptr_offset = 8 ; let gpr = va_list_addr ; let fpr = bx . inbounds_ptradd (va_list_addr , bx . cx . const_usize (i64_offset)) ; let overflow_arg_area = bx . inbounds_ptradd (va_list_addr , bx . cx . const_usize (2 * i64_offset)) ; let reg_save_area = bx . inbounds_ptradd (va_list_addr , bx . cx . const_usize (2 * i64_offset + ptr_offset)) ; let layout = bx . cx . layout_of (target_ty) ; let in_reg = bx . append_sibling_block ("va_arg.in_reg") ; let in_mem = bx . append_sibling_block ("va_arg.in_mem") ; let end = bx . append_sibling_block ("va_arg.end") ; let ptr_align_abi = dl . pointer_align () . abi ; let target_ty_size = bx . cx . size_of (target_ty) . bytes () ; let indirect : bool = target_ty_size > 8 || ! target_ty_size . is_power_of_two () ; let unpadded_size = if indirect { 8 } else { target_ty_size } ; let padded_size = 8 ; let padding = padded_size - unpadded_size ; let gpr_type = indirect || ! layout . is_single_fp_element (bx . cx) ; let (max_regs , reg_count , reg_save_index , reg_padding) = if gpr_type { (5 , gpr , 2 , padding) } else { (4 , fpr , 16 , 0) } ; let reg_count_v = bx . load (bx . type_i64 () , reg_count , Align :: from_bytes (8) . unwrap ()) ; let use_regs = bx . icmp (IntPredicate :: IntULT , reg_count_v , bx . const_u64 (max_regs)) ; bx . cond_br (use_regs , in_reg , in_mem) ; bx . switch_to_block (in_reg) ; let reg_ptr_v = bx . load (bx . type_ptr () , reg_save_area , ptr_align_abi) ; let scaled_reg_count = bx . mul (reg_count_v , bx . const_u64 (8)) ; let reg_off = bx . add (scaled_reg_count , bx . const_u64 (reg_save_index * 8 + reg_padding)) ; let reg_addr = bx . ptradd (reg_ptr_v , reg_off) ; let new_reg_count_v = bx . add (reg_count_v , bx . const_u64 (1)) ; bx . store (new_reg_count_v , reg_count , Align :: from_bytes (8) . unwrap ()) ; bx . br (end) ; bx . switch_to_block (in_mem) ; let arg_ptr_v = bx . load (bx . type_ptr () , overflow_arg_area , ptr_align_abi) ; let arg_off = bx . const_u64 (padding) ; let mem_addr = bx . ptradd (arg_ptr_v , arg_off) ; let arg_size = bx . cx () . const_u64 (padded_size) ; let new_arg_ptr_v = bx . inbounds_ptradd (arg_ptr_v , arg_size) ; bx . store (new_arg_ptr_v , overflow_arg_area , ptr_align_abi) ; bx . br (end) ; bx . switch_to_block (end) ; let val_addr = bx . phi (bx . type_ptr () , & [reg_addr , mem_addr] , & [in_reg , in_mem]) ; let val_type = layout . llvm_type (bx) ; let val_addr = if indirect { bx . load (bx . cx . type_ptr () , val_addr , ptr_align_abi) } else { val_addr } ; bx . load (val_type , val_addr , layout . align . abi) }
}

macro_rules! emit_x86_64_sysv64_va_arg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_x86_64_sysv64_va_arg in module {}", module_path!());
    };
}

mkfn!{
    emit_x86_64_sysv64_va_arg_introspect!();
    fn emit_x86_64_sysv64_va_arg < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , list : OperandRef < 'tcx , & 'll Value > , target_ty : Ty < 'tcx > ,) -> & 'll Value { let dl = bx . cx . data_layout () ; let va_list_addr = list . immediate () ; let layout = { let mut layout = bx . cx . layout_of (target_ty) ; while let Some ((_ , inner)) = layout . non_1zst_field (bx . cx) { layout = inner ; } layout } ; let mut num_gp_registers = 0 ; let mut num_fp_registers = 0 ; let mut registers_for_primitive = | p | match p { Primitive :: Int (integer , _is_signed) => { num_gp_registers += integer . size () . bytes () . div_ceil (8) as u32 ; } Primitive :: Float (float) => { num_fp_registers += float . size () . bytes () . div_ceil (16) as u32 ; } Primitive :: Pointer (_) => { num_gp_registers += 1 ; } } ; match layout . layout . backend_repr () { BackendRepr :: Scalar (scalar) => { registers_for_primitive (scalar . primitive ()) ; } BackendRepr :: ScalarPair (scalar1 , scalar2) => { registers_for_primitive (scalar1 . primitive ()) ; registers_for_primitive (scalar2 . primitive ()) ; } BackendRepr :: SimdVector { .. } => { unreachable ! ("No x86-64 SysV va_arg implementation for {:?}" , layout . layout . backend_repr ()) } BackendRepr :: Memory { .. } => { let mem_addr = x86_64_sysv64_va_arg_from_memory (bx , va_list_addr , layout) ; return bx . load (layout . llvm_type (bx) , mem_addr , layout . align . abi) ; } } ; let unsigned_int_offset = 4 ; let ptr_offset = 8 ; let gp_offset_ptr = va_list_addr ; let fp_offset_ptr = bx . inbounds_ptradd (va_list_addr , bx . cx . const_usize (unsigned_int_offset)) ; let gp_offset_v = bx . load (bx . type_i32 () , gp_offset_ptr , Align :: from_bytes (8) . unwrap ()) ; let fp_offset_v = bx . load (bx . type_i32 () , fp_offset_ptr , Align :: from_bytes (4) . unwrap ()) ; let mut use_regs = bx . const_bool (false) ; if num_gp_registers > 0 { let max_offset_val = 48u32 - num_gp_registers * 8 ; let fits_in_gp = bx . icmp (IntPredicate :: IntULE , gp_offset_v , bx . const_u32 (max_offset_val)) ; use_regs = fits_in_gp ; } if num_fp_registers > 0 { let max_offset_val = 176u32 - num_fp_registers * 16 ; let fits_in_fp = bx . icmp (IntPredicate :: IntULE , fp_offset_v , bx . const_u32 (max_offset_val)) ; use_regs = if num_gp_registers > 0 { bx . and (use_regs , fits_in_fp) } else { fits_in_fp } ; } let in_reg = bx . append_sibling_block ("va_arg.in_reg") ; let in_mem = bx . append_sibling_block ("va_arg.in_mem") ; let end = bx . append_sibling_block ("va_arg.end") ; bx . cond_br (use_regs , in_reg , in_mem) ; bx . switch_to_block (in_reg) ; let reg_save_area_ptr = bx . inbounds_ptradd (va_list_addr , bx . cx . const_usize (2 * unsigned_int_offset + ptr_offset)) ; let reg_save_area_v = bx . load (bx . type_ptr () , reg_save_area_ptr , dl . pointer_align () . abi) ; let reg_addr = match layout . layout . backend_repr () { BackendRepr :: Scalar (scalar) => match scalar . primitive () { Primitive :: Int (_ , _) | Primitive :: Pointer (_) => { let reg_addr = bx . inbounds_ptradd (reg_save_area_v , gp_offset_v) ; let gp_align = Align :: from_bytes (8) . unwrap () ; copy_to_temporary_if_more_aligned (bx , reg_addr , layout , gp_align) } Primitive :: Float (_) => bx . inbounds_ptradd (reg_save_area_v , fp_offset_v) , } , BackendRepr :: ScalarPair (scalar1 , scalar2) => { let ty_lo = bx . cx () . scalar_pair_element_backend_type (layout , 0 , false) ; let ty_hi = bx . cx () . scalar_pair_element_backend_type (layout , 1 , false) ; let align_lo = layout . field (bx . cx , 0) . layout . align () . abi ; let align_hi = layout . field (bx . cx , 1) . layout . align () . abi ; match (scalar1 . primitive () , scalar2 . primitive ()) { (Primitive :: Float (_) , Primitive :: Float (_)) => { let reg_lo_addr = bx . inbounds_ptradd (reg_save_area_v , fp_offset_v) ; let reg_hi_addr = bx . inbounds_ptradd (reg_lo_addr , bx . const_i32 (16)) ; let align = layout . layout . align () . abi ; let tmp = bx . alloca (layout . layout . size () , align) ; let reg_lo = bx . load (ty_lo , reg_lo_addr , align_lo) ; let reg_hi = bx . load (ty_hi , reg_hi_addr , align_hi) ; let offset = scalar1 . size (bx . cx) . align_to (align_hi) . bytes () ; let field0 = tmp ; let field1 = bx . inbounds_ptradd (tmp , bx . const_u32 (offset as u32)) ; bx . store (reg_lo , field0 , align) ; bx . store (reg_hi , field1 , align) ; tmp } (Primitive :: Float (_) , _) | (_ , Primitive :: Float (_)) => { let gp_addr = bx . inbounds_ptradd (reg_save_area_v , gp_offset_v) ; let fp_addr = bx . inbounds_ptradd (reg_save_area_v , fp_offset_v) ; let (reg_lo_addr , reg_hi_addr) = match scalar1 . primitive () { Primitive :: Float (_) => (fp_addr , gp_addr) , Primitive :: Int (_ , _) | Primitive :: Pointer (_) => (gp_addr , fp_addr) , } ; let tmp = bx . alloca (layout . layout . size () , layout . layout . align () . abi) ; let reg_lo = bx . load (ty_lo , reg_lo_addr , align_lo) ; let reg_hi = bx . load (ty_hi , reg_hi_addr , align_hi) ; let offset = scalar1 . size (bx . cx) . align_to (align_hi) . bytes () ; let field0 = tmp ; let field1 = bx . inbounds_ptradd (tmp , bx . const_u32 (offset as u32)) ; bx . store (reg_lo , field0 , align_lo) ; bx . store (reg_hi , field1 , align_hi) ; tmp } (_ , _) => { let reg_addr = bx . inbounds_ptradd (reg_save_area_v , gp_offset_v) ; let gp_align = Align :: from_bytes (8) . unwrap () ; copy_to_temporary_if_more_aligned (bx , reg_addr , layout , gp_align) } } } BackendRepr :: SimdVector { .. } | BackendRepr :: Memory { .. } => unreachable ! () , } ; if num_gp_registers > 0 { let offset = bx . const_u32 (num_gp_registers * 8) ; let sum = bx . add (gp_offset_v , offset) ; bx . store (sum , gp_offset_ptr , Align :: from_bytes (8) . unwrap ()) ; } if num_fp_registers > 0 { let offset = bx . const_u32 (num_fp_registers * 16) ; let sum = bx . add (fp_offset_v , offset) ; bx . store (sum , fp_offset_ptr , Align :: from_bytes (4) . unwrap ()) ; } bx . br (end) ; bx . switch_to_block (in_mem) ; let mem_addr = x86_64_sysv64_va_arg_from_memory (bx , va_list_addr , layout) ; bx . br (end) ; bx . switch_to_block (end) ; let val_type = layout . llvm_type (bx) ; let val_addr = bx . phi (bx . type_ptr () , & [reg_addr , mem_addr] , & [in_reg , in_mem]) ; bx . load (val_type , val_addr , layout . align . abi) }
}

macro_rules! copy_to_temporary_if_more_aligned_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copy_to_temporary_if_more_aligned in module {}", module_path!());
    };
}

mkfn!{
    copy_to_temporary_if_more_aligned_introspect!();
    #[doc = " Copy into a temporary if the type is more aligned than the register save area."] fn copy_to_temporary_if_more_aligned < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , reg_addr : & 'll Value , layout : TyAndLayout < 'tcx , Ty < 'tcx > > , src_align : Align ,) -> & 'll Value { if layout . layout . align . abi > src_align { let tmp = bx . alloca (layout . layout . size () , layout . layout . align () . abi) ; bx . memcpy (tmp , layout . layout . align . abi , reg_addr , src_align , bx . const_u32 (layout . layout . size () . bytes () as u32) , MemFlags :: empty () ,) ; tmp } else { reg_addr } }
}

macro_rules! x86_64_sysv64_va_arg_from_memory_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function x86_64_sysv64_va_arg_from_memory in module {}", module_path!());
    };
}

mkfn!{
    x86_64_sysv64_va_arg_from_memory_introspect!();
    fn x86_64_sysv64_va_arg_from_memory < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , va_list_addr : & 'll Value , layout : TyAndLayout < 'tcx , Ty < 'tcx > > ,) -> & 'll Value { let dl = bx . cx . data_layout () ; let ptr_align_abi = dl . data_layout () . pointer_align () . abi ; let overflow_arg_area_ptr = bx . inbounds_ptradd (va_list_addr , bx . const_usize (8)) ; let overflow_arg_area_v = bx . load (bx . type_ptr () , overflow_arg_area_ptr , ptr_align_abi) ; if layout . layout . align . abi . bytes () > 8 { unreachable ! ("all instances of VaArgSafe have an alignment <= 8") ; } let mem_addr = overflow_arg_area_v ; let size_in_bytes = layout . layout . size () . bytes () ; let offset = bx . const_i32 (size_in_bytes . next_multiple_of (8) as i32) ; let overflow_arg_area = bx . inbounds_ptradd (overflow_arg_area_v , offset) ; bx . store (overflow_arg_area , overflow_arg_area_ptr , ptr_align_abi) ; mem_addr }
}

macro_rules! emit_xtensa_va_arg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_xtensa_va_arg in module {}", module_path!());
    };
}

mkfn!{
    emit_xtensa_va_arg_introspect!();
    fn emit_xtensa_va_arg < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , list : OperandRef < 'tcx , & 'll Value > , target_ty : Ty < 'tcx > ,) -> & 'll Value { let va_list_addr = list . immediate () ; let layout = bx . cx . layout_of (target_ty) ; let from_stack = bx . append_sibling_block ("va_arg.from_stack") ; let from_regsave = bx . append_sibling_block ("va_arg.from_regsave") ; let end = bx . append_sibling_block ("va_arg.end") ; let ptr_align_abi = bx . tcx () . data_layout . pointer_align () . abi ; let va_reg_offset = 4 ; let va_ndx_offset = va_reg_offset + 4 ; let offset_ptr = bx . inbounds_ptradd (va_list_addr , bx . cx . const_usize (va_ndx_offset)) ; let offset = bx . load (bx . type_i32 () , offset_ptr , bx . tcx () . data_layout . i32_align . abi) ; let offset = round_up_to_alignment (bx , offset , layout . align . abi) ; let slot_size = layout . size . align_to (Align :: from_bytes (4) . unwrap ()) . bytes () as i32 ; let offset_next = bx . add (offset , bx . const_i32 (slot_size)) ; let regsave_size = bx . const_i32 (24) ; let use_regsave = bx . icmp (IntPredicate :: IntULE , offset_next , regsave_size) ; bx . cond_br (use_regsave , from_regsave , from_stack) ; bx . switch_to_block (from_regsave) ; bx . store (offset_next , offset_ptr , ptr_align_abi) ; let regsave_area_ptr = bx . inbounds_ptradd (va_list_addr , bx . cx . const_usize (va_reg_offset)) ; let regsave_area = bx . load (bx . type_ptr () , regsave_area_ptr , ptr_align_abi) ; let regsave_value_ptr = bx . inbounds_ptradd (regsave_area , offset) ; bx . br (end) ; bx . switch_to_block (from_stack) ; let stack_offset_start = bx . const_i32 (32) ; let needs_correction = bx . icmp (IntPredicate :: IntULE , offset , stack_offset_start) ; let offset_corrected = bx . select (needs_correction , stack_offset_start , offset) ; let offset_next_corrected = bx . add (offset_next , bx . const_i32 (slot_size)) ; bx . store (offset_next_corrected , offset_ptr , ptr_align_abi) ; let stack_area_ptr = bx . inbounds_ptradd (va_list_addr , bx . cx . const_usize (0)) ; let stack_area = bx . load (bx . type_ptr () , stack_area_ptr , ptr_align_abi) ; let stack_value_ptr = bx . inbounds_ptradd (stack_area , offset_corrected) ; bx . br (end) ; bx . switch_to_block (end) ; assert ! (bx . tcx () . sess . target . endian == Endian :: Little) ; let value_ptr = bx . phi (bx . type_ptr () , & [regsave_value_ptr , stack_value_ptr] , & [from_regsave , from_stack]) ; return bx . load (layout . llvm_type (bx) , value_ptr , layout . align . abi) ; }
}

macro_rules! emit_va_arg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_va_arg in module {}", module_path!());
    };
}

mkfn!{
    emit_va_arg_introspect!();
    pub (super) fn emit_va_arg < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , addr : OperandRef < 'tcx , & 'll Value > , target_ty : Ty < 'tcx > ,) -> & 'll Value { let target = & bx . cx . tcx . sess . target ; match & * target . arch { "x86" => emit_ptr_va_arg (bx , addr , target_ty , PassMode :: Direct , SlotSize :: Bytes4 , if target . is_like_windows { AllowHigherAlign :: No } else { AllowHigherAlign :: Yes } , ForceRightAdjust :: No ,) , "aarch64" | "arm64ec" if target . is_like_windows || target . is_like_darwin => { emit_ptr_va_arg (bx , addr , target_ty , PassMode :: Direct , SlotSize :: Bytes8 , if target . is_like_windows { AllowHigherAlign :: No } else { AllowHigherAlign :: Yes } , ForceRightAdjust :: No ,) } "aarch64" => emit_aapcs_va_arg (bx , addr , target_ty) , "arm" => { assert ! (bx . cx . size_of (target_ty) . bytes () <= 16) ; emit_ptr_va_arg (bx , addr , target_ty , PassMode :: Direct , SlotSize :: Bytes4 , AllowHigherAlign :: Yes , ForceRightAdjust :: No ,) } "s390x" => emit_s390x_va_arg (bx , addr , target_ty) , "powerpc" => emit_powerpc_va_arg (bx , addr , target_ty) , "powerpc64" | "powerpc64le" => emit_ptr_va_arg (bx , addr , target_ty , PassMode :: Direct , SlotSize :: Bytes8 , AllowHigherAlign :: Yes , match & * target . arch { "powerpc64" => ForceRightAdjust :: Yes , _ => ForceRightAdjust :: No , } ,) , "x86_64" if target . is_like_windows => { let target_ty_size = bx . cx . size_of (target_ty) . bytes () ; emit_ptr_va_arg (bx , addr , target_ty , if target_ty_size > 8 || ! target_ty_size . is_power_of_two () { PassMode :: Indirect } else { PassMode :: Direct } , SlotSize :: Bytes8 , AllowHigherAlign :: No , ForceRightAdjust :: No ,) } "x86_64" => emit_x86_64_sysv64_va_arg (bx , addr , target_ty) , "xtensa" => emit_xtensa_va_arg (bx , addr , target_ty) , _ => bx . va_arg (addr . immediate () , bx . cx . layout_of (target_ty) . llvm_type (bx . cx)) , } }
}