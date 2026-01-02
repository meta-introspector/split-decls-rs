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
mkuse!{use std :: borrow :: Cow ;}
mkuse!{use rustc_ast :: * ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexMap ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_session :: config :: FmtDebug ;}
mkuse!{use rustc_span :: { DesugaringKind , Ident , Span , Symbol , sym } ;}
mkuse!{use super :: LoweringContext ;}
mkitem!{mkimpl!{impl < 'hir > LoweringContext < '_ , 'hir > { pub (crate) fn lower_format_args (& mut self , sp : Span , fmt : & FormatArgs) -> hir :: ExprKind < 'hir > { let allow_const = fmt . arguments . all_args () . is_empty () ; let mut fmt = Cow :: Borrowed (fmt) ; let sp = self . mark_span_with_reason (DesugaringKind :: FormatLiteral { source : fmt . is_source_literal } , sp , sp . ctxt () . outer_expn_data () . allow_internal_unstable ,) ; if self . tcx . sess . opts . unstable_opts . flatten_format_args { fmt = flatten_format_args (fmt) ; fmt = self . inline_literals (fmt) ; } expand_format_args (self , sp , & fmt , allow_const) } #[doc = " Try to convert a literal into an interned string"] fn try_inline_lit (& self , lit : token :: Lit) -> Option < Symbol > { match LitKind :: from_token_lit (lit) { Ok (LitKind :: Str (s , _)) => Some (s) , Ok (LitKind :: Int (n , ty)) => { match ty { LitIntType :: Unsuffixed => { (n <= i32 :: MAX as u128) . then_some (Symbol :: intern (& n . to_string ())) } LitIntType :: Signed (int_ty) => { let max_literal = self . int_ty_max (int_ty) ; (n <= max_literal) . then_some (Symbol :: intern (& n . to_string ())) } LitIntType :: Unsigned (uint_ty) => { let max_literal = self . uint_ty_max (uint_ty) ; (n <= max_literal) . then_some (Symbol :: intern (& n . to_string ())) } } } _ => None , } } #[doc = " Get the maximum value of int_ty. It is platform-dependent due to the byte size of isize"] fn int_ty_max (& self , int_ty : IntTy) -> u128 { match int_ty { IntTy :: Isize => self . tcx . data_layout . pointer_size () . signed_int_max () as u128 , IntTy :: I8 => i8 :: MAX as u128 , IntTy :: I16 => i16 :: MAX as u128 , IntTy :: I32 => i32 :: MAX as u128 , IntTy :: I64 => i64 :: MAX as u128 , IntTy :: I128 => i128 :: MAX as u128 , } } #[doc = " Get the maximum value of uint_ty. It is platform-dependent due to the byte size of usize"] fn uint_ty_max (& self , uint_ty : UintTy) -> u128 { match uint_ty { UintTy :: Usize => self . tcx . data_layout . pointer_size () . unsigned_int_max () , UintTy :: U8 => u8 :: MAX as u128 , UintTy :: U16 => u16 :: MAX as u128 , UintTy :: U32 => u32 :: MAX as u128 , UintTy :: U64 => u64 :: MAX as u128 , UintTy :: U128 => u128 :: MAX as u128 , } } #[doc = " Inline literals into the format string."] #[doc = ""] #[doc = " Turns"] #[doc = ""] #[doc = " `format_args!(\"Hello, {}! {} {}\", \"World\", 123, x)`"] #[doc = ""] #[doc = " into"] #[doc = ""] #[doc = " `format_args!(\"Hello, World! 123 {}\", x)`."] fn inline_literals < 'fmt > (& self , mut fmt : Cow < 'fmt , FormatArgs >) -> Cow < 'fmt , FormatArgs > { let mut was_inlined = vec ! [false ; fmt . arguments . all_args () . len ()] ; let mut inlined_anything = false ; for i in 0 .. fmt . template . len () { let FormatArgsPiece :: Placeholder (placeholder) = & fmt . template [i] else { continue } ; let Ok (arg_index) = placeholder . argument . index else { continue } ; let mut literal = None ; if let FormatTrait :: Display = placeholder . format_trait && placeholder . format_options == Default :: default () && let arg = fmt . arguments . all_args () [arg_index] . expr . peel_parens_and_refs () && let ExprKind :: Lit (lit) = arg . kind { literal = self . try_inline_lit (lit) ; } if let Some (literal) = literal { let fmt = fmt . to_mut () ; fmt . template [i] = FormatArgsPiece :: Literal (literal) ; was_inlined [arg_index] = true ; inlined_anything = true ; } } if inlined_anything { let fmt = fmt . to_mut () ; let mut remove = was_inlined ; for_all_argument_indexes (& mut fmt . template , | index | remove [* index] = false) ; let mut remove_it = remove . iter () ; fmt . arguments . all_args_mut () . retain (| _ | remove_it . next () != Some (& true)) ; let index_map : Vec < usize > = remove . into_iter () . scan (0 , | i , remove | { let mapped = * i ; * i += ! remove as usize ; Some (mapped) }) . collect () ; for_all_argument_indexes (& mut fmt . template , | index | * index = index_map [* index]) ; } fmt } }}}

macro_rules! flatten_format_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function flatten_format_args in module {}", module_path!());
    };
}

mkfn!{
    flatten_format_args_introspect!();
    #[doc = " Flattens nested `format_args!()` into one."] #[doc = ""] #[doc = " Turns"] #[doc = ""] #[doc = " `format_args!(\"a {} {} {}.\", 1, format_args!(\"b{}!\", 2), 3)`"] #[doc = ""] #[doc = " into"] #[doc = ""] #[doc = " `format_args!(\"a {} b{}! {}.\", 1, 2, 3)`."] fn flatten_format_args (mut fmt : Cow < '_ , FormatArgs >) -> Cow < '_ , FormatArgs > { let mut i = 0 ; while i < fmt . template . len () { if let FormatArgsPiece :: Placeholder (placeholder) = & fmt . template [i] && let FormatTrait :: Display | FormatTrait :: Debug = & placeholder . format_trait && let Ok (arg_index) = placeholder . argument . index && let arg = fmt . arguments . all_args () [arg_index] . expr . peel_parens_and_refs () && let ExprKind :: FormatArgs (_) = & arg . kind && fmt . template . iter () . enumerate () . all (| (j , p) | i == j || ! matches ! (p , FormatArgsPiece :: Placeholder (placeholder) if placeholder . argument . index == Ok (arg_index))) { let fmt = fmt . to_mut () ; let args = fmt . arguments . all_args_mut () ; let remaining_args = args . split_off (arg_index + 1) ; let old_arg_offset = args . len () ; let mut fmt2 = & mut args . pop () . unwrap () . expr ; let fmt2 = loop { match & mut fmt2 . kind { ExprKind :: Paren (inner) | ExprKind :: AddrOf (BorrowKind :: Ref , _ , inner) => { fmt2 = inner } ExprKind :: FormatArgs (fmt2) => break fmt2 , _ => unreachable ! () , } } ; args . append (fmt2 . arguments . all_args_mut ()) ; let new_arg_offset = args . len () ; args . extend (remaining_args) ; for_all_argument_indexes (& mut fmt . template , | index | { if * index >= old_arg_offset { * index -= old_arg_offset ; * index += new_arg_offset ; } }) ; let rest = fmt . template . split_off (i + 1) ; fmt . template . pop () ; for_all_argument_indexes (& mut fmt2 . template , | index | * index += arg_index) ; fmt . template . append (& mut fmt2 . template) ; fmt . template . extend (rest) ; } else { i += 1 ; } } fmt }
}
mkitem!{mkenum!{#[derive (Copy , Clone , Debug , Hash , PartialEq , Eq)] enum ArgumentType { Format (FormatTrait) , Usize , }}}

macro_rules! make_argument_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_argument in module {}", module_path!());
    };
}

mkfn!{
    make_argument_introspect!();
    #[doc = " Generate a hir expression representing an argument to a format_args invocation."] #[doc = ""] #[doc = " Generates:"] #[doc = ""] #[doc = " ```text"] #[doc = "     <core::fmt::Argument>::new_…(arg)"] #[doc = " ```"] fn make_argument < 'hir > (ctx : & mut LoweringContext < '_ , 'hir > , sp : Span , arg : & 'hir hir :: Expr < 'hir > , ty : ArgumentType ,) -> hir :: Expr < 'hir > { use ArgumentType :: * ; use FormatTrait :: * ; let new_fn = ctx . arena . alloc (ctx . expr_lang_item_type_relative (sp , hir :: LangItem :: FormatArgument , match ty { Format (Display) => sym :: new_display , Format (Debug) => match ctx . tcx . sess . opts . unstable_opts . fmt_debug { FmtDebug :: Full | FmtDebug :: Shallow => sym :: new_debug , FmtDebug :: None => sym :: new_debug_noop , } , Format (LowerExp) => sym :: new_lower_exp , Format (UpperExp) => sym :: new_upper_exp , Format (Octal) => sym :: new_octal , Format (Pointer) => sym :: new_pointer , Format (Binary) => sym :: new_binary , Format (LowerHex) => sym :: new_lower_hex , Format (UpperHex) => sym :: new_upper_hex , Usize => sym :: from_usize , } ,)) ; ctx . expr_call_mut (sp , new_fn , std :: slice :: from_ref (arg)) }
}

macro_rules! make_count_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_count in module {}", module_path!());
    };
}

mkfn!{
    make_count_introspect!();
    #[doc = " Generate a hir expression for a format_args Count."] #[doc = ""] #[doc = " Generates:"] #[doc = ""] #[doc = " ```text"] #[doc = "     <core::fmt::rt::Count>::Is(…)"] #[doc = " ```"] #[doc = ""] #[doc = " or"] #[doc = ""] #[doc = " ```text"] #[doc = "     <core::fmt::rt::Count>::Param(…)"] #[doc = " ```"] #[doc = ""] #[doc = " or"] #[doc = ""] #[doc = " ```text"] #[doc = "     <core::fmt::rt::Count>::Implied"] #[doc = " ```"] fn make_count < 'hir > (ctx : & mut LoweringContext < '_ , 'hir > , sp : Span , count : & Option < FormatCount > , argmap : & mut FxIndexMap < (usize , ArgumentType) , Option < Span > > ,) -> hir :: Expr < 'hir > { match count { Some (FormatCount :: Literal (n)) => { let count_is = ctx . arena . alloc (ctx . expr_lang_item_type_relative (sp , hir :: LangItem :: FormatCount , sym :: Is ,)) ; let value = ctx . arena . alloc_from_iter ([ctx . expr_u16 (sp , * n)]) ; ctx . expr_call_mut (sp , count_is , value) } Some (FormatCount :: Argument (arg)) => { if let Ok (arg_index) = arg . index { let (i , _) = argmap . insert_full ((arg_index , ArgumentType :: Usize) , arg . span) ; let count_param = ctx . arena . alloc (ctx . expr_lang_item_type_relative (sp , hir :: LangItem :: FormatCount , sym :: Param ,)) ; let value = ctx . arena . alloc_from_iter ([ctx . expr_usize (sp , i)]) ; ctx . expr_call_mut (sp , count_param , value) } else { ctx . expr (sp , hir :: ExprKind :: Err (ctx . dcx () . span_delayed_bug (sp , "lowered bad format_args count") ,) ,) } } None => ctx . expr_lang_item_type_relative (sp , hir :: LangItem :: FormatCount , sym :: Implied) , } }
}

macro_rules! make_format_spec_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_format_spec in module {}", module_path!());
    };
}

mkfn!{
    make_format_spec_introspect!();
    #[doc = " Generate a hir expression for a format_args placeholder specification."] #[doc = ""] #[doc = " Generates"] #[doc = ""] #[doc = " ```text"] #[doc = "     <core::fmt::rt::Placeholder {"] #[doc = "         position: …usize,"] #[doc = "         flags: …u32,"] #[doc = "         precision: <core::fmt::rt::Count::…>,"] #[doc = "         width: <core::fmt::rt::Count::…>,"] #[doc = "     }"] #[doc = " ```"] fn make_format_spec < 'hir > (ctx : & mut LoweringContext < '_ , 'hir > , sp : Span , placeholder : & FormatPlaceholder , argmap : & mut FxIndexMap < (usize , ArgumentType) , Option < Span > > ,) -> hir :: Expr < 'hir > { let position = match placeholder . argument . index { Ok (arg_index) => { let (i , _) = argmap . insert_full ((arg_index , ArgumentType :: Format (placeholder . format_trait)) , placeholder . span ,) ; ctx . expr_usize (sp , i) } Err (_) => ctx . expr (sp , hir :: ExprKind :: Err (ctx . dcx () . span_delayed_bug (sp , "lowered bad format_args count")) ,) , } ; let & FormatOptions { ref width , ref precision , alignment , fill , sign , alternate , zero_pad , debug_hex , } = & placeholder . format_options ; let fill = fill . unwrap_or (' ') ; let align = match alignment { Some (FormatAlignment :: Left) => 0 , Some (FormatAlignment :: Right) => 1 , Some (FormatAlignment :: Center) => 2 , None => 3 , } ; let flags : u32 = fill as u32 | ((sign == Some (FormatSign :: Plus)) as u32) << 21 | ((sign == Some (FormatSign :: Minus)) as u32) << 22 | (alternate as u32) << 23 | (zero_pad as u32) << 24 | ((debug_hex == Some (FormatDebugHex :: Lower)) as u32) << 25 | ((debug_hex == Some (FormatDebugHex :: Upper)) as u32) << 26 | (width . is_some () as u32) << 27 | (precision . is_some () as u32) << 28 | align << 29 | 1 << 31 ; let flags = ctx . expr_u32 (sp , flags) ; let precision = make_count (ctx , sp , precision , argmap) ; let width = make_count (ctx , sp , width , argmap) ; let position = ctx . expr_field (Ident :: new (sym :: position , sp) , ctx . arena . alloc (position) , sp) ; let flags = ctx . expr_field (Ident :: new (sym :: flags , sp) , ctx . arena . alloc (flags) , sp) ; let precision = ctx . expr_field (Ident :: new (sym :: precision , sp) , ctx . arena . alloc (precision) , sp) ; let width = ctx . expr_field (Ident :: new (sym :: width , sp) , ctx . arena . alloc (width) , sp) ; let placeholder = ctx . arena . alloc (hir :: QPath :: LangItem (hir :: LangItem :: FormatPlaceholder , sp)) ; let fields = ctx . arena . alloc_from_iter ([position , flags , precision , width]) ; ctx . expr (sp , hir :: ExprKind :: Struct (placeholder , fields , hir :: StructTailExpr :: None)) }
}

macro_rules! expand_format_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_format_args in module {}", module_path!());
    };
}

mkfn!{
    expand_format_args_introspect!();
    fn expand_format_args < 'hir > (ctx : & mut LoweringContext < '_ , 'hir > , macsp : Span , fmt : & FormatArgs , allow_const : bool ,) -> hir :: ExprKind < 'hir > { let macsp = ctx . lower_span (macsp) ; let mut incomplete_lit = String :: new () ; let lit_pieces = ctx . arena . alloc_from_iter (fmt . template . iter () . enumerate () . filter_map (| (i , piece) | { match piece { & FormatArgsPiece :: Literal (s) => { if let Some (FormatArgsPiece :: Literal (_)) = fmt . template . get (i + 1) { incomplete_lit . push_str (s . as_str ()) ; None } else if ! incomplete_lit . is_empty () { incomplete_lit . push_str (s . as_str ()) ; let s = Symbol :: intern (& incomplete_lit) ; incomplete_lit . clear () ; Some (ctx . expr_str (fmt . span , s)) } else { Some (ctx . expr_str (fmt . span , s)) } } & FormatArgsPiece :: Placeholder (_) => { if i == 0 || matches ! (fmt . template [i - 1] , FormatArgsPiece :: Placeholder (_)) { Some (ctx . expr_str (fmt . span , sym :: empty)) } else { None } } } })) ; let lit_pieces = ctx . expr_array_ref (fmt . span , lit_pieces) ; let mut use_format_options = false ; let mut argmap = FxIndexMap :: default () ; for piece in & fmt . template { let FormatArgsPiece :: Placeholder (placeholder) = piece else { continue } ; if placeholder . format_options != Default :: default () { use_format_options = true ; } if let Ok (index) = placeholder . argument . index { if argmap . insert ((index , ArgumentType :: Format (placeholder . format_trait)) , placeholder . span) . is_some () { use_format_options = true ; } } } let format_options = use_format_options . then (| | { let elements = ctx . arena . alloc_from_iter (fmt . template . iter () . filter_map (| piece | { let FormatArgsPiece :: Placeholder (placeholder) = piece else { return None } ; Some (make_format_spec (ctx , macsp , placeholder , & mut argmap)) })) ; ctx . expr_array_ref (macsp , elements) }) ; let arguments = fmt . arguments . all_args () ; if allow_const && arguments . is_empty () && argmap . is_empty () { let new = ctx . arena . alloc (ctx . expr_lang_item_type_relative (macsp , hir :: LangItem :: FormatArguments , sym :: new_const ,)) ; let new_args = ctx . arena . alloc_from_iter ([lit_pieces]) ; return hir :: ExprKind :: Call (new , new_args) ; } let (let_statements , args) = if arguments . is_empty () { (vec ! [] , ctx . arena . alloc (ctx . expr (macsp , hir :: ExprKind :: Array (& [])))) } else if argmap . len () == 1 && arguments . len () == 1 { let args = ctx . arena . alloc_from_iter (argmap . iter () . map (| (& (arg_index , ty) , & placeholder_span) | { let arg = & arguments [arg_index] ; let placeholder_span = placeholder_span . unwrap_or (arg . expr . span) . with_ctxt (macsp . ctxt ()) ; let arg = ctx . lower_expr (& arg . expr) ; let ref_arg = ctx . arena . alloc (ctx . expr_ref (arg . span . with_ctxt (macsp . ctxt ()) , arg)) ; make_argument (ctx , placeholder_span , ref_arg , ty) } ,)) ; let args = ctx . arena . alloc (ctx . expr (macsp , hir :: ExprKind :: Array (args))) ; let args_ident = Ident :: new (sym :: args , macsp) ; let (args_pat , args_hir_id) = ctx . pat_ident (macsp , args_ident) ; let let_statement = ctx . stmt_super_let_pat (macsp , args_pat , Some (args)) ; (vec ! [let_statement] , ctx . arena . alloc (ctx . expr_ident_mut (macsp , args_ident , args_hir_id))) } else { let args_ident = Ident :: new (sym :: args , macsp) ; let (args_pat , args_hir_id) = ctx . pat_ident (macsp , args_ident) ; let elements = ctx . arena . alloc_from_iter (arguments . iter () . map (| arg | { let arg_expr = ctx . lower_expr (& arg . expr) ; ctx . expr (arg . expr . span . with_ctxt (macsp . ctxt ()) , hir :: ExprKind :: AddrOf (hir :: BorrowKind :: Ref , hir :: Mutability :: Not , arg_expr) ,) })) ; let args_tuple = ctx . arena . alloc (ctx . expr (macsp , hir :: ExprKind :: Tup (elements))) ; let let_statement_1 = ctx . stmt_super_let_pat (macsp , args_pat , Some (args_tuple)) ; let args = ctx . arena . alloc_from_iter (argmap . iter () . map (| (& (arg_index , ty) , & placeholder_span) | { let arg = & arguments [arg_index] ; let placeholder_span = placeholder_span . unwrap_or (arg . expr . span) . with_ctxt (macsp . ctxt ()) ; let arg_span = match arg . kind { FormatArgumentKind :: Captured (_) => placeholder_span , _ => arg . expr . span . with_ctxt (macsp . ctxt ()) , } ; let args_ident_expr = ctx . expr_ident (macsp , args_ident , args_hir_id) ; let arg = ctx . arena . alloc (ctx . expr (arg_span , hir :: ExprKind :: Field (args_ident_expr , Ident :: new (sym :: integer (arg_index) , macsp) ,) ,)) ; make_argument (ctx , placeholder_span , arg , ty) } ,)) ; let args = ctx . arena . alloc (ctx . expr (macsp , hir :: ExprKind :: Array (args))) ; let (args_pat , args_hir_id) = ctx . pat_ident (macsp , args_ident) ; let let_statement_2 = ctx . stmt_super_let_pat (macsp , args_pat , Some (args)) ; (vec ! [let_statement_1 , let_statement_2] , ctx . arena . alloc (ctx . expr_ident_mut (macsp , args_ident , args_hir_id)) ,) } ; let args = ctx . expr_ref (macsp , args) ; let call = if let Some (format_options) = format_options { let new_v1_formatted = ctx . arena . alloc (ctx . expr_lang_item_type_relative (macsp , hir :: LangItem :: FormatArguments , sym :: new_v1_formatted ,)) ; let args = ctx . arena . alloc_from_iter ([lit_pieces , args , format_options]) ; let call = ctx . expr_call (macsp , new_v1_formatted , args) ; let hir_id = ctx . next_id () ; hir :: ExprKind :: Block (ctx . arena . alloc (hir :: Block { stmts : & [] , expr : Some (call) , hir_id , rules : hir :: BlockCheckMode :: UnsafeBlock (hir :: UnsafeSource :: CompilerGenerated) , span : macsp , targeted_by_break : false , }) , None ,) } else { let new_v1 = ctx . arena . alloc (ctx . expr_lang_item_type_relative (macsp , hir :: LangItem :: FormatArguments , sym :: new_v1 ,)) ; let new_args = ctx . arena . alloc_from_iter ([lit_pieces , args]) ; hir :: ExprKind :: Call (new_v1 , new_args) } ; if ! let_statements . is_empty () { let call = ctx . arena . alloc (ctx . expr (macsp , call)) ; let block = ctx . block_all (macsp , ctx . arena . alloc_from_iter (let_statements) , Some (call)) ; hir :: ExprKind :: Block (block , None) } else { call } }
}

macro_rules! for_all_argument_indexes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function for_all_argument_indexes in module {}", module_path!());
    };
}

mkfn!{
    for_all_argument_indexes_introspect!();
    fn for_all_argument_indexes (template : & mut [FormatArgsPiece] , mut f : impl FnMut (& mut usize)) { for piece in template { let FormatArgsPiece :: Placeholder (placeholder) = piece else { continue } ; if let Ok (index) = & mut placeholder . argument . index { f (index) ; } if let Some (FormatCount :: Argument (FormatArgPosition { index : Ok (index) , .. })) = & mut placeholder . format_options . width { f (index) ; } if let Some (FormatCount :: Argument (FormatArgPosition { index : Ok (index) , .. })) = & mut placeholder . format_options . precision { f (index) ; } } }
}