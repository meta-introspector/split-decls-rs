macro_rules! is_ascii {
    () => {
        # [doc = " Checks whether the buffer is all-ASCII."] # [doc = ""] # [doc = " May read the entire buffer even if it isn't all-ASCII. (I.e. the function"] # [doc = " is not guaranteed to fail fast.)"] pub fn is_ascii (buffer : & [u8]) -> bool { is_ascii_impl (buffer) }
    };
}

is_ascii!()