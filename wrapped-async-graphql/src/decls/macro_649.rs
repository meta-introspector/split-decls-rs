macro_rules! deps {
    () => {
        Protocols!();
        Response!();
        Timer!();
        Data!();
        Result!();
    };
}

macro_rules! macro_649 {
    () => {
        deps!();
        pin_project ! { # [doc = " A GraphQL connection over websocket."] # [doc = ""] # [doc = " # References"] # [doc = ""] # [doc = " - [subscriptions-transport-ws](https://github.com/apollographql/subscriptions-transport-ws/blob/master/PROTOCOL.md)"] # [doc = " - [graphql-ws](https://github.com/enisdenjo/graphql-ws/blob/master/PROTOCOL.md)"] pub struct WebSocket < S , E , OnInit , OnPing > { on_connection_init : Option < OnInit >, on_ping : OnPing , init_fut : Option < BoxFuture <'static , Result < Data >>>, ping_fut : Option < BoxFuture <'static , Result < Option < serde_json :: Value >>>>, connection_data : Option < Data >, data : Option < Arc < Data >>, executor : E , streams : HashMap < String , Pin < Box < dyn Stream < Item = Response > + Send >>>, # [pin] stream : S , protocol : Protocols , last_msg_at : Instant , keepalive_timer : Option < Timer >, close : bool , } }
    };
}

macro_649!()