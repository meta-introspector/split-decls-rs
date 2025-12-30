// Generated macro for ParseFut (type)
macro_rules! Depcrate_extensionsParseFut {
() => {
// Module: crate::extensions
// Provides: {"ParseFut"}
// Dependencies: {}
type ParseFut < 'a > = & 'a mut (dyn Future < Output = ServerResult < ExecutableDocument > > + Send + Unpin) ;
};
}
