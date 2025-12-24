use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'hir> GenericArgsCtor<'hir> {
    fn is_empty(&self) -> bool {
        self.args.is_empty() && self.constraints.is_empty()
            && self.parenthesized == hir::GenericArgsParentheses::No
    }
    fn into_generic_args(
        self,
        this: &LoweringContext<'_, 'hir>,
    ) -> &'hir hir::GenericArgs<'hir> {
        let ga = hir::GenericArgs {
            args: this.arena.alloc_from_iter(self.args),
            constraints: self.constraints,
            parenthesized: self.parenthesized,
            span_ext: this.lower_span(self.span),
        };
        this.arena.alloc(ga)
    }
}
