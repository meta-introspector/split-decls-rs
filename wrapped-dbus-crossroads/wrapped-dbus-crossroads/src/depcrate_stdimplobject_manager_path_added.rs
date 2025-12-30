// Generated macro for object_manager_path_added (function)
macro_rules! Depcrate_stdimplobject_manager_path_added {
() => {
// Module: crate::stdimpl
// Provides: {"object_manager_path_added"}
// Dependencies: {}
pub fn object_manager_path_added (sender : Arc < dyn Sender + Send + Sync > , name : & dbus :: Path < 'static > , cr : & mut Crossroads) { object_manager_parents (name , cr , | parent , cr | { let n = name . clone () ; let s = sender . clone () ; get_all_for_path (& name , cr , None , move | ictx , _ | { let x = dbus :: blocking :: stdintf :: org_freedesktop_dbus :: ObjectManagerInterfacesAdded { object : n , interfaces : std :: mem :: replace (& mut ictx . ifaces , HashMap :: new ()) , } ; let _ = s . send (dbus :: message :: SignalArgs :: to_emit_message (& x , & parent)) ; }) ; }) ; }
};
}
