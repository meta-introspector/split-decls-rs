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
mkuse!{use crate :: cmp ;}
mkuse!{use crate :: io :: prelude :: * ;}
mkuse!{use crate :: io :: { self , BorrowedCursor , ErrorKind , IoSlice , IoSliceMut , SeekFrom } ;}
mkitem!{mkstruct!{#[doc = " A `Cursor` wraps an in-memory buffer and provides it with a"] #[doc = " [`Seek`] implementation."] #[doc = ""] #[doc = " `Cursor`s are used with in-memory buffers, anything implementing"] #[doc = " <code>[AsRef]<\\[u8]></code>, to allow them to implement [`Read`] and/or [`Write`],"] #[doc = " allowing these buffers to be used anywhere you might use a reader or writer"] #[doc = " that does actual I/O."] #[doc = ""] #[doc = " The standard library implements some I/O traits on various types which"] #[doc = " are commonly used as a buffer, like <code>Cursor<[Vec]\\<u8>></code> and"] #[doc = " <code>Cursor<[&\\[u8\\]][bytes]></code>."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " We may want to write bytes to a [`File`] in our production"] #[doc = " code, but use an in-memory buffer in our tests. We can do this with"] #[doc = " `Cursor`:"] #[doc = ""] #[doc = " [bytes]: crate::slice \"slice\""] #[doc = " [`File`]: crate::fs::File"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::io::prelude::*;"] #[doc = " use std::io::{self, SeekFrom};"] #[doc = " use std::fs::File;"] #[doc = ""] #[doc = " // a library function we've written"] #[doc = " fn write_ten_bytes_at_end<W: Write + Seek>(mut writer: W) -> io::Result<()> {"] #[doc = "     writer.seek(SeekFrom::End(-10))?;"] #[doc = ""] #[doc = "     for i in 0..10 {"] #[doc = "         writer.write(&[i])?;"] #[doc = "     }"] #[doc = ""] #[doc = "     // all went well"] #[doc = "     Ok(())"] #[doc = " }"] #[doc = ""] #[doc = " # fn foo() -> io::Result<()> {"] #[doc = " // Here's some code that uses this library function."] #[doc = " //"] #[doc = " // We might want to use a BufReader here for efficiency, but let's"] #[doc = " // keep this example focused."] #[doc = " let mut file = File::create(\"foo.txt\")?;"] #[doc = " // First, we need to allocate 10 bytes to be able to write into."] #[doc = " file.set_len(10)?;"] #[doc = ""] #[doc = " write_ten_bytes_at_end(&mut file)?;"] #[doc = " # Ok(())"] #[doc = " # }"] #[doc = ""] #[doc = " // now let's write a test"] #[doc = " #[test]"] #[doc = " fn test_writes_bytes() {"] #[doc = "     // setting up a real File is much slower than an in-memory buffer,"] #[doc = "     // let's use a cursor instead"] #[doc = "     use std::io::Cursor;"] #[doc = "     let mut buff = Cursor::new(vec![0; 15]);"] #[doc = ""] #[doc = "     write_ten_bytes_at_end(&mut buff).unwrap();"] #[doc = ""] #[doc = "     assert_eq!(&buff.get_ref()[5..15], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);"] #[doc = " }"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] #[derive (Debug , Default , Eq , PartialEq)] pub struct Cursor < T > { inner : T , pos : u64 , }}}
mkitem!{mkimpl!{impl < T > Cursor < T > { #[doc = " Creates a new cursor wrapping the provided underlying in-memory buffer."] #[doc = ""] #[doc = " Cursor initial position is `0` even if underlying buffer (e.g., [`Vec`])"] #[doc = " is not empty. So writing to cursor starts with overwriting [`Vec`]"] #[doc = " content, not with appending to it."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::io::Cursor;"] #[doc = ""] #[doc = " let buff = Cursor::new(Vec::new());"] #[doc = " # fn force_inference(_: &Cursor<Vec<u8>>) {}"] #[doc = " # force_inference(&buff);"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] #[rustc_const_stable (feature = "const_io_structs" , since = "1.79.0")] pub const fn new (inner : T) -> Cursor < T > { Cursor { pos : 0 , inner } } #[doc = " Consumes this cursor, returning the underlying value."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::io::Cursor;"] #[doc = ""] #[doc = " let buff = Cursor::new(Vec::new());"] #[doc = " # fn force_inference(_: &Cursor<Vec<u8>>) {}"] #[doc = " # force_inference(&buff);"] #[doc = ""] #[doc = " let vec = buff.into_inner();"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] pub fn into_inner (self) -> T { self . inner } #[doc = " Gets a reference to the underlying value in this cursor."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::io::Cursor;"] #[doc = ""] #[doc = " let buff = Cursor::new(Vec::new());"] #[doc = " # fn force_inference(_: &Cursor<Vec<u8>>) {}"] #[doc = " # force_inference(&buff);"] #[doc = ""] #[doc = " let reference = buff.get_ref();"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] #[rustc_const_stable (feature = "const_io_structs" , since = "1.79.0")] pub const fn get_ref (& self) -> & T { & self . inner } #[doc = " Gets a mutable reference to the underlying value in this cursor."] #[doc = ""] #[doc = " Care should be taken to avoid modifying the internal I/O state of the"] #[doc = " underlying value as it may corrupt this cursor's position."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::io::Cursor;"] #[doc = ""] #[doc = " let mut buff = Cursor::new(Vec::new());"] #[doc = " # fn force_inference(_: &Cursor<Vec<u8>>) {}"] #[doc = " # force_inference(&buff);"] #[doc = ""] #[doc = " let reference = buff.get_mut();"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] #[rustc_const_stable (feature = "const_mut_cursor" , since = "1.86.0")] pub const fn get_mut (& mut self) -> & mut T { & mut self . inner } #[doc = " Returns the current position of this cursor."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::io::Cursor;"] #[doc = " use std::io::prelude::*;"] #[doc = " use std::io::SeekFrom;"] #[doc = ""] #[doc = " let mut buff = Cursor::new(vec![1, 2, 3, 4, 5]);"] #[doc = ""] #[doc = " assert_eq!(buff.position(), 0);"] #[doc = ""] #[doc = " buff.seek(SeekFrom::Current(2)).unwrap();"] #[doc = " assert_eq!(buff.position(), 2);"] #[doc = ""] #[doc = " buff.seek(SeekFrom::Current(-1)).unwrap();"] #[doc = " assert_eq!(buff.position(), 1);"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] #[rustc_const_stable (feature = "const_io_structs" , since = "1.79.0")] pub const fn position (& self) -> u64 { self . pos } #[doc = " Sets the position of this cursor."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::io::Cursor;"] #[doc = ""] #[doc = " let mut buff = Cursor::new(vec![1, 2, 3, 4, 5]);"] #[doc = ""] #[doc = " assert_eq!(buff.position(), 0);"] #[doc = ""] #[doc = " buff.set_position(2);"] #[doc = " assert_eq!(buff.position(), 2);"] #[doc = ""] #[doc = " buff.set_position(4);"] #[doc = " assert_eq!(buff.position(), 4);"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] #[rustc_const_stable (feature = "const_mut_cursor" , since = "1.86.0")] pub const fn set_position (& mut self , pos : u64) { self . pos = pos ; } }}}
mkitem!{mkimpl!{impl < T > Cursor < T > where T : AsRef < [u8] > , { #[doc = " Splits the underlying slice at the cursor position and returns them."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(cursor_split)]"] #[doc = " use std::io::Cursor;"] #[doc = ""] #[doc = " let mut buff = Cursor::new(vec![1, 2, 3, 4, 5]);"] #[doc = ""] #[doc = " assert_eq!(buff.split(), ([].as_slice(), [1, 2, 3, 4, 5].as_slice()));"] #[doc = ""] #[doc = " buff.set_position(2);"] #[doc = " assert_eq!(buff.split(), ([1, 2].as_slice(), [3, 4, 5].as_slice()));"] #[doc = ""] #[doc = " buff.set_position(6);"] #[doc = " assert_eq!(buff.split(), ([1, 2, 3, 4, 5].as_slice(), [].as_slice()));"] #[doc = " ```"] #[unstable (feature = "cursor_split" , issue = "86369")] pub fn split (& self) -> (& [u8] , & [u8]) { let slice = self . inner . as_ref () ; let pos = self . pos . min (slice . len () as u64) ; slice . split_at (pos as usize) } }}}
mkitem!{mkimpl!{impl < T > Cursor < T > where T : AsMut < [u8] > , { #[doc = " Splits the underlying slice at the cursor position and returns them"] #[doc = " mutably."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(cursor_split)]"] #[doc = " use std::io::Cursor;"] #[doc = ""] #[doc = " let mut buff = Cursor::new(vec![1, 2, 3, 4, 5]);"] #[doc = ""] #[doc = " assert_eq!(buff.split_mut(), ([].as_mut_slice(), [1, 2, 3, 4, 5].as_mut_slice()));"] #[doc = ""] #[doc = " buff.set_position(2);"] #[doc = " assert_eq!(buff.split_mut(), ([1, 2].as_mut_slice(), [3, 4, 5].as_mut_slice()));"] #[doc = ""] #[doc = " buff.set_position(6);"] #[doc = " assert_eq!(buff.split_mut(), ([1, 2, 3, 4, 5].as_mut_slice(), [].as_mut_slice()));"] #[doc = " ```"] #[unstable (feature = "cursor_split" , issue = "86369")] pub fn split_mut (& mut self) -> (& mut [u8] , & mut [u8]) { let slice = self . inner . as_mut () ; let pos = self . pos . min (slice . len () as u64) ; slice . split_at_mut (pos as usize) } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < T > Clone for Cursor < T > where T : Clone , { #[inline] fn clone (& self) -> Self { Cursor { inner : self . inner . clone () , pos : self . pos } } #[inline] fn clone_from (& mut self , other : & Self) { self . inner . clone_from (& other . inner) ; self . pos = other . pos ; } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < T > io :: Seek for Cursor < T > where T : AsRef < [u8] > , { fn seek (& mut self , style : SeekFrom) -> io :: Result < u64 > { let (base_pos , offset) = match style { SeekFrom :: Start (n) => { self . pos = n ; return Ok (n) ; } SeekFrom :: End (n) => (self . inner . as_ref () . len () as u64 , n) , SeekFrom :: Current (n) => (self . pos , n) , } ; match base_pos . checked_add_signed (offset) { Some (n) => { self . pos = n ; Ok (self . pos) } None => Err (io :: const_error ! (ErrorKind :: InvalidInput , "invalid seek to a negative or overflowing position" ,)) , } } fn stream_len (& mut self) -> io :: Result < u64 > { Ok (self . inner . as_ref () . len () as u64) } fn stream_position (& mut self) -> io :: Result < u64 > { Ok (self . pos) } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < T > Read for Cursor < T > where T : AsRef < [u8] > , { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let n = Read :: read (& mut Cursor :: split (self) . 1 , buf) ? ; self . pos += n as u64 ; Ok (n) } fn read_buf (& mut self , mut cursor : BorrowedCursor < '_ >) -> io :: Result < () > { let prev_written = cursor . written () ; Read :: read_buf (& mut Cursor :: split (self) . 1 , cursor . reborrow ()) ? ; self . pos += (cursor . written () - prev_written) as u64 ; Ok (()) } fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { let mut nread = 0 ; for buf in bufs { let n = self . read (buf) ? ; nread += n ; if n < buf . len () { break ; } } Ok (nread) } fn is_read_vectored (& self) -> bool { true } fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { let result = Read :: read_exact (& mut Cursor :: split (self) . 1 , buf) ; match result { Ok (_) => self . pos += buf . len () as u64 , Err (_) => self . pos = self . inner . as_ref () . len () as u64 , } result } fn read_buf_exact (& mut self , mut cursor : BorrowedCursor < '_ >) -> io :: Result < () > { let prev_written = cursor . written () ; let result = Read :: read_buf_exact (& mut Cursor :: split (self) . 1 , cursor . reborrow ()) ; self . pos += (cursor . written () - prev_written) as u64 ; result } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { let content = Cursor :: split (self) . 1 ; let len = content . len () ; buf . try_reserve (len) ? ; buf . extend_from_slice (content) ; self . pos += len as u64 ; Ok (len) } fn read_to_string (& mut self , buf : & mut String) -> io :: Result < usize > { let content = crate :: str :: from_utf8 (Cursor :: split (self) . 1) . map_err (| _ | io :: Error :: INVALID_UTF8) ? ; let len = content . len () ; buf . try_reserve (len) ? ; buf . push_str (content) ; self . pos += len as u64 ; Ok (len) } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < T > BufRead for Cursor < T > where T : AsRef < [u8] > , { fn fill_buf (& mut self) -> io :: Result < & [u8] > { Ok (Cursor :: split (self) . 1) } fn consume (& mut self , amt : usize) { self . pos += amt as u64 ; } }}}

macro_rules! slice_write_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function slice_write in module {}", module_path!());
    };
}

mkfn!{
    slice_write_introspect!();
    #[inline] fn slice_write (pos_mut : & mut u64 , slice : & mut [u8] , buf : & [u8]) -> io :: Result < usize > { let pos = cmp :: min (* pos_mut , slice . len () as u64) ; let amt = (& mut slice [(pos as usize) ..]) . write (buf) ? ; * pos_mut += amt as u64 ; Ok (amt) }
}

macro_rules! slice_write_vectored_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function slice_write_vectored in module {}", module_path!());
    };
}

mkfn!{
    slice_write_vectored_introspect!();
    #[inline] fn slice_write_vectored (pos_mut : & mut u64 , slice : & mut [u8] , bufs : & [IoSlice < '_ >] ,) -> io :: Result < usize > { let mut nwritten = 0 ; for buf in bufs { let n = slice_write (pos_mut , slice , buf) ? ; nwritten += n ; if n < buf . len () { break ; } } Ok (nwritten) }
}

macro_rules! slice_write_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function slice_write_all in module {}", module_path!());
    };
}

mkfn!{
    slice_write_all_introspect!();
    #[inline] fn slice_write_all (pos_mut : & mut u64 , slice : & mut [u8] , buf : & [u8]) -> io :: Result < () > { let n = slice_write (pos_mut , slice , buf) ? ; if n < buf . len () { Err (io :: Error :: WRITE_ALL_EOF) } else { Ok (()) } }
}

macro_rules! slice_write_all_vectored_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function slice_write_all_vectored in module {}", module_path!());
    };
}

mkfn!{
    slice_write_all_vectored_introspect!();
    #[inline] fn slice_write_all_vectored (pos_mut : & mut u64 , slice : & mut [u8] , bufs : & [IoSlice < '_ >] ,) -> io :: Result < () > { for buf in bufs { let n = slice_write (pos_mut , slice , buf) ? ; if n < buf . len () { return Err (io :: Error :: WRITE_ALL_EOF) ; } } Ok (()) }
}

macro_rules! reserve_and_pad_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function reserve_and_pad in module {}", module_path!());
    };
}

mkfn!{
    reserve_and_pad_introspect!();
    #[doc = " Reserves the required space, and pads the vec with 0s if necessary."] fn reserve_and_pad < A : Allocator > (pos_mut : & mut u64 , vec : & mut Vec < u8 , A > , buf_len : usize ,) -> io :: Result < usize > { let pos : usize = (* pos_mut) . try_into () . map_err (| _ | { io :: const_error ! (ErrorKind :: InvalidInput , "cursor position exceeds maximum possible vector length" ,) }) ? ; let desired_cap = pos . saturating_add (buf_len) ; if desired_cap > vec . capacity () { vec . reserve (desired_cap - vec . len ()) ; } if pos > vec . len () { let diff = pos - vec . len () ; let spare = vec . spare_capacity_mut () ; debug_assert ! (spare . len () >= diff) ; unsafe { spare . get_unchecked_mut (.. diff) . fill (core :: mem :: MaybeUninit :: new (0)) ; vec . set_len (pos) ; } } Ok (pos) }
}

macro_rules! vec_write_all_unchecked_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_write_all_unchecked in module {}", module_path!());
    };
}

mkfn!{
    vec_write_all_unchecked_introspect!();
    #[doc = " Writes the slice to the vec without allocating."] #[doc = ""] #[doc = " # Safety"] #[doc = ""] #[doc = " `vec` must have `buf.len()` spare capacity."] unsafe fn vec_write_all_unchecked < A > (pos : usize , vec : & mut Vec < u8 , A > , buf : & [u8]) -> usize where A : Allocator , { debug_assert ! (vec . capacity () >= pos + buf . len ()) ; unsafe { vec . as_mut_ptr () . add (pos) . copy_from (buf . as_ptr () , buf . len ()) } ; pos + buf . len () }
}

macro_rules! vec_write_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_write_all in module {}", module_path!());
    };
}

mkfn!{
    vec_write_all_introspect!();
    #[doc = " Resizing `write_all` implementation for [`Cursor`]."] #[doc = ""] #[doc = " Cursor is allowed to have a pre-allocated and initialised"] #[doc = " vector body, but with a position of 0. This means the [`Write`]"] #[doc = " will overwrite the contents of the vec."] #[doc = ""] #[doc = " This also allows for the vec body to be empty, but with a position of N."] #[doc = " This means that [`Write`] will pad the vec with 0 initially,"] #[doc = " before writing anything from that point"] fn vec_write_all < A > (pos_mut : & mut u64 , vec : & mut Vec < u8 , A > , buf : & [u8]) -> io :: Result < usize > where A : Allocator , { let buf_len = buf . len () ; let mut pos = reserve_and_pad (pos_mut , vec , buf_len) ? ; unsafe { pos = vec_write_all_unchecked (pos , vec , buf) ; if pos > vec . len () { vec . set_len (pos) ; } } ; * pos_mut += buf_len as u64 ; Ok (buf_len) }
}

macro_rules! vec_write_all_vectored_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_write_all_vectored in module {}", module_path!());
    };
}

mkfn!{
    vec_write_all_vectored_introspect!();
    #[doc = " Resizing `write_all_vectored` implementation for [`Cursor`]."] #[doc = ""] #[doc = " Cursor is allowed to have a pre-allocated and initialised"] #[doc = " vector body, but with a position of 0. This means the [`Write`]"] #[doc = " will overwrite the contents of the vec."] #[doc = ""] #[doc = " This also allows for the vec body to be empty, but with a position of N."] #[doc = " This means that [`Write`] will pad the vec with 0 initially,"] #[doc = " before writing anything from that point"] fn vec_write_all_vectored < A > (pos_mut : & mut u64 , vec : & mut Vec < u8 , A > , bufs : & [IoSlice < '_ >] ,) -> io :: Result < usize > where A : Allocator , { let buf_len = bufs . iter () . fold (0usize , | a , b | a . saturating_add (b . len ())) ; let mut pos = reserve_and_pad (pos_mut , vec , buf_len) ? ; unsafe { for buf in bufs { pos = vec_write_all_unchecked (pos , vec , buf) ; } if pos > vec . len () { vec . set_len (pos) ; } } * pos_mut += buf_len as u64 ; Ok (buf_len) }
}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl Write for Cursor < & mut [u8] > { #[inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { slice_write (& mut self . pos , self . inner , buf) } #[inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { slice_write_vectored (& mut self . pos , self . inner , bufs) } #[inline] fn is_write_vectored (& self) -> bool { true } #[inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { slice_write_all (& mut self . pos , self . inner , buf) } #[inline] fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { slice_write_all_vectored (& mut self . pos , self . inner , bufs) } #[inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{mkimpl!{#[stable (feature = "cursor_mut_vec" , since = "1.25.0")] impl < A > Write for Cursor < & mut Vec < u8 , A > > where A : Allocator , { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { vec_write_all (& mut self . pos , self . inner , buf) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { vec_write_all_vectored (& mut self . pos , self . inner , bufs) } #[inline] fn is_write_vectored (& self) -> bool { true } fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { vec_write_all (& mut self . pos , self . inner , buf) ? ; Ok (()) } fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { vec_write_all_vectored (& mut self . pos , self . inner , bufs) ? ; Ok (()) } #[inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < A > Write for Cursor < Vec < u8 , A > > where A : Allocator , { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { vec_write_all (& mut self . pos , & mut self . inner , buf) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { vec_write_all_vectored (& mut self . pos , & mut self . inner , bufs) } #[inline] fn is_write_vectored (& self) -> bool { true } fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { vec_write_all (& mut self . pos , & mut self . inner , buf) ? ; Ok (()) } fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { vec_write_all_vectored (& mut self . pos , & mut self . inner , bufs) ? ; Ok (()) } #[inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{mkimpl!{#[stable (feature = "cursor_box_slice" , since = "1.5.0")] impl < A > Write for Cursor < Box < [u8] , A > > where A : Allocator , { #[inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { slice_write (& mut self . pos , & mut self . inner , buf) } #[inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { slice_write_vectored (& mut self . pos , & mut self . inner , bufs) } #[inline] fn is_write_vectored (& self) -> bool { true } #[inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { slice_write_all (& mut self . pos , & mut self . inner , buf) } #[inline] fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { slice_write_all_vectored (& mut self . pos , & mut self . inner , bufs) } #[inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{mkimpl!{#[stable (feature = "cursor_array" , since = "1.61.0")] impl < const N : usize > Write for Cursor < [u8 ; N] > { #[inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { slice_write (& mut self . pos , & mut self . inner , buf) } #[inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { slice_write_vectored (& mut self . pos , & mut self . inner , bufs) } #[inline] fn is_write_vectored (& self) -> bool { true } #[inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { slice_write_all (& mut self . pos , & mut self . inner , buf) } #[inline] fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { slice_write_all_vectored (& mut self . pos , & mut self . inner , bufs) } #[inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}