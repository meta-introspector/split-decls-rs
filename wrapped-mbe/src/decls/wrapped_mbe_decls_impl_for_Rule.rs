use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Rule {
    fn parse(
        edition: impl Copy + Fn(SyntaxContext) -> Edition,
        src: &mut TtIter<'_, Span>,
    ) -> Result<Self, ParseError> {
        let (_, lhs) = src
            .expect_subtree()
            .map_err(|()| ParseError::expected("expected subtree"))?;
        src.expect_char('=')
            .map_err(|()| ParseError::expected("expected `=`"))?;
        src.expect_char('>')
            .map_err(|()| ParseError::expected("expected `>`"))?;
        let (_, rhs) = src
            .expect_subtree()
            .map_err(|()| ParseError::expected("expected subtree"))?;
        let lhs = MetaTemplate::parse_pattern(edition, lhs)?;
        let rhs = MetaTemplate::parse_template(edition, rhs)?;
        Ok(crate::Rule { lhs, rhs })
    }
}
