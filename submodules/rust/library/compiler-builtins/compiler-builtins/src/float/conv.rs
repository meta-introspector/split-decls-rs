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
mkuse!{use core :: ops :: Neg ;}
mkuse!{use super :: Float ;}
mkuse!{use crate :: int :: { CastFrom , CastInto , Int , MinInt } ;}
mkmod!{int_to_float, { 
                getname!(int_to_float);
                getsrc!(int_to_float);
                getpath!(int_to_float);
                get_deps!(int_to_float);
                get_crates!(int_to_float);
                mkinclude!(int_to_float);
                mkuse!{use super :: * ;}

macro_rules! exp_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exp in module {}", module_path!());
    };
}

mkfn!{
    exp_introspect!();
    #[doc = " Calculate the exponent from the number of leading zeros."] #[doc = ""] #[doc = " Usually 1 is subtracted from this function's result, so that a mantissa with the implicit"] #[doc = " bit set can be added back later."] fn exp < I : Int , F : Float < Int : CastFrom < u32 > > > (n : u32) -> F :: Int { F :: Int :: cast_from (F :: EXP_BIAS - 1 + I :: BITS - n) }
}

macro_rules! m_adj_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function m_adj in module {}", module_path!());
    };
}

mkfn!{
    m_adj_introspect!();
    #[doc = " Adjust a mantissa with dropped bits to perform correct rounding."] #[doc = ""] #[doc = " The dropped bits should be exactly the bits that get truncated (left-aligned), but they"] #[doc = " can be combined or compressed in some way that simplifies operations."] fn m_adj < F : Float > (m_base : F :: Int , dropped_bits : F :: Int) -> F :: Int { let adj = (dropped_bits - ((dropped_bits >> (F :: BITS - 1)) & ! m_base)) >> (F :: BITS - 1) ; m_base + adj }
}

macro_rules! repr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function repr in module {}", module_path!());
    };
}

mkfn!{
    repr_introspect!();
    #[doc = " Shift the exponent to its position and add the mantissa."] #[doc = ""] #[doc = " If the mantissa has the implicit bit set, the exponent should be one less than its actual"] #[doc = " value to cancel it out."] fn repr < F : Float > (e : F :: Int , m : F :: Int) -> F :: Int { (e << F :: SIG_BITS) + m }
}

macro_rules! shift_f_lt_i_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function shift_f_lt_i in module {}", module_path!());
    };
}

mkfn!{
    shift_f_lt_i_introspect!();
    #[doc = " Shift distance from a left-aligned integer to a smaller float."] fn shift_f_lt_i < I : Int , F : Float > () -> u32 { (I :: BITS - F :: BITS) + F :: EXP_BITS }
}

macro_rules! shift_f_gt_i_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function shift_f_gt_i in module {}", module_path!());
    };
}

mkfn!{
    shift_f_gt_i_introspect!();
    #[doc = " Shift distance from an integer with `n` leading zeros to a smaller float."] fn shift_f_gt_i < I : Int , F : Float > (n : u32) -> u32 { F :: SIG_BITS - I :: BITS + 1 + n }
}

macro_rules! signed_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function signed in module {}", module_path!());
    };
}

mkfn!{
    signed_introspect!();
    #[doc = " Perform a signed operation as unsigned, then add the sign back."] pub fn signed < I , F , Conv > (i : I , conv : Conv) -> F where F : Float , I : Int , F :: Int : CastFrom < I > , Conv : Fn (I :: Unsigned) -> F :: Int , { let sign_bit = F :: Int :: cast_from_lossy (i >> (I :: BITS - 1)) << (F :: BITS - 1) ; F :: from_bits (conv (i . unsigned_abs ()) | sign_bit) }
}

macro_rules! u32_to_f32_bits_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u32_to_f32_bits in module {}", module_path!());
    };
}

mkfn!{
    u32_to_f32_bits_introspect!();
    pub fn u32_to_f32_bits (i : u32) -> u32 { if i == 0 { return 0 ; } let n = i . leading_zeros () ; let m_base = (i << n) >> f32 :: EXP_BITS ; let adj = (i << n) << (f32 :: SIG_BITS + 1) ; let m = m_adj :: < f32 > (m_base , adj) ; let e = exp :: < u32 , f32 > (n) - 1 ; repr :: < f32 > (e , m) }
}

macro_rules! u32_to_f64_bits_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u32_to_f64_bits in module {}", module_path!());
    };
}

mkfn!{
    u32_to_f64_bits_introspect!();
    pub fn u32_to_f64_bits (i : u32) -> u64 { if i == 0 { return 0 ; } let n = i . leading_zeros () ; let m = (i as u64) << shift_f_gt_i :: < u32 , f64 > (n) ; let e = exp :: < u32 , f64 > (n) - 1 ; repr :: < f64 > (e , m) }
}

macro_rules! u32_to_f128_bits_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u32_to_f128_bits in module {}", module_path!());
    };
}

mkfn!{
    u32_to_f128_bits_introspect!();
    #[cfg (f128_enabled)] pub fn u32_to_f128_bits (i : u32) -> u128 { if i == 0 { return 0 ; } let n = i . leading_zeros () ; let m = (i as u64) << (shift_f_gt_i :: < u32 , f128 > (n) - 64) ; let e = exp :: < u32 , f128 > (n) as u64 - 1 ; let h = (e << (f128 :: SIG_BITS - 64)) + m ; (h as u128) << 64 }
}

macro_rules! u64_to_f32_bits_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u64_to_f32_bits in module {}", module_path!());
    };
}

mkfn!{
    u64_to_f32_bits_introspect!();
    pub fn u64_to_f32_bits (i : u64) -> u32 { let n = i . leading_zeros () ; let i_m = i . wrapping_shl (n) ; let m_base : u32 = (i_m >> shift_f_lt_i :: < u64 , f32 > ()) as u32 ; let adj = ((i_m >> f32 :: EXP_BITS) | i_m & 0xFFFF) as u32 ; let m = m_adj :: < f32 > (m_base , adj) ; let e = if i == 0 { 0 } else { exp :: < u64 , f32 > (n) - 1 } ; repr :: < f32 > (e , m) }
}

macro_rules! u64_to_f64_bits_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u64_to_f64_bits in module {}", module_path!());
    };
}

mkfn!{
    u64_to_f64_bits_introspect!();
    pub fn u64_to_f64_bits (i : u64) -> u64 { if i == 0 { return 0 ; } let n = i . leading_zeros () ; let m_base = (i << n) >> f64 :: EXP_BITS ; let adj = (i << n) << (f64 :: SIG_BITS + 1) ; let m = m_adj :: < f64 > (m_base , adj) ; let e = exp :: < u64 , f64 > (n) - 1 ; repr :: < f64 > (e , m) }
}

macro_rules! u64_to_f128_bits_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u64_to_f128_bits in module {}", module_path!());
    };
}

mkfn!{
    u64_to_f128_bits_introspect!();
    #[cfg (f128_enabled)] pub fn u64_to_f128_bits (i : u64) -> u128 { if i == 0 { return 0 ; } let n = i . leading_zeros () ; let m = (i as u128) << shift_f_gt_i :: < u64 , f128 > (n) ; let e = exp :: < u64 , f128 > (n) - 1 ; repr :: < f128 > (e , m) }
}

macro_rules! u128_to_f32_bits_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u128_to_f32_bits in module {}", module_path!());
    };
}

mkfn!{
    u128_to_f32_bits_introspect!();
    pub fn u128_to_f32_bits (i : u128) -> u32 { let n = i . leading_zeros () ; let i_m = i . wrapping_shl (n) ; let m_base : u32 = (i_m >> shift_f_lt_i :: < u128 , f32 > ()) as u32 ; let d1 : u32 = (i_m >> (u128 :: BITS - f32 :: BITS - f32 :: SIG_BITS - 1)) . cast_lossy () ; let d2 : u32 = (i_m << f32 :: BITS >> f32 :: BITS != 0) . into () ; let adj = d1 | d2 ; let m = m_adj :: < f32 > (m_base , adj) ; let e = if i == 0 { 0 } else { exp :: < u128 , f32 > (n) - 1 } ; repr :: < f32 > (e , m) }
}

macro_rules! u128_to_f64_bits_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u128_to_f64_bits in module {}", module_path!());
    };
}

mkfn!{
    u128_to_f64_bits_introspect!();
    pub fn u128_to_f64_bits (i : u128) -> u64 { let n = i . leading_zeros () ; let i_m = i . wrapping_shl (n) ; let m_base : u64 = (i_m >> shift_f_lt_i :: < u128 , f64 > ()) as u64 ; let adj = ((i_m >> f64 :: EXP_BITS) | i_m & 0xFFFF_FFFF) as u64 ; let m = m_adj :: < f64 > (m_base , adj) ; let e = if i == 0 { 0 } else { exp :: < u128 , f64 > (n) - 1 } ; repr :: < f64 > (e , m) }
}

macro_rules! u128_to_f128_bits_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u128_to_f128_bits in module {}", module_path!());
    };
}

mkfn!{
    u128_to_f128_bits_introspect!();
    #[cfg (f128_enabled)] pub fn u128_to_f128_bits (i : u128) -> u128 { if i == 0 { return 0 ; } let n = i . leading_zeros () ; let m_base = (i << n) >> f128 :: EXP_BITS ; let adj = (i << n) << (f128 :: SIG_BITS + 1) ; let m = m_adj :: < f128 > (m_base , adj) ; let e = exp :: < u128 , f128 > (n) - 1 ; repr :: < f128 > (e , m) }
} 
            }}
mkitem!{intrinsics ! { #[arm_aeabi_alias = __aeabi_ui2f] pub extern "C" fn __floatunsisf (i : u32) -> f32 { f32 :: from_bits (int_to_float :: u32_to_f32_bits (i)) } #[arm_aeabi_alias = __aeabi_ui2d] pub extern "C" fn __floatunsidf (i : u32) -> f64 { f64 :: from_bits (int_to_float :: u32_to_f64_bits (i)) } #[arm_aeabi_alias = __aeabi_ul2f] pub extern "C" fn __floatundisf (i : u64) -> f32 { f32 :: from_bits (int_to_float :: u64_to_f32_bits (i)) } #[arm_aeabi_alias = __aeabi_ul2d] pub extern "C" fn __floatundidf (i : u64) -> f64 { f64 :: from_bits (int_to_float :: u64_to_f64_bits (i)) } #[cfg_attr (target_os = "uefi" , unadjusted_on_win64)] pub extern "C" fn __floatuntisf (i : u128) -> f32 { f32 :: from_bits (int_to_float :: u128_to_f32_bits (i)) } #[cfg_attr (target_os = "uefi" , unadjusted_on_win64)] pub extern "C" fn __floatuntidf (i : u128) -> f64 { f64 :: from_bits (int_to_float :: u128_to_f64_bits (i)) } #[ppc_alias = __floatunsikf] #[cfg (f128_enabled)] pub extern "C" fn __floatunsitf (i : u32) -> f128 { f128 :: from_bits (int_to_float :: u32_to_f128_bits (i)) } #[ppc_alias = __floatundikf] #[cfg (f128_enabled)] pub extern "C" fn __floatunditf (i : u64) -> f128 { f128 :: from_bits (int_to_float :: u64_to_f128_bits (i)) } #[ppc_alias = __floatuntikf] #[cfg (f128_enabled)] pub extern "C" fn __floatuntitf (i : u128) -> f128 { f128 :: from_bits (int_to_float :: u128_to_f128_bits (i)) } }}
mkitem!{intrinsics ! { #[arm_aeabi_alias = __aeabi_i2f] pub extern "C" fn __floatsisf (i : i32) -> f32 { int_to_float :: signed (i , int_to_float :: u32_to_f32_bits) } #[arm_aeabi_alias = __aeabi_i2d] pub extern "C" fn __floatsidf (i : i32) -> f64 { int_to_float :: signed (i , int_to_float :: u32_to_f64_bits) } #[arm_aeabi_alias = __aeabi_l2f] pub extern "C" fn __floatdisf (i : i64) -> f32 { int_to_float :: signed (i , int_to_float :: u64_to_f32_bits) } #[arm_aeabi_alias = __aeabi_l2d] pub extern "C" fn __floatdidf (i : i64) -> f64 { int_to_float :: signed (i , int_to_float :: u64_to_f64_bits) } #[cfg_attr (target_os = "uefi" , unadjusted_on_win64)] pub extern "C" fn __floattisf (i : i128) -> f32 { int_to_float :: signed (i , int_to_float :: u128_to_f32_bits) } #[cfg_attr (target_os = "uefi" , unadjusted_on_win64)] pub extern "C" fn __floattidf (i : i128) -> f64 { int_to_float :: signed (i , int_to_float :: u128_to_f64_bits) } #[ppc_alias = __floatsikf] #[cfg (f128_enabled)] pub extern "C" fn __floatsitf (i : i32) -> f128 { int_to_float :: signed (i , int_to_float :: u32_to_f128_bits) } #[ppc_alias = __floatdikf] #[cfg (f128_enabled)] pub extern "C" fn __floatditf (i : i64) -> f128 { int_to_float :: signed (i , int_to_float :: u64_to_f128_bits) } #[ppc_alias = __floattikf] #[cfg (f128_enabled)] pub extern "C" fn __floattitf (i : i128) -> f128 { int_to_float :: signed (i , int_to_float :: u128_to_f128_bits) } }}

macro_rules! float_to_unsigned_int_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function float_to_unsigned_int in module {}", module_path!());
    };
}

mkfn!{
    float_to_unsigned_int_introspect!();
    #[doc = " Generic float to unsigned int conversions."] fn float_to_unsigned_int < F , U > (f : F) -> U where F : Float , U : Int < Unsigned = U > , F :: Int : CastInto < U > , F :: Int : CastFrom < u32 > , F :: Int : CastInto < U :: Unsigned > , u32 : CastFrom < F :: Int > , { float_to_int_inner :: < F , U , _ , _ > (f . to_bits () , | i : U | i , | | U :: MAX) }
}

macro_rules! float_to_signed_int_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function float_to_signed_int in module {}", module_path!());
    };
}

mkfn!{
    float_to_signed_int_introspect!();
    #[doc = " Generic float to signed int conversions."] fn float_to_signed_int < F , I > (f : F) -> I where F : Float , I : Int + Neg < Output = I > , I :: Unsigned : Int , F :: Int : CastInto < I :: Unsigned > , F :: Int : CastFrom < u32 > , u32 : CastFrom < F :: Int > , { float_to_int_inner :: < F , I , _ , _ > (f . to_bits () & ! F :: SIGN_MASK , | i : I | if f . is_sign_negative () { - i } else { i } , | | if f . is_sign_negative () { I :: MIN } else { I :: MAX } ,) }
}

macro_rules! float_to_int_inner_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function float_to_int_inner in module {}", module_path!());
    };
}

mkfn!{
    float_to_int_inner_introspect!();
    #[doc = " Float to int conversions, generic for both signed and unsigned."] #[doc = ""] #[doc = " Parameters:"] #[doc = " - `fbits`: `abg(f)` bitcasted to an integer."] #[doc = " - `map_inbounds`: apply this transformation to integers that are within range (add the sign back)."] #[doc = " - `out_of_bounds`: return value when out of range for `I`."] fn float_to_int_inner < F , I , FnFoo , FnOob > (fbits : F :: Int , map_inbounds : FnFoo , out_of_bounds : FnOob ,) -> I where F : Float , I : Int , FnFoo : FnOnce (I) -> I , FnOob : FnOnce () -> I , I :: Unsigned : Int , F :: Int : CastInto < I :: Unsigned > , F :: Int : CastFrom < u32 > , u32 : CastFrom < F :: Int > , { let int_max_exp = F :: EXP_BIAS + I :: MAX . ilog2 () + 1 ; let foobar = F :: EXP_BIAS + I :: Unsigned :: BITS - 1 ; if fbits < F :: ONE . to_bits () { I :: ZERO } else if fbits < F :: Int :: cast_from (int_max_exp) << F :: SIG_BITS { let m_base = if I :: Unsigned :: BITS >= F :: Int :: BITS { I :: Unsigned :: cast_from (fbits) << (I :: BITS - F :: SIG_BITS - 1) } else { I :: Unsigned :: cast_from_lossy (fbits >> (F :: SIG_BITS - I :: BITS + 1)) } ; let m : I :: Unsigned = (I :: Unsigned :: ONE << (I :: BITS - 1)) | m_base ; let s : u32 = (foobar) - u32 :: cast_from (fbits >> F :: SIG_BITS) ; let unsigned = m >> s ; map_inbounds (I :: from_unsigned (unsigned)) } else if fbits <= F :: EXP_MASK { out_of_bounds () } else { I :: ZERO } }
}
mkitem!{intrinsics ! { #[arm_aeabi_alias = __aeabi_f2uiz] pub extern "C" fn __fixunssfsi (f : f32) -> u32 { float_to_unsigned_int (f) } #[arm_aeabi_alias = __aeabi_f2ulz] pub extern "C" fn __fixunssfdi (f : f32) -> u64 { float_to_unsigned_int (f) } pub extern "C" fn __fixunssfti (f : f32) -> u128 { float_to_unsigned_int (f) } #[arm_aeabi_alias = __aeabi_d2uiz] pub extern "C" fn __fixunsdfsi (f : f64) -> u32 { float_to_unsigned_int (f) } #[arm_aeabi_alias = __aeabi_d2ulz] pub extern "C" fn __fixunsdfdi (f : f64) -> u64 { float_to_unsigned_int (f) } pub extern "C" fn __fixunsdfti (f : f64) -> u128 { float_to_unsigned_int (f) } #[ppc_alias = __fixunskfsi] #[cfg (f128_enabled)] pub extern "C" fn __fixunstfsi (f : f128) -> u32 { float_to_unsigned_int (f) } #[ppc_alias = __fixunskfdi] #[cfg (f128_enabled)] pub extern "C" fn __fixunstfdi (f : f128) -> u64 { float_to_unsigned_int (f) } #[ppc_alias = __fixunskfti] #[cfg (f128_enabled)] pub extern "C" fn __fixunstfti (f : f128) -> u128 { float_to_unsigned_int (f) } }}
mkitem!{intrinsics ! { #[arm_aeabi_alias = __aeabi_f2iz] pub extern "C" fn __fixsfsi (f : f32) -> i32 { float_to_signed_int (f) } #[arm_aeabi_alias = __aeabi_f2lz] pub extern "C" fn __fixsfdi (f : f32) -> i64 { float_to_signed_int (f) } pub extern "C" fn __fixsfti (f : f32) -> i128 { float_to_signed_int (f) } #[arm_aeabi_alias = __aeabi_d2iz] pub extern "C" fn __fixdfsi (f : f64) -> i32 { float_to_signed_int (f) } #[arm_aeabi_alias = __aeabi_d2lz] pub extern "C" fn __fixdfdi (f : f64) -> i64 { float_to_signed_int (f) } pub extern "C" fn __fixdfti (f : f64) -> i128 { float_to_signed_int (f) } #[ppc_alias = __fixkfsi] #[cfg (f128_enabled)] pub extern "C" fn __fixtfsi (f : f128) -> i32 { float_to_signed_int (f) } #[ppc_alias = __fixkfdi] #[cfg (f128_enabled)] pub extern "C" fn __fixtfdi (f : f128) -> i64 { float_to_signed_int (f) } #[ppc_alias = __fixkfti] #[cfg (f128_enabled)] pub extern "C" fn __fixtfti (f : f128) -> i128 { float_to_signed_int (f) } }}