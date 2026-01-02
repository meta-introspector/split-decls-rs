mkuse!{use std :: alloc :: Allocator ;}
mkuse!{use std :: marker :: PointeeSized ;}
mkitem!{mktrait!{# [diagnostic :: on_unimplemented (message = "`{Self}` doesn't implement `DynSend`. \
            Add it to `rustc_data_structures::marker` or use `IntoDynSyncSend` if it's already `Send`")] pub unsafe auto trait DynSend { }}}
mkitem!{mktrait!{# [diagnostic :: on_unimplemented (message = "`{Self}` doesn't implement `DynSync`. \
            Add it to `rustc_data_structures::marker` or use `IntoDynSyncSend` if it's already `Sync`")] pub unsafe auto trait DynSync { }}}
mkitem!{mkimpl!{unsafe impl < T : DynSync + ? Sized + PointeeSized > DynSend for & T { }}}
mkitem!{macro_rules ! impls_dyn_send_neg { ($ ([$ t1 : ty $ (where $ ($ generics1 : tt) *) ?]) *) => { $ (impl $ (<$ ($ generics1) *>) ? ! DynSend for $ t1 { }) * } ; }}
mkitem!{impls_dyn_send_neg ! ([std :: env :: Args] [std :: env :: ArgsOs] [* const T where T : ? Sized + PointeeSized] [* mut T where T : ? Sized + PointeeSized] [std :: ptr :: NonNull < T > where T : ? Sized + PointeeSized] [std :: rc :: Rc < T , A > where T : ? Sized , A : Allocator] [std :: rc :: Weak < T , A > where T : ? Sized , A : Allocator] [std :: sync :: MutexGuard <'_ , T > where T : ? Sized] [std :: sync :: RwLockReadGuard <'_ , T > where T : ? Sized] [std :: sync :: RwLockWriteGuard <'_ , T > where T : ? Sized] [std :: io :: StdoutLock <'_ >] [std :: io :: StderrLock <'_ >]) ;}
mkitem!{mkimpl!{# [cfg (any (unix , target_os = "hermit" , all (target_vendor = "fortanix" , target_env = "sgx") , target_os = "solid_asp3" , target_os = "wasi" , target_os = "xous"))] impl ! DynSend for std :: env :: VarsOs { }}}
mkitem!{macro_rules ! already_send { ($ ([$ ty : ty]) *) => { $ (unsafe impl DynSend for $ ty where $ ty : Send { }) * } ; }}
mkitem!{already_send ! ([std :: backtrace :: Backtrace] [std :: io :: Stdout] [std :: io :: Stderr] [std :: io :: Error] [std :: fs :: File] [rustc_arena :: DroplessArena] [jobserver_crate :: Client] [jobserver_crate :: HelperThread] [crate :: memmap :: Mmap] [crate :: profiling :: SelfProfiler] [crate :: owned_slice :: OwnedSlice]) ;}
mkitem!{macro_rules ! impl_dyn_send { ($ ($ ($ attr : meta) * [$ ty : ty where $ ($ generics2 : tt) *]) *) => { $ (unsafe impl <$ ($ generics2) *> DynSend for $ ty { }) * } ; }}
mkitem!{impl_dyn_send ! ([std :: sync :: atomic :: AtomicPtr < T > where T] [std :: sync :: Mutex < T > where T : ? Sized + DynSend] [std :: sync :: mpsc :: Sender < T > where T : DynSend] [std :: sync :: Arc < T > where T : ? Sized + DynSync + DynSend] [std :: sync :: LazyLock < T , F > where T : DynSend , F : DynSend] [std :: collections :: HashSet < K , S > where K : DynSend , S : DynSend] [std :: collections :: HashMap < K , V , S > where K : DynSend , V : DynSend , S : DynSend] [std :: collections :: BTreeMap < K , V , A > where K : DynSend , V : DynSend , A : std :: alloc :: Allocator + Clone + DynSend] [Vec < T , A > where T : DynSend , A : std :: alloc :: Allocator + DynSend] [Box < T , A > where T : ? Sized + DynSend , A : std :: alloc :: Allocator + DynSend] [crate :: sync :: RwLock < T > where T : DynSend] [crate :: tagged_ptr :: TaggedRef <'a , P , T > where 'a , P : Sync , T : Send + crate :: tagged_ptr :: Tag] [rustc_arena :: TypedArena < T > where T : DynSend] [hashbrown :: HashTable < T > where T : DynSend] [indexmap :: IndexSet < V , S > where V : DynSend , S : DynSend] [indexmap :: IndexMap < K , V , S > where K : DynSend , V : DynSend , S : DynSend] [thin_vec :: ThinVec < T > where T : DynSend] [smallvec :: SmallVec < A > where A : smallvec :: Array + DynSend]) ;}
mkitem!{macro_rules ! impls_dyn_sync_neg { ($ ([$ t1 : ty $ (where $ ($ generics1 : tt) *) ?]) *) => { $ (impl $ (<$ ($ generics1) *>) ? ! DynSync for $ t1 { }) * } ; }}
mkitem!{impls_dyn_sync_neg ! ([std :: env :: Args] [std :: env :: ArgsOs] [* const T where T : ? Sized + PointeeSized] [* mut T where T : ? Sized + PointeeSized] [std :: cell :: Cell < T > where T : ? Sized] [std :: cell :: RefCell < T > where T : ? Sized] [std :: cell :: UnsafeCell < T > where T : ? Sized] [std :: ptr :: NonNull < T > where T : ? Sized + PointeeSized] [std :: rc :: Rc < T , A > where T : ? Sized , A : Allocator] [std :: rc :: Weak < T , A > where T : ? Sized , A : Allocator] [std :: cell :: OnceCell < T > where T] [std :: sync :: mpsc :: Receiver < T > where T] [std :: sync :: mpsc :: Sender < T > where T]) ;}
mkitem!{mkimpl!{# [cfg (any (unix , target_os = "hermit" , all (target_vendor = "fortanix" , target_env = "sgx") , target_os = "solid_asp3" , target_os = "wasi" , target_os = "xous"))] impl ! DynSync for std :: env :: VarsOs { }}}
mkitem!{macro_rules ! already_sync { ($ ([$ ty : ty]) *) => { $ (unsafe impl DynSync for $ ty where $ ty : Sync { }) * } ; }}
mkitem!{already_sync ! ([std :: sync :: atomic :: AtomicBool] [std :: sync :: atomic :: AtomicUsize] [std :: sync :: atomic :: AtomicU8] [std :: sync :: atomic :: AtomicU32] [std :: backtrace :: Backtrace] [std :: io :: Error] [std :: fs :: File] [jobserver_crate :: Client] [jobserver_crate :: HelperThread] [crate :: memmap :: Mmap] [crate :: profiling :: SelfProfiler] [crate :: owned_slice :: OwnedSlice]) ;}
mkitem!{# [cfg (target_has_atomic = "64")] already_sync ! ([std :: sync :: atomic :: AtomicU64]) ;}
mkitem!{# [cfg (not (target_has_atomic = "64"))] already_sync ! ([portable_atomic :: AtomicU64]) ;}
mkitem!{macro_rules ! impl_dyn_sync { ($ ($ ($ attr : meta) * [$ ty : ty where $ ($ generics2 : tt) *]) *) => { $ (unsafe impl <$ ($ generics2) *> DynSync for $ ty { }) * } ; }}
mkitem!{impl_dyn_sync ! ([std :: sync :: atomic :: AtomicPtr < T > where T] [std :: sync :: OnceLock < T > where T : DynSend + DynSync] [std :: sync :: Mutex < T > where T : ? Sized + DynSend] [std :: sync :: Arc < T > where T : ? Sized + DynSync + DynSend] [std :: sync :: LazyLock < T , F > where T : DynSend + DynSync , F : DynSend] [std :: collections :: HashSet < K , S > where K : DynSync , S : DynSync] [std :: collections :: HashMap < K , V , S > where K : DynSync , V : DynSync , S : DynSync] [std :: collections :: BTreeMap < K , V , A > where K : DynSync , V : DynSync , A : std :: alloc :: Allocator + Clone + DynSync] [Vec < T , A > where T : DynSync , A : std :: alloc :: Allocator + DynSync] [Box < T , A > where T : ? Sized + DynSync , A : std :: alloc :: Allocator + DynSync] [crate :: sync :: RwLock < T > where T : DynSend + DynSync] [crate :: sync :: WorkerLocal < T > where T : DynSend] [crate :: intern :: Interned <'a , T > where 'a , T : DynSync] [crate :: tagged_ptr :: TaggedRef <'a , P , T > where 'a , P : Sync , T : Sync + crate :: tagged_ptr :: Tag] [parking_lot :: lock_api :: Mutex < R , T > where R : DynSync , T : ? Sized + DynSend] [parking_lot :: lock_api :: RwLock < R , T > where R : DynSync , T : ? Sized + DynSend + DynSync] [hashbrown :: HashTable < T > where T : DynSync] [indexmap :: IndexSet < V , S > where V : DynSync , S : DynSync] [indexmap :: IndexMap < K , V , S > where K : DynSync , V : DynSync , S : DynSync] [smallvec :: SmallVec < A > where A : smallvec :: Array + DynSync] [thin_vec :: ThinVec < T > where T : DynSync]) ;}

macro_rules! assert_dyn_sync_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function assert_dyn_sync in module {}", module_path!());
    };
}

mkfn!{
    assert_dyn_sync_introspect!();
    pub fn assert_dyn_sync < T : ? Sized + PointeeSized + DynSync > () { }
}

macro_rules! assert_dyn_send_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function assert_dyn_send in module {}", module_path!());
    };
}

mkfn!{
    assert_dyn_send_introspect!();
    pub fn assert_dyn_send < T : ? Sized + PointeeSized + DynSend > () { }
}

macro_rules! assert_dyn_send_val_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function assert_dyn_send_val in module {}", module_path!());
    };
}

mkfn!{
    assert_dyn_send_val_introspect!();
    pub fn assert_dyn_send_val < T : ? Sized + PointeeSized + DynSend > (_t : & T) { }
}

macro_rules! assert_dyn_send_sync_val_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function assert_dyn_send_sync_val in module {}", module_path!());
    };
}

mkfn!{
    assert_dyn_send_sync_val_introspect!();
    pub fn assert_dyn_send_sync_val < T : ? Sized + PointeeSized + DynSync + DynSend > (_t : & T) { }
}
mkitem!{mkstruct!{# [derive (Copy , Clone)] pub struct FromDyn < T > (T) ;}}
mkitem!{mkimpl!{impl < T > FromDyn < T > { # [inline (always)] pub fn from (val : T) -> Self { assert ! (crate :: sync :: is_dyn_thread_safe ()) ; FromDyn (val) } # [inline (always)] pub fn derive < O > (& self , val : O) -> FromDyn < O > { FromDyn (val) } # [inline (always)] pub fn into_inner (self) -> T { self . 0 } }}}
mkitem!{mkimpl!{unsafe impl < T : DynSend > Send for FromDyn < T > { }}}
mkitem!{mkimpl!{unsafe impl < T : DynSync > Sync for FromDyn < T > { }}}
mkitem!{mkimpl!{impl < T > std :: ops :: Deref for FromDyn < T > { type Target = T ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } }}}
mkitem!{mkimpl!{impl < T > std :: ops :: DerefMut for FromDyn < T > { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }}}
mkitem!{mkstruct!{# [derive (Copy , Clone)] pub struct IntoDynSyncSend < T : ? Sized + PointeeSized > (pub T) ;}}
mkitem!{mkimpl!{unsafe impl < T : ? Sized + PointeeSized + Send > DynSend for IntoDynSyncSend < T > { }}}
mkitem!{mkimpl!{unsafe impl < T : ? Sized + PointeeSized + Sync > DynSync for IntoDynSyncSend < T > { }}}
mkitem!{mkimpl!{impl < T > std :: ops :: Deref for IntoDynSyncSend < T > { type Target = T ; # [inline (always)] fn deref (& self) -> & T { & self . 0 } }}}
mkitem!{mkimpl!{impl < T > std :: ops :: DerefMut for IntoDynSyncSend < T > { # [inline (always)] fn deref_mut (& mut self) -> & mut T { & mut self . 0 } }}}