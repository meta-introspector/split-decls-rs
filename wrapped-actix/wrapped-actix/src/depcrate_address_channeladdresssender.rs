// Generated macro for AddressSender (struct)
macro_rules! Depcrate_address_channelAddressSender {
() => {
// Module: crate::address::channel
// Provides: {"AddressSender"}
// Dependencies: {}
# [doc = " The transmission end of a channel which is used to send values."] # [doc = ""] # [doc = " This is created by the `channel` method."] pub struct AddressSender < A : Actor > { inner : Arc < Inner < A > > , sender_task : Arc < Mutex < SenderTask > > , maybe_parked : Arc < AtomicBool > , }
};
}
