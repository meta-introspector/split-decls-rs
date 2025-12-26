use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'s> ParsedArg<'s> {
    fn new(inner: &'s OsStr) -> Self {
        Self { inner }
    }
    /// Argument is length of 0
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// Does the argument look like a stdio argument (`-`)
    pub fn is_stdio(&self) -> bool {
        self.inner == "-"
    }
    /// Does the argument look like an argument escape (`--`)
    pub fn is_escape(&self) -> bool {
        self.inner == "--"
    }
    /// Does the argument look like a negative number?
    ///
    /// This won't parse the number in full but attempts to see if this looks
    /// like something along the lines of `-3`, `-0.3`, or `-33.03`
    pub fn is_negative_number(&self) -> bool {
        self.to_value()
            .ok()
            .and_then(|s| Some(is_number(s.strip_prefix('-')?)))
            .unwrap_or_default()
    }
    /// Treat as a long-flag
    pub fn to_long(&self) -> Option<(Result<&str, &OsStr>, Option<&OsStr>)> {
        let raw = self.inner;
        let remainder = raw.strip_prefix("--")?;
        if remainder.is_empty() {
            debug_assert!(self.is_escape());
            return None;
        }
        let (flag, value) = if let Some((p0, p1)) = remainder.split_once("=") {
            (p0, Some(p1))
        } else {
            (remainder, None)
        };
        let flag = flag.to_str().ok_or(flag);
        Some((flag, value))
    }
    /// Can treat as a long-flag
    pub fn is_long(&self) -> bool {
        self.inner.starts_with("--") && !self.is_escape()
    }
    /// Treat as a short-flag
    pub fn to_short(&self) -> Option<ShortFlags<'_>> {
        if let Some(remainder_os) = self.inner.strip_prefix("-") {
            if remainder_os.starts_with("-") {
                None
            } else if remainder_os.is_empty() {
                debug_assert!(self.is_stdio());
                None
            } else {
                Some(ShortFlags::new(remainder_os))
            }
        } else {
            None
        }
    }
    /// Can treat as a short-flag
    pub fn is_short(&self) -> bool {
        self.inner.starts_with("-") && !self.is_stdio() && !self.inner.starts_with("--")
    }
    /// Treat as a value
    ///
    /// <div class="warning">
    ///
    /// **NOTE:** May return a flag or an escape.
    ///
    /// </div>
    pub fn to_value_os(&self) -> &OsStr {
        self.inner
    }
    /// Treat as a value
    ///
    /// <div class="warning">
    ///
    /// **NOTE:** May return a flag or an escape.
    ///
    /// </div>
    pub fn to_value(&self) -> Result<&str, &OsStr> {
        self.inner.to_str().ok_or(self.inner)
    }
    /// Safely print an argument that may contain non-UTF8 content
    ///
    /// This may perform lossy conversion, depending on the platform. If you would like an implementation which escapes the path please use Debug instead.
    pub fn display(&self) -> impl std::fmt::Display + '_ {
        self.inner.to_string_lossy()
    }
}
