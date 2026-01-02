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
mkuse!{use crate :: cmp ;}
mkuse!{use crate :: io :: { BorrowedCursor , Error as IoError , ErrorKind , IoSlice , IoSliceMut , Result as IoResult , } ;}
mkuse!{use crate :: random :: random ;}
mkuse!{use crate :: time :: { Duration , Instant } ;}
mkmod!{alloc, { 
                getname!(alloc);
                getsrc!(alloc);
                getpath!(alloc);
                get_deps!(alloc);
                get_crates!(alloc);
                mkinclude!(alloc);
                 
            }}
mkmod!{raw, { 
                getname!(raw);
                getsrc!(raw);
                getpath!(raw);
                get_deps!(raw);
                get_crates!(raw);
                mkinclude!(raw);
                 
            }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkuse!{use self :: raw :: * ;}

macro_rules! read_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function read in module {}", module_path!());
    };
}

mkfn!{
    read_introspect!();
    #[doc = " Usercall `read`. See the ABI documentation for more information."] #[doc = ""] #[doc = " This will do a single `read` usercall and scatter the read data among"] #[doc = " `bufs`. To read to a single buffer, just pass a slice of length one."] #[unstable (feature = "sgx_platform" , issue = "56975")] pub fn read (fd : Fd , bufs : & mut [IoSliceMut < '_ >]) -> IoResult < usize > { unsafe { let total_len = bufs . iter () . fold (0usize , | sum , buf | sum . saturating_add (buf . len ())) ; let mut userbuf = alloc :: User :: < [u8] > :: uninitialized (total_len) ; let ret_len = raw :: read (fd , userbuf . as_mut_ptr () , userbuf . len ()) . from_sgx_result () ? ; let userbuf = & userbuf [.. ret_len] ; let mut index = 0 ; for buf in bufs { let end = cmp :: min (index + buf . len () , userbuf . len ()) ; if let Some (buflen) = end . checked_sub (index) { userbuf [index .. end] . copy_to_enclave (& mut buf [.. buflen]) ; index += buf . len () ; } else { break ; } } Ok (userbuf . len ()) } }
}

macro_rules! read_buf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function read_buf in module {}", module_path!());
    };
}

mkfn!{
    read_buf_introspect!();
    #[doc = " Usercall `read` with an uninitialized buffer. See the ABI documentation for"] #[doc = " more information."] #[unstable (feature = "sgx_platform" , issue = "56975")] pub fn read_buf (fd : Fd , mut buf : BorrowedCursor < '_ >) -> IoResult < () > { unsafe { let mut userbuf = alloc :: User :: < [u8] > :: uninitialized (buf . capacity ()) ; let len = raw :: read (fd , userbuf . as_mut_ptr () . cast () , userbuf . len ()) . from_sgx_result () ? ; userbuf [.. len] . copy_to_enclave (& mut buf . as_mut () [.. len]) ; buf . advance_unchecked (len) ; Ok (()) } }
}

macro_rules! read_alloc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function read_alloc in module {}", module_path!());
    };
}

mkfn!{
    read_alloc_introspect!();
    #[doc = " Usercall `read_alloc`. See the ABI documentation for more information."] #[unstable (feature = "sgx_platform" , issue = "56975")] pub fn read_alloc (fd : Fd) -> IoResult < Vec < u8 > > { unsafe { let userbuf = ByteBuffer { data : crate :: ptr :: null_mut () , len : 0 } ; let mut userbuf = alloc :: User :: new_from_enclave (& userbuf) ; raw :: read_alloc (fd , userbuf . as_raw_mut_ptr ()) . from_sgx_result () ? ; Ok (userbuf . copy_user_buffer ()) } }
}

macro_rules! write_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write in module {}", module_path!());
    };
}

mkfn!{
    write_introspect!();
    #[doc = " Usercall `write`. See the ABI documentation for more information."] #[doc = ""] #[doc = " This will do a single `write` usercall and gather the written data from"] #[doc = " `bufs`. To write from a single buffer, just pass a slice of length one."] #[unstable (feature = "sgx_platform" , issue = "56975")] pub fn write (fd : Fd , bufs : & [IoSlice < '_ >]) -> IoResult < usize > { unsafe { let total_len = bufs . iter () . fold (0usize , | sum , buf | sum . saturating_add (buf . len ())) ; let mut userbuf = alloc :: User :: < [u8] > :: uninitialized (total_len) ; let mut index = 0 ; for buf in bufs { let end = cmp :: min (index + buf . len () , userbuf . len ()) ; if let Some (buflen) = end . checked_sub (index) { userbuf [index .. end] . copy_from_enclave (& buf [.. buflen]) ; index += buf . len () ; } else { break ; } } raw :: write (fd , userbuf . as_ptr () , userbuf . len ()) . from_sgx_result () } }
}

macro_rules! flush_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function flush in module {}", module_path!());
    };
}

mkfn!{
    flush_introspect!();
    #[doc = " Usercall `flush`. See the ABI documentation for more information."] #[unstable (feature = "sgx_platform" , issue = "56975")] pub fn flush (fd : Fd) -> IoResult < () > { unsafe { raw :: flush (fd) . from_sgx_result () } }
}

macro_rules! close_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function close in module {}", module_path!());
    };
}

mkfn!{
    close_introspect!();
    #[doc = " Usercall `close`. See the ABI documentation for more information."] #[unstable (feature = "sgx_platform" , issue = "56975")] pub fn close (fd : Fd) { unsafe { raw :: close (fd) } }
}

macro_rules! string_from_bytebuffer_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function string_from_bytebuffer in module {}", module_path!());
    };
}

mkfn!{
    string_from_bytebuffer_introspect!();
    fn string_from_bytebuffer (buf : & alloc :: UserRef < ByteBuffer > , usercall : & str , arg : & str) -> String { String :: from_utf8 (buf . copy_user_buffer ()) . unwrap_or_else (| _ | rtabort ! ("Usercall {usercall}: expected {arg} to be valid UTF-8")) }
}

macro_rules! bind_stream_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function bind_stream in module {}", module_path!());
    };
}

mkfn!{
    bind_stream_introspect!();
    #[doc = " Usercall `bind_stream`. See the ABI documentation for more information."] #[unstable (feature = "sgx_platform" , issue = "56975")] pub fn bind_stream (addr : & str) -> IoResult < (Fd , String) > { unsafe { let addr_user = alloc :: User :: new_from_enclave (addr . as_bytes ()) ; let mut local = alloc :: User :: < ByteBuffer > :: uninitialized () ; let fd = raw :: bind_stream (addr_user . as_ptr () , addr_user . len () , local . as_raw_mut_ptr ()) . from_sgx_result () ? ; let local = string_from_bytebuffer (& local , "bind_stream" , "local_addr") ; Ok ((fd , local)) } }
}

macro_rules! accept_stream_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function accept_stream in module {}", module_path!());
    };
}

mkfn!{
    accept_stream_introspect!();
    #[doc = " Usercall `accept_stream`. See the ABI documentation for more information."] #[unstable (feature = "sgx_platform" , issue = "56975")] pub fn accept_stream (fd : Fd) -> IoResult < (Fd , String , String) > { unsafe { let mut bufs = alloc :: User :: < [ByteBuffer ; 2] > :: uninitialized () ; let mut buf_it = alloc :: UserRef :: iter_mut (& mut * bufs) ; let (local , peer) = (buf_it . next () . unwrap () , buf_it . next () . unwrap ()) ; let fd = raw :: accept_stream (fd , local . as_raw_mut_ptr () , peer . as_raw_mut_ptr ()) . from_sgx_result () ? ; let local = string_from_bytebuffer (& local , "accept_stream" , "local_addr") ; let peer = string_from_bytebuffer (& peer , "accept_stream" , "peer_addr") ; Ok ((fd , local , peer)) } }
}

macro_rules! connect_stream_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function connect_stream in module {}", module_path!());
    };
}

mkfn!{
    connect_stream_introspect!();
    #[doc = " Usercall `connect_stream`. See the ABI documentation for more information."] #[unstable (feature = "sgx_platform" , issue = "56975")] pub fn connect_stream (addr : & str) -> IoResult < (Fd , String , String) > { unsafe { let addr_user = alloc :: User :: new_from_enclave (addr . as_bytes ()) ; let mut bufs = alloc :: User :: < [ByteBuffer ; 2] > :: uninitialized () ; let mut buf_it = alloc :: UserRef :: iter_mut (& mut * bufs) ; let (local , peer) = (buf_it . next () . unwrap () , buf_it . next () . unwrap ()) ; let fd = raw :: connect_stream (addr_user . as_ptr () , addr_user . len () , local . as_raw_mut_ptr () , peer . as_raw_mut_ptr () ,) . from_sgx_result () ? ; let local = string_from_bytebuffer (& local , "connect_stream" , "local_addr") ; let peer = string_from_bytebuffer (& peer , "connect_stream" , "peer_addr") ; Ok ((fd , local , peer)) } }
}

macro_rules! launch_thread_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function launch_thread in module {}", module_path!());
    };
}

mkfn!{
    launch_thread_introspect!();
    #[doc = " Usercall `launch_thread`. See the ABI documentation for more information."] #[unstable (feature = "sgx_platform" , issue = "56975")] pub unsafe fn launch_thread () -> IoResult < () > { unsafe { raw :: launch_thread () . from_sgx_result () } }
}

macro_rules! exit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exit in module {}", module_path!());
    };
}

mkfn!{
    exit_introspect!();
    #[doc = " Usercall `exit`. See the ABI documentation for more information."] #[unstable (feature = "sgx_platform" , issue = "56975")] pub fn exit (panic : bool) -> ! { unsafe { raw :: exit (panic) } }
}

macro_rules! wait_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function wait in module {}", module_path!());
    };
}

mkfn!{
    wait_introspect!();
    #[doc = " Usercall `wait`. See the ABI documentation for more information."] #[unstable (feature = "sgx_platform" , issue = "56975")] pub fn wait (event_mask : u64 , mut timeout : u64) -> IoResult < u64 > { if timeout != WAIT_NO && timeout != WAIT_INDEFINITE { if let Ok (timeout_signed) = i64 :: try_from (timeout) { let tenth = timeout_signed / 10 ; let deviation = random :: < i64 > (..) . checked_rem (tenth) . unwrap_or (0) ; timeout = timeout_signed . saturating_add (deviation) as _ ; } } unsafe { raw :: wait (event_mask , timeout) . from_sgx_result () } }
}

macro_rules! wait_timeout_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function wait_timeout in module {}", module_path!());
    };
}

mkfn!{
    wait_timeout_introspect!();
    #[doc = " Makes an effort to wait for a non-spurious event at least as long as"] #[doc = " `duration`."] #[doc = ""] #[doc = " Note that in general there is no guarantee about accuracy of time and"] #[doc = " timeouts in SGX model. The enclave runner serving usercalls may lie about"] #[doc = " current time and/or ignore timeout values."] #[doc = ""] #[doc = " Once the event is observed, `should_wake_up` will be used to determine"] #[doc = " whether or not the event was spurious."] #[unstable (feature = "sgx_platform" , issue = "56975")] pub fn wait_timeout < F > (event_mask : u64 , duration : Duration , should_wake_up : F) where F : Fn () -> bool , { fn wait_checked (event_mask : u64 , duration : Option < Duration >) -> bool { let timeout = duration . map_or (raw :: WAIT_NO , | duration | { cmp :: min ((u64 :: MAX - 1) as u128 , duration . as_nanos ()) as u64 }) ; match wait (event_mask , timeout) { Ok (eventset) => { if event_mask == 0 { rtabort ! ("expected wait() to return Err, found Ok.") ; } rtassert ! (eventset != 0 && eventset & ! event_mask == 0) ; true } Err (e) => { rtassert ! (e . kind () == ErrorKind :: TimedOut || e . kind () == ErrorKind :: WouldBlock) ; false } } } match wait_checked (event_mask , Some (duration)) { false => return , true if should_wake_up () => return , true => { } } loop { match wait_checked (event_mask , None) { false => break , true if should_wake_up () => return , true => { } } } let start = Instant :: now () ; let mut remaining = duration ; loop { match wait_checked (event_mask , Some (remaining)) { false => return , true if should_wake_up () => return , true => { } } remaining = match duration . checked_sub (start . elapsed ()) { Some (remaining) => remaining , None => break , } } }
}

macro_rules! send_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function send in module {}", module_path!());
    };
}

mkfn!{
    send_introspect!();
    #[doc = " Usercall `send`. See the ABI documentation for more information."] #[unstable (feature = "sgx_platform" , issue = "56975")] pub fn send (event_set : u64 , tcs : Option < Tcs >) -> IoResult < () > { unsafe { raw :: send (event_set , tcs) . from_sgx_result () } }
}

macro_rules! insecure_time_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function insecure_time in module {}", module_path!());
    };
}

mkfn!{
    insecure_time_introspect!();
    #[doc = " Usercall `insecure_time`. See the ABI documentation for more information."] #[unstable (feature = "sgx_platform" , issue = "56975")] pub fn insecure_time () -> Duration { let t = unsafe { raw :: insecure_time () . 0 } ; Duration :: new (t / 1_000_000_000 , (t % 1_000_000_000) as _) }
}

macro_rules! alloc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function alloc in module {}", module_path!());
    };
}

mkfn!{
    alloc_introspect!();
    #[doc = " Usercall `alloc`. See the ABI documentation for more information."] #[unstable (feature = "sgx_platform" , issue = "56975")] pub fn alloc (size : usize , alignment : usize) -> IoResult < * mut u8 > { unsafe { raw :: alloc (size , alignment) . from_sgx_result () } }
}
mkuse!{#[unstable (feature = "sgx_platform" , issue = "56975")] #[doc (inline)] pub use self :: raw :: free ;}

macro_rules! check_os_error_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_os_error in module {}", module_path!());
    };
}

mkfn!{
    check_os_error_introspect!();
    fn check_os_error (err : Result) -> i32 { if err == Error :: NotFound as _ || err == Error :: PermissionDenied as _ || err == Error :: ConnectionRefused as _ || err == Error :: ConnectionReset as _ || err == Error :: ConnectionAborted as _ || err == Error :: NotConnected as _ || err == Error :: AddrInUse as _ || err == Error :: AddrNotAvailable as _ || err == Error :: BrokenPipe as _ || err == Error :: AlreadyExists as _ || err == Error :: WouldBlock as _ || err == Error :: InvalidInput as _ || err == Error :: InvalidData as _ || err == Error :: TimedOut as _ || err == Error :: WriteZero as _ || err == Error :: Interrupted as _ || err == Error :: Other as _ || err == Error :: UnexpectedEof as _ || ((Error :: UserRangeStart as _) ..= (Error :: UserRangeEnd as _)) . contains (& err) { err } else { rtabort ! ("Usercall: returned invalid error value {err}") } }
}
mkitem!{mktrait!{#[doc = " Translate the raw result of an SGX usercall."] #[unstable (feature = "sgx_platform" , issue = "56975")] pub trait FromSgxResult { #[doc = " Return type"] type Return ; #[doc = " Translate the raw result of an SGX usercall."] fn from_sgx_result (self) -> IoResult < Self :: Return > ; }}}
mkitem!{mkimpl!{#[unstable (feature = "sgx_platform" , issue = "56975")] impl < T > FromSgxResult for (Result , T) { type Return = T ; fn from_sgx_result (self) -> IoResult < Self :: Return > { if self . 0 == RESULT_SUCCESS { Ok (self . 1) } else { Err (IoError :: from_raw_os_error (check_os_error (self . 0))) } } }}}
mkitem!{mkimpl!{#[unstable (feature = "sgx_platform" , issue = "56975")] impl FromSgxResult for Result { type Return = () ; fn from_sgx_result (self) -> IoResult < Self :: Return > { if self == RESULT_SUCCESS { Ok (()) } else { Err (IoError :: from_raw_os_error (check_os_error (self))) } } }}}