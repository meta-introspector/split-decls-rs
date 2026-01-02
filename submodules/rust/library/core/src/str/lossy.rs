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
mkuse!{use super :: from_utf8_unchecked ;}
mkuse!{use super :: validations :: utf8_char_width ;}
mkuse!{use crate :: fmt ;}
mkuse!{use crate :: fmt :: { Formatter , Write } ;}
mkuse!{use crate :: iter :: FusedIterator ;}
mkitem!{mkimpl!{impl [u8] { # [doc = " Creates an iterator over the contiguous valid UTF-8 ranges of this"] # [doc = " slice, and the non-UTF-8 fragments in between."] # [doc = ""] # [doc = " See the [`Utf8Chunk`] type for documentation of the items yielded by this iterator."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " This function formats arbitrary but mostly-UTF-8 bytes into Rust source"] # [doc = " code in the form of a C-string literal (`c\"...\"`)."] # [doc = ""] # [doc = " ```"] # [doc = " use std::fmt::Write as _;"] # [doc = ""] # [doc = " pub fn cstr_literal(bytes: &[u8]) -> String {"] # [doc = "     let mut repr = String::new();"] # [doc = "     repr.push_str(\"c\\\"\");"] # [doc = "     for chunk in bytes.utf8_chunks() {"] # [doc = "         for ch in chunk.valid().chars() {"] # [doc = "             // Escapes \\0, \\t, \\r, \\n, \\\\, \\', \\\", and uses \\u{...} for non-printable characters."] # [doc = "             write!(repr, \"{}\", ch.escape_debug()).unwrap();"] # [doc = "         }"] # [doc = "         for byte in chunk.invalid() {"] # [doc = "             write!(repr, \"\\\\x{:02X}\", byte).unwrap();"] # [doc = "         }"] # [doc = "     }"] # [doc = "     repr.push('\"');"] # [doc = "     repr"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let lit = cstr_literal(b\"\\xferris the \\xf0\\x9f\\xa6\\x80\\x07\");"] # [doc = "     let expected = stringify!(c\"\\xFErris the 🦀\\u{7}\");"] # [doc = "     assert_eq!(lit, expected);"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "utf8_chunks" , since = "1.79.0")] pub fn utf8_chunks (& self) -> Utf8Chunks < '_ > { Utf8Chunks { source : self } } }}}
mkitem!{mkstruct!{# [doc = " An item returned by the [`Utf8Chunks`] iterator."] # [doc = ""] # [doc = " A `Utf8Chunk` stores a sequence of [`u8`] up to the first broken character"] # [doc = " when decoding a UTF-8 string."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " // An invalid UTF-8 string"] # [doc = " let bytes = b\"foo\\xF1\\x80bar\";"] # [doc = ""] # [doc = " // Decode the first `Utf8Chunk`"] # [doc = " let chunk = bytes.utf8_chunks().next().unwrap();"] # [doc = ""] # [doc = " // The first three characters are valid UTF-8"] # [doc = " assert_eq!(\"foo\", chunk.valid());"] # [doc = ""] # [doc = " // The fourth character is broken"] # [doc = " assert_eq!(b\"\\xF1\\x80\", chunk.invalid());"] # [doc = " ```"] # [stable (feature = "utf8_chunks" , since = "1.79.0")] # [derive (Clone , Debug , PartialEq , Eq)] pub struct Utf8Chunk < 'a > { valid : & 'a str , invalid : & 'a [u8] , }}}
mkitem!{mkimpl!{impl < 'a > Utf8Chunk < 'a > { # [doc = " Returns the next validated UTF-8 substring."] # [doc = ""] # [doc = " This substring can be empty at the start of the string or between"] # [doc = " broken UTF-8 characters."] # [must_use] # [stable (feature = "utf8_chunks" , since = "1.79.0")] pub fn valid (& self) -> & 'a str { self . valid } # [doc = " Returns the invalid sequence that caused a failure."] # [doc = ""] # [doc = " The returned slice will have a maximum length of 3 and starts after the"] # [doc = " substring given by [`valid`]. Decoding will resume after this sequence."] # [doc = ""] # [doc = " If empty, this is the last chunk in the string. If non-empty, an"] # [doc = " unexpected byte was encountered or the end of the input was reached"] # [doc = " unexpectedly."] # [doc = ""] # [doc = " Lossy decoding would replace this sequence with [`U+FFFD REPLACEMENT"] # [doc = " CHARACTER`]."] # [doc = ""] # [doc = " [`valid`]: Self::valid"] # [doc = " [`U+FFFD REPLACEMENT CHARACTER`]: crate::char::REPLACEMENT_CHARACTER"] # [must_use] # [stable (feature = "utf8_chunks" , since = "1.79.0")] pub fn invalid (& self) -> & 'a [u8] { self . invalid } }}}
mkitem!{mkstruct!{# [must_use] # [unstable (feature = "str_internals" , issue = "none")] pub struct Debug < 'a > (& 'a [u8]) ;}}
mkitem!{mkimpl!{# [unstable (feature = "str_internals" , issue = "none")] impl fmt :: Debug for Debug < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . write_char ('"') ? ; for chunk in self . 0 . utf8_chunks () { { let valid = chunk . valid () ; let mut from = 0 ; for (i , c) in valid . char_indices () { let esc = c . escape_debug () ; if esc . len () != 1 { f . write_str (& valid [from .. i]) ? ; for c in esc { f . write_char (c) ? ; } from = i + c . len_utf8 () ; } } f . write_str (& valid [from ..]) ? ; } for & b in chunk . invalid () { write ! (f , "\\x{:02X}" , b) ? ; } } f . write_char ('"') } }}}
mkitem!{mkstruct!{# [doc = " An iterator used to decode a slice of mostly UTF-8 bytes to string slices"] # [doc = " ([`&str`]) and byte slices ([`&[u8]`][byteslice])."] # [doc = ""] # [doc = " This struct is created by the [`utf8_chunks`] method on bytes slices."] # [doc = " If you want a simple conversion from UTF-8 byte slices to string slices,"] # [doc = " [`from_utf8`] is easier to use."] # [doc = ""] # [doc = " See the [`Utf8Chunk`] type for documentation of the items yielded by this iterator."] # [doc = ""] # [doc = " [byteslice]: slice"] # [doc = " [`utf8_chunks`]: slice::utf8_chunks"] # [doc = " [`from_utf8`]: super::from_utf8"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " This can be used to create functionality similar to"] # [doc = " [`String::from_utf8_lossy`] without allocating heap memory:"] # [doc = ""] # [doc = " ```"] # [doc = " fn from_utf8_lossy<F>(input: &[u8], mut push: F) where F: FnMut(&str) {"] # [doc = "     for chunk in input.utf8_chunks() {"] # [doc = "         push(chunk.valid());"] # [doc = ""] # [doc = "         if !chunk.invalid().is_empty() {"] # [doc = "             push(\"\\u{FFFD}\");"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [`String::from_utf8_lossy`]: ../../std/string/struct.String.html#method.from_utf8_lossy"] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "utf8_chunks" , since = "1.79.0")] # [derive (Clone)] pub struct Utf8Chunks < 'a > { source : & 'a [u8] , }}}
mkitem!{mkimpl!{impl < 'a > Utf8Chunks < 'a > { # [doc (hidden)] # [unstable (feature = "str_internals" , issue = "none")] pub fn debug (& self) -> Debug < '_ > { Debug (self . source) } }}}
mkitem!{mkimpl!{# [stable (feature = "utf8_chunks" , since = "1.79.0")] impl < 'a > Iterator for Utf8Chunks < 'a > { type Item = Utf8Chunk < 'a > ; fn next (& mut self) -> Option < Utf8Chunk < 'a > > { if self . source . is_empty () { return None ; } const TAG_CONT_U8 : u8 = 128 ; fn safe_get (xs : & [u8] , i : usize) -> u8 { * xs . get (i) . unwrap_or (& 0) } let mut i = 0 ; let mut valid_up_to = 0 ; while i < self . source . len () { let byte = unsafe { * self . source . get_unchecked (i) } ; i += 1 ; if byte < 128 { } else { let w = utf8_char_width (byte) ; match w { 2 => { if safe_get (self . source , i) & 192 != TAG_CONT_U8 { break ; } i += 1 ; } 3 => { match (byte , safe_get (self . source , i)) { (0xE0 , 0xA0 ..= 0xBF) => () , (0xE1 ..= 0xEC , 0x80 ..= 0xBF) => () , (0xED , 0x80 ..= 0x9F) => () , (0xEE ..= 0xEF , 0x80 ..= 0xBF) => () , _ => break , } i += 1 ; if safe_get (self . source , i) & 192 != TAG_CONT_U8 { break ; } i += 1 ; } 4 => { match (byte , safe_get (self . source , i)) { (0xF0 , 0x90 ..= 0xBF) => () , (0xF1 ..= 0xF3 , 0x80 ..= 0xBF) => () , (0xF4 , 0x80 ..= 0x8F) => () , _ => break , } i += 1 ; if safe_get (self . source , i) & 192 != TAG_CONT_U8 { break ; } i += 1 ; if safe_get (self . source , i) & 192 != TAG_CONT_U8 { break ; } i += 1 ; } _ => break , } } valid_up_to = i ; } let (inspected , remaining) = unsafe { self . source . split_at_unchecked (i) } ; self . source = remaining ; let (valid , invalid) = unsafe { inspected . split_at_unchecked (valid_up_to) } ; Some (Utf8Chunk { valid : unsafe { from_utf8_unchecked (valid) } , invalid , }) } }}}
mkitem!{mkimpl!{# [stable (feature = "utf8_chunks" , since = "1.79.0")] impl FusedIterator for Utf8Chunks < '_ > { }}}
mkitem!{mkimpl!{# [stable (feature = "utf8_chunks" , since = "1.79.0")] impl fmt :: Debug for Utf8Chunks < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Utf8Chunks") . field ("source" , & self . debug ()) . finish () } }}}