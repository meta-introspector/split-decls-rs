// Generated macro for bool_try_from_tt (function)
macro_rules! Depcrate_testsbool_try_from_tt {
() => {
// Module: crate::tests
// Provides: {"bool_try_from_tt"}
// Dependencies: {}
# [cfg (feature = "proc-macro2")] # [test] fn bool_try_from_tt () { use std :: convert :: TryFrom ; use proc_macro2 :: { Ident , Span , TokenTree } ; use crate :: BoolLit ; let ident = | s : & str | Ident :: new (s , Span :: call_site ()) ; assert_eq ! (BoolLit :: try_from (TokenTree :: Ident (ident ("true"))) . unwrap () , BoolLit :: True) ; assert_eq ! (BoolLit :: try_from (TokenTree :: Ident (ident ("false"))) . unwrap () , BoolLit :: False) ; assert ! (BoolLit :: try_from (TokenTree :: Ident (ident ("falsex"))) . is_err ()) ; assert ! (BoolLit :: try_from (TokenTree :: Ident (ident ("_false"))) . is_err ()) ; assert ! (BoolLit :: try_from (TokenTree :: Ident (ident ("False"))) . is_err ()) ; assert ! (BoolLit :: try_from (TokenTree :: Ident (ident ("True"))) . is_err ()) ; assert ! (BoolLit :: try_from (TokenTree :: Ident (ident ("ltrue"))) . is_err ()) ; assert_eq ! (Literal :: try_from (TokenTree :: Ident (ident ("true"))) . unwrap () , Literal :: Bool (BoolLit :: True) ,) ; assert_eq ! (Literal :: try_from (TokenTree :: Ident (ident ("false"))) . unwrap () , Literal :: Bool (BoolLit :: False) ,) ; assert ! (Literal :: try_from (TokenTree :: Ident (ident ("falsex"))) . is_err ()) ; assert ! (Literal :: try_from (TokenTree :: Ident (ident ("_false"))) . is_err ()) ; assert ! (Literal :: try_from (TokenTree :: Ident (ident ("False"))) . is_err ()) ; assert ! (Literal :: try_from (TokenTree :: Ident (ident ("True"))) . is_err ()) ; assert ! (Literal :: try_from (TokenTree :: Ident (ident ("ltrue"))) . is_err ()) ; }
};
}
