macro_rules! strip_url_protocol {
    () => {
        fn strip_url_protocol (url : & Url) -> Url { let raw = url . to_string () ; raw . split_once ('+') . unwrap () . 1 . parse () . unwrap () }
    };
}

strip_url_protocol!();