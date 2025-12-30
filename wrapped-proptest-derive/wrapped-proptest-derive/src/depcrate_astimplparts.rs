// Generated macro for ImplParts (type)
macro_rules! Depcrate_astImplParts {
() => {
// Module: crate::ast
// Provides: {"ImplParts"}
// Dependencies: {}
# [doc = " The three main parts to deriving `Arbitrary` for a type."] # [doc = " That is: the associated items `Parameters` (`Params`),"] # [doc = " `Strategy` (`Strategy`) as well as the construction of the"] # [doc = " strategy itself (`Ctor`)."] pub type ImplParts = (Params , Strategy , Ctor) ;
};
}
