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
mkuse!{use std :: alloc :: Layout ;}
mkuse!{use std :: cmp :: Ordering ;}
mkuse!{use std :: hash :: { Hash , Hasher } ;}
mkuse!{use std :: ops :: Deref ;}
mkuse!{use std :: { fmt , iter , mem , ptr , slice } ;}
mkuse!{use rustc_data_structures :: aligned :: { Aligned , align_of } ;}
mkuse!{use rustc_data_structures :: sync :: DynSync ;}
mkuse!{use rustc_serialize :: { Encodable , Encoder } ;}
mkuse!{use rustc_type_ir :: FlagComputation ;}
mkuse!{use super :: { DebruijnIndex , TyCtxt , TypeFlags } ;}
mkuse!{use crate :: arena :: Arena ;}
mkitem!{# [doc = " `List<T>` is a bit like `&[T]`, but with some critical differences."] # [doc = " - IMPORTANT: Every `List<T>` is *required* to have unique contents. The"] # [doc = "   type's correctness relies on this, *but it does not enforce it*."] # [doc = "   Therefore, any code that creates a `List<T>` must ensure uniqueness"] # [doc = "   itself. In practice this is achieved by interning."] # [doc = " - The length is stored within the `List<T>`, so `&List<Ty>` is a thin"] # [doc = "   pointer."] # [doc = " - Because of this, you cannot get a `List<T>` that is a sub-list of another"] # [doc = "   `List<T>`. You can get a sub-slice `&[T]`, however."] # [doc = " - `List<T>` can be used with `TaggedRef`, which is useful within"] # [doc = "   structs whose size must be minimized."] # [doc = " - Because of the uniqueness assumption, we can use the address of a"] # [doc = "   `List<T>` for faster equality comparisons and hashing."] # [doc = " - `T` must be `Copy`. This lets `List<T>` be stored in a dropless arena and"] # [doc = "   iterators return a `T` rather than a `&T`."] # [doc = " - `T` must not be zero-sized."] pub type List < T > = RawList < () , T > ;}
mkitem!{mkstruct!{# [doc = " A generic type that can be used to prepend a [`List`] with some header."] # [doc = ""] # [doc = " The header will be ignored for value-based operations like [`PartialEq`],"] # [doc = " [`Hash`] and [`Encodable`]."] # [repr (C)] pub struct RawList < H , T > { skel : ListSkeleton < H , T > , opaque : OpaqueListContents , }}}
mkitem!{mkstruct!{# [doc = " A [`RawList`] without the unsized tail. This type is used for layout computation"] # [doc = " and constructing empty lists."] # [repr (C)] struct ListSkeleton < H , T > { header : H , len : usize , # [doc = " Although this claims to be a zero-length array, in practice `len`"] # [doc = " elements are actually present."] data : [T ; 0] , }}}
mkitem!{mkimpl!{impl < T > Default for & List < T > { fn default () -> Self { List :: empty () } }}}
mkitem!{unsafe extern "C" { # [doc = " A dummy type used to force `List` to be unsized while not requiring"] # [doc = " references to it be wide pointers."] type OpaqueListContents ; }}
mkitem!{mkimpl!{impl < H , T > RawList < H , T > { # [inline (always)] pub fn len (& self) -> usize { self . skel . len } # [inline (always)] pub fn as_slice (& self) -> & [T] { self } # [doc = " Allocates a list from `arena` and copies the contents of `slice` into it."] # [doc = ""] # [doc = " WARNING: the contents *must be unique*, such that no list with these"] # [doc = " contents has been previously created. If not, operations such as `eq`"] # [doc = " and `hash` might give incorrect results."] # [doc = ""] # [doc = " Panics if `T` is `Drop`, or `T` is zero-sized, or the slice is empty"] # [doc = " (because the empty list exists statically, and is available via"] # [doc = " `empty()`)."] # [inline] pub (super) fn from_arena < 'tcx > (arena : & 'tcx Arena < 'tcx > , header : H , slice : & [T] ,) -> & 'tcx RawList < H , T > where T : Copy , { assert ! (! mem :: needs_drop ::< T > ()) ; assert ! (size_of ::< T > () != 0) ; assert ! (! slice . is_empty ()) ; let (layout , _offset) = Layout :: new :: < ListSkeleton < H , T > > () . extend (Layout :: for_value :: < [T] > (slice)) . unwrap () ; let mem = arena . dropless . alloc_raw (layout) as * mut RawList < H , T > ; unsafe { (& raw mut (* mem) . skel . header) . write (header) ; (& raw mut (* mem) . skel . len) . write (slice . len ()) ; (& raw mut (* mem) . skel . data) . cast :: < T > () . copy_from_nonoverlapping (slice . as_ptr () , slice . len ()) ; & * mem } } # [inline (always)] pub fn iter (& self) -> < & '_ RawList < H , T > as IntoIterator > :: IntoIter where T : Copy , { self . into_iter () } }}}
mkitem!{mkimpl!{impl < 'a , H , T : Copy > rustc_type_ir :: inherent :: SliceLike for & 'a RawList < H , T > { type Item = T ; type IntoIter = iter :: Copied < < & 'a [T] as IntoIterator > :: IntoIter > ; fn iter (self) -> Self :: IntoIter { (* self) . iter () } fn as_slice (& self) -> & [Self :: Item] { (* self) . as_slice () } }}}
mkitem!{macro_rules ! impl_list_empty { ($ header_ty : ty , $ header_init : expr) => { impl < T > RawList <$ header_ty , T > { # [doc = " Returns a reference to the (per header unique, static) empty list."] # [inline (always)] pub fn empty <'a > () -> &'a RawList <$ header_ty , T > { # [repr (align (64))] struct MaxAlign ; static EMPTY : ListSkeleton <$ header_ty , MaxAlign > = ListSkeleton { header : $ header_init , len : 0 , data : [] } ; assert ! (align_of ::< T > () <= align_of ::< MaxAlign > ()) ; unsafe { &* ((& raw const EMPTY) as * const RawList <$ header_ty , T >) } } } } ; }}
mkitem!{impl_list_empty ! (() , ()) ;}
mkitem!{mkimpl!{impl < H , T : fmt :: Debug > fmt :: Debug for RawList < H , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }}}
mkitem!{mkimpl!{impl < H , S : Encoder , T : Encodable < S > > Encodable < S > for RawList < H , T > { # [inline] fn encode (& self , s : & mut S) { (* * self) . encode (s) ; } }}}
mkitem!{mkimpl!{impl < H , T : PartialEq > PartialEq for RawList < H , T > { # [inline] fn eq (& self , other : & RawList < H , T >) -> bool { ptr :: eq (self , other) } }}}
mkitem!{mkimpl!{impl < H , T : Eq > Eq for RawList < H , T > { }}}
mkitem!{mkimpl!{impl < H , T > Ord for RawList < H , T > where T : Ord , { fn cmp (& self , other : & RawList < H , T >) -> Ordering { if self == other { Ordering :: Equal } else { < [T] as Ord > :: cmp (& * * self , & * * other) } } }}}
mkitem!{mkimpl!{impl < H , T > PartialOrd for RawList < H , T > where T : PartialOrd , { fn partial_cmp (& self , other : & RawList < H , T >) -> Option < Ordering > { if self == other { Some (Ordering :: Equal) } else { < [T] as PartialOrd > :: partial_cmp (& * * self , & * * other) } } }}}
mkitem!{mkimpl!{impl < Hdr , T > Hash for RawList < Hdr , T > { # [inline] fn hash < H : Hasher > (& self , s : & mut H) { ptr :: from_ref (self) . hash (s) } }}}
mkitem!{mkimpl!{impl < H , T > Deref for RawList < H , T > { type Target = [T] ; # [inline (always)] fn deref (& self) -> & [T] { self . as_ref () } }}}
mkitem!{mkimpl!{impl < H , T > AsRef < [T] > for RawList < H , T > { # [inline (always)] fn as_ref (& self) -> & [T] { let data_ptr = (& raw const self . skel . data) . cast :: < T > () ; unsafe { slice :: from_raw_parts (data_ptr , self . skel . len) } } }}}
mkitem!{mkimpl!{impl < 'a , H , T : Copy > IntoIterator for & 'a RawList < H , T > { type Item = T ; type IntoIter = iter :: Copied < < & 'a [T] as IntoIterator > :: IntoIter > ; # [inline (always)] fn into_iter (self) -> Self :: IntoIter { self [..] . iter () . copied () } }}}
mkitem!{mkimpl!{unsafe impl < H : Sync , T : Sync > Sync for RawList < H , T > { }}}
mkitem!{mkimpl!{unsafe impl < H : DynSync , T : DynSync > DynSync for RawList < H , T > { }}}
mkitem!{mkimpl!{unsafe impl < H , T > Aligned for RawList < H , T > { const ALIGN : ptr :: Alignment = align_of :: < ListSkeleton < H , T > > () ; }}}
mkitem!{# [doc = " A [`List`] that additionally stores type information inline to speed up"] # [doc = " [`TypeVisitableExt`](super::TypeVisitableExt) operations."] pub type ListWithCachedTypeInfo < T > = RawList < TypeInfo , T > ;}
mkitem!{mkimpl!{impl < T > ListWithCachedTypeInfo < T > { # [inline (always)] pub fn flags (& self) -> TypeFlags { self . skel . header . flags } # [inline (always)] pub fn outer_exclusive_binder (& self) -> DebruijnIndex { self . skel . header . outer_exclusive_binder } }}}
mkitem!{impl_list_empty ! (TypeInfo , TypeInfo :: empty ()) ;}
mkitem!{mkstruct!{# [doc = " The additional info that is stored in [`ListWithCachedTypeInfo`]."] # [repr (C)] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct TypeInfo { flags : TypeFlags , outer_exclusive_binder : DebruijnIndex , }}}
mkitem!{mkimpl!{impl TypeInfo { const fn empty () -> Self { Self { flags : TypeFlags :: empty () , outer_exclusive_binder : super :: INNERMOST } } }}}
mkitem!{mkimpl!{impl < 'tcx > From < FlagComputation < TyCtxt < 'tcx > > > for TypeInfo { fn from (computation : FlagComputation < TyCtxt < 'tcx > >) -> TypeInfo { TypeInfo { flags : computation . flags , outer_exclusive_binder : computation . outer_exclusive_binder , } } }}}