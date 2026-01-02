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
mkuse!{use std :: fmt ;}
mkuse!{use std :: iter :: Peekable ;}
mkuse!{use std :: sync :: atomic :: { AtomicU32 , Ordering } ;}
mkuse!{use super :: { Byte , Reference , Region , Tree , Type , Uninhabited } ;}
mkuse!{use crate :: { Map , Set } ;}
mkitem!{mkstruct!{# [derive (PartialEq)] # [cfg_attr (test , derive (Clone))] pub (crate) struct Dfa < R , T > where R : Region , T : Type , { pub (crate) transitions : Map < State , Transitions < R , T > > , pub (crate) start : State , pub (crate) accept : State , }}}
mkitem!{mkstruct!{# [derive (PartialEq , Clone , Debug)] pub (crate) struct Transitions < R , T > where R : Region , T : Type , { byte_transitions : EdgeSet < State > , ref_transitions : Map < Reference < R , T > , State > , }}}
mkitem!{mkimpl!{impl < R , T > Default for Transitions < R , T > where R : Region , T : Type , { fn default () -> Self { Self { byte_transitions : EdgeSet :: empty () , ref_transitions : Map :: default () } } }}}
mkitem!{mkstruct!{# [doc = " The states in a [`Dfa`] represent byte offsets."] # [derive (Hash , Eq , PartialEq , PartialOrd , Ord , Copy , Clone)] pub (crate) struct State (pub (crate) u32) ;}}
mkitem!{mkimpl!{impl State { pub (crate) fn new () -> Self { static COUNTER : AtomicU32 = AtomicU32 :: new (0) ; Self (COUNTER . fetch_add (1 , Ordering :: SeqCst)) } }}}
mkitem!{mkimpl!{impl fmt :: Debug for State { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "S_{}" , self . 0) } }}}
mkitem!{mkimpl!{impl < R , T > Dfa < R , T > where R : Region , T : Type , { # [cfg (test)] pub (crate) fn bool () -> Self { Self :: from_transitions (| accept | Transitions { byte_transitions : EdgeSet :: new (Byte :: new (0x00 ..= 0x01) , accept) , ref_transitions : Map :: default () , }) } pub (crate) fn unit () -> Self { let transitions : Map < State , Transitions < R , T > > = Map :: default () ; let start = State :: new () ; let accept = start ; Self { transitions , start , accept } } pub (crate) fn from_byte (byte : Byte) -> Self { Self :: from_transitions (| accept | Transitions { byte_transitions : EdgeSet :: new (byte , accept) , ref_transitions : Map :: default () , }) } pub (crate) fn from_ref (r : Reference < R , T >) -> Self { Self :: from_transitions (| accept | Transitions { byte_transitions : EdgeSet :: empty () , ref_transitions : [(r , accept)] . into_iter () . collect () , }) } fn from_transitions (f : impl FnOnce (State) -> Transitions < R , T >) -> Self { let start = State :: new () ; let accept = State :: new () ; Self { transitions : [(start , f (accept))] . into_iter () . collect () , start , accept } } pub (crate) fn from_tree (tree : Tree < ! , R , T >) -> Result < Self , Uninhabited > { Ok (match tree { Tree :: Byte (b) => Self :: from_byte (b) , Tree :: Ref (r) => Self :: from_ref (r) , Tree :: Alt (alts) => { let mut alts = alts . into_iter () . map (Self :: from_tree) . filter_map (Result :: ok) ; let dfa = alts . next () . ok_or (Uninhabited) ? ; alts . fold (dfa , | dfa , alt | dfa . union (alt , State :: new)) } Tree :: Seq (elts) => { let mut dfa = Self :: unit () ; for elt in elts . into_iter () . map (Self :: from_tree) { dfa = dfa . concat (elt ?) ; } dfa } }) } # [doc = " Concatenate two `Dfa`s."] pub (crate) fn concat (self , other : Self) -> Self { if self . start == self . accept { return other ; } else if other . start == other . accept { return self ; } let start = self . start ; let accept = other . accept ; let mut transitions : Map < State , Transitions < R , T > > = self . transitions ; for (source , transition) in other . transitions { let fix_state = | state | if state == other . start { self . accept } else { state } ; let byte_transitions = transition . byte_transitions . map_states (& fix_state) ; let ref_transitions = transition . ref_transitions . into_iter () . map (| (r , state) | (r , fix_state (state))) . collect () ; let old = transitions . insert (fix_state (source) , Transitions { byte_transitions , ref_transitions }) ; assert ! (old . is_none ()) ; } Self { transitions , start , accept } } # [doc = " Compute the union of two `Dfa`s."] pub (crate) fn union (self , other : Self , mut new_state : impl FnMut () -> State) -> Self { let a = self ; let b = other ; let accept = new_state () ; let mut mapping : Map < (Option < State > , Option < State >) , State > = Map :: default () ; let mut mapped = | (a_state , b_state) | { if Some (a . accept) == a_state || Some (b . accept) == b_state { accept } else { * mapping . entry ((a_state , b_state)) . or_insert_with (& mut new_state) } } ; let start = mapped ((Some (a . start) , Some (b . start))) ; let mut transitions : Map < State , Transitions < R , T > > = Map :: default () ; let empty_transitions = Transitions :: default () ; struct WorkQueue { queue : Vec < (Option < State > , Option < State >) > , enqueued : Set < (Option < State > , Option < State >) > , } impl WorkQueue { fn enqueue (& mut self , a_state : Option < State > , b_state : Option < State >) { if self . enqueued . insert ((a_state , b_state)) { self . queue . push ((a_state , b_state)) ; } } } let mut queue = WorkQueue { queue : Vec :: new () , enqueued : Set :: default () } ; queue . enqueue (Some (a . start) , Some (b . start)) ; while let Some ((a_src , b_src)) = queue . queue . pop () { let src = mapped ((a_src , b_src)) ; if src == accept { continue ; } let a_transitions = a_src . and_then (| a_src | a . transitions . get (& a_src)) . unwrap_or (& empty_transitions) ; let b_transitions = b_src . and_then (| b_src | b . transitions . get (& b_src)) . unwrap_or (& empty_transitions) ; let byte_transitions = a_transitions . byte_transitions . union (& b_transitions . byte_transitions , | a_dst , b_dst | { assert ! (a_dst . is_some () || b_dst . is_some ()) ; queue . enqueue (a_dst , b_dst) ; mapped ((a_dst , b_dst)) } ,) ; let ref_transitions = a_transitions . ref_transitions . keys () . chain (b_transitions . ref_transitions . keys ()) ; let ref_transitions = ref_transitions . map (| ref_transition | { let a_dst = a_transitions . ref_transitions . get (ref_transition) . copied () ; let b_dst = b_transitions . ref_transitions . get (ref_transition) . copied () ; assert ! (a_dst . is_some () || b_dst . is_some ()) ; queue . enqueue (a_dst , b_dst) ; (* ref_transition , mapped ((a_dst , b_dst))) }) . collect () ; let old = transitions . insert (src , Transitions { byte_transitions , ref_transitions }) ; assert_eq ! (old , None) ; } Self { transitions , start , accept } } pub (crate) fn get_uninit_edge_dst (& self , state : State) -> Option < State > { let transitions = self . transitions . get (& state) ? ; transitions . byte_transitions . get_uninit_edge_dst () } pub (crate) fn bytes_from (& self , start : State) -> impl Iterator < Item = (Byte , State) > { self . transitions . get (& start) . into_iter () . flat_map (| transitions | transitions . byte_transitions . iter ()) } pub (crate) fn refs_from (& self , start : State) -> impl Iterator < Item = (Reference < R , T > , State) > { self . transitions . get (& start) . into_iter () . flat_map (| transitions | transitions . ref_transitions . iter ()) . map (| (r , s) | (* r , * s)) } # [cfg (test)] pub (crate) fn from_edges < B : Clone + Into < Byte > > (start : u32 , accept : u32 , edges : & [(u32 , B , u32)] ,) -> Self { let start = State (start) ; let accept = State (accept) ; let mut transitions : Map < State , Vec < (Byte , State) > > = Map :: default () ; for & (src , ref edge , dst) in edges . iter () { transitions . entry (State (src)) . or_default () . push ((edge . clone () . into () , State (dst))) ; } let transitions = transitions . into_iter () . map (| (src , edges) | { (src , Transitions { byte_transitions : EdgeSet :: from_edges (edges) , ref_transitions : Map :: default () , } ,) }) . collect () ; Self { start , accept , transitions } } }}}
mkitem!{mkimpl!{# [doc = " Serialize the DFA using the Graphviz DOT format."] impl < R , T > fmt :: Debug for Dfa < R , T > where R : Region , T : Type , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { writeln ! (f , "digraph {{") ? ; writeln ! (f , "    {:?} [shape = doublecircle]" , self . start) ? ; writeln ! (f , "    {:?} [shape = doublecircle]" , self . accept) ? ; for (src , transitions) in self . transitions . iter () { for (t , dst) in transitions . byte_transitions . iter () { writeln ! (f , "    {src:?} -> {dst:?} [label=\"{t:?}\"]") ? ; } for (t , dst) in transitions . ref_transitions . iter () { writeln ! (f , "    {src:?} -> {dst:?} [label=\"{t:?}\"]") ? ; } } writeln ! (f , "}}") } }}}
mkuse!{use edge_set :: EdgeSet ;}
mkmod!{edge_set, { 
                getname!(edge_set);
                getsrc!(edge_set);
                getpath!(edge_set);
                get_deps!(edge_set);
                get_crates!(edge_set);
                mkinclude!(edge_set);
                mkuse!{use smallvec :: SmallVec ;}
mkuse!{use super :: * ;}
mkitem!{mkstruct!{# [doc = " The set of outbound byte edges associated with a DFA node."] # [derive (Eq , PartialEq , Clone , Debug)] pub (super) struct EdgeSet < S = State > { runs : SmallVec < [(Byte , S) ; 1] > , }}}
mkitem!{mkimpl!{impl < S > EdgeSet < S > { pub (crate) fn new (range : Byte , dst : S) -> Self { let mut this = Self { runs : SmallVec :: new () } ; if ! range . is_empty () { this . runs . push ((range , dst)) ; } this } pub (crate) fn empty () -> Self { Self { runs : SmallVec :: new () } } # [cfg (test)] pub (crate) fn from_edges (mut edges : Vec < (Byte , S) >) -> Self where S : Ord , { edges . sort () ; Self { runs : edges . into () } } pub (crate) fn iter (& self) -> impl Iterator < Item = (Byte , S) > where S : Copy , { self . runs . iter () . copied () } pub (crate) fn get_uninit_edge_dst (& self) -> Option < S > where S : Copy , { let & (range , dst) = self . runs . last () ? ; if range . contains_uninit () { Some (dst) } else { None } } pub (crate) fn map_states < SS > (self , mut f : impl FnMut (S) -> SS) -> EdgeSet < SS > { EdgeSet { runs : self . runs . into_iter () . map (| (b , s) | (b , f (s))) . collect () , } } # [doc = " Unions two edge sets together."] # [doc = ""] # [doc = " If `u = a.union(b)`, then for each byte value, `u` will have an edge"] # [doc = " with that byte value and with the destination `join(Some(_), None)`,"] # [doc = " `join(None, Some(_))`, or `join(Some(_), Some(_))` depending on whether `a`,"] # [doc = " `b`, or both have an edge with that byte value."] # [doc = ""] # [doc = " If neither `a` nor `b` have an edge with a particular byte value,"] # [doc = " then no edge with that value will be present in `u`."] pub (crate) fn union (& self , other : & Self , mut join : impl FnMut (Option < S > , Option < S >) -> S ,) -> EdgeSet < S > where S : Copy + Eq , { let mut runs : SmallVec < [(Byte , S) ; 1] > = SmallVec :: new () ; let xs = self . runs . iter () . copied () ; let ys = other . runs . iter () . copied () ; for (range , (x , y)) in union (xs , ys) { let state = join (x , y) ; match runs . last_mut () { Some (& mut (ref mut last_range , ref mut last_state)) if last_range . end == range . start && * last_state == state => { last_range . end = range . end } _ => runs . push ((range , state)) , } } EdgeSet { runs } } }}} 
            }}

macro_rules! union_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function union in module {}", module_path!());
    };
}

mkfn!{
    union_introspect!();
    # [doc = " Merges two sorted sequences into one sorted sequence."] pub (crate) fn union < S : Copy , X : Iterator < Item = (Byte , S) > , Y : Iterator < Item = (Byte , S) > > (xs : X , ys : Y ,) -> UnionIter < X , Y > { UnionIter { xs : xs . peekable () , ys : ys . peekable () } }
}
mkitem!{mkstruct!{pub (crate) struct UnionIter < X : Iterator , Y : Iterator > { xs : Peekable < X > , ys : Peekable < Y > , }}}
mkitem!{mkimpl!{impl < S : Copy , X : Iterator < Item = (Byte , S) > , Y : Iterator < Item = (Byte , S) > > Iterator for UnionIter < X , Y > { type Item = (Byte , (Option < S > , Option < S >)) ; fn next (& mut self) -> Option < Self :: Item > { use std :: cmp :: { self , Ordering } ; let ret ; match (self . xs . peek_mut () , self . ys . peek_mut ()) { (None , None) => { ret = None ; } (Some (x) , None) => { ret = Some ((x . 0 , (Some (x . 1) , None))) ; self . xs . next () ; } (None , Some (y)) => { ret = Some ((y . 0 , (None , Some (y . 1)))) ; self . ys . next () ; } (Some (x) , Some (y)) => { let start ; let end ; let dst ; match x . 0 . start . cmp (& y . 0 . start) { Ordering :: Less => { start = x . 0 . start ; end = cmp :: min (x . 0 . end , y . 0 . start) ; dst = (Some (x . 1) , None) ; } Ordering :: Greater => { start = y . 0 . start ; end = cmp :: min (x . 0 . start , y . 0 . end) ; dst = (None , Some (y . 1)) ; } Ordering :: Equal => { start = x . 0 . start ; end = cmp :: min (x . 0 . end , y . 0 . end) ; dst = (Some (x . 1) , Some (y . 1)) ; } } ret = Some ((Byte { start , end } , dst)) ; if start == x . 0 . start { x . 0 . start = end ; } if start == y . 0 . start { y . 0 . start = end ; } if x . 0 . is_empty () { self . xs . next () ; } if y . 0 . is_empty () { self . ys . next () ; } } } ret } }}}