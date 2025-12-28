macro_rules! Response {
    () => {
        # [doc = " A link to a thread who provides data for the contained readers."] # [doc = " The expected order is:"] # [doc = " - write `upload_body`"] # [doc = " - read `headers` to end"] # [doc = " - read `body` to hend"] pub (crate) struct Response { pub headers : pipe :: Reader , pub body : pipe :: Reader , pub upload_body : pipe :: Writer , }
    };
}

Response!()