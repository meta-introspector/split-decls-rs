// Generated macro for impl_1134 (impl)
macro_rules! Depcrate_runtime_method_encoding_iterimpl_1134 {
() => {
// Module: crate::runtime::method_encoding_iter
// Provides: {"impl_1134"}
// Dependencies: {}
impl fmt :: Display for EncodingParseError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if ! matches ! (self , Self :: ParseError (_)) { write ! (f , "failed parsing encoding: ") ? ; } match self { Self :: ParseError (e) => write ! (f , "{e}") ? , Self :: InvalidStackLayoutInteger => write ! (f , "invalid integer for stack layout") ? , Self :: MissingReturn => write ! (f , "return type must be present") ? , Self :: MissingReceiver => write ! (f , "receiver type must be present") ? , Self :: MissingSel => write ! (f , "selector type must be present") ? , Self :: InvalidReceiver (enc) => { write ! (f , "receiver encoding must be '@', but it was '{enc}'") ? ; } Self :: InvalidSel (enc) => { write ! (f , "selector encoding must be '@', but it was '{enc}'") ? ; } } write ! (f , ". This is likely a bug, please report it!") } }
};
}
