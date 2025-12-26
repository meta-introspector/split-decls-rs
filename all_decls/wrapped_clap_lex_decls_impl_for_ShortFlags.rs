use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'s> ShortFlags<'s> {
    fn new(inner: &'s OsStr) -> Self {
        let (utf8_prefix, invalid_suffix) = split_nonutf8_once(inner);
        let utf8_prefix = utf8_prefix.char_indices();
        Self {
            inner,
            utf8_prefix,
            invalid_suffix,
        }
    }
    /// Move the iterator forward by `n` short flags
    pub fn advance_by(&mut self, n: usize) -> Result<(), usize> {
        for i in 0..n {
            self.next().ok_or(i)?.map_err(|_| i)?;
        }
        Ok(())
    }
    /// No short flags left
    pub fn is_empty(&self) -> bool {
        self.invalid_suffix.is_none() && self.utf8_prefix.as_str().is_empty()
    }
    /// Does the short flag look like a number
    ///
    /// Ideally call this before doing any iterator
    pub fn is_negative_number(&self) -> bool {
        self.invalid_suffix.is_none() && is_number(self.utf8_prefix.as_str())
    }
    /// Advance the iterator, returning the next short flag on success
    ///
    /// On error, returns the invalid-UTF8 value
    pub fn next_flag(&mut self) -> Option<Result<char, &'s OsStr>> {
        if let Some((_, flag)) = self.utf8_prefix.next() {
            return Some(Ok(flag));
        }
        if let Some(suffix) = self.invalid_suffix {
            self.invalid_suffix = None;
            return Some(Err(suffix));
        }
        None
    }
    /// Advance the iterator, returning everything left as a value
    pub fn next_value_os(&mut self) -> Option<&'s OsStr> {
        if let Some((index, _)) = self.utf8_prefix.next() {
            self.utf8_prefix = "".char_indices();
            self.invalid_suffix = None;
            let remainder = unsafe { ext::split_at(self.inner, index).1 };
            return Some(remainder);
        }
        if let Some(suffix) = self.invalid_suffix {
            self.invalid_suffix = None;
            return Some(suffix);
        }
        None
    }
}
