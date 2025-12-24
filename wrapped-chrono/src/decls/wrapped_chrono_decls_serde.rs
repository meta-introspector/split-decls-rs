use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Serialization/Deserialization with serde
///
/// The [`DateTime`] type has default implementations for (de)serializing to/from the [RFC 3339]
/// format. This module provides alternatives for serializing to timestamps.
///
/// The alternatives are for use with serde's [`with` annotation] combined with the module name.
/// Alternatively the individual `serialize` and `deserialize` functions in each module can be used
/// with serde's [`serialize_with`] and [`deserialize_with`] annotations.
///
/// *Available on crate feature 'serde' only.*
///
/// [RFC 3339]: https://tools.ietf.org/html/rfc3339
/// [`with` annotation]: https://serde.rs/field-attrs.html#with
/// [`serialize_with`]: https://serde.rs/field-attrs.html#serialize_with
/// [`deserialize_with`]: https://serde.rs/field-attrs.html#deserialize_with
#[cfg(feature = "serde")]
pub mod serde {
    use core::fmt;
    use serde::de;
    pub use super::datetime::serde::*;
    /// Create a custom `de::Error` with `SerdeError::InvalidTimestamp`.
    pub(crate) fn invalid_ts<E, T>(value: T) -> E
    where
        E: de::Error,
        T: fmt::Display,
    {
        E::custom(SerdeError::InvalidTimestamp(value))
    }
    enum SerdeError<T: fmt::Display> {
        InvalidTimestamp(T),
    }
    impl<T: fmt::Display> fmt::Display for SerdeError<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                SerdeError::InvalidTimestamp(ts) => {
                    write!(f, "value is not a legal timestamp: {ts}")
                }
            }
        }
    }
}
