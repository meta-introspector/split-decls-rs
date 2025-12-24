use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Nonce {
    #[inline]
    pub fn new() -> Nonce {
        Nonce(NEXT_NONCE.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
    }
}
