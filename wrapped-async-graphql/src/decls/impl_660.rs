macro_rules! deps {
    () => {
        Protocols!();
        Data!();
        Response!();
        ServerMessage!();
    };
}

macro_rules! impl_660 {
    () => {
        deps!();
        impl Protocols { # [doc = " Returns the `Sec-WebSocket-Protocol` header value for the protocol"] pub fn sec_websocket_protocol (& self) -> & 'static str { match self { Protocols :: SubscriptionsTransportWS => "graphql-ws" , Protocols :: GraphQLWS => "graphql-transport-ws" , } } # [inline] fn next_message < 's > (& self , id : & 's str , payload : Response) -> ServerMessage < 's > { match self { Protocols :: SubscriptionsTransportWS => ServerMessage :: Data { id , payload } , Protocols :: GraphQLWS => ServerMessage :: Next { id , payload } , } } }
    };
}

impl_660!()