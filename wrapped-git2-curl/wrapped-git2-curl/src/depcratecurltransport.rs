// Generated macro for CurlTransport (struct)
macro_rules! DepcrateCurlTransport {
() => {
// Module: crate
// Provides: {"CurlTransport"}
// Dependencies: {}
struct CurlTransport { handle : Arc < Mutex < Easy > > , # [doc = " The URL of the remote server, e.g. `https://github.com/user/repo`"] # [doc = ""] # [doc = " This is an empty string until the first action is performed."] # [doc = " If there is an HTTP redirect, this will be updated with the new URL."] base_url : Arc < Mutex < String > > , }
};
}
