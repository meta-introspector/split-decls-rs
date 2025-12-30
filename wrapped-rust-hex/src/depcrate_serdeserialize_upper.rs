// Generated macro for serialize_upper (function)
macro_rules! Depcrate_serdeserialize_upper {
() => {
// Module: crate::serde
// Provides: {"serialize_upper"}
// Dependencies: {}
# [doc = " Serializes `data` as hex string using uppercase characters."] # [doc = ""] # [doc = " Apart from the characters' casing, this works exactly like `serialize()`."] # [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] pub fn serialize_upper < S , T > (data : T , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , T : ToHex , { let s = data . encode_hex_upper :: < String > () ; serializer . serialize_str (& s) }
};
}
