macro_rules! SEPARATOR_BYTE {
    () => {
        # [doc = " Event IDs are strings conforming to the following grammar:"] # [doc = ""] # [doc = " ```ignore"] # [doc = "   <event_id> = <label> {<argument>}"] # [doc = "   <label> = <text>"] # [doc = "   <argument> = '\\x1E' <text>"] # [doc = "   <text> = regex([[[:^cntrl:]][[:space:]]]+) // Anything but ASCII control characters except for whitespace."] # [doc = "  ```"] # [doc = ""] # [doc = " This means there's always a \"label\", followed by an optional list of"] # [doc = " arguments. Future versions may support other optional suffixes (with a tag"] # [doc = " other than '\\x11' after the '\\x1E' separator), such as a \"category\"."] # [doc = " The byte used to separate arguments from the label and each other."] pub const SEPARATOR_BYTE : & str = "\x1E" ;
    };
}

SEPARATOR_BYTE!()