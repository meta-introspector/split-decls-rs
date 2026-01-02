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
mkuse!{use crate :: cmp ;}
mkuse!{use crate :: fmt :: { self , Debug } ;}
mkuse!{use crate :: iter :: { FusedIterator , InPlaceIterable , SourceIter , TrustedFused , TrustedLen , UncheckedIterator , } ;}
mkuse!{use crate :: num :: NonZero ;}
mkitem!{mkstruct!{#[doc = " An iterator that iterates two other iterators simultaneously."] #[doc = ""] #[doc = " This `struct` is created by [`zip`] or [`Iterator::zip`]."] #[doc = " See their documentation for more."] #[derive (Clone)] #[must_use = "iterators are lazy and do nothing unless consumed"] #[stable (feature = "rust1" , since = "1.0.0")] pub struct Zip < A , B > { a : A , b : B , index : usize , len : usize , }}}
mkitem!{mkimpl!{impl < A : Iterator , B : Iterator > Zip < A , B > { pub (in crate :: iter) fn new (a : A , b : B) -> Zip < A , B > { ZipImpl :: new (a , b) } fn super_nth (& mut self , mut n : usize) -> Option < (A :: Item , B :: Item) > { while let Some (x) = Iterator :: next (self) { if n == 0 { return Some (x) ; } n -= 1 ; } None } }}}

macro_rules! zip_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function zip in module {}", module_path!());
    };
}

mkfn!{
    zip_introspect!();
    #[doc = " Converts the arguments to iterators and zips them."] #[doc = ""] #[doc = " See the documentation of [`Iterator::zip`] for more."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::iter::zip;"] #[doc = ""] #[doc = " let xs = [1, 2, 3];"] #[doc = " let ys = [4, 5, 6];"] #[doc = ""] #[doc = " let mut iter = zip(xs, ys);"] #[doc = ""] #[doc = " assert_eq!(iter.next().unwrap(), (1, 4));"] #[doc = " assert_eq!(iter.next().unwrap(), (2, 5));"] #[doc = " assert_eq!(iter.next().unwrap(), (3, 6));"] #[doc = " assert!(iter.next().is_none());"] #[doc = ""] #[doc = " // Nested zips are also possible:"] #[doc = " let zs = [7, 8, 9];"] #[doc = ""] #[doc = " let mut iter = zip(zip(xs, ys), zs);"] #[doc = ""] #[doc = " assert_eq!(iter.next().unwrap(), ((1, 4), 7));"] #[doc = " assert_eq!(iter.next().unwrap(), ((2, 5), 8));"] #[doc = " assert_eq!(iter.next().unwrap(), ((3, 6), 9));"] #[doc = " assert!(iter.next().is_none());"] #[doc = " ```"] #[stable (feature = "iter_zip" , since = "1.59.0")] pub fn zip < A , B > (a : A , b : B) -> Zip < A :: IntoIter , B :: IntoIter > where A : IntoIterator , B : IntoIterator , { ZipImpl :: new (a . into_iter () , b . into_iter ()) }
}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < A , B > Iterator for Zip < A , B > where A : Iterator , B : Iterator , { type Item = (A :: Item , B :: Item) ; #[inline] fn next (& mut self) -> Option < Self :: Item > { ZipImpl :: next (self) } #[inline] fn size_hint (& self) -> (usize , Option < usize >) { ZipImpl :: size_hint (self) } #[inline] fn nth (& mut self , n : usize) -> Option < Self :: Item > { ZipImpl :: nth (self , n) } #[inline] fn fold < Acc , F > (self , init : Acc , f : F) -> Acc where F : FnMut (Acc , Self :: Item) -> Acc , { ZipImpl :: fold (self , init , f) } #[inline] unsafe fn __iterator_get_unchecked (& mut self , idx : usize) -> Self :: Item where Self : TrustedRandomAccessNoCoerce , { unsafe { ZipImpl :: get_unchecked (self , idx) } } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < A , B > DoubleEndedIterator for Zip < A , B > where A : DoubleEndedIterator + ExactSizeIterator , B : DoubleEndedIterator + ExactSizeIterator , { #[inline] fn next_back (& mut self) -> Option < (A :: Item , B :: Item) > { ZipImpl :: next_back (self) } }}}
mkitem!{mktrait!{#[doc (hidden)] trait ZipImpl < A , B > { type Item ; fn new (a : A , b : B) -> Self ; fn next (& mut self) -> Option < Self :: Item > ; fn size_hint (& self) -> (usize , Option < usize >) ; fn nth (& mut self , n : usize) -> Option < Self :: Item > ; fn next_back (& mut self) -> Option < Self :: Item > where A : DoubleEndedIterator + ExactSizeIterator , B : DoubleEndedIterator + ExactSizeIterator ; fn fold < Acc , F > (self , init : Acc , f : F) -> Acc where F : FnMut (Acc , Self :: Item) -> Acc ; unsafe fn get_unchecked (& mut self , idx : usize) -> < Self as Iterator > :: Item where Self : Iterator + TrustedRandomAccessNoCoerce ; }}}
mkitem!{macro_rules ! zip_impl_general_defaults { () => { default fn new (a : A , b : B) -> Self { Zip { a , b , index : 0 , len : 0 , } } #[inline] default fn next (& mut self) -> Option < (A :: Item , B :: Item) > { let x = self . a . next () ?; let y = self . b . next () ?; Some ((x , y)) } #[inline] default fn nth (& mut self , n : usize) -> Option < Self :: Item > { self . super_nth (n) } #[inline] default fn next_back (& mut self) -> Option < (A :: Item , B :: Item) > where A : DoubleEndedIterator + ExactSizeIterator , B : DoubleEndedIterator + ExactSizeIterator , { let a_sz = self . a . len () ; let b_sz = self . b . len () ; if a_sz != b_sz { if a_sz > b_sz { for _ in 0 .. a_sz - b_sz { self . a . next_back () ; } } else { for _ in 0 .. b_sz - a_sz { self . b . next_back () ; } } } match (self . a . next_back () , self . b . next_back ()) { (Some (x) , Some (y)) => Some ((x , y)) , (None , None) => None , _ => unreachable ! () , } } } ; }}
mkitem!{mkimpl!{#[doc (hidden)] impl < A , B > ZipImpl < A , B > for Zip < A , B > where A : Iterator , B : Iterator , { type Item = (A :: Item , B :: Item) ; zip_impl_general_defaults ! { } #[inline] default fn size_hint (& self) -> (usize , Option < usize >) { let (a_lower , a_upper) = self . a . size_hint () ; let (b_lower , b_upper) = self . b . size_hint () ; let lower = cmp :: min (a_lower , b_lower) ; let upper = match (a_upper , b_upper) { (Some (x) , Some (y)) => Some (cmp :: min (x , y)) , (Some (x) , None) => Some (x) , (None , Some (y)) => Some (y) , (None , None) => None , } ; (lower , upper) } default unsafe fn get_unchecked (& mut self , _idx : usize) -> < Self as Iterator > :: Item where Self : TrustedRandomAccessNoCoerce , { unreachable ! ("Always specialized") ; } #[inline] default fn fold < Acc , F > (self , init : Acc , f : F) -> Acc where F : FnMut (Acc , Self :: Item) -> Acc , { SpecFold :: spec_fold (self , init , f) } }}}
mkitem!{mkimpl!{#[doc (hidden)] impl < A , B > ZipImpl < A , B > for Zip < A , B > where A : TrustedRandomAccessNoCoerce + Iterator , B : TrustedRandomAccessNoCoerce + Iterator , { zip_impl_general_defaults ! { } #[inline] default fn size_hint (& self) -> (usize , Option < usize >) { let size = cmp :: min (self . a . size () , self . b . size ()) ; (size , Some (size)) } #[inline] unsafe fn get_unchecked (& mut self , idx : usize) -> < Self as Iterator > :: Item { let idx = self . index + idx ; unsafe { (self . a . __iterator_get_unchecked (idx) , self . b . __iterator_get_unchecked (idx)) } } #[inline] fn fold < Acc , F > (mut self , init : Acc , mut f : F) -> Acc where F : FnMut (Acc , Self :: Item) -> Acc , { let mut accum = init ; let len = ZipImpl :: size_hint (& self) . 0 ; for i in 0 .. len { unsafe { accum = f (accum , self . get_unchecked (i)) ; } } accum } }}}
mkitem!{mkimpl!{#[doc (hidden)] impl < A , B > ZipImpl < A , B > for Zip < A , B > where A : TrustedRandomAccess + Iterator , B : TrustedRandomAccess + Iterator , { fn new (a : A , b : B) -> Self { let len = cmp :: min (a . size () , b . size ()) ; Zip { a , b , index : 0 , len } } #[inline] fn next (& mut self) -> Option < (A :: Item , B :: Item) > { if self . index < self . len { let i = self . index ; self . index += 1 ; unsafe { Some ((self . a . __iterator_get_unchecked (i) , self . b . __iterator_get_unchecked (i))) } } else { None } } #[inline] fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len - self . index ; (len , Some (len)) } #[inline] fn nth (& mut self , n : usize) -> Option < Self :: Item > { let delta = cmp :: min (n , self . len - self . index) ; let end = self . index + delta ; while self . index < end { let i = self . index ; self . index += 1 ; if A :: MAY_HAVE_SIDE_EFFECT { unsafe { self . a . __iterator_get_unchecked (i) ; } } if B :: MAY_HAVE_SIDE_EFFECT { unsafe { self . b . __iterator_get_unchecked (i) ; } } } self . super_nth (n - delta) } #[inline] fn next_back (& mut self) -> Option < (A :: Item , B :: Item) > where A : DoubleEndedIterator + ExactSizeIterator , B : DoubleEndedIterator + ExactSizeIterator , { if self . index < self . len { let old_len = self . len ; self . len -= 1 ; if A :: MAY_HAVE_SIDE_EFFECT || B :: MAY_HAVE_SIDE_EFFECT { let sz_a = self . a . size () ; let sz_b = self . b . size () ; if sz_a != sz_b && (old_len == sz_a || old_len == sz_b) { if A :: MAY_HAVE_SIDE_EFFECT && sz_a > old_len { for _ in 0 .. sz_a - old_len { self . a . next_back () ; } } if B :: MAY_HAVE_SIDE_EFFECT && sz_b > old_len { for _ in 0 .. sz_b - old_len { self . b . next_back () ; } } debug_assert_eq ! (self . a . size () , self . b . size ()) ; } } let i = self . len ; unsafe { Some ((self . a . __iterator_get_unchecked (i) , self . b . __iterator_get_unchecked (i))) } } else { None } } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < A , B > ExactSizeIterator for Zip < A , B > where A : ExactSizeIterator , B : ExactSizeIterator , { }}}
mkitem!{mkimpl!{#[doc (hidden)] #[unstable (feature = "trusted_random_access" , issue = "none")] unsafe impl < A , B > TrustedRandomAccess for Zip < A , B > where A : TrustedRandomAccess , B : TrustedRandomAccess , { }}}
mkitem!{mkimpl!{#[doc (hidden)] #[unstable (feature = "trusted_random_access" , issue = "none")] unsafe impl < A , B > TrustedRandomAccessNoCoerce for Zip < A , B > where A : TrustedRandomAccessNoCoerce , B : TrustedRandomAccessNoCoerce , { const MAY_HAVE_SIDE_EFFECT : bool = A :: MAY_HAVE_SIDE_EFFECT || B :: MAY_HAVE_SIDE_EFFECT ; }}}
mkitem!{mkimpl!{#[stable (feature = "fused" , since = "1.26.0")] impl < A , B > FusedIterator for Zip < A , B > where A : FusedIterator , B : FusedIterator , { }}}
mkitem!{mkimpl!{#[unstable (issue = "none" , feature = "trusted_fused")] unsafe impl < A , B > TrustedFused for Zip < A , B > where A : TrustedFused , B : TrustedFused , { }}}
mkitem!{mkimpl!{#[unstable (feature = "trusted_len" , issue = "37572")] unsafe impl < A , B > TrustedLen for Zip < A , B > where A : TrustedLen , B : TrustedLen , { }}}
mkitem!{mkimpl!{impl < A , B > UncheckedIterator for Zip < A , B > where A : UncheckedIterator , B : UncheckedIterator , { }}}
mkitem!{mkimpl!{#[unstable (issue = "none" , feature = "inplace_iteration")] unsafe impl < A , B > SourceIter for Zip < A , B > where A : SourceIter , { type Source = A :: Source ; #[inline] unsafe fn as_inner (& mut self) -> & mut A :: Source { unsafe { SourceIter :: as_inner (& mut self . a) } } }}}
mkitem!{mkimpl!{#[unstable (issue = "none" , feature = "inplace_iteration")] unsafe impl < A : InPlaceIterable , B > InPlaceIterable for Zip < A , B > { const EXPAND_BY : Option < NonZero < usize > > = A :: EXPAND_BY ; const MERGE_BY : Option < NonZero < usize > > = A :: MERGE_BY ; }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < A : Debug , B : Debug > Debug for Zip < A , B > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { ZipFmt :: fmt (self , f) } }}}
mkitem!{mktrait!{trait ZipFmt < A , B > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result ; }}}
mkitem!{mkimpl!{impl < A : Debug , B : Debug > ZipFmt < A , B > for Zip < A , B > { default fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Zip") . field ("a" , & self . a) . field ("b" , & self . b) . finish () } }}}
mkitem!{mkimpl!{impl < A : Debug + TrustedRandomAccessNoCoerce , B : Debug + TrustedRandomAccessNoCoerce > ZipFmt < A , B > for Zip < A , B > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Zip") . finish () } }}}
mkitem!{mktrait!{#[doc = " An iterator whose items are random-accessible efficiently"] #[doc = ""] #[doc = " # Safety"] #[doc = ""] #[doc = " The iterator's `size_hint` must be exact and cheap to call."] #[doc = ""] #[doc = " `TrustedRandomAccessNoCoerce::size` may not be overridden."] #[doc = ""] #[doc = " All subtypes and all supertypes of `Self` must also implement `TrustedRandomAccess`."] #[doc = " In particular, this means that types with non-invariant parameters usually can not have"] #[doc = " an impl for `TrustedRandomAccess` that depends on any trait bounds on such parameters, except"] #[doc = " for bounds that come from the respective struct/enum definition itself, or bounds involving"] #[doc = " traits that themselves come with a guarantee similar to this one."] #[doc = ""] #[doc = " If `Self: ExactSizeIterator` then `self.len()` must always produce results consistent"] #[doc = " with `self.size()`."] #[doc = ""] #[doc = " If `Self: Iterator`, then `<Self as Iterator>::__iterator_get_unchecked(&mut self, idx)`"] #[doc = " must be safe to call provided the following conditions are met."] #[doc = ""] #[doc = " 1. `0 <= idx` and `idx < self.size()`."] #[doc = " 2. If `Self: !Clone`, then `self.__iterator_get_unchecked(idx)` is never called with the same"] #[doc = "    index on `self` more than once."] #[doc = " 3. After `self.__iterator_get_unchecked(idx)` has been called, then `self.next_back()` will"] #[doc = "    only be called at most `self.size() - idx - 1` times. If `Self: Clone` and `self` is cloned,"] #[doc = "    then this number is calculated for `self` and its clone individually,"] #[doc = "    but `self.next_back()` calls that happened before the cloning count for both `self` and the clone."] #[doc = " 4. After `self.__iterator_get_unchecked(idx)` has been called, then only the following methods"] #[doc = "    will be called on `self` or on any new clones of `self`:"] #[doc = "     * `std::clone::Clone::clone`"] #[doc = "     * `std::iter::Iterator::size_hint`"] #[doc = "     * `std::iter::DoubleEndedIterator::next_back`"] #[doc = "     * `std::iter::ExactSizeIterator::len`"] #[doc = "     * `std::iter::Iterator::__iterator_get_unchecked`"] #[doc = "     * `std::iter::TrustedRandomAccessNoCoerce::size`"] #[doc = " 5. If `Self` is a subtype of `T`, then `self` is allowed to be coerced"] #[doc = "    to `T`. If `self` is coerced to `T` after `self.__iterator_get_unchecked(idx)` has already"] #[doc = "    been called, then no methods except for the ones listed under 4. are allowed to be called"] #[doc = "    on the resulting value of type `T`, either. Multiple such coercion steps are allowed."] #[doc = "    Regarding 2. and 3., the number of times `__iterator_get_unchecked(idx)` or `next_back()` is"] #[doc = "    called on `self` and the resulting value of type `T` (and on further coercion results with"] #[doc = "    super-supertypes) are added together and their sums must not exceed the specified bounds."] #[doc = ""] #[doc = " Further, given that these conditions are met, it must guarantee that:"] #[doc = ""] #[doc = " * It does not change the value returned from `size_hint`"] #[doc = " * It must be safe to call the methods listed above on `self` after calling"] #[doc = "   `self.__iterator_get_unchecked(idx)`, assuming that the required traits are implemented."] #[doc = " * It must also be safe to drop `self` after calling `self.__iterator_get_unchecked(idx)`."] #[doc = " * If `Self` is a subtype of `T`, then it must be safe to coerce `self` to `T`."] #[doc (hidden)] #[unstable (feature = "trusted_random_access" , issue = "none")] #[rustc_specialization_trait] pub unsafe trait TrustedRandomAccess : TrustedRandomAccessNoCoerce { }}}
mkitem!{mktrait!{#[doc = " Like [`TrustedRandomAccess`] but without any of the requirements / guarantees around"] #[doc = " coercions to supertypes after `__iterator_get_unchecked` (they aren’t allowed here!), and"] #[doc = " without the requirement that subtypes / supertypes implement `TrustedRandomAccessNoCoerce`."] #[doc = ""] #[doc = " This trait was created in PR #85874 to fix soundness issue #85873 without performance regressions."] #[doc = " It is subject to change as we might want to build a more generally useful (for performance"] #[doc = " optimizations) and more sophisticated trait or trait hierarchy that replaces or extends"] #[doc = " [`TrustedRandomAccess`] and `TrustedRandomAccessNoCoerce`."] #[doc (hidden)] #[unstable (feature = "trusted_random_access" , issue = "none")] #[rustc_specialization_trait] pub unsafe trait TrustedRandomAccessNoCoerce : Sized { fn size (& self) -> usize where Self : Iterator , { self . size_hint () . 0 } #[doc = " `true` if getting an iterator element may have side effects."] #[doc = " Remember to take inner iterators into account."] const MAY_HAVE_SIDE_EFFECT : bool ; }}}

macro_rules! try_get_unchecked_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_get_unchecked in module {}", module_path!());
    };
}

mkfn!{
    try_get_unchecked_introspect!();
    #[doc = " Like `Iterator::__iterator_get_unchecked`, but doesn't require the compiler to"] #[doc = " know that `U: TrustedRandomAccess`."] #[doc = ""] #[doc = " ## Safety"] #[doc = ""] #[doc = " Same requirements calling `get_unchecked` directly."] #[doc (hidden)] #[inline] pub (in crate :: iter :: adapters) unsafe fn try_get_unchecked < I > (it : & mut I , idx : usize) -> I :: Item where I : Iterator , { unsafe { it . try_get_unchecked (idx) } }
}
mkitem!{mktrait!{unsafe trait SpecTrustedRandomAccess : Iterator { #[doc = " If `Self: TrustedRandomAccess`, it must be safe to call"] #[doc = " `Iterator::__iterator_get_unchecked(self, index)`."] unsafe fn try_get_unchecked (& mut self , index : usize) -> Self :: Item ; }}}
mkitem!{mkimpl!{unsafe impl < I : Iterator > SpecTrustedRandomAccess for I { default unsafe fn try_get_unchecked (& mut self , _ : usize) -> Self :: Item { panic ! ("Should only be called on TrustedRandomAccess iterators") ; } }}}
mkitem!{mkimpl!{unsafe impl < I : Iterator + TrustedRandomAccessNoCoerce > SpecTrustedRandomAccess for I { #[inline] unsafe fn try_get_unchecked (& mut self , index : usize) -> Self :: Item { unsafe { self . __iterator_get_unchecked (index) } } }}}
mkitem!{mktrait!{trait SpecFold : Iterator { fn spec_fold < B , F > (self , init : B , f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B ; }}}
mkitem!{mkimpl!{impl < A : Iterator , B : Iterator > SpecFold for Zip < A , B > { #[inline] default fn spec_fold < Acc , F > (mut self , init : Acc , mut f : F) -> Acc where F : FnMut (Acc , Self :: Item) -> Acc , { let mut accum = init ; while let Some (x) = ZipImpl :: next (& mut self) { accum = f (accum , x) ; } accum } }}}
mkitem!{mkimpl!{impl < A : TrustedLen , B : TrustedLen > SpecFold for Zip < A , B > { #[inline] fn spec_fold < Acc , F > (mut self , init : Acc , mut f : F) -> Acc where F : FnMut (Acc , Self :: Item) -> Acc , { let mut accum = init ; loop { let (upper , more) = if let Some (upper) = ZipImpl :: size_hint (& self) . 1 { (upper , false) } else { (usize :: MAX , true) } ; for _ in 0 .. upper { let pair = unsafe { (self . a . next () . unwrap_unchecked () , self . b . next () . unwrap_unchecked ()) } ; accum = f (accum , pair) ; } if ! more { break ; } } accum } }}}