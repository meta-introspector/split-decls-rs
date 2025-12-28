macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! CursorType {
    () => {
        deps!();
        # [doc = " Cursor type"] # [doc = ""] # [doc = " A custom scalar that serializes as a string."] # [doc = " <https://relay.dev/graphql/connections.htm#sec-Cursor>"] pub trait CursorType : Sized { # [doc = " Error type for `decode_cursor`."] type Error : Display ; # [doc = " Decode cursor from string."] fn decode_cursor (s : & str) -> Result < Self , Self :: Error > ; # [doc = " Encode cursor to string."] fn encode_cursor (& self) -> String ; }
    };
}

CursorType!();