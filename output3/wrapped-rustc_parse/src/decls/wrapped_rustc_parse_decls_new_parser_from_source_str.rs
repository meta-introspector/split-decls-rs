use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Creates a new parser from a source string.
///
/// On failure, the errors must be consumed via `unwrap_or_emit_fatal`, `emit`, `cancel`,
/// etc., otherwise a panic will occur when they are dropped.
pub fn new_parser_from_source_str(
    psess: &ParseSess,
    name: FileName,
    source: String,
    strip_tokens: StripTokens,
) -> Result<Parser<'_>, Vec<Diag<'_>>> {
    let source_file = psess.source_map().new_source_file(name, source);
    new_parser_from_source_file(psess, source_file, strip_tokens)
}
