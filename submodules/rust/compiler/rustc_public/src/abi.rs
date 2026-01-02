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
mkuse!{use std :: fmt :: { self , Debug } ;}
mkuse!{use std :: num :: NonZero ;}
mkuse!{use std :: ops :: RangeInclusive ;}
mkuse!{use serde :: Serialize ;}
mkuse!{use crate :: compiler_interface :: with ;}
mkuse!{use crate :: mir :: FieldIdx ;}
mkuse!{use crate :: target :: { MachineInfo , MachineSize as Size } ;}
mkuse!{use crate :: ty :: { Align , Ty , VariantIdx } ;}
mkuse!{use crate :: { Error , Opaque , error } ;}
mkitem!{mkstruct!{# [doc = " A function ABI definition."] # [derive (Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub struct FnAbi { # [doc = " The types of each argument."] pub args : Vec < ArgAbi > , # [doc = " The expected return type."] pub ret : ArgAbi , # [doc = " The count of non-variadic arguments."] # [doc = ""] # [doc = " Should only be different from `args.len()` when a function is a C variadic function."] pub fixed_count : u32 , # [doc = " The ABI convention."] pub conv : CallConvention , # [doc = " Whether this is a variadic C function,"] pub c_variadic : bool , }}}
mkitem!{mkstruct!{# [doc = " Information about the ABI of a function's argument, or return value."] # [derive (Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub struct ArgAbi { pub ty : Ty , pub layout : Layout , pub mode : PassMode , }}}
mkitem!{mkenum!{# [doc = " How a function argument should be passed in to the target function."] # [derive (Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub enum PassMode { # [doc = " Ignore the argument."] # [doc = ""] # [doc = " The argument is either uninhabited or a ZST."] Ignore , # [doc = " Pass the argument directly."] # [doc = ""] # [doc = " The argument has a layout abi of `Scalar` or `Vector`."] Direct (Opaque) , # [doc = " Pass a pair's elements directly in two arguments."] # [doc = ""] # [doc = " The argument has a layout abi of `ScalarPair`."] Pair (Opaque , Opaque) , # [doc = " Pass the argument after casting it."] Cast { pad_i32 : bool , cast : Opaque } , # [doc = " Pass the argument indirectly via a hidden pointer."] Indirect { attrs : Opaque , meta_attrs : Opaque , on_stack : bool } , }}}
mkitem!{mkstruct!{# [doc = " The layout of a type, alongside the type itself."] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub struct TyAndLayout { pub ty : Ty , pub layout : Layout , }}}
mkitem!{mkstruct!{# [doc = " The layout of a type in memory."] # [derive (Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub struct LayoutShape { # [doc = " The fields location within the layout"] pub fields : FieldsShape , # [doc = " Encodes information about multi-variant layouts."] # [doc = " Even with `Multiple` variants, a layout still has its own fields! Those are then"] # [doc = " shared between all variants."] # [doc = ""] # [doc = " To access all fields of this layout, both `fields` and the fields of the active variant"] # [doc = " must be taken into account."] pub variants : VariantsShape , # [doc = " The `abi` defines how this data is passed between functions."] pub abi : ValueAbi , # [doc = " The ABI mandated alignment in bytes."] pub abi_align : Align , # [doc = " The size of this layout in bytes."] pub size : Size , }}}
mkitem!{mkimpl!{impl LayoutShape { # [doc = " Returns `true` if the layout corresponds to an unsized type."] # [inline] pub fn is_unsized (& self) -> bool { self . abi . is_unsized () } # [inline] pub fn is_sized (& self) -> bool { ! self . abi . is_unsized () } # [doc = " Returns `true` if the type is sized and a 1-ZST (meaning it has size 0 and alignment 1)."] pub fn is_1zst (& self) -> bool { self . is_sized () && self . size . bits () == 0 && self . abi_align == 1 } }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub struct Layout (usize) ;}}
mkitem!{mkimpl!{impl Layout { pub fn shape (self) -> LayoutShape { with (| cx | cx . layout_shape (self)) } }}}
mkitem!{mkimpl!{impl crate :: IndexedVal for Layout { fn to_val (index : usize) -> Self { Layout (index) } fn to_index (& self) -> usize { self . 0 } }}}
mkitem!{mkenum!{# [doc = " Describes how the fields of a type are shaped in memory."] # [derive (Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub enum FieldsShape { # [doc = " Scalar primitives and `!`, which never have fields."] Primitive , # [doc = " All fields start at no offset. The `usize` is the field count."] Union (NonZero < usize >) , # [doc = " Array/vector-like placement, with all fields of identical types."] Array { stride : Size , count : u64 } , # [doc = " Struct-like placement, with precomputed offsets."] # [doc = ""] # [doc = " Fields are guaranteed to not overlap, but note that gaps"] # [doc = " before, between and after all the fields are NOT always"] # [doc = " padding, and as such their contents may not be discarded."] # [doc = " For example, enum variants leave a gap at the start,"] # [doc = " where the discriminant field in the enum layout goes."] Arbitrary { # [doc = " Offsets for the first byte of each field,"] # [doc = " ordered to match the source definition order."] # [doc = " I.e.: It follows the same order as [super::ty::VariantDef::fields()]."] # [doc = " This vector does not go in increasing order."] offsets : Vec < Size > , } , }}}
mkitem!{mkimpl!{impl FieldsShape { pub fn fields_by_offset_order (& self) -> Vec < FieldIdx > { match self { FieldsShape :: Primitive => vec ! [] , FieldsShape :: Union (_) | FieldsShape :: Array { .. } => (0 .. self . count ()) . collect () , FieldsShape :: Arbitrary { offsets , .. } => { let mut indices = (0 .. offsets . len ()) . collect :: < Vec < _ > > () ; indices . sort_by_key (| idx | offsets [* idx]) ; indices } } } pub fn count (& self) -> usize { match self { FieldsShape :: Primitive => 0 , FieldsShape :: Union (count) => count . get () , FieldsShape :: Array { count , .. } => * count as usize , FieldsShape :: Arbitrary { offsets , .. } => offsets . len () , } } }}}
mkitem!{mkenum!{# [derive (Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub enum VariantsShape { # [doc = " A type with no valid variants. Must be uninhabited."] Empty , # [doc = " Single enum variants, structs/tuples, unions, and all non-ADTs."] Single { index : VariantIdx } , # [doc = " Enum-likes with more than one inhabited variant: each variant comes with"] # [doc = " a *discriminant* (usually the same as the variant index but the user can"] # [doc = " assign explicit discriminant values). That discriminant is encoded"] # [doc = " as a *tag* on the machine. The layout of each variant is"] # [doc = " a struct, and they all have space reserved for the tag."] # [doc = " For enums, the tag is the sole field of the layout."] Multiple { tag : Scalar , tag_encoding : TagEncoding , tag_field : usize , variants : Vec < LayoutShape > , } , }}}
mkitem!{mkenum!{# [derive (Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub enum TagEncoding { # [doc = " The tag directly stores the discriminant, but possibly with a smaller layout"] # [doc = " (so converting the tag to the discriminant can require sign extension)."] Direct , # [doc = " Niche (values invalid for a type) encoding the discriminant:"] # [doc = " Discriminant and variant index coincide."] # [doc = " The variant `untagged_variant` contains a niche at an arbitrary"] # [doc = " offset (field `tag_field` of the enum), which for a variant with"] # [doc = " discriminant `d` is set to"] # [doc = " `(d - niche_variants.start).wrapping_add(niche_start)`."] # [doc = ""] # [doc = " For example, `Option<(usize, &T)>`  is represented such that"] # [doc = " `None` has a null pointer for the second tuple field, and"] # [doc = " `Some` is the identity function (with a non-null reference)."] Niche { untagged_variant : VariantIdx , niche_variants : RangeInclusive < VariantIdx > , niche_start : u128 , } , }}}
mkitem!{mkenum!{# [doc = " Describes how values of the type are passed by target ABIs,"] # [doc = " in terms of categories of C types there are ABI rules for."] # [derive (Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub enum ValueAbi { Scalar (Scalar) , ScalarPair (Scalar , Scalar) , Vector { element : Scalar , count : u64 , } , Aggregate { # [doc = " If true, the size is exact, otherwise it's only a lower bound."] sized : bool , } , }}}
mkitem!{mkimpl!{impl ValueAbi { # [doc = " Returns `true` if the layout corresponds to an unsized type."] pub fn is_unsized (& self) -> bool { match * self { ValueAbi :: Scalar (_) | ValueAbi :: ScalarPair (..) | ValueAbi :: Vector { .. } => false , ValueAbi :: Aggregate { sized } => ! sized , } } }}}
mkitem!{mkenum!{# [doc = " Information about one scalar component of a Rust type."] # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug , Serialize)] pub enum Scalar { Initialized { # [doc = " The primitive type used to represent this value."] value : Primitive , # [doc = " The range that represents valid values."] # [doc = " The range must be valid for the `primitive` size."] valid_range : WrappingRange , } , Union { # [doc = " Unions never have niches, so there is no `valid_range`."] # [doc = " Even for unions, we need to use the correct registers for the kind of"] # [doc = " values inside the union, so we keep the `Primitive` type around."] # [doc = " It is also used to compute the size of the scalar."] value : Primitive , } , }}}
mkitem!{mkimpl!{impl Scalar { pub fn has_niche (& self , target : & MachineInfo) -> bool { match self { Scalar :: Initialized { value , valid_range } => { ! valid_range . is_full (value . size (target)) . unwrap () } Scalar :: Union { .. } => false , } } }}}
mkitem!{mkenum!{# [doc = " Fundamental unit of memory access and layout."] # [derive (Copy , Clone , PartialEq , Eq , Hash , Debug , Serialize)] pub enum Primitive { # [doc = " The `bool` is the signedness of the `Integer` type."] # [doc = ""] # [doc = " One would think we would not care about such details this low down,"] # [doc = " but some ABIs are described in terms of C types and ISAs where the"] # [doc = " integer arithmetic is done on {sign,zero}-extended registers, e.g."] # [doc = " a negative integer passed by zero-extension will appear positive in"] # [doc = " the callee, and most operations on it will produce the wrong values."] Int { length : IntegerLength , signed : bool , } , Float { length : FloatLength , } , Pointer (AddressSpace) , }}}
mkitem!{mkimpl!{impl Primitive { pub fn size (self , target : & MachineInfo) -> Size { match self { Primitive :: Int { length , .. } => Size :: from_bits (length . bits ()) , Primitive :: Float { length } => Size :: from_bits (length . bits ()) , Primitive :: Pointer (_) => target . pointer_width , } } }}}
mkitem!{mkenum!{# [doc = " Enum representing the existing integer lengths."] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Serialize)] pub enum IntegerLength { I8 , I16 , I32 , I64 , I128 , }}}
mkitem!{mkenum!{# [doc = " Enum representing the existing float lengths."] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Serialize)] pub enum FloatLength { F16 , F32 , F64 , F128 , }}}
mkitem!{mkimpl!{impl IntegerLength { pub fn bits (self) -> usize { match self { IntegerLength :: I8 => 8 , IntegerLength :: I16 => 16 , IntegerLength :: I32 => 32 , IntegerLength :: I64 => 64 , IntegerLength :: I128 => 128 , } } }}}
mkitem!{mkimpl!{impl FloatLength { pub fn bits (self) -> usize { match self { FloatLength :: F16 => 16 , FloatLength :: F32 => 32 , FloatLength :: F64 => 64 , FloatLength :: F128 => 128 , } } }}}
mkitem!{mkstruct!{# [doc = " An identifier that specifies the address space that some operation"] # [doc = " should operate on. Special address spaces have an effect on code generation,"] # [doc = " depending on the target and the address spaces it implements."] # [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Serialize)] pub struct AddressSpace (pub u32) ;}}
mkitem!{mkimpl!{impl AddressSpace { # [doc = " The default address space, corresponding to data space."] pub const DATA : Self = AddressSpace (0) ; }}}
mkitem!{mkstruct!{# [doc = " Inclusive wrap-around range of valid values (bitwise representation), that is, if"] # [doc = " start > end, it represents `start..=MAX`, followed by `0..=end`."] # [doc = ""] # [doc = " That is, for an i8 primitive, a range of `254..=2` means following"] # [doc = " sequence:"] # [doc = ""] # [doc = "    254 (-2), 255 (-1), 0, 1, 2"] # [derive (Clone , Copy , PartialEq , Eq , Hash , Serialize)] pub struct WrappingRange { pub start : u128 , pub end : u128 , }}}
mkitem!{mkimpl!{impl WrappingRange { # [doc = " Returns `true` if `size` completely fills the range."] # [inline] pub fn is_full (& self , size : Size) -> Result < bool , Error > { let Some (max_value) = size . unsigned_int_max () else { return Err (error ! ("Expected size <= 128 bits, but found {} instead" , size . bits ())) ; } ; if self . start <= max_value && self . end <= max_value { Ok (self . start == (self . end . wrapping_add (1) & max_value)) } else { Err (error ! ("Range `{self:?}` out of bounds for size `{}` bits." , size . bits ())) } } # [doc = " Returns `true` if `v` is contained in the range."] # [inline (always)] pub fn contains (& self , v : u128) -> bool { if self . wraps_around () { self . start <= v || v <= self . end } else { self . start <= v && v <= self . end } } # [doc = " Returns `true` if the range wraps around."] # [doc = " I.e., the range represents the union of `self.start..=MAX` and `0..=self.end`."] # [doc = " Returns `false` if this is a non-wrapping range, i.e.: `self.start..=self.end`."] # [inline] pub fn wraps_around (& self) -> bool { self . start > self . end } }}}
mkitem!{mkimpl!{impl Debug for WrappingRange { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . start > self . end { write ! (fmt , "(..={}) | ({}..)" , self . end , self . start) ? ; } else { write ! (fmt , "{}..={}" , self . start , self . end) ? ; } Ok (()) } }}}
mkitem!{mkenum!{# [doc = " General language calling conventions."] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub enum CallConvention { C , Rust , Cold , PreserveMost , PreserveAll , Custom , ArmAapcs , CCmseNonSecureCall , CCmseNonSecureEntry , Msp430Intr , PtxKernel , GpuKernel , X86Fastcall , X86Intr , X86Stdcall , X86ThisCall , X86VectorCall , X86_64SysV , X86_64Win64 , AvrInterrupt , AvrNonBlockingInterrupt , RiscvInterrupt , }}}
mkitem!{mkstruct!{# [non_exhaustive] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Serialize)] pub struct ReprFlags { pub is_simd : bool , pub is_c : bool , pub is_transparent : bool , pub is_linear : bool , }}}
mkitem!{mkenum!{# [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Serialize)] pub enum IntegerType { # [doc = " Pointer-sized integer type, i.e. `isize` and `usize`."] Pointer { # [doc = " Signedness. e.g. `true` for `isize`"] is_signed : bool , } , # [doc = " Fixed-sized integer type, e.g. `i8`, `u32`, `i128`."] Fixed { # [doc = " Length of this integer type. e.g. `IntegerLength::I8` for `u8`."] length : IntegerLength , # [doc = " Signedness. e.g. `false` for `u8`"] is_signed : bool , } , }}}
mkitem!{mkstruct!{# [doc = " Representation options provided by the user"] # [non_exhaustive] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Serialize)] pub struct ReprOptions { pub int : Option < IntegerType > , pub align : Option < Align > , pub pack : Option < Align > , pub flags : ReprFlags , }}}