macro_rules! deps {
    () => {
        Error!();
        Kind!();
        BoxError!();
    };
}

macro_rules! redirect {
    () => {
        deps!();
        pub (crate) fn redirect < E : Into < BoxError > > (e : E , url : Url) -> Error { Error :: new (Kind :: Redirect , Some (e)) . with_url (url) }
    };
}

redirect!()