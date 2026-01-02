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
mkuse!{use std :: borrow :: Borrow ;}
mkuse!{use std :: cmp :: Ordering ;}
mkuse!{use std :: fmt :: Debug ;}
mkuse!{use std :: mem ;}
mkuse!{use std :: ops :: { Bound , Index , IndexMut , RangeBounds } ;}
mkuse!{use rustc_macros :: { Decodable_NoContext , Encodable_NoContext } ;}
mkuse!{use crate :: stable_hasher :: { HashStable , StableHasher , StableOrd } ;}
mkmod!{index_map, { 
                getname!(index_map);
                getsrc!(index_map);
                getpath!(index_map);
                get_deps!(index_map);
                get_crates!(index_map);
                mkinclude!(index_map);
                 
            }}
mkuse!{pub use index_map :: SortedIndexMultiMap ;}
mkitem!{mkstruct!{#[doc = " `SortedMap` is a data structure with similar characteristics as BTreeMap but"] #[doc = " slightly different trade-offs: lookup is *O*(log(*n*)), insertion and removal"] #[doc = " are *O*(*n*) but elements can be iterated in order cheaply."] #[doc = ""] #[doc = " `SortedMap` can be faster than a `BTreeMap` for small sizes (<50) since it"] #[doc = " stores data in a more compact way. It also supports accessing contiguous"] #[doc = " ranges of elements as a slice, and slices of already sorted elements can be"] #[doc = " inserted efficiently."] #[derive (Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Encodable_NoContext , Decodable_NoContext)] pub struct SortedMap < K , V > { data : Vec < (K , V) > , }}}
mkitem!{mkimpl!{impl < K , V > Default for SortedMap < K , V > { #[inline] fn default () -> SortedMap < K , V > { SortedMap { data : Vec :: new () } } }}}
mkitem!{mkimpl!{impl < K , V > SortedMap < K , V > { #[inline] pub const fn new () -> SortedMap < K , V > { SortedMap { data : Vec :: new () } } }}}
mkitem!{mkimpl!{impl < K : Ord , V > SortedMap < K , V > { #[doc = " Construct a `SortedMap` from a presorted set of elements. This is faster"] #[doc = " than creating an empty map and then inserting the elements individually."] #[doc = ""] #[doc = " It is up to the caller to make sure that the elements are sorted by key"] #[doc = " and that there are no duplicates."] #[inline] pub fn from_presorted_elements (elements : Vec < (K , V) >) -> SortedMap < K , V > { debug_assert ! (elements . array_windows () . all (| [fst , snd] | fst . 0 < snd . 0)) ; SortedMap { data : elements } } #[inline] pub fn insert (& mut self , key : K , value : V) -> Option < V > { match self . lookup_index_for (& key) { Ok (index) => { let slot = unsafe { self . data . get_unchecked_mut (index) } ; Some (mem :: replace (& mut slot . 1 , value)) } Err (index) => { self . data . insert (index , (key , value)) ; None } } } #[inline] pub fn remove (& mut self , key : & K) -> Option < V > { match self . lookup_index_for (key) { Ok (index) => Some (self . data . remove (index) . 1) , Err (_) => None , } } #[inline] pub fn get < Q > (& self , key : & Q) -> Option < & V > where K : Borrow < Q > , Q : Ord + ? Sized , { match self . lookup_index_for (key) { Ok (index) => unsafe { Some (& self . data . get_unchecked (index) . 1) } , Err (_) => None , } } #[inline] pub fn get_mut < Q > (& mut self , key : & Q) -> Option < & mut V > where K : Borrow < Q > , Q : Ord + ? Sized , { match self . lookup_index_for (key) { Ok (index) => unsafe { Some (& mut self . data . get_unchecked_mut (index) . 1) } , Err (_) => None , } } #[doc = " Gets a mutable reference to the value in the entry, or insert a new one."] #[inline] pub fn get_mut_or_insert_default (& mut self , key : K) -> & mut V where K : Eq , V : Default , { let index = match self . lookup_index_for (& key) { Ok (index) => index , Err (index) => { self . data . insert (index , (key , V :: default ())) ; index } } ; unsafe { & mut self . data . get_unchecked_mut (index) . 1 } } #[inline] pub fn clear (& mut self) { self . data . clear () ; } #[doc = " Iterate over elements, sorted by key"] #[inline] pub fn iter (& self) -> std :: slice :: Iter < '_ , (K , V) > { self . data . iter () } #[doc = " Iterate over the keys, sorted"] #[inline] pub fn keys (& self) -> impl ExactSizeIterator < Item = & K > + DoubleEndedIterator { self . data . iter () . map (| (k , _) | k) } #[doc = " Iterate over values, sorted by key"] #[inline] pub fn values (& self) -> impl ExactSizeIterator < Item = & V > + DoubleEndedIterator { self . data . iter () . map (| (_ , v) | v) } #[inline] pub fn len (& self) -> usize { self . data . len () } #[inline] pub fn is_empty (& self) -> bool { self . len () == 0 } #[inline] pub fn range < R > (& self , range : R) -> & [(K , V)] where R : RangeBounds < K > , { let (start , end) = self . range_slice_indices (range) ; & self . data [start .. end] } #[doc = " `sm.range_is_empty(r)` == `sm.range(r).is_empty()`, but is faster."] #[inline] pub fn range_is_empty < R > (& self , range : R) -> bool where R : RangeBounds < K > , { self . data . binary_search_by (| (x , _) | { match range . start_bound () { Bound :: Included (start) if x < start => return Ordering :: Less , Bound :: Excluded (start) if x <= start => return Ordering :: Less , _ => { } } ; match range . end_bound () { Bound :: Included (end) if x > end => return Ordering :: Greater , Bound :: Excluded (end) if x >= end => return Ordering :: Greater , _ => { } } ; Ordering :: Equal }) . is_err () } #[inline] pub fn remove_range < R > (& mut self , range : R) where R : RangeBounds < K > , { let (start , end) = self . range_slice_indices (range) ; self . data . splice (start .. end , std :: iter :: empty ()) ; } #[doc = " Mutate all keys with the given function `f`. This mutation must not"] #[doc = " change the sort-order of keys."] #[inline] pub fn offset_keys < F > (& mut self , f : F) where F : Fn (& mut K) , { self . data . iter_mut () . map (| (k , _) | k) . for_each (f) ; } #[doc = " Inserts a presorted range of elements into the map. If the range can be"] #[doc = " inserted as a whole in between to existing elements of the map, this"] #[doc = " will be faster than inserting the elements individually."] #[doc = ""] #[doc = " It is up to the caller to make sure that the elements are sorted by key"] #[doc = " and that there are no duplicates."] #[inline] pub fn insert_presorted (& mut self , elements : Vec < (K , V) >) { if elements . is_empty () { return ; } debug_assert ! (elements . array_windows () . all (| [fst , snd] | fst . 0 < snd . 0)) ; let start_index = self . lookup_index_for (& elements [0] . 0) ; let elements = match start_index { Ok (index) => { let mut elements = elements . into_iter () ; self . data [index] = elements . next () . unwrap () ; elements } Err (index) => { if index == self . data . len () || elements . last () . unwrap () . 0 < self . data [index] . 0 { self . data . splice (index .. index , elements) ; return ; } let mut elements = elements . into_iter () ; self . data . insert (index , elements . next () . unwrap ()) ; elements } } ; for (k , v) in elements { self . insert (k , v) ; } } #[doc = " Looks up the key in `self.data` via `slice::binary_search()`."] #[inline (always)] fn lookup_index_for < Q > (& self , key : & Q) -> Result < usize , usize > where K : Borrow < Q > , Q : Ord + ? Sized , { self . data . binary_search_by (| (x , _) | x . borrow () . cmp (key)) } #[inline] fn range_slice_indices < R > (& self , range : R) -> (usize , usize) where R : RangeBounds < K > , { let start = match range . start_bound () { Bound :: Included (k) => match self . lookup_index_for (k) { Ok (index) | Err (index) => index , } , Bound :: Excluded (k) => match self . lookup_index_for (k) { Ok (index) => index + 1 , Err (index) => index , } , Bound :: Unbounded => 0 , } ; let end = match range . end_bound () { Bound :: Included (k) => match self . lookup_index_for (k) { Ok (index) => index + 1 , Err (index) => index , } , Bound :: Excluded (k) => match self . lookup_index_for (k) { Ok (index) | Err (index) => index , } , Bound :: Unbounded => self . data . len () , } ; (start , end) } #[inline] pub fn contains_key < Q > (& self , key : & Q) -> bool where K : Borrow < Q > , Q : Ord + ? Sized , { self . get (key) . is_some () } }}}
mkitem!{mkimpl!{impl < K : Ord , V > IntoIterator for SortedMap < K , V > { type Item = (K , V) ; type IntoIter = std :: vec :: IntoIter < (K , V) > ; fn into_iter (self) -> Self :: IntoIter { self . data . into_iter () } }}}
mkitem!{mkimpl!{impl < 'a , K , Q , V > Index < & 'a Q > for SortedMap < K , V > where K : Ord + Borrow < Q > , Q : Ord + ? Sized , { type Output = V ; fn index (& self , key : & Q) -> & Self :: Output { self . get (key) . expect ("no entry found for key") } }}}
mkitem!{mkimpl!{impl < 'a , K , Q , V > IndexMut < & 'a Q > for SortedMap < K , V > where K : Ord + Borrow < Q > , Q : Ord + ? Sized , { fn index_mut (& mut self , key : & Q) -> & mut Self :: Output { self . get_mut (key) . expect ("no entry found for key") } }}}
mkitem!{mkimpl!{impl < K : Ord , V > FromIterator < (K , V) > for SortedMap < K , V > { fn from_iter < T : IntoIterator < Item = (K , V) > > (iter : T) -> Self { let mut data : Vec < (K , V) > = iter . into_iter () . collect () ; data . sort_unstable_by (| (k1 , _) , (k2 , _) | k1 . cmp (k2)) ; data . dedup_by (| (k1 , _) , (k2 , _) | k1 == k2) ; SortedMap { data } } }}}
mkitem!{mkimpl!{impl < K : HashStable < CTX > + StableOrd , V : HashStable < CTX > , CTX > HashStable < CTX > for SortedMap < K , V > { #[inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . data . hash_stable (ctx , hasher) ; } }}}
mkitem!{mkimpl!{impl < K : Debug , V : Debug > Debug for SortedMap < K , V > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_map () . entries (self . data . iter () . map (| (a , b) | (a , b))) . finish () } }}}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}