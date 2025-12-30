// Generated macro for object_manager (function)
macro_rules! Depcrate_stdimplobject_manager {
() => {
// Module: crate::stdimpl
// Provides: {"object_manager"}
// Dependencies: {}
pub fn object_manager (cr : & mut Crossroads) -> IfaceToken < () > { cr . register ("org.freedesktop.DBus.ObjectManager" , | b | { b . method_with_cr_custom :: < () , (PathPropMap ,) , _ , _ > ("GetManagedObjects" , () , ("objpath_interfaces_and_properties" ,) , get_managed_objects) ; b . signal :: < (dbus :: Path < 'static > , IfacePropMap) , _ > ("InterfacesAdded" , ("object_path" , "interfaces_and_properties")) ; b . signal :: < (dbus :: Path < 'static > , Vec < String >) , _ > ("InterfacesRemoved" , ("object_path" , "interfaces")) ; }) }
};
}
