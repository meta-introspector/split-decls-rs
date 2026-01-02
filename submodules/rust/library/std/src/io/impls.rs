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
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkuse!{use crate :: alloc :: Allocator ;}
mkuse!{use crate :: collections :: VecDeque ;}
mkuse!{use crate :: io :: { self , BorrowedCursor , BufRead , IoSlice , IoSliceMut , Read , Seek , SeekFrom , Write } ;}
mkuse!{use crate :: { cmp , fmt , mem , str } ;}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < R : Read + ? Sized > Read for & mut R { #[inline] fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { (* * self) . read (buf) } #[inline] fn read_buf (& mut self , cursor : BorrowedCursor < '_ >) -> io :: Result < () > { (* * self) . read_buf (cursor) } #[inline] fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { (* * self) . read_vectored (bufs) } #[inline] fn is_read_vectored (& self) -> bool { (* * self) . is_read_vectored () } #[inline] fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { (* * self) . read_to_end (buf) } #[inline] fn read_to_string (& mut self , buf : & mut String) -> io :: Result < usize > { (* * self) . read_to_string (buf) } #[inline] fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { (* * self) . read_exact (buf) } #[inline] fn read_buf_exact (& mut self , cursor : BorrowedCursor < '_ >) -> io :: Result < () > { (* * self) . read_buf_exact (cursor) } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < W : Write + ? Sized > Write for & mut W { #[inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { (* * self) . write (buf) } #[inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { (* * self) . write_vectored (bufs) } #[inline] fn is_write_vectored (& self) -> bool { (* * self) . is_write_vectored () } #[inline] fn flush (& mut self) -> io :: Result < () > { (* * self) . flush () } #[inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { (* * self) . write_all (buf) } #[inline] fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { (* * self) . write_all_vectored (bufs) } #[inline] fn write_fmt (& mut self , fmt : fmt :: Arguments < '_ >) -> io :: Result < () > { (* * self) . write_fmt (fmt) } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < S : Seek + ? Sized > Seek for & mut S { #[inline] fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { (* * self) . seek (pos) } #[inline] fn rewind (& mut self) -> io :: Result < () > { (* * self) . rewind () } #[inline] fn stream_len (& mut self) -> io :: Result < u64 > { (* * self) . stream_len () } #[inline] fn stream_position (& mut self) -> io :: Result < u64 > { (* * self) . stream_position () } #[inline] fn seek_relative (& mut self , offset : i64) -> io :: Result < () > { (* * self) . seek_relative (offset) } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < B : BufRead + ? Sized > BufRead for & mut B { #[inline] fn fill_buf (& mut self) -> io :: Result < & [u8] > { (* * self) . fill_buf () } #[inline] fn consume (& mut self , amt : usize) { (* * self) . consume (amt) } #[inline] fn has_data_left (& mut self) -> io :: Result < bool > { (* * self) . has_data_left () } #[inline] fn read_until (& mut self , byte : u8 , buf : & mut Vec < u8 >) -> io :: Result < usize > { (* * self) . read_until (byte , buf) } #[inline] fn skip_until (& mut self , byte : u8) -> io :: Result < usize > { (* * self) . skip_until (byte) } #[inline] fn read_line (& mut self , buf : & mut String) -> io :: Result < usize > { (* * self) . read_line (buf) } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < R : Read + ? Sized > Read for Box < R > { #[inline] fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { (* * self) . read (buf) } #[inline] fn read_buf (& mut self , cursor : BorrowedCursor < '_ >) -> io :: Result < () > { (* * self) . read_buf (cursor) } #[inline] fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { (* * self) . read_vectored (bufs) } #[inline] fn is_read_vectored (& self) -> bool { (* * self) . is_read_vectored () } #[inline] fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { (* * self) . read_to_end (buf) } #[inline] fn read_to_string (& mut self , buf : & mut String) -> io :: Result < usize > { (* * self) . read_to_string (buf) } #[inline] fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { (* * self) . read_exact (buf) } #[inline] fn read_buf_exact (& mut self , cursor : BorrowedCursor < '_ >) -> io :: Result < () > { (* * self) . read_buf_exact (cursor) } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < W : Write + ? Sized > Write for Box < W > { #[inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { (* * self) . write (buf) } #[inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { (* * self) . write_vectored (bufs) } #[inline] fn is_write_vectored (& self) -> bool { (* * self) . is_write_vectored () } #[inline] fn flush (& mut self) -> io :: Result < () > { (* * self) . flush () } #[inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { (* * self) . write_all (buf) } #[inline] fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { (* * self) . write_all_vectored (bufs) } #[inline] fn write_fmt (& mut self , fmt : fmt :: Arguments < '_ >) -> io :: Result < () > { (* * self) . write_fmt (fmt) } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < S : Seek + ? Sized > Seek for Box < S > { #[inline] fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { (* * self) . seek (pos) } #[inline] fn rewind (& mut self) -> io :: Result < () > { (* * self) . rewind () } #[inline] fn stream_len (& mut self) -> io :: Result < u64 > { (* * self) . stream_len () } #[inline] fn stream_position (& mut self) -> io :: Result < u64 > { (* * self) . stream_position () } #[inline] fn seek_relative (& mut self , offset : i64) -> io :: Result < () > { (* * self) . seek_relative (offset) } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < B : BufRead + ? Sized > BufRead for Box < B > { #[inline] fn fill_buf (& mut self) -> io :: Result < & [u8] > { (* * self) . fill_buf () } #[inline] fn consume (& mut self , amt : usize) { (* * self) . consume (amt) } #[inline] fn has_data_left (& mut self) -> io :: Result < bool > { (* * self) . has_data_left () } #[inline] fn read_until (& mut self , byte : u8 , buf : & mut Vec < u8 >) -> io :: Result < usize > { (* * self) . read_until (byte , buf) } #[inline] fn skip_until (& mut self , byte : u8) -> io :: Result < usize > { (* * self) . skip_until (byte) } #[inline] fn read_line (& mut self , buf : & mut String) -> io :: Result < usize > { (* * self) . read_line (buf) } }}}
mkitem!{mkimpl!{#[doc = " Read is implemented for `&[u8]` by copying from the slice."] #[doc = ""] #[doc = " Note that reading updates the slice to point to the yet unread part."] #[doc = " The slice will be empty when EOF is reached."] #[stable (feature = "rust1" , since = "1.0.0")] impl Read for & [u8] { #[inline] fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let amt = cmp :: min (buf . len () , self . len ()) ; let (a , b) = self . split_at (amt) ; if amt == 1 { buf [0] = a [0] ; } else { buf [.. amt] . copy_from_slice (a) ; } * self = b ; Ok (amt) } #[inline] fn read_buf (& mut self , mut cursor : BorrowedCursor < '_ >) -> io :: Result < () > { let amt = cmp :: min (cursor . capacity () , self . len ()) ; let (a , b) = self . split_at (amt) ; cursor . append (a) ; * self = b ; Ok (()) } #[inline] fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { let mut nread = 0 ; for buf in bufs { nread += self . read (buf) ? ; if self . is_empty () { break ; } } Ok (nread) } #[inline] fn is_read_vectored (& self) -> bool { true } #[inline] fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { if buf . len () > self . len () { * self = & self [self . len () ..] ; return Err (io :: Error :: READ_EXACT_EOF) ; } let (a , b) = self . split_at (buf . len ()) ; if buf . len () == 1 { buf [0] = a [0] ; } else { buf . copy_from_slice (a) ; } * self = b ; Ok (()) } #[inline] fn read_buf_exact (& mut self , mut cursor : BorrowedCursor < '_ >) -> io :: Result < () > { if cursor . capacity () > self . len () { cursor . append (* self) ; * self = & self [self . len () ..] ; return Err (io :: Error :: READ_EXACT_EOF) ; } let (a , b) = self . split_at (cursor . capacity ()) ; cursor . append (a) ; * self = b ; Ok (()) } #[inline] fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { let len = self . len () ; buf . try_reserve (len) ? ; buf . extend_from_slice (* self) ; * self = & self [len ..] ; Ok (len) } #[inline] fn read_to_string (& mut self , buf : & mut String) -> io :: Result < usize > { let content = str :: from_utf8 (self) . map_err (| _ | io :: Error :: INVALID_UTF8) ? ; let len = self . len () ; buf . try_reserve (len) ? ; buf . push_str (content) ; * self = & self [len ..] ; Ok (len) } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl BufRead for & [u8] { #[inline] fn fill_buf (& mut self) -> io :: Result < & [u8] > { Ok (* self) } #[inline] fn consume (& mut self , amt : usize) { * self = & self [amt ..] ; } }}}
mkitem!{mkimpl!{#[doc = " Write is implemented for `&mut [u8]` by copying into the slice, overwriting"] #[doc = " its data."] #[doc = ""] #[doc = " Note that writing updates the slice to point to the yet unwritten part."] #[doc = " The slice will be empty when it has been completely overwritten."] #[doc = ""] #[doc = " If the number of bytes to be written exceeds the size of the slice, write operations will"] #[doc = " return short writes: ultimately, `Ok(0)`; in this situation, `write_all` returns an error of"] #[doc = " kind `ErrorKind::WriteZero`."] #[stable (feature = "rust1" , since = "1.0.0")] impl Write for & mut [u8] { #[inline] fn write (& mut self , data : & [u8]) -> io :: Result < usize > { let amt = cmp :: min (data . len () , self . len ()) ; let (a , b) = mem :: take (self) . split_at_mut (amt) ; a . copy_from_slice (& data [.. amt]) ; * self = b ; Ok (amt) } #[inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { let mut nwritten = 0 ; for buf in bufs { nwritten += self . write (buf) ? ; if self . is_empty () { break ; } } Ok (nwritten) } #[inline] fn is_write_vectored (& self) -> bool { true } #[inline] fn write_all (& mut self , data : & [u8]) -> io :: Result < () > { if self . write (data) ? < data . len () { Err (io :: Error :: WRITE_ALL_EOF) } else { Ok (()) } } #[inline] fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { for buf in bufs { if self . write (buf) ? < buf . len () { return Err (io :: Error :: WRITE_ALL_EOF) ; } } Ok (()) } #[inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{mkimpl!{#[doc = " Write is implemented for `Vec<u8>` by appending to the vector."] #[doc = " The vector will grow as needed."] #[stable (feature = "rust1" , since = "1.0.0")] impl < A : Allocator > Write for Vec < u8 , A > { #[inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . extend_from_slice (buf) ; Ok (buf . len ()) } #[inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { let len = bufs . iter () . map (| b | b . len ()) . sum () ; self . reserve (len) ; for buf in bufs { self . extend_from_slice (buf) ; } Ok (len) } #[inline] fn is_write_vectored (& self) -> bool { true } #[inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { self . extend_from_slice (buf) ; Ok (()) } #[inline] fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { self . write_vectored (bufs) ? ; Ok (()) } #[inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{mkimpl!{#[doc = " Read is implemented for `VecDeque<u8>` by consuming bytes from the front of the `VecDeque`."] #[stable (feature = "vecdeque_read_write" , since = "1.63.0")] impl < A : Allocator > Read for VecDeque < u8 , A > { #[doc = " Fill `buf` with the contents of the \"front\" slice as returned by"] #[doc = " [`as_slices`][`VecDeque::as_slices`]. If the contained byte slices of the `VecDeque` are"] #[doc = " discontiguous, multiple calls to `read` will be needed to read the entire content."] #[inline] fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let (ref mut front , _) = self . as_slices () ; let n = Read :: read (front , buf) ? ; self . drain (.. n) ; Ok (n) } #[inline] fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { let (front , back) = self . as_slices () ; match buf . split_at_mut_checked (front . len ()) { None => buf . copy_from_slice (& front [.. buf . len ()]) , Some ((buf_front , buf_back)) => match back . split_at_checked (buf_back . len ()) { Some ((back , _)) => { buf_front . copy_from_slice (front) ; buf_back . copy_from_slice (back) ; } None => { self . clear () ; return Err (io :: Error :: READ_EXACT_EOF) ; } } , } self . drain (.. buf . len ()) ; Ok (()) } #[inline] fn read_buf (& mut self , cursor : BorrowedCursor < '_ >) -> io :: Result < () > { let (ref mut front , _) = self . as_slices () ; let n = cmp :: min (cursor . capacity () , front . len ()) ; Read :: read_buf (front , cursor) ? ; self . drain (.. n) ; Ok (()) } #[inline] fn read_buf_exact (& mut self , mut cursor : BorrowedCursor < '_ >) -> io :: Result < () > { let len = cursor . capacity () ; let (front , back) = self . as_slices () ; match front . split_at_checked (cursor . capacity ()) { Some ((front , _)) => cursor . append (front) , None => { cursor . append (front) ; match back . split_at_checked (cursor . capacity ()) { Some ((back , _)) => cursor . append (back) , None => { cursor . append (back) ; self . clear () ; return Err (io :: Error :: READ_EXACT_EOF) ; } } } } self . drain (.. len) ; Ok (()) } #[inline] fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { let len = self . len () ; buf . try_reserve (len) ? ; let (front , back) = self . as_slices () ; buf . extend_from_slice (front) ; buf . extend_from_slice (back) ; self . clear () ; Ok (len) } #[inline] fn read_to_string (& mut self , buf : & mut String) -> io :: Result < usize > { unsafe { io :: append_to_string (buf , | buf | self . read_to_end (buf)) } } }}}
mkitem!{mkimpl!{#[doc = " BufRead is implemented for `VecDeque<u8>` by reading bytes from the front of the `VecDeque`."] #[stable (feature = "vecdeque_buf_read" , since = "1.75.0")] impl < A : Allocator > BufRead for VecDeque < u8 , A > { #[doc = " Returns the contents of the \"front\" slice as returned by"] #[doc = " [`as_slices`][`VecDeque::as_slices`]. If the contained byte slices of the `VecDeque` are"] #[doc = " discontiguous, multiple calls to `fill_buf` will be needed to read the entire content."] #[inline] fn fill_buf (& mut self) -> io :: Result < & [u8] > { let (front , _) = self . as_slices () ; Ok (front) } #[inline] fn consume (& mut self , amt : usize) { self . drain (.. amt) ; } }}}
mkitem!{mkimpl!{#[doc = " Write is implemented for `VecDeque<u8>` by appending to the `VecDeque`, growing it as needed."] #[stable (feature = "vecdeque_read_write" , since = "1.63.0")] impl < A : Allocator > Write for VecDeque < u8 , A > { #[inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . extend (buf) ; Ok (buf . len ()) } #[inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { let len = bufs . iter () . map (| b | b . len ()) . sum () ; self . reserve (len) ; for buf in bufs { self . extend (& * * buf) ; } Ok (len) } #[inline] fn is_write_vectored (& self) -> bool { true } #[inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { self . extend (buf) ; Ok (()) } #[inline] fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { self . write_vectored (bufs) ? ; Ok (()) } #[inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{mkimpl!{#[unstable (feature = "read_buf" , issue = "78485")] impl < 'a > io :: Write for core :: io :: BorrowedCursor < 'a > { #[inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let amt = cmp :: min (buf . len () , self . capacity ()) ; self . append (& buf [.. amt]) ; Ok (amt) } #[inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { let mut nwritten = 0 ; for buf in bufs { let n = self . write (buf) ? ; nwritten += n ; if n < buf . len () { break ; } } Ok (nwritten) } #[inline] fn is_write_vectored (& self) -> bool { true } #[inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { if self . write (buf) ? < buf . len () { Err (io :: Error :: WRITE_ALL_EOF) } else { Ok (()) } } #[inline] fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { for buf in bufs { if self . write (buf) ? < buf . len () { return Err (io :: Error :: WRITE_ALL_EOF) ; } } Ok (()) } #[inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}