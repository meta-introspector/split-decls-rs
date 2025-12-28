macro_rules! deps {
    () => {
        Result!();
        Error!();
        MultipartOptions!();
        BatchRequest!();
        ParseRequestError!();
    };
}

macro_rules! receive_batch_body {
    () => {
        deps!();
        # [doc = " Receive a GraphQL request from a content type and body."] pub async fn receive_batch_body (content_type : Option < impl AsRef < str > > , body : impl AsyncRead + Send , opts : MultipartOptions ,) -> Result < BatchRequest , ParseRequestError > { let content_type = content_type . as_ref () . map (AsRef :: as_ref) . unwrap_or ("application/graphql-response+json") ; let content_type : mime :: Mime = content_type . parse () ? ; match (content_type . type_ () , content_type . subtype ()) { (mime :: MULTIPART , _) => { if let Some (boundary) = content_type . get_param ("boundary") { multipart :: receive_batch_multipart (body , boundary . to_string () , opts) . await } else { Err (ParseRequestError :: InvalidMultipart (multer :: Error :: NoBoundary ,)) } } _ => receive_batch_body_no_multipart (& content_type , body) . await , } }
    };
}

receive_batch_body!()