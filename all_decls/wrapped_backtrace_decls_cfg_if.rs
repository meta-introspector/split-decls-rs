use serde::{Deserialize, Serialize};
use std::collections::HashMap;
cfg_if::cfg_if! {
    if #[cfg(all(target_env = "sgx", target_vendor = "fortanix", not(feature = "std")))]
    { pub use self::backtrace::set_image_base; }
}
