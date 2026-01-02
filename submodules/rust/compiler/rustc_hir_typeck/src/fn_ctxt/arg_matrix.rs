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
mkuse!{use core :: cmp :: Ordering ;}
mkuse!{use std :: cmp ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_middle :: ty :: error :: TypeError ;}
mkitem!{rustc_index :: newtype_index ! { #[orderable] #[debug_format = "ExpectedIdx({})"] pub (crate) struct ExpectedIdx { } }}
mkitem!{rustc_index :: newtype_index ! { #[orderable] #[debug_format = "ProvidedIdx({})"] pub (crate) struct ProvidedIdx { } }}
mkitem!{mkimpl!{impl ExpectedIdx { pub (crate) fn to_provided_idx (self) -> ProvidedIdx { ProvidedIdx :: from_usize (self . as_usize ()) } }}}
mkitem!{mkimpl!{impl ProvidedIdx { pub (crate) fn to_expected_idx (self) -> ExpectedIdx { ExpectedIdx :: from_u32 (self . as_u32 ()) } }}}
mkitem!{mkenum!{#[derive (Debug)] enum Issue { #[doc = " The given argument is the invalid type for the input"] Invalid (usize) , #[doc = " There is a missing input"] Missing (usize) , #[doc = " There's a superfluous argument"] Extra (usize) , #[doc = " Two arguments should be swapped"] Swap (usize , usize) , #[doc = " Several arguments should be reordered"] Permutation (Vec < Option < usize > >) , }}}
mkitem!{mkenum!{#[derive (Clone , Debug , Eq , PartialEq)] pub (crate) enum Compatibility < 'tcx > { Compatible , Incompatible (Option < TypeError < 'tcx > >) , }}}
mkitem!{mkenum!{#[doc = " Similar to `Issue`, but contains some extra information"] #[derive (Debug , PartialEq , Eq)] pub (crate) enum Error < 'tcx > { #[doc = " The provided argument is the invalid type for the expected input"] Invalid (ProvidedIdx , ExpectedIdx , Compatibility < 'tcx >) , #[doc = " There is a missing input"] Missing (ExpectedIdx) , #[doc = " There's a superfluous argument"] Extra (ProvidedIdx) , #[doc = " Two arguments should be swapped"] Swap (ProvidedIdx , ProvidedIdx , ExpectedIdx , ExpectedIdx) , #[doc = " Several arguments should be reordered"] Permutation (Vec < (ExpectedIdx , ProvidedIdx) >) , }}}
mkitem!{mkimpl!{impl Ord for Error < '_ > { fn cmp (& self , other : & Self) -> Ordering { let key = | error : & Error < '_ > | -> usize { match error { Error :: Invalid (..) => 0 , Error :: Extra (_) => 1 , Error :: Missing (_) => 2 , Error :: Swap (..) => 3 , Error :: Permutation (..) => 4 , } } ; match (self , other) { (Error :: Invalid (a , _ , _) , Error :: Invalid (b , _ , _)) => a . cmp (b) , (Error :: Extra (a) , Error :: Extra (b)) => a . cmp (b) , (Error :: Missing (a) , Error :: Missing (b)) => a . cmp (b) , (Error :: Swap (a , b , ..) , Error :: Swap (c , d , ..)) => a . cmp (c) . then (b . cmp (d)) , (Error :: Permutation (a) , Error :: Permutation (b)) => a . cmp (b) , _ => key (self) . cmp (& key (other)) , } } }}}
mkitem!{mkimpl!{impl PartialOrd for Error < '_ > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }}}
mkitem!{mkstruct!{pub (crate) struct ArgMatrix < 'tcx > { #[doc = " Maps the indices in the `compatibility_matrix` rows to the indices of"] #[doc = " the *user provided* inputs"] provided_indices : Vec < ProvidedIdx > , #[doc = " Maps the indices in the `compatibility_matrix` columns to the indices"] #[doc = " of the *expected* args"] expected_indices : Vec < ExpectedIdx > , #[doc = " The first dimension (rows) are the remaining user provided inputs to"] #[doc = " match and the second dimension (cols) are the remaining expected args"] #[doc = " to match"] compatibility_matrix : Vec < Vec < Compatibility < 'tcx > > > , }}}
mkitem!{mkimpl!{impl < 'tcx > ArgMatrix < 'tcx > { pub (crate) fn new < F : FnMut (ProvidedIdx , ExpectedIdx) -> Compatibility < 'tcx > > (provided_count : usize , expected_input_count : usize , mut is_compatible : F ,) -> Self { let compatibility_matrix = (0 .. provided_count) . map (| i | { (0 .. expected_input_count) . map (| j | is_compatible (ProvidedIdx :: from_usize (i) , ExpectedIdx :: from_usize (j))) . collect () }) . collect () ; ArgMatrix { provided_indices : (0 .. provided_count) . map (ProvidedIdx :: from_usize) . collect () , expected_indices : (0 .. expected_input_count) . map (ExpectedIdx :: from_usize) . collect () , compatibility_matrix , } } #[doc = " Remove a given input from consideration"] fn eliminate_provided (& mut self , idx : usize) { self . provided_indices . remove (idx) ; self . compatibility_matrix . remove (idx) ; } #[doc = " Remove a given argument from consideration"] fn eliminate_expected (& mut self , idx : usize) { self . expected_indices . remove (idx) ; for row in & mut self . compatibility_matrix { row . remove (idx) ; } } #[doc = " \"satisfy\" an input with a given arg, removing both from consideration"] fn satisfy_input (& mut self , provided_idx : usize , expected_idx : usize) { self . eliminate_provided (provided_idx) ; self . eliminate_expected (expected_idx) ; } fn eliminate_satisfied (& mut self) -> Vec < (ProvidedIdx , ExpectedIdx) > { let num_args = cmp :: min (self . provided_indices . len () , self . expected_indices . len ()) ; let mut eliminated = vec ! [] ; for i in (0 .. num_args) . rev () { if matches ! (self . compatibility_matrix [i] [i] , Compatibility :: Compatible) { eliminated . push ((self . provided_indices [i] , self . expected_indices [i])) ; self . satisfy_input (i , i) ; } } eliminated } fn find_issue (& self) -> Option < Issue > { let mat = & self . compatibility_matrix ; let ai = & self . expected_indices ; let ii = & self . provided_indices ; let mut next_unmatched_idx = 0 ; for i in 0 .. cmp :: max (ai . len () , ii . len ()) { if i >= mat . len () { return Some (Issue :: Missing (next_unmatched_idx)) ; } if mat [i] . len () == 0 { return Some (Issue :: Extra (next_unmatched_idx)) ; } let is_arg = i < ai . len () ; let is_input = i < ii . len () ; if is_arg && is_input && matches ! (mat [i] [i] , Compatibility :: Compatible) { next_unmatched_idx += 1 ; continue ; } let mut useless = true ; let mut unsatisfiable = true ; if is_arg { for j in 0 .. ii . len () { if matches ! (mat [j] [i] , Compatibility :: Compatible) { unsatisfiable = false ; break ; } } } if is_input { for j in 0 .. ai . len () { if matches ! (mat [i] [j] , Compatibility :: Compatible) { useless = false ; break ; } } } match (is_input , is_arg , useless , unsatisfiable) { (true , true , true , true) => return Some (Issue :: Invalid (i)) , (true , _ , true , _) => return Some (Issue :: Extra (i)) , (_ , true , _ , true) => return Some (Issue :: Missing (i)) , (true , true , _ , _) => { for j in 0 .. cmp :: min (ai . len () , ii . len ()) { if i == j || matches ! (mat [j] [j] , Compatibility :: Compatible) { continue ; } if matches ! (mat [i] [j] , Compatibility :: Compatible) && matches ! (mat [j] [i] , Compatibility :: Compatible) { return Some (Issue :: Swap (i , j)) ; } } } _ => { continue ; } } } let mut permutation : Vec < Option < Option < usize > > > = vec ! [None ; mat . len ()] ; let mut permutation_found = false ; for i in 0 .. mat . len () { if permutation [i] . is_some () { continue ; } let mut stack = vec ! [] ; let mut j = i ; let mut last = i ; let mut is_cycle = true ; loop { stack . push (j) ; let compat : Vec < _ > = mat [j] . iter () . enumerate () . filter_map (| (i , c) | { if matches ! (c , Compatibility :: Compatible) { Some (i) } else { None } }) . collect () ; if compat . len () < 1 { is_cycle = false ; break ; } j = compat [0] ; if stack . contains (& j) { last = j ; break ; } } if stack . len () <= 2 { is_cycle = false ; } permutation_found = is_cycle ; while let Some (x) = stack . pop () { if is_cycle { permutation [x] = Some (Some (j)) ; j = x ; if j == last { is_cycle = false ; } } else { permutation [x] = Some (None) ; } } } if permutation_found { let final_permutation : Vec < Option < usize > > = permutation . into_iter () . map (| x | x . unwrap ()) . collect () ; return Some (Issue :: Permutation (final_permutation)) ; } None } pub (crate) fn find_errors (mut self ,) -> (Vec < Error < 'tcx > > , IndexVec < ExpectedIdx , Option < ProvidedIdx > >) { let provided_arg_count = self . provided_indices . len () ; let mut errors : Vec < Error < 'tcx > > = vec ! [] ; let mut matched_inputs : IndexVec < ExpectedIdx , Option < ProvidedIdx > > = IndexVec :: from_elem_n (None , self . expected_indices . len ()) ; for (provided , expected) in self . eliminate_satisfied () { matched_inputs [expected] = Some (provided) ; } while ! self . provided_indices . is_empty () || ! self . expected_indices . is_empty () { let res = self . find_issue () ; match res { Some (Issue :: Invalid (idx)) => { let compatibility = self . compatibility_matrix [idx] [idx] . clone () ; let input_idx = self . provided_indices [idx] ; let arg_idx = self . expected_indices [idx] ; self . satisfy_input (idx , idx) ; errors . push (Error :: Invalid (input_idx , arg_idx , compatibility)) ; } Some (Issue :: Extra (idx)) => { let input_idx = self . provided_indices [idx] ; self . eliminate_provided (idx) ; errors . push (Error :: Extra (input_idx)) ; } Some (Issue :: Missing (idx)) => { let arg_idx = self . expected_indices [idx] ; self . eliminate_expected (idx) ; errors . push (Error :: Missing (arg_idx)) ; } Some (Issue :: Swap (idx , other)) => { let input_idx = self . provided_indices [idx] ; let other_input_idx = self . provided_indices [other] ; let arg_idx = self . expected_indices [idx] ; let other_arg_idx = self . expected_indices [other] ; let (min , max) = (cmp :: min (idx , other) , cmp :: max (idx , other)) ; self . satisfy_input (min , max) ; self . satisfy_input (max - 1 , min) ; errors . push (Error :: Swap (input_idx , other_input_idx , arg_idx , other_arg_idx)) ; matched_inputs [other_arg_idx] = Some (input_idx) ; matched_inputs [arg_idx] = Some (other_input_idx) ; } Some (Issue :: Permutation (args)) => { let mut idxs : Vec < usize > = args . iter () . filter_map (| & a | a) . collect () ; let mut real_idxs : IndexVec < ProvidedIdx , Option < (ExpectedIdx , ProvidedIdx) > > = IndexVec :: from_elem_n (None , provided_arg_count) ; for (src , dst) in args . iter () . enumerate () . filter_map (| (src , dst) | dst . map (| dst | (src , dst))) { let src_input_idx = self . provided_indices [src] ; let dst_input_idx = self . provided_indices [dst] ; let dest_arg_idx = self . expected_indices [dst] ; real_idxs [src_input_idx] = Some ((dest_arg_idx , dst_input_idx)) ; matched_inputs [dest_arg_idx] = Some (src_input_idx) ; } idxs . sort () ; idxs . reverse () ; for i in idxs { self . satisfy_input (i , i) ; } errors . push (Error :: Permutation (real_idxs . into_iter () . flatten () . collect ())) ; } None => { let eliminated = self . eliminate_satisfied () ; assert ! (! eliminated . is_empty () , "didn't eliminated any indice in this round") ; for (inp , arg) in eliminated { matched_inputs [arg] = Some (inp) ; } } } ; } errors . sort () ; (errors , matched_inputs) } }}}