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
mkuse!{use std :: assert_matches :: assert_matches ;}
mkuse!{use std :: ops :: Deref ;}
mkuse!{use rustc_abi :: { Align , Scalar , Size , WrappingRange } ;}
mkuse!{use rustc_middle :: middle :: codegen_fn_attrs :: CodegenFnAttrs ;}
mkuse!{use rustc_middle :: ty :: layout :: { FnAbiOf , LayoutOf , TyAndLayout } ;}
mkuse!{use rustc_middle :: ty :: { AtomicOrdering , Instance , Ty } ;}
mkuse!{use rustc_session :: config :: OptLevel ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use rustc_target :: callconv :: FnAbi ;}
mkuse!{use super :: abi :: AbiBuilderMethods ;}
mkuse!{use super :: asm :: AsmBuilderMethods ;}
mkuse!{use super :: consts :: ConstCodegenMethods ;}
mkuse!{use super :: coverageinfo :: CoverageInfoBuilderMethods ;}
mkuse!{use super :: debuginfo :: DebugInfoBuilderMethods ;}
mkuse!{use super :: intrinsic :: IntrinsicCallBuilderMethods ;}
mkuse!{use super :: misc :: MiscCodegenMethods ;}
mkuse!{use super :: type_ :: { ArgAbiBuilderMethods , BaseTypeCodegenMethods , LayoutTypeCodegenMethods } ;}
mkuse!{use super :: { CodegenMethods , StaticBuilderMethods } ;}
mkuse!{use crate :: MemFlags ;}
mkuse!{use crate :: common :: { AtomicRmwBinOp , IntPredicate , RealPredicate , SynchronizationScope , TypeKind } ;}
mkuse!{use crate :: mir :: operand :: { OperandRef , OperandValue } ;}
mkuse!{use crate :: mir :: place :: { PlaceRef , PlaceValue } ;}
mkitem!{mkenum!{# [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum OverflowOp { Add , Sub , Mul , }}}
mkitem!{mktrait!{pub trait BuilderMethods < 'a , 'tcx > : Sized + LayoutOf < 'tcx , LayoutOfResult = TyAndLayout < 'tcx > > + FnAbiOf < 'tcx , FnAbiOfResult = & 'tcx FnAbi < 'tcx , Ty < 'tcx > > > + Deref < Target = Self :: CodegenCx > + CoverageInfoBuilderMethods < 'tcx > + DebugInfoBuilderMethods + ArgAbiBuilderMethods < 'tcx > + AbiBuilderMethods + IntrinsicCallBuilderMethods < 'tcx > + AsmBuilderMethods < 'tcx > + StaticBuilderMethods { type CodegenCx : CodegenMethods < 'tcx , Value = Self :: Value , Metadata = Self :: Metadata , Function = Self :: Function , BasicBlock = Self :: BasicBlock , Type = Self :: Type , Funclet = Self :: Funclet , DIScope = Self :: DIScope , DILocation = Self :: DILocation , DIVariable = Self :: DIVariable , > ; fn build (cx : & 'a Self :: CodegenCx , llbb : Self :: BasicBlock) -> Self ; fn cx (& self) -> & Self :: CodegenCx ; fn llbb (& self) -> Self :: BasicBlock ; fn set_span (& mut self , span : Span) ; fn append_block (cx : & 'a Self :: CodegenCx , llfn : Self :: Function , name : & str) -> Self :: BasicBlock ; fn append_sibling_block (& mut self , name : & str) -> Self :: BasicBlock ; fn switch_to_block (& mut self , llbb : Self :: BasicBlock) ; fn ret_void (& mut self) ; fn ret (& mut self , v : Self :: Value) ; fn br (& mut self , dest : Self :: BasicBlock) ; fn cond_br (& mut self , cond : Self :: Value , then_llbb : Self :: BasicBlock , else_llbb : Self :: BasicBlock ,) ; fn cond_br_with_expect (& mut self , mut cond : Self :: Value , then_llbb : Self :: BasicBlock , else_llbb : Self :: BasicBlock , expect : Option < bool > ,) { if let Some (expect) = expect { cond = self . expect (cond , expect) ; } self . cond_br (cond , then_llbb , else_llbb) } fn switch (& mut self , v : Self :: Value , else_llbb : Self :: BasicBlock , cases : impl ExactSizeIterator < Item = (u128 , Self :: BasicBlock) > ,) ; fn switch_with_weights (& mut self , v : Self :: Value , else_llbb : Self :: BasicBlock , _else_is_cold : bool , cases : impl ExactSizeIterator < Item = (u128 , Self :: BasicBlock , bool) > ,) { self . switch (v , else_llbb , cases . map (| (val , bb , _) | (val , bb))) } fn invoke (& mut self , llty : Self :: Type , fn_attrs : Option < & CodegenFnAttrs > , fn_abi : Option < & FnAbi < 'tcx , Ty < 'tcx > > > , llfn : Self :: Value , args : & [Self :: Value] , then : Self :: BasicBlock , catch : Self :: BasicBlock , funclet : Option < & Self :: Funclet > , instance : Option < Instance < 'tcx > > ,) -> Self :: Value ; fn unreachable (& mut self) ; # [doc = " Like [`Self::unreachable`], but for use in the middle of a basic block."] fn unreachable_nonterminator (& mut self) { let const_true = self . cx () . const_bool (true) ; let poison_ptr = self . const_poison (self . cx () . type_ptr ()) ; self . store (const_true , poison_ptr , Align :: ONE) ; } fn add (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn fadd (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn fadd_fast (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn fadd_algebraic (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn sub (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn fsub (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn fsub_fast (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn fsub_algebraic (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn mul (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn fmul (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn fmul_fast (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn fmul_algebraic (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn udiv (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn exactudiv (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn sdiv (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn exactsdiv (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn fdiv (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn fdiv_fast (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn fdiv_algebraic (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn urem (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn srem (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn frem (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn frem_fast (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn frem_algebraic (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; # [doc = " Generate a left-shift. Both operands must have the same size. The right operand must be"] # [doc = " interpreted as unsigned and can be assumed to be less than the size of the left operand."] fn shl (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; # [doc = " Generate a logical right-shift. Both operands must have the same size. The right operand"] # [doc = " must be interpreted as unsigned and can be assumed to be less than the size of the left"] # [doc = " operand."] fn lshr (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; # [doc = " Generate an arithmetic right-shift. Both operands must have the same size. The right operand"] # [doc = " must be interpreted as unsigned and can be assumed to be less than the size of the left"] # [doc = " operand."] fn ashr (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn unchecked_sadd (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value { self . add (lhs , rhs) } fn unchecked_uadd (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value { self . add (lhs , rhs) } fn unchecked_suadd (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value { self . unchecked_sadd (lhs , rhs) } fn unchecked_ssub (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value { self . sub (lhs , rhs) } fn unchecked_usub (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value { self . sub (lhs , rhs) } fn unchecked_susub (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value { self . unchecked_ssub (lhs , rhs) } fn unchecked_smul (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value { self . mul (lhs , rhs) } fn unchecked_umul (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value { self . mul (lhs , rhs) } fn unchecked_sumul (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value { self . unchecked_smul (lhs , rhs) } fn and (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn or (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; # [doc = " Defaults to [`Self::or`], but guarantees `(lhs & rhs) == 0` so some backends"] # [doc = " can emit something more helpful for optimizations."] fn or_disjoint (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value { self . or (lhs , rhs) } fn xor (& mut self , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn neg (& mut self , v : Self :: Value) -> Self :: Value ; fn fneg (& mut self , v : Self :: Value) -> Self :: Value ; fn not (& mut self , v : Self :: Value) -> Self :: Value ; fn checked_binop (& mut self , oop : OverflowOp , ty : Ty < 'tcx > , lhs : Self :: Value , rhs : Self :: Value ,) -> (Self :: Value , Self :: Value) ; fn from_immediate (& mut self , val : Self :: Value) -> Self :: Value ; fn to_immediate_scalar (& mut self , val : Self :: Value , scalar : Scalar) -> Self :: Value ; fn alloca (& mut self , size : Size , align : Align) -> Self :: Value ; fn load (& mut self , ty : Self :: Type , ptr : Self :: Value , align : Align) -> Self :: Value ; fn volatile_load (& mut self , ty : Self :: Type , ptr : Self :: Value) -> Self :: Value ; fn atomic_load (& mut self , ty : Self :: Type , ptr : Self :: Value , order : AtomicOrdering , size : Size ,) -> Self :: Value ; fn load_from_place (& mut self , ty : Self :: Type , place : PlaceValue < Self :: Value >) -> Self :: Value { assert_eq ! (place . llextra , None) ; self . load (ty , place . llval , place . align) } fn load_operand (& mut self , place : PlaceRef < 'tcx , Self :: Value >) -> OperandRef < 'tcx , Self :: Value > ; # [doc = " Called for Rvalue::Repeat when the elem is neither a ZST nor optimizable using memset."] fn write_operand_repeatedly (& mut self , elem : OperandRef < 'tcx , Self :: Value > , count : u64 , dest : PlaceRef < 'tcx , Self :: Value > ,) ; # [doc = " Emits an `assume` that the integer value `imm` of type `ty` is contained in `range`."] # [doc = ""] # [doc = " This *always* emits the assumption, so you probably want to check the"] # [doc = " optimization level and `Scalar::is_always_valid` before calling it."] fn assume_integer_range (& mut self , imm : Self :: Value , ty : Self :: Type , range : WrappingRange) { let WrappingRange { start , end } = range ; let shifted = if start == 0 { imm } else { let low = self . const_uint_big (ty , start) ; self . sub (imm , low) } ; let width = self . const_uint_big (ty , u128 :: wrapping_sub (end , start)) ; let cmp = self . icmp (IntPredicate :: IntULE , shifted , width) ; self . assume (cmp) ; } # [doc = " Emits an `assume` that the `val` of pointer type is non-null."] # [doc = ""] # [doc = " You may want to check the optimization level before bothering calling this."] fn assume_nonnull (& mut self , val : Self :: Value) { let null = self . const_null (self . type_ptr ()) ; let is_null = self . icmp (IntPredicate :: IntNE , val , null) ; self . assume (is_null) ; } fn range_metadata (& mut self , load : Self :: Value , range : WrappingRange) ; fn nonnull_metadata (& mut self , load : Self :: Value) ; fn store (& mut self , val : Self :: Value , ptr : Self :: Value , align : Align) -> Self :: Value ; fn store_to_place (& mut self , val : Self :: Value , place : PlaceValue < Self :: Value >) -> Self :: Value { assert_eq ! (place . llextra , None) ; self . store (val , place . llval , place . align) } fn store_with_flags (& mut self , val : Self :: Value , ptr : Self :: Value , align : Align , flags : MemFlags ,) -> Self :: Value ; fn store_to_place_with_flags (& mut self , val : Self :: Value , place : PlaceValue < Self :: Value > , flags : MemFlags ,) -> Self :: Value { assert_eq ! (place . llextra , None) ; self . store_with_flags (val , place . llval , place . align , flags) } fn atomic_store (& mut self , val : Self :: Value , ptr : Self :: Value , order : AtomicOrdering , size : Size ,) ; fn gep (& mut self , ty : Self :: Type , ptr : Self :: Value , indices : & [Self :: Value]) -> Self :: Value ; fn inbounds_gep (& mut self , ty : Self :: Type , ptr : Self :: Value , indices : & [Self :: Value] ,) -> Self :: Value ; fn inbounds_nuw_gep (& mut self , ty : Self :: Type , ptr : Self :: Value , indices : & [Self :: Value] ,) -> Self :: Value { self . inbounds_gep (ty , ptr , indices) } fn ptradd (& mut self , ptr : Self :: Value , offset : Self :: Value) -> Self :: Value { self . gep (self . cx () . type_i8 () , ptr , & [offset]) } fn inbounds_ptradd (& mut self , ptr : Self :: Value , offset : Self :: Value) -> Self :: Value { self . inbounds_gep (self . cx () . type_i8 () , ptr , & [offset]) } fn trunc (& mut self , val : Self :: Value , dest_ty : Self :: Type) -> Self :: Value ; # [doc = " Produces the same value as [`Self::trunc`] (and defaults to that),"] # [doc = " but is UB unless the *zero*-extending the result can reproduce `val`."] fn unchecked_utrunc (& mut self , val : Self :: Value , dest_ty : Self :: Type) -> Self :: Value { self . trunc (val , dest_ty) } # [doc = " Produces the same value as [`Self::trunc`] (and defaults to that),"] # [doc = " but is UB unless the *sign*-extending the result can reproduce `val`."] fn unchecked_strunc (& mut self , val : Self :: Value , dest_ty : Self :: Type) -> Self :: Value { self . trunc (val , dest_ty) } fn sext (& mut self , val : Self :: Value , dest_ty : Self :: Type) -> Self :: Value ; fn fptoui_sat (& mut self , val : Self :: Value , dest_ty : Self :: Type) -> Self :: Value ; fn fptosi_sat (& mut self , val : Self :: Value , dest_ty : Self :: Type) -> Self :: Value ; fn fptoui (& mut self , val : Self :: Value , dest_ty : Self :: Type) -> Self :: Value ; fn fptosi (& mut self , val : Self :: Value , dest_ty : Self :: Type) -> Self :: Value ; fn uitofp (& mut self , val : Self :: Value , dest_ty : Self :: Type) -> Self :: Value ; fn sitofp (& mut self , val : Self :: Value , dest_ty : Self :: Type) -> Self :: Value ; fn fptrunc (& mut self , val : Self :: Value , dest_ty : Self :: Type) -> Self :: Value ; fn fpext (& mut self , val : Self :: Value , dest_ty : Self :: Type) -> Self :: Value ; fn ptrtoint (& mut self , val : Self :: Value , dest_ty : Self :: Type) -> Self :: Value ; fn inttoptr (& mut self , val : Self :: Value , dest_ty : Self :: Type) -> Self :: Value ; fn bitcast (& mut self , val : Self :: Value , dest_ty : Self :: Type) -> Self :: Value ; fn intcast (& mut self , val : Self :: Value , dest_ty : Self :: Type , is_signed : bool) -> Self :: Value ; fn pointercast (& mut self , val : Self :: Value , dest_ty : Self :: Type) -> Self :: Value ; fn cast_float_to_int (& mut self , signed : bool , x : Self :: Value , dest_ty : Self :: Type ,) -> Self :: Value { let in_ty = self . cx () . val_ty (x) ; let (float_ty , int_ty) = if self . cx () . type_kind (dest_ty) == TypeKind :: Vector && self . cx () . type_kind (in_ty) == TypeKind :: Vector { (self . cx () . element_type (in_ty) , self . cx () . element_type (dest_ty)) } else { (in_ty , dest_ty) } ; assert_matches ! (self . cx () . type_kind (float_ty) , TypeKind :: Half | TypeKind :: Float | TypeKind :: Double | TypeKind :: FP128) ; assert_eq ! (self . cx () . type_kind (int_ty) , TypeKind :: Integer) ; if let Some (false) = self . cx () . sess () . opts . unstable_opts . saturating_float_casts { return if signed { self . fptosi (x , dest_ty) } else { self . fptoui (x , dest_ty) } ; } if signed { self . fptosi_sat (x , dest_ty) } else { self . fptoui_sat (x , dest_ty) } } fn icmp (& mut self , op : IntPredicate , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; fn fcmp (& mut self , op : RealPredicate , lhs : Self :: Value , rhs : Self :: Value) -> Self :: Value ; # [doc = " Returns `-1` if `lhs < rhs`, `0` if `lhs == rhs`, and `1` if `lhs > rhs`."] fn three_way_compare (& mut self , _ty : Ty < 'tcx > , _lhs : Self :: Value , _rhs : Self :: Value ,) -> Option < Self :: Value > { None } fn memcpy (& mut self , dst : Self :: Value , dst_align : Align , src : Self :: Value , src_align : Align , size : Self :: Value , flags : MemFlags ,) ; fn memmove (& mut self , dst : Self :: Value , dst_align : Align , src : Self :: Value , src_align : Align , size : Self :: Value , flags : MemFlags ,) ; fn memset (& mut self , ptr : Self :: Value , fill_byte : Self :: Value , size : Self :: Value , align : Align , flags : MemFlags ,) ; # [doc = " *Typed* copy for non-overlapping places."] # [doc = ""] # [doc = " Has a default implementation in terms of `memcpy`, but specific backends"] # [doc = " can override to do something smarter if possible."] # [doc = ""] # [doc = " (For example, typed load-stores with alias metadata.)"] fn typed_place_copy (& mut self , dst : PlaceValue < Self :: Value > , src : PlaceValue < Self :: Value > , layout : TyAndLayout < 'tcx > ,) { self . typed_place_copy_with_flags (dst , src , layout , MemFlags :: empty ()) ; } fn typed_place_copy_with_flags (& mut self , dst : PlaceValue < Self :: Value > , src : PlaceValue < Self :: Value > , layout : TyAndLayout < 'tcx > , flags : MemFlags ,) { assert ! (layout . is_sized () , "cannot typed-copy an unsigned type") ; assert ! (src . llextra . is_none () , "cannot directly copy from unsized values") ; assert ! (dst . llextra . is_none () , "cannot directly copy into unsized values") ; if flags . contains (MemFlags :: NONTEMPORAL) { let ty = self . backend_type (layout) ; let val = self . load_from_place (ty , src) ; self . store_to_place_with_flags (val , dst , flags) ; } else if self . sess () . opts . optimize == OptLevel :: No && self . is_backend_immediate (layout) { let temp = self . load_operand (src . with_type (layout)) ; temp . val . store_with_flags (self , dst . with_type (layout) , flags) ; } else if ! layout . is_zst () { let bytes = self . const_usize (layout . size . bytes ()) ; self . memcpy (dst . llval , dst . align , src . llval , src . align , bytes , flags) ; } } # [doc = " *Typed* swap for non-overlapping places."] # [doc = ""] # [doc = " Avoids `alloca`s for Immediates and ScalarPairs."] # [doc = ""] # [doc = " FIXME: Maybe do something smarter for Ref types too?"] # [doc = " For now, the `typed_swap_nonoverlapping` intrinsic just doesn't call this for those"] # [doc = " cases (in non-debug), preferring the fallback body instead."] fn typed_place_swap (& mut self , left : PlaceValue < Self :: Value > , right : PlaceValue < Self :: Value > , layout : TyAndLayout < 'tcx > ,) { let mut temp = self . load_operand (left . with_type (layout)) ; if let OperandValue :: Ref (..) = temp . val { let alloca = PlaceRef :: alloca (self , layout) ; self . typed_place_copy (alloca . val , left , layout) ; temp = self . load_operand (alloca) ; } self . typed_place_copy (left , right , layout) ; temp . val . store (self , right . with_type (layout)) ; } fn select (& mut self , cond : Self :: Value , then_val : Self :: Value , else_val : Self :: Value ,) -> Self :: Value ; fn va_arg (& mut self , list : Self :: Value , ty : Self :: Type) -> Self :: Value ; fn extract_element (& mut self , vec : Self :: Value , idx : Self :: Value) -> Self :: Value ; fn vector_splat (& mut self , num_elts : usize , elt : Self :: Value) -> Self :: Value ; fn extract_value (& mut self , agg_val : Self :: Value , idx : u64) -> Self :: Value ; fn insert_value (& mut self , agg_val : Self :: Value , elt : Self :: Value , idx : u64) -> Self :: Value ; fn set_personality_fn (& mut self , personality : Self :: Function) ; fn cleanup_landing_pad (& mut self , pers_fn : Self :: Function) -> (Self :: Value , Self :: Value) ; fn filter_landing_pad (& mut self , pers_fn : Self :: Function) ; fn resume (& mut self , exn0 : Self :: Value , exn1 : Self :: Value) ; fn cleanup_pad (& mut self , parent : Option < Self :: Value > , args : & [Self :: Value]) -> Self :: Funclet ; fn cleanup_ret (& mut self , funclet : & Self :: Funclet , unwind : Option < Self :: BasicBlock >) ; fn catch_pad (& mut self , parent : Self :: Value , args : & [Self :: Value]) -> Self :: Funclet ; fn catch_switch (& mut self , parent : Option < Self :: Value > , unwind : Option < Self :: BasicBlock > , handlers : & [Self :: BasicBlock] ,) -> Self :: Value ; fn atomic_cmpxchg (& mut self , dst : Self :: Value , cmp : Self :: Value , src : Self :: Value , order : AtomicOrdering , failure_order : AtomicOrdering , weak : bool ,) -> (Self :: Value , Self :: Value) ; # [doc = " `ret_ptr` indicates whether the return type (which is also the type `dst` points to)"] # [doc = " is a pointer or the same type as `src`."] fn atomic_rmw (& mut self , op : AtomicRmwBinOp , dst : Self :: Value , src : Self :: Value , order : AtomicOrdering , ret_ptr : bool ,) -> Self :: Value ; fn atomic_fence (& mut self , order : AtomicOrdering , scope : SynchronizationScope) ; fn set_invariant_load (& mut self , load : Self :: Value) ; # [doc = " Called for `StorageLive`"] fn lifetime_start (& mut self , ptr : Self :: Value , size : Size) ; # [doc = " Called for `StorageDead`"] fn lifetime_end (& mut self , ptr : Self :: Value , size : Size) ; # [doc = " \"Finally codegen the call\""] # [doc = ""] # [doc = " ## Arguments"] # [doc = ""] # [doc = " The `fn_attrs`, `fn_abi`, and `instance` arguments are Options because they are advisory."] # [doc = " They relate to optional codegen enhancements like LLVM CFI, and do not affect ABI per se."] # [doc = " Any ABI-related transformations should be handled by different, earlier stages of codegen."] # [doc = " For instance, in the caller of `BuilderMethods::call`."] # [doc = ""] # [doc = " This means that a codegen backend which disregards `fn_attrs`, `fn_abi`, and `instance`"] # [doc = " should still do correct codegen, and code should not be miscompiled if they are omitted."] # [doc = " It is not a miscompilation in this sense if it fails to run under CFI, other sanitizers, or"] # [doc = " in the context of other compiler-enhanced security features."] # [doc = ""] # [doc = " The typical case that they are None is during the codegen of intrinsics and lang-items,"] # [doc = " as those are \"fake functions\" with only a trivial ABI if any, et cetera."] # [doc = ""] # [doc = " ## Return"] # [doc = ""] # [doc = " Must return the value the function will return so it can be written to the destination,"] # [doc = " assuming the function does not explicitly pass the destination as a pointer in `args`."] fn call (& mut self , llty : Self :: Type , fn_attrs : Option < & CodegenFnAttrs > , fn_abi : Option < & FnAbi < 'tcx , Ty < 'tcx > > > , fn_val : Self :: Value , args : & [Self :: Value] , funclet : Option < & Self :: Funclet > , instance : Option < Instance < 'tcx > > ,) -> Self :: Value ; fn tail_call (& mut self , llty : Self :: Type , fn_attrs : Option < & CodegenFnAttrs > , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > > , llfn : Self :: Value , args : & [Self :: Value] , funclet : Option < & Self :: Funclet > , instance : Option < Instance < 'tcx > > ,) ; fn zext (& mut self , val : Self :: Value , dest_ty : Self :: Type) -> Self :: Value ; fn apply_attrs_to_cleanup_callsite (& mut self , llret : Self :: Value) ; }}}