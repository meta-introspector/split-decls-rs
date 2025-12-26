use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<P: AsRef<Utf8Path>> Extend<P> for Utf8PathBuf {
    fn extend<I: IntoIterator<Item = P>>(&mut self, iter: I) {
        for path in iter {
            self.push(path);
        }
    }
}
