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
mkuse!{use std :: iter ;}
mkuse!{use rustc_abi :: { FIRST_VARIANT , VariantIdx } ;}
mkuse!{use rustc_errors :: ErrorGuaranteed ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_middle :: mir :: interpret :: LitToConstInput ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: thir :: visit ;}
mkuse!{use rustc_middle :: thir :: visit :: Visitor ;}
mkuse!{use rustc_middle :: ty :: abstract_const :: CastKind ;}
mkuse!{use rustc_middle :: ty :: { self , Expr , TyCtxt , TypeVisitableExt } ;}
mkuse!{use rustc_middle :: { bug , mir , thir } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use crate :: errors :: { GenericConstantTooComplex , GenericConstantTooComplexSub } ;}

macro_rules! destructure_const_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function destructure_const in module {}", module_path!());
    };
}

mkfn!{
    destructure_const_introspect!();
    #[doc = " Destructures array, ADT or tuple constants into the constants"] #[doc = " of their fields."] fn destructure_const < 'tcx > (tcx : TyCtxt < 'tcx > , const_ : ty :: Const < 'tcx > ,) -> ty :: DestructuredConst < 'tcx > { let ty :: ConstKind :: Value (cv) = const_ . kind () else { bug ! ("cannot destructure constant {:?}" , const_) } ; let branches = cv . valtree . unwrap_branch () ; let (fields , variant) = match cv . ty . kind () { ty :: Array (inner_ty , _) | ty :: Slice (inner_ty) => { let field_consts = branches . iter () . map (| b | ty :: Const :: new_value (tcx , * b , * inner_ty)) . collect :: < Vec < _ > > () ; debug ! (? field_consts) ; (field_consts , None) } ty :: Adt (def , _) if def . variants () . is_empty () => bug ! ("unreachable") , ty :: Adt (def , args) => { let (variant_idx , branches) = if def . is_enum () { let (head , rest) = branches . split_first () . unwrap () ; (VariantIdx :: from_u32 (head . unwrap_leaf () . to_u32 ()) , rest) } else { (FIRST_VARIANT , branches) } ; let fields = & def . variant (variant_idx) . fields ; let mut field_consts = Vec :: with_capacity (fields . len ()) ; for (field , field_valtree) in iter :: zip (fields , branches) { let field_ty = field . ty (tcx , args) ; let field_const = ty :: Const :: new_value (tcx , * field_valtree , field_ty) ; field_consts . push (field_const) ; } debug ! (? field_consts) ; (field_consts , Some (variant_idx)) } ty :: Tuple (elem_tys) => { let fields = iter :: zip (* elem_tys , branches) . map (| (elem_ty , elem_valtree) | ty :: Const :: new_value (tcx , * elem_valtree , elem_ty)) . collect :: < Vec < _ > > () ; (fields , None) } _ => bug ! ("cannot destructure constant {:?}" , const_) , } ; let fields = tcx . arena . alloc_from_iter (fields) ; ty :: DestructuredConst { variant , fields } }
}

macro_rules! check_binop_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_binop in module {}", module_path!());
    };
}

mkfn!{
    check_binop_introspect!();
    #[doc = " We do not allow all binary operations in abstract consts, so filter disallowed ones."] fn check_binop (op : mir :: BinOp) -> bool { use mir :: BinOp :: * ; match op { Add | AddUnchecked | AddWithOverflow | Sub | SubUnchecked | SubWithOverflow | Mul | MulUnchecked | MulWithOverflow | Div | Rem | BitXor | BitAnd | BitOr | Shl | ShlUnchecked | Shr | ShrUnchecked | Eq | Lt | Le | Ne | Ge | Gt | Cmp => true , Offset => false , } }
}

macro_rules! check_unop_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_unop in module {}", module_path!());
    };
}

mkfn!{
    check_unop_introspect!();
    #[doc = " While we currently allow all unary operations, we still want to explicitly guard against"] #[doc = " future changes here."] fn check_unop (op : mir :: UnOp) -> bool { use mir :: UnOp :: * ; match op { Not | Neg | PtrMetadata => true , } }
}

macro_rules! recurse_build_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function recurse_build in module {}", module_path!());
    };
}

mkfn!{
    recurse_build_introspect!();
    fn recurse_build < 'tcx > (tcx : TyCtxt < 'tcx > , body : & thir :: Thir < 'tcx > , node : thir :: ExprId , root_span : Span ,) -> Result < ty :: Const < 'tcx > , ErrorGuaranteed > { use thir :: ExprKind ; let node = & body . exprs [node] ; let maybe_supported_error = | a | maybe_supported_error (tcx , a , root_span) ; let error = | a | error (tcx , a , root_span) ; Ok (match & node . kind { & ExprKind :: Scope { value , .. } => recurse_build (tcx , body , value , root_span) ? , & ExprKind :: PlaceTypeAscription { source , .. } | & ExprKind :: ValueTypeAscription { source , .. } => { recurse_build (tcx , body , source , root_span) ? } & ExprKind :: PlaceUnwrapUnsafeBinder { .. } | & ExprKind :: ValueUnwrapUnsafeBinder { .. } | & ExprKind :: WrapUnsafeBinder { .. } => { todo ! ("FIXME(unsafe_binders)") } & ExprKind :: Literal { lit , neg } => { let sp = node . span ; tcx . at (sp) . lit_to_const (LitToConstInput { lit : lit . node , ty : node . ty , neg }) } & ExprKind :: NonHirLiteral { lit , user_ty : _ } => { let val = ty :: ValTree :: from_scalar_int (tcx , lit) ; ty :: Const :: new_value (tcx , val , node . ty) } & ExprKind :: ZstLiteral { user_ty : _ } => ty :: Const :: zero_sized (tcx , node . ty) , & ExprKind :: NamedConst { def_id , args , user_ty : _ } => { let uneval = ty :: UnevaluatedConst :: new (def_id , args) ; ty :: Const :: new_unevaluated (tcx , uneval) } ExprKind :: ConstParam { param , .. } => ty :: Const :: new_param (tcx , * param) , ExprKind :: Call { fun , args , .. } => { let fun_ty = body . exprs [* fun] . ty ; let fun = recurse_build (tcx , body , * fun , root_span) ? ; let mut new_args = Vec :: < ty :: Const < 'tcx > > :: with_capacity (args . len ()) ; for & id in args . iter () { new_args . push (recurse_build (tcx , body , id , root_span) ?) ; } ty :: Const :: new_expr (tcx , Expr :: new_call (tcx , fun_ty , fun , new_args)) } & ExprKind :: Binary { op , lhs , rhs } if check_binop (op) => { let lhs_ty = body . exprs [lhs] . ty ; let lhs = recurse_build (tcx , body , lhs , root_span) ? ; let rhs_ty = body . exprs [rhs] . ty ; let rhs = recurse_build (tcx , body , rhs , root_span) ? ; ty :: Const :: new_expr (tcx , Expr :: new_binop (tcx , op , lhs_ty , rhs_ty , lhs , rhs)) } & ExprKind :: Unary { op , arg } if check_unop (op) => { let arg_ty = body . exprs [arg] . ty ; let arg = recurse_build (tcx , body , arg , root_span) ? ; ty :: Const :: new_expr (tcx , Expr :: new_unop (tcx , op , arg_ty , arg)) } ExprKind :: Block { block } => { if let thir :: Block { stmts : box [] , expr : Some (e) , .. } = & body . blocks [* block] { recurse_build (tcx , body , * e , root_span) ? } else { maybe_supported_error (GenericConstantTooComplexSub :: BlockNotSupported (node . span)) ? } } & ExprKind :: Use { source } => { let value_ty = body . exprs [source] . ty ; let value = recurse_build (tcx , body , source , root_span) ? ; ty :: Const :: new_expr (tcx , Expr :: new_cast (tcx , CastKind :: Use , value_ty , value , node . ty)) } & ExprKind :: Cast { source } => { let value_ty = body . exprs [source] . ty ; let value = recurse_build (tcx , body , source , root_span) ? ; ty :: Const :: new_expr (tcx , Expr :: new_cast (tcx , CastKind :: As , value_ty , value , node . ty)) } ExprKind :: Borrow { arg , .. } => { let arg_node = & body . exprs [* arg] ; if let ExprKind :: Deref { arg } = arg_node . kind { recurse_build (tcx , body , arg , root_span) ? } else { maybe_supported_error (GenericConstantTooComplexSub :: BorrowNotSupported (node . span)) ? } } ExprKind :: RawBorrow { .. } | ExprKind :: Deref { .. } => maybe_supported_error (GenericConstantTooComplexSub :: AddressAndDerefNotSupported (node . span) ,) ? , ExprKind :: Repeat { .. } | ExprKind :: Array { .. } => { maybe_supported_error (GenericConstantTooComplexSub :: ArrayNotSupported (node . span)) ? } ExprKind :: NeverToAny { .. } => { maybe_supported_error (GenericConstantTooComplexSub :: NeverToAnyNotSupported (node . span)) ? } ExprKind :: Tuple { .. } => { maybe_supported_error (GenericConstantTooComplexSub :: TupleNotSupported (node . span)) ? } ExprKind :: Index { .. } => { maybe_supported_error (GenericConstantTooComplexSub :: IndexNotSupported (node . span)) ? } ExprKind :: Field { .. } => { maybe_supported_error (GenericConstantTooComplexSub :: FieldNotSupported (node . span)) ? } ExprKind :: ConstBlock { .. } => { maybe_supported_error (GenericConstantTooComplexSub :: ConstBlockNotSupported (node . span)) ? } ExprKind :: Adt (_) => { maybe_supported_error (GenericConstantTooComplexSub :: AdtNotSupported (node . span)) ? } ExprKind :: PointerCoercion { .. } => { error (GenericConstantTooComplexSub :: PointerNotSupported (node . span)) ? } ExprKind :: Yield { .. } => { error (GenericConstantTooComplexSub :: YieldNotSupported (node . span)) ? } ExprKind :: Continue { .. } | ExprKind :: ConstContinue { .. } | ExprKind :: Break { .. } | ExprKind :: Loop { .. } | ExprKind :: LoopMatch { .. } => { error (GenericConstantTooComplexSub :: LoopNotSupported (node . span)) ? } ExprKind :: Box { .. } => error (GenericConstantTooComplexSub :: BoxNotSupported (node . span)) ? , ExprKind :: ByUse { .. } => { error (GenericConstantTooComplexSub :: ByUseNotSupported (node . span)) ? } ExprKind :: Unary { .. } => unreachable ! () , ExprKind :: Binary { .. } => { error (GenericConstantTooComplexSub :: BinaryNotSupported (node . span)) ? } ExprKind :: LogicalOp { .. } => { error (GenericConstantTooComplexSub :: LogicalOpNotSupported (node . span)) ? } ExprKind :: Assign { .. } | ExprKind :: AssignOp { .. } => { error (GenericConstantTooComplexSub :: AssignNotSupported (node . span)) ? } ExprKind :: Closure { .. } | ExprKind :: Return { .. } | ExprKind :: Become { .. } => { error (GenericConstantTooComplexSub :: ClosureAndReturnNotSupported (node . span)) ? } ExprKind :: Match { .. } | ExprKind :: If { .. } | ExprKind :: Let { .. } => { error (GenericConstantTooComplexSub :: ControlFlowNotSupported (node . span)) ? } ExprKind :: InlineAsm { .. } => { error (GenericConstantTooComplexSub :: InlineAsmNotSupported (node . span)) ? } ExprKind :: VarRef { .. } | ExprKind :: UpvarRef { .. } | ExprKind :: StaticRef { .. } | ExprKind :: OffsetOf { .. } | ExprKind :: ThreadLocalRef (_) => { error (GenericConstantTooComplexSub :: OperationNotSupported (node . span)) ? } }) }
}
mkitem!{mkstruct!{struct IsThirPolymorphic < 'a , 'tcx > { is_poly : bool , thir : & 'a thir :: Thir < 'tcx > , }}}

macro_rules! error_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function error in module {}", module_path!());
    };
}

mkfn!{
    error_introspect!();
    fn error (tcx : TyCtxt < '_ > , sub : GenericConstantTooComplexSub , root_span : Span ,) -> Result < ! , ErrorGuaranteed > { let reported = tcx . dcx () . emit_err (GenericConstantTooComplex { span : root_span , maybe_supported : false , sub , }) ; Err (reported) }
}

macro_rules! maybe_supported_error_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function maybe_supported_error in module {}", module_path!());
    };
}

mkfn!{
    maybe_supported_error_introspect!();
    fn maybe_supported_error (tcx : TyCtxt < '_ > , sub : GenericConstantTooComplexSub , root_span : Span ,) -> Result < ! , ErrorGuaranteed > { let reported = tcx . dcx () . emit_err (GenericConstantTooComplex { span : root_span , maybe_supported : true , sub , }) ; Err (reported) }
}
mkitem!{mkimpl!{impl < 'a , 'tcx > IsThirPolymorphic < 'a , 'tcx > { fn expr_is_poly (& mut self , expr : & thir :: Expr < 'tcx >) -> bool { if expr . ty . has_non_region_param () { return true ; } match expr . kind { thir :: ExprKind :: NamedConst { args , .. } | thir :: ExprKind :: ConstBlock { args , .. } => { args . has_non_region_param () } thir :: ExprKind :: ConstParam { .. } => true , thir :: ExprKind :: Repeat { value , count } => { self . visit_expr (& self . thir () [value]) ; count . has_non_region_param () } thir :: ExprKind :: Scope { .. } | thir :: ExprKind :: Box { .. } | thir :: ExprKind :: If { .. } | thir :: ExprKind :: Call { .. } | thir :: ExprKind :: ByUse { .. } | thir :: ExprKind :: Deref { .. } | thir :: ExprKind :: Binary { .. } | thir :: ExprKind :: LogicalOp { .. } | thir :: ExprKind :: Unary { .. } | thir :: ExprKind :: Cast { .. } | thir :: ExprKind :: Use { .. } | thir :: ExprKind :: NeverToAny { .. } | thir :: ExprKind :: PointerCoercion { .. } | thir :: ExprKind :: Loop { .. } | thir :: ExprKind :: LoopMatch { .. } | thir :: ExprKind :: Let { .. } | thir :: ExprKind :: Match { .. } | thir :: ExprKind :: Block { .. } | thir :: ExprKind :: Assign { .. } | thir :: ExprKind :: AssignOp { .. } | thir :: ExprKind :: Field { .. } | thir :: ExprKind :: Index { .. } | thir :: ExprKind :: VarRef { .. } | thir :: ExprKind :: UpvarRef { .. } | thir :: ExprKind :: Borrow { .. } | thir :: ExprKind :: RawBorrow { .. } | thir :: ExprKind :: Break { .. } | thir :: ExprKind :: Continue { .. } | thir :: ExprKind :: ConstContinue { .. } | thir :: ExprKind :: Return { .. } | thir :: ExprKind :: Become { .. } | thir :: ExprKind :: Array { .. } | thir :: ExprKind :: Tuple { .. } | thir :: ExprKind :: Adt (_) | thir :: ExprKind :: PlaceTypeAscription { .. } | thir :: ExprKind :: ValueTypeAscription { .. } | thir :: ExprKind :: PlaceUnwrapUnsafeBinder { .. } | thir :: ExprKind :: ValueUnwrapUnsafeBinder { .. } | thir :: ExprKind :: WrapUnsafeBinder { .. } | thir :: ExprKind :: Closure (_) | thir :: ExprKind :: Literal { .. } | thir :: ExprKind :: NonHirLiteral { .. } | thir :: ExprKind :: ZstLiteral { .. } | thir :: ExprKind :: StaticRef { .. } | thir :: ExprKind :: InlineAsm (_) | thir :: ExprKind :: OffsetOf { .. } | thir :: ExprKind :: ThreadLocalRef (_) | thir :: ExprKind :: Yield { .. } => false , } } fn pat_is_poly (& mut self , pat : & thir :: Pat < 'tcx >) -> bool { if pat . ty . has_non_region_param () { return true ; } match pat . kind { thir :: PatKind :: Constant { value } => value . has_non_region_param () , thir :: PatKind :: Range (ref range) => { let & thir :: PatRange { lo , hi , .. } = range . as_ref () ; lo . has_non_region_param () || hi . has_non_region_param () } _ => false , } } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > visit :: Visitor < 'a , 'tcx > for IsThirPolymorphic < 'a , 'tcx > { fn thir (& self) -> & 'a thir :: Thir < 'tcx > { self . thir } #[instrument (skip (self) , level = "debug")] fn visit_expr (& mut self , expr : & 'a thir :: Expr < 'tcx >) { self . is_poly |= self . expr_is_poly (expr) ; if ! self . is_poly { visit :: walk_expr (self , expr) } } #[instrument (skip (self) , level = "debug")] fn visit_pat (& mut self , pat : & 'a thir :: Pat < 'tcx >) { self . is_poly |= self . pat_is_poly (pat) ; if ! self . is_poly { visit :: walk_pat (self , pat) ; } } }}}

macro_rules! thir_abstract_const_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function thir_abstract_const in module {}", module_path!());
    };
}

mkfn!{
    thir_abstract_const_introspect!();
    #[doc = " Builds an abstract const, do not use this directly, but use `AbstractConst::new` instead."] fn thir_abstract_const < 'tcx > (tcx : TyCtxt < 'tcx > , def : LocalDefId ,) -> Result < Option < ty :: EarlyBinder < 'tcx , ty :: Const < 'tcx > > > , ErrorGuaranteed > { if ! tcx . features () . generic_const_exprs () { return Ok (None) ; } match tcx . def_kind (def) { DefKind :: AnonConst | DefKind :: InlineConst => () , _ => return Ok (None) , } let body = tcx . thir_body (def) ? ; let (body , body_id) = (& * body . 0 . borrow () , body . 1) ; let mut is_poly_vis = IsThirPolymorphic { is_poly : false , thir : body } ; visit :: walk_expr (& mut is_poly_vis , & body [body_id]) ; if ! is_poly_vis . is_poly { return Ok (None) ; } let root_span = body . exprs [body_id] . span ; Ok (Some (ty :: EarlyBinder :: bind (recurse_build (tcx , body , body_id , root_span) ?))) }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { destructure_const , thir_abstract_const , .. * providers } ; }
}