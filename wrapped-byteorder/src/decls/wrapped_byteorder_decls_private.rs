use serde::{Deserialize, Serialize};
use std::collections::HashMap;
mod private {
    /// Sealed stops crates other than byteorder from implementing any traits
    /// that use it.
    pub trait Sealed {}
    impl Sealed for super::LittleEndian {}
    impl Sealed for super::BigEndian {}
}
