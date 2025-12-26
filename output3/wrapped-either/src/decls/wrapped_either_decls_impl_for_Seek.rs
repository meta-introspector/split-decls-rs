use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(any(test, feature = "std"))]
/// `Either<L, R>` implements `Seek` if both `L` and `R` do.
///
/// Requires crate feature `"std"`
impl<L, R> Seek for Either<L, R>
where
    L: Seek,
    R: Seek,
{
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        for_both!(self, inner => inner.seek(pos))
    }
}
