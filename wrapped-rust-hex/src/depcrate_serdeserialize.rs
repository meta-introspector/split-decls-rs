// Generated macro for serialize (function)
macro_rules! Depcrate_serdeserialize {
() => {
// Module: crate::serde
// Provides: {"serialize"}
// Dependencies: {}
# [doc = " Serializes `data` as hex string using lowercase characters."] # [doc = ""] # [doc = " Lowercase characters are used (e.g. `f9b4ca`). The resulting string's length"] # [doc = " is always even, each byte in data is always encoded using two hex digits."] # [doc = " Thus, the resulting string contains exactly twice as many bytes as the input"] # [doc = " data."] # [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] pub fn serialize < S , T > (data : T , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , T : ToHex , { let s = data . encode_hex :: < String > () ; serializer . serialize_str (& s) }
};
}
