// Generated macro for impl_137 (impl)
macro_rules! Depcrate_serimpl_137 {
() => {
// Module: crate::ser
// Provides: {"impl_137"}
// Dependencies: {}
impl < T > SerializeMap for erase :: Serializer < T > where T : serde :: Serializer , { fn erased_serialize_key (& mut self , key : & dyn Serialize) -> Result < () , ErrorImpl > { let erase :: Serializer :: Map (serializer) = self else { unreachable ! () ; } ; serializer . serialize_key (key) . map_err (| err | { * self = erase :: Serializer :: Error (err) ; ShortCircuit }) } fn erased_serialize_value (& mut self , value : & dyn Serialize) -> Result < () , ErrorImpl > { let erase :: Serializer :: Map (serializer) = self else { unreachable ! () ; } ; serializer . serialize_value (value) . map_err (| err | { * self = erase :: Serializer :: Error (err) ; ShortCircuit }) } fn erased_serialize_entry (& mut self , key : & dyn Serialize , value : & dyn Serialize ,) -> Result < () , ErrorImpl > { let erase :: Serializer :: Map (serializer) = self else { unreachable ! () ; } ; serializer . serialize_entry (key , value) . map_err (| err | { * self = erase :: Serializer :: Error (err) ; ShortCircuit }) } fn erased_end (& mut self) { let erase :: Serializer :: Map (serializer) = self . take () else { unreachable ! () ; } ; * self = match serializer . end () { Ok (ok) => erase :: Serializer :: Complete (ok) , Err (err) => erase :: Serializer :: Error (err) , } ; } }
};
}
