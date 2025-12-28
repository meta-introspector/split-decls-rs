macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! stag {
    () => {
        deps!();
        # [doc = " Parsed a spaced tag."] pub fn stag (s : & str) -> impl Parser < '_ , & str > { spaced (tag (s)) }
    };
}

stag!();