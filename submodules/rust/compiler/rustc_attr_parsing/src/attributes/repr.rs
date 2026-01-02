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
mkuse!{use rustc_abi :: Align ;}
mkuse!{use rustc_ast :: { IntTy , LitIntType , LitKind , UintTy } ;}
mkuse!{use rustc_hir :: attrs :: { IntType , ReprAttr } ;}
mkuse!{use super :: prelude :: * ;}
mkuse!{use crate :: session_diagnostics :: { self , IncorrectReprFormatGenericCause } ;}
mkitem!{mkstruct!{# [doc = " Parse #[repr(...)] forms."] # [doc = ""] # [doc = " Valid repr contents: any of the primitive integral type names (see"] # [doc = " `int_type_of_word`, below) to specify enum discriminant type; `C`, to use"] # [doc = " the same discriminant size that the corresponding C enum would or C"] # [doc = " structure layout, `packed` to remove padding, and `transparent` to delegate representation"] # [doc = " concerns to the only non-ZST field."] pub (crate) struct ReprParser ;}}
mkitem!{mkimpl!{impl < S : Stage > CombineAttributeParser < S > for ReprParser { type Item = (ReprAttr , Span) ; const PATH : & [Symbol] = & [sym :: repr] ; const CONVERT : ConvertFn < Self :: Item > = | items , first_span | AttributeKind :: Repr { reprs : items , first_span } ; const TEMPLATE : AttributeTemplate = template ! (List : & ["C" , "Rust" , "transparent" , "align(...)" , "packed(...)" , "<integer type>"] , "https://doc.rust-lang.org/reference/type-layout.html#representations") ; fn extend < 'c > (cx : & 'c mut AcceptContext < '_ , '_ , S > , args : & 'c ArgParser < '_ > ,) -> impl IntoIterator < Item = Self :: Item > + 'c { let mut reprs = Vec :: new () ; let Some (list) = args . list () else { cx . expected_list (cx . attr_span) ; return reprs ; } ; if list . is_empty () { cx . warn_empty_attribute (cx . attr_span) ; return reprs ; } for param in list . mixed () { if let Some (_) = param . lit () { cx . emit_err (session_diagnostics :: ReprIdent { span : cx . attr_span }) ; continue ; } reprs . extend (param . meta_item () . and_then (| mi | parse_repr (cx , & mi)) . map (| r | (r , param . span ())) ,) ; } reprs } const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; }}}
mkitem!{macro_rules ! int_pat { () => { sym :: i8 | sym :: u8 | sym :: i16 | sym :: u16 | sym :: i32 | sym :: u32 | sym :: i64 | sym :: u64 | sym :: i128 | sym :: u128 | sym :: isize | sym :: usize } ; }}

macro_rules! int_type_of_word_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function int_type_of_word in module {}", module_path!());
    };
}

mkfn!{
    int_type_of_word_introspect!();
    fn int_type_of_word (s : Symbol) -> Option < IntType > { use IntType :: * ; match s { sym :: i8 => Some (SignedInt (IntTy :: I8)) , sym :: u8 => Some (UnsignedInt (UintTy :: U8)) , sym :: i16 => Some (SignedInt (IntTy :: I16)) , sym :: u16 => Some (UnsignedInt (UintTy :: U16)) , sym :: i32 => Some (SignedInt (IntTy :: I32)) , sym :: u32 => Some (UnsignedInt (UintTy :: U32)) , sym :: i64 => Some (SignedInt (IntTy :: I64)) , sym :: u64 => Some (UnsignedInt (UintTy :: U64)) , sym :: i128 => Some (SignedInt (IntTy :: I128)) , sym :: u128 => Some (UnsignedInt (UintTy :: U128)) , sym :: isize => Some (SignedInt (IntTy :: Isize)) , sym :: usize => Some (UnsignedInt (UintTy :: Usize)) , _ => None , } }
}

macro_rules! parse_repr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_repr in module {}", module_path!());
    };
}

mkfn!{
    parse_repr_introspect!();
    fn parse_repr < S : Stage > (cx : & AcceptContext < '_ , '_ , S > , param : & MetaItemParser < '_ > ,) -> Option < ReprAttr > { use ReprAttr :: * ; let (name , ident_span) = if let Some (ident) = param . path () . word () { (Some (ident . name) , ident . span) } else { (None , DUMMY_SP) } ; let args = param . args () ; match (name , args) { (Some (sym :: align) , ArgParser :: NoArgs) => { cx . emit_err (session_diagnostics :: InvalidReprAlignNeedArg { span : ident_span }) ; None } (Some (sym :: align) , ArgParser :: List (l)) => { parse_repr_align (cx , l , param . span () , AlignKind :: Align) } (Some (sym :: packed) , ArgParser :: NoArgs) => Some (ReprPacked (Align :: ONE)) , (Some (sym :: packed) , ArgParser :: List (l)) => { parse_repr_align (cx , l , param . span () , AlignKind :: Packed) } (Some (name @ sym :: align | name @ sym :: packed) , ArgParser :: NameValue (l)) => { cx . emit_err (session_diagnostics :: IncorrectReprFormatGeneric { span : param . span () , repr_arg : name , cause : IncorrectReprFormatGenericCause :: from_lit_kind (param . span () , & l . value_as_lit () . kind , name ,) , }) ; None } (Some (sym :: Rust) , ArgParser :: NoArgs) => Some (ReprRust) , (Some (sym :: C) , ArgParser :: NoArgs) => Some (ReprC) , (Some (sym :: simd) , ArgParser :: NoArgs) => Some (ReprSimd) , (Some (sym :: transparent) , ArgParser :: NoArgs) => Some (ReprTransparent) , (Some (name @ int_pat ! ()) , ArgParser :: NoArgs) => { Some (ReprInt (int_type_of_word (name) . unwrap ())) } (Some (name @ sym :: Rust | name @ sym :: C | name @ sym :: simd | name @ sym :: transparent | name @ int_pat ! () ,) , ArgParser :: NameValue (_) ,) => { cx . emit_err (session_diagnostics :: InvalidReprHintNoValue { span : param . span () , name }) ; None } (Some (name @ sym :: Rust | name @ sym :: C | name @ sym :: simd | name @ sym :: transparent | name @ int_pat ! () ,) , ArgParser :: List (_) ,) => { cx . emit_err (session_diagnostics :: InvalidReprHintNoParen { span : param . span () , name }) ; None } _ => { cx . emit_err (session_diagnostics :: UnrecognizedReprHint { span : param . span () }) ; None } } }
}
mkitem!{mkenum!{enum AlignKind { Packed , Align , }}}

macro_rules! parse_repr_align_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_repr_align in module {}", module_path!());
    };
}

mkfn!{
    parse_repr_align_introspect!();
    fn parse_repr_align < S : Stage > (cx : & AcceptContext < '_ , '_ , S > , list : & MetaItemListParser < '_ > , param_span : Span , align_kind : AlignKind ,) -> Option < ReprAttr > { use AlignKind :: * ; let Some (align) = list . single () else { match align_kind { Packed => { cx . emit_err (session_diagnostics :: IncorrectReprFormatPackedOneOrZeroArg { span : param_span , }) ; } Align => { cx . emit_err (session_diagnostics :: IncorrectReprFormatAlignOneArg { span : param_span , }) ; } } return None ; } ; let Some (lit) = align . lit () else { match align_kind { Packed => { cx . emit_err (session_diagnostics :: IncorrectReprFormatPackedExpectInteger { span : align . span () , }) ; } Align => { cx . emit_err (session_diagnostics :: IncorrectReprFormatExpectInteger { span : align . span () , }) ; } } return None ; } ; match parse_alignment (& lit . kind) { Ok (literal) => Some (match align_kind { AlignKind :: Packed => ReprAttr :: ReprPacked (literal) , AlignKind :: Align => ReprAttr :: ReprAlign (literal) , }) , Err (message) => { cx . emit_err (session_diagnostics :: InvalidReprGeneric { span : lit . span , repr_arg : match align_kind { Packed => "packed" . to_string () , Align => "align" . to_string () , } , error_part : message , }) ; None } } }
}

macro_rules! parse_alignment_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_alignment in module {}", module_path!());
    };
}

mkfn!{
    parse_alignment_introspect!();
    fn parse_alignment (node : & LitKind) -> Result < Align , & 'static str > { if let LitKind :: Int (literal , LitIntType :: Unsuffixed) = node { if literal . get () . is_power_of_two () { literal . get () . try_into () . ok () . and_then (| v | Align :: from_bytes (v) . ok ()) . ok_or ("larger than 2^29") } else { Err ("not a power of two") } } else { Err ("not an unsuffixed integer") } }
}
mkitem!{mkstruct!{# [doc = " Parse #[align(N)]."] # [derive (Default)] pub (crate) struct AlignParser (Option < (Align , Span) >) ;}}
mkitem!{mkimpl!{impl AlignParser { const PATH : & 'static [Symbol] = & [sym :: rustc_align] ; const TEMPLATE : AttributeTemplate = template ! (List : & ["<alignment in bytes>"]) ; fn parse < 'c , S : Stage > (& mut self , cx : & 'c mut AcceptContext < '_ , '_ , S > , args : & 'c ArgParser < '_ > ,) { match args { ArgParser :: NoArgs | ArgParser :: NameValue (_) => { cx . expected_list (cx . attr_span) ; } ArgParser :: List (list) => { let Some (align) = list . single () else { cx . expected_single_argument (list . span) ; return ; } ; let Some (lit) = align . lit () else { cx . emit_err (session_diagnostics :: IncorrectReprFormatExpectInteger { span : align . span () , }) ; return ; } ; match parse_alignment (& lit . kind) { Ok (literal) => self . 0 = Ord :: max (self . 0 , Some ((literal , cx . attr_span))) , Err (message) => { cx . emit_err (session_diagnostics :: InvalidAlignmentValue { span : lit . span , error_part : message , }) ; } } } } } }}}
mkitem!{mkimpl!{impl < S : Stage > AttributeParser < S > for AlignParser { const ATTRIBUTES : AcceptMapping < Self , S > = & [(Self :: PATH , Self :: TEMPLATE , Self :: parse)] ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) , Allow (Target :: Method (MethodKind :: Trait { body : false })) , Allow (Target :: ForeignFn) ,]) ; fn finalize (self , _cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > { let (align , span) = self . 0 ? ; Some (AttributeKind :: Align { align , span }) } }}}
mkitem!{mkstruct!{# [derive (Default)] pub (crate) struct AlignStaticParser (AlignParser) ;}}
mkitem!{mkimpl!{impl AlignStaticParser { const PATH : & 'static [Symbol] = & [sym :: rustc_align_static] ; const TEMPLATE : AttributeTemplate = AlignParser :: TEMPLATE ; fn parse < 'c , S : Stage > (& mut self , cx : & 'c mut AcceptContext < '_ , '_ , S > , args : & 'c ArgParser < '_ > ,) { self . 0 . parse (cx , args) } }}}
mkitem!{mkimpl!{impl < S : Stage > AttributeParser < S > for AlignStaticParser { const ATTRIBUTES : AcceptMapping < Self , S > = & [(Self :: PATH , Self :: TEMPLATE , Self :: parse)] ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Static) , Allow (Target :: ForeignStatic)]) ; fn finalize (self , _cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > { let (align , span) = self . 0 . 0 ? ; Some (AttributeKind :: Align { align , span }) } }}}