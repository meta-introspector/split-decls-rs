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
mkuse!{use rustc_abi :: FieldIdx ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_hir :: lang_items :: LangItem ;}
mkuse!{use rustc_macros :: { HashStable , TyDecodable , TyEncodable , TypeFoldable , TypeVisitable } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use crate :: ty :: { Ty , TyCtxt } ;}
mkitem!{mkenum!{# [derive (Clone , Copy , Debug , PartialEq , Eq , TyEncodable , TyDecodable , Hash , HashStable)] pub enum PointerCoercion { # [doc = " Go from a fn-item type to a fn-pointer type."] ReifyFnPointer , # [doc = " Go from a safe fn pointer to an unsafe fn pointer."] UnsafeFnPointer , # [doc = " Go from a non-capturing closure to an fn pointer or an unsafe fn pointer."] # [doc = " It cannot convert a closure that requires unsafe."] ClosureFnPointer (hir :: Safety) , # [doc = " Go from a mut raw pointer to a const raw pointer."] MutToConstPointer , # [doc = " Go from `*const [T; N]` to `*const T`"] ArrayToPointer , # [doc = " Unsize a pointer/reference value, e.g., `&[T; n]` to"] # [doc = " `&[T]`. Note that the source could be a thin or wide pointer."] # [doc = " This will do things like convert thin pointers to wide"] # [doc = " pointers, or convert structs containing thin pointers to"] # [doc = " structs containing wide pointers, or convert between wide"] # [doc = " pointers. We don't store the details of how the transform is"] # [doc = " done (in fact, we don't know that, because it might depend on"] # [doc = " the precise type parameters). We just store the target"] # [doc = " type. Codegen backends and miri figure out what has to be done"] # [doc = " based on the precise source/target type at hand."] Unsize , }}}
mkitem!{mkstruct!{# [doc = " Represents coercing a value to a different type of value."] # [doc = ""] # [doc = " We transform values by following a number of `Adjust` steps in order."] # [doc = " See the documentation on variants of `Adjust` for more details."] # [doc = ""] # [doc = " Here are some common scenarios:"] # [doc = ""] # [doc = " 1. The simplest cases are where a pointer is not adjusted fat vs thin."] # [doc = "    Here the pointer will be dereferenced N times (where a dereference can"] # [doc = "    happen to raw or borrowed pointers or any smart pointer which implements"] # [doc = "    `Deref`, including `Box<_>`). The types of dereferences is given by"] # [doc = "    `autoderefs`. It can then be auto-referenced zero or one times, indicated"] # [doc = "    by `autoref`, to either a raw or borrowed pointer. In these cases unsize is"] # [doc = "    `false`."] # [doc = ""] # [doc = " 2. A thin-to-fat coercion involves unsizing the underlying data. We start"] # [doc = "    with a thin pointer, deref a number of times, unsize the underlying data,"] # [doc = "    then autoref. The 'unsize' phase may change a fixed length array to a"] # [doc = "    dynamically sized one, a concrete object to a trait object, or statically"] # [doc = "    sized struct to a dynamically sized one. E.g., `&[i32; 4]` -> `&[i32]` is"] # [doc = "    represented by:"] # [doc = ""] # [doc = "    ```ignore (illustrative)"] # [doc = "    Deref(None) -> [i32; 4],"] # [doc = "    Borrow(AutoBorrow::Ref) -> &[i32; 4],"] # [doc = "    Unsize -> &[i32],"] # [doc = "    ```"] # [doc = ""] # [doc = "    Note that for a struct, the 'deep' unsizing of the struct is not recorded."] # [doc = "    E.g., `struct Foo<T> { x: T }` we can coerce `&Foo<[i32; 4]>` to `&Foo<[i32]>`"] # [doc = "    The autoderef and -ref are the same as in the above example, but the type"] # [doc = "    stored in `unsize` is `Foo<[i32]>`, we don't store any further detail about"] # [doc = "    the underlying conversions from `[i32; 4]` to `[i32]`."] # [doc = ""] # [doc = " 3. Coercing a `Box<T>` to `Box<dyn Trait>` is an interesting special case. In"] # [doc = "    that case, we have the pointer we need coming in, so there are no"] # [doc = "    autoderefs, and no autoref. Instead we just do the `Unsize` transformation."] # [doc = "    At some point, of course, `Box` should move out of the compiler, in which"] # [doc = "    case this is analogous to transforming a struct. E.g., `Box<[i32; 4]>` ->"] # [doc = "    `Box<[i32]>` is an `Adjust::Unsize` with the target `Box<[i32]>`."] # [derive (Clone , TyEncodable , TyDecodable , HashStable , TypeFoldable , TypeVisitable)] pub struct Adjustment < 'tcx > { pub kind : Adjust , pub target : Ty < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > Adjustment < 'tcx > { pub fn is_region_borrow (& self) -> bool { matches ! (self . kind , Adjust :: Borrow (AutoBorrow :: Ref (..))) } }}}
mkitem!{mkenum!{# [derive (Clone , Debug , TyEncodable , TyDecodable , HashStable , TypeFoldable , TypeVisitable)] pub enum Adjust { # [doc = " Go from ! to any type."] NeverToAny , # [doc = " Dereference once, producing a place."] Deref (Option < OverloadedDeref >) , # [doc = " Take the address and produce either a `&` or `*` pointer."] Borrow (AutoBorrow) , Pointer (PointerCoercion) , # [doc = " Take a pinned reference and reborrow as a `Pin<&mut T>` or `Pin<&T>`."] ReborrowPin (hir :: Mutability) , }}}
mkitem!{mkstruct!{# [doc = " An overloaded autoderef step, representing a `Deref(Mut)::deref(_mut)`"] # [doc = " call, with the signature `&'a T -> &'a U` or `&'a mut T -> &'a mut U`."] # [doc = " The target type is `U` in both cases, with the region and mutability"] # [doc = " being those shared by both the receiver and the returned reference."] # [derive (Copy , Clone , PartialEq , Debug , TyEncodable , TyDecodable , HashStable)] # [derive (TypeFoldable , TypeVisitable)] pub struct OverloadedDeref { pub mutbl : hir :: Mutability , # [doc = " The `Span` associated with the field access or method call"] # [doc = " that triggered this overloaded deref."] pub span : Span , }}}
mkitem!{mkimpl!{impl OverloadedDeref { # [doc = " Get the [`DefId`] of the method call for the given `Deref`/`DerefMut` trait"] # [doc = " for this overloaded deref's mutability."] pub fn method_call < 'tcx > (& self , tcx : TyCtxt < 'tcx >) -> DefId { let trait_def_id = match self . mutbl { hir :: Mutability :: Not => tcx . require_lang_item (LangItem :: Deref , self . span) , hir :: Mutability :: Mut => tcx . require_lang_item (LangItem :: DerefMut , self . span) , } ; tcx . associated_items (trait_def_id) . in_definition_order () . find (| item | item . is_fn ()) . unwrap () . def_id } }}}
mkitem!{mkenum!{# [doc = " At least for initial deployment, we want to limit two-phase borrows to"] # [doc = " only a few specific cases. Right now, those are mostly \"things that desugar\""] # [doc = " into method calls:"] # [doc = " - using `x.some_method()` syntax, where some_method takes `&mut self`,"] # [doc = " - using `Foo::some_method(&mut x, ...)` syntax,"] # [doc = " - binary assignment operators (`+=`, `-=`, `*=`, etc.)."] # [doc = " Anything else should be rejected until generalized two-phase borrow support"] # [doc = " is implemented. Right now, dataflow can't handle the general case where there"] # [doc = " is more than one use of a mutable borrow, and we don't want to accept too much"] # [doc = " new code via two-phase borrows, so we try to limit where we create two-phase"] # [doc = " capable mutable borrows."] # [doc = " See #49434 for tracking."] # [derive (Copy , Clone , PartialEq , Debug , TyEncodable , TyDecodable , HashStable)] pub enum AllowTwoPhase { Yes , No , }}}
mkitem!{mkenum!{# [derive (Copy , Clone , PartialEq , Debug , TyEncodable , TyDecodable , HashStable)] pub enum AutoBorrowMutability { Mut { allow_two_phase_borrow : AllowTwoPhase } , Not , }}}
mkitem!{mkimpl!{impl AutoBorrowMutability { # [doc = " Creates an `AutoBorrowMutability` from a mutability and allowance of two phase borrows."] # [doc = ""] # [doc = " Note that when `mutbl.is_not()`, `allow_two_phase_borrow` is ignored"] pub fn new (mutbl : hir :: Mutability , allow_two_phase_borrow : AllowTwoPhase) -> Self { match mutbl { hir :: Mutability :: Not => Self :: Not , hir :: Mutability :: Mut => Self :: Mut { allow_two_phase_borrow } , } } }}}
mkitem!{mkimpl!{impl From < AutoBorrowMutability > for hir :: Mutability { fn from (m : AutoBorrowMutability) -> Self { match m { AutoBorrowMutability :: Mut { .. } => hir :: Mutability :: Mut , AutoBorrowMutability :: Not => hir :: Mutability :: Not , } } }}}
mkitem!{mkenum!{# [derive (Copy , Clone , PartialEq , Debug , TyEncodable , TyDecodable , HashStable)] # [derive (TypeFoldable , TypeVisitable)] pub enum AutoBorrow { # [doc = " Converts from T to &T."] Ref (AutoBorrowMutability) , # [doc = " Converts from T to *T."] RawPtr (hir :: Mutability) , }}}
mkitem!{mkstruct!{# [doc = " Information for `CoerceUnsized` impls, storing information we"] # [doc = " have computed about the coercion."] # [doc = ""] # [doc = " This struct can be obtained via the `coerce_impl_info` query."] # [doc = " Demanding this struct also has the side-effect of reporting errors"] # [doc = " for inappropriate impls."] # [derive (Clone , Copy , TyEncodable , TyDecodable , Debug , HashStable)] pub struct CoerceUnsizedInfo { # [doc = " If this is a \"custom coerce\" impl, then what kind of custom"] # [doc = " coercion is it? This applies to impls of `CoerceUnsized` for"] # [doc = " structs, primarily, where we store a bit of info about which"] # [doc = " fields need to be coerced."] pub custom_kind : Option < CustomCoerceUnsized > , }}}
mkitem!{mkenum!{# [derive (Clone , Copy , TyEncodable , TyDecodable , Debug , HashStable)] pub enum CustomCoerceUnsized { # [doc = " Records the index of the field being coerced."] Struct (FieldIdx) , }}}
mkitem!{mkstruct!{# [doc = " Represents an implicit coercion applied to the scrutinee of a match before testing a pattern"] # [doc = " against it. Currently, this is used only for implicit dereferences."] # [derive (Clone , Copy , TyEncodable , TyDecodable , HashStable , TypeFoldable , TypeVisitable)] pub struct PatAdjustment < 'tcx > { pub kind : PatAdjust , # [doc = " The type of the scrutinee before the adjustment is applied, or the \"adjusted type\" of the"] # [doc = " pattern."] pub source : Ty < 'tcx > , }}}
mkitem!{mkenum!{# [doc = " Represents implicit coercions of patterns' types, rather than values' types."] # [derive (Clone , Copy , PartialEq , Debug , TyEncodable , TyDecodable , HashStable)] # [derive (TypeFoldable , TypeVisitable)] pub enum PatAdjust { # [doc = " An implicit dereference before matching, such as when matching the pattern `0` against a"] # [doc = " scrutinee of type `&u8` or `&mut u8`."] BuiltinDeref , # [doc = " An implicit call to `Deref(Mut)::deref(_mut)` before matching, such as when matching the"] # [doc = " pattern `[..]` against a scrutinee of type `Vec<T>`."] OverloadedDeref , }}}