use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl WideChar {
    /// Returns the length in 8-bit UTF-8 code units.
    fn len(&self) -> TextSize {
        self.end - self.start
    }
    /// Returns the length in UTF-16 or UTF-32 code units.
    fn wide_len(&self, enc: WideEncoding) -> u32 {
        match enc {
            WideEncoding::Utf16 => if self.len() == TextSize::from(4) { 2 } else { 1 }
            WideEncoding::Utf32 => 1,
        }
    }
}
