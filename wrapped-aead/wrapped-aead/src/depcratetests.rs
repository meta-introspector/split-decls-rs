// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [doc = " Ensure that `Aead` is `dyn`-compatible"] # [cfg (feature = "alloc")] # [allow (dead_code)] type DynAead < N , T > = dyn Aead < NonceSize = N , TagSize = T > ; # [doc = " Ensure that `AeadInOut` is `dyn`-compatible"] # [allow (dead_code)] type DynAeadInOut < N , T > = dyn AeadInOut < NonceSize = N , TagSize = T > ; }
};
}
