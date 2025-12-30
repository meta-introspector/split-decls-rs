// Generated macro for impl_251 (impl)
macro_rules! Depcrate_serdeimpl_251 {
() => {
// Module: crate::serde
// Provides: {"impl_251"}
// Dependencies: {}
# [cfg (all (feature = "paserk" , feature = "serde"))] # [cfg_attr (docsrs , doc (cfg (all (feature = "paserk" , feature = "serde"))))] impl serde :: Serialize for Id { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { use serde :: ser :: Error ; let mut paserk_id = String :: new () ; self . fmt (& mut paserk_id) . map_err (S :: Error :: custom) ? ; serializer . serialize_str (& paserk_id) } }
};
}
