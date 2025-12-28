macro_rules! is_self {
    () => {
        fn is_self (path : & ast :: Path) -> bool { path . segment () . map (| segment | segment . self_token () . is_some ()) . unwrap_or (false) }
    };
}

is_self!()