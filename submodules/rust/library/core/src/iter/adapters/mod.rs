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
mkuse!{use crate :: iter :: InPlaceIterable ;}
mkuse!{use crate :: num :: NonZero ;}
mkuse!{use crate :: ops :: { ChangeOutputType , ControlFlow , FromResidual , Residual , Try } ;}
mkmod!{array_chunks, { 
                getname!(array_chunks);
                getsrc!(array_chunks);
                getpath!(array_chunks);
                get_deps!(array_chunks);
                get_crates!(array_chunks);
                mkinclude!(array_chunks);
                 
            }}
mkmod!{by_ref_sized, { 
                getname!(by_ref_sized);
                getsrc!(by_ref_sized);
                getpath!(by_ref_sized);
                get_deps!(by_ref_sized);
                get_crates!(by_ref_sized);
                mkinclude!(by_ref_sized);
                 
            }}
mkmod!{chain, { 
                getname!(chain);
                getsrc!(chain);
                getpath!(chain);
                get_deps!(chain);
                get_crates!(chain);
                mkinclude!(chain);
                 
            }}
mkmod!{cloned, { 
                getname!(cloned);
                getsrc!(cloned);
                getpath!(cloned);
                get_deps!(cloned);
                get_crates!(cloned);
                mkinclude!(cloned);
                 
            }}
mkmod!{copied, { 
                getname!(copied);
                getsrc!(copied);
                getpath!(copied);
                get_deps!(copied);
                get_crates!(copied);
                mkinclude!(copied);
                 
            }}
mkmod!{cycle, { 
                getname!(cycle);
                getsrc!(cycle);
                getpath!(cycle);
                get_deps!(cycle);
                get_crates!(cycle);
                mkinclude!(cycle);
                 
            }}
mkmod!{enumerate, { 
                getname!(enumerate);
                getsrc!(enumerate);
                getpath!(enumerate);
                get_deps!(enumerate);
                get_crates!(enumerate);
                mkinclude!(enumerate);
                 
            }}
mkmod!{filter, { 
                getname!(filter);
                getsrc!(filter);
                getpath!(filter);
                get_deps!(filter);
                get_crates!(filter);
                mkinclude!(filter);
                 
            }}
mkmod!{filter_map, { 
                getname!(filter_map);
                getsrc!(filter_map);
                getpath!(filter_map);
                get_deps!(filter_map);
                get_crates!(filter_map);
                mkinclude!(filter_map);
                 
            }}
mkmod!{flatten, { 
                getname!(flatten);
                getsrc!(flatten);
                getpath!(flatten);
                get_deps!(flatten);
                get_crates!(flatten);
                mkinclude!(flatten);
                 
            }}
mkmod!{fuse, { 
                getname!(fuse);
                getsrc!(fuse);
                getpath!(fuse);
                get_deps!(fuse);
                get_crates!(fuse);
                mkinclude!(fuse);
                 
            }}
mkmod!{inspect, { 
                getname!(inspect);
                getsrc!(inspect);
                getpath!(inspect);
                get_deps!(inspect);
                get_crates!(inspect);
                mkinclude!(inspect);
                 
            }}
mkmod!{intersperse, { 
                getname!(intersperse);
                getsrc!(intersperse);
                getpath!(intersperse);
                get_deps!(intersperse);
                get_crates!(intersperse);
                mkinclude!(intersperse);
                 
            }}
mkmod!{map, { 
                getname!(map);
                getsrc!(map);
                getpath!(map);
                get_deps!(map);
                get_crates!(map);
                mkinclude!(map);
                 
            }}
mkmod!{map_while, { 
                getname!(map_while);
                getsrc!(map_while);
                getpath!(map_while);
                get_deps!(map_while);
                get_crates!(map_while);
                mkinclude!(map_while);
                 
            }}
mkmod!{map_windows, { 
                getname!(map_windows);
                getsrc!(map_windows);
                getpath!(map_windows);
                get_deps!(map_windows);
                get_crates!(map_windows);
                mkinclude!(map_windows);
                 
            }}
mkmod!{peekable, { 
                getname!(peekable);
                getsrc!(peekable);
                getpath!(peekable);
                get_deps!(peekable);
                get_crates!(peekable);
                mkinclude!(peekable);
                 
            }}
mkmod!{rev, { 
                getname!(rev);
                getsrc!(rev);
                getpath!(rev);
                get_deps!(rev);
                get_crates!(rev);
                mkinclude!(rev);
                 
            }}
mkmod!{scan, { 
                getname!(scan);
                getsrc!(scan);
                getpath!(scan);
                get_deps!(scan);
                get_crates!(scan);
                mkinclude!(scan);
                 
            }}
mkmod!{skip, { 
                getname!(skip);
                getsrc!(skip);
                getpath!(skip);
                get_deps!(skip);
                get_crates!(skip);
                mkinclude!(skip);
                 
            }}
mkmod!{skip_while, { 
                getname!(skip_while);
                getsrc!(skip_while);
                getpath!(skip_while);
                get_deps!(skip_while);
                get_crates!(skip_while);
                mkinclude!(skip_while);
                 
            }}
mkmod!{step_by, { 
                getname!(step_by);
                getsrc!(step_by);
                getpath!(step_by);
                get_deps!(step_by);
                get_crates!(step_by);
                mkinclude!(step_by);
                 
            }}
mkmod!{take, { 
                getname!(take);
                getsrc!(take);
                getpath!(take);
                get_deps!(take);
                get_crates!(take);
                mkinclude!(take);
                 
            }}
mkmod!{take_while, { 
                getname!(take_while);
                getsrc!(take_while);
                getpath!(take_while);
                get_deps!(take_while);
                get_crates!(take_while);
                mkinclude!(take_while);
                 
            }}
mkmod!{zip, { 
                getname!(zip);
                getsrc!(zip);
                getpath!(zip);
                get_deps!(zip);
                get_crates!(zip);
                mkinclude!(zip);
                 
            }}
mkuse!{#[unstable (feature = "iter_array_chunks" , reason = "recently added" , issue = "100450")] pub use self :: array_chunks :: ArrayChunks ;}
mkuse!{#[unstable (feature = "std_internals" , issue = "none")] pub use self :: by_ref_sized :: ByRefSized ;}
mkuse!{#[stable (feature = "iter_chain" , since = "1.91.0")] pub use self :: chain :: chain ;}
mkuse!{#[stable (feature = "iter_cloned" , since = "1.1.0")] pub use self :: cloned :: Cloned ;}
mkuse!{#[stable (feature = "iter_copied" , since = "1.36.0")] pub use self :: copied :: Copied ;}
mkuse!{#[stable (feature = "iterator_flatten" , since = "1.29.0")] pub use self :: flatten :: Flatten ;}
mkuse!{#[unstable (feature = "iter_intersperse" , reason = "recently added" , issue = "79524")] pub use self :: intersperse :: { Intersperse , IntersperseWith } ;}
mkuse!{#[stable (feature = "iter_map_while" , since = "1.57.0")] pub use self :: map_while :: MapWhile ;}
mkuse!{#[unstable (feature = "iter_map_windows" , reason = "recently added" , issue = "87155")] pub use self :: map_windows :: MapWindows ;}
mkuse!{#[stable (feature = "iterator_step_by" , since = "1.28.0")] pub use self :: step_by :: StepBy ;}
mkuse!{#[unstable (feature = "trusted_random_access" , issue = "none")] pub use self :: zip :: TrustedRandomAccess ;}
mkuse!{#[unstable (feature = "trusted_random_access" , issue = "none")] pub use self :: zip :: TrustedRandomAccessNoCoerce ;}
mkuse!{#[stable (feature = "iter_zip" , since = "1.59.0")] pub use self :: zip :: zip ;}
mkuse!{#[stable (feature = "rust1" , since = "1.0.0")] pub use self :: { chain :: Chain , cycle :: Cycle , enumerate :: Enumerate , filter :: Filter , filter_map :: FilterMap , flatten :: FlatMap , fuse :: Fuse , inspect :: Inspect , map :: Map , peekable :: Peekable , rev :: Rev , scan :: Scan , skip :: Skip , skip_while :: SkipWhile , take :: Take , take_while :: TakeWhile , zip :: Zip , } ;}
mkitem!{mktrait!{#[doc = " This trait provides transitive access to source-stage in an iterator-adapter pipeline"] #[doc = " under the conditions that"] #[doc = " * the iterator source `S` itself implements `SourceIter<Source = S>`"] #[doc = " * there is a delegating implementation of this trait for each adapter in the pipeline between"] #[doc = "   the source and the pipeline consumer."] #[doc = ""] #[doc = " When the source is an owning iterator struct (commonly called `IntoIter`) then"] #[doc = " this can be useful for specializing [`FromIterator`] implementations or recovering the"] #[doc = " remaining elements after an iterator has been partially exhausted."] #[doc = ""] #[doc = " Note that implementations do not necessarily have to provide access to the innermost"] #[doc = " source of a pipeline. A stateful intermediate adapter might eagerly evaluate a part"] #[doc = " of the pipeline and expose its internal storage as source."] #[doc = ""] #[doc = " The trait is unsafe because implementers must uphold additional safety properties."] #[doc = " See [`as_inner`] for details."] #[doc = ""] #[doc = " The primary use of this trait is in-place iteration. Refer to the [`vec::in_place_collect`]"] #[doc = " module documentation for more information."] #[doc = ""] #[doc = " [`vec::in_place_collect`]: ../../../../alloc/vec/in_place_collect/index.html"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " Retrieving a partially consumed source:"] #[doc = ""] #[doc = " ```"] #[doc = " # #![feature(inplace_iteration)]"] #[doc = " # use std::iter::SourceIter;"] #[doc = ""] #[doc = " let mut iter = vec![9, 9, 9].into_iter().map(|i| i * i);"] #[doc = " let _ = iter.next();"] #[doc = " let mut remainder = std::mem::replace(unsafe { iter.as_inner() }, Vec::new().into_iter());"] #[doc = " println!(\"n = {} elements remaining\", remainder.len());"] #[doc = " ```"] #[doc = ""] #[doc = " [`FromIterator`]: crate::iter::FromIterator"] #[doc = " [`as_inner`]: SourceIter::as_inner"] #[unstable (issue = "none" , feature = "inplace_iteration")] #[doc (hidden)] #[rustc_specialization_trait] pub unsafe trait SourceIter { #[doc = " A source stage in an iterator pipeline."] type Source ; #[doc = " Retrieve the source of an iterator pipeline."] #[doc = ""] #[doc = " # Safety"] #[doc = ""] #[doc = " Implementations must return the same mutable reference for their lifetime, unless"] #[doc = " replaced by a caller."] #[doc = ""] #[doc = " Callers may only replace the reference when they stopped iteration and drop the"] #[doc = " iterator pipeline after extracting the source."] #[doc = ""] #[doc = " This means iterator adapters can rely on the source not changing during"] #[doc = " iteration but they cannot rely on it in their Drop implementations."] #[doc = ""] #[doc = " Implementing this method means adapters relinquish private-only access to their"] #[doc = " source and can only rely on guarantees made based on method receiver types."] #[doc = " The lack of restricted access also requires that adapters must uphold the source's"] #[doc = " public API even when they have access to its internals."] #[doc = ""] #[doc = " Callers in turn must expect the source to be in any state that is consistent with"] #[doc = " its public API since adapters sitting between it and the source have the same"] #[doc = " access. In particular an adapter may have consumed more elements than strictly necessary."] #[doc = ""] #[doc = " The overall goal of these requirements is to let the consumer of a pipeline use"] #[doc = " * whatever remains in the source after iteration has stopped"] #[doc = " * the memory that has become unused by advancing a consuming iterator"] #[doc = ""] #[doc = " [`next()`]: Iterator::next()"] unsafe fn as_inner (& mut self) -> & mut Self :: Source ; }}}
mkitem!{mkstruct!{#[doc = " An iterator adapter that produces output as long as the underlying"] #[doc = " iterator produces values where `Try::branch` says to `ControlFlow::Continue`."] #[doc = ""] #[doc = " If a `ControlFlow::Break` is encountered, the iterator stops and the"] #[doc = " residual is stored."] pub (crate) struct GenericShunt < 'a , I , R > { iter : I , residual : & 'a mut Option < R > , }}}

macro_rules! try_process_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_process in module {}", module_path!());
    };
}

mkfn!{
    try_process_introspect!();
    #[doc = " Process the given iterator as if it yielded the item's `Try::Output`"] #[doc = " type instead. Any `Try::Residual`s encountered will stop the inner iterator"] #[doc = " and be propagated back to the overall result."] pub (crate) fn try_process < I , T , R , F , U > (iter : I , mut f : F) -> ChangeOutputType < I :: Item , U > where I : Iterator < Item : Try < Output = T , Residual = R > > , for < 'a > F : FnMut (GenericShunt < 'a , I , R >) -> U , R : Residual < U > , { let mut residual = None ; let shunt = GenericShunt { iter , residual : & mut residual } ; let value = f (shunt) ; match residual { Some (r) => FromResidual :: from_residual (r) , None => Try :: from_output (value) , } }
}
mkitem!{mkimpl!{impl < I , R > Iterator for GenericShunt < '_ , I , R > where I : Iterator < Item : Try < Residual = R > > , { type Item = < I :: Item as Try > :: Output ; fn next (& mut self) -> Option < Self :: Item > { self . try_for_each (ControlFlow :: Break) . break_value () } fn size_hint (& self) -> (usize , Option < usize >) { if self . residual . is_some () { (0 , Some (0)) } else { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } } fn try_fold < B , F , T > (& mut self , init : B , mut f : F) -> T where F : FnMut (B , Self :: Item) -> T , T : Try < Output = B > , { self . iter . try_fold (init , | acc , x | match Try :: branch (x) { ControlFlow :: Continue (x) => ControlFlow :: from_try (f (acc , x)) , ControlFlow :: Break (r) => { * self . residual = Some (r) ; ControlFlow :: Break (try { acc }) } }) . into_try () } impl_fold_via_try_fold ! { fold -> try_fold } }}}
mkitem!{mkimpl!{#[unstable (issue = "none" , feature = "inplace_iteration")] unsafe impl < I , R > SourceIter for GenericShunt < '_ , I , R > where I : SourceIter , { type Source = I :: Source ; #[inline] unsafe fn as_inner (& mut self) -> & mut Self :: Source { unsafe { SourceIter :: as_inner (& mut self . iter) } } }}}
mkitem!{mkimpl!{#[unstable (issue = "none" , feature = "inplace_iteration")] unsafe impl < I , R > InPlaceIterable for GenericShunt < '_ , I , R > where I : InPlaceIterable , { const EXPAND_BY : Option < NonZero < usize > > = I :: EXPAND_BY ; const MERGE_BY : Option < NonZero < usize > > = I :: MERGE_BY ; }}}