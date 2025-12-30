// Generated macro for Context (struct)
macro_rules! Depcrate_contextContext {
() => {
// Module: crate::context
// Provides: {"Context"}
// Dependencies: {}
# [doc = " Context is the struct that accompanies you through your method call handler,"] # [doc = " providing helpful information about the message sent from the client, as well as"] # [doc = " some methods to send extra messages (typically signals) in return."] # [derive (Debug)] pub struct Context { path : dbus :: Path < 'static > , interface : Option < dbus :: strings :: Interface < 'static > > , method : dbus :: strings :: Member < 'static > , message : dbus :: Message , has_error : bool , reply : Option < dbus :: Message > , send_extra : Vec < dbus :: Message > , send_on_drop : Option < Dbg < Arc < dyn Sender + Send + Sync > > > , }
};
}
