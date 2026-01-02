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
mkuse!{use std :: fmt :: { Debug , Formatter } ;}
mkuse!{use std :: io ;}
mkuse!{use rustc_public_bridge :: bridge ;}
mkuse!{use serde :: Serialize ;}
mkuse!{use crate :: abi :: FnAbi ;}
mkuse!{use crate :: crate_def :: CrateDef ;}
mkuse!{use crate :: mir :: Body ;}
mkuse!{use crate :: ty :: { Allocation , ClosureDef , ClosureKind , FnDef , GenericArgs , Ty } ;}
mkuse!{use crate :: { CrateItem , DefId , Error , IndexedVal , ItemKind , Opaque , Symbol , with } ;}
mkitem!{mkenum!{#[derive (Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub enum MonoItem { Fn (Instance) , Static (StaticDef) , GlobalAsm (Opaque) , }}}
mkitem!{mkstruct!{#[derive (Copy , Clone , PartialEq , Eq , Hash , Serialize)] pub struct Instance { #[doc = " The type of instance."] pub kind : InstanceKind , #[doc = " An ID used to get the instance definition from the compiler."] #[doc = " Do not use this field directly."] pub def : InstanceDef , }}}
mkitem!{mkenum!{#[derive (Copy , Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub enum InstanceKind { #[doc = " A user defined item."] Item , #[doc = " A compiler intrinsic function."] Intrinsic , #[doc = " A virtual function definition stored in a VTable."] #[doc = " The `idx` field indicates the position in the VTable for this instance."] Virtual { idx : usize } , #[doc = " A compiler generated shim."] Shim , }}}
mkitem!{mkimpl!{impl Instance { #[doc = " Get the arguments this instance was instantiated with."] pub fn args (& self) -> GenericArgs { with (| cx | cx . instance_args (self . def)) } #[doc = " Get the body of an Instance."] #[doc = ""] #[doc = " The body will be eagerly monomorphized and all constants will already be evaluated."] #[doc = ""] #[doc = " This method will return the intrinsic fallback body if one was defined."] pub fn body (& self) -> Option < Body > { with (| context | context . instance_body (self . def)) } #[doc = " Check whether this instance has a body available."] #[doc = ""] #[doc = " For intrinsics with fallback body, this will return `true`. It is up to the user to decide"] #[doc = " whether to specialize the intrinsic or to use its fallback body."] #[doc = ""] #[doc = " For more information on fallback body, see <https://github.com/rust-lang/rust/issues/93145>."] #[doc = ""] #[doc = " This call is much cheaper than `instance.body().is_some()`, since it doesn't try to build"] #[doc = " the rustc_public's IR body."] pub fn has_body (& self) -> bool { with (| cx | cx . has_body (self . def . def_id ())) } pub fn is_foreign_item (& self) -> bool { with (| cx | cx . is_foreign_item (self . def . def_id ())) } #[doc = " Get the instance type with generic instantiations applied and lifetimes erased."] pub fn ty (& self) -> Ty { with (| context | context . instance_ty (self . def)) } #[doc = " Retrieve information about this instance binary interface."] pub fn fn_abi (& self) -> Result < FnAbi , Error > { with (| cx | cx . instance_abi (self . def)) } #[doc = " Retrieve the instance's mangled name used for calling the given instance."] #[doc = ""] #[doc = " This will also look up the correct name of instances from upstream crates."] pub fn mangled_name (& self) -> Symbol { with (| context | context . instance_mangled_name (self . def)) } #[doc = " Retrieve the instance name for diagnostic messages."] #[doc = ""] #[doc = " This will return the specialized name, e.g., `std::vec::Vec<u8>::new`."] pub fn name (& self) -> Symbol { with (| context | context . instance_name (self . def , false)) } #[doc = " Return a trimmed name of the given instance including its args."] #[doc = ""] #[doc = " If a symbol name can only be imported from one place for a type, and as"] #[doc = " long as it was not glob-imported anywhere in the current crate, we trim its"] #[doc = " path and print only the name."] pub fn trimmed_name (& self) -> Symbol { with (| context | context . instance_name (self . def , true)) } #[doc = " Retrieve the plain intrinsic name of an instance if it's an intrinsic."] #[doc = ""] #[doc = " The plain name does not include type arguments (as `trimmed_name` does),"] #[doc = " which is more convenient to match with intrinsic symbols."] pub fn intrinsic_name (& self) -> Option < Symbol > { match self . kind { InstanceKind :: Intrinsic => { Some (with (| context | context . intrinsic (self . def . def_id ()) . unwrap () . fn_name ())) } InstanceKind :: Item | InstanceKind :: Virtual { .. } | InstanceKind :: Shim => None , } } #[doc = " Resolve an instance starting from a function definition and generic arguments."] pub fn resolve (def : FnDef , args : & GenericArgs) -> Result < Instance , Error > { with (| context | { context . resolve_instance (def , args) . ok_or_else (| | { bridge :: Error :: new (format ! ("Failed to resolve `{def:?}` with `{args:?}`")) }) }) } #[doc = " Resolve the drop in place for a given type."] pub fn resolve_drop_in_place (ty : Ty) -> Instance { with (| cx | cx . resolve_drop_in_place (ty)) } #[doc = " Resolve an instance for a given function pointer."] pub fn resolve_for_fn_ptr (def : FnDef , args : & GenericArgs) -> Result < Instance , Error > { with (| context | { context . resolve_for_fn_ptr (def , args) . ok_or_else (| | { bridge :: Error :: new (format ! ("Failed to resolve `{def:?}` with `{args:?}`")) }) }) } #[doc = " Resolve a closure with the expected kind."] pub fn resolve_closure (def : ClosureDef , args : & GenericArgs , kind : ClosureKind ,) -> Result < Instance , Error > { with (| context | { context . resolve_closure (def , args , kind) . ok_or_else (| | { bridge :: Error :: new (format ! ("Failed to resolve `{def:?}` with `{args:?}`")) }) }) } #[doc = " Check whether this instance is an empty shim."] #[doc = ""] #[doc = " Allow users to check if this shim can be ignored when called directly."] #[doc = ""] #[doc = " We have decided not to export different types of Shims to rustc_public users, however, this"] #[doc = " is a query that can be very helpful for users when processing DropGlue."] #[doc = ""] #[doc = " When generating code for a Drop terminator, users can ignore an empty drop glue."] #[doc = " These shims are only needed to generate a valid Drop call done via VTable."] pub fn is_empty_shim (& self) -> bool { self . kind == InstanceKind :: Shim && with (| cx | cx . is_empty_drop_shim (self . def)) } #[doc = " Try to constant evaluate the instance into a constant with the given type."] #[doc = ""] #[doc = " This can be used to retrieve a constant that represents an intrinsic return such as"] #[doc = " `type_id`."] pub fn try_const_eval (& self , const_ty : Ty) -> Result < Allocation , Error > { with (| cx | cx . eval_instance (self . def , const_ty)) } #[doc = " Emit the body of this instance if it has one."] pub fn emit_mir < W : io :: Write > (& self , w : & mut W) -> io :: Result < () > { if let Some (body) = self . body () { body . dump (w , & self . name ()) } else { Ok (()) } } }}}
mkitem!{mkimpl!{impl Debug for Instance { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Instance") . field ("kind" , & self . kind) . field ("def" , & self . mangled_name ()) . field ("args" , & self . args ()) . finish () } }}}
mkitem!{mkimpl!{#[doc = " Try to convert a crate item into an instance."] #[doc = " The item cannot be generic in order to be converted into an instance."] impl TryFrom < CrateItem > for Instance { type Error = crate :: Error ; fn try_from (item : CrateItem) -> Result < Self , Self :: Error > { with (| context | { let def_id = item . def_id () ; if ! context . requires_monomorphization (def_id) { Ok (context . mono_instance (def_id)) } else { Err (bridge :: Error :: new ("Item requires monomorphization" . to_string ())) } }) } }}}
mkitem!{mkimpl!{#[doc = " Try to convert an instance into a crate item."] #[doc = " Only user defined instances can be converted."] impl TryFrom < Instance > for CrateItem { type Error = crate :: Error ; fn try_from (value : Instance) -> Result < Self , Self :: Error > { with (| context | { if value . kind == InstanceKind :: Item && context . has_body (value . def . def_id ()) { Ok (CrateItem (context . instance_def_id (value . def))) } else { Err (bridge :: Error :: new (format ! ("Item kind `{:?}` cannot be converted" , value . kind))) } }) } }}}
mkitem!{mkimpl!{impl From < Instance > for MonoItem { fn from (value : Instance) -> Self { MonoItem :: Fn (value) } }}}
mkitem!{mkimpl!{impl From < StaticDef > for MonoItem { fn from (value : StaticDef) -> Self { MonoItem :: Static (value) } }}}
mkitem!{mkimpl!{impl From < StaticDef > for CrateItem { fn from (value : StaticDef) -> Self { CrateItem (value . 0) } }}}
mkitem!{mkstruct!{#[derive (Clone , Copy , Debug , PartialEq , Eq , Hash , Serialize)] pub struct InstanceDef (usize) ;}}
mkitem!{mkimpl!{impl CrateDef for InstanceDef { fn def_id (& self) -> DefId { with (| context | context . instance_def_id (* self)) } }}}
mkitem!{crate_def ! { #[doc = " Holds information about a static variable definition."] #[derive (Serialize)] pub StaticDef ; }}
mkitem!{mkimpl!{impl TryFrom < CrateItem > for StaticDef { type Error = crate :: Error ; fn try_from (value : CrateItem) -> Result < Self , Self :: Error > { if matches ! (value . kind () , ItemKind :: Static) { Ok (StaticDef (value . 0)) } else { Err (bridge :: Error :: new (format ! ("Expected a static item, but found: {value:?}"))) } } }}}
mkitem!{mkimpl!{impl TryFrom < Instance > for StaticDef { type Error = crate :: Error ; fn try_from (value : Instance) -> Result < Self , Self :: Error > { StaticDef :: try_from (CrateItem :: try_from (value) ?) } }}}
mkitem!{mkimpl!{impl From < StaticDef > for Instance { fn from (value : StaticDef) -> Self { with (| cx | cx . mono_instance (value . def_id ())) } }}}
mkitem!{mkimpl!{impl StaticDef { #[doc = " Return the type of this static definition."] pub fn ty (& self) -> Ty { with (| cx | cx . def_ty (self . 0)) } #[doc = " Evaluate a static's initializer, returning the allocation of the initializer's memory."] pub fn eval_initializer (& self) -> Result < Allocation , Error > { with (| cx | cx . eval_static_initializer (* self)) } }}}
mkitem!{mkimpl!{impl IndexedVal for InstanceDef { fn to_val (index : usize) -> Self { InstanceDef (index) } fn to_index (& self) -> usize { self . 0 } }}}