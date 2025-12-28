macro_rules! deps {
    () => {
        Server!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        # [doc = " Access"] impl Server { # [doc = " Return the list of capabilities we are allowed to use, as negotiated with the client."] pub fn capabilities (& self) -> & HashSet < String > { & self . capabilities } # [doc = " Return the negotiated version of the protocol."] pub fn version (& self) -> usize { self . version } }
    };
}

impl_81!();