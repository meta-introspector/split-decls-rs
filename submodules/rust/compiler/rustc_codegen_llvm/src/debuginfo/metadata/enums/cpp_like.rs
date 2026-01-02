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
mkuse!{use libc :: c_uint ;}
mkuse!{use rustc_abi :: { Align , Endian , FieldIdx , Size , TagEncoding , VariantIdx , Variants } ;}
mkuse!{use rustc_codegen_ssa :: debuginfo :: type_names :: compute_debuginfo_type_name ;}
mkuse!{use rustc_codegen_ssa :: debuginfo :: { tag_base_type , wants_c_like_enum_debuginfo } ;}
mkuse!{use rustc_codegen_ssa :: traits :: { ConstCodegenMethods , MiscCodegenMethods } ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: layout :: { LayoutOf , TyAndLayout } ;}
mkuse!{use rustc_middle :: ty :: { self , AdtDef , CoroutineArgs , CoroutineArgsExt , Ty } ;}
mkuse!{use smallvec :: smallvec ;}
mkuse!{use crate :: common :: { AsCCharPtr , CodegenCx } ;}
mkuse!{use crate :: debuginfo :: dwarf_const :: DW_TAG_const_type ;}
mkuse!{use crate :: debuginfo :: metadata :: enums :: DiscrResult ;}
mkuse!{use crate :: debuginfo :: metadata :: type_map :: { self , Stub , UniqueTypeId } ;}
mkuse!{use crate :: debuginfo :: metadata :: { DINodeCreationResult , NO_GENERICS , NO_SCOPE_METADATA , SmallVec , UNKNOWN_LINE_NUMBER , build_field_di_node , create_member_type , file_metadata , file_metadata_from_def_id , size_and_align_of , type_di_node , unknown_file_metadata , visibility_di_flags , } ;}
mkuse!{use crate :: debuginfo :: utils :: DIB ;}
mkuse!{use crate :: llvm :: debuginfo :: { DIFile , DIFlags , DIType } ;}
mkuse!{use crate :: llvm :: { self } ;}
mkitem!{const ASSOC_CONST_DISCR_NAME : & str = "NAME" ;}
mkitem!{const ASSOC_CONST_DISCR_EXACT : & str = "DISCR_EXACT" ;}
mkitem!{const ASSOC_CONST_DISCR_BEGIN : & str = "DISCR_BEGIN" ;}
mkitem!{const ASSOC_CONST_DISCR_END : & str = "DISCR_END" ;}
mkitem!{const ASSOC_CONST_DISCR128_EXACT_LO : & str = "DISCR128_EXACT_LO" ;}
mkitem!{const ASSOC_CONST_DISCR128_EXACT_HI : & str = "DISCR128_EXACT_HI" ;}
mkitem!{const ASSOC_CONST_DISCR128_BEGIN_LO : & str = "DISCR128_BEGIN_LO" ;}
mkitem!{const ASSOC_CONST_DISCR128_BEGIN_HI : & str = "DISCR128_BEGIN_HI" ;}
mkitem!{const ASSOC_CONST_DISCR128_END_LO : & str = "DISCR128_END_LO" ;}
mkitem!{const ASSOC_CONST_DISCR128_END_HI : & str = "DISCR128_END_HI" ;}
mkitem!{const TAG_FIELD_NAME : & str = "tag" ;}
mkitem!{const TAG_FIELD_NAME_128_LO : & str = "tag128_lo" ;}
mkitem!{const TAG_FIELD_NAME_128_HI : & str = "tag128_hi" ;}
mkitem!{const SINGLE_VARIANT_VIRTUAL_DISR : u64 = 0 ;}

macro_rules! build_enum_type_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_enum_type_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_enum_type_di_node_introspect!();
    #[doc = " In CPP-like mode, we generate a union with a field for each variant and an"] #[doc = " explicit tag field. The field of each variant has a struct type"] #[doc = " that encodes the discriminant of the variant and it's data layout."] #[doc = " The union also has a nested enumeration type that is only used for encoding"] #[doc = " variant names in an efficient way. Its enumerator values do _not_ correspond"] #[doc = " to the enum's discriminant values."] #[doc = " It's roughly equivalent to the following C/C++ code:"] #[doc = ""] #[doc = " ```c"] #[doc = " union enum2$<{fully-qualified-name}> {"] #[doc = "   struct Variant0 {"] #[doc = "     struct {name-of-variant-0} {"] #[doc = "        <variant 0 fields>"] #[doc = "     } value;"] #[doc = ""] #[doc = "     static VariantNames NAME = {name-of-variant-0};"] #[doc = "     static int_type DISCR_EXACT = {discriminant-of-variant-0};"] #[doc = "   } variant0;"] #[doc = ""] #[doc = "   <other variant structs>"] #[doc = ""] #[doc = "   int_type tag;"] #[doc = ""] #[doc = "   enum VariantNames {"] #[doc = "      <name-of-variant-0> = 0, // The numeric values are variant index,"] #[doc = "      <name-of-variant-1> = 1, // not discriminant values."] #[doc = "      <name-of-variant-2> = 2,"] #[doc = "      ..."] #[doc = "   }"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " As you can see, the type name is wrapped in `enum2$<_>`. This way we can"] #[doc = " have a single NatVis rule for handling all enums. The `2` in `enum2$<_>`"] #[doc = " is an encoding version tag, so that debuggers can decide to decode this"] #[doc = " differently than the previous `enum$<_>` encoding emitted by earlier"] #[doc = " compiler versions."] #[doc = ""] #[doc = " Niche-tag enums have one special variant, usually called the"] #[doc = " \"untagged variant\". This variant has a field that"] #[doc = " doubles as the tag of the enum. The variant is active when the value of"] #[doc = " that field is within a pre-defined range. Therefore the variant struct"] #[doc = " has a `DISCR_BEGIN` and `DISCR_END` field instead of `DISCR_EXACT` in"] #[doc = " that case. Both `DISCR_BEGIN` and `DISCR_END` are inclusive bounds."] #[doc = " Note that these ranges can wrap around, so that `DISCR_END < DISCR_BEGIN`."] #[doc = ""] #[doc = " Single-variant enums don't actually have a tag field. In this case we"] #[doc = " emit a static tag field (that always has the value 0) so we can use the"] #[doc = " same representation (and NatVis)."] #[doc = ""] #[doc = " For niche-layout enums it's possible to have a 128-bit tag. NatVis, VS, and"] #[doc = " WinDbg (the main targets for CPP-like debuginfo at the moment) don't support"] #[doc = " 128-bit integers, so all values involved get split into two 64-bit fields."] #[doc = " Instead of the `tag` field, we generate two fields `tag128_lo` and `tag128_hi`,"] #[doc = " Instead of `DISCR_EXACT`, we generate `DISCR128_EXACT_LO` and `DISCR128_EXACT_HI`,"] #[doc = " and so on."] #[doc = ""] #[doc = ""] #[doc = " The following pseudocode shows how to decode an enum value in a debugger:"] #[doc = ""] #[doc = " ```text"] #[doc = ""] #[doc = " fn find_active_variant(enum_value) -> (VariantName, VariantValue) {"] #[doc = "     let is_128_bit = enum_value.has_field(\"tag128_lo\");"] #[doc = ""] #[doc = "     if !is_128_bit {"] #[doc = "         // Note: `tag` can be a static field for enums with only one"] #[doc = "         //       inhabited variant."] #[doc = "         let tag = enum_value.field(\"tag\").value;"] #[doc = ""] #[doc = "         // For each variant, check if it is a match. Only one of them will match,"] #[doc = "         // so if we find it we can return it immediately."] #[doc = "         for variant_field in enum_value.fields().filter(|f| f.name.starts_with(\"variant\")) {"] #[doc = "             if variant_field.has_field(\"DISCR_EXACT\") {"] #[doc = "                 // This variant corresponds to a single tag value"] #[doc = "                 if variant_field.field(\"DISCR_EXACT\").value == tag {"] #[doc = "                     return (variant_field.field(\"NAME\"), variant_field.value);"] #[doc = "                 }"] #[doc = "             } else {"] #[doc = "                 // This is a range variant"] #[doc = "                 let begin = variant_field.field(\"DISCR_BEGIN\");"] #[doc = "                 let end = variant_field.field(\"DISCR_END\");"] #[doc = ""] #[doc = "                 if is_in_range(tag, begin, end) {"] #[doc = "                     return (variant_field.field(\"NAME\"), variant_field.value);"] #[doc = "                 }"] #[doc = "             }"] #[doc = "         }"] #[doc = "     } else {"] #[doc = "         // Basically the same as with smaller tags, we just have to"] #[doc = "         // stitch the values together."] #[doc = "         let tag: u128 = (enum_value.field(\"tag128_lo\").value as u128) |"] #[doc = "                         (enum_value.field(\"tag128_hi\").value as u128 << 64);"] #[doc = ""] #[doc = "         for variant_field in enum_value.fields().filter(|f| f.name.starts_with(\"variant\")) {"] #[doc = "             if variant_field.has_field(\"DISCR128_EXACT_LO\") {"] #[doc = "                 let discr_exact = (variant_field.field(\"DISCR128_EXACT_LO\" as u128) |"] #[doc = "                                   (variant_field.field(\"DISCR128_EXACT_HI\") as u128 << 64);"] #[doc = ""] #[doc = "                 // This variant corresponds to a single tag value"] #[doc = "                 if discr_exact.value == tag {"] #[doc = "                     return (variant_field.field(\"NAME\"), variant_field.value);"] #[doc = "                 }"] #[doc = "             } else {"] #[doc = "                 // This is a range variant"] #[doc = "                 let begin = (variant_field.field(\"DISCR128_BEGIN_LO\").value as u128) |"] #[doc = "                             (variant_field.field(\"DISCR128_BEGIN_HI\").value as u128 << 64);"] #[doc = "                 let end = (variant_field.field(\"DISCR128_END_LO\").value as u128) |"] #[doc = "                           (variant_field.field(\"DISCR128_END_HI\").value as u128 << 64);"] #[doc = ""] #[doc = "                 if is_in_range(tag, begin, end) {"] #[doc = "                     return (variant_field.field(\"NAME\"), variant_field.value);"] #[doc = "                 }"] #[doc = "             }"] #[doc = "         }"] #[doc = "     }"] #[doc = ""] #[doc = "     // We should have found an active variant at this point."] #[doc = "     unreachable!();"] #[doc = " }"] #[doc = ""] #[doc = " // Check if a value is within the given range"] #[doc = " // (where the range might wrap around the value space)"] #[doc = " fn is_in_range(value, start, end) -> bool {"] #[doc = "     if start < end {"] #[doc = "         value >= start && value <= end"] #[doc = "     } else {"] #[doc = "         value >= start || value <= end"] #[doc = "     }"] #[doc = " }"] #[doc = ""] #[doc = " ```"] pub (super) fn build_enum_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , unique_type_id : UniqueTypeId < 'tcx > ,) -> DINodeCreationResult < 'll > { let enum_type = unique_type_id . expect_ty () ; let & ty :: Adt (enum_adt_def , _) = enum_type . kind () else { bug ! ("build_enum_type_di_node() called with non-enum type: `{:?}`" , enum_type) } ; let enum_type_and_layout = cx . layout_of (enum_type) ; let enum_type_name = compute_debuginfo_type_name (cx . tcx , enum_type , false) ; assert ! (! wants_c_like_enum_debuginfo (cx . tcx , enum_type_and_layout)) ; let def_location = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { Some (file_metadata_from_def_id (cx , Some (enum_adt_def . did ()))) } else { None } ; type_map :: build_type_with_children (cx , type_map :: stub (cx , type_map :: Stub :: Union , unique_type_id , & enum_type_name , def_location , cx . size_and_align_of (enum_type) , NO_SCOPE_METADATA , visibility_di_flags (cx , enum_adt_def . did () , enum_adt_def . did ()) ,) , | cx , enum_type_di_node | { match enum_type_and_layout . variants { Variants :: Empty => { return smallvec ! [] ; } Variants :: Single { index : variant_index } => build_single_variant_union_fields (cx , enum_adt_def , enum_type_and_layout , enum_type_di_node , variant_index ,) , Variants :: Multiple { tag_encoding : TagEncoding :: Direct , ref variants , tag_field , .. } => build_union_fields_for_enum (cx , enum_adt_def , enum_type_and_layout , enum_type_di_node , variants . indices () , tag_field , None ,) , Variants :: Multiple { tag_encoding : TagEncoding :: Niche { untagged_variant , .. } , ref variants , tag_field , .. } => build_union_fields_for_enum (cx , enum_adt_def , enum_type_and_layout , enum_type_di_node , variants . indices () , tag_field , Some (untagged_variant) ,) , } } , NO_GENERICS ,) }
}

macro_rules! build_coroutine_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_coroutine_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_coroutine_di_node_introspect!();
    #[doc = " A coroutine debuginfo node looks the same as a that of an enum type."] #[doc = ""] #[doc = " See [build_enum_type_di_node] for more information."] pub (super) fn build_coroutine_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , unique_type_id : UniqueTypeId < 'tcx > ,) -> DINodeCreationResult < 'll > { let coroutine_type = unique_type_id . expect_ty () ; let def_location = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { let & ty :: Coroutine (coroutine_def_id , _) = coroutine_type . kind () else { bug ! ("build_coroutine_di_node() called with non-coroutine type: `{:?}`" , coroutine_type) } ; Some (file_metadata_from_def_id (cx , Some (coroutine_def_id))) } else { None } ; let coroutine_type_and_layout = cx . layout_of (coroutine_type) ; let coroutine_type_name = compute_debuginfo_type_name (cx . tcx , coroutine_type , false) ; assert ! (! wants_c_like_enum_debuginfo (cx . tcx , coroutine_type_and_layout)) ; type_map :: build_type_with_children (cx , type_map :: stub (cx , type_map :: Stub :: Union , unique_type_id , & coroutine_type_name , def_location , size_and_align_of (coroutine_type_and_layout) , NO_SCOPE_METADATA , DIFlags :: FlagZero ,) , | cx , coroutine_type_di_node | match coroutine_type_and_layout . variants { Variants :: Multiple { tag_encoding : TagEncoding :: Direct , .. } => { build_union_fields_for_direct_tag_coroutine (cx , coroutine_type_and_layout , coroutine_type_di_node ,) } Variants :: Single { .. } | Variants :: Empty | Variants :: Multiple { tag_encoding : TagEncoding :: Niche { .. } , .. } => { bug ! ("Encountered coroutine with non-direct-tag layout: {:?}" , coroutine_type_and_layout) } } , NO_GENERICS ,) }
}

macro_rules! build_single_variant_union_fields_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_single_variant_union_fields in module {}", module_path!());
    };
}

mkfn!{
    build_single_variant_union_fields_introspect!();
    fn build_single_variant_union_fields < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , enum_adt_def : AdtDef < 'tcx > , enum_type_and_layout : TyAndLayout < 'tcx > , enum_type_di_node : & 'll DIType , variant_index : VariantIdx ,) -> SmallVec < & 'll DIType > { let variant_layout = enum_type_and_layout . for_variant (cx , variant_index) ; let visibility_flags = visibility_di_flags (cx , enum_adt_def . did () , enum_adt_def . did ()) ; let variant_struct_type_di_node = super :: build_enum_variant_struct_type_di_node (cx , enum_type_and_layout , enum_type_di_node , variant_index , enum_adt_def . variant (variant_index) , variant_layout , visibility_flags ,) ; let tag_base_type = cx . tcx . types . u32 ; let tag_base_type_di_node = type_di_node (cx , tag_base_type) ; let tag_base_type_align = cx . align_of (tag_base_type) ; let enum_adt_def_id = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { Some (enum_adt_def . did ()) } else { None } ; let variant_names_type_di_node = build_variant_names_type_di_node (cx , enum_type_di_node , std :: iter :: once ((variant_index , Cow :: from (enum_adt_def . variant (variant_index) . name . as_str ()) ,)) , enum_adt_def_id ,) ; let variant_struct_type_wrapper_di_node = build_variant_struct_wrapper_type_di_node (cx , enum_type_and_layout , enum_type_di_node , variant_index , None , variant_struct_type_di_node , variant_names_type_di_node , tag_base_type_di_node , tag_base_type , DiscrResult :: NoDiscriminant , None ,) ; smallvec ! [build_field_di_node (cx , enum_type_di_node , & variant_union_field_name (variant_index) , enum_type_and_layout , Size :: ZERO , visibility_flags , variant_struct_type_wrapper_di_node , None ,) , unsafe { llvm :: LLVMRustDIBuilderCreateStaticMemberType (DIB (cx) , enum_type_di_node , TAG_FIELD_NAME . as_c_char_ptr () , TAG_FIELD_NAME . len () , unknown_file_metadata (cx) , UNKNOWN_LINE_NUMBER , variant_names_type_di_node , visibility_flags , Some (cx . const_u64 (SINGLE_VARIANT_VIRTUAL_DISR)) , tag_base_type_align . bits () as u32 ,) }] }
}

macro_rules! build_union_fields_for_enum_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_union_fields_for_enum in module {}", module_path!());
    };
}

mkfn!{
    build_union_fields_for_enum_introspect!();
    fn build_union_fields_for_enum < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , enum_adt_def : AdtDef < 'tcx > , enum_type_and_layout : TyAndLayout < 'tcx > , enum_type_di_node : & 'll DIType , variant_indices : impl Iterator < Item = VariantIdx > + Clone , tag_field : FieldIdx , untagged_variant_index : Option < VariantIdx > ,) -> SmallVec < & 'll DIType > { let tag_base_type = tag_base_type (cx . tcx , enum_type_and_layout) ; let enum_adt_def_id = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { Some (enum_adt_def . did ()) } else { None } ; let variant_names_type_di_node = build_variant_names_type_di_node (cx , enum_type_di_node , variant_indices . clone () . map (| variant_index | { let variant_name = Cow :: from (enum_adt_def . variant (variant_index) . name . as_str ()) ; (variant_index , variant_name) }) , enum_adt_def_id ,) ; let visibility_flags = visibility_di_flags (cx , enum_adt_def . did () , enum_adt_def . did ()) ; let variant_field_infos : SmallVec < VariantFieldInfo < 'll > > = variant_indices . map (| variant_index | { let variant_layout = enum_type_and_layout . for_variant (cx , variant_index) ; let variant_def = enum_adt_def . variant (variant_index) ; let variant_struct_type_di_node = super :: build_enum_variant_struct_type_di_node (cx , enum_type_and_layout , enum_type_di_node , variant_index , variant_def , variant_layout , visibility_flags ,) ; VariantFieldInfo { variant_index , variant_struct_type_di_node , source_info : None , discr : super :: compute_discriminant_value (cx , enum_type_and_layout , variant_index) , } }) . collect () ; build_union_fields_for_direct_tag_enum_or_coroutine (cx , enum_type_and_layout , enum_type_di_node , & variant_field_infos , variant_names_type_di_node , tag_base_type , tag_field , untagged_variant_index , visibility_flags ,) }
}

macro_rules! variant_names_enum_base_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function variant_names_enum_base_type in module {}", module_path!());
    };
}

mkfn!{
    variant_names_enum_base_type_introspect!();
    fn variant_names_enum_base_type < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx >) -> Ty < 'tcx > { cx . tcx . types . u32 }
}

macro_rules! build_variant_names_type_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_variant_names_type_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_variant_names_type_di_node_introspect!();
    #[doc = " This function builds a DW_AT_enumeration_type that contains an entry for"] #[doc = " each variant. Note that this has nothing to do with the discriminant. The"] #[doc = " numeric value of each enumerator corresponds to the variant index. The"] #[doc = " type is only used for efficiently encoding the name of each variant in"] #[doc = " debuginfo."] fn build_variant_names_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , containing_scope : & 'll DIType , variants : impl Iterator < Item = (VariantIdx , Cow < 'tcx , str >) > , enum_def_id : Option < rustc_span :: def_id :: DefId > ,) -> & 'll DIType { super :: build_enumeration_type_di_node (cx , "VariantNames" , variant_names_enum_base_type (cx) , variants . map (| (variant_index , variant_name) | (variant_name , variant_index . as_u32 () . into ())) , enum_def_id , containing_scope ,) }
}

macro_rules! build_variant_struct_wrapper_type_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_variant_struct_wrapper_type_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_variant_struct_wrapper_type_di_node_introspect!();
    fn build_variant_struct_wrapper_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , enum_or_coroutine_type_and_layout : TyAndLayout < 'tcx > , enum_or_coroutine_type_di_node : & 'll DIType , variant_index : VariantIdx , untagged_variant_index : Option < VariantIdx > , variant_struct_type_di_node : & 'll DIType , variant_names_type_di_node : & 'll DIType , tag_base_type_di_node : & 'll DIType , tag_base_type : Ty < 'tcx > , discr : DiscrResult , source_info : Option < (& 'll DIFile , c_uint) > ,) -> & 'll DIType { type_map :: build_type_with_children (cx , type_map :: stub (cx , Stub :: Struct , UniqueTypeId :: for_enum_variant_struct_type_wrapper (cx . tcx , enum_or_coroutine_type_and_layout . ty , variant_index ,) , & variant_struct_wrapper_type_name (variant_index) , source_info , size_and_align_of (enum_or_coroutine_type_and_layout) , Some (enum_or_coroutine_type_di_node) , DIFlags :: FlagZero ,) , | cx , wrapper_struct_type_di_node | { enum DiscrKind { Exact (u64) , Exact128 (u128) , Range (u64 , u64) , Range128 (u128 , u128) , } let (tag_base_type_size , tag_base_type_align) = cx . size_and_align_of (tag_base_type) ; let is_128_bits = tag_base_type_size . bits () > 64 ; let discr = match discr { DiscrResult :: NoDiscriminant => DiscrKind :: Exact (SINGLE_VARIANT_VIRTUAL_DISR) , DiscrResult :: Value (discr_val) => { if is_128_bits { DiscrKind :: Exact128 (discr_val) } else { assert_eq ! (discr_val , discr_val as u64 as u128) ; DiscrKind :: Exact (discr_val as u64) } } DiscrResult :: Range (min , max) => { assert_eq ! (Some (variant_index) , untagged_variant_index) ; if is_128_bits { DiscrKind :: Range128 (min , max) } else { assert_eq ! (min , min as u64 as u128) ; assert_eq ! (max , max as u64 as u128) ; DiscrKind :: Range (min as u64 , max as u64) } } } ; let mut fields = SmallVec :: new () ; fields . push (build_field_di_node (cx , wrapper_struct_type_di_node , "value" , enum_or_coroutine_type_and_layout , Size :: ZERO , DIFlags :: FlagZero , variant_struct_type_di_node , None ,)) ; let build_assoc_const = | name : & str , type_di_node_ : & 'll DIType , value : u64 , align : Align | unsafe { let (t_di , align) = if name == ASSOC_CONST_DISCR_NAME { (type_di_node_ , align . bits () as u32) } else { let ty_u64 = Ty :: new_uint (cx . tcx , ty :: UintTy :: U64) ; (type_di_node (cx , ty_u64) , Align :: EIGHT . bits () as u32) } ; let field_type = llvm :: LLVMRustDIBuilderCreateQualifiedType (DIB (cx) , DW_TAG_const_type , t_di) ; llvm :: LLVMRustDIBuilderCreateStaticMemberType (DIB (cx) , wrapper_struct_type_di_node , name . as_c_char_ptr () , name . len () , unknown_file_metadata (cx) , UNKNOWN_LINE_NUMBER , field_type , DIFlags :: FlagZero , Some (cx . const_u64 (value)) , align ,) } ; fields . push (build_assoc_const (ASSOC_CONST_DISCR_NAME , variant_names_type_di_node , variant_index . as_u32 () as u64 , cx . align_of (variant_names_enum_base_type (cx)) ,)) ; match discr { DiscrKind :: Exact (discr_val) => { fields . push (build_assoc_const (ASSOC_CONST_DISCR_EXACT , tag_base_type_di_node , discr_val , tag_base_type_align ,)) ; } DiscrKind :: Exact128 (discr_val) => { let align = cx . align_of (cx . tcx . types . u64) ; let type_di_node = type_di_node (cx , cx . tcx . types . u64) ; let Split128 { hi , lo } = split_128 (discr_val) ; fields . push (build_assoc_const (ASSOC_CONST_DISCR128_EXACT_LO , type_di_node , lo , align ,)) ; fields . push (build_assoc_const (ASSOC_CONST_DISCR128_EXACT_HI , type_di_node , hi , align ,)) ; } DiscrKind :: Range (begin , end) => { fields . push (build_assoc_const (ASSOC_CONST_DISCR_BEGIN , tag_base_type_di_node , begin , tag_base_type_align ,)) ; fields . push (build_assoc_const (ASSOC_CONST_DISCR_END , tag_base_type_di_node , end , tag_base_type_align ,)) ; } DiscrKind :: Range128 (begin , end) => { let align = cx . align_of (cx . tcx . types . u64) ; let type_di_node = type_di_node (cx , cx . tcx . types . u64) ; let Split128 { hi : begin_hi , lo : begin_lo } = split_128 (begin) ; let Split128 { hi : end_hi , lo : end_lo } = split_128 (end) ; fields . push (build_assoc_const (ASSOC_CONST_DISCR128_BEGIN_HI , type_di_node , begin_hi , align ,)) ; fields . push (build_assoc_const (ASSOC_CONST_DISCR128_BEGIN_LO , type_di_node , begin_lo , align ,)) ; fields . push (build_assoc_const (ASSOC_CONST_DISCR128_END_HI , type_di_node , end_hi , align ,)) ; fields . push (build_assoc_const (ASSOC_CONST_DISCR128_END_LO , type_di_node , end_lo , align ,)) ; } } fields } , NO_GENERICS ,) . di_node }
}
mkitem!{mkstruct!{struct Split128 { hi : u64 , lo : u64 , }}}

macro_rules! split_128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function split_128 in module {}", module_path!());
    };
}

mkfn!{
    split_128_introspect!();
    fn split_128 (value : u128) -> Split128 { Split128 { hi : (value >> 64) as u64 , lo : value as u64 } }
}

macro_rules! build_union_fields_for_direct_tag_coroutine_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_union_fields_for_direct_tag_coroutine in module {}", module_path!());
    };
}

mkfn!{
    build_union_fields_for_direct_tag_coroutine_introspect!();
    fn build_union_fields_for_direct_tag_coroutine < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , coroutine_type_and_layout : TyAndLayout < 'tcx > , coroutine_type_di_node : & 'll DIType ,) -> SmallVec < & 'll DIType > { let Variants :: Multiple { tag_encoding : TagEncoding :: Direct , tag_field , .. } = coroutine_type_and_layout . variants else { bug ! ("This function only supports layouts with directly encoded tags.") } ; let (coroutine_def_id , coroutine_args) = match coroutine_type_and_layout . ty . kind () { & ty :: Coroutine (def_id , args) => (def_id , args . as_coroutine ()) , _ => unreachable ! () , } ; let coroutine_layout = cx . tcx . coroutine_layout (coroutine_def_id , coroutine_args . args) . unwrap () ; let common_upvar_names = cx . tcx . closure_saved_names_of_captured_variables (coroutine_def_id) ; let variant_range = coroutine_args . variant_range (coroutine_def_id , cx . tcx) ; let variant_count = (variant_range . start . as_u32 () .. variant_range . end . as_u32 ()) . len () ; let tag_base_type = tag_base_type (cx . tcx , coroutine_type_and_layout) ; let variant_names_type_di_node = build_variant_names_type_di_node (cx , coroutine_type_di_node , variant_range . clone () . map (| variant_index | (variant_index , CoroutineArgs :: variant_name (variant_index))) , if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { Some (coroutine_def_id) } else { None } ,) ; let discriminants : IndexVec < VariantIdx , DiscrResult > = { let discriminants_iter = coroutine_args . discriminants (coroutine_def_id , cx . tcx) ; let mut discriminants : IndexVec < VariantIdx , DiscrResult > = IndexVec :: with_capacity (variant_count) ; for (variant_index , discr) in discriminants_iter { assert_eq ! (variant_index , discriminants . next_index ()) ; discriminants . push (DiscrResult :: Value (discr . val)) ; } discriminants } ; let variant_field_infos : SmallVec < VariantFieldInfo < 'll > > = variant_range . map (| variant_index | { let variant_struct_type_di_node = super :: build_coroutine_variant_struct_type_di_node (cx , variant_index , coroutine_type_and_layout , coroutine_type_di_node , coroutine_layout , common_upvar_names ,) ; let span = coroutine_layout . variant_source_info [variant_index] . span ; let source_info = if ! span . is_dummy () { let loc = cx . lookup_debug_loc (span . lo ()) ; Some ((file_metadata (cx , & loc . file) , loc . line as c_uint)) } else { None } ; VariantFieldInfo { variant_index , variant_struct_type_di_node , source_info , discr : discriminants [variant_index] , } }) . collect () ; build_union_fields_for_direct_tag_enum_or_coroutine (cx , coroutine_type_and_layout , coroutine_type_di_node , & variant_field_infos [..] , variant_names_type_di_node , tag_base_type , tag_field , None , DIFlags :: FlagZero ,) }
}

macro_rules! build_union_fields_for_direct_tag_enum_or_coroutine_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_union_fields_for_direct_tag_enum_or_coroutine in module {}", module_path!());
    };
}

mkfn!{
    build_union_fields_for_direct_tag_enum_or_coroutine_introspect!();
    #[doc = " This is a helper function shared between enums and coroutines that makes sure fields have the"] #[doc = " expect names."] fn build_union_fields_for_direct_tag_enum_or_coroutine < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , enum_type_and_layout : TyAndLayout < 'tcx > , enum_type_di_node : & 'll DIType , variant_field_infos : & [VariantFieldInfo < 'll >] , discr_type_di_node : & 'll DIType , tag_base_type : Ty < 'tcx > , tag_field : FieldIdx , untagged_variant_index : Option < VariantIdx > , di_flags : DIFlags ,) -> SmallVec < & 'll DIType > { let tag_base_type_di_node = type_di_node (cx , tag_base_type) ; let mut unions_fields = SmallVec :: with_capacity (variant_field_infos . len () + 1) ; unions_fields . extend (variant_field_infos . into_iter () . map (| variant_member_info | { let (file_di_node , line_number) = variant_member_info . source_info . unwrap_or_else (| | (unknown_file_metadata (cx) , UNKNOWN_LINE_NUMBER)) ; let field_name = variant_union_field_name (variant_member_info . variant_index) ; let variant_struct_type_wrapper = build_variant_struct_wrapper_type_di_node (cx , enum_type_and_layout , enum_type_di_node , variant_member_info . variant_index , untagged_variant_index , variant_member_info . variant_struct_type_di_node , discr_type_di_node , tag_base_type_di_node , tag_base_type , variant_member_info . discr , if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { variant_member_info . source_info } else { None } ,) ; create_member_type (cx , enum_type_di_node , & field_name , file_di_node , line_number , enum_type_and_layout , Size :: ZERO , di_flags , variant_struct_type_wrapper ,) })) ; assert_eq ! (cx . size_and_align_of (enum_type_and_layout . field (cx , tag_field . as_usize ()) . ty) , cx . size_and_align_of (self :: tag_base_type (cx . tcx , enum_type_and_layout))) ; let is_128_bits = cx . size_of (tag_base_type) . bits () > 64 ; if is_128_bits { let type_di_node = type_di_node (cx , cx . tcx . types . u64) ; let u64_layout = cx . layout_of (cx . tcx . types . u64) ; let (lo_offset , hi_offset) = match cx . tcx . data_layout . endian { Endian :: Little => (0 , 8) , Endian :: Big => (8 , 0) , } ; let tag_field_offset = enum_type_and_layout . fields . offset (tag_field . as_usize ()) . bytes () ; let lo_offset = Size :: from_bytes (tag_field_offset + lo_offset) ; let hi_offset = Size :: from_bytes (tag_field_offset + hi_offset) ; unions_fields . push (build_field_di_node (cx , enum_type_di_node , TAG_FIELD_NAME_128_LO , u64_layout , lo_offset , di_flags , type_di_node , None ,)) ; unions_fields . push (build_field_di_node (cx , enum_type_di_node , TAG_FIELD_NAME_128_HI , u64_layout , hi_offset , DIFlags :: FlagZero , type_di_node , None ,)) ; } else { unions_fields . push (build_field_di_node (cx , enum_type_di_node , TAG_FIELD_NAME , enum_type_and_layout . field (cx , tag_field . as_usize ()) , enum_type_and_layout . fields . offset (tag_field . as_usize ()) , di_flags , tag_base_type_di_node , None ,)) ; } unions_fields }
}
mkitem!{mkstruct!{#[doc = " Information about a single field of the top-level DW_TAG_union_type."] struct VariantFieldInfo < 'll > { variant_index : VariantIdx , variant_struct_type_di_node : & 'll DIType , source_info : Option < (& 'll DIFile , c_uint) > , discr : DiscrResult , }}}

macro_rules! variant_union_field_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function variant_union_field_name in module {}", module_path!());
    };
}

mkfn!{
    variant_union_field_name_introspect!();
    fn variant_union_field_name (variant_index : VariantIdx) -> Cow < 'static , str > { const PRE_ALLOCATED : [& str ; 16] = ["variant0" , "variant1" , "variant2" , "variant3" , "variant4" , "variant5" , "variant6" , "variant7" , "variant8" , "variant9" , "variant10" , "variant11" , "variant12" , "variant13" , "variant14" , "variant15" ,] ; PRE_ALLOCATED . get (variant_index . as_usize ()) . map (| & s | Cow :: from (s)) . unwrap_or_else (| | format ! ("variant{}" , variant_index . as_usize ()) . into ()) }
}

macro_rules! variant_struct_wrapper_type_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function variant_struct_wrapper_type_name in module {}", module_path!());
    };
}

mkfn!{
    variant_struct_wrapper_type_name_introspect!();
    fn variant_struct_wrapper_type_name (variant_index : VariantIdx) -> Cow < 'static , str > { const PRE_ALLOCATED : [& str ; 16] = ["Variant0" , "Variant1" , "Variant2" , "Variant3" , "Variant4" , "Variant5" , "Variant6" , "Variant7" , "Variant8" , "Variant9" , "Variant10" , "Variant11" , "Variant12" , "Variant13" , "Variant14" , "Variant15" ,] ; PRE_ALLOCATED . get (variant_index . as_usize ()) . map (| & s | Cow :: from (s)) . unwrap_or_else (| | format ! ("Variant{}" , variant_index . as_usize ()) . into ()) }
}