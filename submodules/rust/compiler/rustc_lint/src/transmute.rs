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
mkuse!{use rustc_ast :: LitKind ;}
mkuse!{use rustc_errors :: Applicability ;}
mkuse!{use rustc_hir :: def :: { DefKind , Res } ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_hir :: { self as hir } ;}
mkuse!{use rustc_macros :: LintDiagnostic ;}
mkuse!{use rustc_middle :: ty :: { self , Ty } ;}
mkuse!{use rustc_session :: { declare_lint , impl_lint_pass } ;}
mkuse!{use rustc_span :: sym ;}
mkuse!{use crate :: lints :: { IntegerToPtrTransmutes , IntegerToPtrTransmutesSuggestion } ;}
mkuse!{use crate :: { LateContext , LateLintPass } ;}
mkitem!{declare_lint ! { # [doc = " The `ptr_to_integer_transmute_in_consts` lint detects pointer to integer"] # [doc = " transmute in const functions and associated constants."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " const fn foo(ptr: *const u8) -> usize {"] # [doc = "    unsafe {"] # [doc = "        std::mem::transmute::<*const u8, usize>(ptr)"] # [doc = "    }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Transmuting pointers to integers in a `const` context is undefined behavior."] # [doc = " Any attempt to use the resulting integer will abort const-evaluation."] # [doc = ""] # [doc = " But sometimes the compiler might not emit an error for pointer to integer transmutes"] # [doc = " inside const functions and associated consts because they are evaluated only when referenced."] # [doc = " Therefore, this lint serves as an extra layer of defense to prevent any undefined behavior"] # [doc = " from compiling without any warnings or errors."] # [doc = ""] # [doc = " See [std::mem::transmute] in the reference for more details."] # [doc = ""] # [doc = " [std::mem::transmute]: https://doc.rust-lang.org/std/mem/fn.transmute.html"] pub PTR_TO_INTEGER_TRANSMUTE_IN_CONSTS , Warn , "detects pointer to integer transmutes in const functions and associated constants" , }}
mkitem!{declare_lint ! { # [doc = " The `unnecessary_transmutes` lint detects transmutations that have safer alternatives."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " fn bytes_at_home(x: [u8; 4]) -> u32 {"] # [doc = "   unsafe { std::mem::transmute(x) }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Using an explicit method is preferable over calls to"] # [doc = " [`transmute`](https://doc.rust-lang.org/std/mem/fn.transmute.html) as"] # [doc = " they more clearly communicate the intent, are easier to review, and"] # [doc = " are less likely to accidentally result in unsoundness."] pub UNNECESSARY_TRANSMUTES , Warn , "detects transmutes that can also be achieved by other operations" }}
mkitem!{declare_lint ! { # [doc = " The `integer_to_ptr_transmutes` lint detects integer to pointer"] # [doc = " transmutes where the resulting pointers are undefined behavior to dereference."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " fn foo(a: usize) -> *const u8 {"] # [doc = "    unsafe {"] # [doc = "        std::mem::transmute::<usize, *const u8>(a)"] # [doc = "    }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Any attempt to use the resulting pointers are undefined behavior as the resulting"] # [doc = " pointers won't have any provenance."] # [doc = ""] # [doc = " Alternatively, [`std::ptr::with_exposed_provenance`] should be used, as they do not"] # [doc = " carry the provenance requirement. If wanting to create pointers without provenance"] # [doc = " [`std::ptr::without_provenance`] should be used instead."] # [doc = ""] # [doc = " See [`std::mem::transmute`] in the reference for more details."] # [doc = ""] # [doc = " [`std::mem::transmute`]: https://doc.rust-lang.org/std/mem/fn.transmute.html"] # [doc = " [`std::ptr::with_exposed_provenance`]: https://doc.rust-lang.org/std/ptr/fn.with_exposed_provenance.html"] # [doc = " [`std::ptr::without_provenance`]: https://doc.rust-lang.org/std/ptr/fn.without_provenance.html"] pub INTEGER_TO_PTR_TRANSMUTES , Warn , "detects integer to pointer transmutes" , }}
mkitem!{mkstruct!{pub (crate) struct CheckTransmutes ;}}
mkitem!{impl_lint_pass ! (CheckTransmutes => [PTR_TO_INTEGER_TRANSMUTE_IN_CONSTS , UNNECESSARY_TRANSMUTES , INTEGER_TO_PTR_TRANSMUTES]) ;}
mkitem!{mkimpl!{impl < 'tcx > LateLintPass < 'tcx > for CheckTransmutes { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx >) { let hir :: ExprKind :: Call (callee , [arg]) = expr . kind else { return ; } ; let hir :: ExprKind :: Path (qpath) = callee . kind else { return ; } ; let Res :: Def (DefKind :: Fn , def_id) = cx . qpath_res (& qpath , callee . hir_id) else { return ; } ; if ! cx . tcx . is_intrinsic (def_id , sym :: transmute) { return ; } ; let body_owner_def_id = cx . tcx . hir_enclosing_body_owner (expr . hir_id) ; let const_context = cx . tcx . hir_body_const_context (body_owner_def_id) ; let args = cx . typeck_results () . node_args (callee . hir_id) ; let src = args . type_at (0) ; let dst = args . type_at (1) ; check_ptr_transmute_in_const (cx , expr , body_owner_def_id , const_context , src , dst) ; check_unnecessary_transmute (cx , expr , callee , arg , const_context , src , dst) ; check_int_to_ptr_transmute (cx , expr , arg , src , dst) ; } }}}

macro_rules! check_int_to_ptr_transmute_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_int_to_ptr_transmute in module {}", module_path!());
    };
}

mkfn!{
    check_int_to_ptr_transmute_introspect!();
    # [doc = " Check for transmutes from integer to pointers (*const/*mut and &/&mut)."] # [doc = ""] # [doc = " Using the resulting pointers would be undefined behavior."] fn check_int_to_ptr_transmute < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx > , arg : & 'tcx hir :: Expr < 'tcx > , src : Ty < 'tcx > , dst : Ty < 'tcx > ,) { if ! matches ! (src . kind () , ty :: Uint (_) | ty :: Int (_)) { return ; } let (ty :: Ref (_ , inner_ty , mutbl) | ty :: RawPtr (inner_ty , mutbl)) = dst . kind () else { return ; } ; if matches ! (arg . kind , hir :: ExprKind :: Lit (hir :: Lit { node : LitKind :: Int (v , _) , .. }) if v == 0) { return ; } let Ok (layout_inner_ty) = cx . tcx . layout_of (cx . typing_env () . as_query_input (* inner_ty)) else { return ; } ; if layout_inner_ty . is_1zst () { return ; } let suffix = if mutbl . is_mut () { "_mut" } else { "" } ; cx . tcx . emit_node_span_lint (INTEGER_TO_PTR_TRANSMUTES , expr . hir_id , expr . span , IntegerToPtrTransmutes { suggestion : if layout_inner_ty . is_sized () { Some (if dst . is_ref () { IntegerToPtrTransmutesSuggestion :: ToRef { dst : * inner_ty , suffix , ref_mutbl : mutbl . prefix_str () , start_call : expr . span . shrink_to_lo () . until (arg . span) , } } else { IntegerToPtrTransmutesSuggestion :: ToPtr { dst : * inner_ty , suffix , start_call : expr . span . shrink_to_lo () . until (arg . span) , } }) } else { None } , } ,) ; }
}

macro_rules! check_ptr_transmute_in_const_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_ptr_transmute_in_const in module {}", module_path!());
    };
}

mkfn!{
    check_ptr_transmute_in_const_introspect!();
    # [doc = " Check for transmutes that exhibit undefined behavior."] # [doc = " For example, transmuting pointers to integers in a const context."] # [doc = ""] # [doc = " Why do we consider const functions and associated constants only?"] # [doc = ""] # [doc = " Generally, undefined behavior in const items are handled by the evaluator."] # [doc = " But, const functions and associated constants are evaluated only when referenced."] # [doc = " This can result in undefined behavior in a library going unnoticed until"] # [doc = " the function or constant is actually used."] # [doc = ""] # [doc = " Therefore, we only consider const functions and associated constants here and leave"] # [doc = " other const items to be handled by the evaluator."] fn check_ptr_transmute_in_const < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx > , body_owner_def_id : LocalDefId , const_context : Option < hir :: ConstContext > , src : Ty < 'tcx > , dst : Ty < 'tcx > ,) { if matches ! (const_context , Some (hir :: ConstContext :: ConstFn)) || matches ! (cx . tcx . def_kind (body_owner_def_id) , DefKind :: AssocConst) { if src . is_raw_ptr () && dst . is_integral () { cx . tcx . emit_node_span_lint (PTR_TO_INTEGER_TRANSMUTE_IN_CONSTS , expr . hir_id , expr . span , UndefinedTransmuteLint ,) ; } } }
}

macro_rules! check_unnecessary_transmute_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_unnecessary_transmute in module {}", module_path!());
    };
}

mkfn!{
    check_unnecessary_transmute_introspect!();
    # [doc = " Check for transmutes that overlap with stdlib methods."] # [doc = " For example, transmuting `[u8; 4]` to `u32`."] # [doc = ""] # [doc = " We chose not to lint u8 -> bool transmutes, see #140431."] fn check_unnecessary_transmute < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx > , callee : & 'tcx hir :: Expr < 'tcx > , arg : & 'tcx hir :: Expr < 'tcx > , const_context : Option < hir :: ConstContext > , src : Ty < 'tcx > , dst : Ty < 'tcx > ,) { let callee_span = callee . span . find_ancestor_inside (expr . span) . unwrap_or (callee . span) ; let (sugg , help) = match (src . kind () , dst . kind ()) { (ty :: Array (t , _) , ty :: Uint (_) | ty :: Float (_) | ty :: Int (_)) if * t . kind () == ty :: Uint (ty :: UintTy :: U8) => { (Some (vec ! [(callee_span , format ! ("{dst}::from_ne_bytes"))]) , Some ("there's also `from_le_bytes` and `from_be_bytes` if you expect a particular byte order" ,) ,) } (ty :: Uint (_) | ty :: Float (_) | ty :: Int (_) , ty :: Array (t , _)) if * t . kind () == ty :: Uint (ty :: UintTy :: U8) => { (Some (vec ! [(callee_span , format ! ("{src}::to_ne_bytes"))]) , Some ("there's also `to_le_bytes` and `to_be_bytes` if you expect a particular byte order" ,) ,) } (ty :: Char , ty :: Uint (ty :: UintTy :: U32)) => { (Some (vec ! [(callee_span , "u32::from" . to_string ())]) , None) } (ty :: Char , ty :: Int (ty :: IntTy :: I32)) => (Some (vec ! [(callee_span , "u32::from" . to_string ()) , (expr . span . shrink_to_hi () , ".cast_signed()" . to_string ()) ,]) , None ,) , (ty :: Uint (ty :: UintTy :: U32) , ty :: Char) => (Some (vec ! [(callee_span , "char::from_u32_unchecked" . to_string ())]) , Some ("consider using `char::from_u32(…).unwrap()`") ,) , (ty :: Int (ty :: IntTy :: I32) , ty :: Char) => (Some (vec ! [(callee_span , "char::from_u32_unchecked(i32::cast_unsigned" . to_string ()) , (expr . span . shrink_to_hi () , ")" . to_string ()) ,]) , Some ("consider using `char::from_u32(i32::cast_unsigned(…)).unwrap()`") ,) , (ty :: Uint (_) , ty :: Int (_)) => { (Some (vec ! [(callee_span , format ! ("{src}::cast_signed"))]) , None) } (ty :: Int (_) , ty :: Uint (_)) => { (Some (vec ! [(callee_span , format ! ("{src}::cast_unsigned"))]) , None) } (ty :: Float (_) , ty :: Uint (ty :: UintTy :: Usize) | ty :: Int (ty :: IntTy :: Isize)) => (Some (vec ! [(callee_span , format ! ("{src}::to_bits")) , (expr . span . shrink_to_hi () , format ! (" as {dst}")) ,]) , None ,) , (ty :: Float (_) , ty :: Int (..)) => (Some (vec ! [(callee_span , format ! ("{src}::to_bits")) , (expr . span . shrink_to_hi () , ".cast_signed()" . to_string ()) ,]) , None ,) , (ty :: Float (_) , ty :: Uint (..)) => { (Some (vec ! [(callee_span , format ! ("{src}::to_bits"))]) , None) } (ty :: Uint (ty :: UintTy :: Usize) | ty :: Int (ty :: IntTy :: Isize) , ty :: Float (_)) => (Some (vec ! [(callee_span , format ! ("{dst}::from_bits")) , (arg . span . shrink_to_hi () , " as _" . to_string ()) ,]) , None ,) , (ty :: Int (_) , ty :: Float (_)) => (Some (vec ! [(callee_span , format ! ("{dst}::from_bits({src}::cast_unsigned")) , (expr . span . shrink_to_hi () , ")" . to_string ()) ,]) , None ,) , (ty :: Uint (_) , ty :: Float (_)) => { (Some (vec ! [(callee_span , format ! ("{dst}::from_bits"))]) , None) } (ty :: Bool , ty :: Int (..) | ty :: Uint (..)) if const_context . is_some () => (Some (vec ! [(callee_span , "" . to_string ()) , (expr . span . shrink_to_hi () , format ! (" as {dst}")) ,]) , None ,) , (ty :: Bool , ty :: Int (..) | ty :: Uint (..)) => { (Some (vec ! [(callee_span , format ! ("{dst}::from"))]) , None) } _ => return , } ; cx . tcx . node_span_lint (UNNECESSARY_TRANSMUTES , expr . hir_id , expr . span , | diag | { diag . primary_message ("unnecessary transmute") ; if let Some (sugg) = sugg { diag . multipart_suggestion ("replace this with" , sugg , Applicability :: MachineApplicable) ; } if let Some (help) = help { diag . help (help) ; } }) ; }
}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (lint_undefined_transmute)] # [note] # [note (lint_note2)] # [help] pub (crate) struct UndefinedTransmuteLint ;}}