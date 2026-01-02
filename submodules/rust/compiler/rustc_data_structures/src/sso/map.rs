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
mkuse!{use std :: fmt ;}
mkuse!{use std :: hash :: Hash ;}
mkuse!{use std :: ops :: Index ;}
mkuse!{use arrayvec :: ArrayVec ;}
mkuse!{use either :: Either ;}
mkuse!{use crate :: fx :: FxHashMap ;}
mkitem!{#[doc = " For pointer-sized arguments arrays"] #[doc = " are faster than set/map for up to 64"] #[doc = " arguments."] #[doc = ""] #[doc = " On the other hand such a big array"] #[doc = " hurts cache performance, makes passing"] #[doc = " sso structures around very expensive."] #[doc = ""] #[doc = " Biggest performance benefit is gained"] #[doc = " for reasonably small arrays that stay"] #[doc = " small in vast majority of cases."] #[doc = ""] #[doc = " '8' is chosen as a sane default, to be"] #[doc = " reevaluated later."] const SSO_ARRAY_SIZE : usize = 8 ;}
mkitem!{mkenum!{#[doc = " Small-storage-optimized implementation of a map."] #[doc = ""] #[doc = " Stores elements in a small array up to a certain length"] #[doc = " and switches to `HashMap` when that length is exceeded."] #[derive (Clone)] pub enum SsoHashMap < K , V > { Array (ArrayVec < (K , V) , SSO_ARRAY_SIZE >) , Map (FxHashMap < K , V >) , }}}
mkitem!{mkimpl!{impl < K , V > SsoHashMap < K , V > { #[doc = " Creates an empty `SsoHashMap`."] #[inline] pub fn new () -> Self { SsoHashMap :: Array (ArrayVec :: new ()) } #[doc = " Creates an empty `SsoHashMap` with the specified capacity."] pub fn with_capacity (cap : usize) -> Self { if cap <= SSO_ARRAY_SIZE { Self :: new () } else { SsoHashMap :: Map (FxHashMap :: with_capacity_and_hasher (cap , Default :: default ())) } } #[doc = " Clears the map, removing all key-value pairs. Keeps the allocated memory"] #[doc = " for reuse."] pub fn clear (& mut self) { match self { SsoHashMap :: Array (array) => array . clear () , SsoHashMap :: Map (map) => map . clear () , } } #[doc = " Returns the number of elements the map can hold without reallocating."] pub fn capacity (& self) -> usize { match self { SsoHashMap :: Array (_) => SSO_ARRAY_SIZE , SsoHashMap :: Map (map) => map . capacity () , } } #[doc = " Returns the number of elements in the map."] pub fn len (& self) -> usize { match self { SsoHashMap :: Array (array) => array . len () , SsoHashMap :: Map (map) => map . len () , } } #[doc = " Returns `true` if the map contains no elements."] pub fn is_empty (& self) -> bool { match self { SsoHashMap :: Array (array) => array . is_empty () , SsoHashMap :: Map (map) => map . is_empty () , } } #[doc = " An iterator visiting all key-value pairs in arbitrary order."] #[doc = " The iterator element type is `(&'a K, &'a V)`."] #[inline] pub fn iter (& self) -> < & Self as IntoIterator > :: IntoIter { self . into_iter () } #[doc = " An iterator visiting all key-value pairs in arbitrary order,"] #[doc = " with mutable references to the values."] #[doc = " The iterator element type is `(&'a K, &'a mut V)`."] #[inline] pub fn iter_mut (& mut self) -> impl Iterator < Item = (& '_ K , & '_ mut V) > { self . into_iter () } #[doc = " An iterator visiting all keys in arbitrary order."] #[doc = " The iterator element type is `&'a K`."] pub fn keys (& self) -> impl Iterator < Item = & '_ K > { match self { SsoHashMap :: Array (array) => Either :: Left (array . iter () . map (| (k , _v) | k)) , SsoHashMap :: Map (map) => Either :: Right (map . keys ()) , } } #[doc = " An iterator visiting all values in arbitrary order."] #[doc = " The iterator element type is `&'a V`."] pub fn values (& self) -> impl Iterator < Item = & '_ V > { match self { SsoHashMap :: Array (array) => Either :: Left (array . iter () . map (| (_k , v) | v)) , SsoHashMap :: Map (map) => Either :: Right (map . values ()) , } } #[doc = " An iterator visiting all values mutably in arbitrary order."] #[doc = " The iterator element type is `&'a mut V`."] pub fn values_mut (& mut self) -> impl Iterator < Item = & '_ mut V > { match self { SsoHashMap :: Array (array) => Either :: Left (array . iter_mut () . map (| (_k , v) | v)) , SsoHashMap :: Map (map) => Either :: Right (map . values_mut ()) , } } #[doc = " Clears the map, returning all key-value pairs as an iterator. Keeps the"] #[doc = " allocated memory for reuse."] pub fn drain (& mut self) -> impl Iterator < Item = (K , V) > { match self { SsoHashMap :: Array (array) => Either :: Left (array . drain (..)) , SsoHashMap :: Map (map) => Either :: Right (map . drain ()) , } } }}}
mkitem!{mkimpl!{impl < K : Eq + Hash , V > SsoHashMap < K , V > { #[doc = " Changes underlying storage from array to hashmap"] #[doc = " if array is full."] fn migrate_if_full (& mut self) { if let SsoHashMap :: Array (array) = self { if array . is_full () { * self = SsoHashMap :: Map (array . drain (..) . collect ()) ; } } } #[doc = " Reserves capacity for at least `additional` more elements to be inserted"] #[doc = " in the `SsoHashMap`. The collection may reserve more space to avoid"] #[doc = " frequent reallocations."] pub fn reserve (& mut self , additional : usize) { match self { SsoHashMap :: Array (array) => { if SSO_ARRAY_SIZE < (array . len () + additional) { let mut map : FxHashMap < K , V > = array . drain (..) . collect () ; map . reserve (additional) ; * self = SsoHashMap :: Map (map) ; } } SsoHashMap :: Map (map) => map . reserve (additional) , } } #[doc = " Shrinks the capacity of the map as much as possible. It will drop"] #[doc = " down as much as possible while maintaining the internal rules"] #[doc = " and possibly leaving some space in accordance with the resize policy."] pub fn shrink_to_fit (& mut self) { if let SsoHashMap :: Map (map) = self { if map . len () <= SSO_ARRAY_SIZE { * self = SsoHashMap :: Array (map . drain () . collect ()) ; } else { map . shrink_to_fit () ; } } } #[doc = " Retains only the elements specified by the predicate."] pub fn retain < F > (& mut self , mut f : F) where F : FnMut (& K , & mut V) -> bool , { match self { SsoHashMap :: Array (array) => array . retain (| (k , v) | f (k , v)) , SsoHashMap :: Map (map) => map . retain (f) , } } #[doc = " Inserts a key-value pair into the map."] #[doc = ""] #[doc = " If the map did not have this key present, [`None`] is returned."] #[doc = ""] #[doc = " If the map did have this key present, the value is updated, and the old"] #[doc = " value is returned. The key is not updated, though; this matters for"] #[doc = " types that can be `==` without being identical. See the [module-level"] #[doc = " documentation] for more."] pub fn insert (& mut self , key : K , value : V) -> Option < V > { match self { SsoHashMap :: Array (array) => { for (k , v) in array . iter_mut () { if * k == key { let old_value = std :: mem :: replace (v , value) ; return Some (old_value) ; } } if let Err (error) = array . try_push ((key , value)) { let mut map : FxHashMap < K , V > = array . drain (..) . collect () ; let (key , value) = error . element () ; map . insert (key , value) ; * self = SsoHashMap :: Map (map) ; } None } SsoHashMap :: Map (map) => map . insert (key , value) , } } #[doc = " Removes a key from the map, returning the value at the key if the key"] #[doc = " was previously in the map."] pub fn remove (& mut self , key : & K) -> Option < V > { match self { SsoHashMap :: Array (array) => { array . iter () . position (| (k , _v) | k == key) . map (| index | array . swap_remove (index) . 1) } SsoHashMap :: Map (map) => map . remove (key) , } } #[doc = " Removes a key from the map, returning the stored key and value if the"] #[doc = " key was previously in the map."] pub fn remove_entry (& mut self , key : & K) -> Option < (K , V) > { match self { SsoHashMap :: Array (array) => { array . iter () . position (| (k , _v) | k == key) . map (| index | array . swap_remove (index)) } SsoHashMap :: Map (map) => map . remove_entry (key) , } } #[doc = " Returns a reference to the value corresponding to the key."] pub fn get (& self , key : & K) -> Option < & V > { match self { SsoHashMap :: Array (array) => { for (k , v) in array { if k == key { return Some (v) ; } } None } SsoHashMap :: Map (map) => map . get (key) , } } #[doc = " Returns a mutable reference to the value corresponding to the key."] pub fn get_mut (& mut self , key : & K) -> Option < & mut V > { match self { SsoHashMap :: Array (array) => { for (k , v) in array { if k == key { return Some (v) ; } } None } SsoHashMap :: Map (map) => map . get_mut (key) , } } #[doc = " Returns the key-value pair corresponding to the supplied key."] pub fn get_key_value (& self , key : & K) -> Option < (& K , & V) > { match self { SsoHashMap :: Array (array) => { for (k , v) in array { if k == key { return Some ((k , v)) ; } } None } SsoHashMap :: Map (map) => map . get_key_value (key) , } } #[doc = " Returns `true` if the map contains a value for the specified key."] pub fn contains_key (& self , key : & K) -> bool { match self { SsoHashMap :: Array (array) => array . iter () . any (| (k , _v) | k == key) , SsoHashMap :: Map (map) => map . contains_key (key) , } } #[doc = " Gets the given key's corresponding entry in the map for in-place manipulation."] #[inline] pub fn entry (& mut self , key : K) -> Entry < '_ , K , V > { Entry { ssomap : self , key } } }}}
mkitem!{mkimpl!{impl < K , V > Default for SsoHashMap < K , V > { #[inline] fn default () -> Self { Self :: new () } }}}
mkitem!{mkimpl!{impl < K : Eq + Hash , V > FromIterator < (K , V) > for SsoHashMap < K , V > { fn from_iter < I : IntoIterator < Item = (K , V) > > (iter : I) -> SsoHashMap < K , V > { let mut map : SsoHashMap < K , V > = Default :: default () ; map . extend (iter) ; map } }}}
mkitem!{mkimpl!{impl < K : Eq + Hash , V > Extend < (K , V) > for SsoHashMap < K , V > { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = (K , V) > , { for (key , value) in iter . into_iter () { self . insert (key , value) ; } } #[inline] fn extend_one (& mut self , (k , v) : (K , V)) { self . insert (k , v) ; } fn extend_reserve (& mut self , additional : usize) { match self { SsoHashMap :: Array (array) => { if SSO_ARRAY_SIZE < (array . len () + additional) { let mut map : FxHashMap < K , V > = array . drain (..) . collect () ; map . extend_reserve (additional) ; * self = SsoHashMap :: Map (map) ; } } SsoHashMap :: Map (map) => map . extend_reserve (additional) , } } }}}
mkitem!{mkimpl!{impl < 'a , K , V > Extend < (& 'a K , & 'a V) > for SsoHashMap < K , V > where K : Eq + Hash + Copy , V : Copy , { fn extend < T : IntoIterator < Item = (& 'a K , & 'a V) > > (& mut self , iter : T) { self . extend (iter . into_iter () . map (| (k , v) | (* k , * v))) } #[inline] fn extend_one (& mut self , (& k , & v) : (& 'a K , & 'a V)) { self . insert (k , v) ; } #[inline] fn extend_reserve (& mut self , additional : usize) { Extend :: < (K , V) > :: extend_reserve (self , additional) } }}}
mkitem!{mkimpl!{impl < K , V > IntoIterator for SsoHashMap < K , V > { type IntoIter = Either < < ArrayVec < (K , V) , SSO_ARRAY_SIZE > as IntoIterator > :: IntoIter , < FxHashMap < K , V > as IntoIterator > :: IntoIter , > ; type Item = < Self :: IntoIter as Iterator > :: Item ; fn into_iter (self) -> Self :: IntoIter { match self { SsoHashMap :: Array (array) => Either :: Left (array . into_iter ()) , SsoHashMap :: Map (map) => Either :: Right (map . into_iter ()) , } } }}}

macro_rules! adapt_array_ref_it_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function adapt_array_ref_it in module {}", module_path!());
    };
}

mkfn!{
    adapt_array_ref_it_introspect!();
    #[doc = " adapts Item of array reference iterator to Item of hashmap reference iterator."] #[inline (always)] fn adapt_array_ref_it < K , V > (pair : & (K , V)) -> (& K , & V) { let (a , b) = pair ; (a , b) }
}

macro_rules! adapt_array_mut_it_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function adapt_array_mut_it in module {}", module_path!());
    };
}

mkfn!{
    adapt_array_mut_it_introspect!();
    #[doc = " adapts Item of array mut reference iterator to Item of hashmap mut reference iterator."] #[inline (always)] fn adapt_array_mut_it < K , V > (pair : & mut (K , V)) -> (& K , & mut V) { let (a , b) = pair ; (a , b) }
}
mkitem!{mkimpl!{impl < 'a , K , V > IntoIterator for & 'a SsoHashMap < K , V > { type IntoIter = Either < std :: iter :: Map < < & 'a ArrayVec < (K , V) , SSO_ARRAY_SIZE > as IntoIterator > :: IntoIter , fn (& 'a (K , V)) -> (& 'a K , & 'a V) , > , < & 'a FxHashMap < K , V > as IntoIterator > :: IntoIter , > ; type Item = < Self :: IntoIter as Iterator > :: Item ; fn into_iter (self) -> Self :: IntoIter { match self { SsoHashMap :: Array (array) => Either :: Left (array . into_iter () . map (adapt_array_ref_it)) , SsoHashMap :: Map (map) => Either :: Right (map . iter ()) , } } }}}
mkitem!{mkimpl!{impl < 'a , K , V > IntoIterator for & 'a mut SsoHashMap < K , V > { type IntoIter = Either < std :: iter :: Map < < & 'a mut ArrayVec < (K , V) , SSO_ARRAY_SIZE > as IntoIterator > :: IntoIter , fn (& 'a mut (K , V)) -> (& 'a K , & 'a mut V) , > , < & 'a mut FxHashMap < K , V > as IntoIterator > :: IntoIter , > ; type Item = < Self :: IntoIter as Iterator > :: Item ; fn into_iter (self) -> Self :: IntoIter { match self { SsoHashMap :: Array (array) => Either :: Left (array . into_iter () . map (adapt_array_mut_it)) , SsoHashMap :: Map (map) => Either :: Right (map . iter_mut ()) , } } }}}
mkitem!{mkimpl!{impl < K , V > fmt :: Debug for SsoHashMap < K , V > where K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_map () . entries (self . iter ()) . finish () } }}}
mkitem!{mkimpl!{impl < 'a , K , V > Index < & 'a K > for SsoHashMap < K , V > where K : Eq + Hash , { type Output = V ; #[inline] fn index (& self , key : & K) -> & V { self . get (key) . expect ("no entry found for key") } }}}
mkitem!{mkstruct!{#[doc = " A view into a single entry in a map."] pub struct Entry < 'a , K , V > { ssomap : & 'a mut SsoHashMap < K , V > , key : K , }}}
mkitem!{mkimpl!{impl < 'a , K : Eq + Hash , V > Entry < 'a , K , V > { #[doc = " Provides in-place mutable access to an occupied entry before any"] #[doc = " potential inserts into the map."] pub fn and_modify < F > (self , f : F) -> Self where F : FnOnce (& mut V) , { if let Some (value) = self . ssomap . get_mut (& self . key) { f (value) ; } self } #[doc = " Ensures a value is in the entry by inserting the default if empty, and returns"] #[doc = " a mutable reference to the value in the entry."] #[inline] pub fn or_insert (self , value : V) -> & 'a mut V { self . or_insert_with (| | value) } #[doc = " Ensures a value is in the entry by inserting the result of the default function if empty,"] #[doc = " and returns a mutable reference to the value in the entry."] pub fn or_insert_with < F : FnOnce () -> V > (self , default : F) -> & 'a mut V { self . ssomap . migrate_if_full () ; match self . ssomap { SsoHashMap :: Array (array) => { let key_ref = & self . key ; let found_index = array . iter () . position (| (k , _v) | k == key_ref) ; let index = if let Some (index) = found_index { index } else { let index = array . len () ; array . try_push ((self . key , default ())) . unwrap () ; index } ; & mut array [index] . 1 } SsoHashMap :: Map (map) => map . entry (self . key) . or_insert_with (default) , } } #[doc = " Returns a reference to this entry's key."] #[inline] pub fn key (& self) -> & K { & self . key } }}}
mkitem!{mkimpl!{impl < 'a , K : Eq + Hash , V : Default > Entry < 'a , K , V > { #[doc = " Ensures a value is in the entry by inserting the default value if empty,"] #[doc = " and returns a mutable reference to the value in the entry."] #[inline] pub fn or_default (self) -> & 'a mut V { self . or_insert_with (Default :: default) } }}}