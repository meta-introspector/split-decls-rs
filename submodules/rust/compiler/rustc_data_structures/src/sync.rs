mkuse!{use std :: collections :: HashMap ;}
mkuse!{use std :: hash :: { BuildHasher , Hash } ;}
mkuse!{pub use parking_lot :: { MappedRwLockReadGuard as MappedReadGuard , MappedRwLockWriteGuard as MappedWriteGuard , RwLockReadGuard as ReadGuard , RwLockWriteGuard as WriteGuard , } ;}
mkuse!{pub use self :: atomic :: AtomicU64 ;}
mkuse!{pub use self :: freeze :: { FreezeLock , FreezeReadGuard , FreezeWriteGuard } ;}
mkuse!{# [doc (no_inline)] pub use self :: lock :: { Lock , LockGuard , Mode } ;}
mkuse!{pub use self :: mode :: { is_dyn_thread_safe , set_dyn_thread_safe_mode } ;}
mkuse!{pub use self :: parallel :: { broadcast , join , par_for_each_in , par_map , parallel_guard , scope , spawn , try_par_for_each_in , } ;}
mkuse!{pub use self :: vec :: { AppendOnlyIndexVec , AppendOnlyVec } ;}
mkuse!{pub use self :: worker_local :: { Registry , WorkerLocal } ;}
mkuse!{pub use crate :: marker :: * ;}
mkmod!{freeze, { 
                getname!(freeze);
                getsrc!(freeze);
                getpath!(freeze);
                get_deps!(freeze);
                get_crates!(freeze);
                mkinclude!(freeze);
                 
            }}
mkmod!{lock, { 
                getname!(lock);
                getsrc!(lock);
                getpath!(lock);
                get_deps!(lock);
                get_crates!(lock);
                mkinclude!(lock);
                 
            }}
mkmod!{parallel, { 
                getname!(parallel);
                getsrc!(parallel);
                getpath!(parallel);
                get_deps!(parallel);
                get_crates!(parallel);
                mkinclude!(parallel);
                 
            }}
mkmod!{vec, { 
                getname!(vec);
                getsrc!(vec);
                getpath!(vec);
                get_deps!(vec);
                get_crates!(vec);
                mkinclude!(vec);
                 
            }}
mkmod!{worker_local, { 
                getname!(worker_local);
                getsrc!(worker_local);
                getpath!(worker_local);
                get_deps!(worker_local);
                get_crates!(worker_local);
                mkinclude!(worker_local);
                 
            }}
mkmod!{atomic, { 
                getname!(atomic);
                getsrc!(atomic);
                getpath!(atomic);
                get_deps!(atomic);
                get_crates!(atomic);
                mkinclude!(atomic);
                mkuse!{# [cfg (target_has_atomic = "64")] pub use std :: sync :: atomic :: AtomicU64 ;}
mkuse!{# [cfg (not (target_has_atomic = "64"))] pub use portable_atomic :: AtomicU64 ;} 
            }}
mkmod!{mode, { 
                getname!(mode);
                getsrc!(mode);
                getpath!(mode);
                get_deps!(mode);
                get_crates!(mode);
                mkinclude!(mode);
                mkuse!{use std :: sync :: atomic :: { AtomicU8 , Ordering } ;}
mkitem!{const UNINITIALIZED : u8 = 0 ;}
mkitem!{const DYN_NOT_THREAD_SAFE : u8 = 1 ;}
mkitem!{const DYN_THREAD_SAFE : u8 = 2 ;}
mkitem!{static DYN_THREAD_SAFE_MODE : AtomicU8 = AtomicU8 :: new (UNINITIALIZED) ;}

macro_rules! is_dyn_thread_safe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_dyn_thread_safe in module {}", module_path!());
    };
}

mkfn!{
    is_dyn_thread_safe_introspect!();
    # [inline] pub fn is_dyn_thread_safe () -> bool { match DYN_THREAD_SAFE_MODE . load (Ordering :: Relaxed) { DYN_NOT_THREAD_SAFE => false , DYN_THREAD_SAFE => true , _ => panic ! ("uninitialized dyn_thread_safe mode!") , } }
}

macro_rules! might_be_dyn_thread_safe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function might_be_dyn_thread_safe in module {}", module_path!());
    };
}

mkfn!{
    might_be_dyn_thread_safe_introspect!();
    # [inline] pub (super) fn might_be_dyn_thread_safe () -> bool { DYN_THREAD_SAFE_MODE . load (Ordering :: Relaxed) != DYN_NOT_THREAD_SAFE }
}

macro_rules! set_dyn_thread_safe_mode_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_dyn_thread_safe_mode in module {}", module_path!());
    };
}

mkfn!{
    set_dyn_thread_safe_mode_introspect!();
    pub fn set_dyn_thread_safe_mode (mode : bool) { let set : u8 = if mode { DYN_THREAD_SAFE } else { DYN_NOT_THREAD_SAFE } ; let previous = DYN_THREAD_SAFE_MODE . compare_exchange (UNINITIALIZED , set , Ordering :: Relaxed , Ordering :: Relaxed ,) ; assert ! (previous . is_ok () || previous == Err (set)) ; }
} 
            }}
mkitem!{mkstruct!{# [derive (Debug , Default)] pub struct MTLock < T > (Lock < T >) ;}}
mkitem!{mkimpl!{impl < T > MTLock < T > { # [inline (always)] pub fn new (inner : T) -> Self { MTLock (Lock :: new (inner)) } # [inline (always)] pub fn into_inner (self) -> T { self . 0 . into_inner () } # [inline (always)] pub fn get_mut (& mut self) -> & mut T { self . 0 . get_mut () } # [inline (always)] pub fn lock (& self) -> LockGuard < '_ , T > { self . 0 . lock () } # [inline (always)] pub fn lock_mut (& self) -> LockGuard < '_ , T > { self . lock () } }}}
mkitem!{# [doc = " This makes locks panic if they are already held."] # [doc = " It is only useful when you are running in a single thread"] const ERROR_CHECKING : bool = false ;}
mkitem!{mkstruct!{# [derive (Default)] # [repr (align (64))] pub struct CacheAligned < T > (pub T) ;}}
mkitem!{mktrait!{pub trait HashMapExt < K , V > { # [doc = " Same as HashMap::insert, but it may panic if there's already an"] # [doc = " entry for `key` with a value not equal to `value`"] fn insert_same (& mut self , key : K , value : V) ; }}}
mkitem!{mkimpl!{impl < K : Eq + Hash , V : Eq , S : BuildHasher > HashMapExt < K , V > for HashMap < K , V , S > { fn insert_same (& mut self , key : K , value : V) { self . entry (key) . and_modify (| old | assert ! (* old == value)) . or_insert (value) ; } }}}
mkitem!{mkstruct!{# [derive (Debug , Default)] pub struct RwLock < T > (parking_lot :: RwLock < T >) ;}}
mkitem!{mkimpl!{impl < T > RwLock < T > { # [inline (always)] pub fn new (inner : T) -> Self { RwLock (parking_lot :: RwLock :: new (inner)) } # [inline (always)] pub fn into_inner (self) -> T { self . 0 . into_inner () } # [inline (always)] pub fn get_mut (& mut self) -> & mut T { self . 0 . get_mut () } # [inline (always)] pub fn read (& self) -> ReadGuard < '_ , T > { if ERROR_CHECKING { self . 0 . try_read () . expect ("lock was already held") } else { self . 0 . read () } } # [inline (always)] pub fn try_write (& self) -> Result < WriteGuard < '_ , T > , () > { self . 0 . try_write () . ok_or (()) } # [inline (always)] pub fn write (& self) -> WriteGuard < '_ , T > { if ERROR_CHECKING { self . 0 . try_write () . expect ("lock was already held") } else { self . 0 . write () } } # [inline (always)] # [track_caller] pub fn borrow (& self) -> ReadGuard < '_ , T > { self . read () } # [inline (always)] # [track_caller] pub fn borrow_mut (& self) -> WriteGuard < '_ , T > { self . write () } }}}