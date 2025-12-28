macro_rules! deps {
    () => {
        Error!();
        IntoCString!();
    };
}

macro_rules! message_prettify {
    () => {
        deps!();
        # [doc = " Clean up a message, removing extraneous whitespace, and ensure that the"] # [doc = " message ends with a newline. If `comment_char` is `Some`, also remove comment"] # [doc = " lines starting with that character."] pub fn message_prettify < T : IntoCString > (message : T , comment_char : Option < u8 > ,) -> Result < String , Error > { _message_prettify (message . into_c_string () ? , comment_char) }
    };
}

message_prettify!();