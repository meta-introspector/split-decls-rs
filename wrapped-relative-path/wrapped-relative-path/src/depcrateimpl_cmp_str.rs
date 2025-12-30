// Generated macro for impl_cmp_str (macro)
macro_rules! Depcrateimpl_cmp_str {
() => {
// Module: crate
// Provides: {"impl_cmp_str"}
// Dependencies: {}
macro_rules ! impl_cmp_str { ($ (# [cfg (feature = $ feature : literal)]) ? $ lhs : ty , $ rhs : ty) => { $ (# [cfg (feature = $ feature)]) * $ (# [cfg_attr (relative_path_docsrs , doc (cfg (feature = $ feature)))]) * impl <'a , 'b > PartialEq <$ rhs > for $ lhs { # [inline] fn eq (& self , other : &$ rhs) -> bool { < RelativePath as PartialEq >:: eq (self , other . as_ref ()) } } $ (# [cfg (feature = $ feature)]) * $ (# [cfg_attr (relative_path_docsrs , doc (cfg (feature = $ feature)))]) * impl <'a , 'b > PartialEq <$ lhs > for $ rhs { # [inline] fn eq (& self , other : &$ lhs) -> bool { < RelativePath as PartialEq >:: eq (self . as_ref () , other) } } $ (# [cfg (feature = $ feature)]) * $ (# [cfg_attr (relative_path_docsrs , doc (cfg (feature = $ feature)))]) * impl <'a , 'b > PartialOrd <$ rhs > for $ lhs { # [inline] fn partial_cmp (& self , other : &$ rhs) -> Option < cmp :: Ordering > { < RelativePath as PartialOrd >:: partial_cmp (self , other . as_ref ()) } } $ (# [cfg (feature = $ feature)]) * $ (# [cfg_attr (relative_path_docsrs , doc (cfg (feature = $ feature)))]) * impl <'a , 'b > PartialOrd <$ lhs > for $ rhs { # [inline] fn partial_cmp (& self , other : &$ lhs) -> Option < cmp :: Ordering > { < RelativePath as PartialOrd >:: partial_cmp (self . as_ref () , other) } } } ; }
};
}
