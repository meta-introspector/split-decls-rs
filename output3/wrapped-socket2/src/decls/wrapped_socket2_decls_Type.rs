use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Specification of communication semantics on a socket.
///
/// This is a newtype wrapper around an integer which provides a nicer API in
/// addition to an injection point for documentation. Convenience constants such
/// as [`Type::STREAM`], [`Type::DGRAM`], etc, are provided to avoid reaching
/// into libc for various constants.
///
/// This type is freely interconvertible with C's `int` type, however, if a raw
/// value needs to be provided.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Type(c_int);
