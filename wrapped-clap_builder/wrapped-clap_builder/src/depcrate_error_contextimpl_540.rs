// Generated macro for impl_540 (impl)
macro_rules! Depcrate_error_contextimpl_540 {
() => {
// Module: crate::error::context
// Provides: {"impl_540"}
// Dependencies: {}
impl std :: fmt :: Display for ContextValue { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: None => "" . fmt (f) , Self :: Bool (v) => v . fmt (f) , Self :: String (v) => v . fmt (f) , Self :: Strings (v) => v . join (", ") . fmt (f) , Self :: StyledStr (v) => v . fmt (f) , Self :: StyledStrs (v) => { for (i , v) in v . iter () . enumerate () { if i != 0 { ", " . fmt (f) ? ; } v . fmt (f) ? ; } Ok (()) } Self :: Number (v) => v . fmt (f) , } } }
};
}
