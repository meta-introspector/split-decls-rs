use serde::{Deserialize, Serialize};
use std::collections::HashMap;
if_wasm! {
    mod wasm; mod util; pub use self::wasm:: { Body, Client, ClientBuilder, Request,
    RequestBuilder, Response }; #[cfg(feature = "multipart")] pub use
    self::wasm::multipart;
}
