// Generated macro for peer (function)
macro_rules! Depcrate_channelpeer {
() => {
// Module: crate::channel
// Provides: {"peer"}
// Dependencies: {}
# [doc = " Replies if this is a call to org.freedesktop.DBus.Peer, otherwise returns None."] fn peer (m : & Message) -> Option < Message > { if let Some (intf) = m . interface () { if & * intf != "org.freedesktop.DBus.Peer" { return None ; } if let Some (method) = m . member () { if & * method == "Ping" { return Some (m . method_return ()) } if & * method == "GetMachineId" { let mut r = m . method_return () ; unsafe { let id = ffi :: dbus_get_local_machine_id () ; if ! id . is_null () { r = r . append1 (c_str_to_slice (& (id as * const _)) . unwrap ()) ; ffi :: dbus_free (id as * mut _) ; return Some (r) } } return Some (m . error (& "org.freedesktop.DBus.Error.Failed" . into () , & to_c_str ("Failed to retrieve UUID"))) } } Some (m . error (& "org.freedesktop.DBus.Error.UnknownMethod" . into () , & to_c_str ("Method does not exist"))) } else { None } }
};
}
