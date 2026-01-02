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
mkuse!{use std :: cmp :: Ordering ;}
mkuse!{use std :: ops :: { Index , IndexMut } ;}
mkuse!{use std :: { mem , slice } ;}
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_data_structures :: graph :: dominators :: Dominators ;}
mkuse!{use rustc_data_structures :: graph :: { self , DirectedGraph , StartNode } ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_index :: bit_set :: DenseBitSet ;}
mkuse!{pub (crate) use rustc_middle :: mir :: coverage :: { BasicCoverageBlock , START_BCB } ;}
mkuse!{use rustc_middle :: mir :: { self , BasicBlock , Terminator , TerminatorKind } ;}
mkuse!{use tracing :: debug ;}
mkitem!{mkstruct!{# [doc = " A coverage-specific simplification of the MIR control flow graph (CFG). The `CoverageGraph`s"] # [doc = " nodes are `BasicCoverageBlock`s, which encompass one or more MIR `BasicBlock`s."] # [derive (Debug)] pub (crate) struct CoverageGraph { bcbs : IndexVec < BasicCoverageBlock , BasicCoverageBlockData > , bb_to_bcb : IndexVec < BasicBlock , Option < BasicCoverageBlock > > , pub (crate) successors : IndexVec < BasicCoverageBlock , Vec < BasicCoverageBlock > > , pub (crate) predecessors : IndexVec < BasicCoverageBlock , Vec < BasicCoverageBlock > > , dominators : Option < Dominators < BasicCoverageBlock > > , # [doc = " Allows nodes to be compared in some total order such that _if_"] # [doc = " `a` dominates `b`, then `a < b`. If neither node dominates the other,"] # [doc = " their relative order is consistent but arbitrary."] dominator_order_rank : IndexVec < BasicCoverageBlock , u32 > , # [doc = " A loop header is a node that dominates one or more of its predecessors."] is_loop_header : DenseBitSet < BasicCoverageBlock > , # [doc = " For each node, the loop header node of its nearest enclosing loop."] # [doc = " This forms a linked list that can be traversed to find all enclosing loops."] enclosing_loop_header : IndexVec < BasicCoverageBlock , Option < BasicCoverageBlock > > , }}}
mkitem!{mkimpl!{impl CoverageGraph { pub (crate) fn from_mir (mir_body : & mir :: Body < '_ >) -> Self { let (bcbs , bb_to_bcb) = Self :: compute_basic_coverage_blocks (mir_body) ; let successors = IndexVec :: < BasicCoverageBlock , _ > :: from_fn_n (| bcb | { let mut seen_bcbs = FxHashSet :: default () ; let terminator = mir_body [bcbs [bcb] . last_bb ()] . terminator () ; bcb_filtered_successors (terminator) . into_iter () . filter_map (| successor_bb | bb_to_bcb [successor_bb]) . filter (| & successor_bcb | seen_bcbs . insert (successor_bcb)) . collect :: < Vec < _ > > () } , bcbs . len () ,) ; let mut predecessors = IndexVec :: from_elem (Vec :: new () , & bcbs) ; for (bcb , bcb_successors) in successors . iter_enumerated () { for & successor in bcb_successors { predecessors [successor] . push (bcb) ; } } let num_nodes = bcbs . len () ; let mut this = Self { bcbs , bb_to_bcb , successors , predecessors , dominators : None , dominator_order_rank : IndexVec :: from_elem_n (0 , num_nodes) , is_loop_header : DenseBitSet :: new_empty (num_nodes) , enclosing_loop_header : IndexVec :: from_elem_n (None , num_nodes) , } ; assert_eq ! (num_nodes , this . num_nodes ()) ; this . dominators = Some (graph :: dominators :: dominators (& this)) ; let dominator_order = graph :: iterate :: reverse_post_order (& this , this . start_node ()) ; assert_eq ! (dominator_order . len () , this . num_nodes ()) ; for (rank , bcb) in (0u32 ..) . zip (dominator_order) { this . dominator_order_rank [bcb] = rank ; if this . reloop_predecessors (bcb) . next () . is_some () { this . is_loop_header . insert (bcb) ; } if let Some (dom) = this . dominators () . immediate_dominator (bcb) { this . enclosing_loop_header [bcb] = this . is_loop_header . contains (dom) . then_some (dom) . or_else (| | this . enclosing_loop_header [dom]) ; } } assert ! (this [START_BCB] . leader_bb () == mir :: START_BLOCK) ; assert ! (this . predecessors [START_BCB] . is_empty ()) ; this } fn compute_basic_coverage_blocks (mir_body : & mir :: Body < '_ > ,) -> (IndexVec < BasicCoverageBlock , BasicCoverageBlockData > , IndexVec < BasicBlock , Option < BasicCoverageBlock > > ,) { let num_basic_blocks = mir_body . basic_blocks . len () ; let mut bcbs = IndexVec :: < BasicCoverageBlock , _ > :: with_capacity (num_basic_blocks) ; let mut bb_to_bcb = IndexVec :: from_elem_n (None , num_basic_blocks) ; let mut flush_chain_into_new_bcb = | current_chain : & mut Vec < BasicBlock > | { let basic_blocks = mem :: take (current_chain) ; let bcb = bcbs . next_index () ; for & bb in basic_blocks . iter () { bb_to_bcb [bb] = Some (bcb) ; } let is_out_summable = basic_blocks . last () . is_some_and (| & bb | { bcb_filtered_successors (mir_body [bb] . terminator ()) . is_out_summable () }) ; let bcb_data = BasicCoverageBlockData { basic_blocks , is_out_summable } ; debug ! ("adding {bcb:?}: {bcb_data:?}") ; bcbs . push (bcb_data) ; } ; let mut current_chain = vec ! [] ; let subgraph = CoverageRelevantSubgraph :: new (& mir_body . basic_blocks) ; for bb in graph :: depth_first_search (subgraph , mir :: START_BLOCK) . filter (| & bb | mir_body [bb] . terminator () . kind != TerminatorKind :: Unreachable) { if let Some (& prev) = current_chain . last () { let can_chain = subgraph . coverage_successors (prev) . is_out_chainable () && mir_body . basic_blocks . predecessors () [bb] . as_slice () == & [prev] ; if ! can_chain { flush_chain_into_new_bcb (& mut current_chain) ; } } current_chain . push (bb) ; } if ! current_chain . is_empty () { debug ! ("flushing accumulated blocks into one last BCB") ; flush_chain_into_new_bcb (& mut current_chain) ; } (bcbs , bb_to_bcb) } # [inline (always)] pub (crate) fn iter_enumerated (& self ,) -> impl Iterator < Item = (BasicCoverageBlock , & BasicCoverageBlockData) > { self . bcbs . iter_enumerated () } # [inline (always)] pub (crate) fn bcb_from_bb (& self , bb : BasicBlock) -> Option < BasicCoverageBlock > { if bb . index () < self . bb_to_bcb . len () { self . bb_to_bcb [bb] } else { None } } # [inline (always)] fn dominators (& self) -> & Dominators < BasicCoverageBlock > { self . dominators . as_ref () . unwrap () } # [inline (always)] pub (crate) fn dominates (& self , dom : BasicCoverageBlock , node : BasicCoverageBlock) -> bool { self . dominators () . dominates (dom , node) } # [inline (always)] pub (crate) fn cmp_in_dominator_order (& self , a : BasicCoverageBlock , b : BasicCoverageBlock ,) -> Ordering { self . dominator_order_rank [a] . cmp (& self . dominator_order_rank [b]) } # [doc = " For the given node, yields the subset of its predecessor nodes that"] # [doc = " it dominates. If that subset is non-empty, the node is a \"loop header\","] # [doc = " and each of those predecessors represents an in-edge that jumps back to"] # [doc = " the top of its loop."] pub (crate) fn reloop_predecessors (& self , to_bcb : BasicCoverageBlock ,) -> impl Iterator < Item = BasicCoverageBlock > { self . predecessors [to_bcb] . iter () . copied () . filter (move | & pred | self . dominates (to_bcb , pred)) } }}}
mkitem!{mkimpl!{impl Index < BasicCoverageBlock > for CoverageGraph { type Output = BasicCoverageBlockData ; # [inline] fn index (& self , index : BasicCoverageBlock) -> & BasicCoverageBlockData { & self . bcbs [index] } }}}
mkitem!{mkimpl!{impl IndexMut < BasicCoverageBlock > for CoverageGraph { # [inline] fn index_mut (& mut self , index : BasicCoverageBlock) -> & mut BasicCoverageBlockData { & mut self . bcbs [index] } }}}
mkitem!{mkimpl!{impl graph :: DirectedGraph for CoverageGraph { type Node = BasicCoverageBlock ; # [inline] fn num_nodes (& self) -> usize { self . bcbs . len () } }}}
mkitem!{mkimpl!{impl graph :: StartNode for CoverageGraph { # [inline] fn start_node (& self) -> Self :: Node { self . bcb_from_bb (mir :: START_BLOCK) . expect ("mir::START_BLOCK should be in a BasicCoverageBlock") } }}}
mkitem!{mkimpl!{impl graph :: Successors for CoverageGraph { # [inline] fn successors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . successors [node] . iter () . copied () } }}}
mkitem!{mkimpl!{impl graph :: Predecessors for CoverageGraph { # [inline] fn predecessors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . predecessors [node] . iter () . copied () } }}}
mkitem!{mkstruct!{# [doc = " `BasicCoverageBlockData` holds the data indexed by a `BasicCoverageBlock`."] # [doc = ""] # [doc = " A `BasicCoverageBlock` (BCB) represents the maximal-length sequence of MIR `BasicBlock`s without"] # [doc = " conditional branches, and form a new, simplified, coverage-specific Control Flow Graph, without"] # [doc = " altering the original MIR CFG."] # [doc = ""] # [doc = " Note that running the MIR `SimplifyCfg` transform is not sufficient (and therefore not"] # [doc = " necessary). The BCB-based CFG is a more aggressive simplification. For example:"] # [doc = ""] # [doc = "   * The BCB CFG ignores (trims) branches not relevant to coverage, such as unwind-related code,"] # [doc = "     that is injected by the Rust compiler but has no physical source code to count. This also"] # [doc = "     means a BasicBlock with a `Call` terminator can be merged into its primary successor target"] # [doc = "     block, in the same BCB. (But, note: Issue #78544: \"MIR InstrumentCoverage: Improve coverage"] # [doc = "     of `#[should_panic]` tests and `catch_unwind()` handlers\")"] # [doc = "   * Some BasicBlock terminators support Rust-specific concerns--like borrow-checking--that are"] # [doc = "     not relevant to coverage analysis. `FalseUnwind`, for example, can be treated the same as"] # [doc = "     a `Goto`, and merged with its successor into the same BCB."] # [doc = ""] # [doc = " Each BCB with at least one computed coverage span will have no more than one `Counter`."] # [doc = " In some cases, a BCB's execution count can be computed by `Expression`. Additional"] # [doc = " disjoint coverage spans in a BCB can also be counted by `Expression` (by adding `ZERO`"] # [doc = " to the BCB's primary counter or expression)."] # [doc = ""] # [doc = " The BCB CFG is critical to simplifying the coverage analysis by ensuring graph path-based"] # [doc = " queries (`dominates()`, `predecessors`, `successors`, etc.) have branch (control flow)"] # [doc = " significance."] # [derive (Debug , Clone)] pub (crate) struct BasicCoverageBlockData { pub (crate) basic_blocks : Vec < BasicBlock > , # [doc = " If true, this node's execution count can be assumed to be the sum of the"] # [doc = " execution counts of all of its **out-edges** (assuming no panics)."] # [doc = ""] # [doc = " Notably, this is false for a node ending with [`TerminatorKind::Yield`],"] # [doc = " because the yielding coroutine might not be resumed."] pub (crate) is_out_summable : bool , }}}
mkitem!{mkimpl!{impl BasicCoverageBlockData { # [inline (always)] pub (crate) fn leader_bb (& self) -> BasicBlock { self . basic_blocks [0] } # [inline (always)] pub (crate) fn last_bb (& self) -> BasicBlock { * self . basic_blocks . last () . unwrap () } }}}
mkitem!{mkstruct!{# [doc = " Holds the coverage-relevant successors of a basic block's terminator, and"] # [doc = " indicates whether that block can potentially be combined into the same BCB"] # [doc = " as its sole successor."] # [derive (Clone , Copy , Debug)] struct CoverageSuccessors < 'a > { # [doc = " Coverage-relevant successors of the corresponding terminator."] # [doc = " There might be 0, 1, or multiple targets."] targets : & 'a [BasicBlock] , # [doc = " `Yield` terminators are not chainable, because their sole out-edge is"] # [doc = " only followed if/when the generator is resumed after the yield."] is_yield : bool , }}}
mkitem!{mkimpl!{impl CoverageSuccessors < '_ > { # [doc = " If `false`, this terminator cannot be chained into another block when"] # [doc = " building the coverage graph."] fn is_out_chainable (& self) -> bool { self . is_out_summable () && self . targets . len () == 1 } # [doc = " Returns true if the terminator itself is assumed to have the same"] # [doc = " execution count as the sum of its out-edges (assuming no panics)."] fn is_out_summable (& self) -> bool { ! self . is_yield && ! self . targets . is_empty () } }}}
mkitem!{mkimpl!{impl IntoIterator for CoverageSuccessors < '_ > { type Item = BasicBlock ; type IntoIter = impl DoubleEndedIterator < Item = Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . targets . iter () . copied () } }}}

macro_rules! bcb_filtered_successors_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function bcb_filtered_successors in module {}", module_path!());
    };
}

mkfn!{
    bcb_filtered_successors_introspect!();
    fn bcb_filtered_successors < 'a , 'tcx > (terminator : & 'a Terminator < 'tcx >) -> CoverageSuccessors < 'a > { use TerminatorKind :: * ; let mut is_yield = false ; let targets = match & terminator . kind { SwitchInt { targets , .. } => targets . all_targets () , Yield { resume , .. } => { is_yield = true ; slice :: from_ref (resume) } Assert { target , .. } | Drop { target , .. } | FalseEdge { real_target : target , .. } | FalseUnwind { real_target : target , .. } | Goto { target } => slice :: from_ref (target) , Call { target : maybe_target , .. } => maybe_target . as_slice () , InlineAsm { targets , .. } => & targets , CoroutineDrop | Return | TailCall { .. } | Unreachable | UnwindResume | UnwindTerminate (_) => & [] , } ; CoverageSuccessors { targets , is_yield } }
}
mkitem!{mkstruct!{# [doc = " Wrapper around a [`mir::BasicBlocks`] graph that restricts each node's"] # [doc = " successors to only the ones considered \"relevant\" when building a coverage"] # [doc = " graph."] # [derive (Clone , Copy)] struct CoverageRelevantSubgraph < 'a , 'tcx > { basic_blocks : & 'a mir :: BasicBlocks < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > CoverageRelevantSubgraph < 'a , 'tcx > { fn new (basic_blocks : & 'a mir :: BasicBlocks < 'tcx >) -> Self { Self { basic_blocks } } fn coverage_successors (& self , bb : BasicBlock) -> CoverageSuccessors < '_ > { bcb_filtered_successors (self . basic_blocks [bb] . terminator ()) } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > graph :: DirectedGraph for CoverageRelevantSubgraph < 'a , 'tcx > { type Node = BasicBlock ; fn num_nodes (& self) -> usize { self . basic_blocks . num_nodes () } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > graph :: Successors for CoverageRelevantSubgraph < 'a , 'tcx > { fn successors (& self , bb : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . coverage_successors (bb) . into_iter () } }}}