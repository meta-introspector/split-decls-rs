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
mkitem!{macro_rules ! emit_message { ($ ($ arg : tt) *) => { } ; }}
mkitem!{macro_rules ! mkfn { ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: catch_fatal_errors_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: catch_fatal_errors_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn parse_crate_attrs < $ lifetime : lifetime > ($ ($ param : tt) *) -> PResult < $ lifetime2 : lifetime , ast :: AttrVec > $ body : block) => { $ (# [$ attr]) * fn parse_crate_attrs < $ lifetime > ($ ($ param) *) -> PResult < $ lifetime2 , ast :: AttrVec > { $ introspect ; emit_message ! ("🚀 MARKER: parse_crate_attrs_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: parse_crate_attrs_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn init_logger_with_additional_layer < F , T > ($ ($ param : tt) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , $ body : block) => { $ (# [$ attr]) * fn init_logger_with_additional_layer < F , T > ($ ($ param) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , { $ introspect ; emit_message ! ("🚀 MARKER: init_logger_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: init_logger_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < F , T > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where F : FnOnce ($ ($ fnonce_args : tt) *) $ ($ where_rest : tt) * $ body : block) => { $ (# [$ attr]) * fn $ name < F , T > ($ ($ param) *) $ (-> $ ret) ? where F : FnOnce ($ ($ fnonce_args) *) $ ($ where_rest) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where $ ($ where_clause : tt) * $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? where $ ($ where_clause) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub ($ vis : ident) fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub ($ vis) fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_vis - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_vis - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; fn $ name : ident () $ body : block) => { fn $ name () { $ introspect ; emit_message ! ("🚀 MARKER: simple - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: simple - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ ($ anything : tt) *) => { $ ($ anything) * } ; }}
mkitem!{macro_rules ! safe_println { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{macro_rules ! safe_print { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{# [macro_export] macro_rules ! include_rust_compiler { ($ crate_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ crate_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_" , $ file , ".rs")) ; } ; ($ crate_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_lib.rs")) ; } ; }}
mkitem!{# [macro_export] macro_rules ! include_rust_library { ($ lib_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ lib_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_" , $ file , ".rs")) ; } ; ($ lib_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_lib.rs")) ; } ; }}
mkitem!{# [macro_export] macro_rules ! include_processed { ($ path : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_" , $ path , ".rs")) ; } ; }}
mkitem!{macro_rules ! mkinclude { ($ path : ident) => { } ; ($ path : literal) => { include ! ($ path) } ; }}
mkitem!{macro_rules ! mkitem { (include ! ($ path : ident) ;) => { } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ string_lit : literal }) => { $ macro_name :: $ macro_sub ! { $ string_lit } } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ ($ args : tt) * }) => { $ macro_name :: $ macro_sub ! { $ ($ args) * } } ; ($ macro_name : ident ! { $ ($ args : tt) * }) => { $ macro_name ! { $ ($ args) * } } ; ($ item : item) => { $ item } ; }}
mkitem!{# [macro_export] macro_rules ! mkmod { ($ name : ident , { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (pub mod $ name : ident { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; pub mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (mod $ name : ident { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; }}
mkitem!{# [macro_export] macro_rules ! mkuse { ($ use_stmt : item) => { compile_error ! (concat ! ("USE|" , module_path ! () , "|" , stringify ! ($ use_stmt))) ; } ; }}
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
mkuse!{use std :: fmt ;}
mkuse!{use std :: num :: NonZero ;}
mkuse!{use rustc_abi :: Size ;}
mkuse!{use rustc_apfloat :: Float ;}
mkuse!{use rustc_apfloat :: ieee :: { Double , Half , Quad , Single } ;}
mkuse!{use rustc_errors :: { DiagArgValue , IntoDiagArg } ;}
mkuse!{use rustc_serialize :: { Decodable , Decoder , Encodable , Encoder } ;}
mkuse!{use crate :: ty :: TyCtxt ;}
mkitem!{mkstruct!{# [derive (Copy , Clone)] # [doc = " A type for representing any integer. Only used for printing."] pub struct ConstInt { # [doc = " The \"untyped\" variant of `ConstInt`."] int : ScalarInt , # [doc = " Whether the value is of a signed integer type."] signed : bool , # [doc = " Whether the value is a `usize` or `isize` type."] is_ptr_sized_integral : bool , }}}
mkitem!{mkimpl!{impl ConstInt { pub fn new (int : ScalarInt , signed : bool , is_ptr_sized_integral : bool) -> Self { Self { int , signed , is_ptr_sized_integral } } }}}
mkitem!{mkenum!{# [doc = " An enum to represent the compiler-side view of `intrinsics::AtomicOrdering`."] # [doc = " This lives here because there's a method in this file that needs it and it is entirely unclear"] # [doc = " where else to put this..."] # [derive (Debug , Copy , Clone)] pub enum AtomicOrdering { Relaxed = 0 , Release = 1 , Acquire = 2 , AcqRel = 3 , SeqCst = 4 , }}}
mkitem!{mkimpl!{impl std :: fmt :: Debug for ConstInt { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let Self { int , signed , is_ptr_sized_integral } = * self ; let size = int . size () . bytes () ; let raw = int . data ; if signed { let bit_size = size * 8 ; let min = 1u128 << (bit_size - 1) ; let max = min - 1 ; if raw == min { match (size , is_ptr_sized_integral) { (_ , true) => write ! (fmt , "isize::MIN") , (1 , _) => write ! (fmt , "i8::MIN") , (2 , _) => write ! (fmt , "i16::MIN") , (4 , _) => write ! (fmt , "i32::MIN") , (8 , _) => write ! (fmt , "i64::MIN") , (16 , _) => write ! (fmt , "i128::MIN") , _ => bug ! ("ConstInt 0x{:x} with size = {} and signed = {}" , raw , size , signed) , } } else if raw == max { match (size , is_ptr_sized_integral) { (_ , true) => write ! (fmt , "isize::MAX") , (1 , _) => write ! (fmt , "i8::MAX") , (2 , _) => write ! (fmt , "i16::MAX") , (4 , _) => write ! (fmt , "i32::MAX") , (8 , _) => write ! (fmt , "i64::MAX") , (16 , _) => write ! (fmt , "i128::MAX") , _ => bug ! ("ConstInt 0x{:x} with size = {} and signed = {}" , raw , size , signed) , } } else { match size { 1 => write ! (fmt , "{}" , raw as i8) ? , 2 => write ! (fmt , "{}" , raw as i16) ? , 4 => write ! (fmt , "{}" , raw as i32) ? , 8 => write ! (fmt , "{}" , raw as i64) ? , 16 => write ! (fmt , "{}" , raw as i128) ? , _ => bug ! ("ConstInt 0x{:x} with size = {} and signed = {}" , raw , size , signed) , } if fmt . alternate () { match (size , is_ptr_sized_integral) { (_ , true) => write ! (fmt , "_isize") ? , (1 , _) => write ! (fmt , "_i8") ? , (2 , _) => write ! (fmt , "_i16") ? , (4 , _) => write ! (fmt , "_i32") ? , (8 , _) => write ! (fmt , "_i64") ? , (16 , _) => write ! (fmt , "_i128") ? , (sz , _) => bug ! ("unexpected int size i{sz}") , } } Ok (()) } } else { let max = Size :: from_bytes (size) . truncate (u128 :: MAX) ; if raw == max { match (size , is_ptr_sized_integral) { (_ , true) => write ! (fmt , "usize::MAX") , (1 , _) => write ! (fmt , "u8::MAX") , (2 , _) => write ! (fmt , "u16::MAX") , (4 , _) => write ! (fmt , "u32::MAX") , (8 , _) => write ! (fmt , "u64::MAX") , (16 , _) => write ! (fmt , "u128::MAX") , _ => bug ! ("ConstInt 0x{:x} with size = {} and signed = {}" , raw , size , signed) , } } else { match size { 1 => write ! (fmt , "{}" , raw as u8) ? , 2 => write ! (fmt , "{}" , raw as u16) ? , 4 => write ! (fmt , "{}" , raw as u32) ? , 8 => write ! (fmt , "{}" , raw as u64) ? , 16 => write ! (fmt , "{}" , raw as u128) ? , _ => bug ! ("ConstInt 0x{:x} with size = {} and signed = {}" , raw , size , signed) , } if fmt . alternate () { match (size , is_ptr_sized_integral) { (_ , true) => write ! (fmt , "_usize") ? , (1 , _) => write ! (fmt , "_u8") ? , (2 , _) => write ! (fmt , "_u16") ? , (4 , _) => write ! (fmt , "_u32") ? , (8 , _) => write ! (fmt , "_u64") ? , (16 , _) => write ! (fmt , "_u128") ? , (sz , _) => bug ! ("unexpected unsigned int size u{sz}") , } } Ok (()) } } } }}}
mkitem!{mkimpl!{impl IntoDiagArg for ConstInt { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (format ! ("{self:?}") . into ()) } }}}
mkitem!{mkstruct!{# [doc = " The raw bytes of a simple value."] # [doc = ""] # [doc = " This is a packed struct in order to allow this type to be optimally embedded in enums"] # [doc = " (like Scalar)."] # [derive (Clone , Copy , Eq , PartialEq , Hash)] # [repr (packed)] pub struct ScalarInt { # [doc = " The first `size` bytes of `data` are the value."] # [doc = " Do not try to read less or more bytes than that. The remaining bytes must be 0."] data : u128 , size : NonZero < u8 > , }}}
mkitem!{mkimpl!{impl < CTX > crate :: ty :: HashStable < CTX > for ScalarInt { fn hash_stable (& self , hcx : & mut CTX , hasher : & mut crate :: ty :: StableHasher) { { self . data } . hash_stable (hcx , hasher) ; self . size . get () . hash_stable (hcx , hasher) ; } }}}
mkitem!{mkimpl!{impl < S : Encoder > Encodable < S > for ScalarInt { fn encode (& self , s : & mut S) { let size = self . size . get () ; s . emit_u8 (size) ; s . emit_raw_bytes (& self . data . to_le_bytes () [.. size as usize]) ; } }}}
mkitem!{mkimpl!{impl < D : Decoder > Decodable < D > for ScalarInt { fn decode (d : & mut D) -> ScalarInt { let mut data = [0u8 ; 16] ; let size = d . read_u8 () ; data [.. size as usize] . copy_from_slice (d . read_raw_bytes (size as usize)) ; ScalarInt { data : u128 :: from_le_bytes (data) , size : NonZero :: new (size) . unwrap () } } }}}
mkitem!{mkimpl!{impl ScalarInt { pub const TRUE : ScalarInt = ScalarInt { data : 1_u128 , size : NonZero :: new (1) . unwrap () } ; pub const FALSE : ScalarInt = ScalarInt { data : 0_u128 , size : NonZero :: new (1) . unwrap () } ; fn raw (data : u128 , size : Size) -> Self { Self { data , size : NonZero :: new (size . bytes () as u8) . unwrap () } } # [inline] pub fn size (self) -> Size { Size :: from_bytes (self . size . get ()) } # [doc = " Make sure the `data` fits in `size`."] # [doc = " This is guaranteed by all constructors here, but having had this check saved us from"] # [doc = " bugs many times in the past, so keeping it around is definitely worth it."] # [inline (always)] fn check_data (self) { debug_assert_eq ! (self . size () . truncate (self . data) , { self . data } , "Scalar value {:#x} exceeds size of {} bytes" , { self . data } , self . size) ; } # [inline] pub fn null (size : Size) -> Self { Self :: raw (0 , size) } # [inline] pub fn is_null (self) -> bool { self . data == 0 } # [inline] pub fn try_from_uint (i : impl Into < u128 > , size : Size) -> Option < Self > { let (r , overflow) = Self :: truncate_from_uint (i , size) ; if overflow { None } else { Some (r) } } # [doc = " Returns the truncated result, and whether truncation changed the value."] # [inline] pub fn truncate_from_uint (i : impl Into < u128 > , size : Size) -> (Self , bool) { let data = i . into () ; let r = Self :: raw (size . truncate (data) , size) ; (r , r . data != data) } # [inline] pub fn try_from_int (i : impl Into < i128 > , size : Size) -> Option < Self > { let (r , overflow) = Self :: truncate_from_int (i , size) ; if overflow { None } else { Some (r) } } # [doc = " Returns the truncated result, and whether truncation changed the value."] # [inline] pub fn truncate_from_int (i : impl Into < i128 > , size : Size) -> (Self , bool) { let data = i . into () ; let r = Self :: raw (size . truncate (data as u128) , size) ; (r , size . sign_extend (r . data) != data) } # [inline] pub fn try_from_target_usize (i : impl Into < u128 > , tcx : TyCtxt < '_ >) -> Option < Self > { Self :: try_from_uint (i , tcx . data_layout . pointer_size ()) } # [doc = " Try to convert this ScalarInt to the raw underlying bits."] # [doc = " Fails if the size is wrong. Generally a wrong size should lead to a panic,"] # [doc = " but Miri sometimes wants to be resilient to size mismatches,"] # [doc = " so the interpreter will generally use this `try` method."] # [inline] pub fn try_to_bits (self , target_size : Size) -> Result < u128 , Size > { assert_ne ! (target_size . bytes () , 0 , "you should never look at the bits of a ZST") ; if target_size . bytes () == u64 :: from (self . size . get ()) { self . check_data () ; Ok (self . data) } else { Err (self . size ()) } } # [inline] pub fn to_bits (self , target_size : Size) -> u128 { self . try_to_bits (target_size) . unwrap_or_else (| size | { bug ! ("expected int of size {}, but got size {}" , target_size . bytes () , size . bytes ()) }) } # [doc = " Extracts the bits from the scalar without checking the size."] # [inline] pub fn to_bits_unchecked (self) -> u128 { self . check_data () ; self . data } # [doc = " Converts the `ScalarInt` to an unsigned integer of the given size."] # [doc = " Panics if the size of the `ScalarInt` is not equal to `size`."] # [inline] pub fn to_uint (self , size : Size) -> u128 { self . to_bits (size) } # [doc = " Converts the `ScalarInt` to `u8`."] # [doc = " Panics if the `size` of the `ScalarInt`in not equal to 1 byte."] # [inline] pub fn to_u8 (self) -> u8 { self . to_uint (Size :: from_bits (8)) . try_into () . unwrap () } # [doc = " Converts the `ScalarInt` to `u16`."] # [doc = " Panics if the size of the `ScalarInt` in not equal to 2 bytes."] # [inline] pub fn to_u16 (self) -> u16 { self . to_uint (Size :: from_bits (16)) . try_into () . unwrap () } # [doc = " Converts the `ScalarInt` to `u32`."] # [doc = " Panics if the `size` of the `ScalarInt` in not equal to 4 bytes."] # [inline] pub fn to_u32 (self) -> u32 { self . to_uint (Size :: from_bits (32)) . try_into () . unwrap () } # [doc = " Converts the `ScalarInt` to `u64`."] # [doc = " Panics if the `size` of the `ScalarInt` in not equal to 8 bytes."] # [inline] pub fn to_u64 (self) -> u64 { self . to_uint (Size :: from_bits (64)) . try_into () . unwrap () } # [doc = " Converts the `ScalarInt` to `u128`."] # [doc = " Panics if the `size` of the `ScalarInt` in not equal to 16 bytes."] # [inline] pub fn to_u128 (self) -> u128 { self . to_uint (Size :: from_bits (128)) } # [inline] pub fn to_target_usize (& self , tcx : TyCtxt < '_ >) -> u64 { self . to_uint (tcx . data_layout . pointer_size ()) . try_into () . unwrap () } # [inline] pub fn to_atomic_ordering (self) -> AtomicOrdering { use AtomicOrdering :: * ; let val = self . to_u32 () ; if val == Relaxed as u32 { Relaxed } else if val == Release as u32 { Release } else if val == Acquire as u32 { Acquire } else if val == AcqRel as u32 { AcqRel } else if val == SeqCst as u32 { SeqCst } else { panic ! ("not a valid atomic ordering") } } # [doc = " Converts the `ScalarInt` to `bool`."] # [doc = " Panics if the `size` of the `ScalarInt` is not equal to 1 byte."] # [doc = " Errors if it is not a valid `bool`."] # [inline] pub fn try_to_bool (self) -> Result < bool , () > { match self . to_u8 () { 0 => Ok (false) , 1 => Ok (true) , _ => Err (()) , } } # [doc = " Converts the `ScalarInt` to a signed integer of the given size."] # [doc = " Panics if the size of the `ScalarInt` is not equal to `size`."] # [inline] pub fn to_int (self , size : Size) -> i128 { let b = self . to_bits (size) ; size . sign_extend (b) } # [doc = " Converts the `ScalarInt` to i8."] # [doc = " Panics if the size of the `ScalarInt` is not equal to 1 byte."] pub fn to_i8 (self) -> i8 { self . to_int (Size :: from_bits (8)) . try_into () . unwrap () } # [doc = " Converts the `ScalarInt` to i16."] # [doc = " Panics if the size of the `ScalarInt` is not equal to 2 bytes."] pub fn to_i16 (self) -> i16 { self . to_int (Size :: from_bits (16)) . try_into () . unwrap () } # [doc = " Converts the `ScalarInt` to i32."] # [doc = " Panics if the size of the `ScalarInt` is not equal to 4 bytes."] pub fn to_i32 (self) -> i32 { self . to_int (Size :: from_bits (32)) . try_into () . unwrap () } # [doc = " Converts the `ScalarInt` to i64."] # [doc = " Panics if the size of the `ScalarInt` is not equal to 8 bytes."] pub fn to_i64 (self) -> i64 { self . to_int (Size :: from_bits (64)) . try_into () . unwrap () } # [doc = " Converts the `ScalarInt` to i128."] # [doc = " Panics if the size of the `ScalarInt` is not equal to 16 bytes."] pub fn to_i128 (self) -> i128 { self . to_int (Size :: from_bits (128)) } # [inline] pub fn to_target_isize (& self , tcx : TyCtxt < '_ >) -> i64 { self . to_int (tcx . data_layout . pointer_size ()) . try_into () . unwrap () } # [inline] pub fn to_float < F : Float > (self) -> F { F :: from_bits (self . to_bits (Size :: from_bits (F :: BITS))) } # [inline] pub fn to_f16 (self) -> Half { self . to_float () } # [inline] pub fn to_f32 (self) -> Single { self . to_float () } # [inline] pub fn to_f64 (self) -> Double { self . to_float () } # [inline] pub fn to_f128 (self) -> Quad { self . to_float () } }}}
mkitem!{macro_rules ! from_x_for_scalar_int { ($ ($ ty : ty) ,*) => { $ (impl From <$ ty > for ScalarInt { # [inline] fn from (u : $ ty) -> Self { Self { data : u128 :: from (u) , size : NonZero :: new (size_of ::<$ ty > () as u8) . unwrap () , } } }) * } }}
mkitem!{macro_rules ! from_scalar_int_for_x { ($ ($ ty : ty) ,*) => { $ (impl From < ScalarInt > for $ ty { # [inline] fn from (int : ScalarInt) -> Self { int . to_uint (Size :: from_bytes (size_of ::<$ ty > ())) . try_into () . unwrap () } }) * } }}
mkitem!{from_x_for_scalar_int ! (u8 , u16 , u32 , u64 , u128 , bool) ;}
mkitem!{from_scalar_int_for_x ! (u8 , u16 , u32 , u64 , u128) ;}
mkitem!{mkimpl!{impl TryFrom < ScalarInt > for bool { type Error = () ; # [inline] fn try_from (int : ScalarInt) -> Result < Self , () > { int . try_to_bool () } }}}
mkitem!{mkimpl!{impl From < char > for ScalarInt { # [inline] fn from (c : char) -> Self { (c as u32) . into () } }}}
mkitem!{macro_rules ! from_x_for_scalar_int_signed { ($ ($ ty : ty) ,*) => { $ (impl From <$ ty > for ScalarInt { # [inline] fn from (u : $ ty) -> Self { Self { data : u128 :: from (u . cast_unsigned ()) , size : NonZero :: new (size_of ::<$ ty > () as u8) . unwrap () , } } }) * } }}
mkitem!{macro_rules ! from_scalar_int_for_x_signed { ($ ($ ty : ty) ,*) => { $ (impl From < ScalarInt > for $ ty { # [inline] fn from (int : ScalarInt) -> Self { int . to_int (Size :: from_bytes (size_of ::<$ ty > ())) . try_into () . unwrap () } }) * } }}
mkitem!{from_x_for_scalar_int_signed ! (i8 , i16 , i32 , i64 , i128) ;}
mkitem!{from_scalar_int_for_x_signed ! (i8 , i16 , i32 , i64 , i128) ;}
mkitem!{mkimpl!{impl From < std :: cmp :: Ordering > for ScalarInt { # [inline] fn from (c : std :: cmp :: Ordering) -> Self { ScalarInt :: from (c as i8) } }}}
mkitem!{mkstruct!{# [doc = " Error returned when a conversion from ScalarInt to char fails."] # [derive (Debug)] pub struct CharTryFromScalarInt ;}}
mkitem!{mkimpl!{impl TryFrom < ScalarInt > for char { type Error = CharTryFromScalarInt ; # [inline] fn try_from (int : ScalarInt) -> Result < Self , Self :: Error > { match char :: from_u32 (int . to_u32 ()) { Some (c) => Ok (c) , None => Err (CharTryFromScalarInt) , } } }}}
mkitem!{mkimpl!{impl From < Half > for ScalarInt { # [inline] fn from (f : Half) -> Self { Self { data : f . to_bits () , size : NonZero :: new ((Half :: BITS / 8) as u8) . unwrap () } } }}}
mkitem!{mkimpl!{impl From < ScalarInt > for Half { # [inline] fn from (int : ScalarInt) -> Self { Self :: from_bits (int . to_bits (Size :: from_bytes (2))) } }}}
mkitem!{mkimpl!{impl From < Single > for ScalarInt { # [inline] fn from (f : Single) -> Self { Self { data : f . to_bits () , size : NonZero :: new ((Single :: BITS / 8) as u8) . unwrap () } } }}}
mkitem!{mkimpl!{impl From < ScalarInt > for Single { # [inline] fn from (int : ScalarInt) -> Self { Self :: from_bits (int . to_bits (Size :: from_bytes (4))) } }}}
mkitem!{mkimpl!{impl From < Double > for ScalarInt { # [inline] fn from (f : Double) -> Self { Self { data : f . to_bits () , size : NonZero :: new ((Double :: BITS / 8) as u8) . unwrap () } } }}}
mkitem!{mkimpl!{impl From < ScalarInt > for Double { # [inline] fn from (int : ScalarInt) -> Self { Self :: from_bits (int . to_bits (Size :: from_bytes (8))) } }}}
mkitem!{mkimpl!{impl From < Quad > for ScalarInt { # [inline] fn from (f : Quad) -> Self { Self { data : f . to_bits () , size : NonZero :: new ((Quad :: BITS / 8) as u8) . unwrap () } } }}}
mkitem!{mkimpl!{impl From < ScalarInt > for Quad { # [inline] fn from (int : ScalarInt) -> Self { Self :: from_bits (int . to_bits (Size :: from_bytes (16))) } }}}
mkitem!{mkimpl!{impl fmt :: Debug for ScalarInt { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "0x{self:x}") } }}}
mkitem!{mkimpl!{impl fmt :: LowerHex for ScalarInt { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . check_data () ; if f . alternate () { write ! (f , "0x") ? ; } write ! (f , "{:01$x}" , { self . data } , self . size . get () as usize * 2) } }}}
mkitem!{mkimpl!{impl fmt :: UpperHex for ScalarInt { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . check_data () ; write ! (f , "{:01$X}" , { self . data } , self . size . get () as usize * 2) } }}}
mkitem!{mkimpl!{impl fmt :: Display for ScalarInt { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . check_data () ; write ! (f , "{}" , { self . data }) } }}}