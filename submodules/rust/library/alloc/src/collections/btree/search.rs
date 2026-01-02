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
mkuse!{use core :: borrow :: Borrow ;}
mkuse!{use core :: cmp :: Ordering ;}
mkuse!{use core :: ops :: { Bound , RangeBounds } ;}
mkuse!{use SearchBound :: * ;}
mkuse!{use SearchResult :: * ;}
mkuse!{use super :: node :: ForceResult :: * ;}
mkuse!{use super :: node :: { Handle , NodeRef , marker } ;}
mkitem!{mkenum!{pub (super) enum SearchBound < T > { #[doc = " An inclusive bound to look for, just like `Bound::Included(T)`."] Included (T) , #[doc = " An exclusive bound to look for, just like `Bound::Excluded(T)`."] Excluded (T) , #[doc = " An unconditional inclusive bound, just like `Bound::Unbounded`."] AllIncluded , #[doc = " An unconditional exclusive bound."] AllExcluded , }}}
mkitem!{mkimpl!{impl < T > SearchBound < T > { pub (super) fn from_range (range_bound : Bound < T >) -> Self { match range_bound { Bound :: Included (t) => Included (t) , Bound :: Excluded (t) => Excluded (t) , Bound :: Unbounded => AllIncluded , } } }}}
mkitem!{mkenum!{pub (super) enum SearchResult < BorrowType , K , V , FoundType , GoDownType > { Found (Handle < NodeRef < BorrowType , K , V , FoundType > , marker :: KV >) , GoDown (Handle < NodeRef < BorrowType , K , V , GoDownType > , marker :: Edge >) , }}}
mkitem!{mkenum!{pub (super) enum IndexResult { KV (usize) , Edge (usize) , }}}
mkitem!{mkimpl!{impl < BorrowType : marker :: BorrowType , K , V > NodeRef < BorrowType , K , V , marker :: LeafOrInternal > { #[doc = " Looks up a given key in a (sub)tree headed by the node, recursively."] #[doc = " Returns a `Found` with the handle of the matching KV, if any. Otherwise,"] #[doc = " returns a `GoDown` with the handle of the leaf edge where the key belongs."] #[doc = ""] #[doc = " The result is meaningful only if the tree is ordered by key, like the tree"] #[doc = " in a `BTreeMap` is."] pub (super) fn search_tree < Q : ? Sized > (mut self , key : & Q ,) -> SearchResult < BorrowType , K , V , marker :: LeafOrInternal , marker :: Leaf > where Q : Ord , K : Borrow < Q > , { loop { self = match self . search_node (key) { Found (handle) => return Found (handle) , GoDown (handle) => match handle . force () { Leaf (leaf) => return GoDown (leaf) , Internal (internal) => internal . descend () , } , } } } #[doc = " Descends to the nearest node where the edge matching the lower bound"] #[doc = " of the range is different from the edge matching the upper bound, i.e.,"] #[doc = " the nearest node that has at least one key contained in the range."] #[doc = ""] #[doc = " If found, returns an `Ok` with that node, the strictly ascending pair of"] #[doc = " edge indices in the node delimiting the range, and the corresponding"] #[doc = " pair of bounds for continuing the search in the child nodes, in case"] #[doc = " the node is internal."] #[doc = ""] #[doc = " If not found, returns an `Err` with the leaf edge matching the entire"] #[doc = " range."] #[doc = ""] #[doc = " As a diagnostic service, panics if the range specifies impossible bounds."] #[doc = ""] #[doc = " The result is meaningful only if the tree is ordered by key."] pub (super) fn search_tree_for_bifurcation < 'r , Q : ? Sized , R > (mut self , range : & 'r R ,) -> Result < (NodeRef < BorrowType , K , V , marker :: LeafOrInternal > , usize , usize , SearchBound < & 'r Q > , SearchBound < & 'r Q > ,) , Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > , > where Q : Ord , K : Borrow < Q > , R : RangeBounds < Q > , { let is_set = < V as super :: set_val :: IsSetVal > :: is_set_val () ; let (start , end) = (range . start_bound () , range . end_bound ()) ; match (start , end) { (Bound :: Excluded (s) , Bound :: Excluded (e)) if s == e => { if is_set { panic ! ("range start and end are equal and excluded in BTreeSet") } else { panic ! ("range start and end are equal and excluded in BTreeMap") } } (Bound :: Included (s) | Bound :: Excluded (s) , Bound :: Included (e) | Bound :: Excluded (e)) if s > e => { if is_set { panic ! ("range start is greater than range end in BTreeSet") } else { panic ! ("range start is greater than range end in BTreeMap") } } _ => { } } let mut lower_bound = SearchBound :: from_range (start) ; let mut upper_bound = SearchBound :: from_range (end) ; loop { let (lower_edge_idx , lower_child_bound) = self . find_lower_bound_index (lower_bound) ; let (upper_edge_idx , upper_child_bound) = unsafe { self . find_upper_bound_index (upper_bound , lower_edge_idx) } ; if lower_edge_idx < upper_edge_idx { return Ok ((self , lower_edge_idx , upper_edge_idx , lower_child_bound , upper_child_bound ,)) ; } debug_assert_eq ! (lower_edge_idx , upper_edge_idx) ; let common_edge = unsafe { Handle :: new_edge (self , lower_edge_idx) } ; match common_edge . force () { Leaf (common_edge) => return Err (common_edge) , Internal (common_edge) => { self = common_edge . descend () ; lower_bound = lower_child_bound ; upper_bound = upper_child_bound ; } } } } #[doc = " Finds an edge in the node delimiting the lower bound of a range."] #[doc = " Also returns the lower bound to be used for continuing the search in"] #[doc = " the matching child node, if `self` is an internal node."] #[doc = ""] #[doc = " The result is meaningful only if the tree is ordered by key."] pub (super) fn find_lower_bound_edge < 'r , Q > (self , bound : SearchBound < & 'r Q > ,) -> (Handle < Self , marker :: Edge > , SearchBound < & 'r Q >) where Q : ? Sized + Ord , K : Borrow < Q > , { let (edge_idx , bound) = self . find_lower_bound_index (bound) ; let edge = unsafe { Handle :: new_edge (self , edge_idx) } ; (edge , bound) } #[doc = " Clone of `find_lower_bound_edge` for the upper bound."] pub (super) fn find_upper_bound_edge < 'r , Q > (self , bound : SearchBound < & 'r Q > ,) -> (Handle < Self , marker :: Edge > , SearchBound < & 'r Q >) where Q : ? Sized + Ord , K : Borrow < Q > , { let (edge_idx , bound) = unsafe { self . find_upper_bound_index (bound , 0) } ; let edge = unsafe { Handle :: new_edge (self , edge_idx) } ; (edge , bound) } }}}
mkitem!{mkimpl!{impl < BorrowType , K , V , Type > NodeRef < BorrowType , K , V , Type > { #[doc = " Looks up a given key in the node, without recursion."] #[doc = " Returns a `Found` with the handle of the matching KV, if any. Otherwise,"] #[doc = " returns a `GoDown` with the handle of the edge where the key might be found"] #[doc = " (if the node is internal) or where the key can be inserted."] #[doc = ""] #[doc = " The result is meaningful only if the tree is ordered by key, like the tree"] #[doc = " in a `BTreeMap` is."] pub (super) fn search_node < Q : ? Sized > (self , key : & Q ,) -> SearchResult < BorrowType , K , V , Type , Type > where Q : Ord , K : Borrow < Q > , { match unsafe { self . find_key_index (key , 0) } { IndexResult :: KV (idx) => Found (unsafe { Handle :: new_kv (self , idx) }) , IndexResult :: Edge (idx) => GoDown (unsafe { Handle :: new_edge (self , idx) }) , } } #[doc = " Returns either the KV index in the node at which the key (or an equivalent)"] #[doc = " exists, or the edge index where the key belongs, starting from a particular index."] #[doc = ""] #[doc = " The result is meaningful only if the tree is ordered by key, like the tree"] #[doc = " in a `BTreeMap` is."] #[doc = ""] #[doc = " # Safety"] #[doc = " `start_index` must be a valid edge index for the node."] unsafe fn find_key_index < Q : ? Sized > (& self , key : & Q , start_index : usize) -> IndexResult where Q : Ord , K : Borrow < Q > , { let node = self . reborrow () ; let keys = node . keys () ; debug_assert ! (start_index <= keys . len ()) ; for (offset , k) in unsafe { keys . get_unchecked (start_index ..) } . iter () . enumerate () { match key . cmp (k . borrow ()) { Ordering :: Greater => { } Ordering :: Equal => return IndexResult :: KV (start_index + offset) , Ordering :: Less => return IndexResult :: Edge (start_index + offset) , } } IndexResult :: Edge (keys . len ()) } #[doc = " Finds an edge index in the node delimiting the lower bound of a range."] #[doc = " Also returns the lower bound to be used for continuing the search in"] #[doc = " the matching child node, if `self` is an internal node."] #[doc = ""] #[doc = " The result is meaningful only if the tree is ordered by key."] fn find_lower_bound_index < 'r , Q > (& self , bound : SearchBound < & 'r Q > ,) -> (usize , SearchBound < & 'r Q >) where Q : ? Sized + Ord , K : Borrow < Q > , { match bound { Included (key) => match unsafe { self . find_key_index (key , 0) } { IndexResult :: KV (idx) => (idx , AllExcluded) , IndexResult :: Edge (idx) => (idx , bound) , } , Excluded (key) => match unsafe { self . find_key_index (key , 0) } { IndexResult :: KV (idx) => (idx + 1 , AllIncluded) , IndexResult :: Edge (idx) => (idx , bound) , } , AllIncluded => (0 , AllIncluded) , AllExcluded => (self . len () , AllExcluded) , } } #[doc = " Mirror image of `find_lower_bound_index` for the upper bound,"] #[doc = " with an additional parameter to skip part of the key array."] #[doc = ""] #[doc = " # Safety"] #[doc = " `start_index` must be a valid edge index for the node."] unsafe fn find_upper_bound_index < 'r , Q > (& self , bound : SearchBound < & 'r Q > , start_index : usize ,) -> (usize , SearchBound < & 'r Q >) where Q : ? Sized + Ord , K : Borrow < Q > , { match bound { Included (key) => match unsafe { self . find_key_index (key , start_index) } { IndexResult :: KV (idx) => (idx + 1 , AllExcluded) , IndexResult :: Edge (idx) => (idx , bound) , } , Excluded (key) => match unsafe { self . find_key_index (key , start_index) } { IndexResult :: KV (idx) => (idx , AllIncluded) , IndexResult :: Edge (idx) => (idx , bound) , } , AllIncluded => (self . len () , AllIncluded) , AllExcluded => (start_index , AllExcluded) , } } }}}