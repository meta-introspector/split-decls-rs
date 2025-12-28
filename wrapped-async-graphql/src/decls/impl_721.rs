macro_rules! deps {
    () => {
        Result!();
        OpaqueCursor!();
        Error!();
        CursorType!();
    };
}

macro_rules! impl_721 {
    () => {
        deps!();
        impl < T > CursorType for OpaqueCursor < T > where T : Serialize + DeserializeOwned , { type Error = Box < dyn std :: error :: Error + Send + Sync > ; fn decode_cursor (s : & str) -> Result < Self , Self :: Error > { use base64 :: Engine ; let data = base64 :: engine :: general_purpose :: URL_SAFE_NO_PAD . decode (s) ? ; Ok (Self (serde_json :: from_slice (& data) ?)) } fn encode_cursor (& self) -> String { use base64 :: Engine ; let value = serde_json :: to_vec (& self . 0) . unwrap_or_default () ; base64 :: engine :: general_purpose :: URL_SAFE_NO_PAD . encode (value) } }
    };
}

impl_721!()