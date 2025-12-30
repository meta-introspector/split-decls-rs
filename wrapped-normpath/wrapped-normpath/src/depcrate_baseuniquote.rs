// Generated macro for uniquote (module)
macro_rules! Depcrate_baseuniquote {
() => {
// Module: crate::base
// Provides: {"uniquote"}
// Dependencies: {}
# [cfg (feature = "uniquote")] # [cfg_attr (normpath_docs_rs , doc (cfg (feature = "uniquote")))] mod uniquote { use uniquote :: Formatter ; use uniquote :: Quote ; use uniquote :: Result ; use super :: BasePath ; use super :: BasePathBuf ; impl Quote for BasePath { # [inline] fn escape (& self , f : & mut Formatter < '_ >) -> Result { self . 0 . escape (f) } } impl Quote for BasePathBuf { # [inline] fn escape (& self , f : & mut Formatter < '_ >) -> Result { (* * self) . escape (f) } } }
};
}
