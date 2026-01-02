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
mkuse!{use crate :: intrinsics ;}
mkuse!{use crate :: iter :: adapters :: SourceIter ;}
mkuse!{use crate :: iter :: adapters :: zip :: try_get_unchecked ;}
mkuse!{use crate :: iter :: { FusedIterator , TrustedFused , TrustedLen , TrustedRandomAccess , TrustedRandomAccessNoCoerce , } ;}
mkuse!{use crate :: ops :: Try ;}
mkitem!{mkstruct!{# [doc = " An iterator that yields `None` forever after the underlying iterator"] # [doc = " yields `None` once."] # [doc = ""] # [doc = " This `struct` is created by [`Iterator::fuse`]. See its documentation"] # [doc = " for more."] # [derive (Clone , Debug)] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct Fuse < I > { iter : Option < I > , }}}
mkitem!{mkimpl!{impl < I > Fuse < I > { pub (in crate :: iter) fn new (iter : I) -> Fuse < I > { Fuse { iter : Some (iter) } } pub (crate) fn into_inner (self) -> Option < I > { self . iter } }}}
mkitem!{mkimpl!{# [stable (feature = "fused" , since = "1.26.0")] impl < I > FusedIterator for Fuse < I > where I : Iterator { }}}
mkitem!{mkimpl!{# [unstable (issue = "none" , feature = "trusted_fused")] unsafe impl < I > TrustedFused for Fuse < I > where I : TrustedFused { }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < I > Iterator for Fuse < I > where I : Iterator , { type Item = < I as Iterator > :: Item ; # [inline] fn next (& mut self) -> Option < Self :: Item > { FuseImpl :: next (self) } # [inline] fn nth (& mut self , n : usize) -> Option < I :: Item > { FuseImpl :: nth (self , n) } # [inline] fn last (self) -> Option < Self :: Item > { match self . iter { Some (iter) => iter . last () , None => None , } } # [inline] fn count (self) -> usize { match self . iter { Some (iter) => iter . count () , None => 0 , } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { match self . iter { Some (ref iter) => iter . size_hint () , None => (0 , Some (0)) , } } # [inline] fn try_fold < Acc , Fold , R > (& mut self , acc : Acc , fold : Fold) -> R where Self : Sized , Fold : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > , { FuseImpl :: try_fold (self , acc , fold) } # [inline] fn fold < Acc , Fold > (self , mut acc : Acc , fold : Fold) -> Acc where Fold : FnMut (Acc , Self :: Item) -> Acc , { if let Some (iter) = self . iter { acc = iter . fold (acc , fold) ; } acc } # [inline] fn find < P > (& mut self , predicate : P) -> Option < Self :: Item > where P : FnMut (& Self :: Item) -> bool , { FuseImpl :: find (self , predicate) } # [inline] unsafe fn __iterator_get_unchecked (& mut self , idx : usize) -> Self :: Item where Self : TrustedRandomAccessNoCoerce , { match self . iter { Some (ref mut iter) => unsafe { try_get_unchecked (iter , idx) } , None => unsafe { intrinsics :: unreachable () } , } } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < I > DoubleEndedIterator for Fuse < I > where I : DoubleEndedIterator , { # [inline] fn next_back (& mut self) -> Option < < I as Iterator > :: Item > { FuseImpl :: next_back (self) } # [inline] fn nth_back (& mut self , n : usize) -> Option < < I as Iterator > :: Item > { FuseImpl :: nth_back (self , n) } # [inline] fn try_rfold < Acc , Fold , R > (& mut self , acc : Acc , fold : Fold) -> R where Self : Sized , Fold : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > , { FuseImpl :: try_rfold (self , acc , fold) } # [inline] fn rfold < Acc , Fold > (self , mut acc : Acc , fold : Fold) -> Acc where Fold : FnMut (Acc , Self :: Item) -> Acc , { if let Some (iter) = self . iter { acc = iter . rfold (acc , fold) ; } acc } # [inline] fn rfind < P > (& mut self , predicate : P) -> Option < Self :: Item > where P : FnMut (& Self :: Item) -> bool , { FuseImpl :: rfind (self , predicate) } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < I > ExactSizeIterator for Fuse < I > where I : ExactSizeIterator , { fn len (& self) -> usize { match self . iter { Some (ref iter) => iter . len () , None => 0 , } } fn is_empty (& self) -> bool { match self . iter { Some (ref iter) => iter . is_empty () , None => true , } } }}}
mkitem!{mkimpl!{# [stable (feature = "default_iters" , since = "1.70.0")] impl < I : Default > Default for Fuse < I > { # [doc = " Creates a `Fuse` iterator from the default value of `I`."] # [doc = ""] # [doc = " ```"] # [doc = " # use core::slice;"] # [doc = " # use std::iter::Fuse;"] # [doc = " let iter: Fuse<slice::Iter<'_, u8>> = Default::default();"] # [doc = " assert_eq!(iter.len(), 0);"] # [doc = " ```"] # [doc = ""] # [doc = " This is equivalent to `I::default().fuse()`[^fuse_note]; e.g. if"] # [doc = " `I::default()` is not an empty iterator, then this will not be"] # [doc = " an empty iterator."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::iter::Fuse;"] # [doc = " #[derive(Default)]"] # [doc = " struct Fourever;"] # [doc = ""] # [doc = " impl Iterator for Fourever {"] # [doc = "     type Item = u32;"] # [doc = "     fn next(&mut self) -> Option<u32> {"] # [doc = "         Some(4)"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " let mut iter: Fuse<Fourever> = Default::default();"] # [doc = " assert_eq!(iter.next(), Some(4));"] # [doc = " ```"] # [doc = ""] # [doc = " [^fuse_note]: if `I` does not override `Iterator::fuse`'s default implementation"] fn default () -> Self { Fuse { iter : Some (I :: default ()) } } }}}
mkitem!{mkimpl!{# [unstable (feature = "trusted_len" , issue = "37572")] unsafe impl < I > TrustedLen for Fuse < I > where I : TrustedLen { }}}
mkitem!{mkimpl!{# [doc (hidden)] # [unstable (feature = "trusted_random_access" , issue = "none")] unsafe impl < I > TrustedRandomAccess for Fuse < I > where I : TrustedRandomAccess { }}}
mkitem!{mkimpl!{# [doc (hidden)] # [unstable (feature = "trusted_random_access" , issue = "none")] unsafe impl < I > TrustedRandomAccessNoCoerce for Fuse < I > where I : TrustedRandomAccessNoCoerce , { const MAY_HAVE_SIDE_EFFECT : bool = I :: MAY_HAVE_SIDE_EFFECT ; }}}
mkitem!{mktrait!{# [doc = " Fuse specialization trait"] # [doc = ""] # [doc = " We only need to worry about `&mut self` methods, which"] # [doc = " may exhaust the iterator without consuming it."] # [doc (hidden)] trait FuseImpl < I > { type Item ; fn next (& mut self) -> Option < Self :: Item > ; fn nth (& mut self , n : usize) -> Option < Self :: Item > ; fn try_fold < Acc , Fold , R > (& mut self , acc : Acc , fold : Fold) -> R where Self : Sized , Fold : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > ; fn find < P > (& mut self , predicate : P) -> Option < Self :: Item > where P : FnMut (& Self :: Item) -> bool ; fn next_back (& mut self) -> Option < Self :: Item > where I : DoubleEndedIterator ; fn nth_back (& mut self , n : usize) -> Option < Self :: Item > where I : DoubleEndedIterator ; fn try_rfold < Acc , Fold , R > (& mut self , acc : Acc , fold : Fold) -> R where Self : Sized , Fold : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > , I : DoubleEndedIterator ; fn rfind < P > (& mut self , predicate : P) -> Option < Self :: Item > where P : FnMut (& Self :: Item) -> bool , I : DoubleEndedIterator ; }}}
mkitem!{mkimpl!{# [doc = " General `Fuse` impl which sets `iter = None` when exhausted."] # [doc (hidden)] impl < I > FuseImpl < I > for Fuse < I > where I : Iterator , { type Item = < I as Iterator > :: Item ; # [inline] default fn next (& mut self) -> Option < < I as Iterator > :: Item > { and_then_or_clear (& mut self . iter , Iterator :: next) } # [inline] default fn nth (& mut self , n : usize) -> Option < I :: Item > { and_then_or_clear (& mut self . iter , | iter | iter . nth (n)) } # [inline] default fn try_fold < Acc , Fold , R > (& mut self , mut acc : Acc , fold : Fold) -> R where Self : Sized , Fold : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > , { if let Some (ref mut iter) = self . iter { acc = iter . try_fold (acc , fold) ? ; self . iter = None ; } try { acc } } # [inline] default fn find < P > (& mut self , predicate : P) -> Option < Self :: Item > where P : FnMut (& Self :: Item) -> bool , { and_then_or_clear (& mut self . iter , | iter | iter . find (predicate)) } # [inline] default fn next_back (& mut self) -> Option < < I as Iterator > :: Item > where I : DoubleEndedIterator , { and_then_or_clear (& mut self . iter , | iter | iter . next_back ()) } # [inline] default fn nth_back (& mut self , n : usize) -> Option < < I as Iterator > :: Item > where I : DoubleEndedIterator , { and_then_or_clear (& mut self . iter , | iter | iter . nth_back (n)) } # [inline] default fn try_rfold < Acc , Fold , R > (& mut self , mut acc : Acc , fold : Fold) -> R where Self : Sized , Fold : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > , I : DoubleEndedIterator , { if let Some (ref mut iter) = self . iter { acc = iter . try_rfold (acc , fold) ? ; self . iter = None ; } try { acc } } # [inline] default fn rfind < P > (& mut self , predicate : P) -> Option < Self :: Item > where P : FnMut (& Self :: Item) -> bool , I : DoubleEndedIterator , { and_then_or_clear (& mut self . iter , | iter | iter . rfind (predicate)) } }}}
mkitem!{mkimpl!{# [doc = " Specialized `Fuse` impl which doesn't bother clearing `iter` when exhausted."] # [doc = " However, we must still be prepared for the possibility that it was already cleared!"] # [doc (hidden)] impl < I > FuseImpl < I > for Fuse < I > where I : FusedIterator , { # [inline] fn next (& mut self) -> Option < < I as Iterator > :: Item > { self . iter . as_mut () ? . next () } # [inline] fn nth (& mut self , n : usize) -> Option < I :: Item > { self . iter . as_mut () ? . nth (n) } # [inline] fn try_fold < Acc , Fold , R > (& mut self , mut acc : Acc , fold : Fold) -> R where Self : Sized , Fold : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > , { if let Some (ref mut iter) = self . iter { acc = iter . try_fold (acc , fold) ? ; } try { acc } } # [inline] fn find < P > (& mut self , predicate : P) -> Option < Self :: Item > where P : FnMut (& Self :: Item) -> bool , { self . iter . as_mut () ? . find (predicate) } # [inline] fn next_back (& mut self) -> Option < < I as Iterator > :: Item > where I : DoubleEndedIterator , { self . iter . as_mut () ? . next_back () } # [inline] fn nth_back (& mut self , n : usize) -> Option < < I as Iterator > :: Item > where I : DoubleEndedIterator , { self . iter . as_mut () ? . nth_back (n) } # [inline] fn try_rfold < Acc , Fold , R > (& mut self , mut acc : Acc , fold : Fold) -> R where Self : Sized , Fold : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > , I : DoubleEndedIterator , { if let Some (ref mut iter) = self . iter { acc = iter . try_rfold (acc , fold) ? ; } try { acc } } # [inline] fn rfind < P > (& mut self , predicate : P) -> Option < Self :: Item > where P : FnMut (& Self :: Item) -> bool , I : DoubleEndedIterator , { self . iter . as_mut () ? . rfind (predicate) } }}}
mkitem!{mkimpl!{# [unstable (issue = "none" , feature = "inplace_iteration")] unsafe impl < I > SourceIter for Fuse < I > where I : SourceIter + TrustedFused , { type Source = I :: Source ; # [inline] unsafe fn as_inner (& mut self) -> & mut I :: Source { unsafe { SourceIter :: as_inner (self . iter . as_mut () . unwrap_unchecked ()) } } }}}

macro_rules! and_then_or_clear_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function and_then_or_clear in module {}", module_path!());
    };
}

mkfn!{
    and_then_or_clear_introspect!();
    # [inline] fn and_then_or_clear < T , U > (opt : & mut Option < T > , f : impl FnOnce (& mut T) -> Option < U >) -> Option < U > { let x = f (opt . as_mut () ?) ; if x . is_none () { * opt = None ; } x }
}