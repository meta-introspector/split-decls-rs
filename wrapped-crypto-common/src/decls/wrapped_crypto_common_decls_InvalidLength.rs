use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The error type returned when key and/or IV used in the [`KeyInit`],
/// [`KeyIvInit`], and [`InnerIvInit`] slice-based methods had
/// an invalid length.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct InvalidLength;
