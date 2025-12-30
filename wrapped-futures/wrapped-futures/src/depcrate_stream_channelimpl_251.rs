// Generated macro for impl_251 (impl)
macro_rules! Depcrate_stream_channelimpl_251 {
() => {
// Module: crate::stream::channel
// Provides: {"impl_251"}
// Dependencies: {}
impl < T , E > Drop for Receiver < T , E > where T : Send + 'static , E : Send + 'static , { fn drop (& mut self) { self . inner . receiver_gone . store (true , Ordering :: SeqCst) ; if let Some (token) = self . on_full_token . take () { self . inner . slot . cancel (token) ; } self . inner . slot . on_full (| slot | { drop (slot . try_consume ()) ; }) ; } }
};
}
