// Generated macro for IfaceCache (struct)
macro_rules! Depcrate_objectpathIfaceCache {
() => {
// Module: crate::objectpath
// Provides: {"IfaceCache"}
// Dependencies: {}
# [derive (Debug)] # [doc = " Cache of built-in interfaces, in order to save memory when many object paths implement the same interface(s)."] pub struct IfaceCache < M : MethodType < D > , D : DataType > (Mutex < ArcMap < IfaceName < 'static > , Interface < M , D > > >) ;
};
}
