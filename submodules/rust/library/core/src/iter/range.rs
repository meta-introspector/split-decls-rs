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
mkuse!{use super :: { FusedIterator , TrustedLen , TrustedRandomAccess , TrustedRandomAccessNoCoerce , TrustedStep , } ;}
mkuse!{use crate :: ascii :: Char as AsciiChar ;}
mkuse!{use crate :: mem ;}
mkuse!{use crate :: net :: { Ipv4Addr , Ipv6Addr } ;}
mkuse!{use crate :: num :: NonZero ;}
mkuse!{use crate :: ops :: { self , Try } ;}
mkitem!{macro_rules ! unsafe_impl_trusted_step { ($ ($ type : ty) *) => { $ (# [unstable (feature = "trusted_step" , issue = "85731")] unsafe impl TrustedStep for $ type { }) * } ; }}
mkitem!{unsafe_impl_trusted_step ! [AsciiChar char i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize Ipv4Addr Ipv6Addr] ;}
mkitem!{mktrait!{# [doc = " Objects that have a notion of *successor* and *predecessor* operations."] # [doc = ""] # [doc = " The *successor* operation moves towards values that compare greater."] # [doc = " The *predecessor* operation moves towards values that compare lesser."] # [rustc_diagnostic_item = "range_step"] # [unstable (feature = "step_trait" , issue = "42168")] pub trait Step : Clone + PartialOrd + Sized { # [doc = " Returns the bounds on the number of *successor* steps required to get from `start` to `end`"] # [doc = " like [`Iterator::size_hint()`][Iterator::size_hint()]."] # [doc = ""] # [doc = " Returns `(usize::MAX, None)` if the number of steps would overflow `usize`, or is infinite."] # [doc = ""] # [doc = " # Invariants"] # [doc = ""] # [doc = " For any `a`, `b`, and `n`:"] # [doc = ""] # [doc = " * `steps_between(&a, &b) == (n, Some(n))` if and only if `Step::forward_checked(&a, n) == Some(b)`"] # [doc = " * `steps_between(&a, &b) == (n, Some(n))` if and only if `Step::backward_checked(&b, n) == Some(a)`"] # [doc = " * `steps_between(&a, &b) == (n, Some(n))` only if `a <= b`"] # [doc = "   * Corollary: `steps_between(&a, &b) == (0, Some(0))` if and only if `a == b`"] # [doc = " * `steps_between(&a, &b) == (0, None)` if `a > b`"] fn steps_between (start : & Self , end : & Self) -> (usize , Option < usize >) ; # [doc = " Returns the value that would be obtained by taking the *successor*"] # [doc = " of `self` `count` times."] # [doc = ""] # [doc = " If this would overflow the range of values supported by `Self`, returns `None`."] # [doc = ""] # [doc = " # Invariants"] # [doc = ""] # [doc = " For any `a`, `n`, and `m`:"] # [doc = ""] # [doc = " * `Step::forward_checked(a, n).and_then(|x| Step::forward_checked(x, m)) == Step::forward_checked(a, m).and_then(|x| Step::forward_checked(x, n))`"] # [doc = " * `Step::forward_checked(a, n).and_then(|x| Step::forward_checked(x, m)) == try { Step::forward_checked(a, n.checked_add(m)) }`"] # [doc = ""] # [doc = " For any `a` and `n`:"] # [doc = ""] # [doc = " * `Step::forward_checked(a, n) == (0..n).try_fold(a, |x, _| Step::forward_checked(&x, 1))`"] # [doc = "   * Corollary: `Step::forward_checked(a, 0) == Some(a)`"] fn forward_checked (start : Self , count : usize) -> Option < Self > ; # [doc = " Returns the value that would be obtained by taking the *successor*"] # [doc = " of `self` `count` times."] # [doc = ""] # [doc = " If this would overflow the range of values supported by `Self`,"] # [doc = " this function is allowed to panic, wrap, or saturate."] # [doc = " The suggested behavior is to panic when debug assertions are enabled,"] # [doc = " and to wrap or saturate otherwise."] # [doc = ""] # [doc = " Unsafe code should not rely on the correctness of behavior after overflow."] # [doc = ""] # [doc = " # Invariants"] # [doc = ""] # [doc = " For any `a`, `n`, and `m`, where no overflow occurs:"] # [doc = ""] # [doc = " * `Step::forward(Step::forward(a, n), m) == Step::forward(a, n + m)`"] # [doc = ""] # [doc = " For any `a` and `n`, where no overflow occurs:"] # [doc = ""] # [doc = " * `Step::forward_checked(a, n) == Some(Step::forward(a, n))`"] # [doc = " * `Step::forward(a, n) == (0..n).fold(a, |x, _| Step::forward(x, 1))`"] # [doc = "   * Corollary: `Step::forward(a, 0) == a`"] # [doc = " * `Step::forward(a, n) >= a`"] # [doc = " * `Step::backward(Step::forward(a, n), n) == a`"] fn forward (start : Self , count : usize) -> Self { Step :: forward_checked (start , count) . expect ("overflow in `Step::forward`") } # [doc = " Returns the value that would be obtained by taking the *successor*"] # [doc = " of `self` `count` times."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " It is undefined behavior for this operation to overflow the"] # [doc = " range of values supported by `Self`. If you cannot guarantee that this"] # [doc = " will not overflow, use `forward` or `forward_checked` instead."] # [doc = ""] # [doc = " # Invariants"] # [doc = ""] # [doc = " For any `a`:"] # [doc = ""] # [doc = " * if there exists `b` such that `b > a`, it is safe to call `Step::forward_unchecked(a, 1)`"] # [doc = " * if there exists `b`, `n` such that `steps_between(&a, &b) == Some(n)`,"] # [doc = "   it is safe to call `Step::forward_unchecked(a, m)` for any `m <= n`."] # [doc = "   * Corollary: `Step::forward_unchecked(a, 0)` is always safe."] # [doc = ""] # [doc = " For any `a` and `n`, where no overflow occurs:"] # [doc = ""] # [doc = " * `Step::forward_unchecked(a, n)` is equivalent to `Step::forward(a, n)`"] unsafe fn forward_unchecked (start : Self , count : usize) -> Self { Step :: forward (start , count) } # [doc = " Returns the value that would be obtained by taking the *predecessor*"] # [doc = " of `self` `count` times."] # [doc = ""] # [doc = " If this would overflow the range of values supported by `Self`, returns `None`."] # [doc = ""] # [doc = " # Invariants"] # [doc = ""] # [doc = " For any `a`, `n`, and `m`:"] # [doc = ""] # [doc = " * `Step::backward_checked(a, n).and_then(|x| Step::backward_checked(x, m)) == n.checked_add(m).and_then(|x| Step::backward_checked(a, x))`"] # [doc = " * `Step::backward_checked(a, n).and_then(|x| Step::backward_checked(x, m)) == try { Step::backward_checked(a, n.checked_add(m)?) }`"] # [doc = ""] # [doc = " For any `a` and `n`:"] # [doc = ""] # [doc = " * `Step::backward_checked(a, n) == (0..n).try_fold(a, |x, _| Step::backward_checked(x, 1))`"] # [doc = "   * Corollary: `Step::backward_checked(a, 0) == Some(a)`"] fn backward_checked (start : Self , count : usize) -> Option < Self > ; # [doc = " Returns the value that would be obtained by taking the *predecessor*"] # [doc = " of `self` `count` times."] # [doc = ""] # [doc = " If this would overflow the range of values supported by `Self`,"] # [doc = " this function is allowed to panic, wrap, or saturate."] # [doc = " The suggested behavior is to panic when debug assertions are enabled,"] # [doc = " and to wrap or saturate otherwise."] # [doc = ""] # [doc = " Unsafe code should not rely on the correctness of behavior after overflow."] # [doc = ""] # [doc = " # Invariants"] # [doc = ""] # [doc = " For any `a`, `n`, and `m`, where no overflow occurs:"] # [doc = ""] # [doc = " * `Step::backward(Step::backward(a, n), m) == Step::backward(a, n + m)`"] # [doc = ""] # [doc = " For any `a` and `n`, where no overflow occurs:"] # [doc = ""] # [doc = " * `Step::backward_checked(a, n) == Some(Step::backward(a, n))`"] # [doc = " * `Step::backward(a, n) == (0..n).fold(a, |x, _| Step::backward(x, 1))`"] # [doc = "   * Corollary: `Step::backward(a, 0) == a`"] # [doc = " * `Step::backward(a, n) <= a`"] # [doc = " * `Step::forward(Step::backward(a, n), n) == a`"] fn backward (start : Self , count : usize) -> Self { Step :: backward_checked (start , count) . expect ("overflow in `Step::backward`") } # [doc = " Returns the value that would be obtained by taking the *predecessor*"] # [doc = " of `self` `count` times."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " It is undefined behavior for this operation to overflow the"] # [doc = " range of values supported by `Self`. If you cannot guarantee that this"] # [doc = " will not overflow, use `backward` or `backward_checked` instead."] # [doc = ""] # [doc = " # Invariants"] # [doc = ""] # [doc = " For any `a`:"] # [doc = ""] # [doc = " * if there exists `b` such that `b < a`, it is safe to call `Step::backward_unchecked(a, 1)`"] # [doc = " * if there exists `b`, `n` such that `steps_between(&b, &a) == (n, Some(n))`,"] # [doc = "   it is safe to call `Step::backward_unchecked(a, m)` for any `m <= n`."] # [doc = "   * Corollary: `Step::backward_unchecked(a, 0)` is always safe."] # [doc = ""] # [doc = " For any `a` and `n`, where no overflow occurs:"] # [doc = ""] # [doc = " * `Step::backward_unchecked(a, n)` is equivalent to `Step::backward(a, n)`"] unsafe fn backward_unchecked (start : Self , count : usize) -> Self { Step :: backward (start , count) } }}}
mkitem!{macro_rules ! step_signed_methods { ($ unsigned : ty) => { # [inline] unsafe fn forward_unchecked (start : Self , n : usize) -> Self { unsafe { start . checked_add_unsigned (n as $ unsigned) . unwrap_unchecked () } } # [inline] unsafe fn backward_unchecked (start : Self , n : usize) -> Self { unsafe { start . checked_sub_unsigned (n as $ unsigned) . unwrap_unchecked () } } } ; }}
mkitem!{macro_rules ! step_unsigned_methods { () => { # [inline] unsafe fn forward_unchecked (start : Self , n : usize) -> Self { unsafe { start . unchecked_add (n as Self) } } # [inline] unsafe fn backward_unchecked (start : Self , n : usize) -> Self { unsafe { start . unchecked_sub (n as Self) } } } ; }}
mkitem!{macro_rules ! step_identical_methods { () => { # [inline] # [allow (arithmetic_overflow)] # [rustc_inherit_overflow_checks] fn forward (start : Self , n : usize) -> Self { if Self :: forward_checked (start , n) . is_none () { let _ = Self :: MAX + 1 ; } start . wrapping_add (n as Self) } # [inline] # [allow (arithmetic_overflow)] # [rustc_inherit_overflow_checks] fn backward (start : Self , n : usize) -> Self { if Self :: backward_checked (start , n) . is_none () { let _ = Self :: MIN - 1 ; } start . wrapping_sub (n as Self) } } ; }}
mkitem!{macro_rules ! step_integer_impls { { narrower than or same width as usize : $ ([$ u_narrower : ident $ i_narrower : ident]) ,+; wider than usize : $ ([$ u_wider : ident $ i_wider : ident]) ,+; } => { $ (# [allow (unreachable_patterns)] # [unstable (feature = "step_trait" , reason = "recently redesigned" , issue = "42168")] impl Step for $ u_narrower { step_identical_methods ! () ; step_unsigned_methods ! () ; # [inline] fn steps_between (start : & Self , end : & Self) -> (usize , Option < usize >) { if * start <= * end { let steps = (* end - * start) as usize ; (steps , Some (steps)) } else { (0 , None) } } # [inline] fn forward_checked (start : Self , n : usize) -> Option < Self > { match Self :: try_from (n) { Ok (n) => start . checked_add (n) , Err (_) => None , } } # [inline] fn backward_checked (start : Self , n : usize) -> Option < Self > { match Self :: try_from (n) { Ok (n) => start . checked_sub (n) , Err (_) => None , } } } # [allow (unreachable_patterns)] # [unstable (feature = "step_trait" , reason = "recently redesigned" , issue = "42168")] impl Step for $ i_narrower { step_identical_methods ! () ; step_signed_methods ! ($ u_narrower) ; # [inline] fn steps_between (start : & Self , end : & Self) -> (usize , Option < usize >) { if * start <= * end { let steps = (* end as isize) . wrapping_sub (* start as isize) as usize ; (steps , Some (steps)) } else { (0 , None) } } # [inline] fn forward_checked (start : Self , n : usize) -> Option < Self > { match $ u_narrower :: try_from (n) { Ok (n) => { let wrapped = start . wrapping_add (n as Self) ; if wrapped >= start { Some (wrapped) } else { None } } Err (_) => None , } } # [inline] fn backward_checked (start : Self , n : usize) -> Option < Self > { match $ u_narrower :: try_from (n) { Ok (n) => { let wrapped = start . wrapping_sub (n as Self) ; if wrapped <= start { Some (wrapped) } else { None } } Err (_) => None , } } }) + $ (# [allow (unreachable_patterns)] # [unstable (feature = "step_trait" , reason = "recently redesigned" , issue = "42168")] impl Step for $ u_wider { step_identical_methods ! () ; step_unsigned_methods ! () ; # [inline] fn steps_between (start : & Self , end : & Self) -> (usize , Option < usize >) { if * start <= * end { if let Ok (steps) = usize :: try_from (* end - * start) { (steps , Some (steps)) } else { (usize :: MAX , None) } } else { (0 , None) } } # [inline] fn forward_checked (start : Self , n : usize) -> Option < Self > { start . checked_add (n as Self) } # [inline] fn backward_checked (start : Self , n : usize) -> Option < Self > { start . checked_sub (n as Self) } } # [allow (unreachable_patterns)] # [unstable (feature = "step_trait" , reason = "recently redesigned" , issue = "42168")] impl Step for $ i_wider { step_identical_methods ! () ; step_signed_methods ! ($ u_wider) ; # [inline] fn steps_between (start : & Self , end : & Self) -> (usize , Option < usize >) { if * start <= * end { match end . checked_sub (* start) { Some (result) => { if let Ok (steps) = usize :: try_from (result) { (steps , Some (steps)) } else { (usize :: MAX , None) } } None => (usize :: MAX , None) , } } else { (0 , None) } } # [inline] fn forward_checked (start : Self , n : usize) -> Option < Self > { start . checked_add (n as Self) } # [inline] fn backward_checked (start : Self , n : usize) -> Option < Self > { start . checked_sub (n as Self) } }) + } ; }}
mkitem!{# [cfg (target_pointer_width = "64")] step_integer_impls ! { narrower than or same width as usize : [u8 i8] , [u16 i16] , [u32 i32] , [u64 i64] , [usize isize] ; wider than usize : [u128 i128] ; }}
mkitem!{# [cfg (target_pointer_width = "32")] step_integer_impls ! { narrower than or same width as usize : [u8 i8] , [u16 i16] , [u32 i32] , [usize isize] ; wider than usize : [u64 i64] , [u128 i128] ; }}
mkitem!{# [cfg (target_pointer_width = "16")] step_integer_impls ! { narrower than or same width as usize : [u8 i8] , [u16 i16] , [usize isize] ; wider than usize : [u32 i32] , [u64 i64] , [u128 i128] ; }}
mkitem!{mkimpl!{# [unstable (feature = "step_trait" , reason = "recently redesigned" , issue = "42168")] impl Step for char { # [inline] fn steps_between (& start : & char , & end : & char) -> (usize , Option < usize >) { let start = start as u32 ; let end = end as u32 ; if start <= end { let count = end - start ; if start < 0xD800 && 0xE000 <= end { if let Ok (steps) = usize :: try_from (count - 0x800) { (steps , Some (steps)) } else { (usize :: MAX , None) } } else { if let Ok (steps) = usize :: try_from (count) { (steps , Some (steps)) } else { (usize :: MAX , None) } } } else { (0 , None) } } # [inline] fn forward_checked (start : char , count : usize) -> Option < char > { let start = start as u32 ; let mut res = Step :: forward_checked (start , count) ? ; if start < 0xD800 && 0xD800 <= res { res = Step :: forward_checked (res , 0x800) ? ; } if res <= char :: MAX as u32 { Some (unsafe { char :: from_u32_unchecked (res) }) } else { None } } # [inline] fn backward_checked (start : char , count : usize) -> Option < char > { let start = start as u32 ; let mut res = Step :: backward_checked (start , count) ? ; if start >= 0xE000 && 0xE000 > res { res = Step :: backward_checked (res , 0x800) ? ; } Some (unsafe { char :: from_u32_unchecked (res) }) } # [inline] unsafe fn forward_unchecked (start : char , count : usize) -> char { let start = start as u32 ; let mut res = unsafe { Step :: forward_unchecked (start , count) } ; if start < 0xD800 && 0xD800 <= res { res = unsafe { Step :: forward_unchecked (res , 0x800) } ; } unsafe { char :: from_u32_unchecked (res) } } # [inline] unsafe fn backward_unchecked (start : char , count : usize) -> char { let start = start as u32 ; let mut res = unsafe { Step :: backward_unchecked (start , count) } ; if start >= 0xE000 && 0xE000 > res { res = unsafe { Step :: backward_unchecked (res , 0x800) } ; } unsafe { char :: from_u32_unchecked (res) } } }}}
mkitem!{mkimpl!{# [unstable (feature = "step_trait" , reason = "recently redesigned" , issue = "42168")] impl Step for AsciiChar { # [inline] fn steps_between (& start : & AsciiChar , & end : & AsciiChar) -> (usize , Option < usize >) { Step :: steps_between (& start . to_u8 () , & end . to_u8 ()) } # [inline] fn forward_checked (start : AsciiChar , count : usize) -> Option < AsciiChar > { let end = Step :: forward_checked (start . to_u8 () , count) ? ; AsciiChar :: from_u8 (end) } # [inline] fn backward_checked (start : AsciiChar , count : usize) -> Option < AsciiChar > { let end = Step :: backward_checked (start . to_u8 () , count) ? ; Some (unsafe { AsciiChar :: from_u8_unchecked (end) }) } # [inline] unsafe fn forward_unchecked (start : AsciiChar , count : usize) -> AsciiChar { let end = unsafe { Step :: forward_unchecked (start . to_u8 () , count) } ; unsafe { AsciiChar :: from_u8_unchecked (end) } } # [inline] unsafe fn backward_unchecked (start : AsciiChar , count : usize) -> AsciiChar { let end = unsafe { Step :: backward_unchecked (start . to_u8 () , count) } ; unsafe { AsciiChar :: from_u8_unchecked (end) } } }}}
mkitem!{mkimpl!{# [unstable (feature = "step_trait" , reason = "recently redesigned" , issue = "42168")] impl Step for Ipv4Addr { # [inline] fn steps_between (& start : & Ipv4Addr , & end : & Ipv4Addr) -> (usize , Option < usize >) { u32 :: steps_between (& start . to_bits () , & end . to_bits ()) } # [inline] fn forward_checked (start : Ipv4Addr , count : usize) -> Option < Ipv4Addr > { u32 :: forward_checked (start . to_bits () , count) . map (Ipv4Addr :: from_bits) } # [inline] fn backward_checked (start : Ipv4Addr , count : usize) -> Option < Ipv4Addr > { u32 :: backward_checked (start . to_bits () , count) . map (Ipv4Addr :: from_bits) } # [inline] unsafe fn forward_unchecked (start : Ipv4Addr , count : usize) -> Ipv4Addr { Ipv4Addr :: from_bits (unsafe { u32 :: forward_unchecked (start . to_bits () , count) }) } # [inline] unsafe fn backward_unchecked (start : Ipv4Addr , count : usize) -> Ipv4Addr { Ipv4Addr :: from_bits (unsafe { u32 :: backward_unchecked (start . to_bits () , count) }) } }}}
mkitem!{mkimpl!{# [unstable (feature = "step_trait" , reason = "recently redesigned" , issue = "42168")] impl Step for Ipv6Addr { # [inline] fn steps_between (& start : & Ipv6Addr , & end : & Ipv6Addr) -> (usize , Option < usize >) { u128 :: steps_between (& start . to_bits () , & end . to_bits ()) } # [inline] fn forward_checked (start : Ipv6Addr , count : usize) -> Option < Ipv6Addr > { u128 :: forward_checked (start . to_bits () , count) . map (Ipv6Addr :: from_bits) } # [inline] fn backward_checked (start : Ipv6Addr , count : usize) -> Option < Ipv6Addr > { u128 :: backward_checked (start . to_bits () , count) . map (Ipv6Addr :: from_bits) } # [inline] unsafe fn forward_unchecked (start : Ipv6Addr , count : usize) -> Ipv6Addr { Ipv6Addr :: from_bits (unsafe { u128 :: forward_unchecked (start . to_bits () , count) }) } # [inline] unsafe fn backward_unchecked (start : Ipv6Addr , count : usize) -> Ipv6Addr { Ipv6Addr :: from_bits (unsafe { u128 :: backward_unchecked (start . to_bits () , count) }) } }}}
mkitem!{macro_rules ! range_exact_iter_impl { ($ ($ t : ty) *) => ($ (# [stable (feature = "rust1" , since = "1.0.0")] impl ExactSizeIterator for ops :: Range <$ t > { }) *) }}
mkitem!{# [doc = " Safety: This macro must only be used on types that are `Copy` and result in ranges"] # [doc = " which have an exact `size_hint()` where the upper bound must not be `None`."] macro_rules ! unsafe_range_trusted_random_access_impl { ($ ($ t : ty) *) => ($ (# [doc (hidden)] # [unstable (feature = "trusted_random_access" , issue = "none")] unsafe impl TrustedRandomAccess for ops :: Range <$ t > { } # [doc (hidden)] # [unstable (feature = "trusted_random_access" , issue = "none")] unsafe impl TrustedRandomAccessNoCoerce for ops :: Range <$ t > { const MAY_HAVE_SIDE_EFFECT : bool = false ; }) *) }}
mkitem!{macro_rules ! range_incl_exact_iter_impl { ($ ($ t : ty) *) => ($ (# [stable (feature = "inclusive_range" , since = "1.26.0")] impl ExactSizeIterator for ops :: RangeInclusive <$ t > { }) *) }}
mkitem!{mktrait!{# [doc = " Specialization implementations for `Range`."] trait RangeIteratorImpl { type Item ; fn spec_next (& mut self) -> Option < Self :: Item > ; fn spec_nth (& mut self , n : usize) -> Option < Self :: Item > ; fn spec_advance_by (& mut self , n : usize) -> Result < () , NonZero < usize > > ; fn spec_next_back (& mut self) -> Option < Self :: Item > ; fn spec_nth_back (& mut self , n : usize) -> Option < Self :: Item > ; fn spec_advance_back_by (& mut self , n : usize) -> Result < () , NonZero < usize > > ; }}}
mkitem!{mkimpl!{impl < A : Step > RangeIteratorImpl for ops :: Range < A > { type Item = A ; # [inline] default fn spec_next (& mut self) -> Option < A > { if self . start < self . end { let n = Step :: forward_checked (self . start . clone () , 1) . expect ("`Step` invariants not upheld") ; Some (mem :: replace (& mut self . start , n)) } else { None } } # [inline] default fn spec_nth (& mut self , n : usize) -> Option < A > { if let Some (plus_n) = Step :: forward_checked (self . start . clone () , n) { if plus_n < self . end { self . start = Step :: forward_checked (plus_n . clone () , 1) . expect ("`Step` invariants not upheld") ; return Some (plus_n) ; } } self . start = self . end . clone () ; None } # [inline] default fn spec_advance_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { let steps = Step :: steps_between (& self . start , & self . end) ; let available = steps . 1 . unwrap_or (steps . 0) ; let taken = available . min (n) ; self . start = Step :: forward_checked (self . start . clone () , taken) . expect ("`Step` invariants not upheld") ; NonZero :: new (n - taken) . map_or (Ok (()) , Err) } # [inline] default fn spec_next_back (& mut self) -> Option < A > { if self . start < self . end { self . end = Step :: backward_checked (self . end . clone () , 1) . expect ("`Step` invariants not upheld") ; Some (self . end . clone ()) } else { None } } # [inline] default fn spec_nth_back (& mut self , n : usize) -> Option < A > { if let Some (minus_n) = Step :: backward_checked (self . end . clone () , n) { if minus_n > self . start { self . end = Step :: backward_checked (minus_n , 1) . expect ("`Step` invariants not upheld") ; return Some (self . end . clone ()) ; } } self . end = self . start . clone () ; None } # [inline] default fn spec_advance_back_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { let steps = Step :: steps_between (& self . start , & self . end) ; let available = steps . 1 . unwrap_or (steps . 0) ; let taken = available . min (n) ; self . end = Step :: backward_checked (self . end . clone () , taken) . expect ("`Step` invariants not upheld") ; NonZero :: new (n - taken) . map_or (Ok (()) , Err) } }}}
mkitem!{mkimpl!{impl < T : TrustedStep > RangeIteratorImpl for ops :: Range < T > { # [inline] fn spec_next (& mut self) -> Option < T > { if self . start < self . end { let old = self . start ; self . start = unsafe { Step :: forward_unchecked (old , 1) } ; Some (old) } else { None } } # [inline] fn spec_nth (& mut self , n : usize) -> Option < T > { if let Some (plus_n) = Step :: forward_checked (self . start , n) { if plus_n < self . end { self . start = unsafe { Step :: forward_unchecked (plus_n , 1) } ; return Some (plus_n) ; } } self . start = self . end ; None } # [inline] fn spec_advance_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { let steps = Step :: steps_between (& self . start , & self . end) ; let available = steps . 1 . unwrap_or (steps . 0) ; let taken = available . min (n) ; self . start = unsafe { Step :: forward_unchecked (self . start , taken) } ; NonZero :: new (n - taken) . map_or (Ok (()) , Err) } # [inline] fn spec_next_back (& mut self) -> Option < T > { if self . start < self . end { self . end = unsafe { Step :: backward_unchecked (self . end , 1) } ; Some (self . end) } else { None } } # [inline] fn spec_nth_back (& mut self , n : usize) -> Option < T > { if let Some (minus_n) = Step :: backward_checked (self . end , n) { if minus_n > self . start { self . end = unsafe { Step :: backward_unchecked (minus_n , 1) } ; return Some (self . end) ; } } self . end = self . start ; None } # [inline] fn spec_advance_back_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { let steps = Step :: steps_between (& self . start , & self . end) ; let available = steps . 1 . unwrap_or (steps . 0) ; let taken = available . min (n) ; self . end = unsafe { Step :: backward_unchecked (self . end , taken) } ; NonZero :: new (n - taken) . map_or (Ok (()) , Err) } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < A : Step > Iterator for ops :: Range < A > { type Item = A ; # [inline] fn next (& mut self) -> Option < A > { self . spec_next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { if self . start < self . end { Step :: steps_between (& self . start , & self . end) } else { (0 , Some (0)) } } # [inline] fn count (self) -> usize { if self . start < self . end { Step :: steps_between (& self . start , & self . end) . 1 . expect ("count overflowed usize") } else { 0 } } # [inline] fn nth (& mut self , n : usize) -> Option < A > { self . spec_nth (n) } # [inline] fn last (mut self) -> Option < A > { self . next_back () } # [inline] fn min (mut self) -> Option < A > where A : Ord , { self . next () } # [inline] fn max (mut self) -> Option < A > where A : Ord , { self . next_back () } # [inline] fn is_sorted (self) -> bool { true } # [inline] fn advance_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { self . spec_advance_by (n) } # [inline] unsafe fn __iterator_get_unchecked (& mut self , idx : usize) -> Self :: Item where Self : TrustedRandomAccessNoCoerce , { unsafe { Step :: forward_unchecked (self . start . clone () , idx) } } }}}
mkitem!{range_exact_iter_impl ! { usize u8 u16 isize i8 i16 u32 i32 }}
mkitem!{unsafe_range_trusted_random_access_impl ! { usize u8 u16 isize i8 i16 }}
mkitem!{# [cfg (target_pointer_width = "32")] unsafe_range_trusted_random_access_impl ! { u32 i32 }}
mkitem!{# [cfg (target_pointer_width = "64")] unsafe_range_trusted_random_access_impl ! { u32 i32 u64 i64 }}
mkitem!{range_incl_exact_iter_impl ! { u8 i8 u16 i16 }}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < A : Step > DoubleEndedIterator for ops :: Range < A > { # [inline] fn next_back (& mut self) -> Option < A > { self . spec_next_back () } # [inline] fn nth_back (& mut self , n : usize) -> Option < A > { self . spec_nth_back (n) } # [inline] fn advance_back_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { self . spec_advance_back_by (n) } }}}
mkitem!{mkimpl!{# [unstable (feature = "trusted_len" , issue = "37572")] unsafe impl < A : TrustedStep > TrustedLen for ops :: Range < A > { }}}
mkitem!{mkimpl!{# [stable (feature = "fused" , since = "1.26.0")] impl < A : Step > FusedIterator for ops :: Range < A > { }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < A : Step > Iterator for ops :: RangeFrom < A > { type Item = A ; # [inline] fn next (& mut self) -> Option < A > { let n = Step :: forward (self . start . clone () , 1) ; Some (mem :: replace (& mut self . start , n)) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (usize :: MAX , None) } # [inline] fn nth (& mut self , n : usize) -> Option < A > { let plus_n = Step :: forward (self . start . clone () , n) ; self . start = Step :: forward (plus_n . clone () , 1) ; Some (plus_n) } }}}
mkitem!{mkimpl!{# [unstable (feature = "trusted_len" , issue = "37572")] unsafe impl < A : TrustedStep > TrustedLen for ops :: RangeFrom < A > { }}}
mkitem!{mkimpl!{# [stable (feature = "fused" , since = "1.26.0")] impl < A : Step > FusedIterator for ops :: RangeFrom < A > { }}}
mkitem!{mktrait!{trait RangeInclusiveIteratorImpl { type Item ; fn spec_next (& mut self) -> Option < Self :: Item > ; fn spec_try_fold < B , F , R > (& mut self , init : B , f : F) -> R where Self : Sized , F : FnMut (B , Self :: Item) -> R , R : Try < Output = B > ; fn spec_next_back (& mut self) -> Option < Self :: Item > ; fn spec_try_rfold < B , F , R > (& mut self , init : B , f : F) -> R where Self : Sized , F : FnMut (B , Self :: Item) -> R , R : Try < Output = B > ; }}}
mkitem!{mkimpl!{impl < A : Step > RangeInclusiveIteratorImpl for ops :: RangeInclusive < A > { type Item = A ; # [inline] default fn spec_next (& mut self) -> Option < A > { if self . is_empty () { return None ; } let is_iterating = self . start < self . end ; Some (if is_iterating { let n = Step :: forward_checked (self . start . clone () , 1) . expect ("`Step` invariants not upheld") ; mem :: replace (& mut self . start , n) } else { self . exhausted = true ; self . start . clone () }) } # [inline] default fn spec_try_fold < B , F , R > (& mut self , init : B , mut f : F) -> R where Self : Sized , F : FnMut (B , A) -> R , R : Try < Output = B > , { if self . is_empty () { return try { init } ; } let mut accum = init ; while self . start < self . end { let n = Step :: forward_checked (self . start . clone () , 1) . expect ("`Step` invariants not upheld") ; let n = mem :: replace (& mut self . start , n) ; accum = f (accum , n) ? ; } self . exhausted = true ; if self . start == self . end { accum = f (accum , self . start . clone ()) ? ; } try { accum } } # [inline] default fn spec_next_back (& mut self) -> Option < A > { if self . is_empty () { return None ; } let is_iterating = self . start < self . end ; Some (if is_iterating { let n = Step :: backward_checked (self . end . clone () , 1) . expect ("`Step` invariants not upheld") ; mem :: replace (& mut self . end , n) } else { self . exhausted = true ; self . end . clone () }) } # [inline] default fn spec_try_rfold < B , F , R > (& mut self , init : B , mut f : F) -> R where Self : Sized , F : FnMut (B , A) -> R , R : Try < Output = B > , { if self . is_empty () { return try { init } ; } let mut accum = init ; while self . start < self . end { let n = Step :: backward_checked (self . end . clone () , 1) . expect ("`Step` invariants not upheld") ; let n = mem :: replace (& mut self . end , n) ; accum = f (accum , n) ? ; } self . exhausted = true ; if self . start == self . end { accum = f (accum , self . start . clone ()) ? ; } try { accum } } }}}
mkitem!{mkimpl!{impl < T : TrustedStep > RangeInclusiveIteratorImpl for ops :: RangeInclusive < T > { # [inline] fn spec_next (& mut self) -> Option < T > { if self . is_empty () { return None ; } let is_iterating = self . start < self . end ; Some (if is_iterating { let n = unsafe { Step :: forward_unchecked (self . start , 1) } ; mem :: replace (& mut self . start , n) } else { self . exhausted = true ; self . start }) } # [inline] fn spec_try_fold < B , F , R > (& mut self , init : B , mut f : F) -> R where Self : Sized , F : FnMut (B , T) -> R , R : Try < Output = B > , { if self . is_empty () { return try { init } ; } let mut accum = init ; while self . start < self . end { let n = unsafe { Step :: forward_unchecked (self . start , 1) } ; let n = mem :: replace (& mut self . start , n) ; accum = f (accum , n) ? ; } self . exhausted = true ; if self . start == self . end { accum = f (accum , self . start) ? ; } try { accum } } # [inline] fn spec_next_back (& mut self) -> Option < T > { if self . is_empty () { return None ; } let is_iterating = self . start < self . end ; Some (if is_iterating { let n = unsafe { Step :: backward_unchecked (self . end , 1) } ; mem :: replace (& mut self . end , n) } else { self . exhausted = true ; self . end }) } # [inline] fn spec_try_rfold < B , F , R > (& mut self , init : B , mut f : F) -> R where Self : Sized , F : FnMut (B , T) -> R , R : Try < Output = B > , { if self . is_empty () { return try { init } ; } let mut accum = init ; while self . start < self . end { let n = unsafe { Step :: backward_unchecked (self . end , 1) } ; let n = mem :: replace (& mut self . end , n) ; accum = f (accum , n) ? ; } self . exhausted = true ; if self . start == self . end { accum = f (accum , self . start) ? ; } try { accum } } }}}
mkitem!{mkimpl!{# [stable (feature = "inclusive_range" , since = "1.26.0")] impl < A : Step > Iterator for ops :: RangeInclusive < A > { type Item = A ; # [inline] fn next (& mut self) -> Option < A > { self . spec_next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { if self . is_empty () { return (0 , Some (0)) ; } let hint = Step :: steps_between (& self . start , & self . end) ; (hint . 0 . saturating_add (1) , hint . 1 . and_then (| steps | steps . checked_add (1))) } # [inline] fn count (self) -> usize { if self . is_empty () { return 0 ; } Step :: steps_between (& self . start , & self . end) . 1 . and_then (| steps | steps . checked_add (1)) . expect ("count overflowed usize") } # [inline] fn nth (& mut self , n : usize) -> Option < A > { if self . is_empty () { return None ; } if let Some (plus_n) = Step :: forward_checked (self . start . clone () , n) { use crate :: cmp :: Ordering :: * ; match plus_n . partial_cmp (& self . end) { Some (Less) => { self . start = Step :: forward (plus_n . clone () , 1) ; return Some (plus_n) ; } Some (Equal) => { self . start = plus_n . clone () ; self . exhausted = true ; return Some (plus_n) ; } _ => { } } } self . start = self . end . clone () ; self . exhausted = true ; None } # [inline] fn try_fold < B , F , R > (& mut self , init : B , f : F) -> R where Self : Sized , F : FnMut (B , Self :: Item) -> R , R : Try < Output = B > , { self . spec_try_fold (init , f) } impl_fold_via_try_fold ! { fold -> try_fold } # [inline] fn last (mut self) -> Option < A > { self . next_back () } # [inline] fn min (mut self) -> Option < A > where A : Ord , { self . next () } # [inline] fn max (mut self) -> Option < A > where A : Ord , { self . next_back () } # [inline] fn is_sorted (self) -> bool { true } }}}
mkitem!{mkimpl!{# [stable (feature = "inclusive_range" , since = "1.26.0")] impl < A : Step > DoubleEndedIterator for ops :: RangeInclusive < A > { # [inline] fn next_back (& mut self) -> Option < A > { self . spec_next_back () } # [inline] fn nth_back (& mut self , n : usize) -> Option < A > { if self . is_empty () { return None ; } if let Some (minus_n) = Step :: backward_checked (self . end . clone () , n) { use crate :: cmp :: Ordering :: * ; match minus_n . partial_cmp (& self . start) { Some (Greater) => { self . end = Step :: backward (minus_n . clone () , 1) ; return Some (minus_n) ; } Some (Equal) => { self . end = minus_n . clone () ; self . exhausted = true ; return Some (minus_n) ; } _ => { } } } self . end = self . start . clone () ; self . exhausted = true ; None } # [inline] fn try_rfold < B , F , R > (& mut self , init : B , f : F) -> R where Self : Sized , F : FnMut (B , Self :: Item) -> R , R : Try < Output = B > , { self . spec_try_rfold (init , f) } impl_fold_via_try_fold ! { rfold -> try_rfold } }}}
mkitem!{mkimpl!{# [unstable (feature = "trusted_len" , issue = "37572")] unsafe impl < A : TrustedStep > TrustedLen for ops :: RangeInclusive < A > { }}}
mkitem!{mkimpl!{# [stable (feature = "fused" , since = "1.26.0")] impl < A : Step > FusedIterator for ops :: RangeInclusive < A > { }}}