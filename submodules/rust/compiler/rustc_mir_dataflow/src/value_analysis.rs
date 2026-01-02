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
mkuse!{use std :: fmt :: { Debug , Formatter } ;}
mkuse!{use std :: ops :: Range ;}
mkuse!{use rustc_abi :: { FieldIdx , VariantIdx } ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashMap , FxIndexSet , StdEntry } ;}
mkuse!{use rustc_data_structures :: stack :: ensure_sufficient_stack ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_index :: bit_set :: DenseBitSet ;}
mkuse!{use rustc_middle :: mir :: visit :: { PlaceContext , Visitor } ;}
mkuse!{use rustc_middle :: mir :: * ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: JoinSemiLattice ;}
mkuse!{use crate :: lattice :: { HasBottom , HasTop } ;}
mkitem!{rustc_index :: newtype_index ! (# [doc = " This index uniquely identifies a place."] # [doc = ""] # [doc = " Not every place has a `PlaceIndex`, and not every `PlaceIndex` corresponds to a tracked"] # [doc = " place. However, every tracked place and all places along its projection have a `PlaceIndex`."] pub struct PlaceIndex { }) ;}
mkitem!{rustc_index :: newtype_index ! (# [doc = " This index uniquely identifies a tracked place and therefore a slot in [`State`]."] # [doc = ""] # [doc = " It is an implementation detail of this module."] struct ValueIndex { }) ;}
mkitem!{mkstruct!{# [doc = " See [`State`]."] # [derive (PartialEq , Eq , Debug)] pub struct StateData < V > { bottom : V , # [doc = " This map only contains values that are not `⊥`."] map : FxHashMap < ValueIndex , V > , }}}
mkitem!{mkimpl!{impl < V : HasBottom > StateData < V > { fn new () -> StateData < V > { StateData { bottom : V :: BOTTOM , map : FxHashMap :: default () } } fn get (& self , idx : ValueIndex) -> & V { self . map . get (& idx) . unwrap_or (& self . bottom) } fn insert (& mut self , idx : ValueIndex , elem : V) { if elem . is_bottom () { self . map . remove (& idx) ; } else { self . map . insert (idx , elem) ; } } }}}
mkitem!{mkimpl!{impl < V : Clone > Clone for StateData < V > { fn clone (& self) -> Self { StateData { bottom : self . bottom . clone () , map : self . map . clone () } } fn clone_from (& mut self , source : & Self) { self . map . clone_from (& source . map) } }}}
mkitem!{mkimpl!{impl < V : JoinSemiLattice + Clone > JoinSemiLattice for StateData < V > { fn join (& mut self , other : & Self) -> bool { let mut changed = false ; # [allow (rustc :: potential_query_instability)] for (i , v) in other . map . iter () { match self . map . entry (* i) { StdEntry :: Vacant (e) => { e . insert (v . clone ()) ; changed = true } StdEntry :: Occupied (e) => changed |= e . into_mut () . join (v) , } } changed } }}}
mkitem!{mkenum!{# [doc = " Dataflow state."] # [doc = ""] # [doc = " Every instance specifies a lattice that represents the possible values of a single tracked"] # [doc = " place. If we call this lattice `V` and set of tracked places `P`, then a [`State`] is an"] # [doc = " element of `{unreachable} ∪ (P -> V)`. This again forms a lattice, where the bottom element is"] # [doc = " `unreachable` and the top element is the mapping `p ↦ ⊤`. Note that the mapping `p ↦ ⊥` is not"] # [doc = " the bottom element (because joining an unreachable and any other reachable state yields a"] # [doc = " reachable state). All operations on unreachable states are ignored."] # [doc = ""] # [doc = " Flooding means assigning a value (by default `⊤`) to all tracked projections of a given place."] # [derive (PartialEq , Eq , Debug)] pub enum State < V > { Unreachable , Reachable (StateData < V >) , }}}
mkitem!{mkimpl!{impl < V : Clone > Clone for State < V > { fn clone (& self) -> Self { match self { Self :: Reachable (x) => Self :: Reachable (x . clone ()) , Self :: Unreachable => Self :: Unreachable , } } fn clone_from (& mut self , source : & Self) { match (& mut * self , source) { (Self :: Reachable (x) , Self :: Reachable (y)) => { x . clone_from (& y) ; } _ => * self = source . clone () , } } }}}
mkitem!{mkimpl!{impl < V : Clone + HasBottom > State < V > { pub fn new_reachable () -> State < V > { State :: Reachable (StateData :: new ()) } pub fn all_bottom (& self) -> bool { match self { State :: Unreachable => false , State :: Reachable (values) => { # [allow (rustc :: potential_query_instability)] values . map . values () . all (V :: is_bottom) } } } pub fn is_reachable (& self) -> bool { matches ! (self , State :: Reachable (_)) } # [doc = " Assign `value` to all places that are contained in `place` or may alias one."] pub fn flood_with (& mut self , place : PlaceRef < '_ > , map : & Map < '_ > , value : V) { self . flood_with_tail_elem (place , None , map , value) } # [doc = " Assign `TOP` to all places that are contained in `place` or may alias one."] pub fn flood (& mut self , place : PlaceRef < '_ > , map : & Map < '_ >) where V : HasTop , { self . flood_with (place , map , V :: TOP) } # [doc = " Assign `value` to the discriminant of `place` and all places that may alias it."] fn flood_discr_with (& mut self , place : PlaceRef < '_ > , map : & Map < '_ > , value : V) { self . flood_with_tail_elem (place , Some (TrackElem :: Discriminant) , map , value) } # [doc = " Assign `TOP` to the discriminant of `place` and all places that may alias it."] pub fn flood_discr (& mut self , place : PlaceRef < '_ > , map : & Map < '_ >) where V : HasTop , { self . flood_discr_with (place , map , V :: TOP) } # [doc = " This method is the most general version of the `flood_*` method."] # [doc = ""] # [doc = " Assign `value` on the given place and all places that may alias it. In particular, when"] # [doc = " the given place has a variant downcast, we invoke the function on all the other variants."] # [doc = ""] # [doc = " `tail_elem` allows to support discriminants that are not a place in MIR, but that we track"] # [doc = " as such."] pub fn flood_with_tail_elem (& mut self , place : PlaceRef < '_ > , tail_elem : Option < TrackElem > , map : & Map < '_ > , value : V ,) { let State :: Reachable (values) = self else { return } ; map . for_each_aliasing_place (place , tail_elem , & mut | vi | values . insert (vi , value . clone ())) ; } # [doc = " Low-level method that assigns to a place."] # [doc = " This does nothing if the place is not tracked."] # [doc = ""] # [doc = " The target place must have been flooded before calling this method."] fn insert_idx (& mut self , target : PlaceIndex , result : ValueOrPlace < V > , map : & Map < '_ >) { match result { ValueOrPlace :: Value (value) => self . insert_value_idx (target , value , map) , ValueOrPlace :: Place (source) => self . insert_place_idx (target , source , map) , } } # [doc = " Low-level method that assigns a value to a place."] # [doc = " This does nothing if the place is not tracked."] # [doc = ""] # [doc = " The target place must have been flooded before calling this method."] pub fn insert_value_idx (& mut self , target : PlaceIndex , value : V , map : & Map < '_ >) { let State :: Reachable (values) = self else { return } ; if let Some (value_index) = map . places [target] . value_index { values . insert (value_index , value) } } # [doc = " Copies `source` to `target`, including all tracked places beneath."] # [doc = ""] # [doc = " If `target` contains a place that is not contained in `source`, it will be overwritten with"] # [doc = " Top. Also, because this will copy all entries one after another, it may only be used for"] # [doc = " places that are non-overlapping or identical."] # [doc = ""] # [doc = " The target place must have been flooded before calling this method."] pub fn insert_place_idx (& mut self , target : PlaceIndex , source : PlaceIndex , map : & Map < '_ >) { let State :: Reachable (values) = self else { return } ; if let Some (target_value) = map . places [target] . value_index && let Some (source_value) = map . places [source] . value_index { values . insert (target_value , values . get (source_value) . clone ()) ; } for target_child in map . children (target) { let projection = map . places [target_child] . proj_elem . unwrap () ; if let Some (source_child) = map . projections . get (& (source , projection)) { self . insert_place_idx (target_child , * source_child , map) ; } } } # [doc = " Helper method to interpret `target = result`."] pub fn assign (& mut self , target : PlaceRef < '_ > , result : ValueOrPlace < V > , map : & Map < '_ >) where V : HasTop , { self . flood (target , map) ; if let Some (target) = map . find (target) { self . insert_idx (target , result , map) ; } } # [doc = " Helper method for assignments to a discriminant."] pub fn assign_discr (& mut self , target : PlaceRef < '_ > , result : ValueOrPlace < V > , map : & Map < '_ >) where V : HasTop , { self . flood_discr (target , map) ; if let Some (target) = map . find_discr (target) { self . insert_idx (target , result , map) ; } } # [doc = " Retrieve the value stored for a place, or `None` if it is not tracked."] pub fn try_get (& self , place : PlaceRef < '_ > , map : & Map < '_ >) -> Option < V > { let place = map . find (place) ? ; self . try_get_idx (place , map) } # [doc = " Retrieve the discriminant stored for a place, or `None` if it is not tracked."] pub fn try_get_discr (& self , place : PlaceRef < '_ > , map : & Map < '_ >) -> Option < V > { let place = map . find_discr (place) ? ; self . try_get_idx (place , map) } # [doc = " Retrieve the slice length stored for a place, or `None` if it is not tracked."] pub fn try_get_len (& self , place : PlaceRef < '_ > , map : & Map < '_ >) -> Option < V > { let place = map . find_len (place) ? ; self . try_get_idx (place , map) } # [doc = " Retrieve the value stored for a place index, or `None` if it is not tracked."] pub fn try_get_idx (& self , place : PlaceIndex , map : & Map < '_ >) -> Option < V > { match self { State :: Reachable (values) => { map . places [place] . value_index . map (| v | values . get (v) . clone ()) } State :: Unreachable => None , } } # [doc = " Retrieve the value stored for a place, or ⊤ if it is not tracked."] # [doc = ""] # [doc = " This method returns ⊥ if the place is tracked and the state is unreachable."] pub fn get (& self , place : PlaceRef < '_ > , map : & Map < '_ >) -> V where V : HasBottom + HasTop , { match self { State :: Reachable (_) => self . try_get (place , map) . unwrap_or (V :: TOP) , State :: Unreachable => V :: BOTTOM , } } # [doc = " Retrieve the value stored for a place, or ⊤ if it is not tracked."] # [doc = ""] # [doc = " This method returns ⊥ the current state is unreachable."] pub fn get_discr (& self , place : PlaceRef < '_ > , map : & Map < '_ >) -> V where V : HasBottom + HasTop , { match self { State :: Reachable (_) => self . try_get_discr (place , map) . unwrap_or (V :: TOP) , State :: Unreachable => V :: BOTTOM , } } # [doc = " Retrieve the value stored for a place, or ⊤ if it is not tracked."] # [doc = ""] # [doc = " This method returns ⊥ the current state is unreachable."] pub fn get_len (& self , place : PlaceRef < '_ > , map : & Map < '_ >) -> V where V : HasBottom + HasTop , { match self { State :: Reachable (_) => self . try_get_len (place , map) . unwrap_or (V :: TOP) , State :: Unreachable => V :: BOTTOM , } } # [doc = " Retrieve the value stored for a place index, or ⊤ if it is not tracked."] # [doc = ""] # [doc = " This method returns ⊥ the current state is unreachable."] pub fn get_idx (& self , place : PlaceIndex , map : & Map < '_ >) -> V where V : HasBottom + HasTop , { match self { State :: Reachable (values) => { map . places [place] . value_index . map (| v | values . get (v) . clone ()) . unwrap_or (V :: TOP) } State :: Unreachable => { V :: BOTTOM } } } }}}
mkitem!{mkimpl!{impl < V : JoinSemiLattice + Clone > JoinSemiLattice for State < V > { fn join (& mut self , other : & Self) -> bool { match (& mut * self , other) { (_ , State :: Unreachable) => false , (State :: Unreachable , _) => { * self = other . clone () ; true } (State :: Reachable (this) , State :: Reachable (other)) => this . join (other) , } } }}}
mkitem!{mkstruct!{# [doc = " Partial mapping from [`Place`] to [`PlaceIndex`], where some places also have a [`ValueIndex`]."] # [doc = ""] # [doc = " This data structure essentially maintains a tree of places and their projections. Some"] # [doc = " additional bookkeeping is done, to speed up traversal over this tree:"] # [doc = " - For iteration, every [`PlaceInfo`] contains an intrusive linked list of its children."] # [doc = " - To directly get the child for a specific projection, there is a `projections` map."] # [derive (Debug)] pub struct Map < 'tcx > { locals : IndexVec < Local , Option < PlaceIndex > > , projections : FxHashMap < (PlaceIndex , TrackElem) , PlaceIndex > , places : IndexVec < PlaceIndex , PlaceInfo < 'tcx > > , value_count : usize , inner_values : IndexVec < PlaceIndex , Range < usize > > , inner_values_buffer : Vec < ValueIndex > , }}}
mkitem!{mkimpl!{impl < 'tcx > Map < 'tcx > { # [doc = " Returns a map that only tracks places whose type has scalar layout."] # [doc = ""] # [doc = " This is currently the only way to create a [`Map`]. The way in which the tracked places are"] # [doc = " chosen is an implementation detail and may not be relied upon (other than that their type"] # [doc = " are scalars)."] pub fn new (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , value_limit : Option < usize >) -> Self { let mut map = Self { locals : IndexVec :: from_elem (None , & body . local_decls) , projections : FxHashMap :: default () , places : IndexVec :: new () , value_count : 0 , inner_values : IndexVec :: new () , inner_values_buffer : Vec :: new () , } ; let exclude = excluded_locals (body) ; map . register (tcx , body , exclude , value_limit) ; debug ! ("registered {} places ({} nodes in total)" , map . value_count , map . places . len ()) ; map } # [doc = " Register all non-excluded places that have scalar layout."] # [tracing :: instrument (level = "trace" , skip (self , tcx , body))] fn register (& mut self , tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , exclude : DenseBitSet < Local > , value_limit : Option < usize > ,) { for (local , decl) in body . local_decls . iter_enumerated () { if exclude . contains (local) { continue ; } if decl . ty . is_async_drop_in_place_coroutine (tcx) { continue ; } debug_assert ! (self . locals [local] . is_none ()) ; let place = self . places . push (PlaceInfo :: new (decl . ty , None)) ; self . locals [local] = Some (place) ; } let mut collector = PlaceCollector { tcx , body , map : self , assignments : Default :: default () } ; collector . visit_body (body) ; let PlaceCollector { mut assignments , .. } = collector ; let mut num_places = 0 ; while num_places < self . places . len () { num_places = self . places . len () ; for assign in 0 .. { let Some (& (lhs , rhs)) = assignments . get_index (assign) else { break } ; let mut child = self . places [lhs] . first_child ; while let Some (lhs_child) = child { let PlaceInfo { ty , proj_elem , next_sibling , .. } = self . places [lhs_child] ; let rhs_child = self . register_place (ty , rhs , proj_elem . expect ("child is not a projection")) ; assignments . insert ((lhs_child , rhs_child)) ; child = next_sibling ; } let mut child = self . places [rhs] . first_child ; while let Some (rhs_child) = child { let PlaceInfo { ty , proj_elem , next_sibling , .. } = self . places [rhs_child] ; let lhs_child = self . register_place (ty , lhs , proj_elem . expect ("child is not a projection")) ; assignments . insert ((lhs_child , rhs_child)) ; child = next_sibling ; } } } drop (assignments) ; let typing_env = body . typing_env (tcx) ; for place_info in self . places . iter_mut () { if let Some (value_limit) = value_limit && self . value_count >= value_limit { break ; } if let Ok (ty) = tcx . try_normalize_erasing_regions (typing_env , place_info . ty) { place_info . ty = ty ; } assert ! (place_info . value_index . is_none ()) ; if let Ok (layout) = tcx . layout_of (typing_env . as_query_input (place_info . ty)) && layout . backend_repr . is_scalar () { place_info . value_index = Some (self . value_count . into ()) ; self . value_count += 1 ; } } self . inner_values_buffer = Vec :: with_capacity (self . value_count) ; self . inner_values = IndexVec :: from_elem (0 .. 0 , & self . places) ; for local in body . local_decls . indices () { if let Some (place) = self . locals [local] { self . cache_preorder_invoke (place) ; } } for opt_place in self . locals . iter_mut () { if let Some (place) = * opt_place && self . inner_values [place] . is_empty () { * opt_place = None ; } } # [allow (rustc :: potential_query_instability)] self . projections . retain (| _ , child | ! self . inner_values [* child] . is_empty ()) ; } # [tracing :: instrument (level = "trace" , skip (self) , ret)] fn register_place (& mut self , ty : Ty < 'tcx > , base : PlaceIndex , elem : TrackElem) -> PlaceIndex { * self . projections . entry ((base , elem)) . or_insert_with (| | { let next = self . places . push (PlaceInfo :: new (ty , Some (elem))) ; self . places [next] . next_sibling = self . places [base] . first_child ; self . places [base] . first_child = Some (next) ; next }) } # [doc = " Precompute the list of values inside `root` and store it inside"] # [doc = " as a slice within `inner_values_buffer`."] fn cache_preorder_invoke (& mut self , root : PlaceIndex) { let start = self . inner_values_buffer . len () ; if let Some (vi) = self . places [root] . value_index { self . inner_values_buffer . push (vi) ; } let mut next_child = self . places [root] . first_child ; while let Some (child) = next_child { ensure_sufficient_stack (| | self . cache_preorder_invoke (child)) ; next_child = self . places [child] . next_sibling ; } let end = self . inner_values_buffer . len () ; self . inner_values [root] = start .. end ; } }}}
mkitem!{mkstruct!{struct PlaceCollector < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , body : & 'a Body < 'tcx > , map : & 'a mut Map < 'tcx > , assignments : FxIndexSet < (PlaceIndex , PlaceIndex) > , }}}
mkitem!{mkimpl!{impl < 'tcx > PlaceCollector < '_ , 'tcx > { # [tracing :: instrument (level = "trace" , skip (self))] fn register_place (& mut self , place : Place < 'tcx >) -> Option < PlaceIndex > { let mut place_index = self . map . locals [place . local] ? ; let mut ty = PlaceTy :: from_ty (self . body . local_decls [place . local] . ty) ; tracing :: trace ! (? place_index , ? ty) ; if let ty :: Ref (_ , ref_ty , _) | ty :: RawPtr (ref_ty , _) = ty . ty . kind () && let ty :: Slice (..) = ref_ty . kind () { self . map . register_place (self . tcx . types . usize , place_index , TrackElem :: DerefLen) ; } else if ty . ty . is_enum () { let discriminant_ty = ty . ty . discriminant_ty (self . tcx) ; self . map . register_place (discriminant_ty , place_index , TrackElem :: Discriminant) ; } for proj in place . projection { let track_elem = proj . try_into () . ok () ? ; ty = ty . projection_ty (self . tcx , proj) ; place_index = self . map . register_place (ty . ty , place_index , track_elem) ; tracing :: trace ! (? proj , ? place_index , ? ty) ; if let ty :: Ref (_ , ref_ty , _) | ty :: RawPtr (ref_ty , _) = ty . ty . kind () && let ty :: Slice (..) = ref_ty . kind () { self . map . register_place (self . tcx . types . usize , place_index , TrackElem :: DerefLen) ; } else if ty . ty . is_enum () { let discriminant_ty = ty . ty . discriminant_ty (self . tcx) ; self . map . register_place (discriminant_ty , place_index , TrackElem :: Discriminant) ; } } Some (place_index) } }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for PlaceCollector < '_ , 'tcx > { # [tracing :: instrument (level = "trace" , skip (self))] fn visit_place (& mut self , place : & Place < 'tcx > , ctxt : PlaceContext , _ : Location) { if ! ctxt . is_use () { return ; } self . register_place (* place) ; } fn visit_assign (& mut self , lhs : & Place < 'tcx > , rhs : & Rvalue < 'tcx > , location : Location) { self . super_assign (lhs , rhs , location) ; match rhs { Rvalue :: Use (Operand :: Move (rhs) | Operand :: Copy (rhs)) | Rvalue :: CopyForDeref (rhs) => { let Some (lhs) = self . register_place (* lhs) else { return } ; let Some (rhs) = self . register_place (* rhs) else { return } ; self . assignments . insert ((lhs , rhs)) ; } Rvalue :: Aggregate (kind , fields) => { let Some (mut lhs) = self . register_place (* lhs) else { return } ; match * * kind { AggregateKind :: Adt (_ , _ , _ , _ , Some (_)) => return , AggregateKind :: Adt (_ , variant , _ , _ , None) => { let ty = self . map . places [lhs] . ty ; if ty . is_enum () { lhs = self . map . register_place (ty , lhs , TrackElem :: Variant (variant)) ; } } AggregateKind :: RawPtr (..) | AggregateKind :: Array (_) | AggregateKind :: Tuple | AggregateKind :: Closure (..) | AggregateKind :: Coroutine (..) | AggregateKind :: CoroutineClosure (..) => { } } for (index , field) in fields . iter_enumerated () { if let Some (rhs) = field . place () && let Some (rhs) = self . register_place (rhs) { let lhs = self . map . register_place (self . map . places [rhs] . ty , lhs , TrackElem :: Field (index) ,) ; self . assignments . insert ((lhs , rhs)) ; } } } _ => { } } } }}}
mkitem!{mkimpl!{impl < 'tcx > Map < 'tcx > { # [doc = " Applies a single projection element, yielding the corresponding child."] pub fn apply (& self , place : PlaceIndex , elem : TrackElem) -> Option < PlaceIndex > { self . projections . get (& (place , elem)) . copied () } # [doc = " Locates the given place, if it exists in the tree."] fn find_extra (& self , place : PlaceRef < '_ > , extra : impl IntoIterator < Item = TrackElem > ,) -> Option < PlaceIndex > { let mut index = * self . locals [place . local] . as_ref () ? ; for & elem in place . projection { index = self . apply (index , elem . try_into () . ok () ?) ? ; } for elem in extra { index = self . apply (index , elem) ? ; } Some (index) } # [doc = " Locates the given place, if it exists in the tree."] pub fn find (& self , place : PlaceRef < '_ >) -> Option < PlaceIndex > { self . find_extra (place , []) } # [doc = " Locates the given place and applies `Discriminant`, if it exists in the tree."] pub fn find_discr (& self , place : PlaceRef < '_ >) -> Option < PlaceIndex > { self . find_extra (place , [TrackElem :: Discriminant]) } # [doc = " Locates the given place and applies `DerefLen`, if it exists in the tree."] pub fn find_len (& self , place : PlaceRef < '_ >) -> Option < PlaceIndex > { self . find_extra (place , [TrackElem :: DerefLen]) } # [doc = " Iterate over all direct children."] fn children (& self , parent : PlaceIndex) -> impl Iterator < Item = PlaceIndex > { Children :: new (self , parent) } # [doc = " Invoke a function on the given place and all places that may alias it."] # [doc = ""] # [doc = " In particular, when the given place has a variant downcast, we invoke the function on all"] # [doc = " the other variants."] # [doc = ""] # [doc = " `tail_elem` allows to support discriminants that are not a place in MIR, but that we track"] # [doc = " as such."] fn for_each_aliasing_place (& self , place : PlaceRef < '_ > , tail_elem : Option < TrackElem > , f : & mut impl FnMut (ValueIndex) ,) { if place . is_indirect_first_projection () { return ; } let Some (mut index) = self . locals [place . local] else { return ; } ; let elems = place . projection . iter () . map (| & elem | elem . try_into ()) . chain (tail_elem . map (Ok)) ; for elem in elems { if let Some (vi) = self . places [index] . value_index { f (vi) ; } let Ok (elem) = elem else { return } ; let sub = self . apply (index , elem) ; if let TrackElem :: Variant (..) | TrackElem :: Discriminant = elem { self . for_each_variant_sibling (index , sub , f) ; } if let Some (sub) = sub { index = sub } else { return ; } } self . for_each_value_inside (index , f) ; } # [doc = " Invoke the given function on all the descendants of the given place, except one branch."] fn for_each_variant_sibling (& self , parent : PlaceIndex , preserved_child : Option < PlaceIndex > , f : & mut impl FnMut (ValueIndex) ,) { for sibling in self . children (parent) { let elem = self . places [sibling] . proj_elem ; if let Some (TrackElem :: Variant (..) | TrackElem :: Discriminant) = elem && Some (sibling) != preserved_child { self . for_each_value_inside (sibling , f) ; } } } # [doc = " Invoke a function on each value in the given place and all descendants."] fn for_each_value_inside (& self , root : PlaceIndex , f : & mut impl FnMut (ValueIndex)) { let range = self . inner_values [root] . clone () ; let values = & self . inner_values_buffer [range] ; for & v in values { f (v) } } # [doc = " Invoke a function on each value in the given place and all descendants."] pub fn for_each_projection_value < O > (& self , root : PlaceIndex , value : O , project : & mut impl FnMut (TrackElem , & O) -> Option < O > , f : & mut impl FnMut (PlaceIndex , & O) ,) { if self . inner_values [root] . is_empty () { return ; } if self . places [root] . value_index . is_some () { f (root , & value) } for child in self . children (root) { let elem = self . places [child] . proj_elem . unwrap () ; if let Some (value) = project (elem , & value) { self . for_each_projection_value (child , value , project , f) ; } } } }}}
mkitem!{mkstruct!{# [doc = " This is the information tracked for every [`PlaceIndex`] and is stored by [`Map`]."] # [doc = ""] # [doc = " Together, `first_child` and `next_sibling` form an intrusive linked list, which is used to"] # [doc = " model a tree structure (a replacement for a member like `children: Vec<PlaceIndex>`)."] # [derive (Debug)] struct PlaceInfo < 'tcx > { # [doc = " Type of the referenced place."] ty : Ty < 'tcx > , # [doc = " We store a [`ValueIndex`] if and only if the placed is tracked by the analysis."] value_index : Option < ValueIndex > , # [doc = " The projection used to go from parent to this node (only None for root)."] proj_elem : Option < TrackElem > , # [doc = " The leftmost child."] first_child : Option < PlaceIndex > , # [doc = " Index of the sibling to the right of this node."] next_sibling : Option < PlaceIndex > , }}}
mkitem!{mkimpl!{impl < 'tcx > PlaceInfo < 'tcx > { fn new (ty : Ty < 'tcx > , proj_elem : Option < TrackElem >) -> Self { Self { ty , next_sibling : None , first_child : None , proj_elem , value_index : None } } }}}
mkitem!{mkstruct!{struct Children < 'a , 'tcx > { map : & 'a Map < 'tcx > , next : Option < PlaceIndex > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Children < 'a , 'tcx > { fn new (map : & 'a Map < 'tcx > , parent : PlaceIndex) -> Self { Self { map , next : map . places [parent] . first_child } } }}}
mkitem!{mkimpl!{impl Iterator for Children < '_ , '_ > { type Item = PlaceIndex ; fn next (& mut self) -> Option < Self :: Item > { match self . next { Some (child) => { self . next = self . map . places [child] . next_sibling ; Some (child) } None => None , } } }}}
mkitem!{mkenum!{# [doc = " Used as the result of an operand or r-value."] # [derive (Debug)] pub enum ValueOrPlace < V > { Value (V) , Place (PlaceIndex) , }}}
mkitem!{mkimpl!{impl < V : HasTop > ValueOrPlace < V > { pub const TOP : Self = ValueOrPlace :: Value (V :: TOP) ; }}}
mkitem!{mkenum!{# [doc = " The set of projection elements that can be used by a tracked place."] # [doc = ""] # [doc = " Although only field projections are currently allowed, this could change in the future."] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] pub enum TrackElem { Field (FieldIdx) , Variant (VariantIdx) , Discriminant , DerefLen , }}}
mkitem!{mkimpl!{impl < V , T > TryFrom < ProjectionElem < V , T > > for TrackElem { type Error = () ; fn try_from (value : ProjectionElem < V , T >) -> Result < Self , Self :: Error > { match value { ProjectionElem :: Field (field , _) => Ok (TrackElem :: Field (field)) , ProjectionElem :: Downcast (_ , idx) => Ok (TrackElem :: Variant (idx)) , _ => Err (()) , } } }}}

macro_rules! iter_fields_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function iter_fields in module {}", module_path!());
    };
}

mkfn!{
    iter_fields_introspect!();
    # [doc = " Invokes `f` on all direct fields of `ty`."] pub fn iter_fields < 'tcx > (ty : Ty < 'tcx > , tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , mut f : impl FnMut (Option < VariantIdx > , FieldIdx , Ty < 'tcx >) ,) { match ty . kind () { ty :: Tuple (list) => { for (field , ty) in list . iter () . enumerate () { f (None , field . into () , ty) ; } } ty :: Adt (def , args) => { if def . is_union () { return ; } for (v_index , v_def) in def . variants () . iter_enumerated () { let variant = if def . is_struct () { None } else { Some (v_index) } ; for (f_index , f_def) in v_def . fields . iter () . enumerate () { let field_ty = f_def . ty (tcx , args) ; let field_ty = tcx . try_normalize_erasing_regions (typing_env , field_ty) . unwrap_or_else (| _ | tcx . erase_and_anonymize_regions (field_ty)) ; f (variant , f_index . into () , field_ty) ; } } } ty :: Closure (_ , args) => { iter_fields (args . as_closure () . tupled_upvars_ty () , tcx , typing_env , f) ; } ty :: Coroutine (_ , args) => { iter_fields (args . as_coroutine () . tupled_upvars_ty () , tcx , typing_env , f) ; } ty :: CoroutineClosure (_ , args) => { iter_fields (args . as_coroutine_closure () . tupled_upvars_ty () , tcx , typing_env , f) ; } _ => () , } }
}

macro_rules! excluded_locals_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function excluded_locals in module {}", module_path!());
    };
}

mkfn!{
    excluded_locals_introspect!();
    # [doc = " Returns all locals with projections that have their reference or address taken."] pub fn excluded_locals (body : & Body < '_ >) -> DenseBitSet < Local > { struct Collector { result : DenseBitSet < Local > , } impl < 'tcx > Visitor < 'tcx > for Collector { fn visit_place (& mut self , place : & Place < 'tcx > , context : PlaceContext , _location : Location) { if context . may_observe_address () && ! place . is_indirect () { self . result . insert (place . local) ; } } } let mut collector = Collector { result : DenseBitSet :: new_empty (body . local_decls . len ()) } ; collector . visit_body (body) ; collector . result }
}

macro_rules! debug_with_context_rec_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function debug_with_context_rec in module {}", module_path!());
    };
}

mkfn!{
    debug_with_context_rec_introspect!();
    fn debug_with_context_rec < V : Debug + Eq + HasBottom > (place : PlaceIndex , place_str : & str , new : & StateData < V > , old : Option < & StateData < V > > , map : & Map < '_ > , f : & mut Formatter < '_ > ,) -> std :: fmt :: Result { if let Some (value) = map . places [place] . value_index { match old { None => writeln ! (f , "{}: {:?}" , place_str , new . get (value)) ? , Some (old) => { if new . get (value) != old . get (value) { writeln ! (f , "\u{001f}-{}: {:?}" , place_str , old . get (value)) ? ; writeln ! (f , "\u{001f}+{}: {:?}" , place_str , new . get (value)) ? ; } } } } for child in map . children (place) { let info_elem = map . places [child] . proj_elem . unwrap () ; let child_place_str = match info_elem { TrackElem :: Discriminant => { format ! ("discriminant({place_str})") } TrackElem :: Variant (idx) => { format ! ("({place_str} as {idx:?})") } TrackElem :: Field (field) => { if place_str . starts_with ('*') { format ! ("({}).{}" , place_str , field . index ()) } else { format ! ("{}.{}" , place_str , field . index ()) } } TrackElem :: DerefLen => { format ! ("Len(*{})" , place_str) } } ; debug_with_context_rec (child , & child_place_str , new , old , map , f) ? ; } Ok (()) }
}

macro_rules! debug_with_context_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function debug_with_context in module {}", module_path!());
    };
}

mkfn!{
    debug_with_context_introspect!();
    pub fn debug_with_context < V : Debug + Eq + HasBottom > (new : & StateData < V > , old : Option < & StateData < V > > , map : & Map < '_ > , f : & mut Formatter < '_ > ,) -> std :: fmt :: Result { for (local , place) in map . locals . iter_enumerated () { if let Some (place) = place { debug_with_context_rec (* place , & format ! ("{local:?}") , new , old , map , f) ? ; } } Ok (()) }
}