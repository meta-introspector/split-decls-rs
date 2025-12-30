// Generated macro for remove_all_parens (function)
macro_rules! Depcrate_unnested_or_patternsremove_all_parens {
() => {
// Module: crate::unnested_or_patterns
// Provides: {"remove_all_parens"}
// Dependencies: {}
# [doc = " Remove all `(p)` patterns in `pat`."] fn remove_all_parens (pat : & mut Pat) { # [derive (Default)] struct Visitor { # [doc = " If is not in the outer most pattern. This is needed to avoid removing the outermost"] # [doc = " parens because top-level or-patterns are not allowed in let statements."] is_inner : bool , } impl MutVisitor for Visitor { fn visit_pat (& mut self , pat : & mut Pat) { let is_inner = mem :: replace (& mut self . is_inner , true) ; walk_pat (self , pat) ; let inner = match & mut pat . kind { Paren (i) if is_inner => mem :: replace (& mut i . kind , Wild) , _ => return , } ; pat . kind = inner ; } } Visitor :: default () . visit_pat (pat) ; }
};
}
