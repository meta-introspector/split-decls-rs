// Generated macro for impl_2095 (impl)
macro_rules! Depcrate_compat_compat03as01impl_2095 {
() => {
// Module: crate::compat::compat03as01
// Provides: {"impl_2095"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < T , Item > Sink01 for CompatSink < T , Item > where T : Sink03 < Item > + Unpin , { type SinkItem = Item ; type SinkError = T :: Error ; fn start_send (& mut self , item : Self :: SinkItem) -> StartSend01 < Self :: SinkItem , Self :: SinkError > { with_sink_context (self , | mut inner , cx | match inner . as_mut () . poll_ready (cx) ? { task03 :: Poll :: Ready (()) => inner . start_send (item) . map (| () | AsyncSink01 :: Ready) , task03 :: Poll :: Pending => Ok (AsyncSink01 :: NotReady (item)) , }) } fn poll_complete (& mut self) -> Poll01 < () , Self :: SinkError > { with_sink_context (self , | inner , cx | poll_03_to_01 (inner . poll_flush (cx))) } fn close (& mut self) -> Poll01 < () , Self :: SinkError > { with_sink_context (self , | inner , cx | poll_03_to_01 (inner . poll_close (cx))) } }
};
}
