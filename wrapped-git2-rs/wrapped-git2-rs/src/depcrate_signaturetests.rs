// Generated macro for tests (module)
macro_rules! Depcrate_signaturetests {
() => {
// Module: crate::signature
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { Signature , Time } ; # [test] fn smoke () { Signature :: new ("foo" , "bar" , & Time :: new (89 , 0)) . unwrap () ; Signature :: now ("foo" , "bar") . unwrap () ; assert ! (Signature :: new ("<foo>" , "bar" , & Time :: new (89 , 0)) . is_err ()) ; assert ! (Signature :: now ("<foo>" , "bar") . is_err ()) ; let s = Signature :: now ("foo" , "bar") . unwrap () ; assert_eq ! (s . name () , Some ("foo")) ; assert_eq ! (s . email () , Some ("bar")) ; drop (s . clone ()) ; drop (s . to_owned ()) ; } }
};
}
