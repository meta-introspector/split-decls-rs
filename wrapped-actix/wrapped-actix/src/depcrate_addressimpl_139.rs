// Generated macro for impl_139 (impl)
macro_rules! Depcrate_addressimpl_139 {
() => {
// Module: crate::address
// Provides: {"impl_139"}
// Dependencies: {}
impl < M > WeakRecipient < M > where M : Message + Send , M :: Result : Send , { pub (crate) fn new (wtx : Box < dyn WeakSender < M > + Sync >) -> WeakRecipient < M > { WeakRecipient { wtx } } # [doc = " Attempts to upgrade the `WeakRecipient<M>` pointer to an `Recipient<M>`, similar to `WeakAddr<A>`"] pub fn upgrade (& self) -> Option < Recipient < M > > { self . wtx . upgrade () . map (Recipient :: new) } }
};
}
