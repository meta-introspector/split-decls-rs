macro_rules! deps {
    () => {
        AuthenticateFn!();
        Any!();
        Remote!();
    };
}

macro_rules! Connection {
    () => {
        deps!();
        # [doc = " A type to represent an ongoing connection to a remote host, typically with the connection already established."] # [doc = ""] # [doc = " It can be used to perform a variety of operations with the remote without worrying about protocol details,"] # [doc = " much like a remote procedure call."] pub struct Connection < 'a , 'repo , T > where T : Transport , { pub (crate) remote : & 'a Remote < 'repo > , pub (crate) authenticate : Option < AuthenticateFn < 'a > > , pub (crate) transport_options : Option < Box < dyn std :: any :: Any > > , pub (crate) transport : gix_protocol :: SendFlushOnDrop < T > , pub (crate) handshake : Option < gix_protocol :: Handshake > , pub (crate) trace : bool , }
    };
}

Connection!();