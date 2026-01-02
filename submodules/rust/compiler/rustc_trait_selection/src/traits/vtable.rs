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
mkuse!{use std :: fmt :: Debug ;}
mkuse!{use std :: ops :: ControlFlow ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_infer :: traits :: util :: PredicateSet ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: { self , GenericArgs , GenericParamDefKind , Ty , TyCtxt , TypeVisitableExt , Upcast , VtblEntry , } ;}
mkuse!{use rustc_span :: DUMMY_SP ;}
mkuse!{use smallvec :: { SmallVec , smallvec } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: traits :: { impossible_predicates , is_vtable_safe_method } ;}
mkitem!{mkenum!{# [derive (Clone , Debug)] pub enum VtblSegment < 'tcx > { MetadataDSA , TraitOwnEntries { trait_ref : ty :: TraitRef < 'tcx > , emit_vptr : bool } , }}}

macro_rules! prepare_vtable_segments_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function prepare_vtable_segments in module {}", module_path!());
    };
}

mkfn!{
    prepare_vtable_segments_introspect!();
    # [doc = " Prepare the segments for a vtable"] pub fn prepare_vtable_segments < 'tcx , T > (tcx : TyCtxt < 'tcx > , trait_ref : ty :: TraitRef < 'tcx > , segment_visitor : impl FnMut (VtblSegment < 'tcx >) -> ControlFlow < T > ,) -> Option < T > { prepare_vtable_segments_inner (tcx , trait_ref , segment_visitor) . break_value () }
}

macro_rules! prepare_vtable_segments_inner_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function prepare_vtable_segments_inner in module {}", module_path!());
    };
}

mkfn!{
    prepare_vtable_segments_inner_introspect!();
    # [doc = " Helper for [`prepare_vtable_segments`] that returns `ControlFlow`,"] # [doc = " such that we can use `?` in the body."] fn prepare_vtable_segments_inner < 'tcx , T > (tcx : TyCtxt < 'tcx > , trait_ref : ty :: TraitRef < 'tcx > , mut segment_visitor : impl FnMut (VtblSegment < 'tcx >) -> ControlFlow < T > ,) -> ControlFlow < T > { segment_visitor (VtblSegment :: MetadataDSA) ? ; let mut emit_vptr_on_new_entry = false ; let mut visited = PredicateSet :: new (tcx) ; let predicate = trait_ref . upcast (tcx) ; let mut stack : SmallVec < [(ty :: TraitRef < 'tcx > , _ , _) ; 5] > = smallvec ! [(trait_ref , emit_vptr_on_new_entry , maybe_iter (None))] ; visited . insert (predicate) ; 'outer : loop { 'diving_in : loop { let & (inner_most_trait_ref , _ , _) = stack . last () . unwrap () ; let mut direct_super_traits_iter = tcx . explicit_super_predicates_of (inner_most_trait_ref . def_id) . iter_identity_copied () . filter_map (move | (pred , _) | { pred . instantiate_supertrait (tcx , ty :: Binder :: dummy (inner_most_trait_ref)) . as_trait_clause () }) . map (move | pred | { tcx . normalize_erasing_late_bound_regions (ty :: TypingEnv :: fully_monomorphized () , pred ,) . trait_ref }) ; match direct_super_traits_iter . find (| & super_trait | visited . insert (super_trait . upcast (tcx))) { Some (next_super_trait) => stack . push ((next_super_trait , emit_vptr_on_new_entry , maybe_iter (Some (direct_super_traits_iter)) ,)) , None => break 'diving_in , } } while let Some ((inner_most_trait_ref , emit_vptr , mut siblings)) = stack . pop () { let has_entries = ty :: elaborate :: supertrait_def_ids (tcx , inner_most_trait_ref . def_id) . any (| def_id | has_own_existential_vtable_entries (tcx , def_id)) ; segment_visitor (VtblSegment :: TraitOwnEntries { trait_ref : inner_most_trait_ref , emit_vptr : emit_vptr && has_entries && ! tcx . sess . opts . unstable_opts . no_trait_vptr , }) ? ; emit_vptr_on_new_entry |= has_entries ; if let Some (next_inner_most_trait_ref) = siblings . find (| & sibling | visited . insert (sibling . upcast (tcx))) { stack . push ((next_inner_most_trait_ref , emit_vptr_on_new_entry , siblings)) ; continue 'outer ; } } return ControlFlow :: Continue (()) ; } }
}

macro_rules! maybe_iter_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function maybe_iter in module {}", module_path!());
    };
}

mkfn!{
    maybe_iter_introspect!();
    # [doc = " Turns option of iterator into an iterator (this is just flatten)"] fn maybe_iter < I : Iterator > (i : Option < I >) -> impl Iterator < Item = I :: Item > { i . into_iter () . flatten () }
}

macro_rules! has_own_existential_vtable_entries_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function has_own_existential_vtable_entries in module {}", module_path!());
    };
}

mkfn!{
    has_own_existential_vtable_entries_introspect!();
    fn has_own_existential_vtable_entries (tcx : TyCtxt < '_ > , trait_def_id : DefId) -> bool { own_existential_vtable_entries_iter (tcx , trait_def_id) . next () . is_some () }
}

macro_rules! own_existential_vtable_entries_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function own_existential_vtable_entries in module {}", module_path!());
    };
}

mkfn!{
    own_existential_vtable_entries_introspect!();
    fn own_existential_vtable_entries (tcx : TyCtxt < '_ > , trait_def_id : DefId) -> & [DefId] { tcx . arena . alloc_from_iter (own_existential_vtable_entries_iter (tcx , trait_def_id)) }
}

macro_rules! own_existential_vtable_entries_iter_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function own_existential_vtable_entries_iter in module {}", module_path!());
    };
}

mkfn!{
    own_existential_vtable_entries_iter_introspect!();
    fn own_existential_vtable_entries_iter (tcx : TyCtxt < '_ > , trait_def_id : DefId ,) -> impl Iterator < Item = DefId > { let trait_methods = tcx . associated_items (trait_def_id) . in_definition_order () . filter (| item | item . is_fn ()) ; let own_entries = trait_methods . filter_map (move | & trait_method | { debug ! ("own_existential_vtable_entry: trait_method={:?}" , trait_method) ; let def_id = trait_method . def_id ; if ! is_vtable_safe_method (tcx , trait_def_id , trait_method) { debug ! ("own_existential_vtable_entry: not vtable safe") ; return None ; } Some (def_id) }) ; own_entries }
}

macro_rules! vtable_entries_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vtable_entries in module {}", module_path!());
    };
}

mkfn!{
    vtable_entries_introspect!();
    # [doc = " Given a trait `trait_ref`, iterates the vtable entries"] # [doc = " that come from `trait_ref`, including its supertraits."] fn vtable_entries < 'tcx > (tcx : TyCtxt < 'tcx > , trait_ref : ty :: TraitRef < 'tcx > ,) -> & 'tcx [VtblEntry < 'tcx >] { debug_assert ! (! trait_ref . has_non_region_infer () && ! trait_ref . has_non_region_param ()) ; debug_assert_eq ! (tcx . normalize_erasing_regions (ty :: TypingEnv :: fully_monomorphized () , trait_ref) , trait_ref , "vtable trait ref should be normalized") ; debug ! ("vtable_entries({:?})" , trait_ref) ; let mut entries = vec ! [] ; let vtable_segment_callback = | segment | -> ControlFlow < () > { match segment { VtblSegment :: MetadataDSA => { entries . extend (TyCtxt :: COMMON_VTABLE_ENTRIES) ; } VtblSegment :: TraitOwnEntries { trait_ref , emit_vptr } => { let existential_trait_ref = ty :: ExistentialTraitRef :: erase_self_ty (tcx , trait_ref) ; let own_existential_entries = tcx . own_existential_vtable_entries (existential_trait_ref . def_id) ; let own_entries = own_existential_entries . iter () . copied () . map (| def_id | { debug ! ("vtable_entries: trait_method={:?}" , def_id) ; let args = tcx . normalize_erasing_regions (ty :: TypingEnv :: fully_monomorphized () , GenericArgs :: for_item (tcx , def_id , | param , _ | match param . kind { GenericParamDefKind :: Lifetime => tcx . lifetimes . re_erased . into () , GenericParamDefKind :: Type { .. } | GenericParamDefKind :: Const { .. } => { trait_ref . args [param . index as usize] } }) ,) ; let predicates = tcx . predicates_of (def_id) . instantiate_own (tcx , args) ; if impossible_predicates (tcx , predicates . map (| (predicate , _) | predicate) . collect () ,) { debug ! ("vtable_entries: predicates do not hold") ; return VtblEntry :: Vacant ; } let instance = ty :: Instance :: expect_resolve_for_vtable (tcx , ty :: TypingEnv :: fully_monomorphized () , def_id , args , DUMMY_SP ,) ; VtblEntry :: Method (instance) }) ; entries . extend (own_entries) ; if emit_vptr { entries . push (VtblEntry :: TraitVPtr (trait_ref)) ; } } } ControlFlow :: Continue (()) } ; let _ = prepare_vtable_segments (tcx , trait_ref , vtable_segment_callback) ; tcx . arena . alloc_from_iter (entries) }
}

macro_rules! first_method_vtable_slot_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function first_method_vtable_slot in module {}", module_path!());
    };
}

mkfn!{
    first_method_vtable_slot_introspect!();
    pub (crate) fn first_method_vtable_slot < 'tcx > (tcx : TyCtxt < 'tcx > , key : ty :: TraitRef < 'tcx >) -> usize { debug_assert ! (! key . has_non_region_infer () && ! key . has_non_region_param ()) ; debug_assert_eq ! (tcx . normalize_erasing_regions (ty :: TypingEnv :: fully_monomorphized () , key) , key , "vtable trait ref should be normalized") ; let ty :: Dynamic (source , _ , _) = * key . self_ty () . kind () else { bug ! () ; } ; let source_principal = tcx . instantiate_bound_regions_with_erased (source . principal () . unwrap () . with_self_ty (tcx , key . self_ty ()) ,) ; if tcx . instantiate_and_check_impossible_predicates ((source_principal . def_id , source_principal . args ,)) { return 0 ; } let target_principal = ty :: ExistentialTraitRef :: erase_self_ty (tcx , key) ; let vtable_segment_callback = { let mut vptr_offset = 0 ; move | segment | { match segment { VtblSegment :: MetadataDSA => { vptr_offset += TyCtxt :: COMMON_VTABLE_ENTRIES . len () ; } VtblSegment :: TraitOwnEntries { trait_ref : vtable_principal , emit_vptr } => { if ty :: ExistentialTraitRef :: erase_self_ty (tcx , vtable_principal) == target_principal { return ControlFlow :: Break (vptr_offset) ; } vptr_offset += tcx . own_existential_vtable_entries (vtable_principal . def_id) . len () ; if emit_vptr { vptr_offset += 1 ; } } } ControlFlow :: Continue (()) } } ; prepare_vtable_segments (tcx , source_principal , vtable_segment_callback) . unwrap () }
}

macro_rules! supertrait_vtable_slot_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function supertrait_vtable_slot in module {}", module_path!());
    };
}

mkfn!{
    supertrait_vtable_slot_introspect!();
    # [doc = " Given a `dyn Subtrait` and `dyn Supertrait` trait object, find the slot of"] # [doc = " the trait vptr in the subtrait's vtable."] # [doc = ""] # [doc = " A return value of `None` means that the original vtable can be reused."] pub (crate) fn supertrait_vtable_slot < 'tcx > (tcx : TyCtxt < 'tcx > , key : (Ty < 'tcx > , Ty < 'tcx > ,) ,) -> Option < usize > { debug_assert ! (! key . has_non_region_infer () && ! key . has_non_region_param ()) ; debug_assert_eq ! (tcx . normalize_erasing_regions (ty :: TypingEnv :: fully_monomorphized () , key) , key , "upcasting trait refs should be normalized") ; let (source , target) = key ; let ty :: Dynamic (target_data , _ , _) = * target . kind () else { bug ! () ; } ; let target_principal = tcx . instantiate_bound_regions_with_erased (target_data . principal () ?) ; let ty :: Dynamic (source_data , _ , _) = * source . kind () else { bug ! () ; } ; let source_principal = tcx . instantiate_bound_regions_with_erased (source_data . principal () . unwrap () . with_self_ty (tcx , source) ,) ; if tcx . instantiate_and_check_impossible_predicates ((source_principal . def_id , source_principal . args ,)) { return None ; } let vtable_segment_callback = { let mut vptr_offset = 0 ; move | segment | { match segment { VtblSegment :: MetadataDSA => { vptr_offset += TyCtxt :: COMMON_VTABLE_ENTRIES . len () ; } VtblSegment :: TraitOwnEntries { trait_ref : vtable_principal , emit_vptr } => { vptr_offset += tcx . own_existential_vtable_entries (vtable_principal . def_id) . len () ; if ty :: ExistentialTraitRef :: erase_self_ty (tcx , vtable_principal) == target_principal { if emit_vptr { return ControlFlow :: Break (Some (vptr_offset)) ; } else { return ControlFlow :: Break (None) ; } } if emit_vptr { vptr_offset += 1 ; } } } ControlFlow :: Continue (()) } } ; prepare_vtable_segments (tcx , source_principal , vtable_segment_callback) . unwrap () }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (super) fn provide (providers : & mut Providers) { * providers = Providers { own_existential_vtable_entries , vtable_entries , first_method_vtable_slot , supertrait_vtable_slot , .. * providers } ; }
}