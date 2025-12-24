use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(any(test, feature = "std"))]
/// `Either<L, R>` implements `Write` if both `L` and `R` do.
///
/// Requires crate feature `"std"`
impl<L, R> Write for Either<L, R>
where
    L: Write,
    R: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for_both!(self, inner => inner.write(buf))
    }
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        for_both!(self, inner => inner.write_all(buf))
    }
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        for_both!(self, inner => inner.write_fmt(fmt))
    }
    fn flush(&mut self) -> io::Result<()> {
        for_both!(self, inner => inner.flush())
    }
}
