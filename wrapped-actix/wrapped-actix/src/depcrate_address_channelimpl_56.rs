// Generated macro for impl_56 (impl)
macro_rules! Depcrate_address_channelimpl_56 {
() => {
// Module: crate::address::channel
// Provides: {"impl_56"}
// Dependencies: {}
impl < A : Actor > WeakAddressSender < A > { # [doc = " Attempts to upgrade the `WeakAddressSender<A>` pointer to an [`AddressSender<A>`]"] # [doc = ""] # [doc = " Returns [`None`] if the actor has since been dropped."] pub fn upgrade (& self) -> Option < AddressSender < A > > { Weak :: upgrade (& self . inner) . map (| inner | AddressSenderProducer { inner } . sender ()) } }
};
}
