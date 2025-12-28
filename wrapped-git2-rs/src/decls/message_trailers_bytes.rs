macro_rules! deps {
    () => {
        IntoCString!();
        Error!();
        MessageTrailersBytes!();
    };
}

macro_rules! message_trailers_bytes {
    () => {
        deps!();
        # [doc = " Get the trailers for the given message."] # [doc = ""] # [doc = " Use this function when the message might not be UTF-8-encoded,"] # [doc = " or if you want to handle the returned trailer key–value pairs"] # [doc = " as bytes."] pub fn message_trailers_bytes < S : IntoCString > (message : S) -> Result < MessageTrailersBytes , Error > { _message_trailers (message . into_c_string () ?) . map (| res | MessageTrailersBytes (res)) }
    };
}

message_trailers_bytes!();