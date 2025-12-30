// Generated macro for WeakAddressSender (struct)
macro_rules! Depcrate_address_channelWeakAddressSender {
() => {
// Module: crate::address::channel
// Provides: {"WeakAddressSender"}
// Dependencies: {}
# [doc = " A weakly referenced version of `AddressSender`."] # [doc = ""] # [doc = " This is created by the `AddressSender::downgrade` method."] pub struct WeakAddressSender < A : Actor > { inner : Weak < Inner < A > > , }
};
}
