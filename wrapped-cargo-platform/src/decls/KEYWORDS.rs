macro_rules! KEYWORDS {
    () => {
        # [doc = " The list of keywords."] # [doc = ""] # [doc = " We should consider all the keywords, but some are conditional on"] # [doc = " the edition so for now we just consider true/false."] # [doc = ""] # [doc = " <https://doc.rust-lang.org/reference/keywords.html>"] pub (crate) const KEYWORDS : & [& str ; 2] = & ["true" , "false"] ;
    };
}

KEYWORDS!()