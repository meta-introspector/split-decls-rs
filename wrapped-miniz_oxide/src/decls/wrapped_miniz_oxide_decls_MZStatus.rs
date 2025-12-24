use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A list of miniz successful status codes.
///
/// These are emitted as the [`Ok`] side of a [`MZResult`] in the [`StreamResult`] returned from
/// [`deflate::stream::deflate()`] or [`inflate::stream::inflate()`].
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(not(feature = "rustc-dep-of-std"), derive(Hash, Debug))]
pub enum MZStatus {
    /// Operation succeeded.
    ///
    /// Some data was decompressed or compressed; see the byte counters in the [`StreamResult`] for
    /// details.
    Ok = 0,
    /// Operation succeeded and end of deflate stream was found.
    ///
    /// X-ref [`TINFLStatus::Done`][inflate::TINFLStatus::Done] or
    /// [`TDEFLStatus::Done`][deflate::core::TDEFLStatus::Done] for `inflate` or `deflate`
    /// respectively.
    StreamEnd = 1,
    /// Unused
    NeedDict = 2,
}
