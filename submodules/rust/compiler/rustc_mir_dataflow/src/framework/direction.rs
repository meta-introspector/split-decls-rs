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
mkuse!{use std :: ops :: RangeInclusive ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: mir :: { self , BasicBlock , CallReturnPlaces , Location , SwitchTargetValue , TerminatorEdges , } ;}
mkuse!{use super :: visitor :: ResultsVisitor ;}
mkuse!{use super :: { Analysis , Effect , EffectIndex } ;}
mkitem!{mktrait!{pub trait Direction { const IS_FORWARD : bool ; const IS_BACKWARD : bool = ! Self :: IS_FORWARD ; # [doc = " Called by `iterate_to_fixpoint` during initial analysis computation."] fn apply_effects_in_block < 'mir , 'tcx , A > (analysis : & mut A , body : & mir :: Body < 'tcx > , state : & mut A :: Domain , block : BasicBlock , block_data : & 'mir mir :: BasicBlockData < 'tcx > , propagate : impl FnMut (BasicBlock , & A :: Domain) ,) where A : Analysis < 'tcx > ; # [doc = " Called by `ResultsCursor` to recompute the domain value for a location"] # [doc = " in a basic block. Applies all effects between the given `EffectIndex`s."] # [doc = ""] # [doc = " `effects.start()` must precede or equal `effects.end()` in this direction."] fn apply_effects_in_range < 'tcx , A > (analysis : & mut A , state : & mut A :: Domain , block : BasicBlock , block_data : & mir :: BasicBlockData < 'tcx > , effects : RangeInclusive < EffectIndex > ,) where A : Analysis < 'tcx > ; # [doc = " Called by `ResultsVisitor` to recompute the analysis domain values for"] # [doc = " all locations in a basic block (starting from `entry_state` and to"] # [doc = " visit them with `vis`."] fn visit_results_in_block < 'mir , 'tcx , A > (state : & mut A :: Domain , block : BasicBlock , block_data : & 'mir mir :: BasicBlockData < 'tcx > , analysis : & mut A , vis : & mut impl ResultsVisitor < 'tcx , A > ,) where A : Analysis < 'tcx > ; }}}
mkitem!{mkstruct!{# [doc = " Dataflow that runs from the exit of a block (terminator), to its entry (the first statement)."] pub struct Backward ;}}
mkitem!{mkimpl!{impl Direction for Backward { const IS_FORWARD : bool = false ; fn apply_effects_in_block < 'mir , 'tcx , A > (analysis : & mut A , body : & mir :: Body < 'tcx > , state : & mut A :: Domain , block : BasicBlock , block_data : & 'mir mir :: BasicBlockData < 'tcx > , mut propagate : impl FnMut (BasicBlock , & A :: Domain) ,) where A : Analysis < 'tcx > , { let terminator = block_data . terminator () ; let location = Location { block , statement_index : block_data . statements . len () } ; analysis . apply_early_terminator_effect (state , terminator , location) ; analysis . apply_primary_terminator_effect (state , terminator , location) ; for (statement_index , statement) in block_data . statements . iter () . enumerate () . rev () { let location = Location { block , statement_index } ; analysis . apply_early_statement_effect (state , statement , location) ; analysis . apply_primary_statement_effect (state , statement , location) ; } let exit_state = state ; for pred in body . basic_blocks . predecessors () [block] . iter () . copied () { match body [pred] . terminator () . kind { mir :: TerminatorKind :: Call { destination , target : Some (dest) , .. } if dest == block => { let mut tmp = exit_state . clone () ; analysis . apply_call_return_effect (& mut tmp , pred , CallReturnPlaces :: Call (destination) ,) ; propagate (pred , & tmp) ; } mir :: TerminatorKind :: InlineAsm { ref targets , ref operands , .. } if targets . contains (& block) => { let mut tmp = exit_state . clone () ; analysis . apply_call_return_effect (& mut tmp , pred , CallReturnPlaces :: InlineAsm (operands) ,) ; propagate (pred , & tmp) ; } mir :: TerminatorKind :: Yield { resume , resume_arg , .. } if resume == block => { let mut tmp = exit_state . clone () ; analysis . apply_call_return_effect (& mut tmp , resume , CallReturnPlaces :: Yield (resume_arg) ,) ; propagate (pred , & tmp) ; } mir :: TerminatorKind :: SwitchInt { ref discr , .. } => { if let Some (_data) = analysis . get_switch_int_data (pred , discr) { bug ! ("SwitchInt edge effects are unsupported in backward dataflow analyses") ; } else { propagate (pred , exit_state) } } _ => propagate (pred , exit_state) , } } } fn apply_effects_in_range < 'tcx , A > (analysis : & mut A , state : & mut A :: Domain , block : BasicBlock , block_data : & mir :: BasicBlockData < 'tcx > , effects : RangeInclusive < EffectIndex > ,) where A : Analysis < 'tcx > , { let (from , to) = (* effects . start () , * effects . end ()) ; let terminator_index = block_data . statements . len () ; assert ! (from . statement_index <= terminator_index) ; assert ! (! to . precedes_in_backward_order (from)) ; let next_effect = match from . effect { _ if from . statement_index == terminator_index => { let location = Location { block , statement_index : from . statement_index } ; let terminator = block_data . terminator () ; if from . effect == Effect :: Early { analysis . apply_early_terminator_effect (state , terminator , location) ; if to == Effect :: Early . at_index (terminator_index) { return ; } } analysis . apply_primary_terminator_effect (state , terminator , location) ; if to == Effect :: Primary . at_index (terminator_index) { return ; } from . statement_index - 1 } Effect :: Primary => { let location = Location { block , statement_index : from . statement_index } ; let statement = & block_data . statements [from . statement_index] ; analysis . apply_primary_statement_effect (state , statement , location) ; if to == Effect :: Primary . at_index (from . statement_index) { return ; } from . statement_index - 1 } Effect :: Early => from . statement_index , } ; for statement_index in (to . statement_index .. next_effect) . rev () . map (| i | i + 1) { let location = Location { block , statement_index } ; let statement = & block_data . statements [statement_index] ; analysis . apply_early_statement_effect (state , statement , location) ; analysis . apply_primary_statement_effect (state , statement , location) ; } let location = Location { block , statement_index : to . statement_index } ; let statement = & block_data . statements [to . statement_index] ; analysis . apply_early_statement_effect (state , statement , location) ; if to . effect == Effect :: Early { return ; } analysis . apply_primary_statement_effect (state , statement , location) ; } fn visit_results_in_block < 'mir , 'tcx , A > (state : & mut A :: Domain , block : BasicBlock , block_data : & 'mir mir :: BasicBlockData < 'tcx > , analysis : & mut A , vis : & mut impl ResultsVisitor < 'tcx , A > ,) where A : Analysis < 'tcx > , { vis . visit_block_end (state) ; let loc = Location { block , statement_index : block_data . statements . len () } ; let term = block_data . terminator () ; analysis . apply_early_terminator_effect (state , term , loc) ; vis . visit_after_early_terminator_effect (analysis , state , term , loc) ; analysis . apply_primary_terminator_effect (state , term , loc) ; vis . visit_after_primary_terminator_effect (analysis , state , term , loc) ; for (statement_index , stmt) in block_data . statements . iter () . enumerate () . rev () { let loc = Location { block , statement_index } ; analysis . apply_early_statement_effect (state , stmt , loc) ; vis . visit_after_early_statement_effect (analysis , state , stmt , loc) ; analysis . apply_primary_statement_effect (state , stmt , loc) ; vis . visit_after_primary_statement_effect (analysis , state , stmt , loc) ; } vis . visit_block_start (state) ; } }}}
mkitem!{mkstruct!{# [doc = " Dataflow that runs from the entry of a block (the first statement), to its exit (terminator)."] pub struct Forward ;}}
mkitem!{mkimpl!{impl Direction for Forward { const IS_FORWARD : bool = true ; fn apply_effects_in_block < 'mir , 'tcx , A > (analysis : & mut A , body : & mir :: Body < 'tcx > , state : & mut A :: Domain , block : BasicBlock , block_data : & 'mir mir :: BasicBlockData < 'tcx > , mut propagate : impl FnMut (BasicBlock , & A :: Domain) ,) where A : Analysis < 'tcx > , { for (statement_index , statement) in block_data . statements . iter () . enumerate () { let location = Location { block , statement_index } ; analysis . apply_early_statement_effect (state , statement , location) ; analysis . apply_primary_statement_effect (state , statement , location) ; } let terminator = block_data . terminator () ; let location = Location { block , statement_index : block_data . statements . len () } ; analysis . apply_early_terminator_effect (state , terminator , location) ; let edges = analysis . apply_primary_terminator_effect (state , terminator , location) ; let exit_state = state ; match edges { TerminatorEdges :: None => { } TerminatorEdges :: Single (target) => propagate (target , exit_state) , TerminatorEdges :: Double (target , unwind) => { propagate (target , exit_state) ; propagate (unwind , exit_state) ; } TerminatorEdges :: AssignOnReturn { return_ , cleanup , place } => { if let Some (cleanup) = cleanup { propagate (cleanup , exit_state) ; } if ! return_ . is_empty () { analysis . apply_call_return_effect (exit_state , block , place) ; for & target in return_ { propagate (target , exit_state) ; } } } TerminatorEdges :: SwitchInt { targets , discr } => { if let Some (mut data) = analysis . get_switch_int_data (block , discr) { let mut tmp = analysis . bottom_value (body) ; for (value , target) in targets . iter () { tmp . clone_from (exit_state) ; let value = SwitchTargetValue :: Normal (value) ; analysis . apply_switch_int_edge_effect (& mut data , & mut tmp , value , targets) ; propagate (target , & tmp) ; } analysis . apply_switch_int_edge_effect (& mut data , exit_state , SwitchTargetValue :: Otherwise , targets ,) ; propagate (targets . otherwise () , exit_state) ; } else { for target in targets . all_targets () { propagate (* target , exit_state) ; } } } } } fn apply_effects_in_range < 'tcx , A > (analysis : & mut A , state : & mut A :: Domain , block : BasicBlock , block_data : & mir :: BasicBlockData < 'tcx > , effects : RangeInclusive < EffectIndex > ,) where A : Analysis < 'tcx > , { let (from , to) = (* effects . start () , * effects . end ()) ; let terminator_index = block_data . statements . len () ; assert ! (to . statement_index <= terminator_index) ; assert ! (! to . precedes_in_forward_order (from)) ; let first_unapplied_index = match from . effect { Effect :: Early => from . statement_index , Effect :: Primary if from . statement_index == terminator_index => { debug_assert_eq ! (from , to) ; let location = Location { block , statement_index : terminator_index } ; let terminator = block_data . terminator () ; analysis . apply_primary_terminator_effect (state , terminator , location) ; return ; } Effect :: Primary => { let location = Location { block , statement_index : from . statement_index } ; let statement = & block_data . statements [from . statement_index] ; analysis . apply_primary_statement_effect (state , statement , location) ; if from == to { return ; } from . statement_index + 1 } } ; for statement_index in first_unapplied_index .. to . statement_index { let location = Location { block , statement_index } ; let statement = & block_data . statements [statement_index] ; analysis . apply_early_statement_effect (state , statement , location) ; analysis . apply_primary_statement_effect (state , statement , location) ; } let location = Location { block , statement_index : to . statement_index } ; if to . statement_index == terminator_index { let terminator = block_data . terminator () ; analysis . apply_early_terminator_effect (state , terminator , location) ; if to . effect == Effect :: Primary { analysis . apply_primary_terminator_effect (state , terminator , location) ; } } else { let statement = & block_data . statements [to . statement_index] ; analysis . apply_early_statement_effect (state , statement , location) ; if to . effect == Effect :: Primary { analysis . apply_primary_statement_effect (state , statement , location) ; } } } fn visit_results_in_block < 'mir , 'tcx , A > (state : & mut A :: Domain , block : BasicBlock , block_data : & 'mir mir :: BasicBlockData < 'tcx > , analysis : & mut A , vis : & mut impl ResultsVisitor < 'tcx , A > ,) where A : Analysis < 'tcx > , { vis . visit_block_start (state) ; for (statement_index , stmt) in block_data . statements . iter () . enumerate () { let loc = Location { block , statement_index } ; analysis . apply_early_statement_effect (state , stmt , loc) ; vis . visit_after_early_statement_effect (analysis , state , stmt , loc) ; analysis . apply_primary_statement_effect (state , stmt , loc) ; vis . visit_after_primary_statement_effect (analysis , state , stmt , loc) ; } let loc = Location { block , statement_index : block_data . statements . len () } ; let term = block_data . terminator () ; analysis . apply_early_terminator_effect (state , term , loc) ; vis . visit_after_early_terminator_effect (analysis , state , term , loc) ; analysis . apply_primary_terminator_effect (state , term , loc) ; vis . visit_after_primary_terminator_effect (analysis , state , term , loc) ; vis . visit_block_end (state) ; } }}}