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
mkuse!{use itertools :: Either ;}
mkuse!{use rustc_abi as abi ;}
mkuse!{use rustc_abi :: { Align , BackendRepr , FIRST_VARIANT , FieldIdx , Primitive , Size , TagEncoding , VariantIdx , Variants , } ;}
mkuse!{use rustc_middle :: mir :: interpret :: { Pointer , Scalar , alloc_range } ;}
mkuse!{use rustc_middle :: mir :: { self , ConstValue } ;}
mkuse!{use rustc_middle :: ty :: Ty ;}
mkuse!{use rustc_middle :: ty :: layout :: { LayoutOf , TyAndLayout } ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_session :: config :: OptLevel ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use super :: place :: { PlaceRef , PlaceValue } ;}
mkuse!{use super :: rvalue :: transmute_scalar ;}
mkuse!{use super :: { FunctionCx , LocalRef } ;}
mkuse!{use crate :: MemFlags ;}
mkuse!{use crate :: common :: IntPredicate ;}
mkuse!{use crate :: traits :: * ;}
mkitem!{mkenum!{# [doc = " The representation of a Rust value. The enum variant is in fact"] # [doc = " uniquely determined by the value's type, but is kept as a"] # [doc = " safety check."] # [derive (Copy , Clone , Debug)] pub enum OperandValue < V > { # [doc = " A reference to the actual operand. The data is guaranteed"] # [doc = " to be valid for the operand's lifetime."] # [doc = " The second value, if any, is the extra data (vtable or length)"] # [doc = " which indicates that it refers to an unsized rvalue."] # [doc = ""] # [doc = " An `OperandValue` *must* be this variant for any type for which"] # [doc = " [`LayoutTypeCodegenMethods::is_backend_ref`] returns `true`."] # [doc = " (That basically amounts to \"isn't one of the other variants\".)"] # [doc = ""] # [doc = " This holds a [`PlaceValue`] (like a [`PlaceRef`] does) with a pointer"] # [doc = " to the location holding the value. The type behind that pointer is the"] # [doc = " one returned by [`LayoutTypeCodegenMethods::backend_type`]."] Ref (PlaceValue < V >) , # [doc = " A single LLVM immediate value."] # [doc = ""] # [doc = " An `OperandValue` *must* be this variant for any type for which"] # [doc = " [`LayoutTypeCodegenMethods::is_backend_immediate`] returns `true`."] # [doc = " The backend value in this variant must be the *immediate* backend type,"] # [doc = " as returned by [`LayoutTypeCodegenMethods::immediate_backend_type`]."] Immediate (V) , # [doc = " A pair of immediate LLVM values. Used by wide pointers too."] # [doc = ""] # [doc = " # Invariants"] # [doc = " - For `Pair(a, b)`, `a` is always at offset 0, but may have `FieldIdx(1..)`"] # [doc = " - `b` is not at offset 0, because `V` is not a 1ZST type."] # [doc = " - `a` and `b` will have a different FieldIdx, but otherwise `b`'s may be lower"] # [doc = "   or they may not be adjacent, due to arbitrary numbers of 1ZST fields that"] # [doc = "   will not affect the shape of the data which determines if `Pair` will be used."] # [doc = " - An `OperandValue` *must* be this variant for any type for which"] # [doc = " [`LayoutTypeCodegenMethods::is_backend_scalar_pair`] returns `true`."] # [doc = " - The backend values in this variant must be the *immediate* backend types,"] # [doc = " as returned by [`LayoutTypeCodegenMethods::scalar_pair_element_backend_type`]"] # [doc = " with `immediate: true`."] Pair (V , V) , # [doc = " A value taking no bytes, and which therefore needs no LLVM value at all."] # [doc = ""] # [doc = " If you ever need a `V` to pass to something, get a fresh poison value"] # [doc = " from [`ConstCodegenMethods::const_poison`]."] # [doc = ""] # [doc = " An `OperandValue` *must* be this variant for any type for which"] # [doc = " `is_zst` on its `Layout` returns `true`. Note however that"] # [doc = " these values can still require alignment."] ZeroSized , }}}
mkitem!{mkimpl!{impl < V : CodegenObject > OperandValue < V > { # [doc = " Treat this value as a pointer and return the data pointer and"] # [doc = " optional metadata as backend values."] # [doc = ""] # [doc = " If you're making a place, use [`Self::deref`] instead."] pub (crate) fn pointer_parts (self) -> (V , Option < V >) { match self { OperandValue :: Immediate (llptr) => (llptr , None) , OperandValue :: Pair (llptr , llextra) => (llptr , Some (llextra)) , _ => bug ! ("OperandValue cannot be a pointer: {self:?}") , } } # [doc = " Treat this value as a pointer and return the place to which it points."] # [doc = ""] # [doc = " The pointer immediate doesn't inherently know its alignment,"] # [doc = " so you need to pass it in. If you want to get it from a type's ABI"] # [doc = " alignment, then maybe you want [`OperandRef::deref`] instead."] # [doc = ""] # [doc = " This is the inverse of [`PlaceValue::address`]."] pub (crate) fn deref (self , align : Align) -> PlaceValue < V > { let (llval , llextra) = self . pointer_parts () ; PlaceValue { llval , llextra , align } } pub (crate) fn is_expected_variant_for_type < 'tcx , Cx : LayoutTypeCodegenMethods < 'tcx > > (& self , cx : & Cx , ty : TyAndLayout < 'tcx > ,) -> bool { match self { OperandValue :: ZeroSized => ty . is_zst () , OperandValue :: Immediate (_) => cx . is_backend_immediate (ty) , OperandValue :: Pair (_ , _) => cx . is_backend_scalar_pair (ty) , OperandValue :: Ref (_) => cx . is_backend_ref (ty) , } } }}}
mkitem!{mkstruct!{# [doc = " An `OperandRef` is an \"SSA\" reference to a Rust value, along with"] # [doc = " its type."] # [doc = ""] # [doc = " NOTE: unless you know a value's type exactly, you should not"] # [doc = " generate LLVM opcodes acting on it and instead act via methods,"] # [doc = " to avoid nasty edge cases. In particular, using `Builder::store`"] # [doc = " directly is sure to cause problems -- use `OperandRef::store`"] # [doc = " instead."] # [derive (Copy , Clone)] pub struct OperandRef < 'tcx , V > { # [doc = " The value."] pub val : OperandValue < V > , # [doc = " The layout of value, based on its Rust type."] pub layout : TyAndLayout < 'tcx > , }}}
mkitem!{mkimpl!{impl < V : CodegenObject > fmt :: Debug for OperandRef < '_ , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "OperandRef({:?} @ {:?})" , self . val , self . layout) } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx , V : CodegenObject > OperandRef < 'tcx , V > { pub fn zero_sized (layout : TyAndLayout < 'tcx >) -> OperandRef < 'tcx , V > { assert ! (layout . is_zst ()) ; OperandRef { val : OperandValue :: ZeroSized , layout } } pub (crate) fn from_const < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (bx : & mut Bx , val : mir :: ConstValue , ty : Ty < 'tcx > ,) -> Self { let layout = bx . layout_of (ty) ; let val = match val { ConstValue :: Scalar (x) => { let BackendRepr :: Scalar (scalar) = layout . backend_repr else { bug ! ("from_const: invalid ByVal layout: {:#?}" , layout) ; } ; let llval = bx . scalar_to_backend (x , scalar , bx . immediate_backend_type (layout)) ; OperandValue :: Immediate (llval) } ConstValue :: ZeroSized => return OperandRef :: zero_sized (layout) , ConstValue :: Slice { alloc_id , meta } => { let BackendRepr :: ScalarPair (a_scalar , _) = layout . backend_repr else { bug ! ("from_const: invalid ScalarPair layout: {:#?}" , layout) ; } ; let a = Scalar :: from_pointer (Pointer :: new (alloc_id . into () , Size :: ZERO) , & bx . tcx ()) ; let a_llval = bx . scalar_to_backend (a , a_scalar , bx . scalar_pair_element_backend_type (layout , 0 , true) ,) ; let b_llval = bx . const_usize (meta) ; OperandValue :: Pair (a_llval , b_llval) } ConstValue :: Indirect { alloc_id , offset } => { let alloc = bx . tcx () . global_alloc (alloc_id) . unwrap_memory () ; return Self :: from_const_alloc (bx , layout , alloc , offset) ; } } ; OperandRef { val , layout } } fn from_const_alloc < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (bx : & mut Bx , layout : TyAndLayout < 'tcx > , alloc : rustc_middle :: mir :: interpret :: ConstAllocation < 'tcx > , offset : Size ,) -> Self { let alloc_align = alloc . inner () . align ; assert ! (alloc_align >= layout . align . abi , "{alloc_align:?} < {:?}" , layout . align . abi) ; let read_scalar = | start , size , s : abi :: Scalar , ty | { match alloc . 0 . read_scalar (bx , alloc_range (start , size) , matches ! (s . primitive () , abi :: Primitive :: Pointer (_)) ,) { Ok (val) => bx . scalar_to_backend (val , s , ty) , Err (_) => bx . const_poison (ty) , } } ; match layout . backend_repr { BackendRepr :: Scalar (s @ abi :: Scalar :: Initialized { .. }) => { let size = s . size (bx) ; assert_eq ! (size , layout . size , "abi::Scalar size does not match layout size") ; let val = read_scalar (offset , size , s , bx . immediate_backend_type (layout)) ; OperandRef { val : OperandValue :: Immediate (val) , layout } } BackendRepr :: ScalarPair (a @ abi :: Scalar :: Initialized { .. } , b @ abi :: Scalar :: Initialized { .. } ,) => { let (a_size , b_size) = (a . size (bx) , b . size (bx)) ; let b_offset = (offset + a_size) . align_to (b . align (bx) . abi) ; assert ! (b_offset . bytes () > 0) ; let a_val = read_scalar (offset , a_size , a , bx . scalar_pair_element_backend_type (layout , 0 , true) ,) ; let b_val = read_scalar (b_offset , b_size , b , bx . scalar_pair_element_backend_type (layout , 1 , true) ,) ; OperandRef { val : OperandValue :: Pair (a_val , b_val) , layout } } _ if layout . is_zst () => OperandRef :: zero_sized (layout) , _ => { let init = bx . const_data_from_alloc (alloc) ; let base_addr = bx . static_addr_of (init , alloc_align , None) ; let llval = bx . const_ptr_byte_offset (base_addr , offset) ; bx . load_operand (PlaceRef :: new_sized (llval , layout)) } } } # [doc = " Asserts that this operand refers to a scalar and returns"] # [doc = " a reference to its value."] pub fn immediate (self) -> V { match self . val { OperandValue :: Immediate (s) => s , _ => bug ! ("not immediate: {:?}" , self) , } } # [doc = " Asserts that this operand is a pointer (or reference) and returns"] # [doc = " the place to which it points.  (This requires no code to be emitted"] # [doc = " as we represent places using the pointer to the place.)"] # [doc = ""] # [doc = " This uses [`Ty::builtin_deref`] to include the type of the place and"] # [doc = " assumes the place is aligned to the pointee's usual ABI alignment."] # [doc = ""] # [doc = " If you don't need the type, see [`OperandValue::pointer_parts`]"] # [doc = " or [`OperandValue::deref`]."] pub fn deref < Cx : CodegenMethods < 'tcx > > (self , cx : & Cx) -> PlaceRef < 'tcx , V > { if self . layout . ty . is_box () { bug ! ("dereferencing {:?} in codegen" , self . layout . ty) ; } let projected_ty = self . layout . ty . builtin_deref (true) . unwrap_or_else (| | bug ! ("deref of non-pointer {:?}" , self)) ; let layout = cx . layout_of (projected_ty) ; self . val . deref (layout . align . abi) . with_type (layout) } # [doc = " If this operand is a `Pair`, we return an aggregate with the two values."] # [doc = " For other cases, see `immediate`."] pub fn immediate_or_packed_pair < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (self , bx : & mut Bx ,) -> V { if let OperandValue :: Pair (a , b) = self . val { let llty = bx . cx () . immediate_backend_type (self . layout) ; debug ! ("Operand::immediate_or_packed_pair: packing {:?} into {:?}" , self , llty) ; let mut llpair = bx . cx () . const_poison (llty) ; llpair = bx . insert_value (llpair , a , 0) ; llpair = bx . insert_value (llpair , b , 1) ; llpair } else { self . immediate () } } # [doc = " If the type is a pair, we return a `Pair`, otherwise, an `Immediate`."] pub fn from_immediate_or_packed_pair < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (bx : & mut Bx , llval : V , layout : TyAndLayout < 'tcx > ,) -> Self { let val = if let BackendRepr :: ScalarPair (..) = layout . backend_repr { debug ! ("Operand::from_immediate_or_packed_pair: unpacking {:?} @ {:?}" , llval , layout) ; let a_llval = bx . extract_value (llval , 0) ; let b_llval = bx . extract_value (llval , 1) ; OperandValue :: Pair (a_llval , b_llval) } else { OperandValue :: Immediate (llval) } ; OperandRef { val , layout } } pub (crate) fn extract_field < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (& self , fx : & mut FunctionCx < 'a , 'tcx , Bx > , bx : & mut Bx , i : usize ,) -> Self { let field = self . layout . field (bx . cx () , i) ; let offset = self . layout . fields . offset (i) ; if ! bx . is_backend_ref (self . layout) && bx . is_backend_ref (field) { span_bug ! (fx . mir . span , "Non-ref type {self:?} cannot project to ref field type {field:?}" ,) ; } let val = if field . is_zst () { OperandValue :: ZeroSized } else if field . size == self . layout . size { assert_eq ! (offset . bytes () , 0) ; fx . codegen_transmute_operand (bx , * self , field) } else { let (in_scalar , imm) = match (self . val , self . layout . backend_repr) { (OperandValue :: Pair (a_llval , b_llval) , BackendRepr :: ScalarPair (a , b)) => { if offset . bytes () == 0 { assert_eq ! (field . size , a . size (bx . cx ())) ; (Some (a) , a_llval) } else { assert_eq ! (offset , a . size (bx . cx ()) . align_to (b . align (bx . cx ()) . abi)) ; assert_eq ! (field . size , b . size (bx . cx ())) ; (Some (b) , b_llval) } } _ => { span_bug ! (fx . mir . span , "OperandRef::extract_field({:?}): not applicable" , self) } } ; OperandValue :: Immediate (match field . backend_repr { BackendRepr :: SimdVector { .. } => imm , BackendRepr :: Scalar (out_scalar) => { let Some (in_scalar) = in_scalar else { span_bug ! (fx . mir . span , "OperandRef::extract_field({:?}): missing input scalar for output scalar" , self) } ; if in_scalar != out_scalar { let backend = bx . from_immediate (imm) ; bx . to_immediate_scalar (backend , out_scalar) } else { imm } } BackendRepr :: ScalarPair (_ , _) | BackendRepr :: Memory { .. } => bug ! () , }) } ; OperandRef { val , layout : field } } # [doc = " Obtain the actual discriminant of a value."] # [instrument (level = "trace" , skip (fx , bx))] pub fn codegen_get_discr < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (self , fx : & mut FunctionCx < 'a , 'tcx , Bx > , bx : & mut Bx , cast_to : Ty < 'tcx > ,) -> V { let dl = & bx . tcx () . data_layout ; let cast_to_layout = bx . cx () . layout_of (cast_to) ; let cast_to = bx . cx () . immediate_backend_type (cast_to_layout) ; if self . layout . is_uninhabited () { return bx . cx () . const_poison (cast_to) ; } let (tag_scalar , tag_encoding , tag_field) = match self . layout . variants { Variants :: Empty => unreachable ! ("we already handled uninhabited types") , Variants :: Single { index } => { let discr_val = if let Some (discr) = self . layout . ty . discriminant_for_variant (bx . tcx () , index) { discr . val } else { assert_eq ! (index , FIRST_VARIANT) ; 0 } ; return bx . cx () . const_uint_big (cast_to , discr_val) ; } Variants :: Multiple { tag , ref tag_encoding , tag_field , .. } => { (tag , tag_encoding , tag_field) } } ; let tag_op = match self . val { OperandValue :: ZeroSized => bug ! () , OperandValue :: Immediate (_) | OperandValue :: Pair (_ , _) => { self . extract_field (fx , bx , tag_field . as_usize ()) } OperandValue :: Ref (place) => { let tag = place . with_type (self . layout) . project_field (bx , tag_field . as_usize ()) ; bx . load_operand (tag) } } ; let tag_imm = tag_op . immediate () ; match * tag_encoding { TagEncoding :: Direct => { let signed = match tag_scalar . primitive () { Primitive :: Int (_ , signed) => ! tag_scalar . is_bool () && signed , _ => false , } ; bx . intcast (tag_imm , cast_to , signed) } TagEncoding :: Niche { untagged_variant , ref niche_variants , niche_start } => { let (tag , tag_llty) = match tag_scalar . primitive () { Primitive :: Pointer (_) => { let t = bx . type_from_integer (dl . ptr_sized_integer ()) ; let tag = bx . ptrtoint (tag_imm , t) ; (tag , t) } _ => (tag_imm , bx . cx () . immediate_backend_type (tag_op . layout)) , } ; let relative_max = niche_variants . end () . as_u32 () - niche_variants . start () . as_u32 () ; let niche_start_const = bx . cx () . const_uint_big (tag_llty , niche_start) ; let (is_niche , tagged_discr , delta) = if relative_max == 0 { let is_niche = bx . icmp (IntPredicate :: IntEQ , tag , niche_start_const) ; let tagged_discr = bx . cx () . const_uint (cast_to , niche_variants . start () . as_u32 () as u64) ; (is_niche , tagged_discr , 0) } else { if niche_variants . contains (& untagged_variant) && bx . cx () . sess () . opts . optimize != OptLevel :: No { let impossible = niche_start . wrapping_add (u128 :: from (untagged_variant . as_u32 ())) . wrapping_sub (u128 :: from (niche_variants . start () . as_u32 ())) ; let impossible = bx . cx () . const_uint_big (tag_llty , impossible) ; let ne = bx . icmp (IntPredicate :: IntNE , tag , impossible) ; bx . assume (ne) ; } let tag_range = tag_scalar . valid_range (& dl) ; let tag_size = tag_scalar . size (& dl) ; let niche_end = u128 :: from (relative_max) . wrapping_add (niche_start) ; let niche_end = tag_size . truncate (niche_end) ; let relative_discr = bx . sub (tag , niche_start_const) ; let cast_tag = bx . intcast (relative_discr , cast_to , false) ; let is_niche = if tag_range . no_unsigned_wraparound (tag_size) == Ok (true) { if niche_start == tag_range . start { let niche_end_const = bx . cx () . const_uint_big (tag_llty , niche_end) ; bx . icmp (IntPredicate :: IntULE , tag , niche_end_const) } else { assert_eq ! (niche_end , tag_range . end) ; bx . icmp (IntPredicate :: IntUGE , tag , niche_start_const) } } else if tag_range . no_signed_wraparound (tag_size) == Ok (true) { if niche_start == tag_range . start { let niche_end_const = bx . cx () . const_uint_big (tag_llty , niche_end) ; bx . icmp (IntPredicate :: IntSLE , tag , niche_end_const) } else { assert_eq ! (niche_end , tag_range . end) ; bx . icmp (IntPredicate :: IntSGE , tag , niche_start_const) } } else { bx . icmp (IntPredicate :: IntULE , relative_discr , bx . cx () . const_uint (tag_llty , relative_max as u64) ,) } ; (is_niche , cast_tag , niche_variants . start () . as_u32 () as u128) } ; let tagged_discr = if delta == 0 { tagged_discr } else { bx . add (tagged_discr , bx . cx () . const_uint_big (cast_to , delta)) } ; let untagged_variant_const = bx . cx () . const_uint (cast_to , u64 :: from (untagged_variant . as_u32 ())) ; let discr = bx . select (is_niche , tagged_discr , untagged_variant_const) ; discr } } } }}}
mkitem!{mkenum!{# [doc = " Each of these variants starts out as `Either::Right` when it's uninitialized,"] # [doc = " then setting the field changes that to `Either::Left` with the backend value."] # [derive (Debug , Copy , Clone)] enum OperandValueBuilder < V > { ZeroSized , Immediate (Either < V , abi :: Scalar >) , Pair (Either < V , abi :: Scalar > , Either < V , abi :: Scalar >) , # [doc = " `repr(simd)` types need special handling because they each have a non-empty"] # [doc = " array field (which uses [`OperandValue::Ref`]) despite the SIMD type itself"] # [doc = " using [`OperandValue::Immediate`] which for any other kind of type would"] # [doc = " mean that its one non-ZST field would also be [`OperandValue::Immediate`]."] Vector (Either < V , () >) , }}}
mkitem!{mkstruct!{# [doc = " Allows building up an `OperandRef` by setting fields one at a time."] # [derive (Debug , Copy , Clone)] pub (super) struct OperandRefBuilder < 'tcx , V > { val : OperandValueBuilder < V > , layout : TyAndLayout < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx , V : CodegenObject > OperandRefBuilder < 'tcx , V > { # [doc = " Creates an uninitialized builder for an instance of the `layout`."] # [doc = ""] # [doc = " ICEs for [`BackendRepr::Memory`] types (other than ZSTs), which should"] # [doc = " be built up inside a [`PlaceRef`] instead as they need an allocated place"] # [doc = " into which to write the values of the fields."] pub (super) fn new (layout : TyAndLayout < 'tcx >) -> Self { let val = match layout . backend_repr { BackendRepr :: Memory { .. } if layout . is_zst () => OperandValueBuilder :: ZeroSized , BackendRepr :: Scalar (s) => OperandValueBuilder :: Immediate (Either :: Right (s)) , BackendRepr :: ScalarPair (a , b) => { OperandValueBuilder :: Pair (Either :: Right (a) , Either :: Right (b)) } BackendRepr :: SimdVector { .. } => OperandValueBuilder :: Vector (Either :: Right (())) , BackendRepr :: Memory { .. } => { bug ! ("Cannot use non-ZST Memory-ABI type in operand builder: {layout:?}") ; } } ; OperandRefBuilder { val , layout } } pub (super) fn insert_field < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (& mut self , bx : & mut Bx , variant : VariantIdx , field : FieldIdx , field_operand : OperandRef < 'tcx , V > ,) { if let OperandValue :: ZeroSized = field_operand . val { return ; } let is_zero_offset = if let abi :: FieldsShape :: Primitive = self . layout . fields { assert ! (! self . layout . is_zst ()) ; assert_eq ! (variant , FIRST_VARIANT) ; assert_eq ! (field , FieldIdx :: ZERO) ; true } else { let variant_layout = self . layout . for_variant (bx . cx () , variant) ; let field_offset = variant_layout . fields . offset (field . as_usize ()) ; field_offset == Size :: ZERO } ; let mut update = | tgt : & mut Either < V , abi :: Scalar > , src , from_scalar | { let to_scalar = tgt . unwrap_right () ; let imm = transmute_scalar (bx , src , from_scalar , to_scalar) ; * tgt = Either :: Left (imm) ; } ; match (field_operand . val , field_operand . layout . backend_repr) { (OperandValue :: ZeroSized , _) => unreachable ! ("Handled above") , (OperandValue :: Immediate (v) , BackendRepr :: Scalar (from_scalar)) => match & mut self . val { OperandValueBuilder :: Immediate (val @ Either :: Right (_)) if is_zero_offset => { update (val , v , from_scalar) ; } OperandValueBuilder :: Pair (fst @ Either :: Right (_) , _) if is_zero_offset => { update (fst , v , from_scalar) ; } OperandValueBuilder :: Pair (_ , snd @ Either :: Right (_)) if ! is_zero_offset => { update (snd , v , from_scalar) ; } _ => { bug ! ("Tried to insert {field_operand:?} into {variant:?}.{field:?} of {self:?}") } } , (OperandValue :: Immediate (v) , BackendRepr :: SimdVector { .. }) => match & mut self . val { OperandValueBuilder :: Vector (val @ Either :: Right (())) if is_zero_offset => { * val = Either :: Left (v) ; } _ => { bug ! ("Tried to insert {field_operand:?} into {variant:?}.{field:?} of {self:?}") } } , (OperandValue :: Pair (a , b) , BackendRepr :: ScalarPair (from_sa , from_sb)) => { match & mut self . val { OperandValueBuilder :: Pair (fst @ Either :: Right (_) , snd @ Either :: Right (_)) => { update (fst , a , from_sa) ; update (snd , b , from_sb) ; } _ => bug ! ("Tried to insert {field_operand:?} into {variant:?}.{field:?} of {self:?}") , } } (OperandValue :: Ref (place) , BackendRepr :: Memory { .. }) => match & mut self . val { OperandValueBuilder :: Vector (val @ Either :: Right (())) => { let ibty = bx . cx () . immediate_backend_type (self . layout) ; let simd = bx . load_from_place (ibty , place) ; * val = Either :: Left (simd) ; } _ => { bug ! ("Tried to insert {field_operand:?} into {variant:?}.{field:?} of {self:?}") } } , _ => bug ! ("Operand cannot be used with `insert_field`: {field_operand:?}") , } } # [doc = " Insert the immediate value `imm` for field `f` in the *type itself*,"] # [doc = " rather than into one of the variants."] # [doc = ""] # [doc = " Most things want [`Self::insert_field`] instead, but this one is"] # [doc = " necessary for writing things like enum tags that aren't in any variant."] pub (super) fn insert_imm (& mut self , f : FieldIdx , imm : V) { let field_offset = self . layout . fields . offset (f . as_usize ()) ; let is_zero_offset = field_offset == Size :: ZERO ; match & mut self . val { OperandValueBuilder :: Immediate (val @ Either :: Right (_)) if is_zero_offset => { * val = Either :: Left (imm) ; } OperandValueBuilder :: Pair (fst @ Either :: Right (_) , _) if is_zero_offset => { * fst = Either :: Left (imm) ; } OperandValueBuilder :: Pair (_ , snd @ Either :: Right (_)) if ! is_zero_offset => { * snd = Either :: Left (imm) ; } _ => bug ! ("Tried to insert {imm:?} into field {f:?} of {self:?}") , } } # [doc = " After having set all necessary fields, this converts the builder back"] # [doc = " to the normal `OperandRef`."] # [doc = ""] # [doc = " ICEs if any required fields were not set."] pub (super) fn build (& self , cx : & impl CodegenMethods < 'tcx , Value = V >) -> OperandRef < 'tcx , V > { let OperandRefBuilder { val , layout } = * self ; let unwrap = | r : Either < V , abi :: Scalar > | match r { Either :: Left (v) => v , Either :: Right (s) if s . is_uninit_valid () => { let bty = cx . type_from_scalar (s) ; cx . const_undef (bty) } Either :: Right (_) => bug ! ("OperandRef::build called while fields are missing {self:?}") , } ; let val = match val { OperandValueBuilder :: ZeroSized => OperandValue :: ZeroSized , OperandValueBuilder :: Immediate (v) => OperandValue :: Immediate (unwrap (v)) , OperandValueBuilder :: Pair (a , b) => OperandValue :: Pair (unwrap (a) , unwrap (b)) , OperandValueBuilder :: Vector (v) => match v { Either :: Left (v) => OperandValue :: Immediate (v) , Either :: Right (()) if let BackendRepr :: SimdVector { element , .. } = layout . backend_repr && element . is_uninit_valid () => { let bty = cx . immediate_backend_type (layout) ; OperandValue :: Immediate (cx . const_undef (bty)) } Either :: Right (()) => { bug ! ("OperandRef::build called while fields are missing {self:?}") } } , } ; OperandRef { val , layout } } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx , V : CodegenObject > OperandValue < V > { # [doc = " Returns an `OperandValue` that's generally UB to use in any way."] # [doc = ""] # [doc = " Depending on the `layout`, returns `ZeroSized` for ZSTs, an `Immediate` or"] # [doc = " `Pair` containing poison value(s), or a `Ref` containing a poison pointer."] # [doc = ""] # [doc = " Supports sized types only."] pub fn poison < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (bx : & mut Bx , layout : TyAndLayout < 'tcx > ,) -> OperandValue < V > { assert ! (layout . is_sized ()) ; if layout . is_zst () { OperandValue :: ZeroSized } else if bx . cx () . is_backend_immediate (layout) { let ibty = bx . cx () . immediate_backend_type (layout) ; OperandValue :: Immediate (bx . const_poison (ibty)) } else if bx . cx () . is_backend_scalar_pair (layout) { let ibty0 = bx . cx () . scalar_pair_element_backend_type (layout , 0 , true) ; let ibty1 = bx . cx () . scalar_pair_element_backend_type (layout , 1 , true) ; OperandValue :: Pair (bx . const_poison (ibty0) , bx . const_poison (ibty1)) } else { let ptr = bx . cx () . type_ptr () ; OperandValue :: Ref (PlaceValue :: new_sized (bx . const_poison (ptr) , layout . align . abi)) } } pub fn store < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (self , bx : & mut Bx , dest : PlaceRef < 'tcx , V > ,) { self . store_with_flags (bx , dest , MemFlags :: empty ()) ; } pub fn volatile_store < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (self , bx : & mut Bx , dest : PlaceRef < 'tcx , V > ,) { self . store_with_flags (bx , dest , MemFlags :: VOLATILE) ; } pub fn unaligned_volatile_store < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (self , bx : & mut Bx , dest : PlaceRef < 'tcx , V > ,) { self . store_with_flags (bx , dest , MemFlags :: VOLATILE | MemFlags :: UNALIGNED) ; } pub fn nontemporal_store < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (self , bx : & mut Bx , dest : PlaceRef < 'tcx , V > ,) { self . store_with_flags (bx , dest , MemFlags :: NONTEMPORAL) ; } pub (crate) fn store_with_flags < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (self , bx : & mut Bx , dest : PlaceRef < 'tcx , V > , flags : MemFlags ,) { debug ! ("OperandRef::store: operand={:?}, dest={:?}" , self , dest) ; match self { OperandValue :: ZeroSized => { } OperandValue :: Ref (val) => { assert ! (dest . layout . is_sized () , "cannot directly store unsized values") ; if val . llextra . is_some () { bug ! ("cannot directly store unsized values") ; } bx . typed_place_copy_with_flags (dest . val , val , dest . layout , flags) ; } OperandValue :: Immediate (s) => { let val = bx . from_immediate (s) ; bx . store_with_flags (val , dest . val . llval , dest . val . align , flags) ; } OperandValue :: Pair (a , b) => { let BackendRepr :: ScalarPair (a_scalar , b_scalar) = dest . layout . backend_repr else { bug ! ("store_with_flags: invalid ScalarPair layout: {:#?}" , dest . layout) ; } ; let b_offset = a_scalar . size (bx) . align_to (b_scalar . align (bx) . abi) ; let val = bx . from_immediate (a) ; let align = dest . val . align ; bx . store_with_flags (val , dest . val . llval , align , flags) ; let llptr = bx . inbounds_ptradd (dest . val . llval , bx . const_usize (b_offset . bytes ())) ; let val = bx . from_immediate (b) ; let align = dest . val . align . restrict_for_offset (b_offset) ; bx . store_with_flags (val , llptr , align , flags) ; } } } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > FunctionCx < 'a , 'tcx , Bx > { fn maybe_codegen_consume_direct (& mut self , bx : & mut Bx , place_ref : mir :: PlaceRef < 'tcx > ,) -> Option < OperandRef < 'tcx , Bx :: Value > > { debug ! ("maybe_codegen_consume_direct(place_ref={:?})" , place_ref) ; match self . locals [place_ref . local] { LocalRef :: Operand (mut o) => { for elem in place_ref . projection { match * elem { mir :: ProjectionElem :: Field (f , _) => { assert ! (! o . layout . ty . is_any_ptr () , "Bad PlaceRef: destructing pointers should use cast/PtrMetadata, \
                                 but tried to access field {f:?} of pointer {o:?}" ,) ; o = o . extract_field (self , bx , f . index ()) ; } mir :: PlaceElem :: Downcast (_ , vidx) => { debug_assert_eq ! (o . layout . variants , abi :: Variants :: Single { index : vidx } ,) ; let layout = o . layout . for_variant (bx . cx () , vidx) ; o = OperandRef { val : o . val , layout } } mir :: PlaceElem :: Subtype (subtype_ty) => { let subtype_ty = self . monomorphize (subtype_ty) ; let layout = self . cx . layout_of (subtype_ty) ; o = OperandRef { val : o . val , layout } } _ => return None , } } Some (o) } LocalRef :: PendingOperand => { bug ! ("use of {:?} before def" , place_ref) ; } LocalRef :: Place (..) | LocalRef :: UnsizedPlace (..) => { None } } } pub fn codegen_consume (& mut self , bx : & mut Bx , place_ref : mir :: PlaceRef < 'tcx > ,) -> OperandRef < 'tcx , Bx :: Value > { debug ! ("codegen_consume(place_ref={:?})" , place_ref) ; let ty = self . monomorphized_place_ty (place_ref) ; let layout = bx . cx () . layout_of (ty) ; if layout . is_zst () { return OperandRef :: zero_sized (layout) ; } if let Some (o) = self . maybe_codegen_consume_direct (bx , place_ref) { return o ; } let place = self . codegen_place (bx , place_ref) ; bx . load_operand (place) } pub fn codegen_operand (& mut self , bx : & mut Bx , operand : & mir :: Operand < 'tcx > ,) -> OperandRef < 'tcx , Bx :: Value > { debug ! ("codegen_operand(operand={:?})" , operand) ; match * operand { mir :: Operand :: Copy (ref place) | mir :: Operand :: Move (ref place) => { self . codegen_consume (bx , place . as_ref ()) } mir :: Operand :: Constant (ref constant) => { let constant_ty = self . monomorphize (constant . ty ()) ; if constant_ty . is_simd () { let layout = bx . layout_of (constant_ty) ; if let BackendRepr :: SimdVector { .. } = layout . backend_repr { let (llval , ty) = self . immediate_const_vector (bx , constant) ; return OperandRef { val : OperandValue :: Immediate (llval) , layout : bx . layout_of (ty) , } ; } } self . eval_mir_constant_to_operand (bx , constant) } } } }}}