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
mkuse!{use std :: ops :: ControlFlow ;}
mkuse!{use derive_where :: derive_where ;}
mkuse!{use rustc_type_ir_macros :: { Lift_Generic , TypeFoldable_Generic , TypeVisitable_Generic } ;}
mkuse!{use crate :: data_structures :: DelayedMap ;}
mkuse!{use crate :: fold :: { TypeFoldable , TypeFolder , TypeSuperFoldable , shift_region } ;}
mkuse!{use crate :: inherent :: * ;}
mkuse!{use crate :: visit :: { TypeSuperVisitable , TypeVisitable , TypeVisitableExt , TypeVisitor } ;}
mkuse!{use crate :: { self as ty , Interner } ;}
mkitem!{mkstruct!{#[doc = " A closure can be modeled as a struct that looks like:"] #[doc = " ```ignore (illustrative)"] #[doc = " struct Closure<'l0...'li, T0...Tj, CK, CS, U>(...U);"] #[doc = " ```"] #[doc = " where:"] #[doc = ""] #[doc = " - 'l0...'li and T0...Tj are the generic parameters"] #[doc = "   in scope on the function that defined the closure,"] #[doc = " - CK represents the *closure kind* (Fn vs FnMut vs FnOnce). This"] #[doc = "   is rather hackily encoded via a scalar type. See"] #[doc = "   `Ty::to_opt_closure_kind` for details."] #[doc = " - CS represents the *closure signature*, representing as a `fn()`"] #[doc = "   type. For example, `fn(u32, u32) -> u32` would mean that the closure"] #[doc = "   implements `CK<(u32, u32), Output = u32>`, where `CK` is the trait"] #[doc = "   specified above."] #[doc = " - U is a type parameter representing the types of its upvars, tupled up"] #[doc = "   (borrowed, if appropriate; that is, if a U field represents a by-ref upvar,"] #[doc = "    and the up-var has the type `Foo`, then that field of U will be `&Foo`)."] #[doc = ""] #[doc = " So, for example, given this function:"] #[doc = " ```ignore (illustrative)"] #[doc = " fn foo<'a, T>(data: &'a mut T) {"] #[doc = "      do(|| data.count += 1)"] #[doc = " }"] #[doc = " ```"] #[doc = " the type of the closure would be something like:"] #[doc = " ```ignore (illustrative)"] #[doc = " struct Closure<'a, T, U>(...U);"] #[doc = " ```"] #[doc = " Note that the type of the upvar is not specified in the struct."] #[doc = " You may wonder how the impl would then be able to use the upvar,"] #[doc = " if it doesn't know it's type? The answer is that the impl is"] #[doc = " (conceptually) not fully generic over Closure but rather tied to"] #[doc = " instances with the expected upvar types:"] #[doc = " ```ignore (illustrative)"] #[doc = " impl<'b, 'a, T> FnMut() for Closure<'a, T, (&'b mut &'a mut T,)> {"] #[doc = "     ..."] #[doc = " }"] #[doc = " ```"] #[doc = " You can see that the *impl* fully specified the type of the upvar"] #[doc = " and thus knows full well that `data` has type `&'b mut &'a mut T`."] #[doc = " (Here, I am assuming that `data` is mut-borrowed.)"] #[doc = ""] #[doc = " Now, the last question you may ask is: Why include the upvar types"] #[doc = " in an extra type parameter? The reason for this design is that the"] #[doc = " upvar types can reference lifetimes that are internal to the"] #[doc = " creating function. In my example above, for example, the lifetime"] #[doc = " `'b` represents the scope of the closure itself; this is some"] #[doc = " subset of `foo`, probably just the scope of the call to the to"] #[doc = " `do()`. If we just had the lifetime/type parameters from the"] #[doc = " enclosing function, we couldn't name this lifetime `'b`. Note that"] #[doc = " there can also be lifetimes in the types of the upvars themselves,"] #[doc = " if one of them happens to be a reference to something that the"] #[doc = " creating fn owns."] #[doc = ""] #[doc = " OK, you say, so why not create a more minimal set of parameters"] #[doc = " that just includes the extra lifetime parameters? The answer is"] #[doc = " primarily that it would be hard --- we don't know at the time when"] #[doc = " we create the closure type what the full types of the upvars are,"] #[doc = " nor do we know which are borrowed and which are not. In this"] #[doc = " design, we can just supply a fresh type parameter and figure that"] #[doc = " out later."] #[doc = ""] #[doc = " All right, you say, but why include the type parameters from the"] #[doc = " original function then? The answer is that codegen may need them"] #[doc = " when monomorphizing, and they may not appear in the upvars. A"] #[doc = " closure could capture no variables but still make use of some"] #[doc = " in-scope type parameter with a bound (e.g., if our example above"] #[doc = " had an extra `U: Default`, and the closure called `U::default()`)."] #[doc = ""] #[doc = " There is another reason. This design (implicitly) prohibits"] #[doc = " closures from capturing themselves (except via a trait"] #[doc = " object). This simplifies closure inference considerably, since it"] #[doc = " means that when we infer the kind of a closure or its upvars, we"] #[doc = " don't have to handle cycles where the decisions we make for"] #[doc = " closure C wind up influencing the decisions we ought to make for"] #[doc = " closure C (which would then require fixed point iteration to"] #[doc = " handle). Plus it fixes an ICE. :P"] #[doc = ""] #[doc = " ## Coroutines"] #[doc = ""] #[doc = " Coroutines are handled similarly in `CoroutineArgs`. The set of"] #[doc = " type parameters is similar, but `CK` and `CS` are replaced by the"] #[doc = " following type parameters:"] #[doc = ""] #[doc = " * `GS`: The coroutine's \"resume type\", which is the type of the"] #[doc = "   argument passed to `resume`, and the type of `yield` expressions"] #[doc = "   inside the coroutine."] #[doc = " * `GY`: The \"yield type\", which is the type of values passed to"] #[doc = "   `yield` inside the coroutine."] #[doc = " * `GR`: The \"return type\", which is the type of value returned upon"] #[doc = "   completion of the coroutine."] #[derive_where (Clone , Copy , PartialEq , Hash , Debug ; I : Interner)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] pub struct ClosureArgs < I : Interner > { #[doc = " Lifetime and type parameters from the enclosing function,"] #[doc = " concatenated with a tuple containing the types of the upvars."] #[doc = ""] #[doc = " These are separated out because codegen wants to pass them around"] #[doc = " when monomorphizing."] pub args : I :: GenericArgs , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for ClosureArgs < I > { }}}
mkitem!{mkstruct!{#[doc = " Struct returned by `split()`."] pub struct ClosureArgsParts < I : Interner > { #[doc = " This is the args of the typeck root."] pub parent_args : I :: GenericArgsSlice , #[doc = " Represents the maximum calling capability of the closure."] pub closure_kind_ty : I :: Ty , #[doc = " Captures the closure's signature. This closure signature is \"tupled\", and"] #[doc = " thus has a peculiar signature of `extern \"rust-call\" fn((Args, ...)) -> Ty`."] pub closure_sig_as_fn_ptr_ty : I :: Ty , #[doc = " The upvars captured by the closure. Remains an inference variable"] #[doc = " until the upvar analysis, which happens late in HIR typeck."] pub tupled_upvars_ty : I :: Ty , }}}
mkitem!{mkimpl!{impl < I : Interner > ClosureArgs < I > { #[doc = " Construct `ClosureArgs` from `ClosureArgsParts`, containing `Args`"] #[doc = " for the closure parent, alongside additional closure-specific components."] pub fn new (cx : I , parts : ClosureArgsParts < I >) -> ClosureArgs < I > { ClosureArgs { args : cx . mk_args_from_iter (parts . parent_args . iter () . chain ([parts . closure_kind_ty . into () , parts . closure_sig_as_fn_ptr_ty . into () , parts . tupled_upvars_ty . into () ,])) , } } #[doc = " Divides the closure args into their respective components."] #[doc = " The ordering assumed here must match that used by `ClosureArgs::new` above."] fn split (self) -> ClosureArgsParts < I > { self . args . split_closure_args () } #[doc = " Returns the generic parameters of the closure's parent."] pub fn parent_args (self) -> I :: GenericArgsSlice { self . split () . parent_args } #[doc = " Returns an iterator over the list of types of captured paths by the closure."] #[doc = " In case there was a type error in figuring out the types of the captured path, an"] #[doc = " empty iterator is returned."] #[inline] pub fn upvar_tys (self) -> I :: Tys { match self . tupled_upvars_ty () . kind () { ty :: Error (_) => Default :: default () , ty :: Tuple (tys) => tys , ty :: Infer (_) => panic ! ("upvar_tys called before capture types are inferred") , ty => panic ! ("Unexpected representation of upvar types tuple {:?}" , ty) , } } #[doc = " Returns the tuple type representing the upvars for this closure."] #[inline] pub fn tupled_upvars_ty (self) -> I :: Ty { self . split () . tupled_upvars_ty } #[doc = " Returns the closure kind for this closure; may return a type"] #[doc = " variable during inference. To get the closure kind during"] #[doc = " inference, use `infcx.closure_kind(args)`."] pub fn kind_ty (self) -> I :: Ty { self . split () . closure_kind_ty } #[doc = " Returns the `fn` pointer type representing the closure signature for this"] #[doc = " closure."] pub fn sig_as_fn_ptr_ty (self) -> I :: Ty { self . split () . closure_sig_as_fn_ptr_ty } #[doc = " Returns the closure kind for this closure; only usable outside"] #[doc = " of an inference context, because in that context we know that"] #[doc = " there are no type variables."] #[doc = ""] #[doc = " If you have an inference context, use `infcx.closure_kind()`."] pub fn kind (self) -> ty :: ClosureKind { self . kind_ty () . to_opt_closure_kind () . unwrap () } #[doc = " Extracts the signature from the closure."] pub fn sig (self) -> ty :: Binder < I , ty :: FnSig < I > > { match self . sig_as_fn_ptr_ty () . kind () { ty :: FnPtr (sig_tys , hdr) => sig_tys . with (hdr) , ty => panic ! ("closure_sig_as_fn_ptr_ty is not a fn-ptr: {ty:?}") , } } }}}
mkitem!{mkstruct!{#[derive_where (Clone , Copy , PartialEq , Hash , Debug ; I : Interner)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] pub struct CoroutineClosureArgs < I : Interner > { pub args : I :: GenericArgs , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for CoroutineClosureArgs < I > { }}}
mkitem!{mkstruct!{#[doc = " See docs for explanation of how each argument is used."] #[doc = ""] #[doc = " See [`CoroutineClosureSignature`] for how these arguments are put together"] #[doc = " to make a callable [`ty::FnSig`] suitable for typeck and borrowck."] pub struct CoroutineClosureArgsParts < I : Interner > { #[doc = " This is the args of the typeck root."] pub parent_args : I :: GenericArgsSlice , #[doc = " Represents the maximum calling capability of the closure."] pub closure_kind_ty : I :: Ty , #[doc = " Represents all of the relevant parts of the coroutine returned by this"] #[doc = " coroutine-closure. This signature parts type will have the general"] #[doc = " shape of `fn(tupled_inputs, resume_ty) -> (return_ty, yield_ty)`, where"] #[doc = " `resume_ty`, `return_ty`, and `yield_ty` are the respective types for the"] #[doc = " coroutine returned by the coroutine-closure."] #[doc = ""] #[doc = " Use `coroutine_closure_sig` to break up this type rather than using it"] #[doc = " yourself."] pub signature_parts_ty : I :: Ty , #[doc = " The upvars captured by the closure. Remains an inference variable"] #[doc = " until the upvar analysis, which happens late in HIR typeck."] pub tupled_upvars_ty : I :: Ty , #[doc = " a function pointer that has the shape `for<'env> fn() -> (&'env T, ...)`."] #[doc = " This allows us to represent the binder of the self-captures of the closure."] #[doc = ""] #[doc = " For example, if the coroutine returned by the closure borrows `String`"] #[doc = " from the closure's upvars, this will be `for<'env> fn() -> (&'env String,)`,"] #[doc = " while the `tupled_upvars_ty`, representing the by-move version of the same"] #[doc = " captures, will be `(String,)`."] pub coroutine_captures_by_ref_ty : I :: Ty , }}}
mkitem!{mkimpl!{impl < I : Interner > CoroutineClosureArgs < I > { pub fn new (cx : I , parts : CoroutineClosureArgsParts < I >) -> CoroutineClosureArgs < I > { CoroutineClosureArgs { args : cx . mk_args_from_iter (parts . parent_args . iter () . chain ([parts . closure_kind_ty . into () , parts . signature_parts_ty . into () , parts . tupled_upvars_ty . into () , parts . coroutine_captures_by_ref_ty . into () ,])) , } } fn split (self) -> CoroutineClosureArgsParts < I > { self . args . split_coroutine_closure_args () } pub fn parent_args (self) -> I :: GenericArgsSlice { self . split () . parent_args } #[inline] pub fn upvar_tys (self) -> I :: Tys { match self . tupled_upvars_ty () . kind () { ty :: Error (_) => Default :: default () , ty :: Tuple (..) => self . tupled_upvars_ty () . tuple_fields () , ty :: Infer (_) => panic ! ("upvar_tys called before capture types are inferred") , ty => panic ! ("Unexpected representation of upvar types tuple {:?}" , ty) , } } #[inline] pub fn tupled_upvars_ty (self) -> I :: Ty { self . split () . tupled_upvars_ty } pub fn kind_ty (self) -> I :: Ty { self . split () . closure_kind_ty } pub fn kind (self) -> ty :: ClosureKind { self . kind_ty () . to_opt_closure_kind () . unwrap () } pub fn signature_parts_ty (self) -> I :: Ty { self . split () . signature_parts_ty } pub fn coroutine_closure_sig (self) -> ty :: Binder < I , CoroutineClosureSignature < I > > { let ty :: FnPtr (sig_tys , hdr) = self . signature_parts_ty () . kind () else { panic ! () } ; sig_tys . map_bound (| sig_tys | { let [resume_ty , tupled_inputs_ty] = * sig_tys . inputs () . as_slice () else { panic ! () ; } ; let [yield_ty , return_ty] = * sig_tys . output () . tuple_fields () . as_slice () else { panic ! () } ; CoroutineClosureSignature { tupled_inputs_ty , resume_ty , yield_ty , return_ty , c_variadic : hdr . c_variadic , safety : hdr . safety , abi : hdr . abi , } }) } pub fn coroutine_captures_by_ref_ty (self) -> I :: Ty { self . split () . coroutine_captures_by_ref_ty } pub fn has_self_borrows (& self) -> bool { match self . coroutine_captures_by_ref_ty () . kind () { ty :: FnPtr (sig_tys , _) => sig_tys . skip_binder () . visit_with (& mut HasRegionsBoundAt { binder : ty :: INNERMOST }) . is_break () , ty :: Error (_) => true , _ => panic ! () , } } }}}
mkitem!{mkstruct!{#[doc = " Unlike `has_escaping_bound_vars` or `outermost_exclusive_binder`, this will"] #[doc = " detect only regions bound *at* the debruijn index."] struct HasRegionsBoundAt { binder : ty :: DebruijnIndex , }}}
mkitem!{mkimpl!{impl < I : Interner > TypeVisitor < I > for HasRegionsBoundAt { type Result = ControlFlow < () > ; fn visit_binder < T : TypeVisitable < I > > (& mut self , t : & ty :: Binder < I , T >) -> Self :: Result { self . binder . shift_in (1) ; t . super_visit_with (self) ? ; self . binder . shift_out (1) ; ControlFlow :: Continue (()) } fn visit_region (& mut self , r : I :: Region) -> Self :: Result { if matches ! (r . kind () , ty :: ReBound (binder , _) if self . binder == binder) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } } }}}
mkitem!{mkstruct!{#[derive_where (Clone , Copy , PartialEq , Hash , Debug ; I : Interner)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic)] pub struct CoroutineClosureSignature < I : Interner > { pub tupled_inputs_ty : I :: Ty , pub resume_ty : I :: Ty , pub yield_ty : I :: Ty , pub return_ty : I :: Ty , #[doc = " Always false"] pub c_variadic : bool , #[doc = " Always `Normal` (safe)"] #[type_visitable (ignore)] #[type_foldable (identity)] pub safety : I :: Safety , #[doc = " Always `RustCall`"] #[type_visitable (ignore)] #[type_foldable (identity)] pub abi : I :: Abi , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for CoroutineClosureSignature < I > { }}}
mkitem!{mkimpl!{impl < I : Interner > CoroutineClosureSignature < I > { #[doc = " Construct a coroutine from the closure signature. Since a coroutine signature"] #[doc = " is agnostic to the type of generator that is returned (by-ref/by-move),"] #[doc = " the caller must specify what \"flavor\" of generator that they'd like to"] #[doc = " create. Additionally, they must manually compute the upvars of the closure."] #[doc = ""] #[doc = " This helper is not really meant to be used directly except for early on"] #[doc = " during typeck, when we want to put inference vars into the kind and upvars tys."] #[doc = " When the kind and upvars are known, use the other helper functions."] pub fn to_coroutine (self , cx : I , parent_args : I :: GenericArgsSlice , coroutine_kind_ty : I :: Ty , coroutine_def_id : I :: CoroutineId , tupled_upvars_ty : I :: Ty ,) -> I :: Ty { let coroutine_args = ty :: CoroutineArgs :: new (cx , ty :: CoroutineArgsParts { parent_args , kind_ty : coroutine_kind_ty , resume_ty : self . resume_ty , yield_ty : self . yield_ty , return_ty : self . return_ty , tupled_upvars_ty , } ,) ; Ty :: new_coroutine (cx , coroutine_def_id , coroutine_args . args) } #[doc = " Given known upvars and a [`ClosureKind`](ty::ClosureKind), compute the coroutine"] #[doc = " returned by that corresponding async fn trait."] #[doc = ""] #[doc = " This function expects the upvars to have been computed already, and doesn't check"] #[doc = " that the `ClosureKind` is actually supported by the coroutine-closure."] pub fn to_coroutine_given_kind_and_upvars (self , cx : I , parent_args : I :: GenericArgsSlice , coroutine_def_id : I :: CoroutineId , goal_kind : ty :: ClosureKind , env_region : I :: Region , closure_tupled_upvars_ty : I :: Ty , coroutine_captures_by_ref_ty : I :: Ty ,) -> I :: Ty { let tupled_upvars_ty = Self :: tupled_upvars_by_closure_kind (cx , goal_kind , self . tupled_inputs_ty , closure_tupled_upvars_ty , coroutine_captures_by_ref_ty , env_region ,) ; self . to_coroutine (cx , parent_args , Ty :: from_coroutine_closure_kind (cx , goal_kind) , coroutine_def_id , tupled_upvars_ty ,) } #[doc = " Compute the tupled upvars that a coroutine-closure's output coroutine"] #[doc = " would return for the given `ClosureKind`."] #[doc = ""] #[doc = " When `ClosureKind` is `FnMut`/`Fn`, then this will use the \"captures by ref\""] #[doc = " to return a set of upvars which are borrowed with the given `env_region`."] #[doc = ""] #[doc = " This ensures that the `AsyncFn::call` will return a coroutine whose upvars'"] #[doc = " lifetimes are related to the lifetime of the borrow on the closure made for"] #[doc = " the call. This allows borrowck to enforce the self-borrows correctly."] pub fn tupled_upvars_by_closure_kind (cx : I , kind : ty :: ClosureKind , tupled_inputs_ty : I :: Ty , closure_tupled_upvars_ty : I :: Ty , coroutine_captures_by_ref_ty : I :: Ty , env_region : I :: Region ,) -> I :: Ty { match kind { ty :: ClosureKind :: Fn | ty :: ClosureKind :: FnMut => { let ty :: FnPtr (sig_tys , _) = coroutine_captures_by_ref_ty . kind () else { panic ! () ; } ; let coroutine_captures_by_ref_ty = sig_tys . output () . skip_binder () . fold_with (& mut FoldEscapingRegions { interner : cx , region : env_region , debruijn : ty :: INNERMOST , cache : Default :: default () , }) ; Ty :: new_tup_from_iter (cx , tupled_inputs_ty . tuple_fields () . iter () . chain (coroutine_captures_by_ref_ty . tuple_fields () . iter ()) ,) } ty :: ClosureKind :: FnOnce => Ty :: new_tup_from_iter (cx , tupled_inputs_ty . tuple_fields () . iter () . chain (closure_tupled_upvars_ty . tuple_fields () . iter ()) ,) , } } }}}
mkitem!{mkstruct!{#[doc = " Instantiates a `for<'env> ...` binder with a specific region."] struct FoldEscapingRegions < I : Interner > { interner : I , debruijn : ty :: DebruijnIndex , region : I :: Region , cache : DelayedMap < (ty :: DebruijnIndex , I :: Ty) , I :: Ty > , }}}
mkitem!{mkimpl!{impl < I : Interner > TypeFolder < I > for FoldEscapingRegions < I > { fn cx (& self) -> I { self . interner } fn fold_ty (& mut self , t : I :: Ty) -> I :: Ty { if ! t . has_vars_bound_at_or_above (self . debruijn) { t } else if let Some (& t) = self . cache . get (& (self . debruijn , t)) { t } else { let res = t . super_fold_with (self) ; assert ! (self . cache . insert ((self . debruijn , t) , res)) ; res } } fn fold_binder < T > (& mut self , t : ty :: Binder < I , T >) -> ty :: Binder < I , T > where T : TypeFoldable < I > , { self . debruijn . shift_in (1) ; let result = t . super_fold_with (self) ; self . debruijn . shift_out (1) ; result } fn fold_region (& mut self , r : < I as Interner > :: Region) -> < I as Interner > :: Region { if let ty :: ReBound (debruijn , _) = r . kind () { assert ! (debruijn <= self . debruijn , "cannot instantiate binder with escaping bound vars") ; if self . debruijn == debruijn { shift_region (self . interner , self . region , self . debruijn . as_u32 ()) } else { r } } else { r } } }}}
mkitem!{mkstruct!{#[derive_where (Clone , Copy , PartialEq , Hash , Debug ; I : Interner)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic)] pub struct GenSig < I : Interner > { pub resume_ty : I :: Ty , pub yield_ty : I :: Ty , pub return_ty : I :: Ty , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for GenSig < I > { }}}
mkitem!{mkstruct!{#[doc = " Similar to `ClosureArgs`; see the above documentation for more."] #[derive_where (Clone , Copy , PartialEq , Hash , Debug ; I : Interner)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] pub struct CoroutineArgs < I : Interner > { pub args : I :: GenericArgs , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for CoroutineArgs < I > { }}}
mkitem!{mkstruct!{pub struct CoroutineArgsParts < I : Interner > { #[doc = " This is the args of the typeck root."] pub parent_args : I :: GenericArgsSlice , #[doc = " The coroutines returned by a coroutine-closure's `AsyncFnOnce`/`AsyncFnMut`"] #[doc = " implementations must be distinguished since the former takes the closure's"] #[doc = " upvars by move, and the latter takes the closure's upvars by ref."] #[doc = ""] #[doc = " This field distinguishes these fields so that codegen can select the right"] #[doc = " body for the coroutine. This has the same type representation as the closure"] #[doc = " kind: `i8`/`i16`/`i32`."] #[doc = ""] #[doc = " For regular coroutines, this field will always just be `()`."] pub kind_ty : I :: Ty , pub resume_ty : I :: Ty , pub yield_ty : I :: Ty , pub return_ty : I :: Ty , #[doc = " The upvars captured by the closure. Remains an inference variable"] #[doc = " until the upvar analysis, which happens late in HIR typeck."] pub tupled_upvars_ty : I :: Ty , }}}
mkitem!{mkimpl!{impl < I : Interner > CoroutineArgs < I > { #[doc = " Construct `CoroutineArgs` from `CoroutineArgsParts`, containing `Args`"] #[doc = " for the coroutine parent, alongside additional coroutine-specific components."] pub fn new (cx : I , parts : CoroutineArgsParts < I >) -> CoroutineArgs < I > { CoroutineArgs { args : cx . mk_args_from_iter (parts . parent_args . iter () . chain ([parts . kind_ty . into () , parts . resume_ty . into () , parts . yield_ty . into () , parts . return_ty . into () , parts . tupled_upvars_ty . into () ,])) , } } #[doc = " Divides the coroutine args into their respective components."] #[doc = " The ordering assumed here must match that used by `CoroutineArgs::new` above."] fn split (self) -> CoroutineArgsParts < I > { self . args . split_coroutine_args () } #[doc = " Returns the generic parameters of the coroutine's parent."] pub fn parent_args (self) -> I :: GenericArgsSlice { self . split () . parent_args } pub fn kind_ty (self) -> I :: Ty { self . split () . kind_ty } #[doc = " Returns an iterator over the list of types of captured paths by the coroutine."] #[doc = " In case there was a type error in figuring out the types of the captured path, an"] #[doc = " empty iterator is returned."] #[inline] pub fn upvar_tys (self) -> I :: Tys { match self . tupled_upvars_ty () . kind () { ty :: Error (_) => Default :: default () , ty :: Tuple (tys) => tys , ty :: Infer (_) => panic ! ("upvar_tys called before capture types are inferred") , ty => panic ! ("Unexpected representation of upvar types tuple {:?}" , ty) , } } #[doc = " Returns the tuple type representing the upvars for this coroutine."] #[inline] pub fn tupled_upvars_ty (self) -> I :: Ty { self . split () . tupled_upvars_ty } #[doc = " Returns the type representing the resume type of the coroutine."] pub fn resume_ty (self) -> I :: Ty { self . split () . resume_ty } #[doc = " Returns the type representing the yield type of the coroutine."] pub fn yield_ty (self) -> I :: Ty { self . split () . yield_ty } #[doc = " Returns the type representing the return type of the coroutine."] pub fn return_ty (self) -> I :: Ty { self . split () . return_ty } #[doc = " Returns the \"coroutine signature\", which consists of its resume, yield"] #[doc = " and return types."] pub fn sig (self) -> GenSig < I > { let parts = self . split () ; GenSig { resume_ty : parts . resume_ty , yield_ty : parts . yield_ty , return_ty : parts . return_ty } } }}}