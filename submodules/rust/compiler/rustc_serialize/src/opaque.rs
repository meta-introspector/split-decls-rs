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
mkuse!{use std :: fs :: File ;}
mkuse!{use std :: io :: { self , Write } ;}
mkuse!{use std :: marker :: PhantomData ;}
mkuse!{use std :: ops :: Range ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use crate :: int_overflow :: DebugStrictAdd ;}
mkuse!{use crate :: leb128 ;}
mkuse!{use crate :: serialize :: { Decodable , Decoder , Encodable , Encoder } ;}
mkmod!{mem_encoder, { 
                getname!(mem_encoder);
                getsrc!(mem_encoder);
                getpath!(mem_encoder);
                get_deps!(mem_encoder);
                get_crates!(mem_encoder);
                mkinclude!(mem_encoder);
                 
            }}
mkitem!{pub type FileEncodeResult = Result < usize , (PathBuf , io :: Error) > ;}
mkitem!{pub const MAGIC_END_BYTES : & [u8] = b"rust-end-file" ;}
mkitem!{# [doc = " The size of the buffer in `FileEncoder`."] const BUF_SIZE : usize = 64 * 1024 ;}
mkitem!{mkstruct!{# [doc = " `FileEncoder` encodes data to file via fixed-size buffer."] # [doc = ""] # [doc = " There used to be a `MemEncoder` type that encoded all the data into a"] # [doc = " `Vec`. `FileEncoder` is better because its memory use is determined by the"] # [doc = " size of the buffer, rather than the full length of the encoded data, and"] # [doc = " because it doesn't need to reallocate memory along the way."] pub struct FileEncoder { buf : Box < [u8 ; BUF_SIZE] > , buffered : usize , flushed : usize , file : File , res : Result < () , io :: Error > , path : PathBuf , # [cfg (debug_assertions)] finished : bool , }}}
mkitem!{mkimpl!{impl FileEncoder { pub fn new < P : AsRef < Path > > (path : P) -> io :: Result < Self > { let file = File :: options () . read (true) . write (true) . create (true) . truncate (true) . open (& path) ? ; Ok (FileEncoder { buf : vec ! [0u8 ; BUF_SIZE] . into_boxed_slice () . try_into () . unwrap () , path : path . as_ref () . into () , buffered : 0 , flushed : 0 , file , res : Ok (()) , # [cfg (debug_assertions)] finished : false , }) } # [inline] pub fn position (& self) -> usize { self . flushed . debug_strict_add (self . buffered) } # [cold] # [inline (never)] pub fn flush (& mut self) { # [cfg (debug_assertions)] { self . finished = false ; } if self . res . is_ok () { self . res = self . file . write_all (& self . buf [.. self . buffered]) ; } self . flushed += self . buffered ; self . buffered = 0 ; } # [inline] pub fn file (& self) -> & File { & self . file } # [inline] pub fn path (& self) -> & Path { & self . path } # [inline] fn buffer_empty (& mut self) -> & mut [u8] { unsafe { self . buf . get_unchecked_mut (self . buffered ..) } } # [cold] # [inline (never)] fn write_all_cold_path (& mut self , buf : & [u8]) { self . flush () ; if let Some (dest) = self . buf . get_mut (.. buf . len ()) { dest . copy_from_slice (buf) ; self . buffered += buf . len () ; } else { if self . res . is_ok () { self . res = self . file . write_all (buf) ; } self . flushed += buf . len () ; } } # [inline] fn write_all (& mut self , buf : & [u8]) { # [cfg (debug_assertions)] { self . finished = false ; } if let Some (dest) = self . buffer_empty () . get_mut (.. buf . len ()) { dest . copy_from_slice (buf) ; self . buffered = self . buffered . debug_strict_add (buf . len ()) ; } else { self . write_all_cold_path (buf) ; } } # [doc = " Write up to `N` bytes to this encoder."] # [doc = ""] # [doc = " This function can be used to avoid the overhead of calling memcpy for writes that"] # [doc = " have runtime-variable length, but are small and have a small fixed upper bound."] # [doc = ""] # [doc = " This can be used to do in-place encoding as is done for leb128 (without this function"] # [doc = " we would need to write to a temporary buffer then memcpy into the encoder), and it can"] # [doc = " also be used to implement the varint scheme we use for rmeta and dep graph encoding,"] # [doc = " where we only want to encode the first few bytes of an integer. Copying in the whole"] # [doc = " integer then only advancing the encoder state for the few bytes we care about is more"] # [doc = " efficient than calling [`FileEncoder::write_all`], because variable-size copies are"] # [doc = " always lowered to `memcpy`, which has overhead and contains a lot of logic we can bypass"] # [doc = " with this function. Note that common architectures support fixed-size writes up to 8 bytes"] # [doc = " with one instruction, so while this does in some sense do wasted work, we come out ahead."] # [inline] pub fn write_with < const N : usize > (& mut self , visitor : impl FnOnce (& mut [u8 ; N]) -> usize) { # [cfg (debug_assertions)] { self . finished = false ; } let flush_threshold = const { BUF_SIZE . checked_sub (N) . unwrap () } ; if std :: intrinsics :: unlikely (self . buffered > flush_threshold) { self . flush () ; } let buf = unsafe { self . buffer_empty () . first_chunk_mut :: < N > () . unwrap_unchecked () } ; let written = visitor (buf) ; if written > N { Self :: panic_invalid_write :: < N > (written) ; } self . buffered = self . buffered . debug_strict_add (written) ; } # [cold] # [inline (never)] fn panic_invalid_write < const N : usize > (written : usize) { panic ! ("FileEncoder::write_with::<{N}> cannot be used to write {written} bytes") ; } # [doc = " Helper for calls where [`FileEncoder::write_with`] always writes the whole array."] # [inline] pub fn write_array < const N : usize > (& mut self , buf : [u8 ; N]) { self . write_with (| dest | { * dest = buf ; N }) } pub fn finish (& mut self) -> FileEncodeResult { self . write_all (MAGIC_END_BYTES) ; self . flush () ; # [cfg (debug_assertions)] { self . finished = true ; } match std :: mem :: replace (& mut self . res , Ok (())) { Ok (()) => Ok (self . position ()) , Err (e) => Err ((self . path . clone () , e)) , } } }}}
mkitem!{mkimpl!{# [cfg (debug_assertions)] impl Drop for FileEncoder { fn drop (& mut self) { if ! std :: thread :: panicking () { assert ! (self . finished) ; } } }}}
mkitem!{macro_rules ! write_leb128 { ($ this_fn : ident , $ int_ty : ty , $ write_leb_fn : ident) => { # [inline] fn $ this_fn (& mut self , v : $ int_ty) { self . write_with (| buf | leb128 ::$ write_leb_fn (buf , v)) } } ; }}
mkitem!{mkimpl!{impl Encoder for FileEncoder { write_leb128 ! (emit_usize , usize , write_usize_leb128) ; write_leb128 ! (emit_u128 , u128 , write_u128_leb128) ; write_leb128 ! (emit_u64 , u64 , write_u64_leb128) ; write_leb128 ! (emit_u32 , u32 , write_u32_leb128) ; # [inline] fn emit_u16 (& mut self , v : u16) { self . write_array (v . to_le_bytes ()) ; } # [inline] fn emit_u8 (& mut self , v : u8) { self . write_array ([v]) ; } write_leb128 ! (emit_isize , isize , write_isize_leb128) ; write_leb128 ! (emit_i128 , i128 , write_i128_leb128) ; write_leb128 ! (emit_i64 , i64 , write_i64_leb128) ; write_leb128 ! (emit_i32 , i32 , write_i32_leb128) ; # [inline] fn emit_i16 (& mut self , v : i16) { self . write_array (v . to_le_bytes ()) ; } # [inline] fn emit_raw_bytes (& mut self , s : & [u8]) { self . write_all (s) ; } }}}
mkitem!{mkstruct!{pub struct MemDecoder < 'a > { start : * const u8 , current : * const u8 , end : * const u8 , _marker : PhantomData < & 'a u8 > , }}}
mkitem!{mkimpl!{impl < 'a > MemDecoder < 'a > { # [inline] pub fn new (data : & 'a [u8] , position : usize) -> Result < MemDecoder < 'a > , () > { let data = data . strip_suffix (MAGIC_END_BYTES) . ok_or (()) ? ; let Range { start , end } = data . as_ptr_range () ; Ok (MemDecoder { start , current : data [position ..] . as_ptr () , end , _marker : PhantomData }) } # [inline] pub fn split_at (& self , position : usize) -> MemDecoder < 'a > { assert ! (position <= self . len ()) ; let current = unsafe { self . start . add (position) } ; MemDecoder { start : self . start , current , end : self . end , _marker : PhantomData } } # [inline] pub fn len (& self) -> usize { unsafe { self . end . offset_from_unsigned (self . start) } } # [inline] pub fn remaining (& self) -> usize { unsafe { self . end . offset_from_unsigned (self . current) } } # [cold] # [inline (never)] fn decoder_exhausted () -> ! { panic ! ("MemDecoder exhausted") } # [inline] pub fn read_array < const N : usize > (& mut self) -> [u8 ; N] { self . read_raw_bytes (N) . try_into () . unwrap () } # [doc = " While we could manually expose manipulation of the decoder position,"] # [doc = " all current users of that method would need to reset the position later,"] # [doc = " incurring the bounds check of set_position twice."] # [inline] pub fn with_position < F , T > (& mut self , pos : usize , func : F) -> T where F : Fn (& mut MemDecoder < 'a >) -> T , { struct SetOnDrop < 'a , 'guarded > { decoder : & 'guarded mut MemDecoder < 'a > , current : * const u8 , } impl Drop for SetOnDrop < '_ , '_ > { fn drop (& mut self) { self . decoder . current = self . current ; } } if pos >= self . len () { Self :: decoder_exhausted () ; } let previous = self . current ; unsafe { self . current = self . start . add (pos) ; } let guard = SetOnDrop { current : previous , decoder : self } ; func (guard . decoder) } }}}
mkitem!{macro_rules ! read_leb128 { ($ this_fn : ident , $ int_ty : ty , $ read_leb_fn : ident) => { # [inline] fn $ this_fn (& mut self) -> $ int_ty { leb128 ::$ read_leb_fn (self) } } ; }}
mkitem!{mkimpl!{impl < 'a > Decoder for MemDecoder < 'a > { read_leb128 ! (read_usize , usize , read_usize_leb128) ; read_leb128 ! (read_u128 , u128 , read_u128_leb128) ; read_leb128 ! (read_u64 , u64 , read_u64_leb128) ; read_leb128 ! (read_u32 , u32 , read_u32_leb128) ; # [inline] fn read_u16 (& mut self) -> u16 { u16 :: from_le_bytes (self . read_array ()) } # [inline] fn read_u8 (& mut self) -> u8 { if self . current == self . end { Self :: decoder_exhausted () ; } unsafe { let byte = * self . current ; self . current = self . current . add (1) ; byte } } read_leb128 ! (read_isize , isize , read_isize_leb128) ; read_leb128 ! (read_i128 , i128 , read_i128_leb128) ; read_leb128 ! (read_i64 , i64 , read_i64_leb128) ; read_leb128 ! (read_i32 , i32 , read_i32_leb128) ; # [inline] fn read_i16 (& mut self) -> i16 { i16 :: from_le_bytes (self . read_array ()) } # [inline] fn read_raw_bytes (& mut self , bytes : usize) -> & 'a [u8] { if bytes > self . remaining () { Self :: decoder_exhausted () ; } unsafe { let slice = std :: slice :: from_raw_parts (self . current , bytes) ; self . current = self . current . add (bytes) ; slice } } # [inline] fn peek_byte (& self) -> u8 { if self . current == self . end { Self :: decoder_exhausted () ; } unsafe { * self . current } } # [inline] fn position (& self) -> usize { unsafe { self . current . offset_from_unsigned (self . start) } } }}}
mkitem!{mkimpl!{impl Encodable < FileEncoder > for [u8] { fn encode (& self , e : & mut FileEncoder) { Encoder :: emit_usize (e , self . len ()) ; e . emit_raw_bytes (self) ; } }}}
mkitem!{mkimpl!{impl < 'a > Decodable < MemDecoder < 'a > > for Vec < u8 > { fn decode (d : & mut MemDecoder < 'a >) -> Self { let len = Decoder :: read_usize (d) ; d . read_raw_bytes (len) . to_owned () } }}}
mkitem!{mkstruct!{# [doc = " An integer that will always encode to 8 bytes."] pub struct IntEncodedWithFixedSize (pub u64) ;}}
mkitem!{mkimpl!{impl IntEncodedWithFixedSize { pub const ENCODED_SIZE : usize = 8 ; }}}
mkitem!{mkimpl!{impl Encodable < FileEncoder > for IntEncodedWithFixedSize { # [inline] fn encode (& self , e : & mut FileEncoder) { let start_pos = e . position () ; e . write_array (self . 0 . to_le_bytes ()) ; let end_pos = e . position () ; debug_assert_eq ! ((end_pos - start_pos) , IntEncodedWithFixedSize :: ENCODED_SIZE) ; } }}}
mkitem!{mkimpl!{impl < 'a > Decodable < MemDecoder < 'a > > for IntEncodedWithFixedSize { # [inline] fn decode (decoder : & mut MemDecoder < 'a >) -> IntEncodedWithFixedSize { let bytes = decoder . read_array :: < { IntEncodedWithFixedSize :: ENCODED_SIZE } > () ; IntEncodedWithFixedSize (u64 :: from_le_bytes (bytes)) } }}}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}