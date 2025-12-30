// Generated macro for impl_143 (impl)
macro_rules! Depcrate_serimpl_143 {
() => {
// Module: crate::ser
// Provides: {"impl_143"}
// Dependencies: {}
impl < T > SerializeStructVariant for erase :: Serializer < T > where T : serde :: Serializer , { fn erased_serialize_field (& mut self , key : & 'static str , value : & dyn Serialize ,) -> Result < () , ErrorImpl > { let erase :: Serializer :: StructVariant (serializer) = self else { unreachable ! () ; } ; serializer . serialize_field (key , value) . map_err (| err | { * self = erase :: Serializer :: Error (err) ; ShortCircuit }) } fn erased_skip_field (& mut self , key : & 'static str) -> Result < () , ErrorImpl > { let erase :: Serializer :: Struct (serializer) = self else { unreachable ! () ; } ; serializer . skip_field (key) . map_err (| err | { * self = erase :: Serializer :: Error (err) ; ShortCircuit }) } fn erased_end (& mut self) { let erase :: Serializer :: StructVariant (serializer) = self . take () else { unreachable ! () ; } ; * self = match serializer . end () { Ok (ok) => erase :: Serializer :: Complete (ok) , Err (err) => erase :: Serializer :: Error (err) , } ; } }
};
}
