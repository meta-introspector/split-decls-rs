use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Read for CurlSubtransport {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.reader.is_none() {
            self.execute(&[])?;
        }
        self.reader.as_mut().unwrap().read(buf)
    }
}
