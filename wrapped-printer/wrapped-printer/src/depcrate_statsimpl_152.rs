// Generated macro for impl_152 (impl)
macro_rules! Depcrate_statsimpl_152 {
() => {
// Module: crate::stats
// Provides: {"impl_152"}
// Dependencies: {}
# [cfg (feature = "serde")] impl serde :: Serialize for Stats { fn serialize < S : serde :: Serializer > (& self , s : S ,) -> Result < S :: Ok , S :: Error > { use serde :: ser :: SerializeStruct ; let mut state = s . serialize_struct ("Stats" , 7) ? ; state . serialize_field ("elapsed" , & self . elapsed) ? ; state . serialize_field ("searches" , & self . searches) ? ; state . serialize_field ("searches_with_match" , & self . searches_with_match ,) ? ; state . serialize_field ("bytes_searched" , & self . bytes_searched) ? ; state . serialize_field ("bytes_printed" , & self . bytes_printed) ? ; state . serialize_field ("matched_lines" , & self . matched_lines) ? ; state . serialize_field ("matches" , & self . matches) ? ; state . end () } }
};
}
