macro_rules! deps {
    () => {
        GitReference!();
    };
}

macro_rules! PrettyRef {
    () => {
        deps!();
        # [doc = " A git reference that can be `Display`ed"] pub struct PrettyRef < 'a > { inner : & 'a GitReference , url_encoded : bool , }
    };
}

PrettyRef!();