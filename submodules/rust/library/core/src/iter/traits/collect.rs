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
mkuse!{use super :: TrustedLen ;}
mkitem!{mktrait!{#[doc = " Conversion from an [`Iterator`]."] #[doc = ""] #[doc = " By implementing `FromIterator` for a type, you define how it will be"] #[doc = " created from an iterator. This is common for types which describe a"] #[doc = " collection of some kind."] #[doc = ""] #[doc = " If you want to create a collection from the contents of an iterator, the"] #[doc = " [`Iterator::collect()`] method is preferred. However, when you need to"] #[doc = " specify the container type, [`FromIterator::from_iter()`] can be more"] #[doc = " readable than using a turbofish (e.g. `::<Vec<_>>()`). See the"] #[doc = " [`Iterator::collect()`] documentation for more examples of its use."] #[doc = ""] #[doc = " See also: [`IntoIterator`]."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " Basic usage:"] #[doc = ""] #[doc = " ```"] #[doc = " let five_fives = std::iter::repeat(5).take(5);"] #[doc = ""] #[doc = " let v = Vec::from_iter(five_fives);"] #[doc = ""] #[doc = " assert_eq!(v, vec![5, 5, 5, 5, 5]);"] #[doc = " ```"] #[doc = ""] #[doc = " Using [`Iterator::collect()`] to implicitly use `FromIterator`:"] #[doc = ""] #[doc = " ```"] #[doc = " let five_fives = std::iter::repeat(5).take(5);"] #[doc = ""] #[doc = " let v: Vec<i32> = five_fives.collect();"] #[doc = ""] #[doc = " assert_eq!(v, vec![5, 5, 5, 5, 5]);"] #[doc = " ```"] #[doc = ""] #[doc = " Using [`FromIterator::from_iter()`] as a more readable alternative to"] #[doc = " [`Iterator::collect()`]:"] #[doc = ""] #[doc = " ```"] #[doc = " use std::collections::VecDeque;"] #[doc = " let first = (0..10).collect::<VecDeque<i32>>();"] #[doc = " let second = VecDeque::from_iter(0..10);"] #[doc = ""] #[doc = " assert_eq!(first, second);"] #[doc = " ```"] #[doc = ""] #[doc = " Implementing `FromIterator` for your type:"] #[doc = ""] #[doc = " ```"] #[doc = " // A sample collection, that's just a wrapper over Vec<T>"] #[doc = " #[derive(Debug)]"] #[doc = " struct MyCollection(Vec<i32>);"] #[doc = ""] #[doc = " // Let's give it some methods so we can create one and add things"] #[doc = " // to it."] #[doc = " impl MyCollection {"] #[doc = "     fn new() -> MyCollection {"] #[doc = "         MyCollection(Vec::new())"] #[doc = "     }"] #[doc = ""] #[doc = "     fn add(&mut self, elem: i32) {"] #[doc = "         self.0.push(elem);"] #[doc = "     }"] #[doc = " }"] #[doc = ""] #[doc = " // and we'll implement FromIterator"] #[doc = " impl FromIterator<i32> for MyCollection {"] #[doc = "     fn from_iter<I: IntoIterator<Item=i32>>(iter: I) -> Self {"] #[doc = "         let mut c = MyCollection::new();"] #[doc = ""] #[doc = "         for i in iter {"] #[doc = "             c.add(i);"] #[doc = "         }"] #[doc = ""] #[doc = "         c"] #[doc = "     }"] #[doc = " }"] #[doc = ""] #[doc = " // Now we can make a new iterator..."] #[doc = " let iter = (0..5).into_iter();"] #[doc = ""] #[doc = " // ... and make a MyCollection out of it"] #[doc = " let c = MyCollection::from_iter(iter);"] #[doc = ""] #[doc = " assert_eq!(c.0, vec![0, 1, 2, 3, 4]);"] #[doc = ""] #[doc = " // collect works too!"] #[doc = ""] #[doc = " let iter = (0..5).into_iter();"] #[doc = " let c: MyCollection = iter.collect();"] #[doc = ""] #[doc = " assert_eq!(c.0, vec![0, 1, 2, 3, 4]);"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] #[rustc_on_unimplemented (on (Self = "&[{A}]" , message = "a slice of type `{Self}` cannot be built since we need to store the elements somewhere" , label = "try explicitly collecting into a `Vec<{A}>`" ,) , on (all (A = "{integer}" , any (Self = "&[{integral}]" ,)) , message = "a slice of type `{Self}` cannot be built since we need to store the elements somewhere" , label = "try explicitly collecting into a `Vec<{A}>`" ,) , on (Self = "[{A}]" , message = "a slice of type `{Self}` cannot be built since `{Self}` has no definite size" , label = "try explicitly collecting into a `Vec<{A}>`" ,) , on (all (A = "{integer}" , any (Self = "[{integral}]" ,)) , message = "a slice of type `{Self}` cannot be built since `{Self}` has no definite size" , label = "try explicitly collecting into a `Vec<{A}>`" ,) , on (Self = "[{A}; _]" , message = "an array of type `{Self}` cannot be built directly from an iterator" , label = "try collecting into a `Vec<{A}>`, then using `.try_into()`" ,) , on (all (A = "{integer}" , any (Self = "[{integral}; _]" ,)) , message = "an array of type `{Self}` cannot be built directly from an iterator" , label = "try collecting into a `Vec<{A}>`, then using `.try_into()`" ,) , message = "a value of type `{Self}` cannot be built from an iterator \
               over elements of type `{A}`" , label = "value of type `{Self}` cannot be built from `std::iter::Iterator<Item={A}>`")] #[rustc_diagnostic_item = "FromIterator"] pub trait FromIterator < A > : Sized { #[doc = " Creates a value from an iterator."] #[doc = ""] #[doc = " See the [module-level documentation] for more."] #[doc = ""] #[doc = " [module-level documentation]: crate::iter"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " let five_fives = std::iter::repeat(5).take(5);"] #[doc = ""] #[doc = " let v = Vec::from_iter(five_fives);"] #[doc = ""] #[doc = " assert_eq!(v, vec![5, 5, 5, 5, 5]);"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] #[rustc_diagnostic_item = "from_iter_fn"] fn from_iter < T : IntoIterator < Item = A > > (iter : T) -> Self ; }}}
mkitem!{mktrait!{#[doc = " Conversion into an [`Iterator`]."] #[doc = ""] #[doc = " By implementing `IntoIterator` for a type, you define how it will be"] #[doc = " converted to an iterator. This is common for types which describe a"] #[doc = " collection of some kind."] #[doc = ""] #[doc = " One benefit of implementing `IntoIterator` is that your type will [work"] #[doc = " with Rust's `for` loop syntax](crate::iter#for-loops-and-intoiterator)."] #[doc = ""] #[doc = " See also: [`FromIterator`]."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " Basic usage:"] #[doc = ""] #[doc = " ```"] #[doc = " let v = [1, 2, 3];"] #[doc = " let mut iter = v.into_iter();"] #[doc = ""] #[doc = " assert_eq!(Some(1), iter.next());"] #[doc = " assert_eq!(Some(2), iter.next());"] #[doc = " assert_eq!(Some(3), iter.next());"] #[doc = " assert_eq!(None, iter.next());"] #[doc = " ```"] #[doc = " Implementing `IntoIterator` for your type:"] #[doc = ""] #[doc = " ```"] #[doc = " // A sample collection, that's just a wrapper over Vec<T>"] #[doc = " #[derive(Debug)]"] #[doc = " struct MyCollection(Vec<i32>);"] #[doc = ""] #[doc = " // Let's give it some methods so we can create one and add things"] #[doc = " // to it."] #[doc = " impl MyCollection {"] #[doc = "     fn new() -> MyCollection {"] #[doc = "         MyCollection(Vec::new())"] #[doc = "     }"] #[doc = ""] #[doc = "     fn add(&mut self, elem: i32) {"] #[doc = "         self.0.push(elem);"] #[doc = "     }"] #[doc = " }"] #[doc = ""] #[doc = " // and we'll implement IntoIterator"] #[doc = " impl IntoIterator for MyCollection {"] #[doc = "     type Item = i32;"] #[doc = "     type IntoIter = std::vec::IntoIter<Self::Item>;"] #[doc = ""] #[doc = "     fn into_iter(self) -> Self::IntoIter {"] #[doc = "         self.0.into_iter()"] #[doc = "     }"] #[doc = " }"] #[doc = ""] #[doc = " // Now we can make a new collection..."] #[doc = " let mut c = MyCollection::new();"] #[doc = ""] #[doc = " // ... add some stuff to it ..."] #[doc = " c.add(0);"] #[doc = " c.add(1);"] #[doc = " c.add(2);"] #[doc = ""] #[doc = " // ... and then turn it into an Iterator:"] #[doc = " for (i, n) in c.into_iter().enumerate() {"] #[doc = "     assert_eq!(i as i32, n);"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " It is common to use `IntoIterator` as a trait bound. This allows"] #[doc = " the input collection type to change, so long as it is still an"] #[doc = " iterator. Additional bounds can be specified by restricting on"] #[doc = " `Item`:"] #[doc = ""] #[doc = " ```rust"] #[doc = " fn collect_as_strings<T>(collection: T) -> Vec<String>"] #[doc = " where"] #[doc = "     T: IntoIterator,"] #[doc = "     T::Item: std::fmt::Debug,"] #[doc = " {"] #[doc = "     collection"] #[doc = "         .into_iter()"] #[doc = "         .map(|item| format!(\"{item:?}\"))"] #[doc = "         .collect()"] #[doc = " }"] #[doc = " ```"] #[rustc_diagnostic_item = "IntoIterator"] #[rustc_on_unimplemented (on (Self = "core::ops::range::RangeTo<Idx>" , label = "if you meant to iterate until a value, add a starting value" , note = "`..end` is a `RangeTo`, which cannot be iterated on; you might have meant to have a \
              bounded `Range`: `0..end`") , on (Self = "core::ops::range::RangeToInclusive<Idx>" , label = "if you meant to iterate until a value (including it), add a starting value" , note = "`..=end` is a `RangeToInclusive`, which cannot be iterated on; you might have meant \
              to have a bounded `RangeInclusive`: `0..=end`") , on (Self = "[]" , label = "`{Self}` is not an iterator; try calling `.into_iter()` or `.iter()`") , on (Self = "&[]" , label = "`{Self}` is not an iterator; try calling `.iter()`") , on (Self = "alloc::vec::Vec<T, A>" , label = "`{Self}` is not an iterator; try calling `.into_iter()` or `.iter()`") , on (Self = "&str" , label = "`{Self}` is not an iterator; try calling `.chars()` or `.bytes()`") , on (Self = "alloc::string::String" , label = "`{Self}` is not an iterator; try calling `.chars()` or `.bytes()`") , on (Self = "{integral}" , note = "if you want to iterate between `start` until a value `end`, use the exclusive range \
              syntax `start..end` or the inclusive range syntax `start..=end`") , on (Self = "{float}" , note = "if you want to iterate between `start` until a value `end`, use the exclusive range \
              syntax `start..end` or the inclusive range syntax `start..=end`") , label = "`{Self}` is not an iterator" , message = "`{Self}` is not an iterator")] #[rustc_skip_during_method_dispatch (array , boxed_slice)] #[stable (feature = "rust1" , since = "1.0.0")] pub trait IntoIterator { #[doc = " The type of the elements being iterated over."] #[stable (feature = "rust1" , since = "1.0.0")] type Item ; #[doc = " Which kind of iterator are we turning this into?"] #[stable (feature = "rust1" , since = "1.0.0")] type IntoIter : Iterator < Item = Self :: Item > ; #[doc = " Creates an iterator from a value."] #[doc = ""] #[doc = " See the [module-level documentation] for more."] #[doc = ""] #[doc = " [module-level documentation]: crate::iter"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " let v = [1, 2, 3];"] #[doc = " let mut iter = v.into_iter();"] #[doc = ""] #[doc = " assert_eq!(Some(1), iter.next());"] #[doc = " assert_eq!(Some(2), iter.next());"] #[doc = " assert_eq!(Some(3), iter.next());"] #[doc = " assert_eq!(None, iter.next());"] #[doc = " ```"] #[lang = "into_iter"] #[stable (feature = "rust1" , since = "1.0.0")] fn into_iter (self) -> Self :: IntoIter ; }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < I : Iterator > IntoIterator for I { type Item = I :: Item ; type IntoIter = I ; #[inline] fn into_iter (self) -> I { self } }}}
mkitem!{mktrait!{#[doc = " Extend a collection with the contents of an iterator."] #[doc = ""] #[doc = " Iterators produce a series of values, and collections can also be thought"] #[doc = " of as a series of values. The `Extend` trait bridges this gap, allowing you"] #[doc = " to extend a collection by including the contents of that iterator. When"] #[doc = " extending a collection with an already existing key, that entry is updated"] #[doc = " or, in the case of collections that permit multiple entries with equal"] #[doc = " keys, that entry is inserted."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " Basic usage:"] #[doc = ""] #[doc = " ```"] #[doc = " // You can extend a String with some chars:"] #[doc = " let mut message = String::from(\"The first three letters are: \");"] #[doc = ""] #[doc = " message.extend(&['a', 'b', 'c']);"] #[doc = ""] #[doc = " assert_eq!(\"abc\", &message[29..32]);"] #[doc = " ```"] #[doc = ""] #[doc = " Implementing `Extend`:"] #[doc = ""] #[doc = " ```"] #[doc = " // A sample collection, that's just a wrapper over Vec<T>"] #[doc = " #[derive(Debug)]"] #[doc = " struct MyCollection(Vec<i32>);"] #[doc = ""] #[doc = " // Let's give it some methods so we can create one and add things"] #[doc = " // to it."] #[doc = " impl MyCollection {"] #[doc = "     fn new() -> MyCollection {"] #[doc = "         MyCollection(Vec::new())"] #[doc = "     }"] #[doc = ""] #[doc = "     fn add(&mut self, elem: i32) {"] #[doc = "         self.0.push(elem);"] #[doc = "     }"] #[doc = " }"] #[doc = ""] #[doc = " // since MyCollection has a list of i32s, we implement Extend for i32"] #[doc = " impl Extend<i32> for MyCollection {"] #[doc = ""] #[doc = "     // This is a bit simpler with the concrete type signature: we can call"] #[doc = "     // extend on anything which can be turned into an Iterator which gives"] #[doc = "     // us i32s. Because we need i32s to put into MyCollection."] #[doc = "     fn extend<T: IntoIterator<Item=i32>>(&mut self, iter: T) {"] #[doc = ""] #[doc = "         // The implementation is very straightforward: loop through the"] #[doc = "         // iterator, and add() each element to ourselves."] #[doc = "         for elem in iter {"] #[doc = "             self.add(elem);"] #[doc = "         }"] #[doc = "     }"] #[doc = " }"] #[doc = ""] #[doc = " let mut c = MyCollection::new();"] #[doc = ""] #[doc = " c.add(5);"] #[doc = " c.add(6);"] #[doc = " c.add(7);"] #[doc = ""] #[doc = " // let's extend our collection with three more numbers"] #[doc = " c.extend(vec![1, 2, 3]);"] #[doc = ""] #[doc = " // we've added these elements onto the end"] #[doc = " assert_eq!(\"MyCollection([5, 6, 7, 1, 2, 3])\", format!(\"{c:?}\"));"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] pub trait Extend < A > { #[doc = " Extends a collection with the contents of an iterator."] #[doc = ""] #[doc = " As this is the only required method for this trait, the [trait-level] docs"] #[doc = " contain more details."] #[doc = ""] #[doc = " [trait-level]: Extend"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " // You can extend a String with some chars:"] #[doc = " let mut message = String::from(\"abc\");"] #[doc = ""] #[doc = " message.extend(['d', 'e', 'f'].iter());"] #[doc = ""] #[doc = " assert_eq!(\"abcdef\", &message);"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] fn extend < T : IntoIterator < Item = A > > (& mut self , iter : T) ; #[doc = " Extends a collection with exactly one element."] #[unstable (feature = "extend_one" , issue = "72631")] fn extend_one (& mut self , item : A) { self . extend (Some (item)) ; } #[doc = " Reserves capacity in a collection for the given number of additional elements."] #[doc = ""] #[doc = " The default implementation does nothing."] #[unstable (feature = "extend_one" , issue = "72631")] fn extend_reserve (& mut self , additional : usize) { let _ = additional ; } #[doc = " Extends a collection with one element, without checking there is enough capacity for it."] #[doc = ""] #[doc = " # Safety"] #[doc = ""] #[doc = " **For callers:** This must only be called when we know the collection has enough capacity"] #[doc = " to contain the new item, for example because we previously called `extend_reserve`."] #[doc = ""] #[doc = " **For implementors:** For a collection to unsafely rely on this method's safety precondition (that is,"] #[doc = " invoke UB if they are violated), it must implement `extend_reserve` correctly. In other words,"] #[doc = " callers may assume that if they `extend_reserve`ed enough space they can call this method."] #[unstable (feature = "extend_one_unchecked" , issue = "none")] #[doc (hidden)] unsafe fn extend_one_unchecked (& mut self , item : A) where Self : Sized , { self . extend_one (item) ; } }}}
mkitem!{mkimpl!{#[stable (feature = "extend_for_unit" , since = "1.28.0")] impl Extend < () > for () { fn extend < T : IntoIterator < Item = () > > (& mut self , iter : T) { iter . into_iter () . for_each (drop) } fn extend_one (& mut self , _item : ()) { } }}}
mkitem!{macro_rules ! spec_tuple_impl { (($ ty_name : ident , $ var_name : ident , $ extend_ty_name : ident , $ trait_name : ident , $ default_fn_name : ident , $ cnt : tt) ,) => { spec_tuple_impl ! ($ trait_name , $ default_fn_name , #[doc (fake_variadic)] #[doc = "This trait is implemented for tuples up to twelve items long. The `impl`s for \
                     1- and 3- through 12-ary tuples were stabilized after 2-tuples, in \
                     1.85.0."] => ($ ty_name , $ var_name , $ extend_ty_name , $ cnt) ,) ; } ; (($ ty_name : ident , $ var_name : ident , $ extend_ty_name : ident , $ trait_name : ident , $ default_fn_name : ident , $ cnt : tt) , $ (($ ty_names : ident , $ var_names : ident , $ extend_ty_names : ident , $ trait_names : ident , $ default_fn_names : ident , $ cnts : tt) ,) *) => { spec_tuple_impl ! ($ (($ ty_names , $ var_names , $ extend_ty_names , $ trait_names , $ default_fn_names , $ cnts) ,) *) ; spec_tuple_impl ! ($ trait_name , $ default_fn_name , #[doc (hidden)] => ($ ty_name , $ var_name , $ extend_ty_name , $ cnt) , $ (($ ty_names , $ var_names , $ extend_ty_names , $ cnts) ,) *) ; } ; ($ trait_name : ident , $ default_fn_name : ident , #[$ meta : meta] $ (#[$ doctext : meta]) ? => $ (($ ty_names : ident , $ var_names : ident , $ extend_ty_names : ident , $ cnts : tt) ,) *) => { #[$ meta] $ (#[$ doctext]) ? #[stable (feature = "extend_for_tuple" , since = "1.56.0")] impl <$ ($ ty_names ,) * $ ($ extend_ty_names ,) *> Extend < ($ ($ ty_names ,) *) > for ($ ($ extend_ty_names ,) *) where $ ($ extend_ty_names : Extend <$ ty_names >,) * { #[doc = " Allows to `extend` a tuple of collections that also implement `Extend`."] #[doc = ""] #[doc = " See also: [`Iterator::unzip`]"] #[doc = ""] #[doc = " # Examples"] #[doc = " ```"] #[doc = " // Example given for a 2-tuple, but 1- through 12-tuples are supported"] #[doc = " let mut tuple = (vec![0], vec![1]);"] #[doc = " tuple.extend([(2, 3), (4, 5), (6, 7)]);"] #[doc = " assert_eq!(tuple.0, [0, 2, 4, 6]);"] #[doc = " assert_eq!(tuple.1, [1, 3, 5, 7]);"] #[doc = ""] #[doc = " // also allows for arbitrarily nested tuples as elements"] #[doc = " let mut nested_tuple = (vec![1], (vec![2], vec![3]));"] #[doc = " nested_tuple.extend([(4, (5, 6)), (7, (8, 9))]);"] #[doc = ""] #[doc = " let (a, (b, c)) = nested_tuple;"] #[doc = " assert_eq!(a, [1, 4, 7]);"] #[doc = " assert_eq!(b, [2, 5, 8]);"] #[doc = " assert_eq!(c, [3, 6, 9]);"] #[doc = " ```"] fn extend < T : IntoIterator < Item = ($ ($ ty_names ,) *) >> (& mut self , into_iter : T) { let ($ ($ var_names ,) *) = self ; let iter = into_iter . into_iter () ; $ trait_name :: extend (iter , $ ($ var_names ,) *) ; } fn extend_one (& mut self , item : ($ ($ ty_names ,) *)) { $ (self .$ cnts . extend_one (item .$ cnts) ;) * } fn extend_reserve (& mut self , additional : usize) { $ (self .$ cnts . extend_reserve (additional) ;) * } unsafe fn extend_one_unchecked (& mut self , item : ($ ($ ty_names ,) *)) { unsafe { $ (self .$ cnts . extend_one_unchecked (item .$ cnts) ;) * } } } trait $ trait_name <$ ($ ty_names) ,*> { fn extend (self , $ ($ var_names : & mut $ ty_names ,) *) ; } fn $ default_fn_name <$ ($ ty_names ,) * $ ($ extend_ty_names ,) *> (iter : impl Iterator < Item = ($ ($ ty_names ,) *) >, $ ($ var_names : & mut $ extend_ty_names ,) *) where $ ($ extend_ty_names : Extend <$ ty_names >,) * { fn extend <'a , $ ($ ty_names ,) *> ($ ($ var_names : &'a mut impl Extend <$ ty_names >,) *) -> impl FnMut (() , ($ ($ ty_names ,) *)) + 'a { #[allow (non_snake_case)] move | () , ($ ($ extend_ty_names ,) *) | { $ ($ var_names . extend_one ($ extend_ty_names) ;) * } } let (lower_bound , _) = iter . size_hint () ; if lower_bound > 0 { $ ($ var_names . extend_reserve (lower_bound) ;) * } iter . fold (() , extend ($ ($ var_names ,) *)) ; } impl <$ ($ ty_names ,) * $ ($ extend_ty_names ,) * Iter > $ trait_name <$ ($ extend_ty_names) ,*> for Iter where $ ($ extend_ty_names : Extend <$ ty_names >,) * Iter : Iterator < Item = ($ ($ ty_names ,) *) >, { default fn extend (self , $ ($ var_names : & mut $ extend_ty_names) ,*) { $ default_fn_name (self , $ ($ var_names) ,*) ; } } impl <$ ($ ty_names ,) * $ ($ extend_ty_names ,) * Iter > $ trait_name <$ ($ extend_ty_names) ,*> for Iter where $ ($ extend_ty_names : Extend <$ ty_names >,) * Iter : TrustedLen < Item = ($ ($ ty_names ,) *) >, { fn extend (self , $ ($ var_names : & mut $ extend_ty_names ,) *) { fn extend <'a , $ ($ ty_names ,) *> ($ ($ var_names : &'a mut impl Extend <$ ty_names >,) *) -> impl FnMut (() , ($ ($ ty_names ,) *)) + 'a { #[allow (non_snake_case)] move | () , ($ ($ extend_ty_names ,) *) | unsafe { $ ($ var_names . extend_one_unchecked ($ extend_ty_names) ;) * } } let (lower_bound , upper_bound) = self . size_hint () ; if upper_bound . is_none () { $ default_fn_name (self , $ ($ var_names ,) *) ; return ; } if lower_bound > 0 { $ ($ var_names . extend_reserve (lower_bound) ;) * } self . fold (() , extend ($ ($ var_names ,) *)) ; } } #[doc = " This implementation turns an iterator of tuples into a tuple of types which implement"] #[doc = " [`Default`] and [`Extend`]."] #[doc = ""] #[doc = " This is similar to [`Iterator::unzip`], but is also composable with other [`FromIterator`]"] #[doc = " implementations:"] #[doc = ""] #[doc = " ```rust"] #[doc = " # fn main() -> Result<(), core::num::ParseIntError> {"] #[doc = " let string = \"1,2,123,4\";"] #[doc = ""] #[doc = " // Example given for a 2-tuple, but 1- through 12-tuples are supported"] #[doc = " let (numbers, lengths): (Vec<_>, Vec<_>) = string"] #[doc = "     .split(',')"] #[doc = "     .map(|s| s.parse().map(|n: u32| (n, s.len())))"] #[doc = "     .collect::<Result<_, _>>()?;"] #[doc = ""] #[doc = " assert_eq!(numbers, [1, 2, 123, 4]);"] #[doc = " assert_eq!(lengths, [1, 1, 3, 1]);"] #[doc = " # Ok(()) }"] #[doc = " ```"] #[$ meta] $ (#[$ doctext]) ? #[stable (feature = "from_iterator_for_tuple" , since = "1.79.0")] impl <$ ($ ty_names ,) * $ ($ extend_ty_names ,) *> FromIterator < ($ ($ extend_ty_names ,) *) > for ($ ($ ty_names ,) *) where $ ($ ty_names : Default + Extend <$ extend_ty_names >,) * { fn from_iter < Iter : IntoIterator < Item = ($ ($ extend_ty_names ,) *) >> (iter : Iter) -> Self { let mut res = < ($ ($ ty_names ,) *) >:: default () ; res . extend (iter) ; res } } } ; }}
mkitem!{spec_tuple_impl ! ((L , l , EL , TraitL , default_extend_tuple_l , 11) , (K , k , EK , TraitK , default_extend_tuple_k , 10) , (J , j , EJ , TraitJ , default_extend_tuple_j , 9) , (I , i , EI , TraitI , default_extend_tuple_i , 8) , (H , h , EH , TraitH , default_extend_tuple_h , 7) , (G , g , EG , TraitG , default_extend_tuple_g , 6) , (F , f , EF , TraitF , default_extend_tuple_f , 5) , (E , e , EE , TraitE , default_extend_tuple_e , 4) , (D , d , ED , TraitD , default_extend_tuple_d , 3) , (C , c , EC , TraitC , default_extend_tuple_c , 2) , (B , b , EB , TraitB , default_extend_tuple_b , 1) , (A , a , EA , TraitA , default_extend_tuple_a , 0) ,) ;}