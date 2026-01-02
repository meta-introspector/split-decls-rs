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
mkuse!{use crate :: error :: Error ;}
mkuse!{use crate :: intrinsics :: { unchecked_add , unchecked_mul , unchecked_sub } ;}
mkuse!{use crate :: mem :: SizedTypeProperties ;}
mkuse!{use crate :: ptr :: { Alignment , NonNull } ;}
mkuse!{use crate :: { assert_unsafe_precondition , fmt , mem } ;}

macro_rules! size_align_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function size_align in module {}", module_path!());
    };
}

mkfn!{
    size_align_introspect!();
    const fn size_align < T > () -> (usize , usize) { (size_of :: < T > () , align_of :: < T > ()) }
}
mkitem!{mkstruct!{# [doc = " Layout of a block of memory."] # [doc = ""] # [doc = " An instance of `Layout` describes a particular layout of memory."] # [doc = " You build a `Layout` up as an input to give to an allocator."] # [doc = ""] # [doc = " All layouts have an associated size and a power-of-two alignment. The size, when rounded up to"] # [doc = " the nearest multiple of `align`, does not overflow `isize` (i.e., the rounded value will always be"] # [doc = " less than or equal to `isize::MAX`)."] # [doc = ""] # [doc = " (Note that layouts are *not* required to have non-zero size,"] # [doc = " even though `GlobalAlloc` requires that all memory requests"] # [doc = " be non-zero in size. A caller must either ensure that conditions"] # [doc = " like this are met, use specific allocators with looser"] # [doc = " requirements, or use the more lenient `Allocator` interface.)"] # [stable (feature = "alloc_layout" , since = "1.28.0")] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] # [lang = "alloc_layout"] pub struct Layout { size : usize , align : Alignment , }}}
mkitem!{mkimpl!{impl Layout { # [doc = " Constructs a `Layout` from a given `size` and `align`,"] # [doc = " or returns `LayoutError` if any of the following conditions"] # [doc = " are not met:"] # [doc = ""] # [doc = " * `align` must not be zero,"] # [doc = ""] # [doc = " * `align` must be a power of two,"] # [doc = ""] # [doc = " * `size`, when rounded up to the nearest multiple of `align`,"] # [doc = "   must not overflow `isize` (i.e., the rounded value must be"] # [doc = "   less than or equal to `isize::MAX`)."] # [stable (feature = "alloc_layout" , since = "1.28.0")] # [rustc_const_stable (feature = "const_alloc_layout_size_align" , since = "1.50.0")] # [inline] pub const fn from_size_align (size : usize , align : usize) -> Result < Self , LayoutError > { if Layout :: is_size_align_valid (size , align) { unsafe { Ok (Layout { size , align : mem :: transmute (align) }) } } else { Err (LayoutError) } } const fn is_size_align_valid (size : usize , align : usize) -> bool { let Some (align) = Alignment :: new (align) else { return false } ; if size > Self :: max_size_for_align (align) { return false ; } true } # [inline (always)] const fn max_size_for_align (align : Alignment) -> usize { unsafe { unchecked_sub (isize :: MAX as usize + 1 , align . as_usize ()) } } # [doc = " Internal helper constructor to skip revalidating alignment validity."] # [inline] const fn from_size_alignment (size : usize , align : Alignment) -> Result < Self , LayoutError > { if size > Self :: max_size_for_align (align) { return Err (LayoutError) ; } Ok (Layout { size , align }) } # [doc = " Creates a layout, bypassing all checks."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is unsafe as it does not verify the preconditions from"] # [doc = " [`Layout::from_size_align`]."] # [stable (feature = "alloc_layout" , since = "1.28.0")] # [rustc_const_stable (feature = "const_alloc_layout_unchecked" , since = "1.36.0")] # [must_use] # [inline] # [track_caller] pub const unsafe fn from_size_align_unchecked (size : usize , align : usize) -> Self { assert_unsafe_precondition ! (check_library_ub , "Layout::from_size_align_unchecked requires that align is a power of 2 \
            and the rounded-up allocation size does not exceed isize::MAX" , (size : usize = size , align : usize = align ,) => Layout :: is_size_align_valid (size , align)) ; unsafe { Layout { size , align : mem :: transmute (align) } } } # [doc = " The minimum size in bytes for a memory block of this layout."] # [stable (feature = "alloc_layout" , since = "1.28.0")] # [rustc_const_stable (feature = "const_alloc_layout_size_align" , since = "1.50.0")] # [must_use] # [inline] pub const fn size (& self) -> usize { self . size } # [doc = " The minimum byte alignment for a memory block of this layout."] # [doc = ""] # [doc = " The returned alignment is guaranteed to be a power of two."] # [stable (feature = "alloc_layout" , since = "1.28.0")] # [rustc_const_stable (feature = "const_alloc_layout_size_align" , since = "1.50.0")] # [must_use = "this returns the minimum alignment, \
                  without modifying the layout"] # [inline] pub const fn align (& self) -> usize { self . align . as_usize () } # [doc = " Constructs a `Layout` suitable for holding a value of type `T`."] # [stable (feature = "alloc_layout" , since = "1.28.0")] # [rustc_const_stable (feature = "alloc_layout_const_new" , since = "1.42.0")] # [must_use] # [inline] pub const fn new < T > () -> Self { let (size , align) = size_align :: < T > () ; unsafe { Layout :: from_size_align_unchecked (size , align) } } # [doc = " Produces layout describing a record that could be used to"] # [doc = " allocate backing structure for `T` (which could be a trait"] # [doc = " or other unsized type like a slice)."] # [stable (feature = "alloc_layout" , since = "1.28.0")] # [rustc_const_stable (feature = "const_alloc_layout" , since = "1.85.0")] # [must_use] # [inline] pub const fn for_value < T : ? Sized > (t : & T) -> Self { let (size , align) = (size_of_val (t) , align_of_val (t)) ; unsafe { Layout :: from_size_align_unchecked (size , align) } } # [doc = " Produces layout describing a record that could be used to"] # [doc = " allocate backing structure for `T` (which could be a trait"] # [doc = " or other unsized type like a slice)."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is only safe to call if the following conditions hold:"] # [doc = ""] # [doc = " - If `T` is `Sized`, this function is always safe to call."] # [doc = " - If the unsized tail of `T` is:"] # [doc = "     - a [slice], then the length of the slice tail must be an initialized"] # [doc = "       integer, and the size of the *entire value*"] # [doc = "       (dynamic tail length + statically sized prefix) must fit in `isize`."] # [doc = "       For the special case where the dynamic tail length is 0, this function"] # [doc = "       is safe to call."] # [doc = "     - a [trait object], then the vtable part of the pointer must point"] # [doc = "       to a valid vtable for the type `T` acquired by an unsizing coercion,"] # [doc = "       and the size of the *entire value*"] # [doc = "       (dynamic tail length + statically sized prefix) must fit in `isize`."] # [doc = "     - an (unstable) [extern type], then this function is always safe to"] # [doc = "       call, but may panic or otherwise return the wrong value, as the"] # [doc = "       extern type's layout is not known. This is the same behavior as"] # [doc = "       [`Layout::for_value`] on a reference to an extern type tail."] # [doc = "     - otherwise, it is conservatively not allowed to call this function."] # [doc = ""] # [doc = " [trait object]: ../../book/ch17-02-trait-objects.html"] # [doc = " [extern type]: ../../unstable-book/language-features/extern-types.html"] # [unstable (feature = "layout_for_ptr" , issue = "69835")] # [must_use] pub const unsafe fn for_value_raw < T : ? Sized > (t : * const T) -> Self { let (size , align) = unsafe { (mem :: size_of_val_raw (t) , mem :: align_of_val_raw (t)) } ; unsafe { Layout :: from_size_align_unchecked (size , align) } } # [doc = " Creates a `NonNull` that is dangling, but well-aligned for this Layout."] # [doc = ""] # [doc = " Note that the address of the returned pointer may potentially"] # [doc = " be that of a valid pointer, which means this must not be used"] # [doc = " as a \"not yet initialized\" sentinel value."] # [doc = " Types that lazily allocate must track initialization by some other means."] # [unstable (feature = "alloc_layout_extra" , issue = "55724")] # [must_use] # [inline] pub const fn dangling (& self) -> NonNull < u8 > { NonNull :: without_provenance (self . align . as_nonzero ()) } # [doc = " Creates a layout describing the record that can hold a value"] # [doc = " of the same layout as `self`, but that also is aligned to"] # [doc = " alignment `align` (measured in bytes)."] # [doc = ""] # [doc = " If `self` already meets the prescribed alignment, then returns"] # [doc = " `self`."] # [doc = ""] # [doc = " Note that this method does not add any padding to the overall"] # [doc = " size, regardless of whether the returned layout has a different"] # [doc = " alignment. In other words, if `K` has size 16, `K.align_to(32)`"] # [doc = " will *still* have size 16."] # [doc = ""] # [doc = " Returns an error if the combination of `self.size()` and the given"] # [doc = " `align` violates the conditions listed in [`Layout::from_size_align`]."] # [stable (feature = "alloc_layout_manipulation" , since = "1.44.0")] # [rustc_const_stable (feature = "const_alloc_layout" , since = "1.85.0")] # [inline] pub const fn align_to (& self , align : usize) -> Result < Self , LayoutError > { if let Some (align) = Alignment :: new (align) { Layout :: from_size_alignment (self . size , Alignment :: max (self . align , align)) } else { Err (LayoutError) } } # [doc = " Returns the amount of padding we must insert after `self`"] # [doc = " to ensure that the following address will satisfy `align`"] # [doc = " (measured in bytes)."] # [doc = ""] # [doc = " e.g., if `self.size()` is 9, then `self.padding_needed_for(4)`"] # [doc = " returns 3, because that is the minimum number of bytes of"] # [doc = " padding required to get a 4-aligned address (assuming that the"] # [doc = " corresponding memory block starts at a 4-aligned address)."] # [doc = ""] # [doc = " The return value of this function has no meaning if `align` is"] # [doc = " not a power-of-two."] # [doc = ""] # [doc = " Note that the utility of the returned value requires `align`"] # [doc = " to be less than or equal to the alignment of the starting"] # [doc = " address for the whole allocated block of memory. One way to"] # [doc = " satisfy this constraint is to ensure `align <= self.align()`."] # [unstable (feature = "alloc_layout_extra" , issue = "55724")] # [must_use = "this returns the padding needed, \
                  without modifying the `Layout`"] # [inline] pub const fn padding_needed_for (& self , align : usize) -> usize { let Some (align) = Alignment :: new (align) else { return usize :: MAX } ; let len_rounded_up = self . size_rounded_up_to_custom_align (align) ; unsafe { unchecked_sub (len_rounded_up , self . size) } } # [doc = " Returns the smallest multiple of `align` greater than or equal to `self.size()`."] # [doc = ""] # [doc = " This can return at most `Alignment::MAX` (aka `isize::MAX + 1`)"] # [doc = " because the original size is at most `isize::MAX`."] # [inline] const fn size_rounded_up_to_custom_align (& self , align : Alignment) -> usize { unsafe { let align_m1 = unchecked_sub (align . as_usize () , 1) ; let size_rounded_up = unchecked_add (self . size , align_m1) & ! align_m1 ; size_rounded_up } } # [doc = " Creates a layout by rounding the size of this layout up to a multiple"] # [doc = " of the layout's alignment."] # [doc = ""] # [doc = " This is equivalent to adding the result of `padding_needed_for`"] # [doc = " to the layout's current size."] # [stable (feature = "alloc_layout_manipulation" , since = "1.44.0")] # [rustc_const_stable (feature = "const_alloc_layout" , since = "1.85.0")] # [must_use = "this returns a new `Layout`, \
                  without modifying the original"] # [inline] pub const fn pad_to_align (& self) -> Layout { let new_size = self . size_rounded_up_to_custom_align (self . align) ; unsafe { Layout :: from_size_align_unchecked (new_size , self . align ()) } } # [doc = " Creates a layout describing the record for `n` instances of"] # [doc = " `self`, with a suitable amount of padding between each to"] # [doc = " ensure that each instance is given its requested size and"] # [doc = " alignment. On success, returns `(k, offs)` where `k` is the"] # [doc = " layout of the array and `offs` is the distance between the start"] # [doc = " of each element in the array."] # [doc = ""] # [doc = " (That distance between elements is sometimes known as \"stride\".)"] # [doc = ""] # [doc = " On arithmetic overflow, returns `LayoutError`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(alloc_layout_extra)]"] # [doc = " use std::alloc::Layout;"] # [doc = ""] # [doc = " // All rust types have a size that's a multiple of their alignment."] # [doc = " let normal = Layout::from_size_align(12, 4).unwrap();"] # [doc = " let repeated = normal.repeat(3).unwrap();"] # [doc = " assert_eq!(repeated, (Layout::from_size_align(36, 4).unwrap(), 12));"] # [doc = ""] # [doc = " // But you can manually make layouts which don't meet that rule."] # [doc = " let padding_needed = Layout::from_size_align(6, 4).unwrap();"] # [doc = " let repeated = padding_needed.repeat(3).unwrap();"] # [doc = " assert_eq!(repeated, (Layout::from_size_align(24, 4).unwrap(), 8));"] # [doc = " ```"] # [unstable (feature = "alloc_layout_extra" , issue = "55724")] # [inline] pub const fn repeat (& self , n : usize) -> Result < (Self , usize) , LayoutError > { let padded = self . pad_to_align () ; if let Ok (repeated) = padded . repeat_packed (n) { Ok ((repeated , padded . size ())) } else { Err (LayoutError) } } # [doc = " Creates a layout describing the record for `self` followed by"] # [doc = " `next`, including any necessary padding to ensure that `next`"] # [doc = " will be properly aligned, but *no trailing padding*."] # [doc = ""] # [doc = " In order to match C representation layout `repr(C)`, you should"] # [doc = " call `pad_to_align` after extending the layout with all fields."] # [doc = " (There is no way to match the default Rust representation"] # [doc = " layout `repr(Rust)`, as it is unspecified.)"] # [doc = ""] # [doc = " Note that the alignment of the resulting layout will be the maximum of"] # [doc = " those of `self` and `next`, in order to ensure alignment of both parts."] # [doc = ""] # [doc = " Returns `Ok((k, offset))`, where `k` is layout of the concatenated"] # [doc = " record and `offset` is the relative location, in bytes, of the"] # [doc = " start of the `next` embedded within the concatenated record"] # [doc = " (assuming that the record itself starts at offset 0)."] # [doc = ""] # [doc = " On arithmetic overflow, returns `LayoutError`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " To calculate the layout of a `#[repr(C)]` structure and the offsets of"] # [doc = " the fields from its fields' layouts:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use std::alloc::{Layout, LayoutError};"] # [doc = " pub fn repr_c(fields: &[Layout]) -> Result<(Layout, Vec<usize>), LayoutError> {"] # [doc = "     let mut offsets = Vec::new();"] # [doc = "     let mut layout = Layout::from_size_align(0, 1)?;"] # [doc = "     for &field in fields {"] # [doc = "         let (new_layout, offset) = layout.extend(field)?;"] # [doc = "         layout = new_layout;"] # [doc = "         offsets.push(offset);"] # [doc = "     }"] # [doc = "     // Remember to finalize with `pad_to_align`!"] # [doc = "     Ok((layout.pad_to_align(), offsets))"] # [doc = " }"] # [doc = " # // test that it works"] # [doc = " # #[repr(C)] struct S { a: u64, b: u32, c: u16, d: u32 }"] # [doc = " # let s = Layout::new::<S>();"] # [doc = " # let u16 = Layout::new::<u16>();"] # [doc = " # let u32 = Layout::new::<u32>();"] # [doc = " # let u64 = Layout::new::<u64>();"] # [doc = " # assert_eq!(repr_c(&[u64, u32, u16, u32]), Ok((s, vec![0, 8, 12, 16])));"] # [doc = " ```"] # [stable (feature = "alloc_layout_manipulation" , since = "1.44.0")] # [rustc_const_stable (feature = "const_alloc_layout" , since = "1.85.0")] # [inline] pub const fn extend (& self , next : Self) -> Result < (Self , usize) , LayoutError > { let new_align = Alignment :: max (self . align , next . align) ; let offset = self . size_rounded_up_to_custom_align (next . align) ; let new_size = unsafe { unchecked_add (offset , next . size) } ; if let Ok (layout) = Layout :: from_size_alignment (new_size , new_align) { Ok ((layout , offset)) } else { Err (LayoutError) } } # [doc = " Creates a layout describing the record for `n` instances of"] # [doc = " `self`, with no padding between each instance."] # [doc = ""] # [doc = " Note that, unlike `repeat`, `repeat_packed` does not guarantee"] # [doc = " that the repeated instances of `self` will be properly"] # [doc = " aligned, even if a given instance of `self` is properly"] # [doc = " aligned. In other words, if the layout returned by"] # [doc = " `repeat_packed` is used to allocate an array, it is not"] # [doc = " guaranteed that all elements in the array will be properly"] # [doc = " aligned."] # [doc = ""] # [doc = " On arithmetic overflow, returns `LayoutError`."] # [unstable (feature = "alloc_layout_extra" , issue = "55724")] # [inline] pub const fn repeat_packed (& self , n : usize) -> Result < Self , LayoutError > { if let Some (size) = self . size . checked_mul (n) { Layout :: from_size_alignment (size , self . align) } else { Err (LayoutError) } } # [doc = " Creates a layout describing the record for `self` followed by"] # [doc = " `next` with no additional padding between the two. Since no"] # [doc = " padding is inserted, the alignment of `next` is irrelevant,"] # [doc = " and is not incorporated *at all* into the resulting layout."] # [doc = ""] # [doc = " On arithmetic overflow, returns `LayoutError`."] # [unstable (feature = "alloc_layout_extra" , issue = "55724")] # [inline] pub const fn extend_packed (& self , next : Self) -> Result < Self , LayoutError > { let new_size = unsafe { unchecked_add (self . size , next . size) } ; Layout :: from_size_alignment (new_size , self . align) } # [doc = " Creates a layout describing the record for a `[T; n]`."] # [doc = ""] # [doc = " On arithmetic overflow or when the total size would exceed"] # [doc = " `isize::MAX`, returns `LayoutError`."] # [stable (feature = "alloc_layout_manipulation" , since = "1.44.0")] # [rustc_const_stable (feature = "const_alloc_layout" , since = "1.85.0")] # [inline] pub const fn array < T > (n : usize) -> Result < Self , LayoutError > { return inner (T :: LAYOUT , n) ; # [inline] const fn inner (element_layout : Layout , n : usize) -> Result < Layout , LayoutError > { let Layout { size : element_size , align } = element_layout ; if element_size != 0 && n > Layout :: max_size_for_align (align) / element_size { return Err (LayoutError) ; } let array_size = unsafe { unchecked_mul (element_size , n) } ; unsafe { Ok (Layout :: from_size_align_unchecked (array_size , align . as_usize ())) } } } # [doc = " Perma-unstable access to `align` as `Alignment` type."] # [unstable (issue = "none" , feature = "std_internals")] # [doc (hidden)] # [inline] pub const fn alignment (& self) -> Alignment { self . align } }}}
mkitem!{# [stable (feature = "alloc_layout" , since = "1.28.0")] # [deprecated (since = "1.52.0" , note = "Name does not follow std convention, use LayoutError" , suggestion = "LayoutError")] pub type LayoutErr = LayoutError ;}
mkitem!{mkstruct!{# [doc = " The `LayoutError` is returned when the parameters given"] # [doc = " to `Layout::from_size_align`"] # [doc = " or some other `Layout` constructor"] # [doc = " do not satisfy its documented constraints."] # [stable (feature = "alloc_layout_error" , since = "1.50.0")] # [non_exhaustive] # [derive (Clone , PartialEq , Eq , Debug)] pub struct LayoutError ;}}
mkitem!{mkimpl!{# [stable (feature = "alloc_layout" , since = "1.28.0")] impl Error for LayoutError { }}}
mkitem!{mkimpl!{# [stable (feature = "alloc_layout" , since = "1.28.0")] impl fmt :: Display for LayoutError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("invalid parameters to Layout::from_size_align") } }}}