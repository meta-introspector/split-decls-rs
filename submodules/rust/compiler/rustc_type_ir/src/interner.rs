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
mkuse!{use std :: borrow :: Borrow ;}
mkuse!{use std :: fmt :: Debug ;}
mkuse!{use std :: hash :: Hash ;}
mkuse!{use std :: ops :: Deref ;}
mkuse!{use rustc_ast_ir :: Movability ;}
mkuse!{use rustc_index :: bit_set :: DenseBitSet ;}
mkuse!{use crate :: fold :: TypeFoldable ;}
mkuse!{use crate :: inherent :: * ;}
mkuse!{use crate :: ir_print :: IrPrint ;}
mkuse!{use crate :: lang_items :: { SolverAdtLangItem , SolverLangItem , SolverTraitLangItem } ;}
mkuse!{use crate :: relate :: Relate ;}
mkuse!{use crate :: solve :: { CanonicalInput , ExternalConstraintsData , PredefinedOpaquesData , QueryResult , inspect , } ;}
mkuse!{use crate :: visit :: { Flags , TypeVisitable } ;}
mkuse!{use crate :: { self as ty , CanonicalParamEnvCacheEntry , search_graph } ;}
mkitem!{mktrait!{#[cfg_attr (feature = "nightly" , rustc_diagnostic_item = "type_ir_interner")] pub trait Interner : Sized + Copy + IrPrint < ty :: AliasTy < Self > > + IrPrint < ty :: AliasTerm < Self > > + IrPrint < ty :: TraitRef < Self > > + IrPrint < ty :: TraitPredicate < Self > > + IrPrint < ty :: HostEffectPredicate < Self > > + IrPrint < ty :: ExistentialTraitRef < Self > > + IrPrint < ty :: ExistentialProjection < Self > > + IrPrint < ty :: ProjectionPredicate < Self > > + IrPrint < ty :: NormalizesTo < Self > > + IrPrint < ty :: SubtypePredicate < Self > > + IrPrint < ty :: CoercePredicate < Self > > + IrPrint < ty :: FnSig < Self > > + IrPrint < ty :: PatternKind < Self > > { fn next_trait_solver_globally (self) -> bool { true } type DefId : DefId < Self > ; type LocalDefId : Copy + Debug + Hash + Eq + Into < Self :: DefId > + TypeFoldable < Self > ; type TraitId : SpecificDefId < Self > ; type ForeignId : SpecificDefId < Self > ; type FunctionId : SpecificDefId < Self > ; type ClosureId : SpecificDefId < Self > ; type CoroutineClosureId : SpecificDefId < Self > ; type CoroutineId : SpecificDefId < Self > ; type AdtId : SpecificDefId < Self > ; type ImplId : SpecificDefId < Self > ; type Span : Span < Self > ; type GenericArgs : GenericArgs < Self > ; type GenericArgsSlice : Copy + Debug + Hash + Eq + SliceLike < Item = Self :: GenericArg > ; type GenericArg : GenericArg < Self > ; type Term : Term < Self > ; type BoundVarKinds : Copy + Debug + Hash + Eq + SliceLike < Item = Self :: BoundVarKind > + Default ; type BoundVarKind : Copy + Debug + Hash + Eq ; type PredefinedOpaques : Copy + Debug + Hash + Eq + TypeFoldable < Self > + Deref < Target = PredefinedOpaquesData < Self > > ; fn mk_predefined_opaques_in_body (self , data : PredefinedOpaquesData < Self > ,) -> Self :: PredefinedOpaques ; type LocalDefIds : Copy + Debug + Hash + Default + Eq + TypeVisitable < Self > + SliceLike < Item = Self :: LocalDefId > ; type CanonicalVarKinds : Copy + Debug + Hash + Eq + SliceLike < Item = ty :: CanonicalVarKind < Self > > + Default ; fn mk_canonical_var_kinds (self , kinds : & [ty :: CanonicalVarKind < Self >] ,) -> Self :: CanonicalVarKinds ; type ExternalConstraints : Copy + Debug + Hash + Eq + TypeFoldable < Self > + Deref < Target = ExternalConstraintsData < Self > > ; fn mk_external_constraints (self , data : ExternalConstraintsData < Self > ,) -> Self :: ExternalConstraints ; type DepNodeIndex ; type Tracked < T : Debug + Clone > : Debug ; fn mk_tracked < T : Debug + Clone > (self , data : T , dep_node : Self :: DepNodeIndex ,) -> Self :: Tracked < T > ; fn get_tracked < T : Debug + Clone > (self , tracked : & Self :: Tracked < T >) -> T ; fn with_cached_task < T > (self , task : impl FnOnce () -> T) -> (T , Self :: DepNodeIndex) ; type Ty : Ty < Self > ; type Tys : Tys < Self > ; type FnInputTys : Copy + Debug + Hash + Eq + SliceLike < Item = Self :: Ty > + TypeVisitable < Self > ; type ParamTy : ParamLike ; type BoundTy : BoundVarLike < Self > ; type PlaceholderTy : PlaceholderLike < Self , Bound = Self :: BoundTy > ; type Symbol : Copy + Hash + PartialEq + Eq + Debug ; type ErrorGuaranteed : Copy + Debug + Hash + Eq ; type BoundExistentialPredicates : BoundExistentialPredicates < Self > ; type AllocId : Copy + Debug + Hash + Eq ; type Pat : Copy + Debug + Hash + Eq + Debug + Relate < Self > + Flags + IntoKind < Kind = ty :: PatternKind < Self > > ; type PatList : Copy + Debug + Hash + Default + Eq + TypeVisitable < Self > + SliceLike < Item = Self :: Pat > ; type Safety : Safety < Self > ; type Abi : Abi < Self > ; type Const : Const < Self > ; type ParamConst : Copy + Debug + Hash + Eq + ParamLike ; type BoundConst : BoundVarLike < Self > ; type PlaceholderConst : PlaceholderConst < Self > ; type ValueConst : ValueConst < Self > ; type ExprConst : ExprConst < Self > ; type ValTree : Copy + Debug + Hash + Eq ; type Region : Region < Self > ; type EarlyParamRegion : ParamLike ; type LateParamRegion : Copy + Debug + Hash + Eq ; type BoundRegion : BoundVarLike < Self > ; type PlaceholderRegion : PlaceholderLike < Self , Bound = Self :: BoundRegion > ; type RegionAssumptions : Copy + Debug + Hash + Eq + SliceLike < Item = ty :: OutlivesPredicate < Self , Self :: GenericArg > > + TypeFoldable < Self > ; type ParamEnv : ParamEnv < Self > ; type Predicate : Predicate < Self > ; type Clause : Clause < Self > ; type Clauses : Clauses < Self > ; fn with_global_cache < R > (self , f : impl FnOnce (& mut search_graph :: GlobalCache < Self >) -> R) -> R ; fn canonical_param_env_cache_get_or_insert < R > (self , param_env : Self :: ParamEnv , f : impl FnOnce () -> CanonicalParamEnvCacheEntry < Self > , from_entry : impl FnOnce (& CanonicalParamEnvCacheEntry < Self >) -> R ,) -> R ; fn evaluation_is_concurrent (& self) -> bool ; fn expand_abstract_consts < T : TypeFoldable < Self > > (self , t : T) -> T ; type GenericsOf : GenericsOf < Self > ; fn generics_of (self , def_id : Self :: DefId) -> Self :: GenericsOf ; type VariancesOf : Copy + Debug + SliceLike < Item = ty :: Variance > ; fn variances_of (self , def_id : Self :: DefId) -> Self :: VariancesOf ; fn opt_alias_variances (self , kind : impl Into < ty :: AliasTermKind > , def_id : Self :: DefId ,) -> Option < Self :: VariancesOf > ; fn type_of (self , def_id : Self :: DefId) -> ty :: EarlyBinder < Self , Self :: Ty > ; fn type_of_opaque_hir_typeck (self , def_id : Self :: LocalDefId) -> ty :: EarlyBinder < Self , Self :: Ty > ; type AdtDef : AdtDef < Self > ; fn adt_def (self , adt_def_id : Self :: AdtId) -> Self :: AdtDef ; fn alias_ty_kind (self , alias : ty :: AliasTy < Self >) -> ty :: AliasTyKind ; fn alias_term_kind (self , alias : ty :: AliasTerm < Self >) -> ty :: AliasTermKind ; fn trait_ref_and_own_args_for_alias (self , def_id : Self :: DefId , args : Self :: GenericArgs ,) -> (ty :: TraitRef < Self > , Self :: GenericArgsSlice) ; fn mk_args (self , args : & [Self :: GenericArg]) -> Self :: GenericArgs ; fn mk_args_from_iter < I , T > (self , args : I) -> T :: Output where I : Iterator < Item = T > , T : CollectAndApply < Self :: GenericArg , Self :: GenericArgs > ; fn check_args_compatible (self , def_id : Self :: DefId , args : Self :: GenericArgs) -> bool ; fn debug_assert_args_compatible (self , def_id : Self :: DefId , args : Self :: GenericArgs) ; #[doc = " Assert that the args from an `ExistentialTraitRef` or `ExistentialProjection`"] #[doc = " are compatible with the `DefId`."] fn debug_assert_existential_args_compatible (self , def_id : Self :: DefId , args : Self :: GenericArgs) ; fn mk_type_list_from_iter < I , T > (self , args : I) -> T :: Output where I : Iterator < Item = T > , T : CollectAndApply < Self :: Ty , Self :: Tys > ; fn parent (self , def_id : Self :: DefId) -> Self :: DefId ; fn recursion_limit (self) -> usize ; type Features : Features < Self > ; fn features (self) -> Self :: Features ; fn coroutine_hidden_types (self , def_id : Self :: CoroutineId ,) -> ty :: EarlyBinder < Self , ty :: Binder < Self , ty :: CoroutineWitnessTypes < Self > > > ; fn fn_sig (self , def_id : Self :: FunctionId ,) -> ty :: EarlyBinder < Self , ty :: Binder < Self , ty :: FnSig < Self > > > ; fn coroutine_movability (self , def_id : Self :: CoroutineId) -> Movability ; fn coroutine_for_closure (self , def_id : Self :: CoroutineClosureId) -> Self :: CoroutineId ; fn generics_require_sized_self (self , def_id : Self :: DefId) -> bool ; fn item_bounds (self , def_id : Self :: DefId ,) -> ty :: EarlyBinder < Self , impl IntoIterator < Item = Self :: Clause > > ; fn item_self_bounds (self , def_id : Self :: DefId ,) -> ty :: EarlyBinder < Self , impl IntoIterator < Item = Self :: Clause > > ; fn item_non_self_bounds (self , def_id : Self :: DefId ,) -> ty :: EarlyBinder < Self , impl IntoIterator < Item = Self :: Clause > > ; fn predicates_of (self , def_id : Self :: DefId ,) -> ty :: EarlyBinder < Self , impl IntoIterator < Item = Self :: Clause > > ; fn own_predicates_of (self , def_id : Self :: DefId ,) -> ty :: EarlyBinder < Self , impl IntoIterator < Item = Self :: Clause > > ; fn explicit_super_predicates_of (self , def_id : Self :: TraitId ,) -> ty :: EarlyBinder < Self , impl IntoIterator < Item = (Self :: Clause , Self :: Span) > > ; fn explicit_implied_predicates_of (self , def_id : Self :: DefId ,) -> ty :: EarlyBinder < Self , impl IntoIterator < Item = (Self :: Clause , Self :: Span) > > ; #[doc = " This is equivalent to computing the super-predicates of the trait for this impl"] #[doc = " and filtering them to the outlives predicates. This is purely for performance."] fn impl_super_outlives (self , impl_def_id : Self :: ImplId ,) -> ty :: EarlyBinder < Self , impl IntoIterator < Item = Self :: Clause > > ; fn impl_is_const (self , def_id : Self :: ImplId) -> bool ; fn fn_is_const (self , def_id : Self :: FunctionId) -> bool ; fn alias_has_const_conditions (self , def_id : Self :: DefId) -> bool ; fn const_conditions (self , def_id : Self :: DefId ,) -> ty :: EarlyBinder < Self , impl IntoIterator < Item = ty :: Binder < Self , ty :: TraitRef < Self > > > > ; fn explicit_implied_const_bounds (self , def_id : Self :: DefId ,) -> ty :: EarlyBinder < Self , impl IntoIterator < Item = ty :: Binder < Self , ty :: TraitRef < Self > > > > ; fn impl_self_is_guaranteed_unsized (self , def_id : Self :: ImplId) -> bool ; fn has_target_features (self , def_id : Self :: FunctionId) -> bool ; fn require_lang_item (self , lang_item : SolverLangItem) -> Self :: DefId ; fn require_trait_lang_item (self , lang_item : SolverTraitLangItem) -> Self :: TraitId ; fn require_adt_lang_item (self , lang_item : SolverAdtLangItem) -> Self :: AdtId ; fn is_lang_item (self , def_id : Self :: DefId , lang_item : SolverLangItem) -> bool ; fn is_trait_lang_item (self , def_id : Self :: TraitId , lang_item : SolverTraitLangItem) -> bool ; fn is_adt_lang_item (self , def_id : Self :: AdtId , lang_item : SolverAdtLangItem) -> bool ; fn is_default_trait (self , def_id : Self :: TraitId) -> bool ; fn as_lang_item (self , def_id : Self :: DefId) -> Option < SolverLangItem > ; fn as_trait_lang_item (self , def_id : Self :: TraitId) -> Option < SolverTraitLangItem > ; fn as_adt_lang_item (self , def_id : Self :: AdtId) -> Option < SolverAdtLangItem > ; fn associated_type_def_ids (self , def_id : Self :: DefId) -> impl IntoIterator < Item = Self :: DefId > ; fn for_each_relevant_impl (self , trait_def_id : Self :: TraitId , self_ty : Self :: Ty , f : impl FnMut (Self :: ImplId) ,) ; fn for_each_blanket_impl (self , trait_def_id : Self :: TraitId , f : impl FnMut (Self :: ImplId)) ; fn has_item_definition (self , def_id : Self :: DefId) -> bool ; fn impl_specializes (self , impl_def_id : Self :: ImplId , victim_def_id : Self :: ImplId) -> bool ; fn impl_is_default (self , impl_def_id : Self :: ImplId) -> bool ; fn impl_trait_ref (self , impl_def_id : Self :: ImplId) -> ty :: EarlyBinder < Self , ty :: TraitRef < Self > > ; fn impl_polarity (self , impl_def_id : Self :: ImplId) -> ty :: ImplPolarity ; fn trait_is_auto (self , trait_def_id : Self :: TraitId) -> bool ; fn trait_is_coinductive (self , trait_def_id : Self :: TraitId) -> bool ; fn trait_is_alias (self , trait_def_id : Self :: TraitId) -> bool ; fn trait_is_dyn_compatible (self , trait_def_id : Self :: TraitId) -> bool ; fn trait_is_fundamental (self , def_id : Self :: TraitId) -> bool ; fn trait_may_be_implemented_via_object (self , trait_def_id : Self :: TraitId) -> bool ; #[doc = " Returns `true` if this is an `unsafe trait`."] fn trait_is_unsafe (self , trait_def_id : Self :: TraitId) -> bool ; fn is_impl_trait_in_trait (self , def_id : Self :: DefId) -> bool ; fn delay_bug (self , msg : impl ToString) -> Self :: ErrorGuaranteed ; fn is_general_coroutine (self , coroutine_def_id : Self :: CoroutineId) -> bool ; fn coroutine_is_async (self , coroutine_def_id : Self :: CoroutineId) -> bool ; fn coroutine_is_gen (self , coroutine_def_id : Self :: CoroutineId) -> bool ; fn coroutine_is_async_gen (self , coroutine_def_id : Self :: CoroutineId) -> bool ; type UnsizingParams : Deref < Target = DenseBitSet < u32 > > ; fn unsizing_params_for_adt (self , adt_def_id : Self :: AdtId) -> Self :: UnsizingParams ; fn anonymize_bound_vars < T : TypeFoldable < Self > > (self , binder : ty :: Binder < Self , T > ,) -> ty :: Binder < Self , T > ; fn opaque_types_defined_by (self , defining_anchor : Self :: LocalDefId) -> Self :: LocalDefIds ; fn opaque_types_and_coroutines_defined_by (self , defining_anchor : Self :: LocalDefId ,) -> Self :: LocalDefIds ; type Probe : Debug + Hash + Eq + Borrow < inspect :: Probe < Self > > ; fn mk_probe (self , probe : inspect :: Probe < Self >) -> Self :: Probe ; fn evaluate_root_goal_for_proof_tree_raw (self , canonical_goal : CanonicalInput < Self > ,) -> (QueryResult < Self > , Self :: Probe) ; }}}
mkitem!{mktrait!{#[doc = " Imagine you have a function `F: FnOnce(&[T]) -> R`, plus an iterator `iter`"] #[doc = " that produces `T` items. You could combine them with"] #[doc = " `f(&iter.collect::<Vec<_>>())`, but this requires allocating memory for the"] #[doc = " `Vec`."] #[doc = ""] #[doc = " This trait allows for faster implementations, intended for cases where the"] #[doc = " number of items produced by the iterator is small. There is a blanket impl"] #[doc = " for `T` items, but there is also a fallible impl for `Result<T, E>` items."] pub trait CollectAndApply < T , R > : Sized { type Output ; #[doc = " Produce a result of type `Self::Output` from `iter`. The result will"] #[doc = " typically be produced by applying `f` on the elements produced by"] #[doc = " `iter`, though this may not happen in some impls, e.g. if an error"] #[doc = " occurred during iteration."] fn collect_and_apply < I , F > (iter : I , f : F) -> Self :: Output where I : Iterator < Item = Self > , F : FnOnce (& [T]) -> R ; }}}
mkitem!{mkimpl!{#[doc = " The blanket impl that always collects all elements and applies `f`."] impl < T , R > CollectAndApply < T , R > for T { type Output = R ; #[doc = " Equivalent to `f(&iter.collect::<Vec<_>>())`."] fn collect_and_apply < I , F > (mut iter : I , f : F) -> R where I : Iterator < Item = T > , F : FnOnce (& [T]) -> R , { let Some (t0) = iter . next () else { return f (& []) ; } ; let Some (t1) = iter . next () else { return f (& [t0]) ; } ; let Some (t2) = iter . next () else { return f (& [t0 , t1]) ; } ; let Some (t3) = iter . next () else { return f (& [t0 , t1 , t2]) ; } ; let Some (t4) = iter . next () else { return f (& [t0 , t1 , t2 , t3]) ; } ; let Some (t5) = iter . next () else { return f (& [t0 , t1 , t2 , t3 , t4]) ; } ; let Some (t6) = iter . next () else { return f (& [t0 , t1 , t2 , t3 , t4 , t5]) ; } ; let Some (t7) = iter . next () else { return f (& [t0 , t1 , t2 , t3 , t4 , t5 , t6]) ; } ; let Some (t8) = iter . next () else { return f (& [t0 , t1 , t2 , t3 , t4 , t5 , t6 , t7]) ; } ; f (& [t0 , t1 , t2 , t3 , t4 , t5 , t6 , t7 , t8] . into_iter () . chain (iter) . collect :: < Vec < _ > > ()) } }}}
mkitem!{mkimpl!{#[doc = " A fallible impl that will fail, without calling `f`, if there are any"] #[doc = " errors during collection."] impl < T , R , E > CollectAndApply < T , R > for Result < T , E > { type Output = Result < R , E > ; #[doc = " Equivalent to `Ok(f(&iter.collect::<Result<Vec<_>>>()?))`."] fn collect_and_apply < I , F > (mut iter : I , f : F) -> Result < R , E > where I : Iterator < Item = Result < T , E > > , F : FnOnce (& [T]) -> R , { let Some (t0) = iter . next () else { return Ok (f (& [])) ; } ; let t0 = t0 ? ; let Some (t1) = iter . next () else { return Ok (f (& [t0])) ; } ; let t1 = t1 ? ; let Some (t2) = iter . next () else { return Ok (f (& [t0 , t1])) ; } ; let t2 = t2 ? ; let Some (t3) = iter . next () else { return Ok (f (& [t0 , t1 , t2])) ; } ; let t3 = t3 ? ; let Some (t4) = iter . next () else { return Ok (f (& [t0 , t1 , t2 , t3])) ; } ; let t4 = t4 ? ; let Some (t5) = iter . next () else { return Ok (f (& [t0 , t1 , t2 , t3 , t4])) ; } ; let t5 = t5 ? ; let Some (t6) = iter . next () else { return Ok (f (& [t0 , t1 , t2 , t3 , t4 , t5])) ; } ; let t6 = t6 ? ; let Some (t7) = iter . next () else { return Ok (f (& [t0 , t1 , t2 , t3 , t4 , t5 , t6])) ; } ; let t7 = t7 ? ; let Some (t8) = iter . next () else { return Ok (f (& [t0 , t1 , t2 , t3 , t4 , t5 , t6 , t7])) ; } ; let t8 = t8 ? ; Ok (f (& [Ok (t0) , Ok (t1) , Ok (t2) , Ok (t3) , Ok (t4) , Ok (t5) , Ok (t6) , Ok (t7) , Ok (t8)] . into_iter () . chain (iter) . collect :: < Result < Vec < _ > , _ > > () ?)) } }}}
mkitem!{mkimpl!{impl < I : Interner > search_graph :: Cx for I { type Input = CanonicalInput < I > ; type Result = QueryResult < I > ; type DepNodeIndex = I :: DepNodeIndex ; type Tracked < T : Debug + Clone > = I :: Tracked < T > ; fn mk_tracked < T : Debug + Clone > (self , data : T , dep_node_index : I :: DepNodeIndex ,) -> I :: Tracked < T > { I :: mk_tracked (self , data , dep_node_index) } fn get_tracked < T : Debug + Clone > (self , tracked : & I :: Tracked < T >) -> T { I :: get_tracked (self , tracked) } fn with_cached_task < T > (self , task : impl FnOnce () -> T) -> (T , I :: DepNodeIndex) { I :: with_cached_task (self , task) } fn with_global_cache < R > (self , f : impl FnOnce (& mut search_graph :: GlobalCache < Self >) -> R) -> R { I :: with_global_cache (self , f) } fn evaluation_is_concurrent (& self) -> bool { self . evaluation_is_concurrent () } }}}