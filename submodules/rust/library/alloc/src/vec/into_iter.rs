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
mkuse!{use core :: iter :: { FusedIterator , InPlaceIterable , SourceIter , TrustedFused , TrustedLen , TrustedRandomAccessNoCoerce , } ;}
mkuse!{use core :: marker :: PhantomData ;}
mkuse!{use core :: mem :: { ManuallyDrop , MaybeUninit , SizedTypeProperties } ;}
mkuse!{use core :: num :: NonZero ;}
mkuse!{# [cfg (not (no_global_oom_handling))] use core :: ops :: Deref ;}
mkuse!{use core :: ptr :: { self , NonNull } ;}
mkuse!{use core :: slice :: { self } ;}
mkuse!{use core :: { array , fmt } ;}
mkuse!{# [cfg (not (no_global_oom_handling))] use super :: AsVecIntoIter ;}
mkuse!{use crate :: alloc :: { Allocator , Global } ;}
mkuse!{# [cfg (not (no_global_oom_handling))] use crate :: collections :: VecDeque ;}
mkuse!{use crate :: raw_vec :: RawVec ;}
mkitem!{macro non_null { (mut $ place : expr , $ t : ident) => { { #! [allow (unused_unsafe)] unsafe { & mut * ((& raw mut $ place) as * mut NonNull <$ t >) } } } , ($ place : expr , $ t : ident) => { { #! [allow (unused_unsafe)] unsafe { * ((& raw const $ place) as * const NonNull <$ t >) } } } , }}
mkitem!{mkstruct!{# [doc = " An iterator that moves out of a vector."] # [doc = ""] # [doc = " This `struct` is created by the `into_iter` method on [`Vec`](super::Vec)"] # [doc = " (provided by the [`IntoIterator`] trait)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let v = vec![0, 1, 2];"] # [doc = " let iter: std::vec::IntoIter<_> = v.into_iter();"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [rustc_insignificant_dtor] pub struct IntoIter < T , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator = Global , > { pub (super) buf : NonNull < T > , pub (super) phantom : PhantomData < T > , pub (super) cap : usize , pub (super) alloc : ManuallyDrop < A > , pub (super) ptr : NonNull < T > , # [doc = " If T is a ZST, this is actually ptr+len. This encoding is picked so that"] # [doc = " ptr == end is a quick test for the Iterator being empty, that works"] # [doc = " for both ZST and non-ZST."] # [doc = " For non-ZSTs the pointer is treated as `NonNull<T>`"] pub (super) end : * const T , }}}
mkitem!{mkimpl!{# [stable (feature = "vec_intoiter_debug" , since = "1.13.0")] impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for IntoIter < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("IntoIter") . field (& self . as_slice ()) . finish () } }}}
mkitem!{mkimpl!{impl < T , A : Allocator > IntoIter < T , A > { # [doc = " Returns the remaining items of this iterator as a slice."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let vec = vec!['a', 'b', 'c'];"] # [doc = " let mut into_iter = vec.into_iter();"] # [doc = " assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);"] # [doc = " let _ = into_iter.next().unwrap();"] # [doc = " assert_eq!(into_iter.as_slice(), &['b', 'c']);"] # [doc = " ```"] # [stable (feature = "vec_into_iter_as_slice" , since = "1.15.0")] pub fn as_slice (& self) -> & [T] { unsafe { slice :: from_raw_parts (self . ptr . as_ptr () , self . len ()) } } # [doc = " Returns the remaining items of this iterator as a mutable slice."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let vec = vec!['a', 'b', 'c'];"] # [doc = " let mut into_iter = vec.into_iter();"] # [doc = " assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);"] # [doc = " into_iter.as_mut_slice()[2] = 'z';"] # [doc = " assert_eq!(into_iter.next().unwrap(), 'a');"] # [doc = " assert_eq!(into_iter.next().unwrap(), 'b');"] # [doc = " assert_eq!(into_iter.next().unwrap(), 'z');"] # [doc = " ```"] # [stable (feature = "vec_into_iter_as_slice" , since = "1.15.0")] pub fn as_mut_slice (& mut self) -> & mut [T] { unsafe { & mut * self . as_raw_mut_slice () } } # [doc = " Returns a reference to the underlying allocator."] # [unstable (feature = "allocator_api" , issue = "32838")] # [inline] pub fn allocator (& self) -> & A { & self . alloc } fn as_raw_mut_slice (& mut self) -> * mut [T] { ptr :: slice_from_raw_parts_mut (self . ptr . as_ptr () , self . len ()) } # [doc = " Drops remaining elements and relinquishes the backing allocation."] # [doc = ""] # [doc = " This method guarantees it won't panic before relinquishing the backing"] # [doc = " allocation."] # [doc = ""] # [doc = " This is roughly equivalent to the following, but more efficient"] # [doc = ""] # [doc = " ```"] # [doc = " # let mut vec = Vec::<u8>::with_capacity(10);"] # [doc = " # let ptr = vec.as_mut_ptr();"] # [doc = " # let mut into_iter = vec.into_iter();"] # [doc = " let mut into_iter = std::mem::replace(&mut into_iter, Vec::new().into_iter());"] # [doc = " (&mut into_iter).for_each(drop);"] # [doc = " std::mem::forget(into_iter);"] # [doc = " # // FIXME(https://github.com/rust-lang/miri/issues/3670):"] # [doc = " # // use -Zmiri-disable-leak-check instead of unleaking in tests meant to leak."] # [doc = " # drop(unsafe { Vec::<u8>::from_raw_parts(ptr, 0, 10) });"] # [doc = " ```"] # [doc = ""] # [doc = " This method is used by in-place iteration, refer to the vec::in_place_collect"] # [doc = " documentation for an overview."] # [cfg (not (no_global_oom_handling))] pub (super) fn forget_allocation_drop_remaining (& mut self) { let remaining = self . as_raw_mut_slice () ; self . cap = 0 ; self . buf = RawVec :: new () . non_null () ; self . ptr = self . buf ; self . end = self . buf . as_ptr () ; unsafe { ptr :: drop_in_place (remaining) ; } } # [doc = " Forgets to Drop the remaining elements while still allowing the backing allocation to be freed."] pub (crate) fn forget_remaining_elements (& mut self) { self . end = self . ptr . as_ptr () ; } # [cfg (not (no_global_oom_handling))] # [inline] pub (crate) fn into_vecdeque (self) -> VecDeque < T , A > { let mut this = ManuallyDrop :: new (self) ; unsafe { let buf = this . buf . as_ptr () ; let initialized = if T :: IS_ZST { 0 .. this . len () } else { this . ptr . offset_from_unsigned (this . buf) .. this . end . offset_from_unsigned (buf) } ; let cap = this . cap ; let alloc = ManuallyDrop :: take (& mut this . alloc) ; VecDeque :: from_contiguous_raw_parts_in (buf , initialized , cap , alloc) } } }}}
mkitem!{mkimpl!{# [stable (feature = "vec_intoiter_as_ref" , since = "1.46.0")] impl < T , A : Allocator > AsRef < [T] > for IntoIter < T , A > { fn as_ref (& self) -> & [T] { self . as_slice () } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] unsafe impl < T : Send , A : Allocator + Send > Send for IntoIter < T , A > { }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] unsafe impl < T : Sync , A : Allocator + Sync > Sync for IntoIter < T , A > { }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator > Iterator for IntoIter < T , A > { type Item = T ; # [inline] fn next (& mut self) -> Option < T > { let ptr = if T :: IS_ZST { if self . ptr . as_ptr () == self . end as * mut T { return None ; } self . end = self . end . wrapping_byte_sub (1) ; self . ptr } else { if self . ptr == non_null ! (self . end , T) { return None ; } let old = self . ptr ; self . ptr = unsafe { old . add (1) } ; old } ; Some (unsafe { ptr . read () }) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let exact = if T :: IS_ZST { self . end . addr () . wrapping_sub (self . ptr . as_ptr () . addr ()) } else { unsafe { non_null ! (self . end , T) . offset_from_unsigned (self . ptr) } } ; (exact , Some (exact)) } # [inline] fn advance_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { let step_size = self . len () . min (n) ; let to_drop = ptr :: slice_from_raw_parts_mut (self . ptr . as_ptr () , step_size) ; if T :: IS_ZST { self . end = self . end . wrapping_byte_sub (step_size) ; } else { self . ptr = unsafe { self . ptr . add (step_size) } ; } unsafe { ptr :: drop_in_place (to_drop) ; } NonZero :: new (n - step_size) . map_or (Ok (()) , Err) } # [inline] fn count (self) -> usize { self . len () } # [inline] fn last (mut self) -> Option < T > { self . next_back () } # [inline] fn next_chunk < const N : usize > (& mut self) -> Result < [T ; N] , core :: array :: IntoIter < T , N > > { let mut raw_ary = [const { MaybeUninit :: uninit () } ; N] ; let len = self . len () ; if T :: IS_ZST { if len < N { self . forget_remaining_elements () ; return Err (unsafe { array :: IntoIter :: new_unchecked (raw_ary , 0 .. len) }) ; } self . end = self . end . wrapping_byte_sub (N) ; return Ok (unsafe { raw_ary . transpose () . assume_init () }) ; } if len < N { unsafe { ptr :: copy_nonoverlapping (self . ptr . as_ptr () , raw_ary . as_mut_ptr () as * mut T , len) ; self . forget_remaining_elements () ; return Err (array :: IntoIter :: new_unchecked (raw_ary , 0 .. len)) ; } } unsafe { ptr :: copy_nonoverlapping (self . ptr . as_ptr () , raw_ary . as_mut_ptr () as * mut T , N) ; self . ptr = self . ptr . add (N) ; Ok (raw_ary . transpose () . assume_init ()) } } fn fold < B , F > (mut self , mut accum : B , mut f : F) -> B where F : FnMut (B , Self :: Item) -> B , { if T :: IS_ZST { while self . ptr . as_ptr () != self . end . cast_mut () { let tmp = unsafe { self . ptr . read () } ; self . end = self . end . wrapping_byte_sub (1) ; accum = f (accum , tmp) ; } } else { while self . ptr != non_null ! (self . end , T) { let tmp = unsafe { self . ptr . read () } ; self . ptr = unsafe { self . ptr . add (1) } ; accum = f (accum , tmp) ; } } accum } fn try_fold < B , F , R > (& mut self , mut accum : B , mut f : F) -> R where Self : Sized , F : FnMut (B , Self :: Item) -> R , R : core :: ops :: Try < Output = B > , { if T :: IS_ZST { while self . ptr . as_ptr () != self . end . cast_mut () { let tmp = unsafe { self . ptr . read () } ; self . end = self . end . wrapping_byte_sub (1) ; accum = f (accum , tmp) ? ; } } else { while self . ptr != non_null ! (self . end , T) { let tmp = unsafe { self . ptr . read () } ; self . ptr = unsafe { self . ptr . add (1) } ; accum = f (accum , tmp) ? ; } } R :: from_output (accum) } unsafe fn __iterator_get_unchecked (& mut self , i : usize) -> Self :: Item where Self : TrustedRandomAccessNoCoerce , { unsafe { self . ptr . add (i) . read () } } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator > DoubleEndedIterator for IntoIter < T , A > { # [inline] fn next_back (& mut self) -> Option < T > { if T :: IS_ZST { if self . ptr . as_ptr () == self . end as * mut _ { return None ; } self . end = self . end . wrapping_byte_sub (1) ; Some (unsafe { ptr :: read (self . ptr . as_ptr ()) }) } else { if self . ptr == non_null ! (self . end , T) { return None ; } unsafe { self . end = self . end . sub (1) ; Some (ptr :: read (self . end)) } } } # [inline] fn advance_back_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { let step_size = self . len () . min (n) ; if T :: IS_ZST { self . end = self . end . wrapping_byte_sub (step_size) ; } else { self . end = unsafe { self . end . sub (step_size) } ; } let to_drop = ptr :: slice_from_raw_parts_mut (self . end as * mut T , step_size) ; unsafe { ptr :: drop_in_place (to_drop) ; } NonZero :: new (n - step_size) . map_or (Ok (()) , Err) } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator > ExactSizeIterator for IntoIter < T , A > { fn is_empty (& self) -> bool { if T :: IS_ZST { self . ptr . as_ptr () == self . end as * mut _ } else { self . ptr == non_null ! (self . end , T) } } }}}
mkitem!{mkimpl!{# [stable (feature = "fused" , since = "1.26.0")] impl < T , A : Allocator > FusedIterator for IntoIter < T , A > { }}}
mkitem!{mkimpl!{# [doc (hidden)] # [unstable (issue = "none" , feature = "trusted_fused")] unsafe impl < T , A : Allocator > TrustedFused for IntoIter < T , A > { }}}
mkitem!{mkimpl!{# [unstable (feature = "trusted_len" , issue = "37572")] unsafe impl < T , A : Allocator > TrustedLen for IntoIter < T , A > { }}}
mkitem!{mkimpl!{# [stable (feature = "default_iters" , since = "1.70.0")] impl < T , A > Default for IntoIter < T , A > where A : Allocator + Default , { # [doc = " Creates an empty `vec::IntoIter`."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::vec;"] # [doc = " let iter: vec::IntoIter<u8> = Default::default();"] # [doc = " assert_eq!(iter.len(), 0);"] # [doc = " assert_eq!(iter.as_slice(), &[]);"] # [doc = " ```"] fn default () -> Self { super :: Vec :: new_in (Default :: default ()) . into_iter () } }}}
mkitem!{mktrait!{# [doc (hidden)] # [unstable (issue = "none" , feature = "std_internals")] # [rustc_unsafe_specialization_marker] pub trait NonDrop { }}}
mkitem!{mkimpl!{# [unstable (issue = "none" , feature = "std_internals")] impl < T : Copy > NonDrop for T { }}}
mkitem!{mkimpl!{# [doc (hidden)] # [unstable (issue = "none" , feature = "std_internals")] unsafe impl < T , A : Allocator > TrustedRandomAccessNoCoerce for IntoIter < T , A > where T : NonDrop , { const MAY_HAVE_SIDE_EFFECT : bool = false ; }}}
mkitem!{mkimpl!{# [cfg (not (no_global_oom_handling))] # [stable (feature = "vec_into_iter_clone" , since = "1.8.0")] impl < T : Clone , A : Allocator + Clone > Clone for IntoIter < T , A > { fn clone (& self) -> Self { self . as_slice () . to_vec_in (self . alloc . deref () . clone ()) . into_iter () } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] unsafe impl < # [may_dangle] T , A : Allocator > Drop for IntoIter < T , A > { fn drop (& mut self) { struct DropGuard < 'a , T , A : Allocator > (& 'a mut IntoIter < T , A >) ; impl < T , A : Allocator > Drop for DropGuard < '_ , T , A > { fn drop (& mut self) { unsafe { let alloc = ManuallyDrop :: take (& mut self . 0 . alloc) ; let _ = RawVec :: from_nonnull_in (self . 0 . buf , self . 0 . cap , alloc) ; } } } let guard = DropGuard (self) ; unsafe { ptr :: drop_in_place (guard . 0 . as_raw_mut_slice ()) ; } } }}}
mkitem!{mkimpl!{# [unstable (issue = "none" , feature = "inplace_iteration")] # [doc (hidden)] unsafe impl < T , A : Allocator > InPlaceIterable for IntoIter < T , A > { const EXPAND_BY : Option < NonZero < usize > > = NonZero :: new (1) ; const MERGE_BY : Option < NonZero < usize > > = NonZero :: new (1) ; }}}
mkitem!{mkimpl!{# [unstable (issue = "none" , feature = "inplace_iteration")] # [doc (hidden)] unsafe impl < T , A : Allocator > SourceIter for IntoIter < T , A > { type Source = Self ; # [inline] unsafe fn as_inner (& mut self) -> & mut Self :: Source { self } }}}
mkitem!{mkimpl!{# [cfg (not (no_global_oom_handling))] unsafe impl < T > AsVecIntoIter for IntoIter < T > { type Item = T ; fn as_into_iter (& mut self) -> & mut IntoIter < Self :: Item > { self } }}}