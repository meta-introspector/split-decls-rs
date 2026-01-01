/* FP:lock.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_USE_0001
/* FP:lock.rs-0002 */ use std :: fmt ;
/* FP:lock.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_ENUM_0002
/* FP:lock.rs-0004 */ # [derive (Clone , Copy , PartialEq)] pub enum Mode { NoSync , Sync , }
/* FP:lock.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_USE_0003
/* FP:lock.rs-0006 */ use std :: cell :: { Cell , UnsafeCell } ;
/* FP:lock.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_USE_0004
/* FP:lock.rs-0008 */ use std :: intrinsics :: unlikely ;
/* FP:lock.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_USE_0005
/* FP:lock.rs-0010 */ use std :: marker :: PhantomData ;
/* FP:lock.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_USE_0006
/* FP:lock.rs-0012 */ use std :: mem :: ManuallyDrop ;
/* FP:lock.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_USE_0007
/* FP:lock.rs-0014 */ use std :: ops :: { Deref , DerefMut } ;
/* FP:lock.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_USE_0008
/* FP:lock.rs-0016 */ use parking_lot :: RawMutex ;
/* FP:lock.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_USE_0009
/* FP:lock.rs-0018 */ use parking_lot :: lock_api :: RawMutex as _ ;
/* FP:lock.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_USE_0010
/* FP:lock.rs-0020 */ use crate :: sync :: { DynSend , DynSync , mode } ;
/* FP:lock.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_STRUCT_0011
/* FP:lock.rs-0022 */ # [doc = " A guard holding mutable access to a `Lock` which is in a locked state."] # [must_use = "if unused the Lock will immediately unlock"] pub struct LockGuard < 'a , T > { lock : & 'a Lock < T > , marker : PhantomData < & 'a mut T > , # [doc = " The synchronization mode of the lock. This is explicitly passed to let LLVM relate it"] # [doc = " to the original lock operation."] mode : Mode , }
/* FP:lock.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_IMPL_0012
/* FP:lock.rs-0024 */ impl < 'a , T : 'a > Deref for LockGuard < 'a , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . lock . data . get () } } }
/* FP:lock.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_IMPL_0013
/* FP:lock.rs-0026 */ impl < 'a , T : 'a > DerefMut for LockGuard < 'a , T > { # [inline] fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . lock . data . get () } } }
/* FP:lock.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_IMPL_0014
/* FP:lock.rs-0028 */ impl < 'a , T : 'a > Drop for LockGuard < 'a , T > { # [inline] fn drop (& mut self) { match self . mode { Mode :: NoSync => { let cell = unsafe { & self . lock . mode_union . no_sync } ; debug_assert ! (cell . get ()) ; cell . set (false) ; } Mode :: Sync => unsafe { self . lock . mode_union . sync . unlock () } , } } }
/* FP:lock.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_OTHER_0015
/* FP:lock.rs-0030 */ union ModeUnion { # [doc = " Indicates if the cell is locked. Only used if `Lock.mode` is `NoSync`."] no_sync : ManuallyDrop < Cell < bool > > , # [doc = " A lock implementation that's only used if `Lock.mode` is `Sync`."] sync : ManuallyDrop < RawMutex > , }
/* FP:lock.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_CONST_0016
/* FP:lock.rs-0032 */ # [doc = " The value representing a locked state for the `Cell`."] const LOCKED : bool = true ;
/* FP:lock.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_STRUCT_0017
/* FP:lock.rs-0034 */ # [doc = " A lock which only uses synchronization if `might_be_dyn_thread_safe` is true."] # [doc = " It implements `DynSend` and `DynSync` instead of the typical `Send` and `Sync`."] pub struct Lock < T > { # [doc = " Indicates if synchronization is used via `mode_union.sync` if it's `Sync`, or if a"] # [doc = " not thread safe cell is used via `mode_union.no_sync` if it's `NoSync`."] # [doc = " This is set on initialization and never changed."] mode : Mode , mode_union : ModeUnion , data : UnsafeCell < T > , }
/* FP:lock.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_IMPL_0018
/* FP:lock.rs-0036 */ impl < T > Lock < T > { # [inline (always)] pub fn new (inner : T) -> Self { let (mode , mode_union) = if unlikely (mode :: might_be_dyn_thread_safe ()) { (Mode :: Sync , ModeUnion { sync : ManuallyDrop :: new (RawMutex :: INIT) }) } else { (Mode :: NoSync , ModeUnion { no_sync : ManuallyDrop :: new (Cell :: new (! LOCKED)) }) } ; Lock { mode , mode_union , data : UnsafeCell :: new (inner) } } # [inline (always)] pub fn into_inner (self) -> T { self . data . into_inner () } # [inline (always)] pub fn get_mut (& mut self) -> & mut T { self . data . get_mut () } # [inline (always)] pub fn try_lock (& self) -> Option < LockGuard < '_ , T > > { let mode = self . mode ; match mode { Mode :: NoSync => { let cell = unsafe { & self . mode_union . no_sync } ; let was_unlocked = cell . get () != LOCKED ; if was_unlocked { cell . set (LOCKED) ; } was_unlocked } Mode :: Sync => unsafe { self . mode_union . sync . try_lock () } , } . then (| | LockGuard { lock : self , marker : PhantomData , mode }) } # [doc = " This acquires the lock assuming synchronization is in a specific mode."] # [doc = ""] # [doc = " Safety"] # [doc = " This method must only be called with `Mode::Sync` if `might_be_dyn_thread_safe` was"] # [doc = " true on lock creation."] # [inline (always)] # [track_caller] pub unsafe fn lock_assume (& self , mode : Mode) -> LockGuard < '_ , T > { # [inline (never)] # [track_caller] # [cold] fn lock_held () -> ! { panic ! ("lock was already held") } unsafe { match mode { Mode :: NoSync => { if unlikely (self . mode_union . no_sync . replace (LOCKED) == LOCKED) { lock_held () } } Mode :: Sync => self . mode_union . sync . lock () , } } LockGuard { lock : self , marker : PhantomData , mode } } # [inline (always)] # [track_caller] pub fn lock (& self) -> LockGuard < '_ , T > { unsafe { self . lock_assume (self . mode) } } }
/* FP:lock.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_IMPL_0019
/* FP:lock.rs-0038 */ unsafe impl < T : DynSend > DynSend for Lock < T > { }
/* FP:lock.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_IMPL_0020
/* FP:lock.rs-0040 */ unsafe impl < T : DynSend > DynSync for Lock < T > { }
/* FP:lock.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_IMPL_0021
/* FP:lock.rs-0042 */ impl < T > Lock < T > { # [inline (always)] # [track_caller] pub fn with_lock < F : FnOnce (& mut T) -> R , R > (& self , f : F) -> R { f (& mut * self . lock ()) } # [inline (always)] # [track_caller] pub fn borrow (& self) -> LockGuard < '_ , T > { self . lock () } # [inline (always)] # [track_caller] pub fn borrow_mut (& self) -> LockGuard < '_ , T > { self . lock () } }
/* FP:lock.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_IMPL_0022
/* FP:lock.rs-0044 */ impl < T : Default > Default for Lock < T > { # [inline] fn default () -> Self { Lock :: new (T :: default ()) } }
/* FP:lock.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_lock_IMPL_0023
/* FP:lock.rs-0046 */ impl < T : fmt :: Debug > fmt :: Debug for Lock < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . try_lock () { Some (guard) => f . debug_struct ("Lock") . field ("data" , & & * guard) . finish () , None => { struct LockedPlaceholder ; impl fmt :: Debug for LockedPlaceholder { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("<locked>") } } f . debug_struct ("Lock") . field ("data" , & LockedPlaceholder) . finish () } } } }