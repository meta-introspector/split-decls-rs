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
mkuse!{use std :: mem ;}
mkuse!{use std :: ops :: Deref ;}
mkuse!{use rustc_index :: bit_set :: BitMatrix ;}
mkuse!{use crate :: frozen :: Frozen ;}
mkuse!{use crate :: fx :: { FxHashSet , FxIndexSet } ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkitem!{mkstruct!{#[derive (Clone , Debug)] pub struct TransitiveRelationBuilder < T > { elements : FxIndexSet < T > , edges : FxHashSet < Edge > , }}}
mkitem!{mkstruct!{#[derive (Debug)] pub struct TransitiveRelation < T > { builder : Frozen < TransitiveRelationBuilder < T > > , closure : Frozen < BitMatrix < usize , usize > > , }}}
mkitem!{mkimpl!{impl < T > Deref for TransitiveRelation < T > { type Target = Frozen < TransitiveRelationBuilder < T > > ; fn deref (& self) -> & Self :: Target { & self . builder } }}}
mkitem!{mkimpl!{impl < T : Clone > Clone for TransitiveRelation < T > { fn clone (& self) -> Self { TransitiveRelation { builder : Frozen :: freeze (self . builder . deref () . clone ()) , closure : Frozen :: freeze (self . closure . deref () . clone ()) , } } }}}
mkitem!{mkimpl!{impl < T : Eq + Hash > Default for TransitiveRelationBuilder < T > { fn default () -> Self { TransitiveRelationBuilder { elements : Default :: default () , edges : Default :: default () } } }}}
mkitem!{mkstruct!{#[derive (Copy , Clone , PartialEq , Eq , PartialOrd , Debug , Hash)] struct Index (usize) ;}}
mkitem!{mkstruct!{#[derive (Clone , PartialEq , Eq , Debug , Hash)] struct Edge { source : Index , target : Index , }}}
mkitem!{mkimpl!{impl < T : Eq + Hash + Copy > TransitiveRelationBuilder < T > { pub fn is_empty (& self) -> bool { self . edges . is_empty () } pub fn elements (& self) -> impl Iterator < Item = & T > { self . elements . iter () } fn index (& self , a : T) -> Option < Index > { self . elements . get_index_of (& a) . map (Index) } fn add_index (& mut self , a : T) -> Index { let (index , _added) = self . elements . insert_full (a) ; Index (index) } #[doc = " Applies the (partial) function to each edge and returns a new"] #[doc = " relation builder. If `f` returns `None` for any end-point,"] #[doc = " returns `None`."] pub fn maybe_map < F , U > (& self , mut f : F) -> Option < TransitiveRelationBuilder < U > > where F : FnMut (T) -> Option < U > , U : Clone + Debug + Eq + Hash + Copy , { let mut result = TransitiveRelationBuilder :: default () ; for edge in & self . edges { result . add (f (self . elements [edge . source . 0]) ? , f (self . elements [edge . target . 0]) ?) ; } Some (result) } #[doc = " Indicate that `a < b` (where `<` is this relation)"] pub fn add (& mut self , a : T , b : T) { let a = self . add_index (a) ; let b = self . add_index (b) ; let edge = Edge { source : a , target : b } ; self . edges . insert (edge) ; } #[doc = " Compute the transitive closure derived from the edges, and converted to"] #[doc = " the final result. After this, all elements will be immutable to maintain"] #[doc = " the correctness of the result."] pub fn freeze (self) -> TransitiveRelation < T > { let mut matrix = BitMatrix :: new (self . elements . len () , self . elements . len ()) ; let mut changed = true ; while changed { changed = false ; for edge in & self . edges { changed |= matrix . insert (edge . source . 0 , edge . target . 0) ; changed |= matrix . union_rows (edge . target . 0 , edge . source . 0) ; } } TransitiveRelation { builder : Frozen :: freeze (self) , closure : Frozen :: freeze (matrix) } } }}}
mkitem!{mkimpl!{impl < T : Eq + Hash + Copy > TransitiveRelation < T > { #[doc = " Applies the (partial) function to each edge and returns a new"] #[doc = " relation including transitive closures."] pub fn maybe_map < F , U > (& self , f : F) -> Option < TransitiveRelation < U > > where F : FnMut (T) -> Option < U > , U : Clone + Debug + Eq + Hash + Copy , { Some (self . builder . maybe_map (f) ? . freeze ()) } #[doc = " Checks whether `a < target` (transitively)"] pub fn contains (& self , a : T , b : T) -> bool { match (self . index (a) , self . index (b)) { (Some (a) , Some (b)) => self . with_closure (| closure | closure . contains (a . 0 , b . 0)) , (None , _) | (_ , None) => false , } } #[doc = " Thinking of `x R y` as an edge `x -> y` in a graph, this"] #[doc = " returns all things reachable from `a`."] #[doc = ""] #[doc = " Really this probably ought to be `impl Iterator<Item = &T>`, but"] #[doc = " I'm too lazy to make that work, and -- given the caching"] #[doc = " strategy -- it'd be a touch tricky anyhow."] pub fn reachable_from (& self , a : T) -> Vec < T > { match self . index (a) { Some (a) => { self . with_closure (| closure | closure . iter (a . 0) . map (| i | self . elements [i]) . collect ()) } None => vec ! [] , } } #[doc = " Picks what I am referring to as the \"postdominating\""] #[doc = " upper-bound for `a` and `b`. This is usually the least upper"] #[doc = " bound, but in cases where there is no single least upper"] #[doc = " bound, it is the \"mutual immediate postdominator\", if you"] #[doc = " imagine a graph where `a < b` means `a -> b`."] #[doc = ""] #[doc = " This function is needed because region inference currently"] #[doc = " requires that we produce a single \"UB\", and there is no best"] #[doc = " choice for the LUB. Rather than pick arbitrarily, I pick a"] #[doc = " less good, but predictable choice. This should help ensure"] #[doc = " that region inference yields predictable results (though it"] #[doc = " itself is not fully sufficient)."] #[doc = ""] #[doc = " Examples are probably clearer than any prose I could write"] #[doc = " (there are corresponding tests below, btw). In each case,"] #[doc = " the query is `postdom_upper_bound(a, b)`:"] #[doc = ""] #[doc = " ```text"] #[doc = " // Returns Some(x), which is also LUB."] #[doc = " a -> a1 -> x"] #[doc = "            ^"] #[doc = "            |"] #[doc = " b -> b1 ---+"] #[doc = ""] #[doc = " // Returns `Some(x)`, which is not LUB (there is none)"] #[doc = " // diagonal edges run left-to-right."] #[doc = " a -> a1 -> x"] #[doc = "   \\/       ^"] #[doc = "   /\\       |"] #[doc = " b -> b1 ---+"] #[doc = ""] #[doc = " // Returns `None`."] #[doc = " a -> a1"] #[doc = " b -> b1"] #[doc = " ```"] pub fn postdom_upper_bound (& self , a : T , b : T) -> Option < T > { let mubs = self . minimal_upper_bounds (a , b) ; self . mutual_immediate_postdominator (mubs) } #[doc = " Viewing the relation as a graph, computes the \"mutual"] #[doc = " immediate postdominator\" of a set of points (if one"] #[doc = " exists). See `postdom_upper_bound` for details."] pub fn mutual_immediate_postdominator (& self , mut mubs : Vec < T >) -> Option < T > { loop { match mubs [..] { [] => return None , [mub] => return Some (mub) , _ => { let m = mubs . pop () . unwrap () ; let n = mubs . pop () . unwrap () ; mubs . extend (self . minimal_upper_bounds (n , m)) ; } } } } #[doc = " Returns the set of bounds `X` such that:"] #[doc = ""] #[doc = " - `a < X` and `b < X`"] #[doc = " - there is no `Y != X` such that `a < Y` and `Y < X`"] #[doc = "   - except for the case where `X < a` (i.e., a strongly connected"] #[doc = "     component in the graph). In that case, the smallest"] #[doc = "     representative of the SCC is returned (as determined by the"] #[doc = "     internal indices)."] #[doc = ""] #[doc = " Note that this set can, in principle, have any size."] pub fn minimal_upper_bounds (& self , a : T , b : T) -> Vec < T > { let (Some (mut a) , Some (mut b)) = (self . index (a) , self . index (b)) else { return vec ! [] ; } ; if a > b { mem :: swap (& mut a , & mut b) ; } let lub_indices = self . with_closure (| closure | { if closure . contains (a . 0 , b . 0) { return vec ! [b . 0] ; } if closure . contains (b . 0 , a . 0) { return vec ! [a . 0] ; } let mut candidates = closure . intersect_rows (a . 0 , b . 0) ; pare_down (& mut candidates , closure) ; candidates . reverse () ; pare_down (& mut candidates , closure) ; candidates }) ; lub_indices . into_iter () . rev () . map (| i | self . elements [i]) . collect () } #[doc = " Given an element A, returns the maximal set {B} of elements B"] #[doc = " such that"] #[doc = ""] #[doc = " - A != B"] #[doc = " - A R B is true"] #[doc = " - for each i, j: `B[i]` R `B[j]` does not hold"] #[doc = ""] #[doc = " The intuition is that this moves \"one step up\" through a lattice"] #[doc = " (where the relation is encoding the `<=` relation for the lattice)."] #[doc = " So e.g., if the relation is `->` and we have"] #[doc = ""] #[doc = " ```text"] #[doc = " a -> b -> d -> f"] #[doc = " |              ^"] #[doc = " +--> c -> e ---+"] #[doc = " ```"] #[doc = ""] #[doc = " then `parents(a)` returns `[b, c]`. The `postdom_parent` function"] #[doc = " would further reduce this to just `f`."] pub fn parents (& self , a : T) -> Vec < T > { let Some (a) = self . index (a) else { return vec ! [] ; } ; let ancestors = self . with_closure (| closure | { let mut ancestors = closure . intersect_rows (a . 0 , a . 0) ; ancestors . retain (| & e | ! closure . contains (e , a . 0)) ; pare_down (& mut ancestors , closure) ; ancestors . reverse () ; pare_down (& mut ancestors , closure) ; ancestors }) ; ancestors . into_iter () . rev () . map (| i | self . elements [i]) . collect () } #[doc = " Given an element A, elements B with the lowest index such that `A R B`"] #[doc = " and `B R A`, or `A` if no such element exists."] pub fn minimal_scc_representative (& self , a : T) -> T { match self . index (a) { Some (a_i) => self . with_closure (| closure | { closure . iter (a_i . 0) . find (| i | closure . contains (* i , a_i . 0)) . map_or (a , | i | self . elements [i]) }) , None => a , } } fn with_closure < OP , R > (& self , op : OP) -> R where OP : FnOnce (& BitMatrix < usize , usize >) -> R , { op (& self . closure) } #[doc = " Lists all the base edges in the graph: the initial _non-transitive_ set of element"] #[doc = " relations, which will be later used as the basis for the transitive closure computation."] pub fn base_edges (& self) -> impl Iterator < Item = (T , T) > { self . edges . iter () . map (move | edge | (self . elements [edge . source . 0] , self . elements [edge . target . 0])) } }}}

macro_rules! pare_down_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pare_down in module {}", module_path!());
    };
}

mkfn!{
    pare_down_introspect!();
    #[doc = " Pare down is used as a step in the LUB computation. It edits the"] #[doc = " candidates array in place by removing any element j for which"] #[doc = " there exists an earlier element i<j such that i -> j. That is,"] #[doc = " after you run `pare_down`, you know that for all elements that"] #[doc = " remain in candidates, they cannot reach any of the elements that"] #[doc = " come after them."] #[doc = ""] #[doc = " Examples follow. Assume that a -> b -> c and x -> y -> z."] #[doc = ""] #[doc = " - Input: `[a, b, x]`. Output: `[a, x]`."] #[doc = " - Input: `[b, a, x]`. Output: `[b, a, x]`."] #[doc = " - Input: `[a, x, b, y]`. Output: `[a, x]`."] fn pare_down (candidates : & mut Vec < usize > , closure : & BitMatrix < usize , usize >) { let mut i = 0 ; while let Some (& candidate_i) = candidates . get (i) { i += 1 ; let mut j = i ; let mut dead = 0 ; while let Some (& candidate_j) = candidates . get (j) { if closure . contains (candidate_i , candidate_j) { dead += 1 ; } else { candidates [j - dead] = candidate_j ; } j += 1 ; } candidates . truncate (j - dead) ; } }
}