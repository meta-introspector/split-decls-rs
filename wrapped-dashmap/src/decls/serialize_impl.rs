macro_rules! serialize_impl {
    () => {
        macro_rules ! serialize_impl { () => { fn serialize < Ser > (& self , serializer : Ser) -> Result < Ser :: Ok , Ser :: Error > where Ser : serde :: Serializer , { std :: ops :: Deref :: deref (self) . serialize (serializer) } } ; }
    };
}

serialize_impl!();