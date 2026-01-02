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
mkuse!{use itertools :: Itertools ;}
mkuse!{use serde :: { Deserialize , Deserializer , Serialize , de } ;}
mkuse!{use crate :: { context :: { self , GlobalContext } , intrinsic :: Intrinsic , predicate_forms :: { PredicateForm , PredicationMask , PredicationMethods } , typekinds :: TypeKind , wildstring :: WildString , } ;}
mkitem!{mkenum!{#[derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] #[serde (untagged)] pub enum InputType { #[doc = " PredicateForm variant argument"] #[serde (skip)] PredicateForm (PredicateForm) , #[doc = " Operand from which to generate an N variant"] #[serde (skip)] NVariantOp (Option < WildString >) , #[doc = " TypeKind variant argument"] Type (TypeKind) , }}}
mkitem!{mkimpl!{impl InputType { #[doc = " Optionally unwraps as a PredicateForm."] pub fn predicate_form (& self) -> Option < & PredicateForm > { match self { InputType :: PredicateForm (pf) => Some (pf) , _ => None , } } #[doc = " Optionally unwraps as a mutable PredicateForm"] pub fn predicate_form_mut (& mut self) -> Option < & mut PredicateForm > { match self { InputType :: PredicateForm (pf) => Some (pf) , _ => None , } } #[doc = " Optionally unwraps as a TypeKind."] pub fn typekind (& self) -> Option < & TypeKind > { match self { InputType :: Type (ty) => Some (ty) , _ => None , } } #[doc = " Optionally unwraps as a NVariantOp"] pub fn n_variant_op (& self) -> Option < & WildString > { match self { InputType :: NVariantOp (Some (op)) => Some (op) , _ => None , } } }}}
mkitem!{mkimpl!{impl PartialOrd for InputType { fn partial_cmp (& self , other : & Self) -> Option < std :: cmp :: Ordering > { Some (self . cmp (other)) } }}}
mkitem!{mkimpl!{impl Ord for InputType { fn cmp (& self , other : & Self) -> std :: cmp :: Ordering { use std :: cmp :: Ordering :: * ; match (self , other) { (InputType :: PredicateForm (pf1) , InputType :: PredicateForm (pf2)) => pf1 . cmp (pf2) , (InputType :: Type (ty1) , InputType :: Type (ty2)) => ty1 . cmp (ty2) , (InputType :: NVariantOp (None) , InputType :: NVariantOp (Some (..))) => Less , (InputType :: NVariantOp (Some (..)) , InputType :: NVariantOp (None)) => Greater , (InputType :: NVariantOp (_) , InputType :: NVariantOp (_)) => Equal , (InputType :: Type (..) , InputType :: PredicateForm (..)) => Less , (InputType :: PredicateForm (..) , InputType :: Type (..)) => Greater , (InputType :: Type (..) , InputType :: NVariantOp (..)) => Less , (InputType :: NVariantOp (..) , InputType :: Type (..)) => Greater , (InputType :: PredicateForm (..) , InputType :: NVariantOp (..)) => Less , (InputType :: NVariantOp (..) , InputType :: PredicateForm (..)) => Greater , } } }}}
mkmod!{many_or_one, { 
                getname!(many_or_one);
                getsrc!(many_or_one);
                getpath!(many_or_one);
                get_deps!(many_or_one);
                get_crates!(many_or_one);
                mkinclude!(many_or_one);
                mkuse!{use serde :: { Deserialize , Serialize , de :: Deserializer , ser :: Serializer } ;}

macro_rules! serialize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function serialize in module {}", module_path!());
    };
}

mkfn!{
    serialize_introspect!();
    pub fn serialize < T , S > (vec : & Vec < T > , serializer : S) -> Result < S :: Ok , S :: Error > where T : Serialize , S : Serializer , { if vec . len () == 1 { vec . first () . unwrap () . serialize (serializer) } else { vec . serialize (serializer) } }
}

macro_rules! deserialize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function deserialize in module {}", module_path!());
    };
}

mkfn!{
    deserialize_introspect!();
    pub fn deserialize < 'de , T , D > (deserializer : D) -> Result < Vec < T > , D :: Error > where T : Deserialize < 'de > , D : Deserializer < 'de > , { #[derive (Debug , Clone , Serialize , Deserialize)] #[serde (untagged)] enum ManyOrOne < T > { Many (Vec < T >) , One (T) , } match ManyOrOne :: deserialize (deserializer) ? { ManyOrOne :: Many (vec) => Ok (vec) , ManyOrOne :: One (val) => Ok (vec ! [val]) , } }
} 
            }}
mkitem!{mkstruct!{#[derive (Debug , Clone , Default , PartialEq , Eq , PartialOrd , Ord , Serialize , Deserialize)] pub struct InputSet (#[serde (with = "many_or_one")] Vec < InputType >) ;}}
mkitem!{mkimpl!{impl InputSet { pub fn get (& self , idx : usize) -> Option < & InputType > { self . 0 . get (idx) } pub fn is_empty (& self) -> bool { self . 0 . is_empty () } pub fn iter (& self) -> impl Iterator < Item = & InputType > + '_ { self . 0 . iter () } pub fn iter_mut (& mut self) -> impl Iterator < Item = & mut InputType > + '_ { self . 0 . iter_mut () } pub fn into_iter (self) -> impl Iterator < Item = InputType > + Clone { self . 0 . into_iter () } pub fn types_len (& self) -> usize { self . iter () . filter_map (| arg | arg . typekind ()) . count () } pub fn typekind (& self , idx : Option < usize >) -> Option < TypeKind > { let types_len = self . types_len () ; self . get (idx . unwrap_or (0)) . and_then (move | arg : & InputType | { if (idx . is_none () && types_len != 1) || (idx . is_some () && types_len == 1) { None } else { arg . typekind () . cloned () } }) } }}}
mkitem!{mkstruct!{#[derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] pub struct InputSetEntry (#[serde (with = "many_or_one")] Vec < InputSet >) ;}}
mkitem!{mkimpl!{impl InputSetEntry { pub fn new (input : Vec < InputSet >) -> Self { Self (input) } pub fn get (& self , idx : usize) -> Option < & InputSet > { self . 0 . get (idx) } }}}

macro_rules! validate_types_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function validate_types in module {}", module_path!());
    };
}

mkfn!{
    validate_types_introspect!();
    fn validate_types < 'de , D > (deserializer : D) -> Result < Vec < InputSetEntry > , D :: Error > where D : Deserializer < 'de > , { let v : Vec < InputSetEntry > = Vec :: deserialize (deserializer) ? ; let mut it = v . iter () ; if let Some (first) = it . next () { it . try_fold (first , | last , cur | { if last . 0 . len () == cur . 0 . len () { Ok (cur) } else { Err ("the length of the InputSets and the product lists must match" . to_string ()) } }) . map_err (de :: Error :: custom) ? ; } Ok (v) }
}
mkitem!{mkstruct!{#[derive (Debug , Clone , Default , Serialize , Deserialize)] pub struct IntrinsicInput { #[serde (default)] #[serde (deserialize_with = "validate_types")] pub types : Vec < InputSetEntry > , #[serde (flatten)] pub predication_methods : PredicationMethods , #[doc = " Generates a _n variant where the specified operand is a primitive type"] #[doc = " that requires conversion to an SVE one. The `{_n}` wildcard is required"] #[doc = " in the intrinsic's name, otherwise an error will be thrown."] #[serde (default)] pub n_variant_op : WildString , }}}
mkitem!{mkimpl!{impl IntrinsicInput { #[doc = " Extracts all the possible variants as an iterator."] pub fn variants (& self , intrinsic : & Intrinsic ,) -> context :: Result < impl Iterator < Item = InputSet > + '_ > { let mut top_product = vec ! [] ; if ! self . types . is_empty () { top_product . push (self . types . iter () . flat_map (| ty_in | { ty_in . 0 . iter () . map (| v | v . clone () . into_iter ()) . multi_cartesian_product () }) . collect_vec () ,) } if let Ok (mask) = PredicationMask :: try_from (& intrinsic . signature . name) { top_product . push (PredicateForm :: compile_list (& mask , & self . predication_methods) ? . into_iter () . map (| pf | vec ! [InputType :: PredicateForm (pf)]) . collect_vec () ,) } if ! self . n_variant_op . is_empty () { top_product . push (vec ! [vec ! [InputType :: NVariantOp (None)] , vec ! [InputType :: NVariantOp (Some (self . n_variant_op . to_owned ()))] ,]) } let it = top_product . into_iter () . map (| v | v . into_iter ()) . multi_cartesian_product () . filter (| set | ! set . is_empty ()) . map (| set | InputSet (set . into_iter () . flatten () . collect_vec ())) ; Ok (it) } }}}
mkitem!{mkstruct!{#[derive (Debug , Clone , Serialize , Deserialize)] pub struct GeneratorInput { #[serde (flatten)] pub ctx : GlobalContext , pub intrinsics : Vec < Intrinsic > , }}}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: { input :: * , predicate_forms :: { DontCareMethod , ZeroingMethod } , } ;}

macro_rules! test_empty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_empty in module {}", module_path!());
    };
}

mkfn!{
    test_empty_introspect!();
    #[test] fn test_empty () { let str = r#"types: []"# ; let input : IntrinsicInput = serde_yaml :: from_str (str) . expect ("failed to parse") ; let mut variants = input . variants (& Intrinsic :: default ()) . unwrap () . into_iter () ; assert_eq ! (variants . next () , None) ; }
}

macro_rules! test_product_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_product in module {}", module_path!());
    };
}

mkfn!{
    test_product_introspect!();
    #[test] fn test_product () { let str = r#"types:
- [f64, f32]
- [i64, [f64, f32]]
"# ; let input : IntrinsicInput = serde_yaml :: from_str (str) . expect ("failed to parse") ; let mut intrinsic = Intrinsic :: default () ; intrinsic . signature . name = "test_intrinsic{_mx}" . parse () . unwrap () ; let mut variants = input . variants (& intrinsic) . unwrap () . into_iter () ; assert_eq ! (variants . next () , Some (InputSet (vec ! [InputType :: Type ("f64" . parse () . unwrap ()) , InputType :: Type ("f32" . parse () . unwrap ()) , InputType :: PredicateForm (PredicateForm :: Merging) ,]))) ; assert_eq ! (variants . next () , Some (InputSet (vec ! [InputType :: Type ("f64" . parse () . unwrap ()) , InputType :: Type ("f32" . parse () . unwrap ()) , InputType :: PredicateForm (PredicateForm :: DontCare (DontCareMethod :: AsMerging)) ,]))) ; assert_eq ! (variants . next () , Some (InputSet (vec ! [InputType :: Type ("i64" . parse () . unwrap ()) , InputType :: Type ("f64" . parse () . unwrap ()) , InputType :: PredicateForm (PredicateForm :: Merging) ,]))) ; assert_eq ! (variants . next () , Some (InputSet (vec ! [InputType :: Type ("i64" . parse () . unwrap ()) , InputType :: Type ("f64" . parse () . unwrap ()) , InputType :: PredicateForm (PredicateForm :: DontCare (DontCareMethod :: AsMerging)) ,]))) ; assert_eq ! (variants . next () , Some (InputSet (vec ! [InputType :: Type ("i64" . parse () . unwrap ()) , InputType :: Type ("f32" . parse () . unwrap ()) , InputType :: PredicateForm (PredicateForm :: Merging) ,]))) ; assert_eq ! (variants . next () , Some (InputSet (vec ! [InputType :: Type ("i64" . parse () . unwrap ()) , InputType :: Type ("f32" . parse () . unwrap ()) , InputType :: PredicateForm (PredicateForm :: DontCare (DontCareMethod :: AsMerging)) ,])) ,) ; assert_eq ! (variants . next () , None) ; }
}

macro_rules! test_n_variant_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_n_variant in module {}", module_path!());
    };
}

mkfn!{
    test_n_variant_introspect!();
    #[test] fn test_n_variant () { let str = r#"types:
- [f64, f32]
n_variant_op: op2
"# ; let input : IntrinsicInput = serde_yaml :: from_str (str) . expect ("failed to parse") ; let mut variants = input . variants (& Intrinsic :: default ()) . unwrap () . into_iter () ; assert_eq ! (variants . next () , Some (InputSet (vec ! [InputType :: Type ("f64" . parse () . unwrap ()) , InputType :: Type ("f32" . parse () . unwrap ()) , InputType :: NVariantOp (None) ,]))) ; assert_eq ! (variants . next () , Some (InputSet (vec ! [InputType :: Type ("f64" . parse () . unwrap ()) , InputType :: Type ("f32" . parse () . unwrap ()) , InputType :: NVariantOp (Some ("op2" . parse () . unwrap ())) ,]))) ; assert_eq ! (variants . next () , None) }
}

macro_rules! test_invalid_length_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_invalid_length in module {}", module_path!());
    };
}

mkfn!{
    test_invalid_length_introspect!();
    #[test] fn test_invalid_length () { let str = r#"types: [i32, [[u64], [u32]]]"# ; serde_yaml :: from_str :: < IntrinsicInput > (str) . expect_err ("failure expected") ; }
}

macro_rules! test_invalid_predication_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_invalid_predication in module {}", module_path!());
    };
}

mkfn!{
    test_invalid_predication_introspect!();
    #[test] fn test_invalid_predication () { let str = "types: []" ; let input : IntrinsicInput = serde_yaml :: from_str (str) . expect ("failed to parse") ; let mut intrinsic = Intrinsic :: default () ; intrinsic . signature . name = "test_intrinsic{_mxz}" . parse () . unwrap () ; input . variants (& intrinsic) . map (| v | v . collect_vec ()) . expect_err ("failure expected") ; }
}

macro_rules! test_invalid_predication_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_invalid_predication_mask in module {}", module_path!());
    };
}

mkfn!{
    test_invalid_predication_mask_introspect!();
    #[test] fn test_invalid_predication_mask () { "test_intrinsic{_mxy}" . parse :: < WildString > () . expect_err ("failure expected") ; "test_intrinsic{_}" . parse :: < WildString > () . expect_err ("failure expected") ; }
}

macro_rules! test_zeroing_predication_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_zeroing_predication in module {}", module_path!());
    };
}

mkfn!{
    test_zeroing_predication_introspect!();
    #[test] fn test_zeroing_predication () { let str = r#"types: [i64]
zeroing_method: { drop: inactive }"# ; let input : IntrinsicInput = serde_yaml :: from_str (str) . expect ("failed to parse") ; let mut intrinsic = Intrinsic :: default () ; intrinsic . signature . name = "test_intrinsic{_mxz}" . parse () . unwrap () ; let mut variants = input . variants (& intrinsic) . unwrap () ; assert_eq ! (variants . next () , Some (InputSet (vec ! [InputType :: Type ("i64" . parse () . unwrap ()) , InputType :: PredicateForm (PredicateForm :: Merging) ,]))) ; assert_eq ! (variants . next () , Some (InputSet (vec ! [InputType :: Type ("i64" . parse () . unwrap ()) , InputType :: PredicateForm (PredicateForm :: DontCare (DontCareMethod :: AsZeroing)) ,]))) ; assert_eq ! (variants . next () , Some (InputSet (vec ! [InputType :: Type ("i64" . parse () . unwrap ()) , InputType :: PredicateForm (PredicateForm :: Zeroing (ZeroingMethod :: Drop { drop : "inactive" . parse () . unwrap () })) ,]))) ; assert_eq ! (variants . next () , None) }
} 
            }}