use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl ast::Expr {
    /// Parses an `ast::Expr` from `text`.
    ///
    /// Note that if the parsed root node is not a valid expression, [`Parse::tree`] will panic.
    /// For example:
    /// ```rust,should_panic
    /// # use syntax::{ast, Edition};
    /// ast::Expr::parse("let fail = true;", Edition::CURRENT).tree();
    /// ```
    pub fn parse(text: &str, edition: Edition) -> Parse<ast::Expr> {
        let _p = tracing::info_span!("Expr::parse").entered();
        let (green, errors) = parsing::parse_text_at(text, parser::TopEntryPoint::Expr, edition);
        let root = SyntaxNode::new_root(green.clone());
        assert!(
            ast::Expr::can_cast(root.kind()) || root.kind() == SyntaxKind::ERROR,
            "{:?} isn't an expression",
            root.kind()
        );
        Parse::new(green, errors)
    }
}
