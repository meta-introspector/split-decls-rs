macro_rules! normalize_expected {
    () => {
        # [doc = " Normalizes the expected string so that it can be compared against the actual output."] fn normalize_expected (content : & str , redactions : & snapbox :: Redactions) -> String { use snapbox :: filter :: Filter as _ ; let content = snapbox :: filter :: FilterPaths . filter (content . into_data ()) ; let content = snapbox :: filter :: FilterNewlines . filter (content) ; let content = content . render () . expect ("came in as a String") ; let content = redactions . clear_unused (& content) ; content . into_owned () }
    };
}

normalize_expected!();