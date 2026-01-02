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
mkuse!{use rustc_errors :: DiagArgFromDisplay ;}
mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_macros :: { Diagnostic , Subdiagnostic } ;}
mkuse!{use rustc_span :: { Ident , Span , Symbol } ;}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_generic_type_with_parentheses , code = E0214)] pub (crate) struct GenericTypeWithParentheses { # [primary_span] # [label] pub span : Span , # [subdiagnostic] pub sub : Option < UseAngleBrackets > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [multipart_suggestion (ast_lowering_use_angle_brackets , applicability = "maybe-incorrect")] pub (crate) struct UseAngleBrackets { # [suggestion_part (code = "<")] pub open_param : Span , # [suggestion_part (code = ">")] pub close_param : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_invalid_abi , code = E0703)] # [note] pub (crate) struct InvalidAbi { # [primary_span] # [label] pub span : Span , pub abi : Symbol , pub command : String , # [subdiagnostic] pub suggestion : Option < InvalidAbiSuggestion > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_default_field_in_tuple)] pub (crate) struct TupleStructWithDefault { # [primary_span] # [label] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [suggestion (ast_lowering_invalid_abi_suggestion , code = "\"{suggestion}\"" , applicability = "maybe-incorrect" , style = "verbose")] pub (crate) struct InvalidAbiSuggestion { # [primary_span] pub span : Span , pub suggestion : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_assoc_ty_parentheses)] pub (crate) struct AssocTyParentheses { # [primary_span] pub span : Span , # [subdiagnostic] pub sub : AssocTyParenthesesSub , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum AssocTyParenthesesSub { # [multipart_suggestion (ast_lowering_remove_parentheses)] Empty { # [suggestion_part (code = "")] parentheses_span : Span , } , # [multipart_suggestion (ast_lowering_use_angle_brackets)] NotEmpty { # [suggestion_part (code = "<")] open_param : Span , # [suggestion_part (code = ">")] close_param : Span , } , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_misplaced_impl_trait , code = E0562)] # [note] pub (crate) struct MisplacedImplTrait < 'a > { # [primary_span] pub span : Span , pub position : DiagArgFromDisplay < 'a > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_assoc_ty_binding_in_dyn)] pub (crate) struct MisplacedAssocTyBinding { # [primary_span] pub span : Span , # [suggestion (code = " = impl" , applicability = "maybe-incorrect" , style = "verbose")] pub suggestion : Option < Span > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_underscore_expr_lhs_assign)] pub (crate) struct UnderscoreExprLhsAssign { # [primary_span] # [label] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_await_only_in_async_fn_and_blocks , code = E0728)] pub (crate) struct AwaitOnlyInAsyncFnAndBlocks { # [primary_span] # [label] pub await_kw_span : Span , # [label (ast_lowering_this_not_async)] pub item_span : Option < Span > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_coroutine_too_many_parameters , code = E0628)] pub (crate) struct CoroutineTooManyParameters { # [primary_span] pub fn_decl_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_closure_cannot_be_static , code = E0697)] pub (crate) struct ClosureCannotBeStatic { # [primary_span] pub fn_decl_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_functional_record_update_destructuring_assignment)] pub (crate) struct FunctionalRecordUpdateDestructuringAssignment { # [primary_span] # [suggestion (code = "" , applicability = "machine-applicable")] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_async_coroutines_not_supported , code = E0727)] pub (crate) struct AsyncCoroutinesNotSupported { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_inline_asm_unsupported_target , code = E0472)] pub (crate) struct InlineAsmUnsupportedTarget { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_att_syntax_only_x86)] pub (crate) struct AttSyntaxOnlyX86 { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_abi_specified_multiple_times)] pub (crate) struct AbiSpecifiedMultipleTimes { # [primary_span] pub abi_span : Span , pub prev_name : Symbol , # [label] pub prev_span : Span , # [note] pub equivalent : bool , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_clobber_abi_not_supported)] pub (crate) struct ClobberAbiNotSupported { # [primary_span] pub abi_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [note] # [diag (ast_lowering_invalid_abi_clobber_abi)] pub (crate) struct InvalidAbiClobberAbi { # [primary_span] pub abi_span : Span , pub supported_abis : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_invalid_register)] pub (crate) struct InvalidRegister < 'a > { # [primary_span] pub op_span : Span , pub reg : Symbol , pub error : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [note] # [diag (ast_lowering_invalid_register_class)] pub (crate) struct InvalidRegisterClass { # [primary_span] pub op_span : Span , pub reg_class : Symbol , pub supported_register_classes : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_invalid_asm_template_modifier_reg_class)] pub (crate) struct InvalidAsmTemplateModifierRegClass { # [primary_span] # [label (ast_lowering_template_modifier)] pub placeholder_span : Span , # [label (ast_lowering_argument)] pub op_span : Span , # [subdiagnostic] pub sub : InvalidAsmTemplateModifierRegClassSub , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum InvalidAsmTemplateModifierRegClassSub { # [note (ast_lowering_support_modifiers)] SupportModifier { class_name : Symbol , modifiers : String } , # [note (ast_lowering_does_not_support_modifiers)] DoesNotSupportModifier { class_name : Symbol } , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_invalid_asm_template_modifier_const)] pub (crate) struct InvalidAsmTemplateModifierConst { # [primary_span] # [label (ast_lowering_template_modifier)] pub placeholder_span : Span , # [label (ast_lowering_argument)] pub op_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_invalid_asm_template_modifier_sym)] pub (crate) struct InvalidAsmTemplateModifierSym { # [primary_span] # [label (ast_lowering_template_modifier)] pub placeholder_span : Span , # [label (ast_lowering_argument)] pub op_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_invalid_asm_template_modifier_label)] pub (crate) struct InvalidAsmTemplateModifierLabel { # [primary_span] # [label (ast_lowering_template_modifier)] pub placeholder_span : Span , # [label (ast_lowering_argument)] pub op_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_register_class_only_clobber)] pub (crate) struct RegisterClassOnlyClobber { # [primary_span] pub op_span : Span , pub reg_class_name : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_register_class_only_clobber_stable)] pub (crate) struct RegisterClassOnlyClobberStable { # [primary_span] pub op_span : Span , pub reg_class_name : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_register_conflict)] pub (crate) struct RegisterConflict < 'a > { # [primary_span] # [label (ast_lowering_register1)] pub op_span1 : Span , # [label (ast_lowering_register2)] pub op_span2 : Span , pub reg1_name : & 'a str , pub reg2_name : & 'a str , # [help] pub in_out : Option < Span > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [help] # [diag (ast_lowering_sub_tuple_binding)] pub (crate) struct SubTupleBinding < 'a > { # [primary_span] # [label] # [suggestion (ast_lowering_sub_tuple_binding_suggestion , style = "verbose" , code = ".." , applicability = "maybe-incorrect")] pub span : Span , pub ident : Ident , pub ident_name : Symbol , pub ctx : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_extra_double_dot)] pub (crate) struct ExtraDoubleDot < 'a > { # [primary_span] # [label] pub span : Span , # [label (ast_lowering_previously_used_here)] pub prev_span : Span , pub ctx : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [note] # [diag (ast_lowering_misplaced_double_dot)] pub (crate) struct MisplacedDoubleDot { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_match_arm_with_no_body)] pub (crate) struct MatchArmWithNoBody { # [primary_span] pub span : Span , # [suggestion (code = " => todo!()," , applicability = "has-placeholders")] pub suggestion : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_never_pattern_with_body)] pub (crate) struct NeverPatternWithBody { # [primary_span] # [label] # [suggestion (code = "" , applicability = "maybe-incorrect")] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_never_pattern_with_guard)] pub (crate) struct NeverPatternWithGuard { # [primary_span] # [suggestion (code = "" , applicability = "maybe-incorrect")] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_arbitrary_expression_in_pattern)] pub (crate) struct ArbitraryExpressionInPattern { # [primary_span] pub span : Span , # [note (ast_lowering_pattern_from_macro_note)] pub pattern_from_macro_note : bool , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_inclusive_range_with_no_end)] pub (crate) struct InclusiveRangeWithNoEnd { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [multipart_suggestion (ast_lowering_bad_return_type_notation_output_suggestion , applicability = "machine-applicable" , style = "verbose")] # [doc = " Given `T: Tr<m() -> Ret>` or `T: Tr<m(Ty) -> Ret>`, suggest `T: Tr<m(..)>`."] pub (crate) struct RTNSuggestion { # [suggestion_part (code = "")] pub output : Span , # [suggestion_part (code = "(..)")] pub input : Span , }}}
mkitem!{mkenum!{# [derive (Diagnostic)] pub (crate) enum BadReturnTypeNotation { # [diag (ast_lowering_bad_return_type_notation_inputs)] Inputs { # [primary_span] # [suggestion (code = "(..)" , applicability = "machine-applicable" , style = "verbose")] span : Span , } , # [diag (ast_lowering_bad_return_type_notation_output)] Output { # [primary_span] span : Span , # [subdiagnostic] suggestion : RTNSuggestion , } , # [diag (ast_lowering_bad_return_type_notation_needs_dots)] NeedsDots { # [primary_span] # [suggestion (code = "(..)" , applicability = "machine-applicable" , style = "verbose")] span : Span , } , # [diag (ast_lowering_bad_return_type_notation_position)] Position { # [primary_span] span : Span , } , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_generic_param_default_in_binder)] pub (crate) struct GenericParamDefaultInBinder { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_async_bound_not_on_trait)] pub (crate) struct AsyncBoundNotOnTrait { # [primary_span] pub span : Span , pub descr : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_async_bound_only_for_fn_traits)] pub (crate) struct AsyncBoundOnlyForFnTraits { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_no_precise_captures_on_apit)] pub (crate) struct NoPreciseCapturesOnApit { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_yield_in_closure)] pub (crate) struct YieldInClosure { # [primary_span] pub span : Span , # [suggestion (code = "#[coroutine] " , applicability = "maybe-incorrect" , style = "verbose")] pub suggestion : Option < Span > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_invalid_legacy_const_generic_arg)] pub (crate) struct InvalidLegacyConstGenericArg { # [primary_span] pub span : Span , # [subdiagnostic] pub suggestion : UseConstGenericArg , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [multipart_suggestion (ast_lowering_invalid_legacy_const_generic_arg_suggestion , applicability = "maybe-incorrect")] pub (crate) struct UseConstGenericArg { # [suggestion_part (code = "::<{const_args}>")] pub end_of_fn : Span , pub const_args : String , pub other_args : String , # [suggestion_part (code = "{other_args}")] pub call_args : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (ast_lowering_union_default_field_values)] pub (crate) struct UnionWithDefault { # [primary_span] pub span : Span , }}}