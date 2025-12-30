// Generated macro for values (module)
macro_rules! Depcrate_resolvervalues {
() => {
// Module: crate::resolver
// Provides: {"values"}
// Dependencies: {}
pub (crate) mod values { use super :: * ; use crate :: parse :: fixture :: ArgumentValue ; pub (crate) fn get < 'a > (values : impl Iterator < Item = & 'a ArgumentValue >) -> impl Resolver + 'a { values . map (| av | (av . arg . clone () , & av . expr)) . collect :: < HashMap < _ , & 'a Expr > > () } # [cfg (test)] mod should { use super :: * ; use crate :: test :: { assert_eq , * } ; # [test] fn resolve_by_use_the_given_name () { let data = vec ! [arg_value ("pippo" , "42") , arg_value ("donaldduck" , "vec![1,2]") ,] ; let resolver = get (data . iter ()) ; assert_eq ! (resolver . resolve (& pat ("pippo")) . unwrap () . into_owned () , "42" . ast ()) ; assert_eq ! (resolver . resolve (& pat ("donaldduck")) . unwrap () . into_owned () , "vec![1,2]" . ast ()) ; } } }
};
}
