// Generated macro for WeakRecipient (struct)
macro_rules! Depcrate_addressWeakRecipient {
() => {
// Module: crate::address
// Provides: {"WeakRecipient"}
// Dependencies: {}
# [doc = " A weakly referenced counterpart to `Recipient<M>`"] pub struct WeakRecipient < M : Message > where M : Message + Send , M :: Result : Send , { wtx : Box < dyn WeakSender < M > + Sync > , }
};
}
