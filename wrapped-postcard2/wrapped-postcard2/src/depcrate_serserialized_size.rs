// Generated macro for serialized_size (function)
macro_rules! Depcrate_serserialized_size {
() => {
// Module: crate::ser
// Provides: {"serialized_size"}
// Dependencies: {}
# [doc = " Compute the size of the postcard serialization of `T`."] pub fn serialized_size < T > (value : & T) -> Result < usize > where T : Serialize + ? Sized , { serialize_with_flavor :: < T , flavors :: Size , usize > (value , flavors :: Size :: default ()) }
};
}
