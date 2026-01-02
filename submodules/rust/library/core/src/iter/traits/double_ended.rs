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
mkuse!{use crate :: num :: NonZero ;}
mkuse!{use crate :: ops :: { ControlFlow , Try } ;}
mkitem!{mktrait!{# [doc = " An iterator able to yield elements from both ends."] # [doc = ""] # [doc = " Something that implements `DoubleEndedIterator` has one extra capability"] # [doc = " over something that implements [`Iterator`]: the ability to also take"] # [doc = " `Item`s from the back, as well as the front."] # [doc = ""] # [doc = " It is important to note that both back and forth work on the same range,"] # [doc = " and do not cross: iteration is over when they meet in the middle."] # [doc = ""] # [doc = " In a similar fashion to the [`Iterator`] protocol, once a"] # [doc = " `DoubleEndedIterator` returns [`None`] from a [`next_back()`], calling it"] # [doc = " again may or may not ever return [`Some`] again. [`next()`] and"] # [doc = " [`next_back()`] are interchangeable for this purpose."] # [doc = ""] # [doc = " [`next_back()`]: DoubleEndedIterator::next_back"] # [doc = " [`next()`]: Iterator::next"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " let numbers = vec![1, 2, 3, 4, 5, 6];"] # [doc = ""] # [doc = " let mut iter = numbers.iter();"] # [doc = ""] # [doc = " assert_eq!(Some(&1), iter.next());"] # [doc = " assert_eq!(Some(&6), iter.next_back());"] # [doc = " assert_eq!(Some(&5), iter.next_back());"] # [doc = " assert_eq!(Some(&2), iter.next());"] # [doc = " assert_eq!(Some(&3), iter.next());"] # [doc = " assert_eq!(Some(&4), iter.next());"] # [doc = " assert_eq!(None, iter.next());"] # [doc = " assert_eq!(None, iter.next_back());"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [rustc_diagnostic_item = "DoubleEndedIterator"] pub trait DoubleEndedIterator : Iterator { # [doc = " Removes and returns an element from the end of the iterator."] # [doc = ""] # [doc = " Returns `None` when there are no more elements."] # [doc = ""] # [doc = " The [trait-level] docs contain more details."] # [doc = ""] # [doc = " [trait-level]: DoubleEndedIterator"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " let numbers = vec![1, 2, 3, 4, 5, 6];"] # [doc = ""] # [doc = " let mut iter = numbers.iter();"] # [doc = ""] # [doc = " assert_eq!(Some(&1), iter.next());"] # [doc = " assert_eq!(Some(&6), iter.next_back());"] # [doc = " assert_eq!(Some(&5), iter.next_back());"] # [doc = " assert_eq!(Some(&2), iter.next());"] # [doc = " assert_eq!(Some(&3), iter.next());"] # [doc = " assert_eq!(Some(&4), iter.next());"] # [doc = " assert_eq!(None, iter.next());"] # [doc = " assert_eq!(None, iter.next_back());"] # [doc = " ```"] # [doc = ""] # [doc = " # Remarks"] # [doc = ""] # [doc = " The elements yielded by `DoubleEndedIterator`'s methods may differ from"] # [doc = " the ones yielded by [`Iterator`]'s methods:"] # [doc = ""] # [doc = " ```"] # [doc = " let vec = vec![(1, 'a'), (1, 'b'), (1, 'c'), (2, 'a'), (2, 'b')];"] # [doc = " let uniq_by_fst_comp = || {"] # [doc = "     let mut seen = std::collections::HashSet::new();"] # [doc = "     vec.iter().copied().filter(move |x| seen.insert(x.0))"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(uniq_by_fst_comp().last(), Some((2, 'a')));"] # [doc = " assert_eq!(uniq_by_fst_comp().next_back(), Some((2, 'b')));"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     uniq_by_fst_comp().fold(vec![], |mut v, x| {v.push(x); v}),"] # [doc = "     vec![(1, 'a'), (2, 'a')]"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     uniq_by_fst_comp().rfold(vec![], |mut v, x| {v.push(x); v}),"] # [doc = "     vec![(2, 'b'), (1, 'c')]"] # [doc = " );"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] fn next_back (& mut self) -> Option < Self :: Item > ; # [doc = " Advances the iterator from the back by `n` elements."] # [doc = ""] # [doc = " `advance_back_by` is the reverse version of [`advance_by`]. This method will"] # [doc = " eagerly skip `n` elements starting from the back by calling [`next_back`] up"] # [doc = " to `n` times until [`None`] is encountered."] # [doc = ""] # [doc = " `advance_back_by(n)` will return `Ok(())` if the iterator successfully advances by"] # [doc = " `n` elements, or a `Err(NonZero<usize>)` with value `k` if [`None`] is encountered, where `k`"] # [doc = " is remaining number of steps that could not be advanced because the iterator ran out."] # [doc = " If `self` is empty and `n` is non-zero, then this returns `Err(n)`."] # [doc = " Otherwise, `k` is always less than `n`."] # [doc = ""] # [doc = " Calling `advance_back_by(0)` can do meaningful work, for example [`Flatten`] can advance its"] # [doc = " outer iterator until it finds an inner iterator that is not empty, which then often"] # [doc = " allows it to return a more accurate `size_hint()` than in its initial state."] # [doc = ""] # [doc = " [`advance_by`]: Iterator::advance_by"] # [doc = " [`Flatten`]: crate::iter::Flatten"] # [doc = " [`next_back`]: DoubleEndedIterator::next_back"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(iter_advance_by)]"] # [doc = ""] # [doc = " use std::num::NonZero;"] # [doc = ""] # [doc = " let a = [3, 4, 5, 6];"] # [doc = " let mut iter = a.iter();"] # [doc = ""] # [doc = " assert_eq!(iter.advance_back_by(2), Ok(()));"] # [doc = " assert_eq!(iter.next_back(), Some(&4));"] # [doc = " assert_eq!(iter.advance_back_by(0), Ok(()));"] # [doc = " assert_eq!(iter.advance_back_by(100), Err(NonZero::new(99).unwrap())); // only `&3` was skipped"] # [doc = " ```"] # [doc = ""] # [doc = " [`Ok(())`]: Ok"] # [doc = " [`Err(k)`]: Err"] # [inline] # [unstable (feature = "iter_advance_by" , reason = "recently added" , issue = "77404")] fn advance_back_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { for i in 0 .. n { if self . next_back () . is_none () { return Err (unsafe { NonZero :: new_unchecked (n - i) }) ; } } Ok (()) } # [doc = " Returns the `n`th element from the end of the iterator."] # [doc = ""] # [doc = " This is essentially the reversed version of [`Iterator::nth()`]."] # [doc = " Although like most indexing operations, the count starts from zero, so"] # [doc = " `nth_back(0)` returns the first value from the end, `nth_back(1)` the"] # [doc = " second, and so on."] # [doc = ""] # [doc = " Note that all elements between the end and the returned element will be"] # [doc = " consumed, including the returned element. This also means that calling"] # [doc = " `nth_back(0)` multiple times on the same iterator will return different"] # [doc = " elements."] # [doc = ""] # [doc = " `nth_back()` will return [`None`] if `n` is greater than or equal to the"] # [doc = " length of the iterator."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " let a = [1, 2, 3];"] # [doc = " assert_eq!(a.iter().nth_back(2), Some(&1));"] # [doc = " ```"] # [doc = ""] # [doc = " Calling `nth_back()` multiple times doesn't rewind the iterator:"] # [doc = ""] # [doc = " ```"] # [doc = " let a = [1, 2, 3];"] # [doc = ""] # [doc = " let mut iter = a.iter();"] # [doc = ""] # [doc = " assert_eq!(iter.nth_back(1), Some(&2));"] # [doc = " assert_eq!(iter.nth_back(1), None);"] # [doc = " ```"] # [doc = ""] # [doc = " Returning `None` if there are less than `n + 1` elements:"] # [doc = ""] # [doc = " ```"] # [doc = " let a = [1, 2, 3];"] # [doc = " assert_eq!(a.iter().nth_back(10), None);"] # [doc = " ```"] # [inline] # [stable (feature = "iter_nth_back" , since = "1.37.0")] fn nth_back (& mut self , n : usize) -> Option < Self :: Item > { if self . advance_back_by (n) . is_err () { return None ; } self . next_back () } # [doc = " This is the reverse version of [`Iterator::try_fold()`]: it takes"] # [doc = " elements starting from the back of the iterator."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " let a = [\"1\", \"2\", \"3\"];"] # [doc = " let sum = a.iter()"] # [doc = "     .map(|&s| s.parse::<i32>())"] # [doc = "     .try_rfold(0, |acc, x| x.and_then(|y| Ok(acc + y)));"] # [doc = " assert_eq!(sum, Ok(6));"] # [doc = " ```"] # [doc = ""] # [doc = " Short-circuiting:"] # [doc = ""] # [doc = " ```"] # [doc = " let a = [\"1\", \"rust\", \"3\"];"] # [doc = " let mut it = a.iter();"] # [doc = " let sum = it"] # [doc = "     .by_ref()"] # [doc = "     .map(|&s| s.parse::<i32>())"] # [doc = "     .try_rfold(0, |acc, x| x.and_then(|y| Ok(acc + y)));"] # [doc = " assert!(sum.is_err());"] # [doc = ""] # [doc = " // Because it short-circuited, the remaining elements are still"] # [doc = " // available through the iterator."] # [doc = " assert_eq!(it.next_back(), Some(&\"1\"));"] # [doc = " ```"] # [inline] # [stable (feature = "iterator_try_fold" , since = "1.27.0")] fn try_rfold < B , F , R > (& mut self , init : B , mut f : F) -> R where Self : Sized , F : FnMut (B , Self :: Item) -> R , R : Try < Output = B > , { let mut accum = init ; while let Some (x) = self . next_back () { accum = f (accum , x) ? ; } try { accum } } # [doc = " An iterator method that reduces the iterator's elements to a single,"] # [doc = " final value, starting from the back."] # [doc = ""] # [doc = " This is the reverse version of [`Iterator::fold()`]: it takes elements"] # [doc = " starting from the back of the iterator."] # [doc = ""] # [doc = " `rfold()` takes two arguments: an initial value, and a closure with two"] # [doc = " arguments: an 'accumulator', and an element. The closure returns the value that"] # [doc = " the accumulator should have for the next iteration."] # [doc = ""] # [doc = " The initial value is the value the accumulator will have on the first"] # [doc = " call."] # [doc = ""] # [doc = " After applying this closure to every element of the iterator, `rfold()`"] # [doc = " returns the accumulator."] # [doc = ""] # [doc = " This operation is sometimes called 'reduce' or 'inject'."] # [doc = ""] # [doc = " Folding is useful whenever you have a collection of something, and want"] # [doc = " to produce a single value from it."] # [doc = ""] # [doc = " Note: `rfold()` combines elements in a *right-associative* fashion. For associative"] # [doc = " operators like `+`, the order the elements are combined in is not important, but for non-associative"] # [doc = " operators like `-` the order will affect the final result."] # [doc = " For a *left-associative* version of `rfold()`, see [`Iterator::fold()`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " let a = [1, 2, 3];"] # [doc = ""] # [doc = " // the sum of all of the elements of a"] # [doc = " let sum = a.iter()"] # [doc = "            .rfold(0, |acc, &x| acc + x);"] # [doc = ""] # [doc = " assert_eq!(sum, 6);"] # [doc = " ```"] # [doc = ""] # [doc = " This example demonstrates the right-associative nature of `rfold()`:"] # [doc = " it builds a string, starting with an initial value"] # [doc = " and continuing with each element from the back until the front:"] # [doc = ""] # [doc = " ```"] # [doc = " let numbers = [1, 2, 3, 4, 5];"] # [doc = ""] # [doc = " let zero = \"0\".to_string();"] # [doc = ""] # [doc = " let result = numbers.iter().rfold(zero, |acc, &x| {"] # [doc = "     format!(\"({x} + {acc})\")"] # [doc = " });"] # [doc = ""] # [doc = " assert_eq!(result, \"(1 + (2 + (3 + (4 + (5 + 0)))))\");"] # [doc = " ```"] # [doc (alias = "foldr")] # [inline] # [stable (feature = "iter_rfold" , since = "1.27.0")] fn rfold < B , F > (mut self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { let mut accum = init ; while let Some (x) = self . next_back () { accum = f (accum , x) ; } accum } # [doc = " Searches for an element of an iterator from the back that satisfies a predicate."] # [doc = ""] # [doc = " `rfind()` takes a closure that returns `true` or `false`. It applies"] # [doc = " this closure to each element of the iterator, starting at the end, and if any"] # [doc = " of them return `true`, then `rfind()` returns [`Some(element)`]. If they all return"] # [doc = " `false`, it returns [`None`]."] # [doc = ""] # [doc = " `rfind()` is short-circuiting; in other words, it will stop processing"] # [doc = " as soon as the closure returns `true`."] # [doc = ""] # [doc = " Because `rfind()` takes a reference, and many iterators iterate over"] # [doc = " references, this leads to a possibly confusing situation where the"] # [doc = " argument is a double reference. You can see this effect in the"] # [doc = " examples below, with `&&x`."] # [doc = ""] # [doc = " [`Some(element)`]: Some"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " let a = [1, 2, 3];"] # [doc = ""] # [doc = " assert_eq!(a.iter().rfind(|&&x| x == 2), Some(&2));"] # [doc = ""] # [doc = " assert_eq!(a.iter().rfind(|&&x| x == 5), None);"] # [doc = " ```"] # [doc = ""] # [doc = " Stopping at the first `true`:"] # [doc = ""] # [doc = " ```"] # [doc = " let a = [1, 2, 3];"] # [doc = ""] # [doc = " let mut iter = a.iter();"] # [doc = ""] # [doc = " assert_eq!(iter.rfind(|&&x| x == 2), Some(&2));"] # [doc = ""] # [doc = " // we can still use `iter`, as there are more elements."] # [doc = " assert_eq!(iter.next_back(), Some(&1));"] # [doc = " ```"] # [inline] # [stable (feature = "iter_rfind" , since = "1.27.0")] fn rfind < P > (& mut self , predicate : P) -> Option < Self :: Item > where Self : Sized , P : FnMut (& Self :: Item) -> bool , { # [inline] fn check < T > (mut predicate : impl FnMut (& T) -> bool) -> impl FnMut (() , T) -> ControlFlow < T > { move | () , x | { if predicate (& x) { ControlFlow :: Break (x) } else { ControlFlow :: Continue (()) } } } self . try_rfold (() , check (predicate)) . break_value () } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , I : DoubleEndedIterator + ? Sized > DoubleEndedIterator for & 'a mut I { fn next_back (& mut self) -> Option < I :: Item > { (* * self) . next_back () } fn advance_back_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { (* * self) . advance_back_by (n) } fn nth_back (& mut self , n : usize) -> Option < I :: Item > { (* * self) . nth_back (n) } fn rfold < B , F > (self , init : B , f : F) -> B where F : FnMut (B , Self :: Item) -> B , { self . spec_rfold (init , f) } fn try_rfold < B , F , R > (& mut self , init : B , f : F) -> R where F : FnMut (B , Self :: Item) -> R , R : Try < Output = B > , { self . spec_try_rfold (init , f) } }}}
mkitem!{mktrait!{# [doc = " Helper trait to specialize `rfold` and `rtry_fold` for `&mut I where I: Sized`"] trait DoubleEndedIteratorRefSpec : DoubleEndedIterator { fn spec_rfold < B , F > (self , init : B , f : F) -> B where F : FnMut (B , Self :: Item) -> B ; fn spec_try_rfold < B , F , R > (& mut self , init : B , f : F) -> R where F : FnMut (B , Self :: Item) -> R , R : Try < Output = B > ; }}}
mkitem!{mkimpl!{impl < I : DoubleEndedIterator + ? Sized > DoubleEndedIteratorRefSpec for & mut I { default fn spec_rfold < B , F > (self , init : B , mut f : F) -> B where F : FnMut (B , Self :: Item) -> B , { let mut accum = init ; while let Some (x) = self . next_back () { accum = f (accum , x) ; } accum } default fn spec_try_rfold < B , F , R > (& mut self , init : B , mut f : F) -> R where F : FnMut (B , Self :: Item) -> R , R : Try < Output = B > , { let mut accum = init ; while let Some (x) = self . next_back () { accum = f (accum , x) ? ; } try { accum } } }}}
mkitem!{mkimpl!{impl < I : DoubleEndedIterator > DoubleEndedIteratorRefSpec for & mut I { impl_fold_via_try_fold ! { spec_rfold -> spec_try_rfold } fn spec_try_rfold < B , F , R > (& mut self , init : B , f : F) -> R where F : FnMut (B , Self :: Item) -> R , R : Try < Output = B > , { (* * self) . try_rfold (init , f) } }}}