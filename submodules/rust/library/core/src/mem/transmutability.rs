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
mkuse!{use crate :: marker :: { ConstParamTy_ , UnsizedConstParamTy } ;}
mkitem!{mktrait!{#[doc = " Marks that `Src` is transmutable into `Self`."] #[doc = ""] #[doc = " # Implementation"] #[doc = ""] #[doc = " This trait cannot be implemented explicitly. It is implemented on-the-fly by"] #[doc = " the compiler for all types `Src` and `Self` such that, given a set of safety"] #[doc = " obligations on the programmer (see [`Assume`]), the compiler has proved that"] #[doc = " the bits of a value of type `Src` can be soundly reinterpreted as a `Self`."] #[doc = ""] #[doc = " # Safety"] #[doc = ""] #[doc = " If `Dst: TransmuteFrom<Src, ASSUMPTIONS>`, the compiler guarantees that"] #[doc = " `Src` is soundly *union-transmutable* into a value of type `Dst`, provided"] #[doc = " that the programmer has guaranteed that the given [`ASSUMPTIONS`](Assume)"] #[doc = " are satisfied."] #[doc = ""] #[doc = " A union-transmute is any bit-reinterpretation conversion in the form of:"] #[doc = ""] #[doc = " ```rust"] #[doc = " pub unsafe fn transmute_via_union<Src, Dst>(src: Src) -> Dst {"] #[doc = "     use core::mem::ManuallyDrop;"] #[doc = ""] #[doc = "     #[repr(C)]"] #[doc = "     union Transmute<Src, Dst> {"] #[doc = "         src: ManuallyDrop<Src>,"] #[doc = "         dst: ManuallyDrop<Dst>,"] #[doc = "     }"] #[doc = ""] #[doc = "     let transmute = Transmute {"] #[doc = "         src: ManuallyDrop::new(src),"] #[doc = "     };"] #[doc = ""] #[doc = "     let dst = unsafe { transmute.dst };"] #[doc = ""] #[doc = "     ManuallyDrop::into_inner(dst)"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " Note that this construction is more permissive than"] #[doc = " [`mem::transmute_copy`](super::transmute_copy); union-transmutes permit"] #[doc = " conversions that extend the bits of `Src` with trailing padding to fill"] #[doc = " trailing uninitialized bytes of `Self`; e.g.:"] #[doc = ""] #[doc = " ```rust"] #[doc = " #![feature(transmutability)]"] #[doc = ""] #[doc = " use core::mem::{Assume, TransmuteFrom};"] #[doc = ""] #[doc = " let src = 42u8; // size = 1"] #[doc = ""] #[doc = " #[repr(C, align(2))]"] #[doc = " struct Dst(u8); // size = 2"] #[doc = " let _ = unsafe {"] #[doc = "     <Dst as TransmuteFrom<u8, { Assume::SAFETY }>>::transmute(src)"] #[doc = " };"] #[doc = " ```"] #[doc = ""] #[doc = " # Caveats"] #[doc = ""] #[doc = " ## Portability"] #[doc = ""] #[doc = " Implementations of this trait do not provide any guarantee of portability"] #[doc = " across toolchains, targets or compilations. This trait may be implemented"] #[doc = " for certain combinations of `Src`, `Self` and `ASSUME` on some toolchains,"] #[doc = " targets or compilations, but not others. For example, if the layouts of"] #[doc = " `Src` or `Self` are non-deterministic, the presence or absence of an"] #[doc = " implementation of this trait may also be non-deterministic. Even if `Src`"] #[doc = " and `Self` have deterministic layouts (e.g., they are `repr(C)` structs),"] #[doc = " Rust does not specify the alignments of its primitive integer types, and"] #[doc = " layouts that involve these types may vary across toolchains, targets or"] #[doc = " compilations."] #[doc = ""] #[doc = " ## Stability"] #[doc = ""] #[doc = " Implementations of this trait do not provide any guarantee of SemVer"] #[doc = " stability across the crate versions that define the `Src` and `Self` types."] #[doc = " If SemVer stability is crucial to your application, you must consult the"] #[doc = " documentation of `Src` and `Self`s' defining crates. Note that the presence"] #[doc = " of `repr(C)`, alone, does not carry a safety invariant of SemVer stability."] #[doc = " Furthermore, stability does not imply portability. For example, the size of"] #[doc = " `usize` is stable, but not portable."] #[unstable (feature = "transmutability" , issue = "99571")] #[lang = "transmute_trait"] #[rustc_deny_explicit_impl] #[rustc_do_not_implement_via_object] #[rustc_coinductive] pub unsafe trait TransmuteFrom < Src , const ASSUME : Assume = { Assume :: NOTHING } > where Src : ? Sized , { #[doc = " Transmutes a `Src` value into a `Self`."] #[doc = ""] #[doc = " # Safety"] #[doc = ""] #[doc = " The safety obligations of the caller depend on the value of `ASSUME`:"] #[doc = " - If [`ASSUME.alignment`](Assume::alignment), the caller must guarantee"] #[doc = "   that the addresses of references in the returned `Self` satisfy the"] #[doc = "   alignment requirements of their referent types."] #[doc = " - If [`ASSUME.lifetimes`](Assume::lifetimes), the caller must guarantee"] #[doc = "   that references in the returned `Self` will not outlive their"] #[doc = "   referents."] #[doc = " - If [`ASSUME.safety`](Assume::safety), the returned value might not"] #[doc = "   satisfy the library safety invariants of `Self`, and the caller must"] #[doc = "   guarantee that undefined behavior does not arise from uses of the"] #[doc = "   returned value."] #[doc = " - If [`ASSUME.validity`](Assume::validity), the caller must guarantee"] #[doc = "   that `src` is a bit-valid instance of `Self`."] #[doc = ""] #[doc = " When satisfying the above obligations (if any), the caller must *not*"] #[doc = " assume that this trait provides any inherent guarantee of layout"] #[doc = " [portability](#portability) or [stability](#stability)."] unsafe fn transmute (src : Src) -> Self where Src : Sized , Self : Sized , { use super :: ManuallyDrop ; #[repr (C)] union Transmute < Src , Dst > { src : ManuallyDrop < Src > , dst : ManuallyDrop < Dst > , } let transmute = Transmute { src : ManuallyDrop :: new (src) } ; let dst = unsafe { transmute . dst } ; ManuallyDrop :: into_inner (dst) } }}}
mkitem!{mkstruct!{#[doc = " Configurable proof assumptions of [`TransmuteFrom`]."] #[doc = ""] #[doc = " When `false`, the respective proof obligation belongs to the compiler. When"] #[doc = " `true`, the onus of the safety proof belongs to the programmer."] #[unstable (feature = "transmutability" , issue = "99571")] #[lang = "transmute_opts"] #[derive (PartialEq , Eq , Clone , Copy , Debug)] pub struct Assume { #[doc = " When `false`, [`TransmuteFrom`] is not implemented for transmutations"] #[doc = " that might violate the alignment requirements of references; e.g.:"] #[doc = ""] #[doc = " ```compile_fail,E0277"] #[doc = " #![feature(transmutability)]"] #[doc = " use core::mem::TransmuteFrom;"] #[doc = ""] #[doc = " assert_eq!(align_of::<[u8; 2]>(), 1);"] #[doc = " assert_eq!(align_of::<u16>(), 2);"] #[doc = ""] #[doc = " let src: &[u8; 2] = &[0xFF, 0xFF];"] #[doc = ""] #[doc = " // SAFETY: No safety obligations."] #[doc = " let dst: &u16 = unsafe {"] #[doc = "     <_ as TransmuteFrom<_>>::transmute(src)"] #[doc = " };"] #[doc = " ```"] #[doc = ""] #[doc = " When `true`, [`TransmuteFrom`] assumes that *you* have ensured"] #[doc = " that references in the transmuted value satisfy the alignment"] #[doc = " requirements of their referent types; e.g.:"] #[doc = ""] #[doc = " ```rust"] #[doc = " #![feature(pointer_is_aligned_to, transmutability)]"] #[doc = " use core::mem::{Assume, TransmuteFrom};"] #[doc = ""] #[doc = " let src: &[u8; 2] = &[0xFF, 0xFF];"] #[doc = ""] #[doc = " let maybe_dst: Option<&u16> = if <*const _>::is_aligned_to(src, align_of::<u16>()) {"] #[doc = "     // SAFETY: We have checked above that the address of `src` satisfies the"] #[doc = "     // alignment requirements of `u16`."] #[doc = "     Some(unsafe {"] #[doc = "         <_ as TransmuteFrom<_, { Assume::ALIGNMENT }>>::transmute(src)"] #[doc = "     })"] #[doc = " } else {"] #[doc = "     None"] #[doc = " };"] #[doc = ""] #[doc = " assert!(matches!(maybe_dst, Some(&u16::MAX) | None));"] #[doc = " ```"] pub alignment : bool , #[doc = " When `false`, [`TransmuteFrom`] is not implemented for transmutations"] #[doc = " that extend the lifetimes of references."] #[doc = ""] #[doc = " When `true`, [`TransmuteFrom`] assumes that *you* have ensured that"] #[doc = " references in the transmuted value do not outlive their referents."] pub lifetimes : bool , #[doc = " When `false`, [`TransmuteFrom`] is not implemented for transmutations"] #[doc = " that might violate the library safety invariants of the destination"] #[doc = " type; e.g.:"] #[doc = ""] #[doc = " ```compile_fail,E0277"] #[doc = " #![feature(transmutability)]"] #[doc = " use core::mem::TransmuteFrom;"] #[doc = ""] #[doc = " let src: u8 = 3;"] #[doc = ""] #[doc = " struct EvenU8 {"] #[doc = "     // SAFETY: `val` must be an even number."] #[doc = "     val: u8,"] #[doc = " }"] #[doc = ""] #[doc = " // SAFETY: No safety obligations."] #[doc = " let dst: EvenU8 = unsafe {"] #[doc = "     <_ as TransmuteFrom<_>>::transmute(src)"] #[doc = " };"] #[doc = " ```"] #[doc = ""] #[doc = " When `true`, [`TransmuteFrom`] assumes that *you* have ensured"] #[doc = " that undefined behavior does not arise from using the transmuted value;"] #[doc = " e.g.:"] #[doc = ""] #[doc = " ```rust"] #[doc = " #![feature(transmutability)]"] #[doc = " use core::mem::{Assume, TransmuteFrom};"] #[doc = ""] #[doc = " let src: u8 = 42;"] #[doc = ""] #[doc = " struct EvenU8 {"] #[doc = "     // SAFETY: `val` must be an even number."] #[doc = "     val: u8,"] #[doc = " }"] #[doc = ""] #[doc = " let maybe_dst: Option<EvenU8> = if src % 2 == 0 {"] #[doc = "     // SAFETY: We have checked above that the value of `src` is even."] #[doc = "     Some(unsafe {"] #[doc = "         <_ as TransmuteFrom<_, { Assume::SAFETY }>>::transmute(src)"] #[doc = "     })"] #[doc = " } else {"] #[doc = "     None"] #[doc = " };"] #[doc = ""] #[doc = " assert!(matches!(maybe_dst, Some(EvenU8 { val: 42 })));"] #[doc = " ```"] pub safety : bool , #[doc = " When `false`, [`TransmuteFrom`] is not implemented for transmutations"] #[doc = " that might violate the language-level bit-validity invariant of the"] #[doc = " destination type; e.g.:"] #[doc = ""] #[doc = " ```compile_fail,E0277"] #[doc = " #![feature(transmutability)]"] #[doc = " use core::mem::TransmuteFrom;"] #[doc = ""] #[doc = " let src: u8 = 3;"] #[doc = ""] #[doc = " // SAFETY: No safety obligations."] #[doc = " let dst: bool = unsafe {"] #[doc = "     <_ as TransmuteFrom<_>>::transmute(src)"] #[doc = " };"] #[doc = " ```"] #[doc = ""] #[doc = " When `true`, [`TransmuteFrom`] assumes that *you* have ensured"] #[doc = " that the value being transmuted is a bit-valid instance of the"] #[doc = " transmuted value; e.g.:"] #[doc = ""] #[doc = " ```rust"] #[doc = " #![feature(transmutability)]"] #[doc = " use core::mem::{Assume, TransmuteFrom};"] #[doc = ""] #[doc = " let src: u8 = 1;"] #[doc = ""] #[doc = " let maybe_dst: Option<bool> = if src == 0 || src == 1 {"] #[doc = "     // SAFETY: We have checked above that the value of `src` is a bit-valid"] #[doc = "     // instance of `bool`."] #[doc = "     Some(unsafe {"] #[doc = "         <_ as TransmuteFrom<_, { Assume::VALIDITY }>>::transmute(src)"] #[doc = "     })"] #[doc = " } else {"] #[doc = "     None"] #[doc = " };"] #[doc = ""] #[doc = " assert_eq!(maybe_dst, Some(true));"] #[doc = " ```"] pub validity : bool , }}}
mkitem!{mkimpl!{#[unstable (feature = "transmutability" , issue = "99571")] impl ConstParamTy_ for Assume { }}}
mkitem!{mkimpl!{#[unstable (feature = "transmutability" , issue = "99571")] impl UnsizedConstParamTy for Assume { }}}
mkitem!{mkimpl!{impl Assume { #[doc = " With this, [`TransmuteFrom`] does not assume you have ensured any safety"] #[doc = " obligations are met, and relies only upon its own analysis to (dis)prove"] #[doc = " transmutability."] #[unstable (feature = "transmutability" , issue = "99571")] pub const NOTHING : Self = Self { alignment : false , lifetimes : false , safety : false , validity : false } ; #[doc = " With this, [`TransmuteFrom`] assumes only that you have ensured that"] #[doc = " references in the transmuted value satisfy the alignment requirements of"] #[doc = " their referent types. See [`Assume::alignment`] for examples."] #[unstable (feature = "transmutability" , issue = "99571")] pub const ALIGNMENT : Self = Self { alignment : true , .. Self :: NOTHING } ; #[doc = " With this, [`TransmuteFrom`] assumes only that you have ensured that"] #[doc = " references in the transmuted value do not outlive their referents. See"] #[doc = " [`Assume::lifetimes`] for examples."] #[unstable (feature = "transmutability" , issue = "99571")] pub const LIFETIMES : Self = Self { lifetimes : true , .. Self :: NOTHING } ; #[doc = " With this, [`TransmuteFrom`] assumes only that you have ensured that"] #[doc = " undefined behavior does not arise from using the transmuted value. See"] #[doc = " [`Assume::safety`] for examples."] #[unstable (feature = "transmutability" , issue = "99571")] pub const SAFETY : Self = Self { safety : true , .. Self :: NOTHING } ; #[doc = " With this, [`TransmuteFrom`] assumes only that you have ensured that the"] #[doc = " value being transmuted is a bit-valid instance of the transmuted value."] #[doc = " See [`Assume::validity`] for examples."] #[unstable (feature = "transmutability" , issue = "99571")] pub const VALIDITY : Self = Self { validity : true , .. Self :: NOTHING } ; #[doc = " Combine the assumptions of `self` and `other_assumptions`."] #[doc = ""] #[doc = " This is especially useful for extending [`Assume`] in generic contexts;"] #[doc = " e.g.:"] #[doc = ""] #[doc = " ```rust"] #[doc = " #![feature("] #[doc = "     adt_const_params,"] #[doc = "     generic_const_exprs,"] #[doc = "     pointer_is_aligned_to,"] #[doc = "     transmutability,"] #[doc = " )]"] #[doc = " #![allow(incomplete_features)]"] #[doc = " use core::mem::{Assume, TransmuteFrom};"] #[doc = ""] #[doc = " /// Attempts to transmute `src` to `&Dst`."] #[doc = " ///"] #[doc = " /// Returns `None` if `src` violates the alignment requirements of `&Dst`."] #[doc = " ///"] #[doc = " /// # Safety"] #[doc = " ///"] #[doc = " /// The caller guarantees that the obligations required by `ASSUME`, except"] #[doc = " /// alignment, are satisfied."] #[doc = " unsafe fn try_transmute_ref<'a, Src, Dst, const ASSUME: Assume>(src: &'a Src) -> Option<&'a Dst>"] #[doc = " where"] #[doc = "     &'a Dst: TransmuteFrom<&'a Src, { ASSUME.and(Assume::ALIGNMENT) }>,"] #[doc = " {"] #[doc = "     if <*const _>::is_aligned_to(src, align_of::<Dst>()) {"] #[doc = "         // SAFETY: By the above dynamic check, we have ensured that the address"] #[doc = "         // of `src` satisfies the alignment requirements of `&Dst`. By contract"] #[doc = "         // on the caller, the safety obligations required by `ASSUME` have also"] #[doc = "         // been satisfied."] #[doc = "         Some(unsafe {"] #[doc = "             <_ as TransmuteFrom<_, { ASSUME.and(Assume::ALIGNMENT) }>>::transmute(src)"] #[doc = "         })"] #[doc = "     } else {"] #[doc = "         None"] #[doc = "     }"] #[doc = " }"] #[doc = ""] #[doc = " let src: &[u8; 2] = &[0xFF, 0xFF];"] #[doc = ""] #[doc = " // SAFETY: No safety obligations."] #[doc = " let maybe_dst: Option<&u16> = unsafe {"] #[doc = "     try_transmute_ref::<_, _, { Assume::NOTHING }>(src)"] #[doc = " };"] #[doc = "```"] #[unstable (feature = "transmutability" , issue = "99571")] pub const fn and (self , other_assumptions : Self) -> Self { Self { alignment : self . alignment || other_assumptions . alignment , lifetimes : self . lifetimes || other_assumptions . lifetimes , safety : self . safety || other_assumptions . safety , validity : self . validity || other_assumptions . validity , } } #[doc = " Remove `other_assumptions` the obligations of `self`; e.g.:"] #[doc = ""] #[doc = " ```rust"] #[doc = " #![feature(transmutability)]"] #[doc = " use core::mem::Assume;"] #[doc = ""] #[doc = " let assumptions = Assume::ALIGNMENT.and(Assume::SAFETY);"] #[doc = " let to_be_removed = Assume::SAFETY.and(Assume::VALIDITY);"] #[doc = ""] #[doc = " assert_eq!("] #[doc = "     assumptions.but_not(to_be_removed),"] #[doc = "     Assume::ALIGNMENT,"] #[doc = " );"] #[doc = " ```"] #[unstable (feature = "transmutability" , issue = "99571")] pub const fn but_not (self , other_assumptions : Self) -> Self { Self { alignment : self . alignment && ! other_assumptions . alignment , lifetimes : self . lifetimes && ! other_assumptions . lifetimes , safety : self . safety && ! other_assumptions . safety , validity : self . validity && ! other_assumptions . validity , } } }}}
mkitem!{mkimpl!{#[unstable (feature = "transmutability" , issue = "99571")] impl core :: ops :: Add for Assume { type Output = Assume ; fn add (self , other_assumptions : Assume) -> Assume { self . and (other_assumptions) } }}}
mkitem!{mkimpl!{#[unstable (feature = "transmutability" , issue = "99571")] impl core :: ops :: Sub for Assume { type Output = Assume ; fn sub (self , other_assumptions : Assume) -> Assume { self . but_not (other_assumptions) } }}}