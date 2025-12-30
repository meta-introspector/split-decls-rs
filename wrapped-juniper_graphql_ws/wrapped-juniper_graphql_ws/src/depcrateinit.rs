// Generated macro for Init (trait)
macro_rules! DepcrateInit {
() => {
// Module: crate
// Provides: {"Init"}
// Dependencies: {}
# [doc = " Init defines the requirements for types that can provide connection configurations when"] # [doc = " ConnectionInit messages are received. Implementations are provided for `ConnectionConfig` and"] # [doc = " closures that meet the requirements."] pub trait Init < S : ScalarValue , CtxT > : Unpin + 'static { # [doc = " The error that is returned on failure. The formatted error will be used as the contents of"] # [doc = " the \"message\" field sent back to the client."] type Error : Error ; # [doc = " The future configuration type."] type Future : Future < Output = Result < ConnectionConfig < CtxT > , Self :: Error > > + Send + 'static ; # [doc = " Returns a future for the configuration to use."] fn init (self , params : Variables < S >) -> Self :: Future ; }
};
}
