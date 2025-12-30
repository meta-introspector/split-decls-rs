// Generated macro for impl_debug_and_serde (macro)
macro_rules! Depcrate_utilsimpl_debug_and_serde {
() => {
// Module: crate::utils
// Provides: {"impl_debug_and_serde"}
// Dependencies: {}
# [doc = " Implements `core::fmt::Debug` and `serde::{Serialize, Deserialize}` (when serde"] # [doc = " feature is enabled) for atomic bool, integer, or float."] macro_rules ! impl_debug_and_serde { (AtomicF16) => { impl_debug ! (AtomicF16) ; } ; (AtomicF128) => { impl_debug ! (AtomicF128) ; } ; ($ atomic_type : ident) => { impl_debug ! ($ atomic_type) ; # [cfg (feature = "serde")] # [cfg_attr (docsrs , doc (cfg (feature = "serde")))] impl serde :: ser :: Serialize for $ atomic_type { # [allow (clippy :: missing_inline_in_public_items)] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { self . load (Ordering :: Relaxed) . serialize (serializer) } } # [cfg (feature = "serde")] # [cfg_attr (docsrs , doc (cfg (feature = "serde")))] impl <'de > serde :: de :: Deserialize <'de > for $ atomic_type { # [allow (clippy :: missing_inline_in_public_items)] fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: de :: Deserializer <'de >, { serde :: de :: Deserialize :: deserialize (deserializer) . map (Self :: new) } } } ; }
};
}
