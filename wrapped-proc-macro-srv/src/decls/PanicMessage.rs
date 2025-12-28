macro_rules! PanicMessage {
    () => {
        # [derive (Debug , Clone)] pub struct PanicMessage { message : Option < String > , }
    };
}

PanicMessage!()