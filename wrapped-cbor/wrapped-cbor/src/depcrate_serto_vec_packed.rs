// Generated macro for to_vec_packed (function)
macro_rules! Depcrate_serto_vec_packed {
() => {
// Module: crate::ser
// Provides: {"to_vec_packed"}
// Dependencies: {}
# [doc = " Serializes a value to a vector in packed format."] # [cfg (feature = "std")] pub fn to_vec_packed < T > (value : & T) -> Result < Vec < u8 > > where T : ser :: Serialize , { let mut vec = Vec :: new () ; value . serialize (& mut Serializer :: new (& mut IoWrite :: new (& mut vec)) . packed_format ()) ? ; Ok (vec) }
};
}
