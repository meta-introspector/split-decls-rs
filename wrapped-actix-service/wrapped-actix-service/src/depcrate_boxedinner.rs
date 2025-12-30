// Generated macro for Inner (type)
macro_rules! Depcrate_boxedInner {
() => {
// Module: crate::boxed
// Provides: {"Inner"}
// Dependencies: {}
type Inner < C , Req , Res , Err , InitErr > = Box < dyn ServiceFactory < Req , Config = C , Response = Res , Error = Err , InitError = InitErr , Service = BoxService < Req , Res , Err > , Future = BoxFuture < Result < BoxService < Req , Res , Err > , InitErr > > , > , > ;
};
}
