use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl VisitMut for SelfToDbRewriter {
    fn visit_expr_path_mut(&mut self, i: &mut syn::ExprPath) {
        if i.path.is_ident("self") {
            i.path = parse_quote_spanned!(i.path.span() => db);
        }
    }
}
