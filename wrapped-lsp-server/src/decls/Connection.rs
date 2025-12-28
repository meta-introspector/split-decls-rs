macro_rules! deps {
    () => {
        Message!();
    };
}

macro_rules! Connection {
    () => {
        deps!();
        # [doc = " Connection is just a pair of channels of LSP messages."] pub struct Connection { pub sender : Sender < Message > , pub receiver : Receiver < Message > , }
    };
}

Connection!();