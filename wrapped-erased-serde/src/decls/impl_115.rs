macro_rules! deps {
    () => {
        Result!();
        Serialize!();
        ErrorImpl!();
        Error!();
        Serializer!();
        SerializeStructVariant!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < T > SerializeStructVariant for erase :: Serializer < T > where T : serde :: Serializer , { fn erased_serialize_field (& mut self , key : & 'static str , value : & dyn Serialize ,) -> Result < () , ErrorImpl > { let erase :: Serializer :: StructVariant (serializer) = self else { unreachable ! () ; } ; serializer . serialize_field (key , value) . map_err (| err | { * self = erase :: Serializer :: Error (err) ; ShortCircuit }) } fn erased_skip_field (& mut self , key : & 'static str) -> Result < () , ErrorImpl > { let erase :: Serializer :: Struct (serializer) = self else { unreachable ! () ; } ; serializer . skip_field (key) . map_err (| err | { * self = erase :: Serializer :: Error (err) ; ShortCircuit }) } fn erased_end (& mut self) { let erase :: Serializer :: StructVariant (serializer) = self . take () else { unreachable ! () ; } ; * self = match serializer . end () { Ok (ok) => erase :: Serializer :: Complete (ok) , Err (err) => erase :: Serializer :: Error (err) , } ; } }
    };
}

impl_115!();