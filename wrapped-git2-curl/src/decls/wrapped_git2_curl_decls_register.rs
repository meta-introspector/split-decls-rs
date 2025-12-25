use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Register the libcurl backend for HTTP requests made by libgit2.
///
/// This function takes one parameter, a `handle`, which is used to perform all
/// future HTTP requests. The handle can be previously configured with
/// information such as proxies, SSL information, etc.
///
/// This function is unsafe largely for the same reasons as
/// `git2::transport::register`:
///
/// * The function needs to be synchronized against all other creations of
///   transport (any API calls to libgit2).
/// * The function will leak `handle` as once registered it is not currently
///   possible to unregister the backend.
///
/// This function may be called concurrently, but only the first `handle` will
/// be used. All others will be discarded.
pub unsafe fn register(handle: Easy) {
    static INIT: Once = Once::new();
    let handle = Arc::new(Mutex::new(handle));
    let handle2 = handle.clone();
    INIT.call_once(move || {
        git2::transport::register("http", move |remote| factory(remote, handle.clone())).unwrap();
        git2::transport::register("https", move |remote| factory(remote, handle2.clone())).unwrap();
    });
}
