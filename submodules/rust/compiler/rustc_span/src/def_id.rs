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
mkuse!{use std :: hash :: { BuildHasherDefault , Hash , Hasher } ;}
mkuse!{use rustc_data_structures :: AtomicRef ;}
mkuse!{use rustc_data_structures :: fingerprint :: Fingerprint ;}
mkuse!{use rustc_data_structures :: stable_hasher :: { HashStable , StableHasher , StableOrd , ToStableHashKey } ;}
mkuse!{use rustc_data_structures :: unhash :: Unhasher ;}
mkuse!{use rustc_hashes :: Hash64 ;}
mkuse!{use rustc_index :: Idx ;}
mkuse!{use rustc_macros :: { Decodable , Encodable , HashStable_Generic } ;}
mkuse!{use rustc_serialize :: { Decodable , Encodable } ;}
mkuse!{use crate :: { HashStableContext , SpanDecoder , SpanEncoder , Symbol } ;}
mkitem!{pub type StableCrateIdMap = indexmap :: IndexMap < StableCrateId , CrateNum , BuildHasherDefault < Unhasher > > ;}
mkitem!{rustc_index :: newtype_index ! { # [orderable] # [debug_format = "crate{}"] pub struct CrateNum { } }}
mkitem!{# [doc = " Item definitions in the currently-compiled crate would have the `CrateNum`"] # [doc = " `LOCAL_CRATE` in their `DefId`."] pub const LOCAL_CRATE : CrateNum = CrateNum :: ZERO ;}
mkitem!{mkimpl!{impl CrateNum { # [inline] pub fn new (x : usize) -> CrateNum { CrateNum :: from_usize (x) } # [inline] pub fn as_def_id (self) -> DefId { DefId { krate : self , index : CRATE_DEF_INDEX } } # [inline] pub fn as_mod_def_id (self) -> ModDefId { ModDefId :: new_unchecked (DefId { krate : self , index : CRATE_DEF_INDEX }) } }}}
mkitem!{mkimpl!{impl fmt :: Display for CrateNum { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . as_u32 () , f) } }}}
mkitem!{mkstruct!{# [doc = " A `DefPathHash` is a fixed-size representation of a `DefPath` that is"] # [doc = " stable across crate and compilation session boundaries. It consists of two"] # [doc = " separate 64-bit hashes. The first uniquely identifies the crate this"] # [doc = " `DefPathHash` originates from (see [StableCrateId]), and the second"] # [doc = " uniquely identifies the corresponding `DefPath` within that crate. Together"] # [doc = " they form a unique identifier within an entire crate graph."] # [doc = ""] # [doc = " There is a very small chance of hash collisions, which would mean that two"] # [doc = " different `DefPath`s map to the same `DefPathHash`. Proceeding compilation"] # [doc = " with such a hash collision would very probably lead to an ICE, and in the"] # [doc = " worst case lead to a silent mis-compilation. The compiler therefore actively"] # [doc = " and exhaustively checks for such hash collisions and aborts compilation if"] # [doc = " it finds one."] # [doc = ""] # [doc = " `DefPathHash` uses 64-bit hashes for both the crate-id part and the"] # [doc = " crate-internal part, even though it is likely that there are many more"] # [doc = " `LocalDefId`s in a single crate than there are individual crates in a crate"] # [doc = " graph. Since we use the same number of bits in both cases, the collision"] # [doc = " probability for the crate-local part will be quite a bit higher (though"] # [doc = " still very small)."] # [doc = ""] # [doc = " This imbalance is not by accident: A hash collision in the"] # [doc = " crate-local part of a `DefPathHash` will be detected and reported while"] # [doc = " compiling the crate in question. Such a collision does not depend on"] # [doc = " outside factors and can be easily fixed by the crate maintainer (e.g. by"] # [doc = " renaming the item in question or by bumping the crate version in a harmless"] # [doc = " way)."] # [doc = ""] # [doc = " A collision between crate-id hashes on the other hand is harder to fix"] # [doc = " because it depends on the set of crates in the entire crate graph of a"] # [doc = " compilation session. Again, using the same crate with a different version"] # [doc = " number would fix the issue with a high probability -- but that might be"] # [doc = " easier said then done if the crates in questions are dependencies of"] # [doc = " third-party crates."] # [doc = ""] # [doc = " That being said, given a high quality hash function, the collision"] # [doc = " probabilities in question are very small. For example, for a big crate like"] # [doc = " `rustc_middle` (with ~50000 `LocalDefId`s as of the time of writing) there"] # [doc = " is a probability of roughly 1 in 14,750,000,000 of a crate-internal"] # [doc = " collision occurring. For a big crate graph with 1000 crates in it, there is"] # [doc = " a probability of 1 in 36,890,000,000,000 of a `StableCrateId` collision."] # [derive (Copy , Clone , Hash , PartialEq , Eq , PartialOrd , Ord , Debug)] # [derive (HashStable_Generic , Encodable , Decodable)] pub struct DefPathHash (pub Fingerprint) ;}}
mkitem!{mkimpl!{impl DefPathHash { # [doc = " Returns the [StableCrateId] identifying the crate this [DefPathHash]"] # [doc = " originates from."] # [inline] pub fn stable_crate_id (& self) -> StableCrateId { StableCrateId (self . 0 . split () . 0) } # [doc = " Returns the crate-local part of the [DefPathHash]."] # [inline] pub fn local_hash (& self) -> Hash64 { self . 0 . split () . 1 } # [doc = " Builds a new [DefPathHash] with the given [StableCrateId] and"] # [doc = " `local_hash`, where `local_hash` must be unique within its crate."] # [inline] pub fn new (stable_crate_id : StableCrateId , local_hash : Hash64) -> DefPathHash { DefPathHash (Fingerprint :: new (stable_crate_id . 0 , local_hash)) } }}}
mkitem!{mkimpl!{impl Default for DefPathHash { fn default () -> Self { DefPathHash (Fingerprint :: ZERO) } }}}
mkitem!{mkimpl!{impl StableOrd for DefPathHash { const CAN_USE_UNSTABLE_SORT : bool = true ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }}}
mkitem!{mkstruct!{# [doc = " A [`StableCrateId`] is a 64-bit hash of a crate name, together with all"] # [doc = " `-Cmetadata` arguments, and some other data. It is to [`CrateNum`] what [`DefPathHash`] is to"] # [doc = " [`DefId`]. It is stable across compilation sessions."] # [doc = ""] # [doc = " Since the ID is a hash value, there is a small chance that two crates"] # [doc = " end up with the same [`StableCrateId`]. The compiler will check for such"] # [doc = " collisions when loading crates and abort compilation in order to avoid"] # [doc = " further trouble."] # [doc = ""] # [doc = " For more information on the possibility of hash collisions in rustc,"] # [doc = " see the discussion in [`DefId`]."] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Debug)] # [derive (Hash , HashStable_Generic , Encodable , Decodable)] pub struct StableCrateId (pub (crate) Hash64) ;}}
mkitem!{mkimpl!{impl StableCrateId { # [doc = " Computes the stable ID for a crate with the given name and"] # [doc = " `-Cmetadata` arguments."] pub fn new (crate_name : Symbol , is_exe : bool , mut metadata : Vec < String > , cfg_version : & 'static str ,) -> StableCrateId { let mut hasher = StableHasher :: new () ; crate_name . as_str () . hash (& mut hasher) ; metadata . sort () ; metadata . dedup () ; hasher . write (b"metadata") ; for s in & metadata { hasher . write_usize (s . len ()) ; hasher . write (s . as_bytes ()) ; } hasher . write (if is_exe { b"exe" } else { b"lib" }) ; if let Some (val) = std :: env :: var_os ("RUSTC_FORCE_RUSTC_VERSION") { hasher . write (val . to_string_lossy () . into_owned () . as_bytes ()) } else { hasher . write (cfg_version . as_bytes ()) } StableCrateId (hasher . finish ()) } # [inline] pub fn as_u64 (self) -> u64 { self . 0 . as_u64 () } }}}
mkitem!{mkimpl!{impl fmt :: LowerHex for StableCrateId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (& self . 0 , f) } }}}
mkitem!{rustc_index :: newtype_index ! { # [doc = " A DefIndex is an index into the hir-map for a crate, identifying a"] # [doc = " particular definition. It should really be considered an interned"] # [doc = " shorthand for a particular DefPath."] # [orderable] # [debug_format = "DefIndex({})"] pub struct DefIndex { # [doc = " The crate root is always assigned index 0 by the AST Map code,"] # [doc = " thanks to `NodeCollector::new`."] const CRATE_DEF_INDEX = 0 ; } }}
mkitem!{mkstruct!{# [doc = " A `DefId` identifies a particular *definition*, by combining a crate"] # [doc = " index and a def index."] # [doc = ""] # [doc = " You can create a `DefId` from a `LocalDefId` using `local_def_id.to_def_id()`."] # [derive (Clone , PartialEq , Eq , Copy)] # [cfg_attr (not (target_pointer_width = "64") , derive (Hash))] # [repr (C)] # [rustc_pass_by_value] pub struct DefId { # [cfg (not (all (target_pointer_width = "64" , target_endian = "big")))] pub index : DefIndex , pub krate : CrateNum , # [cfg (all (target_pointer_width = "64" , target_endian = "big"))] pub index : DefIndex , }}}
mkitem!{mkimpl!{impl ! Ord for DefId { }}}
mkitem!{mkimpl!{impl ! PartialOrd for DefId { }}}
mkitem!{mkimpl!{# [cfg (target_pointer_width = "64")] impl Hash for DefId { fn hash < H : Hasher > (& self , h : & mut H) { (((self . krate . as_u32 () as u64) << 32) | (self . index . as_u32 () as u64)) . hash (h) } }}}
mkitem!{mkimpl!{impl DefId { # [doc = " Makes a local `DefId` from the given `DefIndex`."] # [inline] pub fn local (index : DefIndex) -> DefId { DefId { krate : LOCAL_CRATE , index } } # [doc = " Returns whether the item is defined in the crate currently being compiled."] # [inline] pub fn is_local (self) -> bool { self . krate == LOCAL_CRATE } # [inline] pub fn as_local (self) -> Option < LocalDefId > { self . is_local () . then (| | LocalDefId { local_def_index : self . index }) } # [inline] # [track_caller] pub fn expect_local (self) -> LocalDefId { match self . as_local () { Some (local_def_id) => local_def_id , None => panic ! ("DefId::expect_local: `{self:?}` isn't local") , } } # [inline] pub fn is_crate_root (self) -> bool { self . index == CRATE_DEF_INDEX } # [inline] pub fn as_crate_root (self) -> Option < CrateNum > { self . is_crate_root () . then_some (self . krate) } # [inline] pub fn is_top_level_module (self) -> bool { self . is_local () && self . is_crate_root () } }}}
mkitem!{mkimpl!{impl From < LocalDefId > for DefId { fn from (local : LocalDefId) -> DefId { local . to_def_id () } }}}

macro_rules! default_def_id_debug_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function default_def_id_debug in module {}", module_path!());
    };
}

mkfn!{
    default_def_id_debug_introspect!();
    pub fn default_def_id_debug (def_id : DefId , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("DefId") . field ("krate" , & def_id . krate) . field ("index" , & def_id . index) . finish () }
}
mkitem!{pub static DEF_ID_DEBUG : AtomicRef < fn (DefId , & mut fmt :: Formatter < '_ >) -> fmt :: Result > = AtomicRef :: new (& (default_def_id_debug as fn (_ , & mut fmt :: Formatter < '_ >) -> _)) ;}
mkitem!{mkimpl!{impl fmt :: Debug for DefId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* DEF_ID_DEBUG) (* self , f) } }}}
mkitem!{rustc_data_structures :: define_id_collections ! (DefIdMap , DefIdSet , DefIdMapEntry , DefId) ;}
mkitem!{mkstruct!{# [doc = " A `LocalDefId` is equivalent to a `DefId` with `krate == LOCAL_CRATE`. Since"] # [doc = " we encode this information in the type, we can ensure at compile time that"] # [doc = " no `DefId`s from upstream crates get thrown into the mix. There are quite a"] # [doc = " few cases where we know that only `DefId`s from the local crate are expected;"] # [doc = " a `DefId` from a different crate would signify a bug somewhere. This"] # [doc = " is when `LocalDefId` comes in handy."] # [derive (Clone , Copy , PartialEq , Eq , Hash)] pub struct LocalDefId { pub local_def_index : DefIndex , }}}
mkitem!{mkimpl!{impl ! Ord for LocalDefId { }}}
mkitem!{mkimpl!{impl ! PartialOrd for LocalDefId { }}}
mkitem!{pub const CRATE_DEF_ID : LocalDefId = LocalDefId { local_def_index : CRATE_DEF_INDEX } ;}
mkitem!{mkimpl!{impl Idx for LocalDefId { # [inline] fn new (idx : usize) -> Self { LocalDefId { local_def_index : Idx :: new (idx) } } # [inline] fn index (self) -> usize { self . local_def_index . index () } }}}
mkitem!{mkimpl!{impl LocalDefId { # [inline] pub fn to_def_id (self) -> DefId { DefId { krate : LOCAL_CRATE , index : self . local_def_index } } # [inline] pub fn is_top_level_module (self) -> bool { self == CRATE_DEF_ID } }}}
mkitem!{mkimpl!{impl fmt :: Debug for LocalDefId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . to_def_id () . fmt (f) } }}}
mkitem!{mkimpl!{impl < E : SpanEncoder > Encodable < E > for LocalDefId { fn encode (& self , s : & mut E) { self . to_def_id () . encode (s) ; } }}}
mkitem!{mkimpl!{impl < D : SpanDecoder > Decodable < D > for LocalDefId { fn decode (d : & mut D) -> LocalDefId { DefId :: decode (d) . expect_local () } }}}
mkitem!{rustc_data_structures :: define_id_collections ! (LocalDefIdMap , LocalDefIdSet , LocalDefIdMapEntry , LocalDefId) ;}
mkitem!{mkimpl!{impl < CTX : HashStableContext > HashStable < CTX > for DefId { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { hcx . def_path_hash (* self) . hash_stable (hcx , hasher) ; } }}}
mkitem!{mkimpl!{impl < CTX : HashStableContext > HashStable < CTX > for LocalDefId { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { hcx . def_path_hash (self . to_def_id ()) . local_hash () . hash_stable (hcx , hasher) ; } }}}
mkitem!{mkimpl!{impl < CTX : HashStableContext > HashStable < CTX > for CrateNum { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . as_def_id () . to_stable_hash_key (hcx) . stable_crate_id () . hash_stable (hcx , hasher) ; } }}}
mkitem!{mkimpl!{impl < CTX : HashStableContext > ToStableHashKey < CTX > for DefId { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , hcx : & CTX) -> DefPathHash { hcx . def_path_hash (* self) } }}}
mkitem!{mkimpl!{impl < CTX : HashStableContext > ToStableHashKey < CTX > for LocalDefId { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , hcx : & CTX) -> DefPathHash { hcx . def_path_hash (self . to_def_id ()) } }}}
mkitem!{mkimpl!{impl < CTX : HashStableContext > ToStableHashKey < CTX > for CrateNum { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , hcx : & CTX) -> DefPathHash { self . as_def_id () . to_stable_hash_key (hcx) } }}}
mkitem!{mkimpl!{impl < CTX : HashStableContext > ToStableHashKey < CTX > for DefPathHash { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , _ : & CTX) -> DefPathHash { * self } }}}
mkitem!{macro_rules ! typed_def_id { ($ Name : ident , $ LocalName : ident) => { # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , Encodable , Decodable , HashStable_Generic)] pub struct $ Name (DefId) ; impl $ Name { # [inline] pub const fn new_unchecked (def_id : DefId) -> Self { Self (def_id) } # [inline] pub fn to_def_id (self) -> DefId { self . into () } # [inline] pub fn is_local (self) -> bool { self . 0 . is_local () } # [inline] pub fn as_local (self) -> Option <$ LocalName > { self . 0 . as_local () . map ($ LocalName :: new_unchecked) } } impl From <$ LocalName > for $ Name { # [inline] fn from (local : $ LocalName) -> Self { Self (local . 0 . to_def_id ()) } } impl From <$ Name > for DefId { # [inline] fn from (typed : $ Name) -> Self { typed . 0 } } # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , Encodable , Decodable , HashStable_Generic)] pub struct $ LocalName (LocalDefId) ; impl ! Ord for $ LocalName { } impl ! PartialOrd for $ LocalName { } impl $ LocalName { # [inline] pub const fn new_unchecked (def_id : LocalDefId) -> Self { Self (def_id) } # [inline] pub fn to_def_id (self) -> DefId { self . 0 . into () } # [inline] pub fn to_local_def_id (self) -> LocalDefId { self . 0 } } impl From <$ LocalName > for LocalDefId { # [inline] fn from (typed : $ LocalName) -> Self { typed . 0 } } impl From <$ LocalName > for DefId { # [inline] fn from (typed : $ LocalName) -> Self { typed . 0 . into () } } } ; }}
mkitem!{typed_def_id ! { ModDefId , LocalModDefId }}
mkitem!{mkimpl!{impl LocalModDefId { pub const CRATE_DEF_ID : Self = Self :: new_unchecked (CRATE_DEF_ID) ; }}}
mkitem!{mkimpl!{impl ModDefId { pub fn is_top_level_module (self) -> bool { self . 0 . is_top_level_module () } }}}
mkitem!{mkimpl!{impl LocalModDefId { pub fn is_top_level_module (self) -> bool { self . 0 . is_top_level_module () } }}}