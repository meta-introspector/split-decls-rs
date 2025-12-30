// Generated macro for impl_245 (impl)
macro_rules! Depcrate_serdeimpl_245 {
() => {
// Module: crate::serde
// Provides: {"impl_245"}
// Dependencies: {}
# [cfg (all (feature = "paserk" , feature = "serde"))] # [cfg_attr (docsrs , doc (cfg (all (feature = "paserk" , feature = "serde"))))] impl < V > serde :: Serialize for AsymmetricPublicKey < V > where AsymmetricPublicKey < V > : FormatAsPaserk , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { use serde :: ser :: Error ; let mut paserk_string = String :: new () ; self . fmt (& mut paserk_string) . map_err (S :: Error :: custom) ? ; serializer . serialize_str (& paserk_string) } }
};
}
