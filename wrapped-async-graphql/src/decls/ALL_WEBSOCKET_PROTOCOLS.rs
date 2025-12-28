macro_rules! ALL_WEBSOCKET_PROTOCOLS {
    () => {
        # [doc = " All known protocols based on WebSocket."] pub const ALL_WEBSOCKET_PROTOCOLS : [& str ; 2] = ["graphql-transport-ws" , "graphql-ws"] ;
    };
}

ALL_WEBSOCKET_PROTOCOLS!()