use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Adapter to support [`std::io::Read`] over a [`TryRngCore`]
///
/// # Examples
///
/// ```no_run
/// use std::{io, io::Read};
/// use std::fs::File;
/// use rand::{rngs::OsRng, RngReader};
///
/// io::copy(
///     &mut RngReader(OsRng).take(100),
///     &mut File::create("/tmp/random.bytes").unwrap()
/// ).unwrap();
/// ```
#[cfg(feature = "std")]
pub struct RngReader<R: TryRngCore>(pub R);
