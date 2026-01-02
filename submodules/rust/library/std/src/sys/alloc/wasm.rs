mkuse!{use core :: cell :: SyncUnsafeCell ;}
mkuse!{use crate :: alloc :: { GlobalAlloc , Layout , System } ;}
mkitem!{mkstruct!{struct SyncDlmalloc (dlmalloc :: Dlmalloc) ;}}
mkitem!{mkimpl!{unsafe impl Sync for SyncDlmalloc { }}}
mkitem!{static DLMALLOC : SyncUnsafeCell < SyncDlmalloc > = SyncUnsafeCell :: new (SyncDlmalloc (dlmalloc :: Dlmalloc :: new ())) ;}
mkitem!{mkimpl!{# [stable (feature = "alloc_system_type" , since = "1.28.0")] unsafe impl GlobalAlloc for System { # [inline] unsafe fn alloc (& self , layout : Layout) -> * mut u8 { let _lock = lock :: lock () ; unsafe { (* DLMALLOC . get ()) . 0 . malloc (layout . size () , layout . align ()) } } # [inline] unsafe fn alloc_zeroed (& self , layout : Layout) -> * mut u8 { let _lock = lock :: lock () ; unsafe { (* DLMALLOC . get ()) . 0 . calloc (layout . size () , layout . align ()) } } # [inline] unsafe fn dealloc (& self , ptr : * mut u8 , layout : Layout) { let _lock = lock :: lock () ; unsafe { (* DLMALLOC . get ()) . 0 . free (ptr , layout . size () , layout . align ()) } } # [inline] unsafe fn realloc (& self , ptr : * mut u8 , layout : Layout , new_size : usize) -> * mut u8 { let _lock = lock :: lock () ; unsafe { (* DLMALLOC . get ()) . 0 . realloc (ptr , layout . size () , layout . align () , new_size) } } }}}
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
    pub fn lock () -> DropLock { loop { if LOCKED . swap (1 , Acquire) == 0 { return DropLock ; } } }
}
mkitem!{mkimpl!{impl Drop for DropLock { fn drop (& mut self) { let r = LOCKED . swap (0 , Release) ; debug_assert_eq ! (r , 1) ; } }}} 
            }}
mkmod!{lock, { 
                getname!(lock);
                getsrc!(lock);
                getpath!(lock);
                get_deps!(lock);
                get_crates!(lock);
                mkinclude!(lock);
                
macro_rules! lock_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lock in module {}", module_path!());
    };
}

mkfn!{
    lock_introspect!();
    # [inline] pub fn lock () { }
} 
            }}