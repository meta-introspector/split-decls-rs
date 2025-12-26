use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn convert_doc_comment<S: Copy>(
    token: &syntax::SyntaxToken,
    span: S,
    mode: DocCommentDesugarMode,
    builder: &mut tt::TopSubtreeBuilder<S>,
) {
    let Some(comment) = ast::Comment::cast(token.clone()) else {
        return;
    };
    let Some(doc) = comment.kind().doc else {
        return;
    };
    let mk_ident = |s: &str| {
        tt::Leaf::from(tt::Ident {
            sym: Symbol::intern(s),
            span,
            is_raw: tt::IdentIsRaw::No,
        })
    };
    let mk_punct = |c: char| {
        tt::Leaf::from(tt::Punct {
            char: c,
            spacing: tt::Spacing::Alone,
            span,
        })
    };
    let mk_doc_literal = |comment: &ast::Comment| {
        let prefix_len = comment.prefix().len();
        let mut text = &comment.text()[prefix_len..];
        if comment.kind().shape == ast::CommentShape::Block {
            text = &text[0..text.len() - 2];
        }
        let (text, kind) = desugar_doc_comment_text(text, mode);
        let lit = tt::Literal {
            symbol: text,
            span,
            kind,
            suffix: None,
        };
        tt::Leaf::from(lit)
    };
    let meta_tkns = [mk_ident("doc"), mk_punct('='), mk_doc_literal(&comment)];
    builder.push(mk_punct('#'));
    if let ast::CommentPlacement::Inner = doc {
        builder.push(mk_punct('!'));
    }
    builder.open(tt::DelimiterKind::Bracket, span);
    builder.extend(meta_tkns);
    builder.close(span);
}
