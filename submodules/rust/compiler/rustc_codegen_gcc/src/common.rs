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
mkuse!{use gccjit :: { LValue , RValue , ToRValue , Type } ;}
mkuse!{use rustc_abi :: Primitive :: Pointer ;}
mkuse!{use rustc_abi :: { self as abi , HasDataLayout } ;}
mkuse!{use rustc_codegen_ssa :: traits :: { BaseTypeCodegenMethods , ConstCodegenMethods , MiscCodegenMethods , StaticCodegenMethods , } ;}
mkuse!{use rustc_middle :: mir :: Mutability ;}
mkuse!{use rustc_middle :: mir :: interpret :: { ConstAllocation , GlobalAlloc , Scalar } ;}
mkuse!{use rustc_middle :: ty :: layout :: LayoutOf ;}
mkuse!{use crate :: context :: CodegenCx ;}
mkuse!{use crate :: type_of :: LayoutGccExt ;}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > CodegenCx < 'gcc , 'tcx > { pub fn const_ptrcast (& self , val : RValue < 'gcc > , ty : Type < 'gcc >) -> RValue < 'gcc > { self . context . new_cast (None , val , ty) } pub fn const_bytes (& self , bytes : & [u8]) -> RValue < 'gcc > { bytes_in_context (self , bytes) } fn global_string (& self , string : & str) -> LValue < 'gcc > { let string = self . context . new_string_literal (string) ; let sym = self . generate_local_symbol_name ("str") ; let global = self . declare_private_global (& sym , self . val_ty (string)) ; global . global_set_initializer_rvalue (string) ; global } pub fn const_bitcast (& self , value : RValue < 'gcc > , typ : Type < 'gcc >) -> RValue < 'gcc > { if value . get_type () == self . bool_type . make_pointer () && let Some (pointee) = typ . get_pointee () && pointee . dyncast_vector () . is_some () { panic ! () } self . bitcast_if_needed (value , typ) } }}}

macro_rules! bytes_in_context_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function bytes_in_context in module {}", module_path!());
    };
}

mkfn!{
    bytes_in_context_introspect!();
    pub fn bytes_in_context < 'gcc , 'tcx > (cx : & CodegenCx < 'gcc , 'tcx > , bytes : & [u8]) -> RValue < 'gcc > { match bytes . len () % 8 { 0 => { let context = & cx . context ; let byte_type = context . new_type :: < u64 > () ; let typ = context . new_array_type (None , byte_type , bytes . len () as u64 / 8) ; let elements : Vec < _ > = bytes . chunks_exact (8) . map (| arr | { let arr : [u8 ; 8] = arr . try_into () . unwrap () ; context . new_rvalue_from_long (byte_type , match cx . sess () . target . options . endian { rustc_abi :: Endian :: Little => u64 :: from_le_bytes (arr) as i64 , rustc_abi :: Endian :: Big => u64 :: from_be_bytes (arr) as i64 , } ,) }) . collect () ; context . new_array_constructor (None , typ , & elements) } 4 => { let context = & cx . context ; let byte_type = context . new_type :: < u32 > () ; let typ = context . new_array_type (None , byte_type , bytes . len () as u64 / 4) ; let elements : Vec < _ > = bytes . chunks_exact (4) . map (| arr | { let arr : [u8 ; 4] = arr . try_into () . unwrap () ; context . new_rvalue_from_int (byte_type , match cx . sess () . target . options . endian { rustc_abi :: Endian :: Little => u32 :: from_le_bytes (arr) as i32 , rustc_abi :: Endian :: Big => u32 :: from_be_bytes (arr) as i32 , } ,) }) . collect () ; context . new_array_constructor (None , typ , & elements) } _ => { let context = cx . context ; let byte_type = context . new_type :: < u8 > () ; let typ = context . new_array_type (None , byte_type , bytes . len () as u64) ; let elements : Vec < _ > = bytes . iter () . map (| & byte | context . new_rvalue_from_int (byte_type , byte as i32)) . collect () ; context . new_array_constructor (None , typ , & elements) } } }
}

macro_rules! type_is_pointer_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_is_pointer in module {}", module_path!());
    };
}

mkfn!{
    type_is_pointer_introspect!();
    pub fn type_is_pointer (typ : Type < '_ >) -> bool { typ . get_pointee () . is_some () }
}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > ConstCodegenMethods for CodegenCx < 'gcc , 'tcx > { fn const_null (& self , typ : Type < 'gcc >) -> RValue < 'gcc > { if type_is_pointer (typ) { self . context . new_null (typ) } else { self . const_int (typ , 0) } } fn const_undef (& self , typ : Type < 'gcc >) -> RValue < 'gcc > { let local = self . current_func . borrow () . expect ("func") . new_local (None , typ , "undefined") ; local . to_rvalue () } fn const_poison (& self , typ : Type < 'gcc >) -> RValue < 'gcc > { self . const_undef (typ) } fn const_bool (& self , val : bool) -> RValue < 'gcc > { self . const_uint (self . type_i1 () , val as u64) } fn const_i8 (& self , i : i8) -> RValue < 'gcc > { self . const_int (self . type_i8 () , i as i64) } fn const_i16 (& self , i : i16) -> RValue < 'gcc > { self . const_int (self . type_i16 () , i as i64) } fn const_i32 (& self , i : i32) -> RValue < 'gcc > { self . const_int (self . type_i32 () , i as i64) } fn const_int (& self , typ : Type < 'gcc > , int : i64) -> RValue < 'gcc > { self . gcc_int (typ , int) } fn const_u8 (& self , i : u8) -> RValue < 'gcc > { self . const_uint (self . type_u8 () , i as u64) } fn const_u32 (& self , i : u32) -> RValue < 'gcc > { self . const_uint (self . type_u32 () , i as u64) } fn const_u64 (& self , i : u64) -> RValue < 'gcc > { self . const_uint (self . type_u64 () , i) } fn const_u128 (& self , i : u128) -> RValue < 'gcc > { self . const_uint_big (self . type_u128 () , i) } fn const_usize (& self , i : u64) -> RValue < 'gcc > { let bit_size = self . data_layout () . pointer_size () . bits () ; if bit_size < 64 { assert ! (i < (1 << bit_size)) ; } self . const_uint (self . usize_type , i) } fn const_uint (& self , typ : Type < 'gcc > , int : u64) -> RValue < 'gcc > { self . gcc_uint (typ , int) } fn const_uint_big (& self , typ : Type < 'gcc > , num : u128) -> RValue < 'gcc > { self . gcc_uint_big (typ , num) } fn const_real (& self , typ : Type < 'gcc > , val : f64) -> RValue < 'gcc > { self . context . new_rvalue_from_double (typ , val) } fn const_str (& self , s : & str) -> (RValue < 'gcc > , RValue < 'gcc >) { let mut const_str_cache = self . const_str_cache . borrow_mut () ; let str_global = const_str_cache . get (s) . copied () . unwrap_or_else (| | { let g = self . global_string (s) ; const_str_cache . insert (s . to_owned () , g) ; g }) ; let len = s . len () ; let cs = self . const_ptrcast (str_global . get_address (None) , self . type_ptr_to (self . layout_of (self . tcx . types . str_) . gcc_type (self)) ,) ; (cs , self . const_usize (len as u64)) } fn const_struct (& self , values : & [RValue < 'gcc >] , packed : bool) -> RValue < 'gcc > { let fields : Vec < _ > = values . iter () . map (| value | value . get_type ()) . collect () ; let typ = self . type_struct (& fields , packed) ; let struct_type = typ . is_struct () . expect ("struct type") ; self . context . new_struct_constructor (None , struct_type . as_type () , None , values) } fn const_vector (& self , values : & [RValue < 'gcc >]) -> RValue < 'gcc > { let typ = self . type_vector (values [0] . get_type () , values . len () as u64) ; self . context . new_rvalue_from_vector (None , typ , values) } fn const_to_opt_uint (& self , _v : RValue < 'gcc >) -> Option < u64 > { None } fn const_to_opt_u128 (& self , _v : RValue < 'gcc > , _sign_ext : bool) -> Option < u128 > { None } fn scalar_to_backend (& self , cv : Scalar , layout : abi :: Scalar , ty : Type < 'gcc >) -> RValue < 'gcc > { let bitsize = if layout . is_bool () { 1 } else { layout . size (self) . bits () } ; match cv { Scalar :: Int (int) => { let data = int . to_bits (layout . size (self)) ; let value = self . const_uint_big (self . type_ix (bitsize) , data) ; let bytesize = layout . size (self) . bytes () ; if bitsize > 1 && ty . is_integral () && bytesize as u32 == ty . get_size () { self . context . new_cast (None , value , ty) } else { self . const_bitcast (value , ty) } } Scalar :: Ptr (ptr , _size) => { let (prov , offset) = ptr . prov_and_relative_offset () ; let alloc_id = prov . alloc_id () ; let base_addr = match self . tcx . global_alloc (alloc_id) { GlobalAlloc :: Memory (alloc) => { if alloc . inner () . len () == 0 { assert_eq ! (offset . bytes () , 0) ; let val = self . const_usize (alloc . inner () . align . bytes ()) ; return if matches ! (layout . primitive () , Pointer (_)) { self . context . new_cast (None , val , ty) } else { self . const_bitcast (val , ty) } ; } let init = self . const_data_from_alloc (alloc) ; let alloc = alloc . inner () ; let value = match alloc . mutability { Mutability :: Mut => self . static_addr_of_mut (init , alloc . align , None) , _ => self . static_addr_of (init , alloc . align , None) , } ; if ! self . sess () . fewer_names () { } value } GlobalAlloc :: Function { instance , .. } => self . get_fn_addr (instance) , GlobalAlloc :: VTable (ty , dyn_ty) => { let alloc = self . tcx . global_alloc (self . tcx . vtable_allocation ((ty , dyn_ty . principal () . map (| principal | { self . tcx . instantiate_bound_regions_with_erased (principal) }) ,))) . unwrap_memory () ; let init = self . const_data_from_alloc (alloc) ; self . static_addr_of (init , alloc . inner () . align , None) } GlobalAlloc :: TypeId { .. } => { let val = self . const_usize (offset . bytes ()) ; return self . context . new_cast (None , val , ty) ; } GlobalAlloc :: Static (def_id) => { assert ! (self . tcx . is_static (def_id)) ; self . get_static (def_id) . get_address (None) } } ; let ptr_type = base_addr . get_type () ; let base_addr = self . context . new_cast (None , base_addr , self . usize_type) ; let offset = self . context . new_rvalue_from_long (self . usize_type , offset . bytes () as i64) ; let ptr = self . context . new_cast (None , base_addr + offset , ptr_type) ; if ! matches ! (layout . primitive () , Pointer (_)) { self . const_bitcast (ptr . dereference (None) . to_rvalue () , ty) } else { self . context . new_cast (None , ptr , ty) } } } } fn const_data_from_alloc (& self , alloc : ConstAllocation < '_ >) -> Self :: Value { let mut mock_alloc = alloc . inner () . clone () ; mock_alloc . align = rustc_abi :: Align :: MAX ; if let Some (res) = self . const_cache . borrow () . get (& mock_alloc) { return * res ; } let res = crate :: consts :: const_alloc_to_gcc_uncached (self , alloc) ; self . const_cache . borrow_mut () . insert (mock_alloc , res) ; res } fn const_ptr_byte_offset (& self , base_addr : Self :: Value , offset : abi :: Size) -> Self :: Value { self . context . new_array_access (None , base_addr , self . const_usize (offset . bytes ())) . get_address (None) } }}}
mkitem!{mktrait!{pub trait SignType < 'gcc , 'tcx > { fn is_signed (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_unsigned (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn to_signed (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> Type < 'gcc > ; fn to_unsigned (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> Type < 'gcc > ; }}}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > SignType < 'gcc , 'tcx > for Type < 'gcc > { fn is_signed (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . is_i8 (cx) || self . is_i16 (cx) || self . is_i32 (cx) || self . is_i64 (cx) || self . is_i128 (cx) } fn is_unsigned (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . is_u8 (cx) || self . is_u16 (cx) || self . is_u32 (cx) || self . is_u64 (cx) || self . is_u128 (cx) } fn to_signed (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> Type < 'gcc > { if self . is_u8 (cx) { cx . i8_type } else if self . is_u16 (cx) { cx . i16_type } else if self . is_u32 (cx) { cx . i32_type } else if self . is_u64 (cx) { cx . i64_type } else if self . is_u128 (cx) { cx . i128_type } else if self . is_uchar (cx) { cx . char_type } else if self . is_ushort (cx) { cx . short_type } else if self . is_uint (cx) { cx . int_type } else if self . is_ulong (cx) { cx . long_type } else if self . is_ulonglong (cx) { cx . longlong_type } else { * self } } fn to_unsigned (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> Type < 'gcc > { if self . is_i8 (cx) { cx . u8_type } else if self . is_i16 (cx) { cx . u16_type } else if self . is_i32 (cx) { cx . u32_type } else if self . is_i64 (cx) { cx . u64_type } else if self . is_i128 (cx) { cx . u128_type } else if self . is_char (cx) { cx . uchar_type } else if self . is_short (cx) { cx . ushort_type } else if self . is_int (cx) { cx . uint_type } else if self . is_long (cx) { cx . ulong_type } else if self . is_longlong (cx) { cx . ulonglong_type } else { * self } } }}}
mkitem!{mktrait!{pub trait TypeReflection < 'gcc , 'tcx > { fn is_uchar (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_ushort (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_uint (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_ulong (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_ulonglong (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_char (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_short (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_int (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_long (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_longlong (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_i8 (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_u8 (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_i16 (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_u16 (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_i32 (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_u32 (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_i64 (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_u64 (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_i128 (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_u128 (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_vector (& self) -> bool ; }}}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > TypeReflection < 'gcc , 'tcx > for Type < 'gcc > { fn is_uchar (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . unqualified () == cx . uchar_type } fn is_ushort (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . unqualified () == cx . ushort_type } fn is_uint (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . unqualified () == cx . uint_type } fn is_ulong (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . unqualified () == cx . ulong_type } fn is_ulonglong (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . unqualified () == cx . ulonglong_type } fn is_char (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . unqualified () == cx . char_type } fn is_short (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . unqualified () == cx . short_type } fn is_int (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . unqualified () == cx . int_type } fn is_long (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . unqualified () == cx . long_type } fn is_longlong (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . unqualified () == cx . longlong_type } fn is_i8 (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . is_compatible_with (cx . i8_type) } fn is_u8 (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . is_compatible_with (cx . u8_type) } fn is_i16 (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . is_compatible_with (cx . i16_type) } fn is_u16 (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . is_compatible_with (cx . u16_type) } fn is_i32 (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . is_compatible_with (cx . i32_type) } fn is_u32 (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . is_compatible_with (cx . u32_type) } fn is_i64 (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . is_compatible_with (cx . i64_type) } fn is_u64 (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . is_compatible_with (cx . u64_type) } fn is_i128 (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . unqualified () == cx . i128_type . unqualified () } fn is_u128 (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . unqualified () == cx . u128_type . unqualified () } fn is_vector (& self) -> bool { let mut typ = * self ; loop { if typ . dyncast_vector () . is_some () { return true ; } let old_type = typ ; typ = typ . unqualified () ; if old_type == typ { break ; } } false } }}}