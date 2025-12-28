macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! UNKNOWN {
    () => {
        deps!();
        # [doc = " The default Id to use if there is no need for an id."] # [doc = ""] # [doc = " This is the default unless applications wish to make themselves more introspectable."] pub const UNKNOWN : Id = * b"\0\0\0\0" ;
    };
}

UNKNOWN!();