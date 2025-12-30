// Generated macro for impl_658 (impl)
macro_rules! Depcrate_numberimpl_658 {
() => {
// Module: crate::number
// Provides: {"impl_658"}
// Dependencies: {}
impl Serialize for Number { # [cfg (not (feature = "arbitrary_precision"))] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { match self . n { N :: PosInt (u) => serializer . serialize_u64 (u) , N :: NegInt (i) => serializer . serialize_i64 (i) , N :: Float (f) => serializer . serialize_f64 (f) , } } # [cfg (feature = "arbitrary_precision")] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { use serde :: ser :: SerializeStruct ; let mut s = tri ! (serializer . serialize_struct (TOKEN , 1)) ; tri ! (s . serialize_field (TOKEN , & self . n)) ; s . end () } }
};
}
