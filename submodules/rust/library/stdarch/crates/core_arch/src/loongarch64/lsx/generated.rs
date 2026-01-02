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
mkuse!{use crate :: mem :: transmute ;}
mkuse!{use super :: types :: * ;}
mkitem!{#[allow (improper_ctypes)] unsafe extern "unadjusted" { #[link_name = "llvm.loongarch.lsx.vsll.b"] fn __lsx_vsll_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsll.h"] fn __lsx_vsll_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsll.w"] fn __lsx_vsll_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsll.d"] fn __lsx_vsll_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vslli.b"] fn __lsx_vslli_b (a : __v16i8 , b : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vslli.h"] fn __lsx_vslli_h (a : __v8i16 , b : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vslli.w"] fn __lsx_vslli_w (a : __v4i32 , b : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vslli.d"] fn __lsx_vslli_d (a : __v2i64 , b : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsra.b"] fn __lsx_vsra_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsra.h"] fn __lsx_vsra_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsra.w"] fn __lsx_vsra_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsra.d"] fn __lsx_vsra_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsrai.b"] fn __lsx_vsrai_b (a : __v16i8 , b : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsrai.h"] fn __lsx_vsrai_h (a : __v8i16 , b : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsrai.w"] fn __lsx_vsrai_w (a : __v4i32 , b : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsrai.d"] fn __lsx_vsrai_d (a : __v2i64 , b : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsrar.b"] fn __lsx_vsrar_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsrar.h"] fn __lsx_vsrar_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsrar.w"] fn __lsx_vsrar_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsrar.d"] fn __lsx_vsrar_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsrari.b"] fn __lsx_vsrari_b (a : __v16i8 , b : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsrari.h"] fn __lsx_vsrari_h (a : __v8i16 , b : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsrari.w"] fn __lsx_vsrari_w (a : __v4i32 , b : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsrari.d"] fn __lsx_vsrari_d (a : __v2i64 , b : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsrl.b"] fn __lsx_vsrl_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsrl.h"] fn __lsx_vsrl_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsrl.w"] fn __lsx_vsrl_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsrl.d"] fn __lsx_vsrl_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsrli.b"] fn __lsx_vsrli_b (a : __v16i8 , b : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsrli.h"] fn __lsx_vsrli_h (a : __v8i16 , b : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsrli.w"] fn __lsx_vsrli_w (a : __v4i32 , b : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsrli.d"] fn __lsx_vsrli_d (a : __v2i64 , b : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsrlr.b"] fn __lsx_vsrlr_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsrlr.h"] fn __lsx_vsrlr_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsrlr.w"] fn __lsx_vsrlr_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsrlr.d"] fn __lsx_vsrlr_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsrlri.b"] fn __lsx_vsrlri_b (a : __v16i8 , b : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsrlri.h"] fn __lsx_vsrlri_h (a : __v8i16 , b : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsrlri.w"] fn __lsx_vsrlri_w (a : __v4i32 , b : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsrlri.d"] fn __lsx_vsrlri_d (a : __v2i64 , b : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vbitclr.b"] fn __lsx_vbitclr_b (a : __v16u8 , b : __v16u8) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vbitclr.h"] fn __lsx_vbitclr_h (a : __v8u16 , b : __v8u16) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vbitclr.w"] fn __lsx_vbitclr_w (a : __v4u32 , b : __v4u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vbitclr.d"] fn __lsx_vbitclr_d (a : __v2u64 , b : __v2u64) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vbitclri.b"] fn __lsx_vbitclri_b (a : __v16u8 , b : u32) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vbitclri.h"] fn __lsx_vbitclri_h (a : __v8u16 , b : u32) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vbitclri.w"] fn __lsx_vbitclri_w (a : __v4u32 , b : u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vbitclri.d"] fn __lsx_vbitclri_d (a : __v2u64 , b : u32) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vbitset.b"] fn __lsx_vbitset_b (a : __v16u8 , b : __v16u8) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vbitset.h"] fn __lsx_vbitset_h (a : __v8u16 , b : __v8u16) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vbitset.w"] fn __lsx_vbitset_w (a : __v4u32 , b : __v4u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vbitset.d"] fn __lsx_vbitset_d (a : __v2u64 , b : __v2u64) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vbitseti.b"] fn __lsx_vbitseti_b (a : __v16u8 , b : u32) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vbitseti.h"] fn __lsx_vbitseti_h (a : __v8u16 , b : u32) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vbitseti.w"] fn __lsx_vbitseti_w (a : __v4u32 , b : u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vbitseti.d"] fn __lsx_vbitseti_d (a : __v2u64 , b : u32) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vbitrev.b"] fn __lsx_vbitrev_b (a : __v16u8 , b : __v16u8) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vbitrev.h"] fn __lsx_vbitrev_h (a : __v8u16 , b : __v8u16) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vbitrev.w"] fn __lsx_vbitrev_w (a : __v4u32 , b : __v4u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vbitrev.d"] fn __lsx_vbitrev_d (a : __v2u64 , b : __v2u64) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vbitrevi.b"] fn __lsx_vbitrevi_b (a : __v16u8 , b : u32) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vbitrevi.h"] fn __lsx_vbitrevi_h (a : __v8u16 , b : u32) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vbitrevi.w"] fn __lsx_vbitrevi_w (a : __v4u32 , b : u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vbitrevi.d"] fn __lsx_vbitrevi_d (a : __v2u64 , b : u32) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vadd.b"] fn __lsx_vadd_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vadd.h"] fn __lsx_vadd_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vadd.w"] fn __lsx_vadd_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vadd.d"] fn __lsx_vadd_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vaddi.bu"] fn __lsx_vaddi_bu (a : __v16i8 , b : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vaddi.hu"] fn __lsx_vaddi_hu (a : __v8i16 , b : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vaddi.wu"] fn __lsx_vaddi_wu (a : __v4i32 , b : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vaddi.du"] fn __lsx_vaddi_du (a : __v2i64 , b : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsub.b"] fn __lsx_vsub_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsub.h"] fn __lsx_vsub_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsub.w"] fn __lsx_vsub_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsub.d"] fn __lsx_vsub_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsubi.bu"] fn __lsx_vsubi_bu (a : __v16i8 , b : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsubi.hu"] fn __lsx_vsubi_hu (a : __v8i16 , b : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsubi.wu"] fn __lsx_vsubi_wu (a : __v4i32 , b : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsubi.du"] fn __lsx_vsubi_du (a : __v2i64 , b : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmax.b"] fn __lsx_vmax_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vmax.h"] fn __lsx_vmax_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vmax.w"] fn __lsx_vmax_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vmax.d"] fn __lsx_vmax_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmaxi.b"] fn __lsx_vmaxi_b (a : __v16i8 , b : i32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vmaxi.h"] fn __lsx_vmaxi_h (a : __v8i16 , b : i32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vmaxi.w"] fn __lsx_vmaxi_w (a : __v4i32 , b : i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vmaxi.d"] fn __lsx_vmaxi_d (a : __v2i64 , b : i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmax.bu"] fn __lsx_vmax_bu (a : __v16u8 , b : __v16u8) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vmax.hu"] fn __lsx_vmax_hu (a : __v8u16 , b : __v8u16) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vmax.wu"] fn __lsx_vmax_wu (a : __v4u32 , b : __v4u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vmax.du"] fn __lsx_vmax_du (a : __v2u64 , b : __v2u64) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vmaxi.bu"] fn __lsx_vmaxi_bu (a : __v16u8 , b : u32) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vmaxi.hu"] fn __lsx_vmaxi_hu (a : __v8u16 , b : u32) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vmaxi.wu"] fn __lsx_vmaxi_wu (a : __v4u32 , b : u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vmaxi.du"] fn __lsx_vmaxi_du (a : __v2u64 , b : u32) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vmin.b"] fn __lsx_vmin_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vmin.h"] fn __lsx_vmin_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vmin.w"] fn __lsx_vmin_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vmin.d"] fn __lsx_vmin_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmini.b"] fn __lsx_vmini_b (a : __v16i8 , b : i32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vmini.h"] fn __lsx_vmini_h (a : __v8i16 , b : i32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vmini.w"] fn __lsx_vmini_w (a : __v4i32 , b : i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vmini.d"] fn __lsx_vmini_d (a : __v2i64 , b : i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmin.bu"] fn __lsx_vmin_bu (a : __v16u8 , b : __v16u8) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vmin.hu"] fn __lsx_vmin_hu (a : __v8u16 , b : __v8u16) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vmin.wu"] fn __lsx_vmin_wu (a : __v4u32 , b : __v4u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vmin.du"] fn __lsx_vmin_du (a : __v2u64 , b : __v2u64) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vmini.bu"] fn __lsx_vmini_bu (a : __v16u8 , b : u32) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vmini.hu"] fn __lsx_vmini_hu (a : __v8u16 , b : u32) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vmini.wu"] fn __lsx_vmini_wu (a : __v4u32 , b : u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vmini.du"] fn __lsx_vmini_du (a : __v2u64 , b : u32) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vseq.b"] fn __lsx_vseq_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vseq.h"] fn __lsx_vseq_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vseq.w"] fn __lsx_vseq_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vseq.d"] fn __lsx_vseq_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vseqi.b"] fn __lsx_vseqi_b (a : __v16i8 , b : i32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vseqi.h"] fn __lsx_vseqi_h (a : __v8i16 , b : i32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vseqi.w"] fn __lsx_vseqi_w (a : __v4i32 , b : i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vseqi.d"] fn __lsx_vseqi_d (a : __v2i64 , b : i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vslti.b"] fn __lsx_vslti_b (a : __v16i8 , b : i32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vslt.b"] fn __lsx_vslt_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vslt.h"] fn __lsx_vslt_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vslt.w"] fn __lsx_vslt_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vslt.d"] fn __lsx_vslt_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vslti.h"] fn __lsx_vslti_h (a : __v8i16 , b : i32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vslti.w"] fn __lsx_vslti_w (a : __v4i32 , b : i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vslti.d"] fn __lsx_vslti_d (a : __v2i64 , b : i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vslt.bu"] fn __lsx_vslt_bu (a : __v16u8 , b : __v16u8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vslt.hu"] fn __lsx_vslt_hu (a : __v8u16 , b : __v8u16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vslt.wu"] fn __lsx_vslt_wu (a : __v4u32 , b : __v4u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vslt.du"] fn __lsx_vslt_du (a : __v2u64 , b : __v2u64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vslti.bu"] fn __lsx_vslti_bu (a : __v16u8 , b : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vslti.hu"] fn __lsx_vslti_hu (a : __v8u16 , b : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vslti.wu"] fn __lsx_vslti_wu (a : __v4u32 , b : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vslti.du"] fn __lsx_vslti_du (a : __v2u64 , b : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsle.b"] fn __lsx_vsle_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsle.h"] fn __lsx_vsle_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsle.w"] fn __lsx_vsle_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsle.d"] fn __lsx_vsle_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vslei.b"] fn __lsx_vslei_b (a : __v16i8 , b : i32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vslei.h"] fn __lsx_vslei_h (a : __v8i16 , b : i32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vslei.w"] fn __lsx_vslei_w (a : __v4i32 , b : i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vslei.d"] fn __lsx_vslei_d (a : __v2i64 , b : i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsle.bu"] fn __lsx_vsle_bu (a : __v16u8 , b : __v16u8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsle.hu"] fn __lsx_vsle_hu (a : __v8u16 , b : __v8u16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsle.wu"] fn __lsx_vsle_wu (a : __v4u32 , b : __v4u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsle.du"] fn __lsx_vsle_du (a : __v2u64 , b : __v2u64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vslei.bu"] fn __lsx_vslei_bu (a : __v16u8 , b : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vslei.hu"] fn __lsx_vslei_hu (a : __v8u16 , b : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vslei.wu"] fn __lsx_vslei_wu (a : __v4u32 , b : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vslei.du"] fn __lsx_vslei_du (a : __v2u64 , b : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsat.b"] fn __lsx_vsat_b (a : __v16i8 , b : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsat.h"] fn __lsx_vsat_h (a : __v8i16 , b : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsat.w"] fn __lsx_vsat_w (a : __v4i32 , b : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsat.d"] fn __lsx_vsat_d (a : __v2i64 , b : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsat.bu"] fn __lsx_vsat_bu (a : __v16u8 , b : u32) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vsat.hu"] fn __lsx_vsat_hu (a : __v8u16 , b : u32) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vsat.wu"] fn __lsx_vsat_wu (a : __v4u32 , b : u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vsat.du"] fn __lsx_vsat_du (a : __v2u64 , b : u32) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vadda.b"] fn __lsx_vadda_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vadda.h"] fn __lsx_vadda_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vadda.w"] fn __lsx_vadda_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vadda.d"] fn __lsx_vadda_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsadd.b"] fn __lsx_vsadd_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsadd.h"] fn __lsx_vsadd_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsadd.w"] fn __lsx_vsadd_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsadd.d"] fn __lsx_vsadd_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsadd.bu"] fn __lsx_vsadd_bu (a : __v16u8 , b : __v16u8) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vsadd.hu"] fn __lsx_vsadd_hu (a : __v8u16 , b : __v8u16) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vsadd.wu"] fn __lsx_vsadd_wu (a : __v4u32 , b : __v4u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vsadd.du"] fn __lsx_vsadd_du (a : __v2u64 , b : __v2u64) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vavg.b"] fn __lsx_vavg_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vavg.h"] fn __lsx_vavg_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vavg.w"] fn __lsx_vavg_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vavg.d"] fn __lsx_vavg_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vavg.bu"] fn __lsx_vavg_bu (a : __v16u8 , b : __v16u8) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vavg.hu"] fn __lsx_vavg_hu (a : __v8u16 , b : __v8u16) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vavg.wu"] fn __lsx_vavg_wu (a : __v4u32 , b : __v4u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vavg.du"] fn __lsx_vavg_du (a : __v2u64 , b : __v2u64) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vavgr.b"] fn __lsx_vavgr_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vavgr.h"] fn __lsx_vavgr_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vavgr.w"] fn __lsx_vavgr_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vavgr.d"] fn __lsx_vavgr_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vavgr.bu"] fn __lsx_vavgr_bu (a : __v16u8 , b : __v16u8) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vavgr.hu"] fn __lsx_vavgr_hu (a : __v8u16 , b : __v8u16) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vavgr.wu"] fn __lsx_vavgr_wu (a : __v4u32 , b : __v4u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vavgr.du"] fn __lsx_vavgr_du (a : __v2u64 , b : __v2u64) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vssub.b"] fn __lsx_vssub_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vssub.h"] fn __lsx_vssub_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vssub.w"] fn __lsx_vssub_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vssub.d"] fn __lsx_vssub_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vssub.bu"] fn __lsx_vssub_bu (a : __v16u8 , b : __v16u8) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vssub.hu"] fn __lsx_vssub_hu (a : __v8u16 , b : __v8u16) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vssub.wu"] fn __lsx_vssub_wu (a : __v4u32 , b : __v4u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vssub.du"] fn __lsx_vssub_du (a : __v2u64 , b : __v2u64) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vabsd.b"] fn __lsx_vabsd_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vabsd.h"] fn __lsx_vabsd_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vabsd.w"] fn __lsx_vabsd_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vabsd.d"] fn __lsx_vabsd_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vabsd.bu"] fn __lsx_vabsd_bu (a : __v16u8 , b : __v16u8) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vabsd.hu"] fn __lsx_vabsd_hu (a : __v8u16 , b : __v8u16) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vabsd.wu"] fn __lsx_vabsd_wu (a : __v4u32 , b : __v4u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vabsd.du"] fn __lsx_vabsd_du (a : __v2u64 , b : __v2u64) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vmul.b"] fn __lsx_vmul_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vmul.h"] fn __lsx_vmul_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vmul.w"] fn __lsx_vmul_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vmul.d"] fn __lsx_vmul_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmadd.b"] fn __lsx_vmadd_b (a : __v16i8 , b : __v16i8 , c : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vmadd.h"] fn __lsx_vmadd_h (a : __v8i16 , b : __v8i16 , c : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vmadd.w"] fn __lsx_vmadd_w (a : __v4i32 , b : __v4i32 , c : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vmadd.d"] fn __lsx_vmadd_d (a : __v2i64 , b : __v2i64 , c : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmsub.b"] fn __lsx_vmsub_b (a : __v16i8 , b : __v16i8 , c : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vmsub.h"] fn __lsx_vmsub_h (a : __v8i16 , b : __v8i16 , c : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vmsub.w"] fn __lsx_vmsub_w (a : __v4i32 , b : __v4i32 , c : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vmsub.d"] fn __lsx_vmsub_d (a : __v2i64 , b : __v2i64 , c : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vdiv.b"] fn __lsx_vdiv_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vdiv.h"] fn __lsx_vdiv_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vdiv.w"] fn __lsx_vdiv_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vdiv.d"] fn __lsx_vdiv_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vdiv.bu"] fn __lsx_vdiv_bu (a : __v16u8 , b : __v16u8) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vdiv.hu"] fn __lsx_vdiv_hu (a : __v8u16 , b : __v8u16) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vdiv.wu"] fn __lsx_vdiv_wu (a : __v4u32 , b : __v4u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vdiv.du"] fn __lsx_vdiv_du (a : __v2u64 , b : __v2u64) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vhaddw.h.b"] fn __lsx_vhaddw_h_b (a : __v16i8 , b : __v16i8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vhaddw.w.h"] fn __lsx_vhaddw_w_h (a : __v8i16 , b : __v8i16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vhaddw.d.w"] fn __lsx_vhaddw_d_w (a : __v4i32 , b : __v4i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vhaddw.hu.bu"] fn __lsx_vhaddw_hu_bu (a : __v16u8 , b : __v16u8) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vhaddw.wu.hu"] fn __lsx_vhaddw_wu_hu (a : __v8u16 , b : __v8u16) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vhaddw.du.wu"] fn __lsx_vhaddw_du_wu (a : __v4u32 , b : __v4u32) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vhsubw.h.b"] fn __lsx_vhsubw_h_b (a : __v16i8 , b : __v16i8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vhsubw.w.h"] fn __lsx_vhsubw_w_h (a : __v8i16 , b : __v8i16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vhsubw.d.w"] fn __lsx_vhsubw_d_w (a : __v4i32 , b : __v4i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vhsubw.hu.bu"] fn __lsx_vhsubw_hu_bu (a : __v16u8 , b : __v16u8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vhsubw.wu.hu"] fn __lsx_vhsubw_wu_hu (a : __v8u16 , b : __v8u16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vhsubw.du.wu"] fn __lsx_vhsubw_du_wu (a : __v4u32 , b : __v4u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmod.b"] fn __lsx_vmod_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vmod.h"] fn __lsx_vmod_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vmod.w"] fn __lsx_vmod_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vmod.d"] fn __lsx_vmod_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmod.bu"] fn __lsx_vmod_bu (a : __v16u8 , b : __v16u8) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vmod.hu"] fn __lsx_vmod_hu (a : __v8u16 , b : __v8u16) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vmod.wu"] fn __lsx_vmod_wu (a : __v4u32 , b : __v4u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vmod.du"] fn __lsx_vmod_du (a : __v2u64 , b : __v2u64) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vreplve.b"] fn __lsx_vreplve_b (a : __v16i8 , b : i32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vreplve.h"] fn __lsx_vreplve_h (a : __v8i16 , b : i32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vreplve.w"] fn __lsx_vreplve_w (a : __v4i32 , b : i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vreplve.d"] fn __lsx_vreplve_d (a : __v2i64 , b : i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vreplvei.b"] fn __lsx_vreplvei_b (a : __v16i8 , b : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vreplvei.h"] fn __lsx_vreplvei_h (a : __v8i16 , b : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vreplvei.w"] fn __lsx_vreplvei_w (a : __v4i32 , b : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vreplvei.d"] fn __lsx_vreplvei_d (a : __v2i64 , b : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vpickev.b"] fn __lsx_vpickev_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vpickev.h"] fn __lsx_vpickev_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vpickev.w"] fn __lsx_vpickev_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vpickev.d"] fn __lsx_vpickev_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vpickod.b"] fn __lsx_vpickod_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vpickod.h"] fn __lsx_vpickod_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vpickod.w"] fn __lsx_vpickod_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vpickod.d"] fn __lsx_vpickod_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vilvh.b"] fn __lsx_vilvh_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vilvh.h"] fn __lsx_vilvh_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vilvh.w"] fn __lsx_vilvh_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vilvh.d"] fn __lsx_vilvh_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vilvl.b"] fn __lsx_vilvl_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vilvl.h"] fn __lsx_vilvl_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vilvl.w"] fn __lsx_vilvl_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vilvl.d"] fn __lsx_vilvl_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vpackev.b"] fn __lsx_vpackev_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vpackev.h"] fn __lsx_vpackev_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vpackev.w"] fn __lsx_vpackev_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vpackev.d"] fn __lsx_vpackev_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vpackod.b"] fn __lsx_vpackod_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vpackod.h"] fn __lsx_vpackod_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vpackod.w"] fn __lsx_vpackod_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vpackod.d"] fn __lsx_vpackod_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vshuf.h"] fn __lsx_vshuf_h (a : __v8i16 , b : __v8i16 , c : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vshuf.w"] fn __lsx_vshuf_w (a : __v4i32 , b : __v4i32 , c : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vshuf.d"] fn __lsx_vshuf_d (a : __v2i64 , b : __v2i64 , c : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vand.v"] fn __lsx_vand_v (a : __v16u8 , b : __v16u8) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vandi.b"] fn __lsx_vandi_b (a : __v16u8 , b : u32) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vor.v"] fn __lsx_vor_v (a : __v16u8 , b : __v16u8) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vori.b"] fn __lsx_vori_b (a : __v16u8 , b : u32) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vnor.v"] fn __lsx_vnor_v (a : __v16u8 , b : __v16u8) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vnori.b"] fn __lsx_vnori_b (a : __v16u8 , b : u32) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vxor.v"] fn __lsx_vxor_v (a : __v16u8 , b : __v16u8) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vxori.b"] fn __lsx_vxori_b (a : __v16u8 , b : u32) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vbitsel.v"] fn __lsx_vbitsel_v (a : __v16u8 , b : __v16u8 , c : __v16u8) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vbitseli.b"] fn __lsx_vbitseli_b (a : __v16u8 , b : __v16u8 , c : u32) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vshuf4i.b"] fn __lsx_vshuf4i_b (a : __v16i8 , b : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vshuf4i.h"] fn __lsx_vshuf4i_h (a : __v8i16 , b : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vshuf4i.w"] fn __lsx_vshuf4i_w (a : __v4i32 , b : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vreplgr2vr.b"] fn __lsx_vreplgr2vr_b (a : i32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vreplgr2vr.h"] fn __lsx_vreplgr2vr_h (a : i32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vreplgr2vr.w"] fn __lsx_vreplgr2vr_w (a : i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vreplgr2vr.d"] fn __lsx_vreplgr2vr_d (a : i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vpcnt.b"] fn __lsx_vpcnt_b (a : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vpcnt.h"] fn __lsx_vpcnt_h (a : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vpcnt.w"] fn __lsx_vpcnt_w (a : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vpcnt.d"] fn __lsx_vpcnt_d (a : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vclo.b"] fn __lsx_vclo_b (a : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vclo.h"] fn __lsx_vclo_h (a : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vclo.w"] fn __lsx_vclo_w (a : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vclo.d"] fn __lsx_vclo_d (a : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vclz.b"] fn __lsx_vclz_b (a : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vclz.h"] fn __lsx_vclz_h (a : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vclz.w"] fn __lsx_vclz_w (a : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vclz.d"] fn __lsx_vclz_d (a : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vpickve2gr.b"] fn __lsx_vpickve2gr_b (a : __v16i8 , b : u32) -> i32 ; #[link_name = "llvm.loongarch.lsx.vpickve2gr.h"] fn __lsx_vpickve2gr_h (a : __v8i16 , b : u32) -> i32 ; #[link_name = "llvm.loongarch.lsx.vpickve2gr.w"] fn __lsx_vpickve2gr_w (a : __v4i32 , b : u32) -> i32 ; #[link_name = "llvm.loongarch.lsx.vpickve2gr.d"] fn __lsx_vpickve2gr_d (a : __v2i64 , b : u32) -> i64 ; #[link_name = "llvm.loongarch.lsx.vpickve2gr.bu"] fn __lsx_vpickve2gr_bu (a : __v16i8 , b : u32) -> u32 ; #[link_name = "llvm.loongarch.lsx.vpickve2gr.hu"] fn __lsx_vpickve2gr_hu (a : __v8i16 , b : u32) -> u32 ; #[link_name = "llvm.loongarch.lsx.vpickve2gr.wu"] fn __lsx_vpickve2gr_wu (a : __v4i32 , b : u32) -> u32 ; #[link_name = "llvm.loongarch.lsx.vpickve2gr.du"] fn __lsx_vpickve2gr_du (a : __v2i64 , b : u32) -> u64 ; #[link_name = "llvm.loongarch.lsx.vinsgr2vr.b"] fn __lsx_vinsgr2vr_b (a : __v16i8 , b : i32 , c : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vinsgr2vr.h"] fn __lsx_vinsgr2vr_h (a : __v8i16 , b : i32 , c : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vinsgr2vr.w"] fn __lsx_vinsgr2vr_w (a : __v4i32 , b : i32 , c : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vinsgr2vr.d"] fn __lsx_vinsgr2vr_d (a : __v2i64 , b : i64 , c : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfadd.s"] fn __lsx_vfadd_s (a : __v4f32 , b : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfadd.d"] fn __lsx_vfadd_d (a : __v2f64 , b : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vfsub.s"] fn __lsx_vfsub_s (a : __v4f32 , b : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfsub.d"] fn __lsx_vfsub_d (a : __v2f64 , b : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vfmul.s"] fn __lsx_vfmul_s (a : __v4f32 , b : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfmul.d"] fn __lsx_vfmul_d (a : __v2f64 , b : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vfdiv.s"] fn __lsx_vfdiv_s (a : __v4f32 , b : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfdiv.d"] fn __lsx_vfdiv_d (a : __v2f64 , b : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vfcvt.h.s"] fn __lsx_vfcvt_h_s (a : __v4f32 , b : __v4f32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vfcvt.s.d"] fn __lsx_vfcvt_s_d (a : __v2f64 , b : __v2f64) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfmin.s"] fn __lsx_vfmin_s (a : __v4f32 , b : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfmin.d"] fn __lsx_vfmin_d (a : __v2f64 , b : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vfmina.s"] fn __lsx_vfmina_s (a : __v4f32 , b : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfmina.d"] fn __lsx_vfmina_d (a : __v2f64 , b : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vfmax.s"] fn __lsx_vfmax_s (a : __v4f32 , b : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfmax.d"] fn __lsx_vfmax_d (a : __v2f64 , b : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vfmaxa.s"] fn __lsx_vfmaxa_s (a : __v4f32 , b : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfmaxa.d"] fn __lsx_vfmaxa_d (a : __v2f64 , b : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vfclass.s"] fn __lsx_vfclass_s (a : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfclass.d"] fn __lsx_vfclass_d (a : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfsqrt.s"] fn __lsx_vfsqrt_s (a : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfsqrt.d"] fn __lsx_vfsqrt_d (a : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vfrecip.s"] fn __lsx_vfrecip_s (a : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfrecip.d"] fn __lsx_vfrecip_d (a : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vfrecipe.s"] fn __lsx_vfrecipe_s (a : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfrecipe.d"] fn __lsx_vfrecipe_d (a : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vfrsqrte.s"] fn __lsx_vfrsqrte_s (a : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfrsqrte.d"] fn __lsx_vfrsqrte_d (a : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vfrint.s"] fn __lsx_vfrint_s (a : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfrint.d"] fn __lsx_vfrint_d (a : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vfrsqrt.s"] fn __lsx_vfrsqrt_s (a : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfrsqrt.d"] fn __lsx_vfrsqrt_d (a : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vflogb.s"] fn __lsx_vflogb_s (a : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vflogb.d"] fn __lsx_vflogb_d (a : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vfcvth.s.h"] fn __lsx_vfcvth_s_h (a : __v8i16) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfcvth.d.s"] fn __lsx_vfcvth_d_s (a : __v4f32) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vfcvtl.s.h"] fn __lsx_vfcvtl_s_h (a : __v8i16) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfcvtl.d.s"] fn __lsx_vfcvtl_d_s (a : __v4f32) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vftint.w.s"] fn __lsx_vftint_w_s (a : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vftint.l.d"] fn __lsx_vftint_l_d (a : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vftint.wu.s"] fn __lsx_vftint_wu_s (a : __v4f32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vftint.lu.d"] fn __lsx_vftint_lu_d (a : __v2f64) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vftintrz.w.s"] fn __lsx_vftintrz_w_s (a : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vftintrz.l.d"] fn __lsx_vftintrz_l_d (a : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vftintrz.wu.s"] fn __lsx_vftintrz_wu_s (a : __v4f32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vftintrz.lu.d"] fn __lsx_vftintrz_lu_d (a : __v2f64) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vffint.s.w"] fn __lsx_vffint_s_w (a : __v4i32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vffint.d.l"] fn __lsx_vffint_d_l (a : __v2i64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vffint.s.wu"] fn __lsx_vffint_s_wu (a : __v4u32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vffint.d.lu"] fn __lsx_vffint_d_lu (a : __v2u64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vandn.v"] fn __lsx_vandn_v (a : __v16u8 , b : __v16u8) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vneg.b"] fn __lsx_vneg_b (a : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vneg.h"] fn __lsx_vneg_h (a : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vneg.w"] fn __lsx_vneg_w (a : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vneg.d"] fn __lsx_vneg_d (a : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmuh.b"] fn __lsx_vmuh_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vmuh.h"] fn __lsx_vmuh_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vmuh.w"] fn __lsx_vmuh_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vmuh.d"] fn __lsx_vmuh_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmuh.bu"] fn __lsx_vmuh_bu (a : __v16u8 , b : __v16u8) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vmuh.hu"] fn __lsx_vmuh_hu (a : __v8u16 , b : __v8u16) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vmuh.wu"] fn __lsx_vmuh_wu (a : __v4u32 , b : __v4u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vmuh.du"] fn __lsx_vmuh_du (a : __v2u64 , b : __v2u64) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vsllwil.h.b"] fn __lsx_vsllwil_h_b (a : __v16i8 , b : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsllwil.w.h"] fn __lsx_vsllwil_w_h (a : __v8i16 , b : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsllwil.d.w"] fn __lsx_vsllwil_d_w (a : __v4i32 , b : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsllwil.hu.bu"] fn __lsx_vsllwil_hu_bu (a : __v16u8 , b : u32) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vsllwil.wu.hu"] fn __lsx_vsllwil_wu_hu (a : __v8u16 , b : u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vsllwil.du.wu"] fn __lsx_vsllwil_du_wu (a : __v4u32 , b : u32) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vsran.b.h"] fn __lsx_vsran_b_h (a : __v8i16 , b : __v8i16) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsran.h.w"] fn __lsx_vsran_h_w (a : __v4i32 , b : __v4i32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsran.w.d"] fn __lsx_vsran_w_d (a : __v2i64 , b : __v2i64) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vssran.b.h"] fn __lsx_vssran_b_h (a : __v8i16 , b : __v8i16) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vssran.h.w"] fn __lsx_vssran_h_w (a : __v4i32 , b : __v4i32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vssran.w.d"] fn __lsx_vssran_w_d (a : __v2i64 , b : __v2i64) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vssran.bu.h"] fn __lsx_vssran_bu_h (a : __v8u16 , b : __v8u16) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vssran.hu.w"] fn __lsx_vssran_hu_w (a : __v4u32 , b : __v4u32) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vssran.wu.d"] fn __lsx_vssran_wu_d (a : __v2u64 , b : __v2u64) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vsrarn.b.h"] fn __lsx_vsrarn_b_h (a : __v8i16 , b : __v8i16) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsrarn.h.w"] fn __lsx_vsrarn_h_w (a : __v4i32 , b : __v4i32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsrarn.w.d"] fn __lsx_vsrarn_w_d (a : __v2i64 , b : __v2i64) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vssrarn.b.h"] fn __lsx_vssrarn_b_h (a : __v8i16 , b : __v8i16) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vssrarn.h.w"] fn __lsx_vssrarn_h_w (a : __v4i32 , b : __v4i32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vssrarn.w.d"] fn __lsx_vssrarn_w_d (a : __v2i64 , b : __v2i64) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vssrarn.bu.h"] fn __lsx_vssrarn_bu_h (a : __v8u16 , b : __v8u16) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vssrarn.hu.w"] fn __lsx_vssrarn_hu_w (a : __v4u32 , b : __v4u32) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vssrarn.wu.d"] fn __lsx_vssrarn_wu_d (a : __v2u64 , b : __v2u64) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vsrln.b.h"] fn __lsx_vsrln_b_h (a : __v8i16 , b : __v8i16) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsrln.h.w"] fn __lsx_vsrln_h_w (a : __v4i32 , b : __v4i32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsrln.w.d"] fn __lsx_vsrln_w_d (a : __v2i64 , b : __v2i64) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vssrln.bu.h"] fn __lsx_vssrln_bu_h (a : __v8u16 , b : __v8u16) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vssrln.hu.w"] fn __lsx_vssrln_hu_w (a : __v4u32 , b : __v4u32) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vssrln.wu.d"] fn __lsx_vssrln_wu_d (a : __v2u64 , b : __v2u64) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vsrlrn.b.h"] fn __lsx_vsrlrn_b_h (a : __v8i16 , b : __v8i16) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsrlrn.h.w"] fn __lsx_vsrlrn_h_w (a : __v4i32 , b : __v4i32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsrlrn.w.d"] fn __lsx_vsrlrn_w_d (a : __v2i64 , b : __v2i64) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vssrlrn.bu.h"] fn __lsx_vssrlrn_bu_h (a : __v8u16 , b : __v8u16) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vssrlrn.hu.w"] fn __lsx_vssrlrn_hu_w (a : __v4u32 , b : __v4u32) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vssrlrn.wu.d"] fn __lsx_vssrlrn_wu_d (a : __v2u64 , b : __v2u64) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vfrstpi.b"] fn __lsx_vfrstpi_b (a : __v16i8 , b : __v16i8 , c : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vfrstpi.h"] fn __lsx_vfrstpi_h (a : __v8i16 , b : __v8i16 , c : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vfrstp.b"] fn __lsx_vfrstp_b (a : __v16i8 , b : __v16i8 , c : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vfrstp.h"] fn __lsx_vfrstp_h (a : __v8i16 , b : __v8i16 , c : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vshuf4i.d"] fn __lsx_vshuf4i_d (a : __v2i64 , b : __v2i64 , c : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vbsrl.v"] fn __lsx_vbsrl_v (a : __v16i8 , b : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vbsll.v"] fn __lsx_vbsll_v (a : __v16i8 , b : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vextrins.b"] fn __lsx_vextrins_b (a : __v16i8 , b : __v16i8 , c : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vextrins.h"] fn __lsx_vextrins_h (a : __v8i16 , b : __v8i16 , c : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vextrins.w"] fn __lsx_vextrins_w (a : __v4i32 , b : __v4i32 , c : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vextrins.d"] fn __lsx_vextrins_d (a : __v2i64 , b : __v2i64 , c : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmskltz.b"] fn __lsx_vmskltz_b (a : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vmskltz.h"] fn __lsx_vmskltz_h (a : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vmskltz.w"] fn __lsx_vmskltz_w (a : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vmskltz.d"] fn __lsx_vmskltz_d (a : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsigncov.b"] fn __lsx_vsigncov_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsigncov.h"] fn __lsx_vsigncov_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsigncov.w"] fn __lsx_vsigncov_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsigncov.d"] fn __lsx_vsigncov_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfmadd.s"] fn __lsx_vfmadd_s (a : __v4f32 , b : __v4f32 , c : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfmadd.d"] fn __lsx_vfmadd_d (a : __v2f64 , b : __v2f64 , c : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vfmsub.s"] fn __lsx_vfmsub_s (a : __v4f32 , b : __v4f32 , c : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfmsub.d"] fn __lsx_vfmsub_d (a : __v2f64 , b : __v2f64 , c : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vfnmadd.s"] fn __lsx_vfnmadd_s (a : __v4f32 , b : __v4f32 , c : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfnmadd.d"] fn __lsx_vfnmadd_d (a : __v2f64 , b : __v2f64 , c : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vfnmsub.s"] fn __lsx_vfnmsub_s (a : __v4f32 , b : __v4f32 , c : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfnmsub.d"] fn __lsx_vfnmsub_d (a : __v2f64 , b : __v2f64 , c : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vftintrne.w.s"] fn __lsx_vftintrne_w_s (a : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vftintrne.l.d"] fn __lsx_vftintrne_l_d (a : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vftintrp.w.s"] fn __lsx_vftintrp_w_s (a : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vftintrp.l.d"] fn __lsx_vftintrp_l_d (a : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vftintrm.w.s"] fn __lsx_vftintrm_w_s (a : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vftintrm.l.d"] fn __lsx_vftintrm_l_d (a : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vftint.w.d"] fn __lsx_vftint_w_d (a : __v2f64 , b : __v2f64) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vffint.s.l"] fn __lsx_vffint_s_l (a : __v2i64 , b : __v2i64) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vftintrz.w.d"] fn __lsx_vftintrz_w_d (a : __v2f64 , b : __v2f64) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vftintrp.w.d"] fn __lsx_vftintrp_w_d (a : __v2f64 , b : __v2f64) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vftintrm.w.d"] fn __lsx_vftintrm_w_d (a : __v2f64 , b : __v2f64) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vftintrne.w.d"] fn __lsx_vftintrne_w_d (a : __v2f64 , b : __v2f64) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vftintl.l.s"] fn __lsx_vftintl_l_s (a : __v4f32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vftinth.l.s"] fn __lsx_vftinth_l_s (a : __v4f32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vffinth.d.w"] fn __lsx_vffinth_d_w (a : __v4i32) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vffintl.d.w"] fn __lsx_vffintl_d_w (a : __v4i32) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vftintrzl.l.s"] fn __lsx_vftintrzl_l_s (a : __v4f32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vftintrzh.l.s"] fn __lsx_vftintrzh_l_s (a : __v4f32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vftintrpl.l.s"] fn __lsx_vftintrpl_l_s (a : __v4f32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vftintrph.l.s"] fn __lsx_vftintrph_l_s (a : __v4f32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vftintrml.l.s"] fn __lsx_vftintrml_l_s (a : __v4f32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vftintrmh.l.s"] fn __lsx_vftintrmh_l_s (a : __v4f32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vftintrnel.l.s"] fn __lsx_vftintrnel_l_s (a : __v4f32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vftintrneh.l.s"] fn __lsx_vftintrneh_l_s (a : __v4f32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfrintrne.s"] fn __lsx_vfrintrne_s (a : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfrintrne.d"] fn __lsx_vfrintrne_d (a : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vfrintrz.s"] fn __lsx_vfrintrz_s (a : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfrintrz.d"] fn __lsx_vfrintrz_d (a : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vfrintrp.s"] fn __lsx_vfrintrp_s (a : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfrintrp.d"] fn __lsx_vfrintrp_d (a : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vfrintrm.s"] fn __lsx_vfrintrm_s (a : __v4f32) -> __v4f32 ; #[link_name = "llvm.loongarch.lsx.vfrintrm.d"] fn __lsx_vfrintrm_d (a : __v2f64) -> __v2f64 ; #[link_name = "llvm.loongarch.lsx.vstelm.b"] fn __lsx_vstelm_b (a : __v16i8 , b : * mut i8 , c : i32 , d : u32) ; #[link_name = "llvm.loongarch.lsx.vstelm.h"] fn __lsx_vstelm_h (a : __v8i16 , b : * mut i8 , c : i32 , d : u32) ; #[link_name = "llvm.loongarch.lsx.vstelm.w"] fn __lsx_vstelm_w (a : __v4i32 , b : * mut i8 , c : i32 , d : u32) ; #[link_name = "llvm.loongarch.lsx.vstelm.d"] fn __lsx_vstelm_d (a : __v2i64 , b : * mut i8 , c : i32 , d : u32) ; #[link_name = "llvm.loongarch.lsx.vaddwev.d.w"] fn __lsx_vaddwev_d_w (a : __v4i32 , b : __v4i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vaddwev.w.h"] fn __lsx_vaddwev_w_h (a : __v8i16 , b : __v8i16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vaddwev.h.b"] fn __lsx_vaddwev_h_b (a : __v16i8 , b : __v16i8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vaddwod.d.w"] fn __lsx_vaddwod_d_w (a : __v4i32 , b : __v4i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vaddwod.w.h"] fn __lsx_vaddwod_w_h (a : __v8i16 , b : __v8i16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vaddwod.h.b"] fn __lsx_vaddwod_h_b (a : __v16i8 , b : __v16i8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vaddwev.d.wu"] fn __lsx_vaddwev_d_wu (a : __v4u32 , b : __v4u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vaddwev.w.hu"] fn __lsx_vaddwev_w_hu (a : __v8u16 , b : __v8u16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vaddwev.h.bu"] fn __lsx_vaddwev_h_bu (a : __v16u8 , b : __v16u8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vaddwod.d.wu"] fn __lsx_vaddwod_d_wu (a : __v4u32 , b : __v4u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vaddwod.w.hu"] fn __lsx_vaddwod_w_hu (a : __v8u16 , b : __v8u16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vaddwod.h.bu"] fn __lsx_vaddwod_h_bu (a : __v16u8 , b : __v16u8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vaddwev.d.wu.w"] fn __lsx_vaddwev_d_wu_w (a : __v4u32 , b : __v4i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vaddwev.w.hu.h"] fn __lsx_vaddwev_w_hu_h (a : __v8u16 , b : __v8i16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vaddwev.h.bu.b"] fn __lsx_vaddwev_h_bu_b (a : __v16u8 , b : __v16i8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vaddwod.d.wu.w"] fn __lsx_vaddwod_d_wu_w (a : __v4u32 , b : __v4i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vaddwod.w.hu.h"] fn __lsx_vaddwod_w_hu_h (a : __v8u16 , b : __v8i16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vaddwod.h.bu.b"] fn __lsx_vaddwod_h_bu_b (a : __v16u8 , b : __v16i8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsubwev.d.w"] fn __lsx_vsubwev_d_w (a : __v4i32 , b : __v4i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsubwev.w.h"] fn __lsx_vsubwev_w_h (a : __v8i16 , b : __v8i16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsubwev.h.b"] fn __lsx_vsubwev_h_b (a : __v16i8 , b : __v16i8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsubwod.d.w"] fn __lsx_vsubwod_d_w (a : __v4i32 , b : __v4i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsubwod.w.h"] fn __lsx_vsubwod_w_h (a : __v8i16 , b : __v8i16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsubwod.h.b"] fn __lsx_vsubwod_h_b (a : __v16i8 , b : __v16i8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsubwev.d.wu"] fn __lsx_vsubwev_d_wu (a : __v4u32 , b : __v4u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsubwev.w.hu"] fn __lsx_vsubwev_w_hu (a : __v8u16 , b : __v8u16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsubwev.h.bu"] fn __lsx_vsubwev_h_bu (a : __v16u8 , b : __v16u8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsubwod.d.wu"] fn __lsx_vsubwod_d_wu (a : __v4u32 , b : __v4u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsubwod.w.hu"] fn __lsx_vsubwod_w_hu (a : __v8u16 , b : __v8u16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsubwod.h.bu"] fn __lsx_vsubwod_h_bu (a : __v16u8 , b : __v16u8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vaddwev.q.d"] fn __lsx_vaddwev_q_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vaddwod.q.d"] fn __lsx_vaddwod_q_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vaddwev.q.du"] fn __lsx_vaddwev_q_du (a : __v2u64 , b : __v2u64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vaddwod.q.du"] fn __lsx_vaddwod_q_du (a : __v2u64 , b : __v2u64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsubwev.q.d"] fn __lsx_vsubwev_q_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsubwod.q.d"] fn __lsx_vsubwod_q_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsubwev.q.du"] fn __lsx_vsubwev_q_du (a : __v2u64 , b : __v2u64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsubwod.q.du"] fn __lsx_vsubwod_q_du (a : __v2u64 , b : __v2u64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vaddwev.q.du.d"] fn __lsx_vaddwev_q_du_d (a : __v2u64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vaddwod.q.du.d"] fn __lsx_vaddwod_q_du_d (a : __v2u64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmulwev.d.w"] fn __lsx_vmulwev_d_w (a : __v4i32 , b : __v4i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmulwev.w.h"] fn __lsx_vmulwev_w_h (a : __v8i16 , b : __v8i16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vmulwev.h.b"] fn __lsx_vmulwev_h_b (a : __v16i8 , b : __v16i8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vmulwod.d.w"] fn __lsx_vmulwod_d_w (a : __v4i32 , b : __v4i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmulwod.w.h"] fn __lsx_vmulwod_w_h (a : __v8i16 , b : __v8i16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vmulwod.h.b"] fn __lsx_vmulwod_h_b (a : __v16i8 , b : __v16i8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vmulwev.d.wu"] fn __lsx_vmulwev_d_wu (a : __v4u32 , b : __v4u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmulwev.w.hu"] fn __lsx_vmulwev_w_hu (a : __v8u16 , b : __v8u16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vmulwev.h.bu"] fn __lsx_vmulwev_h_bu (a : __v16u8 , b : __v16u8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vmulwod.d.wu"] fn __lsx_vmulwod_d_wu (a : __v4u32 , b : __v4u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmulwod.w.hu"] fn __lsx_vmulwod_w_hu (a : __v8u16 , b : __v8u16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vmulwod.h.bu"] fn __lsx_vmulwod_h_bu (a : __v16u8 , b : __v16u8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vmulwev.d.wu.w"] fn __lsx_vmulwev_d_wu_w (a : __v4u32 , b : __v4i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmulwev.w.hu.h"] fn __lsx_vmulwev_w_hu_h (a : __v8u16 , b : __v8i16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vmulwev.h.bu.b"] fn __lsx_vmulwev_h_bu_b (a : __v16u8 , b : __v16i8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vmulwod.d.wu.w"] fn __lsx_vmulwod_d_wu_w (a : __v4u32 , b : __v4i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmulwod.w.hu.h"] fn __lsx_vmulwod_w_hu_h (a : __v8u16 , b : __v8i16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vmulwod.h.bu.b"] fn __lsx_vmulwod_h_bu_b (a : __v16u8 , b : __v16i8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vmulwev.q.d"] fn __lsx_vmulwev_q_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmulwod.q.d"] fn __lsx_vmulwod_q_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmulwev.q.du"] fn __lsx_vmulwev_q_du (a : __v2u64 , b : __v2u64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmulwod.q.du"] fn __lsx_vmulwod_q_du (a : __v2u64 , b : __v2u64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmulwev.q.du.d"] fn __lsx_vmulwev_q_du_d (a : __v2u64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmulwod.q.du.d"] fn __lsx_vmulwod_q_du_d (a : __v2u64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vhaddw.q.d"] fn __lsx_vhaddw_q_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vhaddw.qu.du"] fn __lsx_vhaddw_qu_du (a : __v2u64 , b : __v2u64) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vhsubw.q.d"] fn __lsx_vhsubw_q_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vhsubw.qu.du"] fn __lsx_vhsubw_qu_du (a : __v2u64 , b : __v2u64) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vmaddwev.d.w"] fn __lsx_vmaddwev_d_w (a : __v2i64 , b : __v4i32 , c : __v4i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmaddwev.w.h"] fn __lsx_vmaddwev_w_h (a : __v4i32 , b : __v8i16 , c : __v8i16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vmaddwev.h.b"] fn __lsx_vmaddwev_h_b (a : __v8i16 , b : __v16i8 , c : __v16i8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vmaddwev.d.wu"] fn __lsx_vmaddwev_d_wu (a : __v2u64 , b : __v4u32 , c : __v4u32) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vmaddwev.w.hu"] fn __lsx_vmaddwev_w_hu (a : __v4u32 , b : __v8u16 , c : __v8u16) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vmaddwev.h.bu"] fn __lsx_vmaddwev_h_bu (a : __v8u16 , b : __v16u8 , c : __v16u8) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vmaddwod.d.w"] fn __lsx_vmaddwod_d_w (a : __v2i64 , b : __v4i32 , c : __v4i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmaddwod.w.h"] fn __lsx_vmaddwod_w_h (a : __v4i32 , b : __v8i16 , c : __v8i16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vmaddwod.h.b"] fn __lsx_vmaddwod_h_b (a : __v8i16 , b : __v16i8 , c : __v16i8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vmaddwod.d.wu"] fn __lsx_vmaddwod_d_wu (a : __v2u64 , b : __v4u32 , c : __v4u32) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vmaddwod.w.hu"] fn __lsx_vmaddwod_w_hu (a : __v4u32 , b : __v8u16 , c : __v8u16) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vmaddwod.h.bu"] fn __lsx_vmaddwod_h_bu (a : __v8u16 , b : __v16u8 , c : __v16u8) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vmaddwev.d.wu.w"] fn __lsx_vmaddwev_d_wu_w (a : __v2i64 , b : __v4u32 , c : __v4i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmaddwev.w.hu.h"] fn __lsx_vmaddwev_w_hu_h (a : __v4i32 , b : __v8u16 , c : __v8i16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vmaddwev.h.bu.b"] fn __lsx_vmaddwev_h_bu_b (a : __v8i16 , b : __v16u8 , c : __v16i8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vmaddwod.d.wu.w"] fn __lsx_vmaddwod_d_wu_w (a : __v2i64 , b : __v4u32 , c : __v4i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmaddwod.w.hu.h"] fn __lsx_vmaddwod_w_hu_h (a : __v4i32 , b : __v8u16 , c : __v8i16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vmaddwod.h.bu.b"] fn __lsx_vmaddwod_h_bu_b (a : __v8i16 , b : __v16u8 , c : __v16i8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vmaddwev.q.d"] fn __lsx_vmaddwev_q_d (a : __v2i64 , b : __v2i64 , c : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmaddwod.q.d"] fn __lsx_vmaddwod_q_d (a : __v2i64 , b : __v2i64 , c : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmaddwev.q.du"] fn __lsx_vmaddwev_q_du (a : __v2u64 , b : __v2u64 , c : __v2u64) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vmaddwod.q.du"] fn __lsx_vmaddwod_q_du (a : __v2u64 , b : __v2u64 , c : __v2u64) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vmaddwev.q.du.d"] fn __lsx_vmaddwev_q_du_d (a : __v2i64 , b : __v2u64 , c : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmaddwod.q.du.d"] fn __lsx_vmaddwod_q_du_d (a : __v2i64 , b : __v2u64 , c : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vrotr.b"] fn __lsx_vrotr_b (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vrotr.h"] fn __lsx_vrotr_h (a : __v8i16 , b : __v8i16) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vrotr.w"] fn __lsx_vrotr_w (a : __v4i32 , b : __v4i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vrotr.d"] fn __lsx_vrotr_d (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vadd.q"] fn __lsx_vadd_q (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsub.q"] fn __lsx_vsub_q (a : __v2i64 , b : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vldrepl.b"] fn __lsx_vldrepl_b (a : * const i8 , b : i32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vldrepl.h"] fn __lsx_vldrepl_h (a : * const i8 , b : i32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vldrepl.w"] fn __lsx_vldrepl_w (a : * const i8 , b : i32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vldrepl.d"] fn __lsx_vldrepl_d (a : * const i8 , b : i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vmskgez.b"] fn __lsx_vmskgez_b (a : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vmsknz.b"] fn __lsx_vmsknz_b (a : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vexth.h.b"] fn __lsx_vexth_h_b (a : __v16i8) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vexth.w.h"] fn __lsx_vexth_w_h (a : __v8i16) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vexth.d.w"] fn __lsx_vexth_d_w (a : __v4i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vexth.q.d"] fn __lsx_vexth_q_d (a : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vexth.hu.bu"] fn __lsx_vexth_hu_bu (a : __v16u8) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vexth.wu.hu"] fn __lsx_vexth_wu_hu (a : __v8u16) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vexth.du.wu"] fn __lsx_vexth_du_wu (a : __v4u32) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vexth.qu.du"] fn __lsx_vexth_qu_du (a : __v2u64) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vrotri.b"] fn __lsx_vrotri_b (a : __v16i8 , b : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vrotri.h"] fn __lsx_vrotri_h (a : __v8i16 , b : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vrotri.w"] fn __lsx_vrotri_w (a : __v4i32 , b : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vrotri.d"] fn __lsx_vrotri_d (a : __v2i64 , b : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vextl.q.d"] fn __lsx_vextl_q_d (a : __v2i64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsrlni.b.h"] fn __lsx_vsrlni_b_h (a : __v16i8 , b : __v16i8 , c : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsrlni.h.w"] fn __lsx_vsrlni_h_w (a : __v8i16 , b : __v8i16 , c : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsrlni.w.d"] fn __lsx_vsrlni_w_d (a : __v4i32 , b : __v4i32 , c : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsrlni.d.q"] fn __lsx_vsrlni_d_q (a : __v2i64 , b : __v2i64 , c : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsrlrni.b.h"] fn __lsx_vsrlrni_b_h (a : __v16i8 , b : __v16i8 , c : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsrlrni.h.w"] fn __lsx_vsrlrni_h_w (a : __v8i16 , b : __v8i16 , c : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsrlrni.w.d"] fn __lsx_vsrlrni_w_d (a : __v4i32 , b : __v4i32 , c : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsrlrni.d.q"] fn __lsx_vsrlrni_d_q (a : __v2i64 , b : __v2i64 , c : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vssrlni.b.h"] fn __lsx_vssrlni_b_h (a : __v16i8 , b : __v16i8 , c : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vssrlni.h.w"] fn __lsx_vssrlni_h_w (a : __v8i16 , b : __v8i16 , c : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vssrlni.w.d"] fn __lsx_vssrlni_w_d (a : __v4i32 , b : __v4i32 , c : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vssrlni.d.q"] fn __lsx_vssrlni_d_q (a : __v2i64 , b : __v2i64 , c : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vssrlni.bu.h"] fn __lsx_vssrlni_bu_h (a : __v16u8 , b : __v16i8 , c : u32) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vssrlni.hu.w"] fn __lsx_vssrlni_hu_w (a : __v8u16 , b : __v8i16 , c : u32) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vssrlni.wu.d"] fn __lsx_vssrlni_wu_d (a : __v4u32 , b : __v4i32 , c : u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vssrlni.du.q"] fn __lsx_vssrlni_du_q (a : __v2u64 , b : __v2i64 , c : u32) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vssrlrni.b.h"] fn __lsx_vssrlrni_b_h (a : __v16i8 , b : __v16i8 , c : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vssrlrni.h.w"] fn __lsx_vssrlrni_h_w (a : __v8i16 , b : __v8i16 , c : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vssrlrni.w.d"] fn __lsx_vssrlrni_w_d (a : __v4i32 , b : __v4i32 , c : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vssrlrni.d.q"] fn __lsx_vssrlrni_d_q (a : __v2i64 , b : __v2i64 , c : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vssrlrni.bu.h"] fn __lsx_vssrlrni_bu_h (a : __v16u8 , b : __v16i8 , c : u32) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vssrlrni.hu.w"] fn __lsx_vssrlrni_hu_w (a : __v8u16 , b : __v8i16 , c : u32) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vssrlrni.wu.d"] fn __lsx_vssrlrni_wu_d (a : __v4u32 , b : __v4i32 , c : u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vssrlrni.du.q"] fn __lsx_vssrlrni_du_q (a : __v2u64 , b : __v2i64 , c : u32) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vsrani.b.h"] fn __lsx_vsrani_b_h (a : __v16i8 , b : __v16i8 , c : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsrani.h.w"] fn __lsx_vsrani_h_w (a : __v8i16 , b : __v8i16 , c : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsrani.w.d"] fn __lsx_vsrani_w_d (a : __v4i32 , b : __v4i32 , c : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsrani.d.q"] fn __lsx_vsrani_d_q (a : __v2i64 , b : __v2i64 , c : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vsrarni.b.h"] fn __lsx_vsrarni_b_h (a : __v16i8 , b : __v16i8 , c : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vsrarni.h.w"] fn __lsx_vsrarni_h_w (a : __v8i16 , b : __v8i16 , c : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vsrarni.w.d"] fn __lsx_vsrarni_w_d (a : __v4i32 , b : __v4i32 , c : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vsrarni.d.q"] fn __lsx_vsrarni_d_q (a : __v2i64 , b : __v2i64 , c : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vssrani.b.h"] fn __lsx_vssrani_b_h (a : __v16i8 , b : __v16i8 , c : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vssrani.h.w"] fn __lsx_vssrani_h_w (a : __v8i16 , b : __v8i16 , c : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vssrani.w.d"] fn __lsx_vssrani_w_d (a : __v4i32 , b : __v4i32 , c : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vssrani.d.q"] fn __lsx_vssrani_d_q (a : __v2i64 , b : __v2i64 , c : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vssrani.bu.h"] fn __lsx_vssrani_bu_h (a : __v16u8 , b : __v16i8 , c : u32) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vssrani.hu.w"] fn __lsx_vssrani_hu_w (a : __v8u16 , b : __v8i16 , c : u32) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vssrani.wu.d"] fn __lsx_vssrani_wu_d (a : __v4u32 , b : __v4i32 , c : u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vssrani.du.q"] fn __lsx_vssrani_du_q (a : __v2u64 , b : __v2i64 , c : u32) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vssrarni.b.h"] fn __lsx_vssrarni_b_h (a : __v16i8 , b : __v16i8 , c : u32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vssrarni.h.w"] fn __lsx_vssrarni_h_w (a : __v8i16 , b : __v8i16 , c : u32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vssrarni.w.d"] fn __lsx_vssrarni_w_d (a : __v4i32 , b : __v4i32 , c : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vssrarni.d.q"] fn __lsx_vssrarni_d_q (a : __v2i64 , b : __v2i64 , c : u32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vssrarni.bu.h"] fn __lsx_vssrarni_bu_h (a : __v16u8 , b : __v16i8 , c : u32) -> __v16u8 ; #[link_name = "llvm.loongarch.lsx.vssrarni.hu.w"] fn __lsx_vssrarni_hu_w (a : __v8u16 , b : __v8i16 , c : u32) -> __v8u16 ; #[link_name = "llvm.loongarch.lsx.vssrarni.wu.d"] fn __lsx_vssrarni_wu_d (a : __v4u32 , b : __v4i32 , c : u32) -> __v4u32 ; #[link_name = "llvm.loongarch.lsx.vssrarni.du.q"] fn __lsx_vssrarni_du_q (a : __v2u64 , b : __v2i64 , c : u32) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.vpermi.w"] fn __lsx_vpermi_w (a : __v4i32 , b : __v4i32 , c : u32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vld"] fn __lsx_vld (a : * const i8 , b : i32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vst"] fn __lsx_vst (a : __v16i8 , b : * mut i8 , c : i32) ; #[link_name = "llvm.loongarch.lsx.vssrlrn.b.h"] fn __lsx_vssrlrn_b_h (a : __v8i16 , b : __v8i16) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vssrlrn.h.w"] fn __lsx_vssrlrn_h_w (a : __v4i32 , b : __v4i32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vssrlrn.w.d"] fn __lsx_vssrlrn_w_d (a : __v2i64 , b : __v2i64) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vssrln.b.h"] fn __lsx_vssrln_b_h (a : __v8i16 , b : __v8i16) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vssrln.h.w"] fn __lsx_vssrln_h_w (a : __v4i32 , b : __v4i32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vssrln.w.d"] fn __lsx_vssrln_w_d (a : __v2i64 , b : __v2i64) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vorn.v"] fn __lsx_vorn_v (a : __v16i8 , b : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vldi"] fn __lsx_vldi (a : i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vshuf.b"] fn __lsx_vshuf_b (a : __v16i8 , b : __v16i8 , c : __v16i8) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vldx"] fn __lsx_vldx (a : * const i8 , b : i64) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vstx"] fn __lsx_vstx (a : __v16i8 , b : * mut i8 , c : i64) ; #[link_name = "llvm.loongarch.lsx.vextl.qu.du"] fn __lsx_vextl_qu_du (a : __v2u64) -> __v2u64 ; #[link_name = "llvm.loongarch.lsx.bnz.b"] fn __lsx_bnz_b (a : __v16u8) -> i32 ; #[link_name = "llvm.loongarch.lsx.bnz.d"] fn __lsx_bnz_d (a : __v2u64) -> i32 ; #[link_name = "llvm.loongarch.lsx.bnz.h"] fn __lsx_bnz_h (a : __v8u16) -> i32 ; #[link_name = "llvm.loongarch.lsx.bnz.v"] fn __lsx_bnz_v (a : __v16u8) -> i32 ; #[link_name = "llvm.loongarch.lsx.bnz.w"] fn __lsx_bnz_w (a : __v4u32) -> i32 ; #[link_name = "llvm.loongarch.lsx.bz.b"] fn __lsx_bz_b (a : __v16u8) -> i32 ; #[link_name = "llvm.loongarch.lsx.bz.d"] fn __lsx_bz_d (a : __v2u64) -> i32 ; #[link_name = "llvm.loongarch.lsx.bz.h"] fn __lsx_bz_h (a : __v8u16) -> i32 ; #[link_name = "llvm.loongarch.lsx.bz.v"] fn __lsx_bz_v (a : __v16u8) -> i32 ; #[link_name = "llvm.loongarch.lsx.bz.w"] fn __lsx_bz_w (a : __v4u32) -> i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.caf.d"] fn __lsx_vfcmp_caf_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.caf.s"] fn __lsx_vfcmp_caf_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.ceq.d"] fn __lsx_vfcmp_ceq_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.ceq.s"] fn __lsx_vfcmp_ceq_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.cle.d"] fn __lsx_vfcmp_cle_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.cle.s"] fn __lsx_vfcmp_cle_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.clt.d"] fn __lsx_vfcmp_clt_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.clt.s"] fn __lsx_vfcmp_clt_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.cne.d"] fn __lsx_vfcmp_cne_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.cne.s"] fn __lsx_vfcmp_cne_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.cor.d"] fn __lsx_vfcmp_cor_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.cor.s"] fn __lsx_vfcmp_cor_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.cueq.d"] fn __lsx_vfcmp_cueq_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.cueq.s"] fn __lsx_vfcmp_cueq_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.cule.d"] fn __lsx_vfcmp_cule_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.cule.s"] fn __lsx_vfcmp_cule_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.cult.d"] fn __lsx_vfcmp_cult_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.cult.s"] fn __lsx_vfcmp_cult_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.cun.d"] fn __lsx_vfcmp_cun_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.cune.d"] fn __lsx_vfcmp_cune_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.cune.s"] fn __lsx_vfcmp_cune_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.cun.s"] fn __lsx_vfcmp_cun_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.saf.d"] fn __lsx_vfcmp_saf_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.saf.s"] fn __lsx_vfcmp_saf_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.seq.d"] fn __lsx_vfcmp_seq_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.seq.s"] fn __lsx_vfcmp_seq_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.sle.d"] fn __lsx_vfcmp_sle_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.sle.s"] fn __lsx_vfcmp_sle_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.slt.d"] fn __lsx_vfcmp_slt_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.slt.s"] fn __lsx_vfcmp_slt_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.sne.d"] fn __lsx_vfcmp_sne_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.sne.s"] fn __lsx_vfcmp_sne_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.sor.d"] fn __lsx_vfcmp_sor_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.sor.s"] fn __lsx_vfcmp_sor_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.sueq.d"] fn __lsx_vfcmp_sueq_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.sueq.s"] fn __lsx_vfcmp_sueq_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.sule.d"] fn __lsx_vfcmp_sule_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.sule.s"] fn __lsx_vfcmp_sule_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.sult.d"] fn __lsx_vfcmp_sult_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.sult.s"] fn __lsx_vfcmp_sult_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.sun.d"] fn __lsx_vfcmp_sun_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.sune.d"] fn __lsx_vfcmp_sune_d (a : __v2f64 , b : __v2f64) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vfcmp.sune.s"] fn __lsx_vfcmp_sune_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vfcmp.sun.s"] fn __lsx_vfcmp_sun_s (a : __v4f32 , b : __v4f32) -> __v4i32 ; #[link_name = "llvm.loongarch.lsx.vrepli.b"] fn __lsx_vrepli_b (a : i32) -> __v16i8 ; #[link_name = "llvm.loongarch.lsx.vrepli.d"] fn __lsx_vrepli_d (a : i32) -> __v2i64 ; #[link_name = "llvm.loongarch.lsx.vrepli.h"] fn __lsx_vrepli_h (a : i32) -> __v8i16 ; #[link_name = "llvm.loongarch.lsx.vrepli.w"] fn __lsx_vrepli_w (a : i32) -> __v4i32 ; }}

macro_rules! lsx_vsll_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsll_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsll_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsll_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsll_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsll_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsll_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsll_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsll_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsll_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsll_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsll_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsll_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsll_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsll_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsll_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsll_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsll_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsll_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsll_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vslli_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslli_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslli_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslli_b < const IMM3 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM3 , 3) ; unsafe { transmute (__lsx_vslli_b (transmute (a) , IMM3)) } }
}

macro_rules! lsx_vslli_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslli_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslli_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslli_h < const IMM4 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vslli_h (transmute (a) , IMM4)) } }
}

macro_rules! lsx_vslli_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslli_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslli_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslli_w < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vslli_w (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vslli_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslli_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslli_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslli_d < const IMM6 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vslli_d (transmute (a) , IMM6)) } }
}

macro_rules! lsx_vsra_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsra_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsra_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsra_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsra_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsra_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsra_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsra_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsra_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsra_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsra_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsra_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsra_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsra_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsra_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsra_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsra_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsra_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsra_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsra_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrai_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrai_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrai_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrai_b < const IMM3 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM3 , 3) ; unsafe { transmute (__lsx_vsrai_b (transmute (a) , IMM3)) } }
}

macro_rules! lsx_vsrai_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrai_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrai_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrai_h < const IMM4 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vsrai_h (transmute (a) , IMM4)) } }
}

macro_rules! lsx_vsrai_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrai_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrai_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrai_w < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vsrai_w (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vsrai_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrai_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrai_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrai_d < const IMM6 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vsrai_d (transmute (a) , IMM6)) } }
}

macro_rules! lsx_vsrar_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrar_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrar_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrar_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsrar_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrar_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrar_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrar_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrar_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsrar_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrar_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrar_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrar_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrar_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsrar_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrar_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrar_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrar_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrar_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsrar_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrari_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrari_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrari_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrari_b < const IMM3 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM3 , 3) ; unsafe { transmute (__lsx_vsrari_b (transmute (a) , IMM3)) } }
}

macro_rules! lsx_vsrari_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrari_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrari_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrari_h < const IMM4 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vsrari_h (transmute (a) , IMM4)) } }
}

macro_rules! lsx_vsrari_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrari_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrari_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrari_w < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vsrari_w (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vsrari_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrari_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrari_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrari_d < const IMM6 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vsrari_d (transmute (a) , IMM6)) } }
}

macro_rules! lsx_vsrl_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrl_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrl_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrl_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsrl_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrl_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrl_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrl_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrl_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsrl_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrl_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrl_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrl_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrl_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsrl_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrl_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrl_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrl_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrl_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsrl_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrli_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrli_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrli_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrli_b < const IMM3 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM3 , 3) ; unsafe { transmute (__lsx_vsrli_b (transmute (a) , IMM3)) } }
}

macro_rules! lsx_vsrli_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrli_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrli_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrli_h < const IMM4 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vsrli_h (transmute (a) , IMM4)) } }
}

macro_rules! lsx_vsrli_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrli_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrli_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrli_w < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vsrli_w (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vsrli_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrli_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrli_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrli_d < const IMM6 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vsrli_d (transmute (a) , IMM6)) } }
}

macro_rules! lsx_vsrlr_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrlr_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrlr_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrlr_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsrlr_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrlr_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrlr_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrlr_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrlr_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsrlr_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrlr_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrlr_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrlr_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrlr_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsrlr_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrlr_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrlr_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrlr_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrlr_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsrlr_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrlri_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrlri_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrlri_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrlri_b < const IMM3 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM3 , 3) ; unsafe { transmute (__lsx_vsrlri_b (transmute (a) , IMM3)) } }
}

macro_rules! lsx_vsrlri_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrlri_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrlri_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrlri_h < const IMM4 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vsrlri_h (transmute (a) , IMM4)) } }
}

macro_rules! lsx_vsrlri_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrlri_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrlri_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrlri_w < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vsrlri_w (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vsrlri_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrlri_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrlri_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrlri_d < const IMM6 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vsrlri_d (transmute (a) , IMM6)) } }
}

macro_rules! lsx_vbitclr_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitclr_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitclr_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitclr_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vbitclr_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vbitclr_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitclr_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitclr_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitclr_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vbitclr_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vbitclr_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitclr_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitclr_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitclr_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vbitclr_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vbitclr_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitclr_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitclr_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitclr_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vbitclr_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vbitclri_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitclri_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitclri_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitclri_b < const IMM3 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM3 , 3) ; unsafe { transmute (__lsx_vbitclri_b (transmute (a) , IMM3)) } }
}

macro_rules! lsx_vbitclri_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitclri_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitclri_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitclri_h < const IMM4 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vbitclri_h (transmute (a) , IMM4)) } }
}

macro_rules! lsx_vbitclri_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitclri_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitclri_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitclri_w < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vbitclri_w (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vbitclri_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitclri_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitclri_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitclri_d < const IMM6 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vbitclri_d (transmute (a) , IMM6)) } }
}

macro_rules! lsx_vbitset_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitset_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitset_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitset_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vbitset_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vbitset_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitset_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitset_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitset_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vbitset_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vbitset_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitset_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitset_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitset_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vbitset_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vbitset_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitset_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitset_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitset_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vbitset_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vbitseti_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitseti_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitseti_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitseti_b < const IMM3 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM3 , 3) ; unsafe { transmute (__lsx_vbitseti_b (transmute (a) , IMM3)) } }
}

macro_rules! lsx_vbitseti_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitseti_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitseti_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitseti_h < const IMM4 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vbitseti_h (transmute (a) , IMM4)) } }
}

macro_rules! lsx_vbitseti_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitseti_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitseti_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitseti_w < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vbitseti_w (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vbitseti_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitseti_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitseti_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitseti_d < const IMM6 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vbitseti_d (transmute (a) , IMM6)) } }
}

macro_rules! lsx_vbitrev_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitrev_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitrev_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitrev_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vbitrev_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vbitrev_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitrev_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitrev_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitrev_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vbitrev_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vbitrev_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitrev_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitrev_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitrev_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vbitrev_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vbitrev_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitrev_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitrev_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitrev_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vbitrev_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vbitrevi_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitrevi_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitrevi_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitrevi_b < const IMM3 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM3 , 3) ; unsafe { transmute (__lsx_vbitrevi_b (transmute (a) , IMM3)) } }
}

macro_rules! lsx_vbitrevi_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitrevi_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitrevi_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitrevi_h < const IMM4 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vbitrevi_h (transmute (a) , IMM4)) } }
}

macro_rules! lsx_vbitrevi_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitrevi_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitrevi_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitrevi_w < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vbitrevi_w (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vbitrevi_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitrevi_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitrevi_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitrevi_d < const IMM6 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vbitrevi_d (transmute (a) , IMM6)) } }
}

macro_rules! lsx_vadd_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vadd_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vadd_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vadd_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vadd_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vadd_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vadd_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vadd_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vadd_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vadd_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vadd_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vadd_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vadd_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vadd_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vadd_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vadd_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vadd_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vadd_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vadd_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vadd_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddi_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddi_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddi_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddi_bu < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vaddi_bu (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vaddi_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddi_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddi_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddi_hu < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vaddi_hu (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vaddi_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddi_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddi_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddi_wu < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vaddi_wu (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vaddi_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddi_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddi_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddi_du < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vaddi_du (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vsub_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsub_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsub_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsub_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsub_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsub_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsub_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsub_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsub_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsub_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsub_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsub_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsub_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsub_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsub_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsub_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsub_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsub_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsub_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsub_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsubi_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsubi_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsubi_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsubi_bu < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vsubi_bu (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vsubi_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsubi_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsubi_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsubi_hu < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vsubi_hu (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vsubi_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsubi_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsubi_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsubi_wu < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vsubi_wu (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vsubi_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsubi_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsubi_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsubi_du < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vsubi_du (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vmax_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmax_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmax_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmax_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmax_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmax_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmax_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmax_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmax_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmax_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmax_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmax_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmax_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmax_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmax_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmax_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmax_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmax_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmax_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmax_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmaxi_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaxi_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaxi_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaxi_b < const IMM_S5 : i32 > (a : m128i) -> m128i { static_assert_simm_bits ! (IMM_S5 , 5) ; unsafe { transmute (__lsx_vmaxi_b (transmute (a) , IMM_S5)) } }
}

macro_rules! lsx_vmaxi_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaxi_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaxi_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaxi_h < const IMM_S5 : i32 > (a : m128i) -> m128i { static_assert_simm_bits ! (IMM_S5 , 5) ; unsafe { transmute (__lsx_vmaxi_h (transmute (a) , IMM_S5)) } }
}

macro_rules! lsx_vmaxi_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaxi_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaxi_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaxi_w < const IMM_S5 : i32 > (a : m128i) -> m128i { static_assert_simm_bits ! (IMM_S5 , 5) ; unsafe { transmute (__lsx_vmaxi_w (transmute (a) , IMM_S5)) } }
}

macro_rules! lsx_vmaxi_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaxi_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaxi_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaxi_d < const IMM_S5 : i32 > (a : m128i) -> m128i { static_assert_simm_bits ! (IMM_S5 , 5) ; unsafe { transmute (__lsx_vmaxi_d (transmute (a) , IMM_S5)) } }
}

macro_rules! lsx_vmax_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmax_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmax_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmax_bu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmax_bu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmax_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmax_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmax_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmax_hu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmax_hu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmax_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmax_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmax_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmax_wu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmax_wu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmax_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmax_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmax_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmax_du (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmax_du (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmaxi_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaxi_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaxi_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaxi_bu < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vmaxi_bu (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vmaxi_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaxi_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaxi_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaxi_hu < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vmaxi_hu (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vmaxi_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaxi_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaxi_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaxi_wu < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vmaxi_wu (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vmaxi_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaxi_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaxi_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaxi_du < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vmaxi_du (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vmin_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmin_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmin_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmin_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmin_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmin_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmin_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmin_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmin_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmin_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmin_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmin_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmin_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmin_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmin_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmin_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmin_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmin_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmin_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmin_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmini_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmini_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmini_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmini_b < const IMM_S5 : i32 > (a : m128i) -> m128i { static_assert_simm_bits ! (IMM_S5 , 5) ; unsafe { transmute (__lsx_vmini_b (transmute (a) , IMM_S5)) } }
}

macro_rules! lsx_vmini_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmini_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmini_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmini_h < const IMM_S5 : i32 > (a : m128i) -> m128i { static_assert_simm_bits ! (IMM_S5 , 5) ; unsafe { transmute (__lsx_vmini_h (transmute (a) , IMM_S5)) } }
}

macro_rules! lsx_vmini_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmini_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmini_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmini_w < const IMM_S5 : i32 > (a : m128i) -> m128i { static_assert_simm_bits ! (IMM_S5 , 5) ; unsafe { transmute (__lsx_vmini_w (transmute (a) , IMM_S5)) } }
}

macro_rules! lsx_vmini_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmini_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmini_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmini_d < const IMM_S5 : i32 > (a : m128i) -> m128i { static_assert_simm_bits ! (IMM_S5 , 5) ; unsafe { transmute (__lsx_vmini_d (transmute (a) , IMM_S5)) } }
}

macro_rules! lsx_vmin_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmin_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmin_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmin_bu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmin_bu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmin_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmin_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmin_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmin_hu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmin_hu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmin_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmin_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmin_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmin_wu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmin_wu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmin_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmin_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmin_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmin_du (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmin_du (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmini_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmini_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmini_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmini_bu < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vmini_bu (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vmini_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmini_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmini_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmini_hu < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vmini_hu (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vmini_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmini_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmini_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmini_wu < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vmini_wu (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vmini_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmini_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmini_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmini_du < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vmini_du (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vseq_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vseq_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vseq_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vseq_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vseq_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vseq_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vseq_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vseq_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vseq_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vseq_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vseq_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vseq_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vseq_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vseq_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vseq_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vseq_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vseq_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vseq_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vseq_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vseq_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vseqi_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vseqi_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vseqi_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vseqi_b < const IMM_S5 : i32 > (a : m128i) -> m128i { static_assert_simm_bits ! (IMM_S5 , 5) ; unsafe { transmute (__lsx_vseqi_b (transmute (a) , IMM_S5)) } }
}

macro_rules! lsx_vseqi_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vseqi_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vseqi_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vseqi_h < const IMM_S5 : i32 > (a : m128i) -> m128i { static_assert_simm_bits ! (IMM_S5 , 5) ; unsafe { transmute (__lsx_vseqi_h (transmute (a) , IMM_S5)) } }
}

macro_rules! lsx_vseqi_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vseqi_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vseqi_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vseqi_w < const IMM_S5 : i32 > (a : m128i) -> m128i { static_assert_simm_bits ! (IMM_S5 , 5) ; unsafe { transmute (__lsx_vseqi_w (transmute (a) , IMM_S5)) } }
}

macro_rules! lsx_vseqi_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vseqi_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vseqi_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vseqi_d < const IMM_S5 : i32 > (a : m128i) -> m128i { static_assert_simm_bits ! (IMM_S5 , 5) ; unsafe { transmute (__lsx_vseqi_d (transmute (a) , IMM_S5)) } }
}

macro_rules! lsx_vslti_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslti_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslti_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslti_b < const IMM_S5 : i32 > (a : m128i) -> m128i { static_assert_simm_bits ! (IMM_S5 , 5) ; unsafe { transmute (__lsx_vslti_b (transmute (a) , IMM_S5)) } }
}

macro_rules! lsx_vslt_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslt_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslt_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslt_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vslt_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vslt_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslt_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslt_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslt_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vslt_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vslt_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslt_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslt_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslt_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vslt_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vslt_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslt_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslt_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslt_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vslt_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vslti_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslti_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslti_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslti_h < const IMM_S5 : i32 > (a : m128i) -> m128i { static_assert_simm_bits ! (IMM_S5 , 5) ; unsafe { transmute (__lsx_vslti_h (transmute (a) , IMM_S5)) } }
}

macro_rules! lsx_vslti_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslti_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslti_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslti_w < const IMM_S5 : i32 > (a : m128i) -> m128i { static_assert_simm_bits ! (IMM_S5 , 5) ; unsafe { transmute (__lsx_vslti_w (transmute (a) , IMM_S5)) } }
}

macro_rules! lsx_vslti_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslti_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslti_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslti_d < const IMM_S5 : i32 > (a : m128i) -> m128i { static_assert_simm_bits ! (IMM_S5 , 5) ; unsafe { transmute (__lsx_vslti_d (transmute (a) , IMM_S5)) } }
}

macro_rules! lsx_vslt_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslt_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslt_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslt_bu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vslt_bu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vslt_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslt_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslt_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslt_hu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vslt_hu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vslt_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslt_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslt_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslt_wu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vslt_wu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vslt_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslt_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslt_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslt_du (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vslt_du (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vslti_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslti_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslti_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslti_bu < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vslti_bu (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vslti_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslti_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslti_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslti_hu < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vslti_hu (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vslti_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslti_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslti_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslti_wu < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vslti_wu (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vslti_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslti_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslti_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslti_du < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vslti_du (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vsle_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsle_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsle_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsle_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsle_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsle_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsle_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsle_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsle_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsle_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsle_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsle_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsle_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsle_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsle_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsle_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsle_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsle_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsle_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsle_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vslei_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslei_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslei_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslei_b < const IMM_S5 : i32 > (a : m128i) -> m128i { static_assert_simm_bits ! (IMM_S5 , 5) ; unsafe { transmute (__lsx_vslei_b (transmute (a) , IMM_S5)) } }
}

macro_rules! lsx_vslei_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslei_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslei_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslei_h < const IMM_S5 : i32 > (a : m128i) -> m128i { static_assert_simm_bits ! (IMM_S5 , 5) ; unsafe { transmute (__lsx_vslei_h (transmute (a) , IMM_S5)) } }
}

macro_rules! lsx_vslei_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslei_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslei_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslei_w < const IMM_S5 : i32 > (a : m128i) -> m128i { static_assert_simm_bits ! (IMM_S5 , 5) ; unsafe { transmute (__lsx_vslei_w (transmute (a) , IMM_S5)) } }
}

macro_rules! lsx_vslei_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslei_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslei_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslei_d < const IMM_S5 : i32 > (a : m128i) -> m128i { static_assert_simm_bits ! (IMM_S5 , 5) ; unsafe { transmute (__lsx_vslei_d (transmute (a) , IMM_S5)) } }
}

macro_rules! lsx_vsle_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsle_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsle_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsle_bu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsle_bu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsle_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsle_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsle_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsle_hu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsle_hu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsle_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsle_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsle_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsle_wu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsle_wu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsle_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsle_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsle_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsle_du (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsle_du (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vslei_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslei_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslei_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslei_bu < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vslei_bu (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vslei_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslei_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslei_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslei_hu < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vslei_hu (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vslei_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslei_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslei_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslei_wu < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vslei_wu (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vslei_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vslei_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vslei_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vslei_du < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vslei_du (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vsat_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsat_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsat_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsat_b < const IMM3 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM3 , 3) ; unsafe { transmute (__lsx_vsat_b (transmute (a) , IMM3)) } }
}

macro_rules! lsx_vsat_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsat_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsat_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsat_h < const IMM4 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vsat_h (transmute (a) , IMM4)) } }
}

macro_rules! lsx_vsat_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsat_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsat_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsat_w < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vsat_w (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vsat_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsat_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsat_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsat_d < const IMM6 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vsat_d (transmute (a) , IMM6)) } }
}

macro_rules! lsx_vsat_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsat_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsat_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsat_bu < const IMM3 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM3 , 3) ; unsafe { transmute (__lsx_vsat_bu (transmute (a) , IMM3)) } }
}

macro_rules! lsx_vsat_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsat_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsat_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsat_hu < const IMM4 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vsat_hu (transmute (a) , IMM4)) } }
}

macro_rules! lsx_vsat_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsat_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsat_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsat_wu < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vsat_wu (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vsat_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsat_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsat_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsat_du < const IMM6 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vsat_du (transmute (a) , IMM6)) } }
}

macro_rules! lsx_vadda_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vadda_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vadda_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vadda_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vadda_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vadda_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vadda_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vadda_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vadda_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vadda_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vadda_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vadda_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vadda_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vadda_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vadda_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vadda_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vadda_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vadda_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vadda_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vadda_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsadd_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsadd_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsadd_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsadd_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsadd_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsadd_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsadd_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsadd_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsadd_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsadd_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsadd_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsadd_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsadd_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsadd_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsadd_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsadd_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsadd_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsadd_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsadd_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsadd_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsadd_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsadd_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsadd_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsadd_bu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsadd_bu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsadd_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsadd_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsadd_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsadd_hu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsadd_hu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsadd_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsadd_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsadd_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsadd_wu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsadd_wu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsadd_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsadd_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsadd_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsadd_du (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsadd_du (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vavg_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vavg_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vavg_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vavg_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vavg_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vavg_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vavg_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vavg_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vavg_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vavg_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vavg_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vavg_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vavg_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vavg_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vavg_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vavg_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vavg_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vavg_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vavg_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vavg_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vavg_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vavg_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vavg_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vavg_bu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vavg_bu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vavg_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vavg_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vavg_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vavg_hu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vavg_hu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vavg_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vavg_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vavg_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vavg_wu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vavg_wu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vavg_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vavg_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vavg_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vavg_du (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vavg_du (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vavgr_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vavgr_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vavgr_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vavgr_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vavgr_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vavgr_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vavgr_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vavgr_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vavgr_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vavgr_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vavgr_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vavgr_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vavgr_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vavgr_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vavgr_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vavgr_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vavgr_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vavgr_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vavgr_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vavgr_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vavgr_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vavgr_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vavgr_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vavgr_bu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vavgr_bu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vavgr_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vavgr_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vavgr_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vavgr_hu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vavgr_hu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vavgr_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vavgr_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vavgr_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vavgr_wu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vavgr_wu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vavgr_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vavgr_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vavgr_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vavgr_du (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vavgr_du (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssub_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssub_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssub_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssub_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssub_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssub_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssub_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssub_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssub_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssub_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssub_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssub_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssub_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssub_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssub_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssub_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssub_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssub_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssub_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssub_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssub_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssub_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssub_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssub_bu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssub_bu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssub_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssub_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssub_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssub_hu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssub_hu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssub_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssub_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssub_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssub_wu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssub_wu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssub_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssub_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssub_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssub_du (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssub_du (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vabsd_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vabsd_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vabsd_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vabsd_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vabsd_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vabsd_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vabsd_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vabsd_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vabsd_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vabsd_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vabsd_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vabsd_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vabsd_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vabsd_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vabsd_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vabsd_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vabsd_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vabsd_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vabsd_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vabsd_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vabsd_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vabsd_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vabsd_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vabsd_bu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vabsd_bu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vabsd_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vabsd_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vabsd_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vabsd_hu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vabsd_hu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vabsd_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vabsd_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vabsd_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vabsd_wu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vabsd_wu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vabsd_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vabsd_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vabsd_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vabsd_du (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vabsd_du (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmul_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmul_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmul_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmul_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmul_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmul_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmul_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmul_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmul_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmul_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmul_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmul_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmul_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmul_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmul_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmul_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmul_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmul_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmul_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmul_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmadd_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmadd_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmadd_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmadd_b (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmadd_b (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmadd_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmadd_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmadd_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmadd_h (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmadd_h (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmadd_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmadd_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmadd_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmadd_w (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmadd_w (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmadd_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmadd_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmadd_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmadd_d (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmadd_d (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmsub_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmsub_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmsub_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmsub_b (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmsub_b (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmsub_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmsub_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmsub_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmsub_h (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmsub_h (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmsub_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmsub_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmsub_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmsub_w (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmsub_w (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmsub_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmsub_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmsub_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmsub_d (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmsub_d (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vdiv_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vdiv_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vdiv_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vdiv_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vdiv_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vdiv_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vdiv_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vdiv_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vdiv_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vdiv_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vdiv_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vdiv_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vdiv_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vdiv_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vdiv_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vdiv_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vdiv_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vdiv_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vdiv_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vdiv_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vdiv_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vdiv_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vdiv_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vdiv_bu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vdiv_bu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vdiv_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vdiv_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vdiv_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vdiv_hu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vdiv_hu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vdiv_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vdiv_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vdiv_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vdiv_wu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vdiv_wu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vdiv_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vdiv_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vdiv_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vdiv_du (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vdiv_du (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vhaddw_h_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vhaddw_h_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vhaddw_h_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vhaddw_h_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vhaddw_h_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vhaddw_w_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vhaddw_w_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vhaddw_w_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vhaddw_w_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vhaddw_w_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vhaddw_d_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vhaddw_d_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vhaddw_d_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vhaddw_d_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vhaddw_d_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vhaddw_hu_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vhaddw_hu_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vhaddw_hu_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vhaddw_hu_bu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vhaddw_hu_bu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vhaddw_wu_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vhaddw_wu_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vhaddw_wu_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vhaddw_wu_hu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vhaddw_wu_hu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vhaddw_du_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vhaddw_du_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vhaddw_du_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vhaddw_du_wu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vhaddw_du_wu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vhsubw_h_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vhsubw_h_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vhsubw_h_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vhsubw_h_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vhsubw_h_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vhsubw_w_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vhsubw_w_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vhsubw_w_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vhsubw_w_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vhsubw_w_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vhsubw_d_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vhsubw_d_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vhsubw_d_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vhsubw_d_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vhsubw_d_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vhsubw_hu_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vhsubw_hu_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vhsubw_hu_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vhsubw_hu_bu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vhsubw_hu_bu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vhsubw_wu_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vhsubw_wu_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vhsubw_wu_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vhsubw_wu_hu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vhsubw_wu_hu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vhsubw_du_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vhsubw_du_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vhsubw_du_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vhsubw_du_wu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vhsubw_du_wu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmod_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmod_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmod_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmod_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmod_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmod_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmod_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmod_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmod_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmod_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmod_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmod_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmod_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmod_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmod_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmod_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmod_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmod_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmod_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmod_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmod_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmod_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmod_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmod_bu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmod_bu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmod_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmod_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmod_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmod_hu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmod_hu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmod_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmod_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmod_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmod_wu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmod_wu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmod_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmod_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmod_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmod_du (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmod_du (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vreplve_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vreplve_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vreplve_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vreplve_b (a : m128i , b : i32) -> m128i { unsafe { transmute (__lsx_vreplve_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vreplve_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vreplve_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vreplve_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vreplve_h (a : m128i , b : i32) -> m128i { unsafe { transmute (__lsx_vreplve_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vreplve_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vreplve_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vreplve_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vreplve_w (a : m128i , b : i32) -> m128i { unsafe { transmute (__lsx_vreplve_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vreplve_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vreplve_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vreplve_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vreplve_d (a : m128i , b : i32) -> m128i { unsafe { transmute (__lsx_vreplve_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vreplvei_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vreplvei_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vreplvei_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vreplvei_b < const IMM4 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vreplvei_b (transmute (a) , IMM4)) } }
}

macro_rules! lsx_vreplvei_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vreplvei_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vreplvei_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vreplvei_h < const IMM3 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM3 , 3) ; unsafe { transmute (__lsx_vreplvei_h (transmute (a) , IMM3)) } }
}

macro_rules! lsx_vreplvei_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vreplvei_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vreplvei_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vreplvei_w < const IMM2 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM2 , 2) ; unsafe { transmute (__lsx_vreplvei_w (transmute (a) , IMM2)) } }
}

macro_rules! lsx_vreplvei_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vreplvei_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vreplvei_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vreplvei_d < const IMM1 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM1 , 1) ; unsafe { transmute (__lsx_vreplvei_d (transmute (a) , IMM1)) } }
}

macro_rules! lsx_vpickev_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpickev_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpickev_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpickev_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vpickev_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vpickev_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpickev_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpickev_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpickev_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vpickev_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vpickev_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpickev_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpickev_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpickev_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vpickev_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vpickev_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpickev_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpickev_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpickev_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vpickev_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vpickod_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpickod_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpickod_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpickod_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vpickod_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vpickod_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpickod_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpickod_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpickod_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vpickod_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vpickod_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpickod_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpickod_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpickod_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vpickod_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vpickod_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpickod_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpickod_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpickod_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vpickod_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vilvh_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vilvh_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vilvh_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vilvh_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vilvh_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vilvh_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vilvh_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vilvh_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vilvh_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vilvh_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vilvh_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vilvh_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vilvh_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vilvh_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vilvh_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vilvh_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vilvh_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vilvh_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vilvh_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vilvh_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vilvl_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vilvl_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vilvl_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vilvl_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vilvl_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vilvl_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vilvl_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vilvl_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vilvl_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vilvl_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vilvl_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vilvl_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vilvl_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vilvl_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vilvl_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vilvl_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vilvl_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vilvl_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vilvl_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vilvl_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vpackev_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpackev_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpackev_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpackev_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vpackev_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vpackev_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpackev_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpackev_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpackev_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vpackev_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vpackev_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpackev_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpackev_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpackev_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vpackev_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vpackev_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpackev_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpackev_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpackev_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vpackev_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vpackod_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpackod_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpackod_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpackod_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vpackod_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vpackod_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpackod_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpackod_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpackod_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vpackod_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vpackod_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpackod_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpackod_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpackod_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vpackod_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vpackod_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpackod_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpackod_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpackod_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vpackod_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vshuf_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vshuf_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vshuf_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vshuf_h (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vshuf_h (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vshuf_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vshuf_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vshuf_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vshuf_w (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vshuf_w (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vshuf_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vshuf_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vshuf_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vshuf_d (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vshuf_d (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vand_v_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vand_v in module {}", module_path!());
    };
}

mkfn!{
    lsx_vand_v_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vand_v (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vand_v (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vandi_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vandi_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vandi_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vandi_b < const IMM8 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { transmute (__lsx_vandi_b (transmute (a) , IMM8)) } }
}

macro_rules! lsx_vor_v_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vor_v in module {}", module_path!());
    };
}

mkfn!{
    lsx_vor_v_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vor_v (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vor_v (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vori_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vori_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vori_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vori_b < const IMM8 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { transmute (__lsx_vori_b (transmute (a) , IMM8)) } }
}

macro_rules! lsx_vnor_v_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vnor_v in module {}", module_path!());
    };
}

mkfn!{
    lsx_vnor_v_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vnor_v (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vnor_v (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vnori_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vnori_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vnori_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vnori_b < const IMM8 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { transmute (__lsx_vnori_b (transmute (a) , IMM8)) } }
}

macro_rules! lsx_vxor_v_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vxor_v in module {}", module_path!());
    };
}

mkfn!{
    lsx_vxor_v_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vxor_v (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vxor_v (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vxori_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vxori_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vxori_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vxori_b < const IMM8 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { transmute (__lsx_vxori_b (transmute (a) , IMM8)) } }
}

macro_rules! lsx_vbitsel_v_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitsel_v in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitsel_v_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitsel_v (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vbitsel_v (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vbitseli_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbitseli_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbitseli_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbitseli_b < const IMM8 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { transmute (__lsx_vbitseli_b (transmute (a) , transmute (b) , IMM8)) } }
}

macro_rules! lsx_vshuf4i_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vshuf4i_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vshuf4i_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vshuf4i_b < const IMM8 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { transmute (__lsx_vshuf4i_b (transmute (a) , IMM8)) } }
}

macro_rules! lsx_vshuf4i_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vshuf4i_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vshuf4i_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vshuf4i_h < const IMM8 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { transmute (__lsx_vshuf4i_h (transmute (a) , IMM8)) } }
}

macro_rules! lsx_vshuf4i_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vshuf4i_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vshuf4i_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vshuf4i_w < const IMM8 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { transmute (__lsx_vshuf4i_w (transmute (a) , IMM8)) } }
}

macro_rules! lsx_vreplgr2vr_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vreplgr2vr_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vreplgr2vr_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vreplgr2vr_b (a : i32) -> m128i { unsafe { transmute (__lsx_vreplgr2vr_b (transmute (a))) } }
}

macro_rules! lsx_vreplgr2vr_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vreplgr2vr_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vreplgr2vr_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vreplgr2vr_h (a : i32) -> m128i { unsafe { transmute (__lsx_vreplgr2vr_h (transmute (a))) } }
}

macro_rules! lsx_vreplgr2vr_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vreplgr2vr_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vreplgr2vr_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vreplgr2vr_w (a : i32) -> m128i { unsafe { transmute (__lsx_vreplgr2vr_w (transmute (a))) } }
}

macro_rules! lsx_vreplgr2vr_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vreplgr2vr_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vreplgr2vr_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vreplgr2vr_d (a : i64) -> m128i { unsafe { transmute (__lsx_vreplgr2vr_d (transmute (a))) } }
}

macro_rules! lsx_vpcnt_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpcnt_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpcnt_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpcnt_b (a : m128i) -> m128i { unsafe { transmute (__lsx_vpcnt_b (transmute (a))) } }
}

macro_rules! lsx_vpcnt_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpcnt_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpcnt_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpcnt_h (a : m128i) -> m128i { unsafe { transmute (__lsx_vpcnt_h (transmute (a))) } }
}

macro_rules! lsx_vpcnt_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpcnt_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpcnt_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpcnt_w (a : m128i) -> m128i { unsafe { transmute (__lsx_vpcnt_w (transmute (a))) } }
}

macro_rules! lsx_vpcnt_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpcnt_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpcnt_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpcnt_d (a : m128i) -> m128i { unsafe { transmute (__lsx_vpcnt_d (transmute (a))) } }
}

macro_rules! lsx_vclo_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vclo_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vclo_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vclo_b (a : m128i) -> m128i { unsafe { transmute (__lsx_vclo_b (transmute (a))) } }
}

macro_rules! lsx_vclo_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vclo_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vclo_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vclo_h (a : m128i) -> m128i { unsafe { transmute (__lsx_vclo_h (transmute (a))) } }
}

macro_rules! lsx_vclo_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vclo_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vclo_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vclo_w (a : m128i) -> m128i { unsafe { transmute (__lsx_vclo_w (transmute (a))) } }
}

macro_rules! lsx_vclo_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vclo_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vclo_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vclo_d (a : m128i) -> m128i { unsafe { transmute (__lsx_vclo_d (transmute (a))) } }
}

macro_rules! lsx_vclz_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vclz_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vclz_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vclz_b (a : m128i) -> m128i { unsafe { transmute (__lsx_vclz_b (transmute (a))) } }
}

macro_rules! lsx_vclz_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vclz_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vclz_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vclz_h (a : m128i) -> m128i { unsafe { transmute (__lsx_vclz_h (transmute (a))) } }
}

macro_rules! lsx_vclz_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vclz_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vclz_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vclz_w (a : m128i) -> m128i { unsafe { transmute (__lsx_vclz_w (transmute (a))) } }
}

macro_rules! lsx_vclz_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vclz_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vclz_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vclz_d (a : m128i) -> m128i { unsafe { transmute (__lsx_vclz_d (transmute (a))) } }
}

macro_rules! lsx_vpickve2gr_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpickve2gr_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpickve2gr_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpickve2gr_b < const IMM4 : u32 > (a : m128i) -> i32 { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vpickve2gr_b (transmute (a) , IMM4)) } }
}

macro_rules! lsx_vpickve2gr_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpickve2gr_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpickve2gr_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpickve2gr_h < const IMM3 : u32 > (a : m128i) -> i32 { static_assert_uimm_bits ! (IMM3 , 3) ; unsafe { transmute (__lsx_vpickve2gr_h (transmute (a) , IMM3)) } }
}

macro_rules! lsx_vpickve2gr_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpickve2gr_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpickve2gr_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpickve2gr_w < const IMM2 : u32 > (a : m128i) -> i32 { static_assert_uimm_bits ! (IMM2 , 2) ; unsafe { transmute (__lsx_vpickve2gr_w (transmute (a) , IMM2)) } }
}

macro_rules! lsx_vpickve2gr_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpickve2gr_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpickve2gr_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpickve2gr_d < const IMM1 : u32 > (a : m128i) -> i64 { static_assert_uimm_bits ! (IMM1 , 1) ; unsafe { transmute (__lsx_vpickve2gr_d (transmute (a) , IMM1)) } }
}

macro_rules! lsx_vpickve2gr_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpickve2gr_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpickve2gr_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpickve2gr_bu < const IMM4 : u32 > (a : m128i) -> u32 { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vpickve2gr_bu (transmute (a) , IMM4)) } }
}

macro_rules! lsx_vpickve2gr_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpickve2gr_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpickve2gr_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpickve2gr_hu < const IMM3 : u32 > (a : m128i) -> u32 { static_assert_uimm_bits ! (IMM3 , 3) ; unsafe { transmute (__lsx_vpickve2gr_hu (transmute (a) , IMM3)) } }
}

macro_rules! lsx_vpickve2gr_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpickve2gr_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpickve2gr_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpickve2gr_wu < const IMM2 : u32 > (a : m128i) -> u32 { static_assert_uimm_bits ! (IMM2 , 2) ; unsafe { transmute (__lsx_vpickve2gr_wu (transmute (a) , IMM2)) } }
}

macro_rules! lsx_vpickve2gr_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpickve2gr_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpickve2gr_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpickve2gr_du < const IMM1 : u32 > (a : m128i) -> u64 { static_assert_uimm_bits ! (IMM1 , 1) ; unsafe { transmute (__lsx_vpickve2gr_du (transmute (a) , IMM1)) } }
}

macro_rules! lsx_vinsgr2vr_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vinsgr2vr_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vinsgr2vr_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vinsgr2vr_b < const IMM4 : u32 > (a : m128i , b : i32) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vinsgr2vr_b (transmute (a) , transmute (b) , IMM4)) } }
}

macro_rules! lsx_vinsgr2vr_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vinsgr2vr_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vinsgr2vr_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vinsgr2vr_h < const IMM3 : u32 > (a : m128i , b : i32) -> m128i { static_assert_uimm_bits ! (IMM3 , 3) ; unsafe { transmute (__lsx_vinsgr2vr_h (transmute (a) , transmute (b) , IMM3)) } }
}

macro_rules! lsx_vinsgr2vr_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vinsgr2vr_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vinsgr2vr_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vinsgr2vr_w < const IMM2 : u32 > (a : m128i , b : i32) -> m128i { static_assert_uimm_bits ! (IMM2 , 2) ; unsafe { transmute (__lsx_vinsgr2vr_w (transmute (a) , transmute (b) , IMM2)) } }
}

macro_rules! lsx_vinsgr2vr_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vinsgr2vr_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vinsgr2vr_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vinsgr2vr_d < const IMM1 : u32 > (a : m128i , b : i64) -> m128i { static_assert_uimm_bits ! (IMM1 , 1) ; unsafe { transmute (__lsx_vinsgr2vr_d (transmute (a) , transmute (b) , IMM1)) } }
}

macro_rules! lsx_vfadd_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfadd_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfadd_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfadd_s (a : m128 , b : m128) -> m128 { unsafe { transmute (__lsx_vfadd_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfadd_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfadd_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfadd_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfadd_d (a : m128d , b : m128d) -> m128d { unsafe { transmute (__lsx_vfadd_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfsub_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfsub_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfsub_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfsub_s (a : m128 , b : m128) -> m128 { unsafe { transmute (__lsx_vfsub_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfsub_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfsub_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfsub_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfsub_d (a : m128d , b : m128d) -> m128d { unsafe { transmute (__lsx_vfsub_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfmul_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfmul_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfmul_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfmul_s (a : m128 , b : m128) -> m128 { unsafe { transmute (__lsx_vfmul_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfmul_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfmul_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfmul_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfmul_d (a : m128d , b : m128d) -> m128d { unsafe { transmute (__lsx_vfmul_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfdiv_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfdiv_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfdiv_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfdiv_s (a : m128 , b : m128) -> m128 { unsafe { transmute (__lsx_vfdiv_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfdiv_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfdiv_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfdiv_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfdiv_d (a : m128d , b : m128d) -> m128d { unsafe { transmute (__lsx_vfdiv_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcvt_h_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcvt_h_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcvt_h_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcvt_h_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcvt_h_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcvt_s_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcvt_s_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcvt_s_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcvt_s_d (a : m128d , b : m128d) -> m128 { unsafe { transmute (__lsx_vfcvt_s_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfmin_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfmin_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfmin_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfmin_s (a : m128 , b : m128) -> m128 { unsafe { transmute (__lsx_vfmin_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfmin_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfmin_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfmin_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfmin_d (a : m128d , b : m128d) -> m128d { unsafe { transmute (__lsx_vfmin_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfmina_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfmina_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfmina_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfmina_s (a : m128 , b : m128) -> m128 { unsafe { transmute (__lsx_vfmina_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfmina_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfmina_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfmina_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfmina_d (a : m128d , b : m128d) -> m128d { unsafe { transmute (__lsx_vfmina_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfmax_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfmax_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfmax_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfmax_s (a : m128 , b : m128) -> m128 { unsafe { transmute (__lsx_vfmax_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfmax_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfmax_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfmax_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfmax_d (a : m128d , b : m128d) -> m128d { unsafe { transmute (__lsx_vfmax_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfmaxa_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfmaxa_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfmaxa_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfmaxa_s (a : m128 , b : m128) -> m128 { unsafe { transmute (__lsx_vfmaxa_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfmaxa_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfmaxa_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfmaxa_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfmaxa_d (a : m128d , b : m128d) -> m128d { unsafe { transmute (__lsx_vfmaxa_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfclass_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfclass_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfclass_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfclass_s (a : m128) -> m128i { unsafe { transmute (__lsx_vfclass_s (transmute (a))) } }
}

macro_rules! lsx_vfclass_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfclass_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfclass_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfclass_d (a : m128d) -> m128i { unsafe { transmute (__lsx_vfclass_d (transmute (a))) } }
}

macro_rules! lsx_vfsqrt_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfsqrt_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfsqrt_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfsqrt_s (a : m128) -> m128 { unsafe { transmute (__lsx_vfsqrt_s (transmute (a))) } }
}

macro_rules! lsx_vfsqrt_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfsqrt_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfsqrt_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfsqrt_d (a : m128d) -> m128d { unsafe { transmute (__lsx_vfsqrt_d (transmute (a))) } }
}

macro_rules! lsx_vfrecip_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrecip_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrecip_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrecip_s (a : m128) -> m128 { unsafe { transmute (__lsx_vfrecip_s (transmute (a))) } }
}

macro_rules! lsx_vfrecip_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrecip_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrecip_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrecip_d (a : m128d) -> m128d { unsafe { transmute (__lsx_vfrecip_d (transmute (a))) } }
}

macro_rules! lsx_vfrecipe_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrecipe_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrecipe_s_introspect!();
    #[inline] #[target_feature (enable = "lsx,frecipe")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrecipe_s (a : m128) -> m128 { unsafe { transmute (__lsx_vfrecipe_s (transmute (a))) } }
}

macro_rules! lsx_vfrecipe_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrecipe_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrecipe_d_introspect!();
    #[inline] #[target_feature (enable = "lsx,frecipe")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrecipe_d (a : m128d) -> m128d { unsafe { transmute (__lsx_vfrecipe_d (transmute (a))) } }
}

macro_rules! lsx_vfrsqrte_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrsqrte_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrsqrte_s_introspect!();
    #[inline] #[target_feature (enable = "lsx,frecipe")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrsqrte_s (a : m128) -> m128 { unsafe { transmute (__lsx_vfrsqrte_s (transmute (a))) } }
}

macro_rules! lsx_vfrsqrte_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrsqrte_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrsqrte_d_introspect!();
    #[inline] #[target_feature (enable = "lsx,frecipe")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrsqrte_d (a : m128d) -> m128d { unsafe { transmute (__lsx_vfrsqrte_d (transmute (a))) } }
}

macro_rules! lsx_vfrint_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrint_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrint_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrint_s (a : m128) -> m128 { unsafe { transmute (__lsx_vfrint_s (transmute (a))) } }
}

macro_rules! lsx_vfrint_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrint_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrint_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrint_d (a : m128d) -> m128d { unsafe { transmute (__lsx_vfrint_d (transmute (a))) } }
}

macro_rules! lsx_vfrsqrt_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrsqrt_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrsqrt_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrsqrt_s (a : m128) -> m128 { unsafe { transmute (__lsx_vfrsqrt_s (transmute (a))) } }
}

macro_rules! lsx_vfrsqrt_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrsqrt_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrsqrt_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrsqrt_d (a : m128d) -> m128d { unsafe { transmute (__lsx_vfrsqrt_d (transmute (a))) } }
}

macro_rules! lsx_vflogb_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vflogb_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vflogb_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vflogb_s (a : m128) -> m128 { unsafe { transmute (__lsx_vflogb_s (transmute (a))) } }
}

macro_rules! lsx_vflogb_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vflogb_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vflogb_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vflogb_d (a : m128d) -> m128d { unsafe { transmute (__lsx_vflogb_d (transmute (a))) } }
}

macro_rules! lsx_vfcvth_s_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcvth_s_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcvth_s_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcvth_s_h (a : m128i) -> m128 { unsafe { transmute (__lsx_vfcvth_s_h (transmute (a))) } }
}

macro_rules! lsx_vfcvth_d_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcvth_d_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcvth_d_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcvth_d_s (a : m128) -> m128d { unsafe { transmute (__lsx_vfcvth_d_s (transmute (a))) } }
}

macro_rules! lsx_vfcvtl_s_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcvtl_s_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcvtl_s_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcvtl_s_h (a : m128i) -> m128 { unsafe { transmute (__lsx_vfcvtl_s_h (transmute (a))) } }
}

macro_rules! lsx_vfcvtl_d_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcvtl_d_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcvtl_d_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcvtl_d_s (a : m128) -> m128d { unsafe { transmute (__lsx_vfcvtl_d_s (transmute (a))) } }
}

macro_rules! lsx_vftint_w_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftint_w_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftint_w_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftint_w_s (a : m128) -> m128i { unsafe { transmute (__lsx_vftint_w_s (transmute (a))) } }
}

macro_rules! lsx_vftint_l_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftint_l_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftint_l_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftint_l_d (a : m128d) -> m128i { unsafe { transmute (__lsx_vftint_l_d (transmute (a))) } }
}

macro_rules! lsx_vftint_wu_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftint_wu_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftint_wu_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftint_wu_s (a : m128) -> m128i { unsafe { transmute (__lsx_vftint_wu_s (transmute (a))) } }
}

macro_rules! lsx_vftint_lu_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftint_lu_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftint_lu_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftint_lu_d (a : m128d) -> m128i { unsafe { transmute (__lsx_vftint_lu_d (transmute (a))) } }
}

macro_rules! lsx_vftintrz_w_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrz_w_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrz_w_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrz_w_s (a : m128) -> m128i { unsafe { transmute (__lsx_vftintrz_w_s (transmute (a))) } }
}

macro_rules! lsx_vftintrz_l_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrz_l_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrz_l_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrz_l_d (a : m128d) -> m128i { unsafe { transmute (__lsx_vftintrz_l_d (transmute (a))) } }
}

macro_rules! lsx_vftintrz_wu_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrz_wu_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrz_wu_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrz_wu_s (a : m128) -> m128i { unsafe { transmute (__lsx_vftintrz_wu_s (transmute (a))) } }
}

macro_rules! lsx_vftintrz_lu_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrz_lu_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrz_lu_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrz_lu_d (a : m128d) -> m128i { unsafe { transmute (__lsx_vftintrz_lu_d (transmute (a))) } }
}

macro_rules! lsx_vffint_s_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vffint_s_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vffint_s_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vffint_s_w (a : m128i) -> m128 { unsafe { transmute (__lsx_vffint_s_w (transmute (a))) } }
}

macro_rules! lsx_vffint_d_l_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vffint_d_l in module {}", module_path!());
    };
}

mkfn!{
    lsx_vffint_d_l_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vffint_d_l (a : m128i) -> m128d { unsafe { transmute (__lsx_vffint_d_l (transmute (a))) } }
}

macro_rules! lsx_vffint_s_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vffint_s_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vffint_s_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vffint_s_wu (a : m128i) -> m128 { unsafe { transmute (__lsx_vffint_s_wu (transmute (a))) } }
}

macro_rules! lsx_vffint_d_lu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vffint_d_lu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vffint_d_lu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vffint_d_lu (a : m128i) -> m128d { unsafe { transmute (__lsx_vffint_d_lu (transmute (a))) } }
}

macro_rules! lsx_vandn_v_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vandn_v in module {}", module_path!());
    };
}

mkfn!{
    lsx_vandn_v_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vandn_v (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vandn_v (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vneg_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vneg_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vneg_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vneg_b (a : m128i) -> m128i { unsafe { transmute (__lsx_vneg_b (transmute (a))) } }
}

macro_rules! lsx_vneg_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vneg_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vneg_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vneg_h (a : m128i) -> m128i { unsafe { transmute (__lsx_vneg_h (transmute (a))) } }
}

macro_rules! lsx_vneg_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vneg_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vneg_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vneg_w (a : m128i) -> m128i { unsafe { transmute (__lsx_vneg_w (transmute (a))) } }
}

macro_rules! lsx_vneg_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vneg_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vneg_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vneg_d (a : m128i) -> m128i { unsafe { transmute (__lsx_vneg_d (transmute (a))) } }
}

macro_rules! lsx_vmuh_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmuh_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmuh_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmuh_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmuh_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmuh_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmuh_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmuh_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmuh_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmuh_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmuh_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmuh_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmuh_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmuh_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmuh_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmuh_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmuh_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmuh_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmuh_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmuh_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmuh_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmuh_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmuh_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmuh_bu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmuh_bu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmuh_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmuh_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmuh_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmuh_hu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmuh_hu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmuh_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmuh_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmuh_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmuh_wu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmuh_wu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmuh_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmuh_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmuh_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmuh_du (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmuh_du (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsllwil_h_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsllwil_h_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsllwil_h_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsllwil_h_b < const IMM3 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM3 , 3) ; unsafe { transmute (__lsx_vsllwil_h_b (transmute (a) , IMM3)) } }
}

macro_rules! lsx_vsllwil_w_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsllwil_w_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsllwil_w_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsllwil_w_h < const IMM4 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vsllwil_w_h (transmute (a) , IMM4)) } }
}

macro_rules! lsx_vsllwil_d_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsllwil_d_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsllwil_d_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsllwil_d_w < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vsllwil_d_w (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vsllwil_hu_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsllwil_hu_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsllwil_hu_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsllwil_hu_bu < const IMM3 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM3 , 3) ; unsafe { transmute (__lsx_vsllwil_hu_bu (transmute (a) , IMM3)) } }
}

macro_rules! lsx_vsllwil_wu_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsllwil_wu_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsllwil_wu_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsllwil_wu_hu < const IMM4 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vsllwil_wu_hu (transmute (a) , IMM4)) } }
}

macro_rules! lsx_vsllwil_du_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsllwil_du_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsllwil_du_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsllwil_du_wu < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vsllwil_du_wu (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vsran_b_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsran_b_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsran_b_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsran_b_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsran_b_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsran_h_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsran_h_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsran_h_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsran_h_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsran_h_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsran_w_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsran_w_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsran_w_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsran_w_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsran_w_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssran_b_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssran_b_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssran_b_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssran_b_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssran_b_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssran_h_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssran_h_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssran_h_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssran_h_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssran_h_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssran_w_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssran_w_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssran_w_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssran_w_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssran_w_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssran_bu_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssran_bu_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssran_bu_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssran_bu_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssran_bu_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssran_hu_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssran_hu_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssran_hu_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssran_hu_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssran_hu_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssran_wu_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssran_wu_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssran_wu_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssran_wu_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssran_wu_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrarn_b_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrarn_b_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrarn_b_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrarn_b_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsrarn_b_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrarn_h_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrarn_h_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrarn_h_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrarn_h_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsrarn_h_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrarn_w_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrarn_w_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrarn_w_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrarn_w_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsrarn_w_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssrarn_b_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrarn_b_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrarn_b_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrarn_b_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssrarn_b_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssrarn_h_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrarn_h_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrarn_h_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrarn_h_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssrarn_h_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssrarn_w_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrarn_w_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrarn_w_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrarn_w_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssrarn_w_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssrarn_bu_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrarn_bu_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrarn_bu_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrarn_bu_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssrarn_bu_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssrarn_hu_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrarn_hu_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrarn_hu_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrarn_hu_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssrarn_hu_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssrarn_wu_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrarn_wu_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrarn_wu_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrarn_wu_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssrarn_wu_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrln_b_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrln_b_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrln_b_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrln_b_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsrln_b_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrln_h_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrln_h_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrln_h_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrln_h_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsrln_h_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrln_w_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrln_w_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrln_w_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrln_w_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsrln_w_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssrln_bu_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrln_bu_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrln_bu_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrln_bu_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssrln_bu_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssrln_hu_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrln_hu_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrln_hu_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrln_hu_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssrln_hu_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssrln_wu_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrln_wu_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrln_wu_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrln_wu_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssrln_wu_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrlrn_b_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrlrn_b_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrlrn_b_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrlrn_b_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsrlrn_b_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrlrn_h_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrlrn_h_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrlrn_h_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrlrn_h_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsrlrn_h_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsrlrn_w_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrlrn_w_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrlrn_w_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrlrn_w_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsrlrn_w_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssrlrn_bu_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlrn_bu_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlrn_bu_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlrn_bu_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssrlrn_bu_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssrlrn_hu_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlrn_hu_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlrn_hu_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlrn_hu_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssrlrn_hu_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssrlrn_wu_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlrn_wu_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlrn_wu_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlrn_wu_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssrlrn_wu_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfrstpi_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrstpi_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrstpi_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrstpi_b < const IMM5 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vfrstpi_b (transmute (a) , transmute (b) , IMM5)) } }
}

macro_rules! lsx_vfrstpi_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrstpi_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrstpi_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrstpi_h < const IMM5 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vfrstpi_h (transmute (a) , transmute (b) , IMM5)) } }
}

macro_rules! lsx_vfrstp_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrstp_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrstp_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrstp_b (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vfrstp_b (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vfrstp_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrstp_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrstp_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrstp_h (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vfrstp_h (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vshuf4i_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vshuf4i_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vshuf4i_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vshuf4i_d < const IMM8 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { transmute (__lsx_vshuf4i_d (transmute (a) , transmute (b) , IMM8)) } }
}

macro_rules! lsx_vbsrl_v_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbsrl_v in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbsrl_v_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbsrl_v < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vbsrl_v (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vbsll_v_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vbsll_v in module {}", module_path!());
    };
}

mkfn!{
    lsx_vbsll_v_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vbsll_v < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vbsll_v (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vextrins_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vextrins_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vextrins_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vextrins_b < const IMM8 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { transmute (__lsx_vextrins_b (transmute (a) , transmute (b) , IMM8)) } }
}

macro_rules! lsx_vextrins_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vextrins_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vextrins_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vextrins_h < const IMM8 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { transmute (__lsx_vextrins_h (transmute (a) , transmute (b) , IMM8)) } }
}

macro_rules! lsx_vextrins_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vextrins_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vextrins_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vextrins_w < const IMM8 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { transmute (__lsx_vextrins_w (transmute (a) , transmute (b) , IMM8)) } }
}

macro_rules! lsx_vextrins_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vextrins_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vextrins_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vextrins_d < const IMM8 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { transmute (__lsx_vextrins_d (transmute (a) , transmute (b) , IMM8)) } }
}

macro_rules! lsx_vmskltz_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmskltz_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmskltz_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmskltz_b (a : m128i) -> m128i { unsafe { transmute (__lsx_vmskltz_b (transmute (a))) } }
}

macro_rules! lsx_vmskltz_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmskltz_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmskltz_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmskltz_h (a : m128i) -> m128i { unsafe { transmute (__lsx_vmskltz_h (transmute (a))) } }
}

macro_rules! lsx_vmskltz_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmskltz_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmskltz_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmskltz_w (a : m128i) -> m128i { unsafe { transmute (__lsx_vmskltz_w (transmute (a))) } }
}

macro_rules! lsx_vmskltz_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmskltz_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmskltz_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmskltz_d (a : m128i) -> m128i { unsafe { transmute (__lsx_vmskltz_d (transmute (a))) } }
}

macro_rules! lsx_vsigncov_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsigncov_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsigncov_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsigncov_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsigncov_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsigncov_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsigncov_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsigncov_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsigncov_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsigncov_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsigncov_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsigncov_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsigncov_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsigncov_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsigncov_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsigncov_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsigncov_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsigncov_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsigncov_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsigncov_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfmadd_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfmadd_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfmadd_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfmadd_s (a : m128 , b : m128 , c : m128) -> m128 { unsafe { transmute (__lsx_vfmadd_s (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vfmadd_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfmadd_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfmadd_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfmadd_d (a : m128d , b : m128d , c : m128d) -> m128d { unsafe { transmute (__lsx_vfmadd_d (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vfmsub_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfmsub_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfmsub_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfmsub_s (a : m128 , b : m128 , c : m128) -> m128 { unsafe { transmute (__lsx_vfmsub_s (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vfmsub_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfmsub_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfmsub_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfmsub_d (a : m128d , b : m128d , c : m128d) -> m128d { unsafe { transmute (__lsx_vfmsub_d (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vfnmadd_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfnmadd_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfnmadd_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfnmadd_s (a : m128 , b : m128 , c : m128) -> m128 { unsafe { transmute (__lsx_vfnmadd_s (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vfnmadd_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfnmadd_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfnmadd_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfnmadd_d (a : m128d , b : m128d , c : m128d) -> m128d { unsafe { transmute (__lsx_vfnmadd_d (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vfnmsub_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfnmsub_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfnmsub_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfnmsub_s (a : m128 , b : m128 , c : m128) -> m128 { unsafe { transmute (__lsx_vfnmsub_s (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vfnmsub_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfnmsub_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfnmsub_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfnmsub_d (a : m128d , b : m128d , c : m128d) -> m128d { unsafe { transmute (__lsx_vfnmsub_d (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vftintrne_w_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrne_w_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrne_w_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrne_w_s (a : m128) -> m128i { unsafe { transmute (__lsx_vftintrne_w_s (transmute (a))) } }
}

macro_rules! lsx_vftintrne_l_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrne_l_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrne_l_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrne_l_d (a : m128d) -> m128i { unsafe { transmute (__lsx_vftintrne_l_d (transmute (a))) } }
}

macro_rules! lsx_vftintrp_w_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrp_w_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrp_w_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrp_w_s (a : m128) -> m128i { unsafe { transmute (__lsx_vftintrp_w_s (transmute (a))) } }
}

macro_rules! lsx_vftintrp_l_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrp_l_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrp_l_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrp_l_d (a : m128d) -> m128i { unsafe { transmute (__lsx_vftintrp_l_d (transmute (a))) } }
}

macro_rules! lsx_vftintrm_w_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrm_w_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrm_w_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrm_w_s (a : m128) -> m128i { unsafe { transmute (__lsx_vftintrm_w_s (transmute (a))) } }
}

macro_rules! lsx_vftintrm_l_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrm_l_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrm_l_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrm_l_d (a : m128d) -> m128i { unsafe { transmute (__lsx_vftintrm_l_d (transmute (a))) } }
}

macro_rules! lsx_vftint_w_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftint_w_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftint_w_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftint_w_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vftint_w_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vffint_s_l_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vffint_s_l in module {}", module_path!());
    };
}

mkfn!{
    lsx_vffint_s_l_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vffint_s_l (a : m128i , b : m128i) -> m128 { unsafe { transmute (__lsx_vffint_s_l (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vftintrz_w_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrz_w_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrz_w_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrz_w_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vftintrz_w_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vftintrp_w_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrp_w_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrp_w_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrp_w_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vftintrp_w_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vftintrm_w_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrm_w_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrm_w_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrm_w_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vftintrm_w_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vftintrne_w_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrne_w_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrne_w_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrne_w_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vftintrne_w_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vftintl_l_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintl_l_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintl_l_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintl_l_s (a : m128) -> m128i { unsafe { transmute (__lsx_vftintl_l_s (transmute (a))) } }
}

macro_rules! lsx_vftinth_l_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftinth_l_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftinth_l_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftinth_l_s (a : m128) -> m128i { unsafe { transmute (__lsx_vftinth_l_s (transmute (a))) } }
}

macro_rules! lsx_vffinth_d_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vffinth_d_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vffinth_d_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vffinth_d_w (a : m128i) -> m128d { unsafe { transmute (__lsx_vffinth_d_w (transmute (a))) } }
}

macro_rules! lsx_vffintl_d_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vffintl_d_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vffintl_d_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vffintl_d_w (a : m128i) -> m128d { unsafe { transmute (__lsx_vffintl_d_w (transmute (a))) } }
}

macro_rules! lsx_vftintrzl_l_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrzl_l_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrzl_l_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrzl_l_s (a : m128) -> m128i { unsafe { transmute (__lsx_vftintrzl_l_s (transmute (a))) } }
}

macro_rules! lsx_vftintrzh_l_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrzh_l_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrzh_l_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrzh_l_s (a : m128) -> m128i { unsafe { transmute (__lsx_vftintrzh_l_s (transmute (a))) } }
}

macro_rules! lsx_vftintrpl_l_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrpl_l_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrpl_l_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrpl_l_s (a : m128) -> m128i { unsafe { transmute (__lsx_vftintrpl_l_s (transmute (a))) } }
}

macro_rules! lsx_vftintrph_l_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrph_l_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrph_l_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrph_l_s (a : m128) -> m128i { unsafe { transmute (__lsx_vftintrph_l_s (transmute (a))) } }
}

macro_rules! lsx_vftintrml_l_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrml_l_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrml_l_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrml_l_s (a : m128) -> m128i { unsafe { transmute (__lsx_vftintrml_l_s (transmute (a))) } }
}

macro_rules! lsx_vftintrmh_l_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrmh_l_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrmh_l_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrmh_l_s (a : m128) -> m128i { unsafe { transmute (__lsx_vftintrmh_l_s (transmute (a))) } }
}

macro_rules! lsx_vftintrnel_l_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrnel_l_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrnel_l_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrnel_l_s (a : m128) -> m128i { unsafe { transmute (__lsx_vftintrnel_l_s (transmute (a))) } }
}

macro_rules! lsx_vftintrneh_l_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vftintrneh_l_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vftintrneh_l_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vftintrneh_l_s (a : m128) -> m128i { unsafe { transmute (__lsx_vftintrneh_l_s (transmute (a))) } }
}

macro_rules! lsx_vfrintrne_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrintrne_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrintrne_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrintrne_s (a : m128) -> m128 { unsafe { transmute (__lsx_vfrintrne_s (transmute (a))) } }
}

macro_rules! lsx_vfrintrne_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrintrne_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrintrne_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrintrne_d (a : m128d) -> m128d { unsafe { transmute (__lsx_vfrintrne_d (transmute (a))) } }
}

macro_rules! lsx_vfrintrz_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrintrz_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrintrz_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrintrz_s (a : m128) -> m128 { unsafe { transmute (__lsx_vfrintrz_s (transmute (a))) } }
}

macro_rules! lsx_vfrintrz_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrintrz_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrintrz_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrintrz_d (a : m128d) -> m128d { unsafe { transmute (__lsx_vfrintrz_d (transmute (a))) } }
}

macro_rules! lsx_vfrintrp_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrintrp_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrintrp_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrintrp_s (a : m128) -> m128 { unsafe { transmute (__lsx_vfrintrp_s (transmute (a))) } }
}

macro_rules! lsx_vfrintrp_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrintrp_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrintrp_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrintrp_d (a : m128d) -> m128d { unsafe { transmute (__lsx_vfrintrp_d (transmute (a))) } }
}

macro_rules! lsx_vfrintrm_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrintrm_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrintrm_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrintrm_s (a : m128) -> m128 { unsafe { transmute (__lsx_vfrintrm_s (transmute (a))) } }
}

macro_rules! lsx_vfrintrm_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfrintrm_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfrintrm_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfrintrm_d (a : m128d) -> m128d { unsafe { transmute (__lsx_vfrintrm_d (transmute (a))) } }
}

macro_rules! lsx_vstelm_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vstelm_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vstelm_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2 , 3)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn lsx_vstelm_b < const IMM_S8 : i32 , const IMM4 : u32 > (a : m128i , mem_addr : * mut i8) { static_assert_simm_bits ! (IMM_S8 , 8) ; static_assert_uimm_bits ! (IMM4 , 4) ; transmute (__lsx_vstelm_b (transmute (a) , mem_addr , IMM_S8 , IMM4)) }
}

macro_rules! lsx_vstelm_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vstelm_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vstelm_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2 , 3)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn lsx_vstelm_h < const IMM_S8 : i32 , const IMM3 : u32 > (a : m128i , mem_addr : * mut i8) { static_assert_simm_bits ! (IMM_S8 , 8) ; static_assert_uimm_bits ! (IMM3 , 3) ; transmute (__lsx_vstelm_h (transmute (a) , mem_addr , IMM_S8 , IMM3)) }
}

macro_rules! lsx_vstelm_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vstelm_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vstelm_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2 , 3)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn lsx_vstelm_w < const IMM_S8 : i32 , const IMM2 : u32 > (a : m128i , mem_addr : * mut i8) { static_assert_simm_bits ! (IMM_S8 , 8) ; static_assert_uimm_bits ! (IMM2 , 2) ; transmute (__lsx_vstelm_w (transmute (a) , mem_addr , IMM_S8 , IMM2)) }
}

macro_rules! lsx_vstelm_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vstelm_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vstelm_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2 , 3)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn lsx_vstelm_d < const IMM_S8 : i32 , const IMM1 : u32 > (a : m128i , mem_addr : * mut i8) { static_assert_simm_bits ! (IMM_S8 , 8) ; static_assert_uimm_bits ! (IMM1 , 1) ; transmute (__lsx_vstelm_d (transmute (a) , mem_addr , IMM_S8 , IMM1)) }
}

macro_rules! lsx_vaddwev_d_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwev_d_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwev_d_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwev_d_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwev_d_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwev_w_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwev_w_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwev_w_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwev_w_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwev_w_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwev_h_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwev_h_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwev_h_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwev_h_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwev_h_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwod_d_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwod_d_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwod_d_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwod_d_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwod_d_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwod_w_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwod_w_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwod_w_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwod_w_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwod_w_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwod_h_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwod_h_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwod_h_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwod_h_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwod_h_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwev_d_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwev_d_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwev_d_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwev_d_wu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwev_d_wu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwev_w_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwev_w_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwev_w_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwev_w_hu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwev_w_hu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwev_h_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwev_h_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwev_h_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwev_h_bu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwev_h_bu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwod_d_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwod_d_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwod_d_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwod_d_wu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwod_d_wu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwod_w_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwod_w_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwod_w_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwod_w_hu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwod_w_hu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwod_h_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwod_h_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwod_h_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwod_h_bu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwod_h_bu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwev_d_wu_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwev_d_wu_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwev_d_wu_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwev_d_wu_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwev_d_wu_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwev_w_hu_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwev_w_hu_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwev_w_hu_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwev_w_hu_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwev_w_hu_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwev_h_bu_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwev_h_bu_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwev_h_bu_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwev_h_bu_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwev_h_bu_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwod_d_wu_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwod_d_wu_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwod_d_wu_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwod_d_wu_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwod_d_wu_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwod_w_hu_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwod_w_hu_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwod_w_hu_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwod_w_hu_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwod_w_hu_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwod_h_bu_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwod_h_bu_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwod_h_bu_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwod_h_bu_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwod_h_bu_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsubwev_d_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsubwev_d_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsubwev_d_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsubwev_d_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsubwev_d_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsubwev_w_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsubwev_w_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsubwev_w_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsubwev_w_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsubwev_w_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsubwev_h_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsubwev_h_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsubwev_h_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsubwev_h_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsubwev_h_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsubwod_d_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsubwod_d_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsubwod_d_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsubwod_d_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsubwod_d_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsubwod_w_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsubwod_w_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsubwod_w_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsubwod_w_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsubwod_w_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsubwod_h_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsubwod_h_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsubwod_h_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsubwod_h_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsubwod_h_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsubwev_d_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsubwev_d_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsubwev_d_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsubwev_d_wu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsubwev_d_wu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsubwev_w_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsubwev_w_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsubwev_w_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsubwev_w_hu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsubwev_w_hu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsubwev_h_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsubwev_h_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsubwev_h_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsubwev_h_bu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsubwev_h_bu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsubwod_d_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsubwod_d_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsubwod_d_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsubwod_d_wu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsubwod_d_wu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsubwod_w_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsubwod_w_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsubwod_w_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsubwod_w_hu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsubwod_w_hu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsubwod_h_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsubwod_h_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsubwod_h_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsubwod_h_bu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsubwod_h_bu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwev_q_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwev_q_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwev_q_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwev_q_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwev_q_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwod_q_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwod_q_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwod_q_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwod_q_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwod_q_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwev_q_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwev_q_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwev_q_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwev_q_du (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwev_q_du (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwod_q_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwod_q_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwod_q_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwod_q_du (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwod_q_du (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsubwev_q_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsubwev_q_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsubwev_q_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsubwev_q_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsubwev_q_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsubwod_q_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsubwod_q_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsubwod_q_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsubwod_q_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsubwod_q_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsubwev_q_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsubwev_q_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsubwev_q_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsubwev_q_du (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsubwev_q_du (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsubwod_q_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsubwod_q_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsubwod_q_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsubwod_q_du (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsubwod_q_du (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwev_q_du_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwev_q_du_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwev_q_du_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwev_q_du_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwev_q_du_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vaddwod_q_du_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vaddwod_q_du_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vaddwod_q_du_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vaddwod_q_du_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vaddwod_q_du_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwev_d_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwev_d_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwev_d_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwev_d_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwev_d_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwev_w_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwev_w_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwev_w_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwev_w_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwev_w_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwev_h_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwev_h_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwev_h_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwev_h_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwev_h_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwod_d_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwod_d_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwod_d_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwod_d_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwod_d_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwod_w_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwod_w_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwod_w_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwod_w_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwod_w_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwod_h_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwod_h_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwod_h_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwod_h_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwod_h_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwev_d_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwev_d_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwev_d_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwev_d_wu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwev_d_wu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwev_w_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwev_w_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwev_w_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwev_w_hu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwev_w_hu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwev_h_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwev_h_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwev_h_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwev_h_bu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwev_h_bu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwod_d_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwod_d_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwod_d_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwod_d_wu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwod_d_wu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwod_w_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwod_w_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwod_w_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwod_w_hu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwod_w_hu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwod_h_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwod_h_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwod_h_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwod_h_bu (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwod_h_bu (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwev_d_wu_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwev_d_wu_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwev_d_wu_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwev_d_wu_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwev_d_wu_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwev_w_hu_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwev_w_hu_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwev_w_hu_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwev_w_hu_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwev_w_hu_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwev_h_bu_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwev_h_bu_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwev_h_bu_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwev_h_bu_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwev_h_bu_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwod_d_wu_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwod_d_wu_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwod_d_wu_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwod_d_wu_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwod_d_wu_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwod_w_hu_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwod_w_hu_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwod_w_hu_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwod_w_hu_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwod_w_hu_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwod_h_bu_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwod_h_bu_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwod_h_bu_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwod_h_bu_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwod_h_bu_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwev_q_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwev_q_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwev_q_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwev_q_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwev_q_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwod_q_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwod_q_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwod_q_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwod_q_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwod_q_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwev_q_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwev_q_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwev_q_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwev_q_du (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwev_q_du (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwod_q_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwod_q_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwod_q_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwod_q_du (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwod_q_du (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwev_q_du_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwev_q_du_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwev_q_du_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwev_q_du_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwev_q_du_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmulwod_q_du_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmulwod_q_du_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmulwod_q_du_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmulwod_q_du_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vmulwod_q_du_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vhaddw_q_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vhaddw_q_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vhaddw_q_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vhaddw_q_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vhaddw_q_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vhaddw_qu_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vhaddw_qu_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vhaddw_qu_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vhaddw_qu_du (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vhaddw_qu_du (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vhsubw_q_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vhsubw_q_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vhsubw_q_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vhsubw_q_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vhsubw_q_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vhsubw_qu_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vhsubw_qu_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vhsubw_qu_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vhsubw_qu_du (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vhsubw_qu_du (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vmaddwev_d_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwev_d_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwev_d_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwev_d_w (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwev_d_w (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwev_w_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwev_w_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwev_w_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwev_w_h (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwev_w_h (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwev_h_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwev_h_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwev_h_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwev_h_b (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwev_h_b (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwev_d_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwev_d_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwev_d_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwev_d_wu (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwev_d_wu (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwev_w_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwev_w_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwev_w_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwev_w_hu (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwev_w_hu (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwev_h_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwev_h_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwev_h_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwev_h_bu (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwev_h_bu (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwod_d_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwod_d_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwod_d_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwod_d_w (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwod_d_w (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwod_w_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwod_w_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwod_w_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwod_w_h (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwod_w_h (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwod_h_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwod_h_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwod_h_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwod_h_b (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwod_h_b (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwod_d_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwod_d_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwod_d_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwod_d_wu (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwod_d_wu (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwod_w_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwod_w_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwod_w_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwod_w_hu (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwod_w_hu (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwod_h_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwod_h_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwod_h_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwod_h_bu (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwod_h_bu (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwev_d_wu_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwev_d_wu_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwev_d_wu_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwev_d_wu_w (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwev_d_wu_w (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwev_w_hu_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwev_w_hu_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwev_w_hu_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwev_w_hu_h (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwev_w_hu_h (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwev_h_bu_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwev_h_bu_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwev_h_bu_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwev_h_bu_b (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwev_h_bu_b (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwod_d_wu_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwod_d_wu_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwod_d_wu_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwod_d_wu_w (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwod_d_wu_w (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwod_w_hu_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwod_w_hu_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwod_w_hu_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwod_w_hu_h (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwod_w_hu_h (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwod_h_bu_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwod_h_bu_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwod_h_bu_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwod_h_bu_b (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwod_h_bu_b (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwev_q_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwev_q_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwev_q_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwev_q_d (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwev_q_d (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwod_q_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwod_q_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwod_q_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwod_q_d (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwod_q_d (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwev_q_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwev_q_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwev_q_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwev_q_du (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwev_q_du (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwod_q_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwod_q_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwod_q_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwod_q_du (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwod_q_du (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwev_q_du_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwev_q_du_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwev_q_du_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwev_q_du_d (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwev_q_du_d (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vmaddwod_q_du_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmaddwod_q_du_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmaddwod_q_du_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmaddwod_q_du_d (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vmaddwod_q_du_d (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vrotr_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vrotr_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vrotr_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vrotr_b (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vrotr_b (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vrotr_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vrotr_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vrotr_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vrotr_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vrotr_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vrotr_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vrotr_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vrotr_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vrotr_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vrotr_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vrotr_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vrotr_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vrotr_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vrotr_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vrotr_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vadd_q_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vadd_q in module {}", module_path!());
    };
}

mkfn!{
    lsx_vadd_q_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vadd_q (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vadd_q (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vsub_q_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsub_q in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsub_q_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsub_q (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vsub_q (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vldrepl_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vldrepl_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vldrepl_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn lsx_vldrepl_b < const IMM_S12 : i32 > (mem_addr : * const i8) -> m128i { static_assert_simm_bits ! (IMM_S12 , 12) ; transmute (__lsx_vldrepl_b (mem_addr , IMM_S12)) }
}

macro_rules! lsx_vldrepl_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vldrepl_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vldrepl_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn lsx_vldrepl_h < const IMM_S11 : i32 > (mem_addr : * const i8) -> m128i { static_assert_simm_bits ! (IMM_S11 , 11) ; transmute (__lsx_vldrepl_h (mem_addr , IMM_S11)) }
}

macro_rules! lsx_vldrepl_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vldrepl_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vldrepl_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn lsx_vldrepl_w < const IMM_S10 : i32 > (mem_addr : * const i8) -> m128i { static_assert_simm_bits ! (IMM_S10 , 10) ; transmute (__lsx_vldrepl_w (mem_addr , IMM_S10)) }
}

macro_rules! lsx_vldrepl_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vldrepl_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vldrepl_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn lsx_vldrepl_d < const IMM_S9 : i32 > (mem_addr : * const i8) -> m128i { static_assert_simm_bits ! (IMM_S9 , 9) ; transmute (__lsx_vldrepl_d (mem_addr , IMM_S9)) }
}

macro_rules! lsx_vmskgez_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmskgez_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmskgez_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmskgez_b (a : m128i) -> m128i { unsafe { transmute (__lsx_vmskgez_b (transmute (a))) } }
}

macro_rules! lsx_vmsknz_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vmsknz_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vmsknz_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vmsknz_b (a : m128i) -> m128i { unsafe { transmute (__lsx_vmsknz_b (transmute (a))) } }
}

macro_rules! lsx_vexth_h_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vexth_h_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vexth_h_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vexth_h_b (a : m128i) -> m128i { unsafe { transmute (__lsx_vexth_h_b (transmute (a))) } }
}

macro_rules! lsx_vexth_w_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vexth_w_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vexth_w_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vexth_w_h (a : m128i) -> m128i { unsafe { transmute (__lsx_vexth_w_h (transmute (a))) } }
}

macro_rules! lsx_vexth_d_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vexth_d_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vexth_d_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vexth_d_w (a : m128i) -> m128i { unsafe { transmute (__lsx_vexth_d_w (transmute (a))) } }
}

macro_rules! lsx_vexth_q_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vexth_q_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vexth_q_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vexth_q_d (a : m128i) -> m128i { unsafe { transmute (__lsx_vexth_q_d (transmute (a))) } }
}

macro_rules! lsx_vexth_hu_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vexth_hu_bu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vexth_hu_bu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vexth_hu_bu (a : m128i) -> m128i { unsafe { transmute (__lsx_vexth_hu_bu (transmute (a))) } }
}

macro_rules! lsx_vexth_wu_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vexth_wu_hu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vexth_wu_hu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vexth_wu_hu (a : m128i) -> m128i { unsafe { transmute (__lsx_vexth_wu_hu (transmute (a))) } }
}

macro_rules! lsx_vexth_du_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vexth_du_wu in module {}", module_path!());
    };
}

mkfn!{
    lsx_vexth_du_wu_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vexth_du_wu (a : m128i) -> m128i { unsafe { transmute (__lsx_vexth_du_wu (transmute (a))) } }
}

macro_rules! lsx_vexth_qu_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vexth_qu_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vexth_qu_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vexth_qu_du (a : m128i) -> m128i { unsafe { transmute (__lsx_vexth_qu_du (transmute (a))) } }
}

macro_rules! lsx_vrotri_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vrotri_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vrotri_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vrotri_b < const IMM3 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM3 , 3) ; unsafe { transmute (__lsx_vrotri_b (transmute (a) , IMM3)) } }
}

macro_rules! lsx_vrotri_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vrotri_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vrotri_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vrotri_h < const IMM4 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vrotri_h (transmute (a) , IMM4)) } }
}

macro_rules! lsx_vrotri_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vrotri_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vrotri_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vrotri_w < const IMM5 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vrotri_w (transmute (a) , IMM5)) } }
}

macro_rules! lsx_vrotri_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vrotri_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vrotri_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vrotri_d < const IMM6 : u32 > (a : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vrotri_d (transmute (a) , IMM6)) } }
}

macro_rules! lsx_vextl_q_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vextl_q_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vextl_q_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vextl_q_d (a : m128i) -> m128i { unsafe { transmute (__lsx_vextl_q_d (transmute (a))) } }
}

macro_rules! lsx_vsrlni_b_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrlni_b_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrlni_b_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrlni_b_h < const IMM4 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vsrlni_b_h (transmute (a) , transmute (b) , IMM4)) } }
}

macro_rules! lsx_vsrlni_h_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrlni_h_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrlni_h_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrlni_h_w < const IMM5 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vsrlni_h_w (transmute (a) , transmute (b) , IMM5)) } }
}

macro_rules! lsx_vsrlni_w_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrlni_w_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrlni_w_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrlni_w_d < const IMM6 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vsrlni_w_d (transmute (a) , transmute (b) , IMM6)) } }
}

macro_rules! lsx_vsrlni_d_q_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrlni_d_q in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrlni_d_q_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrlni_d_q < const IMM7 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM7 , 7) ; unsafe { transmute (__lsx_vsrlni_d_q (transmute (a) , transmute (b) , IMM7)) } }
}

macro_rules! lsx_vsrlrni_b_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrlrni_b_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrlrni_b_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrlrni_b_h < const IMM4 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vsrlrni_b_h (transmute (a) , transmute (b) , IMM4)) } }
}

macro_rules! lsx_vsrlrni_h_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrlrni_h_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrlrni_h_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrlrni_h_w < const IMM5 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vsrlrni_h_w (transmute (a) , transmute (b) , IMM5)) } }
}

macro_rules! lsx_vsrlrni_w_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrlrni_w_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrlrni_w_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrlrni_w_d < const IMM6 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vsrlrni_w_d (transmute (a) , transmute (b) , IMM6)) } }
}

macro_rules! lsx_vsrlrni_d_q_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrlrni_d_q in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrlrni_d_q_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrlrni_d_q < const IMM7 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM7 , 7) ; unsafe { transmute (__lsx_vsrlrni_d_q (transmute (a) , transmute (b) , IMM7)) } }
}

macro_rules! lsx_vssrlni_b_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlni_b_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlni_b_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlni_b_h < const IMM4 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vssrlni_b_h (transmute (a) , transmute (b) , IMM4)) } }
}

macro_rules! lsx_vssrlni_h_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlni_h_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlni_h_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlni_h_w < const IMM5 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vssrlni_h_w (transmute (a) , transmute (b) , IMM5)) } }
}

macro_rules! lsx_vssrlni_w_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlni_w_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlni_w_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlni_w_d < const IMM6 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vssrlni_w_d (transmute (a) , transmute (b) , IMM6)) } }
}

macro_rules! lsx_vssrlni_d_q_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlni_d_q in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlni_d_q_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlni_d_q < const IMM7 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM7 , 7) ; unsafe { transmute (__lsx_vssrlni_d_q (transmute (a) , transmute (b) , IMM7)) } }
}

macro_rules! lsx_vssrlni_bu_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlni_bu_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlni_bu_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlni_bu_h < const IMM4 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vssrlni_bu_h (transmute (a) , transmute (b) , IMM4)) } }
}

macro_rules! lsx_vssrlni_hu_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlni_hu_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlni_hu_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlni_hu_w < const IMM5 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vssrlni_hu_w (transmute (a) , transmute (b) , IMM5)) } }
}

macro_rules! lsx_vssrlni_wu_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlni_wu_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlni_wu_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlni_wu_d < const IMM6 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vssrlni_wu_d (transmute (a) , transmute (b) , IMM6)) } }
}

macro_rules! lsx_vssrlni_du_q_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlni_du_q in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlni_du_q_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlni_du_q < const IMM7 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM7 , 7) ; unsafe { transmute (__lsx_vssrlni_du_q (transmute (a) , transmute (b) , IMM7)) } }
}

macro_rules! lsx_vssrlrni_b_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlrni_b_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlrni_b_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlrni_b_h < const IMM4 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vssrlrni_b_h (transmute (a) , transmute (b) , IMM4)) } }
}

macro_rules! lsx_vssrlrni_h_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlrni_h_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlrni_h_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlrni_h_w < const IMM5 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vssrlrni_h_w (transmute (a) , transmute (b) , IMM5)) } }
}

macro_rules! lsx_vssrlrni_w_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlrni_w_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlrni_w_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlrni_w_d < const IMM6 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vssrlrni_w_d (transmute (a) , transmute (b) , IMM6)) } }
}

macro_rules! lsx_vssrlrni_d_q_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlrni_d_q in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlrni_d_q_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlrni_d_q < const IMM7 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM7 , 7) ; unsafe { transmute (__lsx_vssrlrni_d_q (transmute (a) , transmute (b) , IMM7)) } }
}

macro_rules! lsx_vssrlrni_bu_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlrni_bu_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlrni_bu_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlrni_bu_h < const IMM4 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vssrlrni_bu_h (transmute (a) , transmute (b) , IMM4)) } }
}

macro_rules! lsx_vssrlrni_hu_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlrni_hu_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlrni_hu_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlrni_hu_w < const IMM5 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vssrlrni_hu_w (transmute (a) , transmute (b) , IMM5)) } }
}

macro_rules! lsx_vssrlrni_wu_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlrni_wu_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlrni_wu_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlrni_wu_d < const IMM6 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vssrlrni_wu_d (transmute (a) , transmute (b) , IMM6)) } }
}

macro_rules! lsx_vssrlrni_du_q_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlrni_du_q in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlrni_du_q_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlrni_du_q < const IMM7 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM7 , 7) ; unsafe { transmute (__lsx_vssrlrni_du_q (transmute (a) , transmute (b) , IMM7)) } }
}

macro_rules! lsx_vsrani_b_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrani_b_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrani_b_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrani_b_h < const IMM4 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vsrani_b_h (transmute (a) , transmute (b) , IMM4)) } }
}

macro_rules! lsx_vsrani_h_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrani_h_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrani_h_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrani_h_w < const IMM5 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vsrani_h_w (transmute (a) , transmute (b) , IMM5)) } }
}

macro_rules! lsx_vsrani_w_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrani_w_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrani_w_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrani_w_d < const IMM6 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vsrani_w_d (transmute (a) , transmute (b) , IMM6)) } }
}

macro_rules! lsx_vsrani_d_q_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrani_d_q in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrani_d_q_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrani_d_q < const IMM7 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM7 , 7) ; unsafe { transmute (__lsx_vsrani_d_q (transmute (a) , transmute (b) , IMM7)) } }
}

macro_rules! lsx_vsrarni_b_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrarni_b_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrarni_b_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrarni_b_h < const IMM4 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vsrarni_b_h (transmute (a) , transmute (b) , IMM4)) } }
}

macro_rules! lsx_vsrarni_h_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrarni_h_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrarni_h_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrarni_h_w < const IMM5 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vsrarni_h_w (transmute (a) , transmute (b) , IMM5)) } }
}

macro_rules! lsx_vsrarni_w_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrarni_w_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrarni_w_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrarni_w_d < const IMM6 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vsrarni_w_d (transmute (a) , transmute (b) , IMM6)) } }
}

macro_rules! lsx_vsrarni_d_q_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vsrarni_d_q in module {}", module_path!());
    };
}

mkfn!{
    lsx_vsrarni_d_q_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vsrarni_d_q < const IMM7 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM7 , 7) ; unsafe { transmute (__lsx_vsrarni_d_q (transmute (a) , transmute (b) , IMM7)) } }
}

macro_rules! lsx_vssrani_b_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrani_b_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrani_b_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrani_b_h < const IMM4 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vssrani_b_h (transmute (a) , transmute (b) , IMM4)) } }
}

macro_rules! lsx_vssrani_h_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrani_h_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrani_h_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrani_h_w < const IMM5 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vssrani_h_w (transmute (a) , transmute (b) , IMM5)) } }
}

macro_rules! lsx_vssrani_w_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrani_w_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrani_w_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrani_w_d < const IMM6 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vssrani_w_d (transmute (a) , transmute (b) , IMM6)) } }
}

macro_rules! lsx_vssrani_d_q_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrani_d_q in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrani_d_q_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrani_d_q < const IMM7 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM7 , 7) ; unsafe { transmute (__lsx_vssrani_d_q (transmute (a) , transmute (b) , IMM7)) } }
}

macro_rules! lsx_vssrani_bu_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrani_bu_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrani_bu_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrani_bu_h < const IMM4 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vssrani_bu_h (transmute (a) , transmute (b) , IMM4)) } }
}

macro_rules! lsx_vssrani_hu_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrani_hu_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrani_hu_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrani_hu_w < const IMM5 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vssrani_hu_w (transmute (a) , transmute (b) , IMM5)) } }
}

macro_rules! lsx_vssrani_wu_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrani_wu_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrani_wu_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrani_wu_d < const IMM6 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vssrani_wu_d (transmute (a) , transmute (b) , IMM6)) } }
}

macro_rules! lsx_vssrani_du_q_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrani_du_q in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrani_du_q_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrani_du_q < const IMM7 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM7 , 7) ; unsafe { transmute (__lsx_vssrani_du_q (transmute (a) , transmute (b) , IMM7)) } }
}

macro_rules! lsx_vssrarni_b_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrarni_b_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrarni_b_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrarni_b_h < const IMM4 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vssrarni_b_h (transmute (a) , transmute (b) , IMM4)) } }
}

macro_rules! lsx_vssrarni_h_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrarni_h_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrarni_h_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrarni_h_w < const IMM5 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vssrarni_h_w (transmute (a) , transmute (b) , IMM5)) } }
}

macro_rules! lsx_vssrarni_w_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrarni_w_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrarni_w_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrarni_w_d < const IMM6 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vssrarni_w_d (transmute (a) , transmute (b) , IMM6)) } }
}

macro_rules! lsx_vssrarni_d_q_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrarni_d_q in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrarni_d_q_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrarni_d_q < const IMM7 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM7 , 7) ; unsafe { transmute (__lsx_vssrarni_d_q (transmute (a) , transmute (b) , IMM7)) } }
}

macro_rules! lsx_vssrarni_bu_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrarni_bu_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrarni_bu_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrarni_bu_h < const IMM4 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM4 , 4) ; unsafe { transmute (__lsx_vssrarni_bu_h (transmute (a) , transmute (b) , IMM4)) } }
}

macro_rules! lsx_vssrarni_hu_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrarni_hu_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrarni_hu_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrarni_hu_w < const IMM5 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { transmute (__lsx_vssrarni_hu_w (transmute (a) , transmute (b) , IMM5)) } }
}

macro_rules! lsx_vssrarni_wu_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrarni_wu_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrarni_wu_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrarni_wu_d < const IMM6 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM6 , 6) ; unsafe { transmute (__lsx_vssrarni_wu_d (transmute (a) , transmute (b) , IMM6)) } }
}

macro_rules! lsx_vssrarni_du_q_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrarni_du_q in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrarni_du_q_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrarni_du_q < const IMM7 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM7 , 7) ; unsafe { transmute (__lsx_vssrarni_du_q (transmute (a) , transmute (b) , IMM7)) } }
}

macro_rules! lsx_vpermi_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vpermi_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vpermi_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vpermi_w < const IMM8 : u32 > (a : m128i , b : m128i) -> m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { transmute (__lsx_vpermi_w (transmute (a) , transmute (b) , IMM8)) } }
}

macro_rules! lsx_vld_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vld in module {}", module_path!());
    };
}

mkfn!{
    lsx_vld_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (1)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn lsx_vld < const IMM_S12 : i32 > (mem_addr : * const i8) -> m128i { static_assert_simm_bits ! (IMM_S12 , 12) ; transmute (__lsx_vld (mem_addr , IMM_S12)) }
}

macro_rules! lsx_vst_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vst in module {}", module_path!());
    };
}

mkfn!{
    lsx_vst_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (2)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn lsx_vst < const IMM_S12 : i32 > (a : m128i , mem_addr : * mut i8) { static_assert_simm_bits ! (IMM_S12 , 12) ; transmute (__lsx_vst (transmute (a) , mem_addr , IMM_S12)) }
}

macro_rules! lsx_vssrlrn_b_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlrn_b_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlrn_b_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlrn_b_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssrlrn_b_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssrlrn_h_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlrn_h_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlrn_h_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlrn_h_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssrlrn_h_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssrlrn_w_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrlrn_w_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrlrn_w_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrlrn_w_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssrlrn_w_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssrln_b_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrln_b_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrln_b_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrln_b_h (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssrln_b_h (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssrln_h_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrln_h_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrln_h_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrln_h_w (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssrln_h_w (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vssrln_w_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vssrln_w_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vssrln_w_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vssrln_w_d (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vssrln_w_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vorn_v_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vorn_v in module {}", module_path!());
    };
}

mkfn!{
    lsx_vorn_v_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vorn_v (a : m128i , b : m128i) -> m128i { unsafe { transmute (__lsx_vorn_v (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vldi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vldi in module {}", module_path!());
    };
}

mkfn!{
    lsx_vldi_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (0)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vldi < const IMM_S13 : i32 > () -> m128i { static_assert_simm_bits ! (IMM_S13 , 13) ; unsafe { transmute (__lsx_vldi (IMM_S13)) } }
}

macro_rules! lsx_vshuf_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vshuf_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vshuf_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vshuf_b (a : m128i , b : m128i , c : m128i) -> m128i { unsafe { transmute (__lsx_vshuf_b (transmute (a) , transmute (b) , transmute (c))) } }
}

macro_rules! lsx_vldx_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vldx in module {}", module_path!());
    };
}

mkfn!{
    lsx_vldx_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn lsx_vldx (mem_addr : * const i8 , b : i64) -> m128i { transmute (__lsx_vldx (mem_addr , transmute (b))) }
}

macro_rules! lsx_vstx_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vstx in module {}", module_path!());
    };
}

mkfn!{
    lsx_vstx_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn lsx_vstx (a : m128i , mem_addr : * mut i8 , b : i64) { transmute (__lsx_vstx (transmute (a) , mem_addr , transmute (b))) }
}

macro_rules! lsx_vextl_qu_du_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vextl_qu_du in module {}", module_path!());
    };
}

mkfn!{
    lsx_vextl_qu_du_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vextl_qu_du (a : m128i) -> m128i { unsafe { transmute (__lsx_vextl_qu_du (transmute (a))) } }
}

macro_rules! lsx_bnz_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_bnz_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_bnz_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_bnz_b (a : m128i) -> i32 { unsafe { transmute (__lsx_bnz_b (transmute (a))) } }
}

macro_rules! lsx_bnz_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_bnz_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_bnz_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_bnz_d (a : m128i) -> i32 { unsafe { transmute (__lsx_bnz_d (transmute (a))) } }
}

macro_rules! lsx_bnz_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_bnz_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_bnz_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_bnz_h (a : m128i) -> i32 { unsafe { transmute (__lsx_bnz_h (transmute (a))) } }
}

macro_rules! lsx_bnz_v_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_bnz_v in module {}", module_path!());
    };
}

mkfn!{
    lsx_bnz_v_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_bnz_v (a : m128i) -> i32 { unsafe { transmute (__lsx_bnz_v (transmute (a))) } }
}

macro_rules! lsx_bnz_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_bnz_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_bnz_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_bnz_w (a : m128i) -> i32 { unsafe { transmute (__lsx_bnz_w (transmute (a))) } }
}

macro_rules! lsx_bz_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_bz_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_bz_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_bz_b (a : m128i) -> i32 { unsafe { transmute (__lsx_bz_b (transmute (a))) } }
}

macro_rules! lsx_bz_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_bz_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_bz_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_bz_d (a : m128i) -> i32 { unsafe { transmute (__lsx_bz_d (transmute (a))) } }
}

macro_rules! lsx_bz_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_bz_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_bz_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_bz_h (a : m128i) -> i32 { unsafe { transmute (__lsx_bz_h (transmute (a))) } }
}

macro_rules! lsx_bz_v_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_bz_v in module {}", module_path!());
    };
}

mkfn!{
    lsx_bz_v_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_bz_v (a : m128i) -> i32 { unsafe { transmute (__lsx_bz_v (transmute (a))) } }
}

macro_rules! lsx_bz_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_bz_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_bz_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_bz_w (a : m128i) -> i32 { unsafe { transmute (__lsx_bz_w (transmute (a))) } }
}

macro_rules! lsx_vfcmp_caf_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_caf_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_caf_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_caf_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_caf_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_caf_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_caf_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_caf_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_caf_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_caf_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_ceq_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_ceq_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_ceq_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_ceq_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_ceq_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_ceq_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_ceq_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_ceq_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_ceq_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_ceq_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_cle_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_cle_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_cle_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_cle_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_cle_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_cle_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_cle_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_cle_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_cle_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_cle_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_clt_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_clt_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_clt_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_clt_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_clt_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_clt_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_clt_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_clt_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_clt_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_clt_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_cne_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_cne_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_cne_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_cne_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_cne_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_cne_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_cne_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_cne_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_cne_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_cne_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_cor_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_cor_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_cor_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_cor_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_cor_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_cor_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_cor_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_cor_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_cor_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_cor_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_cueq_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_cueq_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_cueq_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_cueq_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_cueq_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_cueq_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_cueq_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_cueq_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_cueq_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_cueq_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_cule_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_cule_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_cule_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_cule_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_cule_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_cule_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_cule_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_cule_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_cule_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_cule_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_cult_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_cult_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_cult_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_cult_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_cult_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_cult_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_cult_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_cult_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_cult_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_cult_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_cun_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_cun_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_cun_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_cun_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_cun_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_cune_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_cune_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_cune_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_cune_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_cune_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_cune_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_cune_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_cune_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_cune_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_cune_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_cun_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_cun_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_cun_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_cun_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_cun_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_saf_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_saf_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_saf_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_saf_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_saf_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_saf_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_saf_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_saf_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_saf_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_saf_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_seq_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_seq_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_seq_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_seq_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_seq_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_seq_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_seq_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_seq_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_seq_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_seq_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_sle_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_sle_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_sle_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_sle_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_sle_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_sle_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_sle_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_sle_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_sle_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_sle_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_slt_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_slt_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_slt_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_slt_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_slt_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_slt_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_slt_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_slt_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_slt_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_slt_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_sne_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_sne_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_sne_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_sne_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_sne_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_sne_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_sne_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_sne_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_sne_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_sne_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_sor_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_sor_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_sor_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_sor_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_sor_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_sor_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_sor_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_sor_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_sor_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_sor_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_sueq_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_sueq_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_sueq_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_sueq_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_sueq_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_sueq_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_sueq_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_sueq_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_sueq_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_sueq_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_sule_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_sule_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_sule_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_sule_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_sule_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_sule_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_sule_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_sule_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_sule_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_sule_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_sult_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_sult_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_sult_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_sult_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_sult_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_sult_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_sult_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_sult_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_sult_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_sult_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_sun_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_sun_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_sun_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_sun_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_sun_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_sune_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_sune_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_sune_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_sune_d (a : m128d , b : m128d) -> m128i { unsafe { transmute (__lsx_vfcmp_sune_d (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_sune_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_sune_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_sune_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_sune_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_sune_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vfcmp_sun_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vfcmp_sun_s in module {}", module_path!());
    };
}

mkfn!{
    lsx_vfcmp_sun_s_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vfcmp_sun_s (a : m128 , b : m128) -> m128i { unsafe { transmute (__lsx_vfcmp_sun_s (transmute (a) , transmute (b))) } }
}

macro_rules! lsx_vrepli_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vrepli_b in module {}", module_path!());
    };
}

mkfn!{
    lsx_vrepli_b_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (0)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vrepli_b < const IMM_S10 : i32 > () -> m128i { static_assert_simm_bits ! (IMM_S10 , 10) ; unsafe { transmute (__lsx_vrepli_b (IMM_S10)) } }
}

macro_rules! lsx_vrepli_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vrepli_d in module {}", module_path!());
    };
}

mkfn!{
    lsx_vrepli_d_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (0)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vrepli_d < const IMM_S10 : i32 > () -> m128i { static_assert_simm_bits ! (IMM_S10 , 10) ; unsafe { transmute (__lsx_vrepli_d (IMM_S10)) } }
}

macro_rules! lsx_vrepli_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vrepli_h in module {}", module_path!());
    };
}

mkfn!{
    lsx_vrepli_h_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (0)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vrepli_h < const IMM_S10 : i32 > () -> m128i { static_assert_simm_bits ! (IMM_S10 , 10) ; unsafe { transmute (__lsx_vrepli_h (IMM_S10)) } }
}

macro_rules! lsx_vrepli_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lsx_vrepli_w in module {}", module_path!());
    };
}

mkfn!{
    lsx_vrepli_w_introspect!();
    #[inline] #[target_feature (enable = "lsx")] #[rustc_legacy_const_generics (0)] #[unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn lsx_vrepli_w < const IMM_S10 : i32 > () -> m128i { static_assert_simm_bits ! (IMM_S10 , 10) ; unsafe { transmute (__lsx_vrepli_w (IMM_S10)) } }
}