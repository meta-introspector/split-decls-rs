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
mkuse!{use rustc_abi :: { Align , BackendRepr , FieldIdx , FieldsShape , Size , TagEncoding , VariantIdx , Variants , } ;}
mkuse!{use rustc_middle :: mir :: PlaceTy ;}
mkuse!{use rustc_middle :: mir :: interpret :: Scalar ;}
mkuse!{use rustc_middle :: ty :: layout :: { HasTyCtxt , HasTypingEnv , LayoutOf , TyAndLayout } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty } ;}
mkuse!{use rustc_middle :: { bug , mir } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use super :: operand :: OperandValue ;}
mkuse!{use super :: { FunctionCx , LocalRef } ;}
mkuse!{use crate :: common :: IntPredicate ;}
mkuse!{use crate :: size_of_val ;}
mkuse!{use crate :: traits :: * ;}
mkitem!{mkstruct!{# [doc = " The location and extra runtime properties of the place."] # [doc = ""] # [doc = " Typically found in a [`PlaceRef`] or an [`OperandValue::Ref`]."] # [doc = ""] # [doc = " As a location in memory, this has no specific type. If you want to"] # [doc = " load or store it using a typed operation, use [`Self::with_type`]."] # [derive (Copy , Clone , Debug)] pub struct PlaceValue < V > { # [doc = " A pointer to the contents of the place."] pub llval : V , # [doc = " This place's extra data if it is unsized, or `None` if null."] pub llextra : Option < V > , # [doc = " The alignment we know for this place."] pub align : Align , }}}
mkitem!{mkimpl!{impl < V : CodegenObject > PlaceValue < V > { # [doc = " Constructor for the ordinary case of `Sized` types."] # [doc = ""] # [doc = " Sets `llextra` to `None`."] pub fn new_sized (llval : V , align : Align) -> PlaceValue < V > { PlaceValue { llval , llextra : None , align } } # [doc = " Allocates a stack slot in the function for a value"] # [doc = " of the specified size and alignment."] # [doc = ""] # [doc = " The allocation itself is untyped."] pub fn alloca < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx , Value = V > > (bx : & mut Bx , size : Size , align : Align ,) -> PlaceValue < V > { let llval = bx . alloca (size , align) ; PlaceValue :: new_sized (llval , align) } # [doc = " Creates a `PlaceRef` to this location with the given type."] pub fn with_type < 'tcx > (self , layout : TyAndLayout < 'tcx >) -> PlaceRef < 'tcx , V > { assert ! (layout . is_unsized () || layout . is_uninhabited () || self . llextra . is_none () , "Had pointer metadata {:?} for sized type {layout:?}" , self . llextra ,) ; PlaceRef { val : self , layout } } # [doc = " Gets the pointer to this place as an [`OperandValue::Immediate`]"] # [doc = " or, for those needing metadata, an [`OperandValue::Pair`]."] # [doc = ""] # [doc = " This is the inverse of [`OperandValue::deref`]."] pub fn address (self) -> OperandValue < V > { if let Some (llextra) = self . llextra { OperandValue :: Pair (self . llval , llextra) } else { OperandValue :: Immediate (self . llval) } } }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug)] pub struct PlaceRef < 'tcx , V > { # [doc = " The location and extra runtime properties of the place."] pub val : PlaceValue < V > , # [doc = " The monomorphized type of this place, including variant information."] # [doc = ""] # [doc = " You probably shouldn't use the alignment from this layout;"] # [doc = " rather you should use the `.val.align` of the actual place,"] # [doc = " which might be different from the type's normal alignment."] pub layout : TyAndLayout < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx , V : CodegenObject > PlaceRef < 'tcx , V > { pub fn new_sized (llval : V , layout : TyAndLayout < 'tcx >) -> PlaceRef < 'tcx , V > { PlaceRef :: new_sized_aligned (llval , layout , layout . align . abi) } pub fn new_sized_aligned (llval : V , layout : TyAndLayout < 'tcx > , align : Align ,) -> PlaceRef < 'tcx , V > { assert ! (layout . is_sized ()) ; PlaceValue :: new_sized (llval , align) . with_type (layout) } pub fn alloca < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (bx : & mut Bx , layout : TyAndLayout < 'tcx > ,) -> Self { Self :: alloca_size (bx , layout . size , layout) } pub fn alloca_size < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (bx : & mut Bx , size : Size , layout : TyAndLayout < 'tcx > ,) -> Self { assert ! (layout . is_sized () , "tried to statically allocate unsized place") ; PlaceValue :: alloca (bx , size , layout . align . abi) . with_type (layout) } # [doc = " Returns a place for an indirect reference to an unsized place."] pub fn alloca_unsized_indirect < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (bx : & mut Bx , layout : TyAndLayout < 'tcx > ,) -> Self { assert ! (layout . is_unsized () , "tried to allocate indirect place for sized values") ; let ptr_ty = Ty :: new_mut_ptr (bx . cx () . tcx () , layout . ty) ; let ptr_layout = bx . cx () . layout_of (ptr_ty) ; Self :: alloca (bx , ptr_layout) } pub fn len < Cx : ConstCodegenMethods < Value = V > > (& self , cx : & Cx) -> V { if let FieldsShape :: Array { count , .. } = self . layout . fields { if self . layout . is_unsized () { assert_eq ! (count , 0) ; self . val . llextra . unwrap () } else { cx . const_usize (count) } } else { bug ! ("unexpected layout `{:#?}` in PlaceRef::len" , self . layout) } } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx , V : CodegenObject > PlaceRef < 'tcx , V > { # [doc = " Access a field, at a point when the value's case is known."] pub fn project_field < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (self , bx : & mut Bx , ix : usize ,) -> Self { let field = self . layout . field (bx . cx () , ix) ; let offset = self . layout . fields . offset (ix) ; let effective_field_align = self . val . align . restrict_for_offset (offset) ; let mut simple = | | { let llval = if offset . bytes () == 0 { self . val . llval } else { bx . inbounds_ptradd (self . val . llval , bx . const_usize (offset . bytes ())) } ; let val = PlaceValue { llval , llextra : if bx . cx () . tcx () . type_has_metadata (field . ty , bx . cx () . typing_env ()) { self . val . llextra } else { None } , align : effective_field_align , } ; val . with_type (field) } ; match field . ty . kind () { _ if field . is_sized () => return simple () , ty :: Slice (..) | ty :: Str => return simple () , _ if offset . bytes () == 0 => return simple () , _ => { } } let meta = self . val . llextra ; let unaligned_offset = bx . cx () . const_usize (offset . bytes ()) ; let (_ , mut unsized_align) = size_of_val :: size_and_align_of_dst (bx , field . ty , meta) ; if let ty :: Adt (def , _) = self . layout . ty . kind () && let Some (packed) = def . repr () . pack { let packed = bx . const_usize (packed . bytes ()) ; let cmp = bx . icmp (IntPredicate :: IntULT , unsized_align , packed) ; unsized_align = bx . select (cmp , unsized_align , packed) } let offset = round_up_const_value_to_alignment (bx , unaligned_offset , unsized_align) ; debug ! ("struct_field_ptr: DST field offset: {:?}" , offset) ; let ptr = bx . inbounds_ptradd (self . val . llval , offset) ; let val = PlaceValue { llval : ptr , llextra : self . val . llextra , align : effective_field_align } ; val . with_type (field) } # [doc = " Sets the discriminant for a new value of the given case of the given"] # [doc = " representation."] pub fn codegen_set_discr < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (& self , bx : & mut Bx , variant_index : VariantIdx ,) { match codegen_tag_value (bx . cx () , variant_index , self . layout) { Err (UninhabitedVariantError) => { bx . abort () ; } Ok (Some ((tag_field , imm))) => { let tag_place = self . project_field (bx , tag_field . as_usize ()) ; OperandValue :: Immediate (imm) . store (bx , tag_place) ; } Ok (None) => { } } } pub fn project_index < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (& self , bx : & mut Bx , llindex : V ,) -> Self { let layout = self . layout . field (bx , 0) ; let offset = if let Some (llindex) = bx . const_to_opt_uint (llindex) { layout . size . checked_mul (llindex , bx) . unwrap_or (layout . size) } else { layout . size } ; let llval = bx . inbounds_nuw_gep (bx . cx () . backend_type (layout) , self . val . llval , & [llindex]) ; let align = self . val . align . restrict_for_offset (offset) ; PlaceValue :: new_sized (llval , align) . with_type (layout) } pub fn project_downcast < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (& self , bx : & mut Bx , variant_index : VariantIdx ,) -> Self { let mut downcast = * self ; downcast . layout = self . layout . for_variant (bx . cx () , variant_index) ; downcast } pub fn project_type < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (& self , bx : & mut Bx , ty : Ty < 'tcx > ,) -> Self { let mut downcast = * self ; downcast . layout = bx . cx () . layout_of (ty) ; downcast } pub fn storage_live < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (& self , bx : & mut Bx) { bx . lifetime_start (self . val . llval , self . layout . size) ; } pub fn storage_dead < Bx : BuilderMethods < 'a , 'tcx , Value = V > > (& self , bx : & mut Bx) { bx . lifetime_end (self . val . llval , self . layout . size) ; } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > FunctionCx < 'a , 'tcx , Bx > { # [instrument (level = "trace" , skip (self , bx))] pub fn codegen_place (& mut self , bx : & mut Bx , place_ref : mir :: PlaceRef < 'tcx > ,) -> PlaceRef < 'tcx , Bx :: Value > { let cx = self . cx ; let tcx = self . cx . tcx () ; let mut base = 0 ; let mut cg_base = match self . locals [place_ref . local] { LocalRef :: Place (place) => place , LocalRef :: UnsizedPlace (place) => bx . load_operand (place) . deref (cx) , LocalRef :: Operand (..) => { if place_ref . is_indirect_first_projection () { base = 1 ; let cg_base = self . codegen_consume (bx , mir :: PlaceRef { projection : & place_ref . projection [.. 0] , .. place_ref } ,) ; cg_base . deref (bx . cx ()) } else { bug ! ("using operand local {:?} as place" , place_ref) ; } } LocalRef :: PendingOperand => { bug ! ("using still-pending operand local {:?} as place" , place_ref) ; } } ; for elem in place_ref . projection [base ..] . iter () { cg_base = match * elem { mir :: ProjectionElem :: Deref => bx . load_operand (cg_base) . deref (bx . cx ()) , mir :: ProjectionElem :: Field (ref field , _) => { assert ! (! cg_base . layout . ty . is_any_ptr () , "Bad PlaceRef: destructing pointers should use cast/PtrMetadata, \
                         but tried to access field {field:?} of pointer {cg_base:?}" ,) ; cg_base . project_field (bx , field . index ()) } mir :: ProjectionElem :: OpaqueCast (ty) => { bug ! ("encountered OpaqueCast({ty}) in codegen") } mir :: ProjectionElem :: Subtype (ty) => cg_base . project_type (bx , self . monomorphize (ty)) , mir :: ProjectionElem :: UnwrapUnsafeBinder (ty) => { cg_base . project_type (bx , self . monomorphize (ty)) } mir :: ProjectionElem :: Index (index) => { let index = & mir :: Operand :: Copy (mir :: Place :: from (index)) ; let index = self . codegen_operand (bx , index) ; let llindex = index . immediate () ; cg_base . project_index (bx , llindex) } mir :: ProjectionElem :: ConstantIndex { offset , from_end : false , min_length : _ } => { let lloffset = bx . cx () . const_usize (offset) ; cg_base . project_index (bx , lloffset) } mir :: ProjectionElem :: ConstantIndex { offset , from_end : true , min_length : _ } => { let lloffset = bx . cx () . const_usize (offset) ; let lllen = cg_base . len (bx . cx ()) ; let llindex = bx . sub (lllen , lloffset) ; cg_base . project_index (bx , llindex) } mir :: ProjectionElem :: Subslice { from , to , from_end } => { let mut subslice = cg_base . project_index (bx , bx . cx () . const_usize (from)) ; let projected_ty = PlaceTy :: from_ty (cg_base . layout . ty) . projection_ty (tcx , * elem) . ty ; subslice . layout = bx . cx () . layout_of (self . monomorphize (projected_ty)) ; if subslice . layout . is_unsized () { assert ! (from_end , "slice subslices should be `from_end`") ; subslice . val . llextra = Some (bx . sub (cg_base . val . llextra . unwrap () , bx . cx () . const_usize (from + to)) ,) ; } subslice } mir :: ProjectionElem :: Downcast (_ , v) => cg_base . project_downcast (bx , v) , } ; } debug ! ("codegen_place(place={:?}) => {:?}" , place_ref , cg_base) ; cg_base } pub fn monomorphized_place_ty (& self , place_ref : mir :: PlaceRef < 'tcx >) -> Ty < 'tcx > { let tcx = self . cx . tcx () ; let place_ty = place_ref . ty (self . mir , tcx) ; self . monomorphize (place_ty . ty) } }}}

macro_rules! round_up_const_value_to_alignment_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function round_up_const_value_to_alignment in module {}", module_path!());
    };
}

mkfn!{
    round_up_const_value_to_alignment_introspect!();
    fn round_up_const_value_to_alignment < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx , value : Bx :: Value , align : Bx :: Value ,) -> Bx :: Value { let one = bx . const_usize (1) ; let align_minus_1 = bx . sub (align , one) ; let neg_value = bx . neg (value) ; let offset = bx . and (neg_value , align_minus_1) ; bx . add (value , offset) }
}

macro_rules! codegen_tag_value_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function codegen_tag_value in module {}", module_path!());
    };
}

mkfn!{
    codegen_tag_value_introspect!();
    # [doc = " Calculates the value that needs to be stored to mark the discriminant."] # [doc = ""] # [doc = " This might be `None` for a `struct` or a niched variant (like `Some(&3)`)."] # [doc = ""] # [doc = " If it's `Some`, it returns the value to store and the field in which to"] # [doc = " store it. Note that this value is *not* the same as the discriminant, in"] # [doc = " general, as it might be a niche value or have a different size."] # [doc = ""] # [doc = " It might also be an `Err` because the variant is uninhabited."] pub (super) fn codegen_tag_value < 'tcx , V > (cx : & impl CodegenMethods < 'tcx , Value = V > , variant_index : VariantIdx , layout : TyAndLayout < 'tcx > ,) -> Result < Option < (FieldIdx , V) > , UninhabitedVariantError > { if layout . for_variant (cx , variant_index) . is_uninhabited () { return Err (UninhabitedVariantError) ; } Ok (match layout . variants { Variants :: Empty => unreachable ! ("we already handled uninhabited types") , Variants :: Single { index } => { assert_eq ! (index , variant_index) ; None } Variants :: Multiple { tag_encoding : TagEncoding :: Direct , tag_field , .. } => { let discr = layout . ty . discriminant_for_variant (cx . tcx () , variant_index) ; let to = discr . unwrap () . val ; let tag_layout = layout . field (cx , tag_field . as_usize ()) ; let tag_llty = cx . immediate_backend_type (tag_layout) ; let imm = cx . const_uint_big (tag_llty , to) ; Some ((tag_field , imm)) } Variants :: Multiple { tag_encoding : TagEncoding :: Niche { untagged_variant , ref niche_variants , niche_start } , tag_field , .. } => { if variant_index != untagged_variant { let niche_layout = layout . field (cx , tag_field . as_usize ()) ; let niche_llty = cx . immediate_backend_type (niche_layout) ; let BackendRepr :: Scalar (scalar) = niche_layout . backend_repr else { bug ! ("expected a scalar placeref for the niche") ; } ; let niche_value = variant_index . as_u32 () - niche_variants . start () . as_u32 () ; let niche_value = (niche_value as u128) . wrapping_add (niche_start) ; let niche_value = niche_value & niche_layout . size . unsigned_int_max () ; let niche_llval = cx . scalar_to_backend (Scalar :: from_uint (niche_value , niche_layout . size) , scalar , niche_llty ,) ; Some ((tag_field , niche_llval)) } else { None } } }) }
}
mkitem!{mkstruct!{# [derive (Debug)] pub (super) struct UninhabitedVariantError ;}}