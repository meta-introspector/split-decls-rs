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
mkuse!{use rustc_abi :: { HasDataLayout , Size } ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_macros :: { HashStable , Lift , TyDecodable , TyEncodable , TypeFoldable , TypeVisitable } ;}
mkuse!{use rustc_session :: RemapFileNameExt ;}
mkuse!{use rustc_session :: config :: RemapPathScopeComponents ;}
mkuse!{use rustc_span :: { DUMMY_SP , Span , Symbol } ;}
mkuse!{use rustc_type_ir :: TypeVisitableExt ;}
mkuse!{use super :: interpret :: ReportedErrorInfo ;}
mkuse!{use crate :: mir :: interpret :: { AllocId , AllocRange , ErrorHandled , GlobalAlloc , Scalar , alloc_range } ;}
mkuse!{use crate :: mir :: { Promoted , pretty_print_const_value } ;}
mkuse!{use crate :: ty :: print :: { pretty_print_const , with_no_trimmed_paths } ;}
mkuse!{use crate :: ty :: { self , ConstKind , GenericArgsRef , ScalarInt , Ty , TyCtxt } ;}
mkitem!{mkstruct!{# [doc = " Evaluated Constants"] # [doc = " Represents the result of const evaluation via the `eval_to_allocation` query."] # [doc = " Not to be confused with `ConstAllocation`, which directly refers to the underlying data!"] # [doc = " Here we indirect via an `AllocId`."] # [derive (Copy , Clone , HashStable , TyEncodable , TyDecodable , Debug , Hash , Eq , PartialEq)] pub struct ConstAlloc < 'tcx > { # [doc = " The value lives here, at offset 0, and that allocation definitely is an `AllocKind::Memory`"] # [doc = " (so you can use `AllocMap::unwrap_memory`)."] pub alloc_id : AllocId , pub ty : Ty < 'tcx > , }}}
mkitem!{mkenum!{# [doc = " Represents a constant value in Rust. `Scalar` and `Slice` are optimizations for"] # [doc = " array length computations, enum discriminants and the pattern matching logic."] # [derive (Copy , Clone , Debug , Eq , PartialEq , TyEncodable , TyDecodable , Hash)] # [derive (HashStable)] pub enum ConstValue { # [doc = " Used for types with `layout::abi::Scalar` ABI."] # [doc = ""] # [doc = " Not using the enum `Value` to encode that this must not be `Uninit`."] Scalar (Scalar) , # [doc = " Only for ZSTs."] ZeroSized , # [doc = " Used for references to unsized types with slice tail."] # [doc = ""] # [doc = " This is worth an optimized representation since Rust has literals of type `&str` and"] # [doc = " `&[u8]`. Not having to indirect those through an `AllocId` (or two, if we used `Indirect`)"] # [doc = " has shown measurable performance improvements on stress tests. We then reuse this"] # [doc = " optimization for slice-tail types more generally during valtree-to-constval conversion."] Slice { # [doc = " The allocation storing the slice contents."] # [doc = " This always points to the beginning of the allocation."] alloc_id : AllocId , # [doc = " The metadata field of the reference."] # [doc = " This is a \"target usize\", so we use `u64` as in the interpreter."] meta : u64 , } , # [doc = " A value not representable by the other variants; needs to be stored in-memory."] # [doc = ""] # [doc = " Must *not* be used for scalars or ZST, but having `&str` or other slices in this variant is fine."] Indirect { # [doc = " The backing memory of the value. May contain more memory than needed for just the value"] # [doc = " if this points into some other larger ConstValue."] # [doc = ""] # [doc = " We use an `AllocId` here instead of a `ConstAllocation<'tcx>` to make sure that when a"] # [doc = " raw constant (which is basically just an `AllocId`) is turned into a `ConstValue` and"] # [doc = " back, we can preserve the original `AllocId`."] alloc_id : AllocId , # [doc = " Offset into `alloc`"] offset : Size , } , }}}
mkitem!{# [cfg (target_pointer_width = "64")] rustc_data_structures :: static_assert_size ! (ConstValue , 24) ;}
mkitem!{mkimpl!{impl ConstValue { # [inline] pub fn try_to_scalar (& self) -> Option < Scalar > { match * self { ConstValue :: Indirect { .. } | ConstValue :: Slice { .. } | ConstValue :: ZeroSized => None , ConstValue :: Scalar (val) => Some (val) , } } pub fn try_to_scalar_int (& self) -> Option < ScalarInt > { self . try_to_scalar () ? . try_to_scalar_int () . ok () } pub fn try_to_bits (& self , size : Size) -> Option < u128 > { Some (self . try_to_scalar_int () ? . to_bits (size)) } pub fn try_to_bool (& self) -> Option < bool > { self . try_to_scalar_int () ? . try_into () . ok () } pub fn try_to_target_usize (& self , tcx : TyCtxt < '_ >) -> Option < u64 > { Some (self . try_to_scalar_int () ? . to_target_usize (tcx)) } pub fn try_to_bits_for_ty < 'tcx > (& self , tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , ty : Ty < 'tcx > ,) -> Option < u128 > { let size = tcx . layout_of (typing_env . with_post_analysis_normalized (tcx) . as_query_input (ty)) . ok () ? . size ; self . try_to_bits (size) } pub fn from_bool (b : bool) -> Self { ConstValue :: Scalar (Scalar :: from_bool (b)) } pub fn from_u64 (i : u64) -> Self { ConstValue :: Scalar (Scalar :: from_u64 (i)) } pub fn from_u128 (i : u128) -> Self { ConstValue :: Scalar (Scalar :: from_u128 (i)) } pub fn from_target_usize (i : u64 , cx : & impl HasDataLayout) -> Self { ConstValue :: Scalar (Scalar :: from_target_usize (i , cx)) } # [doc = " Must only be called on constants of type `&str` or `&[u8]`!"] pub fn try_get_slice_bytes_for_diagnostics < 'tcx > (& self , tcx : TyCtxt < 'tcx > ,) -> Option < & 'tcx [u8] > { let (alloc_id , start , len) = match self { ConstValue :: Scalar (_) | ConstValue :: ZeroSized => { bug ! ("`try_get_slice_bytes` on non-slice constant") } & ConstValue :: Slice { alloc_id , meta } => (alloc_id , 0 , meta) , & ConstValue :: Indirect { alloc_id , offset } => { let a = tcx . global_alloc (alloc_id) . unwrap_memory () . inner () ; let ptr_size = tcx . data_layout . pointer_size () ; if a . size () < offset + 2 * ptr_size { return None ; } let ptr = a . read_scalar (& tcx , alloc_range (offset , ptr_size) , true ,) . ok () ? ; let ptr = ptr . to_pointer (& tcx) . discard_err () ? ; let len = a . read_scalar (& tcx , alloc_range (offset + ptr_size , ptr_size) , false ,) . ok () ? ; let len = len . to_target_usize (& tcx) . discard_err () ? ; if len == 0 { return Some (& []) ; } let (inner_prov , offset) = ptr . into_pointer_or_addr () . ok () ? . prov_and_relative_offset () ; (inner_prov . alloc_id () , offset . bytes () , len) } } ; let data = tcx . global_alloc (alloc_id) . unwrap_memory () ; let start = start . try_into () . unwrap () ; let end = start + usize :: try_from (len) . unwrap () ; Some (data . inner () . inspect_with_uninit_and_ptr_outside_interpreter (start .. end)) } # [doc = " Check if a constant may contain provenance information. This is used by MIR opts."] # [doc = " Can return `true` even if there is no provenance."] pub fn may_have_provenance (& self , tcx : TyCtxt < '_ > , size : Size) -> bool { match * self { ConstValue :: ZeroSized | ConstValue :: Scalar (Scalar :: Int (_)) => return false , ConstValue :: Scalar (Scalar :: Ptr (..)) => return true , ConstValue :: Slice { alloc_id , meta : _ } => { ! tcx . global_alloc (alloc_id) . unwrap_memory () . inner () . provenance () . ptrs () . is_empty () } ConstValue :: Indirect { alloc_id , offset } => ! tcx . global_alloc (alloc_id) . unwrap_memory () . inner () . provenance () . range_empty (AllocRange :: from (offset .. offset + size) , & tcx) , } } # [doc = " Check if a constant only contains uninitialized bytes."] pub fn all_bytes_uninit (& self , tcx : TyCtxt < '_ >) -> bool { let ConstValue :: Indirect { alloc_id , .. } = self else { return false ; } ; let alloc = tcx . global_alloc (* alloc_id) ; let GlobalAlloc :: Memory (alloc) = alloc else { return false ; } ; let init_mask = alloc . 0 . init_mask () ; let init_range = init_mask . is_range_initialized (AllocRange { start : Size :: ZERO , size : Size :: from_bytes (alloc . 0 . len ()) , }) ; if let Err (range) = init_range { if range . size == alloc . 0 . size () { return true ; } } false } }}}
mkitem!{mkenum!{# [doc = " Constants"] # [derive (Clone , Copy , PartialEq , Eq , TyEncodable , TyDecodable , Hash , HashStable , Debug)] # [derive (TypeFoldable , TypeVisitable , Lift)] pub enum Const < 'tcx > { # [doc = " This constant came from the type system."] # [doc = ""] # [doc = " Any way of turning `ty::Const` into `ConstValue` should go through `valtree_to_const_val`;"] # [doc = " this ensures that we consistently produce \"clean\" values without data in the padding or"] # [doc = " anything like that."] # [doc = ""] # [doc = " FIXME(BoxyUwU): We should remove this `Ty` and look up the type for params via `ParamEnv`"] Ty (Ty < 'tcx > , ty :: Const < 'tcx >) , # [doc = " An unevaluated mir constant which is not part of the type system."] # [doc = ""] # [doc = " Note that `Ty(ty::ConstKind::Unevaluated)` and this variant are *not* identical! `Ty` will"] # [doc = " always flow through a valtree, so all data not captured in the valtree is lost. This variant"] # [doc = " directly uses the evaluated result of the given constant, including e.g. data stored in"] # [doc = " padding."] Unevaluated (UnevaluatedConst < 'tcx > , Ty < 'tcx >) , # [doc = " This constant cannot go back into the type system, as it represents"] # [doc = " something the type system cannot handle (e.g. pointers)."] Val (ConstValue , Ty < 'tcx >) , }}}
mkitem!{mkimpl!{impl < 'tcx > Const < 'tcx > { # [doc = " Creates an unevaluated const from a `DefId` for a const item."] # [doc = " The binders of the const item still need to be instantiated."] pub fn from_unevaluated (tcx : TyCtxt < 'tcx > , def_id : DefId ,) -> ty :: EarlyBinder < 'tcx , Const < 'tcx > > { ty :: EarlyBinder :: bind (Const :: Unevaluated (UnevaluatedConst { def : def_id , args : ty :: GenericArgs :: identity_for_item (tcx , def_id) , promoted : None , } , tcx . type_of (def_id) . skip_binder () ,)) } # [inline (always)] pub fn ty (& self) -> Ty < 'tcx > { match self { Const :: Ty (ty , ct) => { match ct . kind () { ty :: ConstKind :: Value (cv) => cv . ty , _ => * ty , } } Const :: Val (_ , ty) | Const :: Unevaluated (_ , ty) => * ty , } } # [doc = " Determines whether we need to add this const to `required_consts`. This is the case if and"] # [doc = " only if evaluating it may error."] # [inline] pub fn is_required_const (& self) -> bool { match self { Const :: Ty (_ , c) => match c . kind () { ty :: ConstKind :: Value (_) => false , _ => true , } , Const :: Val (..) => false , Const :: Unevaluated (..) => true , } } # [inline] pub fn try_to_scalar (self) -> Option < Scalar > { match self { Const :: Ty (_ , c) => match c . kind () { ty :: ConstKind :: Value (cv) if cv . ty . is_primitive () => { Some (cv . valtree . unwrap_leaf () . into ()) } _ => None , } , Const :: Val (val , _) => val . try_to_scalar () , Const :: Unevaluated (..) => None , } } # [inline] pub fn try_to_scalar_int (self) -> Option < ScalarInt > { match self { Const :: Val (ConstValue :: Scalar (Scalar :: Int (x)) , _) => Some (x) , Const :: Ty (_ , c) => match c . kind () { ty :: ConstKind :: Value (cv) if cv . ty . is_primitive () => Some (cv . valtree . unwrap_leaf ()) , _ => None , } , _ => None , } } # [inline] pub fn try_to_bits (self , size : Size) -> Option < u128 > { Some (self . try_to_scalar_int () ? . to_bits (size)) } # [inline] pub fn try_to_bool (self) -> Option < bool > { self . try_to_scalar_int () ? . try_into () . ok () } # [inline] pub fn eval (self , tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , span : Span ,) -> Result < ConstValue , ErrorHandled > { match self { Const :: Ty (_ , c) => { if c . has_non_region_param () { return Err (ErrorHandled :: TooGeneric (span)) ; } match c . kind () { ConstKind :: Value (cv) => Ok (tcx . valtree_to_const_val (cv)) , ConstKind :: Expr (_) => { bug ! ("Normalization of `ty::ConstKind::Expr` is unimplemented") } _ => Err (ReportedErrorInfo :: non_const_eval_error (tcx . dcx () . delayed_bug ("Unevaluated `ty::Const` in MIR body") ,) . into ()) , } } Const :: Unevaluated (uneval , _) => { tcx . const_eval_resolve (typing_env , uneval , span) } Const :: Val (val , _) => Ok (val) , } } # [inline] pub fn try_eval_scalar (self , tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > ,) -> Option < Scalar > { if let Const :: Ty (_ , c) = self && let ty :: ConstKind :: Value (cv) = c . kind () && cv . ty . is_primitive () { Some (cv . valtree . unwrap_leaf () . into ()) } else { self . eval (tcx , typing_env , DUMMY_SP) . ok () ? . try_to_scalar () } } # [inline] pub fn try_eval_scalar_int (self , tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > ,) -> Option < ScalarInt > { self . try_eval_scalar (tcx , typing_env) ? . try_to_scalar_int () . ok () } # [inline] pub fn try_eval_bits (& self , tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > ,) -> Option < u128 > { let int = self . try_eval_scalar_int (tcx , typing_env) ? ; let size = tcx . layout_of (typing_env . with_post_analysis_normalized (tcx) . as_query_input (self . ty ())) . ok () ? . size ; Some (int . to_bits (size)) } # [doc = " Panics if the value cannot be evaluated or doesn't contain a valid integer of the given type."] # [inline] pub fn eval_bits (self , tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx >) -> u128 { self . try_eval_bits (tcx , typing_env) . unwrap_or_else (| | bug ! ("expected bits of {:#?}, got {:#?}" , self . ty () , self)) } # [inline] pub fn try_eval_target_usize (self , tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > ,) -> Option < u64 > { Some (self . try_eval_scalar_int (tcx , typing_env) ? . to_target_usize (tcx)) } # [inline] # [doc = " Panics if the value cannot be evaluated or doesn't contain a valid `usize`."] pub fn eval_target_usize (self , tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx >) -> u64 { self . try_eval_target_usize (tcx , typing_env) . unwrap_or_else (| | bug ! ("expected usize, got {:#?}" , self)) } # [inline] pub fn try_eval_bool (self , tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx >) -> Option < bool > { self . try_eval_scalar_int (tcx , typing_env) ? . try_into () . ok () } # [inline] pub fn from_value (val : ConstValue , ty : Ty < 'tcx >) -> Self { Self :: Val (val , ty) } # [inline] pub fn from_ty_value (tcx : TyCtxt < 'tcx > , val : ty :: Value < 'tcx >) -> Self { Self :: Ty (val . ty , ty :: Const :: new_value (tcx , val . valtree , val . ty)) } pub fn from_bits (tcx : TyCtxt < 'tcx > , bits : u128 , typing_env : ty :: TypingEnv < 'tcx > , ty : Ty < 'tcx > ,) -> Self { let size = tcx . layout_of (typing_env . as_query_input (ty)) . unwrap_or_else (| e | bug ! ("could not compute layout for {ty:?}: {e:?}")) . size ; let cv = ConstValue :: Scalar (Scalar :: from_uint (bits , size)) ; Self :: Val (cv , ty) } # [inline] pub fn from_bool (tcx : TyCtxt < 'tcx > , v : bool) -> Self { let cv = ConstValue :: from_bool (v) ; Self :: Val (cv , tcx . types . bool) } # [inline] pub fn zero_sized (ty : Ty < 'tcx >) -> Self { let cv = ConstValue :: ZeroSized ; Self :: Val (cv , ty) } pub fn from_usize (tcx : TyCtxt < 'tcx > , n : u64) -> Self { let ty = tcx . types . usize ; let typing_env = ty :: TypingEnv :: fully_monomorphized () ; Self :: from_bits (tcx , n as u128 , typing_env , ty) } # [inline] pub fn from_scalar (_tcx : TyCtxt < 'tcx > , s : Scalar , ty : Ty < 'tcx >) -> Self { let val = ConstValue :: Scalar (s) ; Self :: Val (val , ty) } # [doc = " Return true if any evaluation of this constant always returns the same value,"] # [doc = " taking into account even pointer identity tests."] pub fn is_deterministic (& self) -> bool { match self { Const :: Ty (_ , c) => match c . kind () { ty :: ConstKind :: Param (..) => true , ty :: ConstKind :: Value (cv) => cv . ty . is_primitive () , ty :: ConstKind :: Unevaluated (..) | ty :: ConstKind :: Expr (..) => false , ty :: ConstKind :: Error (..) => false , ty :: ConstKind :: Infer (..) | ty :: ConstKind :: Bound (..) | ty :: ConstKind :: Placeholder (..) => bug ! () , } , Const :: Unevaluated (..) => false , Const :: Val (ConstValue :: Slice { .. } | ConstValue :: ZeroSized | ConstValue :: Scalar (_) | ConstValue :: Indirect { .. } , _ ,) => true , } } }}}
mkitem!{mkstruct!{# [doc = " An unevaluated (potentially generic) constant used in MIR."] # [derive (Copy , Clone , Debug , Eq , PartialEq , TyEncodable , TyDecodable)] # [derive (Hash , HashStable , TypeFoldable , TypeVisitable , Lift)] pub struct UnevaluatedConst < 'tcx > { pub def : DefId , pub args : GenericArgsRef < 'tcx > , pub promoted : Option < Promoted > , }}}
mkitem!{mkimpl!{impl < 'tcx > UnevaluatedConst < 'tcx > { # [inline] pub fn shrink (self) -> ty :: UnevaluatedConst < 'tcx > { assert_eq ! (self . promoted , None) ; ty :: UnevaluatedConst { def : self . def , args : self . args } } }}}
mkitem!{mkimpl!{impl < 'tcx > UnevaluatedConst < 'tcx > { # [inline] pub fn new (def : DefId , args : GenericArgsRef < 'tcx >) -> UnevaluatedConst < 'tcx > { UnevaluatedConst { def , args , promoted : Default :: default () } } # [inline] pub fn from_instance (instance : ty :: Instance < 'tcx >) -> Self { UnevaluatedConst :: new (instance . def_id () , instance . args) } }}}
mkitem!{mkimpl!{impl < 'tcx > Display for Const < 'tcx > { fn fmt (& self , fmt : & mut Formatter < '_ >) -> fmt :: Result { match * self { Const :: Ty (_ , c) => pretty_print_const (c , fmt , true) , Const :: Val (val , ty) => pretty_print_const_value (val , ty , fmt) , Const :: Unevaluated (c , _ty) => { ty :: tls :: with (move | tcx | { let c = tcx . lift (c) . unwrap () ; let instance = with_no_trimmed_paths ! (tcx . def_path_str_with_args (c . def , c . args)) ; write ! (fmt , "{instance}") ? ; if let Some (promoted) = c . promoted { write ! (fmt , "::{promoted:?}") ? ; } Ok (()) }) } } } }}}
mkitem!{mkimpl!{impl < 'tcx > TyCtxt < 'tcx > { pub fn span_as_caller_location (self , span : Span) -> ConstValue { let topmost = span . ctxt () . outer_expn () . expansion_cause () . unwrap_or (span) ; let caller = self . sess . source_map () . lookup_char_pos (topmost . lo ()) ; self . const_caller_location (Symbol :: intern (& caller . file . name . for_scope (self . sess , RemapPathScopeComponents :: MACRO) . to_string_lossy () ,) , caller . line as u32 , caller . col_display as u32 + 1 ,) } }}}