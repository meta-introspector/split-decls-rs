// Generated macro for impl_134 (impl)
macro_rules! Depcrate_serimpl_134 {
() => {
// Module: crate::ser
// Provides: {"impl_134"}
// Dependencies: {}
impl < T > SerializeTupleVariant for erase :: Serializer < T > where T : serde :: Serializer , { fn erased_serialize_field (& mut self , value : & dyn Serialize) -> Result < () , ErrorImpl > { let erase :: Serializer :: TupleVariant (serializer) = self else { unreachable ! () ; } ; serializer . serialize_field (value) . map_err (| err | { * self = erase :: Serializer :: Error (err) ; ShortCircuit }) } fn erased_end (& mut self) { let erase :: Serializer :: TupleVariant (serializer) = self . take () else { unreachable ! () ; } ; * self = match serializer . end () { Ok (ok) => erase :: Serializer :: Complete (ok) , Err (err) => erase :: Serializer :: Error (err) , } ; } }
};
}
