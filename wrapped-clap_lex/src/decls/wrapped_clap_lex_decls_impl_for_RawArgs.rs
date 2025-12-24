use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl RawArgs {
    ///
    /// <div class="warning">
    ///
    /// **NOTE:** The argument returned will be the current binary.
    ///
    /// </div>
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use std::path::PathBuf;
    /// let raw = clap_lex::RawArgs::from_args();
    /// let mut cursor = raw.cursor();
    /// let _bin = raw.next_os(&mut cursor);
    ///
    /// let mut paths = raw.remaining(&mut cursor).map(PathBuf::from).collect::<Vec<_>>();
    /// println!("{paths:?}");
    /// ```
    pub fn from_args() -> Self {
        Self::new(std::env::args_os())
    }
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use std::path::PathBuf;
    /// let raw = clap_lex::RawArgs::new(["bin", "foo.txt"]);
    /// let mut cursor = raw.cursor();
    /// let _bin = raw.next_os(&mut cursor);
    ///
    /// let mut paths = raw.remaining(&mut cursor).map(PathBuf::from).collect::<Vec<_>>();
    /// println!("{paths:?}");
    /// ```
    pub fn new(iter: impl IntoIterator<Item = impl Into<OsString>>) -> Self {
        let iter = iter.into_iter();
        Self::from(iter)
    }
    /// Create a cursor for walking the arguments
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use std::path::PathBuf;
    /// let raw = clap_lex::RawArgs::new(["bin", "foo.txt"]);
    /// let mut cursor = raw.cursor();
    /// let _bin = raw.next_os(&mut cursor);
    ///
    /// let mut paths = raw.remaining(&mut cursor).map(PathBuf::from).collect::<Vec<_>>();
    /// println!("{paths:?}");
    /// ```
    pub fn cursor(&self) -> ArgCursor {
        ArgCursor::new()
    }
    /// Advance the cursor, returning the next [`ParsedArg`]
    pub fn next(&self, cursor: &mut ArgCursor) -> Option<ParsedArg<'_>> {
        self.next_os(cursor).map(ParsedArg::new)
    }
    /// Advance the cursor, returning a raw argument value.
    pub fn next_os(&self, cursor: &mut ArgCursor) -> Option<&OsStr> {
        let next = self.items.get(cursor.cursor).map(|s| s.as_os_str());
        cursor.cursor = cursor.cursor.saturating_add(1);
        next
    }
    /// Return the next [`ParsedArg`]
    pub fn peek(&self, cursor: &ArgCursor) -> Option<ParsedArg<'_>> {
        self.peek_os(cursor).map(ParsedArg::new)
    }
    /// Return a raw argument value.
    pub fn peek_os(&self, cursor: &ArgCursor) -> Option<&OsStr> {
        self.items.get(cursor.cursor).map(|s| s.as_os_str())
    }
    /// Return all remaining raw arguments, advancing the cursor to the end
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use std::path::PathBuf;
    /// let raw = clap_lex::RawArgs::new(["bin", "foo.txt"]);
    /// let mut cursor = raw.cursor();
    /// let _bin = raw.next_os(&mut cursor);
    ///
    /// let mut paths = raw.remaining(&mut cursor).map(PathBuf::from).collect::<Vec<_>>();
    /// println!("{paths:?}");
    /// ```
    pub fn remaining(&self, cursor: &mut ArgCursor) -> impl Iterator<Item = &OsStr> {
        let remaining = self.items[cursor.cursor..].iter().map(|s| s.as_os_str());
        cursor.cursor = self.items.len();
        remaining
    }
    /// Adjust the cursor's position
    pub fn seek(&self, cursor: &mut ArgCursor, pos: SeekFrom) {
        let pos = match pos {
            SeekFrom::Start(pos) => pos,
            SeekFrom::End(pos) => {
                (self.items.len() as i64).saturating_add(pos).max(0) as u64
            }
            SeekFrom::Current(pos) => {
                (cursor.cursor as i64).saturating_add(pos).max(0) as u64
            }
        };
        let pos = (pos as usize).min(self.items.len());
        cursor.cursor = pos;
    }
    /// Inject arguments before the [`RawArgs::next`]
    pub fn insert(
        &mut self,
        cursor: &ArgCursor,
        insert_items: impl IntoIterator<Item = impl Into<OsString>>,
    ) {
        self.items
            .splice(
                cursor.cursor..cursor.cursor,
                insert_items.into_iter().map(Into::into),
            );
    }
    /// Any remaining args?
    pub fn is_end(&self, cursor: &ArgCursor) -> bool {
        self.peek_os(cursor).is_none()
    }
}
