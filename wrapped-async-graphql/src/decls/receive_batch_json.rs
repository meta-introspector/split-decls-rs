macro_rules! deps {
    () => {
        BatchRequest!();
        ParseRequestError!();
        Result!();
    };
}

macro_rules! receive_batch_json {
    () => {
        deps!();
        # [doc = " Receive a GraphQL batch request from a body as JSON."] pub async fn receive_batch_json (body : impl AsyncRead + Unpin) -> Result < BatchRequest , ParseRequestError > { let mut data = Vec :: new () ; futures_util :: pin_mut ! (body) ; body . read_to_end (& mut data) . await . map_err (ParseRequestError :: Io) ? ; serde_json :: from_slice :: < BatchRequest > (& data) . map_err (| e | ParseRequestError :: InvalidRequest (Box :: new (e))) }
    };
}

receive_batch_json!()