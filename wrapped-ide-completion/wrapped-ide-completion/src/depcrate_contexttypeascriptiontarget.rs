// Generated macro for TypeAscriptionTarget (enum)
macro_rules! Depcrate_contextTypeAscriptionTarget {
() => {
// Module: crate::context
// Provides: {"TypeAscriptionTarget"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Eq)] pub (crate) enum TypeAscriptionTarget { Let (Option < ast :: Pat >) , FnParam (Option < ast :: Pat >) , RetType (Option < ast :: Expr >) , Const (Option < ast :: Expr >) , }
};
}
