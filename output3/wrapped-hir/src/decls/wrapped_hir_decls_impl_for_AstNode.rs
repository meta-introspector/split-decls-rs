use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl AstNode for FieldSource {
    fn can_cast(kind: syntax::SyntaxKind) -> bool
    where
        Self: Sized,
    {
        ast::RecordField::can_cast(kind) || ast::TupleField::can_cast(kind)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if ast::RecordField::can_cast(syntax.kind()) {
            <ast::RecordField as AstNode>::cast(syntax).map(FieldSource::Named)
        } else if ast::TupleField::can_cast(syntax.kind()) {
            <ast::TupleField as AstNode>::cast(syntax).map(FieldSource::Pos)
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            FieldSource::Named(it) => it.syntax(),
            FieldSource::Pos(it) => it.syntax(),
        }
    }
}
