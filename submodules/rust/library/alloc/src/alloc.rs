mkuse!{# [stable (feature = "alloc_module" , since = "1.28.0")] # [doc (inline)] pub use core :: alloc :: * ;}
mkuse!{use core :: hint ;}
mkuse!{use core :: ptr :: { self , NonNull } ;}
mkitem!{unsafe extern "Rust" { # [rustc_allocator] # [rustc_nounwind] # [rustc_std_internal_symbol] fn __rust_alloc (size : usize , align : usize) -> * mut u8 ; # [rustc_deallocator] # [rustc_nounwind] # [rustc_std_internal_symbol] fn __rust_dealloc (ptr : * mut u8 , size : usize , align : usize) ; # [rustc_reallocator] # [rustc_nounwind] # [rustc_std_internal_symbol] fn __rust_realloc (ptr : * mut u8 , old_size : usize , align : usize , new_size : usize) -> * mut u8 ; # [rustc_allocator_zeroed] # [rustc_nounwind] # [rustc_std_internal_symbol] fn __rust_alloc_zeroed (size : usize , align : usize) -> * mut u8 ; # [rustc_nounwind] # [rustc_std_internal_symbol] fn __rust_no_alloc_shim_is_unstable_v2 () ; }}
mkitem!{mkstruct!{# [doc = " The global memory allocator."] # [doc = ""] # [doc = " This type implements the [`Allocator`] trait by forwarding calls"] # [doc = " to the allocator registered with the `#[global_allocator]` attribute"] # [doc = " if there is one, or the `std` crate’s default."] # [doc = ""] # [doc = " Note: while this type is unstable, the functionality it provides can be"] # [doc = " accessed through the [free functions in `alloc`](self#functions)."] # [unstable (feature = "allocator_api" , issue = "32838")] # [derive (Copy , Clone , Default , Debug)] # [lang = "global_alloc_ty"] pub struct Global ;}}

macro_rules! alloc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function alloc in module {}", module_path!());
    };
}

mkfn!{
    alloc_introspect!();
    # [doc = " Allocates memory with the global allocator."] # [doc = ""] # [doc = " This function forwards calls to the [`GlobalAlloc::alloc`] method"] # [doc = " of the allocator registered with the `#[global_allocator]` attribute"] # [doc = " if there is one, or the `std` crate’s default."] # [doc = ""] # [doc = " This function is expected to be deprecated in favor of the `allocate` method"] # [doc = " of the [`Global`] type when it and the [`Allocator`] trait become stable."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See [`GlobalAlloc::alloc`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};"] # [doc = ""] # [doc = " unsafe {"] # [doc = "     let layout = Layout::new::<u16>();"] # [doc = "     let ptr = alloc(layout);"] # [doc = "     if ptr.is_null() {"] # [doc = "         handle_alloc_error(layout);"] # [doc = "     }"] # [doc = ""] # [doc = "     *(ptr as *mut u16) = 42;"] # [doc = "     assert_eq!(*(ptr as *mut u16), 42);"] # [doc = ""] # [doc = "     dealloc(ptr, layout);"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "global_alloc" , since = "1.28.0")] # [must_use = "losing the pointer will leak memory"] # [inline] # [cfg_attr (miri , track_caller)] pub unsafe fn alloc (layout : Layout) -> * mut u8 { unsafe { __rust_no_alloc_shim_is_unstable_v2 () ; __rust_alloc (layout . size () , layout . align ()) } }
}

macro_rules! dealloc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dealloc in module {}", module_path!());
    };
}

mkfn!{
    dealloc_introspect!();
    # [doc = " Deallocates memory with the global allocator."] # [doc = ""] # [doc = " This function forwards calls to the [`GlobalAlloc::dealloc`] method"] # [doc = " of the allocator registered with the `#[global_allocator]` attribute"] # [doc = " if there is one, or the `std` crate’s default."] # [doc = ""] # [doc = " This function is expected to be deprecated in favor of the `deallocate` method"] # [doc = " of the [`Global`] type when it and the [`Allocator`] trait become stable."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See [`GlobalAlloc::dealloc`]."] # [stable (feature = "global_alloc" , since = "1.28.0")] # [inline] # [cfg_attr (miri , track_caller)] pub unsafe fn dealloc (ptr : * mut u8 , layout : Layout) { unsafe { __rust_dealloc (ptr , layout . size () , layout . align ()) } }
}

macro_rules! realloc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function realloc in module {}", module_path!());
    };
}

mkfn!{
    realloc_introspect!();
    # [doc = " Reallocates memory with the global allocator."] # [doc = ""] # [doc = " This function forwards calls to the [`GlobalAlloc::realloc`] method"] # [doc = " of the allocator registered with the `#[global_allocator]` attribute"] # [doc = " if there is one, or the `std` crate’s default."] # [doc = ""] # [doc = " This function is expected to be deprecated in favor of the `grow` and `shrink` methods"] # [doc = " of the [`Global`] type when it and the [`Allocator`] trait become stable."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See [`GlobalAlloc::realloc`]."] # [stable (feature = "global_alloc" , since = "1.28.0")] # [must_use = "losing the pointer will leak memory"] # [inline] # [cfg_attr (miri , track_caller)] pub unsafe fn realloc (ptr : * mut u8 , layout : Layout , new_size : usize) -> * mut u8 { unsafe { __rust_realloc (ptr , layout . size () , layout . align () , new_size) } }
}

macro_rules! alloc_zeroed_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function alloc_zeroed in module {}", module_path!());
    };
}

mkfn!{
    alloc_zeroed_introspect!();
    # [doc = " Allocates zero-initialized memory with the global allocator."] # [doc = ""] # [doc = " This function forwards calls to the [`GlobalAlloc::alloc_zeroed`] method"] # [doc = " of the allocator registered with the `#[global_allocator]` attribute"] # [doc = " if there is one, or the `std` crate’s default."] # [doc = ""] # [doc = " This function is expected to be deprecated in favor of the `allocate_zeroed` method"] # [doc = " of the [`Global`] type when it and the [`Allocator`] trait become stable."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See [`GlobalAlloc::alloc_zeroed`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::alloc::{alloc_zeroed, dealloc, handle_alloc_error, Layout};"] # [doc = ""] # [doc = " unsafe {"] # [doc = "     let layout = Layout::new::<u16>();"] # [doc = "     let ptr = alloc_zeroed(layout);"] # [doc = "     if ptr.is_null() {"] # [doc = "         handle_alloc_error(layout);"] # [doc = "     }"] # [doc = ""] # [doc = "     assert_eq!(*(ptr as *mut u16), 0);"] # [doc = ""] # [doc = "     dealloc(ptr, layout);"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "global_alloc" , since = "1.28.0")] # [must_use = "losing the pointer will leak memory"] # [inline] # [cfg_attr (miri , track_caller)] pub unsafe fn alloc_zeroed (layout : Layout) -> * mut u8 { unsafe { __rust_no_alloc_shim_is_unstable_v2 () ; __rust_alloc_zeroed (layout . size () , layout . align ()) } }
}
mkitem!{mkimpl!{impl Global { # [inline] # [cfg_attr (miri , track_caller)] fn alloc_impl (& self , layout : Layout , zeroed : bool) -> Result < NonNull < [u8] > , AllocError > { match layout . size () { 0 => Ok (NonNull :: slice_from_raw_parts (layout . dangling () , 0)) , size => unsafe { let raw_ptr = if zeroed { alloc_zeroed (layout) } else { alloc (layout) } ; let ptr = NonNull :: new (raw_ptr) . ok_or (AllocError) ? ; Ok (NonNull :: slice_from_raw_parts (ptr , size)) } , } } # [inline] # [cfg_attr (miri , track_caller)] unsafe fn grow_impl (& self , ptr : NonNull < u8 > , old_layout : Layout , new_layout : Layout , zeroed : bool ,) -> Result < NonNull < [u8] > , AllocError > { debug_assert ! (new_layout . size () >= old_layout . size () , "`new_layout.size()` must be greater than or equal to `old_layout.size()`") ; match old_layout . size () { 0 => self . alloc_impl (new_layout , zeroed) , old_size if old_layout . align () == new_layout . align () => unsafe { let new_size = new_layout . size () ; hint :: assert_unchecked (new_size >= old_layout . size ()) ; let raw_ptr = realloc (ptr . as_ptr () , old_layout , new_size) ; let ptr = NonNull :: new (raw_ptr) . ok_or (AllocError) ? ; if zeroed { raw_ptr . add (old_size) . write_bytes (0 , new_size - old_size) ; } Ok (NonNull :: slice_from_raw_parts (ptr , new_size)) } , old_size => unsafe { let new_ptr = self . alloc_impl (new_layout , zeroed) ? ; ptr :: copy_nonoverlapping (ptr . as_ptr () , new_ptr . as_mut_ptr () , old_size) ; self . deallocate (ptr , old_layout) ; Ok (new_ptr) } , } } }}}
mkitem!{mkimpl!{# [unstable (feature = "allocator_api" , issue = "32838")] unsafe impl Allocator for Global { # [inline] # [cfg_attr (miri , track_caller)] fn allocate (& self , layout : Layout) -> Result < NonNull < [u8] > , AllocError > { self . alloc_impl (layout , false) } # [inline] # [cfg_attr (miri , track_caller)] fn allocate_zeroed (& self , layout : Layout) -> Result < NonNull < [u8] > , AllocError > { self . alloc_impl (layout , true) } # [inline] # [cfg_attr (miri , track_caller)] unsafe fn deallocate (& self , ptr : NonNull < u8 > , layout : Layout) { if layout . size () != 0 { unsafe { dealloc (ptr . as_ptr () , layout) } } } # [inline] # [cfg_attr (miri , track_caller)] unsafe fn grow (& self , ptr : NonNull < u8 > , old_layout : Layout , new_layout : Layout ,) -> Result < NonNull < [u8] > , AllocError > { unsafe { self . grow_impl (ptr , old_layout , new_layout , false) } } # [inline] # [cfg_attr (miri , track_caller)] unsafe fn grow_zeroed (& self , ptr : NonNull < u8 > , old_layout : Layout , new_layout : Layout ,) -> Result < NonNull < [u8] > , AllocError > { unsafe { self . grow_impl (ptr , old_layout , new_layout , true) } } # [inline] # [cfg_attr (miri , track_caller)] unsafe fn shrink (& self , ptr : NonNull < u8 > , old_layout : Layout , new_layout : Layout ,) -> Result < NonNull < [u8] > , AllocError > { debug_assert ! (new_layout . size () <= old_layout . size () , "`new_layout.size()` must be smaller than or equal to `old_layout.size()`") ; match new_layout . size () { 0 => unsafe { self . deallocate (ptr , old_layout) ; Ok (NonNull :: slice_from_raw_parts (new_layout . dangling () , 0)) } , new_size if old_layout . align () == new_layout . align () => unsafe { hint :: assert_unchecked (new_size <= old_layout . size ()) ; let raw_ptr = realloc (ptr . as_ptr () , old_layout , new_size) ; let ptr = NonNull :: new (raw_ptr) . ok_or (AllocError) ? ; Ok (NonNull :: slice_from_raw_parts (ptr , new_size)) } , new_size => unsafe { let new_ptr = self . allocate (new_layout) ? ; ptr :: copy_nonoverlapping (ptr . as_ptr () , new_ptr . as_mut_ptr () , new_size) ; self . deallocate (ptr , old_layout) ; Ok (new_ptr) } , } } }}}

macro_rules! exchange_malloc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exchange_malloc in module {}", module_path!());
    };
}

mkfn!{
    exchange_malloc_introspect!();
    # [doc = " The allocator for `Box`."] # [cfg (not (no_global_oom_handling))] # [lang = "exchange_malloc"] # [inline] # [cfg_attr (miri , track_caller)] unsafe fn exchange_malloc (size : usize , align : usize) -> * mut u8 { let layout = unsafe { Layout :: from_size_align_unchecked (size , align) } ; match Global . allocate (layout) { Ok (ptr) => ptr . as_mut_ptr () , Err (_) => handle_alloc_error (layout) , } }
}
mkitem!{# [cfg (not (no_global_oom_handling))] unsafe extern "Rust" { # [rustc_std_internal_symbol] fn __rust_alloc_error_handler (size : usize , align : usize) -> ! ; }}

macro_rules! handle_alloc_error_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function handle_alloc_error in module {}", module_path!());
    };
}

mkfn!{
    handle_alloc_error_introspect!();
    # [doc = " Signals a memory allocation error."] # [doc = ""] # [doc = " Callers of memory allocation APIs wishing to cease execution"] # [doc = " in response to an allocation error are encouraged to call this function,"] # [doc = " rather than directly invoking [`panic!`] or similar."] # [doc = ""] # [doc = " This function is guaranteed to diverge (not return normally with a value), but depending on"] # [doc = " global configuration, it may either panic (resulting in unwinding or aborting as per"] # [doc = " configuration for all panics), or abort the process (with no unwinding)."] # [doc = ""] # [doc = " The default behavior is:"] # [doc = ""] # [doc = "  * If the binary links against `std` (typically the case), then"] # [doc = "   print a message to standard error and abort the process."] # [doc = "   This behavior can be replaced with [`set_alloc_error_hook`] and [`take_alloc_error_hook`]."] # [doc = "   Future versions of Rust may panic by default instead."] # [doc = ""] # [doc = " * If the binary does not link against `std` (all of its crates are marked"] # [doc = "   [`#![no_std]`][no_std]), then call [`panic!`] with a message."] # [doc = "   [The panic handler] applies as to any panic."] # [doc = ""] # [doc = " [`set_alloc_error_hook`]: ../../std/alloc/fn.set_alloc_error_hook.html"] # [doc = " [`take_alloc_error_hook`]: ../../std/alloc/fn.take_alloc_error_hook.html"] # [doc = " [The panic handler]: https://doc.rust-lang.org/reference/runtime.html#the-panic_handler-attribute"] # [doc = " [no_std]: https://doc.rust-lang.org/reference/names/preludes.html#the-no_std-attribute"] # [stable (feature = "global_alloc" , since = "1.28.0")] # [rustc_const_unstable (feature = "const_alloc_error" , issue = "92523")] # [cfg (not (no_global_oom_handling))] # [cold] # [optimize (size)] pub const fn handle_alloc_error (layout : Layout) -> ! { const fn ct_error (_ : Layout) -> ! { panic ! ("allocation failed") ; } # [inline] fn rt_error (layout : Layout) -> ! { unsafe { __rust_alloc_error_handler (layout . size () , layout . align ()) ; } } # [cfg (not (feature = "panic_immediate_abort"))] { core :: intrinsics :: const_eval_select ((layout ,) , ct_error , rt_error) } # [cfg (feature = "panic_immediate_abort")] ct_error (layout) }
}
mkmod!{__alloc_error_handler, { 
                getname!(__alloc_error_handler);
                getsrc!(__alloc_error_handler);
                getpath!(__alloc_error_handler);
                get_deps!(__alloc_error_handler);
                get_crates!(__alloc_error_handler);
                mkinclude!(__alloc_error_handler);
                
macro_rules! __rdl_oom_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __rdl_oom in module {}", module_path!());
    };
}

mkfn!{
    __rdl_oom_introspect!();
    # [rustc_std_internal_symbol] pub unsafe fn __rdl_oom (size : usize , _align : usize) -> ! { unsafe extern "Rust" { # [rustc_std_internal_symbol] fn __rust_alloc_error_handler_should_panic_v2 () -> u8 ; } if unsafe { __rust_alloc_error_handler_should_panic_v2 () != 0 } { panic ! ("memory allocation of {size} bytes failed") } else { core :: panicking :: panic_nounwind_fmt (format_args ! ("memory allocation of {size} bytes failed") , false ,) } }
} 
            }}