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
mkuse!{use crate :: iter :: adapters :: SourceIter ;}
mkuse!{use crate :: iter :: { FusedIterator , TrustedLen } ;}
mkuse!{use crate :: ops :: { ControlFlow , Try } ;}
mkitem!{mkstruct!{# [doc = " An iterator with a `peek()` that returns an optional reference to the next"] # [doc = " element."] # [doc = ""] # [doc = " This `struct` is created by the [`peekable`] method on [`Iterator`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`peekable`]: Iterator::peekable"] # [doc = " [`Iterator`]: trait.Iterator.html"] # [derive (Clone , Debug)] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "rust1" , since = "1.0.0")] # [rustc_diagnostic_item = "IterPeekable"] pub struct Peekable < I : Iterator > { iter : I , # [doc = " Remember a peeked value, even if it was None."] peeked : Option < Option < I :: Item > > , }}}
mkitem!{mkimpl!{impl < I : Iterator > Peekable < I > { pub (in crate :: iter) fn new (iter : I) -> Peekable < I > { Peekable { iter , peeked : None } } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < I : Iterator > Iterator for Peekable < I > { type Item = I :: Item ; # [inline] fn next (& mut self) -> Option < I :: Item > { match self . peeked . take () { Some (v) => v , None => self . iter . next () , } } # [inline] # [rustc_inherit_overflow_checks] fn count (mut self) -> usize { match self . peeked . take () { Some (None) => 0 , Some (Some (_)) => 1 + self . iter . count () , None => self . iter . count () , } } # [inline] fn nth (& mut self , n : usize) -> Option < I :: Item > { match self . peeked . take () { Some (None) => None , Some (v @ Some (_)) if n == 0 => v , Some (Some (_)) => self . iter . nth (n - 1) , None => self . iter . nth (n) , } } # [inline] fn last (mut self) -> Option < I :: Item > { let peek_opt = match self . peeked . take () { Some (None) => return None , Some (v) => v , None => None , } ; self . iter . last () . or (peek_opt) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let peek_len = match self . peeked { Some (None) => return (0 , Some (0)) , Some (Some (_)) => 1 , None => 0 , } ; let (lo , hi) = self . iter . size_hint () ; let lo = lo . saturating_add (peek_len) ; let hi = match hi { Some (x) => x . checked_add (peek_len) , None => None , } ; (lo , hi) } # [inline] fn try_fold < B , F , R > (& mut self , init : B , mut f : F) -> R where Self : Sized , F : FnMut (B , Self :: Item) -> R , R : Try < Output = B > , { let acc = match self . peeked . take () { Some (None) => return try { init } , Some (Some (v)) => f (init , v) ? , None => init , } ; self . iter . try_fold (acc , f) } # [inline] fn fold < Acc , Fold > (self , init : Acc , mut fold : Fold) -> Acc where Fold : FnMut (Acc , Self :: Item) -> Acc , { let acc = match self . peeked { Some (None) => return init , Some (Some (v)) => fold (init , v) , None => init , } ; self . iter . fold (acc , fold) } }}}
mkitem!{mkimpl!{# [stable (feature = "double_ended_peek_iterator" , since = "1.38.0")] impl < I > DoubleEndedIterator for Peekable < I > where I : DoubleEndedIterator , { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { match self . peeked . as_mut () { Some (v @ Some (_)) => self . iter . next_back () . or_else (| | v . take ()) , Some (None) => None , None => self . iter . next_back () , } } # [inline] fn try_rfold < B , F , R > (& mut self , init : B , mut f : F) -> R where Self : Sized , F : FnMut (B , Self :: Item) -> R , R : Try < Output = B > , { match self . peeked . take () { Some (None) => try { init } , Some (Some (v)) => match self . iter . try_rfold (init , & mut f) . branch () { ControlFlow :: Continue (acc) => f (acc , v) , ControlFlow :: Break (r) => { self . peeked = Some (Some (v)) ; R :: from_residual (r) } } , None => self . iter . try_rfold (init , f) , } } # [inline] fn rfold < Acc , Fold > (self , init : Acc , mut fold : Fold) -> Acc where Fold : FnMut (Acc , Self :: Item) -> Acc , { match self . peeked { Some (None) => init , Some (Some (v)) => { let acc = self . iter . rfold (init , & mut fold) ; fold (acc , v) } None => self . iter . rfold (init , fold) , } } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < I : ExactSizeIterator > ExactSizeIterator for Peekable < I > { }}}
mkitem!{mkimpl!{# [stable (feature = "fused" , since = "1.26.0")] impl < I : FusedIterator > FusedIterator for Peekable < I > { }}}
mkitem!{mkimpl!{impl < I : Iterator > Peekable < I > { # [doc = " Returns a reference to the next() value without advancing the iterator."] # [doc = ""] # [doc = " Like [`next`], if there is a value, it is wrapped in a `Some(T)`."] # [doc = " But if the iteration is over, `None` is returned."] # [doc = ""] # [doc = " [`next`]: Iterator::next"] # [doc = ""] # [doc = " Because `peek()` returns a reference, and many iterators iterate over"] # [doc = " references, there can be a possibly confusing situation where the"] # [doc = " return value is a double reference. You can see this effect in the"] # [doc = " examples below."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " let xs = [1, 2, 3];"] # [doc = ""] # [doc = " let mut iter = xs.iter().peekable();"] # [doc = ""] # [doc = " // peek() lets us see into the future"] # [doc = " assert_eq!(iter.peek(), Some(&&1));"] # [doc = " assert_eq!(iter.next(), Some(&1));"] # [doc = ""] # [doc = " assert_eq!(iter.next(), Some(&2));"] # [doc = ""] # [doc = " // The iterator does not advance even if we `peek` multiple times"] # [doc = " assert_eq!(iter.peek(), Some(&&3));"] # [doc = " assert_eq!(iter.peek(), Some(&&3));"] # [doc = ""] # [doc = " assert_eq!(iter.next(), Some(&3));"] # [doc = ""] # [doc = " // After the iterator is finished, so is `peek()`"] # [doc = " assert_eq!(iter.peek(), None);"] # [doc = " assert_eq!(iter.next(), None);"] # [doc = " ```"] # [inline] # [stable (feature = "rust1" , since = "1.0.0")] pub fn peek (& mut self) -> Option < & I :: Item > { let iter = & mut self . iter ; self . peeked . get_or_insert_with (| | iter . next ()) . as_ref () } # [doc = " Returns a mutable reference to the next() value without advancing the iterator."] # [doc = ""] # [doc = " Like [`next`], if there is a value, it is wrapped in a `Some(T)`."] # [doc = " But if the iteration is over, `None` is returned."] # [doc = ""] # [doc = " Because `peek_mut()` returns a reference, and many iterators iterate over"] # [doc = " references, there can be a possibly confusing situation where the"] # [doc = " return value is a double reference. You can see this effect in the examples"] # [doc = " below."] # [doc = ""] # [doc = " [`next`]: Iterator::next"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " let mut iter = [1, 2, 3].iter().peekable();"] # [doc = ""] # [doc = " // Like with `peek()`, we can see into the future without advancing the iterator."] # [doc = " assert_eq!(iter.peek_mut(), Some(&mut &1));"] # [doc = " assert_eq!(iter.peek_mut(), Some(&mut &1));"] # [doc = " assert_eq!(iter.next(), Some(&1));"] # [doc = ""] # [doc = " // Peek into the iterator and set the value behind the mutable reference."] # [doc = " if let Some(p) = iter.peek_mut() {"] # [doc = "     assert_eq!(*p, &2);"] # [doc = "     *p = &5;"] # [doc = " }"] # [doc = ""] # [doc = " // The value we put in reappears as the iterator continues."] # [doc = " assert_eq!(iter.collect::<Vec<_>>(), vec![&5, &3]);"] # [doc = " ```"] # [inline] # [stable (feature = "peekable_peek_mut" , since = "1.53.0")] pub fn peek_mut (& mut self) -> Option < & mut I :: Item > { let iter = & mut self . iter ; self . peeked . get_or_insert_with (| | iter . next ()) . as_mut () } # [doc = " Consume and return the next value of this iterator if a condition is true."] # [doc = ""] # [doc = " If `func` returns `true` for the next value of this iterator, consume and return it."] # [doc = " Otherwise, return `None`."] # [doc = ""] # [doc = " # Examples"] # [doc = " Consume a number if it's equal to 0."] # [doc = " ```"] # [doc = " let mut iter = (0..5).peekable();"] # [doc = " // The first item of the iterator is 0; consume it."] # [doc = " assert_eq!(iter.next_if(|&x| x == 0), Some(0));"] # [doc = " // The next item returned is now 1, so `next_if` will return `None`."] # [doc = " assert_eq!(iter.next_if(|&x| x == 0), None);"] # [doc = " // `next_if` retains the next item if the predicate evaluates to `false` for it."] # [doc = " assert_eq!(iter.next(), Some(1));"] # [doc = " ```"] # [doc = ""] # [doc = " Consume any number less than 10."] # [doc = " ```"] # [doc = " let mut iter = (1..20).peekable();"] # [doc = " // Consume all numbers less than 10"] # [doc = " while iter.next_if(|&x| x < 10).is_some() {}"] # [doc = " // The next value returned will be 10"] # [doc = " assert_eq!(iter.next(), Some(10));"] # [doc = " ```"] # [stable (feature = "peekable_next_if" , since = "1.51.0")] pub fn next_if (& mut self , func : impl FnOnce (& I :: Item) -> bool) -> Option < I :: Item > { match self . next () { Some (matched) if func (& matched) => Some (matched) , other => { assert ! (self . peeked . is_none ()) ; self . peeked = Some (other) ; None } } } # [doc = " Consume and return the next item if it is equal to `expected`."] # [doc = ""] # [doc = " # Example"] # [doc = " Consume a number if it's equal to 0."] # [doc = " ```"] # [doc = " let mut iter = (0..5).peekable();"] # [doc = " // The first item of the iterator is 0; consume it."] # [doc = " assert_eq!(iter.next_if_eq(&0), Some(0));"] # [doc = " // The next item returned is now 1, so `next_if_eq` will return `None`."] # [doc = " assert_eq!(iter.next_if_eq(&0), None);"] # [doc = " // `next_if_eq` retains the next item if it was not equal to `expected`."] # [doc = " assert_eq!(iter.next(), Some(1));"] # [doc = " ```"] # [stable (feature = "peekable_next_if" , since = "1.51.0")] pub fn next_if_eq < T > (& mut self , expected : & T) -> Option < I :: Item > where T : ? Sized , I :: Item : PartialEq < T > , { self . next_if (| next | next == expected) } # [doc = " Consumes the next value of this iterator and applies a function `f` on it,"] # [doc = " returning the result if the closure returns `Ok`."] # [doc = ""] # [doc = " Otherwise if the closure returns `Err` the value is put back for the next iteration."] # [doc = ""] # [doc = " The content of the `Err` variant is typically the original value of the closure,"] # [doc = " but this is not required. If a different value is returned,"] # [doc = " the next `peek()` or `next()` call will result in this new value."] # [doc = " This is similar to modifying the output of `peek_mut()`."] # [doc = ""] # [doc = " If the closure panics, the next value will always be consumed and dropped"] # [doc = " even if the panic is caught, because the closure never returned an `Err` value to put back."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Parse the leading decimal number from an iterator of characters."] # [doc = " ```"] # [doc = " #![feature(peekable_next_if_map)]"] # [doc = " let mut iter = \"125 GOTO 10\".chars().peekable();"] # [doc = " let mut line_num = 0_u32;"] # [doc = " while let Some(digit) = iter.next_if_map(|c| c.to_digit(10).ok_or(c)) {"] # [doc = "     line_num = line_num * 10 + digit;"] # [doc = " }"] # [doc = " assert_eq!(line_num, 125);"] # [doc = " assert_eq!(iter.collect::<String>(), \" GOTO 10\");"] # [doc = " ```"] # [doc = ""] # [doc = " Matching custom types."] # [doc = " ```"] # [doc = " #![feature(peekable_next_if_map)]"] # [doc = ""] # [doc = " #[derive(Debug, PartialEq, Eq)]"] # [doc = " enum Node {"] # [doc = "     Comment(String),"] # [doc = "     Red(String),"] # [doc = "     Green(String),"] # [doc = "     Blue(String),"] # [doc = " }"] # [doc = ""] # [doc = " /// Combines all consecutive `Comment` nodes into a single one."] # [doc = " fn combine_comments(nodes: Vec<Node>) -> Vec<Node> {"] # [doc = "     let mut result = Vec::with_capacity(nodes.len());"] # [doc = "     let mut iter = nodes.into_iter().peekable();"] # [doc = "     let mut comment_text = None::<String>;"] # [doc = "     loop {"] # [doc = "         // Typically the closure in .next_if_map() matches on the input,"] # [doc = "         //  extracts the desired pattern into an `Ok`,"] # [doc = "         //  and puts the rest into an `Err`."] # [doc = "         while let Some(text) = iter.next_if_map(|node| match node {"] # [doc = "             Node::Comment(text) => Ok(text),"] # [doc = "             other => Err(other),"] # [doc = "         }) {"] # [doc = "             comment_text.get_or_insert_default().push_str(&text);"] # [doc = "         }"] # [doc = ""] # [doc = "         if let Some(text) = comment_text.take() {"] # [doc = "             result.push(Node::Comment(text));"] # [doc = "         }"] # [doc = "         if let Some(node) = iter.next() {"] # [doc = "             result.push(node);"] # [doc = "         } else {"] # [doc = "             break;"] # [doc = "         }"] # [doc = "     }"] # [doc = "     result"] # [doc = " }"] # [doc = "# assert_eq!( // hiding the test to avoid cluttering the documentation."] # [doc = "#     combine_comments(vec!["] # [doc = "#         Node::Comment(\"The\".to_owned()),"] # [doc = "#         Node::Comment(\"Quick\".to_owned()),"] # [doc = "#         Node::Comment(\"Brown\".to_owned()),"] # [doc = "#         Node::Red(\"Fox\".to_owned()),"] # [doc = "#         Node::Green(\"Jumped\".to_owned()),"] # [doc = "#         Node::Comment(\"Over\".to_owned()),"] # [doc = "#         Node::Blue(\"The\".to_owned()),"] # [doc = "#         Node::Comment(\"Lazy\".to_owned()),"] # [doc = "#         Node::Comment(\"Dog\".to_owned()),"] # [doc = "#     ]),"] # [doc = "#     vec!["] # [doc = "#         Node::Comment(\"TheQuickBrown\".to_owned()),"] # [doc = "#         Node::Red(\"Fox\".to_owned()),"] # [doc = "#         Node::Green(\"Jumped\".to_owned()),"] # [doc = "#         Node::Comment(\"Over\".to_owned()),"] # [doc = "#         Node::Blue(\"The\".to_owned()),"] # [doc = "#         Node::Comment(\"LazyDog\".to_owned()),"] # [doc = "#     ],"] # [doc = "# )"] # [doc = " ```"] # [unstable (feature = "peekable_next_if_map" , issue = "143702")] pub fn next_if_map < R > (& mut self , f : impl FnOnce (I :: Item) -> Result < R , I :: Item >) -> Option < R > { let unpeek = if let Some (item) = self . next () { match f (item) { Ok (result) => return Some (result) , Err (item) => Some (item) , } } else { None } ; self . peeked = Some (unpeek) ; None } }}}
mkitem!{mkimpl!{# [unstable (feature = "trusted_len" , issue = "37572")] unsafe impl < I > TrustedLen for Peekable < I > where I : TrustedLen { }}}
mkitem!{mkimpl!{# [unstable (issue = "none" , feature = "inplace_iteration")] unsafe impl < I : Iterator > SourceIter for Peekable < I > where I : SourceIter , { type Source = I :: Source ; # [inline] unsafe fn as_inner (& mut self) -> & mut I :: Source { unsafe { SourceIter :: as_inner (& mut self . iter) } } }}}