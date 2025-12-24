use serde::{Deserialize, Serialize};
use std::collections::HashMap;
struct CurlTransport {
    handle: Arc<Mutex<Easy>>,
    /// The URL of the remote server, e.g. `https://github.com/user/repo`
    ///
    /// This is an empty string until the first action is performed.
    /// If there is an HTTP redirect, this will be updated with the new URL.
    base_url: Arc<Mutex<String>>,
}
