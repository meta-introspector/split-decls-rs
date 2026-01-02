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
mkmod!{buffer, { 
                getname!(buffer);
                getsrc!(buffer);
                getpath!(buffer);
                get_deps!(buffer);
                get_crates!(buffer);
                mkinclude!(buffer);
                 
            }}
mkuse!{use buffer :: Buffer ;}
mkuse!{use crate :: fmt ;}
mkuse!{use crate :: io :: { self , BorrowedCursor , BufRead , DEFAULT_BUF_SIZE , IoSliceMut , Read , Seek , SeekFrom , SizeHint , SpecReadByte , uninlined_slow_read_byte , } ;}
mkitem!{mkstruct!{# [doc = " The `BufReader<R>` struct adds buffering to any reader."] # [doc = ""] # [doc = " It can be excessively inefficient to work directly with a [`Read`] instance."] # [doc = " For example, every call to [`read`][`TcpStream::read`] on [`TcpStream`]"] # [doc = " results in a system call. A `BufReader<R>` performs large, infrequent reads on"] # [doc = " the underlying [`Read`] and maintains an in-memory buffer of the results."] # [doc = ""] # [doc = " `BufReader<R>` can improve the speed of programs that make *small* and"] # [doc = " *repeated* read calls to the same file or network socket. It does not"] # [doc = " help when reading very large amounts at once, or reading just one or a few"] # [doc = " times. It also provides no advantage when reading from a source that is"] # [doc = " already in memory, like a <code>[Vec]\\<u8></code>."] # [doc = ""] # [doc = " When the `BufReader<R>` is dropped, the contents of its buffer will be"] # [doc = " discarded. Creating multiple instances of a `BufReader<R>` on the same"] # [doc = " stream can cause data loss. Reading from the underlying reader after"] # [doc = " unwrapping the `BufReader<R>` with [`BufReader::into_inner`] can also cause"] # [doc = " data loss."] # [doc = ""] # [doc = " [`TcpStream::read`]: crate::net::TcpStream::read"] # [doc = " [`TcpStream`]: crate::net::TcpStream"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::io::prelude::*;"] # [doc = " use std::io::BufReader;"] # [doc = " use std::fs::File;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let f = File::open(\"log.txt\")?;"] # [doc = "     let mut reader = BufReader::new(f);"] # [doc = ""] # [doc = "     let mut line = String::new();"] # [doc = "     let len = reader.read_line(&mut line)?;"] # [doc = "     println!(\"First line is {len} bytes long\");"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct BufReader < R : ? Sized > { buf : Buffer , inner : R , }}}
mkitem!{mkimpl!{impl < R : Read > BufReader < R > { # [doc = " Creates a new `BufReader<R>` with a default buffer capacity. The default is currently 8 KiB,"] # [doc = " but may change in the future."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::io::BufReader;"] # [doc = " use std::fs::File;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let f = File::open(\"log.txt\")?;"] # [doc = "     let reader = BufReader::new(f);"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] pub fn new (inner : R) -> BufReader < R > { BufReader :: with_capacity (DEFAULT_BUF_SIZE , inner) } pub (crate) fn try_new_buffer () -> io :: Result < Buffer > { Buffer :: try_with_capacity (DEFAULT_BUF_SIZE) } pub (crate) fn with_buffer (inner : R , buf : Buffer) -> Self { Self { inner , buf } } # [doc = " Creates a new `BufReader<R>` with the specified buffer capacity."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Creating a buffer with ten bytes of capacity:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::io::BufReader;"] # [doc = " use std::fs::File;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let f = File::open(\"log.txt\")?;"] # [doc = "     let reader = BufReader::with_capacity(10, f);"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] pub fn with_capacity (capacity : usize , inner : R) -> BufReader < R > { BufReader { inner , buf : Buffer :: with_capacity (capacity) } } }}}
mkitem!{mkimpl!{impl < R : Read + ? Sized > BufReader < R > { # [doc = " Attempt to look ahead `n` bytes."] # [doc = ""] # [doc = " `n` must be less than or equal to `capacity`."] # [doc = ""] # [doc = " The returned slice may be less than `n` bytes long if"] # [doc = " end of file is reached."] # [doc = ""] # [doc = " After calling this method, you may call [`consume`](BufRead::consume)"] # [doc = " with a value less than or equal to `n` to advance over some or all of"] # [doc = " the returned bytes."] # [doc = ""] # [doc = " ## Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " #![feature(bufreader_peek)]"] # [doc = " use std::io::{Read, BufReader};"] # [doc = ""] # [doc = " let mut bytes = &b\"oh, hello there\"[..];"] # [doc = " let mut rdr = BufReader::with_capacity(6, &mut bytes);"] # [doc = " assert_eq!(rdr.peek(2).unwrap(), b\"oh\");"] # [doc = " let mut buf = [0; 4];"] # [doc = " rdr.read(&mut buf[..]).unwrap();"] # [doc = " assert_eq!(&buf, b\"oh, \");"] # [doc = " assert_eq!(rdr.peek(5).unwrap(), b\"hello\");"] # [doc = " let mut s = String::new();"] # [doc = " rdr.read_to_string(&mut s).unwrap();"] # [doc = " assert_eq!(&s, \"hello there\");"] # [doc = " assert_eq!(rdr.peek(1).unwrap().len(), 0);"] # [doc = " ```"] # [unstable (feature = "bufreader_peek" , issue = "128405")] pub fn peek (& mut self , n : usize) -> io :: Result < & [u8] > { assert ! (n <= self . capacity ()) ; while n > self . buf . buffer () . len () { if self . buf . pos () > 0 { self . buf . backshift () ; } let new = self . buf . read_more (& mut self . inner) ? ; if new == 0 { return Ok (& self . buf . buffer () [..]) ; } debug_assert_eq ! (self . buf . pos () , 0) ; } Ok (& self . buf . buffer () [.. n]) } }}}
mkitem!{mkimpl!{impl < R : ? Sized > BufReader < R > { # [doc = " Gets a reference to the underlying reader."] # [doc = ""] # [doc = " It is inadvisable to directly read from the underlying reader."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::io::BufReader;"] # [doc = " use std::fs::File;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let f1 = File::open(\"log.txt\")?;"] # [doc = "     let reader = BufReader::new(f1);"] # [doc = ""] # [doc = "     let f2 = reader.get_ref();"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] pub fn get_ref (& self) -> & R { & self . inner } # [doc = " Gets a mutable reference to the underlying reader."] # [doc = ""] # [doc = " It is inadvisable to directly read from the underlying reader."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::io::BufReader;"] # [doc = " use std::fs::File;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let f1 = File::open(\"log.txt\")?;"] # [doc = "     let mut reader = BufReader::new(f1);"] # [doc = ""] # [doc = "     let f2 = reader.get_mut();"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] pub fn get_mut (& mut self) -> & mut R { & mut self . inner } # [doc = " Returns a reference to the internally buffered data."] # [doc = ""] # [doc = " Unlike [`fill_buf`], this will not attempt to fill the buffer if it is empty."] # [doc = ""] # [doc = " [`fill_buf`]: BufRead::fill_buf"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::io::{BufReader, BufRead};"] # [doc = " use std::fs::File;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let f = File::open(\"log.txt\")?;"] # [doc = "     let mut reader = BufReader::new(f);"] # [doc = "     assert!(reader.buffer().is_empty());"] # [doc = ""] # [doc = "     if reader.fill_buf()?.len() > 0 {"] # [doc = "         assert!(!reader.buffer().is_empty());"] # [doc = "     }"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "bufreader_buffer" , since = "1.37.0")] pub fn buffer (& self) -> & [u8] { self . buf . buffer () } # [doc = " Returns the number of bytes the internal buffer can hold at once."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::io::{BufReader, BufRead};"] # [doc = " use std::fs::File;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let f = File::open(\"log.txt\")?;"] # [doc = "     let mut reader = BufReader::new(f);"] # [doc = ""] # [doc = "     let capacity = reader.capacity();"] # [doc = "     let buffer = reader.fill_buf()?;"] # [doc = "     assert!(buffer.len() <= capacity);"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "buffered_io_capacity" , since = "1.46.0")] pub fn capacity (& self) -> usize { self . buf . capacity () } # [doc = " Unwraps this `BufReader<R>`, returning the underlying reader."] # [doc = ""] # [doc = " Note that any leftover data in the internal buffer is lost. Therefore,"] # [doc = " a following read from the underlying reader may lead to data loss."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::io::BufReader;"] # [doc = " use std::fs::File;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let f1 = File::open(\"log.txt\")?;"] # [doc = "     let reader = BufReader::new(f1);"] # [doc = ""] # [doc = "     let f2 = reader.into_inner();"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] pub fn into_inner (self) -> R where R : Sized , { self . inner } # [doc = " Invalidates all data in the internal buffer."] # [inline] pub (in crate :: io) fn discard_buffer (& mut self) { self . buf . discard_buffer () } }}}
mkitem!{mkimpl!{# [cfg (test)] impl < R : ? Sized > BufReader < R > { # [allow (missing_docs)] pub fn initialized (& self) -> usize { self . buf . initialized () } }}}
mkitem!{mkimpl!{impl < R : ? Sized + Seek > BufReader < R > { # [doc = " Seeks relative to the current position. If the new position lies within the buffer,"] # [doc = " the buffer will not be flushed, allowing for more efficient seeks."] # [doc = " This method does not return the location of the underlying reader, so the caller"] # [doc = " must track this information themselves if it is required."] # [stable (feature = "bufreader_seek_relative" , since = "1.53.0")] pub fn seek_relative (& mut self , offset : i64) -> io :: Result < () > { let pos = self . buf . pos () as u64 ; if offset < 0 { if let Some (_) = pos . checked_sub ((- offset) as u64) { self . buf . unconsume ((- offset) as usize) ; return Ok (()) ; } } else if let Some (new_pos) = pos . checked_add (offset as u64) { if new_pos <= self . buf . filled () as u64 { self . buf . consume (offset as usize) ; return Ok (()) ; } } self . seek (SeekFrom :: Current (offset)) . map (drop) } }}}
mkitem!{mkimpl!{impl < R > SpecReadByte for BufReader < R > where Self : Read , { # [inline] fn spec_read_byte (& mut self) -> Option < io :: Result < u8 > > { let mut byte = 0 ; if self . buf . consume_with (1 , | claimed | byte = claimed [0]) { return Some (Ok (byte)) ; } uninlined_slow_read_byte (self) } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < R : ? Sized + Read > Read for BufReader < R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { if self . buf . pos () == self . buf . filled () && buf . len () >= self . capacity () { self . discard_buffer () ; return self . inner . read (buf) ; } let mut rem = self . fill_buf () ? ; let nread = rem . read (buf) ? ; self . consume (nread) ; Ok (nread) } fn read_buf (& mut self , mut cursor : BorrowedCursor < '_ >) -> io :: Result < () > { if self . buf . pos () == self . buf . filled () && cursor . capacity () >= self . capacity () { self . discard_buffer () ; return self . inner . read_buf (cursor) ; } let prev = cursor . written () ; let mut rem = self . fill_buf () ? ; rem . read_buf (cursor . reborrow ()) ? ; self . consume (cursor . written () - prev) ; Ok (()) } fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { if self . buf . consume_with (buf . len () , | claimed | buf . copy_from_slice (claimed)) { return Ok (()) ; } crate :: io :: default_read_exact (self , buf) } fn read_buf_exact (& mut self , mut cursor : BorrowedCursor < '_ >) -> io :: Result < () > { if self . buf . consume_with (cursor . capacity () , | claimed | cursor . append (claimed)) { return Ok (()) ; } crate :: io :: default_read_buf_exact (self , cursor) } fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { let total_len = bufs . iter () . map (| b | b . len ()) . sum :: < usize > () ; if self . buf . pos () == self . buf . filled () && total_len >= self . capacity () { self . discard_buffer () ; return self . inner . read_vectored (bufs) ; } let mut rem = self . fill_buf () ? ; let nread = rem . read_vectored (bufs) ? ; self . consume (nread) ; Ok (nread) } fn is_read_vectored (& self) -> bool { self . inner . is_read_vectored () } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { let inner_buf = self . buffer () ; buf . try_reserve (inner_buf . len ()) ? ; buf . extend_from_slice (inner_buf) ; let nread = inner_buf . len () ; self . discard_buffer () ; Ok (nread + self . inner . read_to_end (buf) ?) } fn read_to_string (& mut self , buf : & mut String) -> io :: Result < usize > { if buf . is_empty () { unsafe { crate :: io :: append_to_string (buf , | b | self . read_to_end (b)) } } else { let mut bytes = Vec :: new () ; self . read_to_end (& mut bytes) ? ; let string = crate :: str :: from_utf8 (& bytes) . map_err (| _ | io :: Error :: INVALID_UTF8) ? ; * buf += string ; Ok (string . len ()) } } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < R : ? Sized + Read > BufRead for BufReader < R > { fn fill_buf (& mut self) -> io :: Result < & [u8] > { self . buf . fill_buf (& mut self . inner) } fn consume (& mut self , amt : usize) { self . buf . consume (amt) } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < R > fmt :: Debug for BufReader < R > where R : ? Sized + fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("BufReader") . field ("reader" , & & self . inner) . field ("buffer" , & format_args ! ("{}/{}" , self . buf . filled () - self . buf . pos () , self . capacity ()) ,) . finish () } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < R : ? Sized + Seek > Seek for BufReader < R > { # [doc = " Seek to an offset, in bytes, in the underlying reader."] # [doc = ""] # [doc = " The position used for seeking with <code>[SeekFrom::Current]\\(_)</code> is the"] # [doc = " position the underlying reader would be at if the `BufReader<R>` had no"] # [doc = " internal buffer."] # [doc = ""] # [doc = " Seeking always discards the internal buffer, even if the seek position"] # [doc = " would otherwise fall within it. This guarantees that calling"] # [doc = " [`BufReader::into_inner()`] immediately after a seek yields the underlying reader"] # [doc = " at the same position."] # [doc = ""] # [doc = " To seek without discarding the internal buffer, use [`BufReader::seek_relative`]."] # [doc = ""] # [doc = " See [`std::io::Seek`] for more details."] # [doc = ""] # [doc = " Note: In the edge case where you're seeking with <code>[SeekFrom::Current]\\(n)</code>"] # [doc = " where `n` minus the internal buffer length overflows an `i64`, two"] # [doc = " seeks will be performed instead of one. If the second seek returns"] # [doc = " [`Err`], the underlying reader will be left at the same position it would"] # [doc = " have if you called `seek` with <code>[SeekFrom::Current]\\(0)</code>."] # [doc = ""] # [doc = " [`std::io::Seek`]: Seek"] fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { let result : u64 ; if let SeekFrom :: Current (n) = pos { let remainder = (self . buf . filled () - self . buf . pos ()) as i64 ; if let Some (offset) = n . checked_sub (remainder) { result = self . inner . seek (SeekFrom :: Current (offset)) ? ; } else { self . inner . seek (SeekFrom :: Current (- remainder)) ? ; self . discard_buffer () ; result = self . inner . seek (SeekFrom :: Current (n)) ? ; } } else { result = self . inner . seek (pos) ? ; } self . discard_buffer () ; Ok (result) } # [doc = " Returns the current seek position from the start of the stream."] # [doc = ""] # [doc = " The value returned is equivalent to `self.seek(SeekFrom::Current(0))`"] # [doc = " but does not flush the internal buffer. Due to this optimization the"] # [doc = " function does not guarantee that calling `.into_inner()` immediately"] # [doc = " afterwards will yield the underlying reader at the same position. Use"] # [doc = " [`BufReader::seek`] instead if you require that guarantee."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function will panic if the position of the inner reader is smaller"] # [doc = " than the amount of buffered data. That can happen if the inner reader"] # [doc = " has an incorrect implementation of [`Seek::stream_position`], or if the"] # [doc = " position has gone out of sync due to calling [`Seek::seek`] directly on"] # [doc = " the underlying reader."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::{"] # [doc = "     io::{self, BufRead, BufReader, Seek},"] # [doc = "     fs::File,"] # [doc = " };"] # [doc = ""] # [doc = " fn main() -> io::Result<()> {"] # [doc = "     let mut f = BufReader::new(File::open(\"foo.txt\")?);"] # [doc = ""] # [doc = "     let before = f.stream_position()?;"] # [doc = "     f.read_line(&mut String::new())?;"] # [doc = "     let after = f.stream_position()?;"] # [doc = ""] # [doc = "     println!(\"The first line was {} bytes long\", after - before);"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] fn stream_position (& mut self) -> io :: Result < u64 > { let remainder = (self . buf . filled () - self . buf . pos ()) as u64 ; self . inner . stream_position () . map (| pos | { pos . checked_sub (remainder) . expect ("overflow when subtracting remaining buffer size from inner stream position" ,) }) } # [doc = " Seeks relative to the current position."] # [doc = ""] # [doc = " If the new position lies within the buffer, the buffer will not be"] # [doc = " flushed, allowing for more efficient seeks. This method does not return"] # [doc = " the location of the underlying reader, so the caller must track this"] # [doc = " information themselves if it is required."] fn seek_relative (& mut self , offset : i64) -> io :: Result < () > { self . seek_relative (offset) } }}}
mkitem!{mkimpl!{impl < T : ? Sized > SizeHint for BufReader < T > { # [inline] fn lower_bound (& self) -> usize { SizeHint :: lower_bound (self . get_ref ()) + self . buffer () . len () } # [inline] fn upper_bound (& self) -> Option < usize > { SizeHint :: upper_bound (self . get_ref ()) . and_then (| up | self . buffer () . len () . checked_add (up)) } }}}