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
mkuse!{use std :: hash :: { BuildHasher , Hash , Hasher } ;}
mkuse!{use std :: marker :: PhantomData ;}
mkuse!{use std :: mem ;}
mkuse!{use std :: num :: NonZero ;}
mkuse!{use rustc_index :: bit_set :: { self , DenseBitSet } ;}
mkuse!{use rustc_index :: { Idx , IndexSlice , IndexVec } ;}
mkuse!{use smallvec :: SmallVec ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkuse!{use rustc_hashes :: { Hash64 , Hash128 } ;}
mkuse!{pub use rustc_stable_hash :: { FromStableHash , SipHasher128Hash as StableHasherHash , StableSipHasher128 as StableHasher , } ;}
mkitem!{mktrait!{# [doc = " Something that implements `HashStable<CTX>` can be hashed in a way that is"] # [doc = " stable across multiple compilation sessions."] # [doc = ""] # [doc = " Note that `HashStable` imposes rather more strict requirements than usual"] # [doc = " hash functions:"] # [doc = ""] # [doc = " - Stable hashes are sometimes used as identifiers. Therefore they must"] # [doc = "   conform to the corresponding `PartialEq` implementations:"] # [doc = ""] # [doc = "     - `x == y` implies `hash_stable(x) == hash_stable(y)`, and"] # [doc = "     - `x != y` implies `hash_stable(x) != hash_stable(y)`."] # [doc = ""] # [doc = "   That second condition is usually not required for hash functions"] # [doc = "   (e.g. `Hash`). In practice this means that `hash_stable` must feed any"] # [doc = "   information into the hasher that a `PartialEq` comparison takes into"] # [doc = "   account. See [#49300](https://github.com/rust-lang/rust/issues/49300)"] # [doc = "   for an example where violating this invariant has caused trouble in the"] # [doc = "   past."] # [doc = ""] # [doc = " - `hash_stable()` must be independent of the current"] # [doc = "    compilation session. E.g. they must not hash memory addresses or other"] # [doc = "    things that are \"randomly\" assigned per compilation session."] # [doc = ""] # [doc = " - `hash_stable()` must be independent of the host architecture. The"] # [doc = "   `StableHasher` takes care of endianness and `isize`/`usize` platform"] # [doc = "   differences."] pub trait HashStable < CTX > { fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) ; }}}
mkitem!{mktrait!{# [doc = " Implement this for types that can be turned into stable keys like, for"] # [doc = " example, for DefId that can be converted to a DefPathHash. This is used for"] # [doc = " bringing maps into a predictable order before hashing them."] pub trait ToStableHashKey < HCX > { type KeyType : Ord + Sized + HashStable < HCX > ; fn to_stable_hash_key (& self , hcx : & HCX) -> Self :: KeyType ; }}}
mkitem!{mktrait!{# [doc = " Trait for marking a type as having a sort order that is"] # [doc = " stable across compilation session boundaries. More formally:"] # [doc = ""] # [doc = " ```txt"] # [doc = " Ord::cmp(a1, b1) == Ord::cmp(a2, b2)"] # [doc = "    where a2 = decode(encode(a1, context1), context2)"] # [doc = "          b2 = decode(encode(b1, context1), context2)"] # [doc = " ```"] # [doc = ""] # [doc = " i.e. the result of `Ord::cmp` is not influenced by encoding"] # [doc = " the values in one session and then decoding them in another"] # [doc = " session."] # [doc = ""] # [doc = " This is trivially true for types where encoding and decoding"] # [doc = " don't change the bytes of the values that are used during"] # [doc = " comparison and comparison only depends on these bytes (as"] # [doc = " opposed to some non-local state). Examples are u32, String,"] # [doc = " Path, etc."] # [doc = ""] # [doc = " But it is not true for:"] # [doc = "  - `*const T` and `*mut T` because the values of these pointers"] # [doc = "    will change between sessions."] # [doc = "  - `DefIndex`, `CrateNum`, `LocalDefId`, because their concrete"] # [doc = "    values depend on state that might be different between"] # [doc = "    compilation sessions."] # [doc = ""] # [doc = " The associated constant `CAN_USE_UNSTABLE_SORT` denotes whether"] # [doc = " unstable sorting can be used for this type. Set to true if and"] # [doc = " only if `a == b` implies `a` and `b` are fully indistinguishable."] pub trait StableOrd : Ord { const CAN_USE_UNSTABLE_SORT : bool ; # [doc = " Marker to ensure that implementors have carefully considered"] # [doc = " whether their `Ord` implementation obeys this trait's contract."] const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () ; }}}
mkitem!{mkimpl!{impl < T : StableOrd > StableOrd for & T { const CAN_USE_UNSTABLE_SORT : bool = T :: CAN_USE_UNSTABLE_SORT ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }}}
mkitem!{mktrait!{# [doc = " This is a companion trait to `StableOrd`. Some types like `Symbol` can be"] # [doc = " compared in a cross-session stable way, but their `Ord` implementation is"] # [doc = " not stable. In such cases, a `StableOrd` implementation can be provided"] # [doc = " to offer a lightweight way for stable sorting. (The more heavyweight option"] # [doc = " is to sort via `ToStableHashKey`, but then sorting needs to have access to"] # [doc = " a stable hashing context and `ToStableHashKey` can also be expensive as in"] # [doc = " the case of `Symbol` where it has to allocate a `String`.)"] # [doc = ""] # [doc = " See the documentation of [StableOrd] for how stable sort order is defined."] # [doc = " The same definition applies here. Be careful when implementing this trait."] pub trait StableCompare { const CAN_USE_UNSTABLE_SORT : bool ; fn stable_cmp (& self , other : & Self) -> std :: cmp :: Ordering ; }}}
mkitem!{mkimpl!{# [doc = " `StableOrd` denotes that the type's `Ord` implementation is stable, so"] # [doc = " we can implement `StableCompare` by just delegating to `Ord`."] impl < T : StableOrd > StableCompare for T { const CAN_USE_UNSTABLE_SORT : bool = T :: CAN_USE_UNSTABLE_SORT ; fn stable_cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . cmp (other) } }}}
mkitem!{# [doc = " Implement HashStable by just calling `Hash::hash()`. Also implement `StableOrd` for the type since"] # [doc = " that has the same requirements."] # [doc = ""] # [doc = " **WARNING** This is only valid for types that *really* don't need any context for fingerprinting."] # [doc = " But it is easy to misuse this macro (see [#96013](https://github.com/rust-lang/rust/issues/96013)"] # [doc = " for examples). Therefore this macro is not exported and should only be used in the limited cases"] # [doc = " here in this module."] # [doc = ""] # [doc = " Use `#[derive(HashStable_Generic)]` instead."] macro_rules ! impl_stable_traits_for_trivial_type { ($ t : ty) => { impl < CTX > $ crate :: stable_hasher :: HashStable < CTX > for $ t { # [inline] fn hash_stable (& self , _ : & mut CTX , hasher : & mut $ crate :: stable_hasher :: StableHasher) { :: std :: hash :: Hash :: hash (self , hasher) ; } } impl $ crate :: stable_hasher :: StableOrd for $ t { const CAN_USE_UNSTABLE_SORT : bool = true ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; } } ; }}
mkuse!{pub (crate) use impl_stable_traits_for_trivial_type ;}
mkitem!{impl_stable_traits_for_trivial_type ! (i8) ;}
mkitem!{impl_stable_traits_for_trivial_type ! (i16) ;}
mkitem!{impl_stable_traits_for_trivial_type ! (i32) ;}
mkitem!{impl_stable_traits_for_trivial_type ! (i64) ;}
mkitem!{impl_stable_traits_for_trivial_type ! (isize) ;}
mkitem!{impl_stable_traits_for_trivial_type ! (u8) ;}
mkitem!{impl_stable_traits_for_trivial_type ! (u16) ;}
mkitem!{impl_stable_traits_for_trivial_type ! (u32) ;}
mkitem!{impl_stable_traits_for_trivial_type ! (u64) ;}
mkitem!{impl_stable_traits_for_trivial_type ! (usize) ;}
mkitem!{impl_stable_traits_for_trivial_type ! (u128) ;}
mkitem!{impl_stable_traits_for_trivial_type ! (i128) ;}
mkitem!{impl_stable_traits_for_trivial_type ! (char) ;}
mkitem!{impl_stable_traits_for_trivial_type ! (()) ;}
mkitem!{impl_stable_traits_for_trivial_type ! (Hash64) ;}
mkitem!{mkimpl!{impl < CTX > HashStable < CTX > for Hash128 { # [inline] fn hash_stable (& self , _ : & mut CTX , hasher : & mut StableHasher) { self . as_u128 () . hash (hasher) ; } }}}
mkitem!{mkimpl!{impl StableOrd for Hash128 { const CAN_USE_UNSTABLE_SORT : bool = true ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }}}
mkitem!{mkimpl!{impl < CTX > HashStable < CTX > for ! { fn hash_stable (& self , _ctx : & mut CTX , _hasher : & mut StableHasher) { unreachable ! () } }}}
mkitem!{mkimpl!{impl < CTX , T > HashStable < CTX > for PhantomData < T > { fn hash_stable (& self , _ctx : & mut CTX , _hasher : & mut StableHasher) { } }}}
mkitem!{mkimpl!{impl < CTX > HashStable < CTX > for NonZero < u32 > { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . get () . hash_stable (ctx , hasher) } }}}
mkitem!{mkimpl!{impl < CTX > HashStable < CTX > for NonZero < usize > { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . get () . hash_stable (ctx , hasher) } }}}
mkitem!{mkimpl!{impl < CTX > HashStable < CTX > for f32 { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { let val : u32 = self . to_bits () ; val . hash_stable (ctx , hasher) ; } }}}
mkitem!{mkimpl!{impl < CTX > HashStable < CTX > for f64 { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { let val : u64 = self . to_bits () ; val . hash_stable (ctx , hasher) ; } }}}
mkitem!{mkimpl!{impl < CTX > HashStable < CTX > for :: std :: cmp :: Ordering { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { (* self as i8) . hash_stable (ctx , hasher) ; } }}}
mkitem!{mkimpl!{impl < T1 : HashStable < CTX > , CTX > HashStable < CTX > for (T1 ,) { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { let (ref _0 ,) = * self ; _0 . hash_stable (ctx , hasher) ; } }}}
mkitem!{mkimpl!{impl < T1 : HashStable < CTX > , T2 : HashStable < CTX > , CTX > HashStable < CTX > for (T1 , T2) { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { let (ref _0 , ref _1) = * self ; _0 . hash_stable (ctx , hasher) ; _1 . hash_stable (ctx , hasher) ; } }}}
mkitem!{mkimpl!{impl < T1 : StableOrd , T2 : StableOrd > StableOrd for (T1 , T2) { const CAN_USE_UNSTABLE_SORT : bool = T1 :: CAN_USE_UNSTABLE_SORT && T2 :: CAN_USE_UNSTABLE_SORT ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }}}
mkitem!{mkimpl!{impl < T1 , T2 , T3 , CTX > HashStable < CTX > for (T1 , T2 , T3) where T1 : HashStable < CTX > , T2 : HashStable < CTX > , T3 : HashStable < CTX > , { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { let (ref _0 , ref _1 , ref _2) = * self ; _0 . hash_stable (ctx , hasher) ; _1 . hash_stable (ctx , hasher) ; _2 . hash_stable (ctx , hasher) ; } }}}
mkitem!{mkimpl!{impl < T1 : StableOrd , T2 : StableOrd , T3 : StableOrd > StableOrd for (T1 , T2 , T3) { const CAN_USE_UNSTABLE_SORT : bool = T1 :: CAN_USE_UNSTABLE_SORT && T2 :: CAN_USE_UNSTABLE_SORT && T3 :: CAN_USE_UNSTABLE_SORT ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }}}
mkitem!{mkimpl!{impl < T1 , T2 , T3 , T4 , CTX > HashStable < CTX > for (T1 , T2 , T3 , T4) where T1 : HashStable < CTX > , T2 : HashStable < CTX > , T3 : HashStable < CTX > , T4 : HashStable < CTX > , { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { let (ref _0 , ref _1 , ref _2 , ref _3) = * self ; _0 . hash_stable (ctx , hasher) ; _1 . hash_stable (ctx , hasher) ; _2 . hash_stable (ctx , hasher) ; _3 . hash_stable (ctx , hasher) ; } }}}
mkitem!{mkimpl!{impl < T1 : StableOrd , T2 : StableOrd , T3 : StableOrd , T4 : StableOrd > StableOrd for (T1 , T2 , T3 , T4) { const CAN_USE_UNSTABLE_SORT : bool = T1 :: CAN_USE_UNSTABLE_SORT && T2 :: CAN_USE_UNSTABLE_SORT && T3 :: CAN_USE_UNSTABLE_SORT && T4 :: CAN_USE_UNSTABLE_SORT ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }}}
mkitem!{mkimpl!{impl < T : HashStable < CTX > , CTX > HashStable < CTX > for [T] { default fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . len () . hash_stable (ctx , hasher) ; for item in self { item . hash_stable (ctx , hasher) ; } } }}}
mkitem!{mkimpl!{impl < CTX > HashStable < CTX > for [u8] { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . len () . hash_stable (ctx , hasher) ; hasher . write (self) ; } }}}
mkitem!{mkimpl!{impl < T : HashStable < CTX > , CTX > HashStable < CTX > for Vec < T > { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self [..] . hash_stable (ctx , hasher) ; } }}}
mkitem!{mkimpl!{impl < K , V , R , CTX > HashStable < CTX > for indexmap :: IndexMap < K , V , R > where K : HashStable < CTX > + Eq + Hash , V : HashStable < CTX > , R : BuildHasher , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . len () . hash_stable (ctx , hasher) ; for kv in self { kv . hash_stable (ctx , hasher) ; } } }}}
mkitem!{mkimpl!{impl < K , R , CTX > HashStable < CTX > for indexmap :: IndexSet < K , R > where K : HashStable < CTX > + Eq + Hash , R : BuildHasher , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . len () . hash_stable (ctx , hasher) ; for key in self { key . hash_stable (ctx , hasher) ; } } }}}
mkitem!{mkimpl!{impl < A , const N : usize , CTX > HashStable < CTX > for SmallVec < [A ; N] > where A : HashStable < CTX > , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self [..] . hash_stable (ctx , hasher) ; } }}}
mkitem!{mkimpl!{impl < T : ? Sized + HashStable < CTX > , CTX > HashStable < CTX > for Box < T > { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { (* * self) . hash_stable (ctx , hasher) ; } }}}
mkitem!{mkimpl!{impl < T : ? Sized + HashStable < CTX > , CTX > HashStable < CTX > for :: std :: rc :: Rc < T > { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { (* * self) . hash_stable (ctx , hasher) ; } }}}
mkitem!{mkimpl!{impl < T : ? Sized + HashStable < CTX > , CTX > HashStable < CTX > for :: std :: sync :: Arc < T > { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { (* * self) . hash_stable (ctx , hasher) ; } }}}
mkitem!{mkimpl!{impl < CTX > HashStable < CTX > for str { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . as_bytes () . hash_stable (ctx , hasher) ; } }}}
mkitem!{mkimpl!{impl StableOrd for & str { const CAN_USE_UNSTABLE_SORT : bool = true ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }}}
mkitem!{mkimpl!{impl < CTX > HashStable < CTX > for String { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self [..] . hash_stable (hcx , hasher) ; } }}}
mkitem!{mkimpl!{impl StableOrd for String { const CAN_USE_UNSTABLE_SORT : bool = true ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }}}
mkitem!{mkimpl!{impl < HCX > ToStableHashKey < HCX > for String { type KeyType = String ; # [inline] fn to_stable_hash_key (& self , _ : & HCX) -> Self :: KeyType { self . clone () } }}}
mkitem!{mkimpl!{impl < HCX , T1 : ToStableHashKey < HCX > , T2 : ToStableHashKey < HCX > > ToStableHashKey < HCX > for (T1 , T2) { type KeyType = (T1 :: KeyType , T2 :: KeyType) ; # [inline] fn to_stable_hash_key (& self , hcx : & HCX) -> Self :: KeyType { (self . 0 . to_stable_hash_key (hcx) , self . 1 . to_stable_hash_key (hcx)) } }}}
mkitem!{mkimpl!{impl < CTX > HashStable < CTX > for bool { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { (if * self { 1u8 } else { 0u8 }) . hash_stable (ctx , hasher) ; } }}}
mkitem!{mkimpl!{impl StableOrd for bool { const CAN_USE_UNSTABLE_SORT : bool = true ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }}}
mkitem!{mkimpl!{impl < T , CTX > HashStable < CTX > for Option < T > where T : HashStable < CTX > , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { if let Some (ref value) = * self { 1u8 . hash_stable (ctx , hasher) ; value . hash_stable (ctx , hasher) ; } else { 0u8 . hash_stable (ctx , hasher) ; } } }}}
mkitem!{mkimpl!{impl < T : StableOrd > StableOrd for Option < T > { const CAN_USE_UNSTABLE_SORT : bool = T :: CAN_USE_UNSTABLE_SORT ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }}}
mkitem!{mkimpl!{impl < T1 , T2 , CTX > HashStable < CTX > for Result < T1 , T2 > where T1 : HashStable < CTX > , T2 : HashStable < CTX > , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { mem :: discriminant (self) . hash_stable (ctx , hasher) ; match * self { Ok (ref x) => x . hash_stable (ctx , hasher) , Err (ref x) => x . hash_stable (ctx , hasher) , } } }}}
mkitem!{mkimpl!{impl < 'a , T , CTX > HashStable < CTX > for & 'a T where T : HashStable < CTX > + ? Sized , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { (* * self) . hash_stable (ctx , hasher) ; } }}}
mkitem!{mkimpl!{impl < T , CTX > HashStable < CTX > for :: std :: mem :: Discriminant < T > { # [inline] fn hash_stable (& self , _ : & mut CTX , hasher : & mut StableHasher) { :: std :: hash :: Hash :: hash (self , hasher) ; } }}}
mkitem!{mkimpl!{impl < T , CTX > HashStable < CTX > for :: std :: ops :: RangeInclusive < T > where T : HashStable < CTX > , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . start () . hash_stable (ctx , hasher) ; self . end () . hash_stable (ctx , hasher) ; } }}}
mkitem!{mkimpl!{impl < I : Idx , T , CTX > HashStable < CTX > for IndexSlice < I , T > where T : HashStable < CTX > , { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . len () . hash_stable (ctx , hasher) ; for v in & self . raw { v . hash_stable (ctx , hasher) ; } } }}}
mkitem!{mkimpl!{impl < I : Idx , T , CTX > HashStable < CTX > for IndexVec < I , T > where T : HashStable < CTX > , { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . len () . hash_stable (ctx , hasher) ; for v in & self . raw { v . hash_stable (ctx , hasher) ; } } }}}
mkitem!{mkimpl!{impl < I : Idx , CTX > HashStable < CTX > for DenseBitSet < I > { fn hash_stable (& self , _ctx : & mut CTX , hasher : & mut StableHasher) { :: std :: hash :: Hash :: hash (self , hasher) ; } }}}
mkitem!{mkimpl!{impl < R : Idx , C : Idx , CTX > HashStable < CTX > for bit_set :: BitMatrix < R , C > { fn hash_stable (& self , _ctx : & mut CTX , hasher : & mut StableHasher) { :: std :: hash :: Hash :: hash (self , hasher) ; } }}}
mkitem!{mkimpl!{impl < T , CTX > HashStable < CTX > for bit_set :: FiniteBitSet < T > where T : HashStable < CTX > + bit_set :: FiniteBitSetTy , { fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . 0 . hash_stable (hcx , hasher) ; } }}}
mkitem!{impl_stable_traits_for_trivial_type ! (:: std :: ffi :: OsStr) ;}
mkitem!{impl_stable_traits_for_trivial_type ! (:: std :: path :: Path) ;}
mkitem!{impl_stable_traits_for_trivial_type ! (:: std :: path :: PathBuf) ;}
mkitem!{mkimpl!{impl < V , HCX > ! HashStable < HCX > for std :: collections :: HashSet < V > { }}}
mkitem!{mkimpl!{impl < K , V , HCX > ! HashStable < HCX > for std :: collections :: HashMap < K , V > { }}}
mkitem!{mkimpl!{impl < K , V , HCX > HashStable < HCX > for :: std :: collections :: BTreeMap < K , V > where K : HashStable < HCX > + StableOrd , V : HashStable < HCX > , { fn hash_stable (& self , hcx : & mut HCX , hasher : & mut StableHasher) { self . len () . hash_stable (hcx , hasher) ; for entry in self . iter () { entry . hash_stable (hcx , hasher) ; } } }}}
mkitem!{mkimpl!{impl < K , HCX > HashStable < HCX > for :: std :: collections :: BTreeSet < K > where K : HashStable < HCX > + StableOrd , { fn hash_stable (& self , hcx : & mut HCX , hasher : & mut StableHasher) { self . len () . hash_stable (hcx , hasher) ; for entry in self . iter () { entry . hash_stable (hcx , hasher) ; } } }}}
mkitem!{mkstruct!{# [doc = " Controls what data we do or do not hash."] # [doc = " Whenever a `HashStable` implementation caches its"] # [doc = " result, it needs to include `HashingControls` as part"] # [doc = " of the key, to ensure that it does not produce an incorrect"] # [doc = " result (for example, using a `Fingerprint` produced while"] # [doc = " hashing `Span`s when a `Fingerprint` without `Span`s is"] # [doc = " being requested)"] # [derive (Clone , Hash , Eq , PartialEq , Debug)] pub struct HashingControls { pub hash_spans : bool , }}}