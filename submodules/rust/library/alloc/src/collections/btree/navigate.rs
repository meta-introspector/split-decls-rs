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
mkuse!{use core :: borrow :: Borrow ;}
mkuse!{use core :: ops :: RangeBounds ;}
mkuse!{use core :: { hint , ptr } ;}
mkuse!{use super :: node :: ForceResult :: * ;}
mkuse!{use super :: node :: { Handle , NodeRef , marker } ;}
mkuse!{use super :: search :: SearchBound ;}
mkuse!{use crate :: alloc :: Allocator ;}
mkitem!{mkstruct!{pub (super) struct LeafRange < BorrowType , K , V > { front : Option < Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > > , back : Option < Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > > , }}}
mkitem!{mkimpl!{impl < 'a , K : 'a , V : 'a > Clone for LeafRange < marker :: Immut < 'a > , K , V > { fn clone (& self) -> Self { LeafRange { front : self . front . clone () , back : self . back . clone () } } }}}
mkitem!{mkimpl!{impl < B , K , V > Default for LeafRange < B , K , V > { fn default () -> Self { LeafRange { front : None , back : None } } }}}
mkitem!{mkimpl!{impl < BorrowType , K , V > LeafRange < BorrowType , K , V > { pub (super) fn none () -> Self { LeafRange { front : None , back : None } } fn is_empty (& self) -> bool { self . front == self . back } # [doc = " Temporarily takes out another, immutable equivalent of the same range."] pub (super) fn reborrow (& self) -> LeafRange < marker :: Immut < '_ > , K , V > { LeafRange { front : self . front . as_ref () . map (| f | f . reborrow ()) , back : self . back . as_ref () . map (| b | b . reborrow ()) , } } }}}
mkitem!{mkimpl!{impl < 'a , K , V > LeafRange < marker :: Immut < 'a > , K , V > { # [inline] pub (super) fn next_checked (& mut self) -> Option < (& 'a K , & 'a V) > { self . perform_next_checked (| kv | kv . into_kv ()) } # [inline] pub (super) fn next_back_checked (& mut self) -> Option < (& 'a K , & 'a V) > { self . perform_next_back_checked (| kv | kv . into_kv ()) } }}}
mkitem!{mkimpl!{impl < 'a , K , V > LeafRange < marker :: ValMut < 'a > , K , V > { # [inline] pub (super) fn next_checked (& mut self) -> Option < (& 'a K , & 'a mut V) > { self . perform_next_checked (| kv | unsafe { ptr :: read (kv) } . into_kv_valmut ()) } # [inline] pub (super) fn next_back_checked (& mut self) -> Option < (& 'a K , & 'a mut V) > { self . perform_next_back_checked (| kv | unsafe { ptr :: read (kv) } . into_kv_valmut ()) } }}}
mkitem!{mkimpl!{impl < BorrowType : marker :: BorrowType , K , V > LeafRange < BorrowType , K , V > { # [doc = " If possible, extract some result from the following KV and move to the edge beyond it."] fn perform_next_checked < F , R > (& mut self , f : F) -> Option < R > where F : Fn (& Handle < NodeRef < BorrowType , K , V , marker :: LeafOrInternal > , marker :: KV >) -> R , { if self . is_empty () { None } else { super :: mem :: replace (self . front . as_mut () . unwrap () , | front | { let kv = front . next_kv () . ok () . unwrap () ; let result = f (& kv) ; (kv . next_leaf_edge () , Some (result)) }) } } # [doc = " If possible, extract some result from the preceding KV and move to the edge beyond it."] fn perform_next_back_checked < F , R > (& mut self , f : F) -> Option < R > where F : Fn (& Handle < NodeRef < BorrowType , K , V , marker :: LeafOrInternal > , marker :: KV >) -> R , { if self . is_empty () { None } else { super :: mem :: replace (self . back . as_mut () . unwrap () , | back | { let kv = back . next_back_kv () . ok () . unwrap () ; let result = f (& kv) ; (kv . next_back_leaf_edge () , Some (result)) }) } } }}}
mkitem!{mkenum!{enum LazyLeafHandle < BorrowType , K , V > { Root (NodeRef < BorrowType , K , V , marker :: LeafOrInternal >) , Edge (Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge >) , }}}
mkitem!{mkimpl!{impl < 'a , K : 'a , V : 'a > Clone for LazyLeafHandle < marker :: Immut < 'a > , K , V > { fn clone (& self) -> Self { match self { LazyLeafHandle :: Root (root) => LazyLeafHandle :: Root (* root) , LazyLeafHandle :: Edge (edge) => LazyLeafHandle :: Edge (* edge) , } } }}}
mkitem!{mkimpl!{impl < BorrowType , K , V > LazyLeafHandle < BorrowType , K , V > { fn reborrow (& self) -> LazyLeafHandle < marker :: Immut < '_ > , K , V > { match self { LazyLeafHandle :: Root (root) => LazyLeafHandle :: Root (root . reborrow ()) , LazyLeafHandle :: Edge (edge) => LazyLeafHandle :: Edge (edge . reborrow ()) , } } }}}
mkitem!{mkstruct!{pub (super) struct LazyLeafRange < BorrowType , K , V > { front : Option < LazyLeafHandle < BorrowType , K , V > > , back : Option < LazyLeafHandle < BorrowType , K , V > > , }}}
mkitem!{mkimpl!{impl < B , K , V > Default for LazyLeafRange < B , K , V > { fn default () -> Self { LazyLeafRange { front : None , back : None } } }}}
mkitem!{mkimpl!{impl < 'a , K : 'a , V : 'a > Clone for LazyLeafRange < marker :: Immut < 'a > , K , V > { fn clone (& self) -> Self { LazyLeafRange { front : self . front . clone () , back : self . back . clone () } } }}}
mkitem!{mkimpl!{impl < BorrowType , K , V > LazyLeafRange < BorrowType , K , V > { pub (super) fn none () -> Self { LazyLeafRange { front : None , back : None } } # [doc = " Temporarily takes out another, immutable equivalent of the same range."] pub (super) fn reborrow (& self) -> LazyLeafRange < marker :: Immut < '_ > , K , V > { LazyLeafRange { front : self . front . as_ref () . map (| f | f . reborrow ()) , back : self . back . as_ref () . map (| b | b . reborrow ()) , } } }}}
mkitem!{mkimpl!{impl < 'a , K , V > LazyLeafRange < marker :: Immut < 'a > , K , V > { # [inline] pub (super) unsafe fn next_unchecked (& mut self) -> (& 'a K , & 'a V) { unsafe { self . init_front () . unwrap () . next_unchecked () } } # [inline] pub (super) unsafe fn next_back_unchecked (& mut self) -> (& 'a K , & 'a V) { unsafe { self . init_back () . unwrap () . next_back_unchecked () } } }}}
mkitem!{mkimpl!{impl < 'a , K , V > LazyLeafRange < marker :: ValMut < 'a > , K , V > { # [inline] pub (super) unsafe fn next_unchecked (& mut self) -> (& 'a K , & 'a mut V) { unsafe { self . init_front () . unwrap () . next_unchecked () } } # [inline] pub (super) unsafe fn next_back_unchecked (& mut self) -> (& 'a K , & 'a mut V) { unsafe { self . init_back () . unwrap () . next_back_unchecked () } } }}}
mkitem!{mkimpl!{impl < K , V > LazyLeafRange < marker :: Dying , K , V > { fn take_front (& mut self ,) -> Option < Handle < NodeRef < marker :: Dying , K , V , marker :: Leaf > , marker :: Edge > > { match self . front . take () ? { LazyLeafHandle :: Root (root) => Some (root . first_leaf_edge ()) , LazyLeafHandle :: Edge (edge) => Some (edge) , } } # [inline] pub (super) unsafe fn deallocating_next_unchecked < A : Allocator + Clone > (& mut self , alloc : A ,) -> Handle < NodeRef < marker :: Dying , K , V , marker :: LeafOrInternal > , marker :: KV > { debug_assert ! (self . front . is_some ()) ; let front = self . init_front () . unwrap () ; unsafe { front . deallocating_next_unchecked (alloc) } } # [inline] pub (super) unsafe fn deallocating_next_back_unchecked < A : Allocator + Clone > (& mut self , alloc : A ,) -> Handle < NodeRef < marker :: Dying , K , V , marker :: LeafOrInternal > , marker :: KV > { debug_assert ! (self . back . is_some ()) ; let back = self . init_back () . unwrap () ; unsafe { back . deallocating_next_back_unchecked (alloc) } } # [inline] pub (super) fn deallocating_end < A : Allocator + Clone > (& mut self , alloc : A) { if let Some (front) = self . take_front () { front . deallocating_end (alloc) } } }}}
mkitem!{mkimpl!{impl < BorrowType : marker :: BorrowType , K , V > LazyLeafRange < BorrowType , K , V > { fn init_front (& mut self ,) -> Option < & mut Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > > { if let Some (LazyLeafHandle :: Root (root)) = & self . front { self . front = Some (LazyLeafHandle :: Edge (unsafe { ptr :: read (root) } . first_leaf_edge ())) ; } match & mut self . front { None => None , Some (LazyLeafHandle :: Edge (edge)) => Some (edge) , Some (LazyLeafHandle :: Root (_)) => unsafe { hint :: unreachable_unchecked () } , } } fn init_back (& mut self ,) -> Option < & mut Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > > { if let Some (LazyLeafHandle :: Root (root)) = & self . back { self . back = Some (LazyLeafHandle :: Edge (unsafe { ptr :: read (root) } . last_leaf_edge ())) ; } match & mut self . back { None => None , Some (LazyLeafHandle :: Edge (edge)) => Some (edge) , Some (LazyLeafHandle :: Root (_)) => unsafe { hint :: unreachable_unchecked () } , } } }}}
mkitem!{mkimpl!{impl < BorrowType : marker :: BorrowType , K , V > NodeRef < BorrowType , K , V , marker :: LeafOrInternal > { # [doc = " Finds the distinct leaf edges delimiting a specified range in a tree."] # [doc = ""] # [doc = " If such distinct edges exist, returns them in ascending order, meaning"] # [doc = " that a non-zero number of calls to `next_unchecked` on the `front` of"] # [doc = " the result and/or calls to `next_back_unchecked` on the `back` of the"] # [doc = " result will eventually reach the same edge."] # [doc = ""] # [doc = " If there are no such edges, i.e., if the tree contains no key within"] # [doc = " the range, returns an empty `front` and `back`."] # [doc = ""] # [doc = " # Safety"] # [doc = " Unless `BorrowType` is `Immut`, do not use the handles to visit the same"] # [doc = " KV twice."] unsafe fn find_leaf_edges_spanning_range < Q : ? Sized , R > (self , range : R ,) -> LeafRange < BorrowType , K , V > where Q : Ord , K : Borrow < Q > , R : RangeBounds < Q > , { match self . search_tree_for_bifurcation (& range) { Err (_) => LeafRange :: none () , Ok ((node , lower_edge_idx , upper_edge_idx , mut lower_child_bound , mut upper_child_bound ,)) => { let mut lower_edge = unsafe { Handle :: new_edge (ptr :: read (& node) , lower_edge_idx) } ; let mut upper_edge = unsafe { Handle :: new_edge (node , upper_edge_idx) } ; loop { match (lower_edge . force () , upper_edge . force ()) { (Leaf (f) , Leaf (b)) => return LeafRange { front : Some (f) , back : Some (b) } , (Internal (f) , Internal (b)) => { (lower_edge , lower_child_bound) = f . descend () . find_lower_bound_edge (lower_child_bound) ; (upper_edge , upper_child_bound) = b . descend () . find_upper_bound_edge (upper_child_bound) ; } _ => unreachable ! ("BTreeMap has different depths") , } } } } } }}}

macro_rules! full_range_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function full_range in module {}", module_path!());
    };
}

mkfn!{
    full_range_introspect!();
    fn full_range < BorrowType : marker :: BorrowType , K , V > (root1 : NodeRef < BorrowType , K , V , marker :: LeafOrInternal > , root2 : NodeRef < BorrowType , K , V , marker :: LeafOrInternal > ,) -> LazyLeafRange < BorrowType , K , V > { LazyLeafRange { front : Some (LazyLeafHandle :: Root (root1)) , back : Some (LazyLeafHandle :: Root (root2)) , } }
}
mkitem!{mkimpl!{impl < 'a , K : 'a , V : 'a > NodeRef < marker :: Immut < 'a > , K , V , marker :: LeafOrInternal > { # [doc = " Finds the pair of leaf edges delimiting a specific range in a tree."] # [doc = ""] # [doc = " The result is meaningful only if the tree is ordered by key, like the tree"] # [doc = " in a `BTreeMap` is."] pub (super) fn range_search < Q , R > (self , range : R) -> LeafRange < marker :: Immut < 'a > , K , V > where Q : ? Sized + Ord , K : Borrow < Q > , R : RangeBounds < Q > , { unsafe { self . find_leaf_edges_spanning_range (range) } } # [doc = " Finds the pair of leaf edges delimiting an entire tree."] pub (super) fn full_range (self) -> LazyLeafRange < marker :: Immut < 'a > , K , V > { full_range (self , self) } }}}
mkitem!{mkimpl!{impl < 'a , K : 'a , V : 'a > NodeRef < marker :: ValMut < 'a > , K , V , marker :: LeafOrInternal > { # [doc = " Splits a unique reference into a pair of leaf edges delimiting a specified range."] # [doc = " The result are non-unique references allowing (some) mutation, which must be used"] # [doc = " carefully."] # [doc = ""] # [doc = " The result is meaningful only if the tree is ordered by key, like the tree"] # [doc = " in a `BTreeMap` is."] # [doc = ""] # [doc = " # Safety"] # [doc = " Do not use the duplicate handles to visit the same KV twice."] pub (super) fn range_search < Q , R > (self , range : R) -> LeafRange < marker :: ValMut < 'a > , K , V > where Q : ? Sized + Ord , K : Borrow < Q > , R : RangeBounds < Q > , { unsafe { self . find_leaf_edges_spanning_range (range) } } # [doc = " Splits a unique reference into a pair of leaf edges delimiting the full range of the tree."] # [doc = " The results are non-unique references allowing mutation (of values only), so must be used"] # [doc = " with care."] pub (super) fn full_range (self) -> LazyLeafRange < marker :: ValMut < 'a > , K , V > { let self2 = unsafe { ptr :: read (& self) } ; full_range (self , self2) } }}}
mkitem!{mkimpl!{impl < K , V > NodeRef < marker :: Dying , K , V , marker :: LeafOrInternal > { # [doc = " Splits a unique reference into a pair of leaf edges delimiting the full range of the tree."] # [doc = " The results are non-unique references allowing massively destructive mutation, so must be"] # [doc = " used with the utmost care."] pub (super) fn full_range (self) -> LazyLeafRange < marker :: Dying , K , V > { let self2 = unsafe { ptr :: read (& self) } ; full_range (self , self2) } }}}
mkitem!{mkimpl!{impl < BorrowType : marker :: BorrowType , K , V > Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > { # [doc = " Given a leaf edge handle, returns [`Result::Ok`] with a handle to the neighboring KV"] # [doc = " on the right side, which is either in the same leaf node or in an ancestor node."] # [doc = " If the leaf edge is the last one in the tree, returns [`Result::Err`] with the root node."] pub (super) fn next_kv (self ,) -> Result < Handle < NodeRef < BorrowType , K , V , marker :: LeafOrInternal > , marker :: KV > , NodeRef < BorrowType , K , V , marker :: LeafOrInternal > , > { let mut edge = self . forget_node_type () ; loop { edge = match edge . right_kv () { Ok (kv) => return Ok (kv) , Err (last_edge) => match last_edge . into_node () . ascend () { Ok (parent_edge) => parent_edge . forget_node_type () , Err (root) => return Err (root) , } , } } } # [doc = " Given a leaf edge handle, returns [`Result::Ok`] with a handle to the neighboring KV"] # [doc = " on the left side, which is either in the same leaf node or in an ancestor node."] # [doc = " If the leaf edge is the first one in the tree, returns [`Result::Err`] with the root node."] pub (super) fn next_back_kv (self ,) -> Result < Handle < NodeRef < BorrowType , K , V , marker :: LeafOrInternal > , marker :: KV > , NodeRef < BorrowType , K , V , marker :: LeafOrInternal > , > { let mut edge = self . forget_node_type () ; loop { edge = match edge . left_kv () { Ok (kv) => return Ok (kv) , Err (last_edge) => match last_edge . into_node () . ascend () { Ok (parent_edge) => parent_edge . forget_node_type () , Err (root) => return Err (root) , } , } } } }}}
mkitem!{mkimpl!{impl < BorrowType : marker :: BorrowType , K , V > Handle < NodeRef < BorrowType , K , V , marker :: Internal > , marker :: Edge > { # [doc = " Given an internal edge handle, returns [`Result::Ok`] with a handle to the neighboring KV"] # [doc = " on the right side, which is either in the same internal node or in an ancestor node."] # [doc = " If the internal edge is the last one in the tree, returns [`Result::Err`] with the root node."] fn next_kv (self ,) -> Result < Handle < NodeRef < BorrowType , K , V , marker :: Internal > , marker :: KV > , NodeRef < BorrowType , K , V , marker :: Internal > , > { let mut edge = self ; loop { edge = match edge . right_kv () { Ok (internal_kv) => return Ok (internal_kv) , Err (last_edge) => match last_edge . into_node () . ascend () { Ok (parent_edge) => parent_edge , Err (root) => return Err (root) , } , } } } }}}
mkitem!{mkimpl!{impl < K , V > Handle < NodeRef < marker :: Dying , K , V , marker :: Leaf > , marker :: Edge > { # [doc = " Given a leaf edge handle into a dying tree, returns the next leaf edge"] # [doc = " on the right side, and the key-value pair in between, if they exist."] # [doc = ""] # [doc = " If the given edge is the last one in a leaf, this method deallocates"] # [doc = " the leaf, as well as any ancestor nodes whose last edge was reached."] # [doc = " This implies that if no more key-value pair follows, the entire tree"] # [doc = " will have been deallocated and there is nothing left to return."] # [doc = ""] # [doc = " # Safety"] # [doc = " - The given edge must not have been previously returned by counterpart"] # [doc = "   `deallocating_next_back`."] # [doc = " - The returned KV handle is only valid to access the key and value,"] # [doc = "   and only valid until the next call to a `deallocating_` method."] unsafe fn deallocating_next < A : Allocator + Clone > (self , alloc : A ,) -> Option < (Self , Handle < NodeRef < marker :: Dying , K , V , marker :: LeafOrInternal > , marker :: KV >) > { let mut edge = self . forget_node_type () ; loop { edge = match edge . right_kv () { Ok (kv) => return Some ((unsafe { ptr :: read (& kv) } . next_leaf_edge () , kv)) , Err (last_edge) => { match unsafe { last_edge . into_node () . deallocate_and_ascend (alloc . clone ()) } { Some (parent_edge) => parent_edge . forget_node_type () , None => return None , } } } } } # [doc = " Given a leaf edge handle into a dying tree, returns the next leaf edge"] # [doc = " on the left side, and the key-value pair in between, if they exist."] # [doc = ""] # [doc = " If the given edge is the first one in a leaf, this method deallocates"] # [doc = " the leaf, as well as any ancestor nodes whose first edge was reached."] # [doc = " This implies that if no more key-value pair follows, the entire tree"] # [doc = " will have been deallocated and there is nothing left to return."] # [doc = ""] # [doc = " # Safety"] # [doc = " - The given edge must not have been previously returned by counterpart"] # [doc = "   `deallocating_next`."] # [doc = " - The returned KV handle is only valid to access the key and value,"] # [doc = "   and only valid until the next call to a `deallocating_` method."] unsafe fn deallocating_next_back < A : Allocator + Clone > (self , alloc : A ,) -> Option < (Self , Handle < NodeRef < marker :: Dying , K , V , marker :: LeafOrInternal > , marker :: KV >) > { let mut edge = self . forget_node_type () ; loop { edge = match edge . left_kv () { Ok (kv) => return Some ((unsafe { ptr :: read (& kv) } . next_back_leaf_edge () , kv)) , Err (last_edge) => { match unsafe { last_edge . into_node () . deallocate_and_ascend (alloc . clone ()) } { Some (parent_edge) => parent_edge . forget_node_type () , None => return None , } } } } } # [doc = " Deallocates a pile of nodes from the leaf up to the root."] # [doc = " This is the only way to deallocate the remainder of a tree after"] # [doc = " `deallocating_next` and `deallocating_next_back` have been nibbling at"] # [doc = " both sides of the tree, and have hit the same edge. As it is intended"] # [doc = " only to be called when all keys and values have been returned,"] # [doc = " no cleanup is done on any of the keys or values."] fn deallocating_end < A : Allocator + Clone > (self , alloc : A) { let mut edge = self . forget_node_type () ; while let Some (parent_edge) = unsafe { edge . into_node () . deallocate_and_ascend (alloc . clone ()) } { edge = parent_edge . forget_node_type () ; } } }}}
mkitem!{mkimpl!{impl < 'a , K , V > Handle < NodeRef < marker :: Immut < 'a > , K , V , marker :: Leaf > , marker :: Edge > { # [doc = " Moves the leaf edge handle to the next leaf edge and returns references to the"] # [doc = " key and value in between."] # [doc = ""] # [doc = " # Safety"] # [doc = " There must be another KV in the direction travelled."] unsafe fn next_unchecked (& mut self) -> (& 'a K , & 'a V) { super :: mem :: replace (self , | leaf_edge | { let kv = leaf_edge . next_kv () . ok () . unwrap () ; (kv . next_leaf_edge () , kv . into_kv ()) }) } # [doc = " Moves the leaf edge handle to the previous leaf edge and returns references to the"] # [doc = " key and value in between."] # [doc = ""] # [doc = " # Safety"] # [doc = " There must be another KV in the direction travelled."] unsafe fn next_back_unchecked (& mut self) -> (& 'a K , & 'a V) { super :: mem :: replace (self , | leaf_edge | { let kv = leaf_edge . next_back_kv () . ok () . unwrap () ; (kv . next_back_leaf_edge () , kv . into_kv ()) }) } }}}
mkitem!{mkimpl!{impl < 'a , K , V > Handle < NodeRef < marker :: ValMut < 'a > , K , V , marker :: Leaf > , marker :: Edge > { # [doc = " Moves the leaf edge handle to the next leaf edge and returns references to the"] # [doc = " key and value in between."] # [doc = ""] # [doc = " # Safety"] # [doc = " There must be another KV in the direction travelled."] unsafe fn next_unchecked (& mut self) -> (& 'a K , & 'a mut V) { let kv = super :: mem :: replace (self , | leaf_edge | { let kv = leaf_edge . next_kv () . ok () . unwrap () ; (unsafe { ptr :: read (& kv) } . next_leaf_edge () , kv) }) ; kv . into_kv_valmut () } # [doc = " Moves the leaf edge handle to the previous leaf and returns references to the"] # [doc = " key and value in between."] # [doc = ""] # [doc = " # Safety"] # [doc = " There must be another KV in the direction travelled."] unsafe fn next_back_unchecked (& mut self) -> (& 'a K , & 'a mut V) { let kv = super :: mem :: replace (self , | leaf_edge | { let kv = leaf_edge . next_back_kv () . ok () . unwrap () ; (unsafe { ptr :: read (& kv) } . next_back_leaf_edge () , kv) }) ; kv . into_kv_valmut () } }}}
mkitem!{mkimpl!{impl < K , V > Handle < NodeRef < marker :: Dying , K , V , marker :: Leaf > , marker :: Edge > { # [doc = " Moves the leaf edge handle to the next leaf edge and returns the key and value"] # [doc = " in between, deallocating any node left behind while leaving the corresponding"] # [doc = " edge in its parent node dangling."] # [doc = ""] # [doc = " # Safety"] # [doc = " - There must be another KV in the direction travelled."] # [doc = " - That KV was not previously returned by counterpart"] # [doc = "   `deallocating_next_back_unchecked` on any copy of the handles"] # [doc = "   being used to traverse the tree."] # [doc = ""] # [doc = " The only safe way to proceed with the updated handle is to compare it, drop it,"] # [doc = " or call this method or counterpart `deallocating_next_back_unchecked` again."] unsafe fn deallocating_next_unchecked < A : Allocator + Clone > (& mut self , alloc : A ,) -> Handle < NodeRef < marker :: Dying , K , V , marker :: LeafOrInternal > , marker :: KV > { super :: mem :: replace (self , | leaf_edge | unsafe { leaf_edge . deallocating_next (alloc) . unwrap () }) } # [doc = " Moves the leaf edge handle to the previous leaf edge and returns the key and value"] # [doc = " in between, deallocating any node left behind while leaving the corresponding"] # [doc = " edge in its parent node dangling."] # [doc = ""] # [doc = " # Safety"] # [doc = " - There must be another KV in the direction travelled."] # [doc = " - That leaf edge was not previously returned by counterpart"] # [doc = "   `deallocating_next_unchecked` on any copy of the handles"] # [doc = "   being used to traverse the tree."] # [doc = ""] # [doc = " The only safe way to proceed with the updated handle is to compare it, drop it,"] # [doc = " or call this method or counterpart `deallocating_next_unchecked` again."] unsafe fn deallocating_next_back_unchecked < A : Allocator + Clone > (& mut self , alloc : A ,) -> Handle < NodeRef < marker :: Dying , K , V , marker :: LeafOrInternal > , marker :: KV > { super :: mem :: replace (self , | leaf_edge | unsafe { leaf_edge . deallocating_next_back (alloc) . unwrap () }) } }}}
mkitem!{mkimpl!{impl < BorrowType : marker :: BorrowType , K , V > NodeRef < BorrowType , K , V , marker :: LeafOrInternal > { # [doc = " Returns the leftmost leaf edge in or underneath a node - in other words, the edge"] # [doc = " you need first when navigating forward (or last when navigating backward)."] # [inline] pub (super) fn first_leaf_edge (self ,) -> Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > { let mut node = self ; loop { match node . force () { Leaf (leaf) => return leaf . first_edge () , Internal (internal) => node = internal . first_edge () . descend () , } } } # [doc = " Returns the rightmost leaf edge in or underneath a node - in other words, the edge"] # [doc = " you need last when navigating forward (or first when navigating backward)."] # [inline] pub (super) fn last_leaf_edge (self ,) -> Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > { let mut node = self ; loop { match node . force () { Leaf (leaf) => return leaf . last_edge () , Internal (internal) => node = internal . last_edge () . descend () , } } } }}}
mkitem!{mkenum!{pub (super) enum Position < BorrowType , K , V > { Leaf (NodeRef < BorrowType , K , V , marker :: Leaf >) , Internal (NodeRef < BorrowType , K , V , marker :: Internal >) , InternalKV , }}}
mkitem!{mkimpl!{impl < 'a , K : 'a , V : 'a > NodeRef < marker :: Immut < 'a > , K , V , marker :: LeafOrInternal > { # [doc = " Visits leaf nodes and internal KVs in order of ascending keys, and also"] # [doc = " visits internal nodes as a whole in a depth first order, meaning that"] # [doc = " internal nodes precede their individual KVs and their child nodes."] pub (super) fn visit_nodes_in_order < F > (self , mut visit : F) where F : FnMut (Position < marker :: Immut < 'a > , K , V >) , { match self . force () { Leaf (leaf) => visit (Position :: Leaf (leaf)) , Internal (internal) => { visit (Position :: Internal (internal)) ; let mut edge = internal . first_edge () ; loop { edge = match edge . descend () . force () { Leaf (leaf) => { visit (Position :: Leaf (leaf)) ; match edge . next_kv () { Ok (kv) => { visit (Position :: InternalKV) ; kv . right_edge () } Err (_) => return , } } Internal (internal) => { visit (Position :: Internal (internal)) ; internal . first_edge () } } } } } } # [doc = " Calculates the number of elements in a (sub)tree."] pub (super) fn calc_length (self) -> usize { let mut result = 0 ; self . visit_nodes_in_order (| pos | match pos { Position :: Leaf (node) => result += node . len () , Position :: Internal (node) => result += node . len () , Position :: InternalKV => () , }) ; result } }}}
mkitem!{mkimpl!{impl < BorrowType : marker :: BorrowType , K , V > Handle < NodeRef < BorrowType , K , V , marker :: LeafOrInternal > , marker :: KV > { # [doc = " Returns the leaf edge closest to a KV for forward navigation."] pub (super) fn next_leaf_edge (self ,) -> Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > { match self . force () { Leaf (leaf_kv) => leaf_kv . right_edge () , Internal (internal_kv) => { let next_internal_edge = internal_kv . right_edge () ; next_internal_edge . descend () . first_leaf_edge () } } } # [doc = " Returns the leaf edge closest to a KV for backward navigation."] pub (super) fn next_back_leaf_edge (self ,) -> Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > { match self . force () { Leaf (leaf_kv) => leaf_kv . left_edge () , Internal (internal_kv) => { let next_internal_edge = internal_kv . left_edge () ; next_internal_edge . descend () . last_leaf_edge () } } } }}}
mkitem!{mkimpl!{impl < BorrowType : marker :: BorrowType , K , V > NodeRef < BorrowType , K , V , marker :: LeafOrInternal > { # [doc = " Returns the leaf edge corresponding to the first point at which the"] # [doc = " given bound is true."] pub (super) fn lower_bound < Q : ? Sized > (self , mut bound : SearchBound < & Q > ,) -> Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > where Q : Ord , K : Borrow < Q > , { let mut node = self ; loop { let (edge , new_bound) = node . find_lower_bound_edge (bound) ; match edge . force () { Leaf (edge) => return edge , Internal (edge) => { node = edge . descend () ; bound = new_bound ; } } } } # [doc = " Returns the leaf edge corresponding to the last point at which the"] # [doc = " given bound is true."] pub (super) fn upper_bound < Q : ? Sized > (self , mut bound : SearchBound < & Q > ,) -> Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > where Q : Ord , K : Borrow < Q > , { let mut node = self ; loop { let (edge , new_bound) = node . find_upper_bound_edge (bound) ; match edge . force () { Leaf (edge) => return edge , Internal (edge) => { node = edge . descend () ; bound = new_bound ; } } } } }}}