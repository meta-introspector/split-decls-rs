macro_rules! Capabilities {
    () => {
        # [doc = " A set of capabilities that have been negotiated between client and server."] pub type Capabilities = HashSet < String > ;
    };
}

Capabilities!();