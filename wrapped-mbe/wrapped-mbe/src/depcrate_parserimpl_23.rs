// Generated macro for impl_23 (impl)
macro_rules! Depcrate_parserimpl_23 {
() => {
// Module: crate::parser
// Provides: {"impl_23"}
// Dependencies: {}
impl MetaTemplate { pub (crate) fn parse_pattern (edition : impl Copy + Fn (SyntaxContext) -> Edition , pattern : TtIter < '_ , Span > ,) -> Result < Self , ParseError > { MetaTemplate :: parse (edition , pattern , Mode :: Pattern) } pub (crate) fn parse_template (edition : impl Copy + Fn (SyntaxContext) -> Edition , template : TtIter < '_ , Span > ,) -> Result < Self , ParseError > { MetaTemplate :: parse (edition , template , Mode :: Template) } pub (crate) fn iter (& self) -> impl Iterator < Item = & Op > { self . 0 . iter () } fn parse (edition : impl Copy + Fn (SyntaxContext) -> Edition , mut src : TtIter < '_ , Span > , mode : Mode ,) -> Result < Self , ParseError > { let mut res = Vec :: new () ; while let Some (first) = src . peek () { let op = next_op (edition , first , & mut src , mode) ? ; res . push (op) ; } Ok (MetaTemplate (res . into_boxed_slice ())) } }
};
}
