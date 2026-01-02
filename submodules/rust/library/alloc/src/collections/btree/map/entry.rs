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
mkuse!{use core :: fmt :: { self , Debug } ;}
mkuse!{use core :: marker :: PhantomData ;}
mkuse!{use core :: mem ;}
mkuse!{use Entry :: * ;}
mkuse!{use super :: super :: borrow :: DormantMutRef ;}
mkuse!{use super :: super :: node :: { Handle , NodeRef , marker } ;}
mkuse!{use super :: BTreeMap ;}
mkuse!{use crate :: alloc :: { Allocator , Global } ;}
mkitem!{mkenum!{#[doc = " A view into a single entry in a map, which may either be vacant or occupied."] #[doc = ""] #[doc = " This `enum` is constructed from the [`entry`] method on [`BTreeMap`]."] #[doc = ""] #[doc = " [`entry`]: BTreeMap::entry"] #[stable (feature = "rust1" , since = "1.0.0")] #[cfg_attr (not (test) , rustc_diagnostic_item = "BTreeEntry")] pub enum Entry < 'a , K : 'a , V : 'a , #[unstable (feature = "allocator_api" , issue = "32838")] A : Allocator + Clone = Global , > { #[doc = " A vacant entry."] #[stable (feature = "rust1" , since = "1.0.0")] Vacant (#[stable (feature = "rust1" , since = "1.0.0")] VacantEntry < 'a , K , V , A >) , #[doc = " An occupied entry."] #[stable (feature = "rust1" , since = "1.0.0")] Occupied (#[stable (feature = "rust1" , since = "1.0.0")] OccupiedEntry < 'a , K , V , A >) , }}}
mkitem!{mkimpl!{#[stable (feature = "debug_btree_map" , since = "1.12.0")] impl < K : Debug + Ord , V : Debug , A : Allocator + Clone > Debug for Entry < '_ , K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Vacant (ref v) => f . debug_tuple ("Entry") . field (v) . finish () , Occupied (ref o) => f . debug_tuple ("Entry") . field (o) . finish () , } } }}}
mkitem!{mkstruct!{#[doc = " A view into a vacant entry in a `BTreeMap`."] #[doc = " It is part of the [`Entry`] enum."] #[stable (feature = "rust1" , since = "1.0.0")] pub struct VacantEntry < 'a , K , V , #[unstable (feature = "allocator_api" , issue = "32838")] A : Allocator + Clone = Global , > { pub (super) key : K , #[doc = " `None` for a (empty) map without root"] pub (super) handle : Option < Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: Leaf > , marker :: Edge > > , pub (super) dormant_map : DormantMutRef < 'a , BTreeMap < K , V , A > > , #[doc = " The BTreeMap will outlive this IntoIter so we don't care about drop order for `alloc`."] pub (super) alloc : A , pub (super) _marker : PhantomData < & 'a mut (K , V) > , }}}
mkitem!{mkimpl!{#[stable (feature = "debug_btree_map" , since = "1.12.0")] impl < K : Debug + Ord , V , A : Allocator + Clone > Debug for VacantEntry < '_ , K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("VacantEntry") . field (self . key ()) . finish () } }}}
mkitem!{mkstruct!{#[doc = " A view into an occupied entry in a `BTreeMap`."] #[doc = " It is part of the [`Entry`] enum."] #[stable (feature = "rust1" , since = "1.0.0")] pub struct OccupiedEntry < 'a , K , V , #[unstable (feature = "allocator_api" , issue = "32838")] A : Allocator + Clone = Global , > { pub (super) handle : Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: LeafOrInternal > , marker :: KV > , pub (super) dormant_map : DormantMutRef < 'a , BTreeMap < K , V , A > > , #[doc = " The BTreeMap will outlive this IntoIter so we don't care about drop order for `alloc`."] pub (super) alloc : A , pub (super) _marker : PhantomData < & 'a mut (K , V) > , }}}
mkitem!{mkimpl!{#[stable (feature = "debug_btree_map" , since = "1.12.0")] impl < K : Debug + Ord , V : Debug , A : Allocator + Clone > Debug for OccupiedEntry < '_ , K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OccupiedEntry") . field ("key" , self . key ()) . field ("value" , self . get ()) . finish () } }}}
mkitem!{mkstruct!{#[doc = " The error returned by [`try_insert`](BTreeMap::try_insert) when the key already exists."] #[doc = ""] #[doc = " Contains the occupied entry, and the value that was not inserted."] #[unstable (feature = "map_try_insert" , issue = "82766")] pub struct OccupiedError < 'a , K : 'a , V : 'a , A : Allocator + Clone = Global > { #[doc = " The entry in the map that was already occupied."] pub entry : OccupiedEntry < 'a , K , V , A > , #[doc = " The value which was not inserted, because the entry was already occupied."] pub value : V , }}}
mkitem!{mkimpl!{#[unstable (feature = "map_try_insert" , issue = "82766")] impl < K : Debug + Ord , V : Debug , A : Allocator + Clone > Debug for OccupiedError < '_ , K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OccupiedError") . field ("key" , self . entry . key ()) . field ("old_value" , self . entry . get ()) . field ("new_value" , & self . value) . finish () } }}}
mkitem!{mkimpl!{#[unstable (feature = "map_try_insert" , issue = "82766")] impl < 'a , K : Debug + Ord , V : Debug , A : Allocator + Clone > fmt :: Display for OccupiedError < 'a , K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "failed to insert {:?}, key {:?} already exists with value {:?}" , self . value , self . entry . key () , self . entry . get () ,) } }}}
mkitem!{mkimpl!{#[unstable (feature = "map_try_insert" , issue = "82766")] impl < 'a , K : core :: fmt :: Debug + Ord , V : core :: fmt :: Debug > core :: error :: Error for crate :: collections :: btree_map :: OccupiedError < 'a , K , V > { }}}
mkitem!{mkimpl!{impl < 'a , K : Ord , V , A : Allocator + Clone > Entry < 'a , K , V , A > { #[doc = " Ensures a value is in the entry by inserting the default if empty, and returns"] #[doc = " a mutable reference to the value in the entry."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::collections::BTreeMap;"] #[doc = ""] #[doc = " let mut map: BTreeMap<&str, usize> = BTreeMap::new();"] #[doc = " map.entry(\"poneyland\").or_insert(12);"] #[doc = ""] #[doc = " assert_eq!(map[\"poneyland\"], 12);"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] pub fn or_insert (self , default : V) -> & 'a mut V { match self { Occupied (entry) => entry . into_mut () , Vacant (entry) => entry . insert (default) , } } #[doc = " Ensures a value is in the entry by inserting the result of the default function if empty,"] #[doc = " and returns a mutable reference to the value in the entry."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::collections::BTreeMap;"] #[doc = ""] #[doc = " let mut map: BTreeMap<&str, String> = BTreeMap::new();"] #[doc = " let s = \"hoho\".to_string();"] #[doc = ""] #[doc = " map.entry(\"poneyland\").or_insert_with(|| s);"] #[doc = ""] #[doc = " assert_eq!(map[\"poneyland\"], \"hoho\".to_string());"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] pub fn or_insert_with < F : FnOnce () -> V > (self , default : F) -> & 'a mut V { match self { Occupied (entry) => entry . into_mut () , Vacant (entry) => entry . insert (default ()) , } } #[doc = " Ensures a value is in the entry by inserting, if empty, the result of the default function."] #[doc = ""] #[doc = " This method allows for generating key-derived values for insertion by providing the default"] #[doc = " function a reference to the key that was moved during the `.entry(key)` method call."] #[doc = ""] #[doc = " The reference to the moved key is provided so that cloning or copying the key is"] #[doc = " unnecessary, unlike with `.or_insert_with(|| ... )`."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::collections::BTreeMap;"] #[doc = ""] #[doc = " let mut map: BTreeMap<&str, usize> = BTreeMap::new();"] #[doc = ""] #[doc = " map.entry(\"poneyland\").or_insert_with_key(|key| key.chars().count());"] #[doc = ""] #[doc = " assert_eq!(map[\"poneyland\"], 9);"] #[doc = " ```"] #[inline] #[stable (feature = "or_insert_with_key" , since = "1.50.0")] pub fn or_insert_with_key < F : FnOnce (& K) -> V > (self , default : F) -> & 'a mut V { match self { Occupied (entry) => entry . into_mut () , Vacant (entry) => { let value = default (entry . key ()) ; entry . insert (value) } } } #[doc = " Returns a reference to this entry's key."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::collections::BTreeMap;"] #[doc = ""] #[doc = " let mut map: BTreeMap<&str, usize> = BTreeMap::new();"] #[doc = " assert_eq!(map.entry(\"poneyland\").key(), &\"poneyland\");"] #[doc = " ```"] #[stable (feature = "map_entry_keys" , since = "1.10.0")] pub fn key (& self) -> & K { match * self { Occupied (ref entry) => entry . key () , Vacant (ref entry) => entry . key () , } } #[doc = " Provides in-place mutable access to an occupied entry before any"] #[doc = " potential inserts into the map."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::collections::BTreeMap;"] #[doc = ""] #[doc = " let mut map: BTreeMap<&str, usize> = BTreeMap::new();"] #[doc = ""] #[doc = " map.entry(\"poneyland\")"] #[doc = "    .and_modify(|e| { *e += 1 })"] #[doc = "    .or_insert(42);"] #[doc = " assert_eq!(map[\"poneyland\"], 42);"] #[doc = ""] #[doc = " map.entry(\"poneyland\")"] #[doc = "    .and_modify(|e| { *e += 1 })"] #[doc = "    .or_insert(42);"] #[doc = " assert_eq!(map[\"poneyland\"], 43);"] #[doc = " ```"] #[stable (feature = "entry_and_modify" , since = "1.26.0")] pub fn and_modify < F > (self , f : F) -> Self where F : FnOnce (& mut V) , { match self { Occupied (mut entry) => { f (entry . get_mut ()) ; Occupied (entry) } Vacant (entry) => Vacant (entry) , } } #[doc = " Sets the value of the entry, and returns an `OccupiedEntry`."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(btree_entry_insert)]"] #[doc = " use std::collections::BTreeMap;"] #[doc = ""] #[doc = " let mut map: BTreeMap<&str, String> = BTreeMap::new();"] #[doc = " let entry = map.entry(\"poneyland\").insert_entry(\"hoho\".to_string());"] #[doc = ""] #[doc = " assert_eq!(entry.key(), &\"poneyland\");"] #[doc = " ```"] #[inline] #[unstable (feature = "btree_entry_insert" , issue = "65225")] pub fn insert_entry (self , value : V) -> OccupiedEntry < 'a , K , V , A > { match self { Occupied (mut entry) => { entry . insert (value) ; entry } Vacant (entry) => entry . insert_entry (value) , } } }}}
mkitem!{mkimpl!{impl < 'a , K : Ord , V : Default , A : Allocator + Clone > Entry < 'a , K , V , A > { #[stable (feature = "entry_or_default" , since = "1.28.0")] #[doc = " Ensures a value is in the entry by inserting the default value if empty,"] #[doc = " and returns a mutable reference to the value in the entry."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::collections::BTreeMap;"] #[doc = ""] #[doc = " let mut map: BTreeMap<&str, Option<usize>> = BTreeMap::new();"] #[doc = " map.entry(\"poneyland\").or_default();"] #[doc = ""] #[doc = " assert_eq!(map[\"poneyland\"], None);"] #[doc = " ```"] pub fn or_default (self) -> & 'a mut V { match self { Occupied (entry) => entry . into_mut () , Vacant (entry) => entry . insert (Default :: default ()) , } } }}}
mkitem!{mkimpl!{impl < 'a , K : Ord , V , A : Allocator + Clone > VacantEntry < 'a , K , V , A > { #[doc = " Gets a reference to the key that would be used when inserting a value"] #[doc = " through the VacantEntry."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::collections::BTreeMap;"] #[doc = ""] #[doc = " let mut map: BTreeMap<&str, usize> = BTreeMap::new();"] #[doc = " assert_eq!(map.entry(\"poneyland\").key(), &\"poneyland\");"] #[doc = " ```"] #[stable (feature = "map_entry_keys" , since = "1.10.0")] pub fn key (& self) -> & K { & self . key } #[doc = " Take ownership of the key."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::collections::BTreeMap;"] #[doc = " use std::collections::btree_map::Entry;"] #[doc = ""] #[doc = " let mut map: BTreeMap<&str, usize> = BTreeMap::new();"] #[doc = ""] #[doc = " if let Entry::Vacant(v) = map.entry(\"poneyland\") {"] #[doc = "     v.into_key();"] #[doc = " }"] #[doc = " ```"] #[stable (feature = "map_entry_recover_keys2" , since = "1.12.0")] pub fn into_key (self) -> K { self . key } #[doc = " Sets the value of the entry with the `VacantEntry`'s key,"] #[doc = " and returns a mutable reference to it."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::collections::BTreeMap;"] #[doc = " use std::collections::btree_map::Entry;"] #[doc = ""] #[doc = " let mut map: BTreeMap<&str, u32> = BTreeMap::new();"] #[doc = ""] #[doc = " if let Entry::Vacant(o) = map.entry(\"poneyland\") {"] #[doc = "     o.insert(37);"] #[doc = " }"] #[doc = " assert_eq!(map[\"poneyland\"], 37);"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] #[rustc_confusables ("push" , "put")] pub fn insert (self , value : V) -> & 'a mut V { self . insert_entry (value) . into_mut () } #[doc = " Sets the value of the entry with the `VacantEntry`'s key,"] #[doc = " and returns an `OccupiedEntry`."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(btree_entry_insert)]"] #[doc = " use std::collections::BTreeMap;"] #[doc = " use std::collections::btree_map::Entry;"] #[doc = ""] #[doc = " let mut map: BTreeMap<&str, u32> = BTreeMap::new();"] #[doc = ""] #[doc = " if let Entry::Vacant(o) = map.entry(\"poneyland\") {"] #[doc = "     let entry = o.insert_entry(37);"] #[doc = "     assert_eq!(entry.get(), &37);"] #[doc = " }"] #[doc = " assert_eq!(map[\"poneyland\"], 37);"] #[doc = " ```"] #[unstable (feature = "btree_entry_insert" , issue = "65225")] pub fn insert_entry (mut self , value : V) -> OccupiedEntry < 'a , K , V , A > { let handle = match self . handle { None => { let map = unsafe { self . dormant_map . reborrow () } ; let root = map . root . insert (NodeRef :: new_leaf (self . alloc . clone ()) . forget_type ()) ; unsafe { let mut leaf = root . borrow_mut () . cast_to_leaf_unchecked () ; leaf . push_with_handle (self . key , value) } } Some (handle) => handle . insert_recursing (self . key , value , self . alloc . clone () , | ins | { drop (ins . left) ; let map = unsafe { self . dormant_map . reborrow () } ; let root = map . root . as_mut () . unwrap () ; root . push_internal_level (self . alloc . clone ()) . push (ins . kv . 0 , ins . kv . 1 , ins . right) }) , } ; unsafe { self . dormant_map . reborrow () . length += 1 } ; OccupiedEntry { handle : handle . forget_node_type () , dormant_map : self . dormant_map , alloc : self . alloc , _marker : PhantomData , } } }}}
mkitem!{mkimpl!{impl < 'a , K : Ord , V , A : Allocator + Clone > OccupiedEntry < 'a , K , V , A > { #[doc = " Gets a reference to the key in the entry."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::collections::BTreeMap;"] #[doc = ""] #[doc = " let mut map: BTreeMap<&str, usize> = BTreeMap::new();"] #[doc = " map.entry(\"poneyland\").or_insert(12);"] #[doc = " assert_eq!(map.entry(\"poneyland\").key(), &\"poneyland\");"] #[doc = " ```"] #[must_use] #[stable (feature = "map_entry_keys" , since = "1.10.0")] pub fn key (& self) -> & K { self . handle . reborrow () . into_kv () . 0 } #[doc = " Converts the entry into a reference to its key."] pub (crate) fn into_key (self) -> & 'a K { self . handle . into_kv_mut () . 0 } #[doc = " Take ownership of the key and value from the map."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::collections::BTreeMap;"] #[doc = " use std::collections::btree_map::Entry;"] #[doc = ""] #[doc = " let mut map: BTreeMap<&str, usize> = BTreeMap::new();"] #[doc = " map.entry(\"poneyland\").or_insert(12);"] #[doc = ""] #[doc = " if let Entry::Occupied(o) = map.entry(\"poneyland\") {"] #[doc = "     // We delete the entry from the map."] #[doc = "     o.remove_entry();"] #[doc = " }"] #[doc = ""] #[doc = " // If now try to get the value, it will panic:"] #[doc = " // println!(\"{}\", map[\"poneyland\"]);"] #[doc = " ```"] #[stable (feature = "map_entry_recover_keys2" , since = "1.12.0")] pub fn remove_entry (self) -> (K , V) { self . remove_kv () } #[doc = " Gets a reference to the value in the entry."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::collections::BTreeMap;"] #[doc = " use std::collections::btree_map::Entry;"] #[doc = ""] #[doc = " let mut map: BTreeMap<&str, usize> = BTreeMap::new();"] #[doc = " map.entry(\"poneyland\").or_insert(12);"] #[doc = ""] #[doc = " if let Entry::Occupied(o) = map.entry(\"poneyland\") {"] #[doc = "     assert_eq!(o.get(), &12);"] #[doc = " }"] #[doc = " ```"] #[must_use] #[stable (feature = "rust1" , since = "1.0.0")] pub fn get (& self) -> & V { self . handle . reborrow () . into_kv () . 1 } #[doc = " Gets a mutable reference to the value in the entry."] #[doc = ""] #[doc = " If you need a reference to the `OccupiedEntry` that may outlive the"] #[doc = " destruction of the `Entry` value, see [`into_mut`]."] #[doc = ""] #[doc = " [`into_mut`]: OccupiedEntry::into_mut"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::collections::BTreeMap;"] #[doc = " use std::collections::btree_map::Entry;"] #[doc = ""] #[doc = " let mut map: BTreeMap<&str, usize> = BTreeMap::new();"] #[doc = " map.entry(\"poneyland\").or_insert(12);"] #[doc = ""] #[doc = " assert_eq!(map[\"poneyland\"], 12);"] #[doc = " if let Entry::Occupied(mut o) = map.entry(\"poneyland\") {"] #[doc = "     *o.get_mut() += 10;"] #[doc = "     assert_eq!(*o.get(), 22);"] #[doc = ""] #[doc = "     // We can use the same Entry multiple times."] #[doc = "     *o.get_mut() += 2;"] #[doc = " }"] #[doc = " assert_eq!(map[\"poneyland\"], 24);"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] pub fn get_mut (& mut self) -> & mut V { self . handle . kv_mut () . 1 } #[doc = " Converts the entry into a mutable reference to its value."] #[doc = ""] #[doc = " If you need multiple references to the `OccupiedEntry`, see [`get_mut`]."] #[doc = ""] #[doc = " [`get_mut`]: OccupiedEntry::get_mut"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::collections::BTreeMap;"] #[doc = " use std::collections::btree_map::Entry;"] #[doc = ""] #[doc = " let mut map: BTreeMap<&str, usize> = BTreeMap::new();"] #[doc = " map.entry(\"poneyland\").or_insert(12);"] #[doc = ""] #[doc = " assert_eq!(map[\"poneyland\"], 12);"] #[doc = " if let Entry::Occupied(o) = map.entry(\"poneyland\") {"] #[doc = "     *o.into_mut() += 10;"] #[doc = " }"] #[doc = " assert_eq!(map[\"poneyland\"], 22);"] #[doc = " ```"] #[must_use = "`self` will be dropped if the result is not used"] #[stable (feature = "rust1" , since = "1.0.0")] pub fn into_mut (self) -> & 'a mut V { self . handle . into_val_mut () } #[doc = " Sets the value of the entry with the `OccupiedEntry`'s key,"] #[doc = " and returns the entry's old value."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::collections::BTreeMap;"] #[doc = " use std::collections::btree_map::Entry;"] #[doc = ""] #[doc = " let mut map: BTreeMap<&str, usize> = BTreeMap::new();"] #[doc = " map.entry(\"poneyland\").or_insert(12);"] #[doc = ""] #[doc = " if let Entry::Occupied(mut o) = map.entry(\"poneyland\") {"] #[doc = "     assert_eq!(o.insert(15), 12);"] #[doc = " }"] #[doc = " assert_eq!(map[\"poneyland\"], 15);"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] #[rustc_confusables ("push" , "put")] pub fn insert (& mut self , value : V) -> V { mem :: replace (self . get_mut () , value) } #[doc = " Takes the value of the entry out of the map, and returns it."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::collections::BTreeMap;"] #[doc = " use std::collections::btree_map::Entry;"] #[doc = ""] #[doc = " let mut map: BTreeMap<&str, usize> = BTreeMap::new();"] #[doc = " map.entry(\"poneyland\").or_insert(12);"] #[doc = ""] #[doc = " if let Entry::Occupied(o) = map.entry(\"poneyland\") {"] #[doc = "     assert_eq!(o.remove(), 12);"] #[doc = " }"] #[doc = " // If we try to get \"poneyland\"'s value, it'll panic:"] #[doc = " // println!(\"{}\", map[\"poneyland\"]);"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] #[rustc_confusables ("delete" , "take")] pub fn remove (self) -> V { self . remove_kv () . 1 } pub (super) fn remove_kv (self) -> (K , V) { let mut emptied_internal_root = false ; let (old_kv , _) = self . handle . remove_kv_tracking (| | emptied_internal_root = true , self . alloc . clone ()) ; let map = unsafe { self . dormant_map . awaken () } ; map . length -= 1 ; if emptied_internal_root { let root = map . root . as_mut () . unwrap () ; root . pop_internal_level (self . alloc) ; } old_kv } }}}