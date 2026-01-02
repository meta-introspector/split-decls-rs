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
mkuse!{use itertools :: Itertools ;}
mkuse!{use proc_macro2 :: TokenStream ;}
mkuse!{use quote :: { ToTokens , TokenStreamExt , quote } ;}
mkuse!{use serde_with :: { DeserializeFromStr , SerializeDisplay } ;}
mkuse!{use std :: str :: pattern :: Pattern ;}
mkuse!{use std :: { fmt , str :: FromStr } ;}
mkuse!{use crate :: context :: LocalContext ;}
mkuse!{use crate :: fn_suffix :: make_neon_suffix ;}
mkuse!{use crate :: typekinds :: { ToRepr , TypeRepr } ;}
mkuse!{use crate :: wildcards :: Wildcard ;}
mkitem!{mkenum!{# [derive (Debug , Clone , PartialEq , Eq)] pub enum WildStringPart { String (String) , Wildcard (Wildcard) , }}}
mkitem!{mkstruct!{# [doc = " Wildcard-able string"] # [derive (Debug , Clone , PartialEq , Eq , Default , SerializeDisplay , DeserializeFromStr)] pub struct WildString (pub Vec < WildStringPart >) ;}}
mkitem!{mkimpl!{impl WildString { pub fn has_wildcards (& self) -> bool { for part in self . 0 . iter () { if let WildStringPart :: Wildcard (..) = part { return true ; } } false } pub fn wildcards (& self) -> impl Iterator < Item = & Wildcard > + '_ { self . 0 . iter () . filter_map (| part | match part { WildStringPart :: Wildcard (w) => Some (w) , _ => None , }) } pub fn iter (& self) -> impl Iterator < Item = & WildStringPart > + '_ { self . 0 . iter () } pub fn iter_mut (& mut self) -> impl Iterator < Item = & mut WildStringPart > + '_ { self . 0 . iter_mut () } pub fn starts_with (& self , s2 : & str) -> bool { self . to_string () . starts_with (s2) } pub fn prepend_str (& mut self , s : impl Into < String >) { self . 0 . insert (0 , WildStringPart :: String (s . into ())) } pub fn push_str (& mut self , s : impl Into < String >) { self . 0 . push (WildStringPart :: String (s . into ())) } pub fn push_wildcard (& mut self , w : Wildcard) { self . 0 . push (WildStringPart :: Wildcard (w)) } pub fn is_empty (& self) -> bool { self . 0 . is_empty () } pub fn replace < P > (& self , from : P , to : & str) -> WildString where P : Pattern + Copy , { WildString (self . 0 . iter () . map (| part | match part { WildStringPart :: String (s) => WildStringPart :: String (s . replace (from , to)) , part => part . clone () , }) . collect_vec () ,) } pub fn build_acle (& mut self , ctx : & LocalContext) -> Result < () , String > { self . build (ctx , TypeRepr :: ACLENotation) } pub fn build_neon_intrinsic_signature (& mut self , ctx : & LocalContext) -> Result < () , String > { let repr = TypeRepr :: ACLENotation ; self . iter_mut () . try_for_each (| wp | -> Result < () , String > { if let WildStringPart :: Wildcard (w) = wp { match w { & mut Wildcard :: NEONType (_ , _ , ref maybe_suffix_kind) => { if let Some (suffix_kind) = maybe_suffix_kind { let x = ctx . provide_type_wildcard (w) . unwrap () ; * wp = WildStringPart :: String (make_neon_suffix (x , * suffix_kind)) } else { * wp = WildString :: make_default_build (ctx , repr , w) } } _ => * wp = WildString :: make_default_build (ctx , repr , w) , } } Ok (()) }) } pub fn build (& mut self , ctx : & LocalContext , repr : TypeRepr) -> Result < () , String > { match repr { TypeRepr :: ACLENotation | TypeRepr :: LLVMMachine => { self . iter_mut () . try_for_each (| wp | -> Result < () , String > { if let WildStringPart :: Wildcard (w) = wp { match w { & mut Wildcard :: NEONType (_ , _ , ref maybe_suffix_kind) => { if let Some (suffix_kind) = maybe_suffix_kind { let x = ctx . provide_type_wildcard (w) . unwrap () ; * wp = WildStringPart :: String (make_neon_suffix (x , * suffix_kind)) } else { * wp = WildString :: make_default_build (ctx , repr , w) } } _ => * wp = WildString :: make_default_build (ctx , repr , w) , } } Ok (()) }) } _ => self . iter_mut () . try_for_each (| wp | -> Result < () , String > { if let WildStringPart :: Wildcard (w) = wp { * wp = WildString :: make_default_build (ctx , repr , w) ; } Ok (()) }) , } } fn make_default_build (ctx : & LocalContext , repr : TypeRepr , w : & mut Wildcard) -> WildStringPart { WildStringPart :: String (ctx . provide_substitution_wildcard (w) . or_else (| _ | ctx . provide_type_wildcard (w) . map (| ty | ty . repr (repr))) . unwrap () ,) } }}}
mkitem!{mkimpl!{impl From < String > for WildString { fn from (s : String) -> Self { WildString (vec ! [WildStringPart :: String (s)]) } }}}
mkitem!{mkimpl!{impl FromStr for WildString { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { enum State { Normal { start : usize } , Wildcard { start : usize , count : usize } , EscapeTokenOpen { start : usize , at : usize } , EscapeTokenClose { start : usize , at : usize } , } let mut ws = WildString :: default () ; match s . char_indices () . try_fold (State :: Normal { start : 0 } , | state , (idx , ch) | { match (state , ch) { (State :: Normal { start } , '{') => Ok (State :: EscapeTokenOpen { start , at : idx }) , (State :: Normal { start } , '}') => { Ok (State :: EscapeTokenClose { start , at : idx }) } (State :: EscapeTokenOpen { start , at } , '{') | (State :: EscapeTokenClose { start , at } , '}') => { if start < at { ws . push_str (& s [start .. at]) } Ok (State :: Normal { start : idx }) } (State :: EscapeTokenOpen { at , .. } , '}') => Err (format ! ("empty wildcard given in string {s:?} at position {at}")) , (State :: EscapeTokenOpen { start , at } , _) => { if start < at { ws . push_str (& s [start .. at]) } Ok (State :: Wildcard { start : idx , count : 0 , }) } (State :: EscapeTokenClose { at , .. } , _) => Err (format ! ("closing a non-wildcard/bad escape in string {s:?} at position {at}")) , (State :: Wildcard { start , count } , '{') => Ok (State :: Wildcard { start , count : count + 1 , }) , (State :: Wildcard { start , count : 0 } , '}') => { ws . push_wildcard (s [start .. idx] . parse () ?) ; Ok (State :: Normal { start : idx + 1 }) } (State :: Wildcard { start , count } , '}') => Ok (State :: Wildcard { start , count : count - 1 , }) , (state @ State :: Normal { .. } , _) | (state @ State :: Wildcard { .. } , _) => { Ok (state) } } }) ? { State :: Normal { start } => { if start < s . len () { ws . push_str (& s [start ..]) ; } Ok (ws) } State :: EscapeTokenOpen { at , .. } | State :: Wildcard { start : at , .. } => Err (format ! ("unclosed wildcard in string {s:?} at position {at}")) , State :: EscapeTokenClose { at , .. } => Err (format ! ("closing a non-wildcard/bad escape in string {s:?} at position {at}")) , } } }}}
mkitem!{mkimpl!{impl fmt :: Display for WildString { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . 0 . iter () . map (| part | match part { WildStringPart :: String (s) => s . to_owned () , WildStringPart :: Wildcard (w) => format ! ("{{{w}}}") , }) . join ("")) } }}}
mkitem!{mkimpl!{impl ToTokens for WildString { fn to_tokens (& self , tokens : & mut TokenStream) { assert ! (! self . has_wildcards () , "cannot convert string with wildcards {self:?} to TokenStream") ; let str = self . to_string () ; tokens . append_all (quote ! { # str }) } }}}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: typekinds :: * ;}
mkuse!{use crate :: wildstring :: * ;}

macro_rules! test_empty_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_empty_string in module {}", module_path!());
    };
}

mkfn!{
    test_empty_string_introspect!();
    # [test] fn test_empty_string () { let ws : WildString = "" . parse () . unwrap () ; assert_eq ! (ws . 0 . len () , 0) ; }
}

macro_rules! test_plain_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_plain_string in module {}", module_path!());
    };
}

mkfn!{
    test_plain_string_introspect!();
    # [test] fn test_plain_string () { let ws : WildString = "plain string" . parse () . unwrap () ; assert_eq ! (ws . 0 . len () , 1) ; assert_eq ! (ws , WildString (vec ! [WildStringPart :: String ("plain string" . to_string ())])) }
}

macro_rules! test_escaped_curly_brackets_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_escaped_curly_brackets in module {}", module_path!());
    };
}

mkfn!{
    test_escaped_curly_brackets_introspect!();
    # [test] fn test_escaped_curly_brackets () { let ws : WildString = "VALUE = {{value}}" . parse () . unwrap () ; assert_eq ! (ws . to_string () , "VALUE = {value}") ; assert ! (! ws . has_wildcards ()) ; }
}

macro_rules! test_escaped_curly_brackets_wildcard_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_escaped_curly_brackets_wildcard in module {}", module_path!());
    };
}

mkfn!{
    test_escaped_curly_brackets_wildcard_introspect!();
    # [test] fn test_escaped_curly_brackets_wildcard () { let ws : WildString = "TYPE = {{{type}}}" . parse () . unwrap () ; assert_eq ! (ws . to_string () , "TYPE = {{type}}") ; assert_eq ! (ws . 0 . len () , 4) ; assert ! (ws . has_wildcards ()) ; }
}

macro_rules! test_wildcard_right_boundary_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_wildcard_right_boundary in module {}", module_path!());
    };
}

mkfn!{
    test_wildcard_right_boundary_introspect!();
    # [test] fn test_wildcard_right_boundary () { let s = "string test {type}" ; let ws : WildString = s . parse () . unwrap () ; assert_eq ! (& ws . to_string () , s) ; assert ! (ws . has_wildcards ()) ; }
}

macro_rules! test_wildcard_left_boundary_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_wildcard_left_boundary in module {}", module_path!());
    };
}

mkfn!{
    test_wildcard_left_boundary_introspect!();
    # [test] fn test_wildcard_left_boundary () { let s = "{type} string test" ; let ws : WildString = s . parse () . unwrap () ; assert_eq ! (& ws . to_string () , s) ; assert ! (ws . has_wildcards ()) ; }
}

macro_rules! test_recursive_wildcard_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_recursive_wildcard in module {}", module_path!());
    };
}

mkfn!{
    test_recursive_wildcard_introspect!();
    # [test] fn test_recursive_wildcard () { let s = "string test {type[0] as {type[1]}}" ; let ws : WildString = s . parse () . unwrap () ; assert_eq ! (ws . 0 . len () , 2) ; assert_eq ! (ws , WildString (vec ! [WildStringPart :: String ("string test " . to_string ()) , WildStringPart :: Wildcard (Wildcard :: Scale (Box :: new (Wildcard :: Type (Some (0))) , Box :: new (TypeKind :: Wildcard (Wildcard :: Type (Some (1)))) ,))])) ; }
}

macro_rules! test_scale_wildcard_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_scale_wildcard in module {}", module_path!());
    };
}

mkfn!{
    test_scale_wildcard_introspect!();
    # [test] fn test_scale_wildcard () { let s = "string {type[0] as i8} test" ; let ws : WildString = s . parse () . unwrap () ; assert_eq ! (ws . 0 . len () , 3) ; assert_eq ! (ws , WildString (vec ! [WildStringPart :: String ("string " . to_string ()) , WildStringPart :: Wildcard (Wildcard :: Scale (Box :: new (Wildcard :: Type (Some (0))) , Box :: new (TypeKind :: Base (BaseType :: Sized (BaseTypeKind :: Int , 8))) ,)) , WildStringPart :: String (" test" . to_string ())])) ; }
}

macro_rules! test_solitaire_wildcard_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_solitaire_wildcard in module {}", module_path!());
    };
}

mkfn!{
    test_solitaire_wildcard_introspect!();
    # [test] fn test_solitaire_wildcard () { let ws : WildString = "{type}" . parse () . unwrap () ; assert_eq ! (ws . 0 . len () , 1) ; assert_eq ! (ws , WildString (vec ! [WildStringPart :: Wildcard (Wildcard :: Type (None))])) }
}

macro_rules! test_empty_wildcard_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_empty_wildcard in module {}", module_path!());
    };
}

mkfn!{
    test_empty_wildcard_introspect!();
    # [test] fn test_empty_wildcard () { "string {}" . parse :: < WildString > () . expect_err ("expected parse error") ; }
}

macro_rules! test_invalid_open_wildcard_right_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_invalid_open_wildcard_right in module {}", module_path!());
    };
}

mkfn!{
    test_invalid_open_wildcard_right_introspect!();
    # [test] fn test_invalid_open_wildcard_right () { "string {" . parse :: < WildString > () . expect_err ("expected parse error") ; }
}

macro_rules! test_invalid_close_wildcard_right_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_invalid_close_wildcard_right in module {}", module_path!());
    };
}

mkfn!{
    test_invalid_close_wildcard_right_introspect!();
    # [test] fn test_invalid_close_wildcard_right () { "string }" . parse :: < WildString > () . expect_err ("expected parse error") ; }
}

macro_rules! test_invalid_open_wildcard_left_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_invalid_open_wildcard_left in module {}", module_path!());
    };
}

mkfn!{
    test_invalid_open_wildcard_left_introspect!();
    # [test] fn test_invalid_open_wildcard_left () { "{string" . parse :: < WildString > () . expect_err ("expected parse error") ; }
}

macro_rules! test_invalid_close_wildcard_left_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_invalid_close_wildcard_left in module {}", module_path!());
    };
}

mkfn!{
    test_invalid_close_wildcard_left_introspect!();
    # [test] fn test_invalid_close_wildcard_left () { "}string" . parse :: < WildString > () . expect_err ("expected parse error") ; }
}

macro_rules! test_consecutive_wildcards_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_consecutive_wildcards in module {}", module_path!());
    };
}

mkfn!{
    test_consecutive_wildcards_introspect!();
    # [test] fn test_consecutive_wildcards () { let s = "svprf{size_literal[1]}_gather_{type[0]}{index_or_offset}" ; let ws : WildString = s . parse () . unwrap () ; assert_eq ! (ws . to_string () , s) }
} 
            }}