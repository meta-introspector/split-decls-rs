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
mkuse!{# [cfg (feature = "master")] use gccjit :: FnAttribute ;}
mkuse!{use gccjit :: { ToLValue , ToRValue , Type } ;}
mkuse!{# [cfg (feature = "master")] use rustc_abi :: { ArmCall , CanonAbi , InterruptKind , X86Call } ;}
mkuse!{use rustc_abi :: { Reg , RegKind } ;}
mkuse!{use rustc_codegen_ssa :: traits :: { AbiBuilderMethods , BaseTypeCodegenMethods } ;}
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: Ty ;}
mkuse!{use rustc_middle :: ty :: layout :: LayoutOf ;}
mkuse!{# [cfg (feature = "master")] use rustc_session :: config ;}
mkuse!{use rustc_target :: callconv :: { ArgAttributes , CastTarget , FnAbi , PassMode } ;}
mkuse!{use crate :: builder :: Builder ;}
mkuse!{use crate :: context :: CodegenCx ;}
mkuse!{use crate :: type_of :: LayoutGccExt ;}
mkitem!{mkimpl!{impl AbiBuilderMethods for Builder < '_ , '_ , '_ > { fn get_param (& mut self , index : usize) -> Self :: Value { let func = self . current_func () ; let param = func . get_param (index as i32) ; let on_stack = if let Some (on_stack_param_indices) = self . on_stack_function_params . borrow () . get (& func) { on_stack_param_indices . contains (& index) } else { false } ; if on_stack { param . to_lvalue () . get_address (None) } else { param . to_rvalue () } } }}}
mkitem!{mkimpl!{impl GccType for CastTarget { fn gcc_type < 'gcc > (& self , cx : & CodegenCx < 'gcc , '_ >) -> Type < 'gcc > { let rest_gcc_unit = self . rest . unit . gcc_type (cx) ; let (rest_count , rem_bytes) = if self . rest . unit . size . bytes () == 0 { (0 , 0) } else { (self . rest . total . bytes () / self . rest . unit . size . bytes () , self . rest . total . bytes () % self . rest . unit . size . bytes () ,) } ; if self . prefix . iter () . all (| x | x . is_none ()) { if self . rest . total <= self . rest . unit . size { return rest_gcc_unit ; } if rem_bytes == 0 { return cx . type_array (rest_gcc_unit , rest_count) ; } } let mut args : Vec < _ > = self . prefix . iter () . flat_map (| option_reg | option_reg . map (| reg | reg . gcc_type (cx))) . chain ((0 .. rest_count) . map (| _ | rest_gcc_unit)) . collect () ; if rem_bytes != 0 { assert_eq ! (self . rest . unit . kind , RegKind :: Integer) ; args . push (cx . type_ix (rem_bytes * 8)) ; } cx . type_struct (& args , false) } }}}
mkitem!{mktrait!{pub trait GccType { fn gcc_type < 'gcc > (& self , cx : & CodegenCx < 'gcc , '_ >) -> Type < 'gcc > ; }}}
mkitem!{mkimpl!{impl GccType for Reg { fn gcc_type < 'gcc > (& self , cx : & CodegenCx < 'gcc , '_ >) -> Type < 'gcc > { match self . kind { RegKind :: Integer => cx . type_ix (self . size . bits ()) , RegKind :: Float => match self . size . bits () { 32 => cx . type_f32 () , 64 => cx . type_f64 () , _ => bug ! ("unsupported float: {:?}" , self) , } , RegKind :: Vector => unimplemented ! () , } } }}}
mkitem!{mkstruct!{pub struct FnAbiGcc < 'gcc > { pub return_type : Type < 'gcc > , pub arguments_type : Vec < Type < 'gcc > > , pub is_c_variadic : bool , pub on_stack_param_indices : FxHashSet < usize > , # [cfg (feature = "master")] pub fn_attributes : Vec < FnAttribute < 'gcc > > , }}}
mkitem!{mktrait!{pub trait FnAbiGccExt < 'gcc , 'tcx > { fn gcc_type (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> FnAbiGcc < 'gcc > ; fn ptr_to_gcc_type (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> Type < 'gcc > ; # [cfg (feature = "master")] fn gcc_cconv (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> Option < FnAttribute < 'gcc > > ; }}}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > FnAbiGccExt < 'gcc , 'tcx > for FnAbi < 'tcx , Ty < 'tcx > > { fn gcc_type (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> FnAbiGcc < 'gcc > { let mut on_stack_param_indices = FxHashSet :: default () ; let mut argument_tys = Vec :: with_capacity (self . args . len () + if let PassMode :: Indirect { .. } = self . ret . mode { 1 } else { 0 } ,) ; let return_type = match self . ret . mode { PassMode :: Ignore => cx . type_void () , PassMode :: Direct (_) | PassMode :: Pair (..) => self . ret . layout . immediate_gcc_type (cx) , PassMode :: Cast { ref cast , .. } => cast . gcc_type (cx) , PassMode :: Indirect { .. } => { argument_tys . push (cx . type_ptr_to (self . ret . layout . gcc_type (cx))) ; cx . type_void () } } ; # [cfg (feature = "master")] let mut non_null_args = Vec :: new () ; # [cfg (feature = "master")] let mut apply_attrs = | mut ty : Type < 'gcc > , attrs : & ArgAttributes , arg_index : usize | { if cx . sess () . opts . optimize == config :: OptLevel :: No { return ty ; } if attrs . regular . contains (rustc_target :: callconv :: ArgAttribute :: NoAlias) { ty = ty . make_restrict () } if attrs . regular . contains (rustc_target :: callconv :: ArgAttribute :: NonNull) { non_null_args . push (arg_index as i32 + 1) ; } ty } ; # [cfg (not (feature = "master"))] let apply_attrs = | ty : Type < 'gcc > , _attrs : & ArgAttributes , _arg_index : usize | ty ; for arg in self . args . iter () { let arg_ty = match arg . mode { PassMode :: Ignore => continue , PassMode :: Pair (a , b) => { let arg_pos = argument_tys . len () ; argument_tys . push (apply_attrs (arg . layout . scalar_pair_element_gcc_type (cx , 0) , & a , arg_pos ,)) ; argument_tys . push (apply_attrs (arg . layout . scalar_pair_element_gcc_type (cx , 1) , & b , arg_pos + 1 ,)) ; continue ; } PassMode :: Cast { ref cast , pad_i32 } => { if pad_i32 { argument_tys . push (Reg :: i32 () . gcc_type (cx)) ; } let ty = cast . gcc_type (cx) ; apply_attrs (ty , & cast . attrs , argument_tys . len ()) } PassMode :: Indirect { attrs : _ , meta_attrs : None , on_stack : true } => { on_stack_param_indices . insert (argument_tys . len ()) ; arg . layout . gcc_type (cx) } PassMode :: Direct (attrs) => { apply_attrs (arg . layout . immediate_gcc_type (cx) , & attrs , argument_tys . len ()) } PassMode :: Indirect { attrs , meta_attrs : None , on_stack : false } => { apply_attrs (cx . type_ptr_to (arg . layout . gcc_type (cx)) , & attrs , argument_tys . len ()) } PassMode :: Indirect { attrs , meta_attrs : Some (meta_attrs) , on_stack } => { assert ! (! on_stack) ; let ptr_ty = Ty :: new_mut_ptr (cx . tcx , arg . layout . ty) ; let ptr_layout = cx . layout_of (ptr_ty) ; let typ1 = ptr_layout . scalar_pair_element_gcc_type (cx , 0) ; let typ2 = ptr_layout . scalar_pair_element_gcc_type (cx , 1) ; argument_tys . push (apply_attrs (typ1 , & attrs , argument_tys . len ())) ; argument_tys . push (apply_attrs (typ2 , & meta_attrs , argument_tys . len ())) ; continue ; } } ; argument_tys . push (arg_ty) ; } # [cfg (feature = "master")] let fn_attrs = if non_null_args . is_empty () { Vec :: new () } else { vec ! [FnAttribute :: NonNull (non_null_args)] } ; FnAbiGcc { return_type , arguments_type : argument_tys , is_c_variadic : self . c_variadic , on_stack_param_indices , # [cfg (feature = "master")] fn_attributes : fn_attrs , } } fn ptr_to_gcc_type (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> Type < 'gcc > { let FnAbiGcc { return_type , arguments_type , is_c_variadic , on_stack_param_indices , .. } = self . gcc_type (cx) ; let pointer_type = cx . context . new_function_pointer_type (None , return_type , & arguments_type , is_c_variadic) ; cx . on_stack_params . borrow_mut () . insert (pointer_type . dyncast_function_ptr_type () . expect ("function ptr type") , on_stack_param_indices ,) ; pointer_type } # [cfg (feature = "master")] fn gcc_cconv (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> Option < FnAttribute < 'gcc > > { conv_to_fn_attribute (self . conv , & cx . tcx . sess . target . arch) } }}}

macro_rules! conv_to_fn_attribute_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function conv_to_fn_attribute in module {}", module_path!());
    };
}

mkfn!{
    conv_to_fn_attribute_introspect!();
    # [cfg (feature = "master")] pub fn conv_to_fn_attribute < 'gcc > (conv : CanonAbi , arch : & str) -> Option < FnAttribute < 'gcc > > { let attribute = match conv { CanonAbi :: C | CanonAbi :: Rust => return None , CanonAbi :: RustCold => FnAttribute :: Cold , CanonAbi :: Custom => return None , CanonAbi :: Arm (arm_call) => match arm_call { ArmCall :: CCmseNonSecureCall => FnAttribute :: ArmCmseNonsecureCall , ArmCall :: CCmseNonSecureEntry => FnAttribute :: ArmCmseNonsecureEntry , ArmCall :: Aapcs => FnAttribute :: ArmPcs ("aapcs") , } , CanonAbi :: GpuKernel => { if arch == "amdgpu" { FnAttribute :: GcnAmdGpuHsaKernel } else if arch == "nvptx64" { FnAttribute :: NvptxKernel } else { panic ! ("Architecture {} does not support GpuKernel calling convention" , arch) ; } } CanonAbi :: Interrupt (interrupt_kind) => match interrupt_kind { InterruptKind :: Avr => FnAttribute :: AvrSignal , InterruptKind :: AvrNonBlocking => FnAttribute :: AvrInterrupt , InterruptKind :: Msp430 => FnAttribute :: Msp430Interrupt , InterruptKind :: RiscvMachine => FnAttribute :: RiscvInterrupt ("machine") , InterruptKind :: RiscvSupervisor => FnAttribute :: RiscvInterrupt ("supervisor") , InterruptKind :: X86 => FnAttribute :: X86Interrupt , } , CanonAbi :: X86 (x86_call) => match x86_call { X86Call :: Fastcall => FnAttribute :: X86FastCall , X86Call :: Stdcall => FnAttribute :: X86Stdcall , X86Call :: Thiscall => FnAttribute :: X86ThisCall , X86Call :: Vectorcall => return None , X86Call :: SysV64 => FnAttribute :: X86SysvAbi , X86Call :: Win64 => FnAttribute :: X86MsAbi , } , } ; Some (attribute) }
}