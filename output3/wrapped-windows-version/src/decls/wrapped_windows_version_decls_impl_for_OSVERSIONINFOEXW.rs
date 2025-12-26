use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl OSVERSIONINFOEXW {
    fn new() -> Self {
        Self {
            dwOSVersionInfoSize: core::mem::size_of::<Self>() as u32,
            ..Default::default()
        }
    }
}
