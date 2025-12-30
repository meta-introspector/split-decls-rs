// Generated macro for PostResponse (struct)
macro_rules! Depcrate_client_blocking_io_http_traitsPostResponse {
() => {
// Module: crate::client::blocking_io::http::traits
// Provides: {"PostResponse"}
// Dependencies: {}
# [doc = " The return value of [`Http::post()`]."] pub struct PostResponse < H , B , PB > { # [doc = " The body to post to the server as part of the request."] # [doc = ""] # [doc = " **Note**: Implementations should drop the handle to avoid deadlocks."] pub post_body : PB , # [doc = " The headers of the post response."] pub headers : H , # [doc = " The body of the post response."] pub body : B , }
};
}
