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
mkuse!{#[cfg (feature = "master")] use gccjit :: { FnAttribute , VarAttribute , Visibility } ;}
mkuse!{use gccjit :: { Function , GlobalKind , LValue , RValue , ToRValue , Type } ;}
mkuse!{use rustc_abi :: { self as abi , Align , HasDataLayout , Primitive , Size , WrappingRange } ;}
mkuse!{use rustc_codegen_ssa :: traits :: { BaseTypeCodegenMethods , ConstCodegenMethods , StaticCodegenMethods , } ;}
mkuse!{use rustc_hir :: attrs :: Linkage ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: LOCAL_CRATE ;}
mkuse!{use rustc_middle :: middle :: codegen_fn_attrs :: { CodegenFnAttrFlags , CodegenFnAttrs } ;}
mkuse!{use rustc_middle :: mir :: interpret :: { self , ConstAllocation , ErrorHandled , Scalar as InterpScalar , read_target_uint , } ;}
mkuse!{use rustc_middle :: ty :: layout :: LayoutOf ;}
mkuse!{use rustc_middle :: ty :: { self , Instance } ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_span :: def_id :: DefId ;}
mkuse!{use crate :: base ;}
mkuse!{use crate :: context :: CodegenCx ;}
mkuse!{use crate :: type_of :: LayoutGccExt ;}

macro_rules! set_global_alignment_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_global_alignment in module {}", module_path!());
    };
}

mkfn!{
    set_global_alignment_introspect!();
    fn set_global_alignment < 'gcc , 'tcx > (cx : & CodegenCx < 'gcc , 'tcx > , gv : LValue < 'gcc > , mut align : Align ,) { if let Some (min_global) = cx . sess () . target . min_global_align { align = Ord :: max (align , min_global) ; } gv . set_alignment (align . bytes () as i32) ; }
}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > StaticCodegenMethods for CodegenCx < 'gcc , 'tcx > { fn static_addr_of (& self , cv : RValue < 'gcc > , align : Align , kind : Option < & str >) -> RValue < 'gcc > { if let Some (variable) = self . const_globals . borrow () . get (& cv) { if let Some (global_variable) = self . global_lvalues . borrow () . get (variable) { let alignment = align . bits () as i32 ; if alignment > global_variable . get_alignment () { global_variable . set_alignment (alignment) ; } } return * variable ; } let global_value = self . static_addr_of_mut (cv , align , kind) ; #[cfg (feature = "master")] self . global_lvalues . borrow () . get (& global_value) . expect ("`static_addr_of_mut` did not add the global to `self.global_lvalues`") . global_set_readonly () ; self . const_globals . borrow_mut () . insert (cv , global_value) ; global_value } #[cfg_attr (not (feature = "master") , allow (unused_mut))] fn codegen_static (& mut self , def_id : DefId) { let attrs = self . tcx . codegen_fn_attrs (def_id) ; let Ok ((value , alloc)) = codegen_static_initializer (self , def_id) else { return ; } ; let alloc = alloc . inner () ; let val_llty = self . val_ty (value) ; if val_llty == self . type_i1 () { unimplemented ! () ; } ; let is_thread_local = attrs . flags . contains (CodegenFnAttrFlags :: THREAD_LOCAL) ; let global = self . get_static_inner (def_id , val_llty) ; #[cfg (feature = "master")] if global . to_rvalue () . get_type () != val_llty { global . to_rvalue () . set_type (val_llty) ; } set_global_alignment (self , global , alloc . align) ; global . global_set_initializer_rvalue (value) ; if alloc . mutability . is_not () { #[cfg (feature = "master")] global . global_set_readonly () ; } if is_thread_local { if self . tcx . sess . target . options . is_like_darwin { unimplemented ! () ; } } if self . tcx . sess . target . is_like_wasm { if let Some (_section) = attrs . link_section { unimplemented ! () ; } } else { } if attrs . flags . contains (CodegenFnAttrFlags :: USED_COMPILER) || attrs . flags . contains (CodegenFnAttrFlags :: USED_LINKER) { self . add_used_global (global . to_rvalue ()) ; } } }}}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > CodegenCx < 'gcc , 'tcx > { #[doc = " Add a global value to a list to be stored in the `llvm.used` variable, an array of i8*."] pub fn add_used_global (& mut self , _global : RValue < 'gcc >) { } #[cfg_attr (not (feature = "master") , allow (unused_variables))] pub fn add_used_function (& self , function : Function < 'gcc >) { #[cfg (feature = "master")] function . add_attribute (FnAttribute :: Used) ; } pub fn static_addr_of_mut (& self , cv : RValue < 'gcc > , align : Align , kind : Option < & str > ,) -> RValue < 'gcc > { let global = match kind { Some (kind) if ! self . tcx . sess . fewer_names () => { let name = self . generate_local_symbol_name (kind) ; let typ = self . val_ty (cv) . get_aligned (align . bytes ()) ; self . declare_private_global (& name [..] , typ) } _ => { let typ = self . val_ty (cv) . get_aligned (align . bytes ()) ; self . declare_unnamed_global (typ) } } ; global . global_set_initializer_rvalue (cv) ; let rvalue = global . get_address (None) ; self . global_lvalues . borrow_mut () . insert (rvalue , global) ; rvalue } pub fn get_static (& self , def_id : DefId) -> LValue < 'gcc > { let instance = Instance :: mono (self . tcx , def_id) ; let DefKind :: Static { nested , .. } = self . tcx . def_kind (def_id) else { bug ! () } ; let gcc_type = if nested { self . type_i8 () } else { let ty = instance . ty (self . tcx , ty :: TypingEnv :: fully_monomorphized ()) ; self . layout_of (ty) . gcc_type (self) } ; self . get_static_inner (def_id , gcc_type) } pub (crate) fn get_static_inner (& self , def_id : DefId , gcc_type : Type < 'gcc >) -> LValue < 'gcc > { let instance = Instance :: mono (self . tcx , def_id) ; if let Some (& global) = self . instances . borrow () . get (& instance) { trace ! ("used cached value") ; return global ; } let sym = self . tcx . symbol_name (instance) . name ; let fn_attrs = self . tcx . codegen_fn_attrs (def_id) ; let global = if def_id . is_local () && ! self . tcx . is_foreign_item (def_id) { if let Some (global) = self . get_declared_value (sym) && self . val_ty (global) != self . type_ptr_to (gcc_type) { span_bug ! (self . tcx . def_span (def_id) , "Conflicting types for static") ; } let is_tls = fn_attrs . flags . contains (CodegenFnAttrFlags :: THREAD_LOCAL) ; let global = self . declare_global (sym , gcc_type , GlobalKind :: Imported , is_tls , fn_attrs . link_section ,) ; if ! self . tcx . is_reachable_non_generic (def_id) { #[cfg (feature = "master")] global . add_attribute (VarAttribute :: Visibility (Visibility :: Hidden)) ; } global } else { check_and_apply_linkage (self , fn_attrs , gcc_type , sym) } ; if ! def_id . is_local () { let needs_dll_storage_attr = false ; debug_assert ! (! (self . tcx . sess . opts . cg . linker_plugin_lto . enabled () && self . tcx . sess . target . options . is_like_msvc && self . tcx . sess . opts . cg . prefer_dynamic)) ; if needs_dll_storage_attr { if ! self . tcx . is_codegened_item (def_id) { unimplemented ! () ; } } } self . instances . borrow_mut () . insert (instance , global) ; global } }}}

macro_rules! const_alloc_to_gcc_uncached_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function const_alloc_to_gcc_uncached in module {}", module_path!());
    };
}

mkfn!{
    const_alloc_to_gcc_uncached_introspect!();
    #[doc = " Converts a given const alloc to a gcc Rvalue, without any caching or deduplication."] #[doc = " YOU SHOULD NOT call this function directly - that may break the semantics of Rust."] #[doc = " Use `const_data_from_alloc` instead."] pub (crate) fn const_alloc_to_gcc_uncached < 'gcc > (cx : & CodegenCx < 'gcc , '_ > , alloc : ConstAllocation < '_ > ,) -> RValue < 'gcc > { let alloc = alloc . inner () ; let mut llvals = Vec :: with_capacity (alloc . provenance () . ptrs () . len () + 1) ; let dl = cx . data_layout () ; let pointer_size = dl . pointer_size () . bytes () as usize ; let mut next_offset = 0 ; for & (offset , prov) in alloc . provenance () . ptrs () . iter () { let alloc_id = prov . alloc_id () ; let offset = offset . bytes () ; assert_eq ! (offset as usize as u64 , offset) ; let offset = offset as usize ; if offset > next_offset { let bytes = alloc . inspect_with_uninit_and_ptr_outside_interpreter (next_offset .. offset) ; llvals . push (cx . const_bytes (bytes)) ; } let ptr_offset = read_target_uint (dl . endian , alloc . inspect_with_uninit_and_ptr_outside_interpreter (offset .. (offset + pointer_size)) ,) . expect ("const_alloc_to_gcc_uncached: could not read relocation pointer") as u64 ; let address_space = cx . tcx . global_alloc (alloc_id) . address_space (cx) ; llvals . push (cx . scalar_to_backend (InterpScalar :: from_pointer (interpret :: Pointer :: new (prov , Size :: from_bytes (ptr_offset)) , & cx . tcx ,) , abi :: Scalar :: Initialized { value : Primitive :: Pointer (address_space) , valid_range : WrappingRange :: full (dl . pointer_size ()) , } , cx . type_i8p_ext (address_space) ,)) ; next_offset = offset + pointer_size ; } if alloc . len () >= next_offset { let range = next_offset .. alloc . len () ; let bytes = alloc . inspect_with_uninit_and_ptr_outside_interpreter (range) ; llvals . push (cx . const_bytes (bytes)) ; } cx . const_struct (& llvals , true) }
}

macro_rules! codegen_static_initializer_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function codegen_static_initializer in module {}", module_path!());
    };
}

mkfn!{
    codegen_static_initializer_introspect!();
    fn codegen_static_initializer < 'gcc , 'tcx > (cx : & CodegenCx < 'gcc , 'tcx > , def_id : DefId ,) -> Result < (RValue < 'gcc > , ConstAllocation < 'tcx >) , ErrorHandled > { let alloc = cx . tcx . eval_static_initializer (def_id) ? ; Ok ((cx . const_data_from_alloc (alloc) , alloc)) }
}

macro_rules! check_and_apply_linkage_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_and_apply_linkage in module {}", module_path!());
    };
}

mkfn!{
    check_and_apply_linkage_introspect!();
    fn check_and_apply_linkage < 'gcc , 'tcx > (cx : & CodegenCx < 'gcc , 'tcx > , attrs : & CodegenFnAttrs , gcc_type : Type < 'gcc > , sym : & str ,) -> LValue < 'gcc > { let is_tls = attrs . flags . contains (CodegenFnAttrFlags :: THREAD_LOCAL) ; if let Some (linkage) = attrs . import_linkage { let global1 = cx . declare_global_with_linkage (sym , cx . type_i8 () , base :: global_linkage_to_gcc (linkage)) ; if linkage == Linkage :: ExternalWeak { #[cfg (feature = "master")] global1 . add_attribute (VarAttribute :: Weak) ; } let real_name = format ! ("_rust_extern_with_linkage_{:016x}_{sym}" , cx . tcx . stable_crate_id (LOCAL_CRATE)) ; let global2 = cx . define_global (& real_name , gcc_type , is_tls , attrs . link_section) ; let value = cx . const_ptrcast (global1 . get_address (None) , gcc_type) ; global2 . global_set_initializer_rvalue (value) ; global2 } else { cx . declare_global (sym , gcc_type , GlobalKind :: Imported , is_tls , attrs . link_section) } }
}