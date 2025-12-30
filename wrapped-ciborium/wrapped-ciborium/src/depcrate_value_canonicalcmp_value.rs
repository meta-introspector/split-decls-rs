// Generated macro for cmp_value (function)
macro_rules! Depcrate_value_canonicalcmp_value {
() => {
// Module: crate::value::canonical
// Provides: {"cmp_value"}
// Dependencies: {}
# [doc = " Compares two values uses canonical comparison, as defined in both"] # [doc = " RFC 7049 Section 3.9 (regarding key sorting) and RFC 8949 4.2.3 (as errata)."] # [doc = ""] # [doc = " In short, the comparison follow the following rules:"] # [doc = "   - If two keys have different lengths, the shorter one sorts earlier;"] # [doc = "   - If two keys have the same length, the one with the lower value in"] # [doc = "     (byte-wise) lexical order sorts earlier."] # [doc = ""] # [doc = " This specific comparison allows Maps and sorting that respect these two rules."] pub fn cmp_value (v1 : & Value , v2 : & Value) -> Ordering { use Value :: * ; match (v1 , v2) { (Integer (i) , Integer (o)) => { i . canonical_cmp (o) } (Text (s) , Text (o)) => match s . len () . cmp (& o . len ()) { Ordering :: Equal => s . cmp (o) , x => x , } , (Bool (s) , Bool (o)) => s . cmp (o) , (Null , Null) => Ordering :: Equal , (Tag (t , v) , Tag (ot , ov)) => match Value :: from (* t) . partial_cmp (& Value :: from (* ot)) { Some (Ordering :: Equal) | None => match v . partial_cmp (ov) { Some (x) => x , None => serialized_canonical_cmp (v1 , v2) , } , Some (x) => x , } , (_ , _) => serialized_canonical_cmp (v1 , v2) , } }
};
}
