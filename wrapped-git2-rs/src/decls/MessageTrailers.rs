macro_rules! MessageTrailers {
    () => {
        struct MessageTrailers { raw : raw :: git_message_trailer_array , }
    };
}

MessageTrailers!();