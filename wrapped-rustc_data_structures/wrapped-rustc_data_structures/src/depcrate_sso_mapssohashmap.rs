// Generated macro for SsoHashMap (enum)
macro_rules! Depcrate_sso_mapSsoHashMap {
() => {
// Module: crate::sso::map
// Provides: {"SsoHashMap"}
// Dependencies: {}
# [doc = " Small-storage-optimized implementation of a map."] # [doc = ""] # [doc = " Stores elements in a small array up to a certain length"] # [doc = " and switches to `HashMap` when that length is exceeded."] # [derive (Clone)] pub enum SsoHashMap < K , V > { Array (ArrayVec < (K , V) , SSO_ARRAY_SIZE >) , Map (FxHashMap < K , V >) , }
};
}
