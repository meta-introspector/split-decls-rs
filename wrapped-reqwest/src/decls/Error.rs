macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " The Errors that may occur when processing a `Request`."] # [doc = ""] # [doc = " Note: Errors may include the full URL used to make the `Request`. If the URL"] # [doc = " contains sensitive information (e.g. an API key as a query parameter), be"] # [doc = " sure to remove it ([`without_url`](Error::without_url))"] pub struct Error { inner : Box < Inner > , }
    };
}

Error!()