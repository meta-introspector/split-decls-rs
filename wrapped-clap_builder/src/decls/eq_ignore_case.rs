macro_rules! eq_ignore_case {
    () => {
        # [cfg (not (feature = "unicode"))] pub (crate) fn eq_ignore_case (left : & str , right : & str) -> bool { left . eq_ignore_ascii_case (right) }
    };
}

eq_ignore_case!()