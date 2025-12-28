macro_rules! deps {
    () => {
        RemoteCallbacks!();
        Remote!();
        ProxyOptions!();
    };
}

macro_rules! RemoteConnection {
    () => {
        deps!();
        # [doc = " Holds callbacks for a connection to a `Remote`. Disconnects when dropped"] pub struct RemoteConnection < 'repo , 'connection , 'cb > { _callbacks : Box < RemoteCallbacks < 'cb > > , _proxy : ProxyOptions < 'cb > , remote : & 'connection mut Remote < 'repo > , }
    };
}

RemoteConnection!()