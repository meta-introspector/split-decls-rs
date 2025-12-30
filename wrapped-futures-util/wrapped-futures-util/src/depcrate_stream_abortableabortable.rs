// Generated macro for abortable (function)
macro_rules! Depcrate_stream_abortableabortable {
() => {
// Module: crate::stream::abortable
// Provides: {"abortable"}
// Dependencies: {}
# [doc = " Creates a new `Abortable` stream and an `AbortHandle` which can be used to stop it."] # [doc = ""] # [doc = " This function is a convenient (but less flexible) alternative to calling"] # [doc = " `AbortHandle::new` and `Abortable::new` manually."] # [doc = ""] # [doc = " This function is only available when the `std` or `alloc` feature of this"] # [doc = " library is activated, and it is activated by default."] pub fn abortable < St > (stream : St) -> (Abortable < St > , AbortHandle) where St : Stream , { let (handle , reg) = AbortHandle :: new_pair () ; let abortable = assert_stream :: < St :: Item , _ > (Abortable :: new (stream , reg)) ; (abortable , handle) }
};
}
