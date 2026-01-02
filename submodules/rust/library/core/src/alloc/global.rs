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
mkuse!{use crate :: alloc :: Layout ;}
mkuse!{use crate :: { cmp , ptr } ;}
mkitem!{mktrait!{# [doc = " A memory allocator that can be registered as the standard library’s default"] # [doc = " through the `#[global_allocator]` attribute."] # [doc = ""] # [doc = " Some of the methods require that a memory block be *currently"] # [doc = " allocated* via an allocator. This means that:"] # [doc = ""] # [doc = " * the starting address for that memory block was previously"] # [doc = "   returned by a previous call to an allocation method"] # [doc = "   such as `alloc`, and"] # [doc = ""] # [doc = " * the memory block has not been subsequently deallocated, where"] # [doc = "   blocks are deallocated either by being passed to a deallocation"] # [doc = "   method such as `dealloc` or by being"] # [doc = "   passed to a reallocation method that returns a non-null pointer."] # [doc = ""] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::alloc::{GlobalAlloc, Layout};"] # [doc = " use std::cell::UnsafeCell;"] # [doc = " use std::ptr::null_mut;"] # [doc = " use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};"] # [doc = ""] # [doc = " const ARENA_SIZE: usize = 128 * 1024;"] # [doc = " const MAX_SUPPORTED_ALIGN: usize = 4096;"] # [doc = " #[repr(C, align(4096))] // 4096 == MAX_SUPPORTED_ALIGN"] # [doc = " struct SimpleAllocator {"] # [doc = "     arena: UnsafeCell<[u8; ARENA_SIZE]>,"] # [doc = "     remaining: AtomicUsize, // we allocate from the top, counting down"] # [doc = " }"] # [doc = ""] # [doc = " #[global_allocator]"] # [doc = " static ALLOCATOR: SimpleAllocator = SimpleAllocator {"] # [doc = "     arena: UnsafeCell::new([0x55; ARENA_SIZE]),"] # [doc = "     remaining: AtomicUsize::new(ARENA_SIZE),"] # [doc = " };"] # [doc = ""] # [doc = " unsafe impl Sync for SimpleAllocator {}"] # [doc = ""] # [doc = " unsafe impl GlobalAlloc for SimpleAllocator {"] # [doc = "     unsafe fn alloc(&self, layout: Layout) -> *mut u8 {"] # [doc = "         let size = layout.size();"] # [doc = "         let align = layout.align();"] # [doc = ""] # [doc = "         // `Layout` contract forbids making a `Layout` with align=0, or align not power of 2."] # [doc = "         // So we can safely use a mask to ensure alignment without worrying about UB."] # [doc = "         let align_mask_to_round_down = !(align - 1);"] # [doc = ""] # [doc = "         if align > MAX_SUPPORTED_ALIGN {"] # [doc = "             return null_mut();"] # [doc = "         }"] # [doc = ""] # [doc = "         let mut allocated = 0;"] # [doc = "         if self"] # [doc = "             .remaining"] # [doc = "             .fetch_update(Relaxed, Relaxed, |mut remaining| {"] # [doc = "                 if size > remaining {"] # [doc = "                     return None;"] # [doc = "                 }"] # [doc = "                 remaining -= size;"] # [doc = "                 remaining &= align_mask_to_round_down;"] # [doc = "                 allocated = remaining;"] # [doc = "                 Some(remaining)"] # [doc = "             })"] # [doc = "             .is_err()"] # [doc = "         {"] # [doc = "             return null_mut();"] # [doc = "         };"] # [doc = "         unsafe { self.arena.get().cast::<u8>().add(allocated) }"] # [doc = "     }"] # [doc = "     unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let _s = format!(\"allocating a string!\");"] # [doc = "     let currently = ALLOCATOR.remaining.load(Relaxed);"] # [doc = "     println!(\"allocated so far: {}\", ARENA_SIZE - currently);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `GlobalAlloc` trait is an `unsafe` trait for a number of reasons, and"] # [doc = " implementors must ensure that they adhere to these contracts:"] # [doc = ""] # [doc = " * It's undefined behavior if global allocators unwind. This restriction may"] # [doc = "   be lifted in the future, but currently a panic from any of these"] # [doc = "   functions may lead to memory unsafety."] # [doc = ""] # [doc = " * `Layout` queries and calculations in general must be correct. Callers of"] # [doc = "   this trait are allowed to rely on the contracts defined on each method,"] # [doc = "   and implementors must ensure such contracts remain true."] # [doc = ""] # [doc = " * You must not rely on allocations actually happening, even if there are explicit"] # [doc = "   heap allocations in the source. The optimizer may detect unused allocations that it can either"] # [doc = "   eliminate entirely or move to the stack and thus never invoke the allocator. The"] # [doc = "   optimizer may further assume that allocation is infallible, so code that used to fail due"] # [doc = "   to allocator failures may now suddenly work because the optimizer worked around the"] # [doc = "   need for an allocation. More concretely, the following code example is unsound, irrespective"] # [doc = "   of whether your custom allocator allows counting how many allocations have happened."] # [doc = ""] # [doc = "   ```rust,ignore (unsound and has placeholders)"] # [doc = "   drop(Box::new(42));"] # [doc = "   let number_of_heap_allocs = /* call private allocator API */;"] # [doc = "   unsafe { std::hint::assert_unchecked(number_of_heap_allocs > 0); }"] # [doc = "   ```"] # [doc = ""] # [doc = "   Note that the optimizations mentioned above are not the only"] # [doc = "   optimization that can be applied. You may generally not rely on heap allocations"] # [doc = "   happening if they can be removed without changing program behavior."] # [doc = "   Whether allocations happen or not is not part of the program behavior, even if it"] # [doc = "   could be detected via an allocator that tracks allocations by printing or otherwise"] # [doc = "   having side effects."] # [stable (feature = "global_alloc" , since = "1.28.0")] pub unsafe trait GlobalAlloc { # [doc = " Allocates memory as described by the given `layout`."] # [doc = ""] # [doc = " Returns a pointer to newly-allocated memory,"] # [doc = " or null to indicate allocation failure."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `layout` must have non-zero size. Attempting to allocate for a zero-sized `layout` may"] # [doc = " result in undefined behavior."] # [doc = ""] # [doc = " (Extension subtraits might provide more specific bounds on"] # [doc = " behavior, e.g., guarantee a sentinel address or a null pointer"] # [doc = " in response to a zero-size allocation request.)"] # [doc = ""] # [doc = " The allocated block of memory may or may not be initialized."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returning a null pointer indicates that either memory is exhausted"] # [doc = " or `layout` does not meet this allocator's size or alignment constraints."] # [doc = ""] # [doc = " Implementations are encouraged to return null on memory"] # [doc = " exhaustion rather than aborting, but this is not"] # [doc = " a strict requirement. (Specifically: it is *legal* to"] # [doc = " implement this trait atop an underlying native allocation"] # [doc = " library that aborts on memory exhaustion.)"] # [doc = ""] # [doc = " Clients wishing to abort computation in response to an"] # [doc = " allocation error are encouraged to call the [`handle_alloc_error`] function,"] # [doc = " rather than directly invoking `panic!` or similar."] # [doc = ""] # [doc = " [`handle_alloc_error`]: ../../alloc/alloc/fn.handle_alloc_error.html"] # [stable (feature = "global_alloc" , since = "1.28.0")] unsafe fn alloc (& self , layout : Layout) -> * mut u8 ; # [doc = " Deallocates the block of memory at the given `ptr` pointer with the given `layout`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure:"] # [doc = ""] # [doc = " * `ptr` is a block of memory currently allocated via this allocator and,"] # [doc = ""] # [doc = " * `layout` is the same layout that was used to allocate that block of"] # [doc = "   memory."] # [doc = ""] # [doc = " Otherwise undefined behavior can result."] # [stable (feature = "global_alloc" , since = "1.28.0")] unsafe fn dealloc (& self , ptr : * mut u8 , layout : Layout) ; # [doc = " Behaves like `alloc`, but also ensures that the contents"] # [doc = " are set to zero before being returned."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller has to ensure that `layout` has non-zero size. Like `alloc`"] # [doc = " zero sized `layout` can result in undefined behavior."] # [doc = " However the allocated block of memory is guaranteed to be initialized."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returning a null pointer indicates that either memory is exhausted"] # [doc = " or `layout` does not meet allocator's size or alignment constraints,"] # [doc = " just as in `alloc`."] # [doc = ""] # [doc = " Clients wishing to abort computation in response to an"] # [doc = " allocation error are encouraged to call the [`handle_alloc_error`] function,"] # [doc = " rather than directly invoking `panic!` or similar."] # [doc = ""] # [doc = " [`handle_alloc_error`]: ../../alloc/alloc/fn.handle_alloc_error.html"] # [stable (feature = "global_alloc" , since = "1.28.0")] unsafe fn alloc_zeroed (& self , layout : Layout) -> * mut u8 { let size = layout . size () ; let ptr = unsafe { self . alloc (layout) } ; if ! ptr . is_null () { unsafe { ptr :: write_bytes (ptr , 0 , size) } ; } ptr } # [doc = " Shrinks or grows a block of memory to the given `new_size` in bytes."] # [doc = " The block is described by the given `ptr` pointer and `layout`."] # [doc = ""] # [doc = " If this returns a non-null pointer, then ownership of the memory block"] # [doc = " referenced by `ptr` has been transferred to this allocator."] # [doc = " Any access to the old `ptr` is Undefined Behavior, even if the"] # [doc = " allocation remained in-place. The newly returned pointer is the only valid pointer"] # [doc = " for accessing this memory now."] # [doc = ""] # [doc = " The new memory block is allocated with `layout`,"] # [doc = " but with the `size` updated to `new_size` in bytes."] # [doc = " This new layout must be used when deallocating the new memory block with `dealloc`."] # [doc = " The range `0..min(layout.size(), new_size)` of the new memory block is"] # [doc = " guaranteed to have the same values as the original block."] # [doc = ""] # [doc = " If this method returns null, then ownership of the memory"] # [doc = " block has not been transferred to this allocator, and the"] # [doc = " contents of the memory block are unaltered."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure that:"] # [doc = ""] # [doc = " * `ptr` is allocated via this allocator,"] # [doc = ""] # [doc = " * `layout` is the same layout that was used"] # [doc = "   to allocate that block of memory,"] # [doc = ""] # [doc = " * `new_size` is greater than zero."] # [doc = ""] # [doc = " * `new_size`, when rounded up to the nearest multiple of `layout.align()`,"] # [doc = "   does not overflow `isize` (i.e., the rounded value must be less than or"] # [doc = "   equal to `isize::MAX`)."] # [doc = ""] # [doc = " If these are not followed, undefined behavior can result."] # [doc = ""] # [doc = " (Extension subtraits might provide more specific bounds on"] # [doc = " behavior, e.g., guarantee a sentinel address or a null pointer"] # [doc = " in response to a zero-size allocation request.)"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns null if the new layout does not meet the size"] # [doc = " and alignment constraints of the allocator, or if reallocation"] # [doc = " otherwise fails."] # [doc = ""] # [doc = " Implementations are encouraged to return null on memory"] # [doc = " exhaustion rather than panicking or aborting, but this is not"] # [doc = " a strict requirement. (Specifically: it is *legal* to"] # [doc = " implement this trait atop an underlying native allocation"] # [doc = " library that aborts on memory exhaustion.)"] # [doc = ""] # [doc = " Clients wishing to abort computation in response to a"] # [doc = " reallocation error are encouraged to call the [`handle_alloc_error`] function,"] # [doc = " rather than directly invoking `panic!` or similar."] # [doc = ""] # [doc = " [`handle_alloc_error`]: ../../alloc/alloc/fn.handle_alloc_error.html"] # [stable (feature = "global_alloc" , since = "1.28.0")] unsafe fn realloc (& self , ptr : * mut u8 , layout : Layout , new_size : usize) -> * mut u8 { let new_layout = unsafe { Layout :: from_size_align_unchecked (new_size , layout . align ()) } ; let new_ptr = unsafe { self . alloc (new_layout) } ; if ! new_ptr . is_null () { unsafe { ptr :: copy_nonoverlapping (ptr , new_ptr , cmp :: min (layout . size () , new_size)) ; self . dealloc (ptr , layout) ; } } new_ptr } }}}