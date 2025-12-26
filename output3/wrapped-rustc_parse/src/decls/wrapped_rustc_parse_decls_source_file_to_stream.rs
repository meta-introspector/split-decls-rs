use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Given a source file, produces a sequence of token trees.
///
/// Returns any buffered errors from parsing the token stream.
fn source_file_to_stream<'psess>(
    psess: &'psess ParseSess,
    source_file: Arc<SourceFile>,
    override_span: Option<Span>,
    strip_tokens: StripTokens,
) -> Result<TokenStream, Vec<Diag<'psess>>> {
    let src = source_file.src.as_ref().unwrap_or_else(|| {
        psess.dcx().bug(format!(
            "cannot lex `source_file` without source: {}",
            psess
                .source_map()
                .filename_for_diagnostics(&source_file.name)
        ));
    });
    lexer::lex_token_trees(
        psess,
        src.as_str(),
        source_file.start_pos,
        override_span,
        strip_tokens,
    )
}
