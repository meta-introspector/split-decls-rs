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
mkuse!{use r_efi :: protocols :: { simple_text_input , simple_text_output } ;}
mkuse!{use super :: env :: { CommandEnv , CommandEnvs } ;}
mkuse!{use crate :: collections :: BTreeMap ;}
mkuse!{pub use crate :: ffi :: OsString as EnvKey ;}
mkuse!{use crate :: ffi :: { OsStr , OsString } ;}
mkuse!{use crate :: num :: { NonZero , NonZeroI32 } ;}
mkuse!{use crate :: path :: Path ;}
mkuse!{use crate :: sys :: fs :: File ;}
mkuse!{use crate :: sys :: pal :: helpers ;}
mkuse!{use crate :: sys :: pal :: os :: error_string ;}
mkuse!{use crate :: sys :: pipe :: AnonPipe ;}
mkuse!{use crate :: sys :: unsupported ;}
mkuse!{use crate :: { fmt , io } ;}
mkitem!{mkstruct!{#[derive (Debug)] pub struct Command { prog : OsString , args : Vec < OsString > , stdout : Option < Stdio > , stderr : Option < Stdio > , stdin : Option < Stdio > , env : CommandEnv , }}}
mkitem!{mkstruct!{pub struct StdioPipes { pub stdin : Option < AnonPipe > , pub stdout : Option < AnonPipe > , pub stderr : Option < AnonPipe > , }}}
mkitem!{mkenum!{#[derive (Copy , Clone , Debug)] pub enum Stdio { Inherit , Null , MakePipe , }}}
mkitem!{mkimpl!{impl Command { pub fn new (program : & OsStr) -> Command { Command { prog : program . to_os_string () , args : Vec :: new () , stdout : None , stderr : None , stdin : None , env : Default :: default () , } } pub fn arg (& mut self , arg : & OsStr) { self . args . push (arg . to_os_string ()) ; } pub fn env_mut (& mut self) -> & mut CommandEnv { & mut self . env } pub fn cwd (& mut self , _dir : & OsStr) { panic ! ("unsupported") } pub fn stdin (& mut self , stdin : Stdio) { self . stdin = Some (stdin) ; } pub fn stdout (& mut self , stdout : Stdio) { self . stdout = Some (stdout) ; } pub fn stderr (& mut self , stderr : Stdio) { self . stderr = Some (stderr) ; } pub fn get_program (& self) -> & OsStr { self . prog . as_ref () } pub fn get_args (& self) -> CommandArgs < '_ > { CommandArgs { iter : self . args . iter () } } pub fn get_envs (& self) -> CommandEnvs < '_ > { self . env . iter () } pub fn get_current_dir (& self) -> Option < & Path > { None } pub fn spawn (& mut self , _default : Stdio , _needs_stdin : bool ,) -> io :: Result < (Process , StdioPipes) > { unsupported () } fn create_pipe (s : Stdio ,) -> io :: Result < Option < helpers :: OwnedProtocol < uefi_command_internal :: PipeProtocol > > > { match s { Stdio :: MakePipe => unsafe { helpers :: OwnedProtocol :: create (uefi_command_internal :: PipeProtocol :: new () , simple_text_output :: PROTOCOL_GUID ,) } . map (Some) , Stdio :: Null => unsafe { helpers :: OwnedProtocol :: create (uefi_command_internal :: PipeProtocol :: null () , simple_text_output :: PROTOCOL_GUID ,) } . map (Some) , Stdio :: Inherit => Ok (None) , } } fn create_stdin (s : Stdio ,) -> io :: Result < Option < helpers :: OwnedProtocol < uefi_command_internal :: InputProtocol > > > { match s { Stdio :: Null => unsafe { helpers :: OwnedProtocol :: create (uefi_command_internal :: InputProtocol :: null () , simple_text_input :: PROTOCOL_GUID ,) } . map (Some) , Stdio :: Inherit => Ok (None) , Stdio :: MakePipe => unsupported () , } } }}}

macro_rules! output_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function output in module {}", module_path!());
    };
}

mkfn!{
    output_introspect!();
    pub fn output (command : & mut Command) -> io :: Result < (ExitStatus , Vec < u8 > , Vec < u8 >) > { let mut cmd = uefi_command_internal :: Image :: load_image (& command . prog) ? ; if ! command . args . is_empty () { let args = uefi_command_internal :: create_args (& command . prog , & command . args) ; cmd . set_args (args) ; } let stdout = command . stdout . unwrap_or (Stdio :: MakePipe) ; let stdout = Command :: create_pipe (stdout) ? ; if let Some (con) = stdout { cmd . stdout_init (con) } else { cmd . stdout_inherit () } ; let stderr = command . stderr . unwrap_or (Stdio :: MakePipe) ; let stderr = Command :: create_pipe (stderr) ? ; if let Some (con) = stderr { cmd . stderr_init (con) } else { cmd . stderr_inherit () } ; let stdin = command . stdin . unwrap_or (Stdio :: Null) ; let stdin = Command :: create_stdin (stdin) ? ; if let Some (con) = stdin { cmd . stdin_init (con) } else { cmd . stdin_inherit () } ; let env = env_changes (& command . env) ; if let Some (e) = & env { for (k , (_ , v)) in e { match v { Some (v) => unsafe { crate :: env :: set_var (k , v) } , None => unsafe { crate :: env :: remove_var (k) } , } } } let stat = cmd . start_image () ? ; if let Some (e) = env { for (k , (v , _)) in e { match v { Some (v) => unsafe { crate :: env :: set_var (k , v) } , None => unsafe { crate :: env :: remove_var (k) } , } } } let stdout = cmd . stdout () ? ; let stderr = cmd . stderr () ? ; Ok ((ExitStatus (stat) , stdout , stderr)) }
}
mkitem!{mkimpl!{impl From < AnonPipe > for Stdio { fn from (pipe : AnonPipe) -> Stdio { pipe . diverge () } }}}
mkitem!{mkimpl!{impl From < io :: Stdout > for Stdio { fn from (_ : io :: Stdout) -> Stdio { panic ! ("unsupported") } }}}
mkitem!{mkimpl!{impl From < io :: Stderr > for Stdio { fn from (_ : io :: Stderr) -> Stdio { panic ! ("unsupported") } }}}
mkitem!{mkimpl!{impl From < File > for Stdio { fn from (_file : File) -> Stdio { panic ! ("unsupported") } }}}
mkitem!{mkstruct!{#[derive (PartialEq , Eq , Clone , Copy , Debug)] #[non_exhaustive] pub struct ExitStatus (r_efi :: efi :: Status) ;}}
mkitem!{mkimpl!{impl ExitStatus { pub fn exit_ok (& self) -> Result < () , ExitStatusError > { if self . 0 == r_efi :: efi :: Status :: SUCCESS { Ok (()) } else { Err (ExitStatusError (self . 0)) } } pub fn code (& self) -> Option < i32 > { Some (self . 0 . as_usize () as i32) } }}}
mkitem!{mkimpl!{impl fmt :: Display for ExitStatus { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let err_str = error_string (self . 0 . as_usize ()) ; write ! (f , "{}" , err_str) } }}}
mkitem!{mkimpl!{impl Default for ExitStatus { fn default () -> Self { ExitStatus (r_efi :: efi :: Status :: SUCCESS) } }}}
mkitem!{mkstruct!{#[derive (Clone , Copy , PartialEq , Eq)] pub struct ExitStatusError (r_efi :: efi :: Status) ;}}
mkitem!{mkimpl!{impl fmt :: Debug for ExitStatusError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let err_str = error_string (self . 0 . as_usize ()) ; write ! (f , "{}" , err_str) } }}}
mkitem!{mkimpl!{impl Into < ExitStatus > for ExitStatusError { fn into (self) -> ExitStatus { ExitStatus (self . 0) } }}}
mkitem!{mkimpl!{impl ExitStatusError { pub fn code (self) -> Option < NonZero < i32 > > { NonZeroI32 :: new (self . 0 . as_usize () as i32) } }}}
mkitem!{mkstruct!{#[derive (PartialEq , Eq , Clone , Copy , Debug)] pub struct ExitCode (bool) ;}}
mkitem!{mkimpl!{impl ExitCode { pub const SUCCESS : ExitCode = ExitCode (false) ; pub const FAILURE : ExitCode = ExitCode (true) ; pub fn as_i32 (& self) -> i32 { self . 0 as i32 } }}}
mkitem!{mkimpl!{impl From < u8 > for ExitCode { fn from (code : u8) -> Self { match code { 0 => Self :: SUCCESS , 1 ..= 255 => Self :: FAILURE , } } }}}
mkitem!{mkstruct!{pub struct Process (!) ;}}
mkitem!{mkimpl!{impl Process { pub fn id (& self) -> u32 { self . 0 } pub fn kill (& mut self) -> io :: Result < () > { self . 0 } pub fn wait (& mut self) -> io :: Result < ExitStatus > { self . 0 } pub fn try_wait (& mut self) -> io :: Result < Option < ExitStatus > > { self . 0 } }}}
mkitem!{mkstruct!{pub struct CommandArgs < 'a > { iter : crate :: slice :: Iter < 'a , OsString > , }}}
mkitem!{mkimpl!{impl < 'a > Iterator for CommandArgs < 'a > { type Item = & 'a OsStr ; fn next (& mut self) -> Option < & 'a OsStr > { self . iter . next () . map (| x | x . as_ref ()) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }}}
mkitem!{mkimpl!{impl < 'a > ExactSizeIterator for CommandArgs < 'a > { fn len (& self) -> usize { self . iter . len () } fn is_empty (& self) -> bool { self . iter . is_empty () } }}}
mkitem!{mkimpl!{impl < 'a > fmt :: Debug for CommandArgs < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter . clone ()) . finish () } }}}
mkmod!{uefi_command_internal, { 
                getname!(uefi_command_internal);
                getsrc!(uefi_command_internal);
                getpath!(uefi_command_internal);
                get_deps!(uefi_command_internal);
                get_crates!(uefi_command_internal);
                mkinclude!(uefi_command_internal);
                mkuse!{use r_efi :: protocols :: { loaded_image , simple_text_input , simple_text_output } ;}
mkuse!{use crate :: ffi :: { OsStr , OsString } ;}
mkuse!{use crate :: io :: { self , const_error } ;}
mkuse!{use crate :: mem :: MaybeUninit ;}
mkuse!{use crate :: os :: uefi :: env :: { boot_services , image_handle , system_table } ;}
mkuse!{use crate :: os :: uefi :: ffi :: { OsStrExt , OsStringExt } ;}
mkuse!{use crate :: ptr :: NonNull ;}
mkuse!{use crate :: slice ;}
mkuse!{use crate :: sys :: pal :: helpers :: { self , OwnedTable } ;}
mkuse!{use crate :: sys_common :: wstr :: WStrUnits ;}
mkitem!{mkstruct!{pub struct Image { handle : NonNull < crate :: ffi :: c_void > , stdout : Option < helpers :: OwnedProtocol < PipeProtocol > > , stderr : Option < helpers :: OwnedProtocol < PipeProtocol > > , stdin : Option < helpers :: OwnedProtocol < InputProtocol > > , st : OwnedTable < r_efi :: efi :: SystemTable > , args : Option < (* mut u16 , usize) > , }}}
mkitem!{mkimpl!{impl Image { pub fn load_image (p : & OsStr) -> io :: Result < Self > { let path = helpers :: OwnedDevicePath :: from_text (p) ? ; let boot_services : NonNull < r_efi :: efi :: BootServices > = boot_services () . ok_or_else (| | const_error ! (io :: ErrorKind :: NotFound , "Boot Services not found")) ? . cast () ; let mut child_handle : MaybeUninit < r_efi :: efi :: Handle > = MaybeUninit :: uninit () ; let image_handle = image_handle () ; let r = unsafe { ((* boot_services . as_ptr ()) . load_image) (r_efi :: efi :: Boolean :: FALSE , image_handle . as_ptr () , path . as_ptr () , crate :: ptr :: null_mut () , 0 , child_handle . as_mut_ptr () ,) } ; if r . is_error () { Err (io :: Error :: from_raw_os_error (r . as_usize ())) } else { let child_handle = unsafe { child_handle . assume_init () } ; let child_handle = NonNull :: new (child_handle) . unwrap () ; let loaded_image : NonNull < loaded_image :: Protocol > = helpers :: open_protocol (child_handle , loaded_image :: PROTOCOL_GUID) . unwrap () ; let st = OwnedTable :: from_table (unsafe { (* loaded_image . as_ptr ()) . system_table }) ; Ok (Self { handle : child_handle , stdout : None , stderr : None , stdin : None , st , args : None , }) } } pub (crate) fn start_image (& mut self) -> io :: Result < r_efi :: efi :: Status > { self . update_st_crc32 () ? ; let loaded_image : NonNull < loaded_image :: Protocol > = helpers :: open_protocol (self . handle , loaded_image :: PROTOCOL_GUID) . unwrap () ; unsafe { (* loaded_image . as_ptr ()) . system_table = self . st . as_mut_ptr () ; } let boot_services : NonNull < r_efi :: efi :: BootServices > = boot_services () . ok_or_else (| | const_error ! (io :: ErrorKind :: NotFound , "Boot Services not found")) ? . cast () ; let mut exit_data_size : usize = 0 ; let mut exit_data : MaybeUninit < * mut u16 > = MaybeUninit :: uninit () ; let r = unsafe { ((* boot_services . as_ptr ()) . start_image) (self . handle . as_ptr () , & mut exit_data_size , exit_data . as_mut_ptr () ,) } ; if exit_data_size != 0 { unsafe { let exit_data = exit_data . assume_init () ; ((* boot_services . as_ptr ()) . free_pool) (exit_data as * mut crate :: ffi :: c_void) ; } } Ok (r) } fn set_stdout (& mut self , handle : r_efi :: efi :: Handle , protocol : * mut simple_text_output :: Protocol ,) { unsafe { (* self . st . as_mut_ptr ()) . console_out_handle = handle ; (* self . st . as_mut_ptr ()) . con_out = protocol ; } } fn set_stderr (& mut self , handle : r_efi :: efi :: Handle , protocol : * mut simple_text_output :: Protocol ,) { unsafe { (* self . st . as_mut_ptr ()) . standard_error_handle = handle ; (* self . st . as_mut_ptr ()) . std_err = protocol ; } } fn set_stdin (& mut self , handle : r_efi :: efi :: Handle , protocol : * mut simple_text_input :: Protocol ,) { unsafe { (* self . st . as_mut_ptr ()) . console_in_handle = handle ; (* self . st . as_mut_ptr ()) . con_in = protocol ; } } pub fn stdout_init (& mut self , protocol : helpers :: OwnedProtocol < PipeProtocol >) { self . set_stdout (protocol . handle () . as_ptr () , protocol . as_ref () as * const PipeProtocol as * mut simple_text_output :: Protocol ,) ; self . stdout = Some (protocol) ; } pub fn stdout_inherit (& mut self) { let st : NonNull < r_efi :: efi :: SystemTable > = system_table () . cast () ; unsafe { self . set_stdout ((* st . as_ptr ()) . console_out_handle , (* st . as_ptr ()) . con_out) } } pub fn stderr_init (& mut self , protocol : helpers :: OwnedProtocol < PipeProtocol >) { self . set_stderr (protocol . handle () . as_ptr () , protocol . as_ref () as * const PipeProtocol as * mut simple_text_output :: Protocol ,) ; self . stderr = Some (protocol) ; } pub fn stderr_inherit (& mut self) { let st : NonNull < r_efi :: efi :: SystemTable > = system_table () . cast () ; unsafe { self . set_stderr ((* st . as_ptr ()) . standard_error_handle , (* st . as_ptr ()) . std_err) } } pub (crate) fn stdin_init (& mut self , protocol : helpers :: OwnedProtocol < InputProtocol >) { self . set_stdin (protocol . handle () . as_ptr () , protocol . as_ref () as * const InputProtocol as * mut simple_text_input :: Protocol ,) ; self . stdin = Some (protocol) ; } pub (crate) fn stdin_inherit (& mut self) { let st : NonNull < r_efi :: efi :: SystemTable > = system_table () . cast () ; unsafe { self . set_stdin ((* st . as_ptr ()) . console_in_handle , (* st . as_ptr ()) . con_in) } } pub fn stderr (& self) -> io :: Result < Vec < u8 > > { match & self . stderr { Some (stderr) => stderr . as_ref () . utf8 () , None => Ok (Vec :: new ()) , } } pub fn stdout (& self) -> io :: Result < Vec < u8 > > { match & self . stdout { Some (stdout) => stdout . as_ref () . utf8 () , None => Ok (Vec :: new ()) , } } pub fn set_args (& mut self , args : Box < [u16] >) { let loaded_image : NonNull < loaded_image :: Protocol > = helpers :: open_protocol (self . handle , loaded_image :: PROTOCOL_GUID) . unwrap () ; let len = args . len () ; let args_size : u32 = (len * size_of :: < u16 > ()) . try_into () . unwrap () ; let ptr = Box :: into_raw (args) . as_mut_ptr () ; unsafe { (* loaded_image . as_ptr ()) . load_options = ptr as * mut crate :: ffi :: c_void ; (* loaded_image . as_ptr ()) . load_options_size = args_size ; } self . args = Some ((ptr , len)) ; } fn update_st_crc32 (& mut self) -> io :: Result < () > { let bt : NonNull < r_efi :: efi :: BootServices > = boot_services () . unwrap () . cast () ; let st_size = unsafe { (* self . st . as_ptr ()) . hdr . header_size as usize } ; let mut crc32 : u32 = 0 ; unsafe { (* self . st . as_mut_ptr ()) . hdr . crc32 = 0 ; } let r = unsafe { ((* bt . as_ptr ()) . calculate_crc32) (self . st . as_mut_ptr () as * mut crate :: ffi :: c_void , st_size , & mut crc32 ,) } ; if r . is_error () { Err (io :: Error :: from_raw_os_error (r . as_usize ())) } else { unsafe { (* self . st . as_mut_ptr ()) . hdr . crc32 = crc32 ; } Ok (()) } } }}}
mkitem!{mkimpl!{impl Drop for Image { fn drop (& mut self) { if let Some (bt) = boot_services () { let bt : NonNull < r_efi :: efi :: BootServices > = bt . cast () ; unsafe { ((* bt . as_ptr ()) . unload_image) (self . handle . as_ptr ()) ; } } if let Some ((ptr , len)) = self . args { let _ = unsafe { Box :: from_raw (crate :: ptr :: slice_from_raw_parts_mut (ptr , len)) } ; } } }}}
mkitem!{mkstruct!{#[repr (C)] pub struct PipeProtocol { reset : simple_text_output :: ProtocolReset , output_string : simple_text_output :: ProtocolOutputString , test_string : simple_text_output :: ProtocolTestString , query_mode : simple_text_output :: ProtocolQueryMode , set_mode : simple_text_output :: ProtocolSetMode , set_attribute : simple_text_output :: ProtocolSetAttribute , clear_screen : simple_text_output :: ProtocolClearScreen , set_cursor_position : simple_text_output :: ProtocolSetCursorPosition , enable_cursor : simple_text_output :: ProtocolEnableCursor , mode : * mut simple_text_output :: Mode , _buffer : Vec < u16 > , }}}
mkitem!{mkimpl!{impl PipeProtocol { pub fn new () -> Self { let mode = Box :: new (simple_text_output :: Mode { max_mode : 0 , mode : 0 , attribute : 0 , cursor_column : 0 , cursor_row : 0 , cursor_visible : r_efi :: efi :: Boolean :: FALSE , }) ; Self { reset : Self :: reset , output_string : Self :: output_string , test_string : Self :: test_string , query_mode : Self :: query_mode , set_mode : Self :: set_mode , set_attribute : Self :: set_attribute , clear_screen : Self :: clear_screen , set_cursor_position : Self :: set_cursor_position , enable_cursor : Self :: enable_cursor , mode : Box :: into_raw (mode) , _buffer : Vec :: new () , } } pub fn null () -> Self { let mode = Box :: new (simple_text_output :: Mode { max_mode : 0 , mode : 0 , attribute : 0 , cursor_column : 0 , cursor_row : 0 , cursor_visible : r_efi :: efi :: Boolean :: FALSE , }) ; Self { reset : Self :: reset_null , output_string : Self :: output_string_null , test_string : Self :: test_string , query_mode : Self :: query_mode , set_mode : Self :: set_mode , set_attribute : Self :: set_attribute , clear_screen : Self :: clear_screen , set_cursor_position : Self :: set_cursor_position , enable_cursor : Self :: enable_cursor , mode : Box :: into_raw (mode) , _buffer : Vec :: new () , } } pub fn utf8 (& self) -> io :: Result < Vec < u8 > > { OsString :: from_wide (& self . _buffer) . into_string () . map (Into :: into) . map_err (| _ | const_error ! (io :: ErrorKind :: Other , "UTF-8 conversion failed")) } extern "efiapi" fn reset (proto : * mut simple_text_output :: Protocol , _ : r_efi :: efi :: Boolean ,) -> r_efi :: efi :: Status { let proto : * mut PipeProtocol = proto . cast () ; unsafe { (* proto) . _buffer . clear () ; } r_efi :: efi :: Status :: SUCCESS } extern "efiapi" fn reset_null (_ : * mut simple_text_output :: Protocol , _ : r_efi :: efi :: Boolean ,) -> r_efi :: efi :: Status { r_efi :: efi :: Status :: SUCCESS } extern "efiapi" fn output_string (proto : * mut simple_text_output :: Protocol , buf : * mut r_efi :: efi :: Char16 ,) -> r_efi :: efi :: Status { let proto : * mut PipeProtocol = proto . cast () ; let buf_len = unsafe { if let Some (x) = WStrUnits :: new (buf) { x . count () } else { return r_efi :: efi :: Status :: INVALID_PARAMETER ; } } ; let buf_slice = unsafe { slice :: from_raw_parts (buf , buf_len) } ; unsafe { (* proto) . _buffer . extend_from_slice (buf_slice) ; } ; r_efi :: efi :: Status :: SUCCESS } extern "efiapi" fn output_string_null (_ : * mut simple_text_output :: Protocol , _ : * mut r_efi :: efi :: Char16 ,) -> r_efi :: efi :: Status { r_efi :: efi :: Status :: SUCCESS } extern "efiapi" fn test_string (_ : * mut simple_text_output :: Protocol , _ : * mut r_efi :: efi :: Char16 ,) -> r_efi :: efi :: Status { r_efi :: efi :: Status :: SUCCESS } extern "efiapi" fn query_mode (_ : * mut simple_text_output :: Protocol , _ : usize , _ : * mut usize , _ : * mut usize ,) -> r_efi :: efi :: Status { r_efi :: efi :: Status :: UNSUPPORTED } extern "efiapi" fn set_mode (_ : * mut simple_text_output :: Protocol , _ : usize ,) -> r_efi :: efi :: Status { r_efi :: efi :: Status :: UNSUPPORTED } extern "efiapi" fn set_attribute (_ : * mut simple_text_output :: Protocol , _ : usize ,) -> r_efi :: efi :: Status { r_efi :: efi :: Status :: UNSUPPORTED } extern "efiapi" fn clear_screen (_ : * mut simple_text_output :: Protocol ,) -> r_efi :: efi :: Status { r_efi :: efi :: Status :: UNSUPPORTED } extern "efiapi" fn set_cursor_position (_ : * mut simple_text_output :: Protocol , _ : usize , _ : usize ,) -> r_efi :: efi :: Status { r_efi :: efi :: Status :: UNSUPPORTED } extern "efiapi" fn enable_cursor (_ : * mut simple_text_output :: Protocol , _ : r_efi :: efi :: Boolean ,) -> r_efi :: efi :: Status { r_efi :: efi :: Status :: UNSUPPORTED } }}}
mkitem!{mkimpl!{impl Drop for PipeProtocol { fn drop (& mut self) { unsafe { let _ = Box :: from_raw (self . mode) ; } } }}}
mkitem!{mkstruct!{#[repr (C)] pub (crate) struct InputProtocol { reset : simple_text_input :: ProtocolReset , read_key_stroke : simple_text_input :: ProtocolReadKeyStroke , wait_for_key : r_efi :: efi :: Event , }}}
mkitem!{mkimpl!{impl InputProtocol { pub (crate) fn null () -> Self { let evt = helpers :: OwnedEvent :: new (r_efi :: efi :: EVT_NOTIFY_WAIT , r_efi :: efi :: TPL_CALLBACK , Some (Self :: empty_notify) , None ,) . unwrap () ; Self { reset : Self :: null_reset , read_key_stroke : Self :: null_read_key , wait_for_key : evt . into_raw () , } } extern "efiapi" fn null_reset (_ : * mut simple_text_input :: Protocol , _ : r_efi :: efi :: Boolean ,) -> r_efi :: efi :: Status { r_efi :: efi :: Status :: SUCCESS } extern "efiapi" fn null_read_key (_ : * mut simple_text_input :: Protocol , _ : * mut simple_text_input :: InputKey ,) -> r_efi :: efi :: Status { r_efi :: efi :: Status :: UNSUPPORTED } extern "efiapi" fn empty_notify (_ : r_efi :: efi :: Event , _ : * mut crate :: ffi :: c_void) { } }}}
mkitem!{mkimpl!{impl Drop for InputProtocol { fn drop (& mut self) { unsafe { let _ = helpers :: OwnedEvent :: from_raw (self . wait_for_key) ; } } }}}

macro_rules! create_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_args in module {}", module_path!());
    };
}

mkfn!{
    create_args_introspect!();
    pub fn create_args (prog : & OsStr , args : & [OsString]) -> Box < [u16] > { const QUOTE : u16 = 0x0022 ; const SPACE : u16 = 0x0020 ; const CARET : u16 = 0x005e ; const NULL : u16 = 0 ; let mut res = Vec :: with_capacity (args . iter () . map (| arg | arg . len () + 3) . sum ()) ; res . push (QUOTE) ; res . extend (prog . encode_wide ()) ; res . push (QUOTE) ; for arg in args { res . push (SPACE) ; res . push (QUOTE) ; for c in arg . encode_wide () { if c == QUOTE || c == CARET { res . push (CARET) ; } res . push (c) ; } res . push (QUOTE) ; } res . into_boxed_slice () }
} 
            }}

macro_rules! env_changes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function env_changes in module {}", module_path!());
    };
}

mkfn!{
    env_changes_introspect!();
    #[doc = " Create a map of environment variable changes. Allows efficient setting and rolling back of"] #[doc = " environment variable changes."] #[doc = ""] #[doc = " Entry: (Old Value, New Value)"] fn env_changes (env : & CommandEnv) -> Option < BTreeMap < EnvKey , (Option < OsString > , Option < OsString >) > > { if env . is_unchanged () { return None ; } let mut result = BTreeMap :: < EnvKey , (Option < OsString > , Option < OsString >) > :: new () ; if env . does_clear () { for (k , v) in crate :: env :: vars_os () { result . insert (k . into () , (Some (v) , None)) ; } } for (k , v) in env . iter () { let v : Option < OsString > = v . map (Into :: into) ; result . entry (k . into ()) . and_modify (| cur | * cur = (cur . 0 . clone () , v . clone ())) . or_insert ((crate :: env :: var_os (k) , v)) ; } Some (result) }
}