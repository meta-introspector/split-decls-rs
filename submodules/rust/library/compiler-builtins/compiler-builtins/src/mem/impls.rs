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
mkuse!{use core :: ffi :: c_int ;}
mkuse!{use core :: intrinsics :: likely ;}
mkitem!{const WORD_SIZE : usize = core :: mem :: size_of :: < usize > () ;}
mkitem!{const WORD_MASK : usize = WORD_SIZE - 1 ;}
mkitem!{const WORD_COPY_THRESHOLD : usize = if 2 * WORD_SIZE > 16 { 2 * WORD_SIZE } else { 16 } ;}

macro_rules! read_usize_unaligned_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function read_usize_unaligned in module {}", module_path!());
    };
}

mkfn!{
    read_usize_unaligned_introspect!();
    #[cfg (feature = "mem-unaligned")] unsafe fn read_usize_unaligned (x : * const usize) -> usize { let x_read = (x as * const [u8 ; core :: mem :: size_of :: < usize > ()]) . read () ; usize :: from_ne_bytes (x_read) }
}

macro_rules! load_chunk_aligned_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function load_chunk_aligned in module {}", module_path!());
    };
}

mkfn!{
    load_chunk_aligned_introspect!();
    #[doc = " Loads a `T`-sized chunk from `src` into `dst` at offset `offset`, if that does not exceed"] #[doc = " `load_sz`. The offset pointers must both be `T`-aligned. Returns the new offset, advanced by the"] #[doc = " chunk size if a load happened."] #[cfg (not (feature = "mem-unaligned"))] #[inline (always)] unsafe fn load_chunk_aligned < T : Copy > (src : * const usize , dst : * mut usize , load_sz : usize , offset : usize ,) -> usize { let chunk_sz = core :: mem :: size_of :: < T > () ; if (load_sz & chunk_sz) != 0 { * dst . wrapping_byte_add (offset) . cast :: < T > () = * src . wrapping_byte_add (offset) . cast :: < T > () ; offset | chunk_sz } else { offset } }
}

macro_rules! load_aligned_partial_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function load_aligned_partial in module {}", module_path!());
    };
}

mkfn!{
    load_aligned_partial_introspect!();
    #[doc = " Load `load_sz` many bytes from `src`, which must be usize-aligned. Acts as if we did a `usize`"] #[doc = " read with the out-of-bounds part filled with 0s."] #[doc = " `load_sz` be strictly less than `WORD_SIZE`."] #[cfg (not (feature = "mem-unaligned"))] #[inline (always)] unsafe fn load_aligned_partial (src : * const usize , load_sz : usize) -> usize { debug_assert ! (load_sz < WORD_SIZE) ; const { assert ! (WORD_SIZE <= 8) } ; let mut i = 0 ; let mut out = 0usize ; i = load_chunk_aligned :: < u32 > (src , & raw mut out , load_sz , i) ; i = load_chunk_aligned :: < u16 > (src , & raw mut out , load_sz , i) ; i = load_chunk_aligned :: < u8 > (src , & raw mut out , load_sz , i) ; debug_assert ! (i == load_sz) ; out }
}

macro_rules! load_aligned_end_partial_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function load_aligned_end_partial in module {}", module_path!());
    };
}

mkfn!{
    load_aligned_end_partial_introspect!();
    #[doc = " Load `load_sz` many bytes from `src.wrapping_byte_add(WORD_SIZE - load_sz)`. `src` must be"] #[doc = " `usize`-aligned. The bytes are returned as the *last* bytes of the return value, i.e., this acts"] #[doc = " as if we had done a `usize` read from `src`, with the out-of-bounds part filled with 0s."] #[doc = " `load_sz` be strictly less than `WORD_SIZE`."] #[cfg (not (feature = "mem-unaligned"))] #[inline (always)] unsafe fn load_aligned_end_partial (src : * const usize , load_sz : usize) -> usize { debug_assert ! (load_sz < WORD_SIZE) ; const { assert ! (WORD_SIZE <= 8) } ; let mut i = 0 ; let mut out = 0usize ; let src_shifted = src . wrapping_byte_add (WORD_SIZE - load_sz) ; let out_shifted = (& raw mut out) . wrapping_byte_add (WORD_SIZE - load_sz) ; i = load_chunk_aligned :: < u8 > (src_shifted , out_shifted , load_sz , i) ; i = load_chunk_aligned :: < u16 > (src_shifted , out_shifted , load_sz , i) ; i = load_chunk_aligned :: < u32 > (src_shifted , out_shifted , load_sz , i) ; debug_assert ! (i == load_sz) ; out }
}

macro_rules! copy_forward_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copy_forward in module {}", module_path!());
    };
}

mkfn!{
    copy_forward_introspect!();
    #[inline (always)] pub unsafe fn copy_forward (mut dest : * mut u8 , mut src : * const u8 , mut n : usize) { #[inline (always)] unsafe fn copy_forward_bytes (mut dest : * mut u8 , mut src : * const u8 , n : usize) { let dest_end = dest . wrapping_add (n) ; while dest < dest_end { * dest = * src ; dest = dest . wrapping_add (1) ; src = src . wrapping_add (1) ; } } #[inline (always)] unsafe fn copy_forward_aligned_words (dest : * mut u8 , src : * const u8 , n : usize) { let mut dest_usize = dest as * mut usize ; let mut src_usize = src as * mut usize ; let dest_end = dest . wrapping_add (n) as * mut usize ; while dest_usize < dest_end { * dest_usize = * src_usize ; dest_usize = dest_usize . wrapping_add (1) ; src_usize = src_usize . wrapping_add (1) ; } } #[doc = " `n` is in units of bytes, but must be a multiple of the word size and must not be 0."] #[doc = " `src` *must not* be `usize`-aligned."] #[cfg (not (feature = "mem-unaligned"))] #[inline (always)] unsafe fn copy_forward_misaligned_words (dest : * mut u8 , src : * const u8 , n : usize) { debug_assert ! (n > 0 && n % WORD_SIZE == 0) ; debug_assert ! (src . addr () % WORD_SIZE != 0) ; let mut dest_usize = dest as * mut usize ; let dest_end = dest . wrapping_add (n) as * mut usize ; let offset = src as usize & WORD_MASK ; let shift = offset * 8 ; let mut src_aligned = src . wrapping_byte_sub (offset) as * mut usize ; let mut prev_word = load_aligned_end_partial (src_aligned , WORD_SIZE - offset) ; while dest_usize . wrapping_add (1) < dest_end { src_aligned = src_aligned . wrapping_add (1) ; let cur_word = * src_aligned ; let reassembled = if cfg ! (target_endian = "little") { prev_word >> shift | cur_word << (WORD_SIZE * 8 - shift) } else { prev_word << shift | cur_word >> (WORD_SIZE * 8 - shift) } ; prev_word = cur_word ; * dest_usize = reassembled ; dest_usize = dest_usize . wrapping_add (1) ; } src_aligned = src_aligned . wrapping_add (1) ; let cur_word = load_aligned_partial (src_aligned , offset) ; let reassembled = if cfg ! (target_endian = "little") { prev_word >> shift | cur_word << (WORD_SIZE * 8 - shift) } else { prev_word << shift | cur_word >> (WORD_SIZE * 8 - shift) } ; * dest_usize = reassembled ; } #[doc = " `n` is in units of bytes, but must be a multiple of the word size and must not be 0."] #[doc = " `src` *must not* be `usize`-aligned."] #[cfg (feature = "mem-unaligned")] #[inline (always)] unsafe fn copy_forward_misaligned_words (dest : * mut u8 , src : * const u8 , n : usize) { let mut dest_usize = dest as * mut usize ; let mut src_usize = src as * mut usize ; let dest_end = dest . wrapping_add (n) as * mut usize ; while dest_usize < dest_end { * dest_usize = read_usize_unaligned (src_usize) ; dest_usize = dest_usize . wrapping_add (1) ; src_usize = src_usize . wrapping_add (1) ; } } if n >= WORD_COPY_THRESHOLD { let dest_misalignment = (dest as usize) . wrapping_neg () & WORD_MASK ; copy_forward_bytes (dest , src , dest_misalignment) ; dest = dest . wrapping_add (dest_misalignment) ; src = src . wrapping_add (dest_misalignment) ; n -= dest_misalignment ; let n_words = n & ! WORD_MASK ; let src_misalignment = src as usize & WORD_MASK ; if likely (src_misalignment == 0) { copy_forward_aligned_words (dest , src , n_words) ; } else { copy_forward_misaligned_words (dest , src , n_words) ; } dest = dest . wrapping_add (n_words) ; src = src . wrapping_add (n_words) ; n -= n_words ; } copy_forward_bytes (dest , src , n) ; }
}

macro_rules! copy_backward_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copy_backward in module {}", module_path!());
    };
}

mkfn!{
    copy_backward_introspect!();
    #[inline (always)] pub unsafe fn copy_backward (dest : * mut u8 , src : * const u8 , mut n : usize) { #[inline (always)] unsafe fn copy_backward_bytes (mut dest : * mut u8 , mut src : * const u8 , n : usize) { let dest_start = dest . wrapping_sub (n) ; while dest_start < dest { dest = dest . wrapping_sub (1) ; src = src . wrapping_sub (1) ; * dest = * src ; } } #[inline (always)] unsafe fn copy_backward_aligned_words (dest : * mut u8 , src : * const u8 , n : usize) { let mut dest_usize = dest as * mut usize ; let mut src_usize = src as * mut usize ; let dest_start = dest . wrapping_sub (n) as * mut usize ; while dest_start < dest_usize { dest_usize = dest_usize . wrapping_sub (1) ; src_usize = src_usize . wrapping_sub (1) ; * dest_usize = * src_usize ; } } #[doc = " `n` is in units of bytes, but must be a multiple of the word size and must not be 0."] #[doc = " `src` *must not* be `usize`-aligned."] #[cfg (not (feature = "mem-unaligned"))] #[inline (always)] unsafe fn copy_backward_misaligned_words (dest : * mut u8 , src : * const u8 , n : usize) { debug_assert ! (n > 0 && n % WORD_SIZE == 0) ; debug_assert ! (src . addr () % WORD_SIZE != 0) ; let mut dest_usize = dest as * mut usize ; let dest_start = dest . wrapping_sub (n) as * mut usize ; let offset = src as usize & WORD_MASK ; let shift = offset * 8 ; let mut src_aligned = src . wrapping_byte_sub (offset) as * mut usize ; let mut prev_word = load_aligned_partial (src_aligned , offset) ; while dest_start . wrapping_add (1) < dest_usize { src_aligned = src_aligned . wrapping_sub (1) ; let cur_word = * src_aligned ; let reassembled = if cfg ! (target_endian = "little") { prev_word << (WORD_SIZE * 8 - shift) | cur_word >> shift } else { prev_word >> (WORD_SIZE * 8 - shift) | cur_word << shift } ; prev_word = cur_word ; dest_usize = dest_usize . wrapping_sub (1) ; * dest_usize = reassembled ; } src_aligned = src_aligned . wrapping_sub (1) ; let cur_word = load_aligned_end_partial (src_aligned , WORD_SIZE - offset) ; let reassembled = if cfg ! (target_endian = "little") { prev_word << (WORD_SIZE * 8 - shift) | cur_word >> shift } else { prev_word >> (WORD_SIZE * 8 - shift) | cur_word << shift } ; dest_usize = dest_usize . wrapping_sub (1) ; * dest_usize = reassembled ; } #[doc = " `n` is in units of bytes, but must be a multiple of the word size and must not be 0."] #[doc = " `src` *must not* be `usize`-aligned."] #[cfg (feature = "mem-unaligned")] #[inline (always)] unsafe fn copy_backward_misaligned_words (dest : * mut u8 , src : * const u8 , n : usize) { let mut dest_usize = dest as * mut usize ; let mut src_usize = src as * mut usize ; let dest_start = dest . wrapping_sub (n) as * mut usize ; while dest_start < dest_usize { dest_usize = dest_usize . wrapping_sub (1) ; src_usize = src_usize . wrapping_sub (1) ; * dest_usize = read_usize_unaligned (src_usize) ; } } let mut dest = dest . wrapping_add (n) ; let mut src = src . wrapping_add (n) ; if n >= WORD_COPY_THRESHOLD { let dest_misalignment = dest as usize & WORD_MASK ; copy_backward_bytes (dest , src , dest_misalignment) ; dest = dest . wrapping_sub (dest_misalignment) ; src = src . wrapping_sub (dest_misalignment) ; n -= dest_misalignment ; let n_words = n & ! WORD_MASK ; let src_misalignment = src as usize & WORD_MASK ; if likely (src_misalignment == 0) { copy_backward_aligned_words (dest , src , n_words) ; } else { copy_backward_misaligned_words (dest , src , n_words) ; } dest = dest . wrapping_sub (n_words) ; src = src . wrapping_sub (n_words) ; n -= n_words ; } copy_backward_bytes (dest , src , n) ; }
}

macro_rules! set_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_bytes in module {}", module_path!());
    };
}

mkfn!{
    set_bytes_introspect!();
    #[inline (always)] pub unsafe fn set_bytes (mut s : * mut u8 , c : u8 , mut n : usize) { #[inline (always)] pub unsafe fn set_bytes_bytes (mut s : * mut u8 , c : u8 , n : usize) { let end = s . wrapping_add (n) ; while s < end { * s = c ; s = s . wrapping_add (1) ; } } #[inline (always)] pub unsafe fn set_bytes_words (s : * mut u8 , c : u8 , n : usize) { let mut broadcast = c as usize ; let mut bits = 8 ; while bits < WORD_SIZE * 8 { broadcast |= broadcast << bits ; bits *= 2 ; } let mut s_usize = s as * mut usize ; let end = s . wrapping_add (n) as * mut usize ; while s_usize < end { * s_usize = broadcast ; s_usize = s_usize . wrapping_add (1) ; } } if likely (n >= WORD_COPY_THRESHOLD) { let misalignment = (s as usize) . wrapping_neg () & WORD_MASK ; set_bytes_bytes (s , c , misalignment) ; s = s . wrapping_add (misalignment) ; n -= misalignment ; let n_words = n & ! WORD_MASK ; set_bytes_words (s , c , n_words) ; s = s . wrapping_add (n_words) ; n -= n_words ; } set_bytes_bytes (s , c , n) ; }
}

macro_rules! compare_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compare_bytes in module {}", module_path!());
    };
}

mkfn!{
    compare_bytes_introspect!();
    #[inline (always)] pub unsafe fn compare_bytes (s1 : * const u8 , s2 : * const u8 , n : usize) -> c_int { let mut i = 0 ; while i < n { let a = * s1 . wrapping_add (i) ; let b = * s2 . wrapping_add (i) ; if a != b { return c_int :: from (a) - c_int :: from (b) ; } i += 1 ; } 0 }
}

macro_rules! c_string_length_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function c_string_length in module {}", module_path!());
    };
}

mkfn!{
    c_string_length_introspect!();
    #[inline (always)] pub unsafe fn c_string_length (mut s : * const core :: ffi :: c_char) -> usize { let mut n = 0 ; while * s != 0 { n += 1 ; s = s . wrapping_add (1) ; } n }
}