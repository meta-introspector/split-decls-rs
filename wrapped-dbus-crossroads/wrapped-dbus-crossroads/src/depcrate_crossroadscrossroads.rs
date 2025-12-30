// Generated macro for Crossroads (struct)
macro_rules! Depcrate_crossroadsCrossroads {
() => {
// Module: crate::crossroads
// Provides: {"Crossroads"}
// Dependencies: {}
# [doc = " Crossroads is the \"main\" object, containing object paths, a registry of interfaces, and"] # [doc = " a crossreference of which object paths implement which interfaces."] # [doc = ""] # [doc = " You can store some arbitrary data with every object path if you like. This data can then be"] # [doc = " accessed from within the method callbacks. If you do not want this, just pass `()` as your data."] # [doc = ""] # [doc = " Crossroads can contain callbacks and data which is Send, but Sync is not required. Hence"] # [doc = " Crossroads itself is Send but not Sync."] # [derive (Debug)] pub struct Crossroads { map : BTreeMap < dbus :: Path < 'static > , Object > , registry : Registry , add_standard_ifaces : bool , async_support : Option < AsyncSupport > , object_manager_support : Option < Dbg < Arc < dyn Sender + Send + Sync + 'static > > > , }
};
}
