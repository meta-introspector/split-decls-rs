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
mkuse!{# [cfg (feature = "master")] use std :: convert :: TryInto ;}
mkuse!{# [cfg (feature = "master")] use gccjit :: CType ;}
mkuse!{use gccjit :: { RValue , Struct , Type } ;}
mkuse!{use rustc_abi :: { AddressSpace , Align , Integer , Size } ;}
mkuse!{use rustc_codegen_ssa :: common :: TypeKind ;}
mkuse!{use rustc_codegen_ssa :: traits :: { BaseTypeCodegenMethods , DerivedTypeCodegenMethods , TypeMembershipCodegenMethods , } ;}
mkuse!{use rustc_middle :: ty :: layout :: TyAndLayout ;}
mkuse!{use rustc_middle :: { bug , ty } ;}
mkuse!{use crate :: common :: TypeReflection ;}
mkuse!{use crate :: context :: CodegenCx ;}
mkuse!{use crate :: type_of :: LayoutGccExt ;}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > CodegenCx < 'gcc , 'tcx > { pub fn type_ix (& self , num_bits : u64) -> Type < 'gcc > { let bytes = (num_bits / 8) . next_power_of_two () as i32 ; match bytes { 1 => self . i8_type , 2 => self . i16_type , 4 => self . i32_type , 8 => self . i64_type , 16 => self . i128_type , _ => panic ! ("unexpected num_bits: {}" , num_bits) , } } pub fn type_void (& self) -> Type < 'gcc > { self . context . new_type :: < () > () } pub fn type_size_t (& self) -> Type < 'gcc > { self . context . new_type :: < usize > () } pub fn type_u8 (& self) -> Type < 'gcc > { self . u8_type } pub fn type_u16 (& self) -> Type < 'gcc > { self . u16_type } pub fn type_u32 (& self) -> Type < 'gcc > { self . u32_type } pub fn type_u64 (& self) -> Type < 'gcc > { self . u64_type } pub fn type_u128 (& self) -> Type < 'gcc > { self . u128_type } pub fn type_ptr_to (& self , ty : Type < 'gcc >) -> Type < 'gcc > { ty . make_pointer () } pub fn type_ptr_to_ext (& self , ty : Type < 'gcc > , _address_space : AddressSpace) -> Type < 'gcc > { ty . make_pointer () } pub fn type_i8p (& self) -> Type < 'gcc > { self . type_ptr_to (self . type_i8 ()) } pub fn type_i8p_ext (& self , address_space : AddressSpace) -> Type < 'gcc > { self . type_ptr_to_ext (self . type_i8 () , address_space) } pub fn type_pointee_for_align (& self , align : Align) -> Type < 'gcc > { let ity = Integer :: approximate_align (self , align) ; self . type_from_integer (ity) } pub fn type_vector (& self , ty : Type < 'gcc > , len : u64) -> Type < 'gcc > { self . context . new_vector_type (ty , len) } pub fn type_float_from_ty (& self , t : ty :: FloatTy) -> Type < 'gcc > { match t { ty :: FloatTy :: F16 => self . type_f16 () , ty :: FloatTy :: F32 => self . type_f32 () , ty :: FloatTy :: F64 => self . type_f64 () , ty :: FloatTy :: F128 => self . type_f128 () , } } pub fn type_i1 (& self) -> Type < 'gcc > { self . bool_type } pub fn type_struct (& self , fields : & [Type < 'gcc >] , packed : bool) -> Type < 'gcc > { let types = fields . to_vec () ; if let Some (typ) = self . struct_types . borrow () . get (fields) { return * typ ; } let fields : Vec < _ > = fields . iter () . enumerate () . map (| (index , field) | { self . context . new_field (None , * field , format ! ("field{}_TODO" , index)) }) . collect () ; let typ = self . context . new_struct_type (None , "struct" , & fields) . as_type () ; if packed { # [cfg (feature = "master")] typ . set_packed () ; } self . struct_types . borrow_mut () . insert (types , typ) ; typ } }}}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > BaseTypeCodegenMethods for CodegenCx < 'gcc , 'tcx > { fn type_i8 (& self) -> Type < 'gcc > { self . i8_type } fn type_i16 (& self) -> Type < 'gcc > { self . i16_type } fn type_i32 (& self) -> Type < 'gcc > { self . i32_type } fn type_i64 (& self) -> Type < 'gcc > { self . i64_type } fn type_i128 (& self) -> Type < 'gcc > { self . i128_type } fn type_isize (& self) -> Type < 'gcc > { self . isize_type } fn type_f16 (& self) -> Type < 'gcc > { # [cfg (feature = "master")] if self . supports_f16_type { return self . context . new_c_type (CType :: Float16) ; } bug ! ("unsupported float width 16") } fn type_f32 (& self) -> Type < 'gcc > { # [cfg (feature = "master")] if self . supports_f32_type { return self . context . new_c_type (CType :: Float32) ; } self . float_type } fn type_f64 (& self) -> Type < 'gcc > { # [cfg (feature = "master")] if self . supports_f64_type { return self . context . new_c_type (CType :: Float64) ; } self . double_type } fn type_f128 (& self) -> Type < 'gcc > { # [cfg (feature = "master")] if self . supports_f128_type { return self . context . new_c_type (CType :: Float128) ; } bug ! ("unsupported float width 128") } fn type_func (& self , params : & [Type < 'gcc >] , return_type : Type < 'gcc >) -> Type < 'gcc > { self . context . new_function_pointer_type (None , return_type , params , false) } # [cfg (feature = "master")] fn type_kind (& self , typ : Type < 'gcc >) -> TypeKind { if self . is_int_type_or_bool (typ) { TypeKind :: Integer } else if typ . get_pointee () . is_some () { TypeKind :: Pointer } else if typ . is_vector () { TypeKind :: Vector } else if typ . dyncast_array () . is_some () { TypeKind :: Array } else if typ . is_struct () . is_some () { TypeKind :: Struct } else if typ . dyncast_function_ptr_type () . is_some () { TypeKind :: Function } else if typ . is_compatible_with (self . float_type) { TypeKind :: Float } else if typ . is_compatible_with (self . double_type) { TypeKind :: Double } else if typ . is_floating_point () { match typ . get_size () { 2 => TypeKind :: Half , 4 => TypeKind :: Float , 8 => TypeKind :: Double , 16 => TypeKind :: FP128 , size => unreachable ! ("Floating-point type of size {}" , size) , } } else if typ == self . type_void () { TypeKind :: Void } else { unimplemented ! () ; } } # [cfg (not (feature = "master"))] fn type_kind (& self , typ : Type < 'gcc >) -> TypeKind { if self . is_int_type_or_bool (typ) { TypeKind :: Integer } else if typ . is_compatible_with (self . float_type) { TypeKind :: Float } else if typ . is_compatible_with (self . double_type) { TypeKind :: Double } else if typ . is_vector () { TypeKind :: Vector } else if typ . get_pointee () . is_some () { TypeKind :: Pointer } else if typ . dyncast_array () . is_some () { TypeKind :: Array } else if typ . is_struct () . is_some () { TypeKind :: Struct } else if typ . dyncast_function_ptr_type () . is_some () { TypeKind :: Function } else if typ == self . type_void () { TypeKind :: Void } else { unimplemented ! () ; } } fn type_ptr (& self) -> Type < 'gcc > { self . type_ptr_to (self . type_void ()) } fn type_ptr_ext (& self , address_space : AddressSpace) -> Type < 'gcc > { self . type_ptr_to_ext (self . type_void () , address_space) } fn element_type (& self , ty : Type < 'gcc >) -> Type < 'gcc > { if let Some (typ) = ty . dyncast_array () { typ } else if let Some (vector_type) = ty . dyncast_vector () { vector_type . get_element_type () } else if let Some (typ) = ty . get_pointee () { typ } else { unreachable ! () } } fn vector_length (& self , _ty : Type < 'gcc >) -> usize { unimplemented ! () ; } # [cfg (feature = "master")] fn float_width (& self , typ : Type < 'gcc >) -> usize { if typ . is_floating_point () { (typ . get_size () * u8 :: BITS) . try_into () . unwrap () } else { panic ! ("Cannot get width of float type {:?}" , typ) ; } } # [cfg (not (feature = "master"))] fn float_width (& self , typ : Type < 'gcc >) -> usize { let f32 = self . context . new_type :: < f32 > () ; let f64 = self . context . new_type :: < f64 > () ; if typ . is_compatible_with (f32) { 32 } else if typ . is_compatible_with (f64) { 64 } else { panic ! ("Cannot get width of float type {:?}" , typ) ; } } fn int_width (& self , typ : Type < 'gcc >) -> u64 { self . gcc_int_width (typ) } fn val_ty (& self , value : RValue < 'gcc >) -> Type < 'gcc > { value . get_type () } # [cfg_attr (feature = "master" , allow (unused_mut))] fn type_array (& self , ty : Type < 'gcc > , mut len : u64) -> Type < 'gcc > { # [cfg (not (feature = "master"))] if let Some (struct_type) = ty . is_struct () && struct_type . get_field_count () == 0 { len = 0 ; } self . context . new_array_type (None , ty , len) } }}}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > CodegenCx < 'gcc , 'tcx > { pub fn type_padding_filler (& self , size : Size , align : Align) -> Type < 'gcc > { let unit = Integer :: approximate_align (self , align) ; let size = size . bytes () ; let unit_size = unit . size () . bytes () ; assert_eq ! (size % unit_size , 0) ; self . type_array (self . type_from_integer (unit) , size / unit_size) } pub fn set_struct_body (& self , typ : Struct < 'gcc > , fields : & [Type < 'gcc >] , packed : bool) { let fields : Vec < _ > = fields . iter () . enumerate () . map (| (index , field) | self . context . new_field (None , * field , format ! ("field_{}" , index))) . collect () ; typ . set_fields (None , & fields) ; if packed { # [cfg (feature = "master")] typ . as_type () . set_packed () ; } } pub fn type_named_struct (& self , name : & str) -> Struct < 'gcc > { self . context . new_opaque_struct_type (None , name) } }}}

macro_rules! struct_fields_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function struct_fields in module {}", module_path!());
    };
}

mkfn!{
    struct_fields_introspect!();
    pub fn struct_fields < 'gcc , 'tcx > (cx : & CodegenCx < 'gcc , 'tcx > , layout : TyAndLayout < 'tcx > ,) -> (Vec < Type < 'gcc > > , bool) { let field_count = layout . fields . count () ; let mut packed = false ; let mut offset = Size :: ZERO ; let mut prev_effective_align = layout . align . abi ; let mut result : Vec < _ > = Vec :: with_capacity (1 + field_count * 2) ; for i in layout . fields . index_by_increasing_offset () { let target_offset = layout . fields . offset (i) ; let field = layout . field (cx , i) ; let effective_field_align = layout . align . abi . min (field . align . abi) . restrict_for_offset (target_offset) ; packed |= effective_field_align < field . align . abi ; assert ! (target_offset >= offset) ; let padding = target_offset - offset ; let padding_align = prev_effective_align . min (effective_field_align) ; assert_eq ! (offset . align_to (padding_align) + padding , target_offset) ; result . push (cx . type_padding_filler (padding , padding_align)) ; result . push (field . gcc_type (cx)) ; offset = target_offset + field . size ; prev_effective_align = effective_field_align ; } if layout . is_sized () && field_count > 0 { if offset > layout . size { bug ! ("layout: {:#?} stride: {:?} offset: {:?}" , layout , layout . size , offset) ; } let padding = layout . size - offset ; let padding_align = prev_effective_align ; assert_eq ! (offset . align_to (padding_align) + padding , layout . size) ; result . push (cx . type_padding_filler (padding , padding_align)) ; assert_eq ! (result . len () , 1 + field_count * 2) ; } (result , packed) }
}
mkitem!{mkimpl!{impl < 'gcc , 'tcx > TypeMembershipCodegenMethods < 'tcx > for CodegenCx < 'gcc , 'tcx > { }}}