macro_rules! deps {
    () => {
        Error!();
        Kind!();
    };
}

macro_rules! status_code {
    () => {
        deps!();
        pub (crate) fn status_code (url : Url , status : StatusCode , # [cfg (not (target_arch = "wasm32"))] reason : Option < hyper :: ext :: ReasonPhrase > ,) -> Error { Error :: new (Kind :: Status (status , # [cfg (not (target_arch = "wasm32"))] reason ,) , None :: < Error > ,) . with_url (url) }
    };
}

status_code!()