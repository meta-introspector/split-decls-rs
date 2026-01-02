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
mkuse!{use itertools :: Itertools as _ ;}
mkuse!{use rustc_abi :: { self as abi , FIRST_VARIANT } ;}
mkuse!{use rustc_middle :: ty :: adjustment :: PointerCoercion ;}
mkuse!{use rustc_middle :: ty :: layout :: { HasTyCtxt , HasTypingEnv , LayoutOf , TyAndLayout } ;}
mkuse!{use rustc_middle :: ty :: { self , Instance , Ty , TyCtxt } ;}
mkuse!{use rustc_middle :: { bug , mir , span_bug } ;}
mkuse!{use rustc_session :: config :: OptLevel ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use super :: operand :: { OperandRef , OperandRefBuilder , OperandValue } ;}
mkuse!{use super :: place :: { PlaceRef , PlaceValue , codegen_tag_value } ;}
mkuse!{use super :: { FunctionCx , LocalRef } ;}
mkuse!{use crate :: common :: { IntPredicate , TypeKind } ;}
mkuse!{use crate :: traits :: * ;}
mkuse!{use crate :: { MemFlags , base } ;}
mkitem!{mkimpl!{impl < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > FunctionCx < 'a , 'tcx , Bx > { #[instrument (level = "trace" , skip (self , bx))] pub (crate) fn codegen_rvalue (& mut self , bx : & mut Bx , dest : PlaceRef < 'tcx , Bx :: Value > , rvalue : & mir :: Rvalue < 'tcx > ,) { match * rvalue { mir :: Rvalue :: Use (ref operand) => { let cg_operand = self . codegen_operand (bx , operand) ; cg_operand . val . store (bx , dest) ; } mir :: Rvalue :: Cast (mir :: CastKind :: PointerCoercion (PointerCoercion :: Unsize , _) , ref source , _ ,) => { if bx . cx () . is_backend_scalar_pair (dest . layout) { let temp = self . codegen_rvalue_operand (bx , rvalue) ; temp . val . store (bx , dest) ; return ; } let operand = self . codegen_operand (bx , source) ; match operand . val { OperandValue :: Pair (..) | OperandValue :: Immediate (_) => { debug ! ("codegen_rvalue: creating ugly alloca") ; let scratch = PlaceRef :: alloca (bx , operand . layout) ; scratch . storage_live (bx) ; operand . val . store (bx , scratch) ; base :: coerce_unsized_into (bx , scratch , dest) ; scratch . storage_dead (bx) ; } OperandValue :: Ref (val) => { if val . llextra . is_some () { bug ! ("unsized coercion on an unsized rvalue") ; } base :: coerce_unsized_into (bx , val . with_type (operand . layout) , dest) ; } OperandValue :: ZeroSized => { bug ! ("unsized coercion on a ZST rvalue") ; } } } mir :: Rvalue :: Cast (mir :: CastKind :: Transmute , ref operand , _ty) => { let src = self . codegen_operand (bx , operand) ; self . codegen_transmute (bx , src , dest) ; } mir :: Rvalue :: Repeat (ref elem , count) => { if dest . layout . is_zst () { return ; } if let mir :: Operand :: Constant (const_op) = elem { let val = self . eval_mir_constant (const_op) ; if val . all_bytes_uninit (self . cx . tcx ()) { let size = bx . const_usize (dest . layout . size . bytes ()) ; bx . memset (dest . val . llval , bx . const_undef (bx . type_i8 ()) , size , dest . val . align , MemFlags :: empty () ,) ; return ; } } let cg_elem = self . codegen_operand (bx , elem) ; let try_init_all_same = | bx : & mut Bx , v | { let start = dest . val . llval ; let size = bx . const_usize (dest . layout . size . bytes ()) ; if let Some (int) = bx . cx () . const_to_opt_u128 (v , false) && let bytes = & int . to_le_bytes () [.. cg_elem . layout . size . bytes_usize ()] && let Ok (& byte) = bytes . iter () . all_equal_value () { let fill = bx . cx () . const_u8 (byte) ; bx . memset (start , fill , size , dest . val . align , MemFlags :: empty ()) ; return true ; } let v = bx . from_immediate (v) ; if bx . cx () . val_ty (v) == bx . cx () . type_i8 () { bx . memset (start , v , size , dest . val . align , MemFlags :: empty ()) ; return true ; } false } ; if let OperandValue :: Immediate (v) = cg_elem . val && try_init_all_same (bx , v) { return ; } let count = self . monomorphize (count) . try_to_target_usize (bx . tcx ()) . expect ("expected monomorphic const in codegen") ; bx . write_operand_repeatedly (cg_elem , count , dest) ; } mir :: Rvalue :: Aggregate (ref kind , ref operands) if ! matches ! (** kind , mir :: AggregateKind :: RawPtr (..)) => { let (variant_index , variant_dest , active_field_index) = match * * kind { mir :: AggregateKind :: Adt (_ , variant_index , _ , _ , active_field_index) => { let variant_dest = dest . project_downcast (bx , variant_index) ; (variant_index , variant_dest , active_field_index) } _ => (FIRST_VARIANT , dest , None) , } ; if active_field_index . is_some () { assert_eq ! (operands . len () , 1) ; } for (i , operand) in operands . iter_enumerated () { let op = self . codegen_operand (bx , operand) ; if ! op . layout . is_zst () { let field_index = active_field_index . unwrap_or (i) ; let field = if let mir :: AggregateKind :: Array (_) = * * kind { let llindex = bx . cx () . const_usize (field_index . as_u32 () . into ()) ; variant_dest . project_index (bx , llindex) } else { variant_dest . project_field (bx , field_index . as_usize ()) } ; op . val . store (bx , field) ; } } dest . codegen_set_discr (bx , variant_index) ; } _ => { let temp = self . codegen_rvalue_operand (bx , rvalue) ; temp . val . store (bx , dest) ; } } } #[doc = " Transmutes the `src` value to the destination type by writing it to `dst`."] #[doc = ""] #[doc = " See also [`Self::codegen_transmute_operand`] for cases that can be done"] #[doc = " without needing a pre-allocated place for the destination."] fn codegen_transmute (& mut self , bx : & mut Bx , src : OperandRef < 'tcx , Bx :: Value > , dst : PlaceRef < 'tcx , Bx :: Value > ,) { assert ! (src . layout . is_sized ()) ; assert ! (dst . layout . is_sized ()) ; if src . layout . size != dst . layout . size || src . layout . is_uninhabited () || dst . layout . is_uninhabited () { bx . unreachable_nonterminator () ; } else { src . val . store (bx , dst . val . with_type (src . layout)) ; } } #[doc = " Transmutes an `OperandValue` to another `OperandValue`."] #[doc = ""] #[doc = " This is supported for all cases where the `cast` type is SSA,"] #[doc = " but for non-ZSTs with [`abi::BackendRepr::Memory`] it ICEs."] pub (crate) fn codegen_transmute_operand (& mut self , bx : & mut Bx , operand : OperandRef < 'tcx , Bx :: Value > , cast : TyAndLayout < 'tcx > ,) -> OperandValue < Bx :: Value > { if let abi :: BackendRepr :: Memory { .. } = cast . backend_repr && ! cast . is_zst () { span_bug ! (self . mir . span , "Use `codegen_transmute` to transmute to {cast:?}") ; } if abi :: Layout :: eq (& operand . layout . layout , & cast . layout) { return operand . val ; } if operand . layout . size != cast . size || operand . layout . is_uninhabited () || cast . is_uninhabited () { bx . unreachable_nonterminator () ; return OperandValue :: poison (bx , cast) ; } #[inline] fn vector_can_bitcast (x : abi :: Scalar) -> bool { matches ! (x , abi :: Scalar :: Initialized { value : abi :: Primitive :: Int (..) | abi :: Primitive :: Float (..) , .. }) } let cx = bx . cx () ; match (operand . val , operand . layout . backend_repr , cast . backend_repr) { _ if cast . is_zst () => OperandValue :: ZeroSized , (OperandValue :: Ref (source_place_val) , abi :: BackendRepr :: Memory { .. } , _) => { assert_eq ! (source_place_val . llextra , None) ; bx . load_operand (source_place_val . with_type (cast)) . val } (OperandValue :: Immediate (imm) , abi :: BackendRepr :: Scalar (from_scalar) , abi :: BackendRepr :: Scalar (to_scalar) ,) if from_scalar . size (cx) == to_scalar . size (cx) => { OperandValue :: Immediate (transmute_scalar (bx , imm , from_scalar , to_scalar)) } (OperandValue :: Immediate (imm) , abi :: BackendRepr :: SimdVector { element : from_scalar , .. } , abi :: BackendRepr :: SimdVector { element : to_scalar , .. } ,) if vector_can_bitcast (from_scalar) && vector_can_bitcast (to_scalar) => { let to_backend_ty = bx . cx () . immediate_backend_type (cast) ; OperandValue :: Immediate (bx . bitcast (imm , to_backend_ty)) } (OperandValue :: Pair (imm_a , imm_b) , abi :: BackendRepr :: ScalarPair (in_a , in_b) , abi :: BackendRepr :: ScalarPair (out_a , out_b) ,) if in_a . size (cx) == out_a . size (cx) && in_b . size (cx) == out_b . size (cx) => { OperandValue :: Pair (transmute_scalar (bx , imm_a , in_a , out_a) , transmute_scalar (bx , imm_b , in_b , out_b) ,) } _ => { let align = Ord :: max (operand . layout . align . abi , cast . align . abi) ; let size = Ord :: max (operand . layout . size , cast . size) ; let temp = PlaceValue :: alloca (bx , size , align) ; bx . lifetime_start (temp . llval , size) ; operand . val . store (bx , temp . with_type (operand . layout)) ; let val = bx . load_operand (temp . with_type (cast)) . val ; bx . lifetime_end (temp . llval , size) ; val } } } #[doc = " Cast one of the immediates from an [`OperandValue::Immediate`]"] #[doc = " or an [`OperandValue::Pair`] to an immediate of the target type."] #[doc = ""] #[doc = " Returns `None` if the cast is not possible."] fn cast_immediate (& self , bx : & mut Bx , mut imm : Bx :: Value , from_scalar : abi :: Scalar , from_backend_ty : Bx :: Type , to_scalar : abi :: Scalar , to_backend_ty : Bx :: Type ,) -> Option < Bx :: Value > { use abi :: Primitive :: * ; assume_scalar_range (bx , imm , from_scalar , from_backend_ty , None) ; imm = match (from_scalar . primitive () , to_scalar . primitive ()) { (Int (_ , is_signed) , Int (..)) => bx . intcast (imm , to_backend_ty , is_signed) , (Float (_) , Float (_)) => { let srcsz = bx . cx () . float_width (from_backend_ty) ; let dstsz = bx . cx () . float_width (to_backend_ty) ; if dstsz > srcsz { bx . fpext (imm , to_backend_ty) } else if srcsz > dstsz { bx . fptrunc (imm , to_backend_ty) } else { imm } } (Int (_ , is_signed) , Float (_)) => { if is_signed { bx . sitofp (imm , to_backend_ty) } else { bx . uitofp (imm , to_backend_ty) } } (Pointer (..) , Pointer (..)) => bx . pointercast (imm , to_backend_ty) , (Int (_ , is_signed) , Pointer (..)) => { let usize_imm = bx . intcast (imm , bx . cx () . type_isize () , is_signed) ; bx . inttoptr (usize_imm , to_backend_ty) } (Float (_) , Int (_ , is_signed)) => bx . cast_float_to_int (is_signed , imm , to_backend_ty) , _ => return None , } ; Some (imm) } pub (crate) fn codegen_rvalue_operand (& mut self , bx : & mut Bx , rvalue : & mir :: Rvalue < 'tcx > ,) -> OperandRef < 'tcx , Bx :: Value > { match * rvalue { mir :: Rvalue :: Cast (ref kind , ref source , mir_cast_ty) => { let operand = self . codegen_operand (bx , source) ; debug ! ("cast operand is {:?}" , operand) ; let cast = bx . cx () . layout_of (self . monomorphize (mir_cast_ty)) ; let val = match * kind { mir :: CastKind :: PointerExposeProvenance => { assert ! (bx . cx () . is_backend_immediate (cast)) ; let llptr = operand . immediate () ; let llcast_ty = bx . cx () . immediate_backend_type (cast) ; let lladdr = bx . ptrtoint (llptr , llcast_ty) ; OperandValue :: Immediate (lladdr) } mir :: CastKind :: PointerCoercion (PointerCoercion :: ReifyFnPointer , _) => { match * operand . layout . ty . kind () { ty :: FnDef (def_id , args) => { let instance = ty :: Instance :: resolve_for_fn_ptr (bx . tcx () , bx . typing_env () , def_id , args ,) . unwrap () ; OperandValue :: Immediate (bx . get_fn_addr (instance)) } _ => bug ! ("{} cannot be reified to a fn ptr" , operand . layout . ty) , } } mir :: CastKind :: PointerCoercion (PointerCoercion :: ClosureFnPointer (_) , _) => { match * operand . layout . ty . kind () { ty :: Closure (def_id , args) => { let instance = Instance :: resolve_closure (bx . cx () . tcx () , def_id , args , ty :: ClosureKind :: FnOnce ,) ; OperandValue :: Immediate (bx . cx () . get_fn_addr (instance)) } _ => bug ! ("{} cannot be cast to a fn ptr" , operand . layout . ty) , } } mir :: CastKind :: PointerCoercion (PointerCoercion :: UnsafeFnPointer , _) => { operand . val } mir :: CastKind :: PointerCoercion (PointerCoercion :: Unsize , _) => { assert ! (bx . cx () . is_backend_scalar_pair (cast)) ; let (lldata , llextra) = operand . val . pointer_parts () ; let (lldata , llextra) = base :: unsize_ptr (bx , lldata , operand . layout . ty , cast . ty , llextra) ; OperandValue :: Pair (lldata , llextra) } mir :: CastKind :: PointerCoercion (PointerCoercion :: MutToConstPointer | PointerCoercion :: ArrayToPointer , _) => { bug ! ("{kind:?} is for borrowck, and should never appear in codegen") ; } mir :: CastKind :: PtrToPtr if bx . cx () . is_backend_scalar_pair (operand . layout) => { if let OperandValue :: Pair (data_ptr , meta) = operand . val { if bx . cx () . is_backend_scalar_pair (cast) { OperandValue :: Pair (data_ptr , meta) } else { OperandValue :: Immediate (data_ptr) } } else { bug ! ("unexpected non-pair operand") ; } } | mir :: CastKind :: IntToInt | mir :: CastKind :: FloatToInt | mir :: CastKind :: FloatToFloat | mir :: CastKind :: IntToFloat | mir :: CastKind :: PtrToPtr | mir :: CastKind :: FnPtrToPtr | mir :: CastKind :: PointerWithExposedProvenance => { let imm = operand . immediate () ; let abi :: BackendRepr :: Scalar (from_scalar) = operand . layout . backend_repr else { bug ! ("Found non-scalar for operand {operand:?}") ; } ; let from_backend_ty = bx . cx () . immediate_backend_type (operand . layout) ; assert ! (bx . cx () . is_backend_immediate (cast)) ; let to_backend_ty = bx . cx () . immediate_backend_type (cast) ; if operand . layout . is_uninhabited () { let val = OperandValue :: Immediate (bx . cx () . const_poison (to_backend_ty)) ; return OperandRef { val , layout : cast } ; } let abi :: BackendRepr :: Scalar (to_scalar) = cast . layout . backend_repr else { bug ! ("Found non-scalar for cast {cast:?}") ; } ; self . cast_immediate (bx , imm , from_scalar , from_backend_ty , to_scalar , to_backend_ty) . map (OperandValue :: Immediate) . unwrap_or_else (| | { bug ! ("Unsupported cast of {operand:?} to {cast:?}") ; }) } mir :: CastKind :: Transmute => { self . codegen_transmute_operand (bx , operand , cast) } } ; OperandRef { val , layout : cast } } mir :: Rvalue :: Ref (_ , bk , place) => { let mk_ref = move | tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > | { Ty :: new_ref (tcx , tcx . lifetimes . re_erased , ty , bk . to_mutbl_lossy ()) } ; self . codegen_place_to_pointer (bx , place , mk_ref) } mir :: Rvalue :: CopyForDeref (place) => { self . codegen_operand (bx , & mir :: Operand :: Copy (place)) } mir :: Rvalue :: RawPtr (kind , place) => { let mk_ptr = move | tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > | { Ty :: new_ptr (tcx , ty , kind . to_mutbl_lossy ()) } ; self . codegen_place_to_pointer (bx , place , mk_ptr) } mir :: Rvalue :: Len (place) => { let size = self . evaluate_array_len (bx , place) ; OperandRef { val : OperandValue :: Immediate (size) , layout : bx . cx () . layout_of (bx . tcx () . types . usize) , } } mir :: Rvalue :: BinaryOp (op_with_overflow , box (ref lhs , ref rhs)) if let Some (op) = op_with_overflow . overflowing_to_wrapping () => { let lhs = self . codegen_operand (bx , lhs) ; let rhs = self . codegen_operand (bx , rhs) ; let result = self . codegen_scalar_checked_binop (bx , op , lhs . immediate () , rhs . immediate () , lhs . layout . ty ,) ; let val_ty = op . ty (bx . tcx () , lhs . layout . ty , rhs . layout . ty) ; let operand_ty = Ty :: new_tup (bx . tcx () , & [val_ty , bx . tcx () . types . bool]) ; OperandRef { val : result , layout : bx . cx () . layout_of (operand_ty) } } mir :: Rvalue :: BinaryOp (op , box (ref lhs , ref rhs)) => { let lhs = self . codegen_operand (bx , lhs) ; let rhs = self . codegen_operand (bx , rhs) ; let llresult = match (lhs . val , rhs . val) { (OperandValue :: Pair (lhs_addr , lhs_extra) , OperandValue :: Pair (rhs_addr , rhs_extra) ,) => self . codegen_wide_ptr_binop (bx , op , lhs_addr , lhs_extra , rhs_addr , rhs_extra , lhs . layout . ty ,) , (OperandValue :: Immediate (lhs_val) , OperandValue :: Immediate (rhs_val)) => self . codegen_scalar_binop (bx , op , lhs_val , rhs_val , lhs . layout . ty , rhs . layout . ty ,) , _ => bug ! () , } ; OperandRef { val : OperandValue :: Immediate (llresult) , layout : bx . cx () . layout_of (op . ty (bx . tcx () , lhs . layout . ty , rhs . layout . ty)) , } } mir :: Rvalue :: UnaryOp (op , ref operand) => { let operand = self . codegen_operand (bx , operand) ; let is_float = operand . layout . ty . is_floating_point () ; let (val , layout) = match op { mir :: UnOp :: Not => { let llval = bx . not (operand . immediate ()) ; (OperandValue :: Immediate (llval) , operand . layout) } mir :: UnOp :: Neg => { let llval = if is_float { bx . fneg (operand . immediate ()) } else { bx . neg (operand . immediate ()) } ; (OperandValue :: Immediate (llval) , operand . layout) } mir :: UnOp :: PtrMetadata => { assert ! (operand . layout . ty . is_raw_ptr () || operand . layout . ty . is_ref () ,) ; let (_ , meta) = operand . val . pointer_parts () ; assert_eq ! (operand . layout . fields . count () > 1 , meta . is_some ()) ; if let Some (meta) = meta { (OperandValue :: Immediate (meta) , operand . layout . field (self . cx , 1)) } else { (OperandValue :: ZeroSized , bx . cx () . layout_of (bx . tcx () . types . unit)) } } } ; assert ! (val . is_expected_variant_for_type (self . cx , layout) , "Made wrong variant {val:?} for type {layout:?}" ,) ; OperandRef { val , layout } } mir :: Rvalue :: Discriminant (ref place) => { let discr_ty = rvalue . ty (self . mir , bx . tcx ()) ; let discr_ty = self . monomorphize (discr_ty) ; let operand = self . codegen_consume (bx , place . as_ref ()) ; let discr = operand . codegen_get_discr (self , bx , discr_ty) ; OperandRef { val : OperandValue :: Immediate (discr) , layout : self . cx . layout_of (discr_ty) , } } mir :: Rvalue :: NullaryOp (ref null_op , ty) => { let ty = self . monomorphize (ty) ; let layout = bx . cx () . layout_of (ty) ; let val = match null_op { mir :: NullOp :: SizeOf => { assert ! (bx . cx () . type_is_sized (ty)) ; let val = layout . size . bytes () ; bx . cx () . const_usize (val) } mir :: NullOp :: AlignOf => { assert ! (bx . cx () . type_is_sized (ty)) ; let val = layout . align . abi . bytes () ; bx . cx () . const_usize (val) } mir :: NullOp :: OffsetOf (fields) => { let val = bx . tcx () . offset_of_subfield (bx . typing_env () , layout , fields . iter ()) . bytes () ; bx . cx () . const_usize (val) } mir :: NullOp :: UbChecks => { let val = bx . tcx () . sess . ub_checks () ; bx . cx () . const_bool (val) } mir :: NullOp :: ContractChecks => { let val = bx . tcx () . sess . contract_checks () ; bx . cx () . const_bool (val) } } ; let tcx = self . cx . tcx () ; OperandRef { val : OperandValue :: Immediate (val) , layout : self . cx . layout_of (null_op . ty (tcx)) , } } mir :: Rvalue :: ThreadLocalRef (def_id) => { assert ! (bx . cx () . tcx () . is_static (def_id)) ; let layout = bx . layout_of (bx . cx () . tcx () . static_ptr_ty (def_id , bx . typing_env ())) ; let static_ = if ! def_id . is_local () && bx . cx () . tcx () . needs_thread_local_shim (def_id) { let instance = ty :: Instance { def : ty :: InstanceKind :: ThreadLocalShim (def_id) , args : ty :: GenericArgs :: empty () , } ; let fn_ptr = bx . get_fn_addr (instance) ; let fn_abi = bx . fn_abi_of_instance (instance , ty :: List :: empty ()) ; let fn_ty = bx . fn_decl_backend_type (fn_abi) ; let fn_attrs = if bx . tcx () . def_kind (instance . def_id ()) . has_codegen_attrs () { Some (bx . tcx () . codegen_instance_attrs (instance . def)) } else { None } ; bx . call (fn_ty , fn_attrs . as_deref () , Some (fn_abi) , fn_ptr , & [] , None , Some (instance) ,) } else { bx . get_static (def_id) } ; OperandRef { val : OperandValue :: Immediate (static_) , layout } } mir :: Rvalue :: Use (ref operand) => self . codegen_operand (bx , operand) , mir :: Rvalue :: Repeat (ref elem , len_const) => { let operand = self . codegen_operand (bx , elem) ; let array_ty = Ty :: new_array_with_const_len (bx . tcx () , operand . layout . ty , len_const) ; let array_ty = self . monomorphize (array_ty) ; let array_layout = bx . layout_of (array_ty) ; assert ! (array_layout . is_zst ()) ; OperandRef { val : OperandValue :: ZeroSized , layout : array_layout } } mir :: Rvalue :: Aggregate (ref kind , ref fields) => { let (variant_index , active_field_index) = match * * kind { mir :: AggregateKind :: Adt (_ , variant_index , _ , _ , active_field_index) => { (variant_index , active_field_index) } _ => (FIRST_VARIANT , None) , } ; let ty = rvalue . ty (self . mir , self . cx . tcx ()) ; let ty = self . monomorphize (ty) ; let layout = self . cx . layout_of (ty) ; let mut builder = OperandRefBuilder :: new (layout) ; for (field_idx , field) in fields . iter_enumerated () { let op = self . codegen_operand (bx , field) ; let fi = active_field_index . unwrap_or (field_idx) ; builder . insert_field (bx , variant_index , fi , op) ; } let tag_result = codegen_tag_value (self . cx , variant_index , layout) ; match tag_result { Err (super :: place :: UninhabitedVariantError) => { bx . abort () ; let val = OperandValue :: poison (bx , layout) ; OperandRef { val , layout } } Ok (maybe_tag_value) => { if let Some ((tag_field , tag_imm)) = maybe_tag_value { builder . insert_imm (tag_field , tag_imm) ; } builder . build (bx . cx ()) } } } mir :: Rvalue :: ShallowInitBox (ref operand , content_ty) => { let operand = self . codegen_operand (bx , operand) ; let val = operand . immediate () ; let content_ty = self . monomorphize (content_ty) ; let box_layout = bx . cx () . layout_of (Ty :: new_box (bx . tcx () , content_ty)) ; OperandRef { val : OperandValue :: Immediate (val) , layout : box_layout } } mir :: Rvalue :: WrapUnsafeBinder (ref operand , binder_ty) => { let operand = self . codegen_operand (bx , operand) ; let binder_ty = self . monomorphize (binder_ty) ; let layout = bx . cx () . layout_of (binder_ty) ; OperandRef { val : operand . val , layout } } } } fn evaluate_array_len (& mut self , bx : & mut Bx , place : mir :: Place < 'tcx >) -> Bx :: Value { if let Some (index) = place . as_local () && let LocalRef :: Operand (op) = self . locals [index] && let ty :: Array (_ , n) = op . layout . ty . kind () { let n = n . try_to_target_usize (bx . tcx ()) . expect ("expected monomorphic const in codegen") ; return bx . cx () . const_usize (n) ; } let cg_value = self . codegen_place (bx , place . as_ref ()) ; cg_value . len (bx . cx ()) } #[doc = " Codegen an `Rvalue::RawPtr` or `Rvalue::Ref`"] fn codegen_place_to_pointer (& mut self , bx : & mut Bx , place : mir :: Place < 'tcx > , mk_ptr_ty : impl FnOnce (TyCtxt < 'tcx > , Ty < 'tcx >) -> Ty < 'tcx > ,) -> OperandRef < 'tcx , Bx :: Value > { let cg_place = self . codegen_place (bx , place . as_ref ()) ; let val = cg_place . val . address () ; let ty = cg_place . layout . ty ; assert ! (if bx . cx () . tcx () . type_has_metadata (ty , bx . cx () . typing_env ()) { matches ! (val , OperandValue :: Pair (..)) } else { matches ! (val , OperandValue :: Immediate (..)) } , "Address of place was unexpectedly {val:?} for pointee type {ty:?}" ,) ; OperandRef { val , layout : self . cx . layout_of (mk_ptr_ty (self . cx . tcx () , ty)) } } fn codegen_scalar_binop (& mut self , bx : & mut Bx , op : mir :: BinOp , lhs : Bx :: Value , rhs : Bx :: Value , lhs_ty : Ty < 'tcx > , rhs_ty : Ty < 'tcx > ,) -> Bx :: Value { let is_float = lhs_ty . is_floating_point () ; let is_signed = lhs_ty . is_signed () ; match op { mir :: BinOp :: Add => { if is_float { bx . fadd (lhs , rhs) } else { bx . add (lhs , rhs) } } mir :: BinOp :: AddUnchecked => { if is_signed { bx . unchecked_sadd (lhs , rhs) } else { bx . unchecked_uadd (lhs , rhs) } } mir :: BinOp :: Sub => { if is_float { bx . fsub (lhs , rhs) } else { bx . sub (lhs , rhs) } } mir :: BinOp :: SubUnchecked => { if is_signed { bx . unchecked_ssub (lhs , rhs) } else { bx . unchecked_usub (lhs , rhs) } } mir :: BinOp :: Mul => { if is_float { bx . fmul (lhs , rhs) } else { bx . mul (lhs , rhs) } } mir :: BinOp :: MulUnchecked => { if is_signed { bx . unchecked_smul (lhs , rhs) } else { bx . unchecked_umul (lhs , rhs) } } mir :: BinOp :: Div => { if is_float { bx . fdiv (lhs , rhs) } else if is_signed { bx . sdiv (lhs , rhs) } else { bx . udiv (lhs , rhs) } } mir :: BinOp :: Rem => { if is_float { bx . frem (lhs , rhs) } else if is_signed { bx . srem (lhs , rhs) } else { bx . urem (lhs , rhs) } } mir :: BinOp :: BitOr => bx . or (lhs , rhs) , mir :: BinOp :: BitAnd => bx . and (lhs , rhs) , mir :: BinOp :: BitXor => bx . xor (lhs , rhs) , mir :: BinOp :: Offset => { let pointee_type = lhs_ty . builtin_deref (true) . unwrap_or_else (| | bug ! ("deref of non-pointer {:?}" , lhs_ty)) ; let pointee_layout = bx . cx () . layout_of (pointee_type) ; if pointee_layout . is_zst () { lhs } else { let llty = bx . cx () . backend_type (pointee_layout) ; if ! rhs_ty . is_signed () { bx . inbounds_nuw_gep (llty , lhs , & [rhs]) } else { bx . inbounds_gep (llty , lhs , & [rhs]) } } } mir :: BinOp :: Shl | mir :: BinOp :: ShlUnchecked => { let rhs = base :: build_shift_expr_rhs (bx , lhs , rhs , op == mir :: BinOp :: ShlUnchecked) ; bx . shl (lhs , rhs) } mir :: BinOp :: Shr | mir :: BinOp :: ShrUnchecked => { let rhs = base :: build_shift_expr_rhs (bx , lhs , rhs , op == mir :: BinOp :: ShrUnchecked) ; if is_signed { bx . ashr (lhs , rhs) } else { bx . lshr (lhs , rhs) } } mir :: BinOp :: Ne | mir :: BinOp :: Lt | mir :: BinOp :: Gt | mir :: BinOp :: Eq | mir :: BinOp :: Le | mir :: BinOp :: Ge => { if is_float { bx . fcmp (base :: bin_op_to_fcmp_predicate (op) , lhs , rhs) } else { bx . icmp (base :: bin_op_to_icmp_predicate (op , is_signed) , lhs , rhs) } } mir :: BinOp :: Cmp => { use std :: cmp :: Ordering ; assert ! (! is_float) ; if let Some (value) = bx . three_way_compare (lhs_ty , lhs , rhs) { return value ; } let pred = | op | base :: bin_op_to_icmp_predicate (op , is_signed) ; if bx . cx () . tcx () . sess . opts . optimize == OptLevel :: No { let is_gt = bx . icmp (pred (mir :: BinOp :: Gt) , lhs , rhs) ; let gtext = bx . zext (is_gt , bx . type_i8 ()) ; let is_lt = bx . icmp (pred (mir :: BinOp :: Lt) , lhs , rhs) ; let ltext = bx . zext (is_lt , bx . type_i8 ()) ; bx . unchecked_ssub (gtext , ltext) } else { let is_lt = bx . icmp (pred (mir :: BinOp :: Lt) , lhs , rhs) ; let is_ne = bx . icmp (pred (mir :: BinOp :: Ne) , lhs , rhs) ; let ge = bx . select (is_ne , bx . cx () . const_i8 (Ordering :: Greater as i8) , bx . cx () . const_i8 (Ordering :: Equal as i8) ,) ; bx . select (is_lt , bx . cx () . const_i8 (Ordering :: Less as i8) , ge) } } mir :: BinOp :: AddWithOverflow | mir :: BinOp :: SubWithOverflow | mir :: BinOp :: MulWithOverflow => { bug ! ("{op:?} needs to return a pair, so call codegen_scalar_checked_binop instead") } } } fn codegen_wide_ptr_binop (& mut self , bx : & mut Bx , op : mir :: BinOp , lhs_addr : Bx :: Value , lhs_extra : Bx :: Value , rhs_addr : Bx :: Value , rhs_extra : Bx :: Value , _input_ty : Ty < 'tcx > ,) -> Bx :: Value { match op { mir :: BinOp :: Eq => { let lhs = bx . icmp (IntPredicate :: IntEQ , lhs_addr , rhs_addr) ; let rhs = bx . icmp (IntPredicate :: IntEQ , lhs_extra , rhs_extra) ; bx . and (lhs , rhs) } mir :: BinOp :: Ne => { let lhs = bx . icmp (IntPredicate :: IntNE , lhs_addr , rhs_addr) ; let rhs = bx . icmp (IntPredicate :: IntNE , lhs_extra , rhs_extra) ; bx . or (lhs , rhs) } mir :: BinOp :: Le | mir :: BinOp :: Lt | mir :: BinOp :: Ge | mir :: BinOp :: Gt => { let (op , strict_op) = match op { mir :: BinOp :: Lt => (IntPredicate :: IntULT , IntPredicate :: IntULT) , mir :: BinOp :: Le => (IntPredicate :: IntULE , IntPredicate :: IntULT) , mir :: BinOp :: Gt => (IntPredicate :: IntUGT , IntPredicate :: IntUGT) , mir :: BinOp :: Ge => (IntPredicate :: IntUGE , IntPredicate :: IntUGT) , _ => bug ! () , } ; let lhs = bx . icmp (strict_op , lhs_addr , rhs_addr) ; let and_lhs = bx . icmp (IntPredicate :: IntEQ , lhs_addr , rhs_addr) ; let and_rhs = bx . icmp (op , lhs_extra , rhs_extra) ; let rhs = bx . and (and_lhs , and_rhs) ; bx . or (lhs , rhs) } _ => { bug ! ("unexpected wide ptr binop") ; } } } fn codegen_scalar_checked_binop (& mut self , bx : & mut Bx , op : mir :: BinOp , lhs : Bx :: Value , rhs : Bx :: Value , input_ty : Ty < 'tcx > ,) -> OperandValue < Bx :: Value > { let (val , of) = match op { mir :: BinOp :: Add | mir :: BinOp :: Sub | mir :: BinOp :: Mul => { let oop = match op { mir :: BinOp :: Add => OverflowOp :: Add , mir :: BinOp :: Sub => OverflowOp :: Sub , mir :: BinOp :: Mul => OverflowOp :: Mul , _ => unreachable ! () , } ; bx . checked_binop (oop , input_ty , lhs , rhs) } _ => bug ! ("Operator `{:?}` is not a checkable operator" , op) , } ; OperandValue :: Pair (val , of) } }}}

macro_rules! transmute_scalar_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function transmute_scalar in module {}", module_path!());
    };
}

mkfn!{
    transmute_scalar_introspect!();
    #[doc = " Transmutes a single scalar value `imm` from `from_scalar` to `to_scalar`."] #[doc = ""] #[doc = " This is expected to be in *immediate* form, as seen in [`OperandValue::Immediate`]"] #[doc = " or [`OperandValue::Pair`] (so `i1` for bools, not `i8`, for example)."] #[doc = ""] #[doc = " ICEs if the passed-in `imm` is not a value of the expected type for"] #[doc = " `from_scalar`, such as if it's a vector or a pair."] pub (super) fn transmute_scalar < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx , mut imm : Bx :: Value , from_scalar : abi :: Scalar , to_scalar : abi :: Scalar ,) -> Bx :: Value { assert_eq ! (from_scalar . size (bx . cx ()) , to_scalar . size (bx . cx ())) ; let imm_ty = bx . cx () . val_ty (imm) ; assert_ne ! (bx . cx () . type_kind (imm_ty) , TypeKind :: Vector , "Vector type {imm_ty:?} not allowed in transmute_scalar {from_scalar:?} -> {to_scalar:?}") ; if from_scalar == to_scalar { return imm ; } use abi :: Primitive :: * ; imm = bx . from_immediate (imm) ; let from_backend_ty = bx . cx () . type_from_scalar (from_scalar) ; debug_assert_eq ! (bx . cx () . val_ty (imm) , from_backend_ty) ; let to_backend_ty = bx . cx () . type_from_scalar (to_scalar) ; assume_scalar_range (bx , imm , from_scalar , from_backend_ty , Some (& to_scalar)) ; imm = match (from_scalar . primitive () , to_scalar . primitive ()) { (Int (..) | Float (_) , Int (..) | Float (_)) => bx . bitcast (imm , to_backend_ty) , (Pointer (..) , Pointer (..)) => bx . pointercast (imm , to_backend_ty) , (Int (..) , Pointer (..)) => bx . inttoptr (imm , to_backend_ty) , (Pointer (..) , Int (..)) => { bx . ptrtoint (imm , to_backend_ty) } (Float (_) , Pointer (..)) => { let int_imm = bx . bitcast (imm , bx . cx () . type_isize ()) ; bx . inttoptr (int_imm , to_backend_ty) } (Pointer (..) , Float (_)) => { let int_imm = bx . ptrtoint (imm , bx . cx () . type_isize ()) ; bx . bitcast (int_imm , to_backend_ty) } } ; debug_assert_eq ! (bx . cx () . val_ty (imm) , to_backend_ty) ; assume_scalar_range (bx , imm , to_scalar , to_backend_ty , Some (& from_scalar)) ; imm = bx . to_immediate_scalar (imm , to_scalar) ; imm }
}

macro_rules! assume_scalar_range_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function assume_scalar_range in module {}", module_path!());
    };
}

mkfn!{
    assume_scalar_range_introspect!();
    #[doc = " Emits an `assume` call that `imm`'s value is within the known range of `scalar`."] #[doc = ""] #[doc = " If `known` is `Some`, only emits the assume if it's more specific than"] #[doc = " whatever is already known from the range of *that* scalar."] fn assume_scalar_range < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx , imm : Bx :: Value , scalar : abi :: Scalar , backend_ty : Bx :: Type , known : Option < & abi :: Scalar > ,) { if matches ! (bx . cx () . sess () . opts . optimize , OptLevel :: No) { return ; } match (scalar , known) { (abi :: Scalar :: Union { .. } , _) => return , (_ , None) => { if scalar . is_always_valid (bx . cx ()) { return ; } } (abi :: Scalar :: Initialized { valid_range , .. } , Some (known)) => { let known_range = known . valid_range (bx . cx ()) ; if valid_range . contains_range (known_range , scalar . size (bx . cx ())) { return ; } } } match scalar . primitive () { abi :: Primitive :: Int (..) => { let range = scalar . valid_range (bx . cx ()) ; bx . assume_integer_range (imm , backend_ty , range) ; } abi :: Primitive :: Pointer (abi :: AddressSpace :: ZERO) if ! scalar . valid_range (bx . cx ()) . contains (0) => { bx . assume_nonnull (imm) ; } abi :: Primitive :: Pointer (..) | abi :: Primitive :: Float (..) => { } } }
}