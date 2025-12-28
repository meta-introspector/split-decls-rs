macro_rules! deps {
    () => {
        Request!();
        Result!();
        ParseRequestError!();
    };
}

macro_rules! receive_json {
    () => {
        deps!();
        # [doc = " Receive a GraphQL request from a body as JSON."] pub async fn receive_json (body : impl AsyncRead + Unpin) -> Result < Request , ParseRequestError > { receive_batch_json (body) . await ? . into_single () }
    };
}

receive_json!()