macro_rules! deps {
    () => {
        MessageTrailersStrs!();
        Error!();
    };
}

macro_rules! message_trailers_strs {
    () => {
        deps!();
        # [doc = " Get the trailers for the given message."] # [doc = ""] # [doc = " Use this function when you are dealing with a UTF-8-encoded message."] pub fn message_trailers_strs (message : & str) -> Result < MessageTrailersStrs , Error > { _message_trailers (message . into_c_string () ?) . map (| res | MessageTrailersStrs (res)) }
    };
}

message_trailers_strs!()