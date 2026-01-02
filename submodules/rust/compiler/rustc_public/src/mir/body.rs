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
mkuse!{use std :: io ;}
mkuse!{use serde :: Serialize ;}
mkuse!{use crate :: compiler_interface :: with ;}
mkuse!{use crate :: mir :: pretty :: function_body ;}
mkuse!{use crate :: ty :: { AdtDef , ClosureDef , CoroutineClosureDef , CoroutineDef , GenericArgs , MirConst , Movability , Region , RigidTy , Ty , TyConst , TyKind , VariantIdx , } ;}
mkuse!{use crate :: { Error , Opaque , Span , Symbol } ;}
mkitem!{mkstruct!{#[doc = " The rustc_public's IR representation of a single function."] #[derive (Clone , Debug , Serialize)] pub struct Body { pub blocks : Vec < BasicBlock > , #[doc = " Declarations of locals within the function."] #[doc = ""] #[doc = " The first local is the return value pointer, followed by `arg_count`"] #[doc = " locals for the function arguments, followed by any user-declared"] #[doc = " variables and temporaries."] pub (super) locals : LocalDecls , #[doc = " The number of arguments this function takes."] pub (super) arg_count : usize , #[doc = " Debug information pertaining to user variables, including captures."] pub var_debug_info : Vec < VarDebugInfo > , #[doc = " Mark an argument (which must be a tuple) as getting passed as its individual components."] #[doc = ""] #[doc = " This is used for the \"rust-call\" ABI such as closures."] pub (super) spread_arg : Option < Local > , #[doc = " The span that covers the entire function body."] pub span : Span , }}}
mkitem!{pub type BasicBlockIdx = usize ;}
mkitem!{mkimpl!{impl Body { #[doc = " Constructs a `Body`."] #[doc = ""] #[doc = " A constructor is required to build a `Body` from outside the crate"] #[doc = " because the `arg_count` and `locals` fields are private."] pub fn new (blocks : Vec < BasicBlock > , locals : LocalDecls , arg_count : usize , var_debug_info : Vec < VarDebugInfo > , spread_arg : Option < Local > , span : Span ,) -> Self { assert ! (locals . len () > arg_count , "A Body must contain at least a local for the return value and each of the function's arguments") ; Self { blocks , locals , arg_count , var_debug_info , spread_arg , span } } #[doc = " Return local that holds this function's return value."] pub fn ret_local (& self) -> & LocalDecl { & self . locals [RETURN_LOCAL] } #[doc = " Locals in `self` that correspond to this function's arguments."] pub fn arg_locals (& self) -> & [LocalDecl] { & self . locals [1 ..] [.. self . arg_count] } #[doc = " Inner locals for this function. These are the locals that are"] #[doc = " neither the return local nor the argument locals."] pub fn inner_locals (& self) -> & [LocalDecl] { & self . locals [self . arg_count + 1 ..] } #[doc = " Returns a mutable reference to the local that holds this function's return value."] pub (crate) fn ret_local_mut (& mut self) -> & mut LocalDecl { & mut self . locals [RETURN_LOCAL] } #[doc = " Returns a mutable slice of locals corresponding to this function's arguments."] pub (crate) fn arg_locals_mut (& mut self) -> & mut [LocalDecl] { & mut self . locals [1 ..] [.. self . arg_count] } #[doc = " Returns a mutable slice of inner locals for this function."] #[doc = " Inner locals are those that are neither the return local nor the argument locals."] pub (crate) fn inner_locals_mut (& mut self) -> & mut [LocalDecl] { & mut self . locals [self . arg_count + 1 ..] } #[doc = " Convenience function to get all the locals in this function."] #[doc = ""] #[doc = " Locals are typically accessed via the more specific methods `ret_local`,"] #[doc = " `arg_locals`, and `inner_locals`."] pub fn locals (& self) -> & [LocalDecl] { & self . locals } #[doc = " Get the local declaration for this local."] pub fn local_decl (& self , local : Local) -> Option < & LocalDecl > { self . locals . get (local) } #[doc = " Get an iterator for all local declarations."] pub fn local_decls (& self) -> impl Iterator < Item = (Local , & LocalDecl) > { self . locals . iter () . enumerate () } #[doc = " Emit the body using the provided name for the signature."] pub fn dump < W : io :: Write > (& self , w : & mut W , fn_name : & str) -> io :: Result < () > { function_body (w , self , fn_name) } pub fn spread_arg (& self) -> Option < Local > { self . spread_arg } }}}
mkitem!{type LocalDecls = Vec < LocalDecl > ;}
mkitem!{mkstruct!{#[derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct LocalDecl { pub ty : Ty , pub span : Span , pub mutability : Mutability , }}}
mkitem!{mkstruct!{#[derive (Clone , PartialEq , Eq , Debug , Serialize)] pub struct BasicBlock { pub statements : Vec < Statement > , pub terminator : Terminator , }}}
mkitem!{mkstruct!{#[derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct Terminator { pub kind : TerminatorKind , pub span : Span , }}}
mkitem!{mkimpl!{impl Terminator { pub fn successors (& self) -> Successors { self . kind . successors () } }}}
mkitem!{pub type Successors = Vec < BasicBlockIdx > ;}
mkitem!{mkenum!{#[derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum TerminatorKind { Goto { target : BasicBlockIdx , } , SwitchInt { discr : Operand , targets : SwitchTargets , } , Resume , Abort , Return , Unreachable , Drop { place : Place , target : BasicBlockIdx , unwind : UnwindAction , } , Call { func : Operand , args : Vec < Operand > , destination : Place , target : Option < BasicBlockIdx > , unwind : UnwindAction , } , Assert { cond : Operand , expected : bool , msg : AssertMessage , target : BasicBlockIdx , unwind : UnwindAction , } , InlineAsm { template : String , operands : Vec < InlineAsmOperand > , options : String , line_spans : String , destination : Option < BasicBlockIdx > , unwind : UnwindAction , } , }}}
mkitem!{mkimpl!{impl TerminatorKind { pub fn successors (& self) -> Successors { use self :: TerminatorKind :: * ; match * self { Call { target : Some (t) , unwind : UnwindAction :: Cleanup (u) , .. } | Drop { target : t , unwind : UnwindAction :: Cleanup (u) , .. } | Assert { target : t , unwind : UnwindAction :: Cleanup (u) , .. } | InlineAsm { destination : Some (t) , unwind : UnwindAction :: Cleanup (u) , .. } => { vec ! [t , u] } Goto { target : t } | Call { target : None , unwind : UnwindAction :: Cleanup (t) , .. } | Call { target : Some (t) , unwind : _ , .. } | Drop { target : t , unwind : _ , .. } | Assert { target : t , unwind : _ , .. } | InlineAsm { destination : None , unwind : UnwindAction :: Cleanup (t) , .. } | InlineAsm { destination : Some (t) , unwind : _ , .. } => { vec ! [t] } Return | Resume | Abort | Unreachable | Call { target : None , unwind : _ , .. } | InlineAsm { destination : None , unwind : _ , .. } => { vec ! [] } SwitchInt { ref targets , .. } => targets . all_targets () , } } pub fn unwind (& self) -> Option < & UnwindAction > { match * self { TerminatorKind :: Goto { .. } | TerminatorKind :: Return | TerminatorKind :: Unreachable | TerminatorKind :: Resume | TerminatorKind :: Abort | TerminatorKind :: SwitchInt { .. } => None , TerminatorKind :: Call { ref unwind , .. } | TerminatorKind :: Assert { ref unwind , .. } | TerminatorKind :: Drop { ref unwind , .. } | TerminatorKind :: InlineAsm { ref unwind , .. } => Some (unwind) , } } }}}
mkitem!{mkstruct!{#[derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct InlineAsmOperand { pub in_value : Option < Operand > , pub out_place : Option < Place > , pub raw_rpr : String , }}}
mkitem!{mkenum!{#[derive (Copy , Clone , Debug , Eq , PartialEq , Serialize)] pub enum UnwindAction { Continue , Unreachable , Terminate , Cleanup (BasicBlockIdx) , }}}
mkitem!{mkenum!{#[derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum AssertMessage { BoundsCheck { len : Operand , index : Operand } , Overflow (BinOp , Operand , Operand) , OverflowNeg (Operand) , DivisionByZero (Operand) , RemainderByZero (Operand) , ResumedAfterReturn (CoroutineKind) , ResumedAfterPanic (CoroutineKind) , ResumedAfterDrop (CoroutineKind) , MisalignedPointerDereference { required : Operand , found : Operand } , NullPointerDereference , InvalidEnumConstruction (Operand) , }}}
mkitem!{mkimpl!{impl AssertMessage { pub fn description (& self) -> Result < & 'static str , Error > { match self { AssertMessage :: Overflow (BinOp :: Add , _ , _) => Ok ("attempt to add with overflow") , AssertMessage :: Overflow (BinOp :: Sub , _ , _) => Ok ("attempt to subtract with overflow") , AssertMessage :: Overflow (BinOp :: Mul , _ , _) => Ok ("attempt to multiply with overflow") , AssertMessage :: Overflow (BinOp :: Div , _ , _) => Ok ("attempt to divide with overflow") , AssertMessage :: Overflow (BinOp :: Rem , _ , _) => { Ok ("attempt to calculate the remainder with overflow") } AssertMessage :: OverflowNeg (_) => Ok ("attempt to negate with overflow") , AssertMessage :: Overflow (BinOp :: Shr , _ , _) => Ok ("attempt to shift right with overflow") , AssertMessage :: Overflow (BinOp :: Shl , _ , _) => Ok ("attempt to shift left with overflow") , AssertMessage :: Overflow (op , _ , _) => Err (error ! ("`{:?}` cannot overflow" , op)) , AssertMessage :: DivisionByZero (_) => Ok ("attempt to divide by zero") , AssertMessage :: RemainderByZero (_) => { Ok ("attempt to calculate the remainder with a divisor of zero") } AssertMessage :: ResumedAfterReturn (CoroutineKind :: Coroutine (_)) => { Ok ("coroutine resumed after completion") } AssertMessage :: ResumedAfterReturn (CoroutineKind :: Desugared (CoroutineDesugaring :: Async , _ ,)) => Ok ("`async fn` resumed after completion") , AssertMessage :: ResumedAfterReturn (CoroutineKind :: Desugared (CoroutineDesugaring :: Gen , _ ,)) => Ok ("`async gen fn` resumed after completion") , AssertMessage :: ResumedAfterReturn (CoroutineKind :: Desugared (CoroutineDesugaring :: AsyncGen , _ ,)) => Ok ("`gen fn` should just keep returning `AssertMessage::None` after completion") , AssertMessage :: ResumedAfterPanic (CoroutineKind :: Coroutine (_)) => { Ok ("coroutine resumed after panicking") } AssertMessage :: ResumedAfterPanic (CoroutineKind :: Desugared (CoroutineDesugaring :: Async , _ ,)) => Ok ("`async fn` resumed after panicking") , AssertMessage :: ResumedAfterPanic (CoroutineKind :: Desugared (CoroutineDesugaring :: Gen , _ ,)) => Ok ("`async gen fn` resumed after panicking") , AssertMessage :: ResumedAfterPanic (CoroutineKind :: Desugared (CoroutineDesugaring :: AsyncGen , _ ,)) => Ok ("`gen fn` should just keep returning `AssertMessage::None` after panicking") , AssertMessage :: ResumedAfterDrop (CoroutineKind :: Coroutine (_)) => { Ok ("coroutine resumed after async drop") } AssertMessage :: ResumedAfterDrop (CoroutineKind :: Desugared (CoroutineDesugaring :: Async , _ ,)) => Ok ("`async fn` resumed after async drop") , AssertMessage :: ResumedAfterDrop (CoroutineKind :: Desugared (CoroutineDesugaring :: Gen , _ ,)) => Ok ("`async gen fn` resumed after async drop") , AssertMessage :: ResumedAfterDrop (CoroutineKind :: Desugared (CoroutineDesugaring :: AsyncGen , _ ,)) => Ok ("`gen fn` should just keep returning `AssertMessage::None` after async drop") , AssertMessage :: BoundsCheck { .. } => Ok ("index out of bounds") , AssertMessage :: MisalignedPointerDereference { .. } => { Ok ("misaligned pointer dereference") } AssertMessage :: NullPointerDereference => Ok ("null pointer dereference occurred") , AssertMessage :: InvalidEnumConstruction (_) => { Ok ("trying to construct an enum from an invalid value") } } } }}}
mkitem!{mkenum!{#[derive (Copy , Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum BinOp { Add , AddUnchecked , Sub , SubUnchecked , Mul , MulUnchecked , Div , Rem , BitXor , BitAnd , BitOr , Shl , ShlUnchecked , Shr , ShrUnchecked , Eq , Lt , Le , Ne , Ge , Gt , Cmp , Offset , }}}
mkitem!{mkimpl!{impl BinOp { #[doc = " Return the type of this operation for the given input Ty."] #[doc = " This function does not perform type checking, and it currently doesn't handle SIMD."] pub fn ty (& self , lhs_ty : Ty , rhs_ty : Ty) -> Ty { with (| ctx | ctx . binop_ty (* self , lhs_ty , rhs_ty)) } }}}
mkitem!{mkenum!{#[derive (Copy , Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum UnOp { Not , Neg , PtrMetadata , }}}
mkitem!{mkimpl!{impl UnOp { #[doc = " Return the type of this operation for the given input Ty."] #[doc = " This function does not perform type checking, and it currently doesn't handle SIMD."] pub fn ty (& self , arg_ty : Ty) -> Ty { with (| ctx | ctx . unop_ty (* self , arg_ty)) } }}}
mkitem!{mkenum!{#[derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum CoroutineKind { Desugared (CoroutineDesugaring , CoroutineSource) , Coroutine (Movability) , }}}
mkitem!{mkenum!{#[derive (Copy , Clone , Debug , Eq , PartialEq , Serialize)] pub enum CoroutineSource { Block , Closure , Fn , }}}
mkitem!{mkenum!{#[derive (Copy , Clone , Debug , Eq , PartialEq , Serialize)] pub enum CoroutineDesugaring { Async , Gen , AsyncGen , }}}
mkitem!{pub (crate) type LocalDefId = Opaque ;}
mkitem!{#[doc = " The rustc coverage data structures are heavily tied to internal details of the"] #[doc = " coverage implementation that are likely to change, and are unlikely to be"] #[doc = " useful to third-party tools for the foreseeable future."] pub (crate) type Coverage = Opaque ;}
mkitem!{mkenum!{#[doc = " The FakeReadCause describes the type of pattern why a FakeRead statement exists."] #[derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum FakeReadCause { ForMatchGuard , ForMatchedPlace (LocalDefId) , ForGuardBinding , ForLet (LocalDefId) , ForIndex , }}}
mkitem!{mkenum!{#[doc = " Describes what kind of retag is to be performed"] #[derive (Copy , Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum RetagKind { FnEntry , TwoPhase , Raw , Default , }}}
mkitem!{mkenum!{#[derive (Copy , Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum Variance { Covariant , Invariant , Contravariant , Bivariant , }}}
mkitem!{mkstruct!{#[derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct CopyNonOverlapping { pub src : Operand , pub dst : Operand , pub count : Operand , }}}
mkitem!{mkenum!{#[derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum NonDivergingIntrinsic { Assume (Operand) , CopyNonOverlapping (CopyNonOverlapping) , }}}
mkitem!{mkstruct!{#[derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct Statement { pub kind : StatementKind , pub span : Span , }}}
mkitem!{mkenum!{#[derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum StatementKind { Assign (Place , Rvalue) , FakeRead (FakeReadCause , Place) , SetDiscriminant { place : Place , variant_index : VariantIdx } , Deinit (Place) , StorageLive (Local) , StorageDead (Local) , Retag (RetagKind , Place) , PlaceMention (Place) , AscribeUserType { place : Place , projections : UserTypeProjection , variance : Variance } , Coverage (Coverage) , Intrinsic (NonDivergingIntrinsic) , ConstEvalCounter , Nop , }}}
mkitem!{mkenum!{#[derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum Rvalue { #[doc = " Creates a pointer with the indicated mutability to the place."] #[doc = ""] #[doc = " This is generated by pointer casts like `&v as *const _` or raw address of expressions like"] #[doc = " `&raw v` or `addr_of!(v)`."] AddressOf (RawPtrKind , Place) , #[doc = " Creates an aggregate value, like a tuple or struct."] #[doc = ""] #[doc = " This is needed because dataflow analysis needs to distinguish"] #[doc = " `dest = Foo { x: ..., y: ... }` from `dest.x = ...; dest.y = ...;` in the case that `Foo`"] #[doc = " has a destructor."] #[doc = ""] #[doc = " Disallowed after deaggregation for all aggregate kinds except `Array` and `Coroutine`. After"] #[doc = " coroutine lowering, `Coroutine` aggregate kinds are disallowed too."] Aggregate (AggregateKind , Vec < Operand >) , #[doc = " * `Offset` has the same semantics as `<*const T>::offset`, except that the second"] #[doc = "   parameter may be a `usize` as well."] #[doc = " * The comparison operations accept `bool`s, `char`s, signed or unsigned integers, floats,"] #[doc = "   raw pointers, or function pointers and return a `bool`. The types of the operands must be"] #[doc = "   matching, up to the usual caveat of the lifetimes in function pointers."] #[doc = " * Left and right shift operations accept signed or unsigned integers not necessarily of the"] #[doc = "   same type and return a value of the same type as their LHS. Like in Rust, the RHS is"] #[doc = "   truncated as needed."] #[doc = " * The `Bit*` operations accept signed integers, unsigned integers, or bools with matching"] #[doc = "   types and return a value of that type."] #[doc = " * The remaining operations accept signed integers, unsigned integers, or floats with"] #[doc = "   matching types and return a value of that type."] BinaryOp (BinOp , Operand , Operand) , #[doc = " Performs essentially all of the casts that can be performed via `as`."] #[doc = ""] #[doc = " This allows for casts from/to a variety of types."] Cast (CastKind , Operand , Ty) , #[doc = " Same as `BinaryOp`, but yields `(T, bool)` with a `bool` indicating an error condition."] #[doc = ""] #[doc = " For addition, subtraction, and multiplication on integers the error condition is set when"] #[doc = " the infinite precision result would not be equal to the actual result."] CheckedBinaryOp (BinOp , Operand , Operand) , #[doc = " A CopyForDeref is equivalent to a read from a place."] #[doc = " When such a read happens, it is guaranteed that the only use of the returned value is a"] #[doc = " deref operation, immediately followed by one or more projections."] CopyForDeref (Place) , #[doc = " Computes the discriminant of the place, returning it as an integer."] #[doc = " Returns zero for types without discriminant."] #[doc = ""] #[doc = " The validity requirements for the underlying value are undecided for this rvalue, see"] #[doc = " [#91095]. Note too that the value of the discriminant is not the same thing as the"] #[doc = " variant index;"] #[doc = ""] #[doc = " [#91095]: https://github.com/rust-lang/rust/issues/91095"] Discriminant (Place) , #[doc = " Yields the length of the place, as a `usize`."] #[doc = ""] #[doc = " If the type of the place is an array, this is the array length. For slices (`[T]`, not"] #[doc = " `&[T]`) this accesses the place's metadata to determine the length. This rvalue is"] #[doc = " ill-formed for places of other types."] Len (Place) , #[doc = " Creates a reference to the place."] Ref (Region , BorrowKind , Place) , #[doc = " Creates an array where each element is the value of the operand."] #[doc = ""] #[doc = " This is the cause of a bug in the case where the repetition count is zero because the value"] #[doc = " is not dropped, see [#74836]."] #[doc = ""] #[doc = " Corresponds to source code like `[x; 32]`."] #[doc = ""] #[doc = " [#74836]: https://github.com/rust-lang/rust/issues/74836"] Repeat (Operand , TyConst) , #[doc = " Transmutes a `*mut u8` into shallow-initialized `Box<T>`."] #[doc = ""] #[doc = " This is different from a normal transmute because dataflow analysis will treat the box as"] #[doc = " initialized but its content as uninitialized. Like other pointer casts, this in general"] #[doc = " affects alias analysis."] ShallowInitBox (Operand , Ty) , #[doc = " Creates a pointer/reference to the given thread local."] #[doc = ""] #[doc = " The yielded type is a `*mut T` if the static is mutable, otherwise if the static is extern a"] #[doc = " `*const T`, and if neither of those apply a `&T`."] #[doc = ""] #[doc = " **Note:** This is a runtime operation that actually executes code and is in this sense more"] #[doc = " like a function call. Also, eliminating dead stores of this rvalue causes `fn main() {}` to"] #[doc = " SIGILL for some reason that I (JakobDegen) never got a chance to look into."] #[doc = ""] #[doc = " **Needs clarification**: Are there weird additional semantics here related to the runtime"] #[doc = " nature of this operation?"] ThreadLocalRef (crate :: CrateItem) , #[doc = " Computes a value as described by the operation."] NullaryOp (NullOp , Ty) , #[doc = " Exactly like `BinaryOp`, but less operands."] #[doc = ""] #[doc = " Also does two's-complement arithmetic. Negation requires a signed integer or a float;"] #[doc = " bitwise not requires a signed integer, unsigned integer, or bool. Both operation kinds"] #[doc = " return a value with the same type as their operand."] UnaryOp (UnOp , Operand) , #[doc = " Yields the operand unchanged"] Use (Operand) , }}}
mkitem!{mkimpl!{impl Rvalue { pub fn ty (& self , locals : & [LocalDecl]) -> Result < Ty , Error > { match self { Rvalue :: Use (operand) => operand . ty (locals) , Rvalue :: Repeat (operand , count) => { Ok (Ty :: new_array_with_const_len (operand . ty (locals) ? , count . clone ())) } Rvalue :: ThreadLocalRef (did) => Ok (did . ty ()) , Rvalue :: Ref (reg , bk , place) => { let place_ty = place . ty (locals) ? ; Ok (Ty :: new_ref (reg . clone () , place_ty , bk . to_mutable_lossy ())) } Rvalue :: AddressOf (mutability , place) => { let place_ty = place . ty (locals) ? ; Ok (Ty :: new_ptr (place_ty , mutability . to_mutable_lossy ())) } Rvalue :: Len (..) => Ok (Ty :: usize_ty ()) , Rvalue :: Cast (.. , ty) => Ok (* ty) , Rvalue :: BinaryOp (op , lhs , rhs) => { let lhs_ty = lhs . ty (locals) ? ; let rhs_ty = rhs . ty (locals) ? ; Ok (op . ty (lhs_ty , rhs_ty)) } Rvalue :: CheckedBinaryOp (op , lhs , rhs) => { let lhs_ty = lhs . ty (locals) ? ; let rhs_ty = rhs . ty (locals) ? ; let ty = op . ty (lhs_ty , rhs_ty) ; Ok (Ty :: new_tuple (& [ty , Ty :: bool_ty ()])) } Rvalue :: UnaryOp (op , operand) => { let arg_ty = operand . ty (locals) ? ; Ok (op . ty (arg_ty)) } Rvalue :: Discriminant (place) => { let place_ty = place . ty (locals) ? ; place_ty . kind () . discriminant_ty () . ok_or_else (| | error ! ("Expected a `RigidTy` but found: {place_ty:?}")) } Rvalue :: NullaryOp (NullOp :: SizeOf | NullOp :: AlignOf | NullOp :: OffsetOf (..) , _) => { Ok (Ty :: usize_ty ()) } Rvalue :: NullaryOp (NullOp :: ContractChecks , _) | Rvalue :: NullaryOp (NullOp :: UbChecks , _) => Ok (Ty :: bool_ty ()) , Rvalue :: Aggregate (ak , ops) => match * ak { AggregateKind :: Array (ty) => Ty :: try_new_array (ty , ops . len () as u64) , AggregateKind :: Tuple => Ok (Ty :: new_tuple (& ops . iter () . map (| op | op . ty (locals)) . collect :: < Result < Vec < _ > , _ > > () ? ,)) , AggregateKind :: Adt (def , _ , ref args , _ , _) => Ok (def . ty_with_args (args)) , AggregateKind :: Closure (def , ref args) => Ok (Ty :: new_closure (def , args . clone ())) , AggregateKind :: Coroutine (def , ref args) => Ok (Ty :: new_coroutine (def , args . clone ())) , AggregateKind :: CoroutineClosure (def , ref args) => { Ok (Ty :: new_coroutine_closure (def , args . clone ())) } AggregateKind :: RawPtr (ty , mutability) => Ok (Ty :: new_ptr (ty , mutability)) , } , Rvalue :: ShallowInitBox (_ , ty) => Ok (Ty :: new_box (* ty)) , Rvalue :: CopyForDeref (place) => place . ty (locals) , } } }}}
mkitem!{mkenum!{#[derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum AggregateKind { Array (Ty) , Tuple , Adt (AdtDef , VariantIdx , GenericArgs , Option < UserTypeAnnotationIndex > , Option < FieldIdx >) , Closure (ClosureDef , GenericArgs) , Coroutine (CoroutineDef , GenericArgs) , CoroutineClosure (CoroutineClosureDef , GenericArgs) , RawPtr (Ty , Mutability) , }}}
mkitem!{mkenum!{#[derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum Operand { Copy (Place) , Move (Place) , Constant (ConstOperand) , }}}
mkitem!{mkstruct!{#[derive (Clone , Eq , PartialEq , Hash , Serialize)] pub struct Place { pub local : Local , #[doc = " projection out of a place (access a field, deref a pointer, etc)"] pub projection : Vec < ProjectionElem > , }}}
mkitem!{mkimpl!{impl From < Local > for Place { fn from (local : Local) -> Self { Place { local , projection : vec ! [] } } }}}
mkitem!{mkstruct!{#[derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct ConstOperand { pub span : Span , pub user_ty : Option < UserTypeAnnotationIndex > , pub const_ : MirConst , }}}
mkitem!{mkstruct!{#[doc = " Debug information pertaining to a user variable."] #[derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct VarDebugInfo { #[doc = " The variable name."] pub name : Symbol , #[doc = " Source info of the user variable, including the scope"] #[doc = " within which the variable is visible (to debuginfo)."] pub source_info : SourceInfo , #[doc = " The user variable's data is split across several fragments,"] #[doc = " each described by a `VarDebugInfoFragment`."] pub composite : Option < VarDebugInfoFragment > , #[doc = " Where the data for this user variable is to be found."] pub value : VarDebugInfoContents , #[doc = " When present, indicates what argument number this variable is in the function that it"] #[doc = " originated from (starting from 1). Note, if MIR inlining is enabled, then this is the"] #[doc = " argument number in the original function before it was inlined."] pub argument_index : Option < u16 > , }}}
mkitem!{mkimpl!{impl VarDebugInfo { #[doc = " Return a local variable if this info is related to one."] pub fn local (& self) -> Option < Local > { match & self . value { VarDebugInfoContents :: Place (place) if place . projection . is_empty () => Some (place . local) , VarDebugInfoContents :: Place (_) | VarDebugInfoContents :: Const (_) => None , } } #[doc = " Return a constant if this info is related to one."] pub fn constant (& self) -> Option < & ConstOperand > { match & self . value { VarDebugInfoContents :: Place (_) => None , VarDebugInfoContents :: Const (const_op) => Some (const_op) , } } }}}
mkitem!{pub type SourceScope = u32 ;}
mkitem!{mkstruct!{#[derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct SourceInfo { pub span : Span , pub scope : SourceScope , }}}
mkitem!{mkstruct!{#[derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct VarDebugInfoFragment { pub ty : Ty , pub projection : Vec < ProjectionElem > , }}}
mkitem!{mkenum!{#[derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum VarDebugInfoContents { Place (Place) , Const (ConstOperand) , }}}
mkitem!{mkenum!{#[derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum ProjectionElem { #[doc = " Dereference projections (e.g. `*_1`) project to the address referenced by the base place."] Deref , #[doc = " A field projection (e.g., `f` in `_1.f`) project to a field in the base place. The field is"] #[doc = " referenced by source-order index rather than the name of the field. The fields type is also"] #[doc = " given."] Field (FieldIdx , Ty) , #[doc = " Index into a slice/array. The value of the index is computed at runtime using the `V`"] #[doc = " argument."] #[doc = ""] #[doc = " Note that this does not also dereference, and so it does not exactly correspond to slice"] #[doc = " indexing in Rust. In other words, in the below Rust code:"] #[doc = ""] #[doc = " ```rust"] #[doc = " let x = &[1, 2, 3, 4];"] #[doc = " let i = 2;"] #[doc = " x[i];"] #[doc = " ```"] #[doc = ""] #[doc = " The `x[i]` is turned into a `Deref` followed by an `Index`, not just an `Index`. The same"] #[doc = " thing is true of the `ConstantIndex` and `Subslice` projections below."] Index (Local) , #[doc = " Index into a slice/array given by offsets."] #[doc = ""] #[doc = " These indices are generated by slice patterns. Easiest to explain by example:"] #[doc = ""] #[doc = " ```ignore (illustrative)"] #[doc = " [X, _, .._, _, _] => { offset: 0, min_length: 4, from_end: false },"] #[doc = " [_, X, .._, _, _] => { offset: 1, min_length: 4, from_end: false },"] #[doc = " [_, _, .._, X, _] => { offset: 2, min_length: 4, from_end: true },"] #[doc = " [_, _, .._, _, X] => { offset: 1, min_length: 4, from_end: true },"] #[doc = " ```"] ConstantIndex { #[doc = " index or -index (in Python terms), depending on from_end"] offset : u64 , #[doc = " The thing being indexed must be at least this long -- otherwise, the"] #[doc = " projection is UB."] #[doc = ""] #[doc = " For arrays this is always the exact length."] min_length : u64 , #[doc = " Counting backwards from end? This is always false when indexing an"] #[doc = " array."] from_end : bool , } , #[doc = " Projects a slice from the base place."] #[doc = ""] #[doc = " These indices are generated by slice patterns. If `from_end` is true, this represents"] #[doc = " `slice[from..slice.len() - to]`. Otherwise it represents `array[from..to]`."] Subslice { from : u64 , to : u64 , #[doc = " Whether `to` counts from the start or end of the array/slice."] from_end : bool , } , #[doc = " \"Downcast\" to a variant of an enum or a coroutine."] Downcast (VariantIdx) , #[doc = " Like an explicit cast from an opaque type to a concrete type, but without"] #[doc = " requiring an intermediate variable."] OpaqueCast (Ty) , #[doc = " A `Subtype(T)` projection is applied to any `StatementKind::Assign` where"] #[doc = " type of lvalue doesn't match the type of rvalue, the primary goal is making subtyping"] #[doc = " explicit during optimizations and codegen."] #[doc = ""] #[doc = " This projection doesn't impact the runtime behavior of the program except for potentially changing"] #[doc = " some type metadata of the interpreter or codegen backend."] Subtype (Ty) , }}}
mkitem!{mkstruct!{#[derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct UserTypeProjection { pub base : UserTypeAnnotationIndex , pub projection : Opaque , }}}
mkitem!{pub type Local = usize ;}
mkitem!{pub const RETURN_LOCAL : Local = 0 ;}
mkitem!{#[doc = " The source-order index of a field in a variant."] #[doc = ""] #[doc = " For example, in the following types,"] #[doc = " ```ignore(illustrative)"] #[doc = " enum Demo1 {"] #[doc = "    Variant0 { a: bool, b: i32 },"] #[doc = "    Variant1 { c: u8, d: u64 },"] #[doc = " }"] #[doc = " struct Demo2 { e: u8, f: u16, g: u8 }"] #[doc = " ```"] #[doc = " `a`'s `FieldIdx` is `0`,"] #[doc = " `b`'s `FieldIdx` is `1`,"] #[doc = " `c`'s `FieldIdx` is `0`, and"] #[doc = " `g`'s `FieldIdx` is `2`."] pub type FieldIdx = usize ;}
mkitem!{type UserTypeAnnotationIndex = usize ;}
mkitem!{mkstruct!{#[doc = " The possible branch sites of a [TerminatorKind::SwitchInt]."] #[derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct SwitchTargets { #[doc = " The conditional branches where the first element represents the value that guards this"] #[doc = " branch, and the second element is the branch target."] branches : Vec < (u128 , BasicBlockIdx) > , #[doc = " The `otherwise` branch which will be taken in case none of the conditional branches are"] #[doc = " satisfied."] otherwise : BasicBlockIdx , }}}
mkitem!{mkimpl!{impl SwitchTargets { #[doc = " All possible targets including the `otherwise` target."] pub fn all_targets (& self) -> Successors { self . branches . iter () . map (| (_ , target) | * target) . chain (Some (self . otherwise)) . collect () } #[doc = " The `otherwise` branch target."] pub fn otherwise (& self) -> BasicBlockIdx { self . otherwise } #[doc = " The conditional targets which are only taken if the pattern matches the given value."] pub fn branches (& self) -> impl Iterator < Item = (u128 , BasicBlockIdx) > { self . branches . iter () . copied () } #[doc = " The number of targets including `otherwise`."] pub fn len (& self) -> usize { self . branches . len () + 1 } #[doc = " Create a new SwitchTargets from the given branches and `otherwise` target."] pub fn new (branches : Vec < (u128 , BasicBlockIdx) > , otherwise : BasicBlockIdx) -> SwitchTargets { SwitchTargets { branches , otherwise } } }}}
mkitem!{mkenum!{#[derive (Copy , Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum BorrowKind { #[doc = " Data must be immutable and is aliasable."] Shared , #[doc = " An immutable, aliasable borrow that is discarded after borrow-checking. Can behave either"] #[doc = " like a normal shared borrow or like a special shallow borrow (see [`FakeBorrowKind`])."] Fake (FakeBorrowKind) , #[doc = " Data is mutable and not aliasable."] Mut { #[doc = " `true` if this borrow arose from method-call auto-ref"] kind : MutBorrowKind , } , }}}
mkitem!{mkimpl!{impl BorrowKind { pub fn to_mutable_lossy (self) -> Mutability { match self { BorrowKind :: Mut { .. } => Mutability :: Mut , BorrowKind :: Shared => Mutability :: Not , BorrowKind :: Fake (_) => Mutability :: Not , } } }}}
mkitem!{mkenum!{#[derive (Copy , Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum RawPtrKind { Mut , Const , FakeForPtrMetadata , }}}
mkitem!{mkimpl!{impl RawPtrKind { pub fn to_mutable_lossy (self) -> Mutability { match self { RawPtrKind :: Mut { .. } => Mutability :: Mut , RawPtrKind :: Const => Mutability :: Not , RawPtrKind :: FakeForPtrMetadata => Mutability :: Not , } } }}}
mkitem!{mkenum!{#[derive (Copy , Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum MutBorrowKind { Default , TwoPhaseBorrow , ClosureCapture , }}}
mkitem!{mkenum!{#[derive (Copy , Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum FakeBorrowKind { #[doc = " A shared (deep) borrow. Data must be immutable and is aliasable."] Deep , #[doc = " The immediately borrowed place must be immutable, but projections from"] #[doc = " it don't need to be. This is used to prevent match guards from replacing"] #[doc = " the scrutinee. For example, a fake borrow of `a.b` doesn't"] #[doc = " conflict with a mutable borrow of `a.b.c`."] Shallow , }}}
mkitem!{mkenum!{#[derive (Copy , Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub enum Mutability { Not , Mut , }}}
mkitem!{mkenum!{#[derive (Copy , Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum Safety { Safe , Unsafe , }}}
mkitem!{mkenum!{#[derive (Copy , Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum PointerCoercion { #[doc = " Go from a fn-item type to a fn-pointer type."] ReifyFnPointer , #[doc = " Go from a safe fn pointer to an unsafe fn pointer."] UnsafeFnPointer , #[doc = " Go from a non-capturing closure to a fn pointer or an unsafe fn pointer."] #[doc = " It cannot convert a closure that requires unsafe."] ClosureFnPointer (Safety) , #[doc = " Go from a mut raw pointer to a const raw pointer."] MutToConstPointer , #[doc = " Go from `*const [T; N]` to `*const T`"] ArrayToPointer , #[doc = " Unsize a pointer/reference value, e.g., `&[T; n]` to"] #[doc = " `&[T]`. Note that the source could be a thin or wide pointer."] #[doc = " This will do things like convert thin pointers to wide"] #[doc = " pointers, or convert structs containing thin pointers to"] #[doc = " structs containing wide pointers, or convert between wide"] #[doc = " pointers."] Unsize , }}}
mkitem!{mkenum!{#[derive (Copy , Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum CastKind { PointerExposeAddress , PointerWithExposedProvenance , PointerCoercion (PointerCoercion) , IntToInt , FloatToInt , FloatToFloat , IntToFloat , PtrToPtr , FnPtrToPtr , Transmute , }}}
mkitem!{mkenum!{#[derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum NullOp { #[doc = " Returns the size of a value of that type."] SizeOf , #[doc = " Returns the minimum alignment of a type."] AlignOf , #[doc = " Returns the offset of a field."] OffsetOf (Vec < (VariantIdx , FieldIdx) >) , #[doc = " cfg!(ub_checks), but at codegen time"] UbChecks , #[doc = " cfg!(contract_checks), but at codegen time"] ContractChecks , }}}
mkitem!{mkimpl!{impl Operand { #[doc = " Get the type of an operand relative to the local declaration."] #[doc = ""] #[doc = " In order to retrieve the correct type, the `locals` argument must match the list of all"] #[doc = " locals from the function body where this operand originates from."] #[doc = ""] #[doc = " Errors indicate a malformed operand or incompatible locals list."] pub fn ty (& self , locals : & [LocalDecl]) -> Result < Ty , Error > { match self { Operand :: Copy (place) | Operand :: Move (place) => place . ty (locals) , Operand :: Constant (c) => Ok (c . ty ()) , } } }}}
mkitem!{mkimpl!{impl ConstOperand { pub fn ty (& self) -> Ty { self . const_ . ty () } }}}
mkitem!{mkimpl!{impl Place { #[doc = " Resolve down the chain of projections to get the type referenced at the end of it."] #[doc = " E.g.:"] #[doc = " Calling `ty()` on `var.field` should return the type of `field`."] #[doc = ""] #[doc = " In order to retrieve the correct type, the `locals` argument must match the list of all"] #[doc = " locals from the function body where this place originates from."] pub fn ty (& self , locals : & [LocalDecl]) -> Result < Ty , Error > { self . projection . iter () . try_fold (locals [self . local] . ty , | place_ty , elem | elem . ty (place_ty)) } }}}
mkitem!{mkimpl!{impl ProjectionElem { #[doc = " Get the expected type after applying this projection to a given place type."] pub fn ty (& self , place_ty : Ty) -> Result < Ty , Error > { let ty = place_ty ; match & self { ProjectionElem :: Deref => Self :: deref_ty (ty) , ProjectionElem :: Field (_idx , fty) => Ok (* fty) , ProjectionElem :: Index (_) | ProjectionElem :: ConstantIndex { .. } => Self :: index_ty (ty) , ProjectionElem :: Subslice { from , to , from_end } => { Self :: subslice_ty (ty , * from , * to , * from_end) } ProjectionElem :: Downcast (_) => Ok (ty) , ProjectionElem :: OpaqueCast (ty) | ProjectionElem :: Subtype (ty) => Ok (* ty) , } } fn index_ty (ty : Ty) -> Result < Ty , Error > { ty . kind () . builtin_index () . ok_or_else (| | error ! ("Cannot index non-array type: {ty:?}")) } fn subslice_ty (ty : Ty , from : u64 , to : u64 , from_end : bool) -> Result < Ty , Error > { let ty_kind = ty . kind () ; match ty_kind { TyKind :: RigidTy (RigidTy :: Slice (..)) => Ok (ty) , TyKind :: RigidTy (RigidTy :: Array (inner , _)) if ! from_end => Ty :: try_new_array (inner , to . checked_sub (from) . ok_or_else (| | error ! ("Subslice overflow: {from}..{to}")) ? ,) , TyKind :: RigidTy (RigidTy :: Array (inner , size)) => { let size = size . eval_target_usize () ? ; let len = size - from - to ; Ty :: try_new_array (inner , len) } _ => Err (Error (format ! ("Cannot subslice non-array type: `{ty_kind:?}`"))) , } } fn deref_ty (ty : Ty) -> Result < Ty , Error > { let deref_ty = ty . kind () . builtin_deref (true) . ok_or_else (| | error ! ("Cannot dereference type: {ty:?}")) ? ; Ok (deref_ty . ty) } }}}