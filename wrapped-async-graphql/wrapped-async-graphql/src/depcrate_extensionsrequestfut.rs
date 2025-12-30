// Generated macro for RequestFut (type)
macro_rules! Depcrate_extensionsRequestFut {
() => {
// Module: crate::extensions
// Provides: {"RequestFut"}
// Dependencies: {}
type RequestFut < 'a > = & 'a mut (dyn Future < Output = Response > + Send + Unpin) ;
};
}
