// Generated macro for impl_253 (impl)
macro_rules! Depcrate_stream_channelimpl_253 {
() => {
// Module: crate::stream::channel
// Provides: {"impl_253"}
// Dependencies: {}
impl < T , E > Drop for Sender < T , E > where T : Send + 'static , E : Send + 'static , { fn drop (& mut self) { self . inner . slot . on_empty (| slot | { slot . try_produce (Message :: Done) . ok () . unwrap () ; }) ; } }
};
}
