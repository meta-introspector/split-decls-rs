macro_rules! WsMessage {
    () => {
        # [doc = " An enum representing the various forms of a WebSocket message."] # [derive (Clone , Debug , PartialEq , Eq)] pub enum WsMessage { # [doc = " A text WebSocket message"] Text (String) , # [doc = " A close message with the close frame."] Close (u16 , String) , }
    };
}

WsMessage!();