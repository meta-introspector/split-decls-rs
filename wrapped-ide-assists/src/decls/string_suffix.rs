macro_rules! string_suffix {
    () => {
        # [doc = " Calculate the string literal suffix length"] pub (crate) fn string_suffix (s : & str) -> Option < & str > { s . rfind (['"' , '\'' , '#']) . map (| i | & s [i + 1 ..]) }
    };
}

string_suffix!();