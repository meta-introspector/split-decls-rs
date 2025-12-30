// Generated macro for object_manager_path_removed (function)
macro_rules! Depcrate_stdimplobject_manager_path_removed {
() => {
// Module: crate::stdimpl
// Provides: {"object_manager_path_removed"}
// Dependencies: {}
pub fn object_manager_path_removed (sender : Arc < dyn Sender + Send + Sync > , name : & dbus :: Path < 'static > , cr : & mut Crossroads) { object_manager_parents (name , cr , | parent , cr | { let (reg , ifaces) = cr . registry_and_ifaces (& name) ; let x = dbus :: blocking :: stdintf :: org_freedesktop_dbus :: ObjectManagerInterfacesRemoved { object : name . clone () , interfaces : ifaces . into_iter () . filter_map (| iface | reg . get_intf_name (* iface)) . map (| iface | String :: from (& * * iface)) . collect () , } ; let _ = sender . send (dbus :: message :: SignalArgs :: to_emit_message (& x , & parent)) ; }) ; }
};
}
