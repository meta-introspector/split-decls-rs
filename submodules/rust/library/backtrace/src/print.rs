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
mkuse!{# [cfg (feature = "std")] use super :: { BacktraceFrame , BacktraceSymbol } ;}
mkuse!{use super :: { BytesOrWideString , Frame , SymbolName } ;}
mkuse!{use core :: ffi :: c_void ;}
mkuse!{use core :: fmt ;}
mkitem!{const HEX_WIDTH : usize = 2 + 2 * core :: mem :: size_of :: < usize > () ;}
mkmod!{fuchsia, { 
                getname!(fuchsia);
                getsrc!(fuchsia);
                getpath!(fuchsia);
                get_deps!(fuchsia);
                get_crates!(fuchsia);
                mkinclude!(fuchsia);
                 
            }}
mkitem!{mkstruct!{# [doc = " A formatter for backtraces."] # [doc = ""] # [doc = " This type can be used to print a backtrace regardless of where the backtrace"] # [doc = " itself comes from. If you have a `Backtrace` type then its `Debug`"] # [doc = " implementation already uses this printing format."] pub struct BacktraceFmt < 'a , 'b > { fmt : & 'a mut fmt :: Formatter < 'b > , frame_index : usize , format : PrintFmt , print_path : & 'a mut (dyn FnMut (& mut fmt :: Formatter < '_ > , BytesOrWideString < '_ >) -> fmt :: Result + 'b) , }}}
mkitem!{mkenum!{# [doc = " The styles of printing that we can print"] # [derive (Copy , Clone , Eq , PartialEq)] # [non_exhaustive] pub enum PrintFmt { # [doc = " Prints a terser backtrace which ideally only contains relevant information"] Short , # [doc = " Prints a backtrace that contains all possible information"] Full , }}}
mkitem!{mkimpl!{impl < 'a , 'b > BacktraceFmt < 'a , 'b > { # [doc = " Create a new `BacktraceFmt` which will write output to the provided"] # [doc = " `fmt`."] # [doc = ""] # [doc = " The `format` argument will control the style in which the backtrace is"] # [doc = " printed, and the `print_path` argument will be used to print the"] # [doc = " `BytesOrWideString` instances of filenames. This type itself doesn't do"] # [doc = " any printing of filenames, but this callback is required to do so."] pub fn new (fmt : & 'a mut fmt :: Formatter < 'b > , format : PrintFmt , print_path : & 'a mut (dyn FnMut (& mut fmt :: Formatter < '_ > , BytesOrWideString < '_ >) -> fmt :: Result + 'b) ,) -> Self { BacktraceFmt { fmt , frame_index : 0 , format , print_path , } } # [doc = " Prints a preamble for the backtrace about to be printed."] # [doc = ""] # [doc = " This is required on some platforms for backtraces to be fully"] # [doc = " symbolicated later, and otherwise this should just be the first method"] # [doc = " you call after creating a `BacktraceFmt`."] pub fn add_context (& mut self) -> fmt :: Result { # [cfg (target_os = "fuchsia")] fuchsia :: print_dso_context (self . fmt) ? ; Ok (()) } # [doc = " Adds a frame to the backtrace output."] # [doc = ""] # [doc = " This commit returns an RAII instance of a `BacktraceFrameFmt` which can be used"] # [doc = " to actually print a frame, and on destruction it will increment the"] # [doc = " frame counter."] pub fn frame (& mut self) -> BacktraceFrameFmt < '_ , 'a , 'b > { BacktraceFrameFmt { fmt : self , symbol_index : 0 , } } # [doc = " Completes the backtrace output."] # [doc = ""] # [doc = " This is currently a no-op but is added for future compatibility with"] # [doc = " backtrace formats."] pub fn finish (& mut self) -> fmt :: Result { # [cfg (target_os = "fuchsia")] fuchsia :: finish_context (self . fmt) ? ; Ok (()) } # [doc = " Inserts a message in the backtrace output."] # [doc = ""] # [doc = " This allows information to be inserted between frames,"] # [doc = " and won't increment the `frame_index` unlike the `frame`"] # [doc = " method."] pub fn message (& mut self , msg : & str) -> fmt :: Result { self . fmt . write_str (msg) } # [doc = " Return the inner formatter."] # [doc = ""] # [doc = " This is used for writing custom information between frames with `write!` and `writeln!`,"] # [doc = " and won't increment the `frame_index` unlike the `frame` method."] pub fn formatter (& mut self) -> & mut fmt :: Formatter < 'b > { self . fmt } }}}
mkitem!{mkstruct!{# [doc = " A formatter for just one frame of a backtrace."] # [doc = ""] # [doc = " This type is created by the `BacktraceFmt::frame` function."] pub struct BacktraceFrameFmt < 'fmt , 'a , 'b > { fmt : & 'fmt mut BacktraceFmt < 'a , 'b > , symbol_index : usize , }}}
mkitem!{mkimpl!{impl BacktraceFrameFmt < '_ , '_ , '_ > { # [doc = " Prints a `BacktraceFrame` with this frame formatter."] # [doc = ""] # [doc = " This will recursively print all `BacktraceSymbol` instances within the"] # [doc = " `BacktraceFrame`."] # [doc = ""] # [doc = " # Required features"] # [doc = ""] # [doc = " This function requires the `std` feature of the `backtrace` crate to be"] # [doc = " enabled, and the `std` feature is enabled by default."] # [cfg (feature = "std")] pub fn backtrace_frame (& mut self , frame : & BacktraceFrame) -> fmt :: Result { let symbols = frame . symbols () ; for symbol in symbols { self . backtrace_symbol (frame , symbol) ? ; } if symbols . is_empty () { self . print_raw (frame . ip () , None , None , None) ? ; } Ok (()) } # [doc = " Prints a `BacktraceSymbol` within a `BacktraceFrame`."] # [doc = ""] # [doc = " # Required features"] # [doc = ""] # [doc = " This function requires the `std` feature of the `backtrace` crate to be"] # [doc = " enabled, and the `std` feature is enabled by default."] # [cfg (feature = "std")] pub fn backtrace_symbol (& mut self , frame : & BacktraceFrame , symbol : & BacktraceSymbol ,) -> fmt :: Result { self . print_raw_with_column (frame . ip () , symbol . name () , symbol . filename () . and_then (| p | Some (BytesOrWideString :: Bytes (p . to_str () ? . as_bytes ()))) , symbol . lineno () , symbol . colno () ,) ? ; Ok (()) } # [doc = " Prints a raw traced `Frame` and `Symbol`, typically from within the raw"] # [doc = " callbacks of this crate."] pub fn symbol (& mut self , frame : & Frame , symbol : & super :: Symbol) -> fmt :: Result { self . print_raw_with_column (frame . ip () , symbol . name () , symbol . filename_raw () , symbol . lineno () , symbol . colno () ,) ? ; Ok (()) } # [doc = " Adds a raw frame to the backtrace output."] # [doc = ""] # [doc = " This method, unlike the previous, takes the raw arguments in case"] # [doc = " they're being source from different locations. Note that this may be"] # [doc = " called multiple times for one frame."] pub fn print_raw (& mut self , frame_ip : * mut c_void , symbol_name : Option < SymbolName < '_ > > , filename : Option < BytesOrWideString < '_ > > , lineno : Option < u32 > ,) -> fmt :: Result { self . print_raw_with_column (frame_ip , symbol_name , filename , lineno , None) } # [doc = " Adds a raw frame to the backtrace output, including column information."] # [doc = ""] # [doc = " This method, like the previous, takes the raw arguments in case"] # [doc = " they're being source from different locations. Note that this may be"] # [doc = " called multiple times for one frame."] pub fn print_raw_with_column (& mut self , frame_ip : * mut c_void , symbol_name : Option < SymbolName < '_ > > , filename : Option < BytesOrWideString < '_ > > , lineno : Option < u32 > , colno : Option < u32 > ,) -> fmt :: Result { if cfg ! (target_os = "fuchsia") { self . print_raw_fuchsia (frame_ip) ? ; } else { self . print_raw_generic (frame_ip , symbol_name , filename , lineno , colno) ? ; } self . symbol_index += 1 ; Ok (()) } # [allow (unused_mut)] fn print_raw_generic (& mut self , frame_ip : * mut c_void , symbol_name : Option < SymbolName < '_ > > , filename : Option < BytesOrWideString < '_ > > , lineno : Option < u32 > , colno : Option < u32 > ,) -> fmt :: Result { if let PrintFmt :: Short = self . fmt . format { if frame_ip . is_null () { return Ok (()) ; } } if self . symbol_index == 0 { write ! (self . fmt . fmt , "{:4}: " , self . fmt . frame_index) ? ; if let PrintFmt :: Full = self . fmt . format { write ! (self . fmt . fmt , "{frame_ip:HEX_WIDTH$?} - ") ? ; } } else { write ! (self . fmt . fmt , "      ") ? ; if let PrintFmt :: Full = self . fmt . format { write ! (self . fmt . fmt , "{:1$}" , "" , HEX_WIDTH + 3) ? ; } } match (symbol_name , & self . fmt . format) { (Some (name) , PrintFmt :: Short) => write ! (self . fmt . fmt , "{name:#}") ? , (Some (name) , PrintFmt :: Full) => write ! (self . fmt . fmt , "{name}") ? , (None , _) => write ! (self . fmt . fmt , "<unknown>") ? , } self . fmt . fmt . write_str ("\n") ? ; if let (Some (file) , Some (line)) = (filename , lineno) { self . print_fileline (file , line , colno) ? ; } Ok (()) } fn print_fileline (& mut self , file : BytesOrWideString < '_ > , line : u32 , colno : Option < u32 > ,) -> fmt :: Result { if let PrintFmt :: Full = self . fmt . format { write ! (self . fmt . fmt , "{:1$}" , "" , HEX_WIDTH) ? ; } write ! (self . fmt . fmt , "             at ") ? ; (self . fmt . print_path) (self . fmt . fmt , file) ? ; write ! (self . fmt . fmt , ":{line}") ? ; if let Some (colno) = colno { write ! (self . fmt . fmt , ":{colno}") ? ; } writeln ! (self . fmt . fmt) ? ; Ok (()) } fn print_raw_fuchsia (& mut self , frame_ip : * mut c_void) -> fmt :: Result { if self . symbol_index == 0 { self . fmt . fmt . write_str ("{{{bt:") ? ; write ! (self . fmt . fmt , "{}:{:?}" , self . fmt . frame_index , frame_ip) ? ; self . fmt . fmt . write_str ("}}}\n") ? ; } Ok (()) } }}}
mkitem!{mkimpl!{impl Drop for BacktraceFrameFmt < '_ , '_ , '_ > { fn drop (& mut self) { self . fmt . frame_index += 1 ; } }}}