/* FP:marker.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_USE_0001
/* FP:marker.rs-0002 */ use std :: alloc :: Allocator ;
/* FP:marker.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_USE_0002
/* FP:marker.rs-0004 */ use std :: marker :: PointeeSized ;
/* FP:marker.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_TRAIT_0003
/* FP:marker.rs-0006 */ # [diagnostic :: on_unimplemented (message = "`{Self}` doesn't implement `DynSend`. \
/* FP:marker.rs-0007 */             Add it to `crate::rustc_data_structures::marker` or use `IntoDynSyncSend` if it's already `Send`")] pub unsafe auto trait DynSend { }
/* FP:marker.rs-0008 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_TRAIT_0004
/* FP:marker.rs-0009 */ # [diagnostic :: on_unimplemented (message = "`{Self}` doesn't implement `DynSync`. \
/* FP:marker.rs-0010 */             Add it to `crate::rustc_data_structures::marker` or use `IntoDynSyncSend` if it's already `Sync`")] pub unsafe auto trait DynSync { }
/* FP:marker.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_IMPL_0005
/* FP:marker.rs-0012 */ unsafe impl < T : DynSync + ? Sized + PointeeSized > DynSend for & T { }
/* FP:marker.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_MACRO_0006
/* FP:marker.rs-0014 */ macro_rules ! impls_dyn_send_neg { ($ ([$ t1 : ty $ (where $ ($ generics1 : tt) *) ?]) *) => { $ (impl $ (<$ ($ generics1) *>) ? ! DynSend for $ t1 { }) * } ; }
/* FP:marker.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_MACRO_0007
/* FP:marker.rs-0016 */ impls_dyn_send_neg ! ([std :: env :: Args] [std :: env :: ArgsOs] [* const T where T : ? Sized + PointeeSized] [* mut T where T : ? Sized + PointeeSized] [std :: ptr :: NonNull < T > where T : ? Sized + PointeeSized] [std :: rc :: Rc < T , A > where T : ? Sized , A : Allocator] [std :: rc :: Weak < T , A > where T : ? Sized , A : Allocator] [std :: sync :: MutexGuard <'_ , T > where T : ? Sized] [std :: sync :: RwLockReadGuard <'_ , T > where T : ? Sized] [std :: sync :: RwLockWriteGuard <'_ , T > where T : ? Sized] [std :: io :: StdoutLock <'_ >] [std :: io :: StderrLock <'_ >]) ;
/* FP:marker.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_IMPL_0008
/* FP:marker.rs-0018 */ # [cfg (any (unix , target_os = "hermit" , all (target_vendor = "fortanix" , target_env = "sgx") , target_os = "solid_asp3" , target_os = "wasi" , target_os = "xous"))] impl ! DynSend for std :: env :: VarsOs { }
/* FP:marker.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_MACRO_0009
/* FP:marker.rs-0020 */ macro_rules ! already_send { ($ ([$ ty : ty]) *) => { $ (unsafe impl DynSend for $ ty where $ ty : Send { }) * } ; }
/* FP:marker.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_MACRO_0010
/* FP:marker.rs-0022 */ already_send ! ([std :: backtrace :: Backtrace] [std :: io :: Stdout] [std :: io :: Stderr] [std :: io :: Error] [std :: fs :: File] [rustc_arena :: DroplessArena] [jobserver_crate :: Client] [jobserver_crate :: HelperThread] [crate :: memmap :: Mmap] [crate :: profiling :: SelfProfiler] [crate :: owned_slice :: OwnedSlice]) ;
/* FP:marker.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_MACRO_0011
/* FP:marker.rs-0024 */ macro_rules ! impl_dyn_send { ($ ($ ($ attr : meta) * [$ ty : ty where $ ($ generics2 : tt) *]) *) => { $ (unsafe impl <$ ($ generics2) *> DynSend for $ ty { }) * } ; }
/* FP:marker.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_MACRO_0012
/* FP:marker.rs-0026 */ impl_dyn_send ! ([std :: sync :: atomic :: AtomicPtr < T > where T] [std :: sync :: Mutex < T > where T : ? Sized + DynSend] [std :: sync :: mpsc :: Sender < T > where T : DynSend] [std :: sync :: Arc < T > where T : ? Sized + DynSync + DynSend] [std :: sync :: LazyLock < T , F > where T : DynSend , F : DynSend] [std :: collections :: HashSet < K , S > where K : DynSend , S : DynSend] [std :: collections :: HashMap < K , V , S > where K : DynSend , V : DynSend , S : DynSend] [std :: collections :: BTreeMap < K , V , A > where K : DynSend , V : DynSend , A : std :: alloc :: Allocator + Clone + DynSend] [Vec < T , A > where T : DynSend , A : std :: alloc :: Allocator + DynSend] [Box < T , A > where T : ? Sized + DynSend , A : std :: alloc :: Allocator + DynSend] [crate :: sync :: RwLock < T > where T : DynSend] [crate :: tagged_ptr :: TaggedRef <'a , P , T > where 'a , P : Sync , T : Send + crate :: tagged_ptr :: Tag] [rustc_arena :: TypedArena < T > where T : DynSend] [hashbrown :: HashTable < T > where T : DynSend] [indexmap :: IndexSet < V , S > where V : DynSend , S : DynSend] [indexmap :: IndexMap < K , V , S > where K : DynSend , V : DynSend , S : DynSend] [thin_vec :: ThinVec < T > where T : DynSend] [smallvec :: SmallVec < A > where A : smallvec :: Array + DynSend]) ;
/* FP:marker.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_MACRO_0013
/* FP:marker.rs-0028 */ macro_rules ! impls_dyn_sync_neg { ($ ([$ t1 : ty $ (where $ ($ generics1 : tt) *) ?]) *) => { $ (impl $ (<$ ($ generics1) *>) ? ! DynSync for $ t1 { }) * } ; }
/* FP:marker.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_MACRO_0014
/* FP:marker.rs-0030 */ impls_dyn_sync_neg ! ([std :: env :: Args] [std :: env :: ArgsOs] [* const T where T : ? Sized + PointeeSized] [* mut T where T : ? Sized + PointeeSized] [std :: cell :: Cell < T > where T : ? Sized] [std :: cell :: RefCell < T > where T : ? Sized] [std :: cell :: UnsafeCell < T > where T : ? Sized] [std :: ptr :: NonNull < T > where T : ? Sized + PointeeSized] [std :: rc :: Rc < T , A > where T : ? Sized , A : Allocator] [std :: rc :: Weak < T , A > where T : ? Sized , A : Allocator] [std :: cell :: OnceCell < T > where T] [std :: sync :: mpsc :: Receiver < T > where T] [std :: sync :: mpsc :: Sender < T > where T]) ;
/* FP:marker.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_IMPL_0015
/* FP:marker.rs-0032 */ # [cfg (any (unix , target_os = "hermit" , all (target_vendor = "fortanix" , target_env = "sgx") , target_os = "solid_asp3" , target_os = "wasi" , target_os = "xous"))] impl ! DynSync for std :: env :: VarsOs { }
/* FP:marker.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_MACRO_0016
/* FP:marker.rs-0034 */ macro_rules ! already_sync { ($ ([$ ty : ty]) *) => { $ (unsafe impl DynSync for $ ty where $ ty : Sync { }) * } ; }
/* FP:marker.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_MACRO_0017
/* FP:marker.rs-0036 */ already_sync ! ([std :: sync :: atomic :: AtomicBool] [std :: sync :: atomic :: AtomicUsize] [std :: sync :: atomic :: AtomicU8] [std :: sync :: atomic :: AtomicU32] [std :: backtrace :: Backtrace] [std :: io :: Error] [std :: fs :: File] [jobserver_crate :: Client] [jobserver_crate :: HelperThread] [crate :: memmap :: Mmap] [crate :: profiling :: SelfProfiler] [crate :: owned_slice :: OwnedSlice]) ;
/* FP:marker.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_MACRO_0018
/* FP:marker.rs-0038 */ # [cfg (target_has_atomic = "64")] already_sync ! ([std :: sync :: atomic :: AtomicU64]) ;
/* FP:marker.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_MACRO_0019
/* FP:marker.rs-0040 */ # [cfg (not (target_has_atomic = "64"))] already_sync ! ([portable_atomic :: AtomicU64]) ;
/* FP:marker.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_MACRO_0020
/* FP:marker.rs-0042 */ macro_rules ! impl_dyn_sync { ($ ($ ($ attr : meta) * [$ ty : ty where $ ($ generics2 : tt) *]) *) => { $ (unsafe impl <$ ($ generics2) *> DynSync for $ ty { }) * } ; }
/* FP:marker.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_MACRO_0021
/* FP:marker.rs-0044 */ impl_dyn_sync ! ([std :: sync :: atomic :: AtomicPtr < T > where T] [std :: sync :: OnceLock < T > where T : DynSend + DynSync] [std :: sync :: Mutex < T > where T : ? Sized + DynSend] [std :: sync :: Arc < T > where T : ? Sized + DynSync + DynSend] [std :: sync :: LazyLock < T , F > where T : DynSend + DynSync , F : DynSend] [std :: collections :: HashSet < K , S > where K : DynSync , S : DynSync] [std :: collections :: HashMap < K , V , S > where K : DynSync , V : DynSync , S : DynSync] [std :: collections :: BTreeMap < K , V , A > where K : DynSync , V : DynSync , A : std :: alloc :: Allocator + Clone + DynSync] [Vec < T , A > where T : DynSync , A : std :: alloc :: Allocator + DynSync] [Box < T , A > where T : ? Sized + DynSync , A : std :: alloc :: Allocator + DynSync] [crate :: sync :: RwLock < T > where T : DynSend + DynSync] [crate :: sync :: WorkerLocal < T > where T : DynSend] [crate :: intern :: Interned <'a , T > where 'a , T : DynSync] [crate :: tagged_ptr :: TaggedRef <'a , P , T > where 'a , P : Sync , T : Sync + crate :: tagged_ptr :: Tag] [parking_lot :: lock_api :: Mutex < R , T > where R : DynSync , T : ? Sized + DynSend] [parking_lot :: lock_api :: RwLock < R , T > where R : DynSync , T : ? Sized + DynSend + DynSync] [hashbrown :: HashTable < T > where T : DynSync] [indexmap :: IndexSet < V , S > where V : DynSync , S : DynSync] [indexmap :: IndexMap < K , V , S > where K : DynSync , V : DynSync , S : DynSync] [smallvec :: SmallVec < A > where A : smallvec :: Array + DynSync] [thin_vec :: ThinVec < T > where T : DynSync]) ;
/* FP:marker.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_FN_0022
/* FP:marker.rs-0046 */ pub fn assert_dyn_sync < T : ? Sized + PointeeSized + DynSync > () { }
/* FP:marker.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_FN_0023
/* FP:marker.rs-0048 */ pub fn assert_dyn_send < T : ? Sized + PointeeSized + DynSend > () { }
/* FP:marker.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_FN_0024
/* FP:marker.rs-0050 */ pub fn assert_dyn_send_val < T : ? Sized + PointeeSized + DynSend > (_t : & T) { }
/* FP:marker.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_FN_0025
/* FP:marker.rs-0052 */ pub fn assert_dyn_send_sync_val < T : ? Sized + PointeeSized + DynSync + DynSend > (_t : & T) { }
/* FP:marker.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_STRUCT_0026
/* FP:marker.rs-0054 */ # [derive (Copy , Clone)] pub struct FromDyn < T > (T) ;
/* FP:marker.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_IMPL_0027
/* FP:marker.rs-0056 */ impl < T > FromDyn < T > { # [inline (always)] pub fn from (val : T) -> Self { assert ! (crate :: sync :: is_dyn_thread_safe ()) ; FromDyn (val) } # [inline (always)] pub fn derive < O > (& self , val : O) -> FromDyn < O > { FromDyn (val) } # [inline (always)] pub fn into_inner (self) -> T { self . 0 } }
/* FP:marker.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_IMPL_0028
/* FP:marker.rs-0058 */ unsafe impl < T : DynSend > Send for FromDyn < T > { }
/* FP:marker.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_IMPL_0029
/* FP:marker.rs-0060 */ unsafe impl < T : DynSync > Sync for FromDyn < T > { }
/* FP:marker.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_IMPL_0030
/* FP:marker.rs-0062 */ impl < T > std :: ops :: Deref for FromDyn < T > { type Target = T ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } }
/* FP:marker.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_IMPL_0031
/* FP:marker.rs-0064 */ impl < T > std :: ops :: DerefMut for FromDyn < T > { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }
/* FP:marker.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_STRUCT_0032
/* FP:marker.rs-0066 */ # [derive (Copy , Clone)] pub struct IntoDynSyncSend < T : ? Sized + PointeeSized > (pub T) ;
/* FP:marker.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_IMPL_0033
/* FP:marker.rs-0068 */ unsafe impl < T : ? Sized + PointeeSized + Send > DynSend for IntoDynSyncSend < T > { }
/* FP:marker.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_IMPL_0034
/* FP:marker.rs-0070 */ unsafe impl < T : ? Sized + PointeeSized + Sync > DynSync for IntoDynSyncSend < T > { }
/* FP:marker.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_IMPL_0035
/* FP:marker.rs-0072 */ impl < T > std :: ops :: Deref for IntoDynSyncSend < T > { type Target = T ; # [inline (always)] fn deref (& self) -> & T { & self . 0 } }
/* FP:marker.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_marker_IMPL_0036
/* FP:marker.rs-0074 */ impl < T > std :: ops :: DerefMut for IntoDynSyncSend < T > { # [inline (always)] fn deref_mut (& mut self) -> & mut T { & mut self . 0 } }