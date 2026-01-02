mkuse!{use crate :: alloc :: { GlobalAlloc , Layout , System } ;}
mkitem!{# [cfg (not (test))] # [unsafe (export_name = "_ZN16__rust_internals3std3sys4xous5alloc8DLMALLOCE")] static mut DLMALLOC : dlmalloc :: Dlmalloc = dlmalloc :: Dlmalloc :: new () ;}
mkitem!{# [cfg (test)] unsafe extern "Rust" { # [link_name = "_ZN16__rust_internals3std3sys4xous5alloc8DLMALLOCE"] static mut DLMALLOC : dlmalloc :: Dlmalloc ; }}
mkitem!{mkimpl!{# [stable (feature = "alloc_system_type" , since = "1.28.0")] unsafe impl GlobalAlloc for System { # [inline] unsafe fn alloc (& self , layout : Layout) -> * mut u8 { let _lock = lock :: lock () ; unsafe { DLMALLOC . malloc (layout . size () , layout . align ()) } } # [inline] unsafe fn alloc_zeroed (& self , layout : Layout) -> * mut u8 { let _lock = lock :: lock () ; unsafe { DLMALLOC . calloc (layout . size () , layout . align ()) } } # [inline] unsafe fn dealloc (& self , ptr : * mut u8 , layout : Layout) { let _lock = lock :: lock () ; unsafe { DLMALLOC . free (ptr , layout . size () , layout . align ()) } } # [inline] unsafe fn realloc (& self , ptr : * mut u8 , layout : Layout , new_size : usize) -> * mut u8 { let _lock = lock :: lock () ; unsafe { DLMALLOC . realloc (ptr , layout . size () , layout . align () , new_size) } } }}}
mkmod!{lock, { 
                getname!(lock);
                getsrc!(lock);
                getpath!(lock);
                get_deps!(lock);
                get_crates!(lock);
                mkinclude!(lock);
                mkuse!{use crate :: sync :: atomic :: Ordering :: { Acquire , Release } ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicI32 } ;}
mkitem!{static LOCKED : Atomic < i32 > = AtomicI32 :: new (0) ;}
mkitem!{mkstruct!{pub struct DropLock ;}}

macro_rules! lock_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lock in module {}", module_path!());
    };
}

mkfn!{
    lock_introspect!();
    pub fn lock () -> DropLock { loop { if LOCKED . swap (1 , Acquire) == 0 { return DropLock ; } crate :: os :: xous :: ffi :: do_yield () ; } }
}
mkitem!{mkimpl!{impl Drop for DropLock { fn drop (& mut self) { let r = LOCKED . swap (0 , Release) ; debug_assert_eq ! (r , 1) ; } }}} 
            }}