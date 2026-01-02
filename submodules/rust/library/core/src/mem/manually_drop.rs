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
mkuse!{use crate :: ops :: { Deref , DerefMut , DerefPure } ;}
mkuse!{use crate :: ptr ;}
mkitem!{mkstruct!{#[doc = " A wrapper to inhibit the compiler from automatically calling `T`’s"] #[doc = " destructor. This wrapper is 0-cost."] #[doc = ""] #[doc = " `ManuallyDrop<T>` is guaranteed to have the same layout and bit validity as"] #[doc = " `T`, and is subject to the same layout optimizations as `T`. As a"] #[doc = " consequence, it has *no effect* on the assumptions that the compiler makes"] #[doc = " about its contents. For example, initializing a `ManuallyDrop<&mut T>` with"] #[doc = " [`mem::zeroed`] is undefined behavior. If you need to handle uninitialized"] #[doc = " data, use [`MaybeUninit<T>`] instead."] #[doc = ""] #[doc = " Note that accessing the value inside a `ManuallyDrop<T>` is safe. This means"] #[doc = " that a `ManuallyDrop<T>` whose content has been dropped must not be exposed"] #[doc = " through a public safe API. Correspondingly, `ManuallyDrop::drop` is unsafe."] #[doc = ""] #[doc = " # `ManuallyDrop` and drop order"] #[doc = ""] #[doc = " Rust has a well-defined [drop order] of values. To make sure that fields or"] #[doc = " locals are dropped in a specific order, reorder the declarations such that"] #[doc = " the implicit drop order is the correct one."] #[doc = ""] #[doc = " It is possible to use `ManuallyDrop` to control the drop order, but this"] #[doc = " requires unsafe code and is hard to do correctly in the presence of"] #[doc = " unwinding."] #[doc = ""] #[doc = " For example, if you want to make sure that a specific field is dropped after"] #[doc = " the others, make it the last field of a struct:"] #[doc = ""] #[doc = " ```"] #[doc = " struct Context;"] #[doc = ""] #[doc = " struct Widget {"] #[doc = "     children: Vec<Widget>,"] #[doc = "     // `context` will be dropped after `children`."] #[doc = "     // Rust guarantees that fields are dropped in the order of declaration."] #[doc = "     context: Context,"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " # Interaction with `Box`"] #[doc = ""] #[doc = " Currently, if you have a `ManuallyDrop<T>`, where the type `T` is a `Box` or"] #[doc = " contains a `Box` inside, then dropping the `T` followed by moving the"] #[doc = " `ManuallyDrop<T>` is [considered to be undefined"] #[doc = " behavior](https://github.com/rust-lang/unsafe-code-guidelines/issues/245)."] #[doc = " That is, the following code causes undefined behavior:"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::mem::ManuallyDrop;"] #[doc = ""] #[doc = " let mut x = ManuallyDrop::new(Box::new(42));"] #[doc = " unsafe {"] #[doc = "     ManuallyDrop::drop(&mut x);"] #[doc = " }"] #[doc = " let y = x; // Undefined behavior!"] #[doc = " ```"] #[doc = ""] #[doc = " This is [likely to change in the"] #[doc = " future](https://rust-lang.github.io/rfcs/3336-maybe-dangling.html). In the"] #[doc = " meantime, consider using [`MaybeUninit`] instead."] #[doc = ""] #[doc = " # Safety hazards when storing `ManuallyDrop` in a struct or an enum."] #[doc = ""] #[doc = " Special care is needed when all of the conditions below are met:"] #[doc = " * A struct or enum contains a `ManuallyDrop`."] #[doc = " * The `ManuallyDrop` is not inside a `union`."] #[doc = " * The struct or enum is part of public API, or is stored in a struct or an"] #[doc = "   enum that is part of public API."] #[doc = " * There is code that drops the contents of the `ManuallyDrop` field, and"] #[doc = "   this code is outside the struct or enum's `Drop` implementation."] #[doc = ""] #[doc = " In particular, the following hazards may occur:"] #[doc = ""] #[doc = " #### Storing generic types"] #[doc = ""] #[doc = " If the `ManuallyDrop` contains a client-supplied generic type, the client"] #[doc = " might provide a `Box` as that type. This would cause undefined behavior when"] #[doc = " the struct or enum is later moved, as mentioned in the previous section. For"] #[doc = " example, the following code causes undefined behavior:"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::mem::ManuallyDrop;"] #[doc = ""] #[doc = " pub struct BadOption<T> {"] #[doc = "     // Invariant: Has been dropped if `is_some` is false."] #[doc = "     value: ManuallyDrop<T>,"] #[doc = "     is_some: bool,"] #[doc = " }"] #[doc = " impl<T> BadOption<T> {"] #[doc = "     pub fn new(value: T) -> Self {"] #[doc = "         Self { value: ManuallyDrop::new(value), is_some: true }"] #[doc = "     }"] #[doc = "     pub fn change_to_none(&mut self) {"] #[doc = "         if self.is_some {"] #[doc = "             self.is_some = false;"] #[doc = "             unsafe {"] #[doc = "                 // SAFETY: `value` hasn't been dropped yet, as per the invariant"] #[doc = "                 // (This is actually unsound!)"] #[doc = "                 ManuallyDrop::drop(&mut self.value);"] #[doc = "             }"] #[doc = "         }"] #[doc = "     }"] #[doc = " }"] #[doc = ""] #[doc = " // In another crate:"] #[doc = ""] #[doc = " let mut option = BadOption::new(Box::new(42));"] #[doc = " option.change_to_none();"] #[doc = " let option2 = option; // Undefined behavior!"] #[doc = " ```"] #[doc = ""] #[doc = " #### Deriving traits"] #[doc = ""] #[doc = " Deriving `Debug`, `Clone`, `PartialEq`, `PartialOrd`, `Ord`, or `Hash` on"] #[doc = " the struct or enum could be unsound, since the derived implementations of"] #[doc = " these traits would access the `ManuallyDrop` field. For example, the"] #[doc = " following code causes undefined behavior:"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::mem::ManuallyDrop;"] #[doc = ""] #[doc = " // This derive is unsound in combination with the `ManuallyDrop::drop` call."] #[doc = " #[derive(Debug)]"] #[doc = " pub struct Foo {"] #[doc = "     value: ManuallyDrop<String>,"] #[doc = " }"] #[doc = " impl Foo {"] #[doc = "     pub fn new() -> Self {"] #[doc = "         let mut temp = Self {"] #[doc = "             value: ManuallyDrop::new(String::from(\"Unsafe rust is hard.\"))"] #[doc = "         };"] #[doc = "         unsafe {"] #[doc = "             // SAFETY: `value` hasn't been dropped yet."] #[doc = "             ManuallyDrop::drop(&mut temp.value);"] #[doc = "         }"] #[doc = "         temp"] #[doc = "     }"] #[doc = " }"] #[doc = ""] #[doc = " // In another crate:"] #[doc = ""] #[doc = " let foo = Foo::new();"] #[doc = " println!(\"{:?}\", foo); // Undefined behavior!"] #[doc = " ```"] #[doc = ""] #[doc = " [drop order]: https://doc.rust-lang.org/reference/destructors.html"] #[doc = " [`mem::zeroed`]: crate::mem::zeroed"] #[doc = " [`MaybeUninit<T>`]: crate::mem::MaybeUninit"] #[doc = " [`MaybeUninit`]: crate::mem::MaybeUninit"] #[stable (feature = "manually_drop" , since = "1.20.0")] #[lang = "manually_drop"] #[derive (Copy , Clone , Debug , Default , PartialEq , Eq , PartialOrd , Ord , Hash)] #[repr (transparent)] #[rustc_pub_transparent] pub struct ManuallyDrop < T : ? Sized > { value : T , }}}
mkitem!{mkimpl!{impl < T > ManuallyDrop < T > { #[doc = " Wrap a value to be manually dropped."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```rust"] #[doc = " use std::mem::ManuallyDrop;"] #[doc = " let mut x = ManuallyDrop::new(String::from(\"Hello World!\"));"] #[doc = " x.truncate(5); // You can still safely operate on the value"] #[doc = " assert_eq!(*x, \"Hello\");"] #[doc = " // But `Drop` will not be run here"] #[doc = " # // FIXME(https://github.com/rust-lang/miri/issues/3670):"] #[doc = " # // use -Zmiri-disable-leak-check instead of unleaking in tests meant to leak."] #[doc = " # let _ = ManuallyDrop::into_inner(x);"] #[doc = " ```"] #[must_use = "if you don't need the wrapper, you can use `mem::forget` instead"] #[stable (feature = "manually_drop" , since = "1.20.0")] #[rustc_const_stable (feature = "const_manually_drop" , since = "1.32.0")] #[inline (always)] pub const fn new (value : T) -> ManuallyDrop < T > { ManuallyDrop { value } } #[doc = " Extracts the value from the `ManuallyDrop` container."] #[doc = ""] #[doc = " This allows the value to be dropped again."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```rust"] #[doc = " use std::mem::ManuallyDrop;"] #[doc = " let x = ManuallyDrop::new(Box::new(()));"] #[doc = " let _: Box<()> = ManuallyDrop::into_inner(x); // This drops the `Box`."] #[doc = " ```"] #[stable (feature = "manually_drop" , since = "1.20.0")] #[rustc_const_stable (feature = "const_manually_drop" , since = "1.32.0")] #[inline (always)] pub const fn into_inner (slot : ManuallyDrop < T >) -> T { slot . value } #[doc = " Takes the value from the `ManuallyDrop<T>` container out."] #[doc = ""] #[doc = " This method is primarily intended for moving out values in drop."] #[doc = " Instead of using [`ManuallyDrop::drop`] to manually drop the value,"] #[doc = " you can use this method to take the value and use it however desired."] #[doc = ""] #[doc = " Whenever possible, it is preferable to use [`into_inner`][`ManuallyDrop::into_inner`]"] #[doc = " instead, which prevents duplicating the content of the `ManuallyDrop<T>`."] #[doc = ""] #[doc = " # Safety"] #[doc = ""] #[doc = " This function semantically moves out the contained value without preventing further usage,"] #[doc = " leaving the state of this container unchanged."] #[doc = " It is your responsibility to ensure that this `ManuallyDrop` is not used again."] #[doc = ""] #[must_use = "if you don't need the value, you can use `ManuallyDrop::drop` instead"] #[stable (feature = "manually_drop_take" , since = "1.42.0")] #[inline] pub unsafe fn take (slot : & mut ManuallyDrop < T >) -> T { unsafe { ptr :: read (& slot . value) } } }}}
mkitem!{mkimpl!{impl < T : ? Sized > ManuallyDrop < T > { #[doc = " Manually drops the contained value."] #[doc = ""] #[doc = " This is exactly equivalent to calling [`ptr::drop_in_place`] with a"] #[doc = " pointer to the contained value. As such, unless the contained value is a"] #[doc = " packed struct, the destructor will be called in-place without moving the"] #[doc = " value, and thus can be used to safely drop [pinned] data."] #[doc = ""] #[doc = " If you have ownership of the value, you can use [`ManuallyDrop::into_inner`] instead."] #[doc = ""] #[doc = " # Safety"] #[doc = ""] #[doc = " This function runs the destructor of the contained value. Other than changes made by"] #[doc = " the destructor itself, the memory is left unchanged, and so as far as the compiler is"] #[doc = " concerned still holds a bit-pattern which is valid for the type `T`."] #[doc = ""] #[doc = " However, this \"zombie\" value should not be exposed to safe code, and this function"] #[doc = " should not be called more than once. To use a value after it's been dropped, or drop"] #[doc = " a value multiple times, can cause Undefined Behavior (depending on what `drop` does)."] #[doc = " This is normally prevented by the type system, but users of `ManuallyDrop` must"] #[doc = " uphold those guarantees without assistance from the compiler."] #[doc = ""] #[doc = " [pinned]: crate::pin"] #[stable (feature = "manually_drop" , since = "1.20.0")] #[inline] pub unsafe fn drop (slot : & mut ManuallyDrop < T >) { unsafe { ptr :: drop_in_place (& mut slot . value) } } }}}
mkitem!{#[stable (feature = "manually_drop" , since = "1.20.0")] #[rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl < T : ? Sized > const Deref for ManuallyDrop < T > { type Target = T ; #[inline (always)] fn deref (& self) -> & T { & self . value } }}
mkitem!{#[stable (feature = "manually_drop" , since = "1.20.0")] #[rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl < T : ? Sized > const DerefMut for ManuallyDrop < T > { #[inline (always)] fn deref_mut (& mut self) -> & mut T { & mut self . value } }}
mkitem!{mkimpl!{#[unstable (feature = "deref_pure_trait" , issue = "87121")] unsafe impl < T : ? Sized > DerefPure for ManuallyDrop < T > { }}}