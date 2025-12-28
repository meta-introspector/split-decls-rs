macro_rules! TransportMessage {
    () => {
        # [doc = " Callback for receiving messages delivered by the transport."] # [doc = ""] # [doc = " The return value indicates whether the network operation should continue."] pub type TransportMessage < 'a > = dyn FnMut (& [u8]) -> bool + 'a ;
    };
}

TransportMessage!();