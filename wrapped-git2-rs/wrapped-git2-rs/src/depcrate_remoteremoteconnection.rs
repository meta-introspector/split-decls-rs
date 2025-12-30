// Generated macro for RemoteConnection (struct)
macro_rules! Depcrate_remoteRemoteConnection {
() => {
// Module: crate::remote
// Provides: {"RemoteConnection"}
// Dependencies: {}
# [doc = " Holds callbacks for a connection to a `Remote`. Disconnects when dropped"] pub struct RemoteConnection < 'repo , 'connection , 'cb > { _callbacks : Box < RemoteCallbacks < 'cb > > , _proxy : ProxyOptions < 'cb > , remote : & 'connection mut Remote < 'repo > , }
};
}
