// Generated macro for impl_57 (impl)
macro_rules! Depcrate_address_channelimpl_57 {
() => {
// Module: crate::address::channel
// Provides: {"impl_57"}
// Dependencies: {}
impl < A , M > WeakSender < M > for WeakAddressSender < A > where A : Handler < M > , A :: Context : ToEnvelope < A , M > , M :: Result : Send , M : Message + Send + 'static , { fn upgrade (& self) -> Option < Box < dyn Sender < M > + Sync > > { if let Some (inner) = WeakAddressSender :: upgrade (self) { Some (Box :: new (inner)) } else { None } } fn boxed (& self) -> Box < dyn WeakSender < M > + Sync > { Box :: new (self . clone ()) } }
};
}
