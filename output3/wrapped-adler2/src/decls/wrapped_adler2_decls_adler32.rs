use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Calculates the Adler-32 checksum of a `BufRead`'s contents.
///
/// The passed `BufRead` implementor will be read until it reaches EOF (or until it reports an
/// error).
///
/// If you only have a `Read` implementor, you can wrap it in `std::io::BufReader` before calling
/// this function.
///
/// # Errors
///
/// Any error returned by the reader are bubbled up by this function.
///
/// # Examples
///
/// ```no_run
/// # fn run() -> Result<(), Box<dyn std::error::Error>> {
/// use adler2::adler32;
///
/// use std::fs::File;
/// use std::io::BufReader;
///
/// let file = File::open("input.txt")?;
/// let mut file = BufReader::new(file);
///
/// adler32(&mut file)?;
/// # Ok(()) }
/// # fn main() { run().unwrap() }
/// ```
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub fn adler32<R: BufRead>(mut reader: R) -> io::Result<u32> {
    let mut h = Adler32::new();
    loop {
        let len = {
            let buf = reader.fill_buf()?;
            if buf.is_empty() {
                return Ok(h.checksum());
            }
            h.write_slice(buf);
            buf.len()
        };
        reader.consume(len);
    }
}
