// Generated macro for Proxy (struct)
macro_rules! Depcrate_nonblockProxy {
() => {
// Module: crate::nonblock
// Provides: {"Proxy"}
// Dependencies: {}
# [doc = " A struct that wraps a connection, destination and path."] # [doc = ""] # [doc = " A D-Bus \"Proxy\" is a client-side object that corresponds to a remote object on the server side."] # [doc = " Calling methods on the proxy object calls methods on the remote object."] # [doc = " Read more in the [D-Bus tutorial](https://dbus.freedesktop.org/doc/dbus-tutorial.html#proxies)"] # [derive (Clone , Debug)] pub struct Proxy < 'a , C > { # [doc = " Destination, i e what D-Bus service you're communicating with"] pub destination : BusName < 'a > , # [doc = " Object path on the destination"] pub path : Path < 'a > , # [doc = " Some way to send and/or receive messages, non-blocking."] pub connection : C , # [doc = " Timeout for method calls"] pub timeout : Duration , }
};
}
