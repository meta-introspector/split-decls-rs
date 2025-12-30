// Generated macro for OsslParamBuilder (struct)
macro_rules! Depcrate_ossl_paramOsslParamBuilder {
() => {
// Module: crate::ossl_param
// Provides: {"OsslParamBuilder"}
// Dependencies: {}
# [doc = " Wrapper around the internal OsslParamBuilderInternal that adds lifetime management"] # [doc = " since the builder does not own the key and value data that is added to it."] pub struct OsslParamBuilder < 'a > { builder : OsslParamBuilderInternal , _marker : PhantomData < & 'a () > , }
};
}
