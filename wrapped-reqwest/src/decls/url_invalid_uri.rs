macro_rules! deps {
    () => {
        Error!();
        Kind!();
    };
}

macro_rules! url_invalid_uri {
    () => {
        deps!();
        pub (crate) fn url_invalid_uri (url : Url) -> Error { Error :: new (Kind :: Builder , Some ("Parsed Url is not a valid Uri")) . with_url (url) }
    };
}

url_invalid_uri!()