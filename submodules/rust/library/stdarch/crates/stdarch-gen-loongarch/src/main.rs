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
mkuse!{use std :: env ;}
mkuse!{use std :: fmt ;}
mkuse!{use std :: fs :: File ;}
mkuse!{use std :: io :: prelude :: * ;}
mkuse!{use std :: io :: { self , BufReader } ;}
mkuse!{use std :: path :: PathBuf ;}
mkitem!{mkstruct!{#[doc = " Complete lines of generated source."] #[doc = ""] #[doc = " This enables common generation tasks to be factored out without precluding basic"] #[doc = " context-specific formatting."] #[doc = ""] #[doc = " The convention in this generator is to prefix (not suffix) lines with a newline, so the"] #[doc = " implementation of `std::fmt::Display` behaves in the same way."] struct Lines { indent : usize , lines : Vec < String > , }}}
mkitem!{mkimpl!{impl Lines { fn single (line : String) -> Self { Self :: from (vec ! [line]) } }}}
mkitem!{mkimpl!{impl From < Vec < String > > for Lines { fn from (lines : Vec < String >) -> Self { Self { indent : 0 , lines } } }}}
mkitem!{mkimpl!{impl std :: fmt :: Display for Lines { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> fmt :: Result { for line in self . lines . iter () { write ! (f , "\n{:width$}{line}" , "" , width = self . indent) ? ; } Ok (()) } }}}
mkitem!{mkenum!{#[derive (Clone , Copy , PartialEq)] enum TargetFeature { Lsx , Lasx , }}}
mkitem!{mkimpl!{impl TargetFeature { fn new (ext : & str) -> TargetFeature { match ext { "lasx" => Self :: Lasx , _ => Self :: Lsx , } } #[doc = " A string for use with `#[target_feature(...)]`."] fn as_target_feature_arg (& self , ins : & str) -> String { let vec = match * self { Self :: Lsx => "lsx" , Self :: Lasx => "lasx" , } ; let frecipe = match ins { "lsx_vfrecipe_s" | "lsx_vfrecipe_d" | "lsx_vfrsqrte_s" | "lsx_vfrsqrte_d" | "lasx_xvfrecipe_s" | "lasx_xvfrecipe_d" | "lasx_xvfrsqrte_s" | "lasx_xvfrsqrte_d" => { ",frecipe" } _ => "" , } ; format ! ("{vec}{frecipe}") } fn attr (name : & str , value : impl fmt :: Display) -> String { format ! (r#"#[{name}(enable = "{value}")]"#) } #[doc = " Generate a target_feature attribute"] fn to_target_feature_attr (self , ins : & str) -> Lines { Lines :: single (Self :: attr ("target_feature" , self . as_target_feature_arg (ins) ,)) } fn bytes (& self) -> u8 { match * self { Self :: Lsx => 16 , Self :: Lasx => 32 , } } }}}

macro_rules! gen_spec_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function gen_spec in module {}", module_path!());
    };
}

mkfn!{
    gen_spec_introspect!();
    fn gen_spec (in_file : String , ext_name : & str) -> io :: Result < () > { let f = File :: open (in_file . clone ()) . unwrap_or_else (| _ | panic ! ("Failed to open {in_file}")) ; let f = BufReader :: new (f) ; let mut out = format ! (r#"// This code is automatically generated. DO NOT MODIFY.
// ```
// OUT_DIR=`pwd`/crates/stdarch-gen-loongarch cargo run -p stdarch-gen-loongarch -- {in_file}
// ```
"#) ; out . push ('\n') ; let mut asm_fmts = String :: new () ; let mut data_types = String :: new () ; let fn_pat = format ! ("__{ext_name}_") ; for line in f . lines () { let line = line . unwrap () ; if line . is_empty () { continue ; } if let Some (s) = line . find ("/* Assembly instruction format:") { let e = line . find ('.') . unwrap () ; asm_fmts = line . get (s + 31 .. e) . unwrap () . trim () . to_string () ; } else if let Some (s) = line . find ("/* Data types in instruction templates:") { let e = line . find ('.') . unwrap () ; data_types = line . get (s + 39 .. e) . unwrap () . trim () . to_string () ; } else if let Some (s) = line . find (fn_pat . as_str ()) { let e = line . find ('(') . unwrap () ; let name = line . get (s + 2 .. e) . unwrap () . trim () . to_string () ; out . push_str (& format ! ("/// {name}\n")) ; out . push_str (& format ! ("name = {name}\n")) ; out . push_str (& format ! ("asm-fmts = {asm_fmts}\n")) ; out . push_str (& format ! ("data-types = {data_types}\n")) ; out . push ('\n') ; } } let out_dir_path : PathBuf = PathBuf :: from (env :: var ("OUT_DIR") . unwrap ()) ; std :: fs :: create_dir_all (& out_dir_path) ? ; let mut f = File :: create (out_dir_path . join (format ! ("{ext_name}.spec"))) ? ; f . write_all (out . as_bytes ()) ? ; Ok (()) }
}

macro_rules! gen_bind_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function gen_bind in module {}", module_path!());
    };
}

mkfn!{
    gen_bind_introspect!();
    fn gen_bind (in_file : String , ext_name : & str) -> io :: Result < () > { let f = File :: open (in_file . clone ()) . unwrap_or_else (| _ | panic ! ("Failed to open {in_file}")) ; let f = BufReader :: new (f) ; let target : TargetFeature = TargetFeature :: new (ext_name) ; let mut para_num ; let mut current_name : Option < String > = None ; let mut asm_fmts : Vec < String > = Vec :: new () ; let mut link_function_str = String :: new () ; let mut function_str = String :: new () ; let mut out = String :: new () ; out . push_str (& format ! (r#"// This code is automatically generated. DO NOT MODIFY.
//
// Instead, modify `{in_file}` and run the following command to re-generate this file:
//
// ```
// OUT_DIR=`pwd`/crates/core_arch cargo run -p stdarch-gen-loongarch -- {in_file}
// ```

use crate::mem::transmute;
use super::types::*;
"#)) ; out . push_str (r#"
#[allow(improper_ctypes)]
unsafe extern "unadjusted" {
"# ,) ; for line in f . lines () { let line = line . unwrap () ; if line . is_empty () { continue ; } if let Some (name) = line . strip_prefix ("name = ") { current_name = Some (String :: from (name)) ; } else if line . starts_with ("asm-fmts = ") { asm_fmts = line [10 ..] . split (',') . map (| v | v . trim () . to_string ()) . collect () ; } else if line . starts_with ("data-types = ") { let current_name = current_name . clone () . unwrap () ; let data_types : Vec < & str > = line . get (12 ..) . unwrap () . split (',') . map (| e | e . trim ()) . collect () ; let in_t ; let out_t ; if data_types . len () == 2 { in_t = [data_types [1] , "NULL" , "NULL" , "NULL"] ; out_t = data_types [0] ; para_num = 1 ; } else if data_types . len () == 3 { in_t = [data_types [1] , data_types [2] , "NULL" , "NULL"] ; out_t = data_types [0] ; para_num = 2 ; } else if data_types . len () == 4 { in_t = [data_types [1] , data_types [2] , data_types [3] , "NULL"] ; out_t = data_types [0] ; para_num = 3 ; } else if data_types . len () == 5 { in_t = [data_types [1] , data_types [2] , data_types [3] , data_types [4]] ; out_t = data_types [0] ; para_num = 4 ; } else { panic ! ("DEBUG: line: {0} len: {1}" , line , data_types . len ()) ; } let (link_function , function) = gen_bind_body (& current_name , & asm_fmts , & in_t , out_t , para_num , target) ; link_function_str . push_str (& link_function) ; function_str . push_str (& function) ; } } out . push_str (& link_function_str) ; out . push_str ("}\n") ; out . push_str (& function_str) ; let out_path : PathBuf = PathBuf :: from (env :: var ("OUT_DIR") . unwrap_or ("crates/core_arch" . to_string ())) . join ("src") . join ("loongarch64") . join (ext_name) ; std :: fs :: create_dir_all (& out_path) ? ; let mut file = File :: create (out_path . join ("generated.rs")) ? ; file . write_all (out . as_bytes ()) ? ; Ok (()) }
}

macro_rules! gen_bind_body_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function gen_bind_body in module {}", module_path!());
    };
}

mkfn!{
    gen_bind_body_introspect!();
    fn gen_bind_body (current_name : & str , asm_fmts : & [String] , in_t : & [& str ; 4] , out_t : & str , para_num : i32 , target : TargetFeature ,) -> (String , String) { enum TypeKind { Vector , Intrinsic , } use TypeKind :: * ; let type_to_rst = | t : & str , s : bool , k : TypeKind | -> & str { match (t , s , k) { ("V16QI" , _ , Vector) => "__v16i8" , ("V16QI" , _ , Intrinsic) => "m128i" , ("V32QI" , _ , Vector) => "__v32i8" , ("V32QI" , _ , Intrinsic) => "m256i" , ("V8HI" , _ , Vector) => "__v8i16" , ("V8HI" , _ , Intrinsic) => "m128i" , ("V16HI" , _ , Vector) => "__v16i16" , ("V16HI" , _ , Intrinsic) => "m256i" , ("V4SI" , _ , Vector) => "__v4i32" , ("V4SI" , _ , Intrinsic) => "m128i" , ("V8SI" , _ , Vector) => "__v8i32" , ("V8SI" , _ , Intrinsic) => "m256i" , ("V2DI" , _ , Vector) => "__v2i64" , ("V2DI" , _ , Intrinsic) => "m128i" , ("V4DI" , _ , Vector) => "__v4i64" , ("V4DI" , _ , Intrinsic) => "m256i" , ("UV16QI" , _ , Vector) => "__v16u8" , ("UV16QI" , _ , Intrinsic) => "m128i" , ("UV32QI" , _ , Vector) => "__v32u8" , ("UV32QI" , _ , Intrinsic) => "m256i" , ("UV8HI" , _ , Vector) => "__v8u16" , ("UV8HI" , _ , Intrinsic) => "m128i" , ("UV16HI" , _ , Vector) => "__v16u16" , ("UV16HI" , _ , Intrinsic) => "m256i" , ("UV4SI" , _ , Vector) => "__v4u32" , ("UV4SI" , _ , Intrinsic) => "m128i" , ("UV8SI" , _ , Vector) => "__v8u32" , ("UV8SI" , _ , Intrinsic) => "m256i" , ("UV2DI" , _ , Vector) => "__v2u64" , ("UV2DI" , _ , Intrinsic) => "m128i" , ("UV4DI" , _ , Vector) => "__v4u64" , ("UV4DI" , _ , Intrinsic) => "m256i" , ("SI" , _ , _) => "i32" , ("DI" , _ , _) => "i64" , ("USI" , _ , _) => "u32" , ("UDI" , _ , _) => "u64" , ("V4SF" , _ , Vector) => "__v4f32" , ("V4SF" , _ , Intrinsic) => "m128" , ("V8SF" , _ , Vector) => "__v8f32" , ("V8SF" , _ , Intrinsic) => "m256" , ("V2DF" , _ , Vector) => "__v2f64" , ("V2DF" , _ , Intrinsic) => "m128d" , ("V4DF" , _ , Vector) => "__v4f64" , ("V4DF" , _ , Intrinsic) => "m256d" , ("UQI" , _ , _) => "u32" , ("QI" , _ , _) => "i32" , ("CVPOINTER" , false , _) => "*const i8" , ("CVPOINTER" , true , _) => "*mut i8" , ("HI" , _ , _) => "i32" , (_ , _ , _) => panic ! ("unknown type: {t}") , } } ; let is_mem = in_t . iter () . any (| s | s . contains ("POINTER")) ; let is_store = current_name . to_string () . contains ("vst") ; let link_function = { let fn_decl = { let fn_output = if out_t . to_lowercase () == "void" { String :: new () } else { format ! (" -> {}" , type_to_rst (out_t , is_store , Vector)) } ; let fn_inputs = match para_num { 1 => format ! ("(a: {})" , type_to_rst (in_t [0] , is_store , Vector)) , 2 => format ! ("(a: {}, b: {})" , type_to_rst (in_t [0] , is_store , Vector) , type_to_rst (in_t [1] , is_store , Vector)) , 3 => format ! ("(a: {}, b: {}, c: {})" , type_to_rst (in_t [0] , is_store , Vector) , type_to_rst (in_t [1] , is_store , Vector) , type_to_rst (in_t [2] , is_store , Vector)) , 4 => format ! ("(a: {}, b: {}, c: {}, d: {})" , type_to_rst (in_t [0] , is_store , Vector) , type_to_rst (in_t [1] , is_store , Vector) , type_to_rst (in_t [2] , is_store , Vector) , type_to_rst (in_t [3] , is_store , Vector)) , _ => panic ! ("unsupported parameter number") , } ; format ! ("fn __{current_name}{fn_inputs}{fn_output};") } ; let function = format ! (r#"    #[link_name = "llvm.loongarch.{}"]
    {fn_decl}
"# , current_name . replace ('_' , ".")) ; function } ; let type_to_imm = | t | -> i8 { match t { 'b' => 4 , 'h' => 3 , 'w' => 2 , 'd' => 1 , _ => panic ! ("unsupported type") , } } ; let mut rustc_legacy_const_generics = "" ; let fn_decl = { let fn_output = if out_t . to_lowercase () == "void" { String :: new () } else { format ! ("-> {} " , type_to_rst (out_t , is_store , Intrinsic)) } ; let mut fn_inputs = match para_num { 1 => format ! ("(a: {})" , type_to_rst (in_t [0] , is_store , Intrinsic)) , 2 => format ! ("(a: {}, b: {})" , type_to_rst (in_t [0] , is_store , Intrinsic) , type_to_rst (in_t [1] , is_store , Intrinsic)) , 3 => format ! ("(a: {}, b: {}, c: {})" , type_to_rst (in_t [0] , is_store , Intrinsic) , type_to_rst (in_t [1] , is_store , Intrinsic) , type_to_rst (in_t [2] , is_store , Intrinsic)) , 4 => format ! ("(a: {}, b: {}, c: {}, d: {})" , type_to_rst (in_t [0] , is_store , Intrinsic) , type_to_rst (in_t [1] , is_store , Intrinsic) , type_to_rst (in_t [2] , is_store , Intrinsic) , type_to_rst (in_t [3] , is_store , Intrinsic)) , _ => panic ! ("unsupported parameter number") , } ; if para_num == 1 && in_t [0] == "HI" { fn_inputs = match asm_fmts [1] . as_str () { "si13" | "i13" => format ! ("<const IMM_S13: {}>()" , type_to_rst (in_t [0] , is_store , Intrinsic)) , "si10" => format ! ("<const IMM_S10: {}>()" , type_to_rst (in_t [0] , is_store , Intrinsic)) , _ => panic ! ("unsupported assembly format: {}" , asm_fmts [1]) , } ; rustc_legacy_const_generics = "rustc_legacy_const_generics(0)" ; } else if para_num == 2 && (in_t [1] == "UQI" || in_t [1] == "USI") { fn_inputs = if asm_fmts [2] . starts_with ("ui") { format ! ("<const IMM{2}: {1}>(a: {0})" , type_to_rst (in_t [0] , is_store , Intrinsic) , type_to_rst (in_t [1] , is_store , Intrinsic) , asm_fmts [2] . get (2 ..) . unwrap ()) } else { panic ! ("unsupported assembly format: {}" , asm_fmts [2]) ; } ; rustc_legacy_const_generics = "rustc_legacy_const_generics(1)" ; } else if para_num == 2 && in_t [1] == "QI" { fn_inputs = if asm_fmts [2] . starts_with ("si") { format ! ("<const IMM_S{2}: {1}>(a: {0})" , type_to_rst (in_t [0] , is_store , Intrinsic) , type_to_rst (in_t [1] , is_store , Intrinsic) , asm_fmts [2] . get (2 ..) . unwrap ()) } else { panic ! ("unsupported assembly format: {}" , asm_fmts [2]) ; } ; rustc_legacy_const_generics = "rustc_legacy_const_generics(1)" ; } else if para_num == 2 && in_t [0] == "CVPOINTER" && in_t [1] == "SI" { fn_inputs = if asm_fmts [2] . starts_with ("si") { format ! ("<const IMM_S{2}: {1}>(mem_addr: {0})" , type_to_rst (in_t [0] , is_store , Intrinsic) , type_to_rst (in_t [1] , is_store , Intrinsic) , asm_fmts [2] . get (2 ..) . unwrap ()) } else { panic ! ("unsupported assembly format: {}" , asm_fmts [2]) ; } ; rustc_legacy_const_generics = "rustc_legacy_const_generics(1)" ; } else if para_num == 2 && in_t [0] == "CVPOINTER" && in_t [1] == "DI" { fn_inputs = match asm_fmts [2] . as_str () { "rk" => format ! ("(mem_addr: {}, b: {})" , type_to_rst (in_t [0] , is_store , Intrinsic) , type_to_rst (in_t [1] , is_store , Intrinsic)) , _ => panic ! ("unsupported assembly format: {}" , asm_fmts [2]) , } ; } else if para_num == 3 && (in_t [2] == "USI" || in_t [2] == "UQI") { fn_inputs = if asm_fmts [2] . starts_with ("ui") { format ! ("<const IMM{3}: {2}>(a: {0}, b: {1})" , type_to_rst (in_t [0] , is_store , Intrinsic) , type_to_rst (in_t [1] , is_store , Intrinsic) , type_to_rst (in_t [2] , is_store , Intrinsic) , asm_fmts [2] . get (2 ..) . unwrap ()) } else { panic ! ("unsupported assembly format: {}" , asm_fmts [2]) } ; rustc_legacy_const_generics = "rustc_legacy_const_generics(2)" ; } else if para_num == 3 && in_t [1] == "CVPOINTER" && in_t [2] == "SI" { fn_inputs = match asm_fmts [2] . as_str () { "si12" => format ! ("<const IMM_S12: {2}>(a: {0}, mem_addr: {1})" , type_to_rst (in_t [0] , is_store , Intrinsic) , type_to_rst (in_t [1] , is_store , Intrinsic) , type_to_rst (in_t [2] , is_store , Intrinsic)) , _ => panic ! ("unsupported assembly format: {}" , asm_fmts [2]) , } ; rustc_legacy_const_generics = "rustc_legacy_const_generics(2)" ; } else if para_num == 3 && in_t [1] == "CVPOINTER" && in_t [2] == "DI" { fn_inputs = match asm_fmts [2] . as_str () { "rk" => format ! ("(a: {}, mem_addr: {}, b: {})" , type_to_rst (in_t [0] , is_store , Intrinsic) , type_to_rst (in_t [1] , is_store , Intrinsic) , type_to_rst (in_t [2] , is_store , Intrinsic)) , _ => panic ! ("unsupported assembly format: {}" , asm_fmts [2]) , } ; } else if para_num == 4 { fn_inputs = match (asm_fmts [2] . as_str () , current_name . chars () . last () . unwrap ()) { ("si8" , t) => format ! ("<const IMM_S8: {2}, const IMM{4}: {3}>(a: {0}, mem_addr: {1})" , type_to_rst (in_t [0] , is_store , Intrinsic) , type_to_rst (in_t [1] , is_store , Intrinsic) , type_to_rst (in_t [2] , is_store , Intrinsic) , type_to_rst (in_t [3] , is_store , Intrinsic) , type_to_imm (t) ,) , (_ , _) => panic ! ("unsupported assembly format: {} for {}" , asm_fmts [2] , current_name) , } ; rustc_legacy_const_generics = "rustc_legacy_const_generics(2, 3)" ; } format ! ("pub {}fn {current_name}{fn_inputs} {fn_output}" , if is_mem { "unsafe " } else { "" }) } ; let unsafe_start = if ! is_mem { "unsafe { " } else { "" } ; let unsafe_end = if ! is_mem { " }" } else { "" } ; let mut call_params = { match para_num { 1 => format ! ("{unsafe_start}transmute(__{current_name}(transmute(a))){unsafe_end}") , 2 => format ! ("{unsafe_start}transmute(__{current_name}(transmute(a), transmute(b))){unsafe_end}") , 3 => format ! ("{unsafe_start}transmute(__{current_name}(transmute(a), transmute(b), transmute(c))){unsafe_end}") , 4 => format ! ("{unsafe_start}transmute(__{current_name}(transmute(a), transmute(b), transmute(c), transmute(d))){unsafe_end}") , _ => panic ! ("unsupported parameter number") , } } ; if para_num == 1 && in_t [0] == "HI" { call_params = match asm_fmts [1] . as_str () { "si10" => { format ! ("static_assert_simm_bits!(IMM_S10, 10);\n    {unsafe_start}transmute(__{current_name}(IMM_S10)){unsafe_end}") } "i13" => { format ! ("static_assert_simm_bits!(IMM_S13, 13);\n    {unsafe_start}transmute(__{current_name}(IMM_S13)){unsafe_end}") } _ => panic ! ("unsupported assembly format: {}" , asm_fmts [2]) , } } else if para_num == 2 && (in_t [1] == "UQI" || in_t [1] == "USI") { call_params = if asm_fmts [2] . starts_with ("ui") { format ! ("static_assert_uimm_bits!(IMM{0}, {0});\n    {unsafe_start}transmute(__{current_name}(transmute(a), IMM{0})){unsafe_end}" , asm_fmts [2] . get (2 ..) . unwrap ()) } else { panic ! ("unsupported assembly format: {}" , asm_fmts [2]) } ; } else if para_num == 2 && in_t [1] == "QI" { call_params = match asm_fmts [2] . as_str () { "si5" => { format ! ("static_assert_simm_bits!(IMM_S5, 5);\n    {unsafe_start}transmute(__{current_name}(transmute(a), IMM_S5)){unsafe_end}") } _ => panic ! ("unsupported assembly format: {}" , asm_fmts [2]) , } ; } else if para_num == 2 && in_t [0] == "CVPOINTER" && in_t [1] == "SI" { call_params = if asm_fmts [2] . starts_with ("si") { format ! ("static_assert_simm_bits!(IMM_S{0}, {0});\n    {unsafe_start}transmute(__{current_name}(mem_addr, IMM_S{0})){unsafe_end}" , asm_fmts [2] . get (2 ..) . unwrap ()) } else { panic ! ("unsupported assembly format: {}" , asm_fmts [2]) } } else if para_num == 2 && in_t [0] == "CVPOINTER" && in_t [1] == "DI" { call_params = match asm_fmts [2] . as_str () { "rk" => format ! ("{unsafe_start}transmute(__{current_name}(mem_addr, transmute(b))){unsafe_end}") , _ => panic ! ("unsupported assembly format: {}" , asm_fmts [2]) , } ; } else if para_num == 3 && (in_t [2] == "USI" || in_t [2] == "UQI") { call_params = if asm_fmts [2] . starts_with ("ui") { format ! ("static_assert_uimm_bits!(IMM{0}, {0});\n    {unsafe_start}transmute(__{current_name}(transmute(a), transmute(b), IMM{0})){unsafe_end}" , asm_fmts [2] . get (2 ..) . unwrap ()) } else { panic ! ("unsupported assembly format: {}" , asm_fmts [2]) } } else if para_num == 3 && in_t [1] == "CVPOINTER" && in_t [2] == "SI" { call_params = match asm_fmts [2] . as_str () { "si12" => format ! ("static_assert_simm_bits!(IMM_S12, 12);\n    {unsafe_start}transmute(__{current_name}(transmute(a), mem_addr, IMM_S12)){unsafe_end}") , _ => panic ! ("unsupported assembly format: {}" , asm_fmts [2]) , } ; } else if para_num == 3 && in_t [1] == "CVPOINTER" && in_t [2] == "DI" { call_params = match asm_fmts [2] . as_str () { "rk" => format ! ("{unsafe_start}transmute(__{current_name}(transmute(a), mem_addr, transmute(b))){unsafe_end}") , _ => panic ! ("unsupported assembly format: {}" , asm_fmts [2]) , } ; } else if para_num == 4 { call_params = match (asm_fmts [2] . as_str () , current_name . chars () . last () . unwrap ()) { ("si8" , t) => format ! ("static_assert_simm_bits!(IMM_S8, 8);\n    static_assert_uimm_bits!(IMM{0}, {0});\n    {unsafe_start}transmute(__{current_name}(transmute(a), mem_addr, IMM_S8, IMM{0})){unsafe_end}" , type_to_imm (t)) , (_ , _) => panic ! ("unsupported assembly format: {} for {}" , asm_fmts [2] , current_name) , } } let function = if ! rustc_legacy_const_generics . is_empty () { format ! (r#"
#[inline]{target_feature}
#[{rustc_legacy_const_generics}]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
{fn_decl}{{
    {call_params}
}}
"# , target_feature = target . to_target_feature_attr (current_name)) } else { format ! (r#"
#[inline]{target_feature}
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
{fn_decl}{{
    {call_params}
}}
"# , target_feature = target . to_target_feature_attr (current_name)) } ; (link_function , function) }
}

macro_rules! gen_test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function gen_test in module {}", module_path!());
    };
}

mkfn!{
    gen_test_introspect!();
    fn gen_test (in_file : String , ext_name : & str) -> io :: Result < () > { let f = File :: open (in_file . clone ()) . unwrap_or_else (| _ | panic ! ("Failed to open {in_file}")) ; let f = BufReader :: new (f) ; let target : TargetFeature = TargetFeature :: new (ext_name) ; let mut para_num ; let mut current_name : Option < String > = None ; let mut asm_fmts : Vec < String > = Vec :: new () ; let mut impl_function_str = String :: new () ; let mut call_function_str = String :: new () ; let mut out = String :: new () ; out . push_str (& format ! (r#"/*
 * This code is automatically generated. DO NOT MODIFY.
 *
 * Instead, modify `{in_file}` and run the following command to re-generate this file:
 *
 * ```
 * OUT_DIR=`pwd`/crates/stdarch-gen-loongarch cargo run -p stdarch-gen-loongarch -- {in_file} test
 * ```
 */

#include <stdio.h>
#include <stdint.h>
#include <lsxintrin.h>
#include <lasxintrin.h>

union v16qi
{{
    __m128i v;
    int64_t i64[2];
    int8_t i8[16];
}};

union v32qi
{{
    __m256i v;
    int64_t i64[4];
    int8_t i8[32];
}};

union v8hi
{{
    __m128i v;
    int64_t i64[2];
    int16_t i16[8];
}};

union v16hi
{{
    __m256i v;
    int64_t i64[4];
    int16_t i16[16];
}};

union v4si
{{
    __m128i v;
    int64_t i64[2];
    int32_t i32[4];
}};

union v8si
{{
    __m256i v;
    int64_t i64[4];
    int32_t i32[8];
}};

union v2di
{{
    __m128i v;
    int64_t i64[2];
}};

union v4di
{{
    __m256i v;
    int64_t i64[4];
}};

union uv16qi
{{
    __m128i v;
    uint64_t i64[2];
    uint8_t i8[16];
}};

union uv32qi
{{
    __m256i v;
    uint64_t i64[4];
    uint8_t i8[32];
}};

union uv8hi
{{
    __m128i v;
    uint64_t i64[2];
    uint16_t i16[8];
}};

union uv16hi
{{
    __m256i v;
    uint64_t i64[4];
    uint16_t i16[16];
}};

union uv4si
{{
    __m128i v;
    uint64_t i64[2];
    uint32_t i32[4];
}};

union uv8si
{{
    __m256i v;
    uint64_t i64[4];
    uint32_t i32[8];
}};

union uv2di
{{
    __m128i v;
    uint64_t i64[2];
}};

union uv4di
{{
    __m256i v;
    uint64_t i64[4];
}};

union v4sf
{{
    __m128 v;
    int64_t i64[2];
    uint32_t i32[2];
    float f32[4];
}};

union v8sf
{{
    __m256 v;
    int64_t i64[4];
    uint32_t i32[4];
    float f32[8];
}};

union v2df
{{
    __m128d v;
    uint64_t i64[2];
    double f64[2];
}};

union v4df
{{
    __m256d v;
    uint64_t i64[4];
    double f64[4];
}};
"#)) ; for line in f . lines () { let line = line . unwrap () ; if line . is_empty () { continue ; } if let Some (name) = line . strip_prefix ("name = ") { current_name = Some (String :: from (name)) ; } else if line . starts_with ("asm-fmts = ") { asm_fmts = line [10 ..] . split (',') . map (| v | v . trim () . to_string ()) . collect () ; } else if line . starts_with ("data-types = ") { let current_name = current_name . clone () . unwrap () ; let data_types : Vec < & str > = line . get (12 ..) . unwrap () . split (',') . map (| e | e . trim ()) . collect () ; let in_t ; let out_t ; if data_types . len () == 2 { in_t = [data_types [1] , "NULL" , "NULL" , "NULL"] ; out_t = data_types [0] ; para_num = 1 ; } else if data_types . len () == 3 { in_t = [data_types [1] , data_types [2] , "NULL" , "NULL"] ; out_t = data_types [0] ; para_num = 2 ; } else if data_types . len () == 4 { in_t = [data_types [1] , data_types [2] , data_types [3] , "NULL"] ; out_t = data_types [0] ; para_num = 3 ; } else if data_types . len () == 5 { in_t = [data_types [1] , data_types [2] , data_types [3] , data_types [4]] ; out_t = data_types [0] ; para_num = 4 ; } else { panic ! ("DEBUG: line: {0} len: {1}" , line , data_types . len ()) ; } let (link_function , function) = gen_test_body (& current_name , & asm_fmts , & in_t , out_t , para_num , target) ; impl_function_str . push_str (& link_function) ; call_function_str . push_str (& function) ; } } out . push_str (& impl_function_str) ; out . push ('\n') ; out . push_str ("int main(int argc, char *argv[])\n") ; out . push_str ("{\n") ; out . push_str ("    printf(\"// This code is automatically generated. DO NOT MODIFY.\\n\");\n") ; out . push_str ("    printf(\"// See crates/stdarch-gen-loongarch/README.md\\n\\n\");\n") ; out . push_str ("    printf(\"use crate::{\\n\");\n") ; out . push_str ("    printf(\"    core_arch::{loongarch64::*, simd::*},\\n\");\n") ; out . push_str ("    printf(\"    mem::transmute,\\n\");\n") ; out . push_str ("    printf(\"};\\n\");\n") ; out . push_str ("    printf(\"use stdarch_test::simd_test;\\n\");\n") ; out . push_str (& call_function_str) ; out . push_str ("    return 0;\n") ; out . push ('}') ; let out_dir_path : PathBuf = PathBuf :: from (env :: var ("OUT_DIR") . unwrap ()) ; std :: fs :: create_dir_all (& out_dir_path) ? ; let mut f = File :: create (out_dir_path . join (format ! ("{ext_name}.c"))) ? ; f . write_all (out . as_bytes ()) ? ; Ok (()) }
}

macro_rules! gen_test_body_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function gen_test_body in module {}", module_path!());
    };
}

mkfn!{
    gen_test_body_introspect!();
    fn gen_test_body (current_name : & str , asm_fmts : & [String] , in_t : & [& str ; 4] , out_t : & str , para_num : i32 , target : TargetFeature ,) -> (String , String) { let rand_i32 = | bits : u8 | -> i32 { let val = rand :: random :: < i32 > () ; let bits = 32 - bits ; (val << bits) >> bits } ; let rand_u32 = | bits : u8 | -> u32 { let val = rand :: random :: < u32 > () ; let bits = 32 - bits ; (val << bits) >> bits } ; let rand_i64 = | | -> i64 { rand :: random :: < i64 > () } ; let rand_u64 = | | -> u64 { rand :: random :: < u64 > () } ; let rand_f32 = | | -> f32 { rand :: random :: < f32 > () } ; let rand_f64 = | | -> f64 { rand :: random :: < f64 > () } ; let type_to_ct = | t : & str | -> & str { match t { "V16QI" => "union v16qi" , "V32QI" => "union v32qi" , "V8HI" => "union v8hi" , "V16HI" => "union v16hi" , "V4SI" => "union v4si" , "V8SI" => "union v8si" , "V2DI" => "union v2di" , "V4DI" => "union v4di" , "UV16QI" => "union uv16qi" , "UV32QI" => "union uv32qi" , "UV8HI" => "union uv8hi" , "UV16HI" => "union uv16hi" , "UV4SI" => "union uv4si" , "UV8SI" => "union uv8si" , "UV2DI" => "union uv2di" , "UV4DI" => "union uv4di" , "SI" => "int32_t" , "DI" => "int64_t" , "USI" => "uint32_t" , "UDI" => "uint64_t" , "V4SF" => "union v4sf" , "V8SF" => "union v8sf" , "V2DF" => "union v2df" , "V4DF" => "union v4df" , "UQI" => "uint32_t" , "QI" => "int32_t" , "CVPOINTER" => "void*" , "HI" => "int32_t" , _ => panic ! ("unknown type: {t}") , } } ; let type_to_va = | v : & str , t : & str | -> String { let n = if v . starts_with ('_') { v . get (1 ..) . unwrap () } else { v } ; let mut out = String :: new () ; match t { "A16QI" => { for i in 0 .. 16 { out . push_str (& format ! ("    {v}.i8[{i}] = {};\n" , rand_i32 (8))) ; } out . push_str (& format ! ("    printf(\"    let {n}: [i8; 16] = [%d")) ; for _ in 1 .. 16 { out . push_str (", %d") ; } out . push_str (& format ! ("];\\n\",\n    {v}.i8[0]")) ; for i in 1 .. 16 { out . push_str (& format ! (", {v}.i8[{i}]")) ; } } "AM16QI" => { for i in 0 .. 16 { out . push_str (& format ! ("    {v}.i8[{i}] = {};\n" , rand_i32 (8))) ; } out . push_str (& format ! ("    printf(\"    let mut {n}: [i8; 16] = [%d")) ; for _ in 1 .. 16 { out . push_str (", %d") ; } out . push_str (& format ! ("];\\n\",\n    {v}.i8[0]")) ; for i in 1 .. 16 { out . push_str (& format ! (", {v}.i8[{i}]")) ; } } "V16QI" => { for i in 0 .. 16 { out . push_str (& format ! ("    {v}.i8[{i}] = {};\n" , rand_i32 (8))) ; } out . push_str (& format ! ("    printf(\"    let {n} = i8x16::new(%d")) ; for _ in 1 .. 16 { out . push_str (", %d") ; } out . push_str (& format ! (");\\n\",\n    {v}.i8[0]")) ; for i in 1 .. 16 { out . push_str (& format ! (", {v}.i8[{i}]")) ; } } "V32QI" => { for i in 0 .. 32 { out . push_str (& format ! ("    {v}.i8[{i}] = {};\n" , rand_i32 (8))) ; } out . push_str (& format ! ("    printf(\"    let {n} = i8x32::new(%d")) ; for _ in 1 .. 32 { out . push_str (", %d") ; } out . push_str (& format ! (");\\n\",\n    {v}.i8[0]")) ; for i in 1 .. 32 { out . push_str (& format ! (", {v}.i8[{i}]")) ; } } "A32QI" => { for i in 0 .. 32 { out . push_str (& format ! ("    {v}.i8[{i}] = {};\n" , rand_i32 (8))) ; } out . push_str (& format ! ("    printf(\"    let {n}: [i8; 32] = [%d")) ; for _ in 1 .. 32 { out . push_str (", %d") ; } out . push_str (& format ! ("];\\n\",\n    {v}.i8[0]")) ; for i in 1 .. 32 { out . push_str (& format ! (", {v}.i8[{i}]")) ; } } "AM32QI" => { for i in 0 .. 32 { out . push_str (& format ! ("    {v}.i8[{i}] = {};\n" , rand_i32 (8))) ; } out . push_str (& format ! ("    printf(\"    let mut {n}: [i8; 32] = [%d")) ; for _ in 1 .. 32 { out . push_str (", %d") ; } out . push_str (& format ! ("];\\n\",\n    {v}.i8[0]")) ; for i in 1 .. 32 { out . push_str (& format ! (", {v}.i8[{i}]")) ; } } "V8HI" => { for i in 0 .. 8 { out . push_str (& format ! ("    {v}.i16[{i}] = {};\n" , rand_i32 (16))) ; } out . push_str (& format ! ("    printf(\"    let {n} = i16x8::new(%d")) ; for _ in 1 .. 8 { out . push_str (", %d") ; } out . push_str (& format ! (");\\n\",\n    {v}.i16[0]")) ; for i in 1 .. 8 { out . push_str (& format ! (", {v}.i16[{i}]")) ; } } "V16HI" => { for i in 0 .. 16 { out . push_str (& format ! ("    {v}.i16[{i}] = {};\n" , rand_i32 (16))) ; } out . push_str (& format ! ("    printf(\"    let {n} = i16x16::new(%d")) ; for _ in 1 .. 16 { out . push_str (", %d") ; } out . push_str (& format ! (");\\n\",\n    {v}.i16[0]")) ; for i in 1 .. 16 { out . push_str (& format ! (", {v}.i16[{i}]")) ; } } "V4SI" => { for i in 0 .. 4 { out . push_str (& format ! ("    {v}.i32[{i}] = {};\n" , rand_i32 (32))) ; } out . push_str (& format ! ("    printf(\"    let {n} = i32x4::new(%d")) ; for _ in 1 .. 4 { out . push_str (", %d") ; } out . push_str (& format ! (");\\n\",\n    {v}.i32[0]")) ; for i in 1 .. 4 { out . push_str (& format ! (", {v}.i32[{i}]")) ; } } "V8SI" => { for i in 0 .. 8 { out . push_str (& format ! ("    {v}.i32[{i}] = {};\n" , rand_i32 (32))) ; } out . push_str (& format ! ("    printf(\"    let {n} = i32x8::new(%d")) ; for _ in 1 .. 8 { out . push_str (", %d") ; } out . push_str (& format ! (");\\n\",\n    {v}.i32[0]")) ; for i in 1 .. 8 { out . push_str (& format ! (", {v}.i32[{i}]")) ; } } "V2DI" => { for i in 0 .. 2 { out . push_str (& format ! ("    {v}.i64[{i}] = {}L;\n" , rand_i64 ())) ; } out . push_str (& format ! ("    printf(\"    let {n} = i64x2::new(%ld")) ; for _ in 1 .. 2 { out . push_str (", %ld") ; } out . push_str (& format ! (");\\n\",\n    {v}.i64[0]")) ; for i in 1 .. 2 { out . push_str (& format ! (", {v}.i64[{i}]")) ; } } "V4DI" => { for i in 0 .. 4 { out . push_str (& format ! ("    {v}.i64[{i}] = {}L;\n" , rand_i64 ())) ; } out . push_str (& format ! ("    printf(\"    let {n} = i64x4::new(%ld")) ; for _ in 1 .. 4 { out . push_str (", %ld") ; } out . push_str (& format ! (");\\n\",\n    {v}.i64[0]")) ; for i in 1 .. 4 { out . push_str (& format ! (", {v}.i64[{i}]")) ; } } "UV16QI" => { for i in 0 .. 16 { out . push_str (& format ! ("    {v}.i8[{i}] = {};\n" , rand_u32 (8))) ; } out . push_str (& format ! ("    printf(\"    let {n} = u8x16::new(%u")) ; for _ in 1 .. 16 { out . push_str (", %u") ; } out . push_str (& format ! (");\\n\",\n    {v}.i8[0]")) ; for i in 1 .. 16 { out . push_str (& format ! (", {v}.i8[{i}]")) ; } } "UV32QI" => { for i in 0 .. 32 { out . push_str (& format ! ("    {v}.i8[{i}] = {};\n" , rand_u32 (8))) ; } out . push_str (& format ! ("    printf(\"    let {n} = u8x32::new(%u")) ; for _ in 1 .. 32 { out . push_str (", %u") ; } out . push_str (& format ! (");\\n\",\n    {v}.i8[0]")) ; for i in 1 .. 32 { out . push_str (& format ! (", {v}.i8[{i}]")) ; } } "UV8HI" => { for i in 0 .. 8 { out . push_str (& format ! ("    {v}.i16[{i}] = {};\n" , rand_u32 (16))) ; } out . push_str (& format ! ("    printf(\"    let {n} = u16x8::new(%u")) ; for _ in 1 .. 8 { out . push_str (", %u") ; } out . push_str (& format ! (");\\n\",\n    {v}.i16[0]")) ; for i in 1 .. 8 { out . push_str (& format ! (", {v}.i16[{i}]")) ; } } "UV16HI" => { for i in 0 .. 16 { out . push_str (& format ! ("    {v}.i16[{i}] = {};\n" , rand_u32 (16))) ; } out . push_str (& format ! ("    printf(\"    let {n} = u16x16::new(%u")) ; for _ in 1 .. 16 { out . push_str (", %u") ; } out . push_str (& format ! (");\\n\",\n    {v}.i16[0]")) ; for i in 1 .. 16 { out . push_str (& format ! (", {v}.i16[{i}]")) ; } } "UV4SI" => { for i in 0 .. 4 { out . push_str (& format ! ("    {v}.i32[{i}] = {};\n" , rand_u32 (32))) ; } out . push_str (& format ! ("    printf(\"    let {n} = u32x4::new(%u")) ; for _ in 1 .. 4 { out . push_str (", %u") ; } out . push_str (& format ! (");\\n\",\n    {v}.i32[0]")) ; for i in 1 .. 4 { out . push_str (& format ! (", {v}.i32[{i}]")) ; } } "UV8SI" => { for i in 0 .. 8 { out . push_str (& format ! ("    {v}.i32[{i}] = {};\n" , rand_u32 (32))) ; } out . push_str (& format ! ("    printf(\"    let {n} = u32x8::new(%u")) ; for _ in 1 .. 8 { out . push_str (", %u") ; } out . push_str (& format ! (");\\n\",\n    {v}.i32[0]")) ; for i in 1 .. 8 { out . push_str (& format ! (", {v}.i32[{i}]")) ; } } "UV2DI" => { for i in 0 .. 2 { out . push_str (& format ! ("    {v}.i64[{i}] = {}UL;\n" , rand_u64 ())) ; } out . push_str (& format ! ("    printf(\"    let {n} = u64x2::new(%lu")) ; for _ in 1 .. 2 { out . push_str (", %lu") ; } out . push_str (& format ! (");\\n\",\n    {v}.i64[0]")) ; for i in 1 .. 2 { out . push_str (& format ! (", {v}.i64[{i}]")) ; } } "UV4DI" => { for i in 0 .. 4 { out . push_str (& format ! ("    {v}.i64[{i}] = {}UL;\n" , rand_u64 ())) ; } out . push_str (& format ! ("    printf(\"    let {n} = u64x4::new(%lu")) ; for _ in 1 .. 4 { out . push_str (", %lu") ; } out . push_str (& format ! (");\\n\",\n    {v}.i64[0]")) ; for i in 1 .. 4 { out . push_str (& format ! (", {v}.i64[{i}]")) ; } } "V4SF" => { for i in 0 .. 4 { out . push_str (& format ! ("    {v}.f32[{i}] = {};\n" , rand_f32 ())) ; } out . push_str (& format ! ("    printf(\"    let {n} = u32x4::new(%u")) ; for _ in 1 .. 4 { out . push_str (", %u") ; } out . push_str (& format ! (");\\n\",\n    {v}.i32[0]")) ; for i in 1 .. 4 { out . push_str (& format ! (", {v}.i32[{i}]")) ; } } "V8SF" => { for i in 0 .. 8 { out . push_str (& format ! ("    {v}.f32[{i}] = {};\n" , rand_f32 ())) ; } out . push_str (& format ! ("    printf(\"    let {n} = u32x8::new(%u")) ; for _ in 1 .. 8 { out . push_str (", %u") ; } out . push_str (& format ! (");\\n\",\n    {v}.i32[0]")) ; for i in 1 .. 8 { out . push_str (& format ! (", {v}.i32[{i}]")) ; } } "V2DF" => { for i in 0 .. 2 { out . push_str (& format ! ("    {v}.f64[{i}] = {};\n" , rand_f64 ())) ; } out . push_str (& format ! ("    printf(\"    let {n} = u64x2::new(%lu")) ; for _ in 1 .. 2 { out . push_str (", %lu") ; } out . push_str (& format ! (");\\n\",\n    {v}.i64[0]")) ; for i in 1 .. 2 { out . push_str (& format ! (", {v}.i64[{i}]")) ; } } "V4DF" => { for i in 0 .. 4 { out . push_str (& format ! ("    {v}.f64[{i}] = {};\n" , rand_f64 ())) ; } out . push_str (& format ! ("    printf(\"    let {n} =  u64x4::new(%lu")) ; for _ in 1 .. 4 { out . push_str (", %lu") ; } out . push_str (& format ! (");\\n\",\n    {v}.i64[0]")) ; for i in 1 .. 4 { out . push_str (& format ! (", {v}.i64[{i}]")) ; } } "SI" | "DI" | "USI" | "UDI" | "UQI" | "QI" | "CVPOINTER" | "HI" => () , _ => panic ! ("unknown type: {t}") , } if ! out . is_empty () { out . push_str (");") ; } out } ; let type_to_rp = | t : & str | -> & str { match t { "SI" => "    printf(\"    let r: i32 = %d;\\n\", o);" , "DI" => "    printf(\"    let r: i64 = %ld;\\n\", o);" , "USI" => "    printf(\"    let r: u32 = %u;\\n\", o);" , "UDI" => "    printf(\"    let r: u64 = %lu;\\n\", o);" , "UQI" => "    printf(\"    let r: u32 = %u;\\n\", o);" , "QI" => "    printf(\"    let r: i32 = %d;\\n\", o);" , "HI" => "    printf(\"    let r: i32 = %d;\\n\", o);" , "V32QI" | "V16HI" | "V8SI" | "V4DI" | "UV32QI" | "UV16HI" | "UV8SI" | "UV4DI" | "V8SF" | "V4DF" => { "    printf(\"    let r = i64x4::new(%ld, %ld, %ld, %ld);\\n\", o.i64[0], o.i64[1], o.i64[2], o.i64[3]);" } _ => "    printf(\"    let r = i64x2::new(%ld, %ld);\\n\", o.i64[0], o.i64[1]);" , } } ; let type_to_rx = | t : & str | -> & str { match t { "SI" | "DI" | "USI" | "UDI" | "UQI" | "QI" | "HI" => "o" , _ => "o.v" , } } ; let type_to_imm = | t | -> i8 { match t { 'b' => 4 , 'h' => 3 , 'w' => 2 , 'd' => 1 , _ => panic ! ("unsupported type") , } } ; let impl_function = { let fn_output = if out_t . to_lowercase () == "void" { String :: new () } else { format ! ("    {} o;" , type_to_ct (out_t)) } ; let mut fn_inputs = match para_num { 1 => format ! ("    {} a;\n{}" , type_to_ct (in_t [0]) , type_to_va ("a" , in_t [0])) , 2 => format ! ("    {} a;\n{}\n    {} b;\n{}" , type_to_ct (in_t [0]) , type_to_va ("a" , in_t [0]) , type_to_ct (in_t [1]) , type_to_va ("b" , in_t [1])) , 3 => format ! ("    {} a;\n{}\n    {} b;\n{}\n    {} c;\n{}" , type_to_ct (in_t [0]) , type_to_va ("a" , in_t [0]) , type_to_ct (in_t [1]) , type_to_va ("b" , in_t [1]) , type_to_ct (in_t [2]) , type_to_va ("c" , in_t [2])) , 4 => format ! ("    {} a;\n{}\n    {} b;\n{}\n    {} c;\n{}\n    {} d;\n{}" , type_to_ct (in_t [0]) , type_to_va ("a" , in_t [0]) , type_to_ct (in_t [1]) , type_to_va ("b" , in_t [1]) , type_to_ct (in_t [2]) , type_to_va ("c" , in_t [2]) , type_to_ct (in_t [3]) , type_to_va ("d" , in_t [3])) , _ => panic ! ("unsupported parameter number") , } ; let mut fn_params = match para_num { 1 => "(a.v)" . to_string () , 2 => "(a.v, b.v)" . to_string () , 3 => "(a.v, b.v, c.v)" . to_string () , 4 => "(a.v, b.v, c.v, d.v)" . to_string () , _ => "unsupported parameter number" . to_string () , } ; let mut as_params = match para_num { 1 => "(transmute(a))" . to_string () , 2 => "(transmute(a), transmute(b))" . to_string () , 3 => "(transmute(a), transmute(b), transmute(c))" . to_string () , 4 => "(transmute(a), transmute(b), transmute(c), transmute(d))" . to_string () , _ => panic ! ("unsupported parameter number") , } ; let mut as_args = String :: new () ; if para_num == 1 && in_t [0] == "HI" { fn_inputs = "" . to_string () ; match asm_fmts [1] . as_str () { "si13" => { let val = rand_i32 (13) ; fn_params = format ! ("({val})") ; as_params = format ! ("::<{val}>()") ; } "i13" => { let val = rand_u32 (12) ; fn_params = format ! ("({val})") ; as_params = format ! ("::<{val}>()") ; } "si10" => { let val = rand_i32 (10) ; fn_params = format ! ("({val})") ; as_params = format ! ("::<{val}>()") ; } _ => panic ! ("unsupported assembly format: {}" , asm_fmts [1]) , } } else if para_num == 1 && (in_t [0] == "SI" || in_t [0] == "DI") && asm_fmts [1] . starts_with ("rj") { fn_params = "(a)" . to_string () ; if in_t [0] == "SI" { as_params = "(%d)" . to_string () ; } else { as_params = "(%ld)" . to_string () ; } as_args = ", a" . to_string () ; } else if para_num == 2 && (in_t [1] == "UQI" || in_t [1] == "USI") { if asm_fmts [2] . starts_with ("ui") { fn_inputs = format ! ("    {} a;\n{}" , type_to_ct (in_t [0]) , type_to_va ("a" , in_t [0])) ; let val = rand_u32 (asm_fmts [2] . get (2 ..) . unwrap () . parse :: < u8 > () . unwrap ()) ; fn_params = format ! ("(a.v, {val})") ; as_params = format ! ("::<{val}>(transmute(a))") ; } else { panic ! ("unsupported assembly format: {}" , asm_fmts [2]) ; } } else if para_num == 2 && in_t [1] == "QI" { if asm_fmts [2] . starts_with ("si") { fn_inputs = format ! ("    {} a;\n{}" , type_to_ct (in_t [0]) , type_to_va ("a" , in_t [0])) ; let val = rand_i32 (asm_fmts [2] . get (2 ..) . unwrap () . parse :: < u8 > () . unwrap ()) ; fn_params = format ! ("(a.v, {val})") ; as_params = format ! ("::<{val}>(transmute(a))") ; } else { panic ! ("unsupported assembly format: {}" , asm_fmts [2]) ; } } else if para_num == 2 && in_t [1] == "SI" && asm_fmts [2] . starts_with ("rk") { fn_params = "(a.v, b)" . to_string () ; as_params = "(transmute(a), %d)" . to_string () ; as_args = ", b" . to_string () ; } else if para_num == 2 && in_t [0] == "CVPOINTER" && in_t [1] == "SI" { if asm_fmts [2] . starts_with ("si") { fn_inputs = format ! ("    union v{}qi _a;\n{}\n    {} a = &_a;" , target . bytes () , type_to_va ("_a" , if target == TargetFeature :: Lsx { "A16QI" } else { "A32QI" }) , type_to_ct (in_t [0])) ; fn_params = "(a, 0)" . to_string () ; as_params = "::<0>(a.as_ptr())" . to_string () ; } else { panic ! ("unsupported assembly format: {}" , asm_fmts [2]) ; } } else if para_num == 2 && in_t [0] == "CVPOINTER" && in_t [1] == "DI" { if asm_fmts [2] . as_str () == "rk" { fn_inputs = format ! ("    union v{}qi _a;\n{}\n    {} a = &_a;" , target . bytes () , type_to_va ("_a" , if target == TargetFeature :: Lsx { "A16QI" } else { "A32QI" }) , type_to_ct (in_t [0])) ; fn_params = "(a, 0)" . to_string () ; as_params = "(a.as_ptr(), 0)" . to_string () ; } else { panic ! ("unsupported assembly format: {}" , asm_fmts [2]) ; } } else if para_num == 3 && in_t [2] == "UQI" && asm_fmts [1] . starts_with ("rj") { if asm_fmts [2] . starts_with ("ui") { fn_inputs = format ! ("    {} a;\n{}" , type_to_ct (in_t [0]) , type_to_va ("a" , in_t [0])) ; let ival = rand_i32 (32) ; let uval = rand_u32 (asm_fmts [2] . get (2 ..) . unwrap () . parse :: < u8 > () . unwrap ()) ; fn_params = format ! ("(a.v, {ival}, {uval})") ; as_params = format ! ("::<{uval}>(transmute(a), {ival})") ; } else { panic ! ("unsupported assembly format: {}" , asm_fmts [2]) ; } } else if para_num == 3 && (in_t [2] == "USI" || in_t [2] == "UQI") { if asm_fmts [2] . starts_with ("ui") { fn_inputs = format ! ("    {} a;\n{}\n    {} b;\n{}" , type_to_ct (in_t [0]) , type_to_va ("a" , in_t [0]) , type_to_ct (in_t [1]) , type_to_va ("b" , in_t [1]) ,) ; let val = rand_u32 (asm_fmts [2] . get (2 ..) . unwrap () . parse :: < u8 > () . unwrap ()) ; fn_params = format ! ("(a.v, b.v, {val})") ; as_params = format ! ("::<{val}>(transmute(a), transmute(b))") ; } else { panic ! ("unsupported assembly format: {}" , asm_fmts [2]) ; } } else if para_num == 3 && in_t [1] == "CVPOINTER" && in_t [2] == "SI" { if asm_fmts [2] . as_str () == "si12" { fn_inputs = format ! ("    {} a;\n{}\n    union v{}qi o;\n{}\n    {} b = &o;" , type_to_ct (in_t [0]) , type_to_va ("a" , in_t [0]) , target . bytes () , type_to_va ("o" , if target == TargetFeature :: Lsx { "AM16QI" } else { "AM32QI" }) , type_to_ct (in_t [1])) ; fn_params = "(a.v, b, 0)" . to_string () ; as_params = "::<0>(transmute(a), o.as_mut_ptr())" . to_string () ; } else { panic ! ("unsupported assembly format: {}" , asm_fmts [2]) ; } } else if para_num == 3 && in_t [1] == "CVPOINTER" && in_t [2] == "DI" { if asm_fmts [2] . as_str () == "rk" { fn_inputs = format ! ("    {} a;\n{}\n    union v{}qi o;\n{}\n    {} b = &o;" , type_to_ct (in_t [0]) , type_to_va ("a" , in_t [0]) , target . bytes () , type_to_va ("o" , if target == TargetFeature :: Lsx { "AM16QI" } else { "AM32QI" }) , type_to_ct (in_t [1])) ; fn_params = "(a.v, b, 0)" . to_string () ; as_params = "(transmute(a), o.as_mut_ptr(), 0)" . to_string () ; } else { panic ! ("unsupported assembly format: {}" , asm_fmts [2]) ; } } else if para_num == 4 { match (asm_fmts [2] . as_str () , current_name . chars () . last () . unwrap ()) { ("si8" , t) => { fn_inputs = format ! ("    {} a;\n{}\n    union v{}qi o;\n{}\n    {} b = &o;" , type_to_ct (in_t [0]) , type_to_va ("a" , in_t [0]) , target . bytes () , type_to_va ("o" , if target == TargetFeature :: Lsx { "AM16QI" } else { "AM32QI" }) , type_to_ct (in_t [1])) ; let val = rand_u32 (type_to_imm (t) . try_into () . unwrap ()) ; fn_params = format ! ("(a.v, b, 0, {val})") ; as_params = format ! ("::<0, {val}>(transmute(a), o.as_mut_ptr())") ; } (_ , _) => panic ! ("unsupported assembly format: {} for {}" , asm_fmts [2] , current_name) , } ; } let fn_docall = if out_t . to_lowercase () == "void" { format ! ("    __{current_name}{fn_params};") } else { format ! ("    {} = __{current_name}{fn_params};" , type_to_rx (out_t)) } ; let fn_result = if out_t . to_lowercase () == "void" { if target == TargetFeature :: Lsx { type_to_rp ("V16QI") } else { type_to_rp ("V32QI") } } else { type_to_rp (out_t) } ; let fn_assert = { if out_t . to_lowercase () == "void" { format ! ("    printf(\"\\n    {current_name}{as_params};\\n    assert_eq!(r, transmute(o));\\n\"{as_args});") } else { format ! ("    printf(\"\\n    assert_eq!(r, transmute({current_name}{as_params}));\\n\"{as_args});") } } ; format ! (r#"
static void {current_name}(void)
{{
    printf("\n#[simd_test(enable = \"{}\")]\n");
    printf("unsafe fn test_{current_name}() {{\n");
{fn_inputs}
{fn_output}
{fn_docall}
{fn_result}
{fn_assert}
    printf("}}\n");
}}
"# , target . as_target_feature_arg (current_name)) } ; let call_function = format ! ("    {current_name}();\n") ; (impl_function , call_function) }
}

macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    pub fn main () -> io :: Result < () > { let args : Vec < String > = env :: args () . collect () ; let in_file = args . get (1) . cloned () . expect ("Input file missing!") ; let in_file_path = PathBuf :: from (& in_file) ; let in_file_name = in_file_path . file_name () . unwrap () . to_os_string () . into_string () . unwrap () ; let ext_name = if in_file_name . starts_with ("lasx") { "lasx" } else { "lsx" } ; if in_file_name . ends_with (".h") { gen_spec (in_file , ext_name) } else if args . get (2) . is_some () { gen_test (in_file , ext_name) } else { gen_bind (in_file , ext_name) } }
}