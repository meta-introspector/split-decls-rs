use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl LineIndex {
    /// Returns a `LineIndex` for the `text`.
    pub fn new(text: &str) -> LineIndex {
        let (newlines, line_wide_chars) = analyze_source_file(text);
        LineIndex {
            newlines: newlines.into_boxed_slice(),
            line_wide_chars,
            len: TextSize::of(text),
        }
    }
    /// Transforms the `TextSize` into a `LineCol`.
    ///
    /// # Panics
    ///
    /// If the offset is invalid. See [`Self::try_line_col`].
    pub fn line_col(&self, offset: TextSize) -> LineCol {
        self.try_line_col(offset).expect("invalid offset")
    }
    /// Transforms the `TextSize` into a `LineCol`.
    ///
    /// Returns `None` if the `offset` was invalid, e.g. if it extends past the end of the text or
    /// points to the middle of a multi-byte character.
    pub fn try_line_col(&self, offset: TextSize) -> Option<LineCol> {
        if offset > self.len {
            return None;
        }
        let line = self.newlines.partition_point(|&it| it <= offset);
        let start = self.start_offset(line)?;
        let col = offset - start;
        let ret = LineCol {
            line: line as u32,
            col: col.into(),
        };
        self.line_wide_chars
            .get(&ret.line)
            .into_iter()
            .flat_map(|it| it.iter())
            .all(|it| col <= it.start || it.end <= col)
            .then_some(ret)
    }
    /// Transforms the `LineCol` into a `TextSize`.
    pub fn offset(&self, line_col: LineCol) -> Option<TextSize> {
        self.start_offset(line_col.line as usize)
            .map(|start| start + TextSize::from(line_col.col))
    }
    fn start_offset(&self, line: usize) -> Option<TextSize> {
        match line.checked_sub(1) {
            None => Some(TextSize::from(0)),
            Some(it) => self.newlines.get(it).copied(),
        }
    }
    /// Transforms the `LineCol` with the given `WideEncoding` into a `WideLineCol`.
    pub fn to_wide(&self, enc: WideEncoding, line_col: LineCol) -> Option<WideLineCol> {
        let mut col = line_col.col;
        if let Some(wide_chars) = self.line_wide_chars.get(&line_col.line) {
            for c in wide_chars.iter() {
                if u32::from(c.end) <= line_col.col {
                    col = col.checked_sub(u32::from(c.len()) - c.wide_len(enc))?;
                } else {
                    break;
                }
            }
        }
        Some(WideLineCol {
            line: line_col.line,
            col,
        })
    }
    /// Transforms the `WideLineCol` with the given `WideEncoding` into a `LineCol`.
    pub fn to_utf8(&self, enc: WideEncoding, line_col: WideLineCol) -> Option<LineCol> {
        let mut col = line_col.col;
        if let Some(wide_chars) = self.line_wide_chars.get(&line_col.line) {
            for c in wide_chars.iter() {
                if col > u32::from(c.start) {
                    col = col.checked_add(u32::from(c.len()) - c.wide_len(enc))?;
                } else {
                    break;
                }
            }
        }
        Some(LineCol {
            line: line_col.line,
            col,
        })
    }
    /// Returns the given line's range.
    pub fn line(&self, line: u32) -> Option<TextRange> {
        let start = self.start_offset(line as usize)?;
        let next_newline = self
            .newlines
            .get(line as usize)
            .copied()
            .unwrap_or(self.len);
        let line_length = next_newline - start;
        Some(TextRange::new(start, start + line_length))
    }
    /// Given a range [start, end), returns a sorted iterator of non-empty ranges [start, x1), [x1,
    /// x2), ..., [xn, end) where all the xi, which are positions of newlines, are inside the range
    /// [start, end).
    pub fn lines(&self, range: TextRange) -> impl Iterator<Item = TextRange> + '_ {
        let lo = self.newlines.partition_point(|&it| it < range.start());
        let hi = self.newlines.partition_point(|&it| it <= range.end());
        let all = std::iter::once(range.start())
            .chain(self.newlines[lo..hi].iter().copied())
            .chain(std::iter::once(range.end()));
        all.clone()
            .zip(all.skip(1))
            .map(|(lo, hi)| TextRange::new(lo, hi))
            .filter(|it| !it.is_empty())
    }
    /// Returns the length of the original text.
    pub fn len(&self) -> TextSize {
        self.len
    }
}
