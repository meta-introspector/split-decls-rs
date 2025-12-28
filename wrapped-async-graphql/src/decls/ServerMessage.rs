macro_rules! deps {
    () => {
        Data!();
        Error!();
        Response!();
    };
}

macro_rules! ServerMessage {
    () => {
        deps!();
        # [derive (Serialize)] # [serde (tag = "type" , rename_all = "snake_case")] enum ServerMessage < 'a > { ConnectionError { payload : Error , } , ConnectionAck , # [doc = " subscriptions-transport-ws protocol next payload"] Data { id : & 'a str , payload : Response , } , # [doc = " graphql-ws protocol next payload"] Next { id : & 'a str , payload : Response , } , Complete { id : & 'a str , } , # [doc = " The response to the Ping message."] # [doc = ""] # [doc = " https://github.com/enisdenjo/graphql-ws/blob/master/PROTOCOL.md#pong"] Pong { # [serde (skip_serializing_if = "Option::is_none")] payload : Option < serde_json :: Value > , } , }
    };
}

ServerMessage!()