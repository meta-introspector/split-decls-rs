// Generated macro for insert_necessary_parens (function)
macro_rules! Depcrate_unnested_or_patternsinsert_necessary_parens {
() => {
// Module: crate::unnested_or_patterns
// Provides: {"insert_necessary_parens"}
// Dependencies: {}
# [doc = " Insert parens where necessary according to Rust's precedence rules for patterns."] fn insert_necessary_parens (pat : & mut Pat) { struct Visitor ; impl MutVisitor for Visitor { fn visit_pat (& mut self , pat : & mut Pat) { use ast :: BindingMode ; walk_pat (self , pat) ; let target = match & mut pat . kind { Ident (.. , Some (p)) | Box (p) | Ref (p , _ , _) if matches ! (& p . kind , Or (ps) if ps . len () > 1) => p , Ref (p , Pinnedness :: Not , Mutability :: Not) if matches ! (p . kind , Ident (BindingMode :: MUT , ..)) => p , _ => return , } ; target . kind = Paren (Box :: new (take_pat (target))) ; } } Visitor . visit_pat (pat) ; }
};
}
