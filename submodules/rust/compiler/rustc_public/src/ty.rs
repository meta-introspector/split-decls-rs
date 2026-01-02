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
mkuse!{use std :: fmt :: { self , Debug , Display , Formatter } ;}
mkuse!{use std :: ops :: Range ;}
mkuse!{use serde :: Serialize ;}
mkuse!{use super :: abi :: ReprOptions ;}
mkuse!{use super :: mir :: { Body , Mutability , Safety } ;}
mkuse!{use super :: { DefId , Error , Symbol , with } ;}
mkuse!{use crate :: abi :: { FnAbi , Layout } ;}
mkuse!{use crate :: crate_def :: { CrateDef , CrateDefItems , CrateDefType } ;}
mkuse!{use crate :: mir :: alloc :: { AllocId , read_target_int , read_target_uint } ;}
mkuse!{use crate :: mir :: mono :: StaticDef ;}
mkuse!{use crate :: target :: MachineInfo ;}
mkuse!{use crate :: { Filename , IndexedVal , Opaque } ;}
mkitem!{mkstruct!{# [derive (Copy , Clone , Eq , PartialEq , Hash , Serialize)] pub struct Ty (usize) ;}}
mkitem!{mkimpl!{impl Debug for Ty { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Ty") . field ("id" , & self . 0) . field ("kind" , & self . kind ()) . finish () } }}}
mkitem!{mkimpl!{# [doc = " Constructors for `Ty`."] impl Ty { # [doc = " Create a new type from a given kind."] pub fn from_rigid_kind (kind : RigidTy) -> Ty { with (| cx | cx . new_rigid_ty (kind)) } # [doc = " Create a new array type."] pub fn try_new_array (elem_ty : Ty , size : u64) -> Result < Ty , Error > { Ok (Ty :: from_rigid_kind (RigidTy :: Array (elem_ty , TyConst :: try_from_target_usize (size) ?))) } # [doc = " Create a new array type from Const length."] pub fn new_array_with_const_len (elem_ty : Ty , len : TyConst) -> Ty { Ty :: from_rigid_kind (RigidTy :: Array (elem_ty , len)) } # [doc = " Create a new pointer type."] pub fn new_ptr (pointee_ty : Ty , mutability : Mutability) -> Ty { Ty :: from_rigid_kind (RigidTy :: RawPtr (pointee_ty , mutability)) } # [doc = " Create a new reference type."] pub fn new_ref (reg : Region , pointee_ty : Ty , mutability : Mutability) -> Ty { Ty :: from_rigid_kind (RigidTy :: Ref (reg , pointee_ty , mutability)) } # [doc = " Create a new pointer type."] pub fn new_tuple (tys : & [Ty]) -> Ty { Ty :: from_rigid_kind (RigidTy :: Tuple (Vec :: from (tys))) } # [doc = " Create a new closure type."] pub fn new_closure (def : ClosureDef , args : GenericArgs) -> Ty { Ty :: from_rigid_kind (RigidTy :: Closure (def , args)) } # [doc = " Create a new coroutine type."] pub fn new_coroutine (def : CoroutineDef , args : GenericArgs) -> Ty { Ty :: from_rigid_kind (RigidTy :: Coroutine (def , args)) } # [doc = " Create a new closure type."] pub fn new_coroutine_closure (def : CoroutineClosureDef , args : GenericArgs) -> Ty { Ty :: from_rigid_kind (RigidTy :: CoroutineClosure (def , args)) } # [doc = " Create a new box type that represents `Box<T>`, for the given inner type `T`."] pub fn new_box (inner_ty : Ty) -> Ty { with (| cx | cx . new_box_ty (inner_ty)) } # [doc = " Create a type representing `usize`."] pub fn usize_ty () -> Ty { Ty :: from_rigid_kind (RigidTy :: Uint (UintTy :: Usize)) } # [doc = " Create a type representing `bool`."] pub fn bool_ty () -> Ty { Ty :: from_rigid_kind (RigidTy :: Bool) } # [doc = " Create a type representing a signed integer."] pub fn signed_ty (inner : IntTy) -> Ty { Ty :: from_rigid_kind (RigidTy :: Int (inner)) } # [doc = " Create a type representing an unsigned integer."] pub fn unsigned_ty (inner : UintTy) -> Ty { Ty :: from_rigid_kind (RigidTy :: Uint (inner)) } # [doc = " Get a type layout."] pub fn layout (self) -> Result < Layout , Error > { with (| cx | cx . ty_layout (self)) } }}}
mkitem!{mkimpl!{impl Ty { pub fn kind (& self) -> TyKind { with (| context | context . ty_kind (* self)) } }}}
mkitem!{mkenum!{# [doc = " Represents a pattern in the type system"] # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum Pattern { Range { start : Option < TyConst > , end : Option < TyConst > , include_end : bool } , }}}
mkitem!{mkstruct!{# [doc = " Represents a constant in the type system"] # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct TyConst { pub (crate) kind : TyConstKind , pub id : TyConstId , }}}
mkitem!{mkimpl!{impl TyConst { pub fn new (kind : TyConstKind , id : TyConstId) -> TyConst { Self { kind , id } } # [doc = " Retrieve the constant kind."] pub fn kind (& self) -> & TyConstKind { & self . kind } # [doc = " Creates an interned usize constant."] pub fn try_from_target_usize (val : u64) -> Result < Self , Error > { with (| cx | cx . try_new_ty_const_uint (val . into () , UintTy :: Usize)) } # [doc = " Try to evaluate to a target `usize`."] pub fn eval_target_usize (& self) -> Result < u64 , Error > { with (| cx | cx . eval_target_usize_ty (self)) } }}}
mkitem!{mkenum!{# [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum TyConstKind { Param (ParamConst) , Bound (DebruijnIndex , BoundVar) , Unevaluated (ConstDef , GenericArgs) , Value (Ty , Allocation) , ZSTValue (Ty) , }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct TyConstId (usize) ;}}
mkitem!{mkstruct!{# [doc = " Represents a constant in MIR"] # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct MirConst { # [doc = " The constant kind."] pub (crate) kind : ConstantKind , # [doc = " The constant type."] pub (crate) ty : Ty , # [doc = " Used for internal tracking of the internal constant."] pub id : MirConstId , }}}
mkitem!{mkimpl!{impl MirConst { # [doc = " Build a constant. Note that this should only be used by the compiler."] pub fn new (kind : ConstantKind , ty : Ty , id : MirConstId) -> MirConst { MirConst { kind , ty , id } } # [doc = " Retrieve the constant kind."] pub fn kind (& self) -> & ConstantKind { & self . kind } # [doc = " Get the constant type."] pub fn ty (& self) -> Ty { self . ty } # [doc = " Try to evaluate to a target `usize`."] pub fn eval_target_usize (& self) -> Result < u64 , Error > { with (| cx | cx . eval_target_usize (self)) } # [doc = " Create a constant that represents a new zero-sized constant of type T."] # [doc = " Fails if the type is not a ZST or if it doesn't have a known size."] pub fn try_new_zero_sized (ty : Ty) -> Result < MirConst , Error > { with (| cx | cx . try_new_const_zst (ty)) } # [doc = " Build a new constant that represents the given string."] # [doc = ""] # [doc = " Note that there is no guarantee today about duplication of the same constant."] # [doc = " I.e.: Calling this function multiple times with the same argument may or may not return"] # [doc = " the same allocation."] pub fn from_str (value : & str) -> MirConst { with (| cx | cx . new_const_str (value)) } # [doc = " Build a new constant that represents the given boolean value."] pub fn from_bool (value : bool) -> MirConst { with (| cx | cx . new_const_bool (value)) } # [doc = " Build a new constant that represents the given unsigned integer."] pub fn try_from_uint (value : u128 , uint_ty : UintTy) -> Result < MirConst , Error > { with (| cx | cx . try_new_const_uint (value , uint_ty)) } }}}
mkitem!{mkstruct!{# [derive (Clone , Copy , Debug , PartialEq , Eq , Hash , Serialize)] pub struct MirConstId (usize) ;}}
mkitem!{type Ident = Opaque ;}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct Region { pub kind : RegionKind , }}}
mkitem!{mkenum!{# [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum RegionKind { ReEarlyParam (EarlyParamRegion) , ReBound (DebruijnIndex , BoundRegion) , ReStatic , RePlaceholder (Placeholder < BoundRegion >) , ReErased , }}}
mkitem!{pub (crate) type DebruijnIndex = u32 ;}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct EarlyParamRegion { pub index : u32 , pub name : Symbol , }}}
mkitem!{pub (crate) type BoundVar = u32 ;}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct BoundRegion { pub var : BoundVar , pub kind : BoundRegionKind , }}}
mkitem!{pub (crate) type UniverseIndex = u32 ;}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct Placeholder < T > { pub universe : UniverseIndex , pub bound : T , }}}
mkitem!{mkstruct!{# [derive (Clone , Copy , PartialEq , Eq , Hash , Serialize)] pub struct Span (usize) ;}}
mkitem!{mkimpl!{impl Debug for Span { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Span") . field ("id" , & self . 0) . field ("repr" , & with (| cx | cx . span_to_string (* self))) . finish () } }}}
mkitem!{mkimpl!{impl Span { # [doc = " Return filename for diagnostic purposes"] pub fn get_filename (& self) -> Filename { with (| c | c . get_filename (self)) } # [doc = " Return lines that correspond to this `Span`"] pub fn get_lines (& self) -> LineInfo { with (| c | c . get_lines (self)) } # [doc = " Return the span location to be printed in diagnostic messages."] # [doc = ""] # [doc = " This may leak local file paths and should not be used to build artifacts that may be"] # [doc = " distributed."] pub fn diagnostic (& self) -> String { with (| c | c . span_to_string (* self)) } }}}
mkitem!{mkstruct!{# [derive (Clone , Copy , Debug , Serialize)] # [doc = " Information you get from `Span` in a struct form."] # [doc = " Line and col start from 1."] pub struct LineInfo { pub start_line : usize , pub start_col : usize , pub end_line : usize , pub end_col : usize , }}}
mkitem!{mkimpl!{impl LineInfo { pub fn from (lines : (usize , usize , usize , usize)) -> Self { LineInfo { start_line : lines . 0 , start_col : lines . 1 , end_line : lines . 2 , end_col : lines . 3 } } }}}
mkitem!{mkenum!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum TyKind { RigidTy (RigidTy) , Alias (AliasKind , AliasTy) , Param (ParamTy) , Bound (usize , BoundTy) , }}}
mkitem!{mkimpl!{impl TyKind { pub fn rigid (& self) -> Option < & RigidTy > { if let TyKind :: RigidTy (inner) = self { Some (inner) } else { None } } # [inline] pub fn is_unit (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Tuple (data)) if data . is_empty ()) } # [inline] pub fn is_bool (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Bool)) } # [inline] pub fn is_char (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Char)) } # [inline] pub fn is_trait (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Dynamic (_ , _ , DynKind :: Dyn))) } # [inline] pub fn is_enum (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Adt (def , _)) if def . kind () == AdtKind :: Enum) } # [inline] pub fn is_struct (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Adt (def , _)) if def . kind () == AdtKind :: Struct) } # [inline] pub fn is_union (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Adt (def , _)) if def . kind () == AdtKind :: Union) } # [inline] pub fn is_adt (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Adt (..))) } # [inline] pub fn is_ref (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Ref (..))) } # [inline] pub fn is_fn (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: FnDef (..))) } # [inline] pub fn is_fn_ptr (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: FnPtr (..))) } # [inline] pub fn is_primitive (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Bool | RigidTy :: Char | RigidTy :: Int (_) | RigidTy :: Uint (_) | RigidTy :: Float (_))) } # [inline] pub fn is_float (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Float (_))) } # [inline] pub fn is_integral (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Int (_) | RigidTy :: Uint (_))) } # [inline] pub fn is_numeric (& self) -> bool { self . is_integral () || self . is_float () } # [inline] pub fn is_signed (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Int (_))) } # [inline] pub fn is_str (& self) -> bool { * self == TyKind :: RigidTy (RigidTy :: Str) } # [inline] pub fn is_cstr (& self) -> bool { let TyKind :: RigidTy (RigidTy :: Adt (def , _)) = self else { return false ; } ; with (| cx | cx . adt_is_cstr (* def)) } # [inline] pub fn is_slice (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Slice (_))) } # [inline] pub fn is_array (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Array (..))) } # [inline] pub fn is_mutable_ptr (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: RawPtr (_ , Mutability :: Mut)) | TyKind :: RigidTy (RigidTy :: Ref (_ , _ , Mutability :: Mut))) } # [inline] pub fn is_raw_ptr (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: RawPtr (..))) } # [doc = " Tests if this is any kind of primitive pointer type (reference, raw pointer, fn pointer)."] # [inline] pub fn is_any_ptr (& self) -> bool { self . is_ref () || self . is_raw_ptr () || self . is_fn_ptr () } # [inline] pub fn is_coroutine (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Coroutine (..))) } # [inline] pub fn is_closure (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Closure (..))) } # [inline] pub fn is_box (& self) -> bool { match self { TyKind :: RigidTy (RigidTy :: Adt (def , _)) => def . is_box () , _ => false , } } # [inline] pub fn is_simd (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Adt (def , _)) if def . is_simd ()) } pub fn trait_principal (& self) -> Option < Binder < ExistentialTraitRef > > { if let TyKind :: RigidTy (RigidTy :: Dynamic (predicates , _ , _)) = self { if let Some (Binder { value : ExistentialPredicate :: Trait (trait_ref) , bound_vars }) = predicates . first () { Some (Binder { value : trait_ref . clone () , bound_vars : bound_vars . clone () }) } else { None } } else { None } } # [doc = " Returns the type of `ty[i]` for builtin types."] pub fn builtin_index (& self) -> Option < Ty > { match self . rigid () ? { RigidTy :: Array (ty , _) | RigidTy :: Slice (ty) => Some (* ty) , _ => None , } } # [doc = " Returns the type and mutability of `*ty` for builtin types."] # [doc = ""] # [doc = " The parameter `explicit` indicates if this is an *explicit* dereference."] # [doc = " Some types -- notably raw ptrs -- can only be dereferenced explicitly."] pub fn builtin_deref (& self , explicit : bool) -> Option < TypeAndMut > { match self . rigid () ? { RigidTy :: Adt (def , args) if def . is_box () => { Some (TypeAndMut { ty : * args . 0 . first () ? . ty () ? , mutability : Mutability :: Not }) } RigidTy :: Ref (_ , ty , mutability) => { Some (TypeAndMut { ty : * ty , mutability : * mutability }) } RigidTy :: RawPtr (ty , mutability) if explicit => { Some (TypeAndMut { ty : * ty , mutability : * mutability }) } _ => None , } } # [doc = " Get the function signature for function like types (Fn, FnPtr, and Closure)"] pub fn fn_sig (& self) -> Option < PolyFnSig > { match self { TyKind :: RigidTy (RigidTy :: FnDef (def , args)) => Some (with (| cx | cx . fn_sig (* def , args))) , TyKind :: RigidTy (RigidTy :: FnPtr (sig)) => Some (sig . clone ()) , TyKind :: RigidTy (RigidTy :: Closure (_def , args)) => Some (with (| cx | cx . closure_sig (args))) , _ => None , } } # [doc = " Get the discriminant type for this type."] pub fn discriminant_ty (& self) -> Option < Ty > { self . rigid () . map (| ty | with (| cx | cx . rigid_ty_discriminant_ty (ty))) } # [doc = " Deconstruct a function type if this is one."] pub fn fn_def (& self) -> Option < (FnDef , & GenericArgs) > { if let TyKind :: RigidTy (RigidTy :: FnDef (def , args)) = self { Some ((* def , args)) } else { None } } }}}
mkitem!{mkstruct!{pub struct TypeAndMut { pub ty : Ty , pub mutability : Mutability , }}}
mkitem!{mkenum!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum RigidTy { Bool , Char , Int (IntTy) , Uint (UintTy) , Float (FloatTy) , Adt (AdtDef , GenericArgs) , Foreign (ForeignDef) , Str , Array (Ty , TyConst) , Pat (Ty , Pattern) , Slice (Ty) , RawPtr (Ty , Mutability) , Ref (Region , Ty , Mutability) , FnDef (FnDef , GenericArgs) , FnPtr (PolyFnSig) , Closure (ClosureDef , GenericArgs) , Coroutine (CoroutineDef , GenericArgs) , CoroutineClosure (CoroutineClosureDef , GenericArgs) , Dynamic (Vec < Binder < ExistentialPredicate > > , Region , DynKind) , Never , Tuple (Vec < Ty >) , CoroutineWitness (CoroutineWitnessDef , GenericArgs) , }}}
mkitem!{mkimpl!{impl RigidTy { # [doc = " Get the discriminant type for this type."] pub fn discriminant_ty (& self) -> Ty { with (| cx | cx . rigid_ty_discriminant_ty (self)) } }}}
mkitem!{mkimpl!{impl From < RigidTy > for TyKind { fn from (value : RigidTy) -> Self { TyKind :: RigidTy (value) } }}}
mkitem!{mkenum!{# [derive (Clone , Copy , Debug , PartialEq , Eq , Serialize)] pub enum IntTy { Isize , I8 , I16 , I32 , I64 , I128 , }}}
mkitem!{mkimpl!{impl IntTy { pub fn num_bytes (self) -> usize { match self { IntTy :: Isize => MachineInfo :: target_pointer_width () . bytes () , IntTy :: I8 => 1 , IntTy :: I16 => 2 , IntTy :: I32 => 4 , IntTy :: I64 => 8 , IntTy :: I128 => 16 , } } }}}
mkitem!{mkenum!{# [derive (Clone , Copy , Debug , PartialEq , Eq , Serialize)] pub enum UintTy { Usize , U8 , U16 , U32 , U64 , U128 , }}}
mkitem!{mkimpl!{impl UintTy { pub fn num_bytes (self) -> usize { match self { UintTy :: Usize => MachineInfo :: target_pointer_width () . bytes () , UintTy :: U8 => 1 , UintTy :: U16 => 2 , UintTy :: U32 => 4 , UintTy :: U64 => 8 , UintTy :: U128 => 16 , } } }}}
mkitem!{mkenum!{# [derive (Clone , Copy , Debug , PartialEq , Eq , Serialize)] pub enum FloatTy { F16 , F32 , F64 , F128 , }}}
mkitem!{mkenum!{# [derive (Clone , Copy , Debug , PartialEq , Eq , Serialize)] pub enum Movability { Static , Movable , }}}
mkitem!{crate_def ! { # [derive (Serialize)] pub ForeignModuleDef ; }}
mkitem!{mkimpl!{impl ForeignModuleDef { pub fn module (& self) -> ForeignModule { with (| cx | cx . foreign_module (* self)) } }}}
mkitem!{mkstruct!{pub struct ForeignModule { pub def_id : ForeignModuleDef , pub abi : Abi , }}}
mkitem!{mkimpl!{impl ForeignModule { pub fn items (& self) -> Vec < ForeignDef > { with (| cx | cx . foreign_items (self . def_id)) } }}}
mkitem!{crate_def_with_ty ! { # [doc = " Hold information about a ForeignItem in a crate."] # [derive (Serialize)] pub ForeignDef ; }}
mkitem!{mkimpl!{impl ForeignDef { pub fn kind (& self) -> ForeignItemKind { with (| cx | cx . foreign_item_kind (* self)) } }}}
mkitem!{mkenum!{# [derive (Clone , Copy , PartialEq , Eq , Debug , Hash , Serialize)] pub enum ForeignItemKind { Fn (FnDef) , Static (StaticDef) , Type (Ty) , }}}
mkitem!{crate_def_with_ty ! { # [doc = " Hold information about a function definition in a crate."] # [derive (Serialize)] pub FnDef ; }}
mkitem!{mkimpl!{impl FnDef { pub fn body (& self) -> Option < Body > { with (| ctx | ctx . has_body (self . 0) . then (| | ctx . mir_body (self . 0))) } pub fn has_body (& self) -> bool { with (| ctx | ctx . has_body (self . 0)) } # [doc = " Get the information of the intrinsic if this function is a definition of one."] pub fn as_intrinsic (& self) -> Option < IntrinsicDef > { with (| cx | cx . intrinsic (self . def_id ())) } # [doc = " Check if the function is an intrinsic."] # [inline] pub fn is_intrinsic (& self) -> bool { self . as_intrinsic () . is_some () } # [doc = " Get the function signature for this function definition."] pub fn fn_sig (& self) -> PolyFnSig { let kind = self . ty () . kind () ; kind . fn_sig () . unwrap () } }}}
mkitem!{crate_def_with_ty ! { # [derive (Serialize)] pub IntrinsicDef ; }}
mkitem!{mkimpl!{impl IntrinsicDef { # [doc = " Returns the plain name of the intrinsic."] # [doc = " e.g., `transmute` for `core::intrinsics::transmute`."] pub fn fn_name (& self) -> Symbol { with (| cx | cx . intrinsic_name (* self)) } # [doc = " Returns whether the intrinsic has no meaningful body and all backends"] # [doc = " need to shim all calls to it."] pub fn must_be_overridden (& self) -> bool { with (| cx | ! cx . has_body (self . 0)) } }}}
mkitem!{mkimpl!{impl From < IntrinsicDef > for FnDef { fn from (def : IntrinsicDef) -> Self { FnDef (def . 0) } }}}
mkitem!{crate_def ! { # [derive (Serialize)] pub ClosureDef ; }}
mkitem!{mkimpl!{impl ClosureDef { # [doc = " Retrieves the body of the closure definition. Returns None if the body"] # [doc = " isn't available."] pub fn body (& self) -> Option < Body > { with (| ctx | ctx . has_body (self . 0) . then (| | ctx . mir_body (self . 0))) } }}}
mkitem!{crate_def ! { # [derive (Serialize)] pub CoroutineDef ; }}
mkitem!{mkimpl!{impl CoroutineDef { # [doc = " Retrieves the body of the coroutine definition. Returns None if the body"] # [doc = " isn't available."] pub fn body (& self) -> Option < Body > { with (| cx | cx . has_body (self . 0) . then (| | cx . mir_body (self . 0))) } pub fn discriminant_for_variant (& self , args : & GenericArgs , idx : VariantIdx) -> Discr { with (| cx | cx . coroutine_discr_for_variant (* self , args , idx)) } }}}
mkitem!{crate_def ! { # [derive (Serialize)] pub CoroutineClosureDef ; }}
mkitem!{crate_def ! { # [derive (Serialize)] pub ParamDef ; }}
mkitem!{crate_def ! { # [derive (Serialize)] pub BrNamedDef ; }}
mkitem!{crate_def ! { # [derive (Serialize)] pub AdtDef ; }}
mkitem!{mkenum!{# [derive (Clone , Copy , PartialEq , Eq , Debug , Hash , Serialize)] pub enum AdtKind { Enum , Union , Struct , }}}
mkitem!{mkimpl!{impl AdtDef { pub fn kind (& self) -> AdtKind { with (| cx | cx . adt_kind (* self)) } # [doc = " Retrieve the type of this Adt."] pub fn ty (& self) -> Ty { with (| cx | cx . def_ty (self . 0)) } # [doc = " Retrieve the type of this Adt by instantiating and normalizing it with the given arguments."] # [doc = ""] # [doc = " This will assume the type can be instantiated with these arguments."] pub fn ty_with_args (& self , args : & GenericArgs) -> Ty { with (| cx | cx . def_ty_with_args (self . 0 , args)) } pub fn is_box (& self) -> bool { with (| cx | cx . adt_is_box (* self)) } pub fn is_simd (& self) -> bool { with (| cx | cx . adt_is_simd (* self)) } # [doc = " The number of variants in this ADT."] pub fn num_variants (& self) -> usize { with (| cx | cx . adt_variants_len (* self)) } # [doc = " Retrieve the variants in this ADT."] pub fn variants (& self) -> Vec < VariantDef > { self . variants_iter () . collect () } # [doc = " Iterate over the variants in this ADT."] pub fn variants_iter (& self) -> impl Iterator < Item = VariantDef > { (0 .. self . num_variants ()) . map (| idx | VariantDef { idx : VariantIdx :: to_val (idx) , adt_def : * self }) } pub fn variant (& self , idx : VariantIdx) -> Option < VariantDef > { (idx . to_index () < self . num_variants ()) . then_some (VariantDef { idx , adt_def : * self }) } pub fn repr (& self) -> ReprOptions { with (| cx | cx . adt_repr (* self)) } pub fn discriminant_for_variant (& self , idx : VariantIdx) -> Discr { with (| cx | cx . adt_discr_for_variant (* self , idx)) } }}}
mkitem!{mkstruct!{pub struct Discr { pub val : u128 , pub ty : Ty , }}}
mkitem!{mkstruct!{# [doc = " Definition of a variant, which can be either a struct / union field or an enum variant."] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash , Serialize)] pub struct VariantDef { # [doc = " The variant index."] # [doc = ""] # [doc = " ## Warning"] # [doc = " Do not access this field directly!"] pub idx : VariantIdx , # [doc = " The data type where this variant comes from."] # [doc = " For now, we use this to retrieve information about the variant itself so we don't need to"] # [doc = " cache more information."] # [doc = ""] # [doc = " ## Warning"] # [doc = " Do not access this field directly!"] pub adt_def : AdtDef , }}}
mkitem!{mkimpl!{impl VariantDef { pub fn name (& self) -> Symbol { with (| cx | cx . variant_name (* self)) } # [doc = " Retrieve all the fields in this variant."] pub fn fields (& self) -> Vec < FieldDef > { with (| cx | cx . variant_fields (* self)) } }}}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct FieldDef { # [doc = " The field definition."] # [doc = ""] # [doc = " ## Warning"] # [doc = " Do not access this field directly! This is public for the compiler to have access to it."] pub def : DefId , # [doc = " The field name."] pub name : Symbol , }}}
mkitem!{mkimpl!{impl FieldDef { # [doc = " Retrieve the type of this field instantiating and normalizing it with the given arguments."] # [doc = ""] # [doc = " This will assume the type can be instantiated with these arguments."] pub fn ty_with_args (& self , args : & GenericArgs) -> Ty { with (| cx | cx . def_ty_with_args (self . def , args)) } # [doc = " Retrieve the type of this field."] pub fn ty (& self) -> Ty { with (| cx | cx . def_ty (self . def)) } }}}
mkitem!{mkimpl!{impl Display for AdtKind { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . write_str (match self { AdtKind :: Enum => "enum" , AdtKind :: Union => "union" , AdtKind :: Struct => "struct" , }) } }}}
mkitem!{mkimpl!{impl AdtKind { pub fn is_enum (& self) -> bool { matches ! (self , AdtKind :: Enum) } pub fn is_struct (& self) -> bool { matches ! (self , AdtKind :: Struct) } pub fn is_union (& self) -> bool { matches ! (self , AdtKind :: Union) } }}}
mkitem!{crate_def ! { # [derive (Serialize)] pub AliasDef ; }}
mkitem!{crate_def ! { # [doc = " A trait's definition."] # [derive (Serialize)] pub TraitDef ; }}
mkitem!{impl_crate_def_items ! { TraitDef ; }}
mkitem!{mkimpl!{impl TraitDef { pub fn declaration (trait_def : & TraitDef) -> TraitDecl { with (| cx | cx . trait_decl (trait_def)) } }}}
mkitem!{crate_def ! { # [derive (Serialize)] pub GenericDef ; }}
mkitem!{crate_def_with_ty ! { # [derive (Serialize)] pub ConstDef ; }}
mkitem!{crate_def ! { # [doc = " A trait impl definition."] # [derive (Serialize)] pub ImplDef ; }}
mkitem!{impl_crate_def_items ! { ImplDef ; }}
mkitem!{mkimpl!{impl ImplDef { # [doc = " Retrieve information about this implementation."] pub fn trait_impl (& self) -> ImplTrait { with (| cx | cx . trait_impl (self)) } }}}
mkitem!{crate_def ! { # [derive (Serialize)] pub RegionDef ; }}
mkitem!{crate_def ! { # [derive (Serialize)] pub CoroutineWitnessDef ; }}
mkitem!{mkstruct!{# [doc = " A list of generic arguments."] # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct GenericArgs (pub Vec < GenericArgKind >) ;}}
mkitem!{mkimpl!{impl std :: ops :: Index < ParamTy > for GenericArgs { type Output = Ty ; fn index (& self , index : ParamTy) -> & Self :: Output { self . 0 [index . index as usize] . expect_ty () } }}}
mkitem!{mkimpl!{impl std :: ops :: Index < ParamConst > for GenericArgs { type Output = TyConst ; fn index (& self , index : ParamConst) -> & Self :: Output { self . 0 [index . index as usize] . expect_const () } }}}
mkitem!{mkenum!{# [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum GenericArgKind { Lifetime (Region) , Type (Ty) , Const (TyConst) , }}}
mkitem!{mkimpl!{impl GenericArgKind { # [doc = " Panic if this generic argument is not a type, otherwise"] # [doc = " return the type."] # [track_caller] pub fn expect_ty (& self) -> & Ty { match self { GenericArgKind :: Type (ty) => ty , _ => panic ! ("{self:?}") , } } # [doc = " Panic if this generic argument is not a const, otherwise"] # [doc = " return the const."] # [track_caller] pub fn expect_const (& self) -> & TyConst { match self { GenericArgKind :: Const (c) => c , _ => panic ! ("{self:?}") , } } # [doc = " Return the generic argument type if applicable, otherwise return `None`."] pub fn ty (& self) -> Option < & Ty > { match self { GenericArgKind :: Type (ty) => Some (ty) , _ => None , } } }}}
mkitem!{mkenum!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum TermKind { Type (Ty) , Const (TyConst) , }}}
mkitem!{mkenum!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum AliasKind { Projection , Inherent , Opaque , Free , }}}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct AliasTy { pub def_id : AliasDef , pub args : GenericArgs , }}}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct AliasTerm { pub def_id : AliasDef , pub args : GenericArgs , }}}
mkitem!{pub type PolyFnSig = Binder < FnSig > ;}
mkitem!{mkimpl!{impl PolyFnSig { # [doc = " Compute a `FnAbi` suitable for indirect calls, i.e. to `fn` pointers."] # [doc = ""] # [doc = " NB: this doesn't handle virtual calls - those should use `Instance::fn_abi`"] # [doc = " instead, where the instance is an `InstanceKind::Virtual`."] pub fn fn_ptr_abi (self) -> Result < FnAbi , Error > { with (| cx | cx . fn_ptr_abi (self)) } }}}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct FnSig { pub inputs_and_output : Vec < Ty > , pub c_variadic : bool , pub safety : Safety , pub abi : Abi , }}}
mkitem!{mkimpl!{impl FnSig { pub fn output (& self) -> Ty { self . inputs_and_output [self . inputs_and_output . len () - 1] } pub fn inputs (& self) -> & [Ty] { & self . inputs_and_output [.. self . inputs_and_output . len () - 1] } }}}
mkitem!{mkenum!{# [derive (Clone , PartialEq , Eq , Debug , Serialize)] pub enum Abi { Rust , C { unwind : bool } , Cdecl { unwind : bool } , Stdcall { unwind : bool } , Fastcall { unwind : bool } , Vectorcall { unwind : bool } , Thiscall { unwind : bool } , Aapcs { unwind : bool } , Win64 { unwind : bool } , SysV64 { unwind : bool } , PtxKernel , Msp430Interrupt , X86Interrupt , GpuKernel , EfiApi , AvrInterrupt , AvrNonBlockingInterrupt , CCmseNonSecureCall , CCmseNonSecureEntry , System { unwind : bool } , RustCall , Unadjusted , RustCold , RiscvInterruptM , RiscvInterruptS , RustInvalid , Custom , }}}
mkitem!{mkstruct!{# [doc = " A binder represents a possibly generic type and its bound vars."] # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct Binder < T > { pub value : T , pub bound_vars : Vec < BoundVariableKind > , }}}
mkitem!{mkimpl!{impl < T > Binder < T > { # [doc = " Create a new binder with the given bound vars."] pub fn bind_with_vars (value : T , bound_vars : Vec < BoundVariableKind >) -> Self { Binder { value , bound_vars } } # [doc = " Create a new binder with no bounded variable."] pub fn dummy (value : T) -> Self { Binder { value , bound_vars : vec ! [] } } pub fn skip_binder (self) -> T { self . value } pub fn map_bound_ref < F , U > (& self , f : F) -> Binder < U > where F : FnOnce (& T) -> U , { let Binder { value , bound_vars } = self ; let new_value = f (value) ; Binder { value : new_value , bound_vars : bound_vars . clone () } } pub fn map_bound < F , U > (self , f : F) -> Binder < U > where F : FnOnce (T) -> U , { let Binder { value , bound_vars } = self ; let new_value = f (value) ; Binder { value : new_value , bound_vars } } }}}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct EarlyBinder < T > { pub value : T , }}}
mkitem!{mkenum!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum BoundVariableKind { Ty (BoundTyKind) , Region (BoundRegionKind) , Const , }}}
mkitem!{mkenum!{# [derive (Clone , PartialEq , Eq , Debug , Serialize)] pub enum BoundTyKind { Anon , Param (ParamDef , String) , }}}
mkitem!{mkenum!{# [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum BoundRegionKind { BrAnon , BrNamed (BrNamedDef , String) , BrEnv , }}}
mkitem!{mkenum!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum DynKind { Dyn , }}}
mkitem!{mkenum!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum ExistentialPredicate { Trait (ExistentialTraitRef) , Projection (ExistentialProjection) , AutoTrait (TraitDef) , }}}
mkitem!{mkstruct!{# [doc = " An existential reference to a trait where `Self` is not included."] # [doc = ""] # [doc = " The `generic_args` will include any other known argument."] # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct ExistentialTraitRef { pub def_id : TraitDef , pub generic_args : GenericArgs , }}}
mkitem!{mkimpl!{impl Binder < ExistentialTraitRef > { pub fn with_self_ty (& self , self_ty : Ty) -> Binder < TraitRef > { self . map_bound_ref (| trait_ref | trait_ref . with_self_ty (self_ty)) } }}}
mkitem!{mkimpl!{impl ExistentialTraitRef { pub fn with_self_ty (& self , self_ty : Ty) -> TraitRef { TraitRef :: new (self . def_id , self_ty , & self . generic_args) } }}}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct ExistentialProjection { pub def_id : TraitDef , pub generic_args : GenericArgs , pub term : TermKind , }}}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct ParamTy { pub index : u32 , pub name : String , }}}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct BoundTy { pub var : usize , pub kind : BoundTyKind , }}}
mkitem!{pub type Bytes = Vec < Option < u8 > > ;}
mkitem!{# [doc = " Size in bytes."] pub type Size = usize ;}
mkitem!{mkstruct!{# [derive (Clone , Copy , PartialEq , Eq , Debug , Hash , Serialize)] pub struct Prov (pub AllocId) ;}}
mkitem!{pub type Align = u64 ;}
mkitem!{pub type Promoted = u32 ;}
mkitem!{pub type InitMaskMaterialized = Vec < u64 > ;}
mkitem!{mkstruct!{# [doc = " Stores the provenance information of pointers stored in memory."] # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct ProvenanceMap { # [doc = " Provenance in this map applies from the given offset for an entire pointer-size worth of"] # [doc = " bytes. Two entries in this map are always at least a pointer size apart."] pub ptrs : Vec < (Size , Prov) > , }}}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct Allocation { pub bytes : Bytes , pub provenance : ProvenanceMap , pub align : Align , pub mutability : Mutability , }}}
mkitem!{mkimpl!{impl Allocation { # [doc = " Get a vector of bytes for an Allocation that has been fully initialized"] pub fn raw_bytes (& self) -> Result < Vec < u8 > , Error > { self . bytes . iter () . copied () . collect :: < Option < Vec < _ > > > () . ok_or_else (| | error ! ("Found uninitialized bytes: `{:?}`" , self . bytes)) } # [doc = " Read a uint value from the specified range."] pub fn read_partial_uint (& self , range : Range < usize >) -> Result < u128 , Error > { if range . end - range . start > 16 { return Err (error ! ("Allocation is bigger than largest integer")) ; } if range . end > self . bytes . len () { return Err (error ! ("Range is out of bounds. Allocation length is `{}`, but requested range `{:?}`" , self . bytes . len () , range)) ; } let raw = self . bytes [range] . iter () . copied () . collect :: < Option < Vec < _ > > > () . ok_or_else (| | error ! ("Found uninitialized bytes: `{:?}`" , self . bytes)) ? ; read_target_uint (& raw) } # [doc = " Read this allocation and try to convert it to an unassigned integer."] pub fn read_uint (& self) -> Result < u128 , Error > { if self . bytes . len () > 16 { return Err (error ! ("Allocation is bigger than largest integer")) ; } let raw = self . raw_bytes () ? ; read_target_uint (& raw) } # [doc = " Read this allocation and try to convert it to a signed integer."] pub fn read_int (& self) -> Result < i128 , Error > { if self . bytes . len () > 16 { return Err (error ! ("Allocation is bigger than largest integer")) ; } let raw = self . raw_bytes () ? ; read_target_int (& raw) } # [doc = " Read this allocation and try to convert it to a boolean."] pub fn read_bool (& self) -> Result < bool , Error > { match self . read_int () ? { 0 => Ok (false) , 1 => Ok (true) , val => Err (error ! ("Unexpected value for bool: `{val}`")) , } } # [doc = " Read this allocation as a pointer and return whether it represents a `null` pointer."] pub fn is_null (& self) -> Result < bool , Error > { let len = self . bytes . len () ; let ptr_len = MachineInfo :: target_pointer_width () . bytes () ; if len != ptr_len { return Err (error ! ("Expected width of pointer (`{ptr_len}`), but found: `{len}`")) ; } Ok (self . read_uint () ? == 0 && self . provenance . ptrs . is_empty ()) } }}}
mkitem!{mkenum!{# [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum ConstantKind { Ty (TyConst) , Allocated (Allocation) , Unevaluated (UnevaluatedConst) , Param (ParamConst) , # [doc = " Store ZST constants."] # [doc = " We have to special handle these constants since its type might be generic."] ZeroSized , }}}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct ParamConst { pub index : u32 , pub name : String , }}}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct UnevaluatedConst { pub def : ConstDef , pub args : GenericArgs , pub promoted : Option < Promoted > , }}}
mkitem!{mkenum!{# [derive (Clone , Copy , Debug , PartialEq , Eq , Serialize)] pub enum TraitSpecializationKind { None , Marker , AlwaysApplicable , }}}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct TraitDecl { pub def_id : TraitDef , pub safety : Safety , pub paren_sugar : bool , pub has_auto_impl : bool , pub is_marker : bool , pub is_coinductive : bool , pub skip_array_during_method_dispatch : bool , pub skip_boxed_slice_during_method_dispatch : bool , pub specialization_kind : TraitSpecializationKind , pub must_implement_one_of : Option < Vec < Ident > > , pub implement_via_object : bool , pub deny_explicit_impl : bool , }}}
mkitem!{mkimpl!{impl TraitDecl { pub fn generics_of (& self) -> Generics { with (| cx | cx . generics_of (self . def_id . 0)) } pub fn predicates_of (& self) -> GenericPredicates { with (| cx | cx . predicates_of (self . def_id . 0)) } pub fn explicit_predicates_of (& self) -> GenericPredicates { with (| cx | cx . explicit_predicates_of (self . def_id . 0)) } }}}
mkitem!{pub type ImplTrait = EarlyBinder < TraitRef > ;}
mkitem!{mkstruct!{# [doc = " A complete reference to a trait, i.e., one where `Self` is known."] # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct TraitRef { pub def_id : TraitDef , # [doc = " The generic arguments for this definition."] # [doc = " The first element must always be type, and it represents `Self`."] args : GenericArgs , }}}
mkitem!{mkimpl!{impl TraitRef { pub fn new (def_id : TraitDef , self_ty : Ty , gen_args : & GenericArgs) -> TraitRef { let mut args = vec ! [GenericArgKind :: Type (self_ty)] ; args . extend_from_slice (& gen_args . 0) ; TraitRef { def_id , args : GenericArgs (args) } } pub fn try_new (def_id : TraitDef , args : GenericArgs) -> Result < TraitRef , () > { match & args . 0 [..] { [GenericArgKind :: Type (_) , ..] => Ok (TraitRef { def_id , args }) , _ => Err (()) , } } pub fn args (& self) -> & GenericArgs { & self . args } pub fn self_ty (& self) -> Ty { let GenericArgKind :: Type (self_ty) = self . args . 0 [0] else { panic ! ("Self must be a type, but found: {:?}" , self . args . 0 [0]) } ; self_ty } }}}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct Generics { pub parent : Option < GenericDef > , pub parent_count : usize , pub params : Vec < GenericParamDef > , pub param_def_id_to_index : Vec < (GenericDef , u32) > , pub has_self : bool , pub has_late_bound_regions : Option < Span > , }}}
mkitem!{mkenum!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum GenericParamDefKind { Lifetime , Type { has_default : bool , synthetic : bool } , Const { has_default : bool } , }}}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct GenericParamDef { pub name : super :: Symbol , pub def_id : GenericDef , pub index : u32 , pub pure_wrt_drop : bool , pub kind : GenericParamDefKind , }}}
mkitem!{mkstruct!{pub struct GenericPredicates { pub parent : Option < TraitDef > , pub predicates : Vec < (PredicateKind , Span) > , }}}
mkitem!{mkenum!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum PredicateKind { Clause (ClauseKind) , DynCompatible (TraitDef) , SubType (SubtypePredicate) , Coerce (CoercePredicate) , ConstEquate (TyConst , TyConst) , Ambiguous , AliasRelate (TermKind , TermKind , AliasRelationDirection) , }}}
mkitem!{mkenum!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum ClauseKind { Trait (TraitPredicate) , RegionOutlives (RegionOutlivesPredicate) , TypeOutlives (TypeOutlivesPredicate) , Projection (ProjectionPredicate) , ConstArgHasType (TyConst , Ty) , WellFormed (TermKind) , ConstEvaluatable (TyConst) , }}}
mkitem!{mkenum!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum ClosureKind { Fn , FnMut , FnOnce , }}}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct SubtypePredicate { pub a : Ty , pub b : Ty , }}}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct CoercePredicate { pub a : Ty , pub b : Ty , }}}
mkitem!{mkenum!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum AliasRelationDirection { Equate , Subtype , }}}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct TraitPredicate { pub trait_ref : TraitRef , pub polarity : PredicatePolarity , }}}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct OutlivesPredicate < A , B > (pub A , pub B) ;}}
mkitem!{pub type RegionOutlivesPredicate = OutlivesPredicate < Region , Region > ;}
mkitem!{pub type TypeOutlivesPredicate = OutlivesPredicate < Ty , Region > ;}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct ProjectionPredicate { pub projection_term : AliasTerm , pub term : TermKind , }}}
mkitem!{mkenum!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum ImplPolarity { Positive , Negative , Reservation , }}}
mkitem!{mkenum!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum PredicatePolarity { Positive , Negative , }}}
mkitem!{macro_rules ! index_impl { ($ name : ident) => { impl crate :: IndexedVal for $ name { fn to_val (index : usize) -> Self { $ name (index) } fn to_index (& self) -> usize { self . 0 } } } ; }}
mkitem!{index_impl ! (TyConstId) ;}
mkitem!{index_impl ! (MirConstId) ;}
mkitem!{index_impl ! (Ty) ;}
mkitem!{index_impl ! (Span) ;}
mkitem!{mkstruct!{# [doc = " The source-order index of a variant in a type."] # [doc = ""] # [doc = " For example, in the following types,"] # [doc = " ```ignore(illustrative)"] # [doc = " enum Demo1 {"] # [doc = "    Variant0 { a: bool, b: i32 },"] # [doc = "    Variant1 { c: u8, d: u64 },"] # [doc = " }"] # [doc = " struct Demo2 { e: u8, f: u16, g: u8 }"] # [doc = " ```"] # [doc = " `a` is in the variant with the `VariantIdx` of `0`,"] # [doc = " `c` is in the variant with the `VariantIdx` of `1`, and"] # [doc = " `g` is in the variant with the `VariantIdx` of `0`."] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash , Serialize)] pub struct VariantIdx (usize) ;}}
mkitem!{index_impl ! (VariantIdx) ;}
mkitem!{crate_def ! { # [doc = " Hold information about an Opaque definition, particularly useful in `RPITIT`."] # [derive (Serialize)] pub OpaqueDef ; }}
mkitem!{crate_def ! { # [derive (Serialize)] pub AssocDef ; }}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct AssocItem { pub def_id : AssocDef , pub kind : AssocKind , pub container : AssocContainer , }}}
mkitem!{mkenum!{# [derive (Clone , PartialEq , Debug , Eq , Serialize)] pub enum AssocTypeData { Normal (Symbol) , # [doc = " The associated type comes from an RPITIT. It has no name, and the"] # [doc = " `ImplTraitInTraitData` provides additional information about its"] # [doc = " source."] Rpitit (ImplTraitInTraitData) , }}}
mkitem!{mkenum!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum AssocKind { Const { name : Symbol } , Fn { name : Symbol , has_self : bool } , Type { data : AssocTypeData } , }}}
mkitem!{mkenum!{# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum AssocContainer { InherentImpl , # [doc = " The `AssocDef` points to the trait item being implemented."] TraitImpl (AssocDef) , Trait , }}}
mkitem!{mkenum!{# [derive (Clone , Copy , PartialEq , Eq , Debug , Hash , Serialize)] pub enum ImplTraitInTraitData { Trait { fn_def_id : FnDef , opaque_def_id : OpaqueDef } , Impl { fn_def_id : FnDef } , }}}
mkitem!{mkimpl!{impl AssocItem { pub fn is_impl_trait_in_trait (& self) -> bool { matches ! (self . kind , AssocKind :: Type { data : AssocTypeData :: Rpitit (_) }) } }}}