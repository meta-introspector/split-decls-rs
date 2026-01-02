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
mkuse!{use std :: cell :: RefCell ;}
mkuse!{use std :: collections :: hash_map ;}
mkuse!{use std :: rc :: Rc ;}
mkuse!{use itertools :: Itertools as _ ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashMap , FxHashSet , FxIndexMap } ;}
mkuse!{use rustc_data_structures :: unord :: { UnordMap , UnordSet } ;}
mkuse!{use rustc_errors :: Subdiagnostic ;}
mkuse!{use rustc_hir :: CRATE_HIR_ID ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_index :: bit_set :: MixedBitSet ;}
mkuse!{use rustc_index :: { IndexSlice , IndexVec } ;}
mkuse!{use rustc_macros :: { LintDiagnostic , Subdiagnostic } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: mir :: { self , BackwardIncompatibleDropReason , BasicBlock , Body , ClearCrossCrate , Local , Location , MirDumper , Place , StatementKind , TerminatorKind , } ;}
mkuse!{use rustc_middle :: ty :: significant_drop_order :: { extract_component_with_significant_dtor , ty_dtor_span , } ;}
mkuse!{use rustc_middle :: ty :: { self , TyCtxt } ;}
mkuse!{use rustc_mir_dataflow :: impls :: MaybeInitializedPlaces ;}
mkuse!{use rustc_mir_dataflow :: move_paths :: { LookupResult , MoveData , MovePathIndex } ;}
mkuse!{use rustc_mir_dataflow :: { Analysis , MaybeReachable , ResultsCursor } ;}
mkuse!{use rustc_session :: lint :: builtin :: TAIL_EXPR_DROP_ORDER ;}
mkuse!{use rustc_session :: lint :: { self } ;}
mkuse!{use rustc_span :: { DUMMY_SP , Span , Symbol } ;}
mkuse!{use tracing :: debug ;}

macro_rules! place_has_common_prefix_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function place_has_common_prefix in module {}", module_path!());
    };
}

mkfn!{
    place_has_common_prefix_introspect!();
    fn place_has_common_prefix < 'tcx > (left : & Place < 'tcx > , right : & Place < 'tcx >) -> bool { left . local == right . local && left . projection . iter () . zip (right . projection) . all (| (left , right) | left == right) }
}
mkitem!{mkenum!{# [doc = " Cache entry of `drop` at a `BasicBlock`"] # [derive (Debug , Clone , Copy)] enum MovePathIndexAtBlock { # [doc = " We know nothing yet"] Unknown , # [doc = " We know that the `drop` here has no effect"] None , # [doc = " We know that the `drop` here will invoke a destructor"] Some (MovePathIndex) , }}}
mkitem!{mkstruct!{struct DropsReachable < 'a , 'mir , 'tcx > { body : & 'a Body < 'tcx > , place : & 'a Place < 'tcx > , drop_span : & 'a mut Option < Span > , move_data : & 'a MoveData < 'tcx > , maybe_init : & 'a mut ResultsCursor < 'mir , 'tcx , MaybeInitializedPlaces < 'mir , 'tcx > > , block_drop_value_info : & 'a mut IndexSlice < BasicBlock , MovePathIndexAtBlock > , collected_drops : & 'a mut MixedBitSet < MovePathIndex > , visited : FxHashMap < BasicBlock , Rc < RefCell < MixedBitSet < MovePathIndex > > > > , }}}
mkitem!{mkimpl!{impl < 'a , 'mir , 'tcx > DropsReachable < 'a , 'mir , 'tcx > { fn visit (& mut self , block : BasicBlock) { let move_set_size = self . move_data . move_paths . len () ; let make_new_path_set = | | Rc :: new (RefCell :: new (MixedBitSet :: new_empty (move_set_size))) ; let data = & self . body . basic_blocks [block] ; let Some (terminator) = & data . terminator else { return } ; let dropped_local_here = Rc :: clone (self . visited . entry (block) . or_insert_with (make_new_path_set)) ; match self . block_drop_value_info [block] { MovePathIndexAtBlock :: Some (dropped) => { dropped_local_here . borrow_mut () . insert (dropped) ; } MovePathIndexAtBlock :: Unknown => { if let TerminatorKind :: Drop { place , .. } = & terminator . kind && let LookupResult :: Exact (idx) | LookupResult :: Parent (Some (idx)) = self . move_data . rev_lookup . find (place . as_ref ()) { self . maybe_init . seek_before_primary_effect (Location { block , statement_index : data . statements . len () , }) ; if let MaybeReachable :: Reachable (maybe_init) = self . maybe_init . get () && maybe_init . contains (idx) { self . block_drop_value_info [block] = MovePathIndexAtBlock :: Some (idx) ; dropped_local_here . borrow_mut () . insert (idx) ; } else { self . block_drop_value_info [block] = MovePathIndexAtBlock :: None ; } } } MovePathIndexAtBlock :: None => { } } for succ in terminator . successors () { let target = & self . body . basic_blocks [succ] ; if target . is_cleanup { continue ; } let dropped_local_there = match self . visited . entry (succ) { hash_map :: Entry :: Occupied (occupied_entry) => { if succ == block || ! occupied_entry . get () . borrow_mut () . union (& * dropped_local_here . borrow ()) { continue ; } Rc :: clone (occupied_entry . get ()) } hash_map :: Entry :: Vacant (vacant_entry) => Rc :: clone (vacant_entry . insert (Rc :: new (RefCell :: new (dropped_local_here . borrow () . clone ()))) ,) , } ; if let Some (terminator) = & target . terminator && let TerminatorKind :: Drop { place : dropped_place , target : _ , unwind : _ , replace : _ , drop : _ , async_fut : _ , } = & terminator . kind && place_has_common_prefix (dropped_place , self . place) { self . collected_drops . union (& * dropped_local_there . borrow ()) ; if self . drop_span . is_none () { * self . drop_span = Some (terminator . source_info . span) ; } } else { self . visit (succ) } } } }}}

macro_rules! place_descendent_of_bids_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function place_descendent_of_bids in module {}", module_path!());
    };
}

mkfn!{
    place_descendent_of_bids_introspect!();
    # [doc = " Check if a moved place at `idx` is a part of a BID."] # [doc = " The use of this check is that we will consider drops on these"] # [doc = " as a drop of the overall BID and, thus, we can exclude it from the diagnosis."] fn place_descendent_of_bids < 'tcx > (mut idx : MovePathIndex , move_data : & MoveData < 'tcx > , bids : & UnordSet < & Place < 'tcx > > ,) -> bool { loop { let path = & move_data . move_paths [idx] ; if bids . contains (& path . place) { return true ; } if let Some (parent) = path . parent { idx = parent ; } else { return false ; } } }
}

macro_rules! run_lint_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_lint in module {}", module_path!());
    };
}

mkfn!{
    run_lint_introspect!();
    # [doc = " The core of the lint `tail-expr-drop-order`"] pub (crate) fn run_lint < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId , body : & Body < 'tcx >) { if matches ! (tcx . def_kind (def_id) , rustc_hir :: def :: DefKind :: SyntheticCoroutineBody) { return ; } if body . span . edition () . at_least_rust_2024 () || tcx . lints_that_dont_need_to_run (()) . contains (& lint :: LintId :: of (TAIL_EXPR_DROP_ORDER)) { return ; } let typing_env = ty :: TypingEnv :: non_body_analysis (tcx , def_id) ; let mut bid_per_block = FxIndexMap :: default () ; let mut bid_places = UnordSet :: new () ; let mut ty_dropped_components = UnordMap :: default () ; for (block , data) in body . basic_blocks . iter_enumerated () { for (statement_index , stmt) in data . statements . iter () . enumerate () { if let StatementKind :: BackwardIncompatibleDropHint { place , reason : BackwardIncompatibleDropReason :: Edition2024 , } = & stmt . kind { let ty = place . ty (body , tcx) . ty ; if ty_dropped_components . entry (ty) . or_insert_with (| | extract_component_with_significant_dtor (tcx , typing_env , ty)) . is_empty () { continue ; } bid_per_block . entry (block) . or_insert (vec ! []) . push ((Location { block , statement_index } , & * * place)) ; bid_places . insert (& * * place) ; } } } if bid_per_block . is_empty () { return ; } if let Some (dumper) = MirDumper :: new (tcx , "lint_tail_expr_drop_order" , body) { dumper . dump_mir (body) ; } let locals_with_user_names = collect_user_names (body) ; let is_closure_like = tcx . is_closure_like (def_id . to_def_id ()) ; let move_data = MoveData :: gather_moves (body , tcx , | _ | true) ; let mut maybe_init = MaybeInitializedPlaces :: new (tcx , body , & move_data) . iterate_to_fixpoint (tcx , body , None) . into_results_cursor (body) ; let mut block_drop_value_info = IndexVec :: from_elem_n (MovePathIndexAtBlock :: Unknown , body . basic_blocks . len ()) ; for (& block , candidates) in & bid_per_block { let mut all_locals_dropped = MixedBitSet :: new_empty (move_data . move_paths . len ()) ; let mut drop_span = None ; for & (_ , place) in candidates . iter () { let mut collected_drops = MixedBitSet :: new_empty (move_data . move_paths . len ()) ; DropsReachable { body , place , drop_span : & mut drop_span , move_data : & move_data , maybe_init : & mut maybe_init , block_drop_value_info : & mut block_drop_value_info , collected_drops : & mut collected_drops , visited : Default :: default () , } . visit (block) ; all_locals_dropped . union (& collected_drops) ; } { let mut to_exclude = MixedBitSet :: new_empty (all_locals_dropped . domain_size ()) ; for path_idx in all_locals_dropped . iter () { let move_path = & move_data . move_paths [path_idx] ; let dropped_local = move_path . place . local ; if dropped_local == Local :: ZERO { debug ! (? dropped_local , "skip return value") ; to_exclude . insert (path_idx) ; continue ; } if is_closure_like && matches ! (dropped_local , ty :: CAPTURE_STRUCT_LOCAL) { debug ! (? dropped_local , "skip closure captures") ; to_exclude . insert (path_idx) ; continue ; } if place_descendent_of_bids (path_idx , & move_data , & bid_places) { debug ! (? dropped_local , "skip descendent of bids") ; to_exclude . insert (path_idx) ; continue ; } let observer_ty = move_path . place . ty (body , tcx) . ty ; if ty_dropped_components . entry (observer_ty) . or_insert_with (| | { extract_component_with_significant_dtor (tcx , typing_env , observer_ty) }) . is_empty () { debug ! (? dropped_local , "skip non-droppy types") ; to_exclude . insert (path_idx) ; continue ; } } if let Ok (local) = candidates . iter () . map (| & (_ , place) | place . local) . all_equal_value () { for path_idx in all_locals_dropped . iter () { if move_data . move_paths [path_idx] . place . local == local { to_exclude . insert (path_idx) ; } } } all_locals_dropped . subtract (& to_exclude) ; } if all_locals_dropped . is_empty () { continue ; } let local_names = assign_observables_names (all_locals_dropped . iter () . map (| path_idx | move_data . move_paths [path_idx] . place . local) . chain (candidates . iter () . map (| (_ , place) | place . local)) , & locals_with_user_names ,) ; let mut lint_root = None ; let mut local_labels = vec ! [] ; for & (_ , place) in candidates { let linted_local_decl = & body . local_decls [place . local] ; let Some (& (ref name , is_generated_name)) = local_names . get (& place . local) else { bug ! ("a name should have been assigned") } ; let name = name . as_str () ; if lint_root . is_none () && let ClearCrossCrate :: Set (data) = & body . source_scopes [linted_local_decl . source_info . scope] . local_data { lint_root = Some (data . lint_root) ; } let mut seen_dyn = false ; let destructors = ty_dropped_components . get (& linted_local_decl . ty) . unwrap () . iter () . filter_map (| & ty | { if let Some (span) = ty_dtor_span (tcx , ty) { Some (DestructorLabel { span , name , dtor_kind : "concrete" }) } else if matches ! (ty . kind () , ty :: Dynamic (..)) { if seen_dyn { None } else { seen_dyn = true ; Some (DestructorLabel { span : DUMMY_SP , name , dtor_kind : "dyn" }) } } else { None } }) . collect () ; local_labels . push (LocalLabel { span : linted_local_decl . source_info . span , destructors , name , is_generated_name , is_dropped_first_edition_2024 : true , }) ; } for path_idx in all_locals_dropped . iter () { let place = & move_data . move_paths [path_idx] . place ; let observer_ty = place . ty (body , tcx) . ty ; let observer_local_decl = & body . local_decls [place . local] ; let Some (& (ref name , is_generated_name)) = local_names . get (& place . local) else { bug ! ("a name should have been assigned") } ; let name = name . as_str () ; let mut seen_dyn = false ; let destructors = extract_component_with_significant_dtor (tcx , typing_env , observer_ty) . into_iter () . filter_map (| ty | { if let Some (span) = ty_dtor_span (tcx , ty) { Some (DestructorLabel { span , name , dtor_kind : "concrete" }) } else if matches ! (ty . kind () , ty :: Dynamic (..)) { if seen_dyn { None } else { seen_dyn = true ; Some (DestructorLabel { span : DUMMY_SP , name , dtor_kind : "dyn" }) } } else { None } }) . collect () ; local_labels . push (LocalLabel { span : observer_local_decl . source_info . span , destructors , name , is_generated_name , is_dropped_first_edition_2024 : false , }) ; } let span = local_labels [0] . span ; tcx . emit_node_span_lint (lint :: builtin :: TAIL_EXPR_DROP_ORDER , lint_root . unwrap_or (CRATE_HIR_ID) , span , TailExprDropOrderLint { local_labels , drop_span , _epilogue : () } ,) ; } }
}

macro_rules! collect_user_names_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_user_names in module {}", module_path!());
    };
}

mkfn!{
    collect_user_names_introspect!();
    # [doc = " Extract binding names if available for diagnosis"] fn collect_user_names (body : & Body < '_ >) -> FxIndexMap < Local , Symbol > { let mut names = FxIndexMap :: default () ; for var_debug_info in & body . var_debug_info { if let mir :: VarDebugInfoContents :: Place (place) = & var_debug_info . value && let Some (local) = place . local_or_deref_local () { names . entry (local) . or_insert (var_debug_info . name) ; } } names }
}

macro_rules! assign_observables_names_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function assign_observables_names in module {}", module_path!());
    };
}

mkfn!{
    assign_observables_names_introspect!();
    # [doc = " Assign names for anonymous or temporary values for diagnosis"] fn assign_observables_names (locals : impl IntoIterator < Item = Local > , user_names : & FxIndexMap < Local , Symbol > ,) -> FxIndexMap < Local , (String , bool) > { let mut names = FxIndexMap :: default () ; let mut assigned_names = FxHashSet :: default () ; let mut idx = 0u64 ; let mut fresh_name = | | { idx += 1 ; (format ! ("#{idx}") , true) } ; for local in locals { let name = if let Some (name) = user_names . get (& local) { let name = name . as_str () ; if assigned_names . contains (name) { fresh_name () } else { (name . to_owned () , false) } } else { fresh_name () } ; assigned_names . insert (name . 0 . clone ()) ; names . insert (local , name) ; } names }
}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (mir_transform_tail_expr_drop_order)] struct TailExprDropOrderLint < 'a > { # [subdiagnostic] local_labels : Vec < LocalLabel < 'a > > , # [label (mir_transform_drop_location)] drop_span : Option < Span > , # [note (mir_transform_note_epilogue)] _epilogue : () , }}}
mkitem!{mkstruct!{struct LocalLabel < 'a > { span : Span , name : & 'a str , is_generated_name : bool , is_dropped_first_edition_2024 : bool , destructors : Vec < DestructorLabel < 'a > > , }}}
mkitem!{mkimpl!{# [doc = " A custom `Subdiagnostic` implementation so that the notes are delivered in a specific order"] impl Subdiagnostic for LocalLabel < '_ > { fn add_to_diag < G : rustc_errors :: EmissionGuarantee > (self , diag : & mut rustc_errors :: Diag < '_ , G >) { diag . remove_arg ("name") ; diag . arg ("name" , self . name) ; diag . remove_arg ("is_generated_name") ; diag . arg ("is_generated_name" , self . is_generated_name) ; diag . remove_arg ("is_dropped_first_edition_2024") ; diag . arg ("is_dropped_first_edition_2024" , self . is_dropped_first_edition_2024) ; let msg = diag . eagerly_translate (crate :: fluent_generated :: mir_transform_tail_expr_local) ; diag . span_label (self . span , msg) ; for dtor in self . destructors { dtor . add_to_diag (diag) ; } let msg = diag . eagerly_translate (crate :: fluent_generated :: mir_transform_label_local_epilogue) ; diag . span_label (self . span , msg) ; } }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (mir_transform_tail_expr_dtor)] struct DestructorLabel < 'a > { # [primary_span] span : Span , dtor_kind : & 'static str , name : & 'a str , }}}