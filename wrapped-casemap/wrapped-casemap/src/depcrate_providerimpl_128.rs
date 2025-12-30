// Generated macro for impl_128 (impl)
macro_rules! Depcrate_providerimpl_128 {
() => {
// Module: crate::provider
// Provides: {"impl_128"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde :: Deserialize < 'de > for CaseMap < 'de > { fn deserialize < D : serde :: Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { # [derive (serde :: Deserialize)] pub struct Raw < 'data > { # [serde (borrow)] pub trie : CodePointTrie < 'data , CaseMapData > , # [serde (borrow)] pub exceptions : CaseMapExceptions < 'data > , } let Raw { trie , exceptions } = Raw :: deserialize (deserializer) ? ; let result = Self { trie , exceptions } ; debug_assert ! (result . validate () . is_ok ()) ; Ok (result) } }
};
}
