// Generated macro for impl_23 (impl)
macro_rules! Depcrate_directiveimpl_23 {
() => {
// Module: crate::directive
// Provides: {"impl_23"}
// Dependencies: {}
impl DirectiveKind { # [doc = " Returns both the kind and the path."] # [doc = ""] # [doc = " Returns `None` if the directive isn't from jsondocck (e.g. from compiletest)."] pub fn parse < 'a > (directive_name : & str , negated : bool , args : & 'a [String] ,) -> Option < (Self , & 'a str) > { let kind = match (directive_name , negated) { ("count" , false) => { assert_eq ! (args . len () , 2) ; let expected = args [1] . parse () . expect ("invalid number for `count`") ; Self :: CountIs { expected } } ("ismany" , false) => { assert ! (args . len () >= 2 , "Not enough args to `ismany`") ; let values = args [1 ..] . to_owned () ; Self :: IsMany { values } } ("is" , false) => { assert_eq ! (args . len () , 2) ; Self :: Is { value : args [1] . clone () } } ("is" , true) => { assert_eq ! (args . len () , 2) ; Self :: IsNot { value : args [1] . clone () } } ("set" , false) => { assert_eq ! (args . len () , 3) ; assert_eq ! (args [1] , "=") ; return Some ((Self :: Set { variable : args [0] . clone () } , & args [2])) ; } ("has" , false) => match args { [_path] => Self :: HasPath , [_path , value] => Self :: HasValue { value : value . clone () } , _ => panic ! ("`//@ has` must have 2 or 3 arguments, but got {args:?}") , } , ("has" , true) => match args { [_path] => Self :: HasNotPath , [_path , value] => Self :: HasNotValue { value : value . clone () } , _ => panic ! ("`//@ !has` must have 2 or 3 arguments, but got {args:?}") , } , _ => return None , } ; Some ((kind , & args [0])) } }
};
}
