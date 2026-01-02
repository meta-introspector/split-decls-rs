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
mkuse!{use std :: fmt :: { self , Write } ;}
mkuse!{use std :: hash :: { Hash , Hasher } ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use std :: sync :: Arc ;}
mkuse!{use std :: { iter , ptr } ;}
mkuse!{use libc :: { c_longlong , c_uint } ;}
mkuse!{use rustc_abi :: { Align , Size } ;}
mkuse!{use rustc_codegen_ssa :: debuginfo :: type_names :: { VTableNameKind , cpp_like_debuginfo } ;}
mkuse!{use rustc_codegen_ssa :: traits :: * ;}
mkuse!{use rustc_hir :: def :: { CtorKind , DefKind } ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LOCAL_CRATE } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: layout :: { HasTypingEnv , LayoutOf , TyAndLayout , WIDE_PTR_ADDR , WIDE_PTR_EXTRA , } ;}
mkuse!{use rustc_middle :: ty :: { self , AdtKind , CoroutineArgsExt , ExistentialTraitRef , Instance , Ty , TyCtxt , Visibility , } ;}
mkuse!{use rustc_session :: config :: { self , DebugInfo , Lto } ;}
mkuse!{use rustc_span :: { DUMMY_SP , FileName , FileNameDisplayPreference , SourceFile , Span , Symbol , hygiene , } ;}
mkuse!{use rustc_symbol_mangling :: typeid_for_trait_ref ;}
mkuse!{use rustc_target :: spec :: DebuginfoKind ;}
mkuse!{use smallvec :: smallvec ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{pub (crate) use self :: type_map :: TypeMap ;}
mkuse!{use self :: type_map :: { DINodeCreationResult , Stub , UniqueTypeId } ;}
mkuse!{use super :: CodegenUnitDebugContext ;}
mkuse!{use super :: namespace :: mangled_name_of_instance ;}
mkuse!{use super :: type_names :: { compute_debuginfo_type_name , compute_debuginfo_vtable_name } ;}
mkuse!{use super :: utils :: { DIB , create_DIArray , debug_context , get_namespace_for_item , is_node_local_to_unit , } ;}
mkuse!{use crate :: common :: { AsCCharPtr , CodegenCx } ;}
mkuse!{use crate :: debuginfo :: dwarf_const ;}
mkuse!{use crate :: debuginfo :: metadata :: type_map :: build_type_with_children ;}
mkuse!{use crate :: debuginfo :: utils :: { WidePtrKind , wide_pointer_kind } ;}
mkuse!{use crate :: llvm ;}
mkuse!{use crate :: llvm :: debuginfo :: { DIBasicType , DIBuilder , DICompositeType , DIDescriptor , DIFile , DIFlags , DILexicalBlock , DIScope , DIType , DebugEmissionKind , DebugNameTableKind , } ;}
mkuse!{use crate :: value :: Value ;}
mkitem!{mkimpl!{impl PartialEq for llvm :: Metadata { fn eq (& self , other : & Self) -> bool { ptr :: eq (self , other) } }}}
mkitem!{mkimpl!{impl Eq for llvm :: Metadata { }}}
mkitem!{mkimpl!{impl Hash for llvm :: Metadata { fn hash < H : Hasher > (& self , hasher : & mut H) { (self as * const Self) . hash (hasher) ; } }}}
mkitem!{mkimpl!{impl fmt :: Debug for llvm :: Metadata { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (self as * const Self) . fmt (f) } }}}
mkitem!{pub (super) const UNKNOWN_LINE_NUMBER : c_uint = 0 ;}
mkitem!{pub (super) const UNKNOWN_COLUMN_NUMBER : c_uint = 0 ;}
mkitem!{const NO_SCOPE_METADATA : Option < & DIScope > = None ;}
mkitem!{#[doc = " A function that returns an empty list of generic parameter debuginfo nodes."] const NO_GENERICS : for < 'll > fn (& CodegenCx < 'll , '_ >) -> SmallVec < Option < & 'll DIType > > = | _ | SmallVec :: new () ;}
mkitem!{type SmallVec < T > = smallvec :: SmallVec < [T ; 16] > ;}
mkmod!{enums, { 
                getname!(enums);
                getsrc!(enums);
                getpath!(enums);
                get_deps!(enums);
                get_crates!(enums);
                mkinclude!(enums);
                 
            }}
mkmod!{type_map, { 
                getname!(type_map);
                getsrc!(type_map);
                getpath!(type_map);
                get_deps!(type_map);
                get_crates!(type_map);
                mkinclude!(type_map);
                 
            }}
mkitem!{#[doc = " Returns from the enclosing function if the type debuginfo node with the given"] #[doc = " unique ID can be found in the type map."] macro_rules ! return_if_di_node_created_in_meantime { ($ cx : expr , $ unique_type_id : expr) => { if let Some (di_node) = debug_context ($ cx) . type_map . di_node_for_unique_id ($ unique_type_id) { return DINodeCreationResult :: new (di_node , true) ; } } ; }}

macro_rules! size_and_align_of_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function size_and_align_of in module {}", module_path!());
    };
}

mkfn!{
    size_and_align_of_introspect!();
    #[doc = " Extract size and alignment from a TyAndLayout."] #[inline] fn size_and_align_of (ty_and_layout : TyAndLayout < '_ >) -> (Size , Align) { (ty_and_layout . size , ty_and_layout . align . abi) }
}

macro_rules! build_fixed_size_array_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_fixed_size_array_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_fixed_size_array_di_node_introspect!();
    #[doc = " Creates debuginfo for a fixed size array (e.g. `[u64; 123]`)."] #[doc = " For slices (that is, \"arrays\" of unknown size) use [build_slice_type_di_node]."] fn build_fixed_size_array_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , unique_type_id : UniqueTypeId < 'tcx > , array_type : Ty < 'tcx > , span : Span ,) -> DINodeCreationResult < 'll > { let ty :: Array (element_type , len) = array_type . kind () else { bug ! ("build_fixed_size_array_di_node() called with non-ty::Array type `{:?}`" , array_type) } ; let element_type_di_node = spanned_type_di_node (cx , * element_type , span) ; return_if_di_node_created_in_meantime ! (cx , unique_type_id) ; let (size , align) = cx . spanned_size_and_align_of (array_type , span) ; let upper_bound = len . try_to_target_usize (cx . tcx) . expect ("expected monomorphic const in codegen") as c_longlong ; let subrange = unsafe { Some (llvm :: LLVMRustDIBuilderGetOrCreateSubrange (DIB (cx) , 0 , upper_bound)) } ; let subscripts = create_DIArray (DIB (cx) , & [subrange]) ; let di_node = unsafe { llvm :: LLVMRustDIBuilderCreateArrayType (DIB (cx) , size . bits () , align . bits () as u32 , element_type_di_node , subscripts ,) } ; DINodeCreationResult :: new (di_node , false) }
}

macro_rules! build_pointer_or_reference_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_pointer_or_reference_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_pointer_or_reference_di_node_introspect!();
    #[doc = " Creates debuginfo for built-in pointer-like things:"] #[doc = ""] #[doc = "  - ty::Ref"] #[doc = "  - ty::RawPtr"] #[doc = "  - ty::Adt in the case it's Box"] #[doc = ""] #[doc = " At some point we might want to remove the special handling of Box"] #[doc = " and treat it the same as other smart pointers (like Rc, Arc, ...)."] fn build_pointer_or_reference_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , ptr_type : Ty < 'tcx > , pointee_type : Ty < 'tcx > , unique_type_id : UniqueTypeId < 'tcx > ,) -> DINodeCreationResult < 'll > { assert_eq ! (cx . size_and_align_of (ptr_type) , cx . size_and_align_of (Ty :: new_mut_ptr (cx . tcx , pointee_type))) ; let pointee_type_di_node = type_di_node (cx , pointee_type) ; return_if_di_node_created_in_meantime ! (cx , unique_type_id) ; let data_layout = & cx . tcx . data_layout ; let pointer_size = data_layout . pointer_size () ; let pointer_align = data_layout . pointer_align () ; let ptr_type_debuginfo_name = compute_debuginfo_type_name (cx . tcx , ptr_type , true) ; match wide_pointer_kind (cx , pointee_type) { None => { assert_eq ! ((pointer_size , pointer_align . abi) , cx . size_and_align_of (ptr_type) , "ptr_type={ptr_type}, pointee_type={pointee_type}" ,) ; let di_node = unsafe { llvm :: LLVMRustDIBuilderCreatePointerType (DIB (cx) , pointee_type_di_node , pointer_size . bits () , pointer_align . abi . bits () as u32 , 0 , ptr_type_debuginfo_name . as_c_char_ptr () , ptr_type_debuginfo_name . len () ,) } ; DINodeCreationResult { di_node , already_stored_in_typemap : false } } Some (wide_pointer_kind) => { type_map :: build_type_with_children (cx , type_map :: stub (cx , Stub :: Struct , unique_type_id , & ptr_type_debuginfo_name , None , cx . size_and_align_of (ptr_type) , NO_SCOPE_METADATA , DIFlags :: FlagZero ,) , | cx , owner | { let layout_type = if ptr_type . is_box () { Ty :: new_mut_ptr (cx . tcx , pointee_type) } else { ptr_type } ; let layout = cx . layout_of (layout_type) ; let addr_field = layout . field (cx , WIDE_PTR_ADDR) ; let extra_field = layout . field (cx , WIDE_PTR_EXTRA) ; let (addr_field_name , extra_field_name) = match wide_pointer_kind { WidePtrKind :: Dyn => ("pointer" , "vtable") , WidePtrKind :: Slice => ("data_ptr" , "length") , } ; assert_eq ! (WIDE_PTR_ADDR , 0) ; assert_eq ! (WIDE_PTR_EXTRA , 1) ; let data_ptr_type_di_node = unsafe { llvm :: LLVMRustDIBuilderCreatePointerType (DIB (cx) , pointee_type_di_node , addr_field . size . bits () , addr_field . align . abi . bits () as u32 , 0 , std :: ptr :: null () , 0 ,) } ; smallvec ! [build_field_di_node (cx , owner , addr_field_name , addr_field , layout . fields . offset (WIDE_PTR_ADDR) , DIFlags :: FlagZero , data_ptr_type_di_node , None ,) , build_field_di_node (cx , owner , extra_field_name , extra_field , layout . fields . offset (WIDE_PTR_EXTRA) , DIFlags :: FlagZero , type_di_node (cx , extra_field . ty) , None ,) ,] } , NO_GENERICS ,) } } }
}

macro_rules! build_subroutine_type_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_subroutine_type_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_subroutine_type_di_node_introspect!();
    fn build_subroutine_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , unique_type_id : UniqueTypeId < 'tcx > ,) -> DINodeCreationResult < 'll > { debug_context (cx) . type_map . unique_id_to_di_node . borrow_mut () . insert (unique_type_id , recursion_marker_type_di_node (cx)) ; let fn_ty = unique_type_id . expect_ty () ; let signature = cx . tcx . normalize_erasing_late_bound_regions (cx . typing_env () , fn_ty . fn_sig (cx . tcx)) ; let signature_di_nodes : SmallVec < _ > = iter :: once (match signature . output () . kind () { ty :: Tuple (tys) if tys . is_empty () => { None } _ => Some (type_di_node (cx , signature . output ())) , } ,) . chain (signature . inputs () . iter () . map (| & argument_type | Some (type_di_node (cx , argument_type))) ,) . collect () ; debug_context (cx) . type_map . unique_id_to_di_node . borrow_mut () . remove (& unique_type_id) ; let fn_di_node = create_subroutine_type (cx , create_DIArray (DIB (cx) , & signature_di_nodes [..])) ; let name = compute_debuginfo_type_name (cx . tcx , fn_ty , false) ; let (size , align) = match fn_ty . kind () { ty :: FnDef (..) => (Size :: ZERO , Align :: ONE) , ty :: FnPtr (..) => { (cx . tcx . data_layout . pointer_size () , cx . tcx . data_layout . pointer_align () . abi) } _ => unreachable ! () , } ; let di_node = unsafe { llvm :: LLVMRustDIBuilderCreatePointerType (DIB (cx) , fn_di_node , size . bits () , align . bits () as u32 , 0 , name . as_c_char_ptr () , name . len () ,) } ; DINodeCreationResult :: new (di_node , false) }
}

macro_rules! create_subroutine_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_subroutine_type in module {}", module_path!());
    };
}

mkfn!{
    create_subroutine_type_introspect!();
    pub (super) fn create_subroutine_type < 'll > (cx : & CodegenCx < 'll , '_ > , signature : & 'll DICompositeType ,) -> & 'll DICompositeType { unsafe { llvm :: LLVMRustDIBuilderCreateSubroutineType (DIB (cx) , signature) } }
}

macro_rules! build_dyn_type_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_dyn_type_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_dyn_type_di_node_introspect!();
    #[doc = " Create debuginfo for `dyn SomeTrait` types. Currently these are empty structs"] #[doc = " we with the correct type name (e.g. \"dyn SomeTrait<Foo, Item=u32> + Sync\")."] fn build_dyn_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , dyn_type : Ty < 'tcx > , unique_type_id : UniqueTypeId < 'tcx > ,) -> DINodeCreationResult < 'll > { if let ty :: Dynamic (..) = dyn_type . kind () { let type_name = compute_debuginfo_type_name (cx . tcx , dyn_type , true) ; type_map :: build_type_with_children (cx , type_map :: stub (cx , Stub :: Struct , unique_type_id , & type_name , None , cx . size_and_align_of (dyn_type) , NO_SCOPE_METADATA , DIFlags :: FlagZero ,) , | _ , _ | smallvec ! [] , NO_GENERICS ,) } else { bug ! ("Only ty::Dynamic is valid for build_dyn_type_di_node(). Found {:?} instead." , dyn_type) } }
}

macro_rules! build_slice_type_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_slice_type_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_slice_type_di_node_introspect!();
    #[doc = " Create debuginfo for `[T]` and `str`. These are unsized."] #[doc = ""] #[doc = " NOTE: We currently emit just emit the debuginfo for the element type here"] #[doc = " (i.e. `T` for slices and `u8` for `str`), so that we end up with"] #[doc = " `*const T` for the `data_ptr` field of the corresponding wide-pointer"] #[doc = " debuginfo of `&[T]`."] #[doc = ""] #[doc = " It would be preferable and more accurate if we emitted a DIArray of T"] #[doc = " without an upper bound instead. That is, LLVM already supports emitting"] #[doc = " debuginfo of arrays of unknown size. But GDB currently seems to end up"] #[doc = " in an infinite loop when confronted with such a type."] #[doc = ""] #[doc = " As a side effect of the current encoding every instance of a type like"] #[doc = " `struct Foo { unsized_field: [u8] }` will look like"] #[doc = " `struct Foo { unsized_field: u8 }` in debuginfo. If the length of the"] #[doc = " slice is zero, then accessing `unsized_field` in the debugger would"] #[doc = " result in an out-of-bounds access."] fn build_slice_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , slice_type : Ty < 'tcx > , unique_type_id : UniqueTypeId < 'tcx > ,) -> DINodeCreationResult < 'll > { let element_type = match slice_type . kind () { ty :: Slice (element_type) => * element_type , ty :: Str => cx . tcx . types . u8 , _ => { bug ! ("Only ty::Slice is valid for build_slice_type_di_node(). Found {:?} instead." , slice_type) } } ; let element_type_di_node = type_di_node (cx , element_type) ; return_if_di_node_created_in_meantime ! (cx , unique_type_id) ; DINodeCreationResult { di_node : element_type_di_node , already_stored_in_typemap : false } }
}

macro_rules! type_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_di_node in module {}", module_path!());
    };
}

mkfn!{
    type_di_node_introspect!();
    #[doc = " Get the debuginfo node for the given type."] #[doc = ""] #[doc = " This function will look up the debuginfo node in the TypeMap. If it can't find it, it"] #[doc = " will create the node by dispatching to the corresponding `build_*_di_node()` function."] pub (crate) fn type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , t : Ty < 'tcx >) -> & 'll DIType { spanned_type_di_node (cx , t , DUMMY_SP) }
}

macro_rules! spanned_type_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spanned_type_di_node in module {}", module_path!());
    };
}

mkfn!{
    spanned_type_di_node_introspect!();
    pub (crate) fn spanned_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , t : Ty < 'tcx > , span : Span ,) -> & 'll DIType { let unique_type_id = UniqueTypeId :: for_ty (cx . tcx , t) ; if let Some (existing_di_node) = debug_context (cx) . type_map . di_node_for_unique_id (unique_type_id) { return existing_di_node ; } debug ! ("type_di_node: {:?} kind: {:?}" , t , t . kind ()) ; let DINodeCreationResult { di_node , already_stored_in_typemap } = match * t . kind () { ty :: Never | ty :: Bool | ty :: Char | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) => { build_basic_type_di_node (cx , t) } ty :: Tuple (elements) if elements . is_empty () => build_basic_type_di_node (cx , t) , ty :: Array (..) => build_fixed_size_array_di_node (cx , unique_type_id , t , span) , ty :: Slice (_) | ty :: Str => build_slice_type_di_node (cx , t , unique_type_id) , ty :: Dynamic (..) => build_dyn_type_di_node (cx , t , unique_type_id) , ty :: Foreign (..) => build_foreign_type_di_node (cx , t , unique_type_id) , ty :: RawPtr (pointee_type , _) | ty :: Ref (_ , pointee_type , _) => { build_pointer_or_reference_di_node (cx , t , pointee_type , unique_type_id) } ty :: Adt (def , args) if def . is_box () && args . get (1) . is_none_or (| arg | cx . layout_of (arg . expect_ty ()) . is_1zst ()) => { build_pointer_or_reference_di_node (cx , t , t . expect_boxed_ty () , unique_type_id) } ty :: FnDef (..) | ty :: FnPtr (..) => build_subroutine_type_di_node (cx , unique_type_id) , ty :: Closure (..) => build_closure_env_di_node (cx , unique_type_id) , ty :: CoroutineClosure (..) => build_closure_env_di_node (cx , unique_type_id) , ty :: Coroutine (..) => enums :: build_coroutine_di_node (cx , unique_type_id) , ty :: Adt (def , ..) => match def . adt_kind () { AdtKind :: Struct => build_struct_type_di_node (cx , unique_type_id) , AdtKind :: Union => build_union_type_di_node (cx , unique_type_id) , AdtKind :: Enum => enums :: build_enum_type_di_node (cx , unique_type_id , span) , } , ty :: Tuple (_) => build_tuple_type_di_node (cx , unique_type_id) , _ => bug ! ("debuginfo: unexpected type in type_di_node(): {:?}" , t) , } ; { if already_stored_in_typemap { let di_node_for_uid = match debug_context (cx) . type_map . di_node_for_unique_id (unique_type_id) { Some (di_node) => di_node , None => { bug ! ("expected type debuginfo node for unique \
                               type ID '{:?}' to already be in \
                               the `debuginfo::TypeMap` but it \
                               was not." , unique_type_id ,) ; } } ; assert_eq ! (di_node_for_uid as * const _ , di_node as * const _) ; } else { debug_context (cx) . type_map . insert (unique_type_id , di_node) ; } } di_node }
}

macro_rules! recursion_marker_type_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function recursion_marker_type_di_node in module {}", module_path!());
    };
}

mkfn!{
    recursion_marker_type_di_node_introspect!();
    fn recursion_marker_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx >) -> & 'll DIType { * debug_context (cx) . recursion_marker_type . get_or_init (move | | { create_basic_type (cx , "<recur_type>" , cx . tcx . data_layout . pointer_size () , dwarf_const :: DW_ATE_unsigned ,) }) }
}

macro_rules! hex_encode_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hex_encode in module {}", module_path!());
    };
}

mkfn!{
    hex_encode_introspect!();
    fn hex_encode (data : & [u8]) -> String { let mut hex_string = String :: with_capacity (data . len () * 2) ; for byte in data . iter () { write ! (& mut hex_string , "{byte:02x}") . unwrap () ; } hex_string }
}

macro_rules! file_metadata_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function file_metadata in module {}", module_path!());
    };
}

mkfn!{
    file_metadata_introspect!();
    pub (crate) fn file_metadata < 'll > (cx : & CodegenCx < 'll , '_ > , source_file : & SourceFile) -> & 'll DIFile { let cache_key = Some ((source_file . stable_id , source_file . src_hash)) ; return debug_context (cx) . created_files . borrow_mut () . entry (cache_key) . or_insert_with (| | alloc_new_file_metadata (cx , source_file)) ; #[instrument (skip (cx , source_file) , level = "debug")] fn alloc_new_file_metadata < 'll > (cx : & CodegenCx < 'll , '_ > , source_file : & SourceFile ,) -> & 'll DIFile { debug ! (? source_file . name) ; let filename_display_preference = cx . sess () . filename_display_preference (RemapPathScopeComponents :: DEBUGINFO) ; use rustc_session :: config :: RemapPathScopeComponents ; let (directory , file_name) = match & source_file . name { FileName :: Real (filename) => { let working_directory = & cx . sess () . opts . working_dir ; debug ! (? working_directory) ; if filename_display_preference == FileNameDisplayPreference :: Remapped { let filename = cx . sess () . source_map () . path_mapping () . to_embeddable_absolute_path (filename . clone () , working_directory) ; let abs_path = filename . remapped_path_if_available () ; debug ! (? abs_path) ; if let Ok (rel_path) = abs_path . strip_prefix (working_directory . remapped_path_if_available ()) { (working_directory . to_string_lossy (FileNameDisplayPreference :: Remapped) , rel_path . to_string_lossy () . into_owned () ,) } else { ("" . into () , abs_path . to_string_lossy () . into_owned ()) } } else { let working_directory = working_directory . local_path_if_available () ; let filename = filename . local_path_if_available () ; debug ! (? working_directory , ? filename) ; let abs_path : Cow < '_ , Path > = if filename . is_absolute () { filename . into () } else { let mut p = PathBuf :: new () ; p . push (working_directory) ; p . push (filename) ; p . into () } ; if let Ok (rel_path) = abs_path . strip_prefix (working_directory) { (working_directory . to_string_lossy () , rel_path . to_string_lossy () . into_owned () ,) } else { ("" . into () , abs_path . to_string_lossy () . into_owned ()) } } } other => { debug ! (? other) ; ("" . into () , other . display (filename_display_preference) . to_string ()) } } ; let hash_kind = match source_file . src_hash . kind { rustc_span :: SourceFileHashAlgorithm :: Md5 => llvm :: ChecksumKind :: MD5 , rustc_span :: SourceFileHashAlgorithm :: Sha1 => llvm :: ChecksumKind :: SHA1 , rustc_span :: SourceFileHashAlgorithm :: Sha256 => llvm :: ChecksumKind :: SHA256 , rustc_span :: SourceFileHashAlgorithm :: Blake3 => llvm :: ChecksumKind :: None , } ; let hash_value = hex_encode (source_file . src_hash . hash_bytes ()) ; let source = cx . sess () . opts . unstable_opts . embed_source . then_some (()) . and (source_file . src . as_ref ()) ; create_file (DIB (cx) , & file_name , & directory , & hash_value , hash_kind , source) } }
}

macro_rules! unknown_file_metadata_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unknown_file_metadata in module {}", module_path!());
    };
}

mkfn!{
    unknown_file_metadata_introspect!();
    fn unknown_file_metadata < 'll > (cx : & CodegenCx < 'll , '_ >) -> & 'll DIFile { debug_context (cx) . created_files . borrow_mut () . entry (None) . or_insert_with (| | { create_file (DIB (cx) , "<unknown>" , "" , "" , llvm :: ChecksumKind :: None , None) }) }
}

macro_rules! create_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_file in module {}", module_path!());
    };
}

mkfn!{
    create_file_introspect!();
    fn create_file < 'll > (builder : & DIBuilder < 'll > , file_name : & str , directory : & str , hash_value : & str , hash_kind : llvm :: ChecksumKind , source : Option < & Arc < String > > ,) -> & 'll DIFile { unsafe { llvm :: LLVMRustDIBuilderCreateFile (builder , file_name . as_c_char_ptr () , file_name . len () , directory . as_c_char_ptr () , directory . len () , hash_kind , hash_value . as_c_char_ptr () , hash_value . len () , source . map_or (ptr :: null () , | x | x . as_c_char_ptr ()) , source . map_or (0 , | x | x . len ()) ,) } }
}
mkitem!{mktrait!{trait MsvcBasicName { fn msvc_basic_name (self) -> & 'static str ; }}}
mkitem!{mkimpl!{impl MsvcBasicName for ty :: IntTy { fn msvc_basic_name (self) -> & 'static str { match self { ty :: IntTy :: Isize => "ptrdiff_t" , ty :: IntTy :: I8 => "__int8" , ty :: IntTy :: I16 => "__int16" , ty :: IntTy :: I32 => "__int32" , ty :: IntTy :: I64 => "__int64" , ty :: IntTy :: I128 => "__int128" , } } }}}
mkitem!{mkimpl!{impl MsvcBasicName for ty :: UintTy { fn msvc_basic_name (self) -> & 'static str { match self { ty :: UintTy :: Usize => "size_t" , ty :: UintTy :: U8 => "unsigned __int8" , ty :: UintTy :: U16 => "unsigned __int16" , ty :: UintTy :: U32 => "unsigned __int32" , ty :: UintTy :: U64 => "unsigned __int64" , ty :: UintTy :: U128 => "unsigned __int128" , } } }}}
mkitem!{mkimpl!{impl MsvcBasicName for ty :: FloatTy { fn msvc_basic_name (self) -> & 'static str { match self { ty :: FloatTy :: F16 => { bug ! ("`f16` should have been handled in `build_basic_type_di_node`") } ty :: FloatTy :: F32 => "float" , ty :: FloatTy :: F64 => "double" , ty :: FloatTy :: F128 => "fp128" , } } }}}

macro_rules! build_cpp_f16_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_cpp_f16_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_cpp_f16_di_node_introspect!();
    fn build_cpp_f16_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx >) -> DINodeCreationResult < 'll > { let float_ty = cx . tcx . types . f16 ; let bits_ty = cx . tcx . types . u16 ; let def_location = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { match float_ty . kind () { ty :: Adt (def , _) => Some (file_metadata_from_def_id (cx , Some (def . did ()))) , _ => None , } } else { None } ; type_map :: build_type_with_children (cx , type_map :: stub (cx , Stub :: Struct , UniqueTypeId :: for_ty (cx . tcx , float_ty) , "f16" , def_location , cx . size_and_align_of (float_ty) , NO_SCOPE_METADATA , DIFlags :: FlagZero ,) , | cx , float_di_node | { let def_id = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { match bits_ty . kind () { ty :: Adt (def , _) => Some (def . did ()) , _ => None , } } else { None } ; smallvec ! [build_field_di_node (cx , float_di_node , "bits" , cx . layout_of (bits_ty) , Size :: ZERO , DIFlags :: FlagZero , type_di_node (cx , bits_ty) , def_id ,)] } , NO_GENERICS ,) }
}

macro_rules! build_basic_type_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_basic_type_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_basic_type_di_node_introspect!();
    fn build_basic_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , t : Ty < 'tcx > ,) -> DINodeCreationResult < 'll > { debug ! ("build_basic_type_di_node: {:?}" , t) ; let cpp_like_debuginfo = cpp_like_debuginfo (cx . tcx) ; use dwarf_const :: { DW_ATE_UTF , DW_ATE_boolean , DW_ATE_float , DW_ATE_signed , DW_ATE_unsigned } ; let (name , encoding) = match t . kind () { ty :: Never => ("!" , DW_ATE_unsigned) , ty :: Tuple (elements) if elements . is_empty () => { if cpp_like_debuginfo { return build_tuple_type_di_node (cx , UniqueTypeId :: for_ty (cx . tcx , t)) ; } else { ("()" , DW_ATE_unsigned) } } ty :: Bool => ("bool" , DW_ATE_boolean) , ty :: Char => ("char" , DW_ATE_UTF) , ty :: Int (int_ty) if cpp_like_debuginfo => (int_ty . msvc_basic_name () , DW_ATE_signed) , ty :: Uint (uint_ty) if cpp_like_debuginfo => (uint_ty . msvc_basic_name () , DW_ATE_unsigned) , ty :: Float (ty :: FloatTy :: F16) if cpp_like_debuginfo => { return build_cpp_f16_di_node (cx) ; } ty :: Float (float_ty) if cpp_like_debuginfo => (float_ty . msvc_basic_name () , DW_ATE_float) , ty :: Int (int_ty) => (int_ty . name_str () , DW_ATE_signed) , ty :: Uint (uint_ty) => (uint_ty . name_str () , DW_ATE_unsigned) , ty :: Float (float_ty) => (float_ty . name_str () , DW_ATE_float) , _ => bug ! ("debuginfo::build_basic_type_di_node - `t` is invalid type") , } ; let ty_di_node = create_basic_type (cx , name , cx . size_of (t) , encoding) ; if ! cpp_like_debuginfo { return DINodeCreationResult :: new (ty_di_node , false) ; } let typedef_name = match t . kind () { ty :: Int (int_ty) => int_ty . name_str () , ty :: Uint (uint_ty) => uint_ty . name_str () , ty :: Float (float_ty) => float_ty . name_str () , _ => return DINodeCreationResult :: new (ty_di_node , false) , } ; let typedef_di_node = unsafe { llvm :: LLVMRustDIBuilderCreateTypedef (DIB (cx) , ty_di_node , typedef_name . as_c_char_ptr () , typedef_name . len () , unknown_file_metadata (cx) , 0 , None ,) } ; DINodeCreationResult :: new (typedef_di_node , false) }
}

macro_rules! create_basic_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_basic_type in module {}", module_path!());
    };
}

mkfn!{
    create_basic_type_introspect!();
    fn create_basic_type < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , name : & str , size : Size , encoding : u32 ,) -> & 'll DIBasicType { unsafe { llvm :: LLVMRustDIBuilderCreateBasicType (DIB (cx) , name . as_c_char_ptr () , name . len () , size . bits () , encoding ,) } }
}

macro_rules! build_foreign_type_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_foreign_type_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_foreign_type_di_node_introspect!();
    fn build_foreign_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , t : Ty < 'tcx > , unique_type_id : UniqueTypeId < 'tcx > ,) -> DINodeCreationResult < 'll > { debug ! ("build_foreign_type_di_node: {:?}" , t) ; let & ty :: Foreign (def_id) = unique_type_id . expect_ty () . kind () else { bug ! ("build_foreign_type_di_node() called with unexpected type: {:?}" , unique_type_id . expect_ty ()) ; } ; build_type_with_children (cx , type_map :: stub (cx , Stub :: Struct , unique_type_id , & compute_debuginfo_type_name (cx . tcx , t , false) , None , cx . size_and_align_of (t) , Some (get_namespace_for_item (cx , def_id)) , DIFlags :: FlagZero ,) , | _ , _ | smallvec ! [] , NO_GENERICS ,) }
}

macro_rules! build_compile_unit_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_compile_unit_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_compile_unit_di_node_introspect!();
    pub (crate) fn build_compile_unit_di_node < 'll , 'tcx > (tcx : TyCtxt < 'tcx > , codegen_unit_name : & str , debug_context : & CodegenUnitDebugContext < 'll , 'tcx > ,) -> & 'll DIDescriptor { use rustc_session :: RemapFileNameExt ; use rustc_session :: config :: RemapPathScopeComponents ; let mut name_in_debuginfo = tcx . sess . local_crate_source_file () . map (| src | src . for_scope (& tcx . sess , RemapPathScopeComponents :: DEBUGINFO) . to_path_buf ()) . unwrap_or_else (| | PathBuf :: from (tcx . crate_name (LOCAL_CRATE) . as_str ())) ; name_in_debuginfo . push ("@") ; name_in_debuginfo . push (codegen_unit_name) ; debug ! ("build_compile_unit_di_node: {:?}" , name_in_debuginfo) ; let rustc_producer = format ! ("rustc version {}" , tcx . sess . cfg_version) ; let producer = format ! ("clang LLVM ({rustc_producer})") ; let name_in_debuginfo = name_in_debuginfo . to_string_lossy () ; let work_dir = tcx . sess . opts . working_dir . for_scope (tcx . sess , RemapPathScopeComponents :: DEBUGINFO) . to_string_lossy () ; let output_filenames = tcx . output_filenames (()) ; let split_name = if tcx . sess . target_can_use_split_dwarf () && let Some (f) = output_filenames . split_dwarf_path (tcx . sess . split_debuginfo () , tcx . sess . opts . unstable_opts . split_dwarf_kind , codegen_unit_name , tcx . sess . invocation_temp . as_deref () ,) { Some (tcx . sess . source_map () . path_mapping () . to_real_filename (f)) } else { None } ; let split_name = split_name . as_ref () . map (| f | f . for_scope (tcx . sess , RemapPathScopeComponents :: DEBUGINFO) . to_string_lossy ()) . unwrap_or_default () ; let kind = DebugEmissionKind :: from_generic (tcx . sess . opts . debuginfo) ; let dwarf_version = tcx . sess . dwarf_version () ; let is_dwarf_kind = matches ! (tcx . sess . target . debuginfo_kind , DebuginfoKind :: Dwarf | DebuginfoKind :: DwarfDsym) ; let debug_name_table_kind = if is_dwarf_kind && dwarf_version <= 4 { DebugNameTableKind :: None } else { DebugNameTableKind :: Default } ; unsafe { let compile_unit_file = create_file (debug_context . builder . as_ref () , & name_in_debuginfo , & work_dir , "" , llvm :: ChecksumKind :: None , None ,) ; let unit_metadata = llvm :: LLVMRustDIBuilderCreateCompileUnit (debug_context . builder . as_ref () , dwarf_const :: DW_LANG_Rust , compile_unit_file , producer . as_c_char_ptr () , producer . len () , tcx . sess . opts . optimize != config :: OptLevel :: No , c"" . as_ptr () , 0 , split_name . as_c_char_ptr () , split_name . len () , kind , 0 , tcx . sess . opts . unstable_opts . split_dwarf_inlining , debug_name_table_kind ,) ; return unit_metadata ; } ; }
}

macro_rules! build_field_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_field_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_field_di_node_introspect!();
    #[doc = " Creates a `DW_TAG_member` entry inside the DIE represented by the given `type_di_node`."] fn build_field_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , owner : & 'll DIScope , name : & str , layout : TyAndLayout < 'tcx > , offset : Size , flags : DIFlags , type_di_node : & 'll DIType , def_id : Option < DefId > ,) -> & 'll DIType { let (file_metadata , line_number) = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { file_metadata_from_def_id (cx , def_id) } else { (unknown_file_metadata (cx) , UNKNOWN_LINE_NUMBER) } ; create_member_type (cx , owner , name , file_metadata , line_number , layout , offset , flags , type_di_node ,) }
}

macro_rules! create_member_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_member_type in module {}", module_path!());
    };
}

mkfn!{
    create_member_type_introspect!();
    fn create_member_type < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , owner : & 'll DIScope , name : & str , file_metadata : & 'll DIType , line_number : u32 , layout : TyAndLayout < 'tcx > , offset : Size , flags : DIFlags , type_di_node : & 'll DIType ,) -> & 'll DIType { unsafe { llvm :: LLVMRustDIBuilderCreateMemberType (DIB (cx) , owner , name . as_c_char_ptr () , name . len () , file_metadata , line_number , layout . size . bits () , layout . align . abi . bits () as u32 , offset . bits () , flags , type_di_node ,) } }
}

macro_rules! visibility_di_flags_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function visibility_di_flags in module {}", module_path!());
    };
}

mkfn!{
    visibility_di_flags_introspect!();
    #[doc = " Returns the `DIFlags` corresponding to the visibility of the item identified by `did`."] #[doc = ""] #[doc = " `DIFlags::Flag{Public,Protected,Private}` correspond to `DW_AT_accessibility`"] #[doc = " (public/protected/private) aren't exactly right for Rust, but neither is `DW_AT_visibility`"] #[doc = " (local/exported/qualified), and there's no way to set `DW_AT_visibility` in LLVM's API."] fn visibility_di_flags < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , did : DefId , type_did : DefId ,) -> DIFlags { let parent_did = cx . tcx . parent (type_did) ; let visibility = cx . tcx . visibility (did) ; match visibility { Visibility :: Public => DIFlags :: FlagPublic , Visibility :: Restricted (did) if did == parent_did => DIFlags :: FlagPrivate , Visibility :: Restricted (..) => DIFlags :: FlagProtected , } }
}

macro_rules! build_struct_type_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_struct_type_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_struct_type_di_node_introspect!();
    #[doc = " Creates the debuginfo node for a Rust struct type. Maybe be a regular struct or a tuple-struct."] fn build_struct_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , unique_type_id : UniqueTypeId < 'tcx > ,) -> DINodeCreationResult < 'll > { let struct_type = unique_type_id . expect_ty () ; let ty :: Adt (adt_def , _) = struct_type . kind () else { bug ! ("build_struct_type_di_node() called with non-struct-type: {:?}" , struct_type) ; } ; assert ! (adt_def . is_struct ()) ; let containing_scope = get_namespace_for_item (cx , adt_def . did ()) ; let struct_type_and_layout = cx . layout_of (struct_type) ; let variant_def = adt_def . non_enum_variant () ; let def_location = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { Some (file_metadata_from_def_id (cx , Some (adt_def . did ()))) } else { None } ; type_map :: build_type_with_children (cx , type_map :: stub (cx , Stub :: Struct , unique_type_id , & compute_debuginfo_type_name (cx . tcx , struct_type , false) , def_location , size_and_align_of (struct_type_and_layout) , Some (containing_scope) , visibility_di_flags (cx , adt_def . did () , adt_def . did ()) ,) , | cx , owner | { variant_def . fields . iter () . enumerate () . map (| (i , f) | { let field_name = if variant_def . ctor_kind () == Some (CtorKind :: Fn) { tuple_field_name (i) } else { Cow :: Borrowed (f . name . as_str ()) } ; let field_layout = struct_type_and_layout . field (cx , i) ; let def_id = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { Some (f . did) } else { None } ; build_field_di_node (cx , owner , & field_name [..] , field_layout , struct_type_and_layout . fields . offset (i) , visibility_di_flags (cx , f . did , adt_def . did ()) , type_di_node (cx , field_layout . ty) , def_id ,) }) . collect () } , | cx | build_generic_type_param_di_nodes (cx , struct_type) ,) }
}

macro_rules! build_upvar_field_di_nodes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_upvar_field_di_nodes in module {}", module_path!());
    };
}

mkfn!{
    build_upvar_field_di_nodes_introspect!();
    #[doc = " Builds the DW_TAG_member debuginfo nodes for the upvars of a closure or coroutine."] #[doc = " For a coroutine, this will handle upvars shared by all states."] fn build_upvar_field_di_nodes < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , closure_or_coroutine_ty : Ty < 'tcx > , closure_or_coroutine_di_node : & 'll DIType ,) -> SmallVec < & 'll DIType > { let (& def_id , up_var_tys) = match closure_or_coroutine_ty . kind () { ty :: Coroutine (def_id , args) => (def_id , args . as_coroutine () . prefix_tys ()) , ty :: Closure (def_id , args) => (def_id , args . as_closure () . upvar_tys ()) , ty :: CoroutineClosure (def_id , args) => (def_id , args . as_coroutine_closure () . upvar_tys ()) , _ => { bug ! ("build_upvar_field_di_nodes() called with non-closure-or-coroutine-type: {:?}" , closure_or_coroutine_ty) } } ; assert ! (up_var_tys . iter () . all (| t | t == cx . tcx . normalize_erasing_regions (cx . typing_env () , t))) ; let capture_names = cx . tcx . closure_saved_names_of_captured_variables (def_id) ; let layout = cx . layout_of (closure_or_coroutine_ty) ; up_var_tys . into_iter () . zip (capture_names . iter ()) . enumerate () . map (| (index , (up_var_ty , capture_name)) | { build_field_di_node (cx , closure_or_coroutine_di_node , capture_name . as_str () , cx . layout_of (up_var_ty) , layout . fields . offset (index) , DIFlags :: FlagZero , type_di_node (cx , up_var_ty) , None ,) }) . collect () }
}

macro_rules! build_tuple_type_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_tuple_type_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_tuple_type_di_node_introspect!();
    #[doc = " Builds the DW_TAG_structure_type debuginfo node for a Rust tuple type."] fn build_tuple_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , unique_type_id : UniqueTypeId < 'tcx > ,) -> DINodeCreationResult < 'll > { let tuple_type = unique_type_id . expect_ty () ; let & ty :: Tuple (component_types) = tuple_type . kind () else { bug ! ("build_tuple_type_di_node() called with non-tuple-type: {:?}" , tuple_type) } ; let tuple_type_and_layout = cx . layout_of (tuple_type) ; let type_name = compute_debuginfo_type_name (cx . tcx , tuple_type , false) ; type_map :: build_type_with_children (cx , type_map :: stub (cx , Stub :: Struct , unique_type_id , & type_name , None , size_and_align_of (tuple_type_and_layout) , NO_SCOPE_METADATA , DIFlags :: FlagZero ,) , | cx , tuple_di_node | { component_types . into_iter () . enumerate () . map (| (index , component_type) | { build_field_di_node (cx , tuple_di_node , & tuple_field_name (index) , cx . layout_of (component_type) , tuple_type_and_layout . fields . offset (index) , DIFlags :: FlagZero , type_di_node (cx , component_type) , None ,) }) . collect () } , NO_GENERICS ,) }
}

macro_rules! build_closure_env_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_closure_env_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_closure_env_di_node_introspect!();
    #[doc = " Builds the debuginfo node for a closure environment."] fn build_closure_env_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , unique_type_id : UniqueTypeId < 'tcx > ,) -> DINodeCreationResult < 'll > { let closure_env_type = unique_type_id . expect_ty () ; let & (ty :: Closure (def_id , _) | ty :: CoroutineClosure (def_id , _)) = closure_env_type . kind () else { bug ! ("build_closure_env_di_node() called with non-closure-type: {:?}" , closure_env_type) } ; let containing_scope = get_namespace_for_item (cx , def_id) ; let type_name = compute_debuginfo_type_name (cx . tcx , closure_env_type , false) ; let def_location = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { Some (file_metadata_from_def_id (cx , Some (def_id))) } else { None } ; type_map :: build_type_with_children (cx , type_map :: stub (cx , Stub :: Struct , unique_type_id , & type_name , def_location , cx . size_and_align_of (closure_env_type) , Some (containing_scope) , DIFlags :: FlagZero ,) , | cx , owner | build_upvar_field_di_nodes (cx , closure_env_type , owner) , NO_GENERICS ,) }
}

macro_rules! build_union_type_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_union_type_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_union_type_di_node_introspect!();
    #[doc = " Build the debuginfo node for a Rust `union` type."] fn build_union_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , unique_type_id : UniqueTypeId < 'tcx > ,) -> DINodeCreationResult < 'll > { let union_type = unique_type_id . expect_ty () ; let (union_def_id , variant_def) = match union_type . kind () { ty :: Adt (def , _) => (def . did () , def . non_enum_variant ()) , _ => bug ! ("build_union_type_di_node on a non-ADT") , } ; let containing_scope = get_namespace_for_item (cx , union_def_id) ; let union_ty_and_layout = cx . layout_of (union_type) ; let type_name = compute_debuginfo_type_name (cx . tcx , union_type , false) ; let def_location = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { Some (file_metadata_from_def_id (cx , Some (union_def_id))) } else { None } ; type_map :: build_type_with_children (cx , type_map :: stub (cx , Stub :: Union , unique_type_id , & type_name , def_location , size_and_align_of (union_ty_and_layout) , Some (containing_scope) , DIFlags :: FlagZero ,) , | cx , owner | { variant_def . fields . iter () . enumerate () . map (| (i , f) | { let field_layout = union_ty_and_layout . field (cx , i) ; let def_id = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { Some (f . did) } else { None } ; build_field_di_node (cx , owner , f . name . as_str () , field_layout , Size :: ZERO , DIFlags :: FlagZero , type_di_node (cx , field_layout . ty) , def_id ,) }) . collect () } , | cx | build_generic_type_param_di_nodes (cx , union_type) ,) }
}

macro_rules! build_generic_type_param_di_nodes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_generic_type_param_di_nodes in module {}", module_path!());
    };
}

mkfn!{
    build_generic_type_param_di_nodes_introspect!();
    #[doc = " Computes the type parameters for a type, if any, for the given metadata."] fn build_generic_type_param_di_nodes < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , ty : Ty < 'tcx > ,) -> SmallVec < Option < & 'll DIType > > { if let ty :: Adt (def , args) = * ty . kind () { if args . types () . next () . is_some () { let generics = cx . tcx . generics_of (def . did ()) ; let names = get_parameter_names (cx , generics) ; let template_params : SmallVec < _ > = iter :: zip (args , names) . filter_map (| (kind , name) | { kind . as_type () . map (| ty | { let actual_type = cx . tcx . normalize_erasing_regions (cx . typing_env () , ty) ; let actual_type_di_node = type_di_node (cx , actual_type) ; Some (cx . create_template_type_parameter (name . as_str () , actual_type_di_node)) }) }) . collect () ; return template_params ; } } return smallvec ! [] ; fn get_parameter_names (cx : & CodegenCx < '_ , '_ > , generics : & ty :: Generics) -> Vec < Symbol > { let mut names = generics . parent . map_or_else (Vec :: new , | def_id | get_parameter_names (cx , cx . tcx . generics_of (def_id))) ; names . extend (generics . own_params . iter () . map (| param | param . name)) ; names } }
}

macro_rules! build_global_var_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_global_var_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_global_var_di_node_introspect!();
    #[doc = " Creates debug information for the given global variable."] #[doc = ""] #[doc = " Adds the created debuginfo nodes directly to the crate's IR."] pub (crate) fn build_global_var_di_node < 'll > (cx : & CodegenCx < 'll , '_ > , def_id : DefId , global : & 'll Value ,) { if cx . dbg_cx . is_none () { return ; } if cx . sess () . opts . debuginfo != DebugInfo :: Full { return ; } let tcx = cx . tcx ; let var_scope = get_namespace_for_item (cx , def_id) ; let (file_metadata , line_number) = file_metadata_from_def_id (cx , Some (def_id)) ; let is_local_to_unit = is_node_local_to_unit (cx , def_id) ; let DefKind :: Static { nested , .. } = cx . tcx . def_kind (def_id) else { bug ! () } ; if nested { return ; } let variable_type = Instance :: mono (cx . tcx , def_id) . ty (cx . tcx , cx . typing_env ()) ; let type_di_node = type_di_node (cx , variable_type) ; let var_name = tcx . item_name (def_id) ; let var_name = var_name . as_str () ; let linkage_name = mangled_name_of_instance (cx , Instance :: mono (tcx , def_id)) . name ; let linkage_name = if var_name == linkage_name { "" } else { linkage_name } ; let global_align = cx . align_of (variable_type) ; unsafe { llvm :: LLVMRustDIBuilderCreateStaticVariable (DIB (cx) , Some (var_scope) , var_name . as_c_char_ptr () , var_name . len () , linkage_name . as_c_char_ptr () , linkage_name . len () , file_metadata , line_number , type_di_node , is_local_to_unit , global , None , global_align . bits () as u32 ,) ; } }
}

macro_rules! build_vtable_type_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_vtable_type_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_vtable_type_di_node_introspect!();
    #[doc = " Generates LLVM debuginfo for a vtable."] #[doc = ""] #[doc = " The vtable type looks like a struct with a field for each function pointer and super-trait"] #[doc = " pointer it contains (plus the `size` and `align` fields)."] #[doc = ""] #[doc = " Except for `size`, `align`, and `drop_in_place`, the field names don't try to mirror"] #[doc = " the name of the method they implement. This can be implemented in the future once there"] #[doc = " is a proper disambiguation scheme for dealing with methods from different traits that have"] #[doc = " the same name."] fn build_vtable_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , ty : Ty < 'tcx > , poly_trait_ref : Option < ty :: ExistentialTraitRef < 'tcx > > ,) -> & 'll DIType { let tcx = cx . tcx ; let vtable_entries = if let Some (poly_trait_ref) = poly_trait_ref { let trait_ref = poly_trait_ref . with_self_ty (tcx , ty) ; let trait_ref = tcx . erase_and_anonymize_regions (trait_ref) ; tcx . vtable_entries (trait_ref) } else { TyCtxt :: COMMON_VTABLE_ENTRIES } ; let void_pointer_ty = Ty :: new_imm_ptr (tcx , tcx . types . unit) ; let void_pointer_type_di_node = type_di_node (cx , void_pointer_ty) ; let usize_di_node = type_di_node (cx , tcx . types . usize) ; let pointer_layout = cx . layout_of (void_pointer_ty) ; let pointer_size = pointer_layout . size ; let pointer_align = pointer_layout . align . abi ; assert_eq ! (cx . size_and_align_of (tcx . types . usize) , (pointer_size , pointer_align)) ; let vtable_type_name = compute_debuginfo_vtable_name (cx . tcx , ty , poly_trait_ref , VTableNameKind :: Type) ; let unique_type_id = UniqueTypeId :: for_vtable_ty (tcx , ty , poly_trait_ref) ; let size = pointer_size * vtable_entries . len () as u64 ; let vtable_holder = type_di_node (cx , ty) ; build_type_with_children (cx , type_map :: stub (cx , Stub :: VTableTy { vtable_holder } , unique_type_id , & vtable_type_name , None , (size , pointer_align) , NO_SCOPE_METADATA , DIFlags :: FlagArtificial ,) , | cx , vtable_type_di_node | { vtable_entries . iter () . enumerate () . filter_map (| (index , vtable_entry) | { let (field_name , field_type_di_node) = match vtable_entry { ty :: VtblEntry :: MetadataDropInPlace => { ("drop_in_place" . to_string () , void_pointer_type_di_node) } ty :: VtblEntry :: Method (_) => { (format ! ("__method{index}") , void_pointer_type_di_node) } ty :: VtblEntry :: TraitVPtr (_) => { (format ! ("__super_trait_ptr{index}") , void_pointer_type_di_node) } ty :: VtblEntry :: MetadataAlign => ("align" . to_string () , usize_di_node) , ty :: VtblEntry :: MetadataSize => ("size" . to_string () , usize_di_node) , ty :: VtblEntry :: Vacant => return None , } ; let field_offset = pointer_size * index as u64 ; Some (build_field_di_node (cx , vtable_type_di_node , & field_name , pointer_layout , field_offset , DIFlags :: FlagZero , field_type_di_node , None ,)) }) . collect () } , NO_GENERICS ,) . di_node }
}

macro_rules! find_vtable_behind_cast_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_vtable_behind_cast in module {}", module_path!());
    };
}

mkfn!{
    find_vtable_behind_cast_introspect!();
    #[doc = " Get the global variable for the vtable."] #[doc = ""] #[doc = " When using global variables, we may have created an addrspacecast to get a pointer to the"] #[doc = " default address space if global variables are created in a different address space."] #[doc = " For modifying the vtable, we need the real global variable. This function accepts either a"] #[doc = " global variable (which is simply returned), or an addrspacecast constant expression."] #[doc = " If the given value is an addrspacecast, the cast is removed and the global variable behind"] #[doc = " the cast is returned."] fn find_vtable_behind_cast < 'll > (vtable : & 'll Value) -> & 'll Value { unsafe { if let Some (c) = llvm :: LLVMIsAConstantExpr (vtable) { if llvm :: LLVMGetConstOpcode (c) == llvm :: Opcode :: AddrSpaceCast { return llvm :: LLVMGetOperand (c , 0) . unwrap () ; } } } vtable }
}

macro_rules! apply_vcall_visibility_metadata_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function apply_vcall_visibility_metadata in module {}", module_path!());
    };
}

mkfn!{
    apply_vcall_visibility_metadata_introspect!();
    pub (crate) fn apply_vcall_visibility_metadata < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , ty : Ty < 'tcx > , trait_ref : Option < ExistentialTraitRef < 'tcx > > , vtable : & 'll Value ,) { if ! cx . sess () . opts . unstable_opts . virtual_function_elimination || cx . sess () . lto () != Lto :: Fat { return ; } enum VCallVisibility { Public = 0 , LinkageUnit = 1 , TranslationUnit = 2 , } let Some (trait_ref) = trait_ref else { return } ; let vtable = find_vtable_behind_cast (vtable) ; let trait_ref_self = trait_ref . with_self_ty (cx . tcx , ty) ; let trait_ref_self = cx . tcx . erase_and_anonymize_regions (trait_ref_self) ; let trait_def_id = trait_ref_self . def_id ; let trait_vis = cx . tcx . visibility (trait_def_id) ; let cgus = cx . sess () . codegen_units () . as_usize () ; let single_cgu = cgus == 1 ; let lto = cx . sess () . lto () ; let vcall_visibility = match (lto , trait_vis , single_cgu) { (Lto :: No | Lto :: ThinLocal , Visibility :: Public , _) | (Lto :: No , Visibility :: Restricted (_) , false) => VCallVisibility :: Public , (Lto :: Fat | Lto :: Thin , Visibility :: Public , _) | (Lto :: ThinLocal | Lto :: Thin | Lto :: Fat , Visibility :: Restricted (_) , false) => { VCallVisibility :: LinkageUnit } (_ , Visibility :: Restricted (_) , true) => VCallVisibility :: TranslationUnit , } ; let trait_ref_typeid = typeid_for_trait_ref (cx . tcx , trait_ref) ; let typeid = cx . create_metadata (trait_ref_typeid . as_bytes ()) ; unsafe { let v = [llvm :: LLVMValueAsMetadata (cx . const_usize (0)) , typeid] ; llvm :: LLVMRustGlobalAddMetadata (vtable , llvm :: MD_type as c_uint , llvm :: LLVMMDNodeInContext2 (cx . llcx , v . as_ptr () , v . len ()) ,) ; let vcall_visibility = llvm :: LLVMValueAsMetadata (cx . const_u64 (vcall_visibility as u64)) ; let vcall_visibility_metadata = llvm :: LLVMMDNodeInContext2 (cx . llcx , & vcall_visibility , 1) ; llvm :: LLVMGlobalSetMetadata (vtable , llvm :: MetadataType :: MD_vcall_visibility as c_uint , vcall_visibility_metadata ,) ; } }
}

macro_rules! create_vtable_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_vtable_di_node in module {}", module_path!());
    };
}

mkfn!{
    create_vtable_di_node_introspect!();
    #[doc = " Creates debug information for the given vtable, which is for the"] #[doc = " given type."] #[doc = ""] #[doc = " Adds the created metadata nodes directly to the crate's IR."] pub (crate) fn create_vtable_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , ty : Ty < 'tcx > , poly_trait_ref : Option < ty :: ExistentialTraitRef < 'tcx > > , vtable : & 'll Value ,) { if cx . dbg_cx . is_none () { return ; } if cx . sess () . opts . debuginfo != DebugInfo :: Full { return ; } let vtable = find_vtable_behind_cast (vtable) ; llvm :: set_unnamed_address (vtable , llvm :: UnnamedAddr :: No) ; let vtable_name = compute_debuginfo_vtable_name (cx . tcx , ty , poly_trait_ref , VTableNameKind :: GlobalVariable) ; let vtable_type_di_node = build_vtable_type_di_node (cx , ty , poly_trait_ref) ; let linkage_name = "" ; unsafe { llvm :: LLVMRustDIBuilderCreateStaticVariable (DIB (cx) , NO_SCOPE_METADATA , vtable_name . as_c_char_ptr () , vtable_name . len () , linkage_name . as_c_char_ptr () , linkage_name . len () , unknown_file_metadata (cx) , UNKNOWN_LINE_NUMBER , vtable_type_di_node , true , vtable , None , 0 ,) ; } }
}

macro_rules! extend_scope_to_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function extend_scope_to_file in module {}", module_path!());
    };
}

mkfn!{
    extend_scope_to_file_introspect!();
    #[doc = " Creates an \"extension\" of an existing `DIScope` into another file."] pub (crate) fn extend_scope_to_file < 'll > (cx : & CodegenCx < 'll , '_ > , scope_metadata : & 'll DIScope , file : & SourceFile ,) -> & 'll DILexicalBlock { let file_metadata = file_metadata (cx , file) ; unsafe { llvm :: LLVMDIBuilderCreateLexicalBlockFile (DIB (cx) , scope_metadata , file_metadata , 0u32 ,) } }
}

macro_rules! tuple_field_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function tuple_field_name in module {}", module_path!());
    };
}

mkfn!{
    tuple_field_name_introspect!();
    fn tuple_field_name (field_index : usize) -> Cow < 'static , str > { const TUPLE_FIELD_NAMES : [& 'static str ; 16] = ["__0" , "__1" , "__2" , "__3" , "__4" , "__5" , "__6" , "__7" , "__8" , "__9" , "__10" , "__11" , "__12" , "__13" , "__14" , "__15" ,] ; TUPLE_FIELD_NAMES . get (field_index) . map (| s | Cow :: from (* s)) . unwrap_or_else (| | Cow :: from (format ! ("__{field_index}"))) }
}
mkitem!{pub (crate) type DefinitionLocation < 'll > = (& 'll DIFile , c_uint) ;}

macro_rules! file_metadata_from_def_id_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function file_metadata_from_def_id in module {}", module_path!());
    };
}

mkfn!{
    file_metadata_from_def_id_introspect!();
    pub (crate) fn file_metadata_from_def_id < 'll > (cx : & CodegenCx < 'll , '_ > , def_id : Option < DefId > ,) -> DefinitionLocation < 'll > { if let Some (def_id) = def_id && let span = hygiene :: walk_chain_collapsed (cx . tcx . def_span (def_id) , DUMMY_SP) && ! span . is_dummy () { let loc = cx . lookup_debug_loc (span . lo ()) ; (file_metadata (cx , & loc . file) , loc . line) } else { (unknown_file_metadata (cx) , UNKNOWN_LINE_NUMBER) } }
}