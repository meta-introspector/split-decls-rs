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
mkuse!{use crate :: iter ;}
mkuse!{use crate :: num :: { Saturating , Wrapping } ;}
mkitem!{mktrait!{#[doc = " Trait to represent types that can be created by summing up an iterator."] #[doc = ""] #[doc = " This trait is used to implement [`Iterator::sum()`]. Types which implement"] #[doc = " this trait can be generated by using the [`sum()`] method on an iterator."] #[doc = " Like [`FromIterator`], this trait should rarely be called directly."] #[doc = ""] #[doc = " [`sum()`]: Iterator::sum"] #[doc = " [`FromIterator`]: iter::FromIterator"] #[stable (feature = "iter_arith_traits" , since = "1.12.0")] #[diagnostic :: on_unimplemented (message = "a value of type `{Self}` cannot be made by summing an iterator over elements of type `{A}`" , label = "value of type `{Self}` cannot be made by summing a `std::iter::Iterator<Item={A}>`")] pub trait Sum < A = Self > : Sized { #[doc = " Takes an iterator and generates `Self` from the elements by \"summing up\""] #[doc = " the items."] #[stable (feature = "iter_arith_traits" , since = "1.12.0")] fn sum < I : Iterator < Item = A > > (iter : I) -> Self ; }}}
mkitem!{mktrait!{#[doc = " Trait to represent types that can be created by multiplying elements of an"] #[doc = " iterator."] #[doc = ""] #[doc = " This trait is used to implement [`Iterator::product()`]. Types which implement"] #[doc = " this trait can be generated by using the [`product()`] method on an iterator."] #[doc = " Like [`FromIterator`], this trait should rarely be called directly."] #[doc = ""] #[doc = " [`product()`]: Iterator::product"] #[doc = " [`FromIterator`]: iter::FromIterator"] #[stable (feature = "iter_arith_traits" , since = "1.12.0")] #[diagnostic :: on_unimplemented (message = "a value of type `{Self}` cannot be made by multiplying all elements of type `{A}` from an iterator" , label = "value of type `{Self}` cannot be made by multiplying all elements from a `std::iter::Iterator<Item={A}>`")] pub trait Product < A = Self > : Sized { #[doc = " Takes an iterator and generates `Self` from the elements by multiplying"] #[doc = " the items."] #[stable (feature = "iter_arith_traits" , since = "1.12.0")] fn product < I : Iterator < Item = A > > (iter : I) -> Self ; }}}
mkitem!{macro_rules ! integer_sum_product { (@ impls $ zero : expr , $ one : expr , #[$ attr : meta] , $ ($ a : ty) *) => ($ (#[$ attr] impl Sum for $ a { fn sum < I : Iterator < Item = Self >> (iter : I) -> Self { iter . fold ($ zero , #[rustc_inherit_overflow_checks] | a , b | a + b ,) } } #[$ attr] impl Product for $ a { fn product < I : Iterator < Item = Self >> (iter : I) -> Self { iter . fold ($ one , #[rustc_inherit_overflow_checks] | a , b | a * b ,) } } #[$ attr] impl <'a > Sum <&'a $ a > for $ a { fn sum < I : Iterator < Item =&'a Self >> (iter : I) -> Self { iter . fold ($ zero , #[rustc_inherit_overflow_checks] | a , b | a + b ,) } } #[$ attr] impl <'a > Product <&'a $ a > for $ a { fn product < I : Iterator < Item =&'a Self >> (iter : I) -> Self { iter . fold ($ one , #[rustc_inherit_overflow_checks] | a , b | a * b ,) } }) *) ; ($ ($ a : ty) *) => (integer_sum_product ! (@ impls 0 , 1 , #[stable (feature = "iter_arith_traits" , since = "1.12.0")] , $ ($ a) *) ; integer_sum_product ! (@ impls Wrapping (0) , Wrapping (1) , #[stable (feature = "wrapping_iter_arith" , since = "1.14.0")] , $ (Wrapping <$ a >) *) ;) ; }}
mkitem!{macro_rules ! saturating_integer_sum_product { (@ impls $ zero : expr , $ one : expr , $ doc : expr , #[$ attr : meta] , $ ($ a : ty) *) => ($ (#[$ attr] #[doc = $ doc] impl Sum for $ a { fn sum < I : Iterator < Item = Self >> (iter : I) -> Self { iter . fold ($ zero , | a , b | a + b ,) } } #[$ attr] #[doc = $ doc] impl Product for $ a { fn product < I : Iterator < Item = Self >> (iter : I) -> Self { iter . fold ($ one , | a , b | a * b ,) } } #[$ attr] #[doc = $ doc] impl <'a > Sum <&'a $ a > for $ a { fn sum < I : Iterator < Item =&'a Self >> (iter : I) -> Self { iter . fold ($ zero , | a , b | a + b ,) } } #[$ attr] #[doc = $ doc] impl <'a > Product <&'a $ a > for $ a { fn product < I : Iterator < Item =&'a Self >> (iter : I) -> Self { iter . fold ($ one , | a , b | a * b ,) } }) *) ; ($ ($ a : ty) *) => (saturating_integer_sum_product ! (@ impls Saturating (0) , Saturating (1) , "The short-circuiting behavior of this implementation is unspecified. If you care about \
                short-circuiting, use [`Iterator::fold`] directly." , #[stable (feature = "saturating_iter_arith" , since = "1.91.0")] , $ (Saturating <$ a >) *) ;) ; }}
mkitem!{macro_rules ! float_sum_product { ($ ($ a : ident) *) => ($ (#[stable (feature = "iter_arith_traits" , since = "1.12.0")] impl Sum for $ a { fn sum < I : Iterator < Item = Self >> (iter : I) -> Self { iter . fold (- 0.0 , #[rustc_inherit_overflow_checks] | a , b | a + b ,) } } #[stable (feature = "iter_arith_traits" , since = "1.12.0")] impl Product for $ a { fn product < I : Iterator < Item = Self >> (iter : I) -> Self { iter . fold (1.0 , #[rustc_inherit_overflow_checks] | a , b | a * b ,) } } #[stable (feature = "iter_arith_traits" , since = "1.12.0")] impl <'a > Sum <&'a $ a > for $ a { fn sum < I : Iterator < Item =&'a Self >> (iter : I) -> Self { iter . fold (- 0.0 , #[rustc_inherit_overflow_checks] | a , b | a + b ,) } } #[stable (feature = "iter_arith_traits" , since = "1.12.0")] impl <'a > Product <&'a $ a > for $ a { fn product < I : Iterator < Item =&'a Self >> (iter : I) -> Self { iter . fold (1.0 , #[rustc_inherit_overflow_checks] | a , b | a * b ,) } }) *) }}
mkitem!{integer_sum_product ! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }}
mkitem!{saturating_integer_sum_product ! { u8 u16 u32 u64 u128 usize }}
mkitem!{float_sum_product ! { f16 f32 f64 f128 }}
mkitem!{mkimpl!{#[stable (feature = "iter_arith_traits_result" , since = "1.16.0")] impl < T , U , E > Sum < Result < U , E > > for Result < T , E > where T : Sum < U > , { #[doc = " Takes each element in the [`Iterator`]: if it is an [`Err`], no further"] #[doc = " elements are taken, and the [`Err`] is returned. Should no [`Err`]"] #[doc = " occur, the sum of all elements is returned."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " This sums up every integer in a vector, rejecting the sum if a negative"] #[doc = " element is encountered:"] #[doc = ""] #[doc = " ```"] #[doc = " let f = |&x: &i32| if x < 0 { Err(\"Negative element found\") } else { Ok(x) };"] #[doc = " let v = vec![1, 2];"] #[doc = " let res: Result<i32, _> = v.iter().map(f).sum();"] #[doc = " assert_eq!(res, Ok(3));"] #[doc = " let v = vec![1, -2];"] #[doc = " let res: Result<i32, _> = v.iter().map(f).sum();"] #[doc = " assert_eq!(res, Err(\"Negative element found\"));"] #[doc = " ```"] fn sum < I > (iter : I) -> Result < T , E > where I : Iterator < Item = Result < U , E > > , { iter :: try_process (iter , | i | i . sum ()) } }}}
mkitem!{mkimpl!{#[stable (feature = "iter_arith_traits_result" , since = "1.16.0")] impl < T , U , E > Product < Result < U , E > > for Result < T , E > where T : Product < U > , { #[doc = " Takes each element in the [`Iterator`]: if it is an [`Err`], no further"] #[doc = " elements are taken, and the [`Err`] is returned. Should no [`Err`]"] #[doc = " occur, the product of all elements is returned."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " This multiplies each number in a vector of strings,"] #[doc = " if a string could not be parsed the operation returns `Err`:"] #[doc = ""] #[doc = " ```"] #[doc = " let nums = vec![\"5\", \"10\", \"1\", \"2\"];"] #[doc = " let total: Result<usize, _> = nums.iter().map(|w| w.parse::<usize>()).product();"] #[doc = " assert_eq!(total, Ok(100));"] #[doc = " let nums = vec![\"5\", \"10\", \"one\", \"2\"];"] #[doc = " let total: Result<usize, _> = nums.iter().map(|w| w.parse::<usize>()).product();"] #[doc = " assert!(total.is_err());"] #[doc = " ```"] fn product < I > (iter : I) -> Result < T , E > where I : Iterator < Item = Result < U , E > > , { iter :: try_process (iter , | i | i . product ()) } }}}
mkitem!{mkimpl!{#[stable (feature = "iter_arith_traits_option" , since = "1.37.0")] impl < T , U > Sum < Option < U > > for Option < T > where T : Sum < U > , { #[doc = " Takes each element in the [`Iterator`]: if it is a [`None`], no further"] #[doc = " elements are taken, and the [`None`] is returned. Should no [`None`]"] #[doc = " occur, the sum of all elements is returned."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " This sums up the position of the character 'a' in a vector of strings,"] #[doc = " if a word did not have the character 'a' the operation returns `None`:"] #[doc = ""] #[doc = " ```"] #[doc = " let words = vec![\"have\", \"a\", \"great\", \"day\"];"] #[doc = " let total: Option<usize> = words.iter().map(|w| w.find('a')).sum();"] #[doc = " assert_eq!(total, Some(5));"] #[doc = " let words = vec![\"have\", \"a\", \"good\", \"day\"];"] #[doc = " let total: Option<usize> = words.iter().map(|w| w.find('a')).sum();"] #[doc = " assert_eq!(total, None);"] #[doc = " ```"] fn sum < I > (iter : I) -> Option < T > where I : Iterator < Item = Option < U > > , { iter :: try_process (iter , | i | i . sum ()) } }}}
mkitem!{mkimpl!{#[stable (feature = "iter_arith_traits_option" , since = "1.37.0")] impl < T , U > Product < Option < U > > for Option < T > where T : Product < U > , { #[doc = " Takes each element in the [`Iterator`]: if it is a [`None`], no further"] #[doc = " elements are taken, and the [`None`] is returned. Should no [`None`]"] #[doc = " occur, the product of all elements is returned."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " This multiplies each number in a vector of strings,"] #[doc = " if a string could not be parsed the operation returns `None`:"] #[doc = ""] #[doc = " ```"] #[doc = " let nums = vec![\"5\", \"10\", \"1\", \"2\"];"] #[doc = " let total: Option<usize> = nums.iter().map(|w| w.parse::<usize>().ok()).product();"] #[doc = " assert_eq!(total, Some(100));"] #[doc = " let nums = vec![\"5\", \"10\", \"one\", \"2\"];"] #[doc = " let total: Option<usize> = nums.iter().map(|w| w.parse::<usize>().ok()).product();"] #[doc = " assert_eq!(total, None);"] #[doc = " ```"] fn product < I > (iter : I) -> Option < T > where I : Iterator < Item = Option < U > > , { iter :: try_process (iter , | i | i . product ()) } }}}