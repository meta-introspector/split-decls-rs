// Generated macro for WeakSender (struct)
macro_rules! DepcrateWeakSender {
() => {
// Module: crate
// Provides: {"WeakSender"}
// Dependencies: {}
# [doc = " A [`Sender`] that does not prevent the channel from being closed."] # [doc = ""] # [doc = " This is created through the [`Sender::downgrade`] method. In order to use it, it needs"] # [doc = " to be upgraded into a [`Sender`] through the `upgrade` method."] pub struct WeakSender < T > { channel : Arc < Channel < T > > , }
};
}
