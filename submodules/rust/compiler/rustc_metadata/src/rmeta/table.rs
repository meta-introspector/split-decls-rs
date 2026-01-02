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
mkuse!{use rustc_hir :: def :: CtorOf ;}
mkuse!{use rustc_index :: Idx ;}
mkuse!{use crate :: rmeta :: * ;}
mkitem!{mktrait!{pub (super) trait IsDefault : Default { fn is_default (& self) -> bool ; }}}
mkitem!{mkimpl!{impl < T > IsDefault for Option < T > { fn is_default (& self) -> bool { self . is_none () } }}}
mkitem!{mkimpl!{impl IsDefault for AttrFlags { fn is_default (& self) -> bool { self . is_empty () } }}}
mkitem!{mkimpl!{impl IsDefault for bool { fn is_default (& self) -> bool { ! self } }}}
mkitem!{mkimpl!{impl IsDefault for u32 { fn is_default (& self) -> bool { * self == 0 } }}}
mkitem!{mkimpl!{impl IsDefault for u64 { fn is_default (& self) -> bool { * self == 0 } }}}
mkitem!{mkimpl!{impl < T > IsDefault for LazyArray < T > { fn is_default (& self) -> bool { self . num_elems == 0 } }}}
mkitem!{mkimpl!{impl IsDefault for UnusedGenericParams { fn is_default (& self) -> bool { let is_default = self . bits () == 0 ; debug_assert_eq ! (is_default , self . all_used ()) ; is_default } }}}
mkitem!{mktrait!{# [doc = " Helper trait, for encoding to, and decoding from, a fixed number of bytes."] # [doc = " Used mainly for Lazy positions and lengths."] # [doc = " Unchecked invariant: `Self::default()` should encode as `[0; BYTE_LEN]`,"] # [doc = " but this has no impact on safety."] pub (super) trait FixedSizeEncoding : IsDefault { # [doc = " This should be `[u8; BYTE_LEN]`;"] # [doc = " Cannot use an associated `const BYTE_LEN: usize` instead due to const eval limitations."] type ByteArray ; fn from_bytes (b : & Self :: ByteArray) -> Self ; fn write_to_bytes (self , b : & mut Self :: ByteArray) ; }}}
mkitem!{mkimpl!{impl FixedSizeEncoding for u64 { type ByteArray = [u8 ; 8] ; # [inline] fn from_bytes (b : & [u8 ; 8]) -> Self { Self :: from_le_bytes (* b) } # [inline] fn write_to_bytes (self , b : & mut [u8 ; 8]) { * b = self . to_le_bytes () ; } }}}
mkitem!{macro_rules ! fixed_size_enum { ($ ty : ty { $ (($ ($ pat : tt) *)) * } $ (unreachable { $ (($ ($ upat : tt) *)) + }) ?) => { impl FixedSizeEncoding for Option <$ ty > { type ByteArray = [u8 ; 1] ; # [inline] fn from_bytes (b : & [u8 ; 1]) -> Self { use $ ty ::*; if b [0] == 0 { return None ; } match b [0] - 1 { $ ($ { index () } => Some ($ ($ pat) *) ,) * _ => panic ! ("Unexpected {} code: {:?}" , stringify ! ($ ty) , b [0]) , } } # [inline] fn write_to_bytes (self , b : & mut [u8 ; 1]) { use $ ty ::*; b [0] = match self { None => unreachable ! () , $ (Some ($ ($ pat) *) => 1 + $ { index () } ,) * $ (Some ($ ($ ($ upat) *) |+) => unreachable ! () ,) ? } } } } }}
mkitem!{macro_rules ! const_macro_kinds { ($ ($ name : ident) ,+$ (,) ?) => (MacroKinds :: from_bits_truncate ($ (MacroKinds ::$ name . bits ()) |+)) }}
mkitem!{const MACRO_KINDS_ATTR_BANG : MacroKinds = const_macro_kinds ! (ATTR , BANG) ;}
mkitem!{const MACRO_KINDS_DERIVE_BANG : MacroKinds = const_macro_kinds ! (DERIVE , BANG) ;}
mkitem!{const MACRO_KINDS_DERIVE_ATTR : MacroKinds = const_macro_kinds ! (DERIVE , ATTR) ;}
mkitem!{const MACRO_KINDS_DERIVE_ATTR_BANG : MacroKinds = const_macro_kinds ! (DERIVE , ATTR , BANG) ;}
mkitem!{const _ : () = assert ! (MACRO_KINDS_DERIVE_ATTR_BANG . is_all ()) ;}
mkitem!{fixed_size_enum ! { DefKind { (Mod) (Struct) (Union) (Enum) (Variant) (Trait) (TyAlias) (ForeignTy) (TraitAlias) (AssocTy) (TyParam) (Fn) (Const) (ConstParam) (AssocFn) (AssocConst) (ExternCrate) (Use) (ForeignMod) (AnonConst) (InlineConst) (OpaqueTy) (Field) (LifetimeParam) (GlobalAsm) (Impl { of_trait : false }) (Impl { of_trait : true }) (Closure) (Static { safety : hir :: Safety :: Unsafe , mutability : ast :: Mutability :: Not , nested : false }) (Static { safety : hir :: Safety :: Safe , mutability : ast :: Mutability :: Not , nested : false }) (Static { safety : hir :: Safety :: Unsafe , mutability : ast :: Mutability :: Mut , nested : false }) (Static { safety : hir :: Safety :: Safe , mutability : ast :: Mutability :: Mut , nested : false }) (Static { safety : hir :: Safety :: Unsafe , mutability : ast :: Mutability :: Not , nested : true }) (Static { safety : hir :: Safety :: Safe , mutability : ast :: Mutability :: Not , nested : true }) (Static { safety : hir :: Safety :: Unsafe , mutability : ast :: Mutability :: Mut , nested : true }) (Static { safety : hir :: Safety :: Safe , mutability : ast :: Mutability :: Mut , nested : true }) (Ctor (CtorOf :: Struct , CtorKind :: Fn)) (Ctor (CtorOf :: Struct , CtorKind :: Const)) (Ctor (CtorOf :: Variant , CtorKind :: Fn)) (Ctor (CtorOf :: Variant , CtorKind :: Const)) (Macro (MacroKinds :: BANG)) (Macro (MacroKinds :: ATTR)) (Macro (MacroKinds :: DERIVE)) (Macro (MACRO_KINDS_ATTR_BANG)) (Macro (MACRO_KINDS_DERIVE_ATTR)) (Macro (MACRO_KINDS_DERIVE_BANG)) (Macro (MACRO_KINDS_DERIVE_ATTR_BANG)) (SyntheticCoroutineBody) } unreachable { (Macro (_)) } }}
mkitem!{fixed_size_enum ! { hir :: Constness { (NotConst) (Const) } }}
mkitem!{fixed_size_enum ! { hir :: Defaultness { (Final) (Default { has_value : false }) (Default { has_value : true }) } }}
mkitem!{fixed_size_enum ! { hir :: Safety { (Unsafe) (Safe) } }}
mkitem!{fixed_size_enum ! { ty :: Asyncness { (Yes) (No) } }}
mkitem!{fixed_size_enum ! { hir :: CoroutineKind { (Coroutine (hir :: Movability :: Movable)) (Coroutine (hir :: Movability :: Static)) (Desugared (hir :: CoroutineDesugaring :: Gen , hir :: CoroutineSource :: Block)) (Desugared (hir :: CoroutineDesugaring :: Gen , hir :: CoroutineSource :: Fn)) (Desugared (hir :: CoroutineDesugaring :: Gen , hir :: CoroutineSource :: Closure)) (Desugared (hir :: CoroutineDesugaring :: Async , hir :: CoroutineSource :: Block)) (Desugared (hir :: CoroutineDesugaring :: Async , hir :: CoroutineSource :: Fn)) (Desugared (hir :: CoroutineDesugaring :: Async , hir :: CoroutineSource :: Closure)) (Desugared (hir :: CoroutineDesugaring :: AsyncGen , hir :: CoroutineSource :: Block)) (Desugared (hir :: CoroutineDesugaring :: AsyncGen , hir :: CoroutineSource :: Fn)) (Desugared (hir :: CoroutineDesugaring :: AsyncGen , hir :: CoroutineSource :: Closure)) } }}
mkitem!{fixed_size_enum ! { MacroKind { (Attr) (Bang) (Derive) } }}
mkitem!{mkimpl!{impl FixedSizeEncoding for Option < RawDefId > { type ByteArray = [u8 ; 8] ; # [inline] fn from_bytes (encoded : & [u8 ; 8]) -> Self { let (index , krate) = decode_interleaved (encoded) ; let krate = u32 :: from_le_bytes (krate) ; if krate == 0 { return None ; } let index = u32 :: from_le_bytes (index) ; Some (RawDefId { krate : krate - 1 , index }) } # [inline] fn write_to_bytes (self , dest : & mut [u8 ; 8]) { match self { None => unreachable ! () , Some (RawDefId { krate , index }) => { debug_assert ! (krate < u32 :: MAX) ; let krate = (krate + 1) . to_le_bytes () ; let index = index . to_le_bytes () ; encode_interleaved (index , krate , dest) ; } } } }}}
mkitem!{mkimpl!{impl FixedSizeEncoding for AttrFlags { type ByteArray = [u8 ; 1] ; # [inline] fn from_bytes (b : & [u8 ; 1]) -> Self { AttrFlags :: from_bits_truncate (b [0]) } # [inline] fn write_to_bytes (self , b : & mut [u8 ; 1]) { debug_assert ! (! self . is_default ()) ; b [0] = self . bits () ; } }}}
mkitem!{mkimpl!{impl FixedSizeEncoding for bool { type ByteArray = [u8 ; 1] ; # [inline] fn from_bytes (b : & [u8 ; 1]) -> Self { b [0] != 0 } # [inline] fn write_to_bytes (self , b : & mut [u8 ; 1]) { debug_assert ! (! self . is_default ()) ; b [0] = self as u8 } }}}
mkitem!{mkimpl!{impl < T > FixedSizeEncoding for Option < LazyValue < T > > { type ByteArray = [u8 ; 8] ; # [inline] fn from_bytes (b : & [u8 ; 8]) -> Self { let position = NonZero :: new (u64 :: from_bytes (b) as usize) ? ; Some (LazyValue :: from_position (position)) } # [inline] fn write_to_bytes (self , b : & mut [u8 ; 8]) { match self { None => unreachable ! () , Some (lazy) => { let position = lazy . position . get () ; let position : u64 = position . try_into () . unwrap () ; position . write_to_bytes (b) } } } }}}
mkitem!{mkimpl!{impl < T > LazyArray < T > { # [inline] fn write_to_bytes_impl (self , dest : & mut [u8 ; 16]) { let position = (self . position . get () as u64) . to_le_bytes () ; let len = (self . num_elems as u64) . to_le_bytes () ; encode_interleaved (position , len , dest) } fn from_bytes_impl (position : & [u8 ; 8] , meta : & [u8 ; 8]) -> Option < LazyArray < T > > { let position = NonZero :: new (u64 :: from_bytes (position) as usize) ? ; let len = u64 :: from_bytes (meta) as usize ; Some (LazyArray :: from_position_and_num_elems (position , len)) } }}}

macro_rules! decode_interleaved_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function decode_interleaved in module {}", module_path!());
    };
}

mkfn!{
    decode_interleaved_introspect!();
    # [inline] fn decode_interleaved < const N : usize , const M : usize > (encoded : & [u8 ; N]) -> ([u8 ; M] , [u8 ; M]) { assert_eq ! (M * 2 , N) ; let mut first = [0u8 ; M] ; let mut second = [0u8 ; M] ; for i in 0 .. M { first [i] = encoded [2 * i] ; second [i] = encoded [2 * i + 1] ; } (first , second) }
}

macro_rules! encode_interleaved_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function encode_interleaved in module {}", module_path!());
    };
}

mkfn!{
    encode_interleaved_introspect!();
    # [inline] fn encode_interleaved < const N : usize , const M : usize > (a : [u8 ; M] , b : [u8 ; M] , dest : & mut [u8 ; N]) { assert_eq ! (M * 2 , N) ; for i in 0 .. M { dest [2 * i] = a [i] ; dest [2 * i + 1] = b [i] ; } }
}
mkitem!{mkimpl!{impl < T > FixedSizeEncoding for LazyArray < T > { type ByteArray = [u8 ; 16] ; # [inline] fn from_bytes (b : & [u8 ; 16]) -> Self { let (position , meta) = decode_interleaved (b) ; if meta == [0 ; 8] { return Default :: default () ; } LazyArray :: from_bytes_impl (& position , & meta) . unwrap () } # [inline] fn write_to_bytes (self , b : & mut [u8 ; 16]) { assert ! (! self . is_default ()) ; self . write_to_bytes_impl (b) } }}}
mkitem!{mkimpl!{impl < T > FixedSizeEncoding for Option < LazyArray < T > > { type ByteArray = [u8 ; 16] ; # [inline] fn from_bytes (b : & [u8 ; 16]) -> Self { let (position , meta) = decode_interleaved (b) ; LazyArray :: from_bytes_impl (& position , & meta) } # [inline] fn write_to_bytes (self , b : & mut [u8 ; 16]) { match self { None => unreachable ! () , Some (lazy) => lazy . write_to_bytes_impl (b) , } } }}}
mkitem!{mkstruct!{# [doc = " Helper for constructing a table's serialization (also see `Table`)."] pub (super) struct TableBuilder < I : Idx , T : FixedSizeEncoding > { width : usize , blocks : IndexVec < I , T :: ByteArray > , _marker : PhantomData < T > , }}}
mkitem!{mkimpl!{impl < I : Idx , T : FixedSizeEncoding > Default for TableBuilder < I , T > { fn default () -> Self { TableBuilder { width : 0 , blocks : Default :: default () , _marker : PhantomData } } }}}
mkitem!{mkimpl!{impl < I : Idx , const N : usize , T > TableBuilder < I , Option < T > > where Option < T > : FixedSizeEncoding < ByteArray = [u8 ; N] > , { pub (crate) fn set_some (& mut self , i : I , value : T) { self . set (i , Some (value)) } }}}
mkitem!{mkimpl!{impl < I : Idx , const N : usize , T : FixedSizeEncoding < ByteArray = [u8 ; N] > > TableBuilder < I , T > { # [doc = " Sets the table value if it is not default."] # [doc = " ATTENTION: For optimization default values are simply ignored by this function, because"] # [doc = " right now metadata tables never need to reset non-default values to default. If such need"] # [doc = " arises in the future then a new method (e.g. `clear` or `reset`) will need to be introduced"] # [doc = " for doing that explicitly."] pub (crate) fn set (& mut self , i : I , value : T) { if ! value . is_default () { let block = self . blocks . ensure_contains_elem (i , | | [0 ; N]) ; value . write_to_bytes (block) ; if self . width != N { let width = N - trailing_zeros (block) ; self . width = self . width . max (width) ; } } } pub (crate) fn encode (& self , buf : & mut FileEncoder) -> LazyTable < I , T > { let pos = buf . position () ; let width = self . width ; for block in & self . blocks { buf . write_with (| dest | { * dest = * block ; width }) ; } LazyTable :: from_position_and_encoded_size (NonZero :: new (pos) . unwrap () , width , self . blocks . len () ,) } }}}

macro_rules! trailing_zeros_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trailing_zeros in module {}", module_path!());
    };
}

mkfn!{
    trailing_zeros_introspect!();
    fn trailing_zeros (x : & [u8]) -> usize { x . iter () . rev () . take_while (| b | * * b == 0) . count () }
}
mkitem!{mkimpl!{impl < I : Idx , const N : usize , T : FixedSizeEncoding < ByteArray = [u8 ; N] > + ParameterizedOverTcx > LazyTable < I , T > where for < 'tcx > T :: Value < 'tcx > : FixedSizeEncoding < ByteArray = [u8 ; N] > , { # [doc = " Given the metadata, extract out the value at a particular index (if any)."] pub (super) fn get < 'a , 'tcx , M : Metadata < 'a , 'tcx > > (& self , metadata : M , i : I) -> T :: Value < 'tcx > { if i . index () >= self . len { return Default :: default () ; } let width = self . width ; let start = self . position . get () + (width * i . index ()) ; let end = start + width ; let bytes = & metadata . blob () [start .. end] ; if let Ok (fixed) = bytes . try_into () { FixedSizeEncoding :: from_bytes (fixed) } else { let mut fixed = [0u8 ; N] ; fixed [.. width] . copy_from_slice (bytes) ; FixedSizeEncoding :: from_bytes (& fixed) } } # [doc = " Size of the table in entries, including possible gaps."] pub (super) fn size (& self) -> usize { self . len } }}}