// Generated macro for MultipartOptions (struct)
macro_rules! Depcrate_http_multipartMultipartOptions {
() => {
// Module: crate::http::multipart
// Provides: {"MultipartOptions"}
// Dependencies: {}
# [doc = " Options for `receive_multipart`."] # [derive (Default , Clone , Copy)] # [non_exhaustive] pub struct MultipartOptions { # [doc = " The maximum file size."] pub max_file_size : Option < usize > , # [doc = " The maximum number of files."] pub max_num_files : Option < usize > , }
};
}
