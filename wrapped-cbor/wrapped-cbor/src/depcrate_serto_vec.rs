// Generated macro for to_vec (function)
macro_rules! Depcrate_serto_vec {
() => {
// Module: crate::ser
// Provides: {"to_vec"}
// Dependencies: {}
# [doc = " Serializes a value to a vector."] # [cfg (any (feature = "std" , feature = "alloc"))] pub fn to_vec < T > (value : & T) -> Result < Vec < u8 > > where T : ser :: Serialize , { let mut vec = Vec :: new () ; value . serialize (& mut Serializer :: new (& mut vec)) ? ; Ok (vec) }
};
}
