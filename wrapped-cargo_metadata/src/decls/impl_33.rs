macro_rules! deps {
    () => {
        MessageIter!();
        Message!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl Message { # [doc = " Creates an iterator of Message from a Read outputting a stream of JSON"] # [doc = " messages. For usage information, look at the top-level documentation."] pub fn parse_stream < R : Read > (input : R) -> MessageIter < R > { MessageIter { input } } }
    };
}

impl_33!();