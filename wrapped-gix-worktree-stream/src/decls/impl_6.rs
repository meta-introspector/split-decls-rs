macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl Entry < '_ > { # [doc = " Return the path of this entry as slash-separated path relative to the repository."] pub fn relative_path (& self) -> & BStr { self . path_buf . as_ref () . expect ("always set during our lifetime") . as_ref () } # [doc = " The amount of bytes that remain to be read, or `None` if it's fully streamed."] # [doc = ""] # [doc = " This equals the length of the entry in bytes right before reading it."] pub fn bytes_remaining (& self) -> Option < usize > { self . remaining } }
    };
}

impl_6!();