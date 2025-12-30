// Generated macro for org_freedesktop_dbus_introspectable_server (function)
macro_rules! Depcrate_methodtypeorg_freedesktop_dbus_introspectable_server {
() => {
// Module: crate::methodtype
// Provides: {"org_freedesktop_dbus_introspectable_server"}
// Dependencies: {}
pub fn org_freedesktop_dbus_introspectable_server < M , D > (factory : & super :: Factory < M , D > , data : D :: Interface) -> super :: Interface < M , D > where D : super :: DataType , D :: Method : Default , M : MethodType < D > , { let i = factory . interface ("org.freedesktop.DBus.Introspectable" , data) ; let h = move | minfo : & super :: MethodInfo < M , D > | { let d : & dyn stdintf :: OrgFreedesktopDBusIntrospectable < Err = super :: MethodErr > = minfo ; let arg0 = d . introspect () ? ; let rm = minfo . msg . method_return () ; let rm = rm . append1 (arg0) ; Ok (vec ! (rm)) } ; let m = factory . method_sync ("Introspect" , Default :: default () , h) ; let m = m . out_arg (("xml_data" , "s")) ; i . add_m (m) }
};
}
