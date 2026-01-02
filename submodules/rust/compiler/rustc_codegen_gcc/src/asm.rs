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
mkuse!{use gccjit :: { LValue , RValue , ToRValue , Type } ;}
mkuse!{use rustc_ast :: ast :: { InlineAsmOptions , InlineAsmTemplatePiece } ;}
mkuse!{use rustc_codegen_ssa :: mir :: operand :: OperandValue ;}
mkuse!{use rustc_codegen_ssa :: mir :: place :: PlaceRef ;}
mkuse!{use rustc_codegen_ssa :: traits :: { AsmBuilderMethods , AsmCodegenMethods , BaseTypeCodegenMethods , BuilderMethods , GlobalAsmOperandRef , InlineAsmOperandRef , } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: Instance ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use rustc_target :: asm :: * ;}
mkuse!{use crate :: builder :: Builder ;}
mkuse!{use crate :: callee :: get_fn ;}
mkuse!{use crate :: context :: CodegenCx ;}
mkuse!{use crate :: errors :: UnwindingInlineAsm ;}
mkuse!{use crate :: type_of :: LayoutGccExt ;}
mkitem!{const ATT_SYNTAX_INS : & str = ".att_syntax noprefix\n\t" ;}
mkitem!{const INTEL_SYNTAX_INS : & str = "\n\t.intel_syntax noprefix" ;}
mkitem!{mkstruct!{struct AsmOutOperand < 'a , 'tcx , 'gcc > { rust_idx : usize , constraint : & 'a str , late : bool , readwrite : bool , tmp_var : LValue < 'gcc > , out_place : Option < PlaceRef < 'tcx , RValue < 'gcc > > > , }}}
mkitem!{mkstruct!{struct AsmInOperand < 'a , 'tcx > { rust_idx : usize , constraint : Cow < 'a , str > , val : RValue < 'tcx > , }}}
mkitem!{mkimpl!{impl AsmOutOperand < '_ , '_ , '_ > { fn to_constraint (& self) -> String { let mut res = String :: with_capacity (self . constraint . len () + self . late as usize + 1) ; let sign = if self . readwrite { '+' } else { '=' } ; res . push (sign) ; if ! self . late { res . push ('&') ; } res . push_str (self . constraint) ; res } }}}
mkitem!{mkenum!{enum ConstraintOrRegister { Constraint (& 'static str) , Register (& 'static str) , }}}
mkitem!{mkimpl!{impl < 'a , 'gcc , 'tcx > AsmBuilderMethods < 'tcx > for Builder < 'a , 'gcc , 'tcx > { fn codegen_inline_asm (& mut self , template : & [InlineAsmTemplatePiece] , rust_operands : & [InlineAsmOperandRef < 'tcx , Self >] , options : InlineAsmOptions , span : & [Span] , instance : Instance < '_ > , dest : Option < Self :: BasicBlock > , _dest_catch_funclet : Option < (Self :: BasicBlock , Option < & Self :: Funclet >) > ,) { if options . contains (InlineAsmOptions :: MAY_UNWIND) { self . sess () . dcx () . create_err (UnwindingInlineAsm { span : span [0] }) . emit () ; return ; } let asm_arch = self . tcx . sess . asm_arch . unwrap () ; let is_x86 = matches ! (asm_arch , InlineAsmArch :: X86 | InlineAsmArch :: X86_64) ; let att_dialect = is_x86 && options . contains (InlineAsmOptions :: ATT_SYNTAX) ; let mut outputs = vec ! [] ; let mut inputs = vec ! [] ; let mut labels = vec ! [] ; let mut clobbers = vec ! [] ; let mut constants_len = 0 ; let mut input_registers = vec ! [] ; for op in rust_operands { if let InlineAsmOperandRef :: In { reg , .. } = * op && let ConstraintOrRegister :: Register (reg_name) = reg_to_gcc (reg) { input_registers . push (reg_name) ; } } for (rust_idx , op) in rust_operands . iter () . enumerate () { match * op { InlineAsmOperandRef :: Out { reg , late , place } => { use ConstraintOrRegister :: * ; let (constraint , ty) = match (reg_to_gcc (reg) , place) { (Constraint (constraint) , Some (place)) => { (constraint , place . layout . gcc_type (self . cx)) } (Constraint (constraint) , None) => { (constraint , dummy_output_type (self . cx , reg . reg_class ())) } (Register (_) , Some (_)) => { continue ; } (Register (reg_name) , None) => { if input_registers . contains (& reg_name) { if ! late { bug ! ("input registers can only be used as lateout registers") ; } ("r" , dummy_output_type (self . cx , reg . reg_class ())) } else { let is_target_supported = reg . reg_class () . supported_types (asm_arch , true) . iter () . any (| & (_ , feature) | { if let Some (feature) = feature { self . tcx . asm_target_features (instance . def_id ()) . contains (& feature) } else { true } } ,) ; if is_target_supported && ! clobbers . contains (& reg_name) { clobbers . push (reg_name) ; } continue ; } } } ; let tmp_var = self . current_func () . new_local (None , ty , "output_register") ; outputs . push (AsmOutOperand { constraint , rust_idx , late , readwrite : false , tmp_var , out_place : place , }) ; } InlineAsmOperandRef :: In { reg , value } => { if let ConstraintOrRegister :: Constraint (constraint) = reg_to_gcc (reg) { inputs . push (AsmInOperand { constraint : Cow :: Borrowed (constraint) , rust_idx , val : value . immediate () , }) ; } else { continue ; } } InlineAsmOperandRef :: InOut { reg , late , in_value , out_place } => { let ConstraintOrRegister :: Constraint (constraint) = reg_to_gcc (reg) else { continue ; } ; let ty = in_value . layout . gcc_type (self . cx) ; let tmp_var = self . current_func () . new_local (None , ty , "output_register") ; let readwrite = out_place . is_none () ; outputs . push (AsmOutOperand { constraint , rust_idx , late , readwrite , tmp_var , out_place , }) ; if ! readwrite { let out_gcc_idx = outputs . len () - 1 ; let constraint = Cow :: Owned (out_gcc_idx . to_string ()) ; inputs . push (AsmInOperand { constraint , rust_idx , val : in_value . immediate () , }) ; } } InlineAsmOperandRef :: Const { ref string } => { constants_len += string . len () + att_dialect as usize ; } InlineAsmOperandRef :: SymFn { instance } => { constants_len += self . tcx . symbol_name (instance) . name . len () ; } InlineAsmOperandRef :: SymStatic { def_id } => { constants_len += self . tcx . symbol_name (Instance :: mono (self . tcx , def_id)) . name . len () ; } InlineAsmOperandRef :: Label { label } => { labels . push (label) ; } } } for (rust_idx , op) in rust_operands . iter () . enumerate () { match * op { InlineAsmOperandRef :: Out { reg , late , place } => { if let ConstraintOrRegister :: Register (reg_name) = reg_to_gcc (reg) { let out_place = if let Some (place) = place { place } else { continue ; } ; let ty = out_place . layout . gcc_type (self . cx) ; let tmp_var = self . current_func () . new_local (None , ty , "output_register") ; tmp_var . set_register_name (reg_name) ; outputs . push (AsmOutOperand { constraint : "r" , rust_idx , late , readwrite : false , tmp_var , out_place : Some (out_place) , }) ; } } InlineAsmOperandRef :: In { reg , value } => { if let ConstraintOrRegister :: Register (reg_name) = reg_to_gcc (reg) { let ty = value . layout . gcc_type (self . cx) ; let reg_var = self . current_func () . new_local (None , ty , "input_register") ; reg_var . set_register_name (reg_name) ; self . llbb () . add_assignment (None , reg_var , value . immediate ()) ; inputs . push (AsmInOperand { constraint : "r" . into () , rust_idx , val : reg_var . to_rvalue () , }) ; } } InlineAsmOperandRef :: InOut { reg , late , in_value , out_place } => { if let ConstraintOrRegister :: Register (reg_name) = reg_to_gcc (reg) { let ty = in_value . layout . gcc_type (self . cx) ; let tmp_var = self . current_func () . new_local (None , ty , "output_register") ; tmp_var . set_register_name (reg_name) ; outputs . push (AsmOutOperand { constraint : "r" , rust_idx , late , readwrite : false , tmp_var , out_place , }) ; let constraint = Cow :: Owned ((outputs . len () - 1) . to_string ()) ; inputs . push (AsmInOperand { constraint , rust_idx , val : in_value . immediate () , }) ; } } InlineAsmOperandRef :: SymFn { instance } => { inputs . push (AsmInOperand { constraint : "X" . into () , rust_idx , val : get_fn (self . cx , instance) . get_address (None) , }) ; } InlineAsmOperandRef :: SymStatic { def_id } => { inputs . push (AsmInOperand { constraint : "X" . into () , rust_idx , val : self . cx . get_static (def_id) . get_address (None) , }) ; } InlineAsmOperandRef :: Const { .. } => { } InlineAsmOperandRef :: Label { .. } => { } } } let mut template_str = String :: with_capacity (estimate_template_length (template , constants_len , att_dialect)) ; if att_dialect { template_str . push_str (ATT_SYNTAX_INS) ; } for piece in template { match * piece { InlineAsmTemplatePiece :: String (ref string) => { for char in string . chars () { let escaped_char = match char { '%' => "%%" , '{' => "%{" , '}' => "%}" , _ => { template_str . push (char) ; continue ; } } ; template_str . push_str (escaped_char) ; } } InlineAsmTemplatePiece :: Placeholder { operand_idx , modifier , span : _ } => { let mut push_to_template = | modifier , gcc_idx | { use std :: fmt :: Write ; template_str . push ('%') ; if let Some (modifier) = modifier { template_str . push (modifier) ; } write ! (template_str , "{}" , gcc_idx) . expect ("pushing to string failed") ; } ; match rust_operands [operand_idx] { InlineAsmOperandRef :: Out { reg , .. } => { let modifier = modifier_to_gcc (asm_arch , reg . reg_class () , modifier) ; let gcc_index = outputs . iter () . position (| op | operand_idx == op . rust_idx) . expect ("wrong rust index") ; push_to_template (modifier , gcc_index) ; } InlineAsmOperandRef :: In { reg , .. } => { let modifier = modifier_to_gcc (asm_arch , reg . reg_class () , modifier) ; let in_gcc_index = inputs . iter () . position (| op | operand_idx == op . rust_idx) . expect ("wrong rust index") ; let gcc_index = in_gcc_index + outputs . len () ; push_to_template (modifier , gcc_index) ; } InlineAsmOperandRef :: InOut { reg , .. } => { let modifier = modifier_to_gcc (asm_arch , reg . reg_class () , modifier) ; let gcc_index = outputs . iter () . position (| op | operand_idx == op . rust_idx) . expect ("wrong rust index") ; push_to_template (modifier , gcc_index) ; } InlineAsmOperandRef :: SymFn { instance } => { let name = self . tcx . symbol_name (instance) . name ; template_str . push_str (name) ; } InlineAsmOperandRef :: SymStatic { def_id } => { let instance = Instance :: mono (self . tcx , def_id) ; let name = self . tcx . symbol_name (instance) . name ; template_str . push_str (name) ; } InlineAsmOperandRef :: Const { ref string } => { template_str . push_str (string) ; } InlineAsmOperandRef :: Label { label } => { let label_gcc_index = labels . iter () . position (| & l | l == label) . expect ("wrong rust index") ; let gcc_index = label_gcc_index + outputs . len () + inputs . len () ; push_to_template (Some ('l') , gcc_index) ; } } } } } if att_dialect { template_str . push_str (INTEL_SYNTAX_INS) ; } let block = self . llbb () ; let extended_asm = if let Some (dest) = dest { assert ! (! labels . is_empty ()) ; block . end_with_extended_asm_goto (None , & template_str , & labels , Some (dest)) } else { block . add_extended_asm (None , & template_str) } ; for op in & outputs { extended_asm . add_output_operand (None , & op . to_constraint () , op . tmp_var) ; } for op in & inputs { extended_asm . add_input_operand (None , & op . constraint , op . val) ; } for clobber in clobbers . iter () { extended_asm . add_clobber (clobber) ; } if ! options . contains (InlineAsmOptions :: PRESERVES_FLAGS) { extended_asm . add_clobber ("cc") ; } if ! options . contains (InlineAsmOptions :: NOMEM) { extended_asm . add_clobber ("memory") ; } if ! options . contains (InlineAsmOptions :: PURE) { extended_asm . set_volatile_flag (true) ; } if ! options . contains (InlineAsmOptions :: NOSTACK) { } if dest . is_none () && options . contains (InlineAsmOptions :: NORETURN) { let builtin_unreachable = self . context . get_builtin_function ("__builtin_unreachable") ; let builtin_unreachable : RValue < 'gcc > = unsafe { std :: mem :: transmute (builtin_unreachable) } ; self . call (self . type_void () , None , None , builtin_unreachable , & [] , None , None) ; } for op in & outputs { if let Some (place) = op . out_place { OperandValue :: Immediate (op . tmp_var . to_rvalue ()) . store (self , place) ; } } } }}}

macro_rules! estimate_template_length_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function estimate_template_length in module {}", module_path!());
    };
}

mkfn!{
    estimate_template_length_introspect!();
    fn estimate_template_length (template : & [InlineAsmTemplatePiece] , constants_len : usize , att_dialect : bool ,) -> usize { let len : usize = template . iter () . map (| piece | { match * piece { InlineAsmTemplatePiece :: String (ref string) => string . len () , InlineAsmTemplatePiece :: Placeholder { .. } => { 3 } } }) . sum () ; let mut res = (len as f32 * 1.05) as usize + constants_len ; if att_dialect { res += INTEL_SYNTAX_INS . len () + ATT_SYNTAX_INS . len () ; } res }
}

macro_rules! reg_to_gcc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function reg_to_gcc in module {}", module_path!());
    };
}

mkfn!{
    reg_to_gcc_introspect!();
    #[doc = " Converts a register class to a GCC constraint code."] fn reg_to_gcc (reg_or_reg_class : InlineAsmRegOrRegClass) -> ConstraintOrRegister { match reg_or_reg_class { InlineAsmRegOrRegClass :: Reg (reg) => { ConstraintOrRegister :: Register (explicit_reg_to_gcc (reg)) } InlineAsmRegOrRegClass :: RegClass (reg_class) => { ConstraintOrRegister :: Constraint (reg_class_to_gcc (reg_class)) } } }
}

macro_rules! explicit_reg_to_gcc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function explicit_reg_to_gcc in module {}", module_path!());
    };
}

mkfn!{
    explicit_reg_to_gcc_introspect!();
    fn explicit_reg_to_gcc (reg : InlineAsmReg) -> & 'static str { match reg { InlineAsmReg :: X86 (reg) => { match reg . reg_class () { X86InlineAsmRegClass :: reg_byte => { reg . name () . trim_end_matches ('b') } _ => match reg . name () { "st(0)" => "st" , name => name , } , } } InlineAsmReg :: Arm (reg) => reg . name () , InlineAsmReg :: AArch64 (reg) => reg . name () , _ => unimplemented ! () , } }
}

macro_rules! reg_class_to_gcc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function reg_class_to_gcc in module {}", module_path!());
    };
}

mkfn!{
    reg_class_to_gcc_introspect!();
    #[doc = " They can be retrieved from https://gcc.gnu.org/onlinedocs/gcc/Machine-Constraints.html"] fn reg_class_to_gcc (reg_class : InlineAsmRegClass) -> & 'static str { match reg_class { InlineAsmRegClass :: AArch64 (AArch64InlineAsmRegClass :: reg) => "r" , InlineAsmRegClass :: AArch64 (AArch64InlineAsmRegClass :: vreg) => "w" , InlineAsmRegClass :: AArch64 (AArch64InlineAsmRegClass :: vreg_low16) => "x" , InlineAsmRegClass :: AArch64 (AArch64InlineAsmRegClass :: preg) => { unreachable ! ("clobber-only") } InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: reg) => "r" , InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: sreg) | InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: dreg_low16) | InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: qreg_low8) | InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: sreg_low16) | InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: dreg_low8) | InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: qreg_low4) | InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: dreg) | InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: qreg) => "t" , InlineAsmRegClass :: Avr (AvrInlineAsmRegClass :: reg) => "r" , InlineAsmRegClass :: Avr (AvrInlineAsmRegClass :: reg_upper) => "d" , InlineAsmRegClass :: Avr (AvrInlineAsmRegClass :: reg_pair) => "r" , InlineAsmRegClass :: Avr (AvrInlineAsmRegClass :: reg_iw) => "w" , InlineAsmRegClass :: Avr (AvrInlineAsmRegClass :: reg_ptr) => "e" , InlineAsmRegClass :: Bpf (BpfInlineAsmRegClass :: reg) => "r" , InlineAsmRegClass :: Bpf (BpfInlineAsmRegClass :: wreg) => "w" , InlineAsmRegClass :: Hexagon (HexagonInlineAsmRegClass :: reg) => "r" , InlineAsmRegClass :: Hexagon (HexagonInlineAsmRegClass :: preg) => { unreachable ! ("clobber-only") } InlineAsmRegClass :: LoongArch (LoongArchInlineAsmRegClass :: reg) => "r" , InlineAsmRegClass :: LoongArch (LoongArchInlineAsmRegClass :: freg) => "f" , InlineAsmRegClass :: M68k (M68kInlineAsmRegClass :: reg) => "r" , InlineAsmRegClass :: M68k (M68kInlineAsmRegClass :: reg_addr) => "a" , InlineAsmRegClass :: M68k (M68kInlineAsmRegClass :: reg_data) => "d" , InlineAsmRegClass :: CSKY (CSKYInlineAsmRegClass :: reg) => "r" , InlineAsmRegClass :: CSKY (CSKYInlineAsmRegClass :: freg) => "f" , InlineAsmRegClass :: Mips (MipsInlineAsmRegClass :: reg) => "d" , InlineAsmRegClass :: Mips (MipsInlineAsmRegClass :: freg) => "f" , InlineAsmRegClass :: Msp430 (Msp430InlineAsmRegClass :: reg) => "r" , InlineAsmRegClass :: Nvptx (NvptxInlineAsmRegClass :: reg16) => "h" , InlineAsmRegClass :: Nvptx (NvptxInlineAsmRegClass :: reg32) => "r" , InlineAsmRegClass :: Nvptx (NvptxInlineAsmRegClass :: reg64) => "l" , InlineAsmRegClass :: PowerPC (PowerPCInlineAsmRegClass :: reg) => "r" , InlineAsmRegClass :: PowerPC (PowerPCInlineAsmRegClass :: reg_nonzero) => "b" , InlineAsmRegClass :: PowerPC (PowerPCInlineAsmRegClass :: freg) => "f" , InlineAsmRegClass :: PowerPC (PowerPCInlineAsmRegClass :: vreg) => "v" , InlineAsmRegClass :: PowerPC (PowerPCInlineAsmRegClass :: cr) | InlineAsmRegClass :: PowerPC (PowerPCInlineAsmRegClass :: xer) => { unreachable ! ("clobber-only") } InlineAsmRegClass :: RiscV (RiscVInlineAsmRegClass :: reg) => "r" , InlineAsmRegClass :: RiscV (RiscVInlineAsmRegClass :: freg) => "f" , InlineAsmRegClass :: RiscV (RiscVInlineAsmRegClass :: vreg) => { unreachable ! ("clobber-only") } InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: reg) => "r" , InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: reg_abcd) => "Q" , InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: reg_byte) => "q" , InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: xmm_reg) | InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: ymm_reg) => "x" , InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: zmm_reg) => "v" , InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: kreg) => "Yk" , InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: kreg0 | X86InlineAsmRegClass :: x87_reg | X86InlineAsmRegClass :: mmx_reg | X86InlineAsmRegClass :: tmm_reg ,) => unreachable ! ("clobber-only") , InlineAsmRegClass :: SpirV (SpirVInlineAsmRegClass :: reg) => { bug ! ("GCC backend does not support SPIR-V") } InlineAsmRegClass :: Wasm (WasmInlineAsmRegClass :: local) => "r" , InlineAsmRegClass :: S390x (S390xInlineAsmRegClass :: reg) => "r" , InlineAsmRegClass :: S390x (S390xInlineAsmRegClass :: reg_addr) => "a" , InlineAsmRegClass :: S390x (S390xInlineAsmRegClass :: freg) => "f" , InlineAsmRegClass :: S390x (S390xInlineAsmRegClass :: vreg) => "v" , InlineAsmRegClass :: S390x (S390xInlineAsmRegClass :: areg) => { unreachable ! ("clobber-only") } InlineAsmRegClass :: Sparc (SparcInlineAsmRegClass :: reg) => "r" , InlineAsmRegClass :: Sparc (SparcInlineAsmRegClass :: yreg) => unreachable ! ("clobber-only") , InlineAsmRegClass :: Err => unreachable ! () , } }
}

macro_rules! dummy_output_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dummy_output_type in module {}", module_path!());
    };
}

mkfn!{
    dummy_output_type_introspect!();
    #[doc = " Type to use for outputs that are discarded. It doesn't really matter what"] #[doc = " the type is, as long as it is valid for the constraint code."] fn dummy_output_type < 'gcc , 'tcx > (cx : & CodegenCx < 'gcc , 'tcx > , reg : InlineAsmRegClass) -> Type < 'gcc > { match reg { InlineAsmRegClass :: AArch64 (AArch64InlineAsmRegClass :: reg) => cx . type_i32 () , InlineAsmRegClass :: AArch64 (AArch64InlineAsmRegClass :: vreg) | InlineAsmRegClass :: AArch64 (AArch64InlineAsmRegClass :: vreg_low16) => { cx . type_vector (cx . type_i64 () , 2) } InlineAsmRegClass :: AArch64 (AArch64InlineAsmRegClass :: preg) => { unreachable ! ("clobber-only") } InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: reg) => cx . type_i32 () , InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: sreg) | InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: sreg_low16) => cx . type_f32 () , InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: dreg) | InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: dreg_low16) | InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: dreg_low8) => cx . type_f64 () , InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: qreg) | InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: qreg_low8) | InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: qreg_low4) => { cx . type_vector (cx . type_i64 () , 2) } InlineAsmRegClass :: Hexagon (HexagonInlineAsmRegClass :: reg) => cx . type_i32 () , InlineAsmRegClass :: Hexagon (HexagonInlineAsmRegClass :: preg) => { unreachable ! ("clobber-only") } InlineAsmRegClass :: LoongArch (LoongArchInlineAsmRegClass :: reg) => cx . type_i32 () , InlineAsmRegClass :: LoongArch (LoongArchInlineAsmRegClass :: freg) => cx . type_f32 () , InlineAsmRegClass :: Mips (MipsInlineAsmRegClass :: reg) => cx . type_i32 () , InlineAsmRegClass :: Mips (MipsInlineAsmRegClass :: freg) => cx . type_f32 () , InlineAsmRegClass :: Nvptx (NvptxInlineAsmRegClass :: reg16) => cx . type_i16 () , InlineAsmRegClass :: Nvptx (NvptxInlineAsmRegClass :: reg32) => cx . type_i32 () , InlineAsmRegClass :: Nvptx (NvptxInlineAsmRegClass :: reg64) => cx . type_i64 () , InlineAsmRegClass :: PowerPC (PowerPCInlineAsmRegClass :: reg) => cx . type_i32 () , InlineAsmRegClass :: PowerPC (PowerPCInlineAsmRegClass :: reg_nonzero) => cx . type_i32 () , InlineAsmRegClass :: PowerPC (PowerPCInlineAsmRegClass :: freg) => cx . type_f64 () , InlineAsmRegClass :: PowerPC (PowerPCInlineAsmRegClass :: vreg) => { cx . type_vector (cx . type_i32 () , 4) } InlineAsmRegClass :: PowerPC (PowerPCInlineAsmRegClass :: cr) | InlineAsmRegClass :: PowerPC (PowerPCInlineAsmRegClass :: xer) => { unreachable ! ("clobber-only") } InlineAsmRegClass :: RiscV (RiscVInlineAsmRegClass :: reg) => cx . type_i32 () , InlineAsmRegClass :: RiscV (RiscVInlineAsmRegClass :: freg) => cx . type_f32 () , InlineAsmRegClass :: RiscV (RiscVInlineAsmRegClass :: vreg) => { unreachable ! ("clobber-only") } InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: reg) | InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: reg_abcd) => cx . type_i32 () , InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: reg_byte) => cx . type_i8 () , InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: xmm_reg) | InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: ymm_reg) | InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: zmm_reg) => cx . type_f32 () , InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: kreg) => cx . type_i16 () , InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: x87_reg) | InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: mmx_reg) | InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: kreg0) | InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: tmm_reg) => { unreachable ! ("clobber-only") } InlineAsmRegClass :: Wasm (WasmInlineAsmRegClass :: local) => cx . type_i32 () , InlineAsmRegClass :: Bpf (BpfInlineAsmRegClass :: reg) => cx . type_i64 () , InlineAsmRegClass :: Bpf (BpfInlineAsmRegClass :: wreg) => cx . type_i32 () , InlineAsmRegClass :: Avr (AvrInlineAsmRegClass :: reg) => cx . type_i8 () , InlineAsmRegClass :: Avr (AvrInlineAsmRegClass :: reg_upper) => cx . type_i8 () , InlineAsmRegClass :: Avr (AvrInlineAsmRegClass :: reg_pair) => cx . type_i16 () , InlineAsmRegClass :: Avr (AvrInlineAsmRegClass :: reg_iw) => cx . type_i16 () , InlineAsmRegClass :: Avr (AvrInlineAsmRegClass :: reg_ptr) => cx . type_i16 () , InlineAsmRegClass :: S390x (S390xInlineAsmRegClass :: reg | S390xInlineAsmRegClass :: reg_addr ,) => cx . type_i32 () , InlineAsmRegClass :: S390x (S390xInlineAsmRegClass :: freg) => cx . type_f64 () , InlineAsmRegClass :: S390x (S390xInlineAsmRegClass :: vreg) => cx . type_vector (cx . type_i64 () , 2) , InlineAsmRegClass :: S390x (S390xInlineAsmRegClass :: areg) => { unreachable ! ("clobber-only") } InlineAsmRegClass :: Sparc (SparcInlineAsmRegClass :: reg) => cx . type_i32 () , InlineAsmRegClass :: Sparc (SparcInlineAsmRegClass :: yreg) => unreachable ! ("clobber-only") , InlineAsmRegClass :: Msp430 (Msp430InlineAsmRegClass :: reg) => cx . type_i16 () , InlineAsmRegClass :: M68k (M68kInlineAsmRegClass :: reg) => cx . type_i32 () , InlineAsmRegClass :: M68k (M68kInlineAsmRegClass :: reg_addr) => cx . type_i32 () , InlineAsmRegClass :: M68k (M68kInlineAsmRegClass :: reg_data) => cx . type_i32 () , InlineAsmRegClass :: CSKY (CSKYInlineAsmRegClass :: reg) => cx . type_i32 () , InlineAsmRegClass :: CSKY (CSKYInlineAsmRegClass :: freg) => cx . type_f32 () , InlineAsmRegClass :: SpirV (SpirVInlineAsmRegClass :: reg) => { bug ! ("GCC backend does not support SPIR-V") } InlineAsmRegClass :: Err => unreachable ! () , } }
}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > AsmCodegenMethods < 'tcx > for CodegenCx < 'gcc , 'tcx > { fn codegen_global_asm (& mut self , template : & [InlineAsmTemplatePiece] , operands : & [GlobalAsmOperandRef < 'tcx >] , options : InlineAsmOptions , _line_spans : & [Span] ,) { let asm_arch = self . tcx . sess . asm_arch . unwrap () ; let att_dialect = matches ! (asm_arch , InlineAsmArch :: X86 | InlineAsmArch :: X86_64) && options . contains (InlineAsmOptions :: ATT_SYNTAX) ; let mut template_str = ".pushsection .text\n" . to_owned () ; if att_dialect { template_str . push_str (".att_syntax\n") ; } for piece in template { match * piece { InlineAsmTemplatePiece :: String (ref string) => { let mut index = 0 ; while index < string . len () { let comment_index = string [index ..] . find ("//") . map (| comment_index | comment_index + index) . unwrap_or (string . len ()) ; template_str . push_str (& string [index .. comment_index]) ; index = string [comment_index ..] . find ('\n') . map (| index | index + comment_index) . unwrap_or (string . len ()) ; } } InlineAsmTemplatePiece :: Placeholder { operand_idx , modifier : _ , span : _ } => { match operands [operand_idx] { GlobalAsmOperandRef :: Const { ref string } => { template_str . push_str (string) ; } GlobalAsmOperandRef :: SymFn { instance } => { let function = get_fn (self , instance) ; self . add_used_function (function) ; let name = self . tcx . symbol_name (instance) . name ; template_str . push_str (name) ; } GlobalAsmOperandRef :: SymStatic { def_id } => { let instance = Instance :: mono (self . tcx , def_id) ; let name = self . tcx . symbol_name (instance) . name ; template_str . push_str (name) ; } } } } } if att_dialect { template_str . push_str ("\n\t.intel_syntax noprefix") ; } template_str . push_str ("\n.popsection") ; self . context . add_top_level_asm (None , & template_str) ; } fn mangled_name (& self , instance : Instance < 'tcx >) -> String { self . tcx . symbol_name (instance) . name . to_string () } }}}

macro_rules! modifier_to_gcc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function modifier_to_gcc in module {}", module_path!());
    };
}

mkfn!{
    modifier_to_gcc_introspect!();
    fn modifier_to_gcc (arch : InlineAsmArch , reg : InlineAsmRegClass , modifier : Option < char > ,) -> Option < char > { match reg { InlineAsmRegClass :: AArch64 (AArch64InlineAsmRegClass :: reg) => modifier , InlineAsmRegClass :: AArch64 (AArch64InlineAsmRegClass :: vreg) | InlineAsmRegClass :: AArch64 (AArch64InlineAsmRegClass :: vreg_low16) => { if modifier == Some ('v') { None } else { modifier } } InlineAsmRegClass :: AArch64 (AArch64InlineAsmRegClass :: preg) => { unreachable ! ("clobber-only") } InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: reg) => None , InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: sreg) | InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: sreg_low16) => None , InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: dreg) | InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: dreg_low16) | InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: dreg_low8) => Some ('P') , InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: qreg) | InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: qreg_low8) | InlineAsmRegClass :: Arm (ArmInlineAsmRegClass :: qreg_low4) => { if modifier . is_none () { Some ('q') } else { modifier } } InlineAsmRegClass :: Hexagon (_) => None , InlineAsmRegClass :: LoongArch (_) => None , InlineAsmRegClass :: Mips (_) => None , InlineAsmRegClass :: Nvptx (_) => None , InlineAsmRegClass :: PowerPC (_) => None , InlineAsmRegClass :: RiscV (RiscVInlineAsmRegClass :: reg) | InlineAsmRegClass :: RiscV (RiscVInlineAsmRegClass :: freg) => None , InlineAsmRegClass :: RiscV (RiscVInlineAsmRegClass :: vreg) => { unreachable ! ("clobber-only") } InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: reg) | InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: reg_abcd) => match modifier { None => { if arch == InlineAsmArch :: X86_64 { Some ('q') } else { Some ('k') } } Some ('l') => Some ('b') , Some ('h') => Some ('h') , Some ('x') => Some ('w') , Some ('e') => Some ('k') , Some ('r') => Some ('q') , _ => unreachable ! () , } , InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: reg_byte) => None , InlineAsmRegClass :: X86 (reg @ X86InlineAsmRegClass :: xmm_reg) | InlineAsmRegClass :: X86 (reg @ X86InlineAsmRegClass :: ymm_reg) | InlineAsmRegClass :: X86 (reg @ X86InlineAsmRegClass :: zmm_reg) => match (reg , modifier) { (X86InlineAsmRegClass :: xmm_reg , None) => Some ('x') , (X86InlineAsmRegClass :: ymm_reg , None) => Some ('t') , (X86InlineAsmRegClass :: zmm_reg , None) => Some ('g') , (_ , Some ('x')) => Some ('x') , (_ , Some ('y')) => Some ('t') , (_ , Some ('z')) => Some ('g') , _ => unreachable ! () , } , InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: kreg) => None , InlineAsmRegClass :: X86 (X86InlineAsmRegClass :: x87_reg | X86InlineAsmRegClass :: mmx_reg | X86InlineAsmRegClass :: kreg0 | X86InlineAsmRegClass :: tmm_reg ,) => { unreachable ! ("clobber-only") } InlineAsmRegClass :: Wasm (WasmInlineAsmRegClass :: local) => None , InlineAsmRegClass :: Bpf (_) => None , InlineAsmRegClass :: Avr (AvrInlineAsmRegClass :: reg_pair) | InlineAsmRegClass :: Avr (AvrInlineAsmRegClass :: reg_iw) | InlineAsmRegClass :: Avr (AvrInlineAsmRegClass :: reg_ptr) => match modifier { Some ('h') => Some ('B') , Some ('l') => Some ('A') , _ => None , } , InlineAsmRegClass :: Avr (_) => None , InlineAsmRegClass :: S390x (_) => None , InlineAsmRegClass :: Sparc (_) => None , InlineAsmRegClass :: Msp430 (_) => None , InlineAsmRegClass :: M68k (_) => None , InlineAsmRegClass :: CSKY (_) => None , InlineAsmRegClass :: SpirV (SpirVInlineAsmRegClass :: reg) => { bug ! ("LLVM backend does not support SPIR-V") } InlineAsmRegClass :: Err => unreachable ! () , } }
}