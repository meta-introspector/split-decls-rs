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
mkuse!{use std :: fmt ;}
mkuse!{use std :: num :: NonZero ;}
mkuse!{use rustc_abi :: { HasDataLayout , Size } ;}
mkuse!{use rustc_data_structures :: static_assert_size ;}
mkuse!{use rustc_macros :: { HashStable , TyDecodable , TyEncodable } ;}
mkuse!{use super :: AllocId ;}
mkitem!{mktrait!{pub trait PointerArithmetic : HasDataLayout { # [inline (always)] fn pointer_size (& self) -> Size { self . data_layout () . pointer_size () } # [inline (always)] fn max_size_of_val (& self) -> Size { Size :: from_bytes (self . target_isize_max ()) } # [inline] fn target_usize_max (& self) -> u64 { self . pointer_size () . unsigned_int_max () . try_into () . unwrap () } # [inline] fn target_isize_min (& self) -> i64 { self . pointer_size () . signed_int_min () . try_into () . unwrap () } # [inline] fn target_isize_max (& self) -> i64 { self . pointer_size () . signed_int_max () . try_into () . unwrap () } # [inline] fn truncate_to_target_usize (& self , val : u64) -> u64 { self . pointer_size () . truncate (val . into ()) . try_into () . unwrap () } # [inline] fn sign_extend_to_target_isize (& self , val : u64) -> i64 { self . pointer_size () . sign_extend (val . into ()) . try_into () . unwrap () } }}}
mkitem!{mkimpl!{impl < T : HasDataLayout > PointerArithmetic for T { }}}
mkitem!{mktrait!{# [doc = " This trait abstracts over the kind of provenance that is associated with a `Pointer`. It is"] # [doc = " mostly opaque; the `Machine` trait extends it with some more operations that also have access to"] # [doc = " some global state."] # [doc = " The `Debug` rendering is used to display bare provenance, and for the default impl of `fmt`."] pub trait Provenance : Copy + PartialEq + fmt :: Debug + 'static { # [doc = " Says whether the `offset` field of `Pointer`s with this provenance is the actual physical address."] # [doc = " - If `false`, the offset *must* be relative. This means the bytes representing a pointer are"] # [doc = "   different from what the Abstract Machine prescribes, so the interpreter must prevent any"] # [doc = "   operation that would inspect the underlying bytes of a pointer, such as ptr-to-int"] # [doc = "   transmutation. A `ReadPointerAsBytes` error will be raised in such situations."] # [doc = " - If `true`, the interpreter will permit operations to inspect the underlying bytes of a"] # [doc = "   pointer, and implement ptr-to-int transmutation by stripping provenance."] const OFFSET_IS_ADDR : bool ; # [doc = " If wildcard provenance is implemented, contains the unique, general wildcard provenance variant."] const WILDCARD : Option < Self > ; # [doc = " Determines how a pointer should be printed."] fn fmt (ptr : & Pointer < Self > , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result ; # [doc = " If `OFFSET_IS_ADDR == false`, provenance must always be able to"] # [doc = " identify the allocation this ptr points to (i.e., this must return `Some`)."] # [doc = " Otherwise this function is best-effort (but must agree with `Machine::ptr_get_alloc`)."] # [doc = " (Identifying the offset in that allocation, however, is harder -- use `Memory::ptr_get_alloc` for that.)"] fn get_alloc_id (self) -> Option < AllocId > ; # [doc = " Defines the 'join' of provenance: what happens when doing a pointer load and different bytes have different provenance."] fn join (left : Self , right : Self) -> Option < Self > ; }}}
mkitem!{mkstruct!{# [doc = " The type of provenance in the compile-time interpreter."] # [doc = " This is a packed representation of:"] # [doc = " - an `AllocId` (non-zero)"] # [doc = " - an `immutable: bool`"] # [doc = " - a `shared_ref: bool`"] # [doc = ""] # [doc = " with the extra invariant that if `immutable` is `true`, then so"] # [doc = " is `shared_ref`."] # [derive (Copy , Clone , Eq , Hash , Ord , PartialEq , PartialOrd)] pub struct CtfeProvenance (NonZero < u64 >) ;}}
mkitem!{mkimpl!{impl From < AllocId > for CtfeProvenance { fn from (value : AllocId) -> Self { let prov = CtfeProvenance (value . 0) ; assert ! (prov . alloc_id () == value , "`AllocId` with the highest bits set cannot be used in CTFE") ; prov } }}}
mkitem!{mkimpl!{impl fmt :: Debug for CtfeProvenance { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . alloc_id () , f) ? ; if self . immutable () { write ! (f , "<imm>") ? ; } Ok (()) } }}}
mkitem!{const IMMUTABLE_MASK : u64 = 1 << 63 ;}
mkitem!{const SHARED_REF_MASK : u64 = 1 << 62 ;}
mkitem!{const ALLOC_ID_MASK : u64 = u64 :: MAX & ! IMMUTABLE_MASK & ! SHARED_REF_MASK ;}
mkitem!{mkimpl!{impl CtfeProvenance { # [doc = " Returns the `AllocId` of this provenance."] # [inline (always)] pub fn alloc_id (self) -> AllocId { AllocId (NonZero :: new (self . 0 . get () & ALLOC_ID_MASK) . unwrap ()) } # [doc = " Returns whether this provenance is immutable."] # [inline] pub fn immutable (self) -> bool { self . 0 . get () & IMMUTABLE_MASK != 0 } # [doc = " Returns whether this provenance is derived from a shared reference."] # [inline] pub fn shared_ref (self) -> bool { self . 0 . get () & SHARED_REF_MASK != 0 } pub fn into_parts (self) -> (AllocId , bool , bool) { (self . alloc_id () , self . immutable () , self . shared_ref ()) } pub fn from_parts ((alloc_id , immutable , shared_ref) : (AllocId , bool , bool)) -> Self { let prov = CtfeProvenance :: from (alloc_id) ; if immutable { prov . as_immutable () } else if shared_ref { prov . as_shared_ref () } else { prov } } # [doc = " Returns an immutable version of this provenance."] # [inline] pub fn as_immutable (self) -> Self { CtfeProvenance (self . 0 | IMMUTABLE_MASK | SHARED_REF_MASK) } # [doc = " Returns a \"shared reference\" (but not necessarily immutable!) version of this provenance."] # [inline] pub fn as_shared_ref (self) -> Self { CtfeProvenance (self . 0 | SHARED_REF_MASK) } }}}
mkitem!{mkimpl!{impl Provenance for CtfeProvenance { const OFFSET_IS_ADDR : bool = false ; const WILDCARD : Option < Self > = None ; fn fmt (ptr : & Pointer < Self > , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& ptr . provenance . alloc_id () , f) ? ; if ptr . offset . bytes () > 0 { write ! (f , "+{:#x}" , ptr . offset . bytes ()) ? ; } if ptr . provenance . immutable () { write ! (f , "<imm>") ? ; } Ok (()) } fn get_alloc_id (self) -> Option < AllocId > { Some (self . alloc_id ()) } fn join (left : Self , right : Self) -> Option < Self > { if left == right { Some (left) } else { None } } }}}
mkitem!{mkimpl!{impl Provenance for AllocId { const OFFSET_IS_ADDR : bool = false ; const WILDCARD : Option < Self > = None ; fn fmt (ptr : & Pointer < Self > , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if f . alternate () { write ! (f , "{:#?}" , ptr . provenance) ? ; } else { write ! (f , "{:?}" , ptr . provenance) ? ; } if ptr . offset . bytes () > 0 { write ! (f , "+{:#x}" , ptr . offset . bytes ()) ? ; } Ok (()) } fn get_alloc_id (self) -> Option < AllocId > { Some (self) } fn join (_left : Self , _right : Self) -> Option < Self > { unreachable ! () } }}}
mkitem!{mkstruct!{# [doc = " Represents a pointer in the Miri engine."] # [doc = ""] # [doc = " Pointers are \"tagged\" with provenance information; typically the `AllocId` they belong to."] # [derive (Copy , Clone , Eq , PartialEq , TyEncodable , TyDecodable , Hash)] # [derive (HashStable)] pub struct Pointer < Prov = CtfeProvenance > { pub (super) offset : Size , pub provenance : Prov , }}}
mkitem!{static_assert_size ! (Pointer , 16) ;}
mkitem!{static_assert_size ! (Pointer < Option < CtfeProvenance >>, 16) ;}
mkitem!{mkimpl!{impl < Prov : Provenance > fmt :: Debug for Pointer < Prov > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { Provenance :: fmt (self , f) } }}}
mkitem!{mkimpl!{impl < Prov : Provenance > fmt :: Debug for Pointer < Option < Prov > > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . provenance { Some (prov) => Provenance :: fmt (& Pointer :: new (prov , self . offset) , f) , None => write ! (f , "{:#x}[noalloc]" , self . offset . bytes ()) , } } }}}
mkitem!{mkimpl!{impl < Prov : Provenance > fmt :: Display for Pointer < Option < Prov > > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . provenance . is_none () && self . offset . bytes () == 0 { write ! (f , "null pointer") } else { fmt :: Debug :: fmt (self , f) } } }}}
mkitem!{mkimpl!{# [doc = " Produces a `Pointer` that points to the beginning of the `Allocation`."] impl From < AllocId > for Pointer { # [inline (always)] fn from (alloc_id : AllocId) -> Self { Pointer :: new (alloc_id . into () , Size :: ZERO) } }}}
mkitem!{mkimpl!{impl From < CtfeProvenance > for Pointer { # [inline (always)] fn from (prov : CtfeProvenance) -> Self { Pointer :: new (prov , Size :: ZERO) } }}}
mkitem!{mkimpl!{impl < Prov > From < Pointer < Prov > > for Pointer < Option < Prov > > { # [inline (always)] fn from (ptr : Pointer < Prov >) -> Self { let (prov , offset) = ptr . into_raw_parts () ; Pointer :: new (Some (prov) , offset) } }}}
mkitem!{mkimpl!{impl < Prov > Pointer < Option < Prov > > { # [doc = " Convert this pointer that *might* have a provenance into a pointer that *definitely* has a"] # [doc = " provenance, or an absolute address."] # [doc = ""] # [doc = " This is rarely what you want; call `ptr_try_get_alloc_id` instead."] pub fn into_pointer_or_addr (self) -> Result < Pointer < Prov > , Size > { match self . provenance { Some (prov) => Ok (Pointer :: new (prov , self . offset)) , None => Err (self . offset) , } } # [doc = " Returns the absolute address the pointer points to."] # [doc = " Only works if Prov::OFFSET_IS_ADDR is true!"] pub fn addr (self) -> Size where Prov : Provenance , { assert ! (Prov :: OFFSET_IS_ADDR) ; self . offset } # [doc = " Creates a pointer to the given address, with invalid provenance (i.e., cannot be used for"] # [doc = " any memory access)."] # [inline (always)] pub fn without_provenance (addr : u64) -> Self { Pointer { provenance : None , offset : Size :: from_bytes (addr) } } # [inline (always)] pub fn null () -> Self { Pointer :: without_provenance (0) } }}}
mkitem!{mkimpl!{impl < Prov > Pointer < Prov > { # [inline (always)] pub fn new (provenance : Prov , offset : Size) -> Self { Pointer { provenance , offset } } # [doc = " Obtain the constituents of this pointer. Note that the meaning of the offset depends on the"] # [doc = " type `Prov`! This is a low-level function that should only be used when absolutely"] # [doc = " necessary. Prefer `prov_and_relative_offset` if possible."] # [inline (always)] pub fn into_raw_parts (self) -> (Prov , Size) { (self . provenance , self . offset) } pub fn map_provenance (self , f : impl FnOnce (Prov) -> Prov) -> Self { Pointer { provenance : f (self . provenance) , .. self } } # [inline (always)] pub fn wrapping_offset (self , i : Size , cx : & impl HasDataLayout) -> Self { let res = cx . data_layout () . truncate_to_target_usize (self . offset . bytes () . wrapping_add (i . bytes ())) ; Pointer { offset : Size :: from_bytes (res) , .. self } } # [inline (always)] pub fn wrapping_signed_offset (self , i : i64 , cx : & impl HasDataLayout) -> Self { self . wrapping_offset (Size :: from_bytes (i as u64) , cx) } }}}
mkitem!{mkimpl!{impl Pointer < CtfeProvenance > { # [doc = " Return the provenance and relative offset stored in this pointer. Safer alternative to"] # [doc = " `into_raw_parts` since the type ensures that the offset is indeed relative."] # [inline (always)] pub fn prov_and_relative_offset (self) -> (CtfeProvenance , Size) { (self . provenance , self . offset) } }}}