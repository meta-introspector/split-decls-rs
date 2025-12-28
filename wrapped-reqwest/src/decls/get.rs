macro_rules! deps {
    () => {
        IntoUrl!();
        Result!();
    };
}

macro_rules! get {
    () => {
        deps!();
        # [doc = " Shortcut method to quickly make a `GET` request."] # [doc = ""] # [doc = " See also the methods on the [`reqwest::Response`](./struct.Response.html)"] # [doc = " type."] # [doc = ""] # [doc = " **NOTE**: This function creates a new internal `Client` on each call,"] # [doc = " and so should not be used if making many requests. Create a"] # [doc = " [`Client`](./struct.Client.html) instead."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # async fn run() -> Result<(), reqwest::Error> {"] # [doc = " let body = reqwest::get(\"https://www.rust-lang.org\").await?"] # [doc = "     .text().await?;"] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function fails if:"] # [doc = ""] # [doc = " - native TLS backend cannot be initialized"] # [doc = " - supplied `Url` cannot be parsed"] # [doc = " - there was an error while sending request"] # [doc = " - redirect limit was exhausted"] pub async fn get < T : IntoUrl > (url : T) -> crate :: Result < Response > { Client :: builder () . build () ? . get (url) . send () . await }
    };
}

get!()