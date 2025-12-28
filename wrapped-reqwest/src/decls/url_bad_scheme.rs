macro_rules! deps {
    () => {
        BadScheme!();
        Error!();
        Kind!();
    };
}

macro_rules! url_bad_scheme {
    () => {
        deps!();
        pub (crate) fn url_bad_scheme (url : Url) -> Error { Error :: new (Kind :: Builder , Some (BadScheme)) . with_url (url) }
    };
}

url_bad_scheme!();