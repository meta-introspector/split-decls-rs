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
mkuse!{use rustc_abi :: { Size , TagEncoding , VariantIdx , Variants } ;}
mkuse!{use rustc_codegen_ssa :: debuginfo :: type_names :: compute_debuginfo_type_name ;}
mkuse!{use rustc_codegen_ssa :: debuginfo :: { tag_base_type , wants_c_like_enum_debuginfo } ;}
mkuse!{use rustc_codegen_ssa :: traits :: { ConstCodegenMethods , MiscCodegenMethods } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: layout :: { LayoutOf , TyAndLayout } ;}
mkuse!{use rustc_middle :: ty :: { self } ;}
mkuse!{use smallvec :: smallvec ;}
mkuse!{use crate :: common :: { AsCCharPtr , CodegenCx } ;}
mkuse!{use crate :: debuginfo :: metadata :: type_map :: { self , Stub , StubInfo , UniqueTypeId } ;}
mkuse!{use crate :: debuginfo :: metadata :: { DINodeCreationResult , NO_GENERICS , SmallVec , UNKNOWN_LINE_NUMBER , create_member_type , file_metadata , file_metadata_from_def_id , size_and_align_of , type_di_node , unknown_file_metadata , visibility_di_flags , } ;}
mkuse!{use crate :: debuginfo :: utils :: { DIB , create_DIArray , get_namespace_for_item } ;}
mkuse!{use crate :: llvm :: debuginfo :: { DIFile , DIFlags , DIType } ;}
mkuse!{use crate :: llvm :: { self } ;}

macro_rules! build_enum_type_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_enum_type_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_enum_type_di_node_introspect!();
    #[doc = " Build the debuginfo node for an enum type. The listing below shows how such a"] #[doc = " type looks like at the LLVM IR/DWARF level. It is a `DW_TAG_structure_type`"] #[doc = " with a single `DW_TAG_variant_part` that in turn contains a `DW_TAG_variant`"] #[doc = " for each variant of the enum. The variant-part also contains a single member"] #[doc = " describing the discriminant, and a nested struct type for each of the variants."] #[doc = ""] #[doc = " ```txt"] #[doc = "  ---> DW_TAG_structure_type              (top-level type for enum)"] #[doc = "         DW_TAG_variant_part              (variant part)"] #[doc = "           DW_AT_discr                    (reference to discriminant DW_TAG_member)"] #[doc = "           DW_TAG_member                  (discriminant member)"] #[doc = "           DW_TAG_variant                 (variant 1)"] #[doc = "           DW_TAG_variant                 (variant 2)"] #[doc = "           DW_TAG_variant                 (variant 3)"] #[doc = "         DW_TAG_structure_type            (type of variant 1)"] #[doc = "         DW_TAG_structure_type            (type of variant 2)"] #[doc = "         DW_TAG_structure_type            (type of variant 3)"] #[doc = " ```"] pub (super) fn build_enum_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , unique_type_id : UniqueTypeId < 'tcx > ,) -> DINodeCreationResult < 'll > { let enum_type = unique_type_id . expect_ty () ; let & ty :: Adt (enum_adt_def , _) = enum_type . kind () else { bug ! ("build_enum_type_di_node() called with non-enum type: `{:?}`" , enum_type) } ; let containing_scope = get_namespace_for_item (cx , enum_adt_def . did ()) ; let enum_type_and_layout = cx . layout_of (enum_type) ; let enum_type_name = compute_debuginfo_type_name (cx . tcx , enum_type , false) ; let visibility_flags = visibility_di_flags (cx , enum_adt_def . did () , enum_adt_def . did ()) ; assert ! (! wants_c_like_enum_debuginfo (cx . tcx , enum_type_and_layout)) ; let def_location = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { Some (file_metadata_from_def_id (cx , Some (enum_adt_def . did ()))) } else { None } ; type_map :: build_type_with_children (cx , type_map :: stub (cx , Stub :: Struct , unique_type_id , & enum_type_name , def_location , size_and_align_of (enum_type_and_layout) , Some (containing_scope) , visibility_flags ,) , | cx , enum_type_di_node | { let variant_member_infos : SmallVec < _ > = enum_adt_def . variant_range () . map (| variant_index | VariantMemberInfo { variant_index , variant_name : Cow :: from (enum_adt_def . variant (variant_index) . name . as_str ()) , variant_struct_type_di_node : super :: build_enum_variant_struct_type_di_node (cx , enum_type_and_layout , enum_type_di_node , variant_index , enum_adt_def . variant (variant_index) , enum_type_and_layout . for_variant (cx , variant_index) , visibility_flags ,) , source_info : if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { Some (file_metadata_from_def_id (cx , Some (enum_adt_def . variant (variant_index) . def_id) ,)) } else { None } , }) . collect () ; let enum_adt_def_id = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { Some (enum_adt_def . did ()) } else { None } ; smallvec ! [build_enum_variant_part_di_node (cx , enum_type_and_layout , enum_type_di_node , enum_adt_def_id , & variant_member_infos [..] ,)] } , NO_GENERICS ,) }
}

macro_rules! build_coroutine_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_coroutine_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_coroutine_di_node_introspect!();
    #[doc = " Build the debuginfo node for a coroutine environment. It looks the same as the debuginfo for"] #[doc = " an enum. See [build_enum_type_di_node] for more information."] #[doc = ""] #[doc = " ```txt"] #[doc = ""] #[doc = "  ---> DW_TAG_structure_type              (top-level type for the coroutine)"] #[doc = "         DW_TAG_variant_part              (variant part)"] #[doc = "           DW_AT_discr                    (reference to discriminant DW_TAG_member)"] #[doc = "           DW_TAG_member                  (discriminant member)"] #[doc = "           DW_TAG_variant                 (variant 1)"] #[doc = "           DW_TAG_variant                 (variant 2)"] #[doc = "           DW_TAG_variant                 (variant 3)"] #[doc = "         DW_TAG_structure_type            (type of variant 1)"] #[doc = "         DW_TAG_structure_type            (type of variant 2)"] #[doc = "         DW_TAG_structure_type            (type of variant 3)"] #[doc = ""] #[doc = " ```"] pub (super) fn build_coroutine_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , unique_type_id : UniqueTypeId < 'tcx > ,) -> DINodeCreationResult < 'll > { let coroutine_type = unique_type_id . expect_ty () ; let & ty :: Coroutine (coroutine_def_id , coroutine_args) = coroutine_type . kind () else { bug ! ("build_coroutine_di_node() called with non-coroutine type: `{:?}`" , coroutine_type) } ; let containing_scope = get_namespace_for_item (cx , coroutine_def_id) ; let coroutine_type_and_layout = cx . layout_of (coroutine_type) ; assert ! (! wants_c_like_enum_debuginfo (cx . tcx , coroutine_type_and_layout)) ; let coroutine_type_name = compute_debuginfo_type_name (cx . tcx , coroutine_type , false) ; let def_location = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { Some (file_metadata_from_def_id (cx , Some (coroutine_def_id))) } else { None } ; type_map :: build_type_with_children (cx , type_map :: stub (cx , Stub :: Struct , unique_type_id , & coroutine_type_name , def_location , size_and_align_of (coroutine_type_and_layout) , Some (containing_scope) , DIFlags :: FlagZero ,) , | cx , coroutine_type_di_node | { let coroutine_layout = cx . tcx . coroutine_layout (coroutine_def_id , coroutine_args) . unwrap () ; let Variants :: Multiple { tag_encoding : TagEncoding :: Direct , ref variants , .. } = coroutine_type_and_layout . variants else { bug ! ("Encountered coroutine with non-direct-tag layout: {:?}" , coroutine_type_and_layout) } ; let common_upvar_names = cx . tcx . closure_saved_names_of_captured_variables (coroutine_def_id) ; let variant_struct_type_di_nodes : SmallVec < _ > = variants . indices () . map (| variant_index | { let variant_name = format ! ("{}" , variant_index . as_usize ()) . into () ; let span = coroutine_layout . variant_source_info [variant_index] . span ; let source_info = if ! span . is_dummy () { let loc = cx . lookup_debug_loc (span . lo ()) ; Some ((file_metadata (cx , & loc . file) , loc . line)) } else { None } ; VariantMemberInfo { variant_index , variant_name , variant_struct_type_di_node : super :: build_coroutine_variant_struct_type_di_node (cx , variant_index , coroutine_type_and_layout , coroutine_type_di_node , coroutine_layout , common_upvar_names ,) , source_info , } }) . collect () ; let coroutine_def_id = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { Some (coroutine_def_id) } else { None } ; smallvec ! [build_enum_variant_part_di_node (cx , coroutine_type_and_layout , coroutine_type_di_node , coroutine_def_id , & variant_struct_type_di_nodes [..] ,)] } , NO_GENERICS ,) }
}

macro_rules! build_enum_variant_part_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_enum_variant_part_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_enum_variant_part_di_node_introspect!();
    #[doc = " Builds the DW_TAG_variant_part of an enum or coroutine debuginfo node:"] #[doc = ""] #[doc = " ```txt"] #[doc = "       DW_TAG_structure_type              (top-level type for enum)"] #[doc = " --->    DW_TAG_variant_part              (variant part)"] #[doc = "           DW_AT_discr                    (reference to discriminant DW_TAG_member)"] #[doc = "           DW_TAG_member                  (discriminant member)"] #[doc = "           DW_TAG_variant                 (variant 1)"] #[doc = "           DW_TAG_variant                 (variant 2)"] #[doc = "           DW_TAG_variant                 (variant 3)"] #[doc = "         DW_TAG_structure_type            (type of variant 1)"] #[doc = "         DW_TAG_structure_type            (type of variant 2)"] #[doc = "         DW_TAG_structure_type            (type of variant 3)"] #[doc = " ```"] fn build_enum_variant_part_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , enum_type_and_layout : TyAndLayout < 'tcx > , enum_type_di_node : & 'll DIType , enum_type_def_id : Option < rustc_span :: def_id :: DefId > , variant_member_infos : & [VariantMemberInfo < '_ , 'll >] ,) -> & 'll DIType { let tag_member_di_node = build_discr_member_di_node (cx , enum_type_and_layout , enum_type_di_node) ; let variant_part_unique_type_id = UniqueTypeId :: for_enum_variant_part (cx . tcx , enum_type_and_layout . ty) ; let (file_metadata , line_number) = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { file_metadata_from_def_id (cx , enum_type_def_id) } else { (unknown_file_metadata (cx) , UNKNOWN_LINE_NUMBER) } ; let stub = StubInfo :: new (cx , variant_part_unique_type_id , | cx , variant_part_unique_type_id_str | unsafe { let variant_part_name = "" ; llvm :: LLVMRustDIBuilderCreateVariantPart (DIB (cx) , enum_type_di_node , variant_part_name . as_c_char_ptr () , variant_part_name . len () , file_metadata , line_number , enum_type_and_layout . size . bits () , enum_type_and_layout . align . abi . bits () as u32 , DIFlags :: FlagZero , tag_member_di_node , create_DIArray (DIB (cx) , & []) , variant_part_unique_type_id_str . as_c_char_ptr () , variant_part_unique_type_id_str . len () ,) } ,) ; type_map :: build_type_with_children (cx , stub , | cx , variant_part_di_node | { variant_member_infos . iter () . map (| variant_member_info | { build_enum_variant_member_di_node (cx , enum_type_and_layout , variant_part_di_node , variant_member_info ,) }) . collect () } , NO_GENERICS ,) . di_node }
}

macro_rules! build_discr_member_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_discr_member_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_discr_member_di_node_introspect!();
    #[doc = " Builds the DW_TAG_member describing where we can find the tag of an enum."] #[doc = " Returns `None` if the enum does not have a tag."] #[doc = ""] #[doc = " ```txt"] #[doc = ""] #[doc = "       DW_TAG_structure_type              (top-level type for enum)"] #[doc = "         DW_TAG_variant_part              (variant part)"] #[doc = "           DW_AT_discr                    (reference to discriminant DW_TAG_member)"] #[doc = " --->      DW_TAG_member                  (discriminant member)"] #[doc = "           DW_TAG_variant                 (variant 1)"] #[doc = "           DW_TAG_variant                 (variant 2)"] #[doc = "           DW_TAG_variant                 (variant 3)"] #[doc = "         DW_TAG_structure_type            (type of variant 1)"] #[doc = "         DW_TAG_structure_type            (type of variant 2)"] #[doc = "         DW_TAG_structure_type            (type of variant 3)"] #[doc = ""] #[doc = " ```"] fn build_discr_member_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , enum_or_coroutine_type_and_layout : TyAndLayout < 'tcx > , enum_or_coroutine_type_di_node : & 'll DIType ,) -> Option < & 'll DIType > { let tag_name = match enum_or_coroutine_type_and_layout . ty . kind () { ty :: Coroutine (..) => "__state" , _ => "" , } ; let containing_scope = enum_or_coroutine_type_di_node ; match enum_or_coroutine_type_and_layout . layout . variants () { & Variants :: Single { .. } | & Variants :: Empty => None , & Variants :: Multiple { tag_field , .. } => { let tag_base_type = tag_base_type (cx . tcx , enum_or_coroutine_type_and_layout) ; let ty = type_di_node (cx , tag_base_type) ; let file = unknown_file_metadata (cx) ; let layout = cx . layout_of (tag_base_type) ; Some (create_member_type (cx , containing_scope , & tag_name , file , UNKNOWN_LINE_NUMBER , layout , enum_or_coroutine_type_and_layout . fields . offset (tag_field . as_usize ()) , DIFlags :: FlagArtificial , ty ,)) } } }
}

macro_rules! build_enum_variant_member_di_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_enum_variant_member_di_node in module {}", module_path!());
    };
}

mkfn!{
    build_enum_variant_member_di_node_introspect!();
    #[doc = " Build the debuginfo node for `DW_TAG_variant`:"] #[doc = ""] #[doc = " ```txt"] #[doc = "       DW_TAG_structure_type              (top-level type for enum)"] #[doc = "         DW_TAG_variant_part              (variant part)"] #[doc = "           DW_AT_discr                    (reference to discriminant DW_TAG_member)"] #[doc = "           DW_TAG_member                  (discriminant member)"] #[doc = "  --->     DW_TAG_variant                 (variant 1)"] #[doc = "  --->     DW_TAG_variant                 (variant 2)"] #[doc = "  --->     DW_TAG_variant                 (variant 3)"] #[doc = "         DW_TAG_structure_type            (type of variant 1)"] #[doc = "         DW_TAG_structure_type            (type of variant 2)"] #[doc = "         DW_TAG_structure_type            (type of variant 3)"] #[doc = " ```"] #[doc = ""] #[doc = " This node looks like:"] #[doc = ""] #[doc = " ```txt"] #[doc = " DW_TAG_variant"] #[doc = "   DW_AT_discr_value           0"] #[doc = "   DW_TAG_member"] #[doc = "     DW_AT_name                  None"] #[doc = "     DW_AT_type                  <0x000002a1>"] #[doc = "     DW_AT_alignment             0x00000002"] #[doc = "     DW_AT_data_member_location  0"] #[doc = " ```"] #[doc = ""] #[doc = " The DW_AT_discr_value is optional, and is omitted if"] #[doc = "   - This is the only variant of a univariant enum (i.e. their is no discriminant)"] #[doc = "   - This is the \"untagged\" variant of a niche-layout enum"] #[doc = "     (where only the other variants are identified by a single value)"] #[doc = ""] #[doc = " There is only ever a single member, the type of which is a struct that describes the"] #[doc = " fields of the variant (excluding the discriminant). The name of the member is the name"] #[doc = " of the variant as given in the source code. The DW_AT_data_member_location is always"] #[doc = " zero."] #[doc = ""] #[doc = " Note that the LLVM DIBuilder API is a bit unintuitive here. The DW_TAG_variant subtree"] #[doc = " (including the DW_TAG_member) is built by a single call to"] #[doc = " `LLVMRustDIBuilderCreateVariantMemberType()`."] fn build_enum_variant_member_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , enum_type_and_layout : TyAndLayout < 'tcx > , variant_part_di_node : & 'll DIType , variant_member_info : & VariantMemberInfo < '_ , 'll > ,) -> & 'll DIType { let variant_index = variant_member_info . variant_index ; let discr_value = super :: compute_discriminant_value (cx , enum_type_and_layout , variant_index) ; let (file_di_node , line_number) = variant_member_info . source_info . unwrap_or_else (| | (unknown_file_metadata (cx) , UNKNOWN_LINE_NUMBER)) ; let discr = discr_value . opt_single_val () . map (| value | { let tag_base_type = tag_base_type (cx . tcx , enum_type_and_layout) ; let size = cx . size_of (tag_base_type) ; cx . const_uint_big (cx . type_ix (size . bits ()) , value) }) ; unsafe { llvm :: LLVMRustDIBuilderCreateVariantMemberType (DIB (cx) , variant_part_di_node , variant_member_info . variant_name . as_c_char_ptr () , variant_member_info . variant_name . len () , file_di_node , line_number , enum_type_and_layout . size . bits () , enum_type_and_layout . align . abi . bits () as u32 , Size :: ZERO . bits () , discr , DIFlags :: FlagZero , variant_member_info . variant_struct_type_di_node ,) } }
}
mkitem!{mkstruct!{#[doc = " Information needed for building a `DW_TAG_variant`:"] #[doc = ""] #[doc = " ```txt"] #[doc = "       DW_TAG_structure_type              (top-level type for enum)"] #[doc = "         DW_TAG_variant_part              (variant part)"] #[doc = "           DW_AT_discr                    (reference to discriminant DW_TAG_member)"] #[doc = "           DW_TAG_member                  (discriminant member)"] #[doc = "  --->     DW_TAG_variant                 (variant 1)"] #[doc = "  --->     DW_TAG_variant                 (variant 2)"] #[doc = "  --->     DW_TAG_variant                 (variant 3)"] #[doc = "         DW_TAG_structure_type            (type of variant 1)"] #[doc = "         DW_TAG_structure_type            (type of variant 2)"] #[doc = "         DW_TAG_structure_type            (type of variant 3)"] #[doc = " ```"] struct VariantMemberInfo < 'a , 'll > { variant_index : VariantIdx , variant_name : Cow < 'a , str > , variant_struct_type_di_node : & 'll DIType , source_info : Option < (& 'll DIFile , c_uint) > , }}}