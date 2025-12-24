use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The error type for fallible [`Vec`] and [`String`] methods.
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct CapacityError;
