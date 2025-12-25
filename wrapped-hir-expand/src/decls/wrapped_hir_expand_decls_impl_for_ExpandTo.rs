use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl ExpandTo {
    pub fn from_call_site(call: &ast::MacroCall) -> ExpandTo {
        use syntax::SyntaxKind::*;
        let syn = call.syntax();
        let parent = match syn.parent() {
            Some(it) => it,
            None => return ExpandTo::Statements,
        };
        if parent.kind() == MACRO_EXPR
            && parent
                .parent()
                .is_some_and(|p| matches!(p.kind(), EXPR_STMT | STMT_LIST | MACRO_STMTS))
        {
            return ExpandTo::Statements;
        }
        match parent.kind() {
            MACRO_ITEMS | SOURCE_FILE | ITEM_LIST => ExpandTo::Items,
            MACRO_STMTS | EXPR_STMT | STMT_LIST => ExpandTo::Statements,
            MACRO_PAT => ExpandTo::Pattern,
            MACRO_TYPE => ExpandTo::Type,
            ARG_LIST | ARRAY_EXPR | AWAIT_EXPR | BIN_EXPR | BREAK_EXPR | CALL_EXPR | CAST_EXPR
            | CLOSURE_EXPR | FIELD_EXPR | FOR_EXPR | IF_EXPR | INDEX_EXPR | LET_EXPR
            | MATCH_ARM | MATCH_EXPR | MATCH_GUARD | METHOD_CALL_EXPR | PAREN_EXPR | PATH_EXPR
            | PREFIX_EXPR | RANGE_EXPR | RECORD_EXPR_FIELD | REF_EXPR | RETURN_EXPR | TRY_EXPR
            | TUPLE_EXPR | WHILE_EXPR | MACRO_EXPR => ExpandTo::Expr,
            _ => ExpandTo::Items,
        }
    }
}
