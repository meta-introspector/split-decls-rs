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
mkuse!{use std :: fmt :: Debug ;}
mkuse!{use std :: hash :: Hash ;}
mkuse!{use std :: iter ;}
mkuse!{use std :: marker :: PhantomData ;}
mkuse!{use rustc_ast_ir :: Mutability ;}
mkuse!{#[cfg (feature = "nightly")] use rustc_data_structures :: fingerprint :: Fingerprint ;}
mkuse!{#[cfg (feature = "nightly")] use rustc_data_structures :: stable_hasher :: { HashStable , StableHasher , ToStableHashKey } ;}
mkuse!{#[cfg (feature = "nightly")] use rustc_macros :: { Decodable_NoContext , Encodable_NoContext , HashStable_NoContext } ;}
mkuse!{use crate :: inherent :: * ;}
mkuse!{use crate :: visit :: TypeVisitableExt as _ ;}
mkuse!{use crate :: { self as ty , Interner } ;}
mkitem!{mkenum!{#[doc = " See `simplify_type`."] #[derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] #[cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub enum SimplifiedType < DefId > { Bool , Char , Int (ty :: IntTy) , Uint (ty :: UintTy) , Float (ty :: FloatTy) , Adt (DefId) , Foreign (DefId) , Str , Array , Slice , Ref (Mutability) , Ptr (Mutability) , Never , Tuple (usize) , #[doc = " A trait object, all of whose components are markers"] #[doc = " (e.g., `dyn Send + Sync`)."] MarkerTraitObject , Trait (DefId) , Closure (DefId) , Coroutine (DefId) , CoroutineWitness (DefId) , Function (usize) , UnsafeBinder , Placeholder , Error , }}}
mkitem!{mkimpl!{#[cfg (feature = "nightly")] impl < HCX : Clone , DefId : HashStable < HCX > > ToStableHashKey < HCX > for SimplifiedType < DefId > { type KeyType = Fingerprint ; #[inline] fn to_stable_hash_key (& self , hcx : & HCX) -> Fingerprint { let mut hasher = StableHasher :: new () ; let mut hcx : HCX = hcx . clone () ; self . hash_stable (& mut hcx , & mut hasher) ; hasher . finish () } }}}
mkitem!{mkenum!{#[doc = " Generic parameters are pretty much just bound variables, e.g."] #[doc = " the type of `fn foo<'a, T>(x: &'a T) -> u32 { ... }` can be thought of as"] #[doc = " `for<'a, T> fn(&'a T) -> u32`."] #[doc = ""] #[doc = " Typecheck of `foo` has to succeed for all possible generic arguments, so"] #[doc = " during typeck, we have to treat its generic parameters as if they"] #[doc = " were placeholders."] #[doc = ""] #[doc = " But when calling `foo` we only have to provide a specific generic argument."] #[doc = " In that case the generic parameters are instantiated with inference variables."] #[doc = " As we use `simplify_type` before that instantiation happens, we just treat"] #[doc = " generic parameters as if they were inference variables in that case."] #[derive (PartialEq , Eq , Debug , Clone , Copy)] pub enum TreatParams { #[doc = " Treat parameters as infer vars. This is the correct mode for caching"] #[doc = " an impl's type for lookup."] InstantiateWithInfer , #[doc = " Treat parameters as placeholders in the given environment. This is the"] #[doc = " correct mode for *lookup*, as during candidate selection."] #[doc = ""] #[doc = " This also treats projections with inference variables as infer vars"] #[doc = " since they could be further normalized."] AsRigid , }}}

macro_rules! simplify_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function simplify_type in module {}", module_path!());
    };
}

mkfn!{
    simplify_type_introspect!();
    #[doc = " Tries to simplify a type by only returning the outermost injective¹ layer, if one exists."] #[doc = ""] #[doc = " **This function should only be used if you need to store or retrieve the type from some"] #[doc = " hashmap. If you want to quickly decide whether two types may unify, use the [DeepRejectCtxt]"] #[doc = " instead.**"] #[doc = ""] #[doc = " The idea is to get something simple that we can use to quickly decide if two types could unify,"] #[doc = " for example during method lookup. If this function returns `Some(x)` it can only unify with"] #[doc = " types for which this method returns either `Some(x)` as well or `None`."] #[doc = ""] #[doc = " A special case here are parameters and projections, which are only injective"] #[doc = " if they are treated as placeholders."] #[doc = ""] #[doc = " For example when storing impls based on their simplified self type, we treat"] #[doc = " generic parameters as if they were inference variables. We must not simplify them here,"] #[doc = " as they can unify with any other type."] #[doc = ""] #[doc = " With projections we have to be even more careful, as treating them as placeholders"] #[doc = " is only correct if they are fully normalized."] #[doc = ""] #[doc = " ¹ meaning that if the outermost layers are different, then the whole types are also different."] pub fn simplify_type < I : Interner > (cx : I , ty : I :: Ty , treat_params : TreatParams ,) -> Option < SimplifiedType < I :: DefId > > { match ty . kind () { ty :: Bool => Some (SimplifiedType :: Bool) , ty :: Char => Some (SimplifiedType :: Char) , ty :: Int (int_type) => Some (SimplifiedType :: Int (int_type)) , ty :: Uint (uint_type) => Some (SimplifiedType :: Uint (uint_type)) , ty :: Float (float_type) => Some (SimplifiedType :: Float (float_type)) , ty :: Adt (def , _) => Some (SimplifiedType :: Adt (def . def_id () . into ())) , ty :: Str => Some (SimplifiedType :: Str) , ty :: Array (..) => Some (SimplifiedType :: Array) , ty :: Slice (..) => Some (SimplifiedType :: Slice) , ty :: Pat (ty , ..) => simplify_type (cx , ty , treat_params) , ty :: RawPtr (_ , mutbl) => Some (SimplifiedType :: Ptr (mutbl)) , ty :: Dynamic (trait_info , ..) => match trait_info . principal_def_id () { Some (principal_def_id) if ! cx . trait_is_auto (principal_def_id) => { Some (SimplifiedType :: Trait (principal_def_id . into ())) } _ => Some (SimplifiedType :: MarkerTraitObject) , } , ty :: Ref (_ , _ , mutbl) => Some (SimplifiedType :: Ref (mutbl)) , ty :: FnDef (def_id , _) => Some (SimplifiedType :: Closure (def_id . into ())) , ty :: Closure (def_id , _) => Some (SimplifiedType :: Closure (def_id . into ())) , ty :: CoroutineClosure (def_id , _) => Some (SimplifiedType :: Closure (def_id . into ())) , ty :: Coroutine (def_id , _) => Some (SimplifiedType :: Coroutine (def_id . into ())) , ty :: CoroutineWitness (def_id , _) => Some (SimplifiedType :: CoroutineWitness (def_id . into ())) , ty :: Never => Some (SimplifiedType :: Never) , ty :: Tuple (tys) => Some (SimplifiedType :: Tuple (tys . len ())) , ty :: FnPtr (sig_tys , _hdr) => { Some (SimplifiedType :: Function (sig_tys . skip_binder () . inputs () . len ())) } ty :: UnsafeBinder (_) => Some (SimplifiedType :: UnsafeBinder) , ty :: Placeholder (..) => Some (SimplifiedType :: Placeholder) , ty :: Param (_) => match treat_params { TreatParams :: AsRigid => Some (SimplifiedType :: Placeholder) , TreatParams :: InstantiateWithInfer => None , } , ty :: Alias (..) => match treat_params { TreatParams :: AsRigid if ! ty . has_non_region_infer () || cx . next_trait_solver_globally () => { Some (SimplifiedType :: Placeholder) } TreatParams :: AsRigid | TreatParams :: InstantiateWithInfer => None , } , ty :: Foreign (def_id) => Some (SimplifiedType :: Foreign (def_id . into ())) , ty :: Error (_) => Some (SimplifiedType :: Error) , ty :: Bound (..) | ty :: Infer (_) => None , } }
}
mkitem!{mkimpl!{impl < DefId > SimplifiedType < DefId > { pub fn def (self) -> Option < DefId > { match self { SimplifiedType :: Adt (d) | SimplifiedType :: Foreign (d) | SimplifiedType :: Trait (d) | SimplifiedType :: Closure (d) | SimplifiedType :: Coroutine (d) | SimplifiedType :: CoroutineWitness (d) => Some (d) , _ => None , } } }}}
mkitem!{mkstruct!{#[doc = " Given generic arguments, could they be unified after"] #[doc = " replacing parameters with inference variables or placeholders."] #[doc = " This behavior is toggled using the const generics."] #[doc = ""] #[doc = " We use this to quickly reject impl/wc candidates without needing"] #[doc = " to instantiate generic arguments/having to enter a probe."] #[doc = ""] #[doc = " We also use this function during coherence. For coherence the"] #[doc = " impls only have to overlap for some value, so we treat parameters"] #[doc = " on both sides like inference variables."] #[derive (Debug , Clone , Copy)] pub struct DeepRejectCtxt < I : Interner , const INSTANTIATE_LHS_WITH_INFER : bool , const INSTANTIATE_RHS_WITH_INFER : bool , > { _interner : PhantomData < I > , }}}
mkitem!{mkimpl!{impl < I : Interner > DeepRejectCtxt < I , false , false > { #[doc = " Treat parameters in both the lhs and the rhs as rigid."] pub fn relate_rigid_rigid (_interner : I) -> DeepRejectCtxt < I , false , false > { DeepRejectCtxt { _interner : PhantomData } } }}}
mkitem!{mkimpl!{impl < I : Interner > DeepRejectCtxt < I , true , true > { #[doc = " Treat parameters in both the lhs and the rhs as infer vars."] pub fn relate_infer_infer (_interner : I) -> DeepRejectCtxt < I , true , true > { DeepRejectCtxt { _interner : PhantomData } } }}}
mkitem!{mkimpl!{impl < I : Interner > DeepRejectCtxt < I , false , true > { #[doc = " Treat parameters in the lhs as rigid, and in rhs as infer vars."] pub fn relate_rigid_infer (_interner : I) -> DeepRejectCtxt < I , false , true > { DeepRejectCtxt { _interner : PhantomData } } }}}
mkitem!{mkimpl!{impl < I : Interner , const INSTANTIATE_LHS_WITH_INFER : bool , const INSTANTIATE_RHS_WITH_INFER : bool > DeepRejectCtxt < I , INSTANTIATE_LHS_WITH_INFER , INSTANTIATE_RHS_WITH_INFER > { const STARTING_DEPTH : usize = 8 ; pub fn args_may_unify (self , obligation_args : I :: GenericArgs , impl_args : I :: GenericArgs ,) -> bool { self . args_may_unify_inner (obligation_args , impl_args , Self :: STARTING_DEPTH) } pub fn types_may_unify (self , lhs : I :: Ty , rhs : I :: Ty) -> bool { self . types_may_unify_inner (lhs , rhs , Self :: STARTING_DEPTH) } pub fn types_may_unify_with_depth (self , lhs : I :: Ty , rhs : I :: Ty , depth_limit : usize) -> bool { self . types_may_unify_inner (lhs , rhs , depth_limit) } fn args_may_unify_inner (self , obligation_args : I :: GenericArgs , impl_args : I :: GenericArgs , depth : usize ,) -> bool { iter :: zip (obligation_args . iter () , impl_args . iter ()) . all (| (obl , imp) | { match (obl . kind () , imp . kind ()) { (ty :: GenericArgKind :: Lifetime (_) , ty :: GenericArgKind :: Lifetime (_)) => true , (ty :: GenericArgKind :: Type (obl) , ty :: GenericArgKind :: Type (imp)) => { self . types_may_unify_inner (obl , imp , depth) } (ty :: GenericArgKind :: Const (obl) , ty :: GenericArgKind :: Const (imp)) => { self . consts_may_unify_inner (obl , imp) } _ => panic ! ("kind mismatch: {obl:?} {imp:?}") , } }) } fn types_may_unify_inner (self , lhs : I :: Ty , rhs : I :: Ty , depth : usize) -> bool { if lhs == rhs { return true ; } match rhs . kind () { ty :: Param (_) => { if INSTANTIATE_RHS_WITH_INFER { return true ; } } ty :: Error (_) | ty :: Alias (..) | ty :: Bound (..) => return true , ty :: Infer (var) => return self . var_and_ty_may_unify (var , lhs) , ty :: Bool | ty :: Char | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Adt (..) | ty :: Str | ty :: Array (..) | ty :: Slice (..) | ty :: RawPtr (..) | ty :: Dynamic (..) | ty :: Pat (..) | ty :: Ref (..) | ty :: Never | ty :: Tuple (..) | ty :: FnDef (..) | ty :: FnPtr (..) | ty :: Closure (..) | ty :: CoroutineClosure (..) | ty :: Coroutine (..) | ty :: CoroutineWitness (..) | ty :: Foreign (_) | ty :: Placeholder (_) | ty :: UnsafeBinder (_) => { } } ; let Some (depth) = depth . checked_sub (1) else { return true ; } ; match lhs . kind () { ty :: Ref (_ , lhs_ty , lhs_mutbl) => match rhs . kind () { ty :: Ref (_ , rhs_ty , rhs_mutbl) => { lhs_mutbl == rhs_mutbl && self . types_may_unify_inner (lhs_ty , rhs_ty , depth) } _ => false , } , ty :: Adt (lhs_def , lhs_args) => match rhs . kind () { ty :: Adt (rhs_def , rhs_args) => { lhs_def == rhs_def && self . args_may_unify_inner (lhs_args , rhs_args , depth) } _ => false , } , ty :: Param (lhs) => { INSTANTIATE_LHS_WITH_INFER || match rhs . kind () { ty :: Param (rhs) => lhs == rhs , _ => false , } } ty :: Placeholder (lhs) => { matches ! (rhs . kind () , ty :: Placeholder (rhs) if lhs == rhs) } ty :: Infer (var) => self . var_and_ty_may_unify (var , rhs) , ty :: Alias (..) => true , ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Str | ty :: Bool | ty :: Char | ty :: Never | ty :: Foreign (_) => lhs == rhs , ty :: Tuple (lhs) => match rhs . kind () { ty :: Tuple (rhs) => { lhs . len () == rhs . len () && iter :: zip (lhs . iter () , rhs . iter ()) . all (| (lhs , rhs) | self . types_may_unify_inner (lhs , rhs , depth)) } _ => false , } , ty :: Array (lhs_ty , lhs_len) => match rhs . kind () { ty :: Array (rhs_ty , rhs_len) => { self . types_may_unify_inner (lhs_ty , rhs_ty , depth) && self . consts_may_unify_inner (lhs_len , rhs_len) } _ => false , } , ty :: RawPtr (lhs_ty , lhs_mutbl) => match rhs . kind () { ty :: RawPtr (rhs_ty , rhs_mutbl) => { lhs_mutbl == rhs_mutbl && self . types_may_unify_inner (lhs_ty , rhs_ty , depth) } _ => false , } , ty :: Slice (lhs_ty) => { matches ! (rhs . kind () , ty :: Slice (rhs_ty) if self . types_may_unify_inner (lhs_ty , rhs_ty , depth)) } ty :: Dynamic (lhs_preds , ..) => { matches ! (rhs . kind () , ty :: Dynamic (rhs_preds , ..) if lhs_preds . principal_def_id () == rhs_preds . principal_def_id ()) } ty :: FnPtr (lhs_sig_tys , lhs_hdr) => match rhs . kind () { ty :: FnPtr (rhs_sig_tys , rhs_hdr) => { let lhs_sig_tys = lhs_sig_tys . skip_binder () . inputs_and_output ; let rhs_sig_tys = rhs_sig_tys . skip_binder () . inputs_and_output ; lhs_hdr == rhs_hdr && lhs_sig_tys . len () == rhs_sig_tys . len () && iter :: zip (lhs_sig_tys . iter () , rhs_sig_tys . iter ()) . all (| (lhs , rhs) | self . types_may_unify_inner (lhs , rhs , depth)) } _ => false , } , ty :: Bound (..) => true , ty :: FnDef (lhs_def_id , lhs_args) => match rhs . kind () { ty :: FnDef (rhs_def_id , rhs_args) => { lhs_def_id == rhs_def_id && self . args_may_unify_inner (lhs_args , rhs_args , depth) } _ => false , } , ty :: Closure (lhs_def_id , lhs_args) => match rhs . kind () { ty :: Closure (rhs_def_id , rhs_args) => { lhs_def_id == rhs_def_id && self . args_may_unify_inner (lhs_args , rhs_args , depth) } _ => false , } , ty :: CoroutineClosure (lhs_def_id , lhs_args) => match rhs . kind () { ty :: CoroutineClosure (rhs_def_id , rhs_args) => { lhs_def_id == rhs_def_id && self . args_may_unify_inner (lhs_args , rhs_args , depth) } _ => false , } , ty :: Coroutine (lhs_def_id , lhs_args) => match rhs . kind () { ty :: Coroutine (rhs_def_id , rhs_args) => { lhs_def_id == rhs_def_id && self . args_may_unify_inner (lhs_args , rhs_args , depth) } _ => false , } , ty :: CoroutineWitness (lhs_def_id , lhs_args) => match rhs . kind () { ty :: CoroutineWitness (rhs_def_id , rhs_args) => { lhs_def_id == rhs_def_id && self . args_may_unify_inner (lhs_args , rhs_args , depth) } _ => false , } , ty :: Pat (lhs_ty , _) => { matches ! (rhs . kind () , ty :: Pat (rhs_ty , _) if self . types_may_unify_inner (lhs_ty , rhs_ty , depth)) } ty :: UnsafeBinder (lhs_ty) => match rhs . kind () { ty :: UnsafeBinder (rhs_ty) => { self . types_may_unify (lhs_ty . skip_binder () , rhs_ty . skip_binder ()) } _ => false , } , ty :: Error (..) => true , } } fn consts_may_unify_inner (self , lhs : I :: Const , rhs : I :: Const) -> bool { match rhs . kind () { ty :: ConstKind :: Param (_) => { if INSTANTIATE_RHS_WITH_INFER { return true ; } } ty :: ConstKind :: Expr (_) | ty :: ConstKind :: Unevaluated (_) | ty :: ConstKind :: Error (_) | ty :: ConstKind :: Infer (_) | ty :: ConstKind :: Bound (..) => { return true ; } ty :: ConstKind :: Value (..) | ty :: ConstKind :: Placeholder (_) => { } } ; match lhs . kind () { ty :: ConstKind :: Value (lhs_val) => match rhs . kind () { ty :: ConstKind :: Value (rhs_val) => lhs_val . valtree () == rhs_val . valtree () , _ => false , } , ty :: ConstKind :: Param (lhs) => { INSTANTIATE_LHS_WITH_INFER || match rhs . kind () { ty :: ConstKind :: Param (rhs) => lhs == rhs , _ => false , } } ty :: ConstKind :: Placeholder (lhs) => { matches ! (rhs . kind () , ty :: ConstKind :: Placeholder (rhs) if lhs == rhs) } ty :: ConstKind :: Expr (_) | ty :: ConstKind :: Unevaluated (_) | ty :: ConstKind :: Error (_) => { true } ty :: ConstKind :: Infer (_) | ty :: ConstKind :: Bound (..) => true , } } fn var_and_ty_may_unify (self , var : ty :: InferTy , ty : I :: Ty) -> bool { if ! ty . is_known_rigid () { return true ; } match var { ty :: IntVar (_) => ty . is_integral () , ty :: FloatVar (_) => ty . is_floating_point () , _ => true , } } }}}