macro_rules! deps {
    () => {
        PostBodyDataKind!();
        Options!();
    };
}

macro_rules! Request {
    () => {
        deps!();
        pub (crate) struct Request { pub url : String , pub base_url : String , pub headers : reqwest :: header :: HeaderMap , pub upload_body_kind : Option < PostBodyDataKind > , pub config : http :: Options , }
    };
}

Request!()