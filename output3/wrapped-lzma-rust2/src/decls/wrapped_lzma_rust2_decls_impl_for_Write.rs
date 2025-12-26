use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T: AutoFinish + Write> Write for AutoFinisher<T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        use core::ops::DerefMut;
        self.deref_mut().write(buf)
    }
    fn flush(&mut self) -> Result<()> {
        use core::ops::DerefMut;
        self.deref_mut().flush()
    }
}
