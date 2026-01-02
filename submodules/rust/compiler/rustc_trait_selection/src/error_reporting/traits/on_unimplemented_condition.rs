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
mkuse!{use rustc_ast :: { MetaItemInner , MetaItemKind , MetaItemLit } ;}
mkuse!{use rustc_parse_format :: { ParseMode , Parser , Piece , Position } ;}
mkuse!{use rustc_span :: { DesugaringKind , Ident , Span , Symbol , kw , sym } ;}
mkuse!{use crate :: errors :: InvalidOnClause ;}
mkitem!{mkstruct!{# [doc = " Represents the `on` filter in `#[rustc_on_unimplemented]`."] # [derive (Debug)] pub (crate) struct OnUnimplementedCondition { span : Span , pred : Predicate , }}}
mkitem!{mkimpl!{impl OnUnimplementedCondition { pub (crate) fn span (& self) -> Span { self . span } pub (crate) fn matches_predicate (& self , options : & ConditionOptions) -> bool { self . pred . eval (& mut | p | match p { FlagOrNv :: Flag (b) => options . has_flag (* b) , FlagOrNv :: NameValue (NameValue { name , value }) => { let value = value . format (& options . generic_args) ; options . contains (* name , value) } }) } pub (crate) fn parse (input : & MetaItemInner , generics : & [Symbol] ,) -> Result < Self , InvalidOnClause > { let span = input . span () ; let pred = Predicate :: parse (input , generics) ? ; Ok (OnUnimplementedCondition { span , pred }) } }}}
mkitem!{mkenum!{# [doc = " Predicate(s) in `#[rustc_on_unimplemented]`'s `on` filter. See [`OnUnimplementedCondition`]."] # [doc = ""] # [doc = " It is similar to the predicate in the `cfg` attribute,"] # [doc = " and may contain nested predicates."] # [derive (Debug)] enum Predicate { # [doc = " A condition like `on(crate_local)`."] Flag (Flag) , # [doc = " A match, like `on(Rhs = \"Whatever\")`."] Match (NameValue) , # [doc = " Negation, like `on(not($pred))`."] Not (Box < Predicate >) , # [doc = " True if all predicates are true, like `on(all($a, $b, $c))`."] All (Vec < Predicate >) , # [doc = " True if any predicate is true, like `on(any($a, $b, $c))`."] Any (Vec < Predicate >) , }}}
mkitem!{mkimpl!{impl Predicate { fn parse (input : & MetaItemInner , generics : & [Symbol]) -> Result < Self , InvalidOnClause > { let meta_item = match input { MetaItemInner :: MetaItem (meta_item) => meta_item , MetaItemInner :: Lit (lit) => { return Err (InvalidOnClause :: UnsupportedLiteral { span : lit . span }) ; } } ; let Some (predicate) = meta_item . ident () else { return Err (InvalidOnClause :: ExpectedIdentifier { span : meta_item . path . span , path : meta_item . path . clone () , }) ; } ; match meta_item . kind { MetaItemKind :: List (ref mis) => match predicate . name { sym :: any => Ok (Predicate :: Any (Predicate :: parse_sequence (mis , generics) ?)) , sym :: all => Ok (Predicate :: All (Predicate :: parse_sequence (mis , generics) ?)) , sym :: not => match & * * mis { [one] => Ok (Predicate :: Not (Box :: new (Predicate :: parse (one , generics) ?))) , [first , .. , last] => Err (InvalidOnClause :: ExpectedOnePredInNot { span : first . span () . to (last . span ()) , }) , [] => Err (InvalidOnClause :: ExpectedOnePredInNot { span : meta_item . span }) , } , invalid_pred => { Err (InvalidOnClause :: InvalidPredicate { span : predicate . span , invalid_pred }) } } , MetaItemKind :: NameValue (MetaItemLit { symbol , .. }) => { let name = Name :: parse (predicate , generics) ? ; let value = FilterFormatString :: parse (symbol) ; let kv = NameValue { name , value } ; Ok (Predicate :: Match (kv)) } MetaItemKind :: Word => { let flag = Flag :: parse (predicate) ? ; Ok (Predicate :: Flag (flag)) } } } fn parse_sequence (sequence : & [MetaItemInner] , generics : & [Symbol] ,) -> Result < Vec < Self > , InvalidOnClause > { sequence . iter () . map (| item | Predicate :: parse (item , generics)) . collect () } fn eval (& self , eval : & mut impl FnMut (FlagOrNv < '_ >) -> bool) -> bool { match self { Predicate :: Flag (flag) => eval (FlagOrNv :: Flag (flag)) , Predicate :: Match (nv) => eval (FlagOrNv :: NameValue (nv)) , Predicate :: Not (not) => ! not . eval (eval) , Predicate :: All (preds) => preds . into_iter () . all (| pred | pred . eval (eval)) , Predicate :: Any (preds) => preds . into_iter () . any (| pred | pred . eval (eval)) , } } }}}
mkitem!{mkenum!{# [doc = " Represents a `MetaWord` in an `on`-filter."] # [derive (Debug , Clone , Copy)] enum Flag { # [doc = " Whether the code causing the trait bound to not be fulfilled"] # [doc = " is part of the user's crate."] CrateLocal , # [doc = " Whether the obligation is user-specified rather than derived."] Direct , # [doc = " Whether we are in some kind of desugaring like"] # [doc = " `?` or `try { .. }`."] FromDesugaring , }}}
mkitem!{mkimpl!{impl Flag { fn parse (Ident { name , span } : Ident) -> Result < Self , InvalidOnClause > { match name { sym :: crate_local => Ok (Flag :: CrateLocal) , sym :: direct => Ok (Flag :: Direct) , sym :: from_desugaring => Ok (Flag :: FromDesugaring) , invalid_flag => Err (InvalidOnClause :: InvalidFlag { invalid_flag , span }) , } } }}}
mkitem!{mkstruct!{# [doc = " A `MetaNameValueStr` in an `on`-filter."] # [doc = ""] # [doc = " For example, `#[rustc_on_unimplemented(on(name = \"value\", message = \"hello\"))]`."] # [derive (Debug , Clone)] struct NameValue { name : Name , # [doc = " Something like `\"&str\"` or `\"alloc::string::String\"`,"] # [doc = " in which case it just contains a single string piece."] # [doc = " But if it is something like `\"&[{A}]\"` then it must be formatted later."] value : FilterFormatString , }}}
mkitem!{mkenum!{# [doc = " The valid names of the `on` filter."] # [derive (Debug , Clone , Copy)] enum Name { Cause , FromDesugaring , SelfUpper , GenericArg (Symbol) , }}}
mkitem!{mkimpl!{impl Name { fn parse (Ident { name , span } : Ident , generics : & [Symbol]) -> Result < Self , InvalidOnClause > { match name { kw :: SelfUpper => Ok (Name :: SelfUpper) , sym :: from_desugaring => Ok (Name :: FromDesugaring) , sym :: cause => Ok (Name :: Cause) , generic if generics . contains (& generic) => Ok (Name :: GenericArg (generic)) , invalid_name => Err (InvalidOnClause :: InvalidName { invalid_name , span }) , } } }}}
mkitem!{mkenum!{# [derive (Debug , Clone)] enum FlagOrNv < 'p > { Flag (& 'p Flag) , NameValue (& 'p NameValue) , }}}
mkitem!{mkstruct!{# [doc = " Represents a value inside an `on` filter."] # [doc = ""] # [doc = " For example, `#[rustc_on_unimplemented(on(name = \"value\", message = \"hello\"))]`."] # [doc = " If it is a simple literal like this then `pieces` will be `[LitOrArg::Lit(\"value\")]`."] # [doc = " The `Arg` variant is used when it contains formatting like"] # [doc = " `#[rustc_on_unimplemented(on(Self = \"&[{A}]\", message = \"hello\"))]`."] # [derive (Debug , Clone)] struct FilterFormatString { pieces : Vec < LitOrArg > , }}}
mkitem!{mkenum!{# [derive (Debug , Clone)] enum LitOrArg { Lit (String) , Arg (String) , }}}
mkitem!{mkimpl!{impl FilterFormatString { fn parse (input : Symbol) -> Self { let pieces = Parser :: new (input . as_str () , None , None , false , ParseMode :: Diagnostic) . map (| p | match p { Piece :: Lit (s) => LitOrArg :: Lit (s . to_owned ()) , Piece :: NextArgument (a) => match a . position { Position :: ArgumentNamed (arg @ "integer" | arg @ "integral" | arg @ "float") => { LitOrArg :: Lit (format ! ("{{{arg}}}")) } Position :: ArgumentNamed (arg) => LitOrArg :: Arg (arg . to_owned ()) , Position :: ArgumentImplicitlyIs (_) => LitOrArg :: Lit (String :: from ("{}")) , Position :: ArgumentIs (idx) => LitOrArg :: Lit (format ! ("{{{idx}}}")) , } , }) . collect () ; Self { pieces } } fn format (& self , generic_args : & [(Symbol , String)]) -> String { let mut ret = String :: new () ; for piece in & self . pieces { match piece { LitOrArg :: Lit (s) => ret . push_str (s) , LitOrArg :: Arg (arg) => { let s = Symbol :: intern (arg) ; match generic_args . iter () . find (| (k , _) | * k == s) { Some ((_ , val)) => ret . push_str (val) , None => { let _ = std :: fmt :: write (& mut ret , format_args ! ("{{{s}}}")) ; } } } } } ret } }}}
mkitem!{mkstruct!{# [doc = " Used with `OnUnimplementedCondition::matches_predicate` to evaluate the"] # [doc = " [`OnUnimplementedCondition`]."] # [doc = ""] # [doc = " For example, given a"] # [doc = " ```rust,ignore (just an example)"] # [doc = " #[rustc_on_unimplemented("] # [doc = "     on(all(from_desugaring = \"QuestionMark\"),"] # [doc = "         message = \"the `?` operator can only be used in {ItemContext} \\"] # [doc = "                     that returns `Result` or `Option` \\"] # [doc = "                     (or another type that implements `{FromResidual}`)\","] # [doc = "         label = \"cannot use the `?` operator in {ItemContext} that returns `{Self}`\","] # [doc = "         parent_label = \"this function should return `Result` or `Option` to accept `?`\""] # [doc = "     ),"] # [doc = " )]"] # [doc = " pub trait FromResidual<R = <Self as Try>::Residual> {"] # [doc = "    ..."] # [doc = " }"] # [doc = ""] # [doc = " async fn an_async_function() -> u32 {"] # [doc = "     let x: Option<u32> = None;"] # [doc = "     x?; //~ ERROR the `?` operator"] # [doc = "     22"] # [doc = " }"] # [doc = "  ```"] # [doc = " it will look like this:"] # [doc = ""] # [doc = " ```rust,ignore (just an example)"] # [doc = " ConditionOptions {"] # [doc = "     self_types: [\"u32\", \"{integral}\"],"] # [doc = "     from_desugaring: Some(\"QuestionMark\"),"] # [doc = "     cause: None,"] # [doc = "     crate_local: false,"] # [doc = "     direct: true,"] # [doc = "     generic_args: [(\"Self\",\"u32\"),"] # [doc = "         (\"R\", \"core::option::Option<core::convert::Infallible>\"),"] # [doc = "         (\"R\", \"core::option::Option<T>\" ),"] # [doc = "     ],"] # [doc = " }"] # [doc = " ```"] # [derive (Debug)] pub (crate) struct ConditionOptions { # [doc = " All the self types that may apply."] pub (crate) self_types : Vec < String > , pub (crate) from_desugaring : Option < DesugaringKind > , # [doc = " Match on a variant of [rustc_infer::traits::ObligationCauseCode]."] pub (crate) cause : Option < String > , pub (crate) crate_local : bool , # [doc = " Is the obligation \"directly\" user-specified, rather than derived?"] pub (crate) direct : bool , pub (crate) generic_args : Vec < (Symbol , String) > , }}}
mkitem!{mkimpl!{impl ConditionOptions { fn has_flag (& self , name : Flag) -> bool { match name { Flag :: CrateLocal => self . crate_local , Flag :: Direct => self . direct , Flag :: FromDesugaring => self . from_desugaring . is_some () , } } fn contains (& self , name : Name , value : String) -> bool { match name { Name :: SelfUpper => self . self_types . contains (& value) , Name :: FromDesugaring => self . from_desugaring . is_some_and (| ds | ds . matches (& value)) , Name :: Cause => self . cause == Some (value) , Name :: GenericArg (arg) => self . generic_args . contains (& (arg , value)) , } } }}}