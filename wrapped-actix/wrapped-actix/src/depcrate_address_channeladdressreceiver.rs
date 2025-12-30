// Generated macro for AddressReceiver (struct)
macro_rules! Depcrate_address_channelAddressReceiver {
() => {
// Module: crate::address::channel
// Provides: {"AddressReceiver"}
// Dependencies: {}
# [doc = " The receiving end of a channel which implements the `Stream` trait."] # [doc = ""] # [doc = " This is a concrete implementation of a stream which can be used to represent"] # [doc = " a stream of values being computed elsewhere. This is created by the"] # [doc = " `channel` method."] pub struct AddressReceiver < A : Actor > { inner : Arc < Inner < A > > , }
};
}
