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
mkuse!{use std :: io :: prelude :: * ;}
mkuse!{use libc ;}
mkuse!{use std :: ffi :: CStr ;}
mkuse!{use std :: io ;}
mkuse!{use std :: mem ;}
mkuse!{use std :: str ;}
mkuse!{use std :: sync :: StaticMutex ;}

macro_rules! write_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write in module {}", module_path!());
    };
}

mkfn!{
    write_introspect!();
    # [cfg (not (all (target_os = "ios" , target_arch = "arm")))] # [inline (never)] pub fn write (w : & mut Write) -> io :: Result < () > { struct Context < 'a > { idx : isize , writer : & 'a mut (Write + 'a) , last_error : Option < io :: Error > , } static LOCK : StaticMutex = StaticMutex :: new () ; let _g = LOCK . lock () ; try ! (writeln ! (w , "stack backtrace:")) ; let mut cx = Context { writer : w , last_error : None , idx : 0 } ; return match unsafe { uw :: _Unwind_Backtrace (trace_fn , & mut cx as * mut Context as * mut libc :: c_void) } { uw :: _URC_NO_REASON => { match cx . last_error { Some (err) => Err (err) , None => Ok (()) } } _ => Ok (()) , } ; extern fn trace_fn (ctx : * mut uw :: _Unwind_Context , arg : * mut libc :: c_void) -> uw :: _Unwind_Reason_Code { let cx : & mut Context = unsafe { mem :: transmute (arg) } ; let mut ip_before_insn = 0 ; let mut ip = unsafe { uw :: _Unwind_GetIPInfo (ctx , & mut ip_before_insn) as * mut libc :: c_void } ; if ! ip . is_null () && ip_before_insn == 0 { ip = (ip as usize - 1) as * mut _ ; } let symaddr = if cfg ! (target_os = "macos") || cfg ! (target_os = "ios") { ip } else { unsafe { uw :: _Unwind_FindEnclosingFunction (ip) } } ; cx . idx += 1 ; if cx . idx <= 0 { return uw :: _URC_NO_REASON } if cx . idx > 100 { match write ! (cx . writer , " ... <frames omitted>\n") { Ok (()) => { } Err (e) => { cx . last_error = Some (e) ; } } return uw :: _URC_FAILURE } if cx . last_error . is_some () { return uw :: _URC_FAILURE } match print (cx . writer , cx . idx , ip , symaddr) { Ok (()) => { } Err (e) => { cx . last_error = Some (e) ; } } return uw :: _URC_NO_REASON } }
}

macro_rules! print_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function print in module {}", module_path!());
    };
}

mkfn!{
    print_introspect!();
    # [cfg (any (target_os = "macos" , target_os = "ios"))] fn print (w : & mut Write , idx : isize , addr : * mut libc :: c_void , _symaddr : * mut libc :: c_void) -> io :: Result < () > { use intrinsics ; # [repr (C)] struct Dl_info { dli_fname : * const libc :: c_char , dli_fbase : * mut libc :: c_void , dli_sname : * const libc :: c_char , dli_saddr : * mut libc :: c_void , } extern { fn dladdr (addr : * const libc :: c_void , info : * mut Dl_info) -> libc :: c_int ; } let mut info : Dl_info = unsafe { intrinsics :: init () } ; if unsafe { dladdr (addr , & mut info) == 0 } { output (w , idx , addr , None) } else { output (w , idx , addr , Some (unsafe { CStr :: from_ptr (info . dli_sname) . to_bytes () })) } }
}

macro_rules! print_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function print in module {}", module_path!());
    };
}

mkfn!{
    print_introspect!();
    # [cfg (not (any (target_os = "macos" , target_os = "ios")))] fn print (w : & mut Write , idx : isize , addr : * mut libc :: c_void , symaddr : * mut libc :: c_void) -> io :: Result < () > { use env ; use os :: unix :: prelude :: * ; use ptr ; type backtrace_syminfo_callback = extern "C" fn (data : * mut libc :: c_void , pc : libc :: uintptr_t , symname : * const libc :: c_char , symval : libc :: uintptr_t , symsize : libc :: uintptr_t) ; type backtrace_full_callback = extern "C" fn (data : * mut libc :: c_void , pc : libc :: uintptr_t , filename : * const libc :: c_char , lineno : libc :: c_int , function : * const libc :: c_char) -> libc :: c_int ; type backtrace_error_callback = extern "C" fn (data : * mut libc :: c_void , msg : * const libc :: c_char , errnum : libc :: c_int) ; enum backtrace_state { } # [link (name = "backtrace" , kind = "static")] # [cfg (not (test))] extern { } extern { fn backtrace_create_state (filename : * const libc :: c_char , threaded : libc :: c_int , error : backtrace_error_callback , data : * mut libc :: c_void) -> * mut backtrace_state ; fn backtrace_syminfo (state : * mut backtrace_state , addr : libc :: uintptr_t , cb : backtrace_syminfo_callback , error : backtrace_error_callback , data : * mut libc :: c_void) -> libc :: c_int ; fn backtrace_pcinfo (state : * mut backtrace_state , addr : libc :: uintptr_t , cb : backtrace_full_callback , error : backtrace_error_callback , data : * mut libc :: c_void) -> libc :: c_int ; } type FileLine = (* const libc :: c_char , libc :: c_int) ; extern fn error_cb (_data : * mut libc :: c_void , _msg : * const libc :: c_char , _errnum : libc :: c_int) { } extern fn syminfo_cb (data : * mut libc :: c_void , _pc : libc :: uintptr_t , symname : * const libc :: c_char , _symval : libc :: uintptr_t , _symsize : libc :: uintptr_t) { let slot = data as * mut * const libc :: c_char ; unsafe { * slot = symname ; } } extern fn pcinfo_cb (data : * mut libc :: c_void , _pc : libc :: uintptr_t , filename : * const libc :: c_char , lineno : libc :: c_int , _function : * const libc :: c_char) -> libc :: c_int { if ! filename . is_null () { let slot = data as * mut & mut [FileLine] ; let buffer = unsafe { ptr :: read (slot) } ; if ! buffer . is_empty () { buffer [0] = (filename , lineno) ; unsafe { ptr :: write (slot , & mut buffer [1 ..]) ; } } } 0 } unsafe fn init_state () -> * mut backtrace_state { static mut STATE : * mut backtrace_state = 0 as * mut backtrace_state ; static mut LAST_FILENAME : [libc :: c_char ; 256] = [0 ; 256] ; if ! STATE . is_null () { return STATE } let selfname = if cfg ! (target_os = "freebsd") || cfg ! (target_os = "dragonfly") || cfg ! (target_os = "bitrig") || cfg ! (target_os = "openbsd") { env :: current_exe () . ok () } else { None } ; let filename = match selfname { Some (path) => { let bytes = path . as_os_str () . as_bytes () ; if bytes . len () < LAST_FILENAME . len () { let i = bytes . iter () ; for (slot , val) in LAST_FILENAME . iter_mut () . zip (i) { * slot = * val as libc :: c_char ; } LAST_FILENAME . as_ptr () } else { ptr :: null () } } None => ptr :: null () , } ; STATE = backtrace_create_state (filename , 0 , error_cb , ptr :: null_mut ()) ; return STATE } let state = unsafe { init_state () } ; if state . is_null () { return output (w , idx , addr , None) } let mut data = ptr :: null () ; let data_addr = & mut data as * mut * const libc :: c_char ; let ret = unsafe { backtrace_syminfo (state , symaddr as libc :: uintptr_t , syminfo_cb , error_cb , data_addr as * mut libc :: c_void) } ; if ret == 0 || data . is_null () { try ! (output (w , idx , addr , None)) ; } else { try ! (output (w , idx , addr , Some (unsafe { CStr :: from_ptr (data) . to_bytes () }))) ; } const FILELINE_SIZE : usize = 32 ; let mut fileline_buf = [(ptr :: null () , - 1) ; FILELINE_SIZE] ; let ret ; let fileline_count ; { let mut fileline_win : & mut [FileLine] = & mut fileline_buf ; let fileline_addr = & mut fileline_win as * mut & mut [FileLine] ; ret = unsafe { backtrace_pcinfo (state , addr as libc :: uintptr_t , pcinfo_cb , error_cb , fileline_addr as * mut libc :: c_void) } ; fileline_count = FILELINE_SIZE - fileline_win . len () ; } if ret == 0 { for (i , & (file , line)) in fileline_buf [.. fileline_count] . iter () . enumerate () { if file . is_null () { continue ; } let file = unsafe { CStr :: from_ptr (file) . to_bytes () } ; try ! (output_fileline (w , file , line , i == FILELINE_SIZE - 1)) ; } } Ok (()) }
}

macro_rules! output_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function output in module {}", module_path!());
    };
}

mkfn!{
    output_introspect!();
    fn output (w : & mut Write , idx : isize , addr : * mut libc :: c_void , s : Option < & [u8] >) -> io :: Result < () > { try ! (write ! (w , "  {:2}: {:2$?} - " , idx , addr , HEX_WIDTH)) ; match s . and_then (| s | str :: from_utf8 (s) . ok ()) { Some (string) => try ! (demangle (w , string)) , None => try ! (write ! (w , "<unknown>")) , } w . write_all (& ['\n' as u8]) }
}

macro_rules! output_fileline_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function output_fileline in module {}", module_path!());
    };
}

mkfn!{
    output_fileline_introspect!();
    # [allow (dead_code)] fn output_fileline (w : & mut Write , file : & [u8] , line : libc :: c_int , more : bool) -> io :: Result < () > { let file = str :: from_utf8 (file) . unwrap_or ("<unknown>") ; try ! (write ! (w , "      {:3$}at {}:{}" , "" , file , line , HEX_WIDTH)) ; if more { try ! (write ! (w , " <... and possibly more>")) ; } w . write_all (& ['\n' as u8]) }
}
mkmod!{uw, { 
                getname!(uw);
                getsrc!(uw);
                getpath!(uw);
                get_deps!(uw);
                get_crates!(uw);
                mkinclude!(uw);
                mkuse!{pub use self :: _Unwind_Reason_Code :: * ;}
mkuse!{use libc ;}
mkitem!{mkenum!{# [repr (C)] pub enum _Unwind_Reason_Code { _URC_NO_REASON = 0 , _URC_FOREIGN_EXCEPTION_CAUGHT = 1 , _URC_FATAL_PHASE2_ERROR = 2 , _URC_FATAL_PHASE1_ERROR = 3 , _URC_NORMAL_STOP = 4 , _URC_END_OF_STACK = 5 , _URC_HANDLER_FOUND = 6 , _URC_INSTALL_CONTEXT = 7 , _URC_CONTINUE_UNWIND = 8 , _URC_FAILURE = 9 , }}}
mkitem!{mkenum!{pub enum _Unwind_Context { }}}
mkitem!{pub type _Unwind_Trace_Fn = extern fn (ctx : * mut _Unwind_Context , arg : * mut libc :: c_void) -> _Unwind_Reason_Code ;}
mkitem!{extern { # [cfg (not (all (target_os = "ios" , target_arch = "arm")))] pub fn _Unwind_Backtrace (trace : _Unwind_Trace_Fn , trace_argument : * mut libc :: c_void) -> _Unwind_Reason_Code ; # [cfg (all (not (all (target_os = "android" , target_arch = "arm")) , not (all (target_os = "linux" , target_arch = "arm"))))] pub fn _Unwind_GetIPInfo (ctx : * mut _Unwind_Context , ip_before_insn : * mut libc :: c_int) -> libc :: uintptr_t ; # [cfg (all (not (target_os = "android") , not (all (target_os = "linux" , target_arch = "arm"))))] pub fn _Unwind_FindEnclosingFunction (pc : * mut libc :: c_void) -> * mut libc :: c_void ; }}

macro_rules! _Unwind_GetIP_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _Unwind_GetIP in module {}", module_path!());
    };
}

mkfn!{
    _Unwind_GetIP_introspect!();
    # [cfg (any (all (target_os = "android" , target_arch = "arm") , all (target_os = "linux" , target_arch = "arm")))] pub unsafe fn _Unwind_GetIP (ctx : * mut _Unwind_Context) -> libc :: uintptr_t { # [repr (C)] enum _Unwind_VRS_Result { _UVRSR_OK = 0 , _UVRSR_NOT_IMPLEMENTED = 1 , _UVRSR_FAILED = 2 , } # [repr (C)] enum _Unwind_VRS_RegClass { _UVRSC_CORE = 0 , _UVRSC_VFP = 1 , _UVRSC_FPA = 2 , _UVRSC_WMMXD = 3 , _UVRSC_WMMXC = 4 , } # [repr (C)] enum _Unwind_VRS_DataRepresentation { _UVRSD_UINT32 = 0 , _UVRSD_VFPX = 1 , _UVRSD_FPAX = 2 , _UVRSD_UINT64 = 3 , _UVRSD_FLOAT = 4 , _UVRSD_DOUBLE = 5 , } type _Unwind_Word = libc :: c_uint ; extern { fn _Unwind_VRS_Get (ctx : * mut _Unwind_Context , klass : _Unwind_VRS_RegClass , word : _Unwind_Word , repr : _Unwind_VRS_DataRepresentation , data : * mut libc :: c_void) -> _Unwind_VRS_Result ; } let mut val : _Unwind_Word = 0 ; let ptr = & mut val as * mut _Unwind_Word ; let _ = _Unwind_VRS_Get (ctx , _Unwind_VRS_RegClass :: _UVRSC_CORE , 15 , _Unwind_VRS_DataRepresentation :: _UVRSD_UINT32 , ptr as * mut libc :: c_void) ; (val & ! 1) as libc :: uintptr_t }
}

macro_rules! _Unwind_GetIPInfo_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _Unwind_GetIPInfo in module {}", module_path!());
    };
}

mkfn!{
    _Unwind_GetIPInfo_introspect!();
    # [cfg (any (all (target_os = "android" , target_arch = "arm") , all (target_os = "linux" , target_arch = "arm")))] pub unsafe fn _Unwind_GetIPInfo (ctx : * mut _Unwind_Context , ip_before_insn : * mut libc :: c_int) -> libc :: uintptr_t { * ip_before_insn = 0 ; _Unwind_GetIP (ctx) }
}

macro_rules! _Unwind_FindEnclosingFunction_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _Unwind_FindEnclosingFunction in module {}", module_path!());
    };
}

mkfn!{
    _Unwind_FindEnclosingFunction_introspect!();
    # [cfg (any (target_os = "android" , all (target_os = "linux" , target_arch = "arm")))] pub unsafe fn _Unwind_FindEnclosingFunction (pc : * mut libc :: c_void) -> * mut libc :: c_void { pc }
} 
            }}