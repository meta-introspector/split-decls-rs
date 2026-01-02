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
mkuse!{# [cfg (not (any (target_env = "newlib" , target_os = "l4re" , target_os = "emscripten" , target_os = "redox" , target_os = "hurd" , target_os = "aix" ,)))] use crate :: ffi :: CStr ;}
mkuse!{use crate :: mem :: { self , ManuallyDrop } ;}
mkuse!{use crate :: num :: NonZero ;}
mkuse!{# [cfg (all (target_os = "linux" , target_env = "gnu"))] use crate :: sys :: weak :: dlsym ;}
mkuse!{# [cfg (any (target_os = "solaris" , target_os = "illumos" , target_os = "nto" ,))] use crate :: sys :: weak :: weak ;}
mkuse!{use crate :: sys :: { os , stack_overflow } ;}
mkuse!{use crate :: time :: Duration ;}
mkuse!{use crate :: { cmp , io , ptr } ;}
mkitem!{# [cfg (not (any (target_os = "l4re" , target_os = "vxworks" , target_os = "espidf" , target_os = "nuttx")))] pub const DEFAULT_MIN_STACK_SIZE : usize = 2 * 1024 * 1024 ;}
mkitem!{# [cfg (target_os = "l4re")] pub const DEFAULT_MIN_STACK_SIZE : usize = 1024 * 1024 ;}
mkitem!{# [cfg (target_os = "vxworks")] pub const DEFAULT_MIN_STACK_SIZE : usize = 256 * 1024 ;}
mkitem!{# [cfg (any (target_os = "espidf" , target_os = "nuttx"))] pub const DEFAULT_MIN_STACK_SIZE : usize = 0 ;}
mkitem!{mkstruct!{struct ThreadData { name : Option < Box < str > > , f : Box < dyn FnOnce () > , }}}
mkitem!{mkstruct!{pub struct Thread { id : libc :: pthread_t , }}}
mkitem!{mkimpl!{unsafe impl Send for Thread { }}}
mkitem!{mkimpl!{unsafe impl Sync for Thread { }}}
mkitem!{mkimpl!{impl Thread { # [cfg_attr (miri , track_caller)] pub unsafe fn new (stack : usize , name : Option < & str > , f : Box < dyn FnOnce () > ,) -> io :: Result < Thread > { let data = Box :: into_raw (Box :: new (ThreadData { name : name . map (Box :: from) , f })) ; let mut native : libc :: pthread_t = mem :: zeroed () ; let mut attr : mem :: MaybeUninit < libc :: pthread_attr_t > = mem :: MaybeUninit :: uninit () ; assert_eq ! (libc :: pthread_attr_init (attr . as_mut_ptr ()) , 0) ; # [cfg (any (target_os = "espidf" , target_os = "nuttx"))] if stack > 0 { assert_eq ! (libc :: pthread_attr_setstacksize (attr . as_mut_ptr () , cmp :: max (stack , min_stack_size (attr . as_ptr ()))) , 0) ; } # [cfg (not (any (target_os = "espidf" , target_os = "nuttx")))] { let stack_size = cmp :: max (stack , min_stack_size (attr . as_ptr ())) ; match libc :: pthread_attr_setstacksize (attr . as_mut_ptr () , stack_size) { 0 => { } n => { assert_eq ! (n , libc :: EINVAL) ; let page_size = os :: page_size () ; let stack_size = (stack_size + page_size - 1) & (- (page_size as isize - 1) as usize - 1) ; if libc :: pthread_attr_setstacksize (attr . as_mut_ptr () , stack_size) != 0 { assert_eq ! (libc :: pthread_attr_destroy (attr . as_mut_ptr ()) , 0) ; drop (Box :: from_raw (data)) ; return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "invalid stack size")) ; } } } ; } let ret = libc :: pthread_create (& mut native , attr . as_ptr () , thread_start , data as * mut _) ; assert_eq ! (libc :: pthread_attr_destroy (attr . as_mut_ptr ()) , 0) ; return if ret != 0 { drop (Box :: from_raw (data)) ; Err (io :: Error :: from_raw_os_error (ret)) } else { Ok (Thread { id : native }) } ; extern "C" fn thread_start (data : * mut libc :: c_void) -> * mut libc :: c_void { unsafe { let data = Box :: from_raw (data as * mut ThreadData) ; let _handler = stack_overflow :: Handler :: new (data . name) ; (data . f) () ; } ptr :: null_mut () } } pub fn join (self) { let id = self . into_id () ; let ret = unsafe { libc :: pthread_join (id , ptr :: null_mut ()) } ; assert ! (ret == 0 , "failed to join thread: {}" , io :: Error :: from_raw_os_error (ret)) ; } pub fn id (& self) -> libc :: pthread_t { self . id } pub fn into_id (self) -> libc :: pthread_t { ManuallyDrop :: new (self) . id } }}}
mkitem!{mkimpl!{impl Drop for Thread { fn drop (& mut self) { let ret = unsafe { libc :: pthread_detach (self . id) } ; debug_assert_eq ! (ret , 0) ; } }}}

macro_rules! available_parallelism_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function available_parallelism in module {}", module_path!());
    };
}

mkfn!{
    available_parallelism_introspect!();
    pub fn available_parallelism () -> io :: Result < NonZero < usize > > { cfg_select ! { any (target_os = "android" , target_os = "emscripten" , target_os = "fuchsia" , target_os = "hurd" , target_os = "linux" , target_os = "aix" , target_vendor = "apple" , target_os = "cygwin" ,) => { # [allow (unused_assignments)] # [allow (unused_mut)] let mut quota = usize :: MAX ; # [cfg (any (target_os = "android" , target_os = "linux"))] { quota = cgroups :: quota () . max (1) ; let mut set : libc :: cpu_set_t = unsafe { mem :: zeroed () } ; unsafe { if libc :: sched_getaffinity (0 , size_of ::< libc :: cpu_set_t > () , & mut set) == 0 { let count = libc :: CPU_COUNT (& set) as usize ; let count = count . min (quota) ; if let Some (count) = NonZero :: new (count) { return Ok (count) } } } } match unsafe { libc :: sysconf (libc :: _SC_NPROCESSORS_ONLN) } { - 1 => Err (io :: Error :: last_os_error ()) , 0 => Err (io :: Error :: UNKNOWN_THREAD_COUNT) , cpus => { let count = cpus as usize ; let count = count . min (quota) ; Ok (unsafe { NonZero :: new_unchecked (count) }) } } } any (target_os = "freebsd" , target_os = "dragonfly" , target_os = "openbsd" , target_os = "netbsd" ,) => { use crate :: ptr ; # [cfg (target_os = "freebsd")] { let mut set : libc :: cpuset_t = unsafe { mem :: zeroed () } ; unsafe { if libc :: cpuset_getaffinity (libc :: CPU_LEVEL_WHICH , libc :: CPU_WHICH_PID , - 1 , size_of ::< libc :: cpuset_t > () , & mut set ,) == 0 { let count = libc :: CPU_COUNT (& set) as usize ; if count > 0 { return Ok (NonZero :: new_unchecked (count)) ; } } } } # [cfg (target_os = "netbsd")] { unsafe { let set = libc :: _cpuset_create () ; if ! set . is_null () { let mut count : usize = 0 ; if libc :: pthread_getaffinity_np (libc :: pthread_self () , libc :: _cpuset_size (set) , set) == 0 { for i in 0 .. libc :: cpuid_t :: MAX { match libc :: _cpuset_isset (i , set) { - 1 => break , 0 => continue , _ => count = count + 1 , } } } libc :: _cpuset_destroy (set) ; if let Some (count) = NonZero :: new (count) { return Ok (count) ; } } } } let mut cpus : libc :: c_uint = 0 ; let mut cpus_size = size_of_val (& cpus) ; unsafe { cpus = libc :: sysconf (libc :: _SC_NPROCESSORS_ONLN) as libc :: c_uint ; } if cpus < 1 { let mut mib = [libc :: CTL_HW , libc :: HW_NCPU , 0 , 0] ; let res = unsafe { libc :: sysctl (mib . as_mut_ptr () , 2 , (& raw mut cpus) as * mut _ , (& raw mut cpus_size) as * mut _ , ptr :: null_mut () , 0 ,) } ; if res == - 1 { return Err (io :: Error :: last_os_error ()) ; } else if cpus == 0 { return Err (io :: Error :: UNKNOWN_THREAD_COUNT) ; } } Ok (unsafe { NonZero :: new_unchecked (cpus as usize) }) } target_os = "nto" => { unsafe { use libc :: _syspage_ptr ; if _syspage_ptr . is_null () { Err (io :: const_error ! (io :: ErrorKind :: NotFound , "no syspage available")) } else { let cpus = (* _syspage_ptr) . num_cpu ; NonZero :: new (cpus as usize) . ok_or (io :: Error :: UNKNOWN_THREAD_COUNT) } } } any (target_os = "solaris" , target_os = "illumos") => { let mut cpus = 0u32 ; if unsafe { libc :: pset_info (libc :: PS_MYID , core :: ptr :: null_mut () , & mut cpus , core :: ptr :: null_mut ()) } != 0 { return Err (io :: Error :: UNKNOWN_THREAD_COUNT) ; } Ok (unsafe { NonZero :: new_unchecked (cpus as usize) }) } target_os = "haiku" => { unsafe { let mut sinfo : libc :: system_info = crate :: mem :: zeroed () ; let res = libc :: get_system_info (& mut sinfo) ; if res != libc :: B_OK { return Err (io :: Error :: UNKNOWN_THREAD_COUNT) ; } Ok (NonZero :: new_unchecked (sinfo . cpu_count as usize)) } } target_os = "vxworks" => { unsafe extern "C" { fn vxCpuEnabledGet () -> libc :: cpuset_t ; } unsafe { let set = vxCpuEnabledGet () ; Ok (NonZero :: new_unchecked (set . count_ones () as usize)) } } _ => { Err (io :: const_error ! (io :: ErrorKind :: Unsupported , "getting the number of hardware threads is not supported on the target platform")) } } }
}

macro_rules! current_os_id_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_os_id in module {}", module_path!());
    };
}

mkfn!{
    current_os_id_introspect!();
    pub fn current_os_id () -> Option < u64 > { cfg_select ! { any (target_os = "android" , target_os = "linux") => { use crate :: sys :: pal :: weak :: syscall ; syscall ! (fn gettid () -> libc :: pid_t ;) ; let id : libc :: pid_t = unsafe { gettid () } ; Some (id as u64) } target_os = "nto" => { let id : libc :: pid_t = unsafe { libc :: gettid () } ; Some (id as u64) } target_os = "openbsd" => { let id : libc :: pid_t = unsafe { libc :: getthrid () } ; Some (id as u64) } target_os = "freebsd" => { let id : libc :: c_int = unsafe { libc :: pthread_getthreadid_np () } ; Some (id as u64) } target_os = "netbsd" => { let id : libc :: lwpid_t = unsafe { libc :: _lwp_self () } ; Some (id as u64) } any (target_os = "illumos" , target_os = "solaris") => { let id : libc :: pthread_t = unsafe { libc :: pthread_self () } ; Some (id as u64) } target_vendor = "apple" => { let mut id = 0u64 ; let status : libc :: c_int = unsafe { libc :: pthread_threadid_np (0 , & mut id) } ; if status == 0 { Some (id) } else { None } } _ => None , } }
}

macro_rules! truncate_cstr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function truncate_cstr in module {}", module_path!());
    };
}

mkfn!{
    truncate_cstr_introspect!();
    # [cfg (any (target_os = "linux" , target_os = "nto" , target_os = "solaris" , target_os = "illumos" , target_os = "vxworks" , target_os = "cygwin" , target_vendor = "apple" ,))] fn truncate_cstr < const MAX_WITH_NUL : usize > (cstr : & CStr) -> [libc :: c_char ; MAX_WITH_NUL] { let mut result = [0 ; MAX_WITH_NUL] ; for (src , dst) in cstr . to_bytes () . iter () . zip (& mut result [.. MAX_WITH_NUL - 1]) { * dst = * src as libc :: c_char ; } result }
}

macro_rules! set_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_name in module {}", module_path!());
    };
}

mkfn!{
    set_name_introspect!();
    # [cfg (target_os = "android")] pub fn set_name (name : & CStr) { const PR_SET_NAME : libc :: c_int = 15 ; unsafe { let res = libc :: prctl (PR_SET_NAME , name . as_ptr () , 0 as libc :: c_ulong , 0 as libc :: c_ulong , 0 as libc :: c_ulong ,) ; debug_assert_eq ! (res , 0) ; } }
}

macro_rules! set_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_name in module {}", module_path!());
    };
}

mkfn!{
    set_name_introspect!();
    # [cfg (any (target_os = "linux" , target_os = "freebsd" , target_os = "dragonfly" , target_os = "nuttx" , target_os = "cygwin"))] pub fn set_name (name : & CStr) { unsafe { cfg_select ! { any (target_os = "linux" , target_os = "cygwin") => { const TASK_COMM_LEN : usize = 16 ; let name = truncate_cstr ::< { TASK_COMM_LEN } > (name) ; } _ => { } } ; let res = libc :: pthread_setname_np (libc :: pthread_self () , name . as_ptr ()) ; debug_assert_eq ! (res , 0) ; } }
}

macro_rules! set_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_name in module {}", module_path!());
    };
}

mkfn!{
    set_name_introspect!();
    # [cfg (target_os = "openbsd")] pub fn set_name (name : & CStr) { unsafe { libc :: pthread_set_name_np (libc :: pthread_self () , name . as_ptr ()) ; } }
}

macro_rules! set_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_name in module {}", module_path!());
    };
}

mkfn!{
    set_name_introspect!();
    # [cfg (target_vendor = "apple")] pub fn set_name (name : & CStr) { unsafe { let name = truncate_cstr :: < { libc :: MAXTHREADNAMESIZE } > (name) ; let res = libc :: pthread_setname_np (name . as_ptr ()) ; debug_assert_eq ! (res , 0) ; } }
}

macro_rules! set_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_name in module {}", module_path!());
    };
}

mkfn!{
    set_name_introspect!();
    # [cfg (target_os = "netbsd")] pub fn set_name (name : & CStr) { unsafe { let res = libc :: pthread_setname_np (libc :: pthread_self () , c"%s" . as_ptr () , name . as_ptr () as * mut libc :: c_void ,) ; debug_assert_eq ! (res , 0) ; } }
}

macro_rules! set_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_name in module {}", module_path!());
    };
}

mkfn!{
    set_name_introspect!();
    # [cfg (any (target_os = "solaris" , target_os = "illumos" , target_os = "nto"))] pub fn set_name (name : & CStr) { weak ! (fn pthread_setname_np (thread : libc :: pthread_t , name : * const libc :: c_char) -> libc :: c_int ;) ; if let Some (f) = pthread_setname_np . get () { # [cfg (target_os = "nto")] const THREAD_NAME_MAX : usize = libc :: _NTO_THREAD_NAME_MAX as usize ; # [cfg (any (target_os = "solaris" , target_os = "illumos"))] const THREAD_NAME_MAX : usize = 32 ; let name = truncate_cstr :: < { THREAD_NAME_MAX } > (name) ; let res = unsafe { f (libc :: pthread_self () , name . as_ptr ()) } ; debug_assert_eq ! (res , 0) ; } }
}

macro_rules! set_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_name in module {}", module_path!());
    };
}

mkfn!{
    set_name_introspect!();
    # [cfg (target_os = "fuchsia")] pub fn set_name (name : & CStr) { use crate :: sys :: pal :: fuchsia :: * ; unsafe { zx_object_set_property (zx_thread_self () , ZX_PROP_NAME , name . as_ptr () as * const libc :: c_void , name . to_bytes () . len () ,) ; } }
}

macro_rules! set_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_name in module {}", module_path!());
    };
}

mkfn!{
    set_name_introspect!();
    # [cfg (target_os = "haiku")] pub fn set_name (name : & CStr) { unsafe { let thread_self = libc :: find_thread (ptr :: null_mut ()) ; let res = libc :: rename_thread (thread_self , name . as_ptr ()) ; debug_assert_eq ! (res , libc :: B_OK) ; } }
}

macro_rules! set_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_name in module {}", module_path!());
    };
}

mkfn!{
    set_name_introspect!();
    # [cfg (target_os = "vxworks")] pub fn set_name (name : & CStr) { let mut name = truncate_cstr :: < { (libc :: VX_TASK_RENAME_LENGTH - 1) as usize } > (name) ; let res = unsafe { libc :: taskNameSet (libc :: taskIdSelf () , name . as_mut_ptr ()) } ; debug_assert_eq ! (res , libc :: OK) ; }
}

macro_rules! sleep_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sleep in module {}", module_path!());
    };
}

mkfn!{
    sleep_introspect!();
    # [cfg (not (target_os = "espidf"))] pub fn sleep (dur : Duration) { let mut secs = dur . as_secs () ; let mut nsecs = dur . subsec_nanos () as _ ; unsafe { while secs > 0 || nsecs > 0 { let mut ts = libc :: timespec { tv_sec : cmp :: min (libc :: time_t :: MAX as u64 , secs) as libc :: time_t , tv_nsec : nsecs , } ; secs -= ts . tv_sec as u64 ; let ts_ptr = & raw mut ts ; if libc :: nanosleep (ts_ptr , ts_ptr) == - 1 { assert_eq ! (os :: errno () , libc :: EINTR) ; secs += ts . tv_sec as u64 ; nsecs = ts . tv_nsec ; } else { nsecs = 0 ; } } } }
}

macro_rules! sleep_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sleep in module {}", module_path!());
    };
}

mkfn!{
    sleep_introspect!();
    # [cfg (target_os = "espidf")] pub fn sleep (dur : Duration) { const MAX_MICROS : u32 = u32 :: MAX - 1_000_000 - 1 ; let mut micros = dur . as_micros () + if dur . subsec_nanos () % 1_000 > 0 { 1 } else { 0 } ; while micros > 0 { let st = if micros > MAX_MICROS as u128 { MAX_MICROS } else { micros as u32 } ; unsafe { libc :: usleep (st) ; } micros -= st as u128 ; } }
}

macro_rules! sleep_until_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sleep_until in module {}", module_path!());
    };
}

mkfn!{
    sleep_until_introspect!();
    # [cfg (any (target_os = "freebsd" , target_os = "netbsd" , target_os = "linux" , target_os = "android" , target_os = "solaris" , target_os = "illumos" , target_os = "dragonfly" , target_os = "hurd" , target_os = "fuchsia" , target_os = "vxworks" ,))] pub fn sleep_until (deadline : crate :: time :: Instant) { use crate :: time :: Instant ; let Some (ts) = deadline . into_inner () . into_timespec () . to_timespec () else { let now = Instant :: now () ; if let Some (delay) = deadline . checked_duration_since (now) { sleep (delay) ; } return ; } ; unsafe { loop { let res = libc :: clock_nanosleep (crate :: sys :: time :: Instant :: CLOCK_ID , libc :: TIMER_ABSTIME , & ts , core :: ptr :: null_mut () ,) ; if res == 0 { break ; } else { assert_eq ! (res , libc :: EINTR , "timespec is in range,
                         clockid is valid and kernel should support it") ; } } } }
}

macro_rules! yield_now_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function yield_now in module {}", module_path!());
    };
}

mkfn!{
    yield_now_introspect!();
    pub fn yield_now () { let ret = unsafe { libc :: sched_yield () } ; debug_assert_eq ! (ret , 0) ; }
}
mkmod!{cgroups, { 
                getname!(cgroups);
                getsrc!(cgroups);
                getpath!(cgroups);
                get_deps!(cgroups);
                get_crates!(cgroups);
                mkinclude!(cgroups);
                mkuse!{use crate :: borrow :: Cow ;}
mkuse!{use crate :: ffi :: OsString ;}
mkuse!{use crate :: fs :: { File , exists } ;}
mkuse!{use crate :: io :: { BufRead , Read } ;}
mkuse!{use crate :: os :: unix :: ffi :: OsStringExt ;}
mkuse!{use crate :: path :: { Path , PathBuf } ;}
mkuse!{use crate :: str :: from_utf8 ;}
mkitem!{mkenum!{# [derive (PartialEq)] enum Cgroup { V1 , V2 , }}}

macro_rules! quota_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function quota in module {}", module_path!());
    };
}

mkfn!{
    quota_introspect!();
    # [doc = " Returns cgroup CPU quota in core-equivalents, rounded down or usize::MAX if the quota cannot"] # [doc = " be determined or is not set."] pub (super) fn quota () -> usize { let mut quota = usize :: MAX ; if cfg ! (miri) { return quota ; } let _ : Option < () > = try { let mut buf = Vec :: with_capacity (128) ; File :: open ("/proc/self/cgroup") . ok () ? . read_to_end (& mut buf) . ok () ? ; let (cgroup_path , version) = buf . split (| & c | c == b'\n') . fold (None , | previous , line | { let mut fields = line . splitn (3 , | & c | c == b':') ; let version = match fields . nth (1) { Some (b"") => Cgroup :: V2 , Some (controllers) if from_utf8 (controllers) . is_ok_and (| c | c . split (',') . any (| c | c == "cpu")) => { Cgroup :: V1 } _ => return previous , } ; if previous . is_some () && version == Cgroup :: V2 { return previous ; } let path = fields . last () ? ; Some ((path [1 ..] . to_owned () , version)) }) ? ; let cgroup_path = PathBuf :: from (OsString :: from_vec (cgroup_path)) ; quota = match version { Cgroup :: V1 => quota_v1 (cgroup_path) , Cgroup :: V2 => quota_v2 (cgroup_path) , } ; } ; quota }
}

macro_rules! quota_v2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function quota_v2 in module {}", module_path!());
    };
}

mkfn!{
    quota_v2_introspect!();
    fn quota_v2 (group_path : PathBuf) -> usize { let mut quota = usize :: MAX ; let mut path = PathBuf :: with_capacity (128) ; let mut read_buf = String :: with_capacity (20) ; let cgroup_mount = "/sys/fs/cgroup" ; path . push (cgroup_mount) ; path . push (& group_path) ; path . push ("cgroup.controllers") ; if matches ! (exists (& path) , Err (_) | Ok (false)) { return usize :: MAX ; } ; path . pop () ; let _ : Option < () > = try { while path . starts_with (cgroup_mount) { path . push ("cpu.max") ; read_buf . clear () ; if File :: open (& path) . and_then (| mut f | f . read_to_string (& mut read_buf)) . is_ok () { let raw_quota = read_buf . lines () . next () ? ; let mut raw_quota = raw_quota . split (' ') ; let limit = raw_quota . next () ? ; let period = raw_quota . next () ? ; match (limit . parse :: < usize > () , period . parse :: < usize > ()) { (Ok (limit) , Ok (period)) if period > 0 => { quota = quota . min (limit / period) ; } _ => { } } } path . pop () ; path . pop () ; } } ; quota }
}

macro_rules! quota_v1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function quota_v1 in module {}", module_path!());
    };
}

mkfn!{
    quota_v1_introspect!();
    fn quota_v1 (group_path : PathBuf) -> usize { let mut quota = usize :: MAX ; let mut path = PathBuf :: with_capacity (128) ; let mut read_buf = String :: with_capacity (20) ; let mounts : & [fn (& Path) -> Option < (_ , & Path) >] = & [| p | Some ((Cow :: Borrowed ("/sys/fs/cgroup/cpu") , p)) , | p | Some ((Cow :: Borrowed ("/sys/fs/cgroup/cpu,cpuacct") , p)) , find_mountpoint ,] ; for mount in mounts { let Some ((mount , group_path)) = mount (& group_path) else { continue } ; path . clear () ; path . push (mount . as_ref ()) ; path . push (& group_path) ; if matches ! (exists (& path) , Err (_) | Ok (false)) { continue ; } while path . starts_with (mount . as_ref ()) { let mut parse_file = | name | { path . push (name) ; read_buf . clear () ; let f = File :: open (& path) ; path . pop () ; f . ok () ? . read_to_string (& mut read_buf) . ok () ? ; let parsed = read_buf . trim () . parse :: < usize > () . ok () ? ; Some (parsed) } ; let limit = parse_file ("cpu.cfs_quota_us") ; let period = parse_file ("cpu.cfs_period_us") ; match (limit , period) { (Some (limit) , Some (period)) if period > 0 => quota = quota . min (limit / period) , _ => { } } path . pop () ; } break ; } quota }
}

macro_rules! find_mountpoint_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_mountpoint in module {}", module_path!());
    };
}

mkfn!{
    find_mountpoint_introspect!();
    # [doc = " Scan mountinfo for cgroup v1 mountpoint with a cpu controller"] # [doc = ""] # [doc = " If the cgroupfs is a bind mount then `group_path` is adjusted to skip"] # [doc = " over the already-included prefix"] fn find_mountpoint (group_path : & Path) -> Option < (Cow < 'static , str > , & Path) > { let mut reader = File :: open_buffered ("/proc/self/mountinfo") . ok () ? ; let mut line = String :: with_capacity (256) ; loop { line . clear () ; if reader . read_line (& mut line) . ok () ? == 0 { break ; } let line = line . trim () ; let mut items = line . split (' ') ; let sub_path = items . nth (3) ? ; let mount_point = items . next () ? ; let mount_opts = items . next_back () ? ; let filesystem_type = items . nth_back (1) ? ; if filesystem_type != "cgroup" || ! mount_opts . split (',') . any (| opt | opt == "cpu") { continue ; } let sub_path = Path :: new (sub_path) . strip_prefix ("/") . ok () ? ; if ! group_path . starts_with (sub_path) { continue ; } let trimmed_group_path = group_path . strip_prefix (sub_path) . ok () ? ; return Some ((Cow :: Owned (mount_point . to_owned ()) , trimmed_group_path)) ; } None }
} 
            }}

macro_rules! min_stack_size_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function min_stack_size in module {}", module_path!());
    };
}

mkfn!{
    min_stack_size_introspect!();
    # [cfg (all (target_os = "linux" , target_env = "gnu"))] unsafe fn min_stack_size (attr : * const libc :: pthread_attr_t) -> usize { dlsym ! (fn __pthread_get_minstack (attr : * const libc :: pthread_attr_t) -> libc :: size_t ;) ; match __pthread_get_minstack . get () { None => libc :: PTHREAD_STACK_MIN , Some (f) => unsafe { f (attr) } , } }
}

macro_rules! min_stack_size_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function min_stack_size in module {}", module_path!());
    };
}

mkfn!{
    min_stack_size_introspect!();
    # [cfg (all (not (all (target_os = "linux" , target_env = "gnu")) , not (any (target_os = "netbsd" , target_os = "nuttx"))))] unsafe fn min_stack_size (_ : * const libc :: pthread_attr_t) -> usize { libc :: PTHREAD_STACK_MIN }
}

macro_rules! min_stack_size_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function min_stack_size in module {}", module_path!());
    };
}

mkfn!{
    min_stack_size_introspect!();
    # [cfg (any (target_os = "netbsd" , target_os = "nuttx"))] unsafe fn min_stack_size (_ : * const libc :: pthread_attr_t) -> usize { static STACK : crate :: sync :: OnceLock < usize > = crate :: sync :: OnceLock :: new () ; * STACK . get_or_init (| | { let mut stack = unsafe { libc :: sysconf (libc :: _SC_THREAD_STACK_MIN) } ; if stack < 0 { stack = 2048 ; } stack as usize }) }
}