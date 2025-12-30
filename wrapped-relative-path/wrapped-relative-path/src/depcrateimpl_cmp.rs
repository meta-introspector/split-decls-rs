// Generated macro for impl_cmp (macro)
macro_rules! Depcrateimpl_cmp {
() => {
// Module: crate
// Provides: {"impl_cmp"}
// Dependencies: {}
macro_rules ! impl_cmp { ($ (# [cfg (feature = $ feature : literal)]) ? $ lhs : ty , $ rhs : ty) => { $ (# [cfg (feature = $ feature)]) * $ (# [cfg_attr (relative_path_docsrs , doc (cfg (feature = $ feature)))]) * impl <'a , 'b > PartialEq <$ rhs > for $ lhs { # [inline] fn eq (& self , other : &$ rhs) -> bool { < RelativePath as PartialEq >:: eq (self , other) } } $ (# [cfg (feature = $ feature)]) * $ (# [cfg_attr (relative_path_docsrs , doc (cfg (feature = $ feature)))]) * impl <'a , 'b > PartialEq <$ lhs > for $ rhs { # [inline] fn eq (& self , other : &$ lhs) -> bool { < RelativePath as PartialEq >:: eq (self , other) } } $ (# [cfg (feature = $ feature)]) * $ (# [cfg_attr (relative_path_docsrs , doc (cfg (feature = $ feature)))]) * impl <'a , 'b > PartialOrd <$ rhs > for $ lhs { # [inline] fn partial_cmp (& self , other : &$ rhs) -> Option < cmp :: Ordering > { < RelativePath as PartialOrd >:: partial_cmp (self , other) } } $ (# [cfg (feature = $ feature)]) * $ (# [cfg_attr (relative_path_docsrs , doc (cfg (feature = $ feature)))]) * impl <'a , 'b > PartialOrd <$ lhs > for $ rhs { # [inline] fn partial_cmp (& self , other : &$ lhs) -> Option < cmp :: Ordering > { < RelativePath as PartialOrd >:: partial_cmp (self , other) } } } ; }
};
}
