macro_rules! deps {
    () => {
        Result!();
        SerializeTupleVariant!();
        Error!();
        Serializer!();
        Serialize!();
        ErrorImpl!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < T > SerializeTupleVariant for erase :: Serializer < T > where T : serde :: Serializer , { fn erased_serialize_field (& mut self , value : & dyn Serialize) -> Result < () , ErrorImpl > { let erase :: Serializer :: TupleVariant (serializer) = self else { unreachable ! () ; } ; serializer . serialize_field (value) . map_err (| err | { * self = erase :: Serializer :: Error (err) ; ShortCircuit }) } fn erased_end (& mut self) { let erase :: Serializer :: TupleVariant (serializer) = self . take () else { unreachable ! () ; } ; * self = match serializer . end () { Ok (ok) => erase :: Serializer :: Complete (ok) , Err (err) => erase :: Serializer :: Error (err) , } ; } }
    };
}

impl_106!();