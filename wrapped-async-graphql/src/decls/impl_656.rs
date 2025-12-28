macro_rules! deps {
    () => {
        Executor!();
        MessageMapStream!();
        Protocols!();
        DefaultOnPingType!();
        DefaultOnConnInitType!();
        ClientMessage!();
        Result!();
    };
}

macro_rules! impl_656 {
    () => {
        deps!();
        impl < S , E > WebSocket < MessageMapStream < S > , E , DefaultOnConnInitType , DefaultOnPingType > where E : Executor , S : Stream , S :: Item : AsRef < [u8] > , { # [doc = " Create a new websocket from bytes stream."] pub fn new (executor : E , stream : S , protocol : Protocols) -> Self { let stream = stream . map (ClientMessage :: from_bytes as fn (S :: Item) -> serde_json :: Result < ClientMessage >) ; WebSocket :: from_message_stream (executor , stream , protocol) } }
    };
}

impl_656!()