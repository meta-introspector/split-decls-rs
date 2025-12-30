// Generated macro for WriteHandler (trait)
macro_rules! Depcrate_ioWriteHandler {
() => {
// Module: crate::io
// Provides: {"WriteHandler"}
// Dependencies: {}
# [doc = " A helper trait for write handling."] # [doc = ""] # [doc = " `WriteHandler` is a helper for `AsyncWrite` types. Implementation"] # [doc = " of this trait is required for `Writer` and `FramedWrite` support."] # [allow (unused_variables)] pub trait WriteHandler < E > where Self : Actor , Self :: Context : ActorContext , { # [doc = " Called when the writer emits error."] # [doc = ""] # [doc = " If this method returns `ErrorAction::Continue` writer processing"] # [doc = " continues otherwise stream processing stops."] fn error (& mut self , err : E , ctx : & mut Self :: Context) -> Running { Running :: Stop } # [doc = " Called when the writer finishes."] # [doc = ""] # [doc = " By default this method stops actor's `Context`."] fn finished (& mut self , ctx : & mut Self :: Context) { ctx . stop () } }
};
}
