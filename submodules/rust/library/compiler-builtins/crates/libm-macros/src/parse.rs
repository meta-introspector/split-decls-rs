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
mkuse!{use std :: collections :: BTreeMap ;}
mkuse!{use proc_macro2 :: Span ;}
mkuse!{use quote :: ToTokens ;}
mkuse!{use syn :: parse :: { Parse , ParseStream , Parser } ;}
mkuse!{use syn :: punctuated :: Punctuated ;}
mkuse!{use syn :: spanned :: Spanned ;}
mkuse!{use syn :: token :: { self , Comma } ;}
mkuse!{use syn :: { Arm , Attribute , Expr , ExprMatch , Ident , LitBool , Meta , Token , bracketed } ;}
mkitem!{mkstruct!{# [doc = " The input to our macro; just a list of `field: value` items."] # [derive (Debug)] pub struct Invocation { fields : Punctuated < Mapping , Comma > , }}}
mkitem!{mkimpl!{impl Parse for Invocation { fn parse (input : ParseStream) -> syn :: Result < Self > { Ok (Self { fields : input . parse_terminated (Mapping :: parse , Token ! [,]) ? , }) } }}}
mkitem!{mkstruct!{# [doc = " A `key: expression` mapping with nothing else. Basically a simplified `syn::Field`."] # [derive (Debug)] struct Mapping { name : Ident , _sep : Token ! [:] , expr : Expr , }}}
mkitem!{mkimpl!{impl Parse for Mapping { fn parse (input : ParseStream) -> syn :: Result < Self > { Ok (Self { name : input . parse () ? , _sep : input . parse () ? , expr : input . parse () ? , }) } }}}
mkitem!{mkstruct!{# [doc = " The input provided to our proc macro, after parsing into the form we expect."] # [derive (Debug)] pub struct StructuredInput { # [doc = " Macro to invoke once per function"] pub callback : Ident , # [doc = " Whether or not to provide `CFn` `CArgs` `RustFn` etc. This is really only needed"] # [doc = " once for crate to set up the main trait."] pub emit_types : Vec < Ident > , # [doc = " Skip these functions"] pub skip : Vec < Ident > , # [doc = " If true, omit f16 and f128 functions that aren't present in other libraries."] pub skip_f16_f128 : bool , # [doc = " Invoke only for these functions"] pub only : Option < Vec < Ident > > , # [doc = " Attributes that get applied to specific functions"] pub attributes : Option < Vec < AttributeMap > > , # [doc = " Extra expressions to pass to all invocations of the macro"] pub extra : Option < Expr > , # [doc = " Per-function extra expressions to pass to the macro"] pub fn_extra : Option < BTreeMap < Ident , Expr > > , pub emit_types_span : Option < Span > , pub only_span : Option < Span > , pub fn_extra_span : Option < Span > , }}}
mkitem!{mkimpl!{impl StructuredInput { pub fn from_fields (input : Invocation) -> syn :: Result < Self > { let mut map : Vec < _ > = input . fields . into_iter () . collect () ; let cb_expr = expect_field (& mut map , "callback") ? ; let emit_types_expr = expect_field (& mut map , "emit_types") . ok () ; let skip_expr = expect_field (& mut map , "skip") . ok () ; let skip_f16_f128 = expect_field (& mut map , "skip_f16_f128") . ok () ; let only_expr = expect_field (& mut map , "only") . ok () ; let attr_expr = expect_field (& mut map , "attributes") . ok () ; let extra = expect_field (& mut map , "extra") . ok () ; let fn_extra = expect_field (& mut map , "fn_extra") . ok () ; if ! map . is_empty () { Err (syn :: Error :: new (map . first () . unwrap () . name . span () , format ! ("unexpected fields {map:?}") ,)) ? ; } let emit_types_span = emit_types_expr . as_ref () . map (| expr | expr . span ()) ; let emit_types = match emit_types_expr { Some (expr) => Parser :: parse2 (parse_ident_or_array , expr . into_token_stream ()) ? , None => Vec :: new () , } ; let skip = match skip_expr { Some (expr) => Parser :: parse2 (parse_ident_array , expr . into_token_stream ()) ? , None => Vec :: new () , } ; let skip_f16_f128 = match skip_f16_f128 { Some (expr) => expect_litbool (expr) ? . value , None => false , } ; let only_span = only_expr . as_ref () . map (| expr | expr . span ()) ; let only = match only_expr { Some (expr) => Some (Parser :: parse2 (parse_ident_array , expr . into_token_stream ()) ?) , None => None , } ; let attributes = match attr_expr { Some (expr) => { let mut attributes = Vec :: new () ; let attr_exprs = Parser :: parse2 (parse_expr_array , expr . into_token_stream ()) ? ; for attr in attr_exprs { attributes . push (syn :: parse2 (attr . into_token_stream ()) ?) ; } Some (attributes) } None => None , } ; let fn_extra_span = fn_extra . as_ref () . map (| expr | expr . span ()) ; let fn_extra = match fn_extra { Some (expr) => Some (extract_fn_extra_field (expr) ?) , None => None , } ; Ok (Self { callback : expect_ident (cb_expr) ? , emit_types , skip , skip_f16_f128 , only , only_span , attributes , extra , fn_extra , fn_extra_span , emit_types_span , }) } }}}

macro_rules! extract_fn_extra_field_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function extract_fn_extra_field in module {}", module_path!());
    };
}

mkfn!{
    extract_fn_extra_field_introspect!();
    fn extract_fn_extra_field (expr : Expr) -> syn :: Result < BTreeMap < Ident , Expr > > { let Expr :: Match (mexpr) = expr else { let e = syn :: Error :: new (expr . span () , "`fn_extra` expects a match expression") ; return Err (e) ; } ; let ExprMatch { attrs , match_token : _ , expr , brace_token : _ , arms , } = mexpr ; expect_empty_attrs (& attrs) ? ; let match_on = expect_ident (* expr) ? ; if match_on != "MACRO_FN_NAME" { let e = syn :: Error :: new (match_on . span () , "only allowed to match on `MACRO_FN_NAME`") ; return Err (e) ; } let mut res = BTreeMap :: new () ; for arm in arms { let Arm { attrs , pat , guard , fat_arrow_token : _ , body , comma : _ , } = arm ; expect_empty_attrs (& attrs) ? ; let keys = match pat { syn :: Pat :: Wild (w) => vec ! [Ident :: new ("_" , w . span ())] , _ => Parser :: parse2 (parse_ident_pat , pat . into_token_stream ()) ? , } ; if let Some (guard) = guard { let e = syn :: Error :: new (guard . 0 . span () , "no guards allowed in this position") ; return Err (e) ; } for key in keys { let inserted = res . insert (key . clone () , * body . clone ()) ; if inserted . is_some () { let e = syn :: Error :: new (key . span () , format ! ("key `{key}` specified twice")) ; return Err (e) ; } } } Ok (res) }
}

macro_rules! expect_empty_attrs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expect_empty_attrs in module {}", module_path!());
    };
}

mkfn!{
    expect_empty_attrs_introspect!();
    fn expect_empty_attrs (attrs : & [Attribute]) -> syn :: Result < () > { if attrs . is_empty () { return Ok (()) ; } let e = syn :: Error :: new (attrs . first () . unwrap () . span () , "no attributes allowed in this position" ,) ; Err (e) }
}

macro_rules! expect_field_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expect_field in module {}", module_path!());
    };
}

mkfn!{
    expect_field_introspect!();
    # [doc = " Extract a named field from a map, raising an error if it doesn't exist."] fn expect_field (v : & mut Vec < Mapping > , name : & str) -> syn :: Result < Expr > { let pos = v . iter () . position (| v | v . name == name) . ok_or_else (| | { syn :: Error :: new (Span :: call_site () , format ! ("missing expected field `{name}`") ,) }) ? ; Ok (v . remove (pos) . expr) }
}

macro_rules! expect_ident_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expect_ident in module {}", module_path!());
    };
}

mkfn!{
    expect_ident_introspect!();
    # [doc = " Coerce an expression into a simple identifier."] fn expect_ident (expr : Expr) -> syn :: Result < Ident > { syn :: parse2 (expr . into_token_stream ()) }
}

macro_rules! expect_litbool_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expect_litbool in module {}", module_path!());
    };
}

mkfn!{
    expect_litbool_introspect!();
    # [doc = " Coerce an expression into a simple keyword."] fn expect_litbool (expr : Expr) -> syn :: Result < LitBool > { syn :: parse2 (expr . into_token_stream ()) }
}

macro_rules! parse_ident_or_array_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_ident_or_array in module {}", module_path!());
    };
}

mkfn!{
    parse_ident_or_array_introspect!();
    # [doc = " Parse either a single identifier (`foo`) or an array of identifiers (`[foo, bar, baz]`)."] fn parse_ident_or_array (input : ParseStream) -> syn :: Result < Vec < Ident > > { if ! input . peek (token :: Bracket) { return Ok (vec ! [input . parse () ?]) ; } parse_ident_array (input) }
}

macro_rules! parse_expr_array_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_expr_array in module {}", module_path!());
    };
}

mkfn!{
    parse_expr_array_introspect!();
    # [doc = " Parse an array of expressions."] fn parse_expr_array (input : ParseStream) -> syn :: Result < Vec < Expr > > { let content ; let _ = bracketed ! (content in input) ; let fields = content . parse_terminated (Expr :: parse , Token ! [,]) ? ; Ok (fields . into_iter () . collect ()) }
}

macro_rules! parse_ident_array_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_ident_array in module {}", module_path!());
    };
}

mkfn!{
    parse_ident_array_introspect!();
    # [doc = " Parse an array of idents, e.g. `[foo, bar, baz]`."] fn parse_ident_array (input : ParseStream) -> syn :: Result < Vec < Ident > > { let content ; let _ = bracketed ! (content in input) ; let fields = content . parse_terminated (Ident :: parse , Token ! [,]) ? ; Ok (fields . into_iter () . collect ()) }
}

macro_rules! parse_ident_pat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_ident_pat in module {}", module_path!());
    };
}

mkfn!{
    parse_ident_pat_introspect!();
    # [doc = " Parse an pattern of idents, specifically `(foo | bar | baz)`."] fn parse_ident_pat (input : ParseStream) -> syn :: Result < Vec < Ident > > { if ! input . peek2 (Token ! [|]) { return Ok (vec ! [input . parse () ?]) ; } let fields = Punctuated :: < Ident , Token ! [|] > :: parse_separated_nonempty (input) ? ; Ok (fields . into_iter () . collect ()) }
}
mkitem!{mkstruct!{# [doc = " A mapping of attributes to identifiers (just a simplified `Expr`)."] # [doc = ""] # [doc = " Expressed as:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[meta1]"] # [doc = " #[meta2]"] # [doc = " [foo, bar, baz]"] # [doc = " ```"] # [derive (Debug)] pub struct AttributeMap { pub meta : Vec < Meta > , pub names : Vec < Ident > , }}}
mkitem!{mkimpl!{impl Parse for AttributeMap { fn parse (input : ParseStream) -> syn :: Result < Self > { let attrs = input . call (Attribute :: parse_outer) ? ; Ok (Self { meta : attrs . into_iter () . map (| a | a . meta) . collect () , names : parse_ident_array (input) ? , }) } }}}