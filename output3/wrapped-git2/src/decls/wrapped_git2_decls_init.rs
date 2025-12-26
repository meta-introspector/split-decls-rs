use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn init() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        openssl_env_init();
    });
    raw::init();
}
