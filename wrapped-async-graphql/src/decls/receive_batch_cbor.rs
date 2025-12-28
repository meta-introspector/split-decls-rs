macro_rules! deps {
    () => {
        ParseRequestError!();
        BatchRequest!();
        Result!();
    };
}

macro_rules! receive_batch_cbor {
    () => {
        deps!();
        # [doc = " Receive a GraphQL batch request from a body as CBOR"] # [cfg (feature = "cbor")] # [cfg_attr (docsrs , doc (cfg (feature = "cbor")))] pub async fn receive_batch_cbor (body : impl AsyncRead + Unpin) -> Result < BatchRequest , ParseRequestError > { let mut data = Vec :: new () ; futures_util :: pin_mut ! (body) ; body . read_to_end (& mut data) . await . map_err (ParseRequestError :: Io) ? ; serde_cbor :: from_slice :: < BatchRequest > (& data) . map_err (| e | ParseRequestError :: InvalidRequest (Box :: new (e))) }
    };
}

receive_batch_cbor!();