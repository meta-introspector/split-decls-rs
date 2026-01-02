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
mkuse!{use either :: Either ;}
mkuse!{use rustc_abi :: Size ;}
mkuse!{use rustc_apfloat :: { Float , FloatConvert } ;}
mkuse!{use rustc_middle :: mir :: NullOp ;}
mkuse!{use rustc_middle :: mir :: interpret :: { InterpResult , PointerArithmetic , Scalar } ;}
mkuse!{use rustc_middle :: ty :: layout :: TyAndLayout ;}
mkuse!{use rustc_middle :: ty :: { self , FloatTy , ScalarInt , Ty } ;}
mkuse!{use rustc_middle :: { bug , mir , span_bug } ;}
mkuse!{use rustc_span :: sym ;}
mkuse!{use tracing :: trace ;}
mkuse!{use super :: { ImmTy , InterpCx , Machine , MemPlaceMeta , interp_ok , throw_ub } ;}
mkitem!{mkimpl!{impl < 'tcx , M : Machine < 'tcx > > InterpCx < 'tcx , M > { fn three_way_compare < T : Ord > (& self , lhs : T , rhs : T) -> ImmTy < 'tcx , M :: Provenance > { let res = Ord :: cmp (& lhs , & rhs) ; return ImmTy :: from_ordering (res , * self . tcx) ; } fn binary_char_op (& self , bin_op : mir :: BinOp , l : char , r : char) -> ImmTy < 'tcx , M :: Provenance > { use rustc_middle :: mir :: BinOp :: * ; if bin_op == Cmp { return self . three_way_compare (l , r) ; } let res = match bin_op { Eq => l == r , Ne => l != r , Lt => l < r , Le => l <= r , Gt => l > r , Ge => l >= r , _ => span_bug ! (self . cur_span () , "Invalid operation on char: {:?}" , bin_op) , } ; ImmTy :: from_bool (res , * self . tcx) } fn binary_bool_op (& self , bin_op : mir :: BinOp , l : bool , r : bool) -> ImmTy < 'tcx , M :: Provenance > { use rustc_middle :: mir :: BinOp :: * ; let res = match bin_op { Eq => l == r , Ne => l != r , Lt => l < r , Le => l <= r , Gt => l > r , Ge => l >= r , BitAnd => l & r , BitOr => l | r , BitXor => l ^ r , _ => span_bug ! (self . cur_span () , "Invalid operation on bool: {:?}" , bin_op) , } ; ImmTy :: from_bool (res , * self . tcx) } fn binary_float_op < F : Float + FloatConvert < F > + Into < Scalar < M :: Provenance > > > (& self , bin_op : mir :: BinOp , layout : TyAndLayout < 'tcx > , l : F , r : F ,) -> ImmTy < 'tcx , M :: Provenance > { use rustc_middle :: mir :: BinOp :: * ; let adjust_nan = | f : F | -> F { self . adjust_nan (f , & [l , r]) } ; match bin_op { Eq => ImmTy :: from_bool (l == r , * self . tcx) , Ne => ImmTy :: from_bool (l != r , * self . tcx) , Lt => ImmTy :: from_bool (l < r , * self . tcx) , Le => ImmTy :: from_bool (l <= r , * self . tcx) , Gt => ImmTy :: from_bool (l > r , * self . tcx) , Ge => ImmTy :: from_bool (l >= r , * self . tcx) , Add => ImmTy :: from_scalar (adjust_nan ((l + r) . value) . into () , layout) , Sub => ImmTy :: from_scalar (adjust_nan ((l - r) . value) . into () , layout) , Mul => ImmTy :: from_scalar (adjust_nan ((l * r) . value) . into () , layout) , Div => ImmTy :: from_scalar (adjust_nan ((l / r) . value) . into () , layout) , Rem => ImmTy :: from_scalar (adjust_nan ((l % r) . value) . into () , layout) , _ => span_bug ! (self . cur_span () , "invalid float op: `{:?}`" , bin_op) , } } fn binary_int_op (& self , bin_op : mir :: BinOp , left : & ImmTy < 'tcx , M :: Provenance > , right : & ImmTy < 'tcx , M :: Provenance > ,) -> InterpResult < 'tcx , ImmTy < 'tcx , M :: Provenance > > { use rustc_middle :: mir :: BinOp :: * ; let l = left . to_scalar_int () ? ; let r = right . to_scalar_int () ? ; let l_signed = | | l . to_int (left . layout . size) ; let l_unsigned = | | l . to_uint (left . layout . size) ; let r_signed = | | r . to_int (right . layout . size) ; let r_unsigned = | | r . to_uint (right . layout . size) ; let throw_ub_on_overflow = match bin_op { AddUnchecked => Some (sym :: unchecked_add) , SubUnchecked => Some (sym :: unchecked_sub) , MulUnchecked => Some (sym :: unchecked_mul) , ShlUnchecked => Some (sym :: unchecked_shl) , ShrUnchecked => Some (sym :: unchecked_shr) , _ => None , } ; let with_overflow = bin_op . is_overflowing () ; if matches ! (bin_op , Shl | ShlUnchecked | Shr | ShrUnchecked) { let l_bits = left . layout . size . bits () ; let (shift_amount , overflow) = if right . layout . backend_repr . is_signed () { let shift_amount = r_signed () ; let rem = shift_amount . rem_euclid (l_bits . into ()) ; (u128 :: try_from (rem) . unwrap () , rem != shift_amount) } else { let shift_amount = r_unsigned () ; let rem = shift_amount . rem_euclid (l_bits . into ()) ; (rem , rem != shift_amount) } ; let shift_amount = u32 :: try_from (shift_amount) . unwrap () ; let result = if left . layout . backend_repr . is_signed () { let l = l_signed () ; let result = match bin_op { Shl | ShlUnchecked => l . checked_shl (shift_amount) . unwrap () , Shr | ShrUnchecked => l . checked_shr (shift_amount) . unwrap () , _ => bug ! () , } ; ScalarInt :: truncate_from_int (result , left . layout . size) . 0 } else { let l = l_unsigned () ; let result = match bin_op { Shl | ShlUnchecked => l . checked_shl (shift_amount) . unwrap () , Shr | ShrUnchecked => l . checked_shr (shift_amount) . unwrap () , _ => bug ! () , } ; ScalarInt :: truncate_from_uint (result , left . layout . size) . 0 } ; if overflow && let Some (intrinsic) = throw_ub_on_overflow { throw_ub ! (ShiftOverflow { intrinsic , shift_amount : if right . layout . backend_repr . is_signed () { Either :: Right (r_signed ()) } else { Either :: Left (r_unsigned ()) } }) ; } return interp_ok (ImmTy :: from_scalar_int (result , left . layout)) ; } if left . layout . ty != right . layout . ty { span_bug ! (self . cur_span () , "invalid asymmetric binary op {bin_op:?}: {l:?} ({l_ty}), {r:?} ({r_ty})" , l_ty = left . layout . ty , r_ty = right . layout . ty ,) } let size = left . layout . size ; if left . layout . backend_repr . is_signed () { let op : Option < fn (& i128 , & i128) -> bool > = match bin_op { Lt => Some (i128 :: lt) , Le => Some (i128 :: le) , Gt => Some (i128 :: gt) , Ge => Some (i128 :: ge) , _ => None , } ; if let Some (op) = op { return interp_ok (ImmTy :: from_bool (op (& l_signed () , & r_signed ()) , * self . tcx)) ; } if bin_op == Cmp { return interp_ok (self . three_way_compare (l_signed () , r_signed ())) ; } let op : Option < fn (i128 , i128) -> (i128 , bool) > = match bin_op { Div if r . is_null () => throw_ub ! (DivisionByZero) , Rem if r . is_null () => throw_ub ! (RemainderByZero) , Div => Some (i128 :: overflowing_div) , Rem => Some (i128 :: overflowing_rem) , Add | AddUnchecked | AddWithOverflow => Some (i128 :: overflowing_add) , Sub | SubUnchecked | SubWithOverflow => Some (i128 :: overflowing_sub) , Mul | MulUnchecked | MulWithOverflow => Some (i128 :: overflowing_mul) , _ => None , } ; if let Some (op) = op { let l = l_signed () ; let r = r_signed () ; if matches ! (bin_op , Rem | Div) { if l == size . signed_int_min () && r == - 1 { if bin_op == Rem { throw_ub ! (RemainderOverflow) } else { throw_ub ! (DivisionOverflow) } } } let (result , oflo) = op (l , r) ; let (result , lossy) = ScalarInt :: truncate_from_int (result , left . layout . size) ; let overflow = oflo || lossy ; if overflow && let Some (intrinsic) = throw_ub_on_overflow { throw_ub ! (ArithOverflow { intrinsic }) ; } let res = ImmTy :: from_scalar_int (result , left . layout) ; return interp_ok (if with_overflow { let overflow = ImmTy :: from_bool (overflow , * self . tcx) ; ImmTy :: from_pair (res , overflow , self) } else { res }) ; } } let l = l_unsigned () ; let r = r_unsigned () ; if bin_op == Cmp { return interp_ok (self . three_way_compare (l , r)) ; } interp_ok (match bin_op { Eq => ImmTy :: from_bool (l == r , * self . tcx) , Ne => ImmTy :: from_bool (l != r , * self . tcx) , Lt => ImmTy :: from_bool (l < r , * self . tcx) , Le => ImmTy :: from_bool (l <= r , * self . tcx) , Gt => ImmTy :: from_bool (l > r , * self . tcx) , Ge => ImmTy :: from_bool (l >= r , * self . tcx) , BitOr => ImmTy :: from_uint (l | r , left . layout) , BitAnd => ImmTy :: from_uint (l & r , left . layout) , BitXor => ImmTy :: from_uint (l ^ r , left . layout) , _ => { assert ! (! left . layout . backend_repr . is_signed ()) ; let op : fn (u128 , u128) -> (u128 , bool) = match bin_op { Add | AddUnchecked | AddWithOverflow => u128 :: overflowing_add , Sub | SubUnchecked | SubWithOverflow => u128 :: overflowing_sub , Mul | MulUnchecked | MulWithOverflow => u128 :: overflowing_mul , Div if r == 0 => throw_ub ! (DivisionByZero) , Rem if r == 0 => throw_ub ! (RemainderByZero) , Div => u128 :: overflowing_div , Rem => u128 :: overflowing_rem , _ => span_bug ! (self . cur_span () , "invalid binary op {:?}: {:?}, {:?} (both {})" , bin_op , left , right , right . layout . ty ,) , } ; let (result , oflo) = op (l , r) ; let (result , lossy) = ScalarInt :: truncate_from_uint (result , left . layout . size) ; let overflow = oflo || lossy ; if overflow && let Some (intrinsic) = throw_ub_on_overflow { throw_ub ! (ArithOverflow { intrinsic }) ; } let res = ImmTy :: from_scalar_int (result , left . layout) ; if with_overflow { let overflow = ImmTy :: from_bool (overflow , * self . tcx) ; ImmTy :: from_pair (res , overflow , self) } else { res } } }) } # [doc = " Computes the total size of this access, `count * elem_size`,"] # [doc = " checking for overflow beyond isize::MAX."] pub fn compute_size_in_bytes (& self , elem_size : Size , count : u64) -> Option < Size > { elem_size . bytes () . checked_mul (count) . map (Size :: from_bytes) . filter (| & total | total <= self . max_size_of_val ()) } fn binary_ptr_op (& self , bin_op : mir :: BinOp , left : & ImmTy < 'tcx , M :: Provenance > , right : & ImmTy < 'tcx , M :: Provenance > ,) -> InterpResult < 'tcx , ImmTy < 'tcx , M :: Provenance > > { use rustc_middle :: mir :: BinOp :: * ; match bin_op { Offset => { let ptr = left . to_scalar () . to_pointer (self) ? ; let pointee_ty = left . layout . ty . builtin_deref (true) . unwrap () ; let pointee_layout = self . layout_of (pointee_ty) ? ; assert ! (pointee_layout . is_sized ()) ; let pointee_size = i64 :: try_from (pointee_layout . size . bytes ()) . unwrap () ; let pointee_size = ImmTy :: from_int (pointee_size , right . layout) ; let (val , overflowed) = self . binary_op (mir :: BinOp :: MulWithOverflow , right , & pointee_size) ? . to_scalar_pair () ; if overflowed . to_bool () ? { throw_ub ! (PointerArithOverflow) } let offset_bytes = val . to_target_isize (self) ? ; if ! right . layout . backend_repr . is_signed () && offset_bytes < 0 { throw_ub ! (PointerArithOverflow) } let offset_ptr = self . ptr_offset_inbounds (ptr , offset_bytes) ? ; interp_ok (ImmTy :: from_scalar (Scalar :: from_maybe_pointer (offset_ptr , self) , left . layout ,)) } _ => M :: binary_ptr_op (self , bin_op , left , right) , } } # [doc = " Returns the result of the specified operation."] # [doc = ""] # [doc = " Whether this produces a scalar or a pair depends on the specific `bin_op`."] pub fn binary_op (& self , bin_op : mir :: BinOp , left : & ImmTy < 'tcx , M :: Provenance > , right : & ImmTy < 'tcx , M :: Provenance > ,) -> InterpResult < 'tcx , ImmTy < 'tcx , M :: Provenance > > { trace ! ("Running binary op {:?}: {:?} ({}), {:?} ({})" , bin_op , * left , left . layout . ty , * right , right . layout . ty) ; match left . layout . ty . kind () { ty :: Char => { assert_eq ! (left . layout . ty , right . layout . ty) ; let left = left . to_scalar () ; let right = right . to_scalar () ; interp_ok (self . binary_char_op (bin_op , left . to_char () ? , right . to_char () ?)) } ty :: Bool => { assert_eq ! (left . layout . ty , right . layout . ty) ; let left = left . to_scalar () ; let right = right . to_scalar () ; interp_ok (self . binary_bool_op (bin_op , left . to_bool () ? , right . to_bool () ?)) } ty :: Float (fty) => { assert_eq ! (left . layout . ty , right . layout . ty) ; let layout = left . layout ; let left = left . to_scalar () ; let right = right . to_scalar () ; interp_ok (match fty { FloatTy :: F16 => { self . binary_float_op (bin_op , layout , left . to_f16 () ? , right . to_f16 () ?) } FloatTy :: F32 => { self . binary_float_op (bin_op , layout , left . to_f32 () ? , right . to_f32 () ?) } FloatTy :: F64 => { self . binary_float_op (bin_op , layout , left . to_f64 () ? , right . to_f64 () ?) } FloatTy :: F128 => { self . binary_float_op (bin_op , layout , left . to_f128 () ? , right . to_f128 () ?) } }) } _ if left . layout . ty . is_integral () => { assert ! (right . layout . ty . is_integral () , "Unexpected types for BinOp: {} {:?} {}" , left . layout . ty , bin_op , right . layout . ty) ; self . binary_int_op (bin_op , left , right) } _ if left . layout . ty . is_any_ptr () => { assert ! (right . layout . ty . is_any_ptr () || right . layout . ty . is_integral () , "Unexpected types for BinOp: {} {:?} {}" , left . layout . ty , bin_op , right . layout . ty) ; self . binary_ptr_op (bin_op , left , right) } _ => span_bug ! (self . cur_span () , "Invalid MIR: bad LHS type for binop: {}" , left . layout . ty) , } } # [doc = " Returns the result of the specified operation, whether it overflowed, and"] # [doc = " the result type."] pub fn unary_op (& self , un_op : mir :: UnOp , val : & ImmTy < 'tcx , M :: Provenance > ,) -> InterpResult < 'tcx , ImmTy < 'tcx , M :: Provenance > > { use rustc_middle :: mir :: UnOp :: * ; let layout = val . layout ; trace ! ("Running unary op {:?}: {:?} ({})" , un_op , val , layout . ty) ; match layout . ty . kind () { ty :: Bool => { let val = val . to_scalar () ; let val = val . to_bool () ? ; let res = match un_op { Not => ! val , _ => span_bug ! (self . cur_span () , "Invalid bool op {:?}" , un_op) , } ; interp_ok (ImmTy :: from_bool (res , * self . tcx)) } ty :: Float (fty) => { let val = val . to_scalar () ; if un_op != Neg { span_bug ! (self . cur_span () , "Invalid float op {:?}" , un_op) ; } let res = match fty { FloatTy :: F16 => Scalar :: from_f16 (- val . to_f16 () ?) , FloatTy :: F32 => Scalar :: from_f32 (- val . to_f32 () ?) , FloatTy :: F64 => Scalar :: from_f64 (- val . to_f64 () ?) , FloatTy :: F128 => Scalar :: from_f128 (- val . to_f128 () ?) , } ; interp_ok (ImmTy :: from_scalar (res , layout)) } ty :: Int (..) => { let val = val . to_scalar () . to_int (layout . size) ? ; let res = match un_op { Not => ! val , Neg => val . wrapping_neg () , _ => span_bug ! (self . cur_span () , "Invalid integer op {:?}" , un_op) , } ; let res = ScalarInt :: truncate_from_int (res , layout . size) . 0 ; interp_ok (ImmTy :: from_scalar (res . into () , layout)) } ty :: Uint (..) => { let val = val . to_scalar () . to_uint (layout . size) ? ; let res = match un_op { Not => ! val , _ => span_bug ! (self . cur_span () , "Invalid unsigned integer op {:?}" , un_op) , } ; let res = ScalarInt :: truncate_from_uint (res , layout . size) . 0 ; interp_ok (ImmTy :: from_scalar (res . into () , layout)) } ty :: RawPtr (..) | ty :: Ref (..) => { assert_eq ! (un_op , PtrMetadata) ; let (_ , meta) = val . to_scalar_and_meta () ; interp_ok (match meta { MemPlaceMeta :: Meta (scalar) => { let ty = un_op . ty (* self . tcx , val . layout . ty) ; let layout = self . layout_of (ty) ? ; ImmTy :: from_scalar (scalar , layout) } MemPlaceMeta :: None => { let unit_layout = self . layout_of (self . tcx . types . unit) ? ; ImmTy :: uninit (unit_layout) } }) } _ => { bug ! ("Unexpected unary op argument {val:?}") } } } pub fn nullary_op (& self , null_op : NullOp < 'tcx > , arg_ty : Ty < 'tcx > ,) -> InterpResult < 'tcx , ImmTy < 'tcx , M :: Provenance > > { use rustc_middle :: mir :: NullOp :: * ; let layout = self . layout_of (arg_ty) ? ; let usize_layout = | | self . layout_of (self . tcx . types . usize) . unwrap () ; interp_ok (match null_op { SizeOf => { if ! layout . is_sized () { span_bug ! (self . cur_span () , "unsized type for `NullaryOp::SizeOf`") ; } let val = layout . size . bytes () ; ImmTy :: from_uint (val , usize_layout ()) } AlignOf => { if ! layout . is_sized () { span_bug ! (self . cur_span () , "unsized type for `NullaryOp::AlignOf`") ; } let val = layout . align . abi . bytes () ; ImmTy :: from_uint (val , usize_layout ()) } OffsetOf (fields) => { let val = self . tcx . offset_of_subfield (self . typing_env , layout , fields . iter ()) . bytes () ; ImmTy :: from_uint (val , usize_layout ()) } UbChecks => ImmTy :: from_bool (M :: ub_checks (self) ? , * self . tcx) , ContractChecks => ImmTy :: from_bool (M :: contract_checks (self) ? , * self . tcx) , }) } }}}