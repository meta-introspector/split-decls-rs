// Generated macro for impl_156 (impl)
macro_rules! Depcrate_onceimpl_156 {
() => {
// Module: crate::once
// Provides: {"impl_156"}
// Dependencies: {}
impl DispatchOnce { # [doc = " Creates a new `DispatchOnce`."] # [inline] # [allow (clippy :: new_without_default)] pub const fn new () -> Self { Self { predicate : UnsafeCell :: new (0) , } } # [doc = " Executes a closure once for the lifetime of the application."] # [doc = ""] # [doc = " If called simultaneously from multiple threads, this function waits"] # [doc = " synchronously until the work function has completed."] # [doc = ""] # [doc = ""] # [doc = " # Aborts"] # [doc = ""] # [doc = " The process will trap or abort if:"] # [doc = " - The given initialization closure unwinds."] # [doc = " - The given closure recursively invokes `call_once` on the same"] # [doc = "   `DispatchOnce` instance."] # [inline] # [doc (alias = "dispatch_once")] # [doc (alias = "dispatch_once_f")] pub fn call_once < F > (& self , work : F) where F : FnOnce () , { let predicate = NonNull :: new (self . predicate . get ()) . unwrap () ; if cfg ! (any (target_arch = "x86" , target_arch = "x86_64" , target_vendor = "apple")) { let atomic_predicate : & AtomicIsize = unsafe { predicate . cast () . as_ref () } ; if atomic_predicate . load (Ordering :: Acquire) != ! 0 { invoke_dispatch_once (predicate , work) ; } } else { invoke_dispatch_once (predicate , work) ; } } }
};
}
