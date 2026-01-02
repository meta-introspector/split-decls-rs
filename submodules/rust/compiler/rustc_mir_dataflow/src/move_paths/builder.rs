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
mkuse!{use std :: mem ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_middle :: mir :: * ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt , TypeVisitableExt } ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use smallvec :: { SmallVec , smallvec } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use super :: { Init , InitIndex , InitKind , InitLocation , LocationMap , LookupResult , MoveData , MoveOut , MoveOutIndex , MovePath , MovePathIndex , MovePathLookup , } ;}
mkitem!{mkstruct!{struct MoveDataBuilder < 'a , 'tcx , F > { body : & 'a Body < 'tcx > , loc : Location , tcx : TyCtxt < 'tcx > , data : MoveData < 'tcx > , filter : F , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx , F : Fn (Ty < 'tcx >) -> bool > MoveDataBuilder < 'a , 'tcx , F > { fn new (body : & 'a Body < 'tcx > , tcx : TyCtxt < 'tcx > , filter : F) -> Self { let mut move_paths = IndexVec :: new () ; let mut path_map = IndexVec :: new () ; let mut init_path_map = IndexVec :: new () ; let locals = body . local_decls . iter_enumerated () . map (| (i , l) | { if l . is_deref_temp () { return None ; } if filter (l . ty) { Some (new_move_path (& mut move_paths , & mut path_map , & mut init_path_map , None , Place :: from (i) ,)) } else { None } }) . collect () ; MoveDataBuilder { body , loc : Location :: START , tcx , data : MoveData { moves : IndexVec :: new () , loc_map : LocationMap :: new (body) , rev_lookup : MovePathLookup { locals , projections : Default :: default () , un_derefer : Default :: default () , } , move_paths , path_map , inits : IndexVec :: new () , init_loc_map : LocationMap :: new (body) , init_path_map , } , filter , } } }}}

macro_rules! new_move_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function new_move_path in module {}", module_path!());
    };
}

mkfn!{
    new_move_path_introspect!();
    fn new_move_path < 'tcx > (move_paths : & mut IndexVec < MovePathIndex , MovePath < 'tcx > > , path_map : & mut IndexVec < MovePathIndex , SmallVec < [MoveOutIndex ; 4] > > , init_path_map : & mut IndexVec < MovePathIndex , SmallVec < [InitIndex ; 4] > > , parent : Option < MovePathIndex > , place : Place < 'tcx > ,) -> MovePathIndex { let move_path = move_paths . push (MovePath { next_sibling : None , first_child : None , parent , place }) ; if let Some (parent) = parent { let next_sibling = mem :: replace (& mut move_paths [parent] . first_child , Some (move_path)) ; move_paths [move_path] . next_sibling = next_sibling ; } let path_map_ent = path_map . push (smallvec ! []) ; assert_eq ! (path_map_ent , move_path) ; let init_path_map_ent = init_path_map . push (smallvec ! []) ; assert_eq ! (init_path_map_ent , move_path) ; move_path }
}
mkitem!{mkenum!{enum MovePathResult { Path (MovePathIndex) , Union (MovePathIndex) , Error , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx , F : Fn (Ty < 'tcx >) -> bool > MoveDataBuilder < 'a , 'tcx , F > { #[doc = " This creates a MovePath for a given place, returning an `MovePathError`"] #[doc = " if that place can't be moved from."] #[doc = ""] #[doc = " NOTE: places behind references *do not* get a move path, which is"] #[doc = " problematic for borrowck."] #[doc = ""] #[doc = " Maybe we should have separate \"borrowck\" and \"moveck\" modes."] fn move_path_for (& mut self , place : Place < 'tcx >) -> MovePathResult { let data = & mut self . data ; debug ! ("lookup({:?})" , place) ; let Some (mut base) = data . rev_lookup . find_local (place . local) else { return MovePathResult :: Error ; } ; let mut union_path = None ; for (place_ref , elem) in data . rev_lookup . un_derefer . iter_projections (place . as_ref ()) { let body = self . body ; let tcx = self . tcx ; let place_ty = place_ref . ty (body , tcx) . ty ; if place_ty . references_error () { return MovePathResult :: Error ; } match elem { ProjectionElem :: Deref => match place_ty . kind () { ty :: Ref (..) | ty :: RawPtr (..) => { return MovePathResult :: Error ; } ty :: Adt (adt , _) => { if ! adt . is_box () { bug ! ("Adt should be a box type when Place is deref") ; } } ty :: Bool | ty :: Char | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Foreign (_) | ty :: Str | ty :: Array (_ , _) | ty :: Pat (_ , _) | ty :: Slice (_) | ty :: FnDef (_ , _) | ty :: FnPtr (..) | ty :: Dynamic (_ , _ , _) | ty :: Closure (..) | ty :: CoroutineClosure (..) | ty :: Coroutine (_ , _) | ty :: CoroutineWitness (..) | ty :: Never | ty :: Tuple (_) | ty :: UnsafeBinder (_) | ty :: Alias (_ , _) | ty :: Param (_) | ty :: Bound (_ , _) | ty :: Infer (_) | ty :: Error (_) | ty :: Placeholder (_) => { bug ! ("When Place is Deref it's type shouldn't be {place_ty:#?}") } } , ProjectionElem :: Field (_ , _) => match place_ty . kind () { ty :: Adt (adt , _) => { if adt . has_dtor (tcx) { return MovePathResult :: Error ; } if adt . is_union () { union_path . get_or_insert (base) ; } } ty :: Closure (..) | ty :: CoroutineClosure (..) | ty :: Coroutine (_ , _) | ty :: Tuple (_) => () , ty :: Bool | ty :: Char | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Foreign (_) | ty :: Str | ty :: Array (_ , _) | ty :: Pat (_ , _) | ty :: Slice (_) | ty :: RawPtr (_ , _) | ty :: Ref (_ , _ , _) | ty :: FnDef (_ , _) | ty :: FnPtr (..) | ty :: Dynamic (_ , _ , _) | ty :: CoroutineWitness (..) | ty :: Never | ty :: UnsafeBinder (_) | ty :: Alias (_ , _) | ty :: Param (_) | ty :: Bound (_ , _) | ty :: Infer (_) | ty :: Error (_) | ty :: Placeholder (_) => bug ! ("When Place contains ProjectionElem::Field its type shouldn't be {place_ty:#?}") , } , ProjectionElem :: ConstantIndex { .. } | ProjectionElem :: Subslice { .. } => { match place_ty . kind () { ty :: Slice (_) => { return MovePathResult :: Error ; } ty :: Array (_ , _) => () , _ => bug ! ("Unexpected type {:#?}" , place_ty . is_array ()) , } } ProjectionElem :: Index (_) => match place_ty . kind () { ty :: Array (..) | ty :: Slice (_) => { return MovePathResult :: Error ; } _ => bug ! ("Unexpected type {place_ty:#?}") , } , ProjectionElem :: UnwrapUnsafeBinder (_) => { } ProjectionElem :: OpaqueCast (_) | ProjectionElem :: Subtype (_) | ProjectionElem :: Downcast (_ , _) => () , } let elem_ty = PlaceTy :: from_ty (place_ty) . projection_ty (tcx , elem) . ty ; if ! (self . filter) (elem_ty) { return MovePathResult :: Error ; } if union_path . is_none () { base = * data . rev_lookup . projections . entry ((base , elem . kind ())) . or_insert_with (| | { new_move_path (& mut data . move_paths , & mut data . path_map , & mut data . init_path_map , Some (base) , place_ref . project_deeper (& [elem] , tcx) ,) }) } } if let Some (base) = union_path { MovePathResult :: Union (base) } else { MovePathResult :: Path (base) } } fn add_move_path (& mut self , base : MovePathIndex , elem : PlaceElem < 'tcx > , mk_place : impl FnOnce (TyCtxt < 'tcx >) -> Place < 'tcx > ,) -> MovePathIndex { let MoveDataBuilder { data : MoveData { rev_lookup , move_paths , path_map , init_path_map , .. } , tcx , .. } = self ; * rev_lookup . projections . entry ((base , elem . kind ())) . or_insert_with (move | | { new_move_path (move_paths , path_map , init_path_map , Some (base) , mk_place (* tcx)) }) } fn create_move_path (& mut self , place : Place < 'tcx >) { let _ = self . move_path_for (place) ; } fn finalize (self) -> MoveData < 'tcx > { debug ! ("{}" , { debug ! ("moves for {:?}:" , self . body . span) ; for (j , mo) in self . data . moves . iter_enumerated () { debug ! ("    {:?} = {:?}" , j , mo) ; } debug ! ("move paths for {:?}:" , self . body . span) ; for (j , path) in self . data . move_paths . iter_enumerated () { debug ! ("    {:?} = {:?}" , j , path) ; } "done dumping moves" }) ; self . data } }}}

macro_rules! gather_moves_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function gather_moves in module {}", module_path!());
    };
}

mkfn!{
    gather_moves_introspect!();
    pub (super) fn gather_moves < 'tcx > (body : & Body < 'tcx > , tcx : TyCtxt < 'tcx > , filter : impl Fn (Ty < 'tcx >) -> bool ,) -> MoveData < 'tcx > { let mut builder = MoveDataBuilder :: new (body , tcx , filter) ; builder . gather_args () ; for (bb , block) in body . basic_blocks . iter_enumerated () { for (i , stmt) in block . statements . iter () . enumerate () { builder . loc = Location { block : bb , statement_index : i } ; builder . gather_statement (stmt) ; } builder . loc = Location { block : bb , statement_index : block . statements . len () } ; builder . gather_terminator (block . terminator ()) ; } builder . finalize () }
}
mkitem!{mkimpl!{impl < 'a , 'tcx , F : Fn (Ty < 'tcx >) -> bool > MoveDataBuilder < 'a , 'tcx , F > { fn gather_args (& mut self) { for arg in self . body . args_iter () { if let Some (path) = self . data . rev_lookup . find_local (arg) { let init = self . data . inits . push (Init { path , kind : InitKind :: Deep , location : InitLocation :: Argument (arg) , }) ; debug ! ("gather_args: adding init {:?} of {:?} for argument {:?}" , init , path , arg) ; self . data . init_path_map [path] . push (init) ; } } } fn gather_statement (& mut self , stmt : & Statement < 'tcx >) { debug ! ("gather_statement({:?}, {:?})" , self . loc , stmt) ; match & stmt . kind { StatementKind :: Assign (box (place , Rvalue :: CopyForDeref (reffed))) => { let local = place . as_local () . unwrap () ; assert ! (self . body . local_decls [local] . is_deref_temp ()) ; let rev_lookup = & mut self . data . rev_lookup ; rev_lookup . un_derefer . insert (local , reffed . as_ref ()) ; let base_local = rev_lookup . un_derefer . deref_chain (local) . first () . unwrap () . local ; rev_lookup . locals [local] = rev_lookup . locals [base_local] ; } StatementKind :: Assign (box (place , rval)) => { self . create_move_path (* place) ; if let RvalueInitializationState :: Shallow = rval . initialization_state () { self . create_move_path (self . tcx . mk_place_deref (* place)) ; self . gather_init (place . as_ref () , InitKind :: Shallow) ; } else { self . gather_init (place . as_ref () , InitKind :: Deep) ; } self . gather_rvalue (rval) ; } StatementKind :: FakeRead (box (_ , place)) => { self . create_move_path (* place) ; } StatementKind :: StorageLive (_) => { } StatementKind :: StorageDead (local) => { if ! self . body . local_decls [* local] . is_deref_temp () { self . gather_move (Place :: from (* local)) ; } } StatementKind :: SetDiscriminant { .. } | StatementKind :: Deinit (..) => { span_bug ! (stmt . source_info . span , "SetDiscriminant/Deinit should not exist during borrowck") ; } StatementKind :: Retag { .. } | StatementKind :: AscribeUserType (..) | StatementKind :: PlaceMention (..) | StatementKind :: Coverage (..) | StatementKind :: Intrinsic (..) | StatementKind :: ConstEvalCounter | StatementKind :: BackwardIncompatibleDropHint { .. } | StatementKind :: Nop => { } } } fn gather_rvalue (& mut self , rvalue : & Rvalue < 'tcx >) { match * rvalue { Rvalue :: ThreadLocalRef (_) => { } Rvalue :: Use (ref operand) | Rvalue :: Repeat (ref operand , _) | Rvalue :: Cast (_ , ref operand , _) | Rvalue :: ShallowInitBox (ref operand , _) | Rvalue :: UnaryOp (_ , ref operand) | Rvalue :: WrapUnsafeBinder (ref operand , _) => self . gather_operand (operand) , Rvalue :: BinaryOp (ref _binop , box (ref lhs , ref rhs)) => { self . gather_operand (lhs) ; self . gather_operand (rhs) ; } Rvalue :: Aggregate (ref _kind , ref operands) => { for operand in operands { self . gather_operand (operand) ; } } Rvalue :: CopyForDeref (..) => unreachable ! () , Rvalue :: Ref (..) | Rvalue :: RawPtr (..) | Rvalue :: Discriminant (..) | Rvalue :: Len (..) | Rvalue :: NullaryOp (NullOp :: SizeOf | NullOp :: AlignOf | NullOp :: OffsetOf (..) | NullOp :: UbChecks | NullOp :: ContractChecks , _ ,) => { } } } fn gather_terminator (& mut self , term : & Terminator < 'tcx >) { debug ! ("gather_terminator({:?}, {:?})" , self . loc , term) ; match term . kind { TerminatorKind :: Goto { target : _ } | TerminatorKind :: FalseEdge { .. } | TerminatorKind :: FalseUnwind { .. } | TerminatorKind :: Return | TerminatorKind :: UnwindResume | TerminatorKind :: UnwindTerminate (_) | TerminatorKind :: CoroutineDrop | TerminatorKind :: Unreachable | TerminatorKind :: Drop { .. } => { } TerminatorKind :: Assert { ref cond , .. } => { self . gather_operand (cond) ; } TerminatorKind :: SwitchInt { ref discr , .. } => { self . gather_operand (discr) ; } TerminatorKind :: Yield { ref value , resume_arg : place , .. } => { self . gather_operand (value) ; self . create_move_path (place) ; self . gather_init (place . as_ref () , InitKind :: Deep) ; } TerminatorKind :: Call { ref func , ref args , destination , target , unwind : _ , call_source : _ , fn_span : _ , } => { self . gather_operand (func) ; for arg in args { self . gather_operand (& arg . node) ; } if let Some (_bb) = target { self . create_move_path (destination) ; self . gather_init (destination . as_ref () , InitKind :: NonPanicPathOnly) ; } } TerminatorKind :: TailCall { ref func , ref args , .. } => { self . gather_operand (func) ; for arg in args { self . gather_operand (& arg . node) ; } } TerminatorKind :: InlineAsm { asm_macro : _ , template : _ , ref operands , options : _ , line_spans : _ , targets : _ , unwind : _ , } => { for op in operands { match * op { InlineAsmOperand :: In { reg : _ , ref value } => { self . gather_operand (value) ; } InlineAsmOperand :: Out { reg : _ , late : _ , place , .. } => { if let Some (place) = place { self . create_move_path (place) ; self . gather_init (place . as_ref () , InitKind :: Deep) ; } } InlineAsmOperand :: InOut { reg : _ , late : _ , ref in_value , out_place } => { self . gather_operand (in_value) ; if let Some (out_place) = out_place { self . create_move_path (out_place) ; self . gather_init (out_place . as_ref () , InitKind :: Deep) ; } } InlineAsmOperand :: Const { value : _ } | InlineAsmOperand :: SymFn { value : _ } | InlineAsmOperand :: SymStatic { def_id : _ } | InlineAsmOperand :: Label { target_index : _ } => { } } } } } } fn gather_operand (& mut self , operand : & Operand < 'tcx >) { match * operand { Operand :: Constant (..) | Operand :: Copy (..) => { } Operand :: Move (place) => { self . gather_move (place) ; } } } fn gather_move (& mut self , place : Place < 'tcx >) { debug ! ("gather_move({:?}, {:?})" , self . loc , place) ; if let [ref base @ .. , ProjectionElem :: Subslice { from , to , from_end : false }] = * * place . projection { let base_place = Place { local : place . local , projection : self . tcx . mk_place_elems (base) } ; let base_path = match self . move_path_for (base_place) { MovePathResult :: Path (path) => path , MovePathResult :: Union (path) => { self . record_move (place , path) ; return ; } MovePathResult :: Error => { return ; } } ; let base_ty = base_place . ty (self . body , self . tcx) . ty ; let len : u64 = match base_ty . kind () { ty :: Array (_ , size) => size . try_to_target_usize (self . tcx) . expect ("expected subslice projection on fixed-size array") , _ => bug ! ("from_end: false slice pattern of non-array type") , } ; for offset in from .. to { let elem = ProjectionElem :: ConstantIndex { offset , min_length : len , from_end : false } ; let path = self . add_move_path (base_path , elem , | tcx | tcx . mk_place_elem (base_place , elem)) ; self . record_move (place , path) ; } } else { match self . move_path_for (place) { MovePathResult :: Path (path) | MovePathResult :: Union (path) => { self . record_move (place , path) } MovePathResult :: Error => { } } ; } } fn record_move (& mut self , place : Place < 'tcx > , path : MovePathIndex) { let move_out = self . data . moves . push (MoveOut { path , source : self . loc }) ; debug ! ("gather_move({:?}, {:?}): adding move {:?} of {:?}" , self . loc , place , move_out , path) ; self . data . path_map [path] . push (move_out) ; self . data . loc_map [self . loc] . push (move_out) ; } fn gather_init (& mut self , place : PlaceRef < 'tcx > , kind : InitKind) { debug ! ("gather_init({:?}, {:?})" , self . loc , place) ; let mut place = place ; if let Some ((place_base , ProjectionElem :: Field (_ , _))) = place . last_projection () { if place_base . ty (self . body , self . tcx) . ty . is_union () { place = place_base ; } } if let LookupResult :: Exact (path) = self . data . rev_lookup . find (place) { let init = self . data . inits . push (Init { location : InitLocation :: Statement (self . loc) , path , kind , }) ; debug ! ("gather_init({:?}, {:?}): adding init {:?} of {:?}" , self . loc , place , init , path) ; self . data . init_path_map [path] . push (init) ; self . data . init_loc_map [self . loc] . push (init) ; } } }}}