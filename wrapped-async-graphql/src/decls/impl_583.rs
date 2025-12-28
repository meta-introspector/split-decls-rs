macro_rules! deps {
    () => {
        ServerResult!();
        ExtensionContext!();
        NextPrepareRequest!();
        Request!();
    };
}

macro_rules! impl_583 {
    () => {
        deps!();
        impl NextPrepareRequest < '_ > { # [doc = " Call the [Extension::prepare_request] function of next extension."] pub async fn run (self , ctx : & ExtensionContext < '_ > , request : Request) -> ServerResult < Request > { if let Some ((first , next)) = self . chain . split_first () { first . prepare_request (ctx , request , NextPrepareRequest { chain : next }) . await } else { Ok (request) } } }
    };
}

impl_583!()