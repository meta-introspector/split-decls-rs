// Generated macro for is_trivial_regex (function)
macro_rules! Depcrate_regexis_trivial_regex {
() => {
// Module: crate::regex
// Provides: {"is_trivial_regex"}
// Dependencies: {}
fn is_trivial_regex (s : & regex_syntax :: hir :: Hir) -> Option < & 'static str > { use regex_syntax :: hir :: HirKind :: { Alternation , Concat , Empty , Literal , Look } ; use regex_syntax :: hir :: Look as HirLook ; let is_literal = | e : & [regex_syntax :: hir :: Hir] | e . iter () . all (| e | matches ! (* e . kind () , Literal (_))) ; match * s . kind () { Empty | Look (_) => Some ("the regex is unlikely to be useful as it is") , Literal (_) => Some ("consider using `str::contains`") , Alternation (ref exprs) => { if exprs . iter () . all (| e | matches ! (e . kind () , Empty)) { Some ("the regex is unlikely to be useful as it is") } else { None } } , Concat (ref exprs) => match (exprs [0] . kind () , exprs [exprs . len () - 1] . kind ()) { (& Look (HirLook :: Start) , & Look (HirLook :: End)) if exprs [1 .. (exprs . len () - 1)] . is_empty () => { Some ("consider using `str::is_empty`") } , (& Look (HirLook :: Start) , & Look (HirLook :: End)) if is_literal (& exprs [1 .. (exprs . len () - 1)]) => { Some ("consider using `==` on `str`s") } , (& Look (HirLook :: Start) , & Literal (_)) if is_literal (& exprs [1 ..]) => { Some ("consider using `str::starts_with`") } , (& Literal (_) , & Look (HirLook :: End)) if is_literal (& exprs [1 .. (exprs . len () - 1)]) => { Some ("consider using `str::ends_with`") } , _ if is_literal (exprs) => Some ("consider using `str::contains`") , _ => None , } , _ => None , } }
};
}
