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
mkuse!{use crate :: iter :: adapters :: SourceIter ;}
mkuse!{use crate :: iter :: { Cloned , Copied , Empty , Filter , FilterMap , Fuse , FusedIterator , Map , Once , OnceWith , TrustedFused , TrustedLen , } ;}
mkuse!{use crate :: num :: NonZero ;}
mkuse!{use crate :: ops :: { ControlFlow , Try } ;}
mkuse!{use crate :: { array , fmt , option , result } ;}
mkitem!{mkstruct!{#[doc = " An iterator that maps each element to an iterator, and yields the elements"] #[doc = " of the produced iterators."] #[doc = ""] #[doc = " This `struct` is created by [`Iterator::flat_map`]. See its documentation"] #[doc = " for more."] #[must_use = "iterators are lazy and do nothing unless consumed"] #[stable (feature = "rust1" , since = "1.0.0")] pub struct FlatMap < I , U : IntoIterator , F > { inner : FlattenCompat < Map < I , F > , < U as IntoIterator > :: IntoIter > , }}}
mkitem!{mkimpl!{impl < I : Iterator , U : IntoIterator , F : FnMut (I :: Item) -> U > FlatMap < I , U , F > { pub (in crate :: iter) fn new (iter : I , f : F) -> FlatMap < I , U , F > { FlatMap { inner : FlattenCompat :: new (iter . map (f)) } } pub (crate) fn into_parts (self) -> (Option < U :: IntoIter > , Option < I > , Option < U :: IntoIter >) { (self . inner . frontiter , self . inner . iter . into_inner () . map (Map :: into_inner) , self . inner . backiter ,) } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < I : Clone , U , F : Clone > Clone for FlatMap < I , U , F > where U : Clone + IntoIterator < IntoIter : Clone > , { fn clone (& self) -> Self { FlatMap { inner : self . inner . clone () } } }}}
mkitem!{mkimpl!{#[stable (feature = "core_impl_debug" , since = "1.9.0")] impl < I : fmt :: Debug , U , F > fmt :: Debug for FlatMap < I , U , F > where U : IntoIterator < IntoIter : fmt :: Debug > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FlatMap") . field ("inner" , & self . inner) . finish () } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < I : Iterator , U : IntoIterator , F > Iterator for FlatMap < I , U , F > where F : FnMut (I :: Item) -> U , { type Item = U :: Item ; #[inline] fn next (& mut self) -> Option < U :: Item > { self . inner . next () } #[inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } #[inline] fn try_fold < Acc , Fold , R > (& mut self , init : Acc , fold : Fold) -> R where Self : Sized , Fold : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > , { self . inner . try_fold (init , fold) } #[inline] fn fold < Acc , Fold > (self , init : Acc , fold : Fold) -> Acc where Fold : FnMut (Acc , Self :: Item) -> Acc , { self . inner . fold (init , fold) } #[inline] fn advance_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { self . inner . advance_by (n) } #[inline] fn count (self) -> usize { self . inner . count () } #[inline] fn last (self) -> Option < Self :: Item > { self . inner . last () } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < I : DoubleEndedIterator , U , F > DoubleEndedIterator for FlatMap < I , U , F > where F : FnMut (I :: Item) -> U , U : IntoIterator < IntoIter : DoubleEndedIterator > , { #[inline] fn next_back (& mut self) -> Option < U :: Item > { self . inner . next_back () } #[inline] fn try_rfold < Acc , Fold , R > (& mut self , init : Acc , fold : Fold) -> R where Self : Sized , Fold : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > , { self . inner . try_rfold (init , fold) } #[inline] fn rfold < Acc , Fold > (self , init : Acc , fold : Fold) -> Acc where Fold : FnMut (Acc , Self :: Item) -> Acc , { self . inner . rfold (init , fold) } #[inline] fn advance_back_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { self . inner . advance_back_by (n) } }}}
mkitem!{mkimpl!{#[stable (feature = "fused" , since = "1.26.0")] impl < I , U , F > FusedIterator for FlatMap < I , U , F > where I : FusedIterator , U : IntoIterator , F : FnMut (I :: Item) -> U , { }}}
mkitem!{mkimpl!{#[unstable (feature = "trusted_len" , issue = "37572")] unsafe impl < I , U , F > TrustedLen for FlatMap < I , U , F > where I : Iterator , U : IntoIterator , F : FnMut (I :: Item) -> U , FlattenCompat < Map < I , F > , < U as IntoIterator > :: IntoIter > : TrustedLen , { }}}
mkitem!{mkimpl!{#[unstable (issue = "none" , feature = "inplace_iteration")] unsafe impl < I , U , F > SourceIter for FlatMap < I , U , F > where I : SourceIter + TrustedFused , U : IntoIterator , { type Source = I :: Source ; #[inline] unsafe fn as_inner (& mut self) -> & mut I :: Source { unsafe { SourceIter :: as_inner (& mut self . inner . iter) } } }}}
mkitem!{mkstruct!{#[doc = " An iterator that flattens one level of nesting in an iterator of things"] #[doc = " that can be turned into iterators."] #[doc = ""] #[doc = " This `struct` is created by the [`flatten`] method on [`Iterator`]. See its"] #[doc = " documentation for more."] #[doc = ""] #[doc = " [`flatten`]: Iterator::flatten()"] #[must_use = "iterators are lazy and do nothing unless consumed"] #[stable (feature = "iterator_flatten" , since = "1.29.0")] pub struct Flatten < I : Iterator < Item : IntoIterator > > { inner : FlattenCompat < I , < I :: Item as IntoIterator > :: IntoIter > , }}}
mkitem!{mkimpl!{impl < I : Iterator < Item : IntoIterator > > Flatten < I > { pub (in super :: super) fn new (iter : I) -> Flatten < I > { Flatten { inner : FlattenCompat :: new (iter) } } }}}
mkitem!{mkimpl!{#[stable (feature = "iterator_flatten" , since = "1.29.0")] impl < I , U > fmt :: Debug for Flatten < I > where I : fmt :: Debug + Iterator < Item : IntoIterator < IntoIter = U , Item = U :: Item > > , U : fmt :: Debug + Iterator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Flatten") . field ("inner" , & self . inner) . finish () } }}}
mkitem!{mkimpl!{#[stable (feature = "iterator_flatten" , since = "1.29.0")] impl < I , U > Clone for Flatten < I > where I : Clone + Iterator < Item : IntoIterator < IntoIter = U , Item = U :: Item > > , U : Clone + Iterator , { fn clone (& self) -> Self { Flatten { inner : self . inner . clone () } } }}}
mkitem!{mkimpl!{#[stable (feature = "iterator_flatten" , since = "1.29.0")] impl < I , U > Iterator for Flatten < I > where I : Iterator < Item : IntoIterator < IntoIter = U , Item = U :: Item > > , U : Iterator , { type Item = U :: Item ; #[inline] fn next (& mut self) -> Option < U :: Item > { self . inner . next () } #[inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } #[inline] fn try_fold < Acc , Fold , R > (& mut self , init : Acc , fold : Fold) -> R where Self : Sized , Fold : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > , { self . inner . try_fold (init , fold) } #[inline] fn fold < Acc , Fold > (self , init : Acc , fold : Fold) -> Acc where Fold : FnMut (Acc , Self :: Item) -> Acc , { self . inner . fold (init , fold) } #[inline] fn advance_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { self . inner . advance_by (n) } #[inline] fn count (self) -> usize { self . inner . count () } #[inline] fn last (self) -> Option < Self :: Item > { self . inner . last () } }}}
mkitem!{mkimpl!{#[stable (feature = "iterator_flatten" , since = "1.29.0")] impl < I , U > DoubleEndedIterator for Flatten < I > where I : DoubleEndedIterator < Item : IntoIterator < IntoIter = U , Item = U :: Item > > , U : DoubleEndedIterator , { #[inline] fn next_back (& mut self) -> Option < U :: Item > { self . inner . next_back () } #[inline] fn try_rfold < Acc , Fold , R > (& mut self , init : Acc , fold : Fold) -> R where Self : Sized , Fold : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > , { self . inner . try_rfold (init , fold) } #[inline] fn rfold < Acc , Fold > (self , init : Acc , fold : Fold) -> Acc where Fold : FnMut (Acc , Self :: Item) -> Acc , { self . inner . rfold (init , fold) } #[inline] fn advance_back_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { self . inner . advance_back_by (n) } }}}
mkitem!{mkimpl!{#[stable (feature = "iterator_flatten" , since = "1.29.0")] impl < I , U > FusedIterator for Flatten < I > where I : FusedIterator < Item : IntoIterator < IntoIter = U , Item = U :: Item > > , U : Iterator , { }}}
mkitem!{mkimpl!{#[unstable (feature = "trusted_len" , issue = "37572")] unsafe impl < I > TrustedLen for Flatten < I > where I : Iterator < Item : IntoIterator > , FlattenCompat < I , < I :: Item as IntoIterator > :: IntoIter > : TrustedLen , { }}}
mkitem!{mkimpl!{#[unstable (issue = "none" , feature = "inplace_iteration")] unsafe impl < I > SourceIter for Flatten < I > where I : SourceIter + TrustedFused + Iterator , < I as Iterator > :: Item : IntoIterator , { type Source = I :: Source ; #[inline] unsafe fn as_inner (& mut self) -> & mut I :: Source { unsafe { SourceIter :: as_inner (& mut self . inner . iter) } } }}}
mkitem!{mkimpl!{#[stable (feature = "default_iters" , since = "1.70.0")] impl < I > Default for Flatten < I > where I : Default + Iterator < Item : IntoIterator > , { #[doc = " Creates a `Flatten` iterator from the default value of `I`."] #[doc = ""] #[doc = " ```"] #[doc = " # use core::slice;"] #[doc = " # use std::iter::Flatten;"] #[doc = " let iter: Flatten<slice::Iter<'_, [u8; 4]>> = Default::default();"] #[doc = " assert_eq!(iter.count(), 0);"] #[doc = " ```"] fn default () -> Self { Flatten :: new (Default :: default ()) } }}}
mkitem!{mkstruct!{#[doc = " Real logic of both `Flatten` and `FlatMap` which simply delegate to"] #[doc = " this type."] #[derive (Clone , Debug)] #[unstable (feature = "trusted_len" , issue = "37572")] struct FlattenCompat < I , U > { iter : Fuse < I > , frontiter : Option < U > , backiter : Option < U > , }}}
mkitem!{mkimpl!{impl < I , U > FlattenCompat < I , U > where I : Iterator , { #[doc = " Adapts an iterator by flattening it, for use in `flatten()` and `flat_map()`."] fn new (iter : I) -> FlattenCompat < I , U > { FlattenCompat { iter : iter . fuse () , frontiter : None , backiter : None } } }}}
mkitem!{mkimpl!{impl < I , U > FlattenCompat < I , U > where I : Iterator < Item : IntoIterator < IntoIter = U > > , { #[doc = " Folds the inner iterators into an accumulator by applying an operation."] #[doc = ""] #[doc = " Folds over the inner iterators, not over their elements. Is used by the `fold`, `count`,"] #[doc = " and `last` methods."] #[inline] fn iter_fold < Acc , Fold > (self , mut acc : Acc , mut fold : Fold) -> Acc where Fold : FnMut (Acc , U) -> Acc , { #[inline] fn flatten < T : IntoIterator , Acc > (fold : & mut impl FnMut (Acc , T :: IntoIter) -> Acc ,) -> impl FnMut (Acc , T) -> Acc + '_ { move | acc , iter | fold (acc , iter . into_iter ()) } if let Some (iter) = self . frontiter { acc = fold (acc , iter) ; } acc = self . iter . fold (acc , flatten (& mut fold)) ; if let Some (iter) = self . backiter { acc = fold (acc , iter) ; } acc } #[doc = " Folds over the inner iterators as long as the given function returns successfully,"] #[doc = " always storing the most recent inner iterator in `self.frontiter`."] #[doc = ""] #[doc = " Folds over the inner iterators, not over their elements. Is used by the `try_fold` and"] #[doc = " `advance_by` methods."] #[inline] fn iter_try_fold < Acc , Fold , R > (& mut self , mut acc : Acc , mut fold : Fold) -> R where Fold : FnMut (Acc , & mut U) -> R , R : Try < Output = Acc > , { #[inline] fn flatten < 'a , T : IntoIterator , Acc , R : Try < Output = Acc > > (frontiter : & 'a mut Option < T :: IntoIter > , fold : & 'a mut impl FnMut (Acc , & mut T :: IntoIter) -> R ,) -> impl FnMut (Acc , T) -> R + 'a { move | acc , iter | fold (acc , frontiter . insert (iter . into_iter ())) } if let Some (iter) = & mut self . frontiter { acc = fold (acc , iter) ? ; } self . frontiter = None ; acc = self . iter . try_fold (acc , flatten (& mut self . frontiter , & mut fold)) ? ; self . frontiter = None ; if let Some (iter) = & mut self . backiter { acc = fold (acc , iter) ? ; } self . backiter = None ; try { acc } } }}}
mkitem!{mkimpl!{impl < I , U > FlattenCompat < I , U > where I : DoubleEndedIterator < Item : IntoIterator < IntoIter = U > > , { #[doc = " Folds the inner iterators into an accumulator by applying an operation, starting form the"] #[doc = " back."] #[doc = ""] #[doc = " Folds over the inner iterators, not over their elements. Is used by the `rfold` method."] #[inline] fn iter_rfold < Acc , Fold > (self , mut acc : Acc , mut fold : Fold) -> Acc where Fold : FnMut (Acc , U) -> Acc , { #[inline] fn flatten < T : IntoIterator , Acc > (fold : & mut impl FnMut (Acc , T :: IntoIter) -> Acc ,) -> impl FnMut (Acc , T) -> Acc + '_ { move | acc , iter | fold (acc , iter . into_iter ()) } if let Some (iter) = self . backiter { acc = fold (acc , iter) ; } acc = self . iter . rfold (acc , flatten (& mut fold)) ; if let Some (iter) = self . frontiter { acc = fold (acc , iter) ; } acc } #[doc = " Folds over the inner iterators in reverse order as long as the given function returns"] #[doc = " successfully, always storing the most recent inner iterator in `self.backiter`."] #[doc = ""] #[doc = " Folds over the inner iterators, not over their elements. Is used by the `try_rfold` and"] #[doc = " `advance_back_by` methods."] #[inline] fn iter_try_rfold < Acc , Fold , R > (& mut self , mut acc : Acc , mut fold : Fold) -> R where Fold : FnMut (Acc , & mut U) -> R , R : Try < Output = Acc > , { #[inline] fn flatten < 'a , T : IntoIterator , Acc , R : Try > (backiter : & 'a mut Option < T :: IntoIter > , fold : & 'a mut impl FnMut (Acc , & mut T :: IntoIter) -> R ,) -> impl FnMut (Acc , T) -> R + 'a { move | acc , iter | fold (acc , backiter . insert (iter . into_iter ())) } if let Some (iter) = & mut self . backiter { acc = fold (acc , iter) ? ; } self . backiter = None ; acc = self . iter . try_rfold (acc , flatten (& mut self . backiter , & mut fold)) ? ; self . backiter = None ; if let Some (iter) = & mut self . frontiter { acc = fold (acc , iter) ? ; } self . frontiter = None ; try { acc } } }}}
mkitem!{mkimpl!{impl < I , U > Iterator for FlattenCompat < I , U > where I : Iterator < Item : IntoIterator < IntoIter = U , Item = U :: Item > > , U : Iterator , { type Item = U :: Item ; #[inline] default fn next (& mut self) -> Option < U :: Item > { loop { if let elt @ Some (_) = and_then_or_clear (& mut self . frontiter , Iterator :: next) { return elt ; } match self . iter . next () { None => return and_then_or_clear (& mut self . backiter , Iterator :: next) , Some (inner) => self . frontiter = Some (inner . into_iter ()) , } } } #[inline] default fn size_hint (& self) -> (usize , Option < usize >) { let (flo , fhi) = self . frontiter . as_ref () . map_or ((0 , Some (0)) , U :: size_hint) ; let (blo , bhi) = self . backiter . as_ref () . map_or ((0 , Some (0)) , U :: size_hint) ; let lo = flo . saturating_add (blo) ; if let Some (fixed_size) = < < I as Iterator > :: Item as ConstSizeIntoIterator > :: size () { let (lower , upper) = self . iter . size_hint () ; let lower = lower . saturating_mul (fixed_size) . saturating_add (lo) ; let upper = try { fhi ? . checked_add (bhi ?) ? . checked_add (fixed_size . checked_mul (upper ?) ?) ? } ; return (lower , upper) ; } match (self . iter . size_hint () , fhi , bhi) { ((0 , Some (0)) , Some (a) , Some (b)) => (lo , a . checked_add (b)) , _ => (lo , None) , } } #[inline] default fn try_fold < Acc , Fold , R > (& mut self , init : Acc , fold : Fold) -> R where Self : Sized , Fold : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > , { #[inline] fn flatten < U : Iterator , Acc , R : Try < Output = Acc > > (mut fold : impl FnMut (Acc , U :: Item) -> R ,) -> impl FnMut (Acc , & mut U) -> R { move | acc , iter | iter . try_fold (acc , & mut fold) } self . iter_try_fold (init , flatten (fold)) } #[inline] default fn fold < Acc , Fold > (self , init : Acc , fold : Fold) -> Acc where Fold : FnMut (Acc , Self :: Item) -> Acc , { #[inline] fn flatten < U : Iterator , Acc > (mut fold : impl FnMut (Acc , U :: Item) -> Acc ,) -> impl FnMut (Acc , U) -> Acc { move | acc , iter | iter . fold (acc , & mut fold) } self . iter_fold (init , flatten (fold)) } #[inline] #[rustc_inherit_overflow_checks] default fn advance_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { #[inline] #[rustc_inherit_overflow_checks] fn advance < U : Iterator > (n : usize , iter : & mut U) -> ControlFlow < () , usize > { match iter . advance_by (n) { Ok (()) => ControlFlow :: Break (()) , Err (remaining) => ControlFlow :: Continue (remaining . get ()) , } } match self . iter_try_fold (n , advance) { ControlFlow :: Continue (remaining) => NonZero :: new (remaining) . map_or (Ok (()) , Err) , _ => Ok (()) , } } #[inline] default fn count (self) -> usize { #[inline] #[rustc_inherit_overflow_checks] fn count < U : Iterator > (acc : usize , iter : U) -> usize { acc + iter . count () } self . iter_fold (0 , count) } #[inline] default fn last (self) -> Option < Self :: Item > { #[inline] fn last < U : Iterator > (last : Option < U :: Item > , iter : U) -> Option < U :: Item > { iter . last () . or (last) } self . iter_fold (None , last) } }}}
mkitem!{mkimpl!{impl < I , U > DoubleEndedIterator for FlattenCompat < I , U > where I : DoubleEndedIterator < Item : IntoIterator < IntoIter = U , Item = U :: Item > > , U : DoubleEndedIterator , { #[inline] default fn next_back (& mut self) -> Option < U :: Item > { loop { if let elt @ Some (_) = and_then_or_clear (& mut self . backiter , | b | b . next_back ()) { return elt ; } match self . iter . next_back () { None => return and_then_or_clear (& mut self . frontiter , | f | f . next_back ()) , Some (inner) => self . backiter = Some (inner . into_iter ()) , } } } #[inline] default fn try_rfold < Acc , Fold , R > (& mut self , init : Acc , fold : Fold) -> R where Self : Sized , Fold : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > , { #[inline] fn flatten < U : DoubleEndedIterator , Acc , R : Try < Output = Acc > > (mut fold : impl FnMut (Acc , U :: Item) -> R ,) -> impl FnMut (Acc , & mut U) -> R { move | acc , iter | iter . try_rfold (acc , & mut fold) } self . iter_try_rfold (init , flatten (fold)) } #[inline] default fn rfold < Acc , Fold > (self , init : Acc , fold : Fold) -> Acc where Fold : FnMut (Acc , Self :: Item) -> Acc , { #[inline] fn flatten < U : DoubleEndedIterator , Acc > (mut fold : impl FnMut (Acc , U :: Item) -> Acc ,) -> impl FnMut (Acc , U) -> Acc { move | acc , iter | iter . rfold (acc , & mut fold) } self . iter_rfold (init , flatten (fold)) } #[inline] #[rustc_inherit_overflow_checks] default fn advance_back_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { #[inline] #[rustc_inherit_overflow_checks] fn advance < U : DoubleEndedIterator > (n : usize , iter : & mut U) -> ControlFlow < () , usize > { match iter . advance_back_by (n) { Ok (()) => ControlFlow :: Break (()) , Err (remaining) => ControlFlow :: Continue (remaining . get ()) , } } match self . iter_try_rfold (n , advance) { ControlFlow :: Continue (remaining) => NonZero :: new (remaining) . map_or (Ok (()) , Err) , _ => Ok (()) , } } }}}
mkitem!{mkimpl!{unsafe impl < const N : usize , I , T > TrustedLen for FlattenCompat < I , < [T ; N] as IntoIterator > :: IntoIter > where I : TrustedLen < Item = [T ; N] > , { }}}
mkitem!{mkimpl!{unsafe impl < 'a , const N : usize , I , T > TrustedLen for FlattenCompat < I , < & 'a [T ; N] as IntoIterator > :: IntoIter > where I : TrustedLen < Item = & 'a [T ; N] > , { }}}
mkitem!{mkimpl!{unsafe impl < 'a , const N : usize , I , T > TrustedLen for FlattenCompat < I , < & 'a mut [T ; N] as IntoIterator > :: IntoIter > where I : TrustedLen < Item = & 'a mut [T ; N] > , { }}}
mkitem!{mktrait!{trait ConstSizeIntoIterator : IntoIterator { fn size () -> Option < usize > ; }}}
mkitem!{mkimpl!{impl < T > ConstSizeIntoIterator for T where T : IntoIterator , { #[inline] default fn size () -> Option < usize > { None } }}}
mkitem!{mkimpl!{impl < T , const N : usize > ConstSizeIntoIterator for [T ; N] { #[inline] fn size () -> Option < usize > { Some (N) } }}}
mkitem!{mkimpl!{impl < T , const N : usize > ConstSizeIntoIterator for & [T ; N] { #[inline] fn size () -> Option < usize > { Some (N) } }}}
mkitem!{mkimpl!{impl < T , const N : usize > ConstSizeIntoIterator for & mut [T ; N] { #[inline] fn size () -> Option < usize > { Some (N) } }}}

macro_rules! and_then_or_clear_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function and_then_or_clear in module {}", module_path!());
    };
}

mkfn!{
    and_then_or_clear_introspect!();
    #[inline] fn and_then_or_clear < T , U > (opt : & mut Option < T > , f : impl FnOnce (& mut T) -> Option < U >) -> Option < U > { let x = f (opt . as_mut () ?) ; if x . is_none () { * opt = None ; } x }
}
mkitem!{mktrait!{#[doc = " Specialization trait for iterator types that never return more than one item."] #[doc = ""] #[doc = " Note that we still have to deal with the possibility that the iterator was"] #[doc = " already exhausted before it came into our control."] #[rustc_specialization_trait] trait OneShot { }}}
mkitem!{mkimpl!{impl < T > OneShot for Once < T > { }}}
mkitem!{mkimpl!{impl < F > OneShot for OnceWith < F > { }}}
mkitem!{mkimpl!{impl < T > OneShot for array :: IntoIter < T , 1 > { }}}
mkitem!{mkimpl!{impl < T > OneShot for option :: IntoIter < T > { }}}
mkitem!{mkimpl!{impl < T > OneShot for option :: Iter < '_ , T > { }}}
mkitem!{mkimpl!{impl < T > OneShot for option :: IterMut < '_ , T > { }}}
mkitem!{mkimpl!{impl < T > OneShot for result :: IntoIter < T > { }}}
mkitem!{mkimpl!{impl < T > OneShot for result :: Iter < '_ , T > { }}}
mkitem!{mkimpl!{impl < T > OneShot for result :: IterMut < '_ , T > { }}}
mkitem!{mkimpl!{impl < T > OneShot for Empty < T > { }}}
mkitem!{mkimpl!{impl < T > OneShot for array :: IntoIter < T , 0 > { }}}
mkitem!{mkimpl!{impl < I : OneShot > OneShot for Cloned < I > { }}}
mkitem!{mkimpl!{impl < I : OneShot > OneShot for Copied < I > { }}}
mkitem!{mkimpl!{impl < I : OneShot , P > OneShot for Filter < I , P > { }}}
mkitem!{mkimpl!{impl < I : OneShot , P > OneShot for FilterMap < I , P > { }}}
mkitem!{mkimpl!{impl < I : OneShot , F > OneShot for Map < I , F > { }}}
mkitem!{mkimpl!{impl < I : OneShot > OneShot for & mut I { }}}

macro_rules! into_item_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function into_item in module {}", module_path!());
    };
}

mkfn!{
    into_item_introspect!();
    #[inline] fn into_item < I > (inner : I) -> Option < I :: Item > where I : IntoIterator < IntoIter : OneShot > , { inner . into_iter () . next () }
}

macro_rules! flatten_one_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function flatten_one in module {}", module_path!());
    };
}

mkfn!{
    flatten_one_introspect!();
    #[inline] fn flatten_one < I : IntoIterator < IntoIter : OneShot > , Acc > (mut fold : impl FnMut (Acc , I :: Item) -> Acc ,) -> impl FnMut (Acc , I) -> Acc { move | acc , inner | match inner . into_iter () . next () { Some (item) => fold (acc , item) , None => acc , } }
}

macro_rules! try_flatten_one_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_flatten_one in module {}", module_path!());
    };
}

mkfn!{
    try_flatten_one_introspect!();
    #[inline] fn try_flatten_one < I : IntoIterator < IntoIter : OneShot > , Acc , R : Try < Output = Acc > > (mut fold : impl FnMut (Acc , I :: Item) -> R ,) -> impl FnMut (Acc , I) -> R { move | acc , inner | match inner . into_iter () . next () { Some (item) => fold (acc , item) , None => try { acc } , } }
}

macro_rules! advance_by_one_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function advance_by_one in module {}", module_path!());
    };
}

mkfn!{
    advance_by_one_introspect!();
    #[inline] fn advance_by_one < I > (n : NonZero < usize > , inner : I) -> Option < NonZero < usize > > where I : IntoIterator < IntoIter : OneShot > , { match inner . into_iter () . next () { Some (_) => NonZero :: new (n . get () - 1) , None => Some (n) , } }
}
mkitem!{mkimpl!{impl < I , U > Iterator for FlattenCompat < I , U > where I : Iterator < Item : IntoIterator < IntoIter = U , Item = U :: Item > > , U : Iterator + OneShot , { #[inline] fn next (& mut self) -> Option < U :: Item > { while let Some (inner) = self . iter . next () { if let item @ Some (_) = inner . into_iter () . next () { return item ; } } None } #[inline] fn size_hint (& self) -> (usize , Option < usize >) { let (lower , upper) = self . iter . size_hint () ; match < I :: Item as ConstSizeIntoIterator > :: size () { Some (0) => (0 , Some (0)) , Some (1) => (lower , upper) , _ => (0 , upper) , } } #[inline] fn try_fold < Acc , Fold , R > (& mut self , init : Acc , fold : Fold) -> R where Self : Sized , Fold : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > , { self . iter . try_fold (init , try_flatten_one (fold)) } #[inline] fn fold < Acc , Fold > (self , init : Acc , fold : Fold) -> Acc where Fold : FnMut (Acc , Self :: Item) -> Acc , { self . iter . fold (init , flatten_one (fold)) } #[inline] fn advance_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { if let Some (n) = NonZero :: new (n) { self . iter . try_fold (n , advance_by_one) . map_or (Ok (()) , Err) } else { self . iter . advance_by (0) } } #[inline] fn count (self) -> usize { self . iter . filter_map (into_item) . count () } #[inline] fn last (self) -> Option < Self :: Item > { self . iter . filter_map (into_item) . last () } }}}
mkitem!{mkimpl!{impl < I , U > DoubleEndedIterator for FlattenCompat < I , U > where I : DoubleEndedIterator < Item : IntoIterator < IntoIter = U , Item = U :: Item > > , U : DoubleEndedIterator + OneShot , { #[inline] fn next_back (& mut self) -> Option < U :: Item > { while let Some (inner) = self . iter . next_back () { if let item @ Some (_) = inner . into_iter () . next () { return item ; } } None } #[inline] fn try_rfold < Acc , Fold , R > (& mut self , init : Acc , fold : Fold) -> R where Self : Sized , Fold : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > , { self . iter . try_rfold (init , try_flatten_one (fold)) } #[inline] fn rfold < Acc , Fold > (self , init : Acc , fold : Fold) -> Acc where Fold : FnMut (Acc , Self :: Item) -> Acc , { self . iter . rfold (init , flatten_one (fold)) } #[inline] fn advance_back_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { if let Some (n) = NonZero :: new (n) { self . iter . try_rfold (n , advance_by_one) . map_or (Ok (()) , Err) } else { self . iter . advance_back_by (0) } } }}}