macro_rules! deps {
    () => {
        ParseRequestError!();
        Error!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl From < multer :: Error > for ParseRequestError { fn from (err : multer :: Error) -> Self { match err { multer :: Error :: FieldSizeExceeded { .. } | multer :: Error :: StreamSizeExceeded { .. } => { ParseRequestError :: PayloadTooLarge } _ => ParseRequestError :: InvalidMultipart (err) , } } }
    };
}

impl_50!();