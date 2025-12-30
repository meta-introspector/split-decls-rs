// Generated macro for AndThenServiceFactory (struct)
macro_rules! Depcrate_and_thenAndThenServiceFactory {
() => {
// Module: crate::and_then
// Provides: {"AndThenServiceFactory"}
// Dependencies: {}
# [doc = " `.and_then()` service factory combinator"] pub struct AndThenServiceFactory < A , B , Req > where A : ServiceFactory < Req > , A :: Config : Clone , B : ServiceFactory < A :: Response , Config = A :: Config , Error = A :: Error , InitError = A :: InitError > , { inner : Rc < (A , B) > , _phantom : PhantomData < Req > , }
};
}
