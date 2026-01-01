/* FP:sync.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_USE_0001
/* FP:sync.rs-0002 */ use std :: collections :: HashMap ;
/* FP:sync.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_USE_0002
/* FP:sync.rs-0004 */ use std :: hash :: { BuildHasher , Hash } ;
/* FP:sync.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_USE_0003
/* FP:sync.rs-0006 */ pub use parking_lot :: { MappedRwLockReadGuard as MappedReadGuard , MappedRwLockWriteGuard as MappedWriteGuard , RwLockReadGuard as ReadGuard , RwLockWriteGuard as WriteGuard , } ;
/* FP:sync.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_USE_0004
/* FP:sync.rs-0008 */ pub use self :: atomic :: AtomicU64 ;
/* FP:sync.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_USE_0005
/* FP:sync.rs-0010 */ pub use self :: freeze :: { FreezeLock , FreezeReadGuard , FreezeWriteGuard } ;
/* FP:sync.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_USE_0006
/* FP:sync.rs-0012 */ # [doc (no_inline)] pub use self :: lock :: { Lock , LockGuard , Mode } ;
/* FP:sync.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_USE_0007
/* FP:sync.rs-0014 */ pub use self :: mode :: { is_dyn_thread_safe , set_dyn_thread_safe_mode } ;
/* FP:sync.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_USE_0008
/* FP:sync.rs-0016 */ pub use self :: parallel :: { broadcast , join , par_for_each_in , par_map , parallel_guard , scope , spawn , try_par_for_each_in , } ;
/* FP:sync.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_USE_0009
/* FP:sync.rs-0018 */ pub use self :: vec :: { AppendOnlyIndexVec , AppendOnlyVec } ;
/* FP:sync.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_USE_0010
/* FP:sync.rs-0020 */ pub use self :: worker_local :: { Registry , WorkerLocal } ;
/* FP:sync.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_USE_0011
/* FP:sync.rs-0022 */ pub use crate :: marker :: * ;
/* FP:sync.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_MOD_0012
/* FP:sync.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_MOD_0013
/* FP:sync.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_MOD_0014
/* FP:sync.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_MOD_0015
/* FP:sync.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_MOD_0016
/* FP:sync.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_MOD_0017
/* FP:sync.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_MOD_0018
/* FP:sync.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_STRUCT_0019
/* FP:sync.rs-0038 */ # [derive (Debug , Default)] pub struct MTLock < T > (Lock < T >) ;
/* FP:sync.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_IMPL_0020
/* FP:sync.rs-0040 */ impl < T > MTLock < T > { # [inline (always)] pub fn new (inner : T) -> Self { MTLock (Lock :: new (inner)) } # [inline (always)] pub fn into_inner (self) -> T { self . 0 . into_inner () } # [inline (always)] pub fn get_mut (& mut self) -> & mut T { self . 0 . get_mut () } # [inline (always)] pub fn lock (& self) -> LockGuard < '_ , T > { self . 0 . lock () } # [inline (always)] pub fn lock_mut (& self) -> LockGuard < '_ , T > { self . lock () } }
/* FP:sync.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_CONST_0021
/* FP:sync.rs-0042 */ # [doc = " This makes locks panic if they are already held."] # [doc = " It is only useful when you are running in a single thread"] const ERROR_CHECKING : bool = false ;
/* FP:sync.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_STRUCT_0022
/* FP:sync.rs-0044 */ # [derive (Default)] # [repr (align (64))] pub struct CacheAligned < T > (pub T) ;
/* FP:sync.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_TRAIT_0023
/* FP:sync.rs-0046 */ pub trait HashMapExt < K , V > { # [doc = " Same as HashMap::insert, but it may panic if there's already an"] # [doc = " entry for `key` with a value not equal to `value`"] fn insert_same (& mut self , key : K , value : V) ; }
/* FP:sync.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_IMPL_0024
/* FP:sync.rs-0048 */ impl < K : Eq + Hash , V : Eq , S : BuildHasher > HashMapExt < K , V > for HashMap < K , V , S > { fn insert_same (& mut self , key : K , value : V) { self . entry (key) . and_modify (| old | assert ! (* old == value)) . or_insert (value) ; } }
/* FP:sync.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_STRUCT_0025
/* FP:sync.rs-0050 */ # [derive (Debug , Default)] pub struct RwLock < T > (parking_lot :: RwLock < T >) ;
/* FP:sync.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_IMPL_0026
/* FP:sync.rs-0052 */ impl < T > RwLock < T > { # [inline (always)] pub fn new (inner : T) -> Self { RwLock (parking_lot :: RwLock :: new (inner)) } # [inline (always)] pub fn into_inner (self) -> T { self . 0 . into_inner () } # [inline (always)] pub fn get_mut (& mut self) -> & mut T { self . 0 . get_mut () } # [inline (always)] pub fn read (& self) -> ReadGuard < '_ , T > { if ERROR_CHECKING { self . 0 . try_read () . expect ("lock was already held") } else { self . 0 . read () } } # [inline (always)] pub fn try_write (& self) -> Result < WriteGuard < '_ , T > , () > { self . 0 . try_write () . ok_or (()) } # [inline (always)] pub fn write (& self) -> WriteGuard < '_ , T > { if ERROR_CHECKING { self . 0 . try_write () . expect ("lock was already held") } else { self . 0 . write () } } # [inline (always)] # [track_caller] pub fn borrow (& self) -> ReadGuard < '_ , T > { self . read () } # [inline (always)] # [track_caller] pub fn borrow_mut (& self) -> WriteGuard < '_ , T > { self . write () } }