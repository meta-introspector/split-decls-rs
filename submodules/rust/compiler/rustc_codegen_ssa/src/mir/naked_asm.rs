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
mkuse!{use rustc_abi :: { BackendRepr , Float , Integer , Primitive , RegKind } ;}
mkuse!{use rustc_hir :: attrs :: { InstructionSetAttr , Linkage } ;}
mkuse!{use rustc_middle :: mir :: mono :: { MonoItemData , Visibility } ;}
mkuse!{use rustc_middle :: mir :: { InlineAsmOperand , START_BLOCK } ;}
mkuse!{use rustc_middle :: ty :: layout :: { FnAbiOf , LayoutOf , TyAndLayout } ;}
mkuse!{use rustc_middle :: ty :: { Instance , Ty , TyCtxt , TypeVisitableExt } ;}
mkuse!{use rustc_middle :: { bug , ty } ;}
mkuse!{use rustc_span :: sym ;}
mkuse!{use rustc_target :: callconv :: { ArgAbi , FnAbi , PassMode } ;}
mkuse!{use rustc_target :: spec :: BinaryFormat ;}
mkuse!{use crate :: common ;}
mkuse!{use crate :: mir :: AsmCodegenMethods ;}
mkuse!{use crate :: traits :: GlobalAsmOperandRef ;}

macro_rules! codegen_naked_asm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function codegen_naked_asm in module {}", module_path!());
    };
}

mkfn!{
    codegen_naked_asm_introspect!();
    pub fn codegen_naked_asm < 'a , 'tcx , Cx : LayoutOf < 'tcx , LayoutOfResult = TyAndLayout < 'tcx > > + FnAbiOf < 'tcx , FnAbiOfResult = & 'tcx FnAbi < 'tcx , Ty < 'tcx > > > + AsmCodegenMethods < 'tcx > , > (cx : & 'a mut Cx , instance : Instance < 'tcx > , item_data : MonoItemData ,) { assert ! (! instance . args . has_infer ()) ; let mir = cx . tcx () . instance_mir (instance . def) ; let rustc_middle :: mir :: TerminatorKind :: InlineAsm { asm_macro : _ , template , ref operands , options , line_spans , targets : _ , unwind : _ , } = mir . basic_blocks [START_BLOCK] . terminator () . kind else { bug ! ("#[naked] functions should always terminate with an asm! block") } ; let operands : Vec < _ > = operands . iter () . map (| op | inline_to_global_operand :: < Cx > (cx , instance , op)) . collect () ; let name = cx . mangled_name (instance) ; let fn_abi = cx . fn_abi_of_instance (instance , ty :: List :: empty ()) ; let (begin , end) = prefix_and_suffix (cx . tcx () , instance , & name , item_data , fn_abi) ; let mut template_vec = Vec :: new () ; template_vec . push (rustc_ast :: ast :: InlineAsmTemplatePiece :: String (begin . into ())) ; template_vec . extend (template . iter () . cloned ()) ; template_vec . push (rustc_ast :: ast :: InlineAsmTemplatePiece :: String (end . into ())) ; cx . codegen_global_asm (& template_vec , & operands , options , line_spans) ; }
}

macro_rules! inline_to_global_operand_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function inline_to_global_operand in module {}", module_path!());
    };
}

mkfn!{
    inline_to_global_operand_introspect!();
    fn inline_to_global_operand < 'a , 'tcx , Cx : LayoutOf < 'tcx , LayoutOfResult = TyAndLayout < 'tcx > > > (cx : & 'a Cx , instance : Instance < 'tcx > , op : & InlineAsmOperand < 'tcx > ,) -> GlobalAsmOperandRef < 'tcx > { match op { InlineAsmOperand :: Const { value } => { let const_value = instance . instantiate_mir_and_normalize_erasing_regions (cx . tcx () , cx . typing_env () , ty :: EarlyBinder :: bind (value . const_) ,) . eval (cx . tcx () , cx . typing_env () , value . span) . expect ("erroneous constant missed by mono item collection") ; let mono_type = instance . instantiate_mir_and_normalize_erasing_regions (cx . tcx () , cx . typing_env () , ty :: EarlyBinder :: bind (value . ty ()) ,) ; let string = common :: asm_const_to_str (cx . tcx () , value . span , const_value , cx . layout_of (mono_type) ,) ; GlobalAsmOperandRef :: Const { string } } InlineAsmOperand :: SymFn { value } => { let mono_type = instance . instantiate_mir_and_normalize_erasing_regions (cx . tcx () , cx . typing_env () , ty :: EarlyBinder :: bind (value . ty ()) ,) ; let instance = match mono_type . kind () { & ty :: FnDef (def_id , args) => { Instance :: expect_resolve (cx . tcx () , cx . typing_env () , def_id , args , value . span) } _ => bug ! ("asm sym is not a function") , } ; GlobalAsmOperandRef :: SymFn { instance } } InlineAsmOperand :: SymStatic { def_id } => { GlobalAsmOperandRef :: SymStatic { def_id : * def_id } } InlineAsmOperand :: In { .. } | InlineAsmOperand :: Out { .. } | InlineAsmOperand :: InOut { .. } | InlineAsmOperand :: Label { .. } => { bug ! ("invalid operand type for naked_asm!") } } }
}

macro_rules! prefix_and_suffix_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function prefix_and_suffix in module {}", module_path!());
    };
}

mkfn!{
    prefix_and_suffix_introspect!();
    fn prefix_and_suffix < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , asm_name : & str , item_data : MonoItemData , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > > ,) -> (String , String) { use std :: fmt :: Write ; let asm_binary_format = & tcx . sess . target . binary_format ; let is_arm = tcx . sess . target . arch == "arm" ; let is_thumb = tcx . sess . unstable_target_features . contains (& sym :: thumb_mode) ; let attrs = tcx . codegen_instance_attrs (instance . def) ; let link_section = attrs . link_section . map (| symbol | symbol . as_str () . to_string ()) ; let align_bytes = attrs . alignment . map (| a | a . bytes ()) . unwrap_or (4) ; let (arch_prefix , arch_suffix) = if is_arm { (match attrs . instruction_set { None => match is_thumb { true => ".thumb\n.thumb_func" , false => ".arm" , } , Some (InstructionSetAttr :: ArmT32) => ".thumb\n.thumb_func" , Some (InstructionSetAttr :: ArmA32) => ".arm" , } , match is_thumb { true => ".thumb" , false => ".arm" , } ,) } else { ("" , "") } ; let emit_fatal = | msg | tcx . dcx () . span_fatal (tcx . def_span (instance . def_id ()) , msg) ; let write_linkage = | w : & mut String | -> std :: fmt :: Result { match item_data . linkage { Linkage :: External => { writeln ! (w , ".globl {asm_name}") ? ; } Linkage :: LinkOnceAny | Linkage :: LinkOnceODR | Linkage :: WeakAny | Linkage :: WeakODR => { match asm_binary_format { BinaryFormat :: Elf | BinaryFormat :: Coff | BinaryFormat :: Wasm => { writeln ! (w , ".weak {asm_name}") ? ; } BinaryFormat :: Xcoff => { emit_fatal ("cannot create weak symbols from inline assembly for this target" ,) } BinaryFormat :: MachO => { writeln ! (w , ".globl {asm_name}") ? ; writeln ! (w , ".weak_definition {asm_name}") ? ; } } } Linkage :: Internal => { } Linkage :: Common => emit_fatal ("Functions may not have common linkage") , Linkage :: AvailableExternally => { emit_fatal ("Functions may not have available_externally linkage") } Linkage :: ExternalWeak => { emit_fatal ("Functions may not have external weak linkage") } } Ok (()) } ; let mut begin = String :: new () ; let mut end = String :: new () ; match asm_binary_format { BinaryFormat :: Elf => { let section = link_section . unwrap_or_else (| | format ! (".text.{asm_name}")) ; let progbits = match is_arm { true => "%progbits" , false => "@progbits" , } ; let function = match is_arm { true => "%function" , false => "@function" , } ; writeln ! (begin , ".pushsection {section},\"ax\", {progbits}") . unwrap () ; writeln ! (begin , ".balign {align_bytes}") . unwrap () ; write_linkage (& mut begin) . unwrap () ; match item_data . visibility { Visibility :: Default => { } Visibility :: Protected => writeln ! (begin , ".protected {asm_name}") . unwrap () , Visibility :: Hidden => writeln ! (begin , ".hidden {asm_name}") . unwrap () , } writeln ! (begin , ".type {asm_name}, {function}") . unwrap () ; if ! arch_prefix . is_empty () { writeln ! (begin , "{}" , arch_prefix) . unwrap () ; } writeln ! (begin , "{asm_name}:") . unwrap () ; writeln ! (end) . unwrap () ; writeln ! (end , ".size {asm_name}, . - {asm_name}") . unwrap () ; writeln ! (end , ".popsection") . unwrap () ; if ! arch_suffix . is_empty () { writeln ! (end , "{}" , arch_suffix) . unwrap () ; } } BinaryFormat :: MachO => { let section = link_section . unwrap_or_else (| | "__TEXT,__text" . to_string ()) ; writeln ! (begin , ".pushsection {},regular,pure_instructions" , section) . unwrap () ; writeln ! (begin , ".balign {align_bytes}") . unwrap () ; write_linkage (& mut begin) . unwrap () ; match item_data . visibility { Visibility :: Default | Visibility :: Protected => { } Visibility :: Hidden => writeln ! (begin , ".private_extern {asm_name}") . unwrap () , } writeln ! (begin , "{asm_name}:") . unwrap () ; writeln ! (end) . unwrap () ; writeln ! (end , ".popsection") . unwrap () ; if ! arch_suffix . is_empty () { writeln ! (end , "{}" , arch_suffix) . unwrap () ; } } BinaryFormat :: Coff => { let section = link_section . unwrap_or_else (| | format ! (".text.{asm_name}")) ; writeln ! (begin , ".pushsection {},\"xr\"" , section) . unwrap () ; writeln ! (begin , ".balign {align_bytes}") . unwrap () ; write_linkage (& mut begin) . unwrap () ; writeln ! (begin , ".def {asm_name}") . unwrap () ; writeln ! (begin , ".scl 2") . unwrap () ; writeln ! (begin , ".type 32") . unwrap () ; writeln ! (begin , ".endef") . unwrap () ; writeln ! (begin , "{asm_name}:") . unwrap () ; writeln ! (end) . unwrap () ; writeln ! (end , ".popsection") . unwrap () ; if ! arch_suffix . is_empty () { writeln ! (end , "{}" , arch_suffix) . unwrap () ; } } BinaryFormat :: Wasm => { let section = link_section . unwrap_or_else (| | format ! (".text.{asm_name}")) ; writeln ! (begin , ".section {section},\"\",@") . unwrap () ; write_linkage (& mut begin) . unwrap () ; if let Visibility :: Hidden = item_data . visibility { writeln ! (begin , ".hidden {asm_name}") . unwrap () ; } writeln ! (begin , ".type {asm_name}, @function") . unwrap () ; if ! arch_prefix . is_empty () { writeln ! (begin , "{}" , arch_prefix) . unwrap () ; } writeln ! (begin , "{asm_name}:") . unwrap () ; writeln ! (begin , ".functype {asm_name} {}" , wasm_functype (tcx , fn_abi)) . unwrap () ; writeln ! (end) . unwrap () ; writeln ! (end , "end_function") . unwrap () ; } BinaryFormat :: Xcoff => { writeln ! (begin , ".align {}" , align_bytes) . unwrap () ; write_linkage (& mut begin) . unwrap () ; if let Visibility :: Hidden = item_data . visibility { } writeln ! (begin , "{asm_name}:") . unwrap () ; writeln ! (end) . unwrap () ; } } (begin , end) }
}

macro_rules! wasm_functype_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function wasm_functype in module {}", module_path!());
    };
}

mkfn!{
    wasm_functype_introspect!();
    #[doc = " The webassembly type signature for the given function."] #[doc = ""] #[doc = " Used by the `.functype` directive on wasm targets."] fn wasm_functype < 'tcx > (tcx : TyCtxt < 'tcx > , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > >) -> String { let mut signature = String :: with_capacity (64) ; let ptr_type = match tcx . data_layout . pointer_size () . bits () { 32 => "i32" , 64 => "i64" , other => bug ! ("wasm pointer size cannot be {other} bits") , } ; let hidden_return = matches ! (fn_abi . ret . mode , PassMode :: Indirect { .. }) ; signature . push ('(') ; if hidden_return { signature . push_str (ptr_type) ; if ! fn_abi . args . is_empty () { signature . push_str (", ") ; } } let mut it = fn_abi . args . iter () . peekable () ; while let Some (arg_abi) = it . next () { wasm_type (& mut signature , arg_abi , ptr_type) ; if it . peek () . is_some () { signature . push_str (", ") ; } } signature . push_str (") -> (") ; if ! hidden_return { wasm_type (& mut signature , & fn_abi . ret , ptr_type) ; } signature . push (')') ; signature }
}

macro_rules! wasm_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function wasm_type in module {}", module_path!());
    };
}

mkfn!{
    wasm_type_introspect!();
    fn wasm_type < 'tcx > (signature : & mut String , arg_abi : & ArgAbi < '_ , Ty < 'tcx > > , ptr_type : & 'static str) { match arg_abi . mode { PassMode :: Ignore => { } PassMode :: Direct (_) => { let direct_type = match arg_abi . layout . backend_repr { BackendRepr :: Scalar (scalar) => wasm_primitive (scalar . primitive () , ptr_type) , BackendRepr :: SimdVector { .. } => "v128" , other => unreachable ! ("unexpected BackendRepr: {:?}" , other) , } ; signature . push_str (direct_type) ; } PassMode :: Pair (_ , _) => match arg_abi . layout . backend_repr { BackendRepr :: ScalarPair (a , b) => { signature . push_str (wasm_primitive (a . primitive () , ptr_type)) ; signature . push_str (", ") ; signature . push_str (wasm_primitive (b . primitive () , ptr_type)) ; } other => unreachable ! ("{other:?}") , } , PassMode :: Cast { pad_i32 , ref cast } => { assert ! (! pad_i32 , "not currently used by wasm calling convention") ; assert ! (cast . prefix [0] . is_none () , "no prefix") ; assert_eq ! (cast . rest . total , arg_abi . layout . size , "single item") ; let wrapped_wasm_type = match cast . rest . unit . kind { RegKind :: Integer => match cast . rest . unit . size . bytes () { ..= 4 => "i32" , ..= 8 => "i64" , _ => ptr_type , } , RegKind :: Float => match cast . rest . unit . size . bytes () { ..= 4 => "f32" , ..= 8 => "f64" , _ => ptr_type , } , RegKind :: Vector => "v128" , } ; signature . push_str (wrapped_wasm_type) ; } PassMode :: Indirect { .. } => signature . push_str (ptr_type) , } }
}

macro_rules! wasm_primitive_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function wasm_primitive in module {}", module_path!());
    };
}

mkfn!{
    wasm_primitive_introspect!();
    fn wasm_primitive (primitive : Primitive , ptr_type : & 'static str) -> & 'static str { match primitive { Primitive :: Int (integer , _) => match integer { Integer :: I8 | Integer :: I16 | Integer :: I32 => "i32" , Integer :: I64 => "i64" , Integer :: I128 => "i64, i64" , } , Primitive :: Float (float) => match float { Float :: F16 | Float :: F32 => "f32" , Float :: F64 => "f64" , Float :: F128 => "i64, i64" , } , Primitive :: Pointer (_) => ptr_type , } }
}