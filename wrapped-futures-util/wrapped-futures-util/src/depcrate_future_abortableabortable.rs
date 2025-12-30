// Generated macro for abortable (function)
macro_rules! Depcrate_future_abortableabortable {
() => {
// Module: crate::future::abortable
// Provides: {"abortable"}
// Dependencies: {}
# [doc = " Creates a new `Abortable` future and an `AbortHandle` which can be used to stop it."] # [doc = ""] # [doc = " This function is a convenient (but less flexible) alternative to calling"] # [doc = " `AbortHandle::new` and `Abortable::new` manually."] # [doc = ""] # [doc = " This function is only available when the `std` or `alloc` feature of this"] # [doc = " library is activated, and it is activated by default."] pub fn abortable < Fut > (future : Fut) -> (Abortable < Fut > , AbortHandle) where Fut : Future , { let (handle , reg) = AbortHandle :: new_pair () ; let abortable = assert_future :: < Result < Fut :: Output , Aborted > , _ > (Abortable :: new (future , reg)) ; (abortable , handle) }
};
}
