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
mkuse!{use ast :: StaticItem ;}
mkuse!{use itertools :: { Itertools , Position } ;}
mkuse!{use rustc_ast as ast ;}
mkuse!{use rustc_ast :: ModKind ;}
mkuse!{use rustc_span :: Ident ;}
mkuse!{use crate :: pp :: BoxMarker ;}
mkuse!{use crate :: pp :: Breaks :: Inconsistent ;}
mkuse!{use crate :: pprust :: state :: fixup :: FixupContext ;}
mkuse!{use crate :: pprust :: state :: { AnnNode , INDENT_UNIT , PrintState , State } ;}
mkitem!{mkenum!{enum DelegationKind < 'a > { Single , List (& 'a [(Ident , Option < Ident >)]) , Glob , }}}

macro_rules! visibility_qualified_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function visibility_qualified in module {}", module_path!());
    };
}

mkfn!{
    visibility_qualified_introspect!();
    fn visibility_qualified (vis : & ast :: Visibility , s : & str) -> String { format ! ("{}{}" , State :: to_string (| s | s . print_visibility (vis)) , s) }
}
mkitem!{mkimpl!{impl < 'a > State < 'a > { fn print_foreign_mod (& mut self , nmod : & ast :: ForeignMod , attrs : & [ast :: Attribute]) { self . print_inner_attributes (attrs) ; for item in & nmod . items { self . print_foreign_item (item) ; } } pub (crate) fn print_foreign_item (& mut self , item : & ast :: ForeignItem) { let ast :: Item { id , span , ref attrs , ref kind , ref vis , tokens : _ } = * item ; self . ann . pre (self , AnnNode :: SubItem (id)) ; self . hardbreak_if_not_bol () ; self . maybe_print_comment (span . lo ()) ; self . print_outer_attributes (attrs) ; match kind { ast :: ForeignItemKind :: Fn (func) => { self . print_fn_full (vis , attrs , & * func) ; } ast :: ForeignItemKind :: Static (box ast :: StaticItem { ident , ty , mutability , expr , safety , define_opaque , }) => self . print_item_const (* ident , Some (* mutability) , & ast :: Generics :: default () , ty , expr . as_deref () , vis , * safety , ast :: Defaultness :: Final , define_opaque . as_deref () ,) , ast :: ForeignItemKind :: TyAlias (box ast :: TyAlias { defaultness , ident , generics , where_clauses , bounds , ty , }) => { self . print_associated_type (* ident , generics , * where_clauses , bounds , ty . as_deref () , vis , * defaultness ,) ; } ast :: ForeignItemKind :: MacCall (m) => { self . print_mac (m) ; if m . args . need_semicolon () { self . word (";") ; } } } self . ann . post (self , AnnNode :: SubItem (id)) } fn print_item_const (& mut self , ident : Ident , mutbl : Option < ast :: Mutability > , generics : & ast :: Generics , ty : & ast :: Ty , body : Option < & ast :: Expr > , vis : & ast :: Visibility , safety : ast :: Safety , defaultness : ast :: Defaultness , define_opaque : Option < & [(ast :: NodeId , ast :: Path)] > ,) { self . print_define_opaques (define_opaque) ; let (cb , ib) = self . head ("") ; self . print_visibility (vis) ; self . print_safety (safety) ; self . print_defaultness (defaultness) ; let leading = match mutbl { None => "const" , Some (ast :: Mutability :: Not) => "static" , Some (ast :: Mutability :: Mut) => "static mut" , } ; self . word_space (leading) ; self . print_ident (ident) ; self . print_generic_params (& generics . params) ; self . word_space (":") ; self . print_type (ty) ; if body . is_some () { self . space () ; } self . end (ib) ; if let Some (body) = body { self . word_space ("=") ; self . print_expr (body , FixupContext :: default ()) ; } self . print_where_clause (& generics . where_clause) ; self . word (";") ; self . end (cb) ; } fn print_associated_type (& mut self , ident : Ident , generics : & ast :: Generics , where_clauses : ast :: TyAliasWhereClauses , bounds : & ast :: GenericBounds , ty : Option < & ast :: Ty > , vis : & ast :: Visibility , defaultness : ast :: Defaultness ,) { let (before_predicates , after_predicates) = generics . where_clause . predicates . split_at (where_clauses . split) ; let (cb , ib) = self . head ("") ; self . print_visibility (vis) ; self . print_defaultness (defaultness) ; self . word_space ("type") ; self . print_ident (ident) ; self . print_generic_params (& generics . params) ; if ! bounds . is_empty () { self . word_nbsp (":") ; self . print_type_bounds (bounds) ; } self . print_where_clause_parts (where_clauses . before . has_where_token , before_predicates) ; if let Some (ty) = ty { self . space () ; self . word_space ("=") ; self . print_type (ty) ; } self . print_where_clause_parts (where_clauses . after . has_where_token , after_predicates) ; self . word (";") ; self . end (ib) ; self . end (cb) ; } # [doc = " Pretty-prints an item."] pub (crate) fn print_item (& mut self , item : & ast :: Item) { if self . is_sdylib_interface && item . span . is_dummy () { return ; } self . hardbreak_if_not_bol () ; self . maybe_print_comment (item . span . lo ()) ; self . print_outer_attributes (& item . attrs) ; self . ann . pre (self , AnnNode :: Item (item)) ; match & item . kind { ast :: ItemKind :: ExternCrate (orig_name , ident) => { let (cb , ib) = self . head (visibility_qualified (& item . vis , "extern crate")) ; if let & Some (orig_name) = orig_name { self . print_name (orig_name) ; self . space () ; self . word ("as") ; self . space () ; } self . print_ident (* ident) ; self . word (";") ; self . end (ib) ; self . end (cb) ; } ast :: ItemKind :: Use (tree) => { self . print_visibility (& item . vis) ; self . word_nbsp ("use") ; self . print_use_tree (tree) ; self . word (";") ; } ast :: ItemKind :: Static (box StaticItem { ident , ty , safety , mutability : mutbl , expr : body , define_opaque , }) => { self . print_safety (* safety) ; self . print_item_const (* ident , Some (* mutbl) , & ast :: Generics :: default () , ty , body . as_deref () , & item . vis , ast :: Safety :: Default , ast :: Defaultness :: Final , define_opaque . as_deref () ,) ; } ast :: ItemKind :: Const (box ast :: ConstItem { defaultness , ident , generics , ty , expr , define_opaque , }) => { self . print_item_const (* ident , None , generics , ty , expr . as_deref () , & item . vis , ast :: Safety :: Default , * defaultness , define_opaque . as_deref () ,) ; } ast :: ItemKind :: Fn (func) => { self . print_fn_full (& item . vis , & item . attrs , & * func) ; } ast :: ItemKind :: Mod (safety , ident , mod_kind) => { let (cb , ib) = self . head (Self :: to_string (| s | { s . print_visibility (& item . vis) ; s . print_safety (* safety) ; s . word ("mod") ; })) ; self . print_ident (* ident) ; match mod_kind { ModKind :: Loaded (items , ..) => { self . nbsp () ; self . bopen (ib) ; self . print_inner_attributes (& item . attrs) ; for item in items { self . print_item (item) ; } let empty = item . attrs . is_empty () && items . is_empty () ; self . bclose (item . span , empty , cb) ; } ModKind :: Unloaded => { self . word (";") ; self . end (ib) ; self . end (cb) ; } } } ast :: ItemKind :: ForeignMod (nmod) => { let (cb , ib) = self . head (Self :: to_string (| s | { s . print_safety (nmod . safety) ; s . word ("extern") ; })) ; if let Some (abi) = nmod . abi { self . print_token_literal (abi . as_token_lit () , abi . span) ; self . nbsp () ; } self . bopen (ib) ; self . print_foreign_mod (nmod , & item . attrs) ; let empty = item . attrs . is_empty () && nmod . items . is_empty () ; self . bclose (item . span , empty , cb) ; } ast :: ItemKind :: GlobalAsm (asm) => { let (cb , ib) = self . head (visibility_qualified (& item . vis , "global_asm!")) ; self . print_inline_asm (asm) ; self . word (";") ; self . end (ib) ; self . end (cb) ; } ast :: ItemKind :: TyAlias (box ast :: TyAlias { defaultness , ident , generics , where_clauses , bounds , ty , }) => { self . print_associated_type (* ident , generics , * where_clauses , bounds , ty . as_deref () , & item . vis , * defaultness ,) ; } ast :: ItemKind :: Enum (ident , generics , enum_definition) => { self . print_enum_def (enum_definition , generics , * ident , item . span , & item . vis) ; } ast :: ItemKind :: Struct (ident , generics , struct_def) => { let (cb , ib) = self . head (visibility_qualified (& item . vis , "struct")) ; self . print_struct (struct_def , generics , * ident , item . span , true , cb , ib) ; } ast :: ItemKind :: Union (ident , generics , struct_def) => { let (cb , ib) = self . head (visibility_qualified (& item . vis , "union")) ; self . print_struct (struct_def , generics , * ident , item . span , true , cb , ib) ; } ast :: ItemKind :: Impl (ast :: Impl { generics , of_trait , self_ty , items }) => { let (cb , ib) = self . head ("") ; self . print_visibility (& item . vis) ; let impl_generics = | this : & mut Self | { this . word ("impl") ; if generics . params . is_empty () { this . nbsp () ; } else { this . print_generic_params (& generics . params) ; this . space () ; } } ; if let Some (box of_trait) = of_trait { let ast :: TraitImplHeader { defaultness , safety , constness , polarity , ref trait_ref , } = * of_trait ; self . print_defaultness (defaultness) ; self . print_safety (safety) ; impl_generics (self) ; self . print_constness (constness) ; if let ast :: ImplPolarity :: Negative (_) = polarity { self . word ("!") ; } self . print_trait_ref (trait_ref) ; self . space () ; self . word_space ("for") ; } else { impl_generics (self) ; } self . print_type (self_ty) ; self . print_where_clause (& generics . where_clause) ; self . space () ; self . bopen (ib) ; self . print_inner_attributes (& item . attrs) ; for impl_item in items { self . print_assoc_item (impl_item) ; } let empty = item . attrs . is_empty () && items . is_empty () ; self . bclose (item . span , empty , cb) ; } ast :: ItemKind :: Trait (box ast :: Trait { constness , safety , is_auto , ident , generics , bounds , items , }) => { let (cb , ib) = self . head ("") ; self . print_visibility (& item . vis) ; self . print_constness (* constness) ; self . print_safety (* safety) ; self . print_is_auto (* is_auto) ; self . word_nbsp ("trait") ; self . print_ident (* ident) ; self . print_generic_params (& generics . params) ; if ! bounds . is_empty () { self . word_nbsp (":") ; self . print_type_bounds (bounds) ; } self . print_where_clause (& generics . where_clause) ; self . word (" ") ; self . bopen (ib) ; self . print_inner_attributes (& item . attrs) ; for trait_item in items { self . print_assoc_item (trait_item) ; } let empty = item . attrs . is_empty () && items . is_empty () ; self . bclose (item . span , empty , cb) ; } ast :: ItemKind :: TraitAlias (ident , generics , bounds) => { let (cb , ib) = self . head (visibility_qualified (& item . vis , "trait")) ; self . print_ident (* ident) ; self . print_generic_params (& generics . params) ; self . nbsp () ; if ! bounds . is_empty () { self . word_nbsp ("=") ; self . print_type_bounds (bounds) ; } self . print_where_clause (& generics . where_clause) ; self . word (";") ; self . end (ib) ; self . end (cb) ; } ast :: ItemKind :: MacCall (mac) => { self . print_mac (mac) ; if mac . args . need_semicolon () { self . word (";") ; } } ast :: ItemKind :: MacroDef (ident , macro_def) => { self . print_mac_def (macro_def , & ident , item . span , | state | { state . print_visibility (& item . vis) }) ; } ast :: ItemKind :: Delegation (deleg) => self . print_delegation (& item . attrs , & item . vis , & deleg . qself , & deleg . path , DelegationKind :: Single , & deleg . body ,) , ast :: ItemKind :: DelegationMac (deleg) => self . print_delegation (& item . attrs , & item . vis , & deleg . qself , & deleg . prefix , deleg . suffixes . as_ref () . map_or (DelegationKind :: Glob , | s | DelegationKind :: List (s)) , & deleg . body ,) , } self . ann . post (self , AnnNode :: Item (item)) } fn print_enum_def (& mut self , enum_definition : & ast :: EnumDef , generics : & ast :: Generics , ident : Ident , span : rustc_span :: Span , visibility : & ast :: Visibility ,) { let (cb , ib) = self . head (visibility_qualified (visibility , "enum")) ; self . print_ident (ident) ; self . print_generic_params (& generics . params) ; self . print_where_clause (& generics . where_clause) ; self . space () ; self . bopen (ib) ; for v in enum_definition . variants . iter () { self . space_if_not_bol () ; self . maybe_print_comment (v . span . lo ()) ; self . print_outer_attributes (& v . attrs) ; let ib = self . ibox (0) ; self . print_variant (v) ; self . word (",") ; self . end (ib) ; self . maybe_print_trailing_comment (v . span , None) ; } let empty = enum_definition . variants . is_empty () ; self . bclose (span , empty , cb) } pub (crate) fn print_visibility (& mut self , vis : & ast :: Visibility) { match & vis . kind { ast :: VisibilityKind :: Public => self . word_nbsp ("pub") , ast :: VisibilityKind :: Restricted { path , shorthand , .. } => { let path = Self :: to_string (| s | s . print_path (path , false , 0)) ; if * shorthand && (path == "crate" || path == "self" || path == "super") { self . word_nbsp (format ! ("pub({path})")) } else { self . word_nbsp (format ! ("pub(in {path})")) } } ast :: VisibilityKind :: Inherited => { } } } fn print_defaultness (& mut self , defaultness : ast :: Defaultness) { if let ast :: Defaultness :: Default (_) = defaultness { self . word_nbsp ("default") ; } } fn print_struct (& mut self , struct_def : & ast :: VariantData , generics : & ast :: Generics , ident : Ident , span : rustc_span :: Span , print_finalizer : bool , cb : BoxMarker , ib : BoxMarker ,) { self . print_ident (ident) ; self . print_generic_params (& generics . params) ; match & struct_def { ast :: VariantData :: Tuple (..) | ast :: VariantData :: Unit (..) => { if let ast :: VariantData :: Tuple (..) = struct_def { self . popen () ; self . commasep (Inconsistent , struct_def . fields () , | s , field | { s . maybe_print_comment (field . span . lo ()) ; s . print_outer_attributes (& field . attrs) ; s . print_visibility (& field . vis) ; s . print_type (& field . ty) }) ; self . pclose () ; } self . print_where_clause (& generics . where_clause) ; if print_finalizer { self . word (";") ; } self . end (ib) ; self . end (cb) ; } ast :: VariantData :: Struct { fields , .. } => { self . print_where_clause (& generics . where_clause) ; self . nbsp () ; self . bopen (ib) ; let empty = fields . is_empty () ; if ! empty { self . hardbreak_if_not_bol () ; for field in fields { self . hardbreak_if_not_bol () ; self . maybe_print_comment (field . span . lo ()) ; self . print_outer_attributes (& field . attrs) ; self . print_visibility (& field . vis) ; self . print_ident (field . ident . unwrap ()) ; self . word_nbsp (":") ; self . print_type (& field . ty) ; self . word (",") ; } } self . bclose (span , empty , cb) ; } } } pub (crate) fn print_variant (& mut self , v : & ast :: Variant) { let (cb , ib) = self . head ("") ; self . print_visibility (& v . vis) ; let generics = ast :: Generics :: default () ; self . print_struct (& v . data , & generics , v . ident , v . span , false , cb , ib) ; if let Some (d) = & v . disr_expr { self . space () ; self . word_space ("=") ; self . print_expr (& d . value , FixupContext :: default ()) } } pub (crate) fn print_assoc_item (& mut self , item : & ast :: AssocItem) { let ast :: Item { id , span , ref attrs , ref kind , ref vis , tokens : _ } = * item ; self . ann . pre (self , AnnNode :: SubItem (id)) ; self . hardbreak_if_not_bol () ; self . maybe_print_comment (span . lo ()) ; self . print_outer_attributes (attrs) ; match kind { ast :: AssocItemKind :: Fn (func) => { self . print_fn_full (vis , attrs , & * func) ; } ast :: AssocItemKind :: Const (box ast :: ConstItem { defaultness , ident , generics , ty , expr , define_opaque , }) => { self . print_item_const (* ident , None , generics , ty , expr . as_deref () , vis , ast :: Safety :: Default , * defaultness , define_opaque . as_deref () ,) ; } ast :: AssocItemKind :: Type (box ast :: TyAlias { defaultness , ident , generics , where_clauses , bounds , ty , }) => { self . print_associated_type (* ident , generics , * where_clauses , bounds , ty . as_deref () , vis , * defaultness ,) ; } ast :: AssocItemKind :: MacCall (m) => { self . print_mac (m) ; if m . args . need_semicolon () { self . word (";") ; } } ast :: AssocItemKind :: Delegation (deleg) => self . print_delegation (& item . attrs , vis , & deleg . qself , & deleg . path , DelegationKind :: Single , & deleg . body ,) , ast :: AssocItemKind :: DelegationMac (deleg) => self . print_delegation (& item . attrs , vis , & deleg . qself , & deleg . prefix , deleg . suffixes . as_ref () . map_or (DelegationKind :: Glob , | s | DelegationKind :: List (s)) , & deleg . body ,) , } self . ann . post (self , AnnNode :: SubItem (id)) } fn print_delegation (& mut self , attrs : & [ast :: Attribute] , vis : & ast :: Visibility , qself : & Option < Box < ast :: QSelf > > , path : & ast :: Path , kind : DelegationKind < '_ > , body : & Option < Box < ast :: Block > > ,) { let body_cb_ib = body . as_ref () . map (| body | (body , self . head (""))) ; self . print_visibility (vis) ; self . word_nbsp ("reuse") ; if let Some (qself) = qself { self . print_qpath (path , qself , false) ; } else { self . print_path (path , false , 0) ; } match kind { DelegationKind :: Single => { } DelegationKind :: List (suffixes) => { self . word ("::") ; self . word ("{") ; for (i , (ident , rename)) in suffixes . iter () . enumerate () { self . print_ident (* ident) ; if let Some (rename) = rename { self . nbsp () ; self . word_nbsp ("as") ; self . print_ident (* rename) ; } if i != suffixes . len () - 1 { self . word_space (",") ; } } self . word ("}") ; } DelegationKind :: Glob => { self . word ("::") ; self . word ("*") ; } } if let Some ((body , (cb , ib))) = body_cb_ib { self . nbsp () ; self . print_block_with_attrs (body , attrs , cb , ib) ; } else { self . word (";") ; } } fn print_fn_full (& mut self , vis : & ast :: Visibility , attrs : & [ast :: Attribute] , func : & ast :: Fn) { let ast :: Fn { defaultness , ident , generics , sig , contract , body , define_opaque } = func ; self . print_define_opaques (define_opaque . as_deref ()) ; let body_cb_ib = body . as_ref () . map (| body | (body , self . head (""))) ; self . print_visibility (vis) ; self . print_defaultness (* defaultness) ; self . print_fn (& sig . decl , sig . header , Some (* ident) , generics) ; if let Some (contract) = & contract { self . nbsp () ; self . print_contract (contract) ; } if let Some ((body , (cb , ib))) = body_cb_ib { if self . is_sdylib_interface { self . word (";") ; self . end (ib) ; self . end (cb) ; return ; } self . nbsp () ; self . print_block_with_attrs (body , attrs , cb , ib) ; } else { self . word (";") ; } } fn print_define_opaques (& mut self , define_opaque : Option < & [(ast :: NodeId , ast :: Path)] >) { if let Some (define_opaque) = define_opaque { self . word ("#[define_opaque(") ; for (i , (_ , path)) in define_opaque . iter () . enumerate () { if i != 0 { self . word_space (",") ; } self . print_path (path , false , 0) ; } self . word (")]") ; } self . hardbreak_if_not_bol () ; } fn print_contract (& mut self , contract : & ast :: FnContract) { if let Some (pred) = & contract . requires { self . word ("rustc_requires") ; self . popen () ; self . print_expr (pred , FixupContext :: default ()) ; self . pclose () ; } if let Some (pred) = & contract . ensures { self . word ("rustc_ensures") ; self . popen () ; self . print_expr (pred , FixupContext :: default ()) ; self . pclose () ; } } pub (crate) fn print_fn (& mut self , decl : & ast :: FnDecl , header : ast :: FnHeader , ident : Option < Ident > , generics : & ast :: Generics ,) { self . print_fn_header_info (header) ; if let Some (ident) = ident { self . nbsp () ; self . print_ident (ident) ; } self . print_generic_params (& generics . params) ; self . print_fn_params_and_ret (decl , false) ; self . print_where_clause (& generics . where_clause) ; } pub (crate) fn print_fn_params_and_ret (& mut self , decl : & ast :: FnDecl , is_closure : bool) { let (open , close) = if is_closure { ("|" , "|") } else { ("(" , ")") } ; self . word (open) ; self . commasep (Inconsistent , & decl . inputs , | s , param | s . print_param (param , is_closure)) ; self . word (close) ; self . print_fn_ret_ty (& decl . output) } fn print_where_clause (& mut self , where_clause : & ast :: WhereClause) { self . print_where_clause_parts (where_clause . has_where_token , & where_clause . predicates) ; } fn print_where_clause_parts (& mut self , has_where_token : bool , predicates : & [ast :: WherePredicate] ,) { if predicates . is_empty () && ! has_where_token { return ; } self . space () ; self . word_space ("where") ; for (i , predicate) in predicates . iter () . enumerate () { if i != 0 { self . word_space (",") ; } self . print_where_predicate (predicate) ; } } pub fn print_where_predicate (& mut self , predicate : & ast :: WherePredicate) { let ast :: WherePredicate { attrs , kind , id : _ , span : _ , is_placeholder : _ } = predicate ; self . print_outer_attributes (attrs) ; match kind { ast :: WherePredicateKind :: BoundPredicate (where_bound_predicate) => { self . print_where_bound_predicate (where_bound_predicate) ; } ast :: WherePredicateKind :: RegionPredicate (ast :: WhereRegionPredicate { lifetime , bounds , .. }) => { self . print_lifetime (* lifetime) ; self . word (":") ; if ! bounds . is_empty () { self . nbsp () ; self . print_lifetime_bounds (bounds) ; } } ast :: WherePredicateKind :: EqPredicate (ast :: WhereEqPredicate { lhs_ty , rhs_ty , .. }) => { self . print_type (lhs_ty) ; self . space () ; self . word_space ("=") ; self . print_type (rhs_ty) ; } } } pub (crate) fn print_where_bound_predicate (& mut self , where_bound_predicate : & ast :: WhereBoundPredicate ,) { self . print_formal_generic_params (& where_bound_predicate . bound_generic_params) ; self . print_type (& where_bound_predicate . bounded_ty) ; self . word (":") ; if ! where_bound_predicate . bounds . is_empty () { self . nbsp () ; self . print_type_bounds (& where_bound_predicate . bounds) ; } } fn print_use_tree (& mut self , tree : & ast :: UseTree) { match & tree . kind { ast :: UseTreeKind :: Simple (rename) => { self . print_path (& tree . prefix , false , 0) ; if let & Some (rename) = rename { self . nbsp () ; self . word_nbsp ("as") ; self . print_ident (rename) ; } } ast :: UseTreeKind :: Glob => { if ! tree . prefix . segments . is_empty () { self . print_path (& tree . prefix , false , 0) ; self . word ("::") ; } self . word ("*") ; } ast :: UseTreeKind :: Nested { items , .. } => { if ! tree . prefix . segments . is_empty () { self . print_path (& tree . prefix , false , 0) ; self . word ("::") ; } if items . is_empty () { self . word ("{}") ; } else if let [(item , _)] = items . as_slice () { self . print_use_tree (item) ; } else { let cb = self . cbox (INDENT_UNIT) ; self . word ("{") ; self . zerobreak () ; let ib = self . ibox (0) ; for (pos , use_tree) in items . iter () . with_position () { let is_last = matches ! (pos , Position :: Last | Position :: Only) ; self . print_use_tree (& use_tree . 0) ; if ! is_last { self . word (",") ; if let ast :: UseTreeKind :: Nested { .. } = use_tree . 0 . kind { self . hardbreak () ; } else { self . space () ; } } } self . end (ib) ; self . trailing_comma () ; self . offset (- INDENT_UNIT) ; self . word ("}") ; self . end (cb) ; } } } } }}}