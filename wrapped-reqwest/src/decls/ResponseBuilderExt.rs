macro_rules! ResponseBuilderExt {
    () => {
        # [doc = " Extension trait for http::response::Builder objects"] # [doc = ""] # [doc = " Allows the user to add a `Url` to the http::Response"] pub trait ResponseBuilderExt { # [doc = " A builder method for the `http::response::Builder` type that allows the user to add a `Url`"] # [doc = " to the `http::Response`"] fn url (self , url : Url) -> Self ; }
    };
}

ResponseBuilderExt!();