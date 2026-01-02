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
mkuse!{use std :: borrow :: Borrow ;}
mkuse!{use std :: cmp ;}
mkuse!{use libc :: c_uint ;}
mkuse!{use rustc_abi :: { ArmCall , BackendRepr , CanonAbi , HasDataLayout , InterruptKind , Primitive , Reg , RegKind , Size , X86Call , } ;}
mkuse!{use rustc_codegen_ssa :: MemFlags ;}
mkuse!{use rustc_codegen_ssa :: mir :: operand :: { OperandRef , OperandValue } ;}
mkuse!{use rustc_codegen_ssa :: mir :: place :: { PlaceRef , PlaceValue } ;}
mkuse!{use rustc_codegen_ssa :: traits :: * ;}
mkuse!{use rustc_middle :: ty :: Ty ;}
mkuse!{use rustc_middle :: ty :: layout :: LayoutOf ;}
mkuse!{use rustc_middle :: { bug , ty } ;}
mkuse!{use rustc_session :: config ;}
mkuse!{use rustc_target :: callconv :: { ArgAbi , ArgAttribute , ArgAttributes , ArgExtension , CastTarget , FnAbi , PassMode , } ;}
mkuse!{use rustc_target :: spec :: SanitizerSet ;}
mkuse!{use smallvec :: SmallVec ;}
mkuse!{use crate :: attributes :: { self , llfn_attrs_from_instance } ;}
mkuse!{use crate :: builder :: Builder ;}
mkuse!{use crate :: context :: CodegenCx ;}
mkuse!{use crate :: llvm :: { self , Attribute , AttributePlace } ;}
mkuse!{use crate :: llvm_util ;}
mkuse!{use crate :: type_ :: Type ;}
mkuse!{use crate :: type_of :: LayoutLlvmExt ;}
mkuse!{use crate :: value :: Value ;}
mkitem!{mktrait!{trait ArgAttributesExt { fn apply_attrs_to_llfn (& self , idx : AttributePlace , cx : & CodegenCx < '_ , '_ > , llfn : & Value) ; fn apply_attrs_to_callsite (& self , idx : AttributePlace , cx : & CodegenCx < '_ , '_ > , callsite : & Value ,) ; }}}
mkitem!{const ABI_AFFECTING_ATTRIBUTES : [(ArgAttribute , llvm :: AttributeKind) ; 1] = [(ArgAttribute :: InReg , llvm :: AttributeKind :: InReg)] ;}
mkitem!{const OPTIMIZATION_ATTRIBUTES : [(ArgAttribute , llvm :: AttributeKind) ; 6] = [(ArgAttribute :: NoAlias , llvm :: AttributeKind :: NoAlias) , (ArgAttribute :: CapturesAddress , llvm :: AttributeKind :: CapturesAddress) , (ArgAttribute :: NonNull , llvm :: AttributeKind :: NonNull) , (ArgAttribute :: ReadOnly , llvm :: AttributeKind :: ReadOnly) , (ArgAttribute :: NoUndef , llvm :: AttributeKind :: NoUndef) , (ArgAttribute :: CapturesReadOnly , llvm :: AttributeKind :: CapturesReadOnly) ,] ;}

macro_rules! get_attrs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_attrs in module {}", module_path!());
    };
}

mkfn!{
    get_attrs_introspect!();
    fn get_attrs < 'll > (this : & ArgAttributes , cx : & CodegenCx < 'll , '_ >) -> SmallVec < [& 'll Attribute ; 8] > { let mut regular = this . regular ; let mut attrs = SmallVec :: new () ; for (attr , llattr) in ABI_AFFECTING_ATTRIBUTES { if regular . contains (attr) { attrs . push (llattr . create_attr (cx . llcx)) ; } } if let Some (align) = this . pointee_align { attrs . push (llvm :: CreateAlignmentAttr (cx . llcx , align . bytes ())) ; } match this . arg_ext { ArgExtension :: None => { } ArgExtension :: Zext => attrs . push (llvm :: AttributeKind :: ZExt . create_attr (cx . llcx)) , ArgExtension :: Sext => attrs . push (llvm :: AttributeKind :: SExt . create_attr (cx . llcx)) , } if cx . sess () . opts . optimize != config :: OptLevel :: No { let deref = this . pointee_size . bytes () ; if deref != 0 { if regular . contains (ArgAttribute :: NonNull) { attrs . push (llvm :: CreateDereferenceableAttr (cx . llcx , deref)) ; } else { attrs . push (llvm :: CreateDereferenceableOrNullAttr (cx . llcx , deref)) ; } regular -= ArgAttribute :: NonNull ; } for (attr , llattr) in OPTIMIZATION_ATTRIBUTES { if regular . contains (attr) { if (attr == ArgAttribute :: CapturesReadOnly || attr == ArgAttribute :: CapturesAddress) && llvm_util :: get_version () < (21 , 0 , 0) { continue ; } attrs . push (llattr . create_attr (cx . llcx)) ; } } } else if cx . tcx . sess . opts . unstable_opts . sanitizer . contains (SanitizerSet :: MEMORY) { if regular . contains (ArgAttribute :: NoUndef) { attrs . push (llvm :: AttributeKind :: NoUndef . create_attr (cx . llcx)) ; } } attrs }
}
mkitem!{mkimpl!{impl ArgAttributesExt for ArgAttributes { fn apply_attrs_to_llfn (& self , idx : AttributePlace , cx : & CodegenCx < '_ , '_ > , llfn : & Value) { let attrs = get_attrs (self , cx) ; attributes :: apply_to_llfn (llfn , idx , & attrs) ; } fn apply_attrs_to_callsite (& self , idx : AttributePlace , cx : & CodegenCx < '_ , '_ > , callsite : & Value ,) { let attrs = get_attrs (self , cx) ; attributes :: apply_to_callsite (callsite , idx , & attrs) ; } }}}
mkitem!{mktrait!{pub (crate) trait LlvmType { fn llvm_type < 'll > (& self , cx : & CodegenCx < 'll , '_ >) -> & 'll Type ; }}}
mkitem!{mkimpl!{impl LlvmType for Reg { fn llvm_type < 'll > (& self , cx : & CodegenCx < 'll , '_ >) -> & 'll Type { match self . kind { RegKind :: Integer => cx . type_ix (self . size . bits ()) , RegKind :: Float => match self . size . bits () { 16 => cx . type_f16 () , 32 => cx . type_f32 () , 64 => cx . type_f64 () , 128 => cx . type_f128 () , _ => bug ! ("unsupported float: {:?}" , self) , } , RegKind :: Vector => cx . type_vector (cx . type_i8 () , self . size . bytes ()) , } } }}}
mkitem!{mkimpl!{impl LlvmType for CastTarget { fn llvm_type < 'll > (& self , cx : & CodegenCx < 'll , '_ >) -> & 'll Type { let rest_ll_unit = self . rest . unit . llvm_type (cx) ; let rest_count = if self . rest . total == Size :: ZERO { 0 } else { assert_ne ! (self . rest . unit . size , Size :: ZERO , "total size {:?} cannot be divided into units of zero size" , self . rest . total) ; if ! self . rest . total . bytes () . is_multiple_of (self . rest . unit . size . bytes ()) { assert_eq ! (self . rest . unit . kind , RegKind :: Integer , "only int regs can be split") ; } self . rest . total . bytes () . div_ceil (self . rest . unit . size . bytes ()) } ; if self . prefix . iter () . all (| x | x . is_none ()) { if rest_count == 1 && (! self . rest . is_consecutive || self . rest . unit != Reg :: i128 ()) { return rest_ll_unit ; } return cx . type_array (rest_ll_unit , rest_count) ; } let prefix_args = self . prefix . iter () . flat_map (| option_reg | option_reg . map (| reg | reg . llvm_type (cx))) ; let rest_args = (0 .. rest_count) . map (| _ | rest_ll_unit) ; let args : Vec < _ > = prefix_args . chain (rest_args) . collect () ; cx . type_struct (& args , false) } }}}
mkitem!{mktrait!{trait ArgAbiExt < 'll , 'tcx > { fn store (& self , bx : & mut Builder < '_ , 'll , 'tcx > , val : & 'll Value , dst : PlaceRef < 'tcx , & 'll Value > ,) ; fn store_fn_arg (& self , bx : & mut Builder < '_ , 'll , 'tcx > , idx : & mut usize , dst : PlaceRef < 'tcx , & 'll Value > ,) ; }}}
mkitem!{mkimpl!{impl < 'll , 'tcx > ArgAbiExt < 'll , 'tcx > for ArgAbi < 'tcx , Ty < 'tcx > > { #[doc = " Stores a direct/indirect value described by this ArgAbi into a"] #[doc = " place for the original Rust type of this argument/return."] #[doc = " Can be used for both storing formal arguments into Rust variables"] #[doc = " or results of call/invoke instructions into their destinations."] fn store (& self , bx : & mut Builder < '_ , 'll , 'tcx > , val : & 'll Value , dst : PlaceRef < 'tcx , & 'll Value > ,) { match & self . mode { PassMode :: Ignore => { } PassMode :: Indirect { attrs , meta_attrs : None , on_stack : _ } => { let align = attrs . pointee_align . unwrap_or (self . layout . align . abi) ; OperandValue :: Ref (PlaceValue :: new_sized (val , align)) . store (bx , dst) ; } PassMode :: Indirect { attrs : _ , meta_attrs : Some (_) , on_stack : _ } => { bug ! ("unsized `ArgAbi` must be handled through `store_fn_arg`") ; } PassMode :: Cast { cast , pad_i32 : _ } => { let scratch_size = cast . size (bx) ; let scratch_align = cast . align (bx) ; let copy_bytes = cmp :: min (cast . unaligned_size (bx) . bytes () , self . layout . size . bytes ()) ; let llscratch = bx . alloca (scratch_size , scratch_align) ; bx . lifetime_start (llscratch , scratch_size) ; rustc_codegen_ssa :: mir :: store_cast (bx , cast , val , llscratch , scratch_align) ; bx . memcpy (dst . val . llval , self . layout . align . abi , llscratch , scratch_align , bx . const_usize (copy_bytes) , MemFlags :: empty () ,) ; bx . lifetime_end (llscratch , scratch_size) ; } _ => { OperandRef :: from_immediate_or_packed_pair (bx , val , self . layout) . val . store (bx , dst) ; } } } fn store_fn_arg (& self , bx : & mut Builder < '_ , 'll , 'tcx > , idx : & mut usize , dst : PlaceRef < 'tcx , & 'll Value > ,) { let mut next = | | { let val = llvm :: get_param (bx . llfn () , * idx as c_uint) ; * idx += 1 ; val } ; match self . mode { PassMode :: Ignore => { } PassMode :: Pair (..) => { OperandValue :: Pair (next () , next ()) . store (bx , dst) ; } PassMode :: Indirect { attrs : _ , meta_attrs : Some (_) , on_stack : _ } => { let place_val = PlaceValue { llval : next () , llextra : Some (next ()) , align : self . layout . align . abi , } ; OperandValue :: Ref (place_val) . store (bx , dst) ; } PassMode :: Direct (_) | PassMode :: Indirect { attrs : _ , meta_attrs : None , on_stack : _ } | PassMode :: Cast { .. } => { let next_arg = next () ; self . store (bx , next_arg , dst) ; } } } }}}
mkitem!{mkimpl!{impl < 'll , 'tcx > ArgAbiBuilderMethods < 'tcx > for Builder < '_ , 'll , 'tcx > { fn store_fn_arg (& mut self , arg_abi : & ArgAbi < 'tcx , Ty < 'tcx > > , idx : & mut usize , dst : PlaceRef < 'tcx , Self :: Value > ,) { arg_abi . store_fn_arg (self , idx , dst) } fn store_arg (& mut self , arg_abi : & ArgAbi < 'tcx , Ty < 'tcx > > , val : & 'll Value , dst : PlaceRef < 'tcx , & 'll Value > ,) { arg_abi . store (self , val , dst) } }}}
mkitem!{mktrait!{pub (crate) trait FnAbiLlvmExt < 'll , 'tcx > { fn llvm_type (& self , cx : & CodegenCx < 'll , 'tcx >) -> & 'll Type ; fn ptr_to_llvm_type (& self , cx : & CodegenCx < 'll , 'tcx >) -> & 'll Type ; fn llvm_cconv (& self , cx : & CodegenCx < 'll , 'tcx >) -> llvm :: CallConv ; #[doc = " Apply attributes to a function declaration/definition."] fn apply_attrs_llfn (& self , cx : & CodegenCx < 'll , 'tcx > , llfn : & 'll Value , instance : Option < ty :: Instance < 'tcx > > ,) ; #[doc = " Apply attributes to a function call."] fn apply_attrs_callsite (& self , bx : & mut Builder < '_ , 'll , 'tcx > , callsite : & 'll Value) ; }}}
mkitem!{mkimpl!{impl < 'll , 'tcx > FnAbiLlvmExt < 'll , 'tcx > for FnAbi < 'tcx , Ty < 'tcx > > { fn llvm_type (& self , cx : & CodegenCx < 'll , 'tcx >) -> & 'll Type { let args = if self . c_variadic { & self . args [.. self . fixed_count as usize] } else { & self . args } ; let mut llargument_tys = Vec :: with_capacity (self . args . len () + if let PassMode :: Indirect { .. } = self . ret . mode { 1 } else { 0 } ,) ; let llreturn_ty = match & self . ret . mode { PassMode :: Ignore => cx . type_void () , PassMode :: Direct (_) | PassMode :: Pair (..) => self . ret . layout . immediate_llvm_type (cx) , PassMode :: Cast { cast , pad_i32 : _ } => cast . llvm_type (cx) , PassMode :: Indirect { .. } => { llargument_tys . push (cx . type_ptr ()) ; cx . type_void () } } ; for arg in args { let llarg_ty = match & arg . mode { PassMode :: Ignore => continue , PassMode :: Direct (_) => { arg . layout . immediate_llvm_type (cx) } PassMode :: Pair (..) => { llargument_tys . push (arg . layout . scalar_pair_element_llvm_type (cx , 0 , true)) ; llargument_tys . push (arg . layout . scalar_pair_element_llvm_type (cx , 1 , true)) ; continue ; } PassMode :: Indirect { attrs : _ , meta_attrs : Some (_) , on_stack : _ } => { let ptr_ty = Ty :: new_mut_ptr (cx . tcx , arg . layout . ty) ; let ptr_layout = cx . layout_of (ptr_ty) ; llargument_tys . push (ptr_layout . scalar_pair_element_llvm_type (cx , 0 , true)) ; llargument_tys . push (ptr_layout . scalar_pair_element_llvm_type (cx , 1 , true)) ; continue ; } PassMode :: Indirect { attrs : _ , meta_attrs : None , on_stack : _ } => cx . type_ptr () , PassMode :: Cast { cast , pad_i32 } => { if * pad_i32 { llargument_tys . push (Reg :: i32 () . llvm_type (cx)) ; } cast . llvm_type (cx) } } ; llargument_tys . push (llarg_ty) ; } if self . c_variadic { cx . type_variadic_func (& llargument_tys , llreturn_ty) } else { cx . type_func (& llargument_tys , llreturn_ty) } } fn ptr_to_llvm_type (& self , cx : & CodegenCx < 'll , 'tcx >) -> & 'll Type { cx . type_ptr_ext (cx . data_layout () . instruction_address_space) } fn llvm_cconv (& self , cx : & CodegenCx < 'll , 'tcx >) -> llvm :: CallConv { llvm :: CallConv :: from_conv (self . conv , cx . tcx . sess . target . arch . borrow ()) } fn apply_attrs_llfn (& self , cx : & CodegenCx < 'll , 'tcx > , llfn : & 'll Value , instance : Option < ty :: Instance < 'tcx > > ,) { let mut func_attrs = SmallVec :: < [_ ; 3] > :: new () ; if self . ret . layout . is_uninhabited () { func_attrs . push (llvm :: AttributeKind :: NoReturn . create_attr (cx . llcx)) ; } if ! self . can_unwind { func_attrs . push (llvm :: AttributeKind :: NoUnwind . create_attr (cx . llcx)) ; } match self . conv { CanonAbi :: Interrupt (InterruptKind :: RiscvMachine) => { func_attrs . push (llvm :: CreateAttrStringValue (cx . llcx , "interrupt" , "machine")) } CanonAbi :: Interrupt (InterruptKind :: RiscvSupervisor) => { func_attrs . push (llvm :: CreateAttrStringValue (cx . llcx , "interrupt" , "supervisor")) } CanonAbi :: Arm (ArmCall :: CCmseNonSecureEntry) => { func_attrs . push (llvm :: CreateAttrString (cx . llcx , "cmse_nonsecure_entry")) } _ => () , } attributes :: apply_to_llfn (llfn , llvm :: AttributePlace :: Function , & { func_attrs }) ; let mut i = 0 ; let mut apply = | attrs : & ArgAttributes | { attrs . apply_attrs_to_llfn (llvm :: AttributePlace :: Argument (i) , cx , llfn) ; i += 1 ; i - 1 } ; let apply_range_attr = | idx : AttributePlace , scalar : rustc_abi :: Scalar | { if cx . sess () . opts . optimize != config :: OptLevel :: No && matches ! (scalar . primitive () , Primitive :: Int (..)) && ! scalar . is_bool () && ! scalar . is_always_valid (cx) { attributes :: apply_to_llfn (llfn , idx , & [llvm :: CreateRangeAttr (cx . llcx , scalar . size (cx) , scalar . valid_range (cx))] ,) ; } } ; match & self . ret . mode { PassMode :: Direct (attrs) => { attrs . apply_attrs_to_llfn (llvm :: AttributePlace :: ReturnValue , cx , llfn) ; if let BackendRepr :: Scalar (scalar) = self . ret . layout . backend_repr { apply_range_attr (llvm :: AttributePlace :: ReturnValue , scalar) ; } } PassMode :: Indirect { attrs , meta_attrs : _ , on_stack } => { assert ! (! on_stack) ; let i = apply (attrs) ; let sret = llvm :: CreateStructRetAttr (cx . llcx , cx . type_array (cx . type_i8 () , self . ret . layout . size . bytes ()) ,) ; attributes :: apply_to_llfn (llfn , llvm :: AttributePlace :: Argument (i) , & [sret]) ; if cx . sess () . opts . optimize != config :: OptLevel :: No { attributes :: apply_to_llfn (llfn , llvm :: AttributePlace :: Argument (i) , & [llvm :: AttributeKind :: Writable . create_attr (cx . llcx) , llvm :: AttributeKind :: DeadOnUnwind . create_attr (cx . llcx) ,] ,) ; } } PassMode :: Cast { cast , pad_i32 : _ } => { cast . attrs . apply_attrs_to_llfn (llvm :: AttributePlace :: ReturnValue , cx , llfn) ; } _ => { } } for arg in self . args . iter () { match & arg . mode { PassMode :: Ignore => { } PassMode :: Indirect { attrs , meta_attrs : None , on_stack : true } => { let i = apply (attrs) ; let byval = llvm :: CreateByValAttr (cx . llcx , cx . type_array (cx . type_i8 () , arg . layout . size . bytes ()) ,) ; attributes :: apply_to_llfn (llfn , llvm :: AttributePlace :: Argument (i) , & [byval]) ; } PassMode :: Direct (attrs) => { let i = apply (attrs) ; if let BackendRepr :: Scalar (scalar) = arg . layout . backend_repr { apply_range_attr (llvm :: AttributePlace :: Argument (i) , scalar) ; } } PassMode :: Indirect { attrs , meta_attrs : None , on_stack : false } => { let i = apply (attrs) ; if cx . sess () . opts . optimize != config :: OptLevel :: No && llvm_util :: get_version () >= (21 , 0 , 0) { attributes :: apply_to_llfn (llfn , llvm :: AttributePlace :: Argument (i) , & [llvm :: AttributeKind :: DeadOnReturn . create_attr (cx . llcx)] ,) ; } } PassMode :: Indirect { attrs , meta_attrs : Some (meta_attrs) , on_stack } => { assert ! (! on_stack) ; apply (attrs) ; apply (meta_attrs) ; } PassMode :: Pair (a , b) => { let i = apply (a) ; let ii = apply (b) ; if let BackendRepr :: ScalarPair (scalar_a , scalar_b) = arg . layout . backend_repr { apply_range_attr (llvm :: AttributePlace :: Argument (i) , scalar_a) ; apply_range_attr (llvm :: AttributePlace :: Argument (ii) , scalar_b) ; } } PassMode :: Cast { cast , pad_i32 } => { if * pad_i32 { apply (& ArgAttributes :: new ()) ; } apply (& cast . attrs) ; } } } if let Some (instance) = instance { llfn_attrs_from_instance (cx , llfn , instance) ; } } fn apply_attrs_callsite (& self , bx : & mut Builder < '_ , 'll , 'tcx > , callsite : & 'll Value) { let mut func_attrs = SmallVec :: < [_ ; 2] > :: new () ; if self . ret . layout . is_uninhabited () { func_attrs . push (llvm :: AttributeKind :: NoReturn . create_attr (bx . cx . llcx)) ; } if ! self . can_unwind { func_attrs . push (llvm :: AttributeKind :: NoUnwind . create_attr (bx . cx . llcx)) ; } attributes :: apply_to_callsite (callsite , llvm :: AttributePlace :: Function , & { func_attrs }) ; let mut i = 0 ; let mut apply = | cx : & CodegenCx < '_ , '_ > , attrs : & ArgAttributes | { attrs . apply_attrs_to_callsite (llvm :: AttributePlace :: Argument (i) , cx , callsite) ; i += 1 ; i - 1 } ; match & self . ret . mode { PassMode :: Direct (attrs) => { attrs . apply_attrs_to_callsite (llvm :: AttributePlace :: ReturnValue , bx . cx , callsite) ; } PassMode :: Indirect { attrs , meta_attrs : _ , on_stack } => { assert ! (! on_stack) ; let i = apply (bx . cx , attrs) ; let sret = llvm :: CreateStructRetAttr (bx . cx . llcx , bx . cx . type_array (bx . cx . type_i8 () , self . ret . layout . size . bytes ()) ,) ; attributes :: apply_to_callsite (callsite , llvm :: AttributePlace :: Argument (i) , & [sret]) ; } PassMode :: Cast { cast , pad_i32 : _ } => { cast . attrs . apply_attrs_to_callsite (llvm :: AttributePlace :: ReturnValue , bx . cx , callsite ,) ; } _ => { } } for arg in self . args . iter () { match & arg . mode { PassMode :: Ignore => { } PassMode :: Indirect { attrs , meta_attrs : None , on_stack : true } => { let i = apply (bx . cx , attrs) ; let byval = llvm :: CreateByValAttr (bx . cx . llcx , bx . cx . type_array (bx . cx . type_i8 () , arg . layout . size . bytes ()) ,) ; attributes :: apply_to_callsite (callsite , llvm :: AttributePlace :: Argument (i) , & [byval] ,) ; } PassMode :: Direct (attrs) | PassMode :: Indirect { attrs , meta_attrs : None , on_stack : false } => { apply (bx . cx , attrs) ; } PassMode :: Indirect { attrs , meta_attrs : Some (meta_attrs) , on_stack : _ } => { apply (bx . cx , attrs) ; apply (bx . cx , meta_attrs) ; } PassMode :: Pair (a , b) => { apply (bx . cx , a) ; apply (bx . cx , b) ; } PassMode :: Cast { cast , pad_i32 } => { if * pad_i32 { apply (bx . cx , & ArgAttributes :: new ()) ; } apply (bx . cx , & cast . attrs) ; } } } let cconv = self . llvm_cconv (& bx . cx) ; if cconv != llvm :: CCallConv { llvm :: SetInstructionCallConv (callsite , cconv) ; } if self . conv == CanonAbi :: Arm (ArmCall :: CCmseNonSecureCall) { let cmse_nonsecure_call = llvm :: CreateAttrString (bx . cx . llcx , "cmse_nonsecure_call") ; attributes :: apply_to_callsite (callsite , llvm :: AttributePlace :: Function , & [cmse_nonsecure_call] ,) ; } let element_type_index = unsafe { llvm :: LLVMRustGetElementTypeArgIndex (callsite) } ; if element_type_index >= 0 { let arg_ty = self . args [element_type_index as usize] . layout . ty ; let pointee_ty = arg_ty . builtin_deref (true) . expect ("Must be pointer argument") ; let element_type_attr = unsafe { llvm :: LLVMRustCreateElementTypeAttr (bx . llcx , bx . layout_of (pointee_ty) . llvm_type (bx)) } ; attributes :: apply_to_callsite (callsite , llvm :: AttributePlace :: Argument (element_type_index as u32) , & [element_type_attr] ,) ; } } }}}
mkitem!{mkimpl!{impl AbiBuilderMethods for Builder < '_ , '_ , '_ > { fn get_param (& mut self , index : usize) -> Self :: Value { llvm :: get_param (self . llfn () , index as c_uint) } }}}
mkitem!{mkimpl!{impl llvm :: CallConv { pub (crate) fn from_conv (conv : CanonAbi , arch : & str) -> Self { match conv { CanonAbi :: C | CanonAbi :: Rust => llvm :: CCallConv , CanonAbi :: RustCold => llvm :: PreserveMost , CanonAbi :: Custom => llvm :: CCallConv , CanonAbi :: GpuKernel => { if arch == "amdgpu" { llvm :: AmdgpuKernel } else if arch == "nvptx64" { llvm :: PtxKernel } else { panic ! ("Architecture {arch} does not support GpuKernel calling convention") ; } } CanonAbi :: Interrupt (interrupt_kind) => match interrupt_kind { InterruptKind :: Avr => llvm :: AvrInterrupt , InterruptKind :: AvrNonBlocking => llvm :: AvrNonBlockingInterrupt , InterruptKind :: Msp430 => llvm :: Msp430Intr , InterruptKind :: RiscvMachine | InterruptKind :: RiscvSupervisor => llvm :: CCallConv , InterruptKind :: X86 => llvm :: X86_Intr , } , CanonAbi :: Arm (arm_call) => match arm_call { ArmCall :: Aapcs => llvm :: ArmAapcsCallConv , ArmCall :: CCmseNonSecureCall | ArmCall :: CCmseNonSecureEntry => llvm :: CCallConv , } , CanonAbi :: X86 (x86_call) => match x86_call { X86Call :: Fastcall => llvm :: X86FastcallCallConv , X86Call :: Stdcall => llvm :: X86StdcallCallConv , X86Call :: SysV64 => llvm :: X86_64_SysV , X86Call :: Thiscall => llvm :: X86_ThisCall , X86Call :: Vectorcall => llvm :: X86_VectorCall , X86Call :: Win64 => llvm :: X86_64_Win64 , } , } } }}}