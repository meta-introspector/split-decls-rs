use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A list of miniz failed status codes.
///
/// These are emitted as the [`Err`] side of a [`MZResult`] in the [`StreamResult`] returned from
/// [`deflate::stream::deflate()`] or [`inflate::stream::inflate()`].
#[repr(i32)]
#[cfg_attr(not(feature = "rustc-dep-of-std"), derive(Hash, Debug))]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MZError {
    /// Unused
    ErrNo = -1,
    /// General stream error.
    ///
    /// See [`inflate::stream::inflate()`] docs for details of how it can occur there.
    ///
    /// See [`deflate::stream::deflate()`] docs for how it can in principle occur there, though it's
    /// believed impossible in practice.
    Stream = -2,
    /// Error in inflation; see [`inflate::stream::inflate()`] for details.
    ///
    /// Not returned from [`deflate::stream::deflate()`].
    Data = -3,
    /// Unused
    Mem = -4,
    /// Buffer-related error.
    ///
    /// See the docs of [`deflate::stream::deflate()`] or [`inflate::stream::inflate()`] for details
    /// of when it would trigger in the one you're using.
    Buf = -5,
    /// Unused
    Version = -6,
    /// Bad parameters.
    ///
    /// This can be returned from [`deflate::stream::deflate()`] in the case of bad parameters.  See
    /// [`TDEFLStatus::BadParam`][deflate::core::TDEFLStatus::BadParam].
    Param = -10_000,
}
