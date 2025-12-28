macro_rules! deps {
    () => {
        Request!();
        ParseRequestError!();
        Result!();
        MultipartOptions!();
    };
}

macro_rules! receive_body {
    () => {
        deps!();
        # [doc = " Receive a GraphQL request from a content type and body."] pub async fn receive_body (content_type : Option < impl AsRef < str > > , body : impl AsyncRead + Send , opts : MultipartOptions ,) -> Result < Request , ParseRequestError > { receive_batch_body (content_type , body , opts) . await ? . into_single () }
    };
}

receive_body!()