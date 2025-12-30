// Generated macro for object_manager_interface_removed (function)
macro_rules! Depcrate_stdimplobject_manager_interface_removed {
() => {
// Module: crate::stdimpl
// Provides: {"object_manager_interface_removed"}
// Dependencies: {}
pub fn object_manager_interface_removed (sender : Arc < dyn Sender + Send + Sync > , name : & dbus :: Path < 'static > , itoken : usize , cr : & mut Crossroads) { object_manager_parents (name , cr , | parent , cr | { let (reg , _ifaces) = cr . registry_and_ifaces (& name) ; if let Some (iface) = reg . get_intf_name (itoken) { let x = dbus :: blocking :: stdintf :: org_freedesktop_dbus :: ObjectManagerInterfacesRemoved { object : name . clone () , interfaces : vec ! [iface . to_string ()] , } ; let _ = sender . send (dbus :: message :: SignalArgs :: to_emit_message (& x , & parent)) ; } }) ; }
};
}
