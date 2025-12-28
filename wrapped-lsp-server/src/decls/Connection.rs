macro_rules! Connection {
    () => {
        # [doc = " Connection is just a pair of channels of LSP messages."] pub struct Connection { pub sender : Sender < Message > , pub receiver : Receiver < Message > , }
    };
}

Connection!()