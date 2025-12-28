macro_rules! deps {
    () => {
        Error!();
        Serializer!();
        ErrorImpl!();
        Result!();
        Serialize!();
        SerializeTuple!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < T > SerializeTuple for erase :: Serializer < T > where T : serde :: Serializer , { fn erased_serialize_element (& mut self , value : & dyn Serialize) -> Result < () , ErrorImpl > { let erase :: Serializer :: Tuple (serializer) = self else { unreachable ! () ; } ; serializer . serialize_element (value) . map_err (| err | { * self = erase :: Serializer :: Error (err) ; ShortCircuit }) } fn erased_end (& mut self) { let erase :: Serializer :: Tuple (serializer) = self . take () else { unreachable ! () ; } ; * self = match serializer . end () { Ok (ok) => erase :: Serializer :: Complete (ok) , Err (err) => erase :: Serializer :: Error (err) , } ; } }
    };
}

impl_100!()