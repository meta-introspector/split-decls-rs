macro_rules! update_and_header_value {
    () => {
        # [cfg (feature = "http2")] pub (crate) fn update_and_header_value () -> HeaderValue { CACHED . with (| cache | { let mut cache = cache . borrow_mut () ; cache . check () ; cache . header_value . clone () }) }
    };
}

update_and_header_value!();