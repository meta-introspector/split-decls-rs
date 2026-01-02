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
mkuse!{use proc_macro2 :: TokenStream ;}
mkuse!{use quote :: { ToTokens , TokenStreamExt , quote } ;}
mkuse!{use regex :: Regex ;}
mkuse!{use serde_with :: { DeserializeFromStr , SerializeDisplay } ;}
mkuse!{use std :: fmt ;}
mkuse!{use std :: str :: FromStr ;}
mkuse!{use std :: sync :: LazyLock ;}
mkuse!{use crate :: context ;}
mkuse!{use crate :: expression :: { Expression , FnCall } ;}
mkuse!{use crate :: intrinsic :: AccessLevel ;}
mkuse!{use crate :: wildcards :: Wildcard ;}
mkitem!{const VECTOR_FULL_REGISTER_SIZE : u32 = 128 ;}
mkitem!{const VECTOR_HALF_REGISTER_SIZE : u32 = VECTOR_FULL_REGISTER_SIZE / 2 ;}
mkitem!{mkenum!{#[derive (Debug , Clone , Copy)] pub enum TypeRepr { C , Rust , LLVMMachine , ACLENotation , Size , SizeLiteral , TypeKind , SizeInBytesLog2 , }}}
mkitem!{mktrait!{pub trait ToRepr { fn repr (& self , repr : TypeRepr) -> String ; fn c_repr (& self) -> String { self . repr (TypeRepr :: C) } fn rust_repr (& self) -> String { self . repr (TypeRepr :: Rust) } fn llvm_machine_repr (& self) -> String { self . repr (TypeRepr :: LLVMMachine) } fn acle_notation_repr (& self) -> String { self . repr (TypeRepr :: ACLENotation) } fn size (& self) -> String { self . repr (TypeRepr :: Size) } fn size_literal (& self) -> String { self . repr (TypeRepr :: SizeLiteral) } fn type_kind (& self) -> String { self . repr (TypeRepr :: TypeKind) } fn size_in_bytes_log2 (& self) -> String { self . repr (TypeRepr :: SizeInBytesLog2) } }}}
mkitem!{mkstruct!{#[derive (Debug , Clone , Copy , Default , PartialEq , Eq , Hash)] pub struct TypeKindOptions { f : bool , s : bool , u : bool , p : bool , }}}
mkitem!{mkimpl!{impl TypeKindOptions { pub fn contains (& self , kind : BaseTypeKind) -> bool { match kind { BaseTypeKind :: Float => self . f , BaseTypeKind :: Int => self . s , BaseTypeKind :: UInt => self . u , BaseTypeKind :: Poly => self . p , BaseTypeKind :: Bool => false , } } }}}
mkitem!{mkimpl!{impl FromStr for TypeKindOptions { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let mut result = Self :: default () ; for kind in s . bytes () { match kind { b'f' => result . f = true , b's' => result . s = true , b'u' => result . u = true , b'p' => result . p = true , _ => { return Err (format ! ("unknown type kind: {}" , char :: from (kind))) ; } } } Ok (result) } }}}
mkitem!{mkimpl!{impl fmt :: Display for TypeKindOptions { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . f . then (| | write ! (f , "f")) . transpose () ? ; self . s . then (| | write ! (f , "s")) . transpose () ? ; self . u . then (| | write ! (f , "u")) . transpose () . map (| _ | ()) } }}}
mkitem!{mkenum!{#[derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum BaseTypeKind { Float , Int , UInt , Bool , Poly , }}}
mkitem!{mkenum!{#[derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum BaseType { Sized (BaseTypeKind , u32) , Unsized (BaseTypeKind) , }}}
mkitem!{mkenum!{#[derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash , SerializeDisplay , DeserializeFromStr ,)] pub enum VectorTupleSize { Two , Three , Four , }}}
mkitem!{mkimpl!{impl VectorTupleSize { pub fn to_int (self) -> u32 { match self { Self :: Two => 2 , Self :: Three => 3 , Self :: Four => 4 , } } }}}
mkitem!{mkstruct!{#[derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct VectorType { base_type : BaseType , lanes : u32 , is_scalable : bool , tuple_size : Option < VectorTupleSize > , }}}
mkitem!{mkenum!{#[derive (Debug , Clone , PartialEq , Eq , Hash , SerializeDisplay , DeserializeFromStr)] pub enum TypeKind { Vector (VectorType) , Base (BaseType) , Pointer (Box < TypeKind > , AccessLevel) , Custom (String) , Wildcard (Wildcard) , }}}
mkitem!{mkimpl!{impl TypeKind { pub fn base_type (& self) -> Option < & BaseType > { match self { Self :: Vector (t) => Some (t . base_type ()) , Self :: Pointer (t , _) => t . base_type () , Self :: Base (t) => Some (t) , Self :: Wildcard (..) => None , Self :: Custom (..) => None , } } pub fn base_type_mut (& mut self) -> Option < & mut BaseType > { match self { Self :: Vector (t) => Some (t . base_type_mut ()) , Self :: Pointer (t , _) => t . base_type_mut () , Self :: Base (t) => Some (t) , Self :: Wildcard (..) => None , Self :: Custom (..) => None , } } pub fn populate_wildcard (& mut self , type_kind : TypeKind) -> context :: Result { match self { Self :: Wildcard (..) => * self = type_kind , Self :: Pointer (t , _) => t . populate_wildcard (type_kind) ? , _ => return Err ("no wildcard available to populate" . to_string ()) , } Ok (()) } pub fn base (& self) -> Option < & BaseType > { match self { Self :: Base (ty) => Some (ty) , Self :: Pointer (tk , _) => tk . base () , Self :: Vector (ty) => Some (& ty . base_type) , _ => None , } } pub fn vector (& self) -> Option < & VectorType > { match self { Self :: Vector (ty) => Some (ty) , _ => None , } } pub fn vector_mut (& mut self) -> Option < & mut VectorType > { match self { Self :: Vector (ty) => Some (ty) , _ => None , } } pub fn wildcard (& self) -> Option < & Wildcard > { match self { Self :: Wildcard (w) => Some (w) , Self :: Pointer (w , _) => w . wildcard () , _ => None , } } pub fn make_predicate_from (ty : & TypeKind) -> context :: Result < TypeKind > { Ok (TypeKind :: Vector (VectorType :: make_predicate_from_bitsize (ty . base_type () . ok_or_else (| | format ! ("cannot infer predicate from type {ty}")) ? . get_size () . map_err (| _ | format ! ("cannot infer predicate from unsized type {ty}")) ? ,))) } pub fn make_vector (from : TypeKind , is_scalable : bool , tuple_size : Option < VectorTupleSize > ,) -> context :: Result < TypeKind > { from . base () . cloned () . map_or_else (| | Err (format ! ("cannot make a vector type out of {from}!")) , | base | { let vt = VectorType :: make_from_base (base , is_scalable , tuple_size) ; Ok (TypeKind :: Vector (vt)) } ,) } #[doc = " Return a new expression that converts the provided `expr` from type `other` to `self`."] #[doc = ""] #[doc = " Conversions are bitwise over the whole value, like `transmute`, though `transmute`"] #[doc = " itself is only used as a last resort."] #[doc = ""] #[doc = " This can fail (returning `None`) due to incompatible types, and many conversions are simply"] #[doc = " unimplemented."] pub fn express_reinterpretation_from (& self , other : & TypeKind , expr : impl Into < Expression > ,) -> Option < Expression > { if self == other { Some (expr . into ()) } else if let (Some (self_vty) , Some (other_vty)) = (self . vector () , other . vector ()) { if self_vty . is_scalable && self_vty . tuple_size . is_none () && other_vty . is_scalable && other_vty . tuple_size . is_none () { use BaseTypeKind :: * ; match (self_vty . base_type , other_vty . base_type) { (BaseType :: Sized (Int , self_size) , BaseType :: Sized (UInt , other_size)) if self_size == other_size => { Some (Expression :: MethodCall (Box :: new (expr . into ()) , "as_signed" . parse () . unwrap () , vec ! [] ,)) } (BaseType :: Sized (UInt , self_size) , BaseType :: Sized (Int , other_size)) if self_size == other_size => { Some (Expression :: MethodCall (Box :: new (expr . into ()) , "as_unsigned" . parse () . unwrap () , vec ! [] ,)) } (BaseType :: Sized (Float | Int | UInt , _) , BaseType :: Sized (Float | Int | UInt , _) ,) => Some (FnCall :: new_expression ("simd_reinterpret" . parse () . unwrap () , vec ! [expr . into ()] ,)) , _ => None , } } else { None } } else { None } } }}}
mkitem!{mkimpl!{impl FromStr for TypeKind { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (match s { s if s . starts_with ('{') && s . ends_with ('}') => { Self :: Wildcard (s [1 .. s . len () - 1] . trim () . parse () ?) } s if s . starts_with ('*') => { let mut split = s [1 ..] . split_whitespace () ; let (ty , rw) = match (split . clone () . count () , split . next () , split . next ()) { (2 , Some ("mut") , Some (ty)) => (ty , AccessLevel :: RW) , (2 , Some ("const") , Some (ty)) => (ty , AccessLevel :: R) , (1 , Some (ty) , None) => (ty , AccessLevel :: R) , _ => return Err (format ! ("invalid pointer type {s:#?} given")) , } ; Self :: Pointer (Box :: new (ty . parse () ?) , rw) } _ => s . parse :: < VectorType > () . map (TypeKind :: Vector) . or_else (| _ | s . parse :: < BaseType > () . map (TypeKind :: Base)) . unwrap_or_else (| _ | TypeKind :: Custom (s . to_string ())) , }) } }}}
mkitem!{mkimpl!{impl fmt :: Display for TypeKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Vector (ty) => write ! (f , "{ty}") , Self :: Pointer (ty , _) => write ! (f , "{ty}") , Self :: Base (ty) => write ! (f , "{ty}") , Self :: Wildcard (w) => write ! (f , "{{{w}}}") , Self :: Custom (s) => write ! (f , "{s}") , } } }}}
mkitem!{mkimpl!{impl ToRepr for TypeKind { fn repr (& self , repr : TypeRepr) -> String { match self { Self :: Vector (ty) => ty . repr (repr) , Self :: Pointer (ty , _) => ty . repr (repr) , Self :: Base (ty) => ty . repr (repr) , Self :: Wildcard (w) => format ! ("{w}") , Self :: Custom (s) => s . to_string () , } } }}}
mkitem!{mkimpl!{impl ToTokens for TypeKind { fn to_tokens (& self , tokens : & mut TokenStream) { if let Self :: Pointer (_ , rw) = self { tokens . append_all (match rw { AccessLevel :: RW => quote ! { * mut } , AccessLevel :: R => quote ! { * const } , }) } tokens . append_all (self . to_string () . parse :: < TokenStream > () . expect ("invalid syntax") ,) } }}}
mkitem!{mkimpl!{impl PartialOrd for TypeKind { fn partial_cmp (& self , other : & Self) -> Option < std :: cmp :: Ordering > { Some (self . cmp (other)) } }}}
mkitem!{mkimpl!{impl From < & TypeKind > for usize { fn from (ty : & TypeKind) -> Self { match ty { TypeKind :: Base (_) => 1 , TypeKind :: Pointer (_ , _) => 2 , TypeKind :: Vector (_) => 3 , TypeKind :: Custom (_) => 4 , TypeKind :: Wildcard (_) => 5 , } } }}}
mkitem!{mkimpl!{impl Ord for TypeKind { fn cmp (& self , other : & Self) -> std :: cmp :: Ordering { use std :: cmp :: Ordering :: * ; let self_int : usize = self . into () ; let other_int : usize = other . into () ; if self_int == other_int { match (self , other) { (TypeKind :: Base (ty1) , TypeKind :: Base (ty2)) => ty1 . cmp (ty2) , (TypeKind :: Pointer (ty1 , _) , TypeKind :: Pointer (ty2 , _)) => ty1 . cmp (ty2) , (TypeKind :: Vector (vt1) , TypeKind :: Vector (vt2)) => vt1 . cmp (vt2) , (TypeKind :: Custom (s1) , TypeKind :: Custom (s2)) => s1 . cmp (s2) , (TypeKind :: Wildcard (..) , TypeKind :: Wildcard (..)) => Equal , _ => unreachable ! () , } } else { self_int . cmp (& other_int) } } }}}
mkitem!{mkimpl!{impl VectorType { pub fn base_type (& self) -> & BaseType { & self . base_type } pub fn base_type_mut (& mut self) -> & mut BaseType { & mut self . base_type } fn sanitise_lanes (mut base_type : BaseType , lanes : Option < u32 > ,) -> Result < (BaseType , u32) , String > { let lanes = match (base_type , lanes) { (BaseType :: Sized (BaseTypeKind :: Bool , lanes) , None) => { base_type = BaseType :: Sized (BaseTypeKind :: Bool , VECTOR_FULL_REGISTER_SIZE / lanes) ; lanes } (BaseType :: Unsized (BaseTypeKind :: Bool) , None) => { base_type = BaseType :: Sized (BaseTypeKind :: Bool , 8) ; 16 } (BaseType :: Sized (_ , size) , None) => VECTOR_FULL_REGISTER_SIZE / size , (BaseType :: Sized (_ , size) , Some (lanes)) => match size * lanes { VECTOR_FULL_REGISTER_SIZE | VECTOR_HALF_REGISTER_SIZE => lanes , _ => return Err ("invalid number of lanes" . to_string ()) , } , _ => return Err ("cannot infer number of lanes" . to_string ()) , } ; Ok ((base_type , lanes)) } pub fn make_from_base (base_ty : BaseType , is_scalable : bool , tuple_size : Option < VectorTupleSize > ,) -> VectorType { #[allow (clippy :: collapsible_if)] if is_scalable { if let BaseType :: Sized (BaseTypeKind :: Bool , size) = base_ty { return Self :: make_predicate_from_bitsize (size) ; } } let (base_type , lanes) = Self :: sanitise_lanes (base_ty , None) . unwrap () ; VectorType { base_type , lanes , is_scalable , tuple_size , } } pub fn make_predicate_from_bitsize (size : u32) -> VectorType { VectorType { base_type : BaseType :: Sized (BaseTypeKind :: Bool , size) , lanes : (VECTOR_FULL_REGISTER_SIZE / size) , is_scalable : true , tuple_size : None , } } pub fn cast_base_type_as (& mut self , ty : BaseType) { self . base_type = ty } pub fn lanes (& self) -> u32 { self . lanes } pub fn tuple_size (& self) -> Option < VectorTupleSize > { self . tuple_size } }}}
mkitem!{mkimpl!{impl FromStr for VectorType { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { static RE : LazyLock < Regex > = LazyLock :: new (| | { Regex :: new (r"^(?:(?:sv(?P<sv_ty>(?:uint|int|bool|float)(?:\d+)?))|(?:(?P<ty>(?:uint|int|bool|poly|float)(?:\d+)?)x(?P<lanes>(?:\d+)?)))(?:x(?P<tuple_size>2|3|4))?_t$") . unwrap () }) ; if let Some (c) = RE . captures (s) { let (base_type , lanes) = Self :: sanitise_lanes (c . name ("sv_ty") . or_else (| | c . name ("ty")) . map (< & str > :: from) . map (BaseType :: from_str) . unwrap () ? , c . name ("lanes") . map (< & str > :: from) . map (u32 :: from_str) . transpose () . unwrap () ,) . map_err (| e | format ! ("invalid {s:#?} vector type: {e}")) ? ; let tuple_size = c . name ("tuple_size") . map (< & str > :: from) . map (VectorTupleSize :: from_str) . transpose () . unwrap () ; Ok (VectorType { base_type , is_scalable : c . name ("sv_ty") . is_some () , lanes , tuple_size , }) } else { Err (format ! ("invalid vector type {s:#?} given")) } } }}}
mkitem!{mkimpl!{impl ToRepr for VectorType { fn repr (& self , repr : TypeRepr) -> String { let make_llvm_repr = | show_unsigned | { format ! ("{}v{}{}" , if self . is_scalable { "nx" } else { "" } , self . lanes * (self . tuple_size . map (usize :: from) . unwrap_or (1) as u32) , match self . base_type { BaseType :: Sized (BaseTypeKind :: UInt , size) if show_unsigned => format ! ("u{size}") , _ => self . base_type . llvm_machine_repr () , }) } ; if matches ! (repr , TypeRepr :: ACLENotation) { self . base_type . acle_notation_repr () } else if matches ! (repr , TypeRepr :: LLVMMachine) { make_llvm_repr (false) } else if self . is_scalable { match (self . base_type , self . lanes , self . tuple_size) { (BaseType :: Sized (BaseTypeKind :: Bool , _) , 16 , _) => "svbool_t" . to_string () , (BaseType :: Sized (BaseTypeKind :: Bool , _) , lanes , _) => format ! ("svbool{lanes}_t") , (BaseType :: Sized (_ , size) , lanes , _) if VECTOR_FULL_REGISTER_SIZE != (size * lanes) => { make_llvm_repr (true) } (ty , _ , None) => format ! ("sv{}_t" , ty . c_repr ()) , (ty , _ , Some (tuple_size)) => format ! ("sv{}x{tuple_size}_t" , ty . c_repr ()) , } } else { match self . tuple_size { Some (tuple_size) => format ! ("{}x{}x{}_t" , self . base_type . c_repr () , self . lanes , tuple_size) , None => format ! ("{}x{}_t" , self . base_type . c_repr () , self . lanes) , } } } }}}
mkitem!{mkimpl!{impl fmt :: Display for VectorType { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . c_repr ()) } }}}
mkitem!{mkimpl!{impl From < VectorTupleSize > for usize { fn from (t : VectorTupleSize) -> Self { match t { VectorTupleSize :: Two => 2 , VectorTupleSize :: Three => 3 , VectorTupleSize :: Four => 4 , } } }}}
mkitem!{mkimpl!{impl FromStr for VectorTupleSize { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "2" => Ok (Self :: Two) , "3" => Ok (Self :: Three) , "4" => Ok (Self :: Four) , _ => Err (format ! ("invalid vector tuple size `{s}` provided")) , } } }}}
mkitem!{mkimpl!{impl TryFrom < usize > for VectorTupleSize { type Error = String ; fn try_from (value : usize) -> Result < Self , Self :: Error > { match value { 2 => Ok (Self :: Two) , 3 => Ok (Self :: Three) , 4 => Ok (Self :: Four) , _ => Err (format ! ("invalid vector tuple size `{value}` provided")) , } } }}}
mkitem!{mkimpl!{impl fmt :: Display for VectorTupleSize { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , usize :: from (* self)) } }}}
mkitem!{mkimpl!{impl FromStr for BaseTypeKind { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "float" | "f" => Ok (Self :: Float) , "int" | "i" => Ok (Self :: Int) , "uint" | "u" => Ok (Self :: UInt) , "poly" | "p" => Ok (Self :: Poly) , "bool" | "b" => Ok (Self :: Bool) , _ => Err (format ! ("no match for {s}")) , } } }}}
mkitem!{mkimpl!{impl ToRepr for BaseTypeKind { fn repr (& self , repr : TypeRepr) -> String { match (repr , self) { (TypeRepr :: C , Self :: Float) => "float" , (TypeRepr :: C , Self :: Int) => "int" , (TypeRepr :: C , Self :: UInt) => "uint" , (TypeRepr :: C , Self :: Poly) => "poly" , (TypeRepr :: Rust | TypeRepr :: LLVMMachine | TypeRepr :: ACLENotation , Self :: Float) => "f" , (TypeRepr :: Rust , Self :: Int) | (TypeRepr :: LLVMMachine , Self :: Int | Self :: UInt) => "i" , (TypeRepr :: Rust | TypeRepr :: ACLENotation , Self :: UInt) => "u" , (TypeRepr :: Rust | TypeRepr :: LLVMMachine | TypeRepr :: ACLENotation , Self :: Poly) => "p" , (TypeRepr :: ACLENotation , Self :: Int) => "s" , (TypeRepr :: ACLENotation , Self :: Bool) => "b" , (_ , Self :: Bool) => "bool" , _ => { unreachable ! ("no base type kind available for representation {repr:?}") } } . to_string () } }}}
mkitem!{mkimpl!{impl fmt :: Display for BaseTypeKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . c_repr ()) } }}}
mkitem!{mkimpl!{impl BaseType { pub fn get_size (& self) -> Result < u32 , String > { match self { Self :: Sized (_ , size) => Ok (* size) , _ => Err (format ! ("unexpected invalid base type given {self:#?}")) , } } pub fn kind (& self) -> & BaseTypeKind { match self { BaseType :: Sized (kind , _) | BaseType :: Unsized (kind) => kind , } } pub fn is_bool (& self) -> bool { self . kind () == & BaseTypeKind :: Bool } pub fn is_float (& self) -> bool { self . kind () == & BaseTypeKind :: Float } }}}
mkitem!{mkimpl!{impl FromStr for BaseType { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { static RE : LazyLock < Regex > = LazyLock :: new (| | Regex :: new (r"^(?P<kind>[a-zA-Z]+)(?P<size>\d+)?(_t)?$") . unwrap ()) ; if let Some (c) = RE . captures (s) { let kind = c ["kind"] . parse () ? ; let size = c . name ("size") . map (< & str > :: from) . map (u32 :: from_str) . transpose () . unwrap () ; match size { Some (size) => Ok (Self :: Sized (kind , size)) , None => Ok (Self :: Unsized (kind)) , } } else { Err (format ! ("failed to parse type `{s}`")) } } }}}
mkitem!{mkimpl!{impl ToRepr for BaseType { fn repr (& self , repr : TypeRepr) -> String { use BaseType :: * ; use BaseTypeKind :: * ; use TypeRepr :: * ; match (self , & repr) { (Sized (Bool , _) | Unsized (Bool) , LLVMMachine) => "i1" . to_string () , (Sized (_ , size) , SizeLiteral) if * size == 8 => "b" . to_string () , (Sized (_ , size) , SizeLiteral) if * size == 16 => "h" . to_string () , (Sized (_ , size) , SizeLiteral) if * size == 32 => "w" . to_string () , (Sized (_ , size) , SizeLiteral) if * size == 64 => "d" . to_string () , (Sized (_ , size) , SizeLiteral) if * size == 128 => "q" . to_string () , (_ , SizeLiteral) => unreachable ! ("cannot represent {self:#?} as size literal") , (Sized (Float , _) | Unsized (Float) , TypeKind) => "f" . to_string () , (Sized (Int , _) | Unsized (Int) , TypeKind) => "s" . to_string () , (Sized (UInt , _) | Unsized (UInt) , TypeKind) => "u" . to_string () , (Sized (_ , size) , Size) => size . to_string () , (Sized (_ , size) , SizeInBytesLog2) => { assert ! (size . is_power_of_two () && * size >= 8) ; (size >> 3) . trailing_zeros () . to_string () } (Sized (kind , size) , _) => format ! ("{}{size}" , kind . repr (repr)) , (Unsized (kind) , _) => kind . repr (repr) , } } }}}
mkitem!{mkimpl!{impl fmt :: Display for BaseType { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . rust_repr ()) } }}}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: typekinds :: * ;}

macro_rules! test_predicate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_predicate in module {}", module_path!());
    };
}

mkfn!{
    test_predicate_introspect!();
    #[test] fn test_predicate () { assert_eq ! ("svbool_t" . parse ::< TypeKind > () . unwrap () , TypeKind :: Vector (VectorType { base_type : BaseType :: Sized (BaseTypeKind :: Bool , 8) , is_scalable : true , lanes : 16 , tuple_size : None })) ; }
}

macro_rules! test_llvm_internal_predicate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_llvm_internal_predicate in module {}", module_path!());
    };
}

mkfn!{
    test_llvm_internal_predicate_introspect!();
    #[test] fn test_llvm_internal_predicate () { assert_eq ! ("svbool4_t" . parse ::< TypeKind > () . unwrap () , TypeKind :: Vector (VectorType { base_type : BaseType :: Sized (BaseTypeKind :: Bool , 32) , is_scalable : true , lanes : 4 , tuple_size : None })) ; }
}

macro_rules! test_llvm_internal_predicate_llvm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_llvm_internal_predicate_llvm in module {}", module_path!());
    };
}

mkfn!{
    test_llvm_internal_predicate_llvm_introspect!();
    #[test] fn test_llvm_internal_predicate_llvm () { assert_eq ! ("svbool4_t" . parse ::< TypeKind > () . unwrap () . llvm_machine_repr () , "nxv4i1") ; }
}

macro_rules! test_llvm_internal_predicate_acle_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_llvm_internal_predicate_acle in module {}", module_path!());
    };
}

mkfn!{
    test_llvm_internal_predicate_acle_introspect!();
    #[test] fn test_llvm_internal_predicate_acle () { assert_eq ! ("svbool4_t" . parse ::< TypeKind > () . unwrap () . acle_notation_repr () , "b32") ; }
}

macro_rules! test_predicate_from_bitsize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_predicate_from_bitsize in module {}", module_path!());
    };
}

mkfn!{
    test_predicate_from_bitsize_introspect!();
    #[test] fn test_predicate_from_bitsize () { let pg = VectorType :: make_predicate_from_bitsize (32) ; assert_eq ! (pg . acle_notation_repr () , "b32") ; assert_eq ! (pg , "svbool4_t" . parse () . unwrap ()) ; assert_eq ! (pg . lanes , 4) ; assert_eq ! (pg . base_type , BaseType :: Sized (BaseTypeKind :: Bool , 32)) ; }
}

macro_rules! test_scalable_single_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_scalable_single in module {}", module_path!());
    };
}

mkfn!{
    test_scalable_single_introspect!();
    #[test] fn test_scalable_single () { assert_eq ! ("svuint8_t" . parse ::< TypeKind > () . unwrap () , TypeKind :: Vector (VectorType { base_type : BaseType :: Sized (BaseTypeKind :: UInt , 8) , is_scalable : true , lanes : 16 , tuple_size : None })) ; }
}

macro_rules! test_scalable_tuple_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_scalable_tuple in module {}", module_path!());
    };
}

mkfn!{
    test_scalable_tuple_introspect!();
    #[test] fn test_scalable_tuple () { assert_eq ! ("svint64x3_t" . parse ::< TypeKind > () . unwrap () , TypeKind :: Vector (VectorType { base_type : BaseType :: Sized (BaseTypeKind :: Int , 64) , is_scalable : true , lanes : 2 , tuple_size : Some (VectorTupleSize :: Three) , })) ; }
}

macro_rules! test_scalable_single_llvm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_scalable_single_llvm in module {}", module_path!());
    };
}

mkfn!{
    test_scalable_single_llvm_introspect!();
    #[test] fn test_scalable_single_llvm () { assert_eq ! ("svuint32_t" . parse ::< TypeKind > () . unwrap () . llvm_machine_repr () , "nxv4i32") ; }
}

macro_rules! test_scalable_tuple_llvm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_scalable_tuple_llvm in module {}", module_path!());
    };
}

mkfn!{
    test_scalable_tuple_llvm_introspect!();
    #[test] fn test_scalable_tuple_llvm () { assert_eq ! ("svint32x4_t" . parse ::< TypeKind > () . unwrap () . llvm_machine_repr () , "nxv16i32") ; }
}

macro_rules! test_vector_single_full_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vector_single_full in module {}", module_path!());
    };
}

mkfn!{
    test_vector_single_full_introspect!();
    #[test] fn test_vector_single_full () { assert_eq ! ("uint32x4_t" . parse ::< TypeKind > () . unwrap () , TypeKind :: Vector (VectorType { base_type : BaseType :: Sized (BaseTypeKind :: UInt , 32) , is_scalable : false , lanes : 4 , tuple_size : None , })) ; }
}

macro_rules! test_vector_single_half_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vector_single_half in module {}", module_path!());
    };
}

mkfn!{
    test_vector_single_half_introspect!();
    #[test] fn test_vector_single_half () { assert_eq ! ("uint32x2_t" . parse ::< TypeKind > () . unwrap () , TypeKind :: Vector (VectorType { base_type : BaseType :: Sized (BaseTypeKind :: UInt , 32) , is_scalable : false , lanes : 2 , tuple_size : None , })) ; }
}

macro_rules! test_vector_tuple_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vector_tuple in module {}", module_path!());
    };
}

mkfn!{
    test_vector_tuple_introspect!();
    #[test] fn test_vector_tuple () { assert_eq ! ("uint64x2x4_t" . parse ::< TypeKind > () . unwrap () , TypeKind :: Vector (VectorType { base_type : BaseType :: Sized (BaseTypeKind :: UInt , 64) , is_scalable : false , lanes : 2 , tuple_size : Some (VectorTupleSize :: Four) , })) ; }
}

macro_rules! test_const_pointer_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_const_pointer in module {}", module_path!());
    };
}

mkfn!{
    test_const_pointer_introspect!();
    #[test] fn test_const_pointer () { let p = "*u32" . parse :: < TypeKind > () . unwrap () ; assert_eq ! (p , TypeKind :: Pointer (Box :: new (TypeKind :: Base (BaseType :: Sized (BaseTypeKind :: UInt , 32))) , AccessLevel :: R)) ; assert_eq ! (p . to_token_stream () . to_string () , "* const u32") }
}

macro_rules! test_mut_pointer_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mut_pointer in module {}", module_path!());
    };
}

mkfn!{
    test_mut_pointer_introspect!();
    #[test] fn test_mut_pointer () { let p = "*mut u32" . parse :: < TypeKind > () . unwrap () ; assert_eq ! (p , TypeKind :: Pointer (Box :: new (TypeKind :: Base (BaseType :: Sized (BaseTypeKind :: UInt , 32))) , AccessLevel :: RW)) ; assert_eq ! (p . to_token_stream () . to_string () , "* mut u32") }
}

macro_rules! test_invalid_vector_single_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_invalid_vector_single in module {}", module_path!());
    };
}

mkfn!{
    test_invalid_vector_single_introspect!();
    #[test] #[should_panic] fn test_invalid_vector_single () { assert_eq ! ("uint32x8_t" . parse ::< TypeKind > () . unwrap () , TypeKind :: Vector (VectorType { base_type : BaseType :: Sized (BaseTypeKind :: UInt , 32) , is_scalable : false , lanes : 8 , tuple_size : None , })) ; }
}

macro_rules! test_invalid_vector_tuple_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_invalid_vector_tuple in module {}", module_path!());
    };
}

mkfn!{
    test_invalid_vector_tuple_introspect!();
    #[test] #[should_panic] fn test_invalid_vector_tuple () { assert_eq ! ("uint32x4x5_t" . parse ::< TypeKind > () . unwrap () , TypeKind :: Vector (VectorType { base_type : BaseType :: Sized (BaseTypeKind :: UInt , 32) , is_scalable : false , lanes : 8 , tuple_size : None , })) ; }
}

macro_rules! test_base_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_base in module {}", module_path!());
    };
}

mkfn!{
    test_base_introspect!();
    #[test] fn test_base () { assert_eq ! ("u32" . parse ::< TypeKind > () . unwrap () , TypeKind :: Base (BaseType :: Sized (BaseTypeKind :: UInt , 32)) ,) }
}

macro_rules! test_custom_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_custom in module {}", module_path!());
    };
}

mkfn!{
    test_custom_introspect!();
    #[test] fn test_custom () { assert_eq ! ("svpattern" . parse ::< TypeKind > () . unwrap () , TypeKind :: Custom ("svpattern" . to_string ()) ,) }
}

macro_rules! test_wildcard_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_wildcard_type in module {}", module_path!());
    };
}

mkfn!{
    test_wildcard_type_introspect!();
    #[test] fn test_wildcard_type () { assert_eq ! ("{type}" . parse ::< TypeKind > () . unwrap () , TypeKind :: Wildcard (Wildcard :: Type (None)) ,) }
}

macro_rules! test_wildcard_typeset_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_wildcard_typeset in module {}", module_path!());
    };
}

mkfn!{
    test_wildcard_typeset_introspect!();
    #[test] fn test_wildcard_typeset () { assert_eq ! ("{type[0]}" . parse ::< TypeKind > () . unwrap () , TypeKind :: Wildcard (Wildcard :: Type (Some (0))) ,) }
}

macro_rules! test_wildcard_sve_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_wildcard_sve_type in module {}", module_path!());
    };
}

mkfn!{
    test_wildcard_sve_type_introspect!();
    #[test] fn test_wildcard_sve_type () { assert_eq ! ("{sve_type}" . parse ::< TypeKind > () . unwrap () , TypeKind :: Wildcard (Wildcard :: SVEType (None , None)) ,) }
}

macro_rules! test_wildcard_sve_typeset_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_wildcard_sve_typeset in module {}", module_path!());
    };
}

mkfn!{
    test_wildcard_sve_typeset_introspect!();
    #[test] fn test_wildcard_sve_typeset () { assert_eq ! ("{sve_type[0]}" . parse ::< TypeKind > () . unwrap () , TypeKind :: Wildcard (Wildcard :: SVEType (Some (0) , None)) ,) }
}

macro_rules! test_wildcard_sve_tuple_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_wildcard_sve_tuple_type in module {}", module_path!());
    };
}

mkfn!{
    test_wildcard_sve_tuple_type_introspect!();
    #[test] fn test_wildcard_sve_tuple_type () { assert_eq ! ("{sve_type_x2}" . parse ::< TypeKind > () . unwrap () , TypeKind :: Wildcard (Wildcard :: SVEType (None , Some (VectorTupleSize :: Two))) ,) }
}

macro_rules! test_wildcard_sve_tuple_typeset_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_wildcard_sve_tuple_typeset in module {}", module_path!());
    };
}

mkfn!{
    test_wildcard_sve_tuple_typeset_introspect!();
    #[test] fn test_wildcard_sve_tuple_typeset () { assert_eq ! ("{sve_type_x2[0]}" . parse ::< TypeKind > () . unwrap () , TypeKind :: Wildcard (Wildcard :: SVEType (Some (0) , Some (VectorTupleSize :: Two))) ,) }
}

macro_rules! test_wildcard_predicate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_wildcard_predicate in module {}", module_path!());
    };
}

mkfn!{
    test_wildcard_predicate_introspect!();
    #[test] fn test_wildcard_predicate () { assert_eq ! ("{predicate}" . parse ::< TypeKind > () . unwrap () , TypeKind :: Wildcard (Wildcard :: Predicate (None))) }
}

macro_rules! test_wildcard_scale_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_wildcard_scale in module {}", module_path!());
    };
}

mkfn!{
    test_wildcard_scale_introspect!();
    #[test] fn test_wildcard_scale () { assert_eq ! ("{sve_type as i8}" . parse ::< TypeKind > () . unwrap () , TypeKind :: Wildcard (Wildcard :: Scale (Box :: new (Wildcard :: SVEType (None , None)) , Box :: new (TypeKind :: Base (BaseType :: Sized (BaseTypeKind :: Int , 8)))))) }
}

macro_rules! test_size_in_bytes_log2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_size_in_bytes_log2 in module {}", module_path!());
    };
}

mkfn!{
    test_size_in_bytes_log2_introspect!();
    #[test] fn test_size_in_bytes_log2 () { assert_eq ! ("i8" . parse ::< TypeKind > () . unwrap () . size_in_bytes_log2 () , "0") ; assert_eq ! ("i16" . parse ::< TypeKind > () . unwrap () . size_in_bytes_log2 () , "1") ; assert_eq ! ("i32" . parse ::< TypeKind > () . unwrap () . size_in_bytes_log2 () , "2") ; assert_eq ! ("i64" . parse ::< TypeKind > () . unwrap () . size_in_bytes_log2 () , "3") }
}

macro_rules! test_invalid_size_in_bytes_log2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_invalid_size_in_bytes_log2 in module {}", module_path!());
    };
}

mkfn!{
    test_invalid_size_in_bytes_log2_introspect!();
    #[test] #[should_panic] fn test_invalid_size_in_bytes_log2 () { "i9" . parse :: < TypeKind > () . unwrap () . size_in_bytes_log2 () ; }
} 
            }}