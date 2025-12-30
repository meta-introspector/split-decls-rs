// Generated macro for ParseRequestError (enum)
macro_rules! Depcrate_errorParseRequestError {
() => {
// Module: crate::error
// Provides: {"ParseRequestError"}
// Dependencies: {}
# [doc = " An error parsing the request."] # [derive (Debug , Error)] # [non_exhaustive] pub enum ParseRequestError { # [doc = " An IO error occurred."] # [error ("{0}")] Io (# [from] std :: io :: Error) , # [doc = " The request's syntax was invalid."] # [error ("Invalid request: {0}")] InvalidRequest (Box < dyn std :: error :: Error + Send + Sync >) , # [doc = " The request's files map was invalid."] # [error ("Invalid files map: {0}")] InvalidFilesMap (Box < dyn std :: error :: Error + Send + Sync >) , # [doc = " The request's multipart data was invalid."] # [error ("Invalid multipart data")] InvalidMultipart (multer :: Error) , # [doc = " Missing \"operators\" part for multipart request."] # [error ("Missing \"operators\" part")] MissingOperatorsPart , # [doc = " Missing \"map\" part for multipart request."] # [error ("Missing \"map\" part")] MissingMapPart , # [doc = " It's not an upload operation"] # [error ("It's not an upload operation")] NotUpload , # [doc = " Files were missing the request."] # [error ("Missing files")] MissingFiles , # [doc = " The request's payload is too large, and this server rejected it."] # [error ("Payload too large")] PayloadTooLarge , # [doc = " The request is a batch request, but the server does not support batch"] # [doc = " requests."] # [error ("Batch requests are not supported")] UnsupportedBatch , }
};
}
