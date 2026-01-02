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
mkuse!{use std :: assert_matches :: assert_matches ;}
mkuse!{use rustc_abi :: VariantIdx ;}
mkuse!{use rustc_index :: Idx ;}
mkuse!{use rustc_index :: bit_set :: { DenseBitSet , MixedBitSet } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: mir :: { self , Body , CallReturnPlaces , Location , SwitchTargetValue , TerminatorEdges , } ;}
mkuse!{use rustc_middle :: ty :: util :: Discr ;}
mkuse!{use rustc_middle :: ty :: { self , TyCtxt } ;}
mkuse!{use smallvec :: SmallVec ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use crate :: drop_flag_effects :: { DropFlagState , InactiveVariants } ;}
mkuse!{use crate :: move_paths :: { HasMoveData , InitIndex , InitKind , LookupResult , MoveData , MovePathIndex } ;}
mkuse!{use crate :: { Analysis , GenKill , MaybeReachable , drop_flag_effects , drop_flag_effects_for_function_entry , drop_flag_effects_for_location , on_all_children_bits , on_lookup_result_bits , } ;}
mkitem!{mkstruct!{pub struct MaybePlacesSwitchIntData < 'tcx > { enum_place : mir :: Place < 'tcx > , discriminants : Vec < (VariantIdx , Discr < 'tcx >) > , index : usize , }}}
mkitem!{mkimpl!{impl < 'tcx > MaybePlacesSwitchIntData < 'tcx > { #[doc = " Creates a `SmallVec` mapping each target in `targets` to its `VariantIdx`."] fn variants (& mut self , targets : & mir :: SwitchTargets) -> SmallVec < [VariantIdx ; 4] > { self . index = 0 ; targets . all_values () . iter () . map (| value | self . next_discr (value . get ())) . collect () } fn next_discr (& mut self , value : u128) -> VariantIdx { loop { let (variant , discr) = self . discriminants [self . index] ; self . index += 1 ; if discr . val == value { return variant ; } } } }}}
mkitem!{mkimpl!{impl < 'tcx > MaybePlacesSwitchIntData < 'tcx > { fn new (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , block : mir :: BasicBlock , discr : & mir :: Operand < 'tcx > ,) -> Option < Self > { let Some (discr) = discr . place () else { return None } ; let block_data = & body [block] ; for statement in block_data . statements . iter () . rev () { match statement . kind { mir :: StatementKind :: Assign (box (lhs , mir :: Rvalue :: Discriminant (enum_place))) if lhs == discr => { match enum_place . ty (body , tcx) . ty . kind () { ty :: Adt (enum_def , _) => { return Some (MaybePlacesSwitchIntData { enum_place , discriminants : enum_def . discriminants (tcx) . collect () , index : 0 , }) ; } ty :: Coroutine (..) => break , t => bug ! ("`discriminant` called on unexpected type {:?}" , t) , } } mir :: StatementKind :: Coverage (_) => continue , _ => break , } } None } }}}
mkitem!{mkstruct!{#[doc = " `MaybeInitializedPlaces` tracks all places that might be"] #[doc = " initialized upon reaching a particular point in the control flow"] #[doc = " for a function."] #[doc = ""] #[doc = " For example, in code like the following, we have corresponding"] #[doc = " dataflow information shown in the right-hand comments."] #[doc = ""] #[doc = " ```rust"] #[doc = " struct S;"] #[doc = " #[rustfmt::skip]"] #[doc = " fn foo(pred: bool) {                        // maybe-init:"] #[doc = "                                             // {}"] #[doc = "     let a = S; let mut b = S; let c; let d; // {a, b}"] #[doc = ""] #[doc = "     if pred {"] #[doc = "         drop(a);                            // {   b}"] #[doc = "         b = S;                              // {   b}"] #[doc = ""] #[doc = "     } else {"] #[doc = "         drop(b);                            // {a}"] #[doc = "         d = S;                              // {a,       d}"] #[doc = ""] #[doc = "     }                                       // {a, b,    d}"] #[doc = ""] #[doc = "     c = S;                                  // {a, b, c, d}"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " To determine whether a place is *definitely* initialized at a"] #[doc = " particular control-flow point, one can take the set-complement"] #[doc = " of the data from `MaybeUninitializedPlaces` at the corresponding"] #[doc = " control-flow point."] #[doc = ""] #[doc = " Similarly, at a given `drop` statement, the set-intersection"] #[doc = " between this data and `MaybeUninitializedPlaces` yields the set of"] #[doc = " places that would require a dynamic drop-flag at that statement."] pub struct MaybeInitializedPlaces < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , body : & 'a Body < 'tcx > , move_data : & 'a MoveData < 'tcx > , exclude_inactive_in_otherwise : bool , skip_unreachable_unwind : bool , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > MaybeInitializedPlaces < 'a , 'tcx > { pub fn new (tcx : TyCtxt < 'tcx > , body : & 'a Body < 'tcx > , move_data : & 'a MoveData < 'tcx >) -> Self { MaybeInitializedPlaces { tcx , body , move_data , exclude_inactive_in_otherwise : false , skip_unreachable_unwind : false , } } #[doc = " Ensures definitely inactive variants are excluded from the set of initialized places for"] #[doc = " blocks reached through an `otherwise` edge."] pub fn exclude_inactive_in_otherwise (mut self) -> Self { self . exclude_inactive_in_otherwise = true ; self } pub fn skipping_unreachable_unwind (mut self) -> Self { self . skip_unreachable_unwind = true ; self } pub fn is_unwind_dead (& self , place : mir :: Place < 'tcx > , state : & < Self as Analysis < 'tcx > > :: Domain ,) -> bool { if let LookupResult :: Exact (path) = self . move_data () . rev_lookup . find (place . as_ref ()) { let mut maybe_live = false ; on_all_children_bits (self . move_data () , path , | child | { maybe_live |= state . contains (child) ; }) ; ! maybe_live } else { false } } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > HasMoveData < 'tcx > for MaybeInitializedPlaces < 'a , 'tcx > { fn move_data (& self) -> & MoveData < 'tcx > { self . move_data } }}}
mkitem!{mkstruct!{#[doc = " `MaybeUninitializedPlaces` tracks all places that might be"] #[doc = " uninitialized upon reaching a particular point in the control flow"] #[doc = " for a function."] #[doc = ""] #[doc = " For example, in code like the following, we have corresponding"] #[doc = " dataflow information shown in the right-hand comments."] #[doc = ""] #[doc = " ```rust"] #[doc = " struct S;"] #[doc = " #[rustfmt::skip]"] #[doc = " fn foo(pred: bool) {                        // maybe-uninit:"] #[doc = "                                             // {a, b, c, d}"] #[doc = "     let a = S; let mut b = S; let c; let d; // {      c, d}"] #[doc = ""] #[doc = "     if pred {"] #[doc = "         drop(a);                            // {a,    c, d}"] #[doc = "         b = S;                              // {a,    c, d}"] #[doc = ""] #[doc = "     } else {"] #[doc = "         drop(b);                            // {   b, c, d}"] #[doc = "         d = S;                              // {   b, c   }"] #[doc = ""] #[doc = "     }                                       // {a, b, c, d}"] #[doc = ""] #[doc = "     c = S;                                  // {a, b,    d}"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " To determine whether a place is *definitely* uninitialized at a"] #[doc = " particular control-flow point, one can take the set-complement"] #[doc = " of the data from `MaybeInitializedPlaces` at the corresponding"] #[doc = " control-flow point."] #[doc = ""] #[doc = " Similarly, at a given `drop` statement, the set-intersection"] #[doc = " between this data and `MaybeInitializedPlaces` yields the set of"] #[doc = " places that would require a dynamic drop-flag at that statement."] pub struct MaybeUninitializedPlaces < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , body : & 'a Body < 'tcx > , move_data : & 'a MoveData < 'tcx > , mark_inactive_variants_as_uninit : bool , include_inactive_in_otherwise : bool , skip_unreachable_unwind : DenseBitSet < mir :: BasicBlock > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > MaybeUninitializedPlaces < 'a , 'tcx > { pub fn new (tcx : TyCtxt < 'tcx > , body : & 'a Body < 'tcx > , move_data : & 'a MoveData < 'tcx >) -> Self { MaybeUninitializedPlaces { tcx , body , move_data , mark_inactive_variants_as_uninit : false , include_inactive_in_otherwise : false , skip_unreachable_unwind : DenseBitSet :: new_empty (body . basic_blocks . len ()) , } } #[doc = " Causes inactive enum variants to be marked as \"maybe uninitialized\" after a switch on an"] #[doc = " enum discriminant."] #[doc = ""] #[doc = " This is correct in a vacuum but is not the default because it causes problems in the borrow"] #[doc = " checker, where this information gets propagated along `FakeEdge`s."] pub fn mark_inactive_variants_as_uninit (mut self) -> Self { self . mark_inactive_variants_as_uninit = true ; self } #[doc = " Ensures definitely inactive variants are included in the set of uninitialized places for"] #[doc = " blocks reached through an `otherwise` edge."] pub fn include_inactive_in_otherwise (mut self) -> Self { self . include_inactive_in_otherwise = true ; self } pub fn skipping_unreachable_unwind (mut self , unreachable_unwind : DenseBitSet < mir :: BasicBlock > ,) -> Self { self . skip_unreachable_unwind = unreachable_unwind ; self } }}}
mkitem!{mkimpl!{impl < 'tcx > HasMoveData < 'tcx > for MaybeUninitializedPlaces < '_ , 'tcx > { fn move_data (& self) -> & MoveData < 'tcx > { self . move_data } }}}
mkitem!{mkstruct!{#[doc = " `EverInitializedPlaces` tracks all places that might have ever been"] #[doc = " initialized upon reaching a particular point in the control flow"] #[doc = " for a function, without an intervening `StorageDead`."] #[doc = ""] #[doc = " This dataflow is used to determine if an immutable local variable may"] #[doc = " be assigned to."] #[doc = ""] #[doc = " For example, in code like the following, we have corresponding"] #[doc = " dataflow information shown in the right-hand comments."] #[doc = ""] #[doc = " ```rust"] #[doc = " struct S;"] #[doc = " #[rustfmt::skip]"] #[doc = " fn foo(pred: bool) {                        // ever-init:"] #[doc = "                                             // {          }"] #[doc = "     let a = S; let mut b = S; let c; let d; // {a, b      }"] #[doc = ""] #[doc = "     if pred {"] #[doc = "         drop(a);                            // {a, b,     }"] #[doc = "         b = S;                              // {a, b,     }"] #[doc = ""] #[doc = "     } else {"] #[doc = "         drop(b);                            // {a, b,      }"] #[doc = "         d = S;                              // {a, b,    d }"] #[doc = ""] #[doc = "     }                                       // {a, b,    d }"] #[doc = ""] #[doc = "     c = S;                                  // {a, b, c, d }"] #[doc = " }"] #[doc = " ```"] pub struct EverInitializedPlaces < 'a , 'tcx > { body : & 'a Body < 'tcx > , move_data : & 'a MoveData < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > EverInitializedPlaces < 'a , 'tcx > { pub fn new (body : & 'a Body < 'tcx > , move_data : & 'a MoveData < 'tcx >) -> Self { EverInitializedPlaces { body , move_data } } }}}
mkitem!{mkimpl!{impl < 'tcx > HasMoveData < 'tcx > for EverInitializedPlaces < '_ , 'tcx > { fn move_data (& self) -> & MoveData < 'tcx > { self . move_data } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > MaybeInitializedPlaces < 'a , 'tcx > { fn update_bits (state : & mut < Self as Analysis < 'tcx > > :: Domain , path : MovePathIndex , dfstate : DropFlagState ,) { match dfstate { DropFlagState :: Absent => state . kill (path) , DropFlagState :: Present => state . gen_ (path) , } } }}}
mkitem!{mkimpl!{impl < 'tcx > MaybeUninitializedPlaces < '_ , 'tcx > { fn update_bits (state : & mut < Self as Analysis < 'tcx > > :: Domain , path : MovePathIndex , dfstate : DropFlagState ,) { match dfstate { DropFlagState :: Absent => state . gen_ (path) , DropFlagState :: Present => state . kill (path) , } } }}}
mkitem!{mkimpl!{impl < 'tcx > Analysis < 'tcx > for MaybeInitializedPlaces < '_ , 'tcx > { #[doc = " There can be many more `MovePathIndex` than there are locals in a MIR body."] #[doc = " We use a mixed bitset to avoid paying too high a memory footprint."] type Domain = MaybeReachable < MixedBitSet < MovePathIndex > > ; type SwitchIntData = MaybePlacesSwitchIntData < 'tcx > ; const NAME : & 'static str = "maybe_init" ; fn bottom_value (& self , _ : & mir :: Body < 'tcx >) -> Self :: Domain { MaybeReachable :: Unreachable } fn initialize_start_block (& self , _ : & mir :: Body < 'tcx > , state : & mut Self :: Domain) { * state = MaybeReachable :: Reachable (MixedBitSet :: new_empty (self . move_data () . move_paths . len ())) ; drop_flag_effects_for_function_entry (self . body , self . move_data , | path , s | { assert ! (s == DropFlagState :: Present) ; state . gen_ (path) ; }) ; } fn apply_primary_statement_effect (& mut self , state : & mut Self :: Domain , statement : & mir :: Statement < 'tcx > , location : Location ,) { drop_flag_effects_for_location (self . body , self . move_data , location , | path , s | { Self :: update_bits (state , path , s) }) ; if self . tcx . sess . opts . unstable_opts . precise_enum_drop_elaboration && let Some ((_ , rvalue)) = statement . kind . as_assign () && let mir :: Rvalue :: Ref (_ , mir :: BorrowKind :: Mut { .. } , place) | mir :: Rvalue :: RawPtr (_ , place) = rvalue && let LookupResult :: Exact (mpi) = self . move_data () . rev_lookup . find (place . as_ref ()) { on_all_children_bits (self . move_data () , mpi , | child | { state . gen_ (child) ; }) } } fn apply_primary_terminator_effect < 'mir > (& mut self , state : & mut Self :: Domain , terminator : & 'mir mir :: Terminator < 'tcx > , location : Location ,) -> TerminatorEdges < 'mir , 'tcx > { let mut edges = terminator . edges () ; if self . skip_unreachable_unwind && let mir :: TerminatorKind :: Drop { target , unwind , place , replace : _ , drop : _ , async_fut : _ , } = terminator . kind && matches ! (unwind , mir :: UnwindAction :: Cleanup (_)) && self . is_unwind_dead (place , state) { edges = TerminatorEdges :: Single (target) ; } drop_flag_effects_for_location (self . body , self . move_data , location , | path , s | { Self :: update_bits (state , path , s) }) ; edges } fn apply_call_return_effect (& mut self , state : & mut Self :: Domain , _block : mir :: BasicBlock , return_places : CallReturnPlaces < '_ , 'tcx > ,) { return_places . for_each (| place | { on_lookup_result_bits (self . move_data () , self . move_data () . rev_lookup . find (place . as_ref ()) , | mpi | { state . gen_ (mpi) ; } ,) ; }) ; } fn get_switch_int_data (& mut self , block : mir :: BasicBlock , discr : & mir :: Operand < 'tcx > ,) -> Option < Self :: SwitchIntData > { if ! self . tcx . sess . opts . unstable_opts . precise_enum_drop_elaboration { return None ; } MaybePlacesSwitchIntData :: new (self . tcx , self . body , block , discr) } fn apply_switch_int_edge_effect (& mut self , data : & mut Self :: SwitchIntData , state : & mut Self :: Domain , value : SwitchTargetValue , targets : & mir :: SwitchTargets ,) { let inactive_variants = match value { SwitchTargetValue :: Normal (value) => InactiveVariants :: Active (data . next_discr (value)) , SwitchTargetValue :: Otherwise if self . exclude_inactive_in_otherwise => { InactiveVariants :: Inactives (data . variants (targets)) } _ => return , } ; drop_flag_effects :: on_all_inactive_variants (self . move_data , data . enum_place , & inactive_variants , | mpi | state . kill (mpi) ,) ; } }}}
mkitem!{#[doc = " There can be many more `MovePathIndex` than there are locals in a MIR body."] #[doc = " We use a mixed bitset to avoid paying too high a memory footprint."] pub type MaybeUninitializedPlacesDomain = MixedBitSet < MovePathIndex > ;}
mkitem!{mkimpl!{impl < 'tcx > Analysis < 'tcx > for MaybeUninitializedPlaces < '_ , 'tcx > { type Domain = MaybeUninitializedPlacesDomain ; type SwitchIntData = MaybePlacesSwitchIntData < 'tcx > ; const NAME : & 'static str = "maybe_uninit" ; fn bottom_value (& self , _ : & mir :: Body < 'tcx >) -> Self :: Domain { MixedBitSet :: new_empty (self . move_data () . move_paths . len ()) } fn initialize_start_block (& self , _ : & mir :: Body < 'tcx > , state : & mut Self :: Domain) { state . insert_all () ; drop_flag_effects_for_function_entry (self . body , self . move_data , | path , s | { assert ! (s == DropFlagState :: Present) ; state . remove (path) ; }) ; } fn apply_primary_statement_effect (& mut self , state : & mut Self :: Domain , _statement : & mir :: Statement < 'tcx > , location : Location ,) { drop_flag_effects_for_location (self . body , self . move_data , location , | path , s | { Self :: update_bits (state , path , s) }) ; } fn apply_primary_terminator_effect < 'mir > (& mut self , state : & mut Self :: Domain , terminator : & 'mir mir :: Terminator < 'tcx > , location : Location ,) -> TerminatorEdges < 'mir , 'tcx > { drop_flag_effects_for_location (self . body , self . move_data , location , | path , s | { Self :: update_bits (state , path , s) }) ; if self . skip_unreachable_unwind . contains (location . block) { let mir :: TerminatorKind :: Drop { target , unwind , .. } = terminator . kind else { bug ! () } ; assert_matches ! (unwind , mir :: UnwindAction :: Cleanup (_)) ; TerminatorEdges :: Single (target) } else { terminator . edges () } } fn apply_call_return_effect (& mut self , state : & mut Self :: Domain , _block : mir :: BasicBlock , return_places : CallReturnPlaces < '_ , 'tcx > ,) { return_places . for_each (| place | { on_lookup_result_bits (self . move_data () , self . move_data () . rev_lookup . find (place . as_ref ()) , | mpi | { state . kill (mpi) ; } ,) ; }) ; } fn get_switch_int_data (& mut self , block : mir :: BasicBlock , discr : & mir :: Operand < 'tcx > ,) -> Option < Self :: SwitchIntData > { if ! self . tcx . sess . opts . unstable_opts . precise_enum_drop_elaboration { return None ; } if ! self . mark_inactive_variants_as_uninit { return None ; } MaybePlacesSwitchIntData :: new (self . tcx , self . body , block , discr) } fn apply_switch_int_edge_effect (& mut self , data : & mut Self :: SwitchIntData , state : & mut Self :: Domain , value : SwitchTargetValue , targets : & mir :: SwitchTargets ,) { let inactive_variants = match value { SwitchTargetValue :: Normal (value) => InactiveVariants :: Active (data . next_discr (value)) , SwitchTargetValue :: Otherwise if self . include_inactive_in_otherwise => { InactiveVariants :: Inactives (data . variants (targets)) } _ => return , } ; drop_flag_effects :: on_all_inactive_variants (self . move_data , data . enum_place , & inactive_variants , | mpi | state . gen_ (mpi) ,) ; } }}}
mkitem!{#[doc = " There can be many more `InitIndex` than there are locals in a MIR body."] #[doc = " We use a mixed bitset to avoid paying too high a memory footprint."] pub type EverInitializedPlacesDomain = MixedBitSet < InitIndex > ;}
mkitem!{mkimpl!{impl < 'tcx > Analysis < 'tcx > for EverInitializedPlaces < '_ , 'tcx > { type Domain = EverInitializedPlacesDomain ; const NAME : & 'static str = "ever_init" ; fn bottom_value (& self , _ : & mir :: Body < 'tcx >) -> Self :: Domain { MixedBitSet :: new_empty (self . move_data () . inits . len ()) } fn initialize_start_block (& self , body : & mir :: Body < 'tcx > , state : & mut Self :: Domain) { for arg_init in 0 .. body . arg_count { state . insert (InitIndex :: new (arg_init)) ; } } #[instrument (skip (self , state) , level = "debug")] fn apply_primary_statement_effect (& mut self , state : & mut Self :: Domain , stmt : & mir :: Statement < 'tcx > , location : Location ,) { let move_data = self . move_data () ; let init_path_map = & move_data . init_path_map ; let init_loc_map = & move_data . init_loc_map ; let rev_lookup = & move_data . rev_lookup ; debug ! ("initializes move_indexes {:?}" , init_loc_map [location]) ; state . gen_all (init_loc_map [location] . iter () . copied ()) ; if let mir :: StatementKind :: StorageDead (local) = stmt . kind && let Some (move_path_index) = rev_lookup . find_local (local) { debug ! ("clears the ever initialized status of {:?}" , init_path_map [move_path_index]) ; state . kill_all (init_path_map [move_path_index] . iter () . copied ()) ; } } #[instrument (skip (self , state , terminator) , level = "debug")] fn apply_primary_terminator_effect < 'mir > (& mut self , state : & mut Self :: Domain , terminator : & 'mir mir :: Terminator < 'tcx > , location : Location ,) -> TerminatorEdges < 'mir , 'tcx > { let (body , move_data) = (self . body , self . move_data ()) ; let term = body [location . block] . terminator () ; let init_loc_map = & move_data . init_loc_map ; debug ! (? term) ; debug ! ("initializes move_indexes {:?}" , init_loc_map [location]) ; state . gen_all (init_loc_map [location] . iter () . filter (| init_index | { move_data . inits [* * init_index] . kind != InitKind :: NonPanicPathOnly }) . copied () ,) ; terminator . edges () } fn apply_call_return_effect (& mut self , state : & mut Self :: Domain , block : mir :: BasicBlock , _return_places : CallReturnPlaces < '_ , 'tcx > ,) { let move_data = self . move_data () ; let init_loc_map = & move_data . init_loc_map ; let call_loc = self . body . terminator_loc (block) ; for init_index in & init_loc_map [call_loc] { state . gen_ (* init_index) ; } } }}}