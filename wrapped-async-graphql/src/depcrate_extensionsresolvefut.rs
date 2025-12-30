// Generated macro for ResolveFut (type)
macro_rules! Depcrate_extensionsResolveFut {
() => {
// Module: crate::extensions
// Provides: {"ResolveFut"}
// Dependencies: {}
# [doc = " A future type used to resolve the field"] pub type ResolveFut < 'a > = & 'a mut (dyn Future < Output = ServerResult < Option < Value > > > + Send + Unpin) ;
};
}
