macro_rules! has_forward_slash_root {
    () => {
        # [doc = " Check if the path in the given string has a unix style root"] fn has_forward_slash_root (p : & str) -> bool { p . starts_with ('/') || p . get (1 .. 3) == Some (":/") }
    };
}

has_forward_slash_root!()