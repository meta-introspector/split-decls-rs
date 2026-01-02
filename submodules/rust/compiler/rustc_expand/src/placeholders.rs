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
mkuse!{use rustc_ast :: mut_visit :: * ;}
mkuse!{use rustc_ast :: token :: Delimiter ;}
mkuse!{use rustc_ast :: visit :: AssocCtxt ;}
mkuse!{use rustc_ast :: { self as ast , Safety } ;}
mkuse!{use rustc_data_structures :: fx :: FxHashMap ;}
mkuse!{use rustc_span :: { DUMMY_SP , Ident } ;}
mkuse!{use smallvec :: { SmallVec , smallvec } ;}
mkuse!{use thin_vec :: ThinVec ;}
mkuse!{use crate :: expand :: { AstFragment , AstFragmentKind } ;}

macro_rules! placeholder_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function placeholder in module {}", module_path!());
    };
}

mkfn!{
    placeholder_introspect!();
    pub (crate) fn placeholder (kind : AstFragmentKind , id : ast :: NodeId , vis : Option < ast :: Visibility > ,) -> AstFragment { fn mac_placeholder () -> Box < ast :: MacCall > { Box :: new (ast :: MacCall { path : ast :: Path { span : DUMMY_SP , segments : ThinVec :: new () , tokens : None } , args : Box :: new (ast :: DelimArgs { dspan : ast :: tokenstream :: DelimSpan :: dummy () , delim : Delimiter :: Parenthesis , tokens : ast :: tokenstream :: TokenStream :: new (Vec :: new ()) , }) , }) } let ident = Ident :: dummy () ; let attrs = ast :: AttrVec :: new () ; let vis = vis . unwrap_or (ast :: Visibility { span : DUMMY_SP , kind : ast :: VisibilityKind :: Inherited , tokens : None , }) ; let span = DUMMY_SP ; let expr_placeholder = | | { Box :: new (ast :: Expr { id , span , attrs : ast :: AttrVec :: new () , kind : ast :: ExprKind :: MacCall (mac_placeholder ()) , tokens : None , }) } ; let ty = | | { Box :: new (ast :: Ty { id , kind : ast :: TyKind :: MacCall (mac_placeholder ()) , span , tokens : None }) } ; let pat = | | { Box :: new (ast :: Pat { id , kind : ast :: PatKind :: MacCall (mac_placeholder ()) , span , tokens : None , }) } ; match kind { AstFragmentKind :: Crate => AstFragment :: Crate (ast :: Crate { attrs : Default :: default () , items : Default :: default () , spans : ast :: ModSpans { inner_span : span , .. Default :: default () } , id , is_placeholder : true , }) , AstFragmentKind :: Expr => AstFragment :: Expr (expr_placeholder ()) , AstFragmentKind :: OptExpr => AstFragment :: OptExpr (Some (expr_placeholder ())) , AstFragmentKind :: MethodReceiverExpr => AstFragment :: MethodReceiverExpr (expr_placeholder ()) , AstFragmentKind :: Items => AstFragment :: Items (smallvec ! [Box :: new (ast :: Item { id , span , vis , attrs , kind : ast :: ItemKind :: MacCall (mac_placeholder ()) , tokens : None , })]) , AstFragmentKind :: TraitItems => { AstFragment :: TraitItems (smallvec ! [Box :: new (ast :: AssocItem { id , span , vis , attrs , kind : ast :: AssocItemKind :: MacCall (mac_placeholder ()) , tokens : None , })]) } AstFragmentKind :: ImplItems => AstFragment :: ImplItems (smallvec ! [Box :: new (ast :: AssocItem { id , span , vis , attrs , kind : ast :: AssocItemKind :: MacCall (mac_placeholder ()) , tokens : None , })]) , AstFragmentKind :: TraitImplItems => { AstFragment :: TraitImplItems (smallvec ! [Box :: new (ast :: AssocItem { id , span , vis , attrs , kind : ast :: AssocItemKind :: MacCall (mac_placeholder ()) , tokens : None , })]) } AstFragmentKind :: ForeignItems => { AstFragment :: ForeignItems (smallvec ! [Box :: new (ast :: ForeignItem { id , span , vis , attrs , kind : ast :: ForeignItemKind :: MacCall (mac_placeholder ()) , tokens : None , })]) } AstFragmentKind :: Pat => AstFragment :: Pat (Box :: new (ast :: Pat { id , span , kind : ast :: PatKind :: MacCall (mac_placeholder ()) , tokens : None , })) , AstFragmentKind :: Ty => AstFragment :: Ty (Box :: new (ast :: Ty { id , span , kind : ast :: TyKind :: MacCall (mac_placeholder ()) , tokens : None , })) , AstFragmentKind :: Stmts => AstFragment :: Stmts (smallvec ! [{ let mac = Box :: new (ast :: MacCallStmt { mac : mac_placeholder () , style : ast :: MacStmtStyle :: Braces , attrs : ast :: AttrVec :: new () , tokens : None , }) ; ast :: Stmt { id , span , kind : ast :: StmtKind :: MacCall (mac) } }]) , AstFragmentKind :: Arms => AstFragment :: Arms (smallvec ! [ast :: Arm { attrs : Default :: default () , body : Some (expr_placeholder ()) , guard : None , id , pat : pat () , span , is_placeholder : true , }]) , AstFragmentKind :: ExprFields => AstFragment :: ExprFields (smallvec ! [ast :: ExprField { attrs : Default :: default () , expr : expr_placeholder () , id , ident , is_shorthand : false , span , is_placeholder : true , }]) , AstFragmentKind :: PatFields => AstFragment :: PatFields (smallvec ! [ast :: PatField { attrs : Default :: default () , id , ident , is_shorthand : false , pat : pat () , span , is_placeholder : true , }]) , AstFragmentKind :: GenericParams => AstFragment :: GenericParams (smallvec ! [{ ast :: GenericParam { attrs : Default :: default () , bounds : Default :: default () , id , ident , is_placeholder : true , kind : ast :: GenericParamKind :: Lifetime , colon_span : None , } }]) , AstFragmentKind :: Params => AstFragment :: Params (smallvec ! [ast :: Param { attrs : Default :: default () , id , pat : pat () , span , ty : ty () , is_placeholder : true , }]) , AstFragmentKind :: FieldDefs => AstFragment :: FieldDefs (smallvec ! [ast :: FieldDef { attrs : Default :: default () , id , ident : None , span , ty : ty () , vis , is_placeholder : true , safety : Safety :: Default , default : None , }]) , AstFragmentKind :: Variants => AstFragment :: Variants (smallvec ! [ast :: Variant { attrs : Default :: default () , data : ast :: VariantData :: Struct { fields : Default :: default () , recovered : ast :: Recovered :: No } , disr_expr : None , id , ident , span , vis , is_placeholder : true , }]) , AstFragmentKind :: WherePredicates => { AstFragment :: WherePredicates (smallvec ! [ast :: WherePredicate { attrs : Default :: default () , id , span , kind : ast :: WherePredicateKind :: BoundPredicate (ast :: WhereBoundPredicate { bound_generic_params : Default :: default () , bounded_ty : ty () , bounds : Default :: default () , }) , is_placeholder : true , }]) } } }
}
mkitem!{mkstruct!{#[derive (Default)] pub (crate) struct PlaceholderExpander { expanded_fragments : FxHashMap < ast :: NodeId , AstFragment > , }}}
mkitem!{mkimpl!{impl PlaceholderExpander { pub (crate) fn add (& mut self , id : ast :: NodeId , mut fragment : AstFragment) { fragment . mut_visit_with (self) ; self . expanded_fragments . insert (id , fragment) ; } fn remove (& mut self , id : ast :: NodeId) -> AstFragment { self . expanded_fragments . remove (& id) . unwrap () } }}}
mkitem!{mkimpl!{impl MutVisitor for PlaceholderExpander { fn flat_map_arm (& mut self , arm : ast :: Arm) -> SmallVec < [ast :: Arm ; 1] > { if arm . is_placeholder { self . remove (arm . id) . make_arms () } else { walk_flat_map_arm (self , arm) } } fn flat_map_expr_field (& mut self , field : ast :: ExprField) -> SmallVec < [ast :: ExprField ; 1] > { if field . is_placeholder { self . remove (field . id) . make_expr_fields () } else { walk_flat_map_expr_field (self , field) } } fn flat_map_pat_field (& mut self , fp : ast :: PatField) -> SmallVec < [ast :: PatField ; 1] > { if fp . is_placeholder { self . remove (fp . id) . make_pat_fields () } else { walk_flat_map_pat_field (self , fp) } } fn flat_map_generic_param (& mut self , param : ast :: GenericParam ,) -> SmallVec < [ast :: GenericParam ; 1] > { if param . is_placeholder { self . remove (param . id) . make_generic_params () } else { walk_flat_map_generic_param (self , param) } } fn flat_map_param (& mut self , p : ast :: Param) -> SmallVec < [ast :: Param ; 1] > { if p . is_placeholder { self . remove (p . id) . make_params () } else { walk_flat_map_param (self , p) } } fn flat_map_field_def (& mut self , sf : ast :: FieldDef) -> SmallVec < [ast :: FieldDef ; 1] > { if sf . is_placeholder { self . remove (sf . id) . make_field_defs () } else { walk_flat_map_field_def (self , sf) } } fn flat_map_variant (& mut self , variant : ast :: Variant) -> SmallVec < [ast :: Variant ; 1] > { if variant . is_placeholder { self . remove (variant . id) . make_variants () } else { walk_flat_map_variant (self , variant) } } fn flat_map_where_predicate (& mut self , predicate : ast :: WherePredicate ,) -> SmallVec < [ast :: WherePredicate ; 1] > { if predicate . is_placeholder { self . remove (predicate . id) . make_where_predicates () } else { walk_flat_map_where_predicate (self , predicate) } } fn flat_map_item (& mut self , item : Box < ast :: Item >) -> SmallVec < [Box < ast :: Item > ; 1] > { match item . kind { ast :: ItemKind :: MacCall (_) => self . remove (item . id) . make_items () , _ => walk_flat_map_item (self , item) , } } fn flat_map_assoc_item (& mut self , item : Box < ast :: AssocItem > , ctxt : AssocCtxt ,) -> SmallVec < [Box < ast :: AssocItem > ; 1] > { match item . kind { ast :: AssocItemKind :: MacCall (_) => { let it = self . remove (item . id) ; match ctxt { AssocCtxt :: Trait => it . make_trait_items () , AssocCtxt :: Impl { of_trait : false } => it . make_impl_items () , AssocCtxt :: Impl { of_trait : true } => it . make_trait_impl_items () , } } _ => walk_flat_map_assoc_item (self , item , ctxt) , } } fn flat_map_foreign_item (& mut self , item : Box < ast :: ForeignItem > ,) -> SmallVec < [Box < ast :: ForeignItem > ; 1] > { match item . kind { ast :: ForeignItemKind :: MacCall (_) => self . remove (item . id) . make_foreign_items () , _ => walk_flat_map_foreign_item (self , item) , } } fn visit_expr (& mut self , expr : & mut ast :: Expr) { match expr . kind { ast :: ExprKind :: MacCall (_) => * expr = * self . remove (expr . id) . make_expr () , _ => walk_expr (self , expr) , } } fn visit_method_receiver_expr (& mut self , expr : & mut ast :: Expr) { match expr . kind { ast :: ExprKind :: MacCall (_) => * expr = * self . remove (expr . id) . make_method_receiver_expr () , _ => walk_expr (self , expr) , } } fn filter_map_expr (& mut self , expr : Box < ast :: Expr >) -> Option < Box < ast :: Expr > > { match expr . kind { ast :: ExprKind :: MacCall (_) => self . remove (expr . id) . make_opt_expr () , _ => walk_filter_map_expr (self , expr) , } } fn flat_map_stmt (& mut self , stmt : ast :: Stmt) -> SmallVec < [ast :: Stmt ; 1] > { let (style , mut stmts) = match stmt . kind { ast :: StmtKind :: MacCall (mac) => (mac . style , self . remove (stmt . id) . make_stmts ()) , _ => return walk_flat_map_stmt (self , stmt) , } ; if style == ast :: MacStmtStyle :: Semicolon { let empty_stmt = ast :: Stmt { id : ast :: DUMMY_NODE_ID , kind : ast :: StmtKind :: Empty , span : DUMMY_SP } ; if let Some (stmt) = stmts . pop () { if stmt . has_trailing_semicolon () { stmts . push (stmt) ; stmts . push (empty_stmt) ; } else { stmts . push (stmt . add_trailing_semicolon ()) ; } } else { stmts . push (empty_stmt) ; } } stmts } fn visit_pat (& mut self , pat : & mut ast :: Pat) { match pat . kind { ast :: PatKind :: MacCall (_) => * pat = * self . remove (pat . id) . make_pat () , _ => walk_pat (self , pat) , } } fn visit_ty (& mut self , ty : & mut ast :: Ty) { match ty . kind { ast :: TyKind :: MacCall (_) => * ty = * self . remove (ty . id) . make_ty () , _ => walk_ty (self , ty) , } } fn visit_crate (& mut self , krate : & mut ast :: Crate) { if krate . is_placeholder { * krate = self . remove (krate . id) . make_crate () ; } else { walk_crate (self , krate) } } }}}