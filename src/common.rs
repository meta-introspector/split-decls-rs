// common module - Common utilities and shared types

pub mod util {
    pub struct Util;
}

pub mod collections {
    pub use std::collections::*;
}

pub mod error {
    pub struct Error;
    pub type Result<T> = std::result::Result<T, Error>;
}

pub mod traits {
    pub trait Common {}
}

// Re-export commonly used items
pub use util::*;
pub use collections::*;
pub use error::*;
pub use traits::*;
