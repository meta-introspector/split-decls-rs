macro_rules! deps {
    () => {
        ParseRequestError!();
        Result!();
        BatchRequest!();
    };
}

macro_rules! receive_cbor {
    () => {
        deps!();
        # [doc = " Receive a GraphQL request from a body as CBOR."] # [cfg (feature = "cbor")] # [cfg_attr (docsrs , doc (cfg (feature = "cbor")))] pub async fn receive_cbor (body : impl AsyncRead + Unpin) -> Result < BatchRequest , ParseRequestError > { receive_batch_cbor (body) . await ? . into_single () }
    };
}

receive_cbor!()