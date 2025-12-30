// Generated macro for if_alloc (module)
macro_rules! Depcrate_futureif_alloc {
() => {
// Module: crate::future
// Provides: {"if_alloc"}
// Dependencies: {}
# [cfg (feature = "alloc")] mod if_alloc { use super :: * ; use alloc :: boxed :: Box ; impl < F : FusedFuture + ? Sized + Unpin > FusedFuture for Box < F > { fn is_terminated (& self) -> bool { < F as FusedFuture > :: is_terminated (& * * self) } } # [cfg (feature = "std")] impl < F : FusedFuture > FusedFuture for std :: panic :: AssertUnwindSafe < F > { fn is_terminated (& self) -> bool { < F as FusedFuture > :: is_terminated (& * * self) } } }
};
}
