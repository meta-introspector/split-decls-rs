macro_rules! string_prefix {
    () => {
        # [doc = " Calculate the string literal prefix length"] pub (crate) fn string_prefix (s : & str) -> Option < & str > { s . split_once (['"' , '\'' , '#']) . map (| (prefix , _) | prefix) }
    };
}

string_prefix!();