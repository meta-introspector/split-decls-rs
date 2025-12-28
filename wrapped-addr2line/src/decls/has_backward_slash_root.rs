macro_rules! has_backward_slash_root {
    () => {
        # [doc = " Check if the path in the given string has a windows style root"] fn has_backward_slash_root (p : & str) -> bool { p . starts_with ('\\') || p . get (1 .. 3) == Some (":\\") }
    };
}

has_backward_slash_root!();