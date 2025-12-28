macro_rules! GetResponse {
    () => {
        # [doc = " The return value of [`Http::get()`]."] pub struct GetResponse < H , B > { # [doc = " The response headers."] pub headers : H , # [doc = " The response body."] pub body : B , }
    };
}

GetResponse!();