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
mkuse!{use rustc_abi :: { BackendRepr , Float , Integer , Primitive , Scalar } ;}
mkuse!{use rustc_ast :: { InlineAsmOptions , InlineAsmTemplatePiece } ;}
mkuse!{use rustc_codegen_ssa :: mir :: operand :: OperandValue ;}
mkuse!{use rustc_codegen_ssa :: traits :: * ;}
mkuse!{use rustc_data_structures :: fx :: FxHashMap ;}
mkuse!{use rustc_middle :: ty :: Instance ;}
mkuse!{use rustc_middle :: ty :: layout :: TyAndLayout ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_span :: { Pos , Span , Symbol , sym } ;}
mkuse!{use rustc_target :: asm :: * ;}
mkuse!{use smallvec :: SmallVec ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: builder :: Builder ;}
mkuse!{use crate :: common :: Funclet ;}
mkuse!{use crate :: context :: CodegenCx ;}
mkuse!{use crate :: llvm :: ToLlvmBool ;}
mkuse!{use crate :: type_ :: Type ;}
mkuse!{use crate :: type_of :: LayoutLlvmExt ;}
mkuse!{use crate :: value :: Value ;}
mkuse!{use crate :: { attributes , llvm } ;}
mkitem!{mkimpl!{impl < 'll , 'tcx > AsmBuilderMethods < 'tcx > for Builder < '_ , 'll , 'tcx > { fn codegen_inline_asm (& mut self , template : & [InlineAsmTemplatePiece] , operands : & [InlineAsmOperandRef < 'tcx , Self >] , options : InlineAsmOptions , line_spans : & [Span] , instance : Instance < '_ > , dest : Option < Self :: BasicBlock > , catch_funclet : Option < (Self :: BasicBlock , Option < & Self :: Funclet >) > ,) { let asm_arch = self . tcx . sess . asm_arch . unwrap () ; let mut constraints = vec ! [] ; let mut clobbers = vec ! [] ; let mut output_types = vec ! [] ; let mut op_idx = FxHashMap :: default () ; let mut clobbered_x87 = false ; for (idx , op) in operands . iter () . enumerate () { match * op { InlineAsmOperandRef :: Out { reg , late , place } => { let is_target_supported = | reg_class : InlineAsmRegClass | { for & (_ , feature) in reg_class . supported_types (asm_arch , true) { if let Some (feature) = feature { if self . tcx . asm_target_features (instance . def_id ()) . contains (& feature) { return true ; } } else { return true ; } } false } ; let mut layout = None ; let ty = if let Some (ref place) = place { layout = Some (& place . layout) ; llvm_fixup_output_type (self . cx , reg . reg_class () , & place . layout , instance) } else if matches ! (reg . reg_class () , InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: mmx_reg | X86InlineAsmRegClass :: x87_reg)) { if ! clobbered_x87 { clobbered_x87 = true ; clobbers . push ("~{st}" . to_string ()) ; for i in 1 ..= 7 { clobbers . push (format ! ("~{{st({})}}" , i)) ; } } continue ; } else if ! is_target_supported (reg . reg_class ()) || reg . reg_class () . is_clobber_only (asm_arch , true) { assert_matches ! (reg , InlineAsmRegOrRegClass :: Reg (_)) ; clobbers . push (format ! ("~{}" , reg_to_llvm (reg , None))) ; continue ; } else { dummy_output_type (self . cx , reg . reg_class ()) } ; output_types . push (ty) ; op_idx . insert (idx , constraints . len ()) ; let prefix = if late { "=" } else { "=&" } ; constraints . push (format ! ("{}{}" , prefix , reg_to_llvm (reg , layout))) ; } InlineAsmOperandRef :: InOut { reg , late , in_value , out_place } => { let layout = if let Some (ref out_place) = out_place { & out_place . layout } else { & in_value . layout } ; let ty = llvm_fixup_output_type (self . cx , reg . reg_class () , layout , instance) ; output_types . push (ty) ; op_idx . insert (idx , constraints . len ()) ; let prefix = if late { "=" } else { "=&" } ; constraints . push (format ! ("{}{}" , prefix , reg_to_llvm (reg , Some (layout)))) ; } _ => { } } } let mut inputs = vec ! [] ; for (idx , op) in operands . iter () . enumerate () { match * op { InlineAsmOperandRef :: In { reg , value } => { let llval = llvm_fixup_input (self , value . immediate () , reg . reg_class () , & value . layout , instance ,) ; inputs . push (llval) ; op_idx . insert (idx , constraints . len ()) ; constraints . push (reg_to_llvm (reg , Some (& value . layout))) ; } InlineAsmOperandRef :: InOut { reg , late , in_value , out_place : _ } => { let value = llvm_fixup_input (self , in_value . immediate () , reg . reg_class () , & in_value . layout , instance ,) ; inputs . push (value) ; if late && matches ! (reg , InlineAsmRegOrRegClass :: Reg (_)) { constraints . push (reg_to_llvm (reg , Some (& in_value . layout))) ; } else { constraints . push (format ! ("{}" , op_idx [& idx])) ; } } InlineAsmOperandRef :: SymFn { instance } => { inputs . push (self . cx . get_fn (instance)) ; op_idx . insert (idx , constraints . len ()) ; constraints . push ("s" . to_string ()) ; } InlineAsmOperandRef :: SymStatic { def_id } => { inputs . push (self . cx . get_static (def_id)) ; op_idx . insert (idx , constraints . len ()) ; constraints . push ("s" . to_string ()) ; } _ => { } } } let mut labels = vec ! [] ; let mut template_str = String :: new () ; for piece in template { match * piece { InlineAsmTemplatePiece :: String (ref s) => { if s . contains ('$') { for c in s . chars () { if c == '$' { template_str . push_str ("$$") ; } else { template_str . push (c) ; } } } else { template_str . push_str (s) } } InlineAsmTemplatePiece :: Placeholder { operand_idx , modifier , span : _ } => { match operands [operand_idx] { InlineAsmOperandRef :: In { reg , .. } | InlineAsmOperandRef :: Out { reg , .. } | InlineAsmOperandRef :: InOut { reg , .. } => { let modifier = modifier_to_llvm (asm_arch , reg . reg_class () , modifier) ; if let Some (modifier) = modifier { template_str . push_str (& format ! ("${{{}:{}}}" , op_idx [& operand_idx] , modifier)) ; } else { template_str . push_str (& format ! ("${{{}}}" , op_idx [& operand_idx])) ; } } InlineAsmOperandRef :: Const { ref string } => { template_str . push_str (string) ; } InlineAsmOperandRef :: SymFn { .. } | InlineAsmOperandRef :: SymStatic { .. } => { template_str . push_str (& format ! ("${{{}:c}}" , op_idx [& operand_idx])) ; } InlineAsmOperandRef :: Label { label } => { template_str . push_str (& format ! ("${{{}:l}}" , constraints . len ())) ; constraints . push ("!i" . to_owned ()) ; labels . push (label) ; } } } } } constraints . append (& mut clobbers) ; if ! options . contains (InlineAsmOptions :: PRESERVES_FLAGS) { match asm_arch { InlineAsmArch :: AArch64 | InlineAsmArch :: Arm64EC | InlineAsmArch :: Arm => { constraints . push ("~{cc}" . to_string ()) ; } InlineAsmArch :: X86 | InlineAsmArch :: X86_64 => { constraints . extend_from_slice (& ["~{dirflag}" . to_string () , "~{fpsr}" . to_string () , "~{flags}" . to_string () ,]) ; } InlineAsmArch :: RiscV32 | InlineAsmArch :: RiscV64 => { constraints . extend_from_slice (& ["~{vtype}" . to_string () , "~{vl}" . to_string () , "~{vxsat}" . to_string () , "~{vxrm}" . to_string () ,]) ; } InlineAsmArch :: Avr => { constraints . push ("~{sreg}" . to_string ()) ; } InlineAsmArch :: Nvptx64 => { } InlineAsmArch :: PowerPC | InlineAsmArch :: PowerPC64 => { } InlineAsmArch :: Hexagon => { } InlineAsmArch :: LoongArch32 | InlineAsmArch :: LoongArch64 => { constraints . extend_from_slice (& ["~{$fcc0}" . to_string () , "~{$fcc1}" . to_string () , "~{$fcc2}" . to_string () , "~{$fcc3}" . to_string () , "~{$fcc4}" . to_string () , "~{$fcc5}" . to_string () , "~{$fcc6}" . to_string () , "~{$fcc7}" . to_string () ,]) ; } InlineAsmArch :: Mips | InlineAsmArch :: Mips64 => { } InlineAsmArch :: S390x => { constraints . push ("~{cc}" . to_string ()) ; } InlineAsmArch :: Sparc | InlineAsmArch :: Sparc64 => { constraints . push ("~{icc}" . to_string ()) ; constraints . push ("~{fcc0}" . to_string ()) ; constraints . push ("~{fcc1}" . to_string ()) ; constraints . push ("~{fcc2}" . to_string ()) ; constraints . push ("~{fcc3}" . to_string ()) ; } InlineAsmArch :: SpirV => { } InlineAsmArch :: Wasm32 | InlineAsmArch :: Wasm64 => { } InlineAsmArch :: Bpf => { } InlineAsmArch :: Msp430 => { constraints . push ("~{sr}" . to_string ()) ; } InlineAsmArch :: M68k => { constraints . push ("~{ccr}" . to_string ()) ; } InlineAsmArch :: CSKY => { constraints . push ("~{psr}" . to_string ()) ; } } } if ! options . contains (InlineAsmOptions :: NOMEM) { constraints . push ("~{memory}" . to_string ()) ; } let volatile = ! options . contains (InlineAsmOptions :: PURE) ; let alignstack = ! options . contains (InlineAsmOptions :: NOSTACK) ; let output_type = match & output_types [..] { [] => self . type_void () , [ty] => ty , tys => self . type_struct (tys , false) , } ; let dialect = match asm_arch { InlineAsmArch :: X86 | InlineAsmArch :: X86_64 if ! options . contains (InlineAsmOptions :: ATT_SYNTAX) => { llvm :: AsmDialect :: Intel } _ => llvm :: AsmDialect :: Att , } ; let result = inline_asm_call (self , & template_str , & constraints . join (",") , & inputs , output_type , & labels , volatile , alignstack , dialect , line_spans , options . contains (InlineAsmOptions :: MAY_UNWIND) , dest , catch_funclet ,) . unwrap_or_else (| | span_bug ! (line_spans [0] , "LLVM asm constraint validation failed")) ; let mut attrs = SmallVec :: < [_ ; 2] > :: new () ; if options . contains (InlineAsmOptions :: PURE) { if options . contains (InlineAsmOptions :: NOMEM) { attrs . push (llvm :: MemoryEffects :: None . create_attr (self . cx . llcx)) ; } else if options . contains (InlineAsmOptions :: READONLY) { attrs . push (llvm :: MemoryEffects :: ReadOnly . create_attr (self . cx . llcx)) ; } attrs . push (llvm :: AttributeKind :: WillReturn . create_attr (self . cx . llcx)) ; } else if options . contains (InlineAsmOptions :: NOMEM) { attrs . push (llvm :: MemoryEffects :: InaccessibleMemOnly . create_attr (self . cx . llcx)) ; } else { } attributes :: apply_to_callsite (result , llvm :: AttributePlace :: Function , & { attrs }) ; for block in (if options . contains (InlineAsmOptions :: NORETURN) { None } else { Some (dest) }) . into_iter () . chain (labels . iter () . copied () . map (Some)) { if let Some (block) = block { self . switch_to_block (block) ; } for (idx , op) in operands . iter () . enumerate () { if let InlineAsmOperandRef :: Out { reg , place : Some (place) , .. } | InlineAsmOperandRef :: InOut { reg , out_place : Some (place) , .. } = * op { let value = if output_types . len () == 1 { result } else { self . extract_value (result , op_idx [& idx] as u64) } ; let value = llvm_fixup_output (self , value , reg . reg_class () , & place . layout , instance) ; OperandValue :: Immediate (value) . store (self , place) ; } } } } }}}
mkitem!{mkimpl!{impl < 'tcx > AsmCodegenMethods < 'tcx > for CodegenCx < '_ , 'tcx > { fn codegen_global_asm (& mut self , template : & [InlineAsmTemplatePiece] , operands : & [GlobalAsmOperandRef < 'tcx >] , options : InlineAsmOptions , _line_spans : & [Span] ,) { let asm_arch = self . tcx . sess . asm_arch . unwrap () ; let mut template_str = String :: new () ; if matches ! (asm_arch , InlineAsmArch :: X86 | InlineAsmArch :: X86_64) { if options . contains (InlineAsmOptions :: ATT_SYNTAX) { template_str . push_str (".att_syntax\n") } else { template_str . push_str (".intel_syntax\n") } } for piece in template { match * piece { InlineAsmTemplatePiece :: String (ref s) => template_str . push_str (s) , InlineAsmTemplatePiece :: Placeholder { operand_idx , modifier : _ , span : _ } => { match operands [operand_idx] { GlobalAsmOperandRef :: Const { ref string } => { template_str . push_str (string) ; } GlobalAsmOperandRef :: SymFn { instance } => { let llval = self . get_fn (instance) ; self . add_compiler_used_global (llval) ; let symbol = llvm :: build_string (| s | unsafe { llvm :: LLVMRustGetMangledName (llval , s) ; }) . expect ("symbol is not valid UTF-8") ; template_str . push_str (& symbol) ; } GlobalAsmOperandRef :: SymStatic { def_id } => { let llval = self . renamed_statics . borrow () . get (& def_id) . copied () . unwrap_or_else (| | self . get_static (def_id)) ; self . add_compiler_used_global (llval) ; let symbol = llvm :: build_string (| s | unsafe { llvm :: LLVMRustGetMangledName (llval , s) ; }) . expect ("symbol is not valid UTF-8") ; template_str . push_str (& symbol) ; } } } } } if matches ! (asm_arch , InlineAsmArch :: X86 | InlineAsmArch :: X86_64) && ! options . contains (InlineAsmOptions :: ATT_SYNTAX) { template_str . push_str ("\n.att_syntax\n") ; } llvm :: append_module_inline_asm (self . llmod , template_str . as_bytes ()) ; } fn mangled_name (& self , instance : Instance < 'tcx >) -> String { let llval = self . get_fn (instance) ; llvm :: build_string (| s | unsafe { llvm :: LLVMRustGetMangledName (llval , s) ; }) . expect ("symbol is not valid UTF-8") } }}}

macro_rules! inline_asm_call_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function inline_asm_call in module {}", module_path!());
    };
}

mkfn!{
    inline_asm_call_introspect!();
    pub (crate) fn inline_asm_call < 'll > (bx : & mut Builder < '_ , 'll , '_ > , asm : & str , cons : & str , inputs : & [& 'll Value] , output : & 'll llvm :: Type , labels : & [& 'll llvm :: BasicBlock] , volatile : bool , alignstack : bool , dia : llvm :: AsmDialect , line_spans : & [Span] , unwind : bool , dest : Option < & 'll llvm :: BasicBlock > , catch_funclet : Option < (& 'll llvm :: BasicBlock , Option < & Funclet < 'll > >) > ,) -> Option < & 'll Value > { let argtys = inputs . iter () . map (| v | { debug ! ("Asm Input Type: {:?}" , * v) ; bx . cx . val_ty (* v) }) . collect :: < Vec < _ > > () ; debug ! ("Asm Output Type: {:?}" , output) ; let fty = bx . cx . type_func (& argtys , output) ; let constraints_ok = unsafe { llvm :: LLVMRustInlineAsmVerify (fty , cons . as_ptr () , cons . len ()) } ; debug ! ("constraint verification result: {:?}" , constraints_ok) ; if ! constraints_ok { return None ; } let v = unsafe { llvm :: LLVMGetInlineAsm (fty , asm . as_ptr () , asm . len () , cons . as_ptr () , cons . len () , volatile . to_llvm_bool () , alignstack . to_llvm_bool () , dia , unwind . to_llvm_bool () ,) } ; let call = if ! labels . is_empty () { assert ! (catch_funclet . is_none ()) ; bx . callbr (fty , None , None , v , inputs , dest . unwrap () , labels , None , None) } else if let Some ((catch , funclet)) = catch_funclet { bx . invoke (fty , None , None , v , inputs , dest . unwrap () , catch , funclet , None) } else { bx . call (fty , None , None , v , inputs , None , None) } ; let key = "srcloc" ; let kind = bx . get_md_kind_id (key) ; let mut srcloc = vec ! [] ; if dia == llvm :: AsmDialect :: Intel && line_spans . len () > 1 { srcloc . push (llvm :: LLVMValueAsMetadata (bx . const_u64 (0))) ; } srcloc . extend (line_spans . iter () . map (| span | { llvm :: LLVMValueAsMetadata (bx . const_u64 (u64 :: from (span . lo () . to_u32 ()) | (u64 :: from (span . hi () . to_u32 ()) << 32)) ,) })) ; let md = unsafe { llvm :: LLVMMDNodeInContext2 (bx . llcx , srcloc . as_ptr () , srcloc . len ()) } ; let md = bx . get_metadata_value (md) ; llvm :: LLVMSetMetadata (call , kind , md) ; Some (call) }
}

macro_rules! xmm_reg_index_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function xmm_reg_index in module {}", module_path!());
    };
}

mkfn!{
    xmm_reg_index_introspect!();
    #[doc = " If the register is an xmm/ymm/zmm register then return its index."] fn xmm_reg_index (reg : InlineAsmReg) -> Option < u32 > { use X86InlineAsmReg :: * ; match reg { InlineAsmReg :: X86 (reg) if reg as u32 >= xmm0 as u32 && reg as u32 <= xmm15 as u32 => { Some (reg as u32 - xmm0 as u32) } InlineAsmReg :: X86 (reg) if reg as u32 >= ymm0 as u32 && reg as u32 <= ymm15 as u32 => { Some (reg as u32 - ymm0 as u32) } InlineAsmReg :: X86 (reg) if reg as u32 >= zmm0 as u32 && reg as u32 <= zmm31 as u32 => { Some (reg as u32 - zmm0 as u32) } _ => None , } }
}

macro_rules! a64_reg_index_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function a64_reg_index in module {}", module_path!());
    };
}

mkfn!{
    a64_reg_index_introspect!();
    #[doc = " If the register is an AArch64 integer register then return its index."] fn a64_reg_index (reg : InlineAsmReg) -> Option < u32 > { match reg { InlineAsmReg :: AArch64 (r) => r . reg_index () , _ => None , } }
}

macro_rules! a64_vreg_index_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function a64_vreg_index in module {}", module_path!());
    };
}

mkfn!{
    a64_vreg_index_introspect!();
    #[doc = " If the register is an AArch64 vector register then return its index."] fn a64_vreg_index (reg : InlineAsmReg) -> Option < u32 > { match reg { InlineAsmReg :: AArch64 (reg) => reg . vreg_index () , _ => None , } }
}

macro_rules! reg_to_llvm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function reg_to_llvm in module {}", module_path!());
    };
}

mkfn!{
    reg_to_llvm_introspect!();
    #[doc = " Converts a register class to an LLVM constraint code."] fn reg_to_llvm (reg : InlineAsmRegOrRegClass , layout : Option < & TyAndLayout < '_ > >) -> String { use InlineAsmRegClass :: * ; match reg { InlineAsmRegOrRegClass :: Reg (reg) => { if let Some (idx) = xmm_reg_index (reg) { let class = if let Some (layout) = layout { match layout . size . bytes () { 64 => 'z' , 32 => 'y' , _ => 'x' , } } else { 'x' } ; format ! ("{{{}mm{}}}" , class , idx) } else if let Some (idx) = a64_reg_index (reg) { let class = if let Some (layout) = layout { match layout . size . bytes () { 8 => 'x' , _ => 'w' , } } else { 'w' } ; if class == 'x' && reg == InlineAsmReg :: AArch64 (AArch64InlineAsmReg :: x30) { "{lr}" . to_string () } else { format ! ("{{{}{}}}" , class , idx) } } else if let Some (idx) = a64_vreg_index (reg) { let class = if let Some (layout) = layout { match layout . size . bytes () { 16 => 'q' , 8 => 'd' , 4 => 's' , 2 => 'h' , 1 => 'd' , _ => unreachable ! () , } } else { 'q' } ; format ! ("{{{}{}}}" , class , idx) } else if reg == InlineAsmReg :: Arm (ArmInlineAsmReg :: r14) { "{lr}" . to_string () } else { format ! ("{{{}}}" , reg . name ()) } } InlineAsmRegOrRegClass :: RegClass (reg) => match reg { AArch64 (AArch64InlineAsmRegClass :: reg) => "r" , AArch64 (AArch64InlineAsmRegClass :: vreg) => "w" , AArch64 (AArch64InlineAsmRegClass :: vreg_low16) => "x" , AArch64 (AArch64InlineAsmRegClass :: preg) => unreachable ! ("clobber-only") , Arm (ArmInlineAsmRegClass :: reg) => "r" , Arm (ArmInlineAsmRegClass :: sreg) | Arm (ArmInlineAsmRegClass :: dreg_low16) | Arm (ArmInlineAsmRegClass :: qreg_low8) => "t" , Arm (ArmInlineAsmRegClass :: sreg_low16) | Arm (ArmInlineAsmRegClass :: dreg_low8) | Arm (ArmInlineAsmRegClass :: qreg_low4) => "x" , Arm (ArmInlineAsmRegClass :: dreg) | Arm (ArmInlineAsmRegClass :: qreg) => "w" , Hexagon (HexagonInlineAsmRegClass :: reg) => "r" , Hexagon (HexagonInlineAsmRegClass :: preg) => unreachable ! ("clobber-only") , LoongArch (LoongArchInlineAsmRegClass :: reg) => "r" , LoongArch (LoongArchInlineAsmRegClass :: freg) => "f" , Mips (MipsInlineAsmRegClass :: reg) => "r" , Mips (MipsInlineAsmRegClass :: freg) => "f" , Nvptx (NvptxInlineAsmRegClass :: reg16) => "h" , Nvptx (NvptxInlineAsmRegClass :: reg32) => "r" , Nvptx (NvptxInlineAsmRegClass :: reg64) => "l" , PowerPC (PowerPCInlineAsmRegClass :: reg) => "r" , PowerPC (PowerPCInlineAsmRegClass :: reg_nonzero) => "b" , PowerPC (PowerPCInlineAsmRegClass :: freg) => "f" , PowerPC (PowerPCInlineAsmRegClass :: vreg) => "v" , PowerPC (PowerPCInlineAsmRegClass :: cr) | PowerPC (PowerPCInlineAsmRegClass :: xer) => { unreachable ! ("clobber-only") } RiscV (RiscVInlineAsmRegClass :: reg) => "r" , RiscV (RiscVInlineAsmRegClass :: freg) => "f" , RiscV (RiscVInlineAsmRegClass :: vreg) => unreachable ! ("clobber-only") , X86 (X86InlineAsmRegClass :: reg) => "r" , X86 (X86InlineAsmRegClass :: reg_abcd) => "Q" , X86 (X86InlineAsmRegClass :: reg_byte) => "q" , X86 (X86InlineAsmRegClass :: xmm_reg) | X86 (X86InlineAsmRegClass :: ymm_reg) => "x" , X86 (X86InlineAsmRegClass :: zmm_reg) => "v" , X86 (X86InlineAsmRegClass :: kreg) => "^Yk" , X86 (X86InlineAsmRegClass :: x87_reg | X86InlineAsmRegClass :: mmx_reg | X86InlineAsmRegClass :: kreg0 | X86InlineAsmRegClass :: tmm_reg ,) => unreachable ! ("clobber-only") , Wasm (WasmInlineAsmRegClass :: local) => "r" , Bpf (BpfInlineAsmRegClass :: reg) => "r" , Bpf (BpfInlineAsmRegClass :: wreg) => "w" , Avr (AvrInlineAsmRegClass :: reg) => "r" , Avr (AvrInlineAsmRegClass :: reg_upper) => "d" , Avr (AvrInlineAsmRegClass :: reg_pair) => "r" , Avr (AvrInlineAsmRegClass :: reg_iw) => "w" , Avr (AvrInlineAsmRegClass :: reg_ptr) => "e" , S390x (S390xInlineAsmRegClass :: reg) => "r" , S390x (S390xInlineAsmRegClass :: reg_addr) => "a" , S390x (S390xInlineAsmRegClass :: freg) => "f" , S390x (S390xInlineAsmRegClass :: vreg) => "v" , S390x (S390xInlineAsmRegClass :: areg) => { unreachable ! ("clobber-only") } Sparc (SparcInlineAsmRegClass :: reg) => "r" , Sparc (SparcInlineAsmRegClass :: yreg) => unreachable ! ("clobber-only") , Msp430 (Msp430InlineAsmRegClass :: reg) => "r" , M68k (M68kInlineAsmRegClass :: reg) => "r" , M68k (M68kInlineAsmRegClass :: reg_addr) => "a" , M68k (M68kInlineAsmRegClass :: reg_data) => "d" , CSKY (CSKYInlineAsmRegClass :: reg) => "r" , CSKY (CSKYInlineAsmRegClass :: freg) => "f" , SpirV (SpirVInlineAsmRegClass :: reg) => bug ! ("LLVM backend does not support SPIR-V") , Err => unreachable ! () , } . to_string () , } }
}

macro_rules! modifier_to_llvm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function modifier_to_llvm in module {}", module_path!());
    };
}

mkfn!{
    modifier_to_llvm_introspect!();
    #[doc = " Converts a modifier into LLVM's equivalent modifier."] fn modifier_to_llvm (arch : InlineAsmArch , reg : InlineAsmRegClass , modifier : Option < char > ,) -> Option < char > { use InlineAsmRegClass :: * ; match reg { AArch64 (AArch64InlineAsmRegClass :: reg) => modifier , AArch64 (AArch64InlineAsmRegClass :: vreg) | AArch64 (AArch64InlineAsmRegClass :: vreg_low16) => { if modifier == Some ('v') { None } else { modifier } } AArch64 (AArch64InlineAsmRegClass :: preg) => unreachable ! ("clobber-only") , Arm (ArmInlineAsmRegClass :: reg) => None , Arm (ArmInlineAsmRegClass :: sreg) | Arm (ArmInlineAsmRegClass :: sreg_low16) => None , Arm (ArmInlineAsmRegClass :: dreg) | Arm (ArmInlineAsmRegClass :: dreg_low16) | Arm (ArmInlineAsmRegClass :: dreg_low8) => Some ('P') , Arm (ArmInlineAsmRegClass :: qreg) | Arm (ArmInlineAsmRegClass :: qreg_low8) | Arm (ArmInlineAsmRegClass :: qreg_low4) => { if modifier . is_none () { Some ('q') } else { modifier } } Hexagon (_) => None , LoongArch (_) => None , Mips (_) => None , Nvptx (_) => None , PowerPC (_) => None , RiscV (RiscVInlineAsmRegClass :: reg) | RiscV (RiscVInlineAsmRegClass :: freg) => None , RiscV (RiscVInlineAsmRegClass :: vreg) => unreachable ! ("clobber-only") , X86 (X86InlineAsmRegClass :: reg) | X86 (X86InlineAsmRegClass :: reg_abcd) => match modifier { None if arch == InlineAsmArch :: X86_64 => Some ('q') , None => Some ('k') , Some ('l') => Some ('b') , Some ('h') => Some ('h') , Some ('x') => Some ('w') , Some ('e') => Some ('k') , Some ('r') => Some ('q') , _ => unreachable ! () , } , X86 (X86InlineAsmRegClass :: reg_byte) => None , X86 (reg @ X86InlineAsmRegClass :: xmm_reg) | X86 (reg @ X86InlineAsmRegClass :: ymm_reg) | X86 (reg @ X86InlineAsmRegClass :: zmm_reg) => match (reg , modifier) { (X86InlineAsmRegClass :: xmm_reg , None) => Some ('x') , (X86InlineAsmRegClass :: ymm_reg , None) => Some ('t') , (X86InlineAsmRegClass :: zmm_reg , None) => Some ('g') , (_ , Some ('x')) => Some ('x') , (_ , Some ('y')) => Some ('t') , (_ , Some ('z')) => Some ('g') , _ => unreachable ! () , } , X86 (X86InlineAsmRegClass :: kreg) => None , X86 (X86InlineAsmRegClass :: x87_reg | X86InlineAsmRegClass :: mmx_reg | X86InlineAsmRegClass :: kreg0 | X86InlineAsmRegClass :: tmm_reg ,) => unreachable ! ("clobber-only") , Wasm (WasmInlineAsmRegClass :: local) => None , Bpf (_) => None , Avr (AvrInlineAsmRegClass :: reg_pair) | Avr (AvrInlineAsmRegClass :: reg_iw) | Avr (AvrInlineAsmRegClass :: reg_ptr) => match modifier { Some ('h') => Some ('B') , Some ('l') => Some ('A') , _ => None , } , Avr (_) => None , S390x (_) => None , Sparc (_) => None , Msp430 (_) => None , SpirV (SpirVInlineAsmRegClass :: reg) => bug ! ("LLVM backend does not support SPIR-V") , M68k (_) => None , CSKY (_) => None , Err => unreachable ! () , } }
}

macro_rules! dummy_output_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dummy_output_type in module {}", module_path!());
    };
}

mkfn!{
    dummy_output_type_introspect!();
    #[doc = " Type to use for outputs that are discarded. It doesn't really matter what"] #[doc = " the type is, as long as it is valid for the constraint code."] fn dummy_output_type < 'll > (cx : & CodegenCx < 'll , '_ > , reg : InlineAsmRegClass) -> & 'll Type { use InlineAsmRegClass :: * ; match reg { AArch64 (AArch64InlineAsmRegClass :: reg) => cx . type_i32 () , AArch64 (AArch64InlineAsmRegClass :: vreg) | AArch64 (AArch64InlineAsmRegClass :: vreg_low16) => { cx . type_vector (cx . type_i64 () , 2) } AArch64 (AArch64InlineAsmRegClass :: preg) => unreachable ! ("clobber-only") , Arm (ArmInlineAsmRegClass :: reg) => cx . type_i32 () , Arm (ArmInlineAsmRegClass :: sreg) | Arm (ArmInlineAsmRegClass :: sreg_low16) => cx . type_f32 () , Arm (ArmInlineAsmRegClass :: dreg) | Arm (ArmInlineAsmRegClass :: dreg_low16) | Arm (ArmInlineAsmRegClass :: dreg_low8) => cx . type_f64 () , Arm (ArmInlineAsmRegClass :: qreg) | Arm (ArmInlineAsmRegClass :: qreg_low8) | Arm (ArmInlineAsmRegClass :: qreg_low4) => cx . type_vector (cx . type_i64 () , 2) , Hexagon (HexagonInlineAsmRegClass :: reg) => cx . type_i32 () , Hexagon (HexagonInlineAsmRegClass :: preg) => unreachable ! ("clobber-only") , LoongArch (LoongArchInlineAsmRegClass :: reg) => cx . type_i32 () , LoongArch (LoongArchInlineAsmRegClass :: freg) => cx . type_f32 () , Mips (MipsInlineAsmRegClass :: reg) => cx . type_i32 () , Mips (MipsInlineAsmRegClass :: freg) => cx . type_f32 () , Nvptx (NvptxInlineAsmRegClass :: reg16) => cx . type_i16 () , Nvptx (NvptxInlineAsmRegClass :: reg32) => cx . type_i32 () , Nvptx (NvptxInlineAsmRegClass :: reg64) => cx . type_i64 () , PowerPC (PowerPCInlineAsmRegClass :: reg) => cx . type_i32 () , PowerPC (PowerPCInlineAsmRegClass :: reg_nonzero) => cx . type_i32 () , PowerPC (PowerPCInlineAsmRegClass :: freg) => cx . type_f64 () , PowerPC (PowerPCInlineAsmRegClass :: vreg) => cx . type_vector (cx . type_i32 () , 4) , PowerPC (PowerPCInlineAsmRegClass :: cr) | PowerPC (PowerPCInlineAsmRegClass :: xer) => { unreachable ! ("clobber-only") } RiscV (RiscVInlineAsmRegClass :: reg) => cx . type_i32 () , RiscV (RiscVInlineAsmRegClass :: freg) => cx . type_f32 () , RiscV (RiscVInlineAsmRegClass :: vreg) => unreachable ! ("clobber-only") , X86 (X86InlineAsmRegClass :: reg) | X86 (X86InlineAsmRegClass :: reg_abcd) => cx . type_i32 () , X86 (X86InlineAsmRegClass :: reg_byte) => cx . type_i8 () , X86 (X86InlineAsmRegClass :: xmm_reg) | X86 (X86InlineAsmRegClass :: ymm_reg) | X86 (X86InlineAsmRegClass :: zmm_reg) => cx . type_f32 () , X86 (X86InlineAsmRegClass :: kreg) => cx . type_i16 () , X86 (X86InlineAsmRegClass :: x87_reg | X86InlineAsmRegClass :: mmx_reg | X86InlineAsmRegClass :: kreg0 | X86InlineAsmRegClass :: tmm_reg ,) => unreachable ! ("clobber-only") , Wasm (WasmInlineAsmRegClass :: local) => cx . type_i32 () , Bpf (BpfInlineAsmRegClass :: reg) => cx . type_i64 () , Bpf (BpfInlineAsmRegClass :: wreg) => cx . type_i32 () , Avr (AvrInlineAsmRegClass :: reg) => cx . type_i8 () , Avr (AvrInlineAsmRegClass :: reg_upper) => cx . type_i8 () , Avr (AvrInlineAsmRegClass :: reg_pair) => cx . type_i16 () , Avr (AvrInlineAsmRegClass :: reg_iw) => cx . type_i16 () , Avr (AvrInlineAsmRegClass :: reg_ptr) => cx . type_i16 () , S390x (S390xInlineAsmRegClass :: reg | S390xInlineAsmRegClass :: reg_addr) => cx . type_i32 () , S390x (S390xInlineAsmRegClass :: freg) => cx . type_f64 () , S390x (S390xInlineAsmRegClass :: vreg) => cx . type_vector (cx . type_i64 () , 2) , S390x (S390xInlineAsmRegClass :: areg) => { unreachable ! ("clobber-only") } Sparc (SparcInlineAsmRegClass :: reg) => cx . type_i32 () , Sparc (SparcInlineAsmRegClass :: yreg) => unreachable ! ("clobber-only") , Msp430 (Msp430InlineAsmRegClass :: reg) => cx . type_i16 () , M68k (M68kInlineAsmRegClass :: reg) => cx . type_i32 () , M68k (M68kInlineAsmRegClass :: reg_addr) => cx . type_i32 () , M68k (M68kInlineAsmRegClass :: reg_data) => cx . type_i32 () , CSKY (CSKYInlineAsmRegClass :: reg) => cx . type_i32 () , CSKY (CSKYInlineAsmRegClass :: freg) => cx . type_f32 () , SpirV (SpirVInlineAsmRegClass :: reg) => bug ! ("LLVM backend does not support SPIR-V") , Err => unreachable ! () , } }
}

macro_rules! llvm_asm_scalar_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function llvm_asm_scalar_type in module {}", module_path!());
    };
}

mkfn!{
    llvm_asm_scalar_type_introspect!();
    #[doc = " Helper function to get the LLVM type for a Scalar. Pointers are returned as"] #[doc = " the equivalent integer type."] fn llvm_asm_scalar_type < 'll > (cx : & CodegenCx < 'll , '_ > , scalar : Scalar) -> & 'll Type { let dl = & cx . tcx . data_layout ; match scalar . primitive () { Primitive :: Int (Integer :: I8 , _) => cx . type_i8 () , Primitive :: Int (Integer :: I16 , _) => cx . type_i16 () , Primitive :: Int (Integer :: I32 , _) => cx . type_i32 () , Primitive :: Int (Integer :: I64 , _) => cx . type_i64 () , Primitive :: Float (Float :: F16) => cx . type_f16 () , Primitive :: Float (Float :: F32) => cx . type_f32 () , Primitive :: Float (Float :: F64) => cx . type_f64 () , Primitive :: Float (Float :: F128) => cx . type_f128 () , Primitive :: Pointer (_) => cx . type_from_integer (dl . ptr_sized_integer ()) , _ => unreachable ! () , } }
}

macro_rules! any_target_feature_enabled_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function any_target_feature_enabled in module {}", module_path!());
    };
}

mkfn!{
    any_target_feature_enabled_introspect!();
    fn any_target_feature_enabled (cx : & CodegenCx < '_ , '_ > , instance : Instance < '_ > , features : & [Symbol] ,) -> bool { let enabled = cx . tcx . asm_target_features (instance . def_id ()) ; features . iter () . any (| feat | enabled . contains (feat)) }
}

macro_rules! llvm_fixup_input_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function llvm_fixup_input in module {}", module_path!());
    };
}

mkfn!{
    llvm_fixup_input_introspect!();
    #[doc = " Fix up an input value to work around LLVM bugs."] fn llvm_fixup_input < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , mut value : & 'll Value , reg : InlineAsmRegClass , layout : & TyAndLayout < 'tcx > , instance : Instance < '_ > ,) -> & 'll Value { use InlineAsmRegClass :: * ; let dl = & bx . tcx . data_layout ; match (reg , layout . backend_repr) { (AArch64 (AArch64InlineAsmRegClass :: vreg) , BackendRepr :: Scalar (s)) => { if let Primitive :: Int (Integer :: I8 , _) = s . primitive () { let vec_ty = bx . cx . type_vector (bx . cx . type_i8 () , 8) ; bx . insert_element (bx . const_undef (vec_ty) , value , bx . const_i32 (0)) } else { value } } (AArch64 (AArch64InlineAsmRegClass :: vreg_low16) , BackendRepr :: Scalar (s)) if s . primitive () != Primitive :: Float (Float :: F128) => { let elem_ty = llvm_asm_scalar_type (bx . cx , s) ; let count = 16 / layout . size . bytes () ; let vec_ty = bx . cx . type_vector (elem_ty , count) ; if let Primitive :: Pointer (_) = s . primitive () { let t = bx . type_from_integer (dl . ptr_sized_integer ()) ; value = bx . ptrtoint (value , t) ; } bx . insert_element (bx . const_undef (vec_ty) , value , bx . const_i32 (0)) } (AArch64 (AArch64InlineAsmRegClass :: vreg_low16) , BackendRepr :: SimdVector { element , count } ,) if layout . size . bytes () == 8 => { let elem_ty = llvm_asm_scalar_type (bx . cx , element) ; let vec_ty = bx . cx . type_vector (elem_ty , count) ; let indices : Vec < _ > = (0 .. count * 2) . map (| x | bx . const_i32 (x as i32)) . collect () ; bx . shuffle_vector (value , bx . const_undef (vec_ty) , bx . const_vector (& indices)) } (X86 (X86InlineAsmRegClass :: reg_abcd) , BackendRepr :: Scalar (s)) if s . primitive () == Primitive :: Float (Float :: F64) => { bx . bitcast (value , bx . cx . type_i64 ()) } (X86 (X86InlineAsmRegClass :: xmm_reg | X86InlineAsmRegClass :: zmm_reg) , BackendRepr :: SimdVector { .. } ,) if layout . size . bytes () == 64 => bx . bitcast (value , bx . cx . type_vector (bx . cx . type_f64 () , 8)) , (X86 (X86InlineAsmRegClass :: xmm_reg | X86InlineAsmRegClass :: ymm_reg | X86InlineAsmRegClass :: zmm_reg ,) , BackendRepr :: Scalar (s) ,) if bx . sess () . asm_arch == Some (InlineAsmArch :: X86) && s . primitive () == Primitive :: Float (Float :: F128) => { bx . bitcast (value , bx . type_vector (bx . type_i32 () , 4)) } (X86 (X86InlineAsmRegClass :: xmm_reg | X86InlineAsmRegClass :: ymm_reg | X86InlineAsmRegClass :: zmm_reg ,) , BackendRepr :: Scalar (s) ,) if s . primitive () == Primitive :: Float (Float :: F16) => { let value = bx . insert_element (bx . const_undef (bx . type_vector (bx . type_f16 () , 8)) , value , bx . const_usize (0) ,) ; bx . bitcast (value , bx . type_vector (bx . type_i16 () , 8)) } (X86 (X86InlineAsmRegClass :: xmm_reg | X86InlineAsmRegClass :: ymm_reg | X86InlineAsmRegClass :: zmm_reg ,) , BackendRepr :: SimdVector { element , count : count @ (8 | 16) } ,) if element . primitive () == Primitive :: Float (Float :: F16) => { bx . bitcast (value , bx . type_vector (bx . type_i16 () , count)) } (Arm (ArmInlineAsmRegClass :: sreg | ArmInlineAsmRegClass :: sreg_low16) , BackendRepr :: Scalar (s) ,) => { if let Primitive :: Int (Integer :: I32 , _) = s . primitive () { bx . bitcast (value , bx . cx . type_f32 ()) } else { value } } (Arm (ArmInlineAsmRegClass :: dreg | ArmInlineAsmRegClass :: dreg_low8 | ArmInlineAsmRegClass :: dreg_low16 ,) , BackendRepr :: Scalar (s) ,) => { if let Primitive :: Int (Integer :: I64 , _) = s . primitive () { bx . bitcast (value , bx . cx . type_f64 ()) } else { value } } (Arm (ArmInlineAsmRegClass :: dreg | ArmInlineAsmRegClass :: dreg_low8 | ArmInlineAsmRegClass :: dreg_low16 | ArmInlineAsmRegClass :: qreg | ArmInlineAsmRegClass :: qreg_low4 | ArmInlineAsmRegClass :: qreg_low8 ,) , BackendRepr :: SimdVector { element , count : count @ (4 | 8) } ,) if element . primitive () == Primitive :: Float (Float :: F16) => { bx . bitcast (value , bx . type_vector (bx . type_i16 () , count)) } (LoongArch (LoongArchInlineAsmRegClass :: freg) , BackendRepr :: Scalar (s)) if s . primitive () == Primitive :: Float (Float :: F16) => { let value = bx . bitcast (value , bx . type_i16 ()) ; let value = bx . zext (value , bx . type_i32 ()) ; let value = bx . or (value , bx . const_u32 (0xFFFF_0000)) ; bx . bitcast (value , bx . type_f32 ()) } (Mips (MipsInlineAsmRegClass :: reg) , BackendRepr :: Scalar (s)) => { match s . primitive () { Primitive :: Int (Integer :: I8 | Integer :: I16 , _) => bx . zext (value , bx . cx . type_i32 ()) , Primitive :: Float (Float :: F32) => bx . bitcast (value , bx . cx . type_i32 ()) , Primitive :: Float (Float :: F64) => bx . bitcast (value , bx . cx . type_i64 ()) , _ => value , } } (RiscV (RiscVInlineAsmRegClass :: freg) , BackendRepr :: Scalar (s)) if s . primitive () == Primitive :: Float (Float :: F16) && ! any_target_feature_enabled (bx , instance , & [sym :: zfhmin , sym :: zfh]) => { let value = bx . bitcast (value , bx . type_i16 ()) ; let value = bx . zext (value , bx . type_i32 ()) ; let value = bx . or (value , bx . const_u32 (0xFFFF_0000)) ; bx . bitcast (value , bx . type_f32 ()) } (PowerPC (PowerPCInlineAsmRegClass :: vreg) , BackendRepr :: Scalar (s)) if s . primitive () == Primitive :: Float (Float :: F32) => { let value = bx . insert_element (bx . const_undef (bx . type_vector (bx . type_f32 () , 4)) , value , bx . const_usize (0) ,) ; bx . bitcast (value , bx . type_vector (bx . type_f32 () , 4)) } (PowerPC (PowerPCInlineAsmRegClass :: vreg) , BackendRepr :: Scalar (s)) if s . primitive () == Primitive :: Float (Float :: F64) => { let value = bx . insert_element (bx . const_undef (bx . type_vector (bx . type_f64 () , 2)) , value , bx . const_usize (0) ,) ; bx . bitcast (value , bx . type_vector (bx . type_f64 () , 2)) } _ => value , } }
}

macro_rules! llvm_fixup_output_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function llvm_fixup_output in module {}", module_path!());
    };
}

mkfn!{
    llvm_fixup_output_introspect!();
    #[doc = " Fix up an output value to work around LLVM bugs."] fn llvm_fixup_output < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , mut value : & 'll Value , reg : InlineAsmRegClass , layout : & TyAndLayout < 'tcx > , instance : Instance < '_ > ,) -> & 'll Value { use InlineAsmRegClass :: * ; match (reg , layout . backend_repr) { (AArch64 (AArch64InlineAsmRegClass :: vreg) , BackendRepr :: Scalar (s)) => { if let Primitive :: Int (Integer :: I8 , _) = s . primitive () { bx . extract_element (value , bx . const_i32 (0)) } else { value } } (AArch64 (AArch64InlineAsmRegClass :: vreg_low16) , BackendRepr :: Scalar (s)) if s . primitive () != Primitive :: Float (Float :: F128) => { value = bx . extract_element (value , bx . const_i32 (0)) ; if let Primitive :: Pointer (_) = s . primitive () { value = bx . inttoptr (value , layout . llvm_type (bx . cx)) ; } value } (AArch64 (AArch64InlineAsmRegClass :: vreg_low16) , BackendRepr :: SimdVector { element , count } ,) if layout . size . bytes () == 8 => { let elem_ty = llvm_asm_scalar_type (bx . cx , element) ; let vec_ty = bx . cx . type_vector (elem_ty , count * 2) ; let indices : Vec < _ > = (0 .. count) . map (| x | bx . const_i32 (x as i32)) . collect () ; bx . shuffle_vector (value , bx . const_undef (vec_ty) , bx . const_vector (& indices)) } (X86 (X86InlineAsmRegClass :: reg_abcd) , BackendRepr :: Scalar (s)) if s . primitive () == Primitive :: Float (Float :: F64) => { bx . bitcast (value , bx . cx . type_f64 ()) } (X86 (X86InlineAsmRegClass :: xmm_reg | X86InlineAsmRegClass :: zmm_reg) , BackendRepr :: SimdVector { .. } ,) if layout . size . bytes () == 64 => bx . bitcast (value , layout . llvm_type (bx . cx)) , (X86 (X86InlineAsmRegClass :: xmm_reg | X86InlineAsmRegClass :: ymm_reg | X86InlineAsmRegClass :: zmm_reg ,) , BackendRepr :: Scalar (s) ,) if bx . sess () . asm_arch == Some (InlineAsmArch :: X86) && s . primitive () == Primitive :: Float (Float :: F128) => { bx . bitcast (value , bx . type_f128 ()) } (X86 (X86InlineAsmRegClass :: xmm_reg | X86InlineAsmRegClass :: ymm_reg | X86InlineAsmRegClass :: zmm_reg ,) , BackendRepr :: Scalar (s) ,) if s . primitive () == Primitive :: Float (Float :: F16) => { let value = bx . bitcast (value , bx . type_vector (bx . type_f16 () , 8)) ; bx . extract_element (value , bx . const_usize (0)) } (X86 (X86InlineAsmRegClass :: xmm_reg | X86InlineAsmRegClass :: ymm_reg | X86InlineAsmRegClass :: zmm_reg ,) , BackendRepr :: SimdVector { element , count : count @ (8 | 16) } ,) if element . primitive () == Primitive :: Float (Float :: F16) => { bx . bitcast (value , bx . type_vector (bx . type_f16 () , count)) } (Arm (ArmInlineAsmRegClass :: sreg | ArmInlineAsmRegClass :: sreg_low16) , BackendRepr :: Scalar (s) ,) => { if let Primitive :: Int (Integer :: I32 , _) = s . primitive () { bx . bitcast (value , bx . cx . type_i32 ()) } else { value } } (Arm (ArmInlineAsmRegClass :: dreg | ArmInlineAsmRegClass :: dreg_low8 | ArmInlineAsmRegClass :: dreg_low16 ,) , BackendRepr :: Scalar (s) ,) => { if let Primitive :: Int (Integer :: I64 , _) = s . primitive () { bx . bitcast (value , bx . cx . type_i64 ()) } else { value } } (Arm (ArmInlineAsmRegClass :: dreg | ArmInlineAsmRegClass :: dreg_low8 | ArmInlineAsmRegClass :: dreg_low16 | ArmInlineAsmRegClass :: qreg | ArmInlineAsmRegClass :: qreg_low4 | ArmInlineAsmRegClass :: qreg_low8 ,) , BackendRepr :: SimdVector { element , count : count @ (4 | 8) } ,) if element . primitive () == Primitive :: Float (Float :: F16) => { bx . bitcast (value , bx . type_vector (bx . type_f16 () , count)) } (LoongArch (LoongArchInlineAsmRegClass :: freg) , BackendRepr :: Scalar (s)) if s . primitive () == Primitive :: Float (Float :: F16) => { let value = bx . bitcast (value , bx . type_i32 ()) ; let value = bx . trunc (value , bx . type_i16 ()) ; bx . bitcast (value , bx . type_f16 ()) } (Mips (MipsInlineAsmRegClass :: reg) , BackendRepr :: Scalar (s)) => { match s . primitive () { Primitive :: Int (Integer :: I8 , _) => bx . trunc (value , bx . cx . type_i8 ()) , Primitive :: Int (Integer :: I16 , _) => bx . trunc (value , bx . cx . type_i16 ()) , Primitive :: Float (Float :: F32) => bx . bitcast (value , bx . cx . type_f32 ()) , Primitive :: Float (Float :: F64) => bx . bitcast (value , bx . cx . type_f64 ()) , _ => value , } } (RiscV (RiscVInlineAsmRegClass :: freg) , BackendRepr :: Scalar (s)) if s . primitive () == Primitive :: Float (Float :: F16) && ! any_target_feature_enabled (bx , instance , & [sym :: zfhmin , sym :: zfh]) => { let value = bx . bitcast (value , bx . type_i32 ()) ; let value = bx . trunc (value , bx . type_i16 ()) ; bx . bitcast (value , bx . type_f16 ()) } (PowerPC (PowerPCInlineAsmRegClass :: vreg) , BackendRepr :: Scalar (s)) if s . primitive () == Primitive :: Float (Float :: F32) => { let value = bx . bitcast (value , bx . type_vector (bx . type_f32 () , 4)) ; bx . extract_element (value , bx . const_usize (0)) } (PowerPC (PowerPCInlineAsmRegClass :: vreg) , BackendRepr :: Scalar (s)) if s . primitive () == Primitive :: Float (Float :: F64) => { let value = bx . bitcast (value , bx . type_vector (bx . type_f64 () , 2)) ; bx . extract_element (value , bx . const_usize (0)) } _ => value , } }
}

macro_rules! llvm_fixup_output_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function llvm_fixup_output_type in module {}", module_path!());
    };
}

mkfn!{
    llvm_fixup_output_type_introspect!();
    #[doc = " Output type to use for llvm_fixup_output."] fn llvm_fixup_output_type < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , reg : InlineAsmRegClass , layout : & TyAndLayout < 'tcx > , instance : Instance < '_ > ,) -> & 'll Type { use InlineAsmRegClass :: * ; match (reg , layout . backend_repr) { (AArch64 (AArch64InlineAsmRegClass :: vreg) , BackendRepr :: Scalar (s)) => { if let Primitive :: Int (Integer :: I8 , _) = s . primitive () { cx . type_vector (cx . type_i8 () , 8) } else { layout . llvm_type (cx) } } (AArch64 (AArch64InlineAsmRegClass :: vreg_low16) , BackendRepr :: Scalar (s)) if s . primitive () != Primitive :: Float (Float :: F128) => { let elem_ty = llvm_asm_scalar_type (cx , s) ; let count = 16 / layout . size . bytes () ; cx . type_vector (elem_ty , count) } (AArch64 (AArch64InlineAsmRegClass :: vreg_low16) , BackendRepr :: SimdVector { element , count } ,) if layout . size . bytes () == 8 => { let elem_ty = llvm_asm_scalar_type (cx , element) ; cx . type_vector (elem_ty , count * 2) } (X86 (X86InlineAsmRegClass :: reg_abcd) , BackendRepr :: Scalar (s)) if s . primitive () == Primitive :: Float (Float :: F64) => { cx . type_i64 () } (X86 (X86InlineAsmRegClass :: xmm_reg | X86InlineAsmRegClass :: zmm_reg) , BackendRepr :: SimdVector { .. } ,) if layout . size . bytes () == 64 => cx . type_vector (cx . type_f64 () , 8) , (X86 (X86InlineAsmRegClass :: xmm_reg | X86InlineAsmRegClass :: ymm_reg | X86InlineAsmRegClass :: zmm_reg ,) , BackendRepr :: Scalar (s) ,) if cx . sess () . asm_arch == Some (InlineAsmArch :: X86) && s . primitive () == Primitive :: Float (Float :: F128) => { cx . type_vector (cx . type_i32 () , 4) } (X86 (X86InlineAsmRegClass :: xmm_reg | X86InlineAsmRegClass :: ymm_reg | X86InlineAsmRegClass :: zmm_reg ,) , BackendRepr :: Scalar (s) ,) if s . primitive () == Primitive :: Float (Float :: F16) => cx . type_vector (cx . type_i16 () , 8) , (X86 (X86InlineAsmRegClass :: xmm_reg | X86InlineAsmRegClass :: ymm_reg | X86InlineAsmRegClass :: zmm_reg ,) , BackendRepr :: SimdVector { element , count : count @ (8 | 16) } ,) if element . primitive () == Primitive :: Float (Float :: F16) => { cx . type_vector (cx . type_i16 () , count) } (Arm (ArmInlineAsmRegClass :: sreg | ArmInlineAsmRegClass :: sreg_low16) , BackendRepr :: Scalar (s) ,) => { if let Primitive :: Int (Integer :: I32 , _) = s . primitive () { cx . type_f32 () } else { layout . llvm_type (cx) } } (Arm (ArmInlineAsmRegClass :: dreg | ArmInlineAsmRegClass :: dreg_low8 | ArmInlineAsmRegClass :: dreg_low16 ,) , BackendRepr :: Scalar (s) ,) => { if let Primitive :: Int (Integer :: I64 , _) = s . primitive () { cx . type_f64 () } else { layout . llvm_type (cx) } } (Arm (ArmInlineAsmRegClass :: dreg | ArmInlineAsmRegClass :: dreg_low8 | ArmInlineAsmRegClass :: dreg_low16 | ArmInlineAsmRegClass :: qreg | ArmInlineAsmRegClass :: qreg_low4 | ArmInlineAsmRegClass :: qreg_low8 ,) , BackendRepr :: SimdVector { element , count : count @ (4 | 8) } ,) if element . primitive () == Primitive :: Float (Float :: F16) => { cx . type_vector (cx . type_i16 () , count) } (LoongArch (LoongArchInlineAsmRegClass :: freg) , BackendRepr :: Scalar (s)) if s . primitive () == Primitive :: Float (Float :: F16) => { cx . type_f32 () } (Mips (MipsInlineAsmRegClass :: reg) , BackendRepr :: Scalar (s)) => { match s . primitive () { Primitive :: Int (Integer :: I8 | Integer :: I16 , _) => cx . type_i32 () , Primitive :: Float (Float :: F32) => cx . type_i32 () , Primitive :: Float (Float :: F64) => cx . type_i64 () , _ => layout . llvm_type (cx) , } } (RiscV (RiscVInlineAsmRegClass :: freg) , BackendRepr :: Scalar (s)) if s . primitive () == Primitive :: Float (Float :: F16) && ! any_target_feature_enabled (cx , instance , & [sym :: zfhmin , sym :: zfh]) => { cx . type_f32 () } (PowerPC (PowerPCInlineAsmRegClass :: vreg) , BackendRepr :: Scalar (s)) if s . primitive () == Primitive :: Float (Float :: F32) => { cx . type_vector (cx . type_f32 () , 4) } (PowerPC (PowerPCInlineAsmRegClass :: vreg) , BackendRepr :: Scalar (s)) if s . primitive () == Primitive :: Float (Float :: F64) => { cx . type_vector (cx . type_f64 () , 2) } _ => layout . llvm_type (cx) , } }
}