macro_rules! wrap {
    () => {
        # [cfg (not (feature = "wrap_help"))] pub (crate) fn wrap (content : & str , _hard_width : usize) -> String { content . to_owned () }
    };
}

wrap!()