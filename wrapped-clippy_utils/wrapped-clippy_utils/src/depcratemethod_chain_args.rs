// Generated macro for method_chain_args (function)
macro_rules! Depcratemethod_chain_args {
() => {
// Module: crate
// Provides: {"method_chain_args"}
// Dependencies: {}
# [doc = " Matches an `Expr` against a chain of methods, and return the matched `Expr`s."] # [doc = ""] # [doc = " For example, if `expr` represents the `.baz()` in `foo.bar().baz()`,"] # [doc = " `method_chain_args(expr, &[\"bar\", \"baz\"])` will return a `Vec`"] # [doc = " containing the `Expr`s for"] # [doc = " `.bar()` and `.baz()`"] pub fn method_chain_args < 'a > (expr : & 'a Expr < '_ > , methods : & [Symbol]) -> Option < Vec < (& 'a Expr < 'a > , & 'a [Expr < 'a >]) > > { let mut current = expr ; let mut matched = Vec :: with_capacity (methods . len ()) ; for method_name in methods . iter () . rev () { if let ExprKind :: MethodCall (path , receiver , args , _) = current . kind { if path . ident . name == * method_name { if receiver . span . from_expansion () || args . iter () . any (| e | e . span . from_expansion ()) { return None ; } matched . push ((receiver , args)) ; current = receiver ; } else { return None ; } } else { return None ; } } matched . reverse () ; Some (matched) }
};
}
