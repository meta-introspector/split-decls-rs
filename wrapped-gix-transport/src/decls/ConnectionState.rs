macro_rules! deps {
    () => {
        ConnectMode!();
        Connection!();
        Protocol!();
    };
}

macro_rules! ConnectionState {
    () => {
        deps!();
        # [doc = " Connection state shared between blocking and async connections."] pub (crate) struct ConnectionState { pub (in crate :: client) path : BString , pub (in crate :: client) virtual_host : Option < (String , Option < u16 >) > , pub (in crate :: client) desired_version : Protocol , custom_url : Option < BString > , pub (in crate :: client) mode : ConnectMode , }
    };
}

ConnectionState!();