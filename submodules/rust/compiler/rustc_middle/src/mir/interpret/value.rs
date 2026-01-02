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
mkuse!{use std :: fmt ;}
mkuse!{use either :: { Either , Left , Right } ;}
mkuse!{use rustc_abi :: { HasDataLayout , Size } ;}
mkuse!{use rustc_apfloat :: Float ;}
mkuse!{use rustc_apfloat :: ieee :: { Double , Half , Quad , Single } ;}
mkuse!{use rustc_macros :: { HashStable , TyDecodable , TyEncodable } ;}
mkuse!{use super :: { AllocId , CtfeProvenance , InterpResult , Pointer , PointerArithmetic , Provenance , ScalarSizeMismatch , interp_ok , } ;}
mkuse!{use crate :: ty :: ScalarInt ;}
mkitem!{mkenum!{#[doc = " A `Scalar` represents an immediate, primitive value existing outside of a"] #[doc = " `memory::Allocation`. It is in many ways like a small chunk of an `Allocation`, up to 16 bytes in"] #[doc = " size. Like a range of bytes in an `Allocation`, a `Scalar` can either represent the raw bytes"] #[doc = " of a simple value or a pointer into another `Allocation`"] #[doc = ""] #[doc = " These variants would be private if there was a convenient way to achieve that in Rust."] #[doc = " Do *not* match on a `Scalar`! Use the various `to_*` methods instead."] #[derive (Clone , Copy , Eq , PartialEq , TyEncodable , TyDecodable , Hash)] #[derive (HashStable)] pub enum Scalar < Prov = CtfeProvenance > { #[doc = " The raw bytes of a simple value."] Int (ScalarInt) , #[doc = " A pointer."] #[doc = ""] #[doc = " We also store the size of the pointer, such that a `Scalar` always knows how big it is."] #[doc = " The size is always the pointer size of the current target, but this is not information"] #[doc = " that we always have readily available."] Ptr (Pointer < Prov > , u8) , }}}
mkitem!{#[cfg (target_pointer_width = "64")] rustc_data_structures :: static_assert_size ! (Scalar , 24) ;}
mkitem!{mkimpl!{impl < Prov : Provenance > fmt :: Debug for Scalar < Prov > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Scalar :: Ptr (ptr , _size) => write ! (f , "{ptr:?}") , Scalar :: Int (int) => write ! (f , "{int:?}") , } } }}}
mkitem!{mkimpl!{impl < Prov : Provenance > fmt :: Display for Scalar < Prov > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Scalar :: Ptr (ptr , _size) => write ! (f , "pointer to {ptr:?}") , Scalar :: Int (int) => write ! (f , "{int}") , } } }}}
mkitem!{mkimpl!{impl < Prov : Provenance > fmt :: LowerHex for Scalar < Prov > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Scalar :: Ptr (ptr , _size) => write ! (f , "pointer to {ptr:?}") , Scalar :: Int (int) => write ! (f , "{int:#x}") , } } }}}
mkitem!{mkimpl!{impl < Prov > From < Half > for Scalar < Prov > { #[inline (always)] fn from (f : Half) -> Self { Scalar :: from_f16 (f) } }}}
mkitem!{mkimpl!{impl < Prov > From < Single > for Scalar < Prov > { #[inline (always)] fn from (f : Single) -> Self { Scalar :: from_f32 (f) } }}}
mkitem!{mkimpl!{impl < Prov > From < Double > for Scalar < Prov > { #[inline (always)] fn from (f : Double) -> Self { Scalar :: from_f64 (f) } }}}
mkitem!{mkimpl!{impl < Prov > From < Quad > for Scalar < Prov > { #[inline (always)] fn from (f : Quad) -> Self { Scalar :: from_f128 (f) } }}}
mkitem!{mkimpl!{impl < Prov > From < ScalarInt > for Scalar < Prov > { #[inline (always)] fn from (ptr : ScalarInt) -> Self { Scalar :: Int (ptr) } }}}
mkitem!{mkimpl!{impl < Prov > Scalar < Prov > { #[inline (always)] pub fn from_pointer (ptr : Pointer < Prov > , cx : & impl HasDataLayout) -> Self { Scalar :: Ptr (ptr , u8 :: try_from (cx . pointer_size () . bytes ()) . unwrap ()) } #[doc = " Create a Scalar from a pointer with an `Option<_>` provenance (where `None` represents a"] #[doc = " plain integer / \"invalid\" pointer)."] pub fn from_maybe_pointer (ptr : Pointer < Option < Prov > > , cx : & impl HasDataLayout) -> Self { match ptr . into_raw_parts () { (Some (prov) , offset) => Scalar :: from_pointer (Pointer :: new (prov , offset) , cx) , (None , offset) => { Scalar :: Int (ScalarInt :: try_from_uint (offset . bytes () , cx . pointer_size ()) . unwrap ()) } } } #[inline] pub fn null_ptr (cx : & impl HasDataLayout) -> Self { Scalar :: Int (ScalarInt :: null (cx . pointer_size ())) } #[inline] pub fn from_bool (b : bool) -> Self { Scalar :: Int (b . into ()) } #[inline] pub fn from_char (c : char) -> Self { Scalar :: Int (c . into ()) } #[inline] pub fn from_uint (i : impl Into < u128 > , size : Size) -> Self { let i = i . into () ; ScalarInt :: try_from_uint (i , size) . unwrap_or_else (| | bug ! ("Unsigned value {:#x} does not fit in {} bits" , i , size . bits ())) . into () } #[inline] pub fn from_u8 (i : u8) -> Self { Scalar :: Int (i . into ()) } #[inline] pub fn from_u16 (i : u16) -> Self { Scalar :: Int (i . into ()) } #[inline] pub fn from_u32 (i : u32) -> Self { Scalar :: Int (i . into ()) } #[inline] pub fn from_u64 (i : u64) -> Self { Scalar :: Int (i . into ()) } #[inline] pub fn from_u128 (i : u128) -> Self { Scalar :: Int (i . into ()) } #[inline] pub fn from_target_usize (i : u64 , cx : & impl HasDataLayout) -> Self { Self :: from_uint (i , cx . data_layout () . pointer_offset ()) } #[inline] pub fn from_int (i : impl Into < i128 > , size : Size) -> Self { let i = i . into () ; ScalarInt :: try_from_int (i , size) . unwrap_or_else (| | bug ! ("Signed value {:#x} does not fit in {} bits" , i , size . bits ())) . into () } #[inline] pub fn from_i8 (i : i8) -> Self { Self :: Int (i . into ()) } #[inline] pub fn from_i16 (i : i16) -> Self { Self :: Int (i . into ()) } #[inline] pub fn from_i32 (i : i32) -> Self { Self :: Int (i . into ()) } #[inline] pub fn from_i64 (i : i64) -> Self { Self :: Int (i . into ()) } #[inline] pub fn from_i128 (i : i128) -> Self { Self :: Int (i . into ()) } #[inline] pub fn from_target_isize (i : i64 , cx : & impl HasDataLayout) -> Self { Self :: from_int (i , cx . data_layout () . pointer_offset ()) } #[inline] pub fn from_f16 (f : Half) -> Self { Scalar :: Int (f . into ()) } #[inline] pub fn from_f32 (f : Single) -> Self { Scalar :: Int (f . into ()) } #[inline] pub fn from_f64 (f : Double) -> Self { Scalar :: Int (f . into ()) } #[inline] pub fn from_f128 (f : Quad) -> Self { Scalar :: Int (f . into ()) } #[doc = " This is almost certainly not the method you want!  You should dispatch on the type"] #[doc = " and use `to_{u8,u16,...}`/`to_pointer` to perform ptr-to-int / int-to-ptr casts as needed."] #[doc = ""] #[doc = " This method only exists for the benefit of low-level operations that truly need to treat the"] #[doc = " scalar in whatever form it is."] #[doc = ""] #[doc = " This throws UB (instead of ICEing) on a size mismatch since size mismatches can arise in"] #[doc = " Miri when someone declares a function that we shim (such as `malloc`) with a wrong type."] #[inline] pub fn to_bits_or_ptr_internal (self , target_size : Size ,) -> Result < Either < u128 , Pointer < Prov > > , ScalarSizeMismatch > { assert_ne ! (target_size . bytes () , 0 , "you should never look at the bits of a ZST") ; Ok (match self { Scalar :: Int (int) => Left (int . try_to_bits (target_size) . map_err (| size | { ScalarSizeMismatch { target_size : target_size . bytes () , data_size : size . bytes () } }) ?) , Scalar :: Ptr (ptr , sz) => { if target_size . bytes () != u64 :: from (sz) { return Err (ScalarSizeMismatch { target_size : target_size . bytes () , data_size : sz . into () , }) ; } Right (ptr) } }) } #[inline] pub fn size (self) -> Size { match self { Scalar :: Int (int) => int . size () , Scalar :: Ptr (_ptr , sz) => Size :: from_bytes (sz) , } } }}}
mkitem!{mkimpl!{impl < 'tcx , Prov : Provenance > Scalar < Prov > { pub fn to_pointer (self , cx : & impl HasDataLayout) -> InterpResult < 'tcx , Pointer < Option < Prov > > > { match self . to_bits_or_ptr_internal (cx . pointer_size ()) . map_err (| s | err_ub ! (ScalarSizeMismatch (s))) ? { Right (ptr) => interp_ok (ptr . into ()) , Left (bits) => { let addr = u64 :: try_from (bits) . unwrap () ; interp_ok (Pointer :: without_provenance (addr)) } } } #[doc = " Fundamental scalar-to-int (cast) operation. Many convenience wrappers exist below, that you"] #[doc = " likely want to use instead."] #[doc = ""] #[doc = " Will perform ptr-to-int casts if needed and possible."] #[doc = " If that fails, we know the offset is relative, so we return an \"erased\" Scalar"] #[doc = " (which is useful for error messages but not much else)."] #[doc = ""] #[doc = " The error type is `AllocId`, not `CtfeProvenance`, since `AllocId` is the \"minimal\""] #[doc = " component all provenance types must have."] #[inline] pub fn try_to_scalar_int (self) -> Result < ScalarInt , Scalar < AllocId > > { match self { Scalar :: Int (int) => Ok (int) , Scalar :: Ptr (ptr , sz) => { if Prov :: OFFSET_IS_ADDR { Ok (ScalarInt :: try_from_uint (ptr . offset . bytes () , Size :: from_bytes (sz)) . unwrap ()) } else { let (prov , offset) = ptr . into_raw_parts () ; Err (Scalar :: Ptr (Pointer :: new (prov . get_alloc_id () . unwrap () , offset) , sz)) } } } } pub fn clear_provenance (& mut self) -> InterpResult < 'tcx > { if matches ! (self , Scalar :: Ptr (..)) { * self = self . to_scalar_int () ? . into () ; } interp_ok (()) } #[inline (always)] pub fn to_scalar_int (self) -> InterpResult < 'tcx , ScalarInt > { self . try_to_scalar_int () . map_err (| _ | err_unsup ! (ReadPointerAsInt (None))) . into () } #[inline (always)] #[cfg_attr (debug_assertions , track_caller)] pub fn assert_scalar_int (self) -> ScalarInt { self . try_to_scalar_int () . expect ("got a pointer where a ScalarInt was expected") } #[doc = " This throws UB (instead of ICEing) on a size mismatch since size mismatches can arise in"] #[doc = " Miri when someone declares a function that we shim (such as `malloc`) with a wrong type."] #[inline] pub fn to_bits (self , target_size : Size) -> InterpResult < 'tcx , u128 > { assert_ne ! (target_size . bytes () , 0 , "you should never look at the bits of a ZST") ; self . to_scalar_int () ? . try_to_bits (target_size) . map_err (| size | { err_ub ! (ScalarSizeMismatch (ScalarSizeMismatch { target_size : target_size . bytes () , data_size : size . bytes () , })) }) . into () } pub fn to_bool (self) -> InterpResult < 'tcx , bool > { let val = self . to_u8 () ? ; match val { 0 => interp_ok (false) , 1 => interp_ok (true) , _ => throw_ub ! (InvalidBool (val)) , } } pub fn to_char (self) -> InterpResult < 'tcx , char > { let val = self . to_u32 () ? ; match std :: char :: from_u32 (val) { Some (c) => interp_ok (c) , None => throw_ub ! (InvalidChar (val)) , } } #[doc = " Converts the scalar to produce an unsigned integer of the given size."] #[doc = " Fails if the scalar is a pointer."] #[inline] pub fn to_uint (self , size : Size) -> InterpResult < 'tcx , u128 > { self . to_bits (size) } #[doc = " Converts the scalar to produce a `u8`. Fails if the scalar is a pointer."] pub fn to_u8 (self) -> InterpResult < 'tcx , u8 > { self . to_uint (Size :: from_bits (8)) . map (| v | u8 :: try_from (v) . unwrap ()) } #[doc = " Converts the scalar to produce a `u16`. Fails if the scalar is a pointer."] pub fn to_u16 (self) -> InterpResult < 'tcx , u16 > { self . to_uint (Size :: from_bits (16)) . map (| v | u16 :: try_from (v) . unwrap ()) } #[doc = " Converts the scalar to produce a `u32`. Fails if the scalar is a pointer."] pub fn to_u32 (self) -> InterpResult < 'tcx , u32 > { self . to_uint (Size :: from_bits (32)) . map (| v | u32 :: try_from (v) . unwrap ()) } #[doc = " Converts the scalar to produce a `u64`. Fails if the scalar is a pointer."] pub fn to_u64 (self) -> InterpResult < 'tcx , u64 > { self . to_uint (Size :: from_bits (64)) . map (| v | u64 :: try_from (v) . unwrap ()) } #[doc = " Converts the scalar to produce a `u128`. Fails if the scalar is a pointer."] pub fn to_u128 (self) -> InterpResult < 'tcx , u128 > { self . to_uint (Size :: from_bits (128)) } #[doc = " Converts the scalar to produce a machine-pointer-sized unsigned integer."] #[doc = " Fails if the scalar is a pointer."] pub fn to_target_usize (self , cx : & impl HasDataLayout) -> InterpResult < 'tcx , u64 > { let b = self . to_uint (cx . data_layout () . pointer_size ()) ? ; interp_ok (u64 :: try_from (b) . unwrap ()) } #[doc = " Converts the scalar to produce a signed integer of the given size."] #[doc = " Fails if the scalar is a pointer."] #[inline] pub fn to_int (self , size : Size) -> InterpResult < 'tcx , i128 > { let b = self . to_bits (size) ? ; interp_ok (size . sign_extend (b)) } #[doc = " Converts the scalar to produce an `i8`. Fails if the scalar is a pointer."] pub fn to_i8 (self) -> InterpResult < 'tcx , i8 > { self . to_int (Size :: from_bits (8)) . map (| v | i8 :: try_from (v) . unwrap ()) } #[doc = " Converts the scalar to produce an `i16`. Fails if the scalar is a pointer."] pub fn to_i16 (self) -> InterpResult < 'tcx , i16 > { self . to_int (Size :: from_bits (16)) . map (| v | i16 :: try_from (v) . unwrap ()) } #[doc = " Converts the scalar to produce an `i32`. Fails if the scalar is a pointer."] pub fn to_i32 (self) -> InterpResult < 'tcx , i32 > { self . to_int (Size :: from_bits (32)) . map (| v | i32 :: try_from (v) . unwrap ()) } #[doc = " Converts the scalar to produce an `i64`. Fails if the scalar is a pointer."] pub fn to_i64 (self) -> InterpResult < 'tcx , i64 > { self . to_int (Size :: from_bits (64)) . map (| v | i64 :: try_from (v) . unwrap ()) } #[doc = " Converts the scalar to produce an `i128`. Fails if the scalar is a pointer."] pub fn to_i128 (self) -> InterpResult < 'tcx , i128 > { self . to_int (Size :: from_bits (128)) } #[doc = " Converts the scalar to produce a machine-pointer-sized signed integer."] #[doc = " Fails if the scalar is a pointer."] pub fn to_target_isize (self , cx : & impl HasDataLayout) -> InterpResult < 'tcx , i64 > { let b = self . to_int (cx . data_layout () . pointer_size ()) ? ; interp_ok (i64 :: try_from (b) . unwrap ()) } #[inline] pub fn to_float < F : Float > (self) -> InterpResult < 'tcx , F > { interp_ok (F :: from_bits (self . to_bits (Size :: from_bits (F :: BITS)) ?)) } #[inline] pub fn to_f16 (self) -> InterpResult < 'tcx , Half > { self . to_float () } #[inline] pub fn to_f32 (self) -> InterpResult < 'tcx , Single > { self . to_float () } #[inline] pub fn to_f64 (self) -> InterpResult < 'tcx , Double > { self . to_float () } #[inline] pub fn to_f128 (self) -> InterpResult < 'tcx , Quad > { self . to_float () } }}}