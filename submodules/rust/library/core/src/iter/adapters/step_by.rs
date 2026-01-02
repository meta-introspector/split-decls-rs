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
mkuse!{use crate :: intrinsics ;}
mkuse!{use crate :: iter :: { TrustedLen , TrustedRandomAccess , from_fn } ;}
mkuse!{use crate :: num :: NonZero ;}
mkuse!{use crate :: ops :: { Range , Try } ;}
mkitem!{mkstruct!{#[doc = " An iterator for stepping iterators by a custom amount."] #[doc = ""] #[doc = " This `struct` is created by the [`step_by`] method on [`Iterator`]. See"] #[doc = " its documentation for more."] #[doc = ""] #[doc = " [`step_by`]: Iterator::step_by"] #[doc = " [`Iterator`]: trait.Iterator.html"] #[must_use = "iterators are lazy and do nothing unless consumed"] #[stable (feature = "iterator_step_by" , since = "1.28.0")] #[derive (Clone , Debug)] pub struct StepBy < I > { #[doc = " This field is guaranteed to be preprocessed by the specialized `SpecRangeSetup::setup`"] #[doc = " in the constructor."] #[doc = " For most iterators that processing is a no-op, but for Range<{integer}> types it is lossy"] #[doc = " which means the inner iterator cannot be returned to user code."] #[doc = " Additionally this type-dependent preprocessing means specialized implementations"] #[doc = " cannot be used interchangeably."] iter : I , #[doc = " This field is `step - 1`, aka the correct amount to pass to `nth` when iterating."] #[doc = " It MUST NOT be `usize::MAX`, as `unsafe` code depends on being able to add one"] #[doc = " without the risk of overflow.  (This is important so that length calculations"] #[doc = " don't need to check for division-by-zero, for example.)"] step_minus_one : usize , first_take : bool , }}}
mkitem!{mkimpl!{impl < I > StepBy < I > { #[inline] pub (in crate :: iter) fn new (iter : I , step : usize) -> StepBy < I > { assert ! (step != 0) ; let iter = < I as SpecRangeSetup < I > > :: setup (iter , step) ; StepBy { iter , step_minus_one : step - 1 , first_take : true } } #[doc = " The `step` that was originally passed to `Iterator::step_by(step)`,"] #[doc = " aka `self.step_minus_one + 1`."] #[inline] fn original_step (& self) -> NonZero < usize > { unsafe { NonZero :: new_unchecked (intrinsics :: unchecked_add (self . step_minus_one , 1)) } } }}}
mkitem!{mkimpl!{#[stable (feature = "iterator_step_by" , since = "1.28.0")] impl < I > Iterator for StepBy < I > where I : Iterator , { type Item = I :: Item ; #[inline] fn next (& mut self) -> Option < Self :: Item > { self . spec_next () } #[inline] fn size_hint (& self) -> (usize , Option < usize >) { self . spec_size_hint () } #[inline] fn nth (& mut self , n : usize) -> Option < Self :: Item > { self . spec_nth (n) } fn try_fold < Acc , F , R > (& mut self , acc : Acc , f : F) -> R where F : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > , { self . spec_try_fold (acc , f) } #[inline] fn fold < Acc , F > (self , acc : Acc , f : F) -> Acc where F : FnMut (Acc , Self :: Item) -> Acc , { self . spec_fold (acc , f) } }}}
mkitem!{mkimpl!{impl < I > StepBy < I > where I : ExactSizeIterator , { fn next_back_index (& self) -> usize { let rem = self . iter . len () % self . original_step () ; if self . first_take { if rem == 0 { self . step_minus_one } else { rem - 1 } } else { rem } } }}}
mkitem!{mkimpl!{#[stable (feature = "double_ended_step_by_iterator" , since = "1.38.0")] impl < I > DoubleEndedIterator for StepBy < I > where I : DoubleEndedIterator + ExactSizeIterator , { #[inline] fn next_back (& mut self) -> Option < Self :: Item > { self . spec_next_back () } #[inline] fn nth_back (& mut self , n : usize) -> Option < Self :: Item > { self . spec_nth_back (n) } fn try_rfold < Acc , F , R > (& mut self , init : Acc , f : F) -> R where F : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > , { self . spec_try_rfold (init , f) } #[inline] fn rfold < Acc , F > (self , init : Acc , f : F) -> Acc where Self : Sized , F : FnMut (Acc , Self :: Item) -> Acc , { self . spec_rfold (init , f) } }}}
mkitem!{mkimpl!{#[stable (feature = "iterator_step_by" , since = "1.28.0")] impl < I > ExactSizeIterator for StepBy < I > where I : ExactSizeIterator { }}}
mkitem!{mkimpl!{#[unstable (feature = "trusted_len" , issue = "37572")] unsafe impl < I > TrustedLen for StepBy < I > where I : Iterator + TrustedRandomAccess { }}}
mkitem!{mktrait!{trait SpecRangeSetup < T > { fn setup (inner : T , step : usize) -> T ; }}}
mkitem!{mkimpl!{impl < T > SpecRangeSetup < T > for T { #[inline] default fn setup (inner : T , _step : usize) -> T { inner } }}}
mkitem!{mktrait!{#[doc = " Specialization trait to optimize `StepBy<Range<{integer}>>` iteration."] #[doc = ""] #[doc = " # Safety"] #[doc = ""] #[doc = " Technically this is safe to implement (look ma, no unsafe!), but in reality"] #[doc = " a lot of unsafe code relies on ranges over integers being correct."] #[doc = ""] #[doc = " For correctness *all* public StepBy methods must be specialized"] #[doc = " because `setup` drastically alters the meaning of the struct fields so that mixing"] #[doc = " different implementations would lead to incorrect results."] unsafe trait StepByImpl < I > { type Item ; fn spec_next (& mut self) -> Option < Self :: Item > ; fn spec_size_hint (& self) -> (usize , Option < usize >) ; fn spec_nth (& mut self , n : usize) -> Option < Self :: Item > ; fn spec_try_fold < Acc , F , R > (& mut self , acc : Acc , f : F) -> R where F : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > ; fn spec_fold < Acc , F > (self , acc : Acc , f : F) -> Acc where F : FnMut (Acc , Self :: Item) -> Acc ; }}}
mkitem!{mktrait!{#[doc = " Specialization trait for double-ended iteration."] #[doc = ""] #[doc = " See also: `StepByImpl`"] #[doc = ""] #[doc = " # Safety"] #[doc = ""] #[doc = " The specializations must be implemented together with `StepByImpl`"] #[doc = " where applicable. I.e. if `StepBy` does support backwards iteration"] #[doc = " for a given iterator and that is specialized for forward iteration then"] #[doc = " it must also be specialized for backwards iteration."] unsafe trait StepByBackImpl < I > { type Item ; fn spec_next_back (& mut self) -> Option < Self :: Item > where I : DoubleEndedIterator + ExactSizeIterator ; fn spec_nth_back (& mut self , n : usize) -> Option < Self :: Item > where I : DoubleEndedIterator + ExactSizeIterator ; fn spec_try_rfold < Acc , F , R > (& mut self , init : Acc , f : F) -> R where I : DoubleEndedIterator + ExactSizeIterator , F : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > ; fn spec_rfold < Acc , F > (self , init : Acc , f : F) -> Acc where I : DoubleEndedIterator + ExactSizeIterator , F : FnMut (Acc , Self :: Item) -> Acc ; }}}
mkitem!{mkimpl!{unsafe impl < I : Iterator > StepByImpl < I > for StepBy < I > { type Item = I :: Item ; #[inline] default fn spec_next (& mut self) -> Option < I :: Item > { let step_size = if self . first_take { 0 } else { self . step_minus_one } ; self . first_take = false ; self . iter . nth (step_size) } #[inline] default fn spec_size_hint (& self) -> (usize , Option < usize >) { #[inline] fn first_size (step : NonZero < usize >) -> impl Fn (usize) -> usize { move | n | if n == 0 { 0 } else { 1 + (n - 1) / step } } #[inline] fn other_size (step : NonZero < usize >) -> impl Fn (usize) -> usize { move | n | n / step } let (low , high) = self . iter . size_hint () ; if self . first_take { let f = first_size (self . original_step ()) ; (f (low) , high . map (f)) } else { let f = other_size (self . original_step ()) ; (f (low) , high . map (f)) } } #[inline] default fn spec_nth (& mut self , mut n : usize) -> Option < I :: Item > { if self . first_take { self . first_take = false ; let first = self . iter . next () ; if n == 0 { return first ; } n -= 1 ; } let mut step = self . original_step () . get () ; if n == usize :: MAX { self . iter . nth (step - 1) ; } else { n += 1 ; } loop { let mul = n . checked_mul (step) ; { if intrinsics :: likely (mul . is_some ()) { return self . iter . nth (mul . unwrap () - 1) ; } } let div_n = usize :: MAX / n ; let div_step = usize :: MAX / step ; let nth_n = div_n * n ; let nth_step = div_step * step ; let nth = if nth_n > nth_step { step -= div_n ; nth_n } else { n -= div_step ; nth_step } ; self . iter . nth (nth - 1) ; } } default fn spec_try_fold < Acc , F , R > (& mut self , mut acc : Acc , mut f : F) -> R where F : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > , { #[inline] fn nth < I : Iterator > (iter : & mut I , step_minus_one : usize ,) -> impl FnMut () -> Option < I :: Item > + '_ { move | | iter . nth (step_minus_one) } if self . first_take { self . first_take = false ; match self . iter . next () { None => return try { acc } , Some (x) => acc = f (acc , x) ? , } } from_fn (nth (& mut self . iter , self . step_minus_one)) . try_fold (acc , f) } default fn spec_fold < Acc , F > (mut self , mut acc : Acc , mut f : F) -> Acc where F : FnMut (Acc , Self :: Item) -> Acc , { #[inline] fn nth < I : Iterator > (iter : & mut I , step_minus_one : usize ,) -> impl FnMut () -> Option < I :: Item > + '_ { move | | iter . nth (step_minus_one) } if self . first_take { self . first_take = false ; match self . iter . next () { None => return acc , Some (x) => acc = f (acc , x) , } } from_fn (nth (& mut self . iter , self . step_minus_one)) . fold (acc , f) } }}}
mkitem!{mkimpl!{unsafe impl < I : DoubleEndedIterator + ExactSizeIterator > StepByBackImpl < I > for StepBy < I > { type Item = I :: Item ; #[inline] default fn spec_next_back (& mut self) -> Option < Self :: Item > { self . iter . nth_back (self . next_back_index ()) } #[inline] default fn spec_nth_back (& mut self , n : usize) -> Option < I :: Item > { let n = n . saturating_mul (self . original_step () . get ()) . saturating_add (self . next_back_index ()) ; self . iter . nth_back (n) } default fn spec_try_rfold < Acc , F , R > (& mut self , init : Acc , mut f : F) -> R where F : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > , { #[inline] fn nth_back < I : DoubleEndedIterator > (iter : & mut I , step_minus_one : usize ,) -> impl FnMut () -> Option < I :: Item > + '_ { move | | iter . nth_back (step_minus_one) } match self . next_back () { None => try { init } , Some (x) => { let acc = f (init , x) ? ; from_fn (nth_back (& mut self . iter , self . step_minus_one)) . try_fold (acc , f) } } } #[inline] default fn spec_rfold < Acc , F > (mut self , init : Acc , mut f : F) -> Acc where Self : Sized , F : FnMut (Acc , I :: Item) -> Acc , { #[inline] fn nth_back < I : DoubleEndedIterator > (iter : & mut I , step_minus_one : usize ,) -> impl FnMut () -> Option < I :: Item > + '_ { move | | iter . nth_back (step_minus_one) } match self . next_back () { None => init , Some (x) => { let acc = f (init , x) ; from_fn (nth_back (& mut self . iter , self . step_minus_one)) . fold (acc , f) } } } }}}
mkitem!{#[doc = " For these implementations, `SpecRangeSetup` calculates the number"] #[doc = " of iterations that will be needed and stores that in `iter.end`."] #[doc = ""] #[doc = " The various iterator implementations then rely on that to not need"] #[doc = " overflow checking, letting loops just be counted instead."] #[doc = ""] #[doc = " These only work for unsigned types, and will need to be reworked"] #[doc = " if you want to use it to specialize on signed types."] #[doc = ""] #[doc = " Currently these are only implemented for integers up to `usize` due to"] #[doc = " correctness issues around `ExactSizeIterator` impls on 16bit platforms."] #[doc = " And since `ExactSizeIterator` is a prerequisite for backwards iteration"] #[doc = " and we must consistently specialize backwards and forwards iteration"] #[doc = " that makes the situation complicated enough that it's not covered"] #[doc = " for now."] macro_rules ! spec_int_ranges { ($ ($ t : ty) *) => ($ (const _ : () = assert ! (usize :: BITS >= <$ t >:: BITS) ; impl SpecRangeSetup < Range <$ t >> for Range <$ t > { #[inline] fn setup (mut r : Range <$ t >, step : usize) -> Range <$ t > { let inner_len = r . size_hint () . 0 ; let yield_count = inner_len . div_ceil (step) ; r . end = yield_count as $ t ; r } } unsafe impl StepByImpl < Range <$ t >> for StepBy < Range <$ t >> { #[inline] fn spec_next (& mut self) -> Option <$ t > { let step = <$ t >:: try_from (self . original_step () . get ()) . unwrap_or (<$ t >:: MAX) ; let remaining = self . iter . end ; if remaining > 0 { let val = self . iter . start ; self . iter . start = val . wrapping_add (step) ; self . iter . end = remaining - 1 ; Some (val) } else { None } } #[inline] fn spec_size_hint (& self) -> (usize , Option < usize >) { let remaining = self . iter . end as usize ; (remaining , Some (remaining)) } #[inline] fn spec_nth (& mut self , n : usize) -> Option < Self :: Item > { self . advance_by (n) . ok () ?; self . next () } #[inline] fn spec_try_fold < Acc , F , R > (& mut self , init : Acc , mut f : F) -> R where F : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > { let mut accum = init ; while let Some (x) = self . next () { accum = f (accum , x) ?; } try { accum } } #[inline] fn spec_fold < Acc , F > (self , init : Acc , mut f : F) -> Acc where F : FnMut (Acc , Self :: Item) -> Acc { let step = <$ t >:: try_from (self . original_step () . get ()) . unwrap_or (<$ t >:: MAX) ; let remaining = self . iter . end ; let mut acc = init ; let mut val = self . iter . start ; for _ in 0 .. remaining { acc = f (acc , val) ; val = val . wrapping_add (step) ; } acc } }) *) }}
mkitem!{macro_rules ! spec_int_ranges_r { ($ ($ t : ty) *) => ($ (const _ : () = assert ! (usize :: BITS >= <$ t >:: BITS) ; unsafe impl StepByBackImpl < Range <$ t >> for StepBy < Range <$ t >> { #[inline] fn spec_next_back (& mut self) -> Option < Self :: Item > { let step = self . original_step () . get () as $ t ; let remaining = self . iter . end ; if remaining > 0 { let start = self . iter . start ; self . iter . end = remaining - 1 ; Some (start + step * (remaining - 1)) } else { None } } #[inline] fn spec_nth_back (& mut self , n : usize) -> Option < Self :: Item > { if self . advance_back_by (n) . is_err () { return None ; } self . next_back () } #[inline] fn spec_try_rfold < Acc , F , R > (& mut self , init : Acc , mut f : F) -> R where F : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > { let mut accum = init ; while let Some (x) = self . next_back () { accum = f (accum , x) ?; } try { accum } } #[inline] fn spec_rfold < Acc , F > (mut self , init : Acc , mut f : F) -> Acc where F : FnMut (Acc , Self :: Item) -> Acc { let mut accum = init ; while let Some (x) = self . next_back () { accum = f (accum , x) ; } accum } }) *) }}
mkitem!{#[cfg (target_pointer_width = "64")] spec_int_ranges ! (u8 u16 u32 u64 usize) ;}
mkitem!{#[cfg (target_pointer_width = "64")] spec_int_ranges_r ! (u8 u16 u32 usize) ;}
mkitem!{#[cfg (target_pointer_width = "32")] spec_int_ranges ! (u8 u16 u32 usize) ;}
mkitem!{#[cfg (target_pointer_width = "32")] spec_int_ranges_r ! (u8 u16 u32 usize) ;}
mkitem!{#[cfg (target_pointer_width = "16")] spec_int_ranges ! (u8 u16 usize) ;}
mkitem!{#[cfg (target_pointer_width = "16")] spec_int_ranges_r ! (u8 u16 usize) ;}