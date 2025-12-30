// Generated macro for impl_32 (impl)
macro_rules! Depcrate_precedenceimpl_32 {
() => {
// Module: crate::precedence
// Provides: {"impl_32"}
// Dependencies: {}
impl < P1 , P2 , P3 , Q > Operator < P1 , P2 , P3 , Q > where Q : Ord + Copy , { fn precedence (& self) -> Q { match self { Operator :: Prefix (_ , p) => * p , Operator :: Postfix (_ , p) => * p , Operator :: Binary (_ , p , _) => * p , } } fn is_postfix (& self) -> bool { match self { Operator :: Postfix (_ , _) => true , _ => false , } } }
};
}
