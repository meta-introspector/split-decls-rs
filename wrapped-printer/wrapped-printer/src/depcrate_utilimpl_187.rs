// Generated macro for impl_187 (impl)
macro_rules! Depcrate_utilimpl_187 {
() => {
// Module: crate::util
// Provides: {"impl_187"}
// Dependencies: {}
# [cfg (feature = "serde")] impl serde :: Serialize for NiceDuration { fn serialize < S : serde :: Serializer > (& self , ser : S ,) -> Result < S :: Ok , S :: Error > { use serde :: ser :: SerializeStruct ; let mut state = ser . serialize_struct ("Duration" , 3) ? ; state . serialize_field ("secs" , & self . 0 . as_secs ()) ? ; state . serialize_field ("nanos" , & self . 0 . subsec_nanos ()) ? ; state . serialize_field ("human" , & format ! ("{}" , self)) ? ; state . end () } }
};
}
