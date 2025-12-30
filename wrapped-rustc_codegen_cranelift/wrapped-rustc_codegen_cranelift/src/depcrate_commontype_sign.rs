// Generated macro for type_sign (function)
macro_rules! Depcrate_commontype_sign {
() => {
// Module: crate::common
// Provides: {"type_sign"}
// Dependencies: {}
pub (crate) fn type_sign (ty : Ty < '_ >) -> bool { match ty . kind () { ty :: Ref (..) | ty :: RawPtr (..) | ty :: FnPtr (..) | ty :: Char | ty :: Uint (..) | ty :: Bool => false , ty :: Int (..) => true , ty :: Float (..) => false , _ => panic ! ("{}" , ty) , } }
};
}
