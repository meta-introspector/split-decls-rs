macro_rules! deps {
    () => {
        Protocols!();
        Result!();
        DefaultOnConnInitType!();
        Executor!();
        DefaultOnPingType!();
        ClientMessage!();
    };
}

macro_rules! impl_655 {
    () => {
        deps!();
        impl < S , E > WebSocket < S , E , DefaultOnConnInitType , DefaultOnPingType > where E : Executor , S : Stream < Item = serde_json :: Result < ClientMessage > > , { # [doc = " Create a new websocket from [`ClientMessage`] stream."] pub fn from_message_stream (executor : E , stream : S , protocol : Protocols) -> Self { WebSocket { on_connection_init : Some (default_on_connection_init) , on_ping : default_on_ping , init_fut : None , ping_fut : None , connection_data : None , data : None , executor , streams : HashMap :: new () , stream , protocol , last_msg_at : Instant :: now () , keepalive_timer : None , close : false , } } }
    };
}

impl_655!();