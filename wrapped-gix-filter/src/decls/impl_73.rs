macro_rules! deps {
    () => {
        Client!();
        Capabilities!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        # [doc = " Access"] impl Client { # [doc = " Return the list of capabilities reported by the serving process."] pub fn capabilities (& self) -> & Capabilities { & self . capabilities } # [doc = " Return the mutable list of capabilities reported by the serving process."] pub fn capabilities_mut (& mut self) -> & mut Capabilities { & mut self . capabilities } # [doc = " Return the negotiated version of the protocol."] # [doc = ""] # [doc = " Note that it is the highest one that both the client and the server support."] pub fn version (& self) -> usize { self . version } }
    };
}

impl_73!()