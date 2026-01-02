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
mkuse!{use crate :: io :: { self , DEFAULT_BUF_SIZE , ErrorKind , IntoInnerError , IoSlice , Seek , SeekFrom , Write , } ;}
mkuse!{use crate :: mem :: { self , ManuallyDrop } ;}
mkuse!{use crate :: { error , fmt , ptr } ;}
mkitem!{mkstruct!{#[doc = " Wraps a writer and buffers its output."] #[doc = ""] #[doc = " It can be excessively inefficient to work directly with something that"] #[doc = " implements [`Write`]. For example, every call to"] #[doc = " [`write`][`TcpStream::write`] on [`TcpStream`] results in a system call. A"] #[doc = " `BufWriter<W>` keeps an in-memory buffer of data and writes it to an underlying"] #[doc = " writer in large, infrequent batches."] #[doc = ""] #[doc = " `BufWriter<W>` can improve the speed of programs that make *small* and"] #[doc = " *repeated* write calls to the same file or network socket. It does not"] #[doc = " help when writing very large amounts at once, or writing just one or a few"] #[doc = " times. It also provides no advantage when writing to a destination that is"] #[doc = " in memory, like a <code>[Vec]\\<u8></code>."] #[doc = ""] #[doc = " It is critical to call [`flush`] before `BufWriter<W>` is dropped. Though"] #[doc = " dropping will attempt to flush the contents of the buffer, any errors"] #[doc = " that happen in the process of dropping will be ignored. Calling [`flush`]"] #[doc = " ensures that the buffer is empty and thus dropping will not even attempt"] #[doc = " file operations."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " Let's write the numbers one through ten to a [`TcpStream`]:"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::io::prelude::*;"] #[doc = " use std::net::TcpStream;"] #[doc = ""] #[doc = " let mut stream = TcpStream::connect(\"127.0.0.1:34254\").unwrap();"] #[doc = ""] #[doc = " for i in 0..10 {"] #[doc = "     stream.write(&[i+1]).unwrap();"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " Because we're not buffering, we write each one in turn, incurring the"] #[doc = " overhead of a system call per byte written. We can fix this with a"] #[doc = " `BufWriter<W>`:"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::io::prelude::*;"] #[doc = " use std::io::BufWriter;"] #[doc = " use std::net::TcpStream;"] #[doc = ""] #[doc = " let mut stream = BufWriter::new(TcpStream::connect(\"127.0.0.1:34254\").unwrap());"] #[doc = ""] #[doc = " for i in 0..10 {"] #[doc = "     stream.write(&[i+1]).unwrap();"] #[doc = " }"] #[doc = " stream.flush().unwrap();"] #[doc = " ```"] #[doc = ""] #[doc = " By wrapping the stream with a `BufWriter<W>`, these ten writes are all grouped"] #[doc = " together by the buffer and will all be written out in one system call when"] #[doc = " the `stream` is flushed."] #[doc = ""] #[doc = " [`TcpStream::write`]: crate::net::TcpStream::write"] #[doc = " [`TcpStream`]: crate::net::TcpStream"] #[doc = " [`flush`]: BufWriter::flush"] #[stable (feature = "rust1" , since = "1.0.0")] pub struct BufWriter < W : ? Sized + Write > { buf : Vec < u8 > , panicked : bool , inner : W , }}}
mkitem!{mkimpl!{impl < W : Write > BufWriter < W > { #[doc = " Creates a new `BufWriter<W>` with a default buffer capacity. The default is currently 8 KiB,"] #[doc = " but may change in the future."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::io::BufWriter;"] #[doc = " use std::net::TcpStream;"] #[doc = ""] #[doc = " let mut buffer = BufWriter::new(TcpStream::connect(\"127.0.0.1:34254\").unwrap());"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] pub fn new (inner : W) -> BufWriter < W > { BufWriter :: with_capacity (DEFAULT_BUF_SIZE , inner) } pub (crate) fn try_new_buffer () -> io :: Result < Vec < u8 > > { Vec :: try_with_capacity (DEFAULT_BUF_SIZE) . map_err (| _ | { io :: const_error ! (ErrorKind :: OutOfMemory , "failed to allocate write buffer") }) } pub (crate) fn with_buffer (inner : W , buf : Vec < u8 >) -> Self { Self { inner , buf , panicked : false } } #[doc = " Creates a new `BufWriter<W>` with at least the specified buffer capacity."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " Creating a buffer with a buffer of at least a hundred bytes."] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::io::BufWriter;"] #[doc = " use std::net::TcpStream;"] #[doc = ""] #[doc = " let stream = TcpStream::connect(\"127.0.0.1:34254\").unwrap();"] #[doc = " let mut buffer = BufWriter::with_capacity(100, stream);"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] pub fn with_capacity (capacity : usize , inner : W) -> BufWriter < W > { BufWriter { inner , buf : Vec :: with_capacity (capacity) , panicked : false } } #[doc = " Unwraps this `BufWriter<W>`, returning the underlying writer."] #[doc = ""] #[doc = " The buffer is written out before returning the writer."] #[doc = ""] #[doc = " # Errors"] #[doc = ""] #[doc = " An [`Err`] will be returned if an error occurs while flushing the buffer."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::io::BufWriter;"] #[doc = " use std::net::TcpStream;"] #[doc = ""] #[doc = " let mut buffer = BufWriter::new(TcpStream::connect(\"127.0.0.1:34254\").unwrap());"] #[doc = ""] #[doc = " // unwrap the TcpStream and flush the buffer"] #[doc = " let stream = buffer.into_inner().unwrap();"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] pub fn into_inner (mut self) -> Result < W , IntoInnerError < BufWriter < W > > > { match self . flush_buf () { Err (e) => Err (IntoInnerError :: new (self , e)) , Ok (()) => Ok (self . into_parts () . 0) , } } #[doc = " Disassembles this `BufWriter<W>`, returning the underlying writer, and any buffered but"] #[doc = " unwritten data."] #[doc = ""] #[doc = " If the underlying writer panicked, it is not known what portion of the data was written."] #[doc = " In this case, we return `WriterPanicked` for the buffered data (from which the buffer"] #[doc = " contents can still be recovered)."] #[doc = ""] #[doc = " `into_parts` makes no attempt to flush data and cannot fail."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::io::{BufWriter, Write};"] #[doc = ""] #[doc = " let mut buffer = [0u8; 10];"] #[doc = " let mut stream = BufWriter::new(buffer.as_mut());"] #[doc = " write!(stream, \"too much data\").unwrap();"] #[doc = " stream.flush().expect_err(\"it doesn't fit\");"] #[doc = " let (recovered_writer, buffered_data) = stream.into_parts();"] #[doc = " assert_eq!(recovered_writer.len(), 0);"] #[doc = " assert_eq!(&buffered_data.unwrap(), b\"ata\");"] #[doc = " ```"] #[stable (feature = "bufwriter_into_parts" , since = "1.56.0")] pub fn into_parts (self) -> (W , Result < Vec < u8 > , WriterPanicked >) { let mut this = ManuallyDrop :: new (self) ; let buf = mem :: take (& mut this . buf) ; let buf = if ! this . panicked { Ok (buf) } else { Err (WriterPanicked { buf }) } ; let inner = unsafe { ptr :: read (& this . inner) } ; (inner , buf) } }}}
mkitem!{mkimpl!{impl < W : ? Sized + Write > BufWriter < W > { #[doc = " Send data in our local buffer into the inner writer, looping as"] #[doc = " necessary until either it's all been sent or an error occurs."] #[doc = ""] #[doc = " Because all the data in the buffer has been reported to our owner as"] #[doc = " \"successfully written\" (by returning nonzero success values from"] #[doc = " `write`), any 0-length writes from `inner` must be reported as i/o"] #[doc = " errors from this method."] pub (in crate :: io) fn flush_buf (& mut self) -> io :: Result < () > { #[doc = " Helper struct to ensure the buffer is updated after all the writes"] #[doc = " are complete. It tracks the number of written bytes and drains them"] #[doc = " all from the front of the buffer when dropped."] struct BufGuard < 'a > { buffer : & 'a mut Vec < u8 > , written : usize , } impl < 'a > BufGuard < 'a > { fn new (buffer : & 'a mut Vec < u8 >) -> Self { Self { buffer , written : 0 } } #[doc = " The unwritten part of the buffer"] fn remaining (& self) -> & [u8] { & self . buffer [self . written ..] } #[doc = " Flag some bytes as removed from the front of the buffer"] fn consume (& mut self , amt : usize) { self . written += amt ; } #[doc = " true if all of the bytes have been written"] fn done (& self) -> bool { self . written >= self . buffer . len () } } impl Drop for BufGuard < '_ > { fn drop (& mut self) { if self . written > 0 { self . buffer . drain (.. self . written) ; } } } let mut guard = BufGuard :: new (& mut self . buf) ; while ! guard . done () { self . panicked = true ; let r = self . inner . write (guard . remaining ()) ; self . panicked = false ; match r { Ok (0) => { return Err (io :: const_error ! (ErrorKind :: WriteZero , "failed to write the buffered data" ,)) ; } Ok (n) => guard . consume (n) , Err (ref e) if e . is_interrupted () => { } Err (e) => return Err (e) , } } Ok (()) } #[doc = " Buffer some data without flushing it, regardless of the size of the"] #[doc = " data. Writes as much as possible without exceeding capacity. Returns"] #[doc = " the number of bytes written."] pub (super) fn write_to_buf (& mut self , buf : & [u8]) -> usize { let available = self . spare_capacity () ; let amt_to_buffer = available . min (buf . len ()) ; unsafe { self . write_to_buffer_unchecked (& buf [.. amt_to_buffer]) ; } amt_to_buffer } #[doc = " Gets a reference to the underlying writer."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::io::BufWriter;"] #[doc = " use std::net::TcpStream;"] #[doc = ""] #[doc = " let mut buffer = BufWriter::new(TcpStream::connect(\"127.0.0.1:34254\").unwrap());"] #[doc = ""] #[doc = " // we can use reference just like buffer"] #[doc = " let reference = buffer.get_ref();"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] pub fn get_ref (& self) -> & W { & self . inner } #[doc = " Gets a mutable reference to the underlying writer."] #[doc = ""] #[doc = " It is inadvisable to directly write to the underlying writer."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::io::BufWriter;"] #[doc = " use std::net::TcpStream;"] #[doc = ""] #[doc = " let mut buffer = BufWriter::new(TcpStream::connect(\"127.0.0.1:34254\").unwrap());"] #[doc = ""] #[doc = " // we can use reference just like buffer"] #[doc = " let reference = buffer.get_mut();"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] pub fn get_mut (& mut self) -> & mut W { & mut self . inner } #[doc = " Returns a reference to the internally buffered data."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::io::BufWriter;"] #[doc = " use std::net::TcpStream;"] #[doc = ""] #[doc = " let buf_writer = BufWriter::new(TcpStream::connect(\"127.0.0.1:34254\").unwrap());"] #[doc = ""] #[doc = " // See how many bytes are currently buffered"] #[doc = " let bytes_buffered = buf_writer.buffer().len();"] #[doc = " ```"] #[stable (feature = "bufreader_buffer" , since = "1.37.0")] pub fn buffer (& self) -> & [u8] { & self . buf } #[doc = " Returns a mutable reference to the internal buffer."] #[doc = ""] #[doc = " This can be used to write data directly into the buffer without triggering writers"] #[doc = " to the underlying writer."] #[doc = ""] #[doc = " That the buffer is a `Vec` is an implementation detail."] #[doc = " Callers should not modify the capacity as there currently is no public API to do so"] #[doc = " and thus any capacity changes would be unexpected by the user."] pub (in crate :: io) fn buffer_mut (& mut self) -> & mut Vec < u8 > { & mut self . buf } #[doc = " Returns the number of bytes the internal buffer can hold without flushing."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::io::BufWriter;"] #[doc = " use std::net::TcpStream;"] #[doc = ""] #[doc = " let buf_writer = BufWriter::new(TcpStream::connect(\"127.0.0.1:34254\").unwrap());"] #[doc = ""] #[doc = " // Check the capacity of the inner buffer"] #[doc = " let capacity = buf_writer.capacity();"] #[doc = " // Calculate how many bytes can be written without flushing"] #[doc = " let without_flush = capacity - buf_writer.buffer().len();"] #[doc = " ```"] #[stable (feature = "buffered_io_capacity" , since = "1.46.0")] pub fn capacity (& self) -> usize { self . buf . capacity () } #[cold] #[inline (never)] fn write_cold (& mut self , buf : & [u8]) -> io :: Result < usize > { if buf . len () > self . spare_capacity () { self . flush_buf () ? ; } if buf . len () >= self . buf . capacity () { self . panicked = true ; let r = self . get_mut () . write (buf) ; self . panicked = false ; r } else { unsafe { self . write_to_buffer_unchecked (buf) ; } Ok (buf . len ()) } } #[cold] #[inline (never)] fn write_all_cold (& mut self , buf : & [u8]) -> io :: Result < () > { if buf . len () > self . spare_capacity () { self . flush_buf () ? ; } if buf . len () >= self . buf . capacity () { self . panicked = true ; let r = self . get_mut () . write_all (buf) ; self . panicked = false ; r } else { unsafe { self . write_to_buffer_unchecked (buf) ; } Ok (()) } } #[inline] unsafe fn write_to_buffer_unchecked (& mut self , buf : & [u8]) { debug_assert ! (buf . len () <= self . spare_capacity ()) ; let old_len = self . buf . len () ; let buf_len = buf . len () ; let src = buf . as_ptr () ; unsafe { let dst = self . buf . as_mut_ptr () . add (old_len) ; ptr :: copy_nonoverlapping (src , dst , buf_len) ; self . buf . set_len (old_len + buf_len) ; } } #[inline] fn spare_capacity (& self) -> usize { self . buf . capacity () - self . buf . len () } }}}
mkitem!{mkstruct!{#[stable (feature = "bufwriter_into_parts" , since = "1.56.0")] #[doc = " Error returned for the buffered data from `BufWriter::into_parts`, when the underlying"] #[doc = " writer has previously panicked.  Contains the (possibly partly written) buffered data."] #[doc = ""] #[doc = " # Example"] #[doc = ""] #[doc = " ```"] #[doc = " use std::io::{self, BufWriter, Write};"] #[doc = " use std::panic::{catch_unwind, AssertUnwindSafe};"] #[doc = ""] #[doc = " struct PanickingWriter;"] #[doc = " impl Write for PanickingWriter {"] #[doc = "   fn write(&mut self, buf: &[u8]) -> io::Result<usize> { panic!() }"] #[doc = "   fn flush(&mut self) -> io::Result<()> { panic!() }"] #[doc = " }"] #[doc = ""] #[doc = " let mut stream = BufWriter::new(PanickingWriter);"] #[doc = " write!(stream, \"some data\").unwrap();"] #[doc = " let result = catch_unwind(AssertUnwindSafe(|| {"] #[doc = "     stream.flush().unwrap()"] #[doc = " }));"] #[doc = " assert!(result.is_err());"] #[doc = " let (recovered_writer, buffered_data) = stream.into_parts();"] #[doc = " assert!(matches!(recovered_writer, PanickingWriter));"] #[doc = " assert_eq!(buffered_data.unwrap_err().into_inner(), b\"some data\");"] #[doc = " ```"] pub struct WriterPanicked { buf : Vec < u8 > , }}}
mkitem!{mkimpl!{impl WriterPanicked { #[doc = " Returns the perhaps-unwritten data.  Some of this data may have been written by the"] #[doc = " panicking call(s) to the underlying writer, so simply writing it again is not a good idea."] #[must_use = "`self` will be dropped if the result is not used"] #[stable (feature = "bufwriter_into_parts" , since = "1.56.0")] pub fn into_inner (self) -> Vec < u8 > { self . buf } }}}
mkitem!{mkimpl!{#[stable (feature = "bufwriter_into_parts" , since = "1.56.0")] impl error :: Error for WriterPanicked { }}}
mkitem!{mkimpl!{#[stable (feature = "bufwriter_into_parts" , since = "1.56.0")] impl fmt :: Display for WriterPanicked { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { "BufWriter inner writer panicked, what data remains unwritten is not known" . fmt (f) } }}}
mkitem!{mkimpl!{#[stable (feature = "bufwriter_into_parts" , since = "1.56.0")] impl fmt :: Debug for WriterPanicked { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("WriterPanicked") . field ("buffer" , & format_args ! ("{}/{}" , self . buf . len () , self . buf . capacity ())) . finish () } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < W : ? Sized + Write > Write for BufWriter < W > { #[inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { if buf . len () < self . spare_capacity () { unsafe { self . write_to_buffer_unchecked (buf) ; } Ok (buf . len ()) } else { self . write_cold (buf) } } #[inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { if buf . len () < self . spare_capacity () { unsafe { self . write_to_buffer_unchecked (buf) ; } Ok (()) } else { self . write_all_cold (buf) } } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { if self . get_ref () . is_write_vectored () { let mut saturated_total_len : usize = 0 ; for buf in bufs { saturated_total_len = saturated_total_len . saturating_add (buf . len ()) ; if saturated_total_len > self . spare_capacity () && ! self . buf . is_empty () { self . flush_buf () ? ; } if saturated_total_len >= self . buf . capacity () { self . panicked = true ; let r = self . get_mut () . write_vectored (bufs) ; self . panicked = false ; return r ; } } unsafe { bufs . iter () . for_each (| b | self . write_to_buffer_unchecked (b)) ; } ; Ok (saturated_total_len) } else { let mut iter = bufs . iter () ; let mut total_written = if let Some (buf) = iter . by_ref () . find (| & buf | ! buf . is_empty ()) { if buf . len () > self . spare_capacity () { self . flush_buf () ? ; } if buf . len () >= self . buf . capacity () { self . panicked = true ; let r = self . get_mut () . write (buf) ; self . panicked = false ; return r ; } else { unsafe { self . write_to_buffer_unchecked (buf) ; } buf . len () } } else { return Ok (0) ; } ; debug_assert ! (total_written != 0) ; for buf in iter { if buf . len () <= self . spare_capacity () { unsafe { self . write_to_buffer_unchecked (buf) ; } total_written += buf . len () ; } else { break ; } } Ok (total_written) } } fn is_write_vectored (& self) -> bool { true } fn flush (& mut self) -> io :: Result < () > { self . flush_buf () . and_then (| () | self . get_mut () . flush ()) } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < W : ? Sized + Write > fmt :: Debug for BufWriter < W > where W : fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("BufWriter") . field ("writer" , & & self . inner) . field ("buffer" , & format_args ! ("{}/{}" , self . buf . len () , self . buf . capacity ())) . finish () } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < W : ? Sized + Write + Seek > Seek for BufWriter < W > { #[doc = " Seek to the offset, in bytes, in the underlying writer."] #[doc = ""] #[doc = " Seeking always writes out the internal buffer before seeking."] fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { self . flush_buf () ? ; self . get_mut () . seek (pos) } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < W : ? Sized + Write > Drop for BufWriter < W > { fn drop (& mut self) { if ! self . panicked { let _r = self . flush_buf () ; } } }}}