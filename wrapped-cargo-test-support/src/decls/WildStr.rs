macro_rules! WildStr {
    () => {
        # [doc = " A single line string that supports `[..]` wildcard matching."] struct WildStr < 'a > { has_meta : bool , line : & 'a str , }
    };
}

WildStr!()