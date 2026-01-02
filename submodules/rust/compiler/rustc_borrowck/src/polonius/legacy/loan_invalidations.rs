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
mkuse!{use std :: ops :: ControlFlow ;}
mkuse!{use rustc_data_structures :: graph :: dominators :: Dominators ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: mir :: visit :: Visitor ;}
mkuse!{use rustc_middle :: mir :: * ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use tracing :: debug ;}
mkuse!{use super :: { PoloniusFacts , PoloniusLocationTable } ;}
mkuse!{use crate :: borrow_set :: BorrowSet ;}
mkuse!{use crate :: path_utils :: * ;}
mkuse!{use crate :: { AccessDepth , Activation , ArtificialField , BorrowIndex , Deep , LocalMutationIsAllowed , Read , ReadKind , ReadOrWrite , Reservation , Shallow , Write , WriteKind , } ;}

macro_rules! emit_loan_invalidations_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_loan_invalidations in module {}", module_path!());
    };
}

mkfn!{
    emit_loan_invalidations_introspect!();
    #[doc = " Emit `loan_invalidated_at` facts."] pub (super) fn emit_loan_invalidations < 'tcx > (tcx : TyCtxt < 'tcx > , facts : & mut PoloniusFacts , body : & Body < 'tcx > , location_table : & PoloniusLocationTable , borrow_set : & BorrowSet < 'tcx > ,) { let dominators = body . basic_blocks . dominators () ; let mut visitor = LoanInvalidationsGenerator { facts , borrow_set , tcx , location_table , body , dominators } ; visitor . visit_body (body) ; }
}
mkitem!{mkstruct!{struct LoanInvalidationsGenerator < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , facts : & 'a mut PoloniusFacts , body : & 'a Body < 'tcx > , location_table : & 'a PoloniusLocationTable , dominators : & 'a Dominators < BasicBlock > , borrow_set : & 'a BorrowSet < 'tcx > , }}}
mkitem!{mkimpl!{#[doc = " Visits the whole MIR and generates `invalidates()` facts."] #[doc = " Most of the code implementing this was stolen from `borrow_check/mod.rs`."] impl < 'a , 'tcx > Visitor < 'tcx > for LoanInvalidationsGenerator < 'a , 'tcx > { fn visit_statement (& mut self , statement : & Statement < 'tcx > , location : Location) { self . check_activations (location) ; match & statement . kind { StatementKind :: Assign (box (lhs , rhs)) => { self . consume_rvalue (location , rhs) ; self . mutate_place (location , * lhs , Shallow (None)) ; } StatementKind :: FakeRead (box (_ , _)) => { } StatementKind :: Intrinsic (box NonDivergingIntrinsic :: Assume (op)) => { self . consume_operand (location , op) ; } StatementKind :: Intrinsic (box NonDivergingIntrinsic :: CopyNonOverlapping (CopyNonOverlapping { src , dst , count , })) => { self . consume_operand (location , src) ; self . consume_operand (location , dst) ; self . consume_operand (location , count) ; } StatementKind :: AscribeUserType (..) | StatementKind :: PlaceMention (..) | StatementKind :: Coverage (..) | StatementKind :: StorageLive (..) => { } StatementKind :: StorageDead (local) => { self . access_place (location , Place :: from (* local) , (Shallow (None) , Write (WriteKind :: StorageDeadOrDrop)) , LocalMutationIsAllowed :: Yes ,) ; } StatementKind :: ConstEvalCounter | StatementKind :: Nop | StatementKind :: Retag { .. } | StatementKind :: Deinit (..) | StatementKind :: BackwardIncompatibleDropHint { .. } | StatementKind :: SetDiscriminant { .. } => { bug ! ("Statement not allowed in this MIR phase") } } self . super_statement (statement , location) ; } fn visit_terminator (& mut self , terminator : & Terminator < 'tcx > , location : Location) { self . check_activations (location) ; match & terminator . kind { TerminatorKind :: SwitchInt { discr , targets : _ } => { self . consume_operand (location , discr) ; } TerminatorKind :: Drop { place : drop_place , target : _ , unwind : _ , replace , drop : _ , async_fut : _ , } => { let write_kind = if * replace { WriteKind :: Replace } else { WriteKind :: StorageDeadOrDrop } ; self . access_place (location , * drop_place , (AccessDepth :: Drop , Write (write_kind)) , LocalMutationIsAllowed :: Yes ,) ; } TerminatorKind :: Call { func , args , destination , target : _ , unwind : _ , call_source : _ , fn_span : _ , } => { self . consume_operand (location , func) ; for arg in args { self . consume_operand (location , & arg . node) ; } self . mutate_place (location , * destination , Deep) ; } TerminatorKind :: TailCall { func , args , .. } => { self . consume_operand (location , func) ; for arg in args { self . consume_operand (location , & arg . node) ; } } TerminatorKind :: Assert { cond , expected : _ , msg , target : _ , unwind : _ } => { self . consume_operand (location , cond) ; use rustc_middle :: mir :: AssertKind ; if let AssertKind :: BoundsCheck { len , index } = & * * msg { self . consume_operand (location , len) ; self . consume_operand (location , index) ; } } TerminatorKind :: Yield { value , resume , resume_arg , drop : _ } => { self . consume_operand (location , value) ; let borrow_set = self . borrow_set ; let resume = self . location_table . start_index (resume . start_location ()) ; for (i , data) in borrow_set . iter_enumerated () { if borrow_of_local_data (data . borrowed_place) { self . facts . loan_invalidated_at . push ((resume , i)) ; } } self . mutate_place (location , * resume_arg , Deep) ; } TerminatorKind :: UnwindResume | TerminatorKind :: Return | TerminatorKind :: CoroutineDrop => { let borrow_set = self . borrow_set ; let start = self . location_table . start_index (location) ; for (i , data) in borrow_set . iter_enumerated () { if borrow_of_local_data (data . borrowed_place) { self . facts . loan_invalidated_at . push ((start , i)) ; } } } TerminatorKind :: InlineAsm { asm_macro : _ , template : _ , operands , options : _ , line_spans : _ , targets : _ , unwind : _ , } => { for op in operands { match op { InlineAsmOperand :: In { reg : _ , value } => { self . consume_operand (location , value) ; } InlineAsmOperand :: Out { reg : _ , late : _ , place , .. } => { if let & Some (place) = place { self . mutate_place (location , place , Shallow (None)) ; } } InlineAsmOperand :: InOut { reg : _ , late : _ , in_value , out_place } => { self . consume_operand (location , in_value) ; if let & Some (out_place) = out_place { self . mutate_place (location , out_place , Shallow (None)) ; } } InlineAsmOperand :: Const { value : _ } | InlineAsmOperand :: SymFn { value : _ } | InlineAsmOperand :: SymStatic { def_id : _ } | InlineAsmOperand :: Label { target_index : _ } => { } } } } TerminatorKind :: Goto { target : _ } | TerminatorKind :: UnwindTerminate (_) | TerminatorKind :: Unreachable | TerminatorKind :: FalseEdge { real_target : _ , imaginary_target : _ } | TerminatorKind :: FalseUnwind { real_target : _ , unwind : _ } => { } } self . super_terminator (terminator , location) ; } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > LoanInvalidationsGenerator < 'a , 'tcx > { #[doc = " Simulates mutation of a place."] fn mutate_place (& mut self , location : Location , place : Place < 'tcx > , kind : AccessDepth) { self . access_place (location , place , (kind , Write (WriteKind :: Mutate)) , LocalMutationIsAllowed :: ExceptUpvars ,) ; } #[doc = " Simulates consumption of an operand."] fn consume_operand (& mut self , location : Location , operand : & Operand < 'tcx >) { match * operand { Operand :: Copy (place) => { self . access_place (location , place , (Deep , Read (ReadKind :: Copy)) , LocalMutationIsAllowed :: No ,) ; } Operand :: Move (place) => { self . access_place (location , place , (Deep , Write (WriteKind :: Move)) , LocalMutationIsAllowed :: Yes ,) ; } Operand :: Constant (_) => { } } } fn consume_rvalue (& mut self , location : Location , rvalue : & Rvalue < 'tcx >) { match rvalue { & Rvalue :: Ref (_ , bk , place) => { let access_kind = match bk { BorrowKind :: Fake (FakeBorrowKind :: Shallow) => { (Shallow (Some (ArtificialField :: FakeBorrow)) , Read (ReadKind :: Borrow (bk))) } BorrowKind :: Shared | BorrowKind :: Fake (FakeBorrowKind :: Deep) => { (Deep , Read (ReadKind :: Borrow (bk))) } BorrowKind :: Mut { .. } => { let wk = WriteKind :: MutableBorrow (bk) ; if bk . allows_two_phase_borrow () { (Deep , Reservation (wk)) } else { (Deep , Write (wk)) } } } ; self . access_place (location , place , access_kind , LocalMutationIsAllowed :: No) ; } & Rvalue :: RawPtr (kind , place) => { let access_kind = match kind { RawPtrKind :: Mut => (Deep , Write (WriteKind :: MutableBorrow (BorrowKind :: Mut { kind : MutBorrowKind :: Default , })) ,) , RawPtrKind :: Const => (Deep , Read (ReadKind :: Borrow (BorrowKind :: Shared))) , RawPtrKind :: FakeForPtrMetadata => { (Shallow (Some (ArtificialField :: ArrayLength)) , Read (ReadKind :: Copy)) } } ; self . access_place (location , place , access_kind , LocalMutationIsAllowed :: No) ; } Rvalue :: ThreadLocalRef (_) => { } Rvalue :: Use (operand) | Rvalue :: Repeat (operand , _) | Rvalue :: UnaryOp (_ , operand) | Rvalue :: Cast (_ , operand , _) | Rvalue :: ShallowInitBox (operand , _) => self . consume_operand (location , operand) , & Rvalue :: CopyForDeref (place) => { let op = & Operand :: Copy (place) ; self . consume_operand (location , op) ; } & (Rvalue :: Len (place) | Rvalue :: Discriminant (place)) => { let af = match rvalue { Rvalue :: Len (..) => Some (ArtificialField :: ArrayLength) , Rvalue :: Discriminant (..) => None , _ => unreachable ! () , } ; self . access_place (location , place , (Shallow (af) , Read (ReadKind :: Copy)) , LocalMutationIsAllowed :: No ,) ; } Rvalue :: BinaryOp (_bin_op , box (operand1 , operand2)) => { self . consume_operand (location , operand1) ; self . consume_operand (location , operand2) ; } Rvalue :: NullaryOp (_op , _ty) => { } Rvalue :: Aggregate (_ , operands) => { for operand in operands { self . consume_operand (location , operand) ; } } Rvalue :: WrapUnsafeBinder (op , _) => { self . consume_operand (location , op) ; } } } #[doc = " Simulates an access to a place."] fn access_place (& mut self , location : Location , place : Place < 'tcx > , kind : (AccessDepth , ReadOrWrite) , _is_local_mutation_allowed : LocalMutationIsAllowed ,) { let (sd , rw) = kind ; self . check_access_for_conflict (location , place , sd , rw) ; } fn check_access_for_conflict (& mut self , location : Location , place : Place < 'tcx > , sd : AccessDepth , rw : ReadOrWrite ,) { debug ! ("check_access_for_conflict(location={:?}, place={:?}, sd={:?}, rw={:?})" , location , place , sd , rw ,) ; each_borrow_involving_path (self , self . tcx , self . body , (sd , place) , self . borrow_set , | _ | true , | this , borrow_index , borrow | { match (rw , borrow . kind) { (Activation (_ , activating) , _) if activating == borrow_index => { } (Read (_) , BorrowKind :: Fake (_) | BorrowKind :: Shared) | (Read (ReadKind :: Borrow (BorrowKind :: Fake (FakeBorrowKind :: Shallow))) , BorrowKind :: Mut { .. } ,) => { } (Read (_) , BorrowKind :: Mut { .. }) => { if ! is_active (this . dominators , borrow , location) { assert ! (borrow . kind . allows_two_phase_borrow ()) ; return ControlFlow :: Continue (()) ; } this . emit_loan_invalidated_at (borrow_index , location) ; } (Reservation (_) | Activation (_ , _) | Write (_) , _) => { this . emit_loan_invalidated_at (borrow_index , location) ; } } ControlFlow :: Continue (()) } ,) ; } #[doc = " Generates a new `loan_invalidated_at(L, B)` fact."] fn emit_loan_invalidated_at (& mut self , b : BorrowIndex , l : Location) { let lidx = self . location_table . start_index (l) ; self . facts . loan_invalidated_at . push ((lidx , b)) ; } fn check_activations (& mut self , location : Location) { for & borrow_index in self . borrow_set . activations_at_location (location) { let borrow = & self . borrow_set [borrow_index] ; assert ! (match borrow . kind { BorrowKind :: Shared | BorrowKind :: Fake (_) => false , BorrowKind :: Mut { .. } => true , }) ; self . access_place (location , borrow . borrowed_place , (Deep , Activation (WriteKind :: MutableBorrow (borrow . kind) , borrow_index)) , LocalMutationIsAllowed :: No ,) ; } } }}}