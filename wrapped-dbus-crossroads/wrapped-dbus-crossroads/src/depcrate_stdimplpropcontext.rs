// Generated macro for PropContext (struct)
macro_rules! Depcrate_stdimplPropContext {
() => {
// Module: crate::stdimpl
// Provides: {"PropContext"}
// Dependencies: {}
# [derive (Debug)] # [doc = " PropContext is a struct that provides helpful information inside a get/set property handler."] # [doc = ""] # [doc = " Like Context, but for get/set property handlers."] pub struct PropContext { path : dbus :: Path < 'static > , interface : dbus :: strings :: Interface < 'static > , name : String , context : Option < Context > , iface_token : usize , emits_changed : Option < & 'static str > , get_all : Option < Arc < Mutex < PropAllCtx > > > , }
};
}
