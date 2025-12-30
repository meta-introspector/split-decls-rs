// Generated macro for prelude (module)
macro_rules! Depcrate_syncprelude {
() => {
// Module: crate::sync
// Provides: {"prelude"}
// Dependencies: {}
# [cfg (not (loom))] pub (crate) mod prelude { use super :: { atomic , cell } ; # [doc = " Emulate `loom::UnsafeCell`'s API."] pub (crate) trait UnsafeCellExt { type Value ; fn with_mut < R , F > (& self , f : F) -> R where F : FnOnce (* mut Self :: Value) -> R ; } impl < T > UnsafeCellExt for cell :: UnsafeCell < T > { type Value = T ; fn with_mut < R , F > (& self , f : F) -> R where F : FnOnce (* mut Self :: Value) -> R , { f (self . get ()) } } # [doc = " Emulate `loom::Atomic*`'s API."] pub (crate) trait AtomicExt { type Value ; fn with_mut < R , F > (& mut self , f : F) -> R where F : FnOnce (& mut Self :: Value) -> R ; } impl AtomicExt for atomic :: AtomicUsize { type Value = usize ; fn with_mut < R , F > (& mut self , f : F) -> R where F : FnOnce (& mut Self :: Value) -> R , { f (self . get_mut ()) } } impl < T > AtomicExt for atomic :: AtomicPtr < T > { type Value = * mut T ; fn with_mut < R , F > (& mut self , f : F) -> R where F : FnOnce (& mut Self :: Value) -> R , { f (self . get_mut ()) } } }
};
}
