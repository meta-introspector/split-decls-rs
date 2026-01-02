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
mkuse!{use std :: fmt :: Write ;}
mkuse!{use hir :: def_id :: DefId ;}
mkuse!{use hir :: { HirId , ItemKind } ;}
mkuse!{use rustc_ast :: join_path_idents ;}
mkuse!{use rustc_errors :: Applicability ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_lint :: { ARRAY_INTO_ITER , BOXED_SLICE_INTO_ITER } ;}
mkuse!{use rustc_middle :: span_bug ;}
mkuse!{use rustc_middle :: ty :: { self , Ty } ;}
mkuse!{use rustc_session :: lint :: builtin :: { RUST_2021_PRELUDE_COLLISIONS , RUST_2024_PRELUDE_COLLISIONS } ;}
mkuse!{use rustc_span :: { Ident , STDLIB_STABLE_CRATES , Span , kw , sym } ;}
mkuse!{use rustc_trait_selection :: infer :: InferCtxtExt ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: FnCtxt ;}
mkuse!{use crate :: method :: probe :: { self , Pick } ;}
mkitem!{mkimpl!{impl < 'a , 'tcx > FnCtxt < 'a , 'tcx > { pub (super) fn lint_edition_dependent_dot_call (& self , self_ty : Ty < 'tcx > , segment : & hir :: PathSegment < '_ > , span : Span , call_expr : & 'tcx hir :: Expr < 'tcx > , self_expr : & 'tcx hir :: Expr < 'tcx > , pick : & Pick < 'tcx > , args : & 'tcx [hir :: Expr < 'tcx >] ,) { debug ! ("lookup(method_name={}, self_ty={:?}, call_expr={:?}, self_expr={:?})" , segment . ident , self_ty , call_expr , self_expr) ; let (prelude_or_array_lint , edition) = match segment . ident . name { sym :: try_into if ! span . at_least_rust_2021 () => (RUST_2021_PRELUDE_COLLISIONS , "2021") , sym :: poll if ! span . at_least_rust_2024 () && let ty :: Adt (adt_def , args) = self_ty . kind () && self . tcx . is_lang_item (adt_def . did () , hir :: LangItem :: Pin) && let ty :: Ref (_ , _ , ty :: Mutability :: Mut) = args [0] . as_type () . unwrap () . kind () => { (RUST_2024_PRELUDE_COLLISIONS , "2024") } sym :: into_future if ! span . at_least_rust_2024 () => { (RUST_2024_PRELUDE_COLLISIONS , "2024") } sym :: into_iter => { if let ty :: Array (..) = self_ty . kind () && ! span . at_least_rust_2021 () { (ARRAY_INTO_ITER , "2021") } else if self_ty . boxed_ty () . is_some_and (Ty :: is_slice) && ! span . at_least_rust_2024 () { (BOXED_SLICE_INTO_ITER , "2024") } else { return ; } } _ => return , } ; if STDLIB_STABLE_CRATES . contains (& self . tcx . crate_name (pick . item . def_id . krate)) { return ; } if matches ! (pick . kind , probe :: PickKind :: InherentImplPick | probe :: PickKind :: ObjectPick) { if pick . autoderefs == 1 && matches ! (pick . autoref_or_ptr_adjustment , Some (probe :: AutorefOrPtrAdjustment :: Autoref { .. })) && matches ! (self_ty . kind () , ty :: Ref (..)) { return ; } if pick . autoderefs == 0 && pick . autoref_or_ptr_adjustment . is_none () { return ; } self . tcx . node_span_lint (prelude_or_array_lint , self_expr . hir_id , self_expr . span , | lint | { lint . primary_message (format ! ("trait method `{}` will become ambiguous in Rust {edition}" , segment . ident . name)) ; let sp = self_expr . span ; let derefs = "*" . repeat (pick . autoderefs) ; let autoref = match pick . autoref_or_ptr_adjustment { Some (probe :: AutorefOrPtrAdjustment :: Autoref { mutbl , .. }) => { mutbl . ref_prefix_str () } Some (probe :: AutorefOrPtrAdjustment :: ToConstPtr) | None => "" , Some (probe :: AutorefOrPtrAdjustment :: ReborrowPin (mutbl)) => match mutbl { hir :: Mutability :: Mut => "Pin<&mut " , hir :: Mutability :: Not => "Pin<&" , } , } ; if let Ok (self_expr) = self . sess () . source_map () . span_to_snippet (self_expr . span) { let mut self_adjusted = if let Some (probe :: AutorefOrPtrAdjustment :: ToConstPtr) = pick . autoref_or_ptr_adjustment { format ! ("{derefs}{self_expr} as *const _") } else { format ! ("{autoref}{derefs}{self_expr}") } ; if let Some (probe :: AutorefOrPtrAdjustment :: ReborrowPin (_)) = pick . autoref_or_ptr_adjustment { self_adjusted . push ('>') ; } lint . span_suggestion (sp , "disambiguate the method call" , format ! ("({self_adjusted})") , Applicability :: MachineApplicable ,) ; } else { let self_adjusted = if let Some (probe :: AutorefOrPtrAdjustment :: ToConstPtr) = pick . autoref_or_ptr_adjustment { format ! ("{derefs}(...) as *const _") } else { format ! ("{autoref}{derefs}...") } ; lint . span_help (sp , format ! ("disambiguate the method call with `({self_adjusted})`" ,) ,) ; } } ,) ; } else { self . tcx . node_span_lint (prelude_or_array_lint , call_expr . hir_id , call_expr . span , | lint | { lint . primary_message (format ! ("trait method `{}` will become ambiguous in Rust {edition}" , segment . ident . name)) ; let sp = call_expr . span ; let trait_name = self . trait_path_or_bare_name (span , call_expr . hir_id , pick . item . container_id (self . tcx) ,) ; let (self_adjusted , precise) = self . adjust_expr (pick , self_expr , sp) ; if precise { let args = args . iter () . fold (String :: new () , | mut string , arg | { let span = arg . span . find_ancestor_inside (sp) . unwrap_or_default () ; write ! (string , ", {}" , self . sess () . source_map () . span_to_snippet (span) . unwrap ()) . unwrap () ; string }) ; lint . span_suggestion (sp , "disambiguate the associated function" , format ! ("{}::{}{}({}{})" , trait_name , segment . ident . name , if let Some (args) = segment . args . as_ref () . and_then (| args | self . sess () . source_map () . span_to_snippet (args . span_ext) . ok ()) { format ! ("::{args}") } else { String :: new () } , self_adjusted , args ,) , Applicability :: MachineApplicable ,) ; } else { lint . span_help (sp , format ! ("disambiguate the associated function with `{}::{}(...)`" , trait_name , segment . ident ,) ,) ; } } ,) ; } } pub (super) fn lint_fully_qualified_call_from_2018 (& self , span : Span , method_name : Ident , self_ty : Ty < 'tcx > , self_ty_span : Span , expr_id : hir :: HirId , pick : & Pick < 'tcx > ,) { if span . at_least_rust_2021 () { return ; } if ! matches ! (method_name . name , sym :: try_into | sym :: try_from | sym :: from_iter) { return ; } if STDLIB_STABLE_CRATES . contains (& self . tcx . crate_name (pick . item . def_id . krate)) { return ; } if method_name . name == sym :: from_iter { if let Some (trait_def_id) = self . tcx . get_diagnostic_item (sym :: FromIterator) { let any_type = self . infcx . next_ty_var (span) ; if ! self . infcx . type_implements_trait (trait_def_id , [self_ty , any_type] , self . param_env) . may_apply () { return ; } } } if matches ! (pick . kind , probe :: PickKind :: InherentImplPick) { return ; } self . tcx . node_span_lint (RUST_2021_PRELUDE_COLLISIONS , expr_id , span , | lint | { lint . primary_message (format ! ("trait-associated function `{}` will become ambiguous in Rust 2021" , method_name . name)) ; let container_id = pick . item . container_id (self . tcx) ; let trait_path = self . trait_path_or_bare_name (span , expr_id , container_id) ; let trait_generics = self . tcx . generics_of (container_id) ; let trait_name = if trait_generics . own_params . len () <= trait_generics . has_self as usize { trait_path } else { let counts = trait_generics . own_counts () ; format ! ("{}<{}>" , trait_path , std :: iter :: repeat ("'_") . take (counts . lifetimes) . chain (std :: iter :: repeat ("_") . take (counts . types + counts . consts - trait_generics . has_self as usize)) . collect ::< Vec < _ >> () . join (", ")) } ; let mut self_ty_name = self_ty_span . find_ancestor_inside (span) . and_then (| span | self . sess () . source_map () . span_to_snippet (span) . ok ()) . unwrap_or_else (| | self_ty . to_string ()) ; if ! self_ty_name . contains ('<') { if let ty :: Adt (def , _) = self_ty . kind () { let generics = self . tcx . generics_of (def . did ()) ; if ! generics . is_own_empty () { let counts = generics . own_counts () ; self_ty_name += & format ! ("<{}>" , std :: iter :: repeat ("'_") . take (counts . lifetimes) . chain (std :: iter :: repeat ("_") . take (counts . types + counts . consts)) . collect ::< Vec < _ >> () . join (", ")) ; } } } lint . span_suggestion (span , "disambiguate the associated function" , format ! ("<{} as {}>::{}" , self_ty_name , trait_name , method_name . name ,) , Applicability :: MachineApplicable ,) ; }) ; } fn trait_path_or_bare_name (& self , span : Span , expr_hir_id : HirId , trait_def_id : DefId ,) -> String { self . trait_path (span , expr_hir_id , trait_def_id) . unwrap_or_else (| | { let key = self . tcx . def_key (trait_def_id) ; format ! ("{}" , key . disambiguated_data . data) }) } fn trait_path (& self , span : Span , expr_hir_id : HirId , trait_def_id : DefId) -> Option < String > { let applicable_traits = self . tcx . in_scope_traits (expr_hir_id) ? ; let applicable_trait = applicable_traits . iter () . find (| t | t . def_id == trait_def_id) ? ; if applicable_trait . import_ids . is_empty () { return None ; } let import_items : Vec < _ > = applicable_trait . import_ids . iter () . map (| & import_id | self . tcx . hir_expect_item (import_id)) . collect () ; for item in import_items . iter () { let (_ , kind) = item . expect_use () ; match kind { hir :: UseKind :: Single (ident) => { if ident . name != kw :: Underscore { return Some (format ! ("{}" , ident . name)) ; } } hir :: UseKind :: Glob => return None , hir :: UseKind :: ListStem => unreachable ! () , } } match import_items [0] . kind { ItemKind :: Use (path , _) => { Some (join_path_idents (path . segments . iter () . map (| seg | seg . ident))) } _ => { span_bug ! (span , "unexpected item kind, expected a use: {:?}" , import_items [0] . kind) ; } } } # [doc = " Creates a string version of the `expr` that includes explicit adjustments."] # [doc = " Returns the string and also a bool indicating whether this is a *precise*"] # [doc = " suggestion."] fn adjust_expr (& self , pick : & Pick < 'tcx > , expr : & hir :: Expr < 'tcx > , outer : Span ,) -> (String , bool) { let derefs = "*" . repeat (pick . autoderefs) ; let autoref = match pick . autoref_or_ptr_adjustment { Some (probe :: AutorefOrPtrAdjustment :: Autoref { mutbl , .. }) => mutbl . ref_prefix_str () , Some (probe :: AutorefOrPtrAdjustment :: ToConstPtr) | None => "" , Some (probe :: AutorefOrPtrAdjustment :: ReborrowPin (mutbl)) => match mutbl { hir :: Mutability :: Mut => "Pin<&mut " , hir :: Mutability :: Not => "Pin<&" , } , } ; let (expr_text , precise) = if let Some (expr_text) = expr . span . find_ancestor_inside (outer) . and_then (| span | self . sess () . source_map () . span_to_snippet (span) . ok ()) { (expr_text , true) } else { ("(..)" . to_string () , false) } ; let mut adjusted_text = if let Some (probe :: AutorefOrPtrAdjustment :: ToConstPtr) = pick . autoref_or_ptr_adjustment { format ! ("{derefs}{expr_text} as *const _") } else { format ! ("{autoref}{derefs}{expr_text}") } ; if let Some (probe :: AutorefOrPtrAdjustment :: ReborrowPin (_)) = pick . autoref_or_ptr_adjustment { adjusted_text . push ('>') ; } (adjusted_text , precise) } }}}