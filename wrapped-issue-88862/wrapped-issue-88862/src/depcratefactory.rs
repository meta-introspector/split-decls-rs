// Generated macro for factory (function)
macro_rules! Depcratefactory {
() => {
// Module: crate
// Provides: {"factory"}
// Dependencies: {}
fn factory < X , Req > (factory : X) -> BoxServiceFactory < Req > where X : ServiceFactory < Req > + 'static { BoxServiceFactory (Box :: new (FactoryWrapper (factory))) }
};
}
