macro_rules! deps {
    () => {
        ProtocolError!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl ProtocolError { pub (crate) fn new (msg : impl Into < String >) -> Self { ProtocolError (msg . into () , false) } pub (crate) fn disconnected () -> ProtocolError { ProtocolError ("disconnected channel" . into () , true) } # [doc = " Whether this error occurred due to a disconnected channel."] pub fn channel_is_disconnected (& self) -> bool { self . 1 } }
    };
}

impl_1!()