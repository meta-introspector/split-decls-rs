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
mkuse!{use intravisit :: InferKind ;}
mkuse!{use rustc_data_structures :: sorted_map :: SortedMap ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def_id :: { LocalDefId , LocalDefIdMap } ;}
mkuse!{use rustc_hir :: intravisit :: Visitor ;}
mkuse!{use rustc_hir :: * ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_middle :: span_bug ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_span :: { DUMMY_SP , Span } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkitem!{mkstruct!{#[doc = " A visitor that walks over the HIR and collects `Node`s into a HIR map."] struct NodeCollector < 'a , 'hir > { tcx : TyCtxt < 'hir > , bodies : & 'a SortedMap < ItemLocalId , & 'hir Body < 'hir > > , #[doc = " Outputs"] nodes : IndexVec < ItemLocalId , ParentedNode < 'hir > > , parenting : LocalDefIdMap < ItemLocalId > , #[doc = " The parent of this node"] parent_node : ItemLocalId , owner : OwnerId , }}}

macro_rules! index_hir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function index_hir in module {}", module_path!());
    };
}

mkfn!{
    index_hir_introspect!();
    #[instrument (level = "debug" , skip (tcx , bodies))] pub (super) fn index_hir < 'hir > (tcx : TyCtxt < 'hir > , item : hir :: OwnerNode < 'hir > , bodies : & SortedMap < ItemLocalId , & 'hir Body < 'hir > > , num_nodes : usize ,) -> (IndexVec < ItemLocalId , ParentedNode < 'hir > > , LocalDefIdMap < ItemLocalId >) { let err_node = ParentedNode { parent : ItemLocalId :: ZERO , node : Node :: Err (item . span ()) } ; let mut nodes = IndexVec :: from_elem_n (err_node , num_nodes) ; nodes [ItemLocalId :: ZERO] = ParentedNode { parent : ItemLocalId :: INVALID , node : item . into () } ; let mut collector = NodeCollector { tcx , owner : item . def_id () , parent_node : ItemLocalId :: ZERO , nodes , bodies , parenting : Default :: default () , } ; match item { OwnerNode :: Crate (citem) => { collector . visit_mod (citem , citem . spans . inner_span , hir :: CRATE_HIR_ID) } OwnerNode :: Item (item) => collector . visit_item (item) , OwnerNode :: TraitItem (item) => collector . visit_trait_item (item) , OwnerNode :: ImplItem (item) => collector . visit_impl_item (item) , OwnerNode :: ForeignItem (item) => collector . visit_foreign_item (item) , OwnerNode :: Synthetic => unreachable ! () , } ; for (local_id , node) in collector . nodes . iter_enumerated () { if let Node :: Err (span) = node . node { let hir_id = HirId { owner : item . def_id () , local_id } ; let msg = format ! ("ID {hir_id} not encountered when visiting item HIR") ; tcx . dcx () . span_delayed_bug (span , msg) ; } } (collector . nodes , collector . parenting) }
}
mkitem!{mkimpl!{impl < 'a , 'hir > NodeCollector < 'a , 'hir > { #[instrument (level = "debug" , skip (self))] fn insert (& mut self , span : Span , hir_id : HirId , node : Node < 'hir >) { debug_assert_eq ! (self . owner , hir_id . owner) ; debug_assert_ne ! (hir_id . local_id . as_u32 () , 0) ; debug_assert_ne ! (hir_id . local_id , self . parent_node) ; if cfg ! (debug_assertions) { if hir_id . owner != self . owner { span_bug ! (span , "inconsistent HirId at `{:?}` for `{node:?}`: \
                     current_dep_node_owner={} ({:?}), hir_id.owner={} ({:?})" , self . tcx . sess . source_map () . span_to_diagnostic_string (span) , self . tcx . definitions_untracked () . def_path (self . owner . def_id) . to_string_no_crate_verbose () , self . owner , self . tcx . definitions_untracked () . def_path (hir_id . owner . def_id) . to_string_no_crate_verbose () , hir_id . owner ,) } if self . tcx . sess . opts . incremental . is_some () && span . parent () . is_none () && ! span . is_dummy () { span_bug ! (span , "span without a parent: {:#?}, {node:?}" , span . data ()) } } self . nodes [hir_id . local_id] = ParentedNode { parent : self . parent_node , node } ; } fn with_parent < F : FnOnce (& mut Self) > (& mut self , parent_node_id : HirId , f : F) { debug_assert_eq ! (parent_node_id . owner , self . owner) ; let parent_node = self . parent_node ; self . parent_node = parent_node_id . local_id ; f (self) ; self . parent_node = parent_node ; } fn insert_nested (& mut self , item : LocalDefId) { if self . parent_node != ItemLocalId :: ZERO { self . parenting . insert (item , self . parent_node) ; } } }}}
mkitem!{mkimpl!{impl < 'a , 'hir > Visitor < 'hir > for NodeCollector < 'a , 'hir > { #[doc = " Because we want to track parent items and so forth, enable"] #[doc = " deep walking so that we walk nested items in the context of"] #[doc = " their outer items."] fn visit_nested_item (& mut self , item : ItemId) { debug ! ("visit_nested_item: {:?}" , item) ; self . insert_nested (item . owner_id . def_id) ; } fn visit_nested_trait_item (& mut self , item_id : TraitItemId) { self . insert_nested (item_id . owner_id . def_id) ; } fn visit_nested_impl_item (& mut self , item_id : ImplItemId) { self . insert_nested (item_id . owner_id . def_id) ; } fn visit_nested_foreign_item (& mut self , foreign_id : ForeignItemId) { self . insert_nested (foreign_id . owner_id . def_id) ; } fn visit_nested_body (& mut self , id : BodyId) { debug_assert_eq ! (id . hir_id . owner , self . owner) ; let body = self . bodies [& id . hir_id . local_id] ; self . visit_body (body) ; } fn visit_param (& mut self , param : & 'hir Param < 'hir >) { let node = Node :: Param (param) ; self . insert (param . pat . span , param . hir_id , node) ; self . with_parent (param . hir_id , | this | { intravisit :: walk_param (this , param) ; }) ; } #[instrument (level = "debug" , skip (self))] fn visit_item (& mut self , i : & 'hir Item < 'hir >) { debug_assert_eq ! (i . owner_id , self . owner) ; self . with_parent (i . hir_id () , | this | { if let ItemKind :: Struct (_ , _ , struct_def) = & i . kind && let Some (ctor_hir_id) = struct_def . ctor_hir_id () { this . insert (i . span , ctor_hir_id , Node :: Ctor (struct_def)) ; } intravisit :: walk_item (this , i) ; }) ; } #[instrument (level = "debug" , skip (self))] fn visit_foreign_item (& mut self , fi : & 'hir ForeignItem < 'hir >) { debug_assert_eq ! (fi . owner_id , self . owner) ; self . with_parent (fi . hir_id () , | this | { intravisit :: walk_foreign_item (this , fi) ; }) ; } fn visit_generic_param (& mut self , param : & 'hir GenericParam < 'hir >) { self . insert (param . span , param . hir_id , Node :: GenericParam (param)) ; intravisit :: walk_generic_param (self , param) ; } fn visit_const_param_default (& mut self , param : HirId , ct : & 'hir ConstArg < 'hir >) { self . with_parent (param , | this | { intravisit :: walk_const_param_default (this , ct) ; }) } #[instrument (level = "debug" , skip (self))] fn visit_trait_item (& mut self , ti : & 'hir TraitItem < 'hir >) { debug_assert_eq ! (ti . owner_id , self . owner) ; self . with_parent (ti . hir_id () , | this | { intravisit :: walk_trait_item (this , ti) ; }) ; } #[instrument (level = "debug" , skip (self))] fn visit_impl_item (& mut self , ii : & 'hir ImplItem < 'hir >) { debug_assert_eq ! (ii . owner_id , self . owner) ; self . with_parent (ii . hir_id () , | this | { intravisit :: walk_impl_item (this , ii) ; }) ; } fn visit_pat (& mut self , pat : & 'hir Pat < 'hir >) { self . insert (pat . span , pat . hir_id , Node :: Pat (pat)) ; self . with_parent (pat . hir_id , | this | { intravisit :: walk_pat (this , pat) ; }) ; } fn visit_pat_expr (& mut self , expr : & 'hir PatExpr < 'hir >) { self . insert (expr . span , expr . hir_id , Node :: PatExpr (expr)) ; self . with_parent (expr . hir_id , | this | { intravisit :: walk_pat_expr (this , expr) ; }) ; } fn visit_pat_field (& mut self , field : & 'hir PatField < 'hir >) { self . insert (field . span , field . hir_id , Node :: PatField (field)) ; self . with_parent (field . hir_id , | this | { intravisit :: walk_pat_field (this , field) ; }) ; } fn visit_arm (& mut self , arm : & 'hir Arm < 'hir >) { let node = Node :: Arm (arm) ; self . insert (arm . span , arm . hir_id , node) ; self . with_parent (arm . hir_id , | this | { intravisit :: walk_arm (this , arm) ; }) ; } fn visit_opaque_ty (& mut self , opaq : & 'hir OpaqueTy < 'hir >) { self . insert (opaq . span , opaq . hir_id , Node :: OpaqueTy (opaq)) ; self . with_parent (opaq . hir_id , | this | { intravisit :: walk_opaque_ty (this , opaq) ; }) ; } fn visit_anon_const (& mut self , constant : & 'hir AnonConst) { self . insert (constant . span , constant . hir_id , Node :: AnonConst (constant)) ; self . with_parent (constant . hir_id , | this | { intravisit :: walk_anon_const (this , constant) ; }) ; } fn visit_inline_const (& mut self , constant : & 'hir ConstBlock) { self . insert (DUMMY_SP , constant . hir_id , Node :: ConstBlock (constant)) ; self . with_parent (constant . hir_id , | this | { intravisit :: walk_inline_const (this , constant) ; }) ; } fn visit_expr (& mut self , expr : & 'hir Expr < 'hir >) { self . insert (expr . span , expr . hir_id , Node :: Expr (expr)) ; self . with_parent (expr . hir_id , | this | { intravisit :: walk_expr (this , expr) ; }) ; } fn visit_expr_field (& mut self , field : & 'hir ExprField < 'hir >) { self . insert (field . span , field . hir_id , Node :: ExprField (field)) ; self . with_parent (field . hir_id , | this | { intravisit :: walk_expr_field (this , field) ; }) ; } fn visit_stmt (& mut self , stmt : & 'hir Stmt < 'hir >) { self . insert (stmt . span , stmt . hir_id , Node :: Stmt (stmt)) ; self . with_parent (stmt . hir_id , | this | { intravisit :: walk_stmt (this , stmt) ; }) ; } fn visit_path_segment (& mut self , path_segment : & 'hir PathSegment < 'hir >) { self . insert (path_segment . ident . span , path_segment . hir_id , Node :: PathSegment (path_segment)) ; intravisit :: walk_path_segment (self , path_segment) ; } fn visit_ty (& mut self , ty : & 'hir Ty < 'hir , AmbigArg >) { self . insert (ty . span , ty . hir_id , Node :: Ty (ty . as_unambig_ty ())) ; self . with_parent (ty . hir_id , | this | { intravisit :: walk_ty (this , ty) ; }) ; } fn visit_const_arg (& mut self , const_arg : & 'hir ConstArg < 'hir , AmbigArg >) { self . insert (const_arg . as_unambig_ct () . span () , const_arg . hir_id , Node :: ConstArg (const_arg . as_unambig_ct ()) ,) ; self . with_parent (const_arg . hir_id , | this | { intravisit :: walk_const_arg (this , const_arg) ; }) ; } fn visit_infer (& mut self , inf_id : HirId , inf_span : Span , kind : InferKind < 'hir > ,) -> Self :: Result { match kind { InferKind :: Ty (ty) => self . insert (inf_span , inf_id , Node :: Ty (ty)) , InferKind :: Const (ct) => self . insert (inf_span , inf_id , Node :: ConstArg (ct)) , InferKind :: Ambig (inf) => self . insert (inf_span , inf_id , Node :: Infer (inf)) , } self . visit_id (inf_id) ; } fn visit_trait_ref (& mut self , tr : & 'hir TraitRef < 'hir >) { self . insert (tr . path . span , tr . hir_ref_id , Node :: TraitRef (tr)) ; self . with_parent (tr . hir_ref_id , | this | { intravisit :: walk_trait_ref (this , tr) ; }) ; } fn visit_block (& mut self , block : & 'hir Block < 'hir >) { self . insert (block . span , block . hir_id , Node :: Block (block)) ; self . with_parent (block . hir_id , | this | { intravisit :: walk_block (this , block) ; }) ; } fn visit_local (& mut self , l : & 'hir LetStmt < 'hir >) { self . insert (l . span , l . hir_id , Node :: LetStmt (l)) ; self . with_parent (l . hir_id , | this | { intravisit :: walk_local (this , l) ; }) } fn visit_lifetime (& mut self , lifetime : & 'hir Lifetime) { self . insert (lifetime . ident . span , lifetime . hir_id , Node :: Lifetime (lifetime)) ; } fn visit_variant (& mut self , v : & 'hir Variant < 'hir >) { self . insert (v . span , v . hir_id , Node :: Variant (v)) ; self . with_parent (v . hir_id , | this | { if let Some (ctor_hir_id) = v . data . ctor_hir_id () { this . insert (v . span , ctor_hir_id , Node :: Ctor (& v . data)) ; } intravisit :: walk_variant (this , v) ; }) ; } fn visit_field_def (& mut self , field : & 'hir FieldDef < 'hir >) { self . insert (field . span , field . hir_id , Node :: Field (field)) ; self . with_parent (field . hir_id , | this | { intravisit :: walk_field_def (this , field) ; }) ; } fn visit_assoc_item_constraint (& mut self , constraint : & 'hir AssocItemConstraint < 'hir >) { self . insert (constraint . span , constraint . hir_id , Node :: AssocItemConstraint (constraint)) ; self . with_parent (constraint . hir_id , | this | { intravisit :: walk_assoc_item_constraint (this , constraint) }) } fn visit_trait_item_ref (& mut self , id : & 'hir TraitItemId) { self . visit_nested_trait_item (* id) ; } fn visit_impl_item_ref (& mut self , id : & 'hir ImplItemId) { self . visit_nested_impl_item (* id) ; } fn visit_foreign_item_ref (& mut self , id : & 'hir ForeignItemId) { self . visit_nested_foreign_item (* id) ; } fn visit_where_predicate (& mut self , predicate : & 'hir WherePredicate < 'hir >) { self . insert (predicate . span , predicate . hir_id , Node :: WherePredicate (predicate)) ; self . with_parent (predicate . hir_id , | this | { intravisit :: walk_where_predicate (this , predicate) }) ; } fn visit_pattern_type_pattern (& mut self , pat : & 'hir hir :: TyPat < 'hir >) { self . insert (pat . span , pat . hir_id , Node :: TyPat (pat)) ; self . with_parent (pat . hir_id , | this | { intravisit :: walk_ty_pat (this , pat) ; }) ; } fn visit_precise_capturing_arg (& mut self , arg : & 'hir PreciseCapturingArg < 'hir > ,) -> Self :: Result { match arg { PreciseCapturingArg :: Lifetime (_) => { } PreciseCapturingArg :: Param (param) => self . insert (param . ident . span , param . hir_id , Node :: PreciseCapturingNonLifetimeArg (param) ,) , } intravisit :: walk_precise_capturing_arg (self , arg) ; } }}}