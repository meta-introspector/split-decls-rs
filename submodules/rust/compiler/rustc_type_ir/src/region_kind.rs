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
mkuse!{use derive_where :: derive_where ;}
mkuse!{# [cfg (feature = "nightly")] use rustc_data_structures :: stable_hasher :: { HashStable , StableHasher } ;}
mkuse!{# [cfg (feature = "nightly")] use rustc_macros :: { Decodable_NoContext , Encodable_NoContext , HashStable_NoContext } ;}
mkuse!{use self :: RegionKind :: * ;}
mkuse!{use crate :: { DebruijnIndex , Interner } ;}
mkitem!{rustc_index :: newtype_index ! { # [doc = " A **region** **v**ariable **ID**."] # [encodable] # [orderable] # [debug_format = "'?{}"] # [gate_rustc_only] # [cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] pub struct RegionVid { } }}
mkitem!{mkenum!{# [doc = " Representation of regions. Note that the NLL checker uses a distinct"] # [doc = " representation of regions. For this reason, it internally replaces all the"] # [doc = " regions with inference variables -- the index of the variable is then used"] # [doc = " to index into internal NLL data structures. See `rustc_const_eval::borrow_check`"] # [doc = " module for more information."] # [doc = ""] # [doc = " Note: operations are on the wrapper `Region` type, which is interned,"] # [doc = " rather than this type."] # [doc = ""] # [doc = " ## The Region lattice within a given function"] # [doc = ""] # [doc = " In general, the region lattice looks like"] # [doc = ""] # [doc = " ```text"] # [doc = " static ----------+-----...------+       (greatest)"] # [doc = " |                |              |"] # [doc = " param regions    |              |"] # [doc = " |                |              |"] # [doc = " |                |              |"] # [doc = " |                |              |"] # [doc = " empty(root)   placeholder(U1)   |"] # [doc = " |            /                  |"] # [doc = " |           /         placeholder(Un)"] # [doc = " empty(U1) --         /"] # [doc = " |                   /"] # [doc = " ...                /"] # [doc = " |                 /"] # [doc = " empty(Un) --------                      (smallest)"] # [doc = " ```"] # [doc = ""] # [doc = " Early-bound/free regions are the named lifetimes in scope from the"] # [doc = " function declaration. They have relationships to one another"] # [doc = " determined based on the declared relationships from the"] # [doc = " function."] # [doc = ""] # [doc = " Note that inference variables and bound regions are not included"] # [doc = " in this diagram. In the case of inference variables, they should"] # [doc = " be inferred to some other region from the diagram. In the case of"] # [doc = " bound regions, they are excluded because they don't make sense to"] # [doc = " include -- the diagram indicates the relationship between free"] # [doc = " regions."] # [doc = ""] # [doc = " ## Inference variables"] # [doc = ""] # [doc = " During region inference, we sometimes create inference variables,"] # [doc = " represented as `ReVar`. These will be inferred by the code in"] # [doc = " `infer::lexical_region_resolve` to some free region from the"] # [doc = " lattice above (the minimal region that meets the"] # [doc = " constraints)."] # [doc = ""] # [doc = " During NLL checking, where regions are defined differently, we"] # [doc = " also use `ReVar` -- in that case, the index is used to index into"] # [doc = " the NLL region checker's data structures. The variable may in fact"] # [doc = " represent either a free region or an inference variable, in that"] # [doc = " case."] # [doc = ""] # [doc = " ## Bound Regions"] # [doc = ""] # [doc = " These are regions that are stored behind a binder and must be instantiated"] # [doc = " with some concrete region before being used. There are two kind of"] # [doc = " bound regions: early-bound, which are bound in an item's `Generics`,"] # [doc = " and are instantiated by an `GenericArgs`, and late-bound, which are part of"] # [doc = " higher-ranked types (e.g., `for<'a> fn(&'a ())`), and are instantiated by"] # [doc = " the likes of `liberate_late_bound_regions`. The distinction exists"] # [doc = " because higher-ranked lifetimes aren't supported in all places. See [1][2]."] # [doc = ""] # [doc = " Unlike `Param`s, bound regions are not supposed to exist \"in the wild\""] # [doc = " outside their binder, e.g., in types passed to type inference, and"] # [doc = " should first be instantiated (by placeholder regions, free regions,"] # [doc = " or region variables)."] # [doc = ""] # [doc = " ## Placeholder and Free Regions"] # [doc = ""] # [doc = " One often wants to work with bound regions without knowing their precise"] # [doc = " identity. For example, when checking a function, the lifetime of a borrow"] # [doc = " can end up being assigned to some region parameter. In these cases,"] # [doc = " it must be ensured that bounds on the region can't be accidentally"] # [doc = " assumed without being checked."] # [doc = ""] # [doc = " To do this, we replace the bound regions with placeholder markers,"] # [doc = " which don't satisfy any relation not explicitly provided."] # [doc = ""] # [doc = " There are two kinds of placeholder regions in rustc: `ReLateParam` and"] # [doc = " `RePlaceholder`. When checking an item's body, `ReLateParam` is supposed"] # [doc = " to be used. These also support explicit bounds: both the internally-stored"] # [doc = " *scope*, which the region is assumed to outlive, as well as other"] # [doc = " relations stored in the `FreeRegionMap`. Note that these relations"] # [doc = " aren't checked when you `make_subregion` (or `eq_types`), only by"] # [doc = " `resolve_regions_and_report_errors`."] # [doc = ""] # [doc = " When working with higher-ranked types, some region relations aren't"] # [doc = " yet known, so you can't just call `resolve_regions_and_report_errors`."] # [doc = " `RePlaceholder` is designed for this purpose. In these contexts,"] # [doc = " there's also the risk that some inference variable laying around will"] # [doc = " get unified with your placeholder region: if you want to check whether"] # [doc = " `for<'a> Foo<'_>: 'a`, and you instantiate your bound region `'a`"] # [doc = " with a placeholder region `'%a`, the variable `'_` would just be"] # [doc = " instantiated to the placeholder region `'%a`, which is wrong because"] # [doc = " the inference variable is supposed to satisfy the relation"] # [doc = " *for every value of the placeholder region*. To ensure that doesn't"] # [doc = " happen, you can use `leak_check`. This is more clearly explained"] # [doc = " by the [rustc dev guide]."] # [doc = ""] # [doc = " [1]: https://smallcultfollowing.com/babysteps/blog/2013/10/29/intermingled-parameter-lists/"] # [doc = " [2]: https://smallcultfollowing.com/babysteps/blog/2013/11/04/intermingled-parameter-lists/"] # [doc = " [rustc dev guide]: https://rustc-dev-guide.rust-lang.org/traits/hrtb.html"] # [derive_where (Clone , Copy , Hash , PartialEq ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext))] pub enum RegionKind < I : Interner > { # [doc = " A region parameter; for example `'a` in `impl<'a> Trait for &'a ()`."] # [doc = ""] # [doc = " There are some important differences between region and type parameters."] # [doc = " Not all region parameters in the source are represented via `ReEarlyParam`:"] # [doc = " late-bound function parameters are instead lowered to a `ReBound`. Late-bound"] # [doc = " regions get eagerly replaced with `ReLateParam` which behaves in the same way as"] # [doc = " `ReEarlyParam`. Region parameters are also sometimes implicit,"] # [doc = " e.g. in `impl Trait for &()`."] ReEarlyParam (I :: EarlyParamRegion) , # [doc = " A higher-ranked region. These represent either late-bound function parameters"] # [doc = " or bound variables from a `for<'a>`-binder."] # [doc = ""] # [doc = " While inside of a function, e.g. during typeck, the late-bound function parameters"] # [doc = " can be converted to `ReLateParam` by calling `tcx.liberate_late_bound_regions`."] # [doc = ""] # [doc = " Bound regions inside of types **must not** be erased, as they impact trait"] # [doc = " selection and the `TypeId` of that type. `for<'a> fn(&'a ())` and"] # [doc = " `fn(&'static ())` are different types and have to be treated as such."] ReBound (DebruijnIndex , I :: BoundRegion) , # [doc = " Late-bound function parameters are represented using a `ReBound`. When"] # [doc = " inside of a function, we convert these bound variables to placeholder"] # [doc = " parameters via `tcx.liberate_late_bound_regions`. They are then treated"] # [doc = " the same way as `ReEarlyParam` while inside of the function."] # [doc = ""] # [doc = " See <https://rustc-dev-guide.rust-lang.org/early_late_parameters.html> for"] # [doc = " more info about early and late bound lifetime parameters."] ReLateParam (I :: LateParamRegion) , # [doc = " Static data that has an \"infinite\" lifetime. Top in the region lattice."] ReStatic , # [doc = " A region variable. Should not exist outside of type inference."] ReVar (RegionVid) , # [doc = " A placeholder region -- the higher-ranked version of `ReLateParam`."] # [doc = " Should not exist outside of type inference."] # [doc = ""] # [doc = " Used when instantiating a `forall` binder via `infcx.enter_forall`."] RePlaceholder (I :: PlaceholderRegion) , # [doc = " Erased region, used by trait selection, in MIR and during codegen."] ReErased , # [doc = " A region that resulted from some other error. Used exclusively for diagnostics."] ReError (I :: ErrorGuaranteed) , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for RegionKind < I > { }}}
mkitem!{mkimpl!{impl < I : Interner > fmt :: Debug for RegionKind < I > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ReEarlyParam (data) => write ! (f , "{data:?}") , ReBound (binder_id , bound_region) => { write ! (f , "'") ? ; crate :: debug_bound_var (f , * binder_id , bound_region) } ReLateParam (fr) => write ! (f , "{fr:?}") , ReStatic => f . write_str ("'static") , ReVar (vid) => write ! (f , "{vid:?}") , RePlaceholder (placeholder) => write ! (f , "'{placeholder:?}") , ReErased => f . write_str ("'{erased}") , ReError (_) => f . write_str ("'{region error}") , } } }}}
mkitem!{mkimpl!{# [cfg (feature = "nightly")] impl < CTX , I : Interner > HashStable < CTX > for RegionKind < I > where I :: EarlyParamRegion : HashStable < CTX > , I :: BoundRegion : HashStable < CTX > , I :: LateParamRegion : HashStable < CTX > , I :: PlaceholderRegion : HashStable < CTX > , { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { std :: mem :: discriminant (self) . hash_stable (hcx , hasher) ; match self { ReErased | ReStatic | ReError (_) => { } ReBound (d , r) => { d . hash_stable (hcx , hasher) ; r . hash_stable (hcx , hasher) ; } ReEarlyParam (r) => { r . hash_stable (hcx , hasher) ; } ReLateParam (r) => { r . hash_stable (hcx , hasher) ; } RePlaceholder (r) => { r . hash_stable (hcx , hasher) ; } ReVar (_) => { panic ! ("region variables should not be hashed: {self:?}") } } } }}}