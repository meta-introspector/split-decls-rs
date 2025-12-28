macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! parse_ignore_errors {
    () => {
        deps!();
        # [doc = " Similar to [parse()], but will skip all lines that didn't parse correctly, silently squelching all errors."] pub fn parse_ignore_errors (buf : & [u8]) -> impl Iterator < Item = Entry < '_ > > { parse (buf) . filter_map (Result :: ok) }
    };
}

parse_ignore_errors!();