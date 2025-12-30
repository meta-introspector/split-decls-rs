// Generated macro for ule (module)
macro_rules! Depcrateule {
() => {
// Module: crate
// Provides: {"ule"}
// Dependencies: {}
pub mod ule { # ! [doc = " Traits that data provider implementations can use to optimize storage"] # ! [doc = " by using [`VarULE`](zerovec::ule::VarULE)."] # ! [doc = ""] # ! [doc = " See [`MaybeAsVarULE`] for details."] pub use super :: varule_traits :: MaybeAsVarULE ; # [cfg (feature = "export")] pub use super :: varule_traits :: MaybeEncodeAsVarULE ; }
};
}
