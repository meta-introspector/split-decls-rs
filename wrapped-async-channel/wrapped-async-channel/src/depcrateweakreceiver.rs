// Generated macro for WeakReceiver (struct)
macro_rules! DepcrateWeakReceiver {
() => {
// Module: crate
// Provides: {"WeakReceiver"}
// Dependencies: {}
# [doc = " A [`Receiver`] that does not prevent the channel from being closed."] # [doc = ""] # [doc = " This is created through the [`Receiver::downgrade`] method. In order to use it, it needs"] # [doc = " to be upgraded into a [`Receiver`] through the `upgrade` method."] pub struct WeakReceiver < T > { channel : Arc < Channel < T > > , }
};
}
