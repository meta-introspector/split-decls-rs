// Generated macro for object_manager_interface_added (function)
macro_rules! Depcrate_stdimplobject_manager_interface_added {
() => {
// Module: crate::stdimpl
// Provides: {"object_manager_interface_added"}
// Dependencies: {}
pub fn object_manager_interface_added (sender : Arc < dyn Sender + Send + Sync > , name : & dbus :: Path < 'static > , itoken : usize , cr : & mut Crossroads) { object_manager_parents (name , cr , | parent , cr | { let n = name . clone () ; let s = sender . clone () ; for_each_interface_with_properties (& name , vec ! [itoken] , cr , None , move | ictx , _ | { let x = dbus :: blocking :: stdintf :: org_freedesktop_dbus :: ObjectManagerInterfacesAdded { object : n , interfaces : std :: mem :: replace (& mut ictx . ifaces , HashMap :: new ()) , } ; let _ = s . send (dbus :: message :: SignalArgs :: to_emit_message (& x , & parent)) ; }) ; }) ; }
};
}
