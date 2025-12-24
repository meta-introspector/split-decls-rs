use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Creates an iterator that produces tokens from the input string.
///
/// When parsing a full Rust document,
/// first [`strip_shebang`] and then allow frontmatters with [`FrontmatterAllowed::Yes`].
///
/// When tokenizing a slice of a document, be sure to disallow frontmatters with [`FrontmatterAllowed::No`]
pub fn tokenize(
    input: &str,
    frontmatter_allowed: FrontmatterAllowed,
) -> impl Iterator<Item = Token> {
    let mut cursor = Cursor::new(input, frontmatter_allowed);
    std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if token.kind != TokenKind::Eof { Some(token) } else { None }
    })
}
