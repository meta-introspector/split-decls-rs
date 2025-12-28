macro_rules! deps {
    () => {
        Serialize!();
        ErrorImpl!();
        Serializer!();
        Result!();
        SerializeSeq!();
        Error!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < T > SerializeSeq for erase :: Serializer < T > where T : serde :: Serializer , { fn erased_serialize_element (& mut self , value : & dyn Serialize) -> Result < () , ErrorImpl > { let erase :: Serializer :: Seq (serializer) = self else { unreachable ! () ; } ; serializer . serialize_element (value) . map_err (| err | { * self = erase :: Serializer :: Error (err) ; ShortCircuit }) } fn erased_end (& mut self) { let erase :: Serializer :: Seq (serializer) = self . take () else { unreachable ! () ; } ; * self = match serializer . end () { Ok (ok) => erase :: Serializer :: Complete (ok) , Err (err) => erase :: Serializer :: Error (err) , } ; } }
    };
}

impl_97!();